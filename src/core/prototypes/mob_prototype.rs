use super::Prototyped;
use crate::core::entities::mob::{Description, Mob};
use crate::core::*;

#[derive(Debug, Default)]
pub struct MobPrototype {
    pub prototype_name: String,
    pub name: String,
    pub space_id: Identifier,
    pub description: Description,
    pub hp: usize,
    pub attributes: Attributes,
}

impl MobPrototype {
    fn unique_name(&self) -> String {
        format!("{}{}", &self.name, rand::thread_rng().gen::<u8>())
    }
}

impl Prototyped for MobPrototype {
    type Item = Mob;

    fn create(&self) -> Self::Item {
        let mut output = Mob::new();

        output.prototype = self.prototype_name();
        output.name = self.unique_name();
        output.space_id = self.space_id.clone();
        output.description = self.description.clone();
        output.attributes = self.attributes;
        output.hp = self.hp;
        output
    }

    fn prototype_name(&self) -> String {
        self.prototype_name.to_owned()
    }
}
