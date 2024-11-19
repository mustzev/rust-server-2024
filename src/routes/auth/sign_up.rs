use axum::{extract::State, http::StatusCode, Json};
use mongodb::Database;

use super::SignUp;
use crate::{
    mongodb::schemas::users::USERS_COLLECTION_NAME,
    utilities::{
        auth::{encode_jwt, hash_password},
        error::internal_error,
    },
};

pub async fn sign_up(
    State(db): State<Database>,
    Json(mut input): Json<SignUp>,
) -> Result<Json<String>, (StatusCode, String)> {
    let password_hash = hash_password(&input.password).map_err(internal_error)?;
    input.password = password_hash;

    let _result = db
        .collection::<SignUp>(USERS_COLLECTION_NAME)
        .insert_one(&input)
        .await
        .map_err(internal_error)?;

    let token = encode_jwt(input.username)?;

    Ok(Json(token))
}
