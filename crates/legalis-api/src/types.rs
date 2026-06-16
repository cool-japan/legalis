//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Statute version list response.
#[derive(Serialize)]
pub struct StatuteVersionListResponse {
    pub base_id: String,
    pub versions: Vec<StatuteVersionInfo>,
    pub total_versions: usize,
}
/// Conflict detection response.
#[derive(Serialize)]
pub struct ConflictDetectionResponse {
    pub conflicts: Vec<ConflictInfo>,
    pub conflict_count: usize,
}
/// Statute comparison response.
#[derive(Serialize)]
pub struct StatuteComparisonResponse {
    pub statute_a: StatuteSummary,
    pub statute_b: StatuteSummary,
    pub differences: ComparisonDifferences,
    pub similarity_score: f64,
}
/// Statute list response.
#[derive(Serialize)]
pub struct StatuteListResponse {
    pub statutes: Vec<StatuteSummary>,
}
/// Statute comparison matrix request.
#[derive(Deserialize)]
pub struct StatuteComparisonMatrixRequest {
    /// List of statute IDs to compare
    pub statute_ids: Vec<String>,
}
/// Verification response.
#[derive(Serialize)]
pub struct VerifyResponse {
    pub passed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
/// API key response (with the actual key value shown only once).
#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub key: Option<String>,
    pub name: String,
    pub role: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub active: bool,
    pub last_used_at: Option<String>,
}
/// Search/filter parameters for statutes.
#[derive(Deserialize)]
pub struct StatuteSearchQuery {
    /// Search by title (case-insensitive substring match)
    pub title: Option<String>,
    /// Filter by whether statute has discretion
    pub has_discretion: Option<bool>,
    /// Filter by minimum number of preconditions
    pub min_preconditions: Option<usize>,
    /// Filter by maximum number of preconditions
    pub max_preconditions: Option<usize>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
    /// Cursor for cursor-based pagination
    pub cursor: Option<String>,
    /// Field selection (comma-separated list of fields)
    pub fields: Option<String>,
}
/// Response metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseMeta {
    pub total: Option<usize>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
    pub has_more: Option<bool>,
}
/// API errors.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid request: {0}")]
    BadRequest(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error(transparent)]
    Auth(#[from] crate::auth::AuthError),
}
/// Result for a single verification job in a batch.
#[derive(Serialize)]
pub struct BatchVerifyResult {
    /// Job ID (if provided)
    pub job_id: Option<String>,
    /// Verification result
    pub passed: bool,
    /// Errors found
    pub errors: Vec<String>,
    /// Warnings found
    pub warnings: Vec<String>,
    /// Number of statutes verified
    pub statute_count: usize,
}
/// Statute permission list response.
#[derive(Serialize)]
pub struct StatutePermissionsResponse {
    pub statute_id: String,
    pub permissions: Vec<StatutePermissionEntry>,
}
/// A single verification job within a batch.
#[derive(Deserialize)]
pub struct VerifyJob {
    /// Optional job ID for tracking
    pub job_id: Option<String>,
    /// Statute IDs to verify in this job
    pub statute_ids: Vec<String>,
}
/// Batch statute create request.
#[derive(Deserialize)]
pub struct BatchCreateStatutesRequest {
    pub statutes: Vec<Statute>,
}
/// Information about a statute version.
#[derive(Serialize)]
pub struct StatuteVersionInfo {
    pub id: String,
    pub version: u32,
    pub title: String,
    pub created_at: Option<String>,
}
/// API key list response.
#[derive(Serialize)]
pub struct ApiKeyListResponse {
    pub keys: Vec<ApiKeyResponse>,
}
/// Conflict detection request.
#[derive(Deserialize)]
pub struct ConflictDetectionRequest {
    pub statute_ids: Vec<String>,
}
/// Simulation comparison request.
#[derive(Deserialize)]
pub struct SimulationComparisonRequest {
    pub statute_ids_a: Vec<String>,
    pub statute_ids_b: Vec<String>,
    pub population_size: usize,
}
/// Visualization format options.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum VizFormat {
    Dot,
    Ascii,
    Mermaid,
    PlantUml,
    #[default]
    Svg,
    Html,
}
/// Server configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
}
impl ServerConfig {
    /// Returns the bind address.
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
/// Differences between two simulation scenarios.
#[derive(Serialize)]
pub struct SimulationDifferences {
    pub deterministic_diff: f64,
    pub discretionary_diff: f64,
    pub void_diff: f64,
    pub significant_change: bool,
}
/// Entry in the comparison matrix.
#[derive(Serialize)]
pub struct ComparisonMatrixEntry {
    pub statute_a_id: String,
    pub statute_b_id: String,
    pub similarity_score: f64,
    pub precondition_diff: i32,
    pub discretion_differs: bool,
}
/// API key rotation response.
#[derive(Serialize)]
pub struct ApiKeyRotationResponse {
    pub old_key_id: String,
    pub new_key: ApiKeyResponse,
}
/// Verification request.
#[derive(Deserialize)]
pub struct VerifyRequest {
    pub statute_ids: Vec<String>,
}
/// Application state.
pub struct AppState {
    /// In-memory statute storage
    pub statutes: RwLock<Vec<Statute>>,
    /// ReBAC authorization engine
    pub rebac: RwLock<crate::rebac::ReBACEngine>,
    /// Async verification job manager
    pub verification_jobs: crate::async_jobs::JobManager<VerificationJobResult>,
    /// Saved simulations
    pub saved_simulations: RwLock<Vec<SavedSimulation>>,
    /// Response cache
    pub cache: Arc<crate::cache::CacheStore>,
    /// WebSocket broadcaster for real-time notifications
    pub ws_broadcaster: crate::websocket::WsBroadcaster,
    /// Audit log for tracking all mutations
    pub audit_log: Arc<crate::audit::AuditLog>,
    /// API keys storage
    pub api_keys: RwLock<Vec<crate::auth::ApiKey>>,
    /// Collaborative editor for real-time editing
    pub collaborative_editor: Arc<crate::collaborative::CollaborativeEditor>,
    /// Presence manager for tracking active users
    pub presence_manager: Arc<crate::presence::PresenceManager>,
    /// Consent ledger for data-processing purposes
    pub consent_store: crate::consent::ConsentStore,
    /// API usage policy set (definitions + quota enforcement)
    pub usage_policies: crate::usage_policy::PolicySet,
    /// Data classification registry
    pub classification: Arc<RwLock<crate::data_classification::ClassificationRegistry>>,
    /// API key lifecycle / rotation manager
    pub key_rotation: crate::key_rotation::KeyRotationManager,
    /// Predictive cache / access-pattern model
    pub predictive_cache: crate::predictive_cache::PredictiveCache,
    /// Adaptive (intelligent) rate limiter
    pub intelligent_limiter: crate::intelligent_rate_limit::IntelligentRateLimiter,
    /// Abuse / request-anomaly detector
    pub abuse_detector: crate::abuse_detection::AbuseDetector,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            statutes: RwLock::new(Vec::new()),
            rebac: RwLock::new(crate::rebac::ReBACEngine::new()),
            verification_jobs: crate::async_jobs::JobManager::new(),
            saved_simulations: RwLock::new(Vec::new()),
            cache: Arc::new(crate::cache::CacheStore::new()),
            ws_broadcaster: crate::websocket::WsBroadcaster::new(),
            audit_log: Arc::new(crate::audit::AuditLog::new()),
            api_keys: RwLock::new(Vec::new()),
            collaborative_editor: Arc::new(crate::collaborative::CollaborativeEditor::new()),
            presence_manager: Arc::new(crate::presence::PresenceManager::new(30)),
            consent_store: crate::consent::ConsentStore::new(),
            usage_policies: crate::usage_policy::PolicySet::new(),
            classification: Arc::new(RwLock::new(
                crate::data_classification::ClassificationRegistry::new(),
            )),
            key_rotation: crate::key_rotation::KeyRotationManager::with_default_policy(),
            predictive_cache: crate::predictive_cache::PredictiveCache::new(),
            intelligent_limiter:
                crate::intelligent_rate_limit::IntelligentRateLimiter::with_defaults(),
            abuse_detector: crate::abuse_detection::AbuseDetector::with_defaults(),
        }
    }
}
/// Create new version of statute request.
#[derive(Deserialize)]
pub struct CreateVersionRequest {
    /// Optional modifications to apply to the new version
    pub title: Option<String>,
    pub preconditions: Option<Vec<legalis_core::Condition>>,
    pub effect: Option<legalis_core::Effect>,
    pub discretion_logic: Option<String>,
}
/// Verification job result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationJobResult {
    pub passed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub statute_count: usize,
}
/// Async verification start response.
#[derive(Serialize)]
pub struct AsyncVerifyStartResponse {
    pub job_id: String,
    pub status: String,
    pub poll_url: String,
}
/// Differences between two statutes.
#[derive(Serialize)]
pub struct ComparisonDifferences {
    pub precondition_count_diff: i32,
    pub nesting_depth_diff: i32,
    pub both_have_discretion: bool,
    pub discretion_differs: bool,
}
/// Batch verification response.
#[derive(Serialize)]
pub struct BatchVerifyResponse {
    /// Results for each verification job
    pub results: Vec<BatchVerifyResult>,
    /// Total jobs processed
    pub total_jobs: usize,
    /// Number of jobs that passed
    pub passed_jobs: usize,
    /// Number of jobs that failed
    pub failed_jobs: usize,
}
/// Simulation response.
#[derive(Serialize, Deserialize)]
pub struct SimulationResponse {
    pub simulation_id: String,
    pub total_entities: usize,
    pub deterministic_outcomes: usize,
    pub discretionary_outcomes: usize,
    pub void_outcomes: usize,
    pub deterministic_rate: f64,
    pub discretionary_rate: f64,
    pub void_rate: f64,
    pub completed_at: String,
}
/// What-if analysis response.
#[derive(Serialize)]
pub struct WhatIfResponse {
    pub baseline_compliant: bool,
    pub modified_compliant: bool,
    pub impact: String,
    pub baseline_requires_discretion: bool,
    pub modified_requires_discretion: bool,
    pub changed_attribute_count: usize,
}
/// Visualization request query parameters.
#[derive(Deserialize)]
pub struct VizQuery {
    /// Output format
    #[serde(default)]
    pub format: VizFormat,
    /// Theme (light, dark, high_contrast, colorblind_friendly)
    pub theme: Option<String>,
}
/// Batch statute create response.
#[derive(Serialize)]
pub struct BatchCreateStatutesResponse {
    pub created: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}
/// Job status response.
#[derive(Serialize)]
pub struct JobStatusResponse<T> {
    pub id: String,
    pub status: String,
    pub progress: f32,
    pub result: Option<T>,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
/// Saved simulation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSimulation {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub statute_ids: Vec<String>,
    pub population_size: usize,
    pub deterministic_outcomes: usize,
    pub discretionary_outcomes: usize,
    pub void_outcomes: usize,
    pub deterministic_rate: f64,
    pub discretionary_rate: f64,
    pub void_rate: f64,
    pub created_at: String,
    pub created_by: String,
}
/// Error response body.
#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    pub(crate) error: String,
}
/// Save simulation request.
#[derive(Deserialize)]
pub struct SaveSimulationRequest {
    pub name: String,
    pub description: Option<String>,
    pub simulation_result: SimulationResponse,
}
/// Success response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<ResponseMeta>,
}
impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data, meta: None }
    }
    pub fn with_meta(mut self, meta: ResponseMeta) -> Self {
        self.meta = Some(meta);
        self
    }
}
/// Statute permission update request.
#[derive(Deserialize)]
pub struct StatutePermissionRequest {
    /// User ID to grant/revoke permission
    pub user_id: String,
    /// Permission type (viewer, editor, owner)
    pub permission: String,
}
/// Create statute request.
#[derive(Deserialize)]
pub struct CreateStatuteRequest {
    pub statute: Statute,
}
/// Detailed verification report response.
#[derive(Serialize)]
pub struct DetailedVerifyResponse {
    pub passed: bool,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub total_suggestions: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub statute_count: usize,
    pub verified_at: String,
}
/// Batch statute delete request.
#[derive(Deserialize)]
pub struct BatchDeleteStatutesRequest {
    pub statute_ids: Vec<String>,
}
/// List saved simulations query.
#[derive(Deserialize)]
pub struct ListSavedSimulationsQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
/// Compliance check request.
#[derive(Deserialize)]
pub struct ComplianceCheckRequest {
    pub statute_ids: Vec<String>,
    pub entity_attributes: std::collections::HashMap<String, String>,
}
/// Information about a detected conflict.
#[derive(Serialize)]
pub struct ConflictInfo {
    pub statute_a_id: String,
    pub statute_b_id: String,
    pub conflict_type: String,
    pub description: String,
}
/// Results for a single simulation scenario.
#[derive(Serialize)]
pub struct SimulationScenarioResult {
    pub name: String,
    pub deterministic_rate: f64,
    pub discretionary_rate: f64,
    pub void_rate: f64,
}
/// Simulation comparison response.
#[derive(Serialize)]
pub struct SimulationComparisonResponse {
    pub scenario_a: SimulationScenarioResult,
    pub scenario_b: SimulationScenarioResult,
    pub differences: SimulationDifferences,
}
/// Statute permission entry.
#[derive(Serialize)]
pub struct StatutePermissionEntry {
    pub user_id: String,
    pub permission: String,
}
/// API key creation request.
#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    /// Name/description for the key
    pub name: String,
    /// Role for the key
    pub role: crate::auth::Role,
    /// Optional scoped permissions (if not provided, uses all role permissions)
    pub scopes: Option<Vec<String>>,
    /// Optional expiration in days (if not provided, never expires)
    pub expires_in_days: Option<i64>,
}
/// Simulation request.
#[derive(Deserialize)]
pub struct SimulationRequest {
    pub statute_ids: Vec<String>,
    pub population_size: usize,
    pub entity_params: std::collections::HashMap<String, String>,
}
/// Statute comparison matrix response.
#[derive(Serialize)]
pub struct StatuteComparisonMatrixResponse {
    /// Statutes being compared
    pub statutes: Vec<StatuteSummary>,
    /// Matrix of similarity scores (indexed by statute order)
    pub similarity_matrix: Vec<Vec<f64>>,
    /// Detailed comparison pairs
    pub comparisons: Vec<ComparisonMatrixEntry>,
}
/// Complexity analysis response.
#[derive(Serialize)]
pub struct ComplexityResponse {
    pub statute_id: String,
    pub complexity_score: f64,
    pub precondition_count: usize,
    pub nesting_depth: usize,
    pub has_discretion: bool,
}
/// Batch statute delete response.
#[derive(Serialize)]
pub struct BatchDeleteStatutesResponse {
    pub deleted: usize,
    pub not_found: Vec<String>,
}
/// Visualization response.
#[derive(Serialize)]
pub struct VisualizationResponse {
    pub statute_id: String,
    pub format: String,
    pub content: String,
    pub node_count: usize,
    pub discretionary_count: usize,
}
/// Statute comparison request.
#[derive(Deserialize)]
pub struct StatuteComparisonRequest {
    pub statute_id_a: String,
    pub statute_id_b: String,
}
/// Statute summary for list views.
#[derive(Serialize)]
pub struct StatuteSummary {
    pub id: String,
    pub title: String,
    pub has_discretion: bool,
    pub precondition_count: usize,
}
/// Batch verification request - verifies multiple groups of statutes independently.
#[derive(Deserialize)]
pub struct BatchVerifyRequest {
    /// Each entry is a separate verification job with its own statute IDs
    pub jobs: Vec<VerifyJob>,
}
/// What-if analysis request.
#[derive(Deserialize)]
pub struct WhatIfRequest {
    pub statute_ids: Vec<String>,
    pub baseline_attributes: std::collections::HashMap<String, String>,
    pub modified_attributes: std::collections::HashMap<String, String>,
}
/// Compliance check response.
#[derive(Serialize)]
pub struct ComplianceCheckResponse {
    pub compliant: bool,
    pub requires_discretion: bool,
    pub not_applicable: bool,
    pub applicable_statutes: Vec<String>,
    pub checked_statute_count: usize,
}
