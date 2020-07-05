use serde::Deserialize;

use crate::core::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Spawn {
    pub name: String,
    pub max: usize,
    pub chance: usize,
}

impl Spawn {
    pub fn should_spawn(&self, dice: &mut Dice) -> bool {
        // we have a 1 in x chance of spawning
        dice.range(0, self.chance) == 0
    }
}
