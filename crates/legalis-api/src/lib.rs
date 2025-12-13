//! Legalis-API: REST/GraphQL API server for Legalis-RS.
//!
//! This crate provides a web API for interacting with the Legalis-RS framework:
//! - CRUD operations for statutes
//! - Verification endpoints
//! - Simulation endpoints
//! - Registry queries
//! - OpenAPI 3.0 documentation

mod openapi;

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
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ValidationFailed(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
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
}

impl AppState {
    pub fn new() -> Self {
        Self {
            statutes: RwLock::new(Vec::new()),
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

/// Creates the API router.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/statutes", get(list_statutes).post(create_statute))
        .route(
            "/api/v1/statutes/{id}",
            get(get_statute).delete(delete_statute),
        )
        .route("/api/v1/verify", post(verify_statutes))
        .route("/api-docs/openapi.json", get(openapi_spec))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Returns the OpenAPI 3.0 specification.
async fn openapi_spec() -> impl IntoResponse {
    Json(openapi::generate_spec())
}

/// Health check endpoint.
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "legalis-api",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// List all statutes.
async fn list_statutes(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, ApiError> {
    let statutes = state.statutes.read().await;
    let summaries: Vec<StatuteSummary> = statutes.iter().map(StatuteSummary::from).collect();

    Ok(Json(ApiResponse::new(StatuteListResponse {
        statutes: summaries,
    })))
}

/// Get a specific statute.
async fn get_statute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;

    Ok(Json(ApiResponse::new(statute.clone())))
}

/// Create a new statute.
async fn create_statute(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateStatuteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut statutes = state.statutes.write().await;

    // Check for duplicate ID
    if statutes.iter().any(|s| s.id == req.statute.id) {
        return Err(ApiError::BadRequest(format!(
            "Statute with ID '{}' already exists",
            req.statute.id
        )));
    }

    info!("Creating statute: {}", req.statute.id);
    statutes.push(req.statute.clone());

    Ok((StatusCode::CREATED, Json(ApiResponse::new(req.statute))))
}

/// Delete a statute.
async fn delete_statute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut statutes = state.statutes.write().await;
    let initial_len = statutes.len();
    statutes.retain(|s| s.id != id);

    if statutes.len() == initial_len {
        return Err(ApiError::NotFound(format!("Statute not found: {}", id)));
    }

    info!("Deleted statute: {}", id);
    Ok(StatusCode::NO_CONTENT)
}

/// Verify statutes.
async fn verify_statutes(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
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
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
