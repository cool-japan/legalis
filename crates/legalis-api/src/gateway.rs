//! API Gateway features.
//!
//! This module provides API gateway capabilities including request/response transformation,
//! circuit breakers, load balancing, and service mesh integration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

// ============================================================================
// Request Transformation
// ============================================================================

/// Request transformation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTransform {
    /// Transform ID
    pub id: String,
    /// Path pattern to match (glob-style)
    pub path_pattern: String,
    /// Header transformations
    pub header_transforms: Vec<HeaderTransform>,
    /// Query parameter transformations
    pub query_transforms: Vec<QueryTransform>,
    /// Body transformations
    pub body_transforms: Vec<BodyTransform>,
}

/// Header transformation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HeaderTransform {
    /// Add a header
    Add { name: String, value: String },
    /// Remove a header
    Remove { name: String },
    /// Rename a header
    Rename { from: String, to: String },
    /// Set header value (overwrite if exists)
    Set { name: String, value: String },
}

/// Query parameter transformation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueryTransform {
    /// Add a query parameter
    Add { name: String, value: String },
    /// Remove a query parameter
    Remove { name: String },
    /// Rename a query parameter
    Rename { from: String, to: String },
}

/// Body transformation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BodyTransform {
    /// JSON path transformation
    JsonPath {
        path: String,
        value: serde_json::Value,
    },
    /// Template-based transformation
    Template { template: String },
}

/// Request transformer.
pub struct RequestTransformer {
    transforms: Arc<RwLock<Vec<RequestTransform>>>,
}

impl RequestTransformer {
    /// Create a new request transformer.
    pub fn new() -> Self {
        Self {
            transforms: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a transformation rule.
    pub async fn add_transform(&self, transform: RequestTransform) {
        let mut transforms = self.transforms.write().await;
        transforms.push(transform);
    }

    /// Remove a transformation rule.
    pub async fn remove_transform(&self, id: &str) -> bool {
        let mut transforms = self.transforms.write().await;
        let len_before = transforms.len();
        transforms.retain(|t| t.id != id);
        transforms.len() < len_before
    }

    /// Get all transformation rules.
    pub async fn get_transforms(&self) -> Vec<RequestTransform> {
        let transforms = self.transforms.read().await;
        transforms.clone()
    }

    /// Apply transformations to headers.
    pub fn apply_header_transforms(
        &self,
        headers: &mut HashMap<String, String>,
        transforms: &[HeaderTransform],
    ) {
        for transform in transforms {
            match transform {
                HeaderTransform::Add { name, value } => {
                    headers.entry(name.clone()).or_insert_with(|| value.clone());
                }
                HeaderTransform::Remove { name } => {
                    headers.remove(name);
                }
                HeaderTransform::Rename { from, to } => {
                    if let Some(value) = headers.remove(from) {
                        headers.insert(to.clone(), value);
                    }
                }
                HeaderTransform::Set { name, value } => {
                    headers.insert(name.clone(), value.clone());
                }
            }
        }
    }
}

impl Default for RequestTransformer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Response Transformation
// ============================================================================

/// Response transformation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTransform {
    /// Transform ID
    pub id: String,
    /// Status code pattern to match
    pub status_pattern: Option<u16>,
    /// Header transformations
    pub header_transforms: Vec<HeaderTransform>,
    /// Body transformations
    pub body_transforms: Vec<BodyTransform>,
}

/// Response transformer.
pub struct ResponseTransformer {
    transforms: Arc<RwLock<Vec<ResponseTransform>>>,
}

impl ResponseTransformer {
    /// Create a new response transformer.
    pub fn new() -> Self {
        Self {
            transforms: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a transformation rule.
    pub async fn add_transform(&self, transform: ResponseTransform) {
        let mut transforms = self.transforms.write().await;
        transforms.push(transform);
    }

    /// Remove a transformation rule.
    pub async fn remove_transform(&self, id: &str) -> bool {
        let mut transforms = self.transforms.write().await;
        let len_before = transforms.len();
        transforms.retain(|t| t.id != id);
        transforms.len() < len_before
    }

    /// Get all transformation rules.
    pub async fn get_transforms(&self) -> Vec<ResponseTransform> {
        let transforms = self.transforms.read().await;
        transforms.clone()
    }
}

impl Default for ResponseTransformer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Circuit Breaker
// ============================================================================

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Circuit is closed, requests are allowed
    Closed,
    /// Circuit is open, requests are rejected
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: usize,
    /// Success threshold to close circuit (in half-open state)
    pub success_threshold: usize,
    /// Timeout before trying half-open (ms)
    pub timeout_ms: u64,
    /// Window size for tracking failures
    pub window_size: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout_ms: 60000, // 1 minute
            window_size: 10,
        }
    }
}

/// Circuit breaker for protecting against cascading failures.
pub struct CircuitBreaker {
    /// Service name
    name: String,
    /// Current state
    state: Arc<RwLock<CircuitState>>,
    /// Failure count
    failures: Arc<RwLock<Vec<Instant>>>,
    /// Success count (in half-open state)
    successes: Arc<RwLock<usize>>,
    /// Time when circuit was opened
    opened_at: Arc<RwLock<Option<Instant>>>,
    /// Configuration
    config: CircuitBreakerConfig,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    pub fn new(name: String, config: CircuitBreakerConfig) -> Self {
        Self {
            name,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failures: Arc::new(RwLock::new(Vec::new())),
            successes: Arc::new(RwLock::new(0)),
            opened_at: Arc::new(RwLock::new(None)),
            config,
        }
    }

    /// Check if request should be allowed.
    pub async fn allow_request(&self) -> Result<(), CircuitBreakerError> {
        let mut state = self.state.write().await;

        match *state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                // Check if timeout has elapsed
                let opened_at = self.opened_at.read().await;
                if let Some(opened_time) = *opened_at {
                    if opened_time.elapsed().as_millis() as u64 > self.config.timeout_ms {
                        // Try half-open
                        *state = CircuitState::HalfOpen;
                        drop(state);
                        drop(opened_at);
                        let mut successes = self.successes.write().await;
                        *successes = 0;
                        Ok(())
                    } else {
                        Err(CircuitBreakerError::CircuitOpen {
                            service: self.name.clone(),
                        })
                    }
                } else {
                    Err(CircuitBreakerError::CircuitOpen {
                        service: self.name.clone(),
                    })
                }
            }
            CircuitState::HalfOpen => Ok(()),
        }
    }

    /// Record a successful request.
    pub async fn record_success(&self) {
        let mut state = self.state.write().await;

        match *state {
            CircuitState::HalfOpen => {
                let mut successes = self.successes.write().await;
                *successes += 1;

                if *successes >= self.config.success_threshold {
                    // Close the circuit
                    *state = CircuitState::Closed;
                    *successes = 0;
                    let mut failures = self.failures.write().await;
                    failures.clear();
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                let mut failures = self.failures.write().await;
                failures.clear();
            }
            CircuitState::Open => {}
        }
    }

    /// Record a failed request.
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        let mut failures = self.failures.write().await;

        // Add failure to window
        failures.push(Instant::now());

        // Keep only recent failures within window
        if failures.len() > self.config.window_size {
            failures.remove(0);
        }

        // Count recent failures
        let recent_failures = failures.len();

        match *state {
            CircuitState::Closed => {
                if recent_failures >= self.config.failure_threshold {
                    // Open the circuit
                    *state = CircuitState::Open;
                    let mut opened_at = self.opened_at.write().await;
                    *opened_at = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                // Any failure in half-open reopens the circuit
                *state = CircuitState::Open;
                let mut opened_at = self.opened_at.write().await;
                *opened_at = Some(Instant::now());
                let mut successes = self.successes.write().await;
                *successes = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Get current circuit state.
    pub async fn get_state(&self) -> CircuitState {
        *self.state.read().await
    }

    /// Get circuit breaker statistics.
    pub async fn get_stats(&self) -> CircuitBreakerStats {
        let state = *self.state.read().await;
        let failures = self.failures.read().await;
        let successes = *self.successes.read().await;

        CircuitBreakerStats {
            name: self.name.clone(),
            state,
            recent_failures: failures.len(),
            consecutive_successes: successes,
        }
    }
}

/// Circuit breaker statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerStats {
    /// Service name
    pub name: String,
    /// Current state
    pub state: CircuitState,
    /// Number of recent failures
    pub recent_failures: usize,
    /// Consecutive successes (in half-open state)
    pub consecutive_successes: usize,
}

/// Circuit breaker errors.
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum CircuitBreakerError {
    #[error("Circuit breaker is open for service: {service}")]
    CircuitOpen { service: String },
}

// ============================================================================
// Load Balancer
// ============================================================================

/// Load balancing strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Random selection
    Random,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
}

/// Backend server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backend {
    /// Server ID
    pub id: String,
    /// Server address
    pub address: String,
    /// Server weight (for weighted strategies)
    pub weight: u32,
    /// Whether server is healthy
    pub healthy: bool,
    /// Current connection count
    pub connections: usize,
}

impl Backend {
    /// Create a new backend.
    pub fn new(id: String, address: String) -> Self {
        Self {
            id,
            address,
            weight: 1,
            healthy: true,
            connections: 0,
        }
    }

    /// Set weight for weighted load balancing.
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }
}

/// Load balancer.
pub struct LoadBalancer {
    /// Backends
    backends: Arc<RwLock<Vec<Backend>>>,
    /// Load balancing strategy
    strategy: LoadBalancingStrategy,
    /// Current index (for round-robin)
    current_index: Arc<RwLock<usize>>,
}

impl LoadBalancer {
    /// Create a new load balancer.
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            backends: Arc::new(RwLock::new(Vec::new())),
            strategy,
            current_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Add a backend server.
    pub async fn add_backend(&self, backend: Backend) {
        let mut backends = self.backends.write().await;
        backends.push(backend);
    }

    /// Remove a backend server.
    pub async fn remove_backend(&self, id: &str) -> bool {
        let mut backends = self.backends.write().await;
        let len_before = backends.len();
        backends.retain(|b| b.id != id);
        backends.len() < len_before
    }

    /// Select a backend using the configured strategy.
    pub async fn select_backend(&self) -> Result<String, LoadBalancerError> {
        let backends = self.backends.read().await;
        let healthy_backends: Vec<_> = backends.iter().filter(|b| b.healthy).collect();

        if healthy_backends.is_empty() {
            return Err(LoadBalancerError::NoHealthyBackends);
        }

        let selected = match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                let mut index = self.current_index.write().await;
                let backend = &healthy_backends[*index % healthy_backends.len()];
                *index = (*index + 1) % healthy_backends.len();
                backend
            }
            LoadBalancingStrategy::Random => {
                use rand::Rng;
                let idx = rand::rng().random_range(0..healthy_backends.len());
                healthy_backends[idx]
            }
            LoadBalancingStrategy::LeastConnections => healthy_backends
                .iter()
                .min_by_key(|b| b.connections)
                .unwrap(),
            LoadBalancingStrategy::WeightedRoundRobin => {
                // Simplified weighted selection
                let total_weight: u32 = healthy_backends.iter().map(|b| b.weight).sum();
                let mut index = self.current_index.write().await;
                let target = (*index as u32) % total_weight;
                *index += 1;

                let mut cumulative = 0u32;
                healthy_backends
                    .iter()
                    .find(|b| {
                        cumulative += b.weight;
                        cumulative > target
                    })
                    .unwrap()
            }
        };

        Ok(selected.address.clone())
    }

    /// Mark a backend as healthy or unhealthy.
    pub async fn set_backend_health(
        &self,
        id: &str,
        healthy: bool,
    ) -> Result<(), LoadBalancerError> {
        let mut backends = self.backends.write().await;
        let backend = backends
            .iter_mut()
            .find(|b| b.id == id)
            .ok_or_else(|| LoadBalancerError::BackendNotFound { id: id.to_string() })?;
        backend.healthy = healthy;
        Ok(())
    }

    /// Get all backends.
    pub async fn get_backends(&self) -> Vec<Backend> {
        let backends = self.backends.read().await;
        backends.clone()
    }

    /// Get load balancer statistics.
    pub async fn get_stats(&self) -> LoadBalancerStats {
        let backends = self.backends.read().await;
        let total_backends = backends.len();
        let healthy_backends = backends.iter().filter(|b| b.healthy).count();

        LoadBalancerStats {
            strategy: self.strategy,
            total_backends,
            healthy_backends,
        }
    }
}

/// Load balancer statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    /// Total number of backends
    pub total_backends: usize,
    /// Number of healthy backends
    pub healthy_backends: usize,
}

/// Load balancer errors.
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum LoadBalancerError {
    #[error("No healthy backends available")]
    NoHealthyBackends,

    #[error("Backend not found: {id}")]
    BackendNotFound { id: String },
}

// ============================================================================
// Service Mesh Integration
// ============================================================================

/// Service mesh configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    /// Service name
    pub service_name: String,
    /// Service namespace
    pub namespace: String,
    /// Mesh type (e.g., "istio", "linkerd", "consul")
    pub mesh_type: String,
    /// Enable mutual TLS
    pub enable_mtls: bool,
    /// Enable distributed tracing
    pub enable_tracing: bool,
}

impl Default for ServiceMeshConfig {
    fn default() -> Self {
        Self {
            service_name: "legalis-api".to_string(),
            namespace: "default".to_string(),
            mesh_type: "istio".to_string(),
            enable_mtls: true,
            enable_tracing: true,
        }
    }
}

/// Service mesh integration.
pub struct ServiceMesh {
    config: ServiceMeshConfig,
    /// Service discovery registry
    services: Arc<RwLock<HashMap<String, ServiceEndpoint>>>,
}

impl ServiceMesh {
    /// Create a new service mesh integration.
    pub fn new(config: ServiceMeshConfig) -> Self {
        Self {
            config,
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service in the mesh.
    pub async fn register_service(&self, service: ServiceEndpoint) {
        let mut services = self.services.write().await;
        services.insert(service.name.clone(), service);
    }

    /// Discover a service by name.
    pub async fn discover_service(&self, name: &str) -> Option<ServiceEndpoint> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    /// Get all registered services.
    pub async fn get_services(&self) -> Vec<ServiceEndpoint> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    /// Get mesh configuration.
    pub fn get_config(&self) -> &ServiceMeshConfig {
        &self.config
    }
}

/// Service endpoint in the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Service name
    pub name: String,
    /// Service address
    pub address: String,
    /// Service port
    pub port: u16,
    /// Service metadata
    pub metadata: HashMap<String, String>,
}

impl ServiceEndpoint {
    /// Create a new service endpoint.
    pub fn new(name: String, address: String, port: u16) -> Self {
        Self {
            name,
            address,
            port,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the service.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Request Transformation Tests
    #[tokio::test]
    async fn test_request_transformer_add() {
        let transformer = RequestTransformer::new();
        let transform = RequestTransform {
            id: "test".to_string(),
            path_pattern: "/api/*".to_string(),
            header_transforms: vec![],
            query_transforms: vec![],
            body_transforms: vec![],
        };

        transformer.add_transform(transform).await;
        let transforms = transformer.get_transforms().await;
        assert_eq!(transforms.len(), 1);
    }

    #[tokio::test]
    async fn test_request_transformer_remove() {
        let transformer = RequestTransformer::new();
        let transform = RequestTransform {
            id: "test".to_string(),
            path_pattern: "/api/*".to_string(),
            header_transforms: vec![],
            query_transforms: vec![],
            body_transforms: vec![],
        };

        transformer.add_transform(transform).await;
        let removed = transformer.remove_transform("test").await;
        assert!(removed);

        let transforms = transformer.get_transforms().await;
        assert_eq!(transforms.len(), 0);
    }

    #[test]
    fn test_header_transform_add() {
        let transformer = RequestTransformer::new();
        let mut headers = HashMap::new();
        let transforms = vec![HeaderTransform::Add {
            name: "X-Custom".to_string(),
            value: "test".to_string(),
        }];

        transformer.apply_header_transforms(&mut headers, &transforms);
        assert_eq!(headers.get("X-Custom"), Some(&"test".to_string()));
    }

    #[test]
    fn test_header_transform_remove() {
        let transformer = RequestTransformer::new();
        let mut headers = HashMap::new();
        headers.insert("X-Remove".to_string(), "value".to_string());

        let transforms = vec![HeaderTransform::Remove {
            name: "X-Remove".to_string(),
        }];

        transformer.apply_header_transforms(&mut headers, &transforms);
        assert!(!headers.contains_key("X-Remove"));
    }

    #[test]
    fn test_header_transform_rename() {
        let transformer = RequestTransformer::new();
        let mut headers = HashMap::new();
        headers.insert("Old-Name".to_string(), "value".to_string());

        let transforms = vec![HeaderTransform::Rename {
            from: "Old-Name".to_string(),
            to: "New-Name".to_string(),
        }];

        transformer.apply_header_transforms(&mut headers, &transforms);
        assert!(!headers.contains_key("Old-Name"));
        assert_eq!(headers.get("New-Name"), Some(&"value".to_string()));
    }

    // Response Transformation Tests
    #[tokio::test]
    async fn test_response_transformer() {
        let transformer = ResponseTransformer::new();
        let transform = ResponseTransform {
            id: "test".to_string(),
            status_pattern: Some(200),
            header_transforms: vec![],
            body_transforms: vec![],
        };

        transformer.add_transform(transform).await;
        let transforms = transformer.get_transforms().await;
        assert_eq!(transforms.len(), 1);
    }

    // Circuit Breaker Tests
    #[tokio::test]
    async fn test_circuit_breaker_initial_state() {
        let config = CircuitBreakerConfig::default();
        let breaker = CircuitBreaker::new("test-service".to_string(), config);

        assert_eq!(breaker.get_state().await, CircuitState::Closed);
        assert!(breaker.allow_request().await.is_ok());
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_on_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let breaker = CircuitBreaker::new("test-service".to_string(), config);

        // Record failures
        for _ in 0..3 {
            breaker.record_failure().await;
        }

        assert_eq!(breaker.get_state().await, CircuitState::Open);
        assert!(breaker.allow_request().await.is_err());
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout_ms: 100, // Short timeout for testing
            ..Default::default()
        };
        let breaker = CircuitBreaker::new("test-service".to_string(), config);

        // Open the circuit
        breaker.record_failure().await;
        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Open);

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should transition to half-open
        assert!(breaker.allow_request().await.is_ok());
        assert_eq!(breaker.get_state().await, CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_circuit_breaker_closes_on_success() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout_ms: 100,
            ..Default::default()
        };
        let breaker = CircuitBreaker::new("test-service".to_string(), config);

        // Open the circuit
        breaker.record_failure().await;
        breaker.record_failure().await;

        // Wait and transition to half-open
        tokio::time::sleep(Duration::from_millis(150)).await;
        let _ = breaker.allow_request().await;

        // Record successes
        breaker.record_success().await;
        breaker.record_success().await;

        assert_eq!(breaker.get_state().await, CircuitState::Closed);
    }

    // Load Balancer Tests
    #[tokio::test]
    async fn test_load_balancer_round_robin() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_backend(Backend::new(
            "backend1".to_string(),
            "http://localhost:8001".to_string(),
        ))
        .await;
        lb.add_backend(Backend::new(
            "backend2".to_string(),
            "http://localhost:8002".to_string(),
        ))
        .await;

        let first = lb.select_backend().await.unwrap();
        let second = lb.select_backend().await.unwrap();

        assert_ne!(first, second);
    }

    #[tokio::test]
    async fn test_load_balancer_no_backends() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        let result = lb.select_backend().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_balancer_health_check() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_backend(Backend::new(
            "backend1".to_string(),
            "http://localhost:8001".to_string(),
        ))
        .await;

        lb.set_backend_health("backend1", false).await.unwrap();

        let result = lb.select_backend().await;
        assert!(result.is_err());

        lb.set_backend_health("backend1", true).await.unwrap();
        let result = lb.select_backend().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_load_balancer_remove_backend() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_backend(Backend::new(
            "backend1".to_string(),
            "http://localhost:8001".to_string(),
        ))
        .await;

        let removed = lb.remove_backend("backend1").await;
        assert!(removed);

        let backends = lb.get_backends().await;
        assert_eq!(backends.len(), 0);
    }

    // Service Mesh Tests
    #[tokio::test]
    async fn test_service_mesh_register() {
        let config = ServiceMeshConfig::default();
        let mesh = ServiceMesh::new(config);

        let endpoint =
            ServiceEndpoint::new("test-service".to_string(), "localhost".to_string(), 8080);

        mesh.register_service(endpoint).await;

        let discovered = mesh.discover_service("test-service").await;
        assert!(discovered.is_some());
        assert_eq!(discovered.unwrap().port, 8080);
    }

    #[tokio::test]
    async fn test_service_mesh_discover_nonexistent() {
        let config = ServiceMeshConfig::default();
        let mesh = ServiceMesh::new(config);

        let discovered = mesh.discover_service("nonexistent").await;
        assert!(discovered.is_none());
    }

    #[tokio::test]
    async fn test_service_endpoint_with_metadata() {
        let endpoint = ServiceEndpoint::new("test".to_string(), "localhost".to_string(), 8080)
            .with_metadata("version".to_string(), "1.0.0".to_string());

        assert_eq!(endpoint.metadata.get("version"), Some(&"1.0.0".to_string()));
    }

    #[tokio::test]
    async fn test_circuit_breaker_stats() {
        let config = CircuitBreakerConfig::default();
        let breaker = CircuitBreaker::new("test".to_string(), config);

        breaker.record_failure().await;
        let stats = breaker.get_stats().await;

        assert_eq!(stats.name, "test");
        assert_eq!(stats.state, CircuitState::Closed);
        assert_eq!(stats.recent_failures, 1);
    }

    #[tokio::test]
    async fn test_load_balancer_stats() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_backend(Backend::new(
            "backend1".to_string(),
            "http://localhost:8001".to_string(),
        ))
        .await;

        let stats = lb.get_stats().await;
        assert_eq!(stats.total_backends, 1);
        assert_eq!(stats.healthy_backends, 1);
        assert_eq!(stats.strategy, LoadBalancingStrategy::RoundRobin);
    }
}
