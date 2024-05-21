use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWToken{
    access_token: String
}
impl JWToken{
    pub fn new(token: String) -> Self{
        Self { access_token: token }
    }
}