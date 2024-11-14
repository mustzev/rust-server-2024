use axum::{extract::State, routing, Router};
use mongodb::Database;

pub fn make_product_router(mongodb_db: Database) -> Router {
    Router::new()
        .route("/", routing::get(read_member))
        .with_state(mongodb_db)
}

async fn read_member(State(db): State<Database>) -> String {
    "test".to_string()
}
