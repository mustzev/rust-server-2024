use super::{
    create::create_product, delete::delete_product, read::read_product, update::update_product,
};

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use mongodb::Database;

pub fn make_product_router(mongodb_db: &Database) -> Router {
    Router::new()
        .route("/:id", get(read_product))
        .route("/", post(create_product))
        .route("/:id", put(update_product))
        .route("/:id", delete(delete_product))
        .with_state(mongodb_db)
}
