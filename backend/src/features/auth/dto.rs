use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}
