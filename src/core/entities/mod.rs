pub mod mob;
pub mod space;

pub use mob::{Attack, Doing, Mob, Restore};
pub use space::Space;

use crate::core::{Dice, Identifier, Markup, TCError, Update, World};
use std::collections::HashMap;
use std::sync::RwLock;

pub trait Entity {
    fn entity_id(&self) -> &Identifier;
}

pub trait Tickable {
    fn tick(&mut self, world: &World, dice: &mut Dice) -> Vec<Update>;
}

pub trait Fightable {
    fn is_alive(&self) -> bool;

    fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    fn fight(&self, world: &World, dice: &mut Dice) -> Option<Attack>;
    fn harm(&mut self, attack: Attack) -> Vec<Update>;
    fn restore(&mut self, restore: Restore) -> Vec<Update>;
}

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Describe {
    fn describe(&self, world: &World) -> Markup;
}

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

impl<F: Fightable> HashStore<F> {
    pub fn fight(&self, world: &World, dice: &mut Dice) -> Vec<Update> {
        let mut items = self.items.write().unwrap();
        let attacks: Vec<Attack> = items
            .values_mut()
            .flat_map(|i| i.fight(world, dice))
            .collect();

        attacks
            .into_iter()
            .flat_map(|a| items.get_mut(&a.to).map(|i| i.harm(a)).unwrap_or_default())
            .collect()
    }
}

impl<T: Entity + Clone> EntityStore<T> for HashStore<T> {
    fn get(&self, id: &Identifier) -> Result<T, TCError> {
        self.items
            .read()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| TCError::System(format!("HashStore - could not get {:?}", id)))
    }

    fn insert(&self, item: T) {
        self.items
            .write()
            .unwrap()
            .insert(item.entity_id().to_owned(), item);
    }
}
