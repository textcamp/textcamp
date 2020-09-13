use crate::core::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn contains(&self, name: &str) -> bool {
        for item in self.items() {
            if item.name() == name {
                return true;
            }
        }
        false
    }

    pub fn count(&self, name: &str) -> usize {
        self.items().iter().filter(|i| i.prototype == name).count()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
        self.items.sort_by_key(|a| a.name());
    }

    pub fn remove(&mut self, name: &str) -> Option<Item> {
        let gross = name.to_owned();
        match self
            .items
            .binary_search_by(|probe| probe.name().cmp(&gross))
        {
            Ok(idx) => Some(self.items.remove(idx)),
            Err(_) => None,
        }
    }
}
