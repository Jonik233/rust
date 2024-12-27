use warp::Filter;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use futures_util::{StreamExt};
use std::collections::HashMap;
use std::sync::Arc;

// Типи даних
#[derive(Serialize, Deserialize, Clone)]
struct Message {
    sender: String,
    recipient: String,
    content: String,
    timestamp: String,
}

#[derive(Default)]
struct ChatState {
    clients: RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>,
    history: RwLock<Vec<Message>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(ChatState::default());

    // Обробка WebSocket з'єднань
    let ws_route = warp::path("chat")
        .and(warp::ws())
        .and(with_state(state.clone()))
        .map(|ws: warp::ws::Ws, state| {
            ws.on_upgrade(move |socket| handle_connection(socket, state))
        });

    let static_files = warp::fs::dir("./static");

    // Запуск сервера
    warp::serve(ws_route.or(static_files))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

fn with_state(state: Arc<ChatState>) -> impl Filter<Extract = (Arc<ChatState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_connection(ws: warp::ws::WebSocket, state: Arc<ChatState>) {
    let (mut tx, mut rx) = ws.split();
    let (user_tx, user_rx) = mpsc::unbounded_channel();

    // Register client
    let username = "user_".to_string();
    state.clients.write().await.insert(username.clone(), user_tx);

    // Stream for outgoing messages
    let user_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(user_rx);
    tokio::task::spawn(async move {
        user_rx
            .map(|msg| Ok(warp::ws::Message::text(serde_json::to_string(&msg).unwrap())))
            .forward(&mut tx)
            .await
            .unwrap_or_else(|e| eprintln!("WebSocket send error: {}", e));
    });

    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if let Ok(text) = msg.to_str() {
                if let Ok(parsed) = serde_json::from_str::<Message>(text) {
                    state.history.write().await.push(parsed.clone());

                    // Send message to recipient
                    if let Some(recipient_tx) = state.clients.read().await.get(&parsed.recipient) {
                        recipient_tx.send(parsed).unwrap();
                    }
                }
            }
        }
    }

    state.clients.write().await.remove(&username);
}