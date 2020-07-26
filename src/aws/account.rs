use crate::aws::dynamo::{DynamoRecord, DynamoTable, Fields, HasPrimaryKey};
use crate::core::Identifier;
use std::convert::TryFrom;

pub const ACCOUNTS_TABLE: DynamoTable<'static> = DynamoTable {
    name: "Accounts",
    primary_key: "Email",
};

#[derive(Debug)]
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

impl TryFrom<Fields> for Account {
    type Error = String;

    fn try_from(values: Fields) -> Result<Self, Self::Error> {
        let email = values.get_string("Email")?;
        let identifier_value = values.get_string("Identifier")?;
        let identifier = Identifier::from(identifier_value);

        Ok(Account { email, identifier })
    }
}

impl Into<Fields> for Account {
    fn into(self) -> Fields {
        let mut fields = Fields::new();

        fields.set_string("Email", self.email.to_owned());
        fields.set_string("Identifier", self.identifier.into());

        fields
    }
}
