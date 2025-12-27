//! WebSocket support for real-time updates.

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::{auth::AuthUser, AppState};

/// WebSocket notification types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsNotification {
    /// A statute was created
    StatuteCreated {
        statute_id: String,
        title: String,
        created_by: String,
    },
    /// A statute was updated
    StatuteUpdated {
        statute_id: String,
        title: String,
        updated_by: String,
    },
    /// A statute was deleted
    StatuteDeleted {
        statute_id: String,
        deleted_by: String,
    },
    /// A verification job completed
    VerificationCompleted {
        job_id: String,
        passed: bool,
        errors_count: usize,
        warnings_count: usize,
    },
    /// A simulation completed
    SimulationCompleted {
        simulation_id: String,
        total_entities: usize,
        deterministic_rate: f64,
        discretionary_rate: f64,
        void_rate: f64,
    },
    /// System status update
    SystemStatus {
        status: String,
        message: String,
    },
}

/// WebSocket message from client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum WsClientMessage {
    /// Subscribe to specific event types
    Subscribe { events: Vec<String> },
    /// Unsubscribe from event types
    Unsubscribe { events: Vec<String> },
    /// Ping to keep connection alive
    Ping,
}

/// WebSocket message to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsServerMessage {
    /// Notification event
    Notification(WsNotification),
    /// Pong response
    Pong,
    /// Subscription confirmation
    Subscribed { events: Vec<String> },
    /// Unsubscription confirmation
    Unsubscribed { events: Vec<String> },
    /// Error message
    Error { message: String },
}

/// WebSocket notification broadcaster.
#[derive(Clone)]
pub struct WsBroadcaster {
    tx: broadcast::Sender<WsNotification>,
}

impl WsBroadcaster {
    /// Creates a new WebSocket broadcaster.
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self { tx }
    }

    /// Broadcasts a notification to all connected WebSocket clients.
    pub fn broadcast(&self, notification: WsNotification) {
        if let Err(e) = self.tx.send(notification) {
            warn!("Failed to broadcast WebSocket notification: {}", e);
        }
    }

    /// Subscribes to notifications.
    pub fn subscribe(&self) -> broadcast::Receiver<WsNotification> {
        self.tx.subscribe()
    }
}

impl Default for WsBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket handler.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    info!("WebSocket connection request from user: {}", user.username);

    ws.on_upgrade(move |socket| handle_socket(socket, user, state))
}

/// Handles a WebSocket connection.
async fn handle_socket(socket: WebSocket, user: AuthUser, state: Arc<AppState>) {
    info!("WebSocket connection established for user: {}", user.username);

    let (mut sender, mut receiver) = socket.split();

    // Subscribe to notifications
    let mut rx = state.ws_broadcaster.subscribe();

    // Create a channel for control messages
    let (ctrl_tx, mut ctrl_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<String>>();

    // Track subscribed events (empty means all events)
    let subscribed_events = std::sync::Arc::new(tokio::sync::RwLock::new(Vec::<String>::new()));
    let subscribed_events_clone = subscribed_events.clone();

    // Spawn a task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(notification) = rx.recv() => {
                    // Check if we should send this notification based on subscriptions
                    let events = subscribed_events_clone.read().await;
                    let should_send = events.is_empty()
                        || events.contains(&notification_type(&notification));
                    drop(events);

                    if !should_send {
                        continue;
                    }

                    let msg = WsServerMessage::Notification(notification);
                    let json = match serde_json::to_string(&msg) {
                        Ok(json) => json,
                        Err(e) => {
                            error!("Failed to serialize WebSocket message: {}", e);
                            continue;
                        }
                    };

                    if sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
                Some(new_events) = ctrl_rx.recv() => {
                    let mut events = subscribed_events_clone.write().await;
                    *events = new_events;
                }
            }
        }
    });

    // Handle incoming messages
    let username = user.username.clone();
    let subscribed_events_task = subscribed_events.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    debug!(
                        "Received WebSocket message from {}: {}",
                        username, text
                    );

                    match serde_json::from_str::<WsClientMessage>(&text) {
                        Ok(WsClientMessage::Subscribe { events }) => {
                            let _ = ctrl_tx.send(events.clone());
                        }
                        Ok(WsClientMessage::Unsubscribe { events }) => {
                            let mut current_events = subscribed_events_task.write().await;
                            current_events.retain(|e| !events.contains(e));
                            let new_events = current_events.clone();
                            drop(current_events);
                            let _ = ctrl_tx.send(new_events);
                        }
                        Ok(WsClientMessage::Ping) => {
                            // Ping/Pong is handled automatically by the WebSocket protocol
                        }
                        Err(_e) => {
                            // Ignore invalid messages
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed by client: {}", username);
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    error!("WebSocket error for user {}: {}", username, e);
                    break;
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    info!("WebSocket connection terminated for user: {}", user.username);
}

/// Returns the notification type as a string for subscription filtering.
fn notification_type(notification: &WsNotification) -> String {
    match notification {
        WsNotification::StatuteCreated { .. } => "statute_created".to_string(),
        WsNotification::StatuteUpdated { .. } => "statute_updated".to_string(),
        WsNotification::StatuteDeleted { .. } => "statute_deleted".to_string(),
        WsNotification::VerificationCompleted { .. } => "verification_completed".to_string(),
        WsNotification::SimulationCompleted { .. } => "simulation_completed".to_string(),
        WsNotification::SystemStatus { .. } => "system_status".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_type() {
        let notification = WsNotification::StatuteCreated {
            statute_id: "test".to_string(),
            title: "Test".to_string(),
            created_by: "user".to_string(),
        };
        assert_eq!(notification_type(&notification), "statute_created");

        let notification = WsNotification::VerificationCompleted {
            job_id: "job1".to_string(),
            passed: true,
            errors_count: 0,
            warnings_count: 0,
        };
        assert_eq!(notification_type(&notification), "verification_completed");
    }

    #[test]
    fn test_ws_broadcaster() {
        let broadcaster = WsBroadcaster::new();
        let mut rx = broadcaster.subscribe();

        let notification = WsNotification::SystemStatus {
            status: "online".to_string(),
            message: "All systems operational".to_string(),
        };

        broadcaster.broadcast(notification.clone());

        // Try to receive (this would block in real usage, but for testing we just verify the channel works)
        assert!(rx.try_recv().is_ok());
    }
}
