use crate::core::entities::*;
use crate::core::*;

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Item {
    pub identifier: Identifier,
    pub prototype: String,
    pub name: String,
    pub description: Description,
}

impl Item {
    pub fn new() -> Self {
        Self {
            identifier: Identifier::random(),
            prototype: String::new(),
            name: String::new(),
            description: Description::default(),
        }
    }
}

impl Named for Item {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Describe for Item {
    fn describe(&self, _world: &World) -> Markup {
        Markup {
            text: self.description.text.clone(),
            clicks: self.description.clicks.clone(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
