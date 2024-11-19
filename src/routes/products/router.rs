use super::{
    create::create_product, delete::delete_product, read::read_product, update::update_product,
};
use crate::utilities::auth::authorize;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use mongodb::Database;

pub fn make_product_router(db: Database) -> Router {
    Router::new()
        .route("/:id", get(read_product))
        .route("/", post(create_product))
        .route("/:id", put(update_product))
        .route("/:id", delete(delete_product))
        .layer(middleware::from_fn(authorize))
        .with_state(db)
}
