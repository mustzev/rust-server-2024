mod routes;

use crate::routes::router::make_router;
use axum;
use dotenvy;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").expect("Cannot load env variables");

    let server_address = env::var("server_address").unwrap();
    let mongo_uri = env::var("mongo_uri").unwrap();

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

    let app = make_router();

    axum::serve(listener, app).await.unwrap();
}
