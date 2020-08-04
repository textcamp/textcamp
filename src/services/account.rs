use crate::core::Identifier;
use crate::services::db::{DynamoRecord, DynamoTable, HasPrimaryKey};
use serde::{Deserialize, Serialize};

pub const ACCOUNTS_TABLE: DynamoTable<'static> = DynamoTable {
    name: "Accounts",
    primary_key: "Email",
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    email: String,
    identifier: Identifier,
}

impl DynamoRecord for Account {}

impl HasPrimaryKey for Account {
    fn primary_key(&self) -> String {
        self.email.to_owned()
    }
}
