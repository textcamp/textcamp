pub mod item_prototype;
pub mod mob_prototype;

pub use item_prototype::ItemPrototype;
pub use mob_prototype::MobPrototype;

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Prototypes<T: Prototyped> {
    things: HashMap<String, T>,
}

impl<T: Prototyped> Prototypes<T> {
    pub fn add(&mut self, p: T) {
        self.things.insert(p.name().to_owned(), p);
    }

    pub fn create(&self, key: &str) -> Option<T::Item> {
        match self.things.get(key) {
            None => None,
            Some(prototype) => Some(prototype.create()),
        }
    }
}

pub trait Prototyped {
    type Item;
    fn create(&self) -> Self::Item;
    fn name(&self) -> &str;
}
