pub mod item_prototype;
pub mod mob_prototype;

pub use item_prototype::ItemPrototype;
pub use mob_prototype::MobPrototype;

use log::{trace, warn};

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Prototypes<T: Prototyped> {
    things: HashMap<String, T>,
}

impl<T: Prototyped> Prototypes<T> {
    pub fn add(&mut self, p: T) {
        self.things.insert(p.prototype_name(), p);
    }

    pub fn create(&self, key: &str) -> Option<T::Item> {
        match self.things.get(key) {
            None => {
                warn!("FAILED TO SPAWN - {}", key);
                None
            }
            Some(prototype) => {
                let thing = prototype.create();
                trace!("SPAWNING - {}", key);
                Some(thing)
            }
        }
    }
}

pub trait Prototyped {
    type Item;
    fn create(&self) -> Self::Item;
    fn prototype_name(&self) -> String;
}
