use crate::core::*;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Population {
    pub mobs: Vec<Identifier>,
}

impl Population {
    pub fn add(&mut self, identifier: &Identifier) {
        self.mobs.push(identifier.to_owned());
        self.mobs.sort();
        self.mobs.dedup();
    }

    pub fn remove(&mut self, identifier: &Identifier) {
        self.mobs = self
            .mobs
            .iter()
            .filter(|id| id != &identifier)
            .cloned()
            .collect();
    }

    pub fn identifiers(&self) -> &[Identifier] {
        &self.mobs
    }

    pub fn names(&self, world: &World) -> Vec<String> {
        self.mobs
            .iter()
            .flat_map(|i| world.mobs.get(i))
            .map(|m| m.name)
            .collect()
    }

    pub fn prototypes(&self, world: &World) -> Vec<String> {
        self.mobs
            .iter()
            .flat_map(|i| world.mobs.get(i))
            .map(|m| m.prototype)
            .collect()
    }
}
