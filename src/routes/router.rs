use super::{
    auth::router::make_auth_router, merchants::router::make_merchant_router,
    products::router::make_product_router,
};
use crate::utilities::{cors::make_cors_layer, tracing::make_trace_layer};

use axum::Router;
use mongodb::Database;

pub fn make_router(db: Database) -> Router {
    Router::new()
        .nest("/auth", make_auth_router(db.clone()))
        .nest("/products", make_product_router(db.clone()))
        .nest("/merchants", make_merchant_router(db))
        .layer(make_trace_layer())
        .layer(make_cors_layer())
}
