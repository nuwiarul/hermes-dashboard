use axum::{Extension, extract::Request, http::StatusCode, Json, http::header::SET_COOKIE, response::{IntoResponse, Response}};
use std::sync::Arc;

use crate::AppState;
use super::dto::{LoginRequest, LoginResponse};
use super::jwt;

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, (StatusCode, Json<LoginResponse>)> {
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

    // Build HttpOnly, Secure cookies
    let access_cookie = format!(
        "access_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        access_token,
        state.jwt.access_duration_secs
    );
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        refresh_token,
        state.jwt.refresh_duration_secs
    );

    // Build response with Set-Cookie headers
    let mut response = (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": "Login successful"
        }))
    ).into_response();

    let headers = response.headers_mut();
    headers.insert(SET_COOKIE, access_cookie.parse().unwrap());
    headers.append(SET_COOKIE, refresh_cookie.parse().unwrap());

    Ok(response)
}

pub async fn logout() -> Response {
    // Clear cookies by setting expired cookies
    let clear_access = "access_token=; HttpOnly; Secure; SameSite=None; Path=/; Max-Age=0";
    let clear_refresh = "refresh_token=; HttpOnly; Secure; SameSite=None; Path=/; Max-Age=0";

    let mut response = (
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            message: "Logged out successfully".to_string(),
        })
    ).into_response();

    let headers = response.headers_mut();
    headers.insert(SET_COOKIE, clear_access.parse().unwrap());
    headers.append(SET_COOKIE, clear_refresh.parse().unwrap());

    response
}

/// GET /api/auth/me — Validate access_token cookie and return user info
pub async fn me(
    Extension(state): Extension<Arc<AppState>>,
    request: Request<axum::body::Body>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Extract access_token from Cookie header
    let token = extract_access_token_from_cookie(&request)?;

    // Validate token
    let claims = jwt::validate_access_token(&token, &state.jwt)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(serde_json::json!({
        "success": true,
        "username": claims.sub
    })))
}

/// Extract access_token from Cookie header in request
fn extract_access_token_from_cookie(request: &Request<axum::body::Body>) -> Result<String, StatusCode> {
    let cookie_header = request.headers()
        .get("cookie")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let cookie_str = cookie_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix("access_token=") {
            return Ok(value.to_string());
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
