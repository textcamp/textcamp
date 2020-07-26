use crate::aws::dynamo::{DynamoRecord, DynamoTable, Fields, HasPrimaryKey};
use crate::core::Identifier;
use std::convert::TryFrom;

pub const ACCOUNTS_TABLE: DynamoTable<'static> = DynamoTable {
    name: "Accounts",
    primary_key: "Email",
};

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::dynamo::*;

    // TODO: Move testing into aws::dynamo
    #[test]
    fn put_get_accounts() {
        let test_accounts_table = DynamoTable {
            name: "TestAccounts",
            primary_key: "Email",
        };

        let db = Dynamo::new();

        let put_account = Account {
            email: "test@text.camp".to_owned(),
            identifier: Identifier::random(),
        };

        let result = tokio_test::block_on(test_accounts_table.put(&db, put_account.clone()));
        assert!(result.is_ok());

        let result =
            tokio_test::block_on(test_accounts_table.get::<Account>(&db, &put_account.email));
        assert!(result.is_some());

        if let Some(get_account) = result {
            assert_eq!(get_account.email, put_account.email);
            assert_eq!(get_account.identifier, put_account.identifier);
        }
    }
}
