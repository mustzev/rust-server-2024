use serde::{Deserialize, Serialize};

pub mod router;
mod sign_in;
mod sign_up;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
