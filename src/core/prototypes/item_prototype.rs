use super::Prototyped;
use crate::core::item::{Description, Item};

#[derive(Debug, Default)]
pub struct ItemPrototype {
    pub prototype_name: String,
    pub name: String,
    pub description: Description,
}

impl Prototyped for ItemPrototype {
    type Item = Item; // gross, I know

    fn create(&self) -> Self::Item {
        let mut output = Item::new();
        output.prototype = self.prototype_name.clone();
        output.name = self.name.clone();
        output.description = self.description.clone();

        output
    }

    fn name(&self) -> &str {
        &self.prototype_name
    }
}

impl ItemPrototype {
    pub fn new() -> Self {
        Self {
            prototype_name: String::new(),
            name: String::new(),
            description: Description::default(),
        }
    }
}
