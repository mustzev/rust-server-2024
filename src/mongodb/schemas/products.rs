use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::DateTime;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

use crate::utilities::serde_helpers::{
    serialize_bson_datetime_as_rfc3339_string_option, serialize_object_id_as_hex_string_option,
};

pub const PRODUCTS_COLLECTION_NAME: &str = "products";

pub async fn create_products_collection(db: &Database) {
    let validator = doc! {
        "$jsonSchema": doc! {
            "bsonType": "object",
            "title": "product object validation",
            "additionalProperties": false,
            "properties": doc! {
                "_id": doc! { "bsonType": "objectId" },
                "name": doc! { "bsonType": "string" },
                "description": doc! { "bsonType": "string" },
                "price": doc! { "bsonType": "double" },
                "quantity": doc! { "bsonType": "int" },
            }
        }
    };
    let _ = db.create_collection(PRODUCTS_COLLECTION_NAME).await;
    let _ = db
        .run_command(doc! {
            "collMod": PRODUCTS_COLLECTION_NAME,
            "validator": validator,
            "validationAction": "error",
            "validationLevel": "moderate",
        })
        .await;
}

#[derive(Debug, Deserialize, Serialize, Clone, TsBind)]
#[serde(rename_all = "camelCase")]
#[ts_bind(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub quantity: i32,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string_option"
    )]
    pub created_at: Option<DateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id_as_hex_string_option"
    )]
    pub created_by: Option<ObjectId>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string_option"
    )]
    pub updated_at: Option<DateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id_as_hex_string_option"
    )]
    pub updated_by: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string_option"
    )]
    pub deleted_at: Option<DateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id_as_hex_string_option"
    )]
    pub deleted_by: Option<ObjectId>,
}
