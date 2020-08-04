use crate::core::entities::*;
use crate::core::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Description {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
