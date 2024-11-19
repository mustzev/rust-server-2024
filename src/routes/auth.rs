use serde::{Deserialize, Serialize};

pub mod router;
mod sign_in;
mod sign_up;

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUp {
    username: String,
    password: String,
    email: String,
    birthday: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignIn {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    password: String,
    email: String,
    birthday: String,
}
