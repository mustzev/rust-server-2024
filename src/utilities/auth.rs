use axum::{
    body::Body,
    extract::{Json, Request},
    http::{header, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::error::internal_error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
}

pub struct AuthError {
    status_code: StatusCode,
    message: String,
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json! ({
            "error": self.message
        }));
        (self.status_code, body).into_response()
    }
}

pub fn encode_jwt(username: String) -> Result<String, (StatusCode, String)> {
    dotenvy::from_filename(".env").expect("Cannot load env variables");
    let jwt_secret = env::var("jwt_secret").expect("Env jwt_secret not found");

    let now = SystemTime::now();
    let expire = Duration::from_secs(24 * 60 * 60);
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

pub async fn authorize(mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers().get(header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            status_code: StatusCode::FORBIDDEN,
            message: "Empty header is not allowed".to_string(),
        })?,
        None => {
            return Err(AuthError {
                status_code: StatusCode::FORBIDDEN,
                message: "Please add the JWT token to the header".to_string(),
            })
        }
    };
    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                status_code: StatusCode::UNAUTHORIZED,
                message: "Unable to decode token".to_string(),
            })
        }
    };
    req.extensions_mut().insert(token_data.claims.username);

    Ok(next.run(req).await)
}
