use log::{info, trace};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Kind {
    Space,
    Mob,
    Item,
}

// TODO: Rename Importer to Template
#[derive(Debug, Deserialize)]
pub struct Importer {
    pub item: Option<Meta>,
    pub space: Option<Meta>,
    pub mob: Option<Meta>,
    pub description: Description,
    pub actions: Option<HashMap<String, String>>,
    pub exits: Option<HashMap<String, String>>,
    pub items: Option<Vec<crate::core::Spawn>>,
    pub mobs: Option<Vec<crate::core::Spawn>>,
    pub attributes: Option<Attributes>,
}

impl Importer {
    pub fn from_file(path: &PathBuf) -> Self {
        let data = std::fs::read_to_string(path).expect("Failed to read");
        toml::from_str(&data).expect("Failed to parse")
    }

    pub fn load_dir(path: &str) -> Vec<Self> {
        let mut output = vec![];
        let pattern = format!("{}/**/*.toml", path);

        glob::glob(&pattern)
            .unwrap_or_else(|_| panic!("Failed to read pattern: {}", pattern))
            .for_each(|entry| {
                let path = entry.expect("Failed to open");
                info!("Loading {:?} ... ", path);
                let importer = Importer::from_file(&path);
                trace!("=> {:?}", importer);
                output.push(importer);
            });

        output
    }

    pub fn is_a(&self) -> Kind {
        if self.space.is_some() {
            Kind::Space
        } else if self.mob.is_some() {
            Kind::Mob
        } else {
            Kind::Item
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub identifier: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Description {
    pub day: String,
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
    pub strength: usize,
    pub dexterity: usize,
    pub constitution: usize,
    pub intelligence: usize,
    pub wisdom: usize,
    pub charisma: usize,
}

// TODO: mob 'attacks'
