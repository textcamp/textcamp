use crate::core::Identifier;
use crate::services::db::{DynamoRecord, HasPrimaryKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub identifier: Identifier,
}

impl DynamoRecord for Account {}

impl HasPrimaryKey for Account {
    fn primary_key(&self) -> String {
        self.email.to_owned()
    }
}
