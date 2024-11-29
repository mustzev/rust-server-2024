use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use mongodb::Database;

use super::{
    create::create_merchant, delete::delete_merchant, read::read_merchant, update::update_merchant,
};
use crate::{mongodb::cdb::make_cdb, utilities::auth::authorize};

pub fn make_merchant_router(db: Database) -> Router {
    let cdb = make_cdb(db.clone());
    Router::new()
        .route("/:id", get(read_merchant))
        .route("/", post(create_merchant))
        .route("/:id", put(update_merchant))
        .route("/:id", delete(delete_merchant))
        .layer(middleware::from_fn_with_state(db, authorize))
        .with_state(cdb)
}
