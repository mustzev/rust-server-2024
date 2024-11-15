use super::Product;
use crate::{
    mongodb::schemas::products::PRODUCTS_COLLECTION_NAME, utilities::internal_error::internal_error,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

pub async fn read_product(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<Option<Product>>, (StatusCode, String)> {
    let result = db
        .collection::<Product>(PRODUCTS_COLLECTION_NAME)
        .find_one(doc! { "_id": ObjectId::parse_str(id).unwrap() })
        .await
        .map_err(internal_error)?;

    match &result {
        Some(product) => {
            println!("id: {}", product.id)
        }
        None => {}
    }

    Ok(Json(result))
}
