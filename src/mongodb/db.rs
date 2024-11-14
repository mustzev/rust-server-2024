use mongodb::{bson::doc, Client, Database};

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
    db
}
