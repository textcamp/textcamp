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
    pub item_spawn: Vec<Spawn>,
    pub mob_spawn: Vec<Spawn>,
}

impl Space {
    pub fn new(entity_id: &Identifier) -> Self {
        Self {
            entity_id: entity_id.to_owned(),
            description: Description::default(),
            exits: HashMap::new(),
            population: Population::default(),
            inventory: Inventory::new(),
            item_spawn: vec![],
            mob_spawn: vec![],
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
        for s in self.item_spawn.iter() {
            // based on the chance of spawning
            if s.should_spawn(dice) {
                // do we already have the maximum inventory of this item?
                if self.inventory.count(&s.name) < s.max {
                    if let Some(item) = world.item_prototypes.create(&s.name) {
                        self.inventory.add(item);
                    }
                }
            }
        }

        let mut mob_counter: HashMap<String, usize> = HashMap::new();
        for p in self.population.prototypes(&world) {
            let count = mob_counter.entry(p).or_insert(0);
            *count += 1;
        }

        for s in self.mob_spawn.iter() {
            if s.should_spawn(dice) {
                // do we already have the maximum population of this mob?
                if mob_counter.get(&s.name).unwrap_or(&0) < &s.max {
                    if let Some(mut mob) = world.mob_prototypes.create(&s.name) {
                        mob.space_id = self.entity_id().clone();
                        self.population.add(mob.entity_id());
                        world.mobs.insert(mob);
                    }
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

impl Melee for Space {
    fn population(&self) -> &[Identifier] {
        self.population.identifiers()
    }

    fn melee(&mut self, world: &World, dice: &mut Dice) -> Vec<Update> {
        // filter out mobs that are dead or busy
        let mobs: Vec<Mob> = self
            .population()
            .iter()
            .flat_map(|id| world.mobs.get(id))
            .filter(|m| m.is_alive() && !m.is_busy())
            .collect();

        // get the list of attacks from the mobs
        let attacks: Vec<Attack> = mobs.iter().flat_map(|m| m.fight(&mobs, dice)).collect();

        let mut updates = vec![];

        // apply the attacks to the mobs
        for attack in attacks {
            if let Ok(mut target) = world.mobs.get(&attack.to) {
                trace!("Applying attack ... {:?}!", attack);
                updates.append(&mut target.harm(attack));

                // remove the dead from the population
                if target.is_dead() {
                    self.population.remove(target.entity_id());
                }

                // update the target mob
                world.mobs.insert(target);
            }
        }

        updates
    }
}

#[derive(Default, Clone, Debug)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
