use mongodb::{bson::doc, Database};

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
                "quantity": doc! { "bsonType": "double" },
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
