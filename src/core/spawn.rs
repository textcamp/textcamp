use serde::Deserialize;

use crate::core::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Spawn {
    pub item: String,
    pub max: usize,
    pub chance: usize,
}

impl Spawn {
    pub fn should_spawn(&self, dice: &mut Dice, space: &Space) -> bool {
        // we have a 1 in x chance of spawning
        if dice.range(0, self.chance) != 0 {
            return false;
        }

        // we also have a maximum number of these items that can exist in the space.
        if space.inventory.count(&self.item) >= self.max {
            return false;
        }

        true
    }
}
