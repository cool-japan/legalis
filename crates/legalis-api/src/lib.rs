//! Legalis-API: REST/GraphQL API server for Legalis-RS.
//!
//! This crate provides a web API for interacting with the Legalis-RS framework:
//! - CRUD operations for statutes
//! - Verification endpoints
//! - Simulation endpoints
//! - Registry queries
//! - OpenAPI 3.0 documentation
//! - Authentication and authorization (RBAC + ReBAC)

pub mod ai_suggestions;
pub mod anomaly;
pub mod async_jobs;
pub mod audit;
pub mod auth;
pub mod cache;
pub mod collaborative;
pub mod config;
pub mod contract_test;
// pub mod dataloader; // TODO: Re-enable when Loader trait signature issues are resolved
pub mod edge_cache;
pub mod field_selection;
pub mod gateway;
pub mod graphql;
#[cfg(feature = "grpc")]
pub mod grpc;
pub mod live_queries;
pub mod load_test;
pub mod logging;
mod metrics;
pub mod multitenancy;
pub mod oauth2_provider;
pub mod observability;
mod openapi;
pub mod persisted_queries;
pub mod presence;
pub mod query_batch;
pub mod query_cost;
pub mod rate_limit;
pub mod rebac;
pub mod sampling;
pub mod schema_stitching;
pub mod security;
pub mod slo;
pub mod telemetry;
pub mod versioning;
pub mod websocket;

use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures::stream::{self, Stream};
use legalis_core::Statute;
use legalis_viz::DecisionTree;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use tracing::info;

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
    Auth(#[from] auth::AuthError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ValidationFailed(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ApiError::Auth(err) => return err.into_response(),
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

/// Error response body.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Success response wrapper.
#[derive(Serialize)]
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

/// Response metadata.
#[derive(Serialize, Default)]
pub struct ResponseMeta {
    pub total: Option<usize>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
    pub has_more: Option<bool>,
}

/// Verification job result.
#[derive(Clone, Serialize)]
pub struct VerificationJobResult {
    pub passed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub statute_count: usize,
}

/// Saved simulation result.
#[derive(Clone, Serialize, Deserialize)]
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

/// Application state.
pub struct AppState {
    /// In-memory statute storage
    pub statutes: RwLock<Vec<Statute>>,
    /// ReBAC authorization engine
    pub rebac: RwLock<rebac::ReBACEngine>,
    /// Async verification job manager
    pub verification_jobs: async_jobs::JobManager<VerificationJobResult>,
    /// Saved simulations
    pub saved_simulations: RwLock<Vec<SavedSimulation>>,
    /// Response cache
    pub cache: Arc<cache::CacheStore>,
    /// WebSocket broadcaster for real-time notifications
    pub ws_broadcaster: websocket::WsBroadcaster,
    /// Audit log for tracking all mutations
    pub audit_log: Arc<audit::AuditLog>,
    /// API keys storage
    pub api_keys: RwLock<Vec<auth::ApiKey>>,
    /// Collaborative editor for real-time editing
    pub collaborative_editor: Arc<collaborative::CollaborativeEditor>,
    /// Presence manager for tracking active users
    pub presence_manager: Arc<presence::PresenceManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            statutes: RwLock::new(Vec::new()),
            rebac: RwLock::new(rebac::ReBACEngine::new()),
            verification_jobs: async_jobs::JobManager::new(),
            saved_simulations: RwLock::new(Vec::new()),
            cache: Arc::new(cache::CacheStore::new()),
            ws_broadcaster: websocket::WsBroadcaster::new(),
            audit_log: Arc::new(audit::AuditLog::new()),
            api_keys: RwLock::new(Vec::new()),
            collaborative_editor: Arc::new(collaborative::CollaborativeEditor::new()),
            presence_manager: Arc::new(presence::PresenceManager::new(30)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Statute list response.
#[derive(Serialize)]
pub struct StatuteListResponse {
    pub statutes: Vec<StatuteSummary>,
}

/// Statute summary for list views.
#[derive(Serialize)]
pub struct StatuteSummary {
    pub id: String,
    pub title: String,
    pub has_discretion: bool,
    pub precondition_count: usize,
}

/// Statute permission update request.
#[derive(Deserialize)]
pub struct StatutePermissionRequest {
    /// User ID to grant/revoke permission
    pub user_id: String,
    /// Permission type (viewer, editor, owner)
    pub permission: String,
}

/// Statute permission list response.
#[derive(Serialize)]
pub struct StatutePermissionsResponse {
    pub statute_id: String,
    pub permissions: Vec<StatutePermissionEntry>,
}

/// Statute permission entry.
#[derive(Serialize)]
pub struct StatutePermissionEntry {
    pub user_id: String,
    pub permission: String,
}

impl From<&Statute> for StatuteSummary {
    fn from(s: &Statute) -> Self {
        Self {
            id: s.id.clone(),
            title: s.title.clone(),
            has_discretion: s.discretion_logic.is_some(),
            precondition_count: s.preconditions.len(),
        }
    }
}

/// Create statute request.
#[derive(Deserialize)]
pub struct CreateStatuteRequest {
    pub statute: Statute,
}

/// Verification request.
#[derive(Deserialize)]
pub struct VerifyRequest {
    pub statute_ids: Vec<String>,
}

/// Verification response.
#[derive(Serialize)]
pub struct VerifyResponse {
    pub passed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Async verification start response.
#[derive(Serialize)]
pub struct AsyncVerifyStartResponse {
    pub job_id: String,
    pub status: String,
    pub poll_url: String,
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

/// Batch verification request - verifies multiple groups of statutes independently.
#[derive(Deserialize)]
pub struct BatchVerifyRequest {
    /// Each entry is a separate verification job with its own statute IDs
    pub jobs: Vec<VerifyJob>,
}

/// A single verification job within a batch.
#[derive(Deserialize)]
pub struct VerifyJob {
    /// Optional job ID for tracking
    pub job_id: Option<String>,
    /// Statute IDs to verify in this job
    pub statute_ids: Vec<String>,
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

/// Complexity analysis response.
#[derive(Serialize)]
pub struct ComplexityResponse {
    pub statute_id: String,
    pub complexity_score: f64,
    pub precondition_count: usize,
    pub nesting_depth: usize,
    pub has_discretion: bool,
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

/// Statute comparison request.
#[derive(Deserialize)]
pub struct StatuteComparisonRequest {
    pub statute_id_a: String,
    pub statute_id_b: String,
}

/// Statute comparison matrix request.
#[derive(Deserialize)]
pub struct StatuteComparisonMatrixRequest {
    /// List of statute IDs to compare
    pub statute_ids: Vec<String>,
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

/// Entry in the comparison matrix.
#[derive(Serialize)]
pub struct ComparisonMatrixEntry {
    pub statute_a_id: String,
    pub statute_b_id: String,
    pub similarity_score: f64,
    pub precondition_diff: i32,
    pub discretion_differs: bool,
}

/// API key creation request.
#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    /// Name/description for the key
    pub name: String,
    /// Role for the key
    pub role: auth::Role,
    /// Optional scoped permissions (if not provided, uses all role permissions)
    pub scopes: Option<Vec<String>>,
    /// Optional expiration in days (if not provided, never expires)
    pub expires_in_days: Option<i64>,
}

/// API key response (with the actual key value shown only once).
#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub key: Option<String>, // Only shown on creation
    pub name: String,
    pub role: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub active: bool,
    pub last_used_at: Option<String>,
}

/// API key list response.
#[derive(Serialize)]
pub struct ApiKeyListResponse {
    pub keys: Vec<ApiKeyResponse>,
}

/// API key rotation response.
#[derive(Serialize)]
pub struct ApiKeyRotationResponse {
    pub old_key_id: String,
    pub new_key: ApiKeyResponse,
}

/// Statute comparison response.
#[derive(Serialize)]
pub struct StatuteComparisonResponse {
    pub statute_a: StatuteSummary,
    pub statute_b: StatuteSummary,
    pub differences: ComparisonDifferences,
    pub similarity_score: f64,
}

/// Differences between two statutes.
#[derive(Serialize)]
pub struct ComparisonDifferences {
    pub precondition_count_diff: i32,
    pub nesting_depth_diff: i32,
    pub both_have_discretion: bool,
    pub discretion_differs: bool,
}

/// Batch statute create request.
#[derive(Deserialize)]
pub struct BatchCreateStatutesRequest {
    pub statutes: Vec<Statute>,
}

/// Batch statute create response.
#[derive(Serialize)]
pub struct BatchCreateStatutesResponse {
    pub created: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

/// Batch statute delete request.
#[derive(Deserialize)]
pub struct BatchDeleteStatutesRequest {
    pub statute_ids: Vec<String>,
}

/// Batch statute delete response.
#[derive(Serialize)]
pub struct BatchDeleteStatutesResponse {
    pub deleted: usize,
    pub not_found: Vec<String>,
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

/// Statute version list response.
#[derive(Serialize)]
pub struct StatuteVersionListResponse {
    pub base_id: String,
    pub versions: Vec<StatuteVersionInfo>,
    pub total_versions: usize,
}

/// Information about a statute version.
#[derive(Serialize)]
pub struct StatuteVersionInfo {
    pub id: String,
    pub version: u32,
    pub title: String,
    pub created_at: Option<String>,
}

/// Conflict detection request.
#[derive(Deserialize)]
pub struct ConflictDetectionRequest {
    pub statute_ids: Vec<String>,
}

/// Conflict detection response.
#[derive(Serialize)]
pub struct ConflictDetectionResponse {
    pub conflicts: Vec<ConflictInfo>,
    pub conflict_count: usize,
}

/// Information about a detected conflict.
#[derive(Serialize)]
pub struct ConflictInfo {
    pub statute_a_id: String,
    pub statute_b_id: String,
    pub conflict_type: String,
    pub description: String,
}

/// Simulation request.
#[derive(Deserialize)]
pub struct SimulationRequest {
    pub statute_ids: Vec<String>,
    pub population_size: usize,
    pub entity_params: std::collections::HashMap<String, String>,
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

/// Simulation comparison request.
#[derive(Deserialize)]
pub struct SimulationComparisonRequest {
    pub statute_ids_a: Vec<String>,
    pub statute_ids_b: Vec<String>,
    pub population_size: usize,
}

/// Simulation comparison response.
#[derive(Serialize)]
pub struct SimulationComparisonResponse {
    pub scenario_a: SimulationScenarioResult,
    pub scenario_b: SimulationScenarioResult,
    pub differences: SimulationDifferences,
}

/// Save simulation request.
#[derive(Deserialize)]
pub struct SaveSimulationRequest {
    pub name: String,
    pub description: Option<String>,
    pub simulation_result: SimulationResponse,
}

/// Compliance check request.
#[derive(Deserialize)]
pub struct ComplianceCheckRequest {
    pub statute_ids: Vec<String>,
    pub entity_attributes: std::collections::HashMap<String, String>,
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

/// What-if analysis request.
#[derive(Deserialize)]
pub struct WhatIfRequest {
    pub statute_ids: Vec<String>,
    pub baseline_attributes: std::collections::HashMap<String, String>,
    pub modified_attributes: std::collections::HashMap<String, String>,
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

/// List saved simulations query.
#[derive(Deserialize)]
pub struct ListSavedSimulationsQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Visualization format options.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VizFormat {
    Dot,
    Ascii,
    Mermaid,
    PlantUml,
    Svg,
    Html,
}

impl Default for VizFormat {
    fn default() -> Self {
        Self::Svg
    }
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

/// Visualization response.
#[derive(Serialize)]
pub struct VisualizationResponse {
    pub statute_id: String,
    pub format: String,
    pub content: String,
    pub node_count: usize,
    pub discretionary_count: usize,
}

/// Results for a single simulation scenario.
#[derive(Serialize)]
pub struct SimulationScenarioResult {
    pub name: String,
    pub deterministic_rate: f64,
    pub discretionary_rate: f64,
    pub void_rate: f64,
}

/// Differences between two simulation scenarios.
#[derive(Serialize)]
pub struct SimulationDifferences {
    pub deterministic_diff: f64,
    pub discretionary_diff: f64,
    pub void_diff: f64,
    pub significant_change: bool,
}

/// Get permissions for a specific statute.
async fn get_statute_permissions(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(statute_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    // Check if statute exists
    let statutes = state.statutes.read().await;
    if !statutes.iter().any(|s| s.id == statute_id) {
        return Err(ApiError::NotFound(format!(
            "Statute not found: {}",
            statute_id
        )));
    }
    drop(statutes);

    // Get all users who have access to this statute
    // This is a simplified version - in production, you'd have a way to iterate
    // through users or store reverse mappings via the ReBAC engine
    // For now, we'll return a placeholder response
    let permissions_list = vec![StatutePermissionEntry {
        user_id: "system".to_string(),
        permission: "owner".to_string(),
    }];

    Ok(Json(ApiResponse::new(StatutePermissionsResponse {
        statute_id,
        permissions: permissions_list,
    })))
}

/// Grant permission on a statute to a user.
async fn grant_statute_permission(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(statute_id): Path<String>,
    Json(req): Json<StatutePermissionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageUsers)?;

    // Check if statute exists
    let statutes = state.statutes.read().await;
    if !statutes.iter().any(|s| s.id == statute_id) {
        return Err(ApiError::NotFound(format!(
            "Statute not found: {}",
            statute_id
        )));
    }
    drop(statutes);

    // Parse user ID
    let target_user_id = uuid::Uuid::parse_str(&req.user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    // Create a deterministic UUID from statute_id using hash
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    statute_id.hash(&mut hasher);
    let hash_value = hasher.finish();

    // Convert hash to UUID (deterministic based on statute_id)
    let resource_uuid = uuid::Uuid::from_u128(hash_value as u128);

    // Parse relation type
    let relation = match req.permission.as_str() {
        "owner" => rebac::Relation::Owner,
        "editor" => rebac::Relation::Editor,
        "viewer" => rebac::Relation::Viewer,
        _ => {
            return Err(ApiError::BadRequest(format!(
                "Invalid permission type: {}. Must be one of: owner, editor, viewer",
                req.permission
            )));
        }
    };

    let mut rebac = state.rebac.write().await;

    // Grant the permission via ReBAC
    let tuple = rebac::RelationTuple::new(
        target_user_id,
        relation,
        rebac::ResourceType::Statute,
        resource_uuid,
    );
    rebac.add_tuple(tuple);

    // Update metrics
    metrics::PERMISSION_OPERATIONS
        .with_label_values(&["grant"])
        .inc();

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::PermissionGranted,
            user.id.to_string(),
            user.username.clone(),
            "grant_statute_permission".to_string(),
            Some(statute_id.clone()),
            Some("statute".to_string()),
            serde_json::json!({
                "statute_id": statute_id,
                "granted_to": req.user_id,
                "permission": req.permission
            }),
        )
        .await;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(serde_json::json!({
            "message": "Permission granted successfully",
            "statute_id": statute_id,
            "user_id": req.user_id,
            "permission": req.permission
        }))),
    ))
}

/// Revoke permission on a statute from a user.
async fn revoke_statute_permission(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(statute_id): Path<String>,
    Json(req): Json<StatutePermissionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageUsers)?;

    // Check if statute exists
    let statutes = state.statutes.read().await;
    if !statutes.iter().any(|s| s.id == statute_id) {
        return Err(ApiError::NotFound(format!(
            "Statute not found: {}",
            statute_id
        )));
    }
    drop(statutes);

    // Parse user ID
    let target_user_id = uuid::Uuid::parse_str(&req.user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    // Create a deterministic UUID from statute_id using hash
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    statute_id.hash(&mut hasher);
    let hash_value = hasher.finish();

    // Convert hash to UUID (deterministic based on statute_id)
    let resource_uuid = uuid::Uuid::from_u128(hash_value as u128);

    // Parse relation type
    let relation = match req.permission.as_str() {
        "owner" => rebac::Relation::Owner,
        "editor" => rebac::Relation::Editor,
        "viewer" => rebac::Relation::Viewer,
        _ => {
            return Err(ApiError::BadRequest(format!(
                "Invalid permission type: {}. Must be one of: owner, editor, viewer",
                req.permission
            )));
        }
    };

    let mut rebac = state.rebac.write().await;

    // Revoke the permission via ReBAC
    let tuple = rebac::RelationTuple::new(
        target_user_id,
        relation,
        rebac::ResourceType::Statute,
        resource_uuid,
    );
    rebac.remove_tuple(&tuple);

    // Update metrics
    metrics::PERMISSION_OPERATIONS
        .with_label_values(&["revoke"])
        .inc();

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::PermissionRevoked,
            user.id.to_string(),
            user.username.clone(),
            "revoke_statute_permission".to_string(),
            Some(statute_id.clone()),
            Some("statute".to_string()),
            serde_json::json!({
                "statute_id": statute_id,
                "revoked_from": req.user_id,
                "permission": req.permission
            }),
        )
        .await;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(serde_json::json!({
            "message": "Permission revoked successfully",
            "statute_id": statute_id,
            "user_id": req.user_id,
            "permission": req.permission
        }))),
    ))
}

/// Create a new API key.
async fn create_api_key(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageApiKeys)?;

    // Parse scopes if provided
    let scopes = if let Some(scope_strs) = req.scopes {
        let mut parsed_scopes = std::collections::HashSet::new();
        for scope_str in scope_strs {
            let permission = match scope_str.as_str() {
                "read_statutes" => auth::Permission::ReadStatutes,
                "create_statutes" => auth::Permission::CreateStatutes,
                "update_statutes" => auth::Permission::UpdateStatutes,
                "delete_statutes" => auth::Permission::DeleteStatutes,
                "verify_statutes" => auth::Permission::VerifyStatutes,
                "run_simulations" => auth::Permission::RunSimulations,
                "view_analytics" => auth::Permission::ViewAnalytics,
                "manage_users" => auth::Permission::ManageUsers,
                "manage_api_keys" => auth::Permission::ManageApiKeys,
                "admin" => auth::Permission::Admin,
                _ => {
                    return Err(ApiError::BadRequest(format!(
                        "Invalid permission: {}",
                        scope_str
                    )));
                }
            };
            parsed_scopes.insert(permission);
        }
        parsed_scopes
    } else {
        req.role.permissions()
    };

    // Create API key
    let api_key = if let Some(expires_in_days) = req.expires_in_days {
        auth::ApiKey::with_expiration(req.name, user.id, req.role, expires_in_days)
    } else {
        auth::ApiKey::with_scopes(req.name, user.id, req.role, scopes)
    };

    let key_id = api_key.id.to_string();
    let key_value = api_key.key.clone();

    // Store API key
    let mut api_keys = state.api_keys.write().await;
    api_keys.push(api_key.clone());
    drop(api_keys);

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::ApiKeyCreated,
            user.id.to_string(),
            user.username.clone(),
            "create_api_key".to_string(),
            Some(key_id.clone()),
            Some("api_key".to_string()),
            serde_json::json!({
                "key_id": key_id,
                "name": api_key.name,
                "role": format!("{:?}", api_key.role)
            }),
        )
        .await;

    let response = ApiKeyResponse {
        id: key_id,
        key: Some(key_value), // Only shown on creation
        name: api_key.name,
        role: format!("{:?}", api_key.role),
        scopes: api_key.scopes.iter().map(|s| format!("{:?}", s)).collect(),
        created_at: chrono::DateTime::from_timestamp(api_key.created_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
        expires_at: api_key.expires_at.map(|ts| {
            chrono::DateTime::from_timestamp(ts, 0)
                .unwrap_or_default()
                .to_rfc3339()
        }),
        active: api_key.active,
        last_used_at: None,
    };

    Ok((StatusCode::CREATED, Json(ApiResponse::new(response))))
}

/// List all API keys for the current user.
async fn list_api_keys(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageApiKeys)?;

    let api_keys = state.api_keys.read().await;

    let keys: Vec<ApiKeyResponse> = api_keys
        .iter()
        .filter(|key| key.owner_id == user.id || user.has_permission(auth::Permission::Admin))
        .map(|key| ApiKeyResponse {
            id: key.id.to_string(),
            key: None, // Never show the key value in list
            name: key.name.clone(),
            role: format!("{:?}", key.role),
            scopes: key.scopes.iter().map(|s| format!("{:?}", s)).collect(),
            created_at: chrono::DateTime::from_timestamp(key.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            expires_at: key.expires_at.map(|ts| {
                chrono::DateTime::from_timestamp(ts, 0)
                    .unwrap_or_default()
                    .to_rfc3339()
            }),
            active: key.active,
            last_used_at: key.last_used_at.map(|ts| {
                chrono::DateTime::from_timestamp(ts, 0)
                    .unwrap_or_default()
                    .to_rfc3339()
            }),
        })
        .collect();

    Ok(Json(ApiResponse::new(ApiKeyListResponse { keys })))
}

/// Get a specific API key.
#[allow(dead_code)]
async fn get_api_key(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageApiKeys)?;

    let key_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid key ID format".to_string()))?;

    let api_keys = state.api_keys.read().await;

    let key = api_keys
        .iter()
        .find(|k| {
            k.id == key_id
                && (k.owner_id == user.id || user.has_permission(auth::Permission::Admin))
        })
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;

    let response = ApiKeyResponse {
        id: key.id.to_string(),
        key: None, // Never show the key value
        name: key.name.clone(),
        role: format!("{:?}", key.role),
        scopes: key.scopes.iter().map(|s| format!("{:?}", s)).collect(),
        created_at: chrono::DateTime::from_timestamp(key.created_at, 0)
            .unwrap_or_default()
            .to_rfc3339(),
        expires_at: key.expires_at.map(|ts| {
            chrono::DateTime::from_timestamp(ts, 0)
                .unwrap_or_default()
                .to_rfc3339()
        }),
        active: key.active,
        last_used_at: key.last_used_at.map(|ts| {
            chrono::DateTime::from_timestamp(ts, 0)
                .unwrap_or_default()
                .to_rfc3339()
        }),
    };

    Ok(Json(ApiResponse::new(response)))
}

/// Revoke an API key.
async fn revoke_api_key(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageApiKeys)?;

    let key_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid key ID format".to_string()))?;

    let mut api_keys = state.api_keys.write().await;

    let key_index = api_keys
        .iter()
        .position(|k| {
            k.id == key_id
                && (k.owner_id == user.id || user.has_permission(auth::Permission::Admin))
        })
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;

    let key = api_keys.remove(key_index);

    drop(api_keys);

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::ApiKeyRevoked,
            user.id.to_string(),
            user.username.clone(),
            "revoke_api_key".to_string(),
            Some(key.id.to_string()),
            Some("api_key".to_string()),
            serde_json::json!({
                "key_id": key.id.to_string(),
                "name": key.name
            }),
        )
        .await;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(serde_json::json!({
            "message": "API key revoked successfully",
            "key_id": key.id.to_string()
        }))),
    ))
}

/// Rotate an API key (creates a new key, deactivates the old one).
async fn rotate_api_key(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ManageApiKeys)?;

    let key_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid key ID format".to_string()))?;

    let mut api_keys = state.api_keys.write().await;

    let old_key = api_keys
        .iter_mut()
        .find(|k| {
            k.id == key_id
                && (k.owner_id == user.id || user.has_permission(auth::Permission::Admin))
        })
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;

    // Create new key
    let new_key = old_key.rotate();
    let new_key_value = new_key.key.clone();

    // Deactivate old key
    old_key.active = false;

    // Add new key
    api_keys.push(new_key.clone());
    drop(api_keys);

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::ApiKeyRotated,
            user.id.to_string(),
            user.username.clone(),
            "rotate_api_key".to_string(),
            Some(new_key.id.to_string()),
            Some("api_key".to_string()),
            serde_json::json!({
                "old_key_id": key_id.to_string(),
                "new_key_id": new_key.id.to_string()
            }),
        )
        .await;

    let response = ApiKeyRotationResponse {
        old_key_id: key_id.to_string(),
        new_key: ApiKeyResponse {
            id: new_key.id.to_string(),
            key: Some(new_key_value), // Only shown on rotation
            name: new_key.name,
            role: format!("{:?}", new_key.role),
            scopes: new_key.scopes.iter().map(|s| format!("{:?}", s)).collect(),
            created_at: chrono::DateTime::from_timestamp(new_key.created_at, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            expires_at: new_key.expires_at.map(|ts| {
                chrono::DateTime::from_timestamp(ts, 0)
                    .unwrap_or_default()
                    .to_rfc3339()
            }),
            active: new_key.active,
            last_used_at: None,
        },
    };

    Ok((StatusCode::OK, Json(ApiResponse::new(response))))
}

/// Query audit logs with filtering.
async fn query_audit_logs(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Query(filter): Query<audit::AuditQueryFilter>,
) -> Result<impl IntoResponse, ApiError> {
    // Only admins can view audit logs
    user.require_permission(auth::Permission::Admin)?;

    let entries = state.audit_log.query(filter.clone()).await;
    let total = state.audit_log.count_filtered(filter).await;

    let meta = ResponseMeta {
        total: Some(total),
        ..Default::default()
    };

    Ok(Json(ApiResponse::new(entries).with_meta(meta)))
}

/// Get audit log statistics.
async fn audit_stats(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    // Only admins can view audit stats
    user.require_permission(auth::Permission::Admin)?;

    let total_count = state.audit_log.count().await;

    // Count by event type
    let statute_created = state
        .audit_log
        .count_filtered(audit::AuditQueryFilter {
            event_type: Some(audit::AuditEventType::StatuteCreated),
            ..Default::default()
        })
        .await;

    let statute_deleted = state
        .audit_log
        .count_filtered(audit::AuditQueryFilter {
            event_type: Some(audit::AuditEventType::StatuteDeleted),
            ..Default::default()
        })
        .await;

    let simulations_saved = state
        .audit_log
        .count_filtered(audit::AuditQueryFilter {
            event_type: Some(audit::AuditEventType::SimulationSaved),
            ..Default::default()
        })
        .await;

    let stats = serde_json::json!({
        "total_audit_entries": total_count,
        "by_event_type": {
            "statute_created": statute_created,
            "statute_deleted": statute_deleted,
            "simulations_saved": simulations_saved
        }
    });

    Ok(Json(ApiResponse::new(stats)))
}

/// GraphQL handler.
async fn graphql_handler(
    schema: axum::extract::Extension<graphql::LegalisSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphQL playground handler.
async fn graphql_playground() -> impl IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

/// Creates the API router.
pub fn create_router(state: Arc<AppState>) -> Router {
    // Initialize metrics
    metrics::init();

    // Create GraphQL schema with shared WebSocket broadcaster
    let graphql_state = graphql::GraphQLState::with_broadcaster(state.ws_broadcaster.clone());
    let graphql_schema = graphql::create_schema(graphql_state);

    Router::new()
        .route("/health", get(health_check))
        .route("/health/ready", get(readiness_check))
        .route("/metrics", get(metrics_endpoint))
        .route("/api/v1/statutes", get(list_statutes).post(create_statute))
        .route("/api/v1/statutes/search", get(search_statutes))
        .route("/api/v1/statutes/suggest", post(suggest_statutes))
        .route("/api/v1/statutes/batch", post(batch_create_statutes))
        .route("/api/v1/statutes/batch/delete", post(batch_delete_statutes))
        .route("/api/v1/statutes/compare", post(compare_statutes))
        .route(
            "/api/v1/statutes/compare/matrix",
            post(compare_statutes_matrix),
        )
        .route(
            "/api/v1/statutes/{id}",
            get(get_statute).delete(delete_statute),
        )
        .route("/api/v1/statutes/{id}/complexity", get(analyze_complexity))
        .route("/api/v1/statutes/{id}/versions", get(get_statute_versions))
        .route(
            "/api/v1/statutes/{id}/versions/new",
            post(create_statute_version),
        )
        .route("/api/v1/verify", post(verify_statutes))
        .route("/api/v1/verify/detailed", post(verify_statutes_detailed))
        .route("/api/v1/verify/conflicts", post(detect_conflicts))
        .route("/api/v1/verify/batch", post(verify_batch))
        .route("/api/v1/verify/bulk/stream", post(verify_bulk_stream))
        .route("/api/v1/verify/async", post(verify_statutes_async))
        .route(
            "/api/v1/verify/async/{job_id}",
            get(get_verification_job_status),
        )
        .route("/api/v1/simulate", post(run_simulation))
        .route("/api/v1/simulate/stream", post(stream_simulation))
        .route("/api/v1/simulate/compare", post(compare_simulations))
        .route("/api/v1/simulate/compliance", post(check_compliance))
        .route("/api/v1/simulate/whatif", post(whatif_analysis))
        .route(
            "/api/v1/simulate/saved",
            get(list_saved_simulations).post(save_simulation),
        )
        .route(
            "/api/v1/simulate/saved/{id}",
            get(get_saved_simulation).delete(delete_saved_simulation),
        )
        .route("/api/v1/visualize/{id}", get(visualize_statute))
        .route("/api-docs/openapi.json", get(openapi_spec))
        .route("/api-docs", get(swagger_ui))
        .route("/graphql", post(graphql_handler))
        .route("/graphql/playground", get(graphql_playground))
        .route("/ws", get(websocket::ws_handler))
        .route("/api/v1/audit", get(query_audit_logs))
        .route("/api/v1/audit/stats", get(audit_stats))
        .route(
            "/api/v1/statutes/{id}/permissions",
            get(get_statute_permissions)
                .post(grant_statute_permission)
                .delete(revoke_statute_permission),
        )
        .route("/api/v1/api-keys", get(list_api_keys).post(create_api_key))
        .route(
            "/api/v1/api-keys/{id}",
            get(get_api_key).delete(revoke_api_key),
        )
        .route("/api/v1/api-keys/{id}/rotate", post(rotate_api_key))
        .layer(Extension(graphql_schema))
        .layer(middleware::from_fn(logging::log_request))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Returns the OpenAPI 3.0 specification.
async fn openapi_spec() -> impl IntoResponse {
    Json(openapi::generate_spec())
}

/// Returns the Swagger UI HTML page.
async fn swagger_ui() -> impl IntoResponse {
    axum::response::Html(openapi::generate_swagger_ui_html())
}

/// Health check endpoint - liveness probe.
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "legalis-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Readiness check endpoint - checks if the service is ready to accept requests.
async fn readiness_check(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    // Check if we can access the statutes (test read lock)
    let statutes_available = state.statutes.try_read().is_ok();
    let rebac_available = state.rebac.try_read().is_ok();

    let is_ready = statutes_available && rebac_available;

    let response = serde_json::json!({
        "status": if is_ready { "ready" } else { "not_ready" },
        "service": "legalis-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "statutes_store": if statutes_available { "ok" } else { "unavailable" },
            "rebac_engine": if rebac_available { "ok" } else { "unavailable" }
        }
    });

    if is_ready {
        Ok(Json(response))
    } else {
        Err(ApiError::Internal("Service not ready".to_string()))
    }
}

/// Prometheus metrics endpoint.
async fn metrics_endpoint() -> Result<String, ApiError> {
    metrics::encode().map_err(|e| ApiError::Internal(format!("Failed to encode metrics: {}", e)))
}

/// List all statutes.
async fn list_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let statutes = state.statutes.read().await;
    let summaries: Vec<StatuteSummary> = statutes.iter().map(StatuteSummary::from).collect();

    Ok(Json(ApiResponse::new(StatuteListResponse {
        statutes: summaries,
    })))
}

/// Search/filter statutes.
async fn search_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<StatuteSearchQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    // Parse field selection
    let _field_query = field_selection::FieldsQuery {
        fields: query.fields.clone(),
    };

    let statutes = state.statutes.read().await;

    let mut filtered: Vec<&Statute> = statutes.iter().collect();

    // Filter by title
    if let Some(ref title_query) = query.title {
        let title_lower = title_query.to_lowercase();
        filtered.retain(|s| s.title.to_lowercase().contains(&title_lower));
    }

    // Filter by discretion
    if let Some(has_discretion) = query.has_discretion {
        filtered.retain(|s| s.discretion_logic.is_some() == has_discretion);
    }

    // Filter by min preconditions
    if let Some(min) = query.min_preconditions {
        filtered.retain(|s| s.preconditions.len() >= min);
    }

    // Filter by max preconditions
    if let Some(max) = query.max_preconditions {
        filtered.retain(|s| s.preconditions.len() <= max);
    }

    let total = filtered.len();

    // Support both cursor-based and offset-based pagination
    let (paginated, meta) = if let Some(cursor) = query.cursor {
        // Cursor-based pagination
        let limit = query.limit.unwrap_or(100).min(1000);

        // Decode cursor (format: base64(id:version))
        let cursor_decoded = base64_decode(&cursor)
            .map_err(|_| ApiError::BadRequest("Invalid cursor".to_string()))?;

        let cursor_parts: Vec<&str> = cursor_decoded.split(':').collect();
        if cursor_parts.len() != 2 {
            return Err(ApiError::BadRequest("Invalid cursor format".to_string()));
        }

        let cursor_id = cursor_parts[0];
        let cursor_version: u32 = cursor_parts[1]
            .parse()
            .map_err(|_| ApiError::BadRequest("Invalid cursor version".to_string()))?;

        // Find position of cursor
        let cursor_pos = filtered
            .iter()
            .position(|s| s.id == cursor_id && s.version == cursor_version);

        let start_pos = cursor_pos.map(|p| p + 1).unwrap_or(0);

        let results: Vec<StatuteSummary> = filtered
            .iter()
            .skip(start_pos)
            .take(limit + 1) // Take one extra to check if there are more
            .map(|s| StatuteSummary::from(*s))
            .collect();

        let has_more = results.len() > limit;
        let mut final_results = results;
        if has_more {
            final_results.pop(); // Remove the extra item
        }

        // Generate next cursor if there are more results
        let next_cursor = if has_more && !final_results.is_empty() {
            let last = &final_results[final_results.len() - 1];
            Some(base64_encode(&format!("{}:{}", last.id, 1))) // Use version 1 as default
        } else {
            None
        };

        let meta = ResponseMeta {
            total: Some(total),
            next_cursor,
            has_more: Some(has_more),
            ..Default::default()
        };

        (final_results, meta)
    } else {
        // Offset-based pagination
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(100).min(1000);

        let paginated = filtered
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(StatuteSummary::from)
            .collect();

        let meta = ResponseMeta {
            total: Some(total),
            page: Some(offset / limit),
            per_page: Some(limit),
            ..Default::default()
        };

        (paginated, meta)
    };

    Ok(Json(
        ApiResponse::new(StatuteListResponse {
            statutes: paginated,
        })
        .with_meta(meta),
    ))
}

/// AI-powered statute suggestion endpoint.
async fn suggest_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(request): Json<ai_suggestions::SuggestionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    // Get available statutes
    let statutes = state.statutes.read().await;
    let statute_vec: Vec<_> = statutes.iter().cloned().collect();

    // Create suggestion engine (without LLM provider for now, uses rule-based)
    let engine = ai_suggestions::SuggestionEngine::new();

    // Generate suggestions
    let response = engine
        .suggest(request, &statute_vec)
        .await
        .map_err(|e| ApiError::Internal(format!("Suggestion failed: {}", e)))?;

    Ok(Json(ApiResponse::new(response)))
}

/// Base64 encode a string.
fn base64_encode(s: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(s)
}

/// Base64 decode a string.
fn base64_decode(s: &str) -> Result<String, base64::DecodeError> {
    use base64::{Engine as _, engine::general_purpose};
    let bytes = general_purpose::STANDARD.decode(s)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

/// Get a specific statute.
async fn get_statute(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;

    Ok(Json(ApiResponse::new(statute.clone())))
}

/// Create a new statute.
async fn create_statute(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateStatuteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::CreateStatutes)?;

    let mut statutes = state.statutes.write().await;

    // Check for duplicate ID
    if statutes.iter().any(|s| s.id == req.statute.id) {
        return Err(ApiError::BadRequest(format!(
            "Statute with ID '{}' already exists",
            req.statute.id
        )));
    }

    info!(
        "Creating statute: {} by user {}",
        req.statute.id, user.username
    );

    let statute_id = req.statute.id.clone();
    let statute_title = req.statute.title.clone();
    statutes.push(req.statute.clone());

    // Update metrics
    metrics::STATUTE_OPERATIONS
        .with_label_values(&["create"])
        .inc();
    metrics::STATUTES_TOTAL.inc();

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::StatuteCreated,
            user.id.to_string(),
            user.username.clone(),
            "create_statute".to_string(),
            Some(statute_id.clone()),
            Some("statute".to_string()),
            serde_json::json!({
                "statute_id": statute_id,
                "title": statute_title
            }),
        )
        .await;

    // Broadcast WebSocket notification
    state
        .ws_broadcaster
        .broadcast(websocket::WsNotification::StatuteCreated {
            statute_id: statute_id.clone(),
            title: statute_title,
            created_by: user.username.clone(),
        });

    Ok((StatusCode::CREATED, Json(ApiResponse::new(req.statute))))
}

/// Compare multiple statutes in a matrix format.
async fn compare_statutes_matrix(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<StatuteComparisonMatrixRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    if req.statute_ids.len() < 2 {
        return Err(ApiError::BadRequest(
            "At least 2 statutes required for comparison matrix".to_string(),
        ));
    }

    if req.statute_ids.len() > 20 {
        return Err(ApiError::BadRequest(
            "Maximum 20 statutes allowed for comparison matrix".to_string(),
        ));
    }

    let statutes = state.statutes.read().await;

    // Fetch all requested statutes
    let mut statute_list = Vec::new();
    for id in &req.statute_ids {
        if let Some(statute) = statutes.iter().find(|s| &s.id == id) {
            statute_list.push(statute.clone());
        } else {
            return Err(ApiError::NotFound(format!("Statute not found: {}", id)));
        }
    }

    let count = statute_list.len();

    // Build similarity matrix (symmetric matrix)
    let mut similarity_matrix = vec![vec![0.0; count]; count];
    let mut comparisons = Vec::new();

    for i in 0..count {
        for j in i..count {
            if i == j {
                // Same statute: 100% similarity
                similarity_matrix[i][j] = 100.0;
            } else {
                // Calculate similarity between statute i and j
                let stat_a = &statute_list[i];
                let stat_b = &statute_list[j];

                let precond_count_a = stat_a.preconditions.len() as i32;
                let precond_count_b = stat_b.preconditions.len() as i32;
                let precondition_diff = precond_count_b - precond_count_a;

                let depth_a = calculate_nesting_depth(&stat_a.preconditions) as i32;
                let depth_b = calculate_nesting_depth(&stat_b.preconditions) as i32;
                let depth_diff = depth_b - depth_a;

                let discretion_a = stat_a.discretion_logic.is_some();
                let discretion_b = stat_b.discretion_logic.is_some();
                let discretion_differs = discretion_a != discretion_b;

                // Calculate similarity score
                let mut similarity = 100.0;
                similarity -= (precondition_diff.abs() as f64) * 5.0;
                similarity -= (depth_diff.abs() as f64) * 10.0;
                if discretion_differs {
                    similarity -= 20.0;
                }
                similarity = similarity.clamp(0.0, 100.0);

                // Store in matrix (symmetric)
                similarity_matrix[i][j] = similarity;
                similarity_matrix[j][i] = similarity;

                comparisons.push(ComparisonMatrixEntry {
                    statute_a_id: stat_a.id.clone(),
                    statute_b_id: stat_b.id.clone(),
                    similarity_score: similarity,
                    precondition_diff,
                    discretion_differs,
                });
            }
        }
    }

    let summaries: Vec<StatuteSummary> = statute_list.iter().map(StatuteSummary::from).collect();

    Ok(Json(ApiResponse::new(StatuteComparisonMatrixResponse {
        statutes: summaries,
        similarity_matrix,
        comparisons,
    })))
}

/// Compare two statutes.
async fn compare_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<StatuteComparisonRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let statutes = state.statutes.read().await;

    let statute_a = statutes
        .iter()
        .find(|s| s.id == req.statute_id_a)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", req.statute_id_a)))?;

    let statute_b = statutes
        .iter()
        .find(|s| s.id == req.statute_id_b)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", req.statute_id_b)))?;

    let summary_a = StatuteSummary::from(statute_a);
    let summary_b = StatuteSummary::from(statute_b);

    let precondition_count_a = statute_a.preconditions.len() as i32;
    let precondition_count_b = statute_b.preconditions.len() as i32;

    let nesting_depth_a = calculate_nesting_depth(&statute_a.preconditions) as i32;
    let nesting_depth_b = calculate_nesting_depth(&statute_b.preconditions) as i32;

    let has_discretion_a = statute_a.discretion_logic.is_some();
    let has_discretion_b = statute_b.discretion_logic.is_some();

    let differences = ComparisonDifferences {
        precondition_count_diff: precondition_count_b - precondition_count_a,
        nesting_depth_diff: nesting_depth_b - nesting_depth_a,
        both_have_discretion: has_discretion_a && has_discretion_b,
        discretion_differs: has_discretion_a != has_discretion_b,
    };

    // Calculate similarity score (0.0 to 100.0)
    let mut similarity_score = 100.0;

    // Penalize for precondition differences
    let precond_diff = (precondition_count_b - precondition_count_a).abs() as f64;
    similarity_score -= precond_diff * 5.0;

    // Penalize for nesting depth differences
    let depth_diff = (nesting_depth_b - nesting_depth_a).abs() as f64;
    similarity_score -= depth_diff * 10.0;

    // Penalize if discretion differs
    if differences.discretion_differs {
        similarity_score -= 20.0;
    }

    similarity_score = similarity_score.clamp(0.0, 100.0);

    Ok(Json(ApiResponse::new(StatuteComparisonResponse {
        statute_a: summary_a,
        statute_b: summary_b,
        differences,
        similarity_score,
    })))
}

/// Batch create statutes.
async fn batch_create_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchCreateStatutesRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::CreateStatutes)?;

    if req.statutes.is_empty() {
        return Err(ApiError::BadRequest("No statutes provided".to_string()));
    }

    let mut statutes = state.statutes.write().await;
    let mut created = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    let total_requested = req.statutes.len();

    for statute in req.statutes {
        // Check for duplicate ID
        if statutes.iter().any(|s| s.id == statute.id) {
            errors.push(format!("Statute with ID '{}' already exists", statute.id));
            failed += 1;
            continue;
        }

        info!(
            "Creating statute: {} by user {} (batch)",
            statute.id, user.username
        );
        statutes.push(statute);
        created += 1;
    }

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::BatchStatutesCreated,
            user.id.to_string(),
            user.username.clone(),
            "batch_create_statutes".to_string(),
            None,
            Some("statute".to_string()),
            serde_json::json!({
                "created": created,
                "failed": failed,
                "total": total_requested
            }),
        )
        .await;

    Ok((
        if created > 0 {
            StatusCode::CREATED
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(ApiResponse::new(BatchCreateStatutesResponse {
            created,
            failed,
            errors,
        })),
    ))
}

/// Batch delete statutes.
async fn batch_delete_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchDeleteStatutesRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::DeleteStatutes)?;

    if req.statute_ids.is_empty() {
        return Err(ApiError::BadRequest("No statute IDs provided".to_string()));
    }

    let mut statutes = state.statutes.write().await;
    let mut deleted = 0;
    let mut not_found = Vec::new();
    let total_requested = req.statute_ids.len();

    for id in req.statute_ids {
        let initial_len = statutes.len();
        statutes.retain(|s| s.id != id);

        if statutes.len() < initial_len {
            info!("Deleted statute: {} by user {} (batch)", id, user.username);
            deleted += 1;
        } else {
            not_found.push(id);
        }
    }

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::BatchStatutesDeleted,
            user.id.to_string(),
            user.username.clone(),
            "batch_delete_statutes".to_string(),
            None,
            Some("statute".to_string()),
            serde_json::json!({
                "deleted": deleted,
                "not_found": not_found.len(),
                "total": total_requested
            }),
        )
        .await;

    Ok(Json(ApiResponse::new(BatchDeleteStatutesResponse {
        deleted,
        not_found,
    })))
}

/// Delete a statute.
async fn delete_statute(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::DeleteStatutes)?;

    let mut statutes = state.statutes.write().await;
    let initial_len = statutes.len();
    statutes.retain(|s| s.id != id);

    if statutes.len() == initial_len {
        return Err(ApiError::NotFound(format!("Statute not found: {}", id)));
    }

    info!("Deleted statute: {} by user {}", id, user.username);

    // Update metrics
    metrics::STATUTE_OPERATIONS
        .with_label_values(&["delete"])
        .inc();
    metrics::STATUTES_TOTAL.dec();

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::StatuteDeleted,
            user.id.to_string(),
            user.username.clone(),
            "delete_statute".to_string(),
            Some(id.clone()),
            Some("statute".to_string()),
            serde_json::json!({
                "statute_id": id
            }),
        )
        .await;

    // Broadcast WebSocket notification
    state
        .ws_broadcaster
        .broadcast(websocket::WsNotification::StatuteDeleted {
            statute_id: id.clone(),
            deleted_by: user.username.clone(),
        });

    Ok(StatusCode::NO_CONTENT)
}

/// Verify statutes.
async fn verify_statutes(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    let statutes = state.statutes.read().await;

    let to_verify: Vec<&Statute> = if req.statute_ids.is_empty() {
        statutes.iter().collect()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .collect()
    };

    if to_verify.is_empty() {
        return Err(ApiError::BadRequest("No statutes to verify".to_string()));
    }

    let verifier = legalis_verifier::StatuteVerifier::new();
    let to_verify_owned: Vec<Statute> = to_verify.into_iter().cloned().collect();
    let result = verifier.verify(&to_verify_owned);

    // Update metrics
    metrics::VERIFICATIONS_TOTAL.inc();
    metrics::VERIFICATION_RESULTS
        .with_label_values(&[if result.passed { "passed" } else { "failed" }])
        .inc();

    Ok(Json(ApiResponse::new(VerifyResponse {
        passed: result.passed,
        errors: result.errors.iter().map(|e| e.to_string()).collect(),
        warnings: result.warnings.clone(),
    })))
}

/// Verify statutes with detailed report.
async fn verify_statutes_detailed(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    let statutes = state.statutes.read().await;

    let to_verify: Vec<&Statute> = if req.statute_ids.is_empty() {
        statutes.iter().collect()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .collect()
    };

    if to_verify.is_empty() {
        return Err(ApiError::BadRequest("No statutes to verify".to_string()));
    }

    let verifier = legalis_verifier::StatuteVerifier::new();
    let to_verify_owned: Vec<Statute> = to_verify.into_iter().cloned().collect();
    let result = verifier.verify(&to_verify_owned);

    let errors: Vec<String> = result.errors.iter().map(|e| e.to_string()).collect();
    let warnings = result.warnings.clone();
    let suggestions = result.suggestions.clone();

    Ok(Json(ApiResponse::new(DetailedVerifyResponse {
        passed: result.passed,
        total_errors: errors.len(),
        total_warnings: warnings.len(),
        total_suggestions: suggestions.len(),
        errors,
        warnings,
        suggestions,
        statute_count: to_verify_owned.len(),
        verified_at: chrono::Utc::now().to_rfc3339(),
    })))
}

/// Detect conflicts between statutes.
async fn detect_conflicts(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ConflictDetectionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    let statutes = state.statutes.read().await;

    let to_check: Vec<&Statute> = if req.statute_ids.is_empty() {
        statutes.iter().collect()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .collect()
    };

    if to_check.len() < 2 {
        return Err(ApiError::BadRequest(
            "At least 2 statutes required for conflict detection".to_string(),
        ));
    }

    let verifier = legalis_verifier::StatuteVerifier::new();
    let to_check_owned: Vec<Statute> = to_check.into_iter().cloned().collect();
    let result = verifier.verify(&to_check_owned);

    let mut conflicts = Vec::new();

    // Extract conflicts from verification errors
    for error in result.errors.iter() {
        let error_str = error.to_string();
        if error_str.contains("conflict") || error_str.contains("contradiction") {
            // Parse conflict information from error message
            // This is a simplified version; in production, we'd want more structured data
            conflicts.push(ConflictInfo {
                statute_a_id: "statute-a".to_string(), // Would need to parse from error
                statute_b_id: "statute-b".to_string(), // Would need to parse from error
                conflict_type: "logical-contradiction".to_string(),
                description: error_str,
            });
        }
    }

    Ok(Json(ApiResponse::new(ConflictDetectionResponse {
        conflict_count: conflicts.len(),
        conflicts,
    })))
}

/// Start async verification of statutes.
/// Returns a job ID that can be used to poll for results.
async fn verify_statutes_async(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    // Create a job
    let job_id = state.verification_jobs.create_job().await;

    // Clone state for the background task
    let state_clone = Arc::clone(&state);
    let statute_ids = req.statute_ids.clone();
    let job_id_clone = job_id.clone();

    // Spawn background task
    tokio::spawn(async move {
        let job_id = job_id_clone;
        // Mark job as running
        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.set_running();
            })
            .await;

        // Get statutes
        let statutes = state_clone.statutes.read().await;

        let to_verify: Vec<&Statute> = if statute_ids.is_empty() {
            statutes.iter().collect()
        } else {
            statutes
                .iter()
                .filter(|s| statute_ids.contains(&s.id))
                .collect()
        };

        if to_verify.is_empty() {
            state_clone
                .verification_jobs
                .update_job(&job_id, |job| {
                    job.fail("No statutes to verify".to_string());
                })
                .await;
            return;
        }

        // Update progress
        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.set_progress(30.0);
            })
            .await;

        // Run verification
        let verifier = legalis_verifier::StatuteVerifier::new();
        let to_verify_owned: Vec<Statute> = to_verify.into_iter().cloned().collect();
        let statute_count = to_verify_owned.len();

        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.set_progress(60.0);
            })
            .await;

        let result = verifier.verify(&to_verify_owned);

        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.set_progress(90.0);
            })
            .await;

        // Complete job
        let job_result = VerificationJobResult {
            passed: result.passed,
            errors: result.errors.iter().map(|e| e.to_string()).collect(),
            warnings: result.warnings,
            statute_count,
        };

        let passed = job_result.passed;
        let errors_count = job_result.errors.len();
        let warnings_count = job_result.warnings.len();

        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.complete(job_result);
            })
            .await;

        // Broadcast WebSocket notification
        state_clone
            .ws_broadcaster
            .broadcast(websocket::WsNotification::VerificationCompleted {
                job_id: job_id.clone(),
                passed,
                errors_count,
                warnings_count,
            });
    });

    let poll_url = format!("/api/v1/verify/async/{}", job_id);

    Ok((
        StatusCode::ACCEPTED,
        Json(ApiResponse::new(AsyncVerifyStartResponse {
            job_id,
            status: "pending".to_string(),
            poll_url,
        })),
    ))
}

/// Get async verification job status.
async fn get_verification_job_status(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    let job = state
        .verification_jobs
        .get_job(&job_id)
        .await
        .ok_or_else(|| ApiError::NotFound(format!("Job not found: {}", job_id)))?;

    let status_str = match job.status {
        async_jobs::JobStatus::Pending => "pending",
        async_jobs::JobStatus::Running => "running",
        async_jobs::JobStatus::Completed => "completed",
        async_jobs::JobStatus::Failed => "failed",
    }
    .to_string();

    Ok(Json(ApiResponse::new(JobStatusResponse {
        id: job.id,
        status: status_str,
        progress: job.progress,
        result: job.result,
        error: job.error,
        created_at: job.created_at.to_rfc3339(),
        updated_at: job.updated_at.to_rfc3339(),
    })))
}

/// Bulk verification with streaming results via Server-Sent Events.
/// Verifies statutes in bulk and streams progress updates in real-time.
async fn verify_bulk_stream(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchVerifyRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    if req.jobs.is_empty() {
        return Err(ApiError::BadRequest(
            "No verification jobs provided".to_string(),
        ));
    }

    // Clone data for async stream
    let statutes = state.statutes.read().await.clone();

    // Create stream
    let stream = stream::unfold(
        (req.jobs, statutes, 0usize),
        |(mut jobs, statutes, processed)| async move {
            if processed == 0 {
                // Send start event
                let event = Event::default()
                    .event("start")
                    .json_data(serde_json::json!({
                        "total_jobs": jobs.len(),
                        "status": "started"
                    }))
                    .ok()?;
                return Some((Ok::<_, Infallible>(event), (jobs, statutes, processed)));
            }

            if jobs.is_empty() {
                // Send completion event
                let event = Event::default()
                    .event("complete")
                    .json_data(serde_json::json!({
                        "status": "completed",
                        "total_processed": processed
                    }))
                    .ok()?;
                return Some((Ok::<_, Infallible>(event), (jobs, statutes, processed)));
            }

            // Process next job
            let job = jobs.remove(0);
            let verifier = legalis_verifier::StatuteVerifier::new();

            let to_verify: Vec<&Statute> = if job.statute_ids.is_empty() {
                statutes.iter().collect()
            } else {
                statutes
                    .iter()
                    .filter(|s| job.statute_ids.contains(&s.id))
                    .collect()
            };

            let to_verify_owned: Vec<Statute> = to_verify.into_iter().cloned().collect();
            let statute_count = to_verify_owned.len();

            let result = if statute_count == 0 {
                BatchVerifyResult {
                    job_id: job.job_id.clone(),
                    passed: false,
                    errors: vec!["No statutes found for verification".to_string()],
                    warnings: vec![],
                    statute_count: 0,
                }
            } else {
                let verify_result = verifier.verify(&to_verify_owned);
                BatchVerifyResult {
                    job_id: job.job_id,
                    passed: verify_result.passed,
                    errors: verify_result.errors.iter().map(|e| e.to_string()).collect(),
                    warnings: verify_result.warnings.clone(),
                    statute_count,
                }
            };

            let processed_count = processed + 1;
            let event = Event::default()
                .event("result")
                .json_data(serde_json::json!({
                    "job_index": processed_count,
                    "total_jobs": processed_count + jobs.len(),
                    "result": result,
                    "progress": (processed_count as f64 / (processed_count + jobs.len()) as f64) * 100.0
                }))
                .ok()?;

            Some((
                Ok::<_, Infallible>(event),
                (jobs, statutes, processed_count),
            ))
        },
    );

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// Batch verification of multiple statute groups.
/// Each job is verified independently, allowing parallel processing.
async fn verify_batch(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchVerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    if req.jobs.is_empty() {
        return Err(ApiError::BadRequest(
            "No verification jobs provided".to_string(),
        ));
    }

    let statutes = state.statutes.read().await;
    let verifier = legalis_verifier::StatuteVerifier::new();

    // Process each job
    let mut results = Vec::new();
    let total_jobs = req.jobs.len();
    for job in req.jobs {
        let to_verify: Vec<&Statute> = if job.statute_ids.is_empty() {
            statutes.iter().collect()
        } else {
            statutes
                .iter()
                .filter(|s| job.statute_ids.contains(&s.id))
                .collect()
        };

        let to_verify_owned: Vec<Statute> = to_verify.into_iter().cloned().collect();
        let statute_count = to_verify_owned.len();

        // Skip empty jobs
        if statute_count == 0 {
            results.push(BatchVerifyResult {
                job_id: job.job_id.clone(),
                passed: false,
                errors: vec!["No statutes found for verification".to_string()],
                warnings: vec![],
                statute_count: 0,
            });
            continue;
        }

        let result = verifier.verify(&to_verify_owned);

        results.push(BatchVerifyResult {
            job_id: job.job_id,
            passed: result.passed,
            errors: result.errors.iter().map(|e| e.to_string()).collect(),
            warnings: result.warnings.clone(),
            statute_count,
        });
    }

    let passed_jobs = results.iter().filter(|r| r.passed).count();
    let failed_jobs = results.len() - passed_jobs;

    Ok(Json(ApiResponse::new(BatchVerifyResponse {
        results,
        total_jobs,
        passed_jobs,
        failed_jobs,
    })))
}

/// Analyze complexity of a statute.
async fn analyze_complexity(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;

    // Calculate complexity metrics
    let precondition_count = statute.preconditions.len();
    let nesting_depth = calculate_nesting_depth(&statute.preconditions);
    let has_discretion = statute.discretion_logic.is_some();

    // Simple complexity score formula
    let complexity_score = (precondition_count as f64 * 1.5)
        + (nesting_depth as f64 * 2.0)
        + if has_discretion { 5.0 } else { 0.0 };

    Ok(Json(ApiResponse::new(ComplexityResponse {
        statute_id: id,
        complexity_score,
        precondition_count,
        nesting_depth,
        has_discretion,
    })))
}

/// Get all versions of a statute by base ID.
/// Statutes are grouped by their base ID (the part before the version suffix).
async fn get_statute_versions(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(base_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let statutes = state.statutes.read().await;

    // Find all statutes that match the base_id
    // We consider statutes with the same ID or ID that starts with base_id
    let versions: Vec<StatuteVersionInfo> = statutes
        .iter()
        .filter(|s| s.id == base_id || s.id.starts_with(&format!("{}-v", base_id)))
        .map(|s| StatuteVersionInfo {
            id: s.id.clone(),
            version: s.version,
            title: s.title.clone(),
            created_at: None, // Would need to track creation timestamps
        })
        .collect();

    if versions.is_empty() {
        return Err(ApiError::NotFound(format!(
            "No statutes found with base ID: {}",
            base_id
        )));
    }

    let total_versions = versions.len();

    Ok(Json(ApiResponse::new(StatuteVersionListResponse {
        base_id,
        versions,
        total_versions,
    })))
}

/// Create a new version of an existing statute.
async fn create_statute_version(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateVersionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::CreateStatutes)?;

    let mut statutes = state.statutes.write().await;

    // Find the original statute
    let original = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?
        .clone();

    // Find the highest version number for this base statute
    let base_id = if original.id.contains("-v") {
        original.id.split("-v").next().unwrap_or(&original.id)
    } else {
        &original.id
    };

    let max_version = statutes
        .iter()
        .filter(|s| s.id == base_id || s.id.starts_with(&format!("{}-v", base_id)))
        .map(|s| s.version)
        .max()
        .unwrap_or(original.version);

    let new_version = max_version + 1;
    let new_id = format!("{}-v{}", base_id, new_version);

    // Check if new ID already exists
    if statutes.iter().any(|s| s.id == new_id) {
        return Err(ApiError::BadRequest(format!(
            "Statute version already exists: {}",
            new_id
        )));
    }

    // Create new version based on original with optional modifications
    let mut new_statute = original.clone();
    new_statute.id = new_id.clone();
    new_statute.version = new_version;

    if let Some(title) = req.title {
        new_statute.title = title;
    }

    if let Some(preconditions) = req.preconditions {
        new_statute.preconditions = preconditions;
    }

    if let Some(effect) = req.effect {
        new_statute.effect = effect;
    }

    if let Some(discretion) = req.discretion_logic {
        new_statute.discretion_logic = Some(discretion);
    }

    info!(
        "Creating statute version: {} (v{}) by user {}",
        new_id, new_version, user.username
    );
    statutes.push(new_statute.clone());

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::StatuteVersionCreated,
            user.id.to_string(),
            user.username.clone(),
            "create_statute_version".to_string(),
            Some(new_id.clone()),
            Some("statute".to_string()),
            serde_json::json!({
                "statute_id": new_id,
                "version": new_version,
                "base_id": base_id
            }),
        )
        .await;

    Ok((StatusCode::CREATED, Json(ApiResponse::new(new_statute))))
}

/// Helper function to calculate nesting depth of conditions.
fn calculate_nesting_depth(conditions: &[legalis_core::Condition]) -> usize {
    use legalis_core::Condition;

    fn depth_of_condition(cond: &Condition) -> usize {
        match cond {
            Condition::And(left, right) | Condition::Or(left, right) => {
                1 + depth_of_condition(left).max(depth_of_condition(right))
            }
            Condition::Not(inner) => 1 + depth_of_condition(inner),
            _ => 0,
        }
    }

    conditions.iter().map(depth_of_condition).max().unwrap_or(0)
}

/// Run a simulation on statutes with a generated population.
async fn run_simulation(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SimulationRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    if req.population_size == 0 {
        return Err(ApiError::BadRequest(
            "Population size must be greater than 0".to_string(),
        ));
    }

    if req.population_size > 10000 {
        return Err(ApiError::BadRequest(
            "Population size cannot exceed 10000".to_string(),
        ));
    }

    let statutes = state.statutes.read().await;

    let to_simulate: Vec<Statute> = if req.statute_ids.is_empty() {
        statutes.clone()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .cloned()
            .collect()
    };

    if to_simulate.is_empty() {
        return Err(ApiError::BadRequest("No statutes to simulate".to_string()));
    }

    // Create population
    use legalis_core::{LegalEntity, TypedEntity};
    let mut population: Vec<Box<dyn LegalEntity>> = Vec::new();
    for i in 0..req.population_size {
        let mut entity = TypedEntity::new();

        // Set age with some variation
        entity.set_u32("age", 18 + (i % 50) as u32);

        // Set income with variation
        entity.set_u64("income", 20000 + ((i * 1000) % 80000) as u64);

        // Apply custom entity parameters from request
        for (key, value) in &req.entity_params {
            entity.set_string(key, value);
        }

        population.push(Box::new(entity));
    }

    // Run simulation
    use legalis_sim::SimEngine;
    let engine = SimEngine::new(to_simulate.clone(), population);
    let sim_metrics = engine.run_simulation().await;

    // Update business metrics
    metrics::SIMULATIONS_TOTAL.inc();
    metrics::SIMULATION_OUTCOMES
        .with_label_values(&["deterministic"])
        .inc_by(sim_metrics.deterministic_count as u64);
    metrics::SIMULATION_OUTCOMES
        .with_label_values(&["discretionary"])
        .inc_by(sim_metrics.discretion_count as u64);
    metrics::SIMULATION_OUTCOMES
        .with_label_values(&["void"])
        .inc_by(sim_metrics.void_count as u64);

    let total = sim_metrics.total_applications as f64;
    let deterministic_rate = if total > 0.0 {
        (sim_metrics.deterministic_count as f64 / total) * 100.0
    } else {
        0.0
    };
    let discretionary_rate = if total > 0.0 {
        (sim_metrics.discretion_count as f64 / total) * 100.0
    } else {
        0.0
    };
    let void_rate = if total > 0.0 {
        (sim_metrics.void_count as f64 / total) * 100.0
    } else {
        0.0
    };

    Ok(Json(ApiResponse::new(SimulationResponse {
        simulation_id: uuid::Uuid::new_v4().to_string(),
        total_entities: req.population_size,
        deterministic_outcomes: sim_metrics.deterministic_count,
        discretionary_outcomes: sim_metrics.discretion_count,
        void_outcomes: sim_metrics.void_count,
        deterministic_rate,
        discretionary_rate,
        void_rate,
        completed_at: chrono::Utc::now().to_rfc3339(),
    })))
}

/// Stream simulation results in real-time using Server-Sent Events.
async fn stream_simulation(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SimulationRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    if req.population_size == 0 {
        return Err(ApiError::BadRequest(
            "Population size must be greater than 0".to_string(),
        ));
    }

    if req.population_size > 10000 {
        return Err(ApiError::BadRequest(
            "Population size cannot exceed 10000".to_string(),
        ));
    }

    let statutes = state.statutes.read().await;

    let to_simulate: Vec<Statute> = if req.statute_ids.is_empty() {
        statutes.clone()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .cloned()
            .collect()
    };

    if to_simulate.is_empty() {
        return Err(ApiError::BadRequest("No statutes to simulate".to_string()));
    }

    drop(statutes); // Release the read lock

    // Create population
    use legalis_core::{LegalEntity, TypedEntity};
    let mut population: Vec<Box<dyn LegalEntity>> = Vec::new();
    for i in 0..req.population_size {
        let mut entity = TypedEntity::new();
        entity.set_u32("age", 18 + (i % 50) as u32);
        entity.set_u64("income", 20000 + ((i * 1000) % 80000) as u64);

        for (key, value) in &req.entity_params {
            entity.set_string(key, value);
        }

        population.push(Box::new(entity));
    }

    let simulation_id = uuid::Uuid::new_v4().to_string();
    let total_entities = req.population_size;

    // Create an async stream
    let stream = stream::unfold(
        (
            to_simulate,
            population,
            0usize,
            simulation_id.clone(),
            total_entities,
        ),
        |(statutes, population, progress, sim_id, total_entities)| async move {
            if progress == 0 {
                // Send start event
                let event = Event::default()
                    .event("start")
                    .json_data(serde_json::json!({
                        "simulation_id": sim_id,
                        "total_entities": population.len(),
                        "status": "started"
                    }))
                    .ok()?;
                return Some((
                    Ok::<_, Infallible>(event),
                    (statutes, population, 10, sim_id, total_entities),
                ));
            }

            if progress < 100 {
                // Send progress update
                tokio::time::sleep(Duration::from_millis(100)).await;
                let event = Event::default()
                    .event("progress")
                    .json_data(serde_json::json!({
                        "simulation_id": sim_id,
                        "progress": progress,
                        "status": "running"
                    }))
                    .ok()?;
                return Some((
                    Ok::<_, Infallible>(event),
                    (statutes, population, progress + 10, sim_id, total_entities),
                ));
            }

            if progress == 100 {
                // Run actual simulation
                use legalis_sim::SimEngine;
                let engine = SimEngine::new(statutes.clone(), population);
                let metrics = engine.run_simulation().await;

                let total = metrics.total_applications as f64;
                let deterministic_rate = if total > 0.0 {
                    (metrics.deterministic_count as f64 / total) * 100.0
                } else {
                    0.0
                };
                let discretionary_rate = if total > 0.0 {
                    (metrics.discretion_count as f64 / total) * 100.0
                } else {
                    0.0
                };
                let void_rate = if total > 0.0 {
                    (metrics.void_count as f64 / total) * 100.0
                } else {
                    0.0
                };

                // Send completion event
                let event = Event::default()
                    .event("complete")
                    .json_data(serde_json::json!({
                        "simulation_id": sim_id,
                        "status": "completed",
                        "total_entities": total_entities,
                        "deterministic_outcomes": metrics.deterministic_count,
                        "discretionary_outcomes": metrics.discretion_count,
                        "void_outcomes": metrics.void_count,
                        "deterministic_rate": deterministic_rate,
                        "discretionary_rate": discretionary_rate,
                        "void_rate": void_rate,
                        "completed_at": chrono::Utc::now().to_rfc3339()
                    }))
                    .ok()?;
                return Some((
                    Ok::<_, Infallible>(event),
                    (statutes, vec![], 101, sim_id, total_entities),
                ));
            }

            None
        },
    );

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// Compare two simulation scenarios.
async fn compare_simulations(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SimulationComparisonRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    if req.population_size == 0 || req.population_size > 10000 {
        return Err(ApiError::BadRequest(
            "Population size must be between 1 and 10000".to_string(),
        ));
    }

    let statutes = state.statutes.read().await;

    let statutes_a: Vec<Statute> = statutes
        .iter()
        .filter(|s| req.statute_ids_a.contains(&s.id))
        .cloned()
        .collect();

    let statutes_b: Vec<Statute> = statutes
        .iter()
        .filter(|s| req.statute_ids_b.contains(&s.id))
        .cloned()
        .collect();

    if statutes_a.is_empty() || statutes_b.is_empty() {
        return Err(ApiError::BadRequest(
            "Both scenarios must have at least one statute".to_string(),
        ));
    }

    // Helper function to create population
    fn create_population(size: usize) -> Vec<Box<dyn legalis_core::LegalEntity>> {
        use legalis_core::TypedEntity;
        let mut population: Vec<Box<dyn legalis_core::LegalEntity>> = Vec::new();
        for i in 0..size {
            let mut entity = TypedEntity::new();
            entity.set_u32("age", 18 + (i % 50) as u32);
            entity.set_u64("income", 20000 + ((i * 1000) % 80000) as u64);
            population.push(Box::new(entity));
        }
        population
    }

    // Run scenario A
    use legalis_sim::SimEngine;
    let population_a = create_population(req.population_size);
    let engine_a = SimEngine::new(statutes_a, population_a);
    let metrics_a = engine_a.run_simulation().await;

    // Run scenario B
    let population_b = create_population(req.population_size);
    let engine_b = SimEngine::new(statutes_b, population_b);
    let metrics_b = engine_b.run_simulation().await;

    let total = req.population_size as f64;

    let det_rate_a = (metrics_a.deterministic_count as f64 / total) * 100.0;
    let disc_rate_a = (metrics_a.discretion_count as f64 / total) * 100.0;
    let void_rate_a = (metrics_a.void_count as f64 / total) * 100.0;

    let det_rate_b = (metrics_b.deterministic_count as f64 / total) * 100.0;
    let disc_rate_b = (metrics_b.discretion_count as f64 / total) * 100.0;
    let void_rate_b = (metrics_b.void_count as f64 / total) * 100.0;

    let det_diff = det_rate_b - det_rate_a;
    let disc_diff = disc_rate_b - disc_rate_a;
    let void_diff = void_rate_b - void_rate_a;

    // Consider change significant if any rate changes by more than 10%
    let significant_change =
        det_diff.abs() > 10.0 || disc_diff.abs() > 10.0 || void_diff.abs() > 10.0;

    Ok(Json(ApiResponse::new(SimulationComparisonResponse {
        scenario_a: SimulationScenarioResult {
            name: "Scenario A".to_string(),
            deterministic_rate: det_rate_a,
            discretionary_rate: disc_rate_a,
            void_rate: void_rate_a,
        },
        scenario_b: SimulationScenarioResult {
            name: "Scenario B".to_string(),
            deterministic_rate: det_rate_b,
            discretionary_rate: disc_rate_b,
            void_rate: void_rate_b,
        },
        differences: SimulationDifferences {
            deterministic_diff: det_diff,
            discretionary_diff: disc_diff,
            void_diff,
            significant_change,
        },
    })))
}

/// Check compliance of a specific entity against statutes.
async fn check_compliance(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ComplianceCheckRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    let statutes = state.statutes.read().await;

    let to_check: Vec<Statute> = if req.statute_ids.is_empty() {
        statutes.clone()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .cloned()
            .collect()
    };

    if to_check.is_empty() {
        return Err(ApiError::BadRequest("No statutes to check".to_string()));
    }

    drop(statutes);

    // Create entity from provided attributes
    use legalis_core::TypedEntity;
    let mut entity = TypedEntity::new();
    for (key, value) in &req.entity_attributes {
        // Try to parse as different types
        if let Ok(num) = value.parse::<u32>() {
            entity.set_u32(key, num);
        } else if let Ok(num) = value.parse::<u64>() {
            entity.set_u64(key, num);
        } else {
            entity.set_string(key, value);
        }
    }

    // Check compliance by simulating with single entity
    use legalis_sim::SimEngine;
    let population: Vec<Box<dyn legalis_core::LegalEntity>> = vec![Box::new(entity)];
    let engine = SimEngine::new(to_check.clone(), population);
    let metrics = engine.run_simulation().await;

    let compliant = metrics.deterministic_count > 0;
    let requires_discretion = metrics.discretion_count > 0;
    let not_applicable = metrics.void_count > 0;

    // Determine which statutes apply
    let applicable_statutes: Vec<String> = to_check.iter().map(|s| s.id.clone()).collect();

    Ok(Json(ApiResponse::new(ComplianceCheckResponse {
        compliant,
        requires_discretion,
        not_applicable,
        applicable_statutes,
        checked_statute_count: to_check.len(),
    })))
}

/// Perform what-if analysis by comparing entity with modified attributes.
async fn whatif_analysis(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<WhatIfRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::VerifyStatutes)?;

    let statutes = state.statutes.read().await;

    let to_analyze: Vec<Statute> = if req.statute_ids.is_empty() {
        statutes.clone()
    } else {
        statutes
            .iter()
            .filter(|s| req.statute_ids.contains(&s.id))
            .cloned()
            .collect()
    };

    if to_analyze.is_empty() {
        return Err(ApiError::BadRequest(
            "No statutes for what-if analysis".to_string(),
        ));
    }

    drop(statutes);

    // Helper to create entity from attributes
    fn create_entity(
        attributes: &std::collections::HashMap<String, String>,
    ) -> legalis_core::TypedEntity {
        use legalis_core::TypedEntity;
        let mut entity = TypedEntity::new();
        for (key, value) in attributes {
            if let Ok(num) = value.parse::<u32>() {
                entity.set_u32(key, num);
            } else if let Ok(num) = value.parse::<u64>() {
                entity.set_u64(key, num);
            } else {
                entity.set_string(key, value);
            }
        }
        entity
    }

    // Baseline scenario
    let baseline_entity = create_entity(&req.baseline_attributes);
    let baseline_pop: Vec<Box<dyn legalis_core::LegalEntity>> = vec![Box::new(baseline_entity)];

    use legalis_sim::SimEngine;
    let baseline_engine = SimEngine::new(to_analyze.clone(), baseline_pop);
    let baseline_metrics = baseline_engine.run_simulation().await;

    // Modified scenario
    let modified_entity = create_entity(&req.modified_attributes);
    let modified_pop: Vec<Box<dyn legalis_core::LegalEntity>> = vec![Box::new(modified_entity)];

    let modified_engine = SimEngine::new(to_analyze.clone(), modified_pop);
    let modified_metrics = modified_engine.run_simulation().await;

    let baseline_compliant = baseline_metrics.deterministic_count > 0;
    let modified_compliant = modified_metrics.deterministic_count > 0;

    let impact = if baseline_compliant && !modified_compliant {
        "negative".to_string()
    } else if !baseline_compliant && modified_compliant {
        "positive".to_string()
    } else {
        "none".to_string()
    };

    Ok(Json(ApiResponse::new(WhatIfResponse {
        baseline_compliant,
        modified_compliant,
        impact,
        baseline_requires_discretion: baseline_metrics.discretion_count > 0,
        modified_requires_discretion: modified_metrics.discretion_count > 0,
        changed_attribute_count: req.modified_attributes.len(),
    })))
}

/// Save a simulation result for later retrieval.
async fn save_simulation(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SaveSimulationRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::CreateStatutes)?;

    let saved = SavedSimulation {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        description: req.description,
        statute_ids: vec![], // Would need to track in SimulationResponse
        population_size: req.simulation_result.total_entities,
        deterministic_outcomes: req.simulation_result.deterministic_outcomes,
        discretionary_outcomes: req.simulation_result.discretionary_outcomes,
        void_outcomes: req.simulation_result.void_outcomes,
        deterministic_rate: req.simulation_result.deterministic_rate,
        discretionary_rate: req.simulation_result.discretionary_rate,
        void_rate: req.simulation_result.void_rate,
        created_at: chrono::Utc::now().to_rfc3339(),
        created_by: user.username.clone(),
    };

    let mut simulations = state.saved_simulations.write().await;
    simulations.push(saved.clone());

    info!("Saved simulation: {} by user {}", saved.id, user.username);

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::SimulationSaved,
            user.id.to_string(),
            user.username.clone(),
            "save_simulation".to_string(),
            Some(saved.id.clone()),
            Some("simulation".to_string()),
            serde_json::json!({
                "simulation_id": saved.id,
                "name": saved.name
            }),
        )
        .await;

    Ok((StatusCode::CREATED, Json(ApiResponse::new(saved))))
}

/// List all saved simulations.
async fn list_saved_simulations(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListSavedSimulationsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let simulations = state.saved_simulations.read().await;
    let total = simulations.len();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(100).min(1000);

    let paginated: Vec<SavedSimulation> = simulations
        .iter()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect();

    let meta = ResponseMeta {
        total: Some(total),
        page: Some(offset / limit),
        per_page: Some(limit),
        next_cursor: None,
        prev_cursor: None,
        has_more: None,
    };

    Ok(Json(ApiResponse::new(paginated).with_meta(meta)))
}

/// Get a specific saved simulation.
async fn get_saved_simulation(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let simulations = state.saved_simulations.read().await;
    let simulation = simulations
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Saved simulation not found: {}", id)))?;

    Ok(Json(ApiResponse::new(simulation.clone())))
}

/// Delete a saved simulation.
async fn delete_saved_simulation(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::DeleteStatutes)?;

    let mut simulations = state.saved_simulations.write().await;
    let initial_len = simulations.len();
    simulations.retain(|s| s.id != id);

    if simulations.len() == initial_len {
        return Err(ApiError::NotFound(format!(
            "Saved simulation not found: {}",
            id
        )));
    }

    info!("Deleted saved simulation: {} by user {}", id, user.username);

    // Audit log
    state
        .audit_log
        .log_success(
            audit::AuditEventType::SimulationDeleted,
            user.id.to_string(),
            user.username.clone(),
            "delete_saved_simulation".to_string(),
            Some(id.clone()),
            Some("simulation".to_string()),
            serde_json::json!({
                "simulation_id": id
            }),
        )
        .await;

    Ok(StatusCode::NO_CONTENT)
}

/// Visualize a statute in various formats.
async fn visualize_statute(
    user: auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<VizQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(auth::Permission::ReadStatutes)?;

    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;

    // Create decision tree
    let tree = DecisionTree::from_statute(statute)
        .map_err(|e| ApiError::Internal(format!("Visualization error: {}", e)))?;

    // Get theme
    let theme = match query.theme.as_deref() {
        Some("dark") => legalis_viz::Theme::dark(),
        Some("high_contrast") => legalis_viz::Theme::high_contrast(),
        Some("colorblind_friendly") => legalis_viz::Theme::colorblind_friendly(),
        _ => legalis_viz::Theme::light(),
    };

    // Generate visualization based on format
    let (content, format_str) = match query.format {
        VizFormat::Dot => (tree.to_dot(), "dot"),
        VizFormat::Ascii => (tree.to_ascii(), "ascii"),
        VizFormat::Mermaid => (tree.to_mermaid(), "mermaid"),
        VizFormat::PlantUml => (tree.to_plantuml(), "plantuml"),
        VizFormat::Svg => (tree.to_svg_with_theme(&theme), "svg"),
        VizFormat::Html => (tree.to_html_with_theme(&theme), "html"),
    };

    Ok(Json(ApiResponse::new(VisualizationResponse {
        statute_id: id,
        format: format_str.to_string(),
        content,
        node_count: tree.node_count(),
        discretionary_count: tree.discretionary_count(),
    })))
}

/// Server configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

impl ServerConfig {
    /// Returns the bind address.
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    #[allow(unused_imports)]
    use legalis_core::{Effect, EffectType};
    use tower::ServiceExt;

    fn create_test_router() -> Router {
        let state = Arc::new(AppState::new());
        create_router(state)
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = create_test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_statutes_empty() {
        let app = create_test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/statutes")
                    .header("Authorization", "ApiKey lgl_12345678901234567890")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_statutes_unauthorized() {
        let app = create_test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/statutes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_statute_search() {
        let state = Arc::new(AppState::new());

        // Add test statute directly to state
        {
            let mut statutes = state.statutes.write().await;
            statutes.push(
                Statute::new(
                    "search-test-1",
                    "Searchable Statute",
                    Effect::new(EffectType::Grant, "Test grant"),
                )
                .with_jurisdiction("TEST"),
            );
        }

        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/statutes/search?title=Searchable")
                    .header("Authorization", "ApiKey lgl_12345678901234567890")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(!json["data"]["statutes"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_graphql_integration() {
        // GraphQL create and query test - uses GraphQL schema
        let state = graphql::GraphQLState::new();
        let schema = graphql::create_schema(state);

        // Create an admin user for testing
        use auth::{AuthMethod, AuthUser, Role};
        let admin_user = AuthUser::new(
            uuid::Uuid::new_v4(),
            "admin".to_string(),
            Role::Admin,
            AuthMethod::Jwt,
        );

        let mutation = r#"
            mutation {
                createStatute(input: {
                    id: "graphql-test-1"
                    title: "GraphQL Test Statute"
                    effectDescription: "Test benefit"
                    effectType: "Grant"
                    jurisdiction: "TEST"
                }) {
                    id
                    title
                }
            }
        "#;

        let request = async_graphql::Request::new(mutation).data(admin_user);
        let result = schema.execute(request).await;
        assert!(result.errors.is_empty());

        let query = r#"
            {
                statutes {
                    id
                    title
                }
            }
        "#;

        let result = schema.execute(query).await;
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_readiness_check() {
        let app = create_test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_metrics_endpoint() {
        let app = create_test_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
