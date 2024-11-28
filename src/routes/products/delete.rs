use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::DeleteResult,
    Database,
};

use crate::{
    mongodb::schemas::products::{Product, PRODUCTS_COLLECTION_NAME},
    utilities::error::internal_error,
};

pub async fn delete_product(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<DeleteResult>, (StatusCode, String)> {
    let result = db
        .collection::<Product>(PRODUCTS_COLLECTION_NAME)
        .delete_one(doc! { "_id": ObjectId::parse_str(id).unwrap() })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}
