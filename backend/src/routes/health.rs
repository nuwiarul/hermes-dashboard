use axum::Json;
use serde_json::{json, Value};

pub async fn handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "hermes-dashboard",
        "version": "0.1.0"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_handler() {
        let response = handler().await;
        let json = response.0;
        
        assert_eq!(json["status"], "ok");
        assert_eq!(json["service"], "hermes-dashboard");
        assert_eq!(json["version"], "0.1.0");
    }

    #[tokio::test]
    async fn test_health_handler_response_format() {
        let response = handler().await;
        let json = response.0;
        
        assert!(json.get("status").is_some());
        assert!(json.get("service").is_some());
        assert!(json.get("version").is_some());
    }
}
