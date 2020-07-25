use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
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
            client: DynamoDbClient::new(Region::UsWest2),
        }
    }
}

/// Describes the attributes of a Dynamo collection: the name of the table, and the name of the primary key
#[derive(Debug)]
pub struct DynamoTable {
    name: String,
    primary_key: String,
}

impl DynamoTable {
    /// Returns an Option of the record for a given primary key.
    pub async fn get<T: DynamoRecord>(&self, db: &Dynamo, pk_value: &str) -> Option<T> {
        db.client
            .get_item(self.build_get_query(pk_value))
            .await
            .ok()?
            .item?
            .try_into()
            .ok()
    }

    /// Inserts a record into the table, relying on the PrimaryKey trait to determine the value of the primary key.
    pub async fn put<T: DynamoRecord>(&self, db: &Dynamo, record: T) -> Result<(), String> {
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
        GetItemInput {
            key,
            table_name: self.name.to_owned(),
            ..Default::default()
        }
    }

    fn build_put_query<T: DynamoRecord>(&self, record: T) -> PutItemInput {
        PutItemInput {
            item: record.into(),
            table_name: self.name.to_owned(),
            ..Default::default()
        }
    }
}

pub trait HasPrimaryKey {
    fn primary_key(&self) -> String;
}

/// Supertrait describing what needs to be implemented in order to store and retrieve a record from DynamoDB.
/// We can guarantee the types and structure going into Dynamo, but not what comes out -- hence `Into` and `TryFrom`
pub trait DynamoRecord:
    HasPrimaryKey + Into<HashMap<String, AttributeValue>> + TryFrom<HashMap<String, AttributeValue>>
{
}
