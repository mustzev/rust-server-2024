mod routes;
mod utilities;

use crate::{routes::router::make_router, utilities::tracing::init_tracing_subscriber};
use axum;
use dotenvy;
use std::env;
use tokio;
use tracing;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").expect("Cannot load env variables");

    let server_address = env::var("server_address").unwrap();
    let mongo_uri = env::var("mongo_uri").unwrap();

    init_tracing_subscriber();

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let app = make_router();

    axum::serve(listener, app).await.unwrap();
}
