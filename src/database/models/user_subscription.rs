use scylla::macros::{IntoUserType,FromUserType,FromRow};
use scylla::cql_to_rust::FromCqlVal;
/// Represents a channel that a user has subcribed to 
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct UserSubscribed {
    pub uid: String, // partition key
    pub subuuid: String, // clustering key
    pub channel_id: String,
}