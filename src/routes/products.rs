mod create;
mod read;
pub mod router;

use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductInput {
    name: String,
    description: String,
    price: f32,
    quantity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    #[serde(flatten)]
    input: ProductInput,
}
