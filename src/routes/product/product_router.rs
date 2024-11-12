use axum;

pub fn make_product_router() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(|| async { "GET product" }))
}
