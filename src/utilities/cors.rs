use tower_http::cors::{Any, CorsLayer};

pub fn make_cors_layer() -> CorsLayer {
    CorsLayer::new().allow_origin(Any).allow_methods(Any)
}
