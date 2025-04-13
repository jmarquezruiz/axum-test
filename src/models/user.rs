use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id_mongo: Option<bson::oid::ObjectId>,
    pub username: String,
}
