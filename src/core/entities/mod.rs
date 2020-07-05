pub mod mob;
pub mod space;

pub use mob::{Attack, Doing, Mob, Restore};
pub use space::Space;

use crate::core::{Dice, Identifier, Markup, TCError, Update, World};
use log::trace;
use std::collections::HashMap;
use std::sync::RwLock;

pub trait Entity {
    fn entity_id(&self) -> &Identifier;
}

// TODO: Expiration to clean up dead / inactive entities?
pub trait Tickable {
    fn tick(&mut self, world: &World, dice: &mut Dice) -> Vec<Update>;
}

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Describe {
    fn describe(&self, world: &World) -> Markup;
}

pub trait Located {
    fn location(&self) -> &Identifier;
}

pub trait Melee {
    fn population(&self) -> &[Identifier];
    fn melee(&mut self, world: &World, dice: &mut Dice) -> Vec<Update>;
}

// TODO: Removing entities!
pub trait EntityStore<T: Entity + Clone> {
    fn get(&self, id: &Identifier) -> Result<T, TCError>;
    fn insert(&self, item: T);
}

#[derive(Default, Debug)]
pub struct HashStore<T> {
    items: RwLock<HashMap<Identifier, T>>,
}

impl<T: Tickable> HashStore<T> {
    pub fn new() -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.items.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn tick(&self, world: &World, dice: &mut Dice) -> Vec<Update> {
        let mut items = self.items.write().unwrap();
        items
            .values_mut()
            .flat_map(|i| i.tick(world, dice))
            .collect()
    }
}

impl<F: Melee> HashStore<F> {
    pub fn melee(&self, world: &World, dice: &mut Dice) -> Vec<Update> {
        self.items
            .write()
            .unwrap()
            .values_mut()
            .flat_map(|space| space.melee(world, dice))
            .collect()
    }
}

impl<T: Entity + Clone + std::fmt::Debug> EntityStore<T> for HashStore<T> {
    fn get(&self, id: &Identifier) -> Result<T, TCError> {
        self.items
            .read()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| TCError::System(format!("HashStore - could not get {:?}", id)))
    }

    fn insert(&self, item: T) {
        trace!("HashStore - Inserting {:?}", item);

        self.items
            .write()
            .unwrap()
            .insert(item.entity_id().to_owned(), item);
    }
}
