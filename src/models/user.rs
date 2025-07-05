use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}
