use axum;

use crate::{
    routes::product::product_router::make_product_router, utilities::tracing::make_trace_layer,
};

pub fn make_router() -> axum::Router {
    axum::Router::new()
        .nest("/product", make_product_router())
        .layer(make_trace_layer())
}
