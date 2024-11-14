use super::{create::create_product, read::read_product};

use axum::{
    routing::{get, post},
    Router,
};
use mongodb::Database;

pub fn make_product_router(mongodb_db: Database) -> Router {
    Router::new()
        .route("/:id", get(read_product))
        .route("/", post(create_product))
        .with_state(mongodb_db)
}
