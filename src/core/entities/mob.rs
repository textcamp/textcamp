use crate::core::entities::*;
use crate::core::*;
use crate::services::db::*;

use log::warn;
use serde::{Deserialize, Serialize};

use std::time::{Duration, Instant};

/// Represents a mobile characters in the game, including PCs (Player Characters) and NPCs (Non-Player Characters).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mob {
    /// A globally unique identifier for this specific Mob
    entity_id: Identifier,

    /// What prototype was this Mob derived from?
    pub prototype: String,

    /// The name of the Mob
    pub name: String,

    /// A description of the Mob
    pub description: Description,

    /// The current health of the character (eg: hit points, health points)
    pub hp: usize,

    /// The current location of the character (See )
    pub space_id: Identifier,

    /// Strength attribute
    pub strength: usize,

    /// Constitution attribute
    pub constitution: usize,

    /// Dexterity attribute
    pub dexterity: usize,

    /// Intelligence attribute
    pub intelligence: usize,

    /// Wisdom attribute
    pub wisdom: usize,

    /// Charisma attribute
    pub charisma: usize,

    /// The inventory for this Mob
    pub inventory: Inventory,

    /// A unique, sorted list of Mobs that will be attacked on sight
    pub enemies: Vec<Identifier>,

    /// Determines whether a Mob is currently busy with an activity
    #[serde(skip)]
    pub delay: Option<Instant>,

    /// What the Mob is currently doing
    pub doing: Doing,
}

impl Mob {
    pub fn new() -> Self {
        Self {
            entity_id: Identifier::random(),
            name: String::new(),
            prototype: String::new(),
            description: Description::default(),
            space_id: Identifier::origin(),
            inventory: Inventory::new(),

            strength: 0,
            constitution: 0,
            dexterity: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
            hp: 0,

            enemies: vec![],
            delay: None,
            doing: Doing::Nothing,
        }
    }

    pub fn add_enemy(&mut self, identifier: &Identifier) {
        self.enemies.push(identifier.to_owned());
        self.enemies.sort();
        self.enemies.dedup();
    }

    pub fn remove_enemy(&mut self, identifier: &Identifier) {
        self.enemies = self
            .enemies
            .iter()
            .filter(|e| e != &identifier)
            .cloned()
            .collect();
    }

    pub fn doing(&self) -> &Doing {
        if self.is_busy() {
            &self.doing
        } else {
            &Doing::Nothing
        }
    }

    pub fn busy(&mut self, doing: Doing, duration: Duration) {
        self.doing = doing;
        self.delay = Some(Instant::now() + duration);
    }

    pub fn is_busy(&self) -> bool {
        if let Some(delay) = self.delay {
            return delay >= Instant::now();
        }
        false
    }

    pub fn max_health(&self) -> usize {
        self.constitution
    }

    pub fn health(&self) -> usize {
        (self.hp * 100) / self.constitution
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    pub fn fight(&self, mobs: &[Mob], _dice: &mut Dice) -> Vec<Action> {
        // no enemies? no worries
        if self.enemies.is_empty() {
            trace!("{} won't fight - no enemies!", self.name());
            return vec![];
        }

        // see if any of the mobs are enemies.
        let enemies: Vec<Mob> = mobs
            .iter()
            .filter(|m| self.enemies.contains(m.entity_id())) // are they in your enemy list?
            .cloned()
            .collect();

        trace!("{} has enemies here! {:?}", self.name(), enemies);

        // target the first enemy, if there are any
        let target = match enemies.first() {
            Some(mob) => mob,
            None => {
                return vec![];
            }
        };

        // TODO: determine different kinds of attacks
        // TODO: multiple attacks
        // TODO: healing actions
        let attack = Action::new(
            &self.entity_id,
            &target.entity_id,
            Effect::Harm(Damage::Blunt(1)),
        );

        vec![attack]
    }

    pub fn act(&mut self, action: Action, world: &World) -> Vec<Update> {
        if action.to != self.entity_id {
            return vec![];
        }

        let from_mob = match world.mobs.get(&action.from) {
            Ok(m) => m,
            Err(e) => {
                warn!("ACT - from_mob - {:?}", e);
                return vec![];
            }
        };

        match action.effect {
            Effect::Harm(damage) => self.harm(&from_mob, damage, world),
            Effect::Heal(_restore) => vec![],
        }
    }

    fn harm(&mut self, attacker: &Mob, damage: Damage, _world: &World) -> Vec<Update> {
        if damage.health() >= self.hp {
            self.hp = 0;
        } else {
            self.hp -= damage.health();
        }

        let target_update =
            Update::combat(&self.entity_id, format!("{} hurt you!", attacker.name()));

        let attacker_update = if self.is_dead() {
            Update::combat(&attacker.entity_id, format!("You killed {}!", self.name()))
        } else {
            Update::combat(&attacker.entity_id, format!("You hit {}!", self.name()))
        };

        vec![target_update, attacker_update]
    }

    fn heal(&mut self, _healer: Option<&Mob>, restore: Restore, _world: &World) -> Vec<Update> {
        let new_hp = self.hp + restore.health();
        if new_hp > self.constitution {
            self.hp = self.constitution;
        } else {
            self.hp = new_hp;
        }

        vec![Update::health(&self.entity_id, self.health())]
    }
}

impl Default for Mob {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for Mob {
    fn entity_id(&self) -> &Identifier {
        &self.entity_id
    }
}

impl Tickable for Mob {
    fn tick(&mut self, world: &World, _dice: &mut Dice) -> Vec<Update> {
        let mut output = vec![];

        // default healing per tick
        output.append(&mut self.heal(None, Restore::Health(1), world));

        // time transition? hero? let the player know
        if self.prototype.eq("HERO") {
            for transition in world.clock().transition() {
                match transition {
                    Transition::Morning => output.push(Update::transition(
                        self.entity_id(),
                        "The sky lightens in the east.",
                    )),
                    Transition::Day => {
                        output.push(Update::transition(self.entity_id(), "It is day time."))
                    }
                    Transition::Evening => output.push(Update::transition(
                        self.entity_id(),
                        "The sky darkens as the sun sets in the west.",
                    )),
                    Transition::Night => output.push(Update::transition(
                        self.entity_id(),
                        "Night falls and the stars appear.",
                    )),
                    Transition::Spring => {
                        output.push(Update::transition(self.entity_id(), "Spring has arrived."))
                    }
                    Transition::Summer => {
                        output.push(Update::transition(self.entity_id(), "Summer has arrived."))
                    }
                    Transition::Autumn => {
                        output.push(Update::transition(self.entity_id(), "Autumn has arrived."))
                    }
                    Transition::Winter => {
                        output.push(Update::transition(self.entity_id(), "Winter has arrived."))
                    }
                }
            }
        }

        output
    }
}

impl Describe for Mob {
    fn describe(&self, _world: &World) -> Markup {
        Markup {
            text: self.description.text.clone(),
            clicks: self.description.clicks.clone(),
        }
    }
}

impl Named for Mob {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Located for Mob {
    fn location(&self) -> &Identifier {
        &self.space_id
    }
}

impl DynamoRecord for Mob {}

impl HasPrimaryKey for Mob {
    fn primary_key(&self) -> String {
        self.entity_id.value.to_owned()
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Doing {
    Nothing,
    Casting,
    Fighting,
}

impl Default for Doing {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Debug, Clone)]
pub enum Damage {
    Blunt(usize),
    Edged(usize),
    Pointed(usize),
    Poison(usize),
    Fire(usize),
}

impl Damage {
    pub fn health(&self) -> usize {
        match self {
            Self::Blunt(hp) => *hp,
            Self::Edged(hp) => *hp,
            Self::Pointed(hp) => *hp,
            Self::Poison(hp) => *hp,
            Self::Fire(hp) => *hp,
        }
    }
}

#[derive(Debug)]
pub enum Restore {
    Max,
    Health(usize),
    Antidote(usize),
}

impl Restore {
    pub fn health(&self) -> usize {
        match self {
            Self::Max => std::usize::MAX,
            Self::Antidote(hp) => *hp,
            Self::Health(hp) => *hp,
        }
    }
}

#[derive(Debug)]
pub struct Action {
    pub from: Identifier,
    pub to: Identifier,
    pub effect: Effect,
}

impl Action {
    pub fn new(from: &Identifier, to: &Identifier, effect: Effect) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
            effect,
        }
    }
}

#[derive(Debug)]
pub enum Effect {
    Harm(Damage),
    Heal(Restore),
}
