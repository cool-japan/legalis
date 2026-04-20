//! Trait implementations for legalis-api types.

use axum::{Json, http::StatusCode, response::IntoResponse};
use legalis_core::Statute;

use super::types::{ApiError, AppState, ErrorResponse, ServerConfig, StatuteSummary};

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

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
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
