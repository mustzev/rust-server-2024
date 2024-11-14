use axum::http::StatusCode;
use std::error::Error;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
