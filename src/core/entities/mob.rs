use crate::core::entities::*;
use crate::core::*;

use std::time::{Duration, Instant};

/// Represents a mobile characters in the game, including PCs (Player Characters) and NPCs (Non-Player Characters).
#[derive(Clone, Debug)]
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
    pub health: usize,

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
    pub delay: Instant,

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
            health: 0,

            enemies: vec![],
            delay: Instant::now(),
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
        if self.delay <= Instant::now() {
            &Doing::Nothing
        } else {
            &self.doing
        }
    }

    pub fn busy(&mut self, doing: Doing, duration: Duration) {
        self.doing = doing;
        self.delay = Instant::now() + duration;
    }

    pub fn is_busy(&self) -> bool {
        self.delay >= Instant::now()
    }

    pub fn max_health(&self) -> usize {
        self.constitution
    }
}

impl Fightable for Mob {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn fight(&self, world: &World, _dice: &mut Dice) -> Option<Attack> {
        // dead characters can't fight!
        if self.is_dead() {
            return None;
        }

        // no enemies? no worries
        if self.enemies.is_empty() {
            return None;
        }

        // busy characters can't fight either!
        if self.is_busy() {
            return None;
        }

        // get the characters in the current space.
        let this_mob = world.mobs.get(&self.entity_id).ok()?;
        let space = world.spaces.get(&this_mob.space_id).ok()?;

        // see if any of them are enemies.
        let enemies: Vec<Identifier> = space
            .population
            .mobs
            .iter()
            .filter(|c| self.enemies.contains(c))
            .cloned()
            .collect();

        // target the first enemy, if there are any
        let target_id = match enemies.first() {
            Some(id) => id,
            None => {
                return None;
            }
        };

        // TODO: determine different kinds of attacks
        let attack = Attack::new(&self.entity_id, target_id, Damage::Blunt(1));

        Some(attack)
    }

    fn harm(&mut self, attack: Attack) -> Vec<Update> {
        let mut output = vec![];

        // TODO: ensure the attack is directed at this particular character
        // TODO: handle different kinds of damage
        if attack.damage.health() >= self.health {
            self.health = 0;
        } else {
            self.health -= attack.damage.health();
        }

        // TODO: handle death and subsequent activites (looting? xp? oh my!)
        let target_update = Update::combat(&attack.to, "You've been hurt!");

        let attacker_update = if self.is_dead() {
            Update::combat(&attack.from, &format!("You killed {:?}!", self.name()))
        } else {
            Update::combat(&attack.from, &format!("You hit {:?}!", self.name()))
        };

        output.push(target_update);
        output.push(attacker_update);

        output
    }

    fn restore(&mut self, restore: Restore) -> Vec<Update> {
        let mut output = vec![];

        // TODO: handle different kinds of healing
        let new_hp = self.health + restore.heal.health();
        if new_hp >= self.max_health() {
            self.health = self.max_health();
        } else {
            self.health = new_hp;
        }

        let target_update = Update::combat(&restore.to, "You've been healed!");

        let attacker_update =
            Update::combat(&restore.from, &format!("You healed {:?}!", self.name()));

        output.push(target_update);
        output.push(attacker_update);

        output
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
    fn tick(&mut self, _world: &World, _dice: &mut Dice) -> Vec<Update> {
        vec![]
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

#[derive(Default, Clone, Debug)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug)]
pub struct Attack {
    pub from: Identifier,
    pub to: Identifier,
    pub damage: Damage,
}

impl Attack {
    pub fn new(from: &Identifier, to: &Identifier, damage: Damage) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            damage,
        }
    }
}

#[derive(Debug)]
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
pub struct Restore {
    pub from: Identifier,
    pub to: Identifier,
    pub heal: Heal,
}

#[derive(Debug)]
pub enum Heal {
    Max,
    Health(usize),
    Antidote(usize),
}

impl Heal {
    pub fn health(&self) -> usize {
        match self {
            Self::Max => std::usize::MAX,
            Self::Antidote(hp) => *hp,
            Self::Health(hp) => *hp,
        }
    }
}