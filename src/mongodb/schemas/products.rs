use mongodb::{bson::doc, Database};

pub const PRODUCTS_COLLECTION_NAME: &str = "products";

pub fn create_products_collection(db: Database) {
    let validator = doc! {
        "$jsonSchema": doc! {
           "bsonType": "object",
           "title": "Answer Value Validation",
           "properties": doc! {
              "answer": doc! {
                 "enum": vec! [ "yes", "no" ],
              }
           }
        }
    };
    let _products = db
        .create_collection("products")
        .validator(validator)
        .validation_action(mongodb::options::ValidationAction::Error)
        .validation_level(mongodb::options::ValidationLevel::Moderate);
}
