use super::products::router::make_product_router;
use crate::utilities::{cors::make_cors_layer, tracing::make_trace_layer};

use axum::Router;
use mongodb::Database;

pub fn make_router(mongodb_db: Database) -> Router {
    Router::new()
        .nest("/products", make_product_router(mongodb_db))
        .layer(make_trace_layer())
        .layer(make_cors_layer())
}
