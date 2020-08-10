pub mod cache;
pub mod mob;
pub mod space;

pub use mob::{Action, Doing, Mob, Restore};
pub use space::Space;

use crate::core::{Dice, Identifier, Markup, TCError, Update, World};
use log::trace;
use std::collections::HashMap;

pub trait Entity {
    fn identifier(&self) -> &Identifier;
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

pub trait Located {
    fn location(&self) -> &Identifier;
}

pub trait Melee {
    fn population(&self) -> &[Identifier];
    fn melee(&mut self, world: &World, dice: &mut Dice) -> Vec<Update>;
}

pub trait EntityStore<T: Entity + Clone> {
    fn get(&self, id: &Identifier) -> Result<T, TCError>;
    fn insert(&self, item: T);
    fn remove(&self, id: &Identifier);
}
