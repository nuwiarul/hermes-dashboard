use axum::{Extension, http::StatusCode, Json};
use std::sync::Arc;

use crate::AppState;
use super::dto::{LoginRequest, LoginResponse};
use super::jwt;

#[derive(serde::Serialize)]
pub struct LoginSuccessResponse {
    pub success: bool,
    pub message: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginSuccessResponse>, (StatusCode, Json<LoginResponse>)> {
    // Get credentials from env
    let valid_username = std::env::var("DASHBOARD_USERNAME")
        .unwrap_or_else(|_| "admin".to_string());
    let valid_password = std::env::var("DASHBOARD_PASSWORD")
        .unwrap_or_else(|_| "admin".to_string());

    // Validate credentials
    if payload.username != valid_username || payload.password != valid_password {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                success: false,
                message: "Invalid username or password".to_string(),
            }),
        ));
    }

    // Generate tokens
    let (access_token, refresh_token) = jwt::generate_token_pair(&payload.username, &state.jwt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse {
                    success: false,
                    message: format!("Failed to generate token: {}", e),
                }),
            )
        })?;

    Ok(Json(LoginSuccessResponse {
        success: true,
        message: "Login successful".to_string(),
        access_token,
        refresh_token,
    }))
}

pub async fn logout() -> (StatusCode, Json<LoginResponse>) {
    (
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            message: "Logged out successfully".to_string(),
        }),
    )
}
