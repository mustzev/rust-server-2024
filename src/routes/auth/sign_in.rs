use axum::{extract::State, http::StatusCode, Json};
use mongodb::{bson::doc, Database};

use super::{SignIn, User};
use crate::{
    mongodb::schemas::users::USERS_COLLECTION_NAME,
    utilities::{
        auth::{encode_jwt, verify_password},
        error::internal_error,
    },
};

pub async fn sign_in(
    State(db): State<Database>,
    Json(input): Json<SignIn>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = db
        .collection::<User>(USERS_COLLECTION_NAME)
        .find_one(doc! { "username": &input.username })
        .await
        .map_err(internal_error)?;

    let user = match result {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "User not found".to_string())),
    };

    if !verify_password(&input.password, &user.password).map_err(internal_error)? {
        return Err((StatusCode::UNAUTHORIZED, "Password incorrect".to_string()));
    }

    let token = encode_jwt(input.username)?;

    Ok(Json(token))
}
