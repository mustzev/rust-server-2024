use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::DateTime;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

pub const USERS_COLLECTION_NAME: &str = "users";

pub async fn create_users_collection(db: &Database) {
    let validator = doc! {
        "$jsonSchema": doc! {
            "bsonType": "object",
            "title": "user object validation",
            "additionalProperties": false,
            "properties": doc! {
                "_id": doc! { "bsonType": "objectId" },
                "username": doc! { "bsonType": "string" },
                "password": doc! { "bsonType": "string" },
                "email": doc! { "bsonType": "string" },
                "birthday": doc! { "bsonType": "string" },
            }
        }
    };
    let _ = db.create_collection(USERS_COLLECTION_NAME).await;
    let _ = db
        .run_command(doc! {
            "collMod": USERS_COLLECTION_NAME,
            "validator": validator,
            "validationAction": "error",
            "validationLevel": "moderate",
        })
        .await;
}

#[derive(Debug, Deserialize, Serialize, Clone, TsBind)]
#[serde(rename_all = "camelCase")]
#[ts_bind(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub birthday: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
