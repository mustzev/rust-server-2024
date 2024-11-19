use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, Response, StatusCode},
    middleware::Next,
};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::error::internal_error;
use crate::{mongodb::schemas::users::USERS_COLLECTION_NAME, routes::auth::User};

const JWT_EXPIRE_IN_HOURS: u64 = 24;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(username: String) -> Result<String, (StatusCode, String)> {
    dotenvy::from_filename(".env").expect("Cannot load env variables");
    let jwt_secret = env::var("jwt_secret").expect("Env jwt_secret not found");

    let now = SystemTime::now();
    let expire = Duration::from_secs(JWT_EXPIRE_IN_HOURS * 60 * 60);
    let exp = (now + expire)
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let iat = now.duration_since(UNIX_EPOCH).unwrap().as_millis();

    let claims = Claims {
        exp: exp.try_into().unwrap(),
        iat: iat.try_into().unwrap(),
        username,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(internal_error)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, (StatusCode, String)> {
    dotenvy::from_filename(".env").expect("Cannot load env variables");
    let jwt_secret = env::var("jwt_secret").expect("Env jwt_secret not found");

    decode(
        &jwt,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(internal_error)
}

pub async fn authorize(
    State(db): State<Database>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, (StatusCode, String)> {
    let auth_header = req.headers().get(header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| {
            (
                StatusCode::FORBIDDEN,
                "Empty header is not allowed".to_string(),
            )
        })?,
        None => {
            return Err((
                StatusCode::FORBIDDEN,
                "Please add the JWT token to the header".to_string(),
            ))
        }
    };
    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unable to decode token".to_string(),
            ))
        }
    };

    let result = db
        .collection::<User>(USERS_COLLECTION_NAME)
        .find_one(doc! { "username": token_data.claims.username })
        .await
        .map_err(internal_error)?;
    let user = match result {
        Some(user) => user,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "You are not an authorized user".to_string(),
            ))
        }
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
