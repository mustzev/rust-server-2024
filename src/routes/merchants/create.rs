use axum::{extract::State, http::StatusCode, Extension, Json};
use mongodb::{bson::to_document, results::InsertOneResult};

use super::MerchantCreate;
use crate::mongodb::{cdb::Cdb, schemas::users::User};

pub async fn create_merchant(
    State(cdb): State<Cdb>,
    Extension(user): Extension<User>,
    Json(input): Json<MerchantCreate>,
) -> Result<Json<InsertOneResult>, (StatusCode, String)> {
    let result = cdb
        .merchants
        .insert_one(to_document(&input).unwrap(), user)
        .await?;

    Ok(Json(result))
}
