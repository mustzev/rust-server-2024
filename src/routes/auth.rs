use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

pub mod router;
mod sign_in;
mod sign_up;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignUp {
    username: String,
    password: String,
    email: String,
    birthday: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignIn {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, TsBind)]
pub struct User {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    username: String,
    password: String,
    email: String,
    birthday: String,
}
