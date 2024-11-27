use axum::{extract::State, http::StatusCode, Extension, Json};
use mongodb::bson::to_document;

use super::MerchantCreate;
use crate::{mongodb::cdb::Cdb, routes::auth::User};

pub async fn create_merchant(
    State(cdb): State<Cdb>,
    Extension(user): Extension<User>,
    Json(input): Json<MerchantCreate>,
) -> Result<String, (StatusCode, String)> {
    let result = cdb.merchants.insert_one(to_document(&input).unwrap(), user);
    Ok("test".to_string())
}
