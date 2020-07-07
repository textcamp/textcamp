use crate::core::*;
use crate::templates::Importer;

#[derive(Debug, Default)]
pub struct Injector {
    pub prototype: MobPrototype,
}

impl Injector {
    pub fn inject(self, world: &mut World) {
        world.mob_prototypes.add(self.prototype);
    }
}

impl From<Importer> for Injector {
    fn from(template: Importer) -> Self {
        let mut injector = Injector::default();
        let t_item = template.mob.unwrap();

        if let Some(name) = t_item.name {
            injector.prototype.name = name;
        }

        injector.prototype.prototype_name = t_item.identifier;
        injector.prototype.description.text = template.description.day.clone();

        if let Some(attrs) = template.attributes {
            injector.prototype.strength = attrs.strength;
            injector.prototype.dexterity = attrs.dexterity;
            injector.prototype.constitution = attrs.constitution;
            injector.prototype.intelligence = attrs.intelligence;
            injector.prototype.wisdom = attrs.wisdom;
            injector.prototype.charisma = attrs.charisma;

            // default health = constitution
            injector.prototype.hp = attrs.constitution;
        }

        injector
    }
}
