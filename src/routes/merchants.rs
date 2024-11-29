use serde::{Deserialize, Serialize};

mod create;
mod delete;
mod read;
pub mod router;
mod update;

#[derive(Debug, Deserialize, Serialize)]
pub struct MerchantCreate {
    name: String,
    description: String,
    location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MerchantUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
}
