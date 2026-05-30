use crate::AppState;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Send initial status
    let status = get_status(&state).await;
    let _ = sender
        .send(Message::Text(
            serde_json::to_string(&status).unwrap().into(),
        ))
        .await;

    // Use tokio::select to handle both incoming messages and periodic updates
    let mut update_interval = tokio::time::interval(std::time::Duration::from_secs(5));
    let mut last_status = status.clone();

    loop {
        tokio::select! {
            // Handle incoming messages
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                            if data.get("type").and_then(|t| t.as_str()) == Some("ping") {
                                let pong = json!({
                                    "type": "pong",
                                    "timestamp": chrono::Utc::now().timestamp()
                                });
                                if sender.send(Message::Text(serde_json::to_string(&pong).unwrap().into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            // Send periodic updates
            _ = update_interval.tick() => {
                let new_status = get_status(&state).await;
                if new_status != last_status {
                    if sender.send(Message::Text(serde_json::to_string(&new_status).unwrap().into())).await.is_err() {
                        break;
                    }
                    last_status = new_status;
                }
            }
        }
    }
}

async fn get_status(state: &AppState) -> serde_json::Value {
    let sessions = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM sessions")
        .fetch_one(&state.db)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let messages = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM messages")
        .fetch_one(&state.db)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    json!({
        "type": "status",
        "online": true,
        "total_sessions": sessions,
        "total_messages": messages,
        "timestamp": chrono::Utc::now().timestamp()
    })
}
