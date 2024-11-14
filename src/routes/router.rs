use crate::{
    routes::product::product_router::make_product_router, utilities::tracing::make_trace_layer,
};
use mongodb::Database;

pub fn make_router(mongodb_db: Database) -> axum::Router {
    axum::Router::new()
        .nest("/product", make_product_router(mongodb_db))
        .layer(make_trace_layer())
}
