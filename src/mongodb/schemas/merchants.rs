use mongodb::{bson::doc, Database};

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
