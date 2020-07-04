use log::{error, trace};

use crate::core::entities::*;
use crate::core::*;

#[derive(Debug)]
pub struct Action {
    from: Identifier,
    phrase: Phrase,
}

impl Action {
    pub fn new(from: &Identifier, phrase: Phrase) -> Self {
        Self {
            from: from.to_owned(),
            phrase,
        }
    }
}

#[derive(Debug)]
pub struct World {
    // capabilities
    pub mobs: HashStore<Mob>,
    pub spaces: HashStore<Space>,

    // master list of item and mob templates
    pub item_prototypes: Prototypes<ItemPrototype>,
    pub mob_prototypes: Prototypes<MobPrototype>,

    // world clock
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
            mobs: HashStore::new(),
            spaces: HashStore::new(),
            item_prototypes: Prototypes::default(),
            mob_prototypes: Prototypes::default(),
            clock: Clock::new(1_000_000_000),
        }
    }

    pub fn tick(&mut self) -> Vec<Update> {
        let mut output = vec![];
        let mut dice = Dice::new();

        self.clock.tick();

        let mut new_spaces = HashStore::new();
        for space in self.spaces.all() {
            let mut new_space = space.clone();
            output.append(&mut new_space.tick(&self, &mut dice));
            new_spaces.insert(new_space);
        }
        self.spaces = new_spaces;

        let mut new_mobs = HashStore::new();
        for mob in self.mobs.all() {
            let mut new_mob = mob.clone();
            output.append(&mut new_mob.tick(&self, &mut dice));
            new_mobs.insert(new_mob);
        }
        self.mobs = new_mobs;

        output
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn command(&mut self, msg: Action) -> Vec<Update> {
        trace!("COMMAND - msg: {:?}", msg);

        let results = match msg.phrase.verb().to_uppercase().as_ref() {
            "LOOK" => self.look(&msg.from, msg.phrase.args().first()),
            "FIGHT" => self.fight(&msg.from, msg.phrase.args().first()),
            "GO" => self.go(&msg.from, msg.phrase.args().first()),
            "INVENTORY" => self.inventory(&msg.from),
            "TAKE" => self.take(&msg.from, msg.phrase.args().first()),
            "DROP" => self.drop(&msg.from, msg.phrase.args().first()),
            "REFRESH" => self.refresh(&msg.from),
            "TIME" => self.time(&msg.from),
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

    pub fn melee(&mut self) -> Vec<Update> {
        // get the attacks
        let attacks: Vec<Attack> = self
            .mobs
            .all()
            .iter()
            .flat_map(|c| c.fight(&self))
            .collect();

        // apply the attacks
        attacks
            .into_iter()
            .flat_map(|a| {
                if let Ok(original) = self.mobs.get(&a.to) {
                    let mut fighting_mob = original.clone();
                    let updates = fighting_mob.harm(a);
                    self.mobs.insert(fighting_mob);
                    updates
                } else {
                    vec![]
                }
            })
            .collect()
    }

    pub fn mob_space(&self, mob_id: &Identifier) -> Result<(&Mob, &Space), TCError> {
        let mob = self.mobs.get(mob_id)?;
        let space = self.spaces.get(&mob.space_id)?;

        Ok((mob, space))
    }

    // ACTIONS! ---------------------------------------------------------------------------------

    pub fn refresh(&self, mob_id: &Identifier) -> Result<Vec<Update>, TCError> {
        let mut output = vec![];
        let (mob, space) = self.mob_space(mob_id)?;

        output.push(Update::exits(mob_id, &space.exits));
        output.push(Update::character(mob_id, mob.describe(&self)));
        output.push(Update::population(mob_id, &space.population));
        output.push(Update::space(mob_id, space.describe(&self)));
        output.push(Update::time(mob_id, &self.clock.into()));
        output.push(Update::inventory(mob_id, &mob.inventory));

        Ok(output)
    }

    pub fn go(
        &mut self,
        mob_id: &Identifier,
        arg: Option<&String>,
    ) -> Result<Vec<Update>, TCError> {
        let mut output = vec![];

        // parse the direction
        let direction_src = arg.ok_or_else(|| TCError::user("Which way?"))?;
        let direction = Direction::from(direction_src)
            .ok_or_else(|| TCError::user("You can't go that way."))?;

        let mut mob = self.mobs.get(mob_id)?.clone();
        let mut current_space = self.spaces.get(&mob.space_id)?.clone();

        // get the new space ID based on the direction
        let new_space_id = current_space
            .exits
            .get(&direction)
            .ok_or_else(|| TCError::user("You can't go that way."))?
            .clone();

        let mut new_space = self.spaces.get(&new_space_id)?.clone();

        // remove them from the old space, add them to the new space
        current_space.population.remove(mob_id);
        new_space.population.add(mob_id);

        self.spaces.insert(current_space);
        self.spaces.insert(new_space);

        // set the location
        mob.space_id = new_space_id.to_owned();
        self.mobs.insert(mob); // save the updated mob

        let new_space = self.spaces.get(&new_space_id)?;

        // craft the update
        output.push(Update::space(mob_id, new_space.describe(&self)));
        output.push(Update::exits(mob_id, &new_space.exits));
        output.push(Update::population(mob_id, &new_space.population));

        Ok(output)
    }

    pub fn look(
        &mut self,
        mob_id: &Identifier,
        arg: Option<&String>,
    ) -> Result<Vec<Update>, TCError> {
        let (_, space) = self.mob_space(mob_id)?;

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

    pub fn take(
        &mut self,
        mob_id: &Identifier,
        arg: Option<&String>,
    ) -> Result<Vec<Update>, TCError> {
        let mut output = vec![];

        let item_name = arg.ok_or_else(|| TCError::user("Take what?"))?;

        let mut mob = self.mobs.get(mob_id)?.clone();
        let mut space = self.spaces.get(&mob.space_id)?.clone();

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

    pub fn drop(
        &mut self,
        mob_id: &Identifier,
        arg: Option<&String>,
    ) -> Result<Vec<Update>, TCError> {
        let mut output = vec![];

        let item_name = arg.ok_or_else(|| TCError::user("Take what?"))?;

        let mut mob = self.mobs.get(mob_id)?.clone();
        let mut space = self.spaces.get(&mob.space_id)?.clone();

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

    pub fn inventory(&self, mob_id: &Identifier) -> Result<Vec<Update>, TCError> {
        let mob = self.mobs.get(mob_id)?;
        let inventory = &mob.inventory;
        let update = Update::inventory(mob_id, inventory);

        Ok(vec![update])
    }

    pub fn fight(
        &mut self,
        mob_id: &Identifier,
        arg: Option<&String>,
    ) -> Result<Vec<Update>, TCError> {
        let name = arg.ok_or_else(|| TCError::user("Fight who?"))?;

        let (mob, space) = self.mob_space(mob_id)?;

        for local_mob in &space.population.mobs {
            let target_mob = self.mobs.get(&local_mob)?;
            if target_mob.name() == name {
                let mut player = mob.clone();
                player.add_enemy(local_mob);
                self.mobs.insert(player);
                let output = vec![Update::combat(mob_id, &format!("You attack {}!", name))];
                return Ok(output);
            }
        }

        Err(TCError::user("You don't see them here."))
    }

    pub fn time(&self, mob_id: &Identifier) -> Result<Vec<Update>, TCError> {
        Ok(vec![Update::time(mob_id, &self.clock.into())])
    }
}
