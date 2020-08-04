use crate::core::Identifier;
use crate::services::db::{DynamoRecord, HasPrimaryKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub identifier: Identifier,
}

impl DynamoRecord for Session {}

impl HasPrimaryKey for Session {
    fn primary_key(&self) -> String {
        self.token.to_owned()
    }
}
