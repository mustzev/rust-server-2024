use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::{
    serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string,
};
use mongodb::bson::DateTime;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

pub const MERCHANTS_COLLECTION_NAME: &str = "merchants";

pub async fn create_merchants_collection(db: &Database) {
    let validator = doc! {
        "$jsonSchema": doc! {
            "bsonType": "object",
            "title": "merchant object validation",
            "additionalProperties": false,
            "properties": doc! {
                "_id": doc! { "bsonType": "objectId" },
                "name": doc! { "bsonType": "string" },
                "description": doc! { "bsonType": "string" },
                "location": doc! { "bsonType": "string" },
                "createdAt": doc! { "bsonType": "date" },
                "createdBy": doc! { "bsonType": "objectId" },
                "updatedAt": doc! { "bsonType": "date" },
                "updatedBy": doc! { "bsonType": "objectId" },
                "isDeleted": doc! { "bsonType": "bool" },
                "deletedAt": doc! { "bsonType": "date" },
                "deletedBy": doc! { "bsonType": "objectId" },
            }
        }
    };
    let _ = db.create_collection(MERCHANTS_COLLECTION_NAME).await;
    let _ = db
        .run_command(doc! {
            "collMod": MERCHANTS_COLLECTION_NAME,
            "validator": validator,
            "validationAction": "error",
            "validationLevel": "moderate",
        })
        .await;
}

#[derive(Debug, Deserialize, Serialize, Clone, TsBind)]
#[serde(rename_all = "camelCase")]
#[ts_bind(rename_all = "camelCase")]
pub struct Merchant {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub location: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        // serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: Option<DateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        // serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub created_by: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<ObjectId>,
}
