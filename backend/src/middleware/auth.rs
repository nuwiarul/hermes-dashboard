use axum::http::header;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::features::auth::jwt;

/// Auth middleware that validates JWT from Authorization header
/// or access_token cookie
pub async fn require_auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    // Try to extract token from Authorization header first
    let token = extract_token_from_header(&request).or_else(|| extract_token_from_cookie(&request));

    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate the token
    let state = request
        .extensions()
        .get::<std::sync::Arc<crate::AppState>>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let claims =
        jwt::validate_access_token(&token, &state.jwt).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Insert claims into request extensions for handlers to use
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Extract Bearer token from Authorization header
fn extract_token_from_header(request: &Request) -> Option<String> {
    let auth_header = request.headers().get(header::AUTHORIZATION)?;
    let auth_str = auth_header.to_str().ok()?;

    if auth_str.starts_with("Bearer ") {
        Some(auth_str[7..].to_string())
    } else {
        None
    }
}

/// Extract access_token from Cookie header
fn extract_token_from_cookie(request: &Request) -> Option<String> {
    let cookie_header = request.headers().get(header::COOKIE)?;
    let cookie_str = cookie_header.to_str().ok()?;

    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix("access_token=") {
            return Some(value.to_string());
        }
    }

    None
}
