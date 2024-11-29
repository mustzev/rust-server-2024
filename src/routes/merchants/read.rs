use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use mongodb::bson::{doc, oid::ObjectId};

use crate::mongodb::{
    cdb::Cdb,
    schemas::{merchants::Merchant, users::User},
};

pub async fn read_merchant(
    State(cdb): State<Cdb>,
    Extension(_user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<Option<Merchant>>, (StatusCode, String)> {
    let result = cdb
        .merchants
        .find_one(doc! { "_id": ObjectId::parse_str(id).unwrap() })
        .await?;

    Ok(Json(result))
}
