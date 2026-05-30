use axum::{
    extract::Request,
    http::header::SET_COOKIE,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use std::sync::Arc;

use super::dto::{LoginRequest, LoginResponse};
use super::jwt;
use crate::shared::validation::{validate_password, validate_username};
use crate::AppState;

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    let username = validate_username(&payload.username).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": e.message
            })),
        )
    })?;

    let password = validate_password(&payload.password).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": e.message
            })),
        )
    })?;

    // Get credentials from env
    let valid_username =
        std::env::var("DASHBOARD_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let valid_password =
        std::env::var("DASHBOARD_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    // Validate credentials
    if username != valid_username || password != valid_password {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "message": "Invalid username or password"
            })),
        ));
    }

    // Generate tokens
    let (access_token, refresh_token) = jwt::generate_token_pair(&payload.username, &state.jwt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": format!("Failed to generate token: {}", e)
                })),
            )
        })?;

    // Build HttpOnly, Secure cookies
    let access_cookie = format!(
        "access_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        access_token, state.jwt.access_duration_secs
    );
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        refresh_token, state.jwt.refresh_duration_secs
    );

    // Build response with Set-Cookie headers
    let mut response = (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": "Login successful"
        })),
    )
        .into_response();

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
        }),
    )
        .into_response();

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
    // Try access_token first
    if let Ok(token) = extract_token_from_cookie(&request, "access_token") {
        if let Ok(claims) = jwt::validate_access_token(&token, &state.jwt) {
            return Ok(Json(serde_json::json!({
                "success": true,
                "username": claims.sub
            })));
        }
    }

    // Access token invalid/expired — try refresh
    Err(StatusCode::UNAUTHORIZED)
}

/// POST /api/auth/refresh — Use refresh_token to get new access_token + refresh_token
pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    request: Request<axum::body::Body>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Extract refresh_token from cookie
    let refresh_token = extract_token_from_cookie(&request, "refresh_token").map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "message": "No refresh token found"
            })),
        )
    })?;

    // Validate refresh token
    let claims = jwt::validate_refresh_token(&refresh_token, &state.jwt).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "message": "Invalid or expired refresh token"
            })),
        )
    })?;

    // Generate new token pair
    let (new_access, new_refresh) =
        jwt::generate_token_pair(&claims.sub, &state.jwt).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": format!("Failed to generate token: {}", e)
                })),
            )
        })?;

    // Build new cookies
    let access_cookie = format!(
        "access_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        new_access, state.jwt.access_duration_secs
    );
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        new_refresh, state.jwt.refresh_duration_secs
    );

    // Build response
    let mut response = (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": "Token refreshed successfully"
        })),
    )
        .into_response();

    let headers = response.headers_mut();
    headers.insert(SET_COOKIE, access_cookie.parse().unwrap());
    headers.append(SET_COOKIE, refresh_cookie.parse().unwrap());

    Ok(response)
}

/// Extract a specific token from Cookie header
fn extract_token_from_cookie(
    request: &Request<axum::body::Body>,
    name: &str,
) -> Result<String, StatusCode> {
    let cookie_header = request
        .headers()
        .get("cookie")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let cookie_str = cookie_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix(&format!("{}=", name)) {
            return Ok(value.to_string());
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
