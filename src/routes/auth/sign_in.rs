use axum::{extract::State, http::StatusCode, Json};
use mongodb::{results::InsertOneResult, Database};

use super::SignIn;
use crate::{
    mongodb::schemas::users::USERS_COLLECTION_NAME,
    utilities::{auth::encode_jwt, error::internal_error},
};

pub async fn sign_in(
    State(db): State<Database>,
    Json(input): Json<SignIn>,
) -> Result<Json<String>, (StatusCode, String)> {
    let token = encode_jwt(input.username).map_err(internal_error)?;

    Ok(Json(token))
}
