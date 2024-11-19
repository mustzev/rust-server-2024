use mongodb::{bson::doc, Database};

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
