use crate::core::entities::*;
use crate::core::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Space {
    entity_id: Identifier,
    pub description: Description,
    pub exits: HashMap<Direction, Identifier>,
    pub population: Population,
    pub inventory: Inventory,
    pub spawn: Vec<Spawn>,
}

impl Space {
    pub fn new(entity_id: &Identifier) -> Self {
        Self {
            entity_id: entity_id.to_owned(),
            description: Description::default(),
            exits: HashMap::new(),
            population: Population::default(),
            inventory: Inventory::new(),
            spawn: vec![],
        }
    }
}

impl Entity for Space {
    fn entity_id(&self) -> &Identifier {
        &self.entity_id
    }
}

impl Tickable for Space {
    fn tick(&mut self, world: &World, dice: &mut Dice) -> Vec<Update> {
        for s in self.spawn.iter() {
            if s.should_spawn(dice, &self) {
                trace!("{} spawning {}", self.entity_id.value, s.item);
                if let Some(item) = world.item_prototypes.create(&s.item) {
                    self.inventory.add(item.clone());
                }
            }
        }
        vec![]
    }
}

impl Describe for Space {
    fn describe(&self, _world: &World) -> Markup {
        let mut text = self.description.text.clone();

        if !self.inventory.is_empty() {
            let items: Vec<&str> = self.inventory.items().iter().map(|i| i.name()).collect();

            text += &format!("\n\nYou see {} here.", items.join(", "));
        }

        Markup {
            text,
            clicks: self.description.clicks.clone(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
