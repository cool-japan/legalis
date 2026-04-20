use super::*;

/// GraphQL subscription events for real-time updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionEvent {
    /// Statute was registered
    StatuteRegistered {
        statute_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Statute was updated
    StatuteUpdated {
        statute_id: String,
        version: u32,
        timestamp: DateTime<Utc>,
    },
    /// Statute was deleted
    StatuteDeleted {
        statute_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Statute status changed
    StatusChanged {
        statute_id: String,
        old_status: StatuteStatus,
        new_status: StatuteStatus,
        timestamp: DateTime<Utc>,
    },
}

/// Subscription manager for GraphQL subscriptions.
#[derive(Debug, Clone)]
pub struct SubscriptionManager {
    /// Active subscriptions
    subscriptions: Arc<Mutex<HashMap<Uuid, SubscriptionFilter>>>,
    /// Published events (stored for testing/replay)
    published_events: Arc<Mutex<Vec<SubscriptionEvent>>>,
}

/// Filter for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionFilter {
    /// Filter by statute IDs
    pub statute_ids: Option<Vec<String>>,
    /// Filter by jurisdictions
    pub jurisdictions: Option<Vec<String>>,
    /// Filter by tags
    pub tags: Option<Vec<String>>,
    /// Filter by event types
    pub event_types: Option<Vec<String>>,
}

impl SubscriptionManager {
    /// Creates a new subscription manager.
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            published_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribes with a filter.
    pub fn subscribe(&self, filter: SubscriptionFilter) -> Uuid {
        let subscription_id = Uuid::new_v4();
        self.subscriptions
            .lock()
            .expect("subscriptions mutex poisoned")
            .insert(subscription_id, filter);
        subscription_id
    }

    /// Unsubscribes.
    pub fn unsubscribe(&self, subscription_id: Uuid) -> bool {
        self.subscriptions
            .lock()
            .expect("subscriptions mutex poisoned")
            .remove(&subscription_id)
            .is_some()
    }

    /// Publishes an event to all subscribers.
    /// In production with the async feature, this would use tokio::sync::broadcast.
    pub fn publish(&self, event: SubscriptionEvent) {
        self.published_events
            .lock()
            .expect("published_events mutex poisoned")
            .push(event);
    }

    /// Gets active subscription count.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions
            .lock()
            .expect("subscriptions mutex poisoned")
            .len()
    }

    /// Gets published events (for testing).
    pub fn get_published_events(&self) -> Vec<SubscriptionEvent> {
        self.published_events
            .lock()
            .expect("published_events mutex poisoned")
            .clone()
    }

    /// Clears published events.
    pub fn clear_events(&self) {
        self.published_events
            .lock()
            .expect("published_events mutex poisoned")
            .clear();
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// gRPC service definition (placeholder for protobuf generation).
pub mod grpc {
    use super::*;

    /// gRPC request for getting a statute.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetStatuteRequest {
        pub statute_id: String,
    }

    /// gRPC response for getting a statute.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetStatuteResponse {
        pub statute: Option<StatuteEntry>,
        pub found: bool,
    }

    /// gRPC request for listing statutes.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ListStatutesRequest {
        pub page: u32,
        pub page_size: u32,
        pub jurisdiction: Option<String>,
        pub tags: Vec<String>,
    }

    /// gRPC response for listing statutes.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ListStatutesResponse {
        pub statutes: Vec<StatuteEntry>,
        pub total_count: usize,
        pub has_more: bool,
    }

    /// gRPC request for registering a statute.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RegisterStatuteRequest {
        pub statute: StatuteEntry,
    }

    /// gRPC response for registering a statute.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RegisterStatuteResponse {
        pub success: bool,
        pub statute_id: String,
        pub error: Option<String>,
    }

    /// gRPC service implementation.
    pub struct GrpcStatuteService {
        registry: Arc<Mutex<StatuteRegistry>>,
    }

    impl GrpcStatuteService {
        /// Creates a new gRPC service.
        pub fn new(registry: Arc<Mutex<StatuteRegistry>>) -> Self {
            Self { registry }
        }

        /// Gets a statute.
        pub fn get_statute(&self, request: GetStatuteRequest) -> GetStatuteResponse {
            let mut registry = self.registry.lock().expect("registry mutex poisoned");
            match registry.get(&request.statute_id) {
                Some(statute) => GetStatuteResponse {
                    statute: Some(statute),
                    found: true,
                },
                None => GetStatuteResponse {
                    statute: None,
                    found: false,
                },
            }
        }

        /// Lists statutes.
        pub fn list_statutes(&self, request: ListStatutesRequest) -> ListStatutesResponse {
            let registry = self.registry.lock().expect("registry mutex poisoned");
            let mut statutes: Vec<_> = registry.list().into_iter().cloned().collect();

            // Apply jurisdiction filter
            if let Some(ref jurisdiction) = request.jurisdiction {
                statutes.retain(|s| &s.jurisdiction == jurisdiction);
            }

            // Apply tag filter
            if !request.tags.is_empty() {
                statutes.retain(|s| request.tags.iter().any(|tag| s.tags.contains(tag)));
            }

            let total_count = statutes.len();
            let start = (request.page * request.page_size) as usize;
            let end = std::cmp::min(start + request.page_size as usize, total_count);

            let page_statutes = if start < total_count {
                statutes[start..end].to_vec()
            } else {
                Vec::new()
            };

            ListStatutesResponse {
                statutes: page_statutes,
                total_count,
                has_more: end < total_count,
            }
        }

        /// Registers a statute.
        pub fn register_statute(&self, request: RegisterStatuteRequest) -> RegisterStatuteResponse {
            let mut registry = self.registry.lock().expect("registry mutex poisoned");
            match registry.register(request.statute) {
                Ok(statute_id) => RegisterStatuteResponse {
                    success: true,
                    statute_id: statute_id.to_string(),
                    error: None,
                },
                Err(e) => RegisterStatuteResponse {
                    success: false,
                    statute_id: String::new(),
                    error: Some(e.to_string()),
                },
            }
        }
    }
}

/// Event streaming infrastructure.
pub mod streaming {
    use super::*;

    /// Stream destination type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum StreamDestination {
        /// Apache Kafka
        Kafka,
        /// NATS messaging
        Nats,
        /// Amazon Kinesis
        Kinesis,
        /// Custom webhook
        Webhook,
    }

    /// Stream configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StreamConfig {
        /// Stream name
        pub name: String,
        /// Destination type
        pub destination: StreamDestination,
        /// Connection string (URL, broker address, etc.)
        pub connection: String,
        /// Topic/subject name
        pub topic: String,
        /// Optional authentication
        pub auth: Option<HashMap<String, String>>,
        /// Buffer size
        pub buffer_size: usize,
        /// Enable/disable flag
        pub enabled: bool,
    }

    impl StreamConfig {
        /// Creates a new stream configuration.
        pub fn new(
            name: impl Into<String>,
            destination: StreamDestination,
            connection: impl Into<String>,
            topic: impl Into<String>,
        ) -> Self {
            Self {
                name: name.into(),
                destination,
                connection: connection.into(),
                topic: topic.into(),
                auth: None,
                buffer_size: 1000,
                enabled: true,
            }
        }

        /// Adds authentication.
        pub fn with_auth(mut self, auth: HashMap<String, String>) -> Self {
            self.auth = Some(auth);
            self
        }

        /// Sets buffer size.
        pub fn with_buffer_size(mut self, size: usize) -> Self {
            self.buffer_size = size;
            self
        }

        /// Sets enabled flag.
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
    }

    /// Event stream message.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StreamMessage {
        /// Message ID
        pub message_id: Uuid,
        /// Event type
        pub event_type: String,
        /// Statute ID
        pub statute_id: String,
        /// Event payload (JSON)
        pub payload: String,
        /// Timestamp
        pub timestamp: DateTime<Utc>,
        /// Metadata
        pub metadata: HashMap<String, String>,
    }

    impl StreamMessage {
        /// Creates a new stream message.
        pub fn new(
            event_type: impl Into<String>,
            statute_id: impl Into<String>,
            payload: impl Into<String>,
        ) -> Self {
            Self {
                message_id: Uuid::new_v4(),
                event_type: event_type.into(),
                statute_id: statute_id.into(),
                payload: payload.into(),
                timestamp: Utc::now(),
                metadata: HashMap::new(),
            }
        }

        /// Adds metadata.
        pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.metadata.insert(key.into(), value.into());
            self
        }
    }

    /// Event stream manager.
    #[derive(Debug, Clone)]
    pub struct EventStreamManager {
        /// Stream configurations
        configs: HashMap<String, StreamConfig>,
        /// Published message count by stream
        message_count: HashMap<String, usize>,
    }

    impl EventStreamManager {
        /// Creates a new event stream manager.
        pub fn new() -> Self {
            Self {
                configs: HashMap::new(),
                message_count: HashMap::new(),
            }
        }

        /// Adds a stream configuration.
        pub fn add_stream(&mut self, config: StreamConfig) {
            let name = config.name.clone();
            self.configs.insert(name.clone(), config);
            self.message_count.insert(name, 0);
        }

        /// Removes a stream configuration.
        pub fn remove_stream(&mut self, name: &str) -> bool {
            self.message_count.remove(name);
            self.configs.remove(name).is_some()
        }

        /// Gets a stream configuration.
        pub fn get_stream(&self, name: &str) -> Option<&StreamConfig> {
            self.configs.get(name)
        }

        /// Lists all streams.
        pub fn list_streams(&self) -> Vec<&StreamConfig> {
            self.configs.values().collect()
        }

        /// Publishes a message to a stream.
        /// In production, this would actually publish to Kafka/NATS/etc.
        pub fn publish(
            &mut self,
            stream_name: &str,
            _message: StreamMessage,
        ) -> Result<(), String> {
            let config = self
                .configs
                .get(stream_name)
                .ok_or_else(|| format!("Stream '{}' not found", stream_name))?;

            if !config.enabled {
                return Err(format!("Stream '{}' is disabled", stream_name));
            }

            // Placeholder: In production, actually publish to the stream
            // match config.destination {
            //     StreamDestination::Kafka => { /* kafka publish */ },
            //     StreamDestination::Nats => { /* nats publish */ },
            //     ...
            // }

            // Increment message count
            *self
                .message_count
                .get_mut(stream_name)
                .expect("invariant: message_count is in sync with configs") += 1;
            Ok(())
        }

        /// Gets message count for a stream.
        pub fn get_message_count(&self, stream_name: &str) -> usize {
            self.message_count.get(stream_name).copied().unwrap_or(0)
        }

        /// Resets message count for a stream.
        pub fn reset_count(&mut self, stream_name: &str) {
            if let Some(count) = self.message_count.get_mut(stream_name) {
                *count = 0;
            }
        }
    }

    impl Default for EventStreamManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Enhanced bulk operations.
pub mod bulk {
    use super::*;

    /// Bulk operation type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum BulkOperationType {
        /// Register multiple statutes
        Register,
        /// Update multiple statutes
        Update,
        /// Delete multiple statutes
        Delete,
        /// Archive multiple statutes
        Archive,
        /// Change status for multiple statutes
        ChangeStatus,
    }

    /// Bulk operation request.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BulkOperationRequest {
        /// Operation type
        pub operation_type: BulkOperationType,
        /// Statute IDs (for update/delete/archive/status change)
        pub statute_ids: Vec<String>,
        /// Statute entries (for register)
        pub statute_entries: Vec<StatuteEntry>,
        /// New status (for status change)
        pub new_status: Option<StatuteStatus>,
        /// Continue on error flag
        pub continue_on_error: bool,
    }

    /// Bulk operation response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BulkOperationResponse {
        /// Operation type
        pub operation_type: BulkOperationType,
        /// Total items processed
        pub total_processed: usize,
        /// Successful operations
        pub successful: usize,
        /// Failed operations
        pub failed: usize,
        /// Error details
        pub errors: Vec<BulkOperationError>,
        /// Duration in milliseconds
        pub duration_ms: u64,
    }

    /// Bulk operation error.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BulkOperationError {
        /// Statute ID
        pub statute_id: String,
        /// Error message
        pub error: String,
    }

    impl BulkOperationResponse {
        /// Creates a new bulk operation response.
        pub fn new(operation_type: BulkOperationType) -> Self {
            Self {
                operation_type,
                total_processed: 0,
                successful: 0,
                failed: 0,
                errors: Vec::new(),
                duration_ms: 0,
            }
        }

        /// Calculates success rate (0.0 to 1.0).
        pub fn success_rate(&self) -> f64 {
            if self.total_processed == 0 {
                0.0
            } else {
                self.successful as f64 / self.total_processed as f64
            }
        }

        /// Checks if all operations succeeded.
        pub fn is_complete_success(&self) -> bool {
            self.failed == 0 && self.total_processed > 0
        }
    }

    /// Bulk operation executor.
    pub struct BulkOperationExecutor {
        registry: Arc<Mutex<StatuteRegistry>>,
    }

    impl BulkOperationExecutor {
        /// Creates a new bulk operation executor.
        pub fn new(registry: Arc<Mutex<StatuteRegistry>>) -> Self {
            Self { registry }
        }

        /// Executes a bulk operation.
        pub fn execute(&self, request: BulkOperationRequest) -> BulkOperationResponse {
            let start = std::time::Instant::now();
            let mut response = BulkOperationResponse::new(request.operation_type);

            match request.operation_type {
                BulkOperationType::Register => {
                    for entry in request.statute_entries {
                        response.total_processed += 1;
                        let statute_id = entry.statute.id.clone();
                        let mut registry = self.registry.lock().expect("registry mutex poisoned");
                        match registry.register(entry) {
                            Ok(_) => response.successful += 1,
                            Err(e) => {
                                response.failed += 1;
                                response.errors.push(BulkOperationError {
                                    statute_id: statute_id.clone(),
                                    error: e.to_string(),
                                });
                                if !request.continue_on_error {
                                    break;
                                }
                            }
                        }
                    }
                }
                BulkOperationType::Delete => {
                    for statute_id in &request.statute_ids {
                        response.total_processed += 1;
                        let mut registry = self.registry.lock().expect("registry mutex poisoned");
                        match registry.delete(statute_id) {
                            Ok(_) => response.successful += 1,
                            Err(e) => {
                                response.failed += 1;
                                response.errors.push(BulkOperationError {
                                    statute_id: statute_id.clone(),
                                    error: e.to_string(),
                                });
                                if !request.continue_on_error {
                                    break;
                                }
                            }
                        }
                    }
                }
                BulkOperationType::Archive => {
                    for statute_id in &request.statute_ids {
                        response.total_processed += 1;
                        let mut registry = self.registry.lock().expect("registry mutex poisoned");
                        match registry.archive_statute(statute_id, "Bulk archive".to_string()) {
                            Ok(_) => response.successful += 1,
                            Err(e) => {
                                response.failed += 1;
                                response.errors.push(BulkOperationError {
                                    statute_id: statute_id.clone(),
                                    error: e.to_string(),
                                });
                                if !request.continue_on_error {
                                    break;
                                }
                            }
                        }
                    }
                }
                BulkOperationType::ChangeStatus => {
                    if let Some(new_status) = request.new_status {
                        for statute_id in &request.statute_ids {
                            response.total_processed += 1;
                            let mut registry =
                                self.registry.lock().expect("registry mutex poisoned");
                            match registry.set_status(statute_id, new_status) {
                                Ok(_) => response.successful += 1,
                                Err(e) => {
                                    response.failed += 1;
                                    response.errors.push(BulkOperationError {
                                        statute_id: statute_id.clone(),
                                        error: e.to_string(),
                                    });
                                    if !request.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                BulkOperationType::Update => {
                    // Update operations would need statute entries
                    for entry in request.statute_entries {
                        response.total_processed += 1;
                        let statute_id = entry.statute.id.clone();
                        let mut registry = self.registry.lock().expect("registry mutex poisoned");
                        match registry.update(&statute_id, entry.statute.clone()) {
                            Ok(_) => response.successful += 1,
                            Err(e) => {
                                response.failed += 1;
                                response.errors.push(BulkOperationError {
                                    statute_id,
                                    error: e.to_string(),
                                });
                                if !request.continue_on_error {
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            response.duration_ms = start.elapsed().as_millis() as u64;
            response
        }
    }
}

/// SDK generation templates.
pub mod sdk_gen {
    use super::*;

    /// Supported SDK languages.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SdkLanguage {
        Python,
        JavaScript,
        TypeScript,
        Rust,
        Go,
        Java,
        CSharp,
        Ruby,
    }

    /// SDK generation configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SdkConfig {
        /// Target language
        pub language: SdkLanguage,
        /// Package name
        pub package_name: String,
        /// API base URL
        pub api_base_url: String,
        /// Include async support
        pub async_support: bool,
        /// Include type definitions
        pub type_definitions: bool,
        /// Include documentation
        pub include_docs: bool,
    }

    /// SDK code generator.
    pub struct SdkGenerator;

    impl SdkGenerator {
        /// Generates SDK code for the specified language.
        pub fn generate(config: &SdkConfig) -> Result<String, String> {
            match config.language {
                SdkLanguage::Python => Self::generate_python(config),
                SdkLanguage::JavaScript => Self::generate_javascript(config),
                SdkLanguage::TypeScript => Self::generate_typescript(config),
                SdkLanguage::Rust => Self::generate_rust(config),
                SdkLanguage::Go => Self::generate_go(config),
                SdkLanguage::Java => Self::generate_java(config),
                SdkLanguage::CSharp => Self::generate_csharp(config),
                SdkLanguage::Ruby => Self::generate_ruby(config),
            }
        }

        fn generate_python(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"# {} Python SDK
import requests
from typing import Optional, List, Dict, Any

class StatuteRegistryClient:
"""Client for the Statute Registry API."""

def __init__(self, base_url: str = "{}"):
    self.base_url = base_url

def get_statute(self, statute_id: str) -> Optional[Dict[str, Any]]:
    """Gets a statute by ID."""
    response = requests.get(f"{{self.base_url}}/statutes/{{statute_id}}")
    if response.status_code == 200:
        return response.json()
    return None

def list_statutes(self, page: int = 0, per_page: int = 50) -> List[Dict[str, Any]]:
    """Lists statutes with pagination."""
    params = {{"page": page, "per_page": per_page}}
    response = requests.get(f"{{self.base_url}}/statutes", params=params)
    return response.json() if response.status_code == 200 else []
"#,
                config.package_name, config.api_base_url
            ))
        }

        fn generate_javascript(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"// {} JavaScript SDK
class StatuteRegistryClient {{
constructor(baseUrl = "{}") {{
    this.baseUrl = baseUrl;
}}

async getStatute(statuteId) {{
    const response = await fetch(`${{this.baseUrl}}/statutes/${{statuteId}}`);
    if (response.ok) {{
        return await response.json();
    }}
    return null;
}}

async listStatutes(page = 0, perPage = 50) {{
    const params = new URLSearchParams({{ page, per_page: perPage }});
    const response = await fetch(`${{this.baseUrl}}/statutes?${{params}}`);
    return response.ok ? await response.json() : [];
}}
}}

module.exports = {{ StatuteRegistryClient }};
"#,
                config.package_name, config.api_base_url
            ))
        }

        fn generate_typescript(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"// {} TypeScript SDK
export interface Statute {{
id: string;
title: string;
version: number;
status: string;
jurisdiction: string;
}}

export class StatuteRegistryClient {{
private baseUrl: string;

constructor(baseUrl: string = "{}") {{
    this.baseUrl = baseUrl;
}}

async getStatute(statuteId: string): Promise<Statute | null> {{
    const response = await fetch(`${{this.baseUrl}}/statutes/${{statuteId}}`);
    if (response.ok) {{
        return await response.json();
    }}
    return null;
}}

async listStatutes(page: number = 0, perPage: number = 50): Promise<Statute[]> {{
    const params = new URLSearchParams({{ page: page.toString(), per_page: perPage.toString() }});
    const response = await fetch(`${{this.baseUrl}}/statutes?${{params}}`);
    return response.ok ? await response.json() : [];
}}
}}
"#,
                config.package_name, config.api_base_url
            ))
        }

        fn generate_rust(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"// {} Rust SDK
use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statute {{
pub id: String,
pub title: String,
pub version: u32,
pub status: String,
pub jurisdiction: String,
}}

pub struct StatuteRegistryClient {{
base_url: String,
client: reqwest::Client,
}}

impl StatuteRegistryClient {{
pub fn new(base_url: impl Into<String>) -> Self {{
    Self {{
        base_url: base_url.into(),
        client: reqwest::Client::new(),
    }}
}}

pub async fn get_statute(&self, statute_id: &str) -> Result<Option<Statute>, reqwest::Error> {{
    let url = format!("{{}}/statutes/{{}}", self.base_url, statute_id);
    let response = self.client.get(&url).send().await?;
    if response.status().is_success() {{
        Ok(Some(response.json().await?))
    }} else {{
        Ok(None)
    }}
}}

pub async fn list_statutes(&self, page: u32, per_page: u32) -> Result<Vec<Statute>, reqwest::Error> {{
    let url = format!("{{}}/statutes?page={{}}&per_page={{}}", self.base_url, page, per_page);
    let response = self.client.get(&url).send().await?;
    if response.status().is_success() {{
        Ok(response.json().await?)
    }} else {{
        Ok(Vec::new())
    }}
}}
}}
"#,
                config.package_name
            ))
        }

        fn generate_go(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"// {} Go SDK
package {}

import (
"encoding/json"
"fmt"
"net/http"
)

type Statute struct {{
ID           string `json:"id"`
Title        string `json:"title"`
Version      int    `json:"version"`
Status       string `json:"status"`
Jurisdiction string `json:"jurisdiction"`
}}

type Client struct {{
BaseURL    string
HTTPClient *http.Client
}}

func NewClient(baseURL string) *Client {{
return &Client{{
    BaseURL:    baseURL,
    HTTPClient: &http.Client{{}},
}}
}}

func (c *Client) GetStatute(statuteID string) (*Statute, error) {{
url := fmt.Sprintf("%s/statutes/%s", c.BaseURL, statuteID)
resp, err := c.HTTPClient.Get(url)
if err != nil {{
    return nil, err
}}
defer resp.Body.Close()

if resp.StatusCode != http.StatusOK {{
    return nil, nil
}}

var statute Statute
if err := json.NewDecoder(resp.Body).Decode(&statute); err != nil {{
    return nil, err
}}
return &statute, nil
}}
"#,
                config.package_name,
                config.package_name.to_lowercase()
            ))
        }

        fn generate_java(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"// {} Java SDK
package {};

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class StatuteRegistryClient {{
private final String baseUrl;
private final HttpClient client;

public StatuteRegistryClient(String baseUrl) {{
    this.baseUrl = baseUrl;
    this.client = HttpClient.newHttpClient();
}}

public String getStatute(String statuteId) throws IOException, InterruptedException {{
    HttpRequest request = HttpRequest.newBuilder()
        .uri(URI.create(baseUrl + "/statutes/" + statuteId))
        .GET()
        .build();

    HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());
    if (response.statusCode() == 200) {{
        return response.body();
    }}
    return null;
}}
}}
"#,
                config.package_name,
                config.package_name.to_lowercase()
            ))
        }

        fn generate_csharp(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                r#"// {} C# SDK
using System;
using System.Net.Http;
using System.Threading.Tasks;

namespace {}
{{
public class StatuteRegistryClient
{{
    private readonly string baseUrl;
    private readonly HttpClient client;

    public StatuteRegistryClient(string baseUrl)
    {{
        this.baseUrl = baseUrl;
        this.client = new HttpClient();
    }}

    public async Task<string> GetStatuteAsync(string statuteId)
    {{
        var response = await client.GetAsync($"{{baseUrl}}/statutes/{{statuteId}}");
        if (response.IsSuccessStatusCode)
        {{
            return await response.Content.ReadAsStringAsync();
        }}
        return null;
    }}
}}
}}
"#,
                config.package_name, config.package_name
            ))
        }

        fn generate_ruby(config: &SdkConfig) -> Result<String, String> {
            Ok(format!(
                "# {} Ruby SDK\nrequire 'net/http'\nrequire 'json'\n\nmodule {}\n  class StatuteRegistryClient\n    attr_reader :base_url\n\n    def initialize(base_url = \"{}\")\n      @base_url = base_url\n    end\n\n    def get_statute(statute_id)\n      uri = URI(\"#{{@base_url}}/statutes/#{{statute_id}}\")\n      response = Net::HTTP.get_response(uri)\n      JSON.parse(response.body) if response.is_a?(Net::HTTPSuccess)\n    end\n\n    def list_statutes(page = 0, per_page = 50)\n      uri = URI(\"#{{@base_url}}/statutes?page=#{{page}}&per_page=#{{per_page}}\")\n      response = Net::HTTP.get_response(uri)\n      response.is_a?(Net::HTTPSuccess) ? JSON.parse(response.body) : []\n    end\n  end\nend\n",
                config.package_name, config.package_name, config.api_base_url
            ))
        }
    }
}
