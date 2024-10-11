use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub account_id: String,
    pub exp: usize,
    pub nbf: usize,
}
