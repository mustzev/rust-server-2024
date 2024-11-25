use mongodb::{bson::doc, Client, Database};

use super::schemas::products::create_products_collection;
use crate::mongodb::schemas::{
    merchants::create_merchants_collection, users::create_users_collection,
};

const MONGODB_DATABASE: &str = "rust-server-2024";

pub async fn init_mongodb_client(mongodb_uri: String) -> Database {
    let client = Client::with_uri_str(mongodb_uri).await.unwrap();
    client
        .database(MONGODB_DATABASE)
        .run_command(doc! { "ping": 1 })
        .await
        .unwrap();
    println!("Pinged your database. Successfully connected to MongoDB!");
    let db = client.database(MONGODB_DATABASE);
    create_products_collection(&db).await;
    create_users_collection(&db).await;
    create_merchants_collection(&db).await;
    db
}
