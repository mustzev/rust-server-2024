use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::UpdateResult,
};

use crate::mongodb::{cdb::Cdb, schemas::users::User};

pub async fn delete_merchant(
    State(cdb): State<Cdb>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<UpdateResult>, (StatusCode, String)> {
    let result = cdb
        .merchants
        .delete_one(doc! { "_id": ObjectId::parse_str(id).unwrap() }, user)
        .await?;

    Ok(Json(result))
}
