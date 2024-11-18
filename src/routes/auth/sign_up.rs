use axum::{extract::State, http::StatusCode, Json};
use mongodb::{results::InsertOneResult, Database};

use crate::{mongodb::schemas::users::USERS_COLLECTION_NAME, utilities::error::internal_error};

use super::SignUp;

pub async fn sign_up(
    State(db): State<Database>,
    Json(input): Json<SignUp>,
) -> Result<Json<InsertOneResult>, (StatusCode, String)> {
    let result = db
        .collection(USERS_COLLECTION_NAME)
        .insert_one(input)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}
