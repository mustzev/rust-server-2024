use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    results::UpdateResult,
    Database,
};

use super::{Product, ProductUpdate};
use crate::{
    mongodb::schemas::products::PRODUCTS_COLLECTION_NAME, utilities::error::internal_error,
};

pub async fn update_product(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(input): Json<ProductUpdate>,
) -> Result<Json<UpdateResult>, (StatusCode, String)> {
    let result = db
        .collection::<Product>(PRODUCTS_COLLECTION_NAME)
        .update_one(
            doc! { "_id": ObjectId::parse_str(id).unwrap() },
            doc! { "$set": to_document(&input).unwrap() },
        )
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}
