use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    results::UpdateResult,
};

use super::MerchantUpdate;
use crate::mongodb::{cdb::Cdb, schemas::users::User};

pub async fn update_merchant(
    State(cdb): State<Cdb>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    Json(input): Json<MerchantUpdate>,
) -> Result<Json<UpdateResult>, (StatusCode, String)> {
    let result = cdb
        .merchants
        .update_one(
            doc! { "_id": ObjectId::parse_str(id).unwrap() },
            doc! { "$set": to_document(&input).unwrap() },
            user,
        )
        .await?;

    Ok(Json(result))
}
