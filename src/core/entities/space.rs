use crate::core::entities::*;
use crate::core::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Space {
    entity_id: Identifier,
    pub description: Description,
    pub exits: HashMap<String, Identifier>,
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

    pub fn population_update(&self, world: &World) -> Vec<Update> {
        self.population()
            .iter()
            .map(|id| Update::population(id, self.population.names(world)))
            .collect()
    }

    pub fn space_update(&self, world: &World) -> Vec<Update> {
        self.population()
            .iter()
            .map(|id| Update::space(id, self.describe(world)))
            .collect()
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

        let mut output = vec![];

        output.append(&mut self.population_update(world));
        output.append(&mut self.space_update(world));

        output
    }
}

impl Describe for Space {
    fn describe(&self, _world: &World) -> Markup {
        let mut text = self.description.text.clone();
        let mut clicks = self.description.clicks.clone();

        if !self.inventory.is_empty() {
            // build up a count for each item in the space
            let mut item_counts: HashMap<String, usize> = HashMap::new();

            self.inventory.items().iter().for_each(|i| {
                let count = item_counts.entry(i.name().to_owned()).or_insert(0);
                *count += 1;
            });

            // use the count to create descriptive slugs for each kind of item
            let mut item_slugs = vec![];

            for (name, count) in item_counts.into_iter() {
                let slug = match count {
                    1 => format!("a [[{}]]", name),
                    2 => format!("a couple of [[{}]]s", name),
                    _ => format!("several [[{}]]s", name),
                };

                item_slugs.push(slug);

                // make the items clickable
                clicks.insert(name.clone(), format!("take {}", name));
            }

            text += &format!("\n\nYou see {} here.", item_slugs.join(", "));
        }

        Markup { text, clicks }
    }
}

impl Melee for Space {
    fn population(&self) -> &[Identifier] {
        self.population.identifiers()
    }

    fn melee(&mut self, world: &World, dice: &mut Dice) -> Vec<Update> {
        let population_count = self.population().len();

        // filter out mobs that are busy or have no enemies
        let mobs: Vec<Mob> = self
            .population()
            .iter()
            .flat_map(|id| world.mobs.get(id))
            .filter(|m| !(m.enemies.is_empty() || m.is_busy()))
            .collect();

        // get the list of attacks from the mobs
        let actions: Vec<Action> = mobs.iter().flat_map(|m| m.fight(&mobs, dice)).collect();

        let mut updates = vec![];

        // apply the actions to the mobs
        for action in actions {
            if let Ok(mut target) = world.mobs.get(&action.to) {
                trace!("Applying action ... {:?}!", action);
                updates.append(&mut target.act(action, world));
                updates.push(Update::health(target.entity_id(), target.health()));

                // remove the dead from the population and the world
                if target.is_dead() {
                    updates.push(Update::combat(
                        target.entity_id(),
                        "You have been killed!".to_owned(),
                    ));
                    self.population.remove(target.entity_id());
                    world.mobs.remove(target.entity_id());
                } else {
                    // mob is still alive, so update the target mob
                    world.mobs.insert(target);
                }
            }
        }

        if population_count != self.population().len() {
            updates.append(&mut self.population_update(world));
        }

        updates
    }
}

#[derive(Default, Clone, Debug)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
