use axum::{middleware, routing::post, Router};
use mongodb::Database;

use super::create::create_merchant;
use crate::{mongodb::cdb::make_cdb, utilities::auth::authorize};

pub fn make_merchant_router(db: Database) -> Router {
    let cdb = make_cdb(db.clone());
    Router::new()
        .route("/", post(create_merchant))
        .layer(middleware::from_fn_with_state(db, authorize))
        .with_state(cdb)
}
