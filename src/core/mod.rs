/// Provides the authentication framework
pub mod authentication;

/// In game time calculations
pub mod clock;

/// Random number generation using the dice model
pub mod dice;

/// Entities are tickable, describable, and stateful
pub mod entities;
pub mod errors;

/// Manages items for a Space or Mob
pub mod inventory;
pub mod item;
pub mod markup;

/// Parser for input text from players
pub mod phrase;

/// Represents the Mobs in a given Space
pub mod population;
pub mod prototypes;
pub mod spawn;

/// Update messages that are sent to the client
pub mod update;

/// Universal, shared game state
pub mod world;

pub use authentication::Authentication;
pub use clock::{Clock, DateTime, Transition};
pub use dice::Dice;
pub use errors::TCError;
pub use inventory::Inventory;
pub use item::Item;
pub use markup::Markup;
pub use phrase::Phrase;
pub use population::Population;
pub use prototypes::{ItemPrototype, MobPrototype, Prototyped, Prototypes};
pub use spawn::Spawn;
pub use update::Update;
pub use world::{Command, World};

pub use entities::*;

use log::trace;
use rand::Rng;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd, Default)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn origin() -> Self {
        Self {
            value: "ORIGIN".to_owned(),
        }
    }

    pub fn random() -> Self {
        let rand_string: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .collect();

        let output = Self { value: rand_string };
        trace!("Created {:?}", output);

        output
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.value
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

struct IdentifierVisitor;

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Identifier, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(IdentifierVisitor)
    }
}

impl<'de> Visitor<'de> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let identifier = Identifier::from(value.to_owned());
        Ok(identifier)
    }
}
