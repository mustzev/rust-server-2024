use axum::{routing::post, Router};
use mongodb::Database;

use super::{sign_in::sign_in, sign_up::sign_up};

pub fn make_auth_router(mongodb_db: &Database) -> Router {
    Router::new()
        .route("/sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
        .with_state(mongodb_db)
}
