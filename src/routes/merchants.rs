use serde::{Deserialize, Serialize};

mod create;
pub mod router;

#[derive(Debug, Deserialize, Serialize)]
pub struct MerchantCreate {
    name: String,
    description: String,
    location: String,
}
