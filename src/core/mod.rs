pub mod clock;
pub mod dice;
pub mod direction;
pub mod entities;
pub mod errors;
pub mod inventory;
pub mod item;
pub mod markup;
pub mod phrase;
pub mod population;
pub mod prototypes;
pub mod spawn;
pub mod update;
pub mod world;

pub use clock::{Clock, DateTime};
pub use dice::Dice;
pub use direction::Direction;
pub use errors::TCError;
pub use inventory::Inventory;
pub use item::Item;
pub use markup::Markup;
pub use phrase::Phrase;
pub use population::Population;
pub use prototypes::{ItemPrototype, MobPrototype, Prototyped, Prototypes};
pub use spawn::Spawn;
pub use update::Update;
pub use world::{Action, World};

pub use entities::*;

use log::trace;
use rand::Rng;
use serde::Serialize;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    value: String,
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

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}