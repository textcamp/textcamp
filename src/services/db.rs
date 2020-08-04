use log::{trace, warn};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;
use std::fmt;

/// Maintains the connection information required to interact with Dynamo
pub struct Dynamo {
    client: DynamoDbClient,
}

impl fmt::Debug for Dynamo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dynamo")
            .field("client", &"rusoto_dynamodb::DynamoDbClient".to_owned())
            .finish()
    }
}

impl Default for Dynamo {
    fn default() -> Self {
        Self::new()
    }
}

impl Dynamo {
    pub fn new() -> Self {
        Self {
            client: DynamoDbClient::new(Region::default()),
        }
    }
}

/// Describes the attributes of a Dynamo collection: the name of the table, and the name of the primary key
#[derive(Debug)]
pub struct DynamoTable<'a> {
    pub name: &'a str,
    pub primary_key: &'a str,
}

impl<'a> DynamoTable<'a> {
    /// Returns an Option of the record for a given primary key.
    pub async fn get<T: DynamoRecord>(&self, db: &Dynamo, pk_value: &str) -> Option<T> {
        trace!("DynamoTable get: {:?}", pk_value);

        if !super::service_credentials() {
            warn!("DynamoTable get: no service credentials!");
            return None;
        };

        db.client
            .get_item(self.build_get_query(pk_value))
            .await
            .ok()?
            .item
            .map(|i| serde_dynamodb::from_hashmap(i).unwrap())
    }

    /// Inserts a record into the table, relying on the PrimaryKey trait to determine the value of the primary key.
    pub async fn put<T: DynamoRecord>(&self, db: &Dynamo, record: &T) -> Result<(), String> {
        trace!("DynamoTable put: {:?}", record);

        if !super::service_credentials() {
            warn!("DynamoTable put: no service credentials!");
            return Err("Missing service credentials".to_owned());
        };

        db.client
            .put_item(self.build_put_query(record))
            .await
            .map_err(|e| format!("Error inserting into {}: {}", self.name, e))
            .map(|_| {})
    }

    fn build_get_query(&self, pk_value: &str) -> GetItemInput {
        let pk = AttributeValue {
            s: Some(pk_value.to_owned()),
            ..Default::default()
        };
        let mut key = HashMap::new();
        key.insert(self.primary_key.to_owned(), pk);
        trace!("DynamoTable build_get_query key: {:?}", key);
        GetItemInput {
            key,
            table_name: self.name.to_owned(),
            ..Default::default()
        }
    }

    fn build_put_query<T: DynamoRecord>(&self, record: &T) -> PutItemInput {
        PutItemInput {
            item: serde_dynamodb::to_hashmap(record).unwrap(),
            table_name: self.name.to_owned(),
            ..Default::default()
        }
    }
}

/// Provides the value of the primary key for a given record
pub trait HasPrimaryKey {
    fn primary_key(&self) -> String;
}

/// Supertrait describing what needs to be implemented in order to store and retrieve a
/// record from DynamoDB.
pub trait DynamoRecord: HasPrimaryKey + Serialize + DeserializeOwned + std::fmt::Debug {}

#[cfg(test)]
mod tests {
    use super::{Dynamo, DynamoRecord, DynamoTable, HasPrimaryKey};
    use crate::core::Identifier;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestRecord {
        identifier: Identifier,
        name: String,
    }

    impl DynamoRecord for TestRecord {}

    impl HasPrimaryKey for TestRecord {
        fn primary_key(&self) -> String {
            self.identifier.clone().into()
        }
    }

    #[test]
    fn put_get_records() {
        if !crate::services::service_credentials() {
            println!("AWS Credentials not configured, skipping test.");
            return;
        };

        let test_table = DynamoTable {
            name: "TestRecords",
            primary_key: "identifier",
        };

        let db = Dynamo::new();

        let put_record = TestRecord {
            name: "test record".to_owned(),
            identifier: Identifier::random(),
        };

        let result = tokio_test::block_on(test_table.put(&db, &put_record));
        assert!(result.is_ok());

        let result =
            tokio_test::block_on(test_table.get::<TestRecord>(&db, &put_record.primary_key()));
        assert!(result.is_some());

        if let Some(get_record) = result {
            assert_eq!(get_record.name, put_record.name);
            assert_eq!(get_record.identifier, put_record.identifier);
        }
    }
}
