mod mongodb;
mod routes;
mod utilities;

use crate::{
    mongodb::db::init_mongodb_client, routes::router::make_router,
    utilities::tracing::init_tracing_subscriber,
};

use axum;
use dotenvy;
use std::env;
use tokio::net::TcpListener;
use tracing;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").expect("Cannot load env variables");

    let server_address = env::var("server_address").expect("Env server_address not found");
    let mongodb_uri = env::var("mongodb_uri").expect("Env mongodb_uri not found");

    let mongodb_db = init_mongodb_client(mongodb_uri).await;

    init_tracing_subscriber();

    let listener = TcpListener::bind(server_address).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let app = make_router(mongodb_db);

    axum::serve(listener, app).await.unwrap();
}
