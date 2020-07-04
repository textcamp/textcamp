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
}
