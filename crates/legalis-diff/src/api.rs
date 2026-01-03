//! REST API structures for diff operations.
//!
//! This module provides data structures and types for exposing diff operations
//! through a REST API, including request/response models and error handling.
//!
//! # Examples
//!
//! ```
//! use legalis_diff::api::{DiffRequest, DiffResponse};
//! use serde_json;
//!
//! // Create a diff request
//! let request = DiffRequest {
//!     statute_id: "tax-law-123".to_string(),
//!     old_version: "v1".to_string(),
//!     new_version: "v2".to_string(),
//!     options: Default::default(),
//! };
//!
//! // Serialize to JSON for API transmission
//! let json = serde_json::to_string(&request).unwrap();
//! ```

use crate::{Severity, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to compute a diff between two statute versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffRequest {
    /// The statute ID.
    pub statute_id: String,
    /// Old version identifier.
    pub old_version: String,
    /// New version identifier.
    pub new_version: String,
    /// Optional diff options.
    #[serde(default)]
    pub options: DiffOptions,
}

/// Options for diff computation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffOptions {
    /// Include semantic analysis.
    #[serde(default)]
    pub include_semantic: bool,
    /// Include natural language summary.
    #[serde(default)]
    pub include_nlp_summary: bool,
    /// Include recommendations.
    #[serde(default)]
    pub include_recommendations: bool,
    /// Generate visualizations.
    #[serde(default)]
    pub generate_visualizations: bool,
    /// Additional metadata.
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Response containing diff results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResponse {
    /// The computed diff.
    pub diff: StatuteDiff,
    /// Natural language summary (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlp_summary: Option<String>,
    /// Recommendations (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,
    /// Visualization URLs or data (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualizations: Option<HashMap<String, String>>,
    /// Response metadata.
    #[serde(default)]
    pub metadata: ResponseMetadata,
}

/// Metadata about the API response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Time taken to compute the diff (in milliseconds).
    pub computation_time_ms: Option<u64>,
    /// API version.
    pub api_version: String,
    /// Timestamp of the response.
    pub timestamp: String,
}

/// Request to compute batch diffs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDiffRequest {
    /// List of diff requests.
    pub diffs: Vec<DiffRequest>,
    /// Batch options.
    #[serde(default)]
    pub options: BatchOptions,
}

/// Options for batch diff computation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchOptions {
    /// Enable parallel processing.
    #[serde(default)]
    pub parallel: bool,
    /// Maximum number of concurrent diffs.
    pub max_concurrency: Option<usize>,
    /// Stop on first error.
    #[serde(default)]
    pub fail_fast: bool,
}

/// Response for batch diff operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDiffResponse {
    /// Successful diff results.
    pub results: Vec<DiffResponse>,
    /// Errors that occurred.
    pub errors: Vec<DiffError>,
    /// Batch metadata.
    pub metadata: BatchMetadata,
}

/// Metadata for batch operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMetadata {
    /// Total number of diffs requested.
    pub total_requested: usize,
    /// Number of successful diffs.
    pub successful: usize,
    /// Number of failed diffs.
    pub failed: usize,
    /// Total computation time (in milliseconds).
    pub total_time_ms: u64,
}

/// API error information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffError {
    /// Error code.
    pub code: String,
    /// Error message.
    pub message: String,
    /// Statute ID (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statute_id: Option<String>,
    /// Additional error details.
    #[serde(default)]
    pub details: HashMap<String, String>,
}

impl DiffError {
    /// Creates a new API error.
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            statute_id: None,
            details: HashMap::new(),
        }
    }

    /// Sets the statute ID for the error.
    pub fn with_statute_id(mut self, statute_id: &str) -> Self {
        self.statute_id = Some(statute_id.to_string());
        self
    }

    /// Adds a detail field to the error.
    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.to_string(), value.to_string());
        self
    }
}

/// Request to analyze diff impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisRequest {
    /// The statute ID.
    pub statute_id: String,
    /// Version identifiers.
    pub versions: Vec<String>,
    /// Analysis options.
    #[serde(default)]
    pub options: ImpactAnalysisOptions,
}

/// Options for impact analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImpactAnalysisOptions {
    /// Include stakeholder analysis.
    #[serde(default)]
    pub include_stakeholders: bool,
    /// Include compliance impact.
    #[serde(default)]
    pub include_compliance: bool,
    /// Include migration effort estimation.
    #[serde(default)]
    pub include_migration_effort: bool,
}

/// Response for impact analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisResponse {
    /// Statute ID.
    pub statute_id: String,
    /// Overall impact severity.
    pub severity: Severity,
    /// Impact score (0-100).
    pub impact_score: f64,
    /// Affected areas.
    pub affected_areas: Vec<String>,
    /// Stakeholder impacts (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stakeholder_impacts: Option<Vec<StakeholderImpact>>,
    /// Compliance impacts (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_impacts: Option<Vec<ComplianceImpact>>,
    /// Migration effort (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_effort: Option<MigrationEffort>,
}

/// Impact on a stakeholder group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderImpact {
    /// Stakeholder group name.
    pub group: String,
    /// Impact description.
    pub description: String,
    /// Impact level (0-10).
    pub impact_level: u8,
}

/// Compliance impact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceImpact {
    /// Compliance framework or regulation.
    pub framework: String,
    /// Impact description.
    pub description: String,
    /// Risk level.
    pub risk_level: RiskLevel,
}

/// Risk level for compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Critical risk.
    Critical,
}

/// Migration effort estimation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationEffort {
    /// Effort level (Low, Medium, High).
    pub level: String,
    /// Estimated person-days.
    pub estimated_days: Option<f64>,
    /// Required changes.
    pub required_changes: Vec<String>,
    /// Rollback difficulty (0-10).
    pub rollback_difficulty: u8,
}

/// Webhook notification for diff events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotification {
    /// Event type.
    pub event_type: WebhookEventType,
    /// Statute ID.
    pub statute_id: String,
    /// Event payload.
    pub payload: serde_json::Value,
    /// Timestamp.
    pub timestamp: String,
}

/// Types of webhook events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// A new diff was computed.
    DiffComputed,
    /// A diff was approved.
    DiffApproved,
    /// A diff was rejected.
    DiffRejected,
    /// A comment was added.
    CommentAdded,
    /// A vote was cast.
    VoteCast,
}

/// Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Service status.
    pub status: ServiceStatus,
    /// Service version.
    pub version: String,
    /// Uptime in seconds.
    pub uptime_seconds: u64,
    /// Component statuses.
    pub components: HashMap<String, ComponentStatus>,
}

/// Service status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// Service is healthy.
    Healthy,
    /// Service is degraded.
    Degraded,
    /// Service is unhealthy.
    Unhealthy,
}

/// Component status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// Status of the component.
    pub status: ServiceStatus,
    /// Status message.
    pub message: Option<String>,
}

/// Authentication request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Authentication method.
    pub auth_method: AuthMethod,
    /// Credentials.
    pub credentials: AuthCredentials,
}

/// Authentication method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    /// API key authentication.
    ApiKey,
    /// OAuth 2.0.
    OAuth2,
    /// JWT token.
    Jwt,
    /// Basic authentication.
    Basic,
}

/// Authentication credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthCredentials {
    /// API key.
    ApiKey { key: String },
    /// OAuth 2.0 token.
    OAuth2 { token: String },
    /// JWT token.
    Jwt { token: String },
    /// Basic auth.
    Basic { username: String, password: String },
}

/// Authentication response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Whether authentication was successful.
    pub authenticated: bool,
    /// Access token (if successful).
    pub access_token: Option<String>,
    /// Token expiration time.
    pub expires_in: Option<u64>,
    /// Refresh token.
    pub refresh_token: Option<String>,
    /// Error message (if failed).
    pub error: Option<String>,
}

/// Authorization check request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// User or service identifier.
    pub subject: String,
    /// Resource being accessed.
    pub resource: String,
    /// Action being performed.
    pub action: Action,
}

/// Actions that can be performed on resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Read access.
    Read,
    /// Write access.
    Write,
    /// Delete access.
    Delete,
    /// Execute access.
    Execute,
    /// Admin access.
    Admin,
}

/// Authorization response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    /// Whether the action is authorized.
    pub authorized: bool,
    /// Reason (if not authorized).
    pub reason: Option<String>,
    /// Required permissions.
    pub required_permissions: Vec<String>,
}

/// API rate limit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Maximum requests allowed.
    pub limit: u32,
    /// Remaining requests in current window.
    pub remaining: u32,
    /// Time until rate limit resets (seconds).
    pub reset_in: u64,
    /// Rate limit window duration (seconds).
    pub window: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_request_serialization() {
        let request = DiffRequest {
            statute_id: "test-law".to_string(),
            old_version: "v1".to_string(),
            new_version: "v2".to_string(),
            options: DiffOptions::default(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: DiffRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.statute_id, deserialized.statute_id);
        assert_eq!(request.old_version, deserialized.old_version);
    }

    #[test]
    fn test_diff_error_builder() {
        let error = DiffError::new("NOT_FOUND", "Statute not found")
            .with_statute_id("test-law")
            .with_detail("version", "v1");

        assert_eq!(error.code, "NOT_FOUND");
        assert_eq!(error.statute_id, Some("test-law".to_string()));
        assert_eq!(error.details.get("version"), Some(&"v1".to_string()));
    }

    #[test]
    fn test_batch_diff_request() {
        let request = BatchDiffRequest {
            diffs: vec![DiffRequest {
                statute_id: "law1".to_string(),
                old_version: "v1".to_string(),
                new_version: "v2".to_string(),
                options: DiffOptions::default(),
            }],
            options: BatchOptions {
                parallel: true,
                max_concurrency: Some(4),
                fail_fast: false,
            },
        };

        assert_eq!(request.diffs.len(), 1);
        assert!(request.options.parallel);
    }

    #[test]
    fn test_webhook_notification() {
        let notification = WebhookNotification {
            event_type: WebhookEventType::DiffComputed,
            statute_id: "test-law".to_string(),
            payload: serde_json::json!({"diff_id": "12345"}),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&notification).unwrap();
        assert!(json.contains("diff_computed"));
    }

    #[test]
    fn test_health_check_response() {
        let mut components = HashMap::new();
        components.insert(
            "database".to_string(),
            ComponentStatus {
                status: ServiceStatus::Healthy,
                message: Some("Connected".to_string()),
            },
        );

        let response = HealthCheckResponse {
            status: ServiceStatus::Healthy,
            version: "0.2.0".to_string(),
            uptime_seconds: 3600,
            components,
        };

        assert_eq!(response.status, ServiceStatus::Healthy);
        assert_eq!(response.components.len(), 1);
    }
}
