use crate::core::*;
use crate::templates::importer::*;

#[derive(Debug)]
pub struct Injector {
    space: Space,
}

impl Injector {
    pub fn new(id: Identifier) -> Self {
        let space = Space::new(&id);

        Self { space }
    }

    pub fn id(&self) -> &Identifier {
        self.space.entity_id()
    }

    pub fn inject(self, world: &mut World) {
        world.spaces.insert(self.space);
    }
}

impl From<Importer> for Injector {
    fn from(template: Importer) -> Self {
        let identifier = Identifier::from(template.space.unwrap().identifier);
        let mut injector = Injector::new(identifier);

        for (raw_direction, raw_id) in template.exits.unwrap() {
            let exit_id = Identifier::from(raw_id);
            let direction = Direction::from(&raw_direction).unwrap();
            injector.space.exits.insert(direction, exit_id);
        }

        injector.space.item_spawn = template.items.unwrap_or_default();
        injector.space.mob_spawn = template.mobs.unwrap_or_default();
        injector.space.description.text = template.description.day.clone();

        for (label, action) in template.actions.unwrap() {
            injector
                .space
                .description
                .clicks
                .insert(label.to_owned(), action.to_owned());
        }

        injector
    }
}
