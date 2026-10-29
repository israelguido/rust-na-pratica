use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}