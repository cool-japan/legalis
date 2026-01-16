//! Request Mocking implementation
//!
//! This module provides request mocking capabilities including:
//! - Mock response definitions
//! - Request matching rules
//! - Response delays and failures
//! - Mock recording and replay

use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

/// Error types for mocking operations
#[derive(Debug, Error)]
pub enum MockError {
    #[error("Mock error: {0}")]
    Error(String),

    #[error("Mock not found: {0}")]
    MockNotFound(String),

    #[error("Invalid mock rule: {0}")]
    InvalidRule(String),
}

/// Result type for mocking operations
pub type MockResult<T> = Result<T, MockError>;

/// Request matcher for defining mock conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMatcher {
    /// Path pattern (supports wildcards)
    pub path: Option<String>,

    /// HTTP method
    pub method: Option<String>,

    /// Required headers
    pub headers: HashMap<String, String>,

    /// Required query parameters
    pub query_params: HashMap<String, String>,

    /// Body pattern (JSON)
    pub body_pattern: Option<serde_json::Value>,
}

impl RequestMatcher {
    /// Create a new request matcher
    pub fn new() -> Self {
        Self {
            path: None,
            method: None,
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body_pattern: None,
        }
    }

    /// Set path pattern
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    /// Set method
    pub fn with_method(mut self, method: String) -> Self {
        self.method = Some(method);
        self
    }

    /// Add header requirement
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Add query parameter requirement
    pub fn with_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self
    }

    /// Set body pattern
    pub fn with_body_pattern(mut self, pattern: serde_json::Value) -> Self {
        self.body_pattern = Some(pattern);
        self
    }

    /// Check if a request matches this matcher
    pub fn matches(
        &self,
        path: &str,
        method: &str,
        headers: &HeaderMap,
        query: &HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> bool {
        // Check path
        if let Some(pattern) = &self.path {
            if !self.path_matches(pattern, path) {
                return false;
            }
        }

        // Check method
        if let Some(expected_method) = &self.method {
            if expected_method != method {
                return false;
            }
        }

        // Check headers
        for (key, value) in &self.headers {
            if let Some(header_value) = headers.get(key) {
                if header_value.to_str().unwrap_or("") != value {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check query parameters
        for (key, value) in &self.query_params {
            if query.get(key) != Some(value) {
                return false;
            }
        }

        // Check body pattern
        if let Some(pattern) = &self.body_pattern {
            if let Some(request_body) = body {
                if !self.body_matches(pattern, request_body) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn path_matches(&self, pattern: &str, path: &str) -> bool {
        // Simple wildcard matching
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return path.starts_with(parts[0]) && path.ends_with(parts[1]);
            }
        }
        pattern == path
    }

    fn body_matches(&self, _pattern: &serde_json::Value, _body: &serde_json::Value) -> bool {
        // Simple implementation - in real world, this would do deep pattern matching
        true
    }
}

impl Default for RequestMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock response definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponse {
    /// HTTP status code
    pub status: u16,

    /// Response headers
    pub headers: HashMap<String, String>,

    /// Response body
    pub body: serde_json::Value,

    /// Delay before sending response
    pub delay_ms: Option<u64>,

    /// Failure rate (0.0 to 1.0)
    pub failure_rate: f64,
}

impl MockResponse {
    /// Create a new mock response
    pub fn new(status: u16, body: serde_json::Value) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
            delay_ms: None,
            failure_rate: 0.0,
        }
    }

    /// Add header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set delay
    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = Some(delay_ms);
        self
    }

    /// Set failure rate
    pub fn with_failure_rate(mut self, rate: f64) -> Self {
        self.failure_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Check if this response should fail
    pub fn should_fail(&self) -> bool {
        use rand::Rng;
        if self.failure_rate > 0.0 {
            let mut rng = rand::rng();
            rng.random_range(0.0..1.0) < self.failure_rate
        } else {
            false
        }
    }

    /// Get delay duration
    pub fn delay(&self) -> Option<Duration> {
        self.delay_ms.map(Duration::from_millis)
    }
}

/// Mock rule combining matcher and response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockRule {
    /// Rule ID
    pub id: Uuid,

    /// Rule name
    pub name: String,

    /// Request matcher
    pub matcher: RequestMatcher,

    /// Response to return
    pub response: MockResponse,

    /// Priority (higher priority rules are checked first)
    pub priority: i32,

    /// Whether this rule is enabled
    pub enabled: bool,

    /// Number of times this rule has been matched
    #[serde(skip)]
    pub match_count: usize,
}

impl MockRule {
    /// Create a new mock rule
    pub fn new(name: String, matcher: RequestMatcher, response: MockResponse) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            matcher,
            response,
            priority: 0,
            enabled: true,
            match_count: 0,
        }
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Disable this rule
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Mock server for managing mock rules
pub struct MockServer {
    rules: Arc<RwLock<Vec<MockRule>>>,
    recording: Arc<RwLock<bool>>,
    recorded_requests: Arc<RwLock<Vec<RecordedRequest>>>,
}

impl MockServer {
    /// Create a new mock server
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            recording: Arc::new(RwLock::new(false)),
            recorded_requests: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a mock rule
    pub fn add_rule(&self, rule: MockRule) -> MockResult<Uuid> {
        let id = rule.id;
        let mut rules = self
            .rules
            .write()
            .map_err(|e| MockError::Error(format!("Failed to acquire write lock: {}", e)))?;

        rules.push(rule);
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(id)
    }

    /// Remove a mock rule
    pub fn remove_rule(&self, id: Uuid) -> MockResult<()> {
        let mut rules = self
            .rules
            .write()
            .map_err(|e| MockError::Error(format!("Failed to acquire write lock: {}", e)))?;

        let index = rules
            .iter()
            .position(|r| r.id == id)
            .ok_or_else(|| MockError::MockNotFound(id.to_string()))?;

        rules.remove(index);
        Ok(())
    }

    /// Find matching rule for a request
    pub fn find_match(
        &self,
        path: &str,
        method: &str,
        headers: &HeaderMap,
        query: &HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> Option<MockResponse> {
        let mut rules = self.rules.write().unwrap();

        for rule in rules.iter_mut() {
            if rule.enabled && rule.matcher.matches(path, method, headers, query, body) {
                rule.match_count += 1;
                return Some(rule.response.clone());
            }
        }

        None
    }

    /// List all rules
    pub fn list_rules(&self) -> Vec<MockRule> {
        self.rules.read().unwrap().clone()
    }

    /// Clear all rules
    pub fn clear_rules(&self) -> MockResult<()> {
        let mut rules = self
            .rules
            .write()
            .map_err(|e| MockError::Error(format!("Failed to acquire write lock: {}", e)))?;

        rules.clear();
        Ok(())
    }

    /// Start recording requests
    pub fn start_recording(&self) {
        *self.recording.write().unwrap() = true;
    }

    /// Stop recording requests
    pub fn stop_recording(&self) {
        *self.recording.write().unwrap() = false;
    }

    /// Record a request (if recording is enabled)
    pub fn record_request(&self, request: RecordedRequest) {
        if *self.recording.read().unwrap() {
            self.recorded_requests.write().unwrap().push(request);
        }
    }

    /// Get recorded requests
    pub fn get_recorded_requests(&self) -> Vec<RecordedRequest> {
        self.recorded_requests.read().unwrap().clone()
    }

    /// Clear recorded requests
    pub fn clear_recorded_requests(&self) {
        self.recorded_requests.write().unwrap().clear();
    }

    /// Generate mock rules from recorded requests
    pub fn generate_mocks_from_recordings(&self) -> Vec<MockRule> {
        let requests = self.recorded_requests.read().unwrap();
        let mut rules = Vec::new();

        for (i, request) in requests.iter().enumerate() {
            let matcher = RequestMatcher::new()
                .with_path(request.path.clone())
                .with_method(request.method.clone());

            let response =
                MockResponse::new(request.response_status, request.response_body.clone());

            let rule = MockRule::new(format!("Recorded request {}", i + 1), matcher, response);

            rules.push(rule);
        }

        rules
    }
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Recorded request for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedRequest {
    /// Request path
    pub path: String,

    /// HTTP method
    pub method: String,

    /// Request headers
    pub headers: HashMap<String, String>,

    /// Query parameters
    pub query_params: HashMap<String, String>,

    /// Request body
    pub body: Option<serde_json::Value>,

    /// Response status
    pub response_status: u16,

    /// Response body
    pub response_body: serde_json::Value,

    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_matcher() {
        let matcher = RequestMatcher::new()
            .with_path("/api/v1/statutes".to_string())
            .with_method("GET".to_string());

        let headers = HeaderMap::new();
        let query = HashMap::new();

        assert!(matcher.matches("/api/v1/statutes", "GET", &headers, &query, None));
        assert!(!matcher.matches("/api/v1/statutes", "POST", &headers, &query, None));
    }

    #[test]
    fn test_wildcard_path_matching() {
        let matcher = RequestMatcher::new().with_path("/api/v1/*".to_string());

        let headers = HeaderMap::new();
        let query = HashMap::new();

        assert!(matcher.matches("/api/v1/statutes", "GET", &headers, &query, None));
        assert!(matcher.matches("/api/v1/users", "GET", &headers, &query, None));
        assert!(!matcher.matches("/api/v2/statutes", "GET", &headers, &query, None));
    }

    #[test]
    fn test_mock_response_creation() {
        let response = MockResponse::new(200, serde_json::json!({"status": "ok"}))
            .with_header("Content-Type".to_string(), "application/json".to_string())
            .with_delay(100);

        assert_eq!(response.status, 200);
        assert_eq!(response.delay_ms, Some(100));
    }

    #[test]
    fn test_mock_server() {
        let server = MockServer::new();

        let matcher = RequestMatcher::new()
            .with_path("/test".to_string())
            .with_method("GET".to_string());

        let response = MockResponse::new(200, serde_json::json!({"result": "mocked"}));

        let rule = MockRule::new("Test rule".to_string(), matcher, response);
        let id = server.add_rule(rule).unwrap();

        let headers = HeaderMap::new();
        let query = HashMap::new();

        let matched = server.find_match("/test", "GET", &headers, &query, None);
        assert!(matched.is_some());

        server.remove_rule(id).unwrap();
        let matched = server.find_match("/test", "GET", &headers, &query, None);
        assert!(matched.is_none());
    }

    #[test]
    fn test_rule_priority() {
        let server = MockServer::new();

        let rule1 = MockRule::new(
            "Low priority".to_string(),
            RequestMatcher::new().with_path("/test".to_string()),
            MockResponse::new(200, serde_json::json!({"priority": "low"})),
        )
        .with_priority(1);

        let rule2 = MockRule::new(
            "High priority".to_string(),
            RequestMatcher::new().with_path("/test".to_string()),
            MockResponse::new(200, serde_json::json!({"priority": "high"})),
        )
        .with_priority(10);

        server.add_rule(rule1).unwrap();
        server.add_rule(rule2).unwrap();

        let headers = HeaderMap::new();
        let query = HashMap::new();

        let matched = server
            .find_match("/test", "GET", &headers, &query, None)
            .unwrap();
        assert_eq!(matched.body["priority"], "high");
    }

    #[test]
    fn test_recording() {
        let server = MockServer::new();

        server.start_recording();

        let request = RecordedRequest {
            path: "/test".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            response_status: 200,
            response_body: serde_json::json!({"status": "ok"}),
            timestamp: chrono::Utc::now(),
        };

        server.record_request(request);

        let recorded = server.get_recorded_requests();
        assert_eq!(recorded.len(), 1);

        server.stop_recording();
        server.clear_recorded_requests();

        let recorded = server.get_recorded_requests();
        assert_eq!(recorded.len(), 0);
    }
}
