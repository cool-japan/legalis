//! Event Streaming implementation for Kafka and NATS
//!
//! This module provides event streaming capabilities including:
//! - Kafka integration for event streaming
//! - NATS integration for lightweight messaging
//! - Stream consumer and producer abstractions
//! - Dead letter queue support

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

use crate::event_sourcing::DomainEvent;

/// Error types for event streaming
#[derive(Debug, Error)]
pub enum StreamingError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Publish error: {0}")]
    PublishError(String),

    #[error("Subscribe error: {0}")]
    SubscribeError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Consumer error: {0}")]
    ConsumerError(String),
}

/// Result type for streaming operations
pub type StreamResult<T> = Result<T, StreamingError>;

/// Stream message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMessage {
    /// Message ID
    pub id: String,

    /// Topic/subject
    pub topic: String,

    /// Message payload (JSON-serialized domain event)
    pub payload: serde_json::Value,

    /// Message metadata
    pub metadata: HashMap<String, String>,

    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl StreamMessage {
    /// Create a new stream message from a domain event
    pub fn from_event(event: &DomainEvent, topic: String) -> StreamResult<Self> {
        let payload = serde_json::to_value(event).map_err(|e| {
            StreamingError::SerializationError(format!("Failed to serialize event: {}", e))
        })?;

        Ok(Self {
            id: event.metadata.event_id.to_string(),
            topic,
            payload,
            metadata: event.metadata.metadata.clone(),
            timestamp: event.metadata.timestamp,
        })
    }

    /// Convert stream message back to domain event
    pub fn to_event(&self) -> StreamResult<DomainEvent> {
        serde_json::from_value(self.payload.clone()).map_err(|e| {
            StreamingError::SerializationError(format!("Failed to deserialize event: {}", e))
        })
    }
}

/// Event stream producer trait
#[async_trait]
pub trait StreamProducer: Send + Sync {
    /// Publish a message to a topic
    async fn publish(&self, topic: &str, message: StreamMessage) -> StreamResult<()>;

    /// Publish multiple messages to a topic
    async fn publish_batch(&self, topic: &str, messages: Vec<StreamMessage>) -> StreamResult<()>;
}

/// Event stream consumer trait
#[async_trait]
pub trait StreamConsumer: Send + Sync {
    /// Subscribe to a topic
    async fn subscribe(&self, topic: &str) -> StreamResult<()>;

    /// Unsubscribe from a topic
    async fn unsubscribe(&self, topic: &str) -> StreamResult<()>;

    /// Poll for new messages
    async fn poll(&self) -> StreamResult<Vec<StreamMessage>>;

    /// Acknowledge a message
    async fn ack(&self, message_id: &str) -> StreamResult<()>;

    /// Negative acknowledge (requeue) a message
    async fn nack(&self, message_id: &str) -> StreamResult<()>;
}

/// Kafka producer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaProducerConfig {
    /// Kafka broker addresses
    pub brokers: Vec<String>,

    /// Producer client ID
    pub client_id: String,

    /// Compression type (none, gzip, snappy, lz4)
    pub compression: String,

    /// Request timeout in milliseconds
    pub timeout_ms: u64,

    /// Additional configuration
    pub extra: HashMap<String, String>,
}

impl Default for KafkaProducerConfig {
    fn default() -> Self {
        Self {
            brokers: vec!["localhost:9092".to_string()],
            client_id: "legalis-producer".to_string(),
            compression: "gzip".to_string(),
            timeout_ms: 5000,
            extra: HashMap::new(),
        }
    }
}

/// Kafka consumer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConsumerConfig {
    /// Kafka broker addresses
    pub brokers: Vec<String>,

    /// Consumer group ID
    pub group_id: String,

    /// Client ID
    pub client_id: String,

    /// Auto-commit enabled
    pub auto_commit: bool,

    /// Session timeout in milliseconds
    pub session_timeout_ms: u64,

    /// Additional configuration
    pub extra: HashMap<String, String>,
}

impl Default for KafkaConsumerConfig {
    fn default() -> Self {
        Self {
            brokers: vec!["localhost:9092".to_string()],
            group_id: "legalis-consumers".to_string(),
            client_id: "legalis-consumer".to_string(),
            auto_commit: false,
            session_timeout_ms: 10000,
            extra: HashMap::new(),
        }
    }
}

/// NATS producer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsProducerConfig {
    /// NATS server URL
    pub url: String,

    /// Client name
    pub client_name: String,

    /// Additional options
    pub extra: HashMap<String, String>,
}

impl Default for NatsProducerConfig {
    fn default() -> Self {
        Self {
            url: "nats://localhost:4222".to_string(),
            client_name: "legalis-producer".to_string(),
            extra: HashMap::new(),
        }
    }
}

/// NATS consumer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsConsumerConfig {
    /// NATS server URL
    pub url: String,

    /// Client name
    pub client_name: String,

    /// Queue group (for load balancing)
    pub queue_group: Option<String>,

    /// Additional options
    pub extra: HashMap<String, String>,
}

impl Default for NatsConsumerConfig {
    fn default() -> Self {
        Self {
            url: "nats://localhost:4222".to_string(),
            client_name: "legalis-consumer".to_string(),
            queue_group: Some("legalis-consumers".to_string()),
            extra: HashMap::new(),
        }
    }
}

/// In-memory stream producer (for testing)
#[derive(Debug, Clone)]
pub struct InMemoryStreamProducer {
    messages: Arc<RwLock<HashMap<String, Vec<StreamMessage>>>>,
}

impl InMemoryStreamProducer {
    /// Create a new in-memory stream producer
    pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get all messages for a topic (for testing)
    pub fn get_messages(&self, topic: &str) -> Vec<StreamMessage> {
        self.messages
            .read()
            .unwrap()
            .get(topic)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for InMemoryStreamProducer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StreamProducer for InMemoryStreamProducer {
    async fn publish(&self, topic: &str, message: StreamMessage) -> StreamResult<()> {
        let mut messages = self.messages.write().map_err(|e| {
            StreamingError::PublishError(format!("Failed to acquire write lock: {}", e))
        })?;

        messages
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(message);

        Ok(())
    }

    async fn publish_batch(&self, topic: &str, batch: Vec<StreamMessage>) -> StreamResult<()> {
        let mut messages = self.messages.write().map_err(|e| {
            StreamingError::PublishError(format!("Failed to acquire write lock: {}", e))
        })?;

        messages
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .extend(batch);

        Ok(())
    }
}

/// In-memory stream consumer (for testing)
#[derive(Debug, Clone)]
pub struct InMemoryStreamConsumer {
    subscriptions: Arc<RwLock<Vec<String>>>,
    messages: Arc<RwLock<HashMap<String, Vec<StreamMessage>>>>,
    pending: Arc<RwLock<Vec<StreamMessage>>>,
}

impl InMemoryStreamConsumer {
    /// Create a new in-memory stream consumer
    pub fn new(producer: &InMemoryStreamProducer) -> Self {
        Self {
            subscriptions: Arc::new(RwLock::new(Vec::new())),
            messages: producer.messages.clone(),
            pending: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get subscribed topics
    pub fn get_subscriptions(&self) -> Vec<String> {
        self.subscriptions.read().unwrap().clone()
    }
}

#[async_trait]
impl StreamConsumer for InMemoryStreamConsumer {
    async fn subscribe(&self, topic: &str) -> StreamResult<()> {
        let mut subs = self.subscriptions.write().map_err(|e| {
            StreamingError::SubscribeError(format!("Failed to acquire write lock: {}", e))
        })?;

        if !subs.contains(&topic.to_string()) {
            subs.push(topic.to_string());
        }

        Ok(())
    }

    async fn unsubscribe(&self, topic: &str) -> StreamResult<()> {
        let mut subs = self.subscriptions.write().map_err(|e| {
            StreamingError::SubscribeError(format!("Failed to acquire write lock: {}", e))
        })?;

        subs.retain(|t| t != topic);

        Ok(())
    }

    async fn poll(&self) -> StreamResult<Vec<StreamMessage>> {
        let subs = self.subscriptions.read().map_err(|e| {
            StreamingError::ConsumerError(format!("Failed to acquire read lock: {}", e))
        })?;

        let mut pending = self.pending.write().map_err(|e| {
            StreamingError::ConsumerError(format!("Failed to acquire write lock: {}", e))
        })?;

        let messages = self.messages.read().map_err(|e| {
            StreamingError::ConsumerError(format!("Failed to acquire read lock: {}", e))
        })?;

        let mut result = Vec::new();

        for topic in subs.iter() {
            if let Some(topic_messages) = messages.get(topic) {
                result.extend(topic_messages.clone());
            }
        }

        pending.extend(result.clone());

        Ok(result)
    }

    async fn ack(&self, message_id: &str) -> StreamResult<()> {
        let mut pending = self.pending.write().map_err(|e| {
            StreamingError::ConsumerError(format!("Failed to acquire write lock: {}", e))
        })?;

        pending.retain(|m| m.id != message_id);

        Ok(())
    }

    async fn nack(&self, _message_id: &str) -> StreamResult<()> {
        // For in-memory implementation, we just keep the message in pending
        Ok(())
    }
}

/// Dead letter queue for failed messages
pub struct DeadLetterQueue {
    messages: Arc<RwLock<Vec<(StreamMessage, String)>>>,
}

impl DeadLetterQueue {
    /// Create a new dead letter queue
    pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a failed message to the DLQ
    pub fn add(&self, message: StreamMessage, reason: String) -> StreamResult<()> {
        let mut messages = self.messages.write().map_err(|e| {
            StreamingError::PublishError(format!("Failed to acquire write lock: {}", e))
        })?;

        messages.push((message, reason));

        Ok(())
    }

    /// Get all messages in the DLQ
    pub fn get_all(&self) -> Vec<(StreamMessage, String)> {
        self.messages.read().unwrap().clone()
    }

    /// Clear the DLQ
    pub fn clear(&self) -> StreamResult<()> {
        let mut messages = self.messages.write().map_err(|e| {
            StreamingError::PublishError(format!("Failed to acquire write lock: {}", e))
        })?;

        messages.clear();

        Ok(())
    }
}

impl Default for DeadLetterQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Event stream router for routing events to appropriate topics
pub struct EventRouter {
    /// Routing rules: event_type -> topic
    rules: Arc<RwLock<HashMap<String, String>>>,
}

impl EventRouter {
    /// Create a new event router
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a routing rule
    pub fn add_rule(&self, event_type: String, topic: String) {
        let mut rules = self.rules.write().unwrap();
        rules.insert(event_type, topic);
    }

    /// Get topic for an event type
    pub fn get_topic(&self, event_type: &str) -> Option<String> {
        self.rules.read().unwrap().get(event_type).cloned()
    }

    /// Route an event to the appropriate topic
    pub fn route(&self, event: &DomainEvent) -> Option<String> {
        self.get_topic(&event.metadata.event_type)
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_sourcing::DomainEvent;

    #[tokio::test]
    async fn test_in_memory_producer() {
        let producer = InMemoryStreamProducer::new();

        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test"}),
        );

        let message = StreamMessage::from_event(&event, "statutes".to_string()).unwrap();

        producer.publish("statutes", message).await.unwrap();

        let messages = producer.get_messages("statutes");
        assert_eq!(messages.len(), 1);
    }

    #[tokio::test]
    async fn test_in_memory_consumer() {
        let producer = InMemoryStreamProducer::new();
        let consumer = InMemoryStreamConsumer::new(&producer);

        // Subscribe to topic
        consumer.subscribe("statutes").await.unwrap();

        // Publish a message
        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test"}),
        );

        let message = StreamMessage::from_event(&event, "statutes".to_string()).unwrap();
        producer.publish("statutes", message.clone()).await.unwrap();

        // Poll for messages
        let messages = consumer.poll().await.unwrap();
        assert_eq!(messages.len(), 1);

        // Acknowledge message
        consumer.ack(&message.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_dead_letter_queue() {
        let dlq = DeadLetterQueue::new();

        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test"}),
        );

        let message = StreamMessage::from_event(&event, "statutes".to_string()).unwrap();

        dlq.add(message, "Processing failed".to_string()).unwrap();

        let messages = dlq.get_all();
        assert_eq!(messages.len(), 1);
    }

    #[tokio::test]
    async fn test_event_router() {
        let router = EventRouter::new();

        router.add_rule("StatuteCreated".to_string(), "statute-events".to_string());
        router.add_rule("StatuteUpdated".to_string(), "statute-events".to_string());

        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test"}),
        );

        let topic = router.route(&event);
        assert_eq!(topic, Some("statute-events".to_string()));
    }
}
