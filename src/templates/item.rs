use crate::core::*;
use crate::templates::Importer;

#[derive(Debug, Default)]
pub struct Injector {
    pub prototype: ItemPrototype,
}

impl Injector {
    pub fn inject(self, world: &mut World) {
        world.item_prototypes.add(self.prototype);
    }
}

impl From<Importer> for Injector {
    fn from(template: Importer) -> Self {
        let mut injector = Injector::default();
        let t_item = template.item.unwrap();

        if let Some(name) = t_item.name {
            injector.prototype.name = name;
        }

        injector.prototype.prototype_name = t_item.identifier;
        injector.prototype.description.text = template.description.day;

        injector
    }
}
