use axum::{extract::State, http::StatusCode, Json};
use mongodb::{results::InsertOneResult, Database};

use super::ProductCreate;
use crate::{
    mongodb::schemas::products::PRODUCTS_COLLECTION_NAME, utilities::error::internal_error,
};

pub async fn create_product(
    State(db): State<Database>,
    Json(input): Json<ProductCreate>,
) -> Result<Json<InsertOneResult>, (StatusCode, String)> {
    let result = db
        .collection(PRODUCTS_COLLECTION_NAME)
        .insert_one(input)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}
