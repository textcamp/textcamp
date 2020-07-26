/// Abstractions around AWS DynamoDB. The goal here is to cloak the complexity of DynamoDB calls, and
/// facilitate the conversion between DynamoDB record sets and our local structs.
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
pub struct DynamoTable<'a> {
    pub name: &'a str,
    pub primary_key: &'a str,
}

impl<'a> DynamoTable<'a> {
    /// Returns an Option of the record for a given primary key.
    pub async fn get<T: DynamoRecord>(&self, db: &Dynamo, pk_value: &str) -> Option<T> {
        let attribute_values = db
            .client
            .get_item(self.build_get_query(pk_value))
            .await
            .ok()?
            .item?;

        let fields: Fields = attribute_values.into();

        fields.try_into().ok()
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
            item: record.into().values,
            table_name: self.name.to_owned(),
            ..Default::default()
        }
    }
}

/// Provides the value of the primary key for a given record
pub trait HasPrimaryKey {
    fn primary_key(&self) -> String;
}

/// Supertrait describing what needs to be implemented in order to store and retrieve a record from DynamoDB.
/// We can guarantee the types and structure going into Dynamo, but not what comes out -- hence `Into` and `TryFrom`
pub trait DynamoRecord: HasPrimaryKey + Into<Fields> + TryFrom<Fields> {}

/// A wrapper around the Dynamo attribute value set to make extracting data a little simpler
#[derive(Debug, Default)]
pub struct Fields {
    values: HashMap<String, AttributeValue>,
}

impl Fields {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set_string(&mut self, key: &str, value: String) {
        self.values.insert(
            key.to_owned(),
            AttributeValue {
                s: Some(value),
                ..Default::default()
            },
        );
    }

    pub fn get_string(&self, key: &str) -> Result<String, String> {
        self.values
            .get(key)
            .ok_or(format!("Missing key {}", key))?
            .s
            .clone()
            .ok_or(format!("Expected string type for {}", key))
    }
}

impl From<HashMap<String, AttributeValue>> for Fields {
    fn from(values: HashMap<String, AttributeValue>) -> Self {
        Self { values }
    }
}
