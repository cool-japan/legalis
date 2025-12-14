//! Legalis-API: REST/GraphQL API server for Legalis-RS.
//!
//! This crate provides a web API for interacting with the Legalis-RS framework:
//! - CRUD operations for statutes
//! - Verification endpoints
//! - Simulation endpoints
//! - Registry queries
//! - OpenAPI 3.0 documentation
//! - Authentication and authorization (RBAC + ReBAC)

pub mod auth;
mod metrics;
mod openapi;
pub mod rate_limit;
pub mod rebac;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
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
}

/// Application state.
pub struct AppState {
    /// In-memory statute storage
    pub statutes: RwLock<Vec<Statute>>,
    /// ReBAC authorization engine
    pub rebac: RwLock<rebac::ReBACEngine>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            statutes: RwLock::new(Vec::new()),
            rebac: RwLock::new(rebac::ReBACEngine::new()),
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
#[derive(Serialize)]
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

/// Creates the API router.
pub fn create_router(state: Arc<AppState>) -> Router {
    // Initialize metrics
    metrics::init();

    Router::new()
        .route("/health", get(health_check))
        .route("/health/ready", get(readiness_check))
        .route("/metrics", get(metrics_endpoint))
        .route("/api/v1/statutes", get(list_statutes).post(create_statute))
        .route(
            "/api/v1/statutes/{id}",
            get(get_statute).delete(delete_statute),
        )
        .route("/api/v1/statutes/{id}/complexity", get(analyze_complexity))
        .route("/api/v1/verify", post(verify_statutes))
        .route("/api/v1/verify/detailed", post(verify_statutes_detailed))
        .route("/api/v1/verify/conflicts", post(detect_conflicts))
        .route("/api/v1/verify/batch", post(verify_batch))
        .route("/api/v1/simulate", post(run_simulation))
        .route("/api/v1/simulate/compare", post(compare_simulations))
        .route("/api-docs/openapi.json", get(openapi_spec))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Returns the OpenAPI 3.0 specification.
async fn openapi_spec() -> impl IntoResponse {
    Json(openapi::generate_spec())
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
    statutes.push(req.statute.clone());

    Ok((StatusCode::CREATED, Json(ApiResponse::new(req.statute))))
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

    Ok(Json(ApiResponse::new(SimulationResponse {
        simulation_id: uuid::Uuid::new_v4().to_string(),
        total_entities: req.population_size,
        deterministic_outcomes: metrics.deterministic_count,
        discretionary_outcomes: metrics.discretion_count,
        void_outcomes: metrics.void_count,
        deterministic_rate,
        discretionary_rate,
        void_rate,
        completed_at: chrono::Utc::now().to_rfc3339(),
    })))
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
}
