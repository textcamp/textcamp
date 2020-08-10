use crate::core::*;

use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct Cache<T> {
    items: RwLock<HashMap<Identifier, T>>,
}

impl<T: Tickable> Cache<T> {
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
        self.items
            .write()
            .unwrap()
            .values_mut()
            .flat_map(|i| i.tick(world, dice))
            .collect()
    }
}

impl<F: Melee> Cache<F> {
    pub fn melee(&self, world: &World, dice: &mut Dice) -> Vec<Update> {
        self.items
            .write()
            .unwrap()
            .values_mut()
            .flat_map(|space| space.melee(world, dice))
            .collect()
    }
}

impl<T: Entity + Clone + std::fmt::Debug> EntityStore<T> for Cache<T> {
    fn get(&self, id: &Identifier) -> Result<T, TCError> {
        self.items
            .read()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| TCError::System(format!("Cache - could not get {:?}", id)))
    }

    fn insert(&self, item: T) {
        trace!("Cache - Inserting {:?}", item);

        self.items
            .write()
            .unwrap()
            .insert(item.identifier().to_owned(), item);
    }

    fn remove(&self, id: &Identifier) {
        trace!("Cache - Deleting {:?}", id);
        self.items.write().unwrap().remove(id);
    }
}
