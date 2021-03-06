use log::{error, trace, warn};

use crate::core::entities::cache::*;
use crate::core::entities::*;
use crate::core::*;
use crate::services::{accounts::Account, db::Dynamo};

use std::time::Instant;

type CommandOutput = Result<Vec<Update>, TCError>;

/// Represents a command from a player, sent from the Connection actor into
/// the shared World instance.
#[derive(Debug)]
pub struct Command {
    from: Identifier,
    phrase: Phrase,
}

impl Command {
    pub fn new(from: &Identifier, phrase: Phrase) -> Self {
        Self {
            from: from.to_owned(),
            phrase,
        }
    }
}

/// Represents the state of the world. Deep, man.
#[derive(Debug)]
pub struct World {
    /// Token and authentication management.
    pub authentication: Authentication,

    /// Cache for Mobs
    pub mobs: Cache<Mob>,

    /// Cache for Spaces
    pub spaces: Cache<Space>,

    /// Master list of Item templates
    pub item_prototypes: Prototypes<ItemPrototype>,

    /// Master list of Mob templates
    pub mob_prototypes: Prototypes<MobPrototype>,

    /// World clock
    clock: Clock,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            authentication: Authentication::new(),
            mobs: Cache::new(),
            spaces: Cache::new(),
            item_prototypes: Prototypes::default(),
            mob_prototypes: Prototypes::default(),
            clock: Clock::new(1_000_000_000),
        }
    }

    /// Performs a game "tick" by iterating through all of the
    /// cached (active) mobs and spaces.
    pub fn tick(&mut self) -> Vec<Update> {
        let mut output = vec![];
        let mut dice = Dice::new();

        self.clock.tick();

        output.append(&mut self.spaces.tick(&self, &mut dice));
        output.append(&mut self.mobs.tick(&self, &mut dice));

        output
    }

    /// Performs a round of melee combat
    pub fn melee(&self) -> Vec<Update> {
        let started = Instant::now();
        let output = self.spaces.melee(&self, &mut Dice::new());

        trace!("⚔️  {:?} - {:?}", started.elapsed(), output);

        output
    }

    /// Provides a reference to the world clock
    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    /// Handles input from the clients, called by the Connection actor
    pub async fn command(&mut self, msg: Command) -> Vec<Update> {
        trace!("COMMAND - msg: {:?}", msg);

        let results = match msg.phrase.verb().to_uppercase().as_ref() {
            "LOOK" => self.look(&msg.from, msg.phrase.args().first()).await,
            "FIGHT" => self.fight(&msg.from, msg.phrase.args().first()).await,
            "GO" => self.go(&msg.from, msg.phrase.args().first()).await,
            "INVENTORY" => self.inventory(&msg.from).await,
            "TAKE" => self.take(&msg.from, msg.phrase.args().first()).await,
            "DROP" => self.drop(&msg.from, msg.phrase.args().first()).await,
            "REFRESH" => self.refresh(&msg.from).await,
            "TIME" => self.time(&msg.from).await,
            "SAVE" => self.save(&msg.from).await,
            "QUIT" => self.quit(&msg.from).await,
            _ => Err(TCError::user("... What?")),
        };

        let updates = match results {
            Ok(messages) => messages,
            Err(tce) => match tce {
                TCError::Fatal(f) => {
                    error!("Received fatal error: {:?}", f);
                    std::process::exit(1);
                }
                TCError::User(message) => vec![Update::error(&msg.from, &message)],
                TCError::System(message) => {
                    error!("{}", message);
                    vec![Update::error(&msg.from, "Something went wrong!")]
                }
            },
        };

        trace!("COMMAND - updates: {:?}", updates);

        updates
    }

    /// Validates the OTP token in the e-mail authentication flow
    pub async fn authenticate_otp(&mut self, otp_token: String) -> Option<String> {
        let account_email = self.authentication.consume_otp_token(otp_token)?;
        trace!("Good OTP, looking up account for {}", account_email);

        let db = Dynamo::new();
        let account = match db.accounts.get::<Account>(&account_email).await {
            Some(account) => {
                // we have an account; make sure the hero is loaded into the local cache
                if self.load_hero(&account.identifier).await.is_none() {
                    // whoa, we lost the hero!! bad move!!
                    error!("Lost hero for {:?}", account);
                    return None;
                };

                account
            }
            None => {
                trace!("No account found for {}", account_email);
                // create a new hero and account because we don't have one!
                let identifier = self.create_hero().await;
                let account = Account {
                    email: account_email.clone(),
                    identifier,
                };
                if let Err(e) = db.accounts.put(&account).await {
                    warn!("Error creating account: {:?} => {}", account, e);
                }

                account
            }
        };

        trace!("Setting up session for {:?}", account);
        let session_token = self.authentication.start_session(&account.identifier).await;
        Some(session_token)
    }

    /// Validates the session token to support reconnections
    pub async fn authenticate_session(&self, session_token: &str) -> Option<Identifier> {
        let identifier = self.authentication.valid_session(session_token).await?;
        if self.load_hero(&identifier).await.is_none() {
            error!(
                "Lost hero for valid session {} => {:?}",
                session_token, identifier
            );
            return None;
        };
        Some(identifier)
    }

    /// Creates a new hero from the "HERO" prototype, and puts them in the "ORIGIN" space.
    /// If either can't be found, this will panic!
    pub async fn create_hero(&self) -> Identifier {
        let mut hero = self
            .mob_prototypes
            .create("HERO")
            .expect("Could not find HERO prototype!!");

        let mut origin = self
            .spaces
            .get(&Identifier::origin())
            .expect("Could not load ORIGIN space!!");

        hero.space_id = origin.identifier().clone();
        hero.name = format!("Hero{}", rand::thread_rng().gen::<u8>());
        let hero_identifier = hero.identifier().clone();

        // add them to the local cache
        self.mobs.insert(hero.clone());

        let db = crate::services::db::Dynamo::new();
        db.mobs
            .put::<Mob>(&hero)
            .await
            .expect("Failed to persist Hero!");

        origin.population.add(&hero_identifier);
        self.spaces.insert(origin);

        hero_identifier
    }

    /// Retrieves a Mob from long term storage, inserts it into the mob cache, and adds
    /// it to it's assigned space.
    pub async fn load_hero(&self, identifier: &Identifier) -> Option<Identifier> {
        let db = Dynamo::new();
        let mut hero = match db.mobs.get::<Mob>(&identifier.value).await {
            Some(h) => h,
            None => return None,
        };

        let identifier = hero.identifier.clone();

        let mut space = match self.spaces.get(&hero.space_id) {
            Ok(s) => s,
            Err(e) => {
                error!(
                    "Couldn't find space {}, relocating hero {} to ORIGIN. {:?}",
                    hero.space_id, hero.identifier, e
                );

                let origin = self
                    .spaces
                    .get(&Identifier::origin())
                    .expect("Could not load ORIGIN space!!");

                hero.space_id = origin.identifier().clone();

                origin
            }
        };

        space.population.add(hero.identifier());

        self.mobs.insert(hero);
        self.spaces.insert(space);

        Some(identifier)
    }

    // ACTIONS! ---------------------------------------------------------------------------------

    async fn refresh(&self, mob_id: &Identifier) -> CommandOutput {
        let mob = self.mobs.get(mob_id)?;
        let space = self.spaces.get(&mob.space_id)?;

        let mut output = vec![];

        output.push(Update::character(mob_id, mob.describe(&self)));
        output.push(Update::population(mob_id, space.population.names(&self)));
        output.push(Update::space(mob_id, space.describe(&self)));
        output.push(Update::time(mob_id, &self.clock.into()));
        output.push(Update::inventory(mob_id, &mob.inventory));
        output.push(Update::health(mob_id, mob.health()));

        Ok(output)
    }

    async fn go(&mut self, mob_id: &Identifier, arg: Option<&String>) -> CommandOutput {
        let mut output = vec![];

        // parse the direction
        let direction_src = arg.ok_or_else(|| TCError::user("Which way?"))?;
        let direction = direction_src.to_lowercase();

        let mut mob = self.mobs.get(mob_id)?;
        let mut current_space = self.spaces.get(&mob.space_id)?;

        // get the new space ID based on the direction
        let new_space_id = current_space
            .exits
            .get(&direction)
            .ok_or_else(|| TCError::user("You can't go that way."))?
            .clone();

        let mut new_space = self.spaces.get(&new_space_id)?;

        // remove them from the old space, add them to the new space
        current_space.population.remove(mob_id);
        new_space.population.add(mob_id);

        self.spaces.insert(current_space);
        self.spaces.insert(new_space);

        // set the location
        mob.space_id = new_space_id.to_owned();
        // reset the enemies list
        mob.enemies = vec![];
        self.mobs.insert(mob); // save the updated mob

        let new_space = self.spaces.get(&new_space_id)?;

        // craft the update
        output.push(Update::space(mob_id, new_space.describe(&self)));
        output.push(Update::population(
            mob_id,
            new_space.population.names(&self),
        ));

        Ok(output)
    }

    async fn look(&mut self, mob_id: &Identifier, arg: Option<&String>) -> CommandOutput {
        let mob = self.mobs.get(mob_id)?;
        let space = self.spaces.get(&mob.space_id)?;

        // if arg is None, we're looking at the current space description. Easy!
        if arg.is_none() {
            let update = Update::space(mob_id, space.describe(&self));
            return Ok(vec![update]);
        };

        // otherwise, we're search populations and items ...
        let name = arg.unwrap();

        for character_id in &space.population.mobs {
            let maybe_mob = self.mobs.get(&character_id)?;
            if maybe_mob.name() == name {
                let update = Update::character(mob_id, maybe_mob.describe(&self));
                return Ok(vec![update]);
            }
        }

        for item in space.inventory.items() {
            if item.name() == name {
                let update = Update::item(mob_id, item.describe(&self));
                return Ok(vec![update]);
            }
        }

        Err(TCError::user("You don't see that here."))
    }

    async fn take(&mut self, mob_id: &Identifier, arg: Option<&String>) -> CommandOutput {
        let mut output = vec![];

        let item_name = arg.ok_or_else(|| TCError::user("Take what?"))?;

        let mut mob = self.mobs.get(mob_id)?;
        let mut space = self.spaces.get(&mob.space_id)?;

        if let Some(item) = space.inventory.remove(item_name) {
            mob.inventory.add(item);
        } else {
            return Err(TCError::user("You don't see that."));
        }

        output.push(Update::info(
            mob_id,
            &format!("You took the {}.", item_name),
        ));

        output.push(Update::inventory(mob_id, &mob.inventory));
        output.push(Update::space(mob_id, space.describe(&self)));

        self.mobs.insert(mob);
        self.spaces.insert(space);

        Ok(output)
    }

    async fn drop(&mut self, mob_id: &Identifier, arg: Option<&String>) -> CommandOutput {
        let mut output = vec![];

        let item_name = arg.ok_or_else(|| TCError::user("Take what?"))?;

        let mut mob = self.mobs.get(mob_id)?;
        let mut space = self.spaces.get(&mob.space_id)?;

        if let Some(item) = mob.inventory.remove(item_name) {
            space.inventory.add(item);
        } else {
            return Err(TCError::user("You don't have that."));
        }

        output.push(Update::info(
            mob_id,
            &format!("You dropped {:?}.", item_name),
        ));

        output.push(Update::inventory(mob_id, &mob.inventory));
        output.push(Update::space(mob_id, space.describe(&self)));

        self.mobs.insert(mob);
        self.spaces.insert(space);

        Ok(output)
    }

    async fn inventory(&self, mob_id: &Identifier) -> CommandOutput {
        let mob = self.mobs.get(mob_id)?;
        let inventory = &mob.inventory;
        let update = Update::inventory(mob_id, inventory);

        Ok(vec![update])
    }

    async fn fight(&mut self, mob_id: &Identifier, arg: Option<&String>) -> CommandOutput {
        let target_name = arg.ok_or_else(|| TCError::user("Fight who?"))?;

        let mob = self.mobs.get(mob_id)?;
        let space = self.spaces.get(&mob.space_id)?;

        for local_mob in &space.population.mobs {
            let mut target_mob = self.mobs.get(&local_mob)?;
            // add the enemy if the name matches and it isn't yourself!
            if target_mob.name() == target_name && target_mob.identifier() != mob.identifier() {
                let mut player = mob;

                // both mobs become enemies!
                player.add_enemy(target_mob.identifier());
                target_mob.add_enemy(player.identifier());

                let mut output = vec![];

                output.push(Update::combat(
                    player.identifier(),
                    format!("You attack {}!", target_name),
                ));

                output.push(Update::combat(
                    target_mob.identifier(),
                    format!("{} attacks you!", player.name()),
                ));

                self.mobs.insert(player);
                self.mobs.insert(target_mob);

                return Ok(output);
            }
        }

        Err(TCError::user("You don't see them here."))
    }

    async fn time(&self, mob_id: &Identifier) -> CommandOutput {
        Ok(vec![Update::time(mob_id, &self.clock.into())])
    }

    async fn save(&self, mob_id: &Identifier) -> CommandOutput {
        let mob = self.mobs.get(mob_id)?;
        let db = crate::services::db::Dynamo::new();

        match db.mobs.put(&mob).await {
            Ok(_) => Ok(vec![Update::info(mob_id, "Saved!")]),
            Err(e) => {
                warn!("db.mobs.put ERROR: {}", e);
                Err(TCError::user("Something went wrong ..."))
            }
        }
    }

    async fn quit(&self, mob_id: &Identifier) -> CommandOutput {
        // fetch the affected entities
        let mob = self.mobs.get(mob_id)?;
        let mut space = self.spaces.get(&mob.space_id)?;

        // remove the mob from the population of the space
        space.population.remove(mob.identifier());

        // save the space
        self.spaces.insert(space);

        // axe the mob from the cache
        self.mobs.remove(mob_id);

        // say buh-bye!
        Ok(vec![Update::info(mob_id, "See you later!")])

        // TODO: Figure out how to close the connection!!
    }
}
