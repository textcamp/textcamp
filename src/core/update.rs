use crate::core::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Update {
    pub to: Identifier,
    pub message: Wrapper,
}

impl Update {
    fn new(to: &Identifier, message: Wrapper) -> Self {
        Self {
            to: to.clone(),
            message,
        }
    }

    pub fn exits(to: &Identifier, raw_exits: &HashMap<Direction, Identifier>) -> Self {
        let exits = raw_exits.keys().cloned().collect();
        let wrapper = Wrapper::Exits(exits);
        Update::new(to, wrapper)
    }

    pub fn population(to: &Identifier, message: Vec<String>) -> Self {
        let wrapper = Wrapper::Population(message);
        Update::new(to, wrapper)
    }

    pub fn info(to: &Identifier, message: &str) -> Self {
        let wrapper = Wrapper::Info(message.to_owned());
        Update::new(to, wrapper)
    }

    pub fn error(to: &Identifier, message: &str) -> Self {
        let wrapper = Wrapper::Error(message.to_owned());
        Update::new(to, wrapper)
    }

    pub fn combat(to: &Identifier, message: String) -> Self {
        let wrapper = Wrapper::Combat(message);
        Update::new(to, wrapper)
    }

    pub fn space(to: &Identifier, content: Markup) -> Self {
        let wrapper = Wrapper::Space(content);
        Update::new(to, wrapper)
    }

    pub fn character(to: &Identifier, content: Markup) -> Self {
        let wrapper = Wrapper::Character(content);
        Update::new(to, wrapper)
    }

    pub fn item(to: &Identifier, content: Markup) -> Self {
        let wrapper = Wrapper::Item(content);
        Update::new(to, wrapper)
    }

    pub fn time(to: &Identifier, date_time: &DateTime) -> Self {
        let wrapper = Wrapper::Time(date_time.clone());
        Update::new(to, wrapper)
    }

    pub fn inventory(to: &Identifier, inventory: &Inventory) -> Self {
        let items = inventory
            .items()
            .iter()
            .map(|i| i.name().to_owned())
            .collect();
        let wrapper = Wrapper::Inventory(items);
        Update::new(to, wrapper)
    }

    pub fn health(to: &Identifier, pct: usize) -> Self {
        let wrapper = Wrapper::Health(pct);
        Update::new(to, wrapper)
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Wrapper {
    Info(String),
    Error(String),
    Extra(String),
    Combat(String),
    Exits(Vec<Direction>),
    Space(Markup),
    Character(Markup),
    Item(Markup),
    Population(Vec<String>),
    Time(DateTime),
    Inventory(Vec<String>),
    Health(usize),
}
