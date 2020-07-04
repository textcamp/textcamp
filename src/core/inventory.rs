use crate::core::*;

#[derive(Debug, Clone, Default)]
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
        self.items()
            .iter()
            .filter(|i| i.prototype == name)
            .count()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
        self.items.sort_by(|a, b| a.name().cmp(b.name()));
    }

    pub fn remove(&mut self, name: &str) -> Option<Item> {
        match self.items.binary_search_by(|probe| probe.name().cmp(name)) {
            Ok(idx) => Some(self.items.remove(idx)),
            Err(_) => None,
        }
    }
}
