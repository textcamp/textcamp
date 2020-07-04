pub mod mob;
pub mod space;

pub use mob::{Attack, Doing, Mob};
pub use space::Space;

use crate::core::{Dice, Identifier, Markup, TCError, Update, World};
use std::collections::HashMap;

pub trait Entity {
    fn entity_id(&self) -> &Identifier;
}

pub trait Tickable {
    fn tick(&mut self, world: &World, dice: &mut Dice) -> Vec<Update>;
}

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Describe {
    fn describe(&self, world: &World) -> Markup;
}

pub trait EntityStore<T: Entity + Clone> {
    fn get(&self, id: &Identifier) -> Result<&T, TCError>;
    fn insert(&mut self, item: T);
    fn all(&self) -> Vec<&T>;
}

#[derive(Default, Clone, Debug)]
pub struct HashStore<T> {
    items: HashMap<Identifier, T>,
}

impl<T> HashStore<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Entity + Clone> EntityStore<T> for HashStore<T> {
    fn get(&self, id: &Identifier) -> Result<&T, TCError> {
        self.items
            .get(id)
            .ok_or_else(|| TCError::System(format!("HashStore - could not get {:?}", id)))
    }

    fn insert(&mut self, item: T) {
        self.items.insert(item.entity_id().to_owned(), item);
    }

    fn all(&self) -> Vec<&T> {
        self.items.values().collect()
    }
}
