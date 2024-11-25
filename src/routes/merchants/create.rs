use axum::{http::StatusCode, Json};

use super::MerchantCreate;

pub async fn create_merchant(
    Json(input): Json<MerchantCreate>,
) -> Result<String, (StatusCode, String)> {
    Ok("test".to_string())
}
