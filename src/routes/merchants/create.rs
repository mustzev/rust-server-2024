use axum::{extract::State, http::StatusCode, Json};

use super::MerchantCreate;
use crate::mongodb::cdb::Cdb;

pub async fn create_merchant(
    // State(cdb): State<Cdb>,
    Json(input): Json<MerchantCreate>,
) -> Result<String, (StatusCode, String)> {
    Ok("test".to_string())
}
