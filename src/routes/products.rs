mod create;
mod delete;
mod read;
pub mod router;
mod update;

use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCreate {
    name: String,
    description: String,
    price: f32,
    quantity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    name: String,
    description: String,
    price: f32,
    quantity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<i32>,
}
