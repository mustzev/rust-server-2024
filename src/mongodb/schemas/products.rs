use mongodb::{
    bson::doc,
    options::{ValidationAction, ValidationLevel},
    Database,
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
                "answer": doc! { "enum": vec! [ "yes", "no" ] }
            }
        }
    };
    let _result = db
        .create_collection(PRODUCTS_COLLECTION_NAME)
        .validator(validator)
        .validation_action(ValidationAction::Error)
        .validation_level(ValidationLevel::Moderate)
        .await;
}
