use scylla::macros::{IntoUserType,FromUserType, FromRow};
use scylla::cql_to_rust::FromCqlVal;

/// Represents a user queried from the database
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct DBUser {
    pub uuid: String, // partition key
    pub name: String, // clustering key
    pub password: String,
    pub token: String,
    pub feed_needs_update: bool,
}

impl DBUser {
    pub fn is_authenticated(&self) -> bool {
        self.name.is_empty()
    }
}