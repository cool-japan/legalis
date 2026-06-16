//! HTTP route handlers for compliance/governance, security and intelligent
//! features, wiring the dedicated logic modules into the REST API.
//!
//! These handlers follow the crate's established conventions: they take an
//! [`crate::auth::AuthUser`] for authn/authz, operate on [`crate::types::AppState`]
//! via shared state, and return [`crate::types::ApiError`] on failure.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::audit::AuditQueryFilter;
use crate::audit_export::{self, ExportFormat};
use crate::auth::{AuthUser, Permission};
use crate::consent::ConsentStore;
use crate::data_classification::Classification;
use crate::regulatory_reporting::{self, ReportPeriod};
use crate::types::{ApiError, ApiResponse, AppState};

/// Query parameters for audit export.
#[derive(Debug, Deserialize)]
pub struct AuditExportQuery {
    /// Export format: `json`, `ndjson`, or `csv`. Defaults to `json`.
    pub format: Option<String>,
    /// Optional user-id filter.
    pub user_id: Option<String>,
}

/// Exports audit records in the requested format.
///
/// Requires the `Admin` permission. The response is a downloadable document with
/// the appropriate content type and a `Content-Disposition` attachment header.
async fn export_audit(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuditExportQuery>,
) -> Result<Response, ApiError> {
    user.require_permission(Permission::Admin)?;

    let format = match query.format.as_deref() {
        None => ExportFormat::Json,
        Some(name) => ExportFormat::from_name(name)
            .ok_or_else(|| ApiError::BadRequest(format!("unknown export format: {name}")))?,
    };

    let filter = AuditQueryFilter {
        user_id: query.user_id.clone(),
        ..Default::default()
    };
    let entries = state.audit_log.query(filter).await;

    let (body, content_type) = audit_export::export(&entries, format)
        .map_err(|e| ApiError::Internal(format!("audit export failed: {e}")))?;

    let filename = format!("audit-export.{}", format.extension());
    let disposition = format!("attachment; filename=\"{filename}\"");

    let mut response = (StatusCode::OK, body).into_response();
    let headers = response.headers_mut();
    if let Ok(ct) = HeaderValue::from_str(content_type) {
        headers.insert(header::CONTENT_TYPE, ct);
    }
    if let Ok(cd) = HeaderValue::from_str(&disposition) {
        headers.insert(header::CONTENT_DISPOSITION, cd);
    }
    Ok(response)
}

/// Query parameters for the regulatory report.
#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    /// Inclusive start of the reporting period (RFC 3339).
    pub start: chrono::DateTime<chrono::Utc>,
    /// Exclusive end of the reporting period (RFC 3339).
    pub end: chrono::DateTime<chrono::Utc>,
}

/// Generates a compliance report over the requested period.
///
/// Requires `ViewAnalytics`.
async fn compliance_report(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ReportQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::ViewAnalytics)?;
    let entries = state.audit_log.query(AuditQueryFilter::default()).await;
    let period = ReportPeriod::new(query.start, query.end);
    let report = regulatory_reporting::generate_report(&entries, period);
    Ok(Json(ApiResponse::new(report)))
}

/// Generates a per-actor activity report (data-subject access support).
///
/// Requires `Admin`.
async fn actor_report(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    Query(query): Query<ReportQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let entries = state.audit_log.query(AuditQueryFilter::default()).await;
    let period = ReportPeriod::new(query.start, query.end);
    let report = regulatory_reporting::generate_actor_report(&entries, &user_id, period);
    Ok(Json(ApiResponse::new(report)))
}

/// Request body for granting consent.
#[derive(Debug, Deserialize)]
pub struct GrantConsentRequest {
    /// Subject (data subject) identifier.
    pub subject_id: String,
    /// Processing purpose.
    pub purpose: String,
    /// Optional expiry (RFC 3339).
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Optional provenance.
    pub source: Option<String>,
}

/// Request body for withdrawing consent.
#[derive(Debug, Deserialize)]
pub struct WithdrawConsentRequest {
    /// Subject (data subject) identifier.
    pub subject_id: String,
    /// Processing purpose.
    pub purpose: String,
    /// Optional provenance.
    pub source: Option<String>,
}

/// Records a grant of consent. Requires `Admin`.
async fn grant_consent(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<GrantConsentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    if req.subject_id.is_empty() || req.purpose.is_empty() {
        return Err(ApiError::BadRequest(
            "subject_id and purpose are required".to_string(),
        ));
    }
    let record = state
        .consent_store
        .grant(req.subject_id, req.purpose, req.expires_at, req.source)
        .await;
    Ok((StatusCode::CREATED, Json(ApiResponse::new(record))))
}

/// Records a withdrawal of consent. Requires `Admin`.
async fn withdraw_consent(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<WithdrawConsentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let record = state
        .consent_store
        .withdraw(req.subject_id, req.purpose, req.source)
        .await;
    Ok(Json(ApiResponse::new(record)))
}

/// Returns the full consent history for a subject. Requires `Admin`.
async fn consent_history(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(subject_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let history = ConsentStore::history_for(&state.consent_store, &subject_id).await;
    Ok(Json(ApiResponse::new(history)))
}

/// Query parameters for checking consent status.
#[derive(Debug, Deserialize)]
pub struct ConsentCheckQuery {
    /// The purpose to check.
    pub purpose: String,
}

/// Checks whether a subject currently consents to a purpose. Requires `Admin`.
async fn check_consent(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(subject_id): Path<String>,
    Query(query): Query<ConsentCheckQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let now = chrono::Utc::now();
    let has = state
        .consent_store
        .has_consent(&subject_id, &query.purpose, now)
        .await;
    Ok(Json(ApiResponse::new(serde_json::json!({
        "subject_id": subject_id,
        "purpose": query.purpose,
        "consented": has,
    }))))
}

/// Request body for classifying a field.
#[derive(Debug, Deserialize)]
pub struct ClassifyRequest {
    /// Dotted field path.
    pub field_path: String,
    /// Classification label (`public`..`restricted`).
    pub classification: String,
}

/// Registers a data classification for a field path. Requires `Admin`.
async fn classify_field(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ClassifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let level = Classification::from_label(&req.classification).ok_or_else(|| {
        ApiError::BadRequest(format!("unknown classification: {}", req.classification))
    })?;
    let mut registry = state.classification.write().await;
    registry.classify(req.field_path.clone(), level);
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::new(serde_json::json!({
            "field_path": req.field_path,
            "classification": level.label(),
        }))),
    ))
}

/// Lists all registered data classifications. Requires `Admin`.
async fn list_classifications(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let registry = state.classification.read().await;
    let max = registry.max_classification();
    Ok(Json(ApiResponse::new(serde_json::json!({
        "count": registry.len(),
        "max_classification": max.label(),
    }))))
}

/// Returns the list of currently-flagged abusive clients. Requires `Admin`.
async fn abuse_status(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::Admin)?;
    let abusive = state.abuse_detector.abusive_clients().await;
    let entries: Vec<serde_json::Value> = abusive
        .into_iter()
        .map(|(client, assessment)| {
            serde_json::json!({
                "client": client,
                "score": assessment.score,
                "burst_score": assessment.burst_score,
                "error_score": assessment.error_score,
                "scanning_score": assessment.scanning_score,
                "sample_size": assessment.sample_size,
            })
        })
        .collect();
    Ok(Json(ApiResponse::new(serde_json::json!({
        "abusive_clients": entries,
    }))))
}

/// Returns the predictive-cache model statistics. Requires `ViewAnalytics`.
async fn predictive_cache_stats(
    user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(Permission::ViewAnalytics)?;
    let stats = state.predictive_cache.stats().await;
    Ok(Json(ApiResponse::new(stats)))
}

/// Builds the governance / security / intelligent feature sub-router.
///
/// This router is merged into the main application router and shares its
/// [`AppState`]; it does not call `.with_state` itself.
pub fn governance_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/audit/export", get(export_audit))
        .route("/api/v1/reports/compliance", get(compliance_report))
        .route("/api/v1/reports/actors/{user_id}", get(actor_report))
        .route("/api/v1/consent/grant", post(grant_consent))
        .route("/api/v1/consent/withdraw", post(withdraw_consent))
        .route("/api/v1/consent/{subject_id}/history", get(consent_history))
        .route("/api/v1/consent/{subject_id}/check", get(check_consent))
        .route(
            "/api/v1/governance/classifications",
            get(list_classifications).post(classify_field),
        )
        .route("/api/v1/governance/abuse", get(abuse_status))
        .route(
            "/api/v1/governance/predictive-cache",
            get(predictive_cache_stats),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Role;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn admin_app() -> Router {
        let state = Arc::new(AppState::new());
        governance_router().with_state(state)
    }

    async fn body_json(resp: Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("collect body");
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
    }

    // A valid JWT-style bearer token (handlers treat any sufficiently long token
    // as the Admin user via the existing auth extractor).
    const ADMIN_BEARER: &str = "Bearer admin_jwt_token_placeholder";

    #[test]
    fn test_role_admin_has_required_perms() {
        // The auth extractor yields Role::Admin for bearer tokens; confirm it can
        // satisfy the permissions these routes demand.
        assert!(Role::Admin.has_permission(Permission::Admin));
        assert!(Role::Admin.has_permission(Permission::ViewAnalytics));
    }

    #[tokio::test]
    async fn test_export_audit_requires_auth() {
        let app = admin_app();
        let req = Request::builder()
            .uri("/api/v1/audit/export")
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        // No credentials -> unauthorized.
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_export_audit_csv() {
        let app = admin_app();
        let req = Request::builder()
            .uri("/api/v1/audit/export?format=csv")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some("text/csv")
        );
        assert!(
            resp.headers()
                .get(header::CONTENT_DISPOSITION)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.contains("audit-export.csv"))
                .unwrap_or(false)
        );
    }

    #[tokio::test]
    async fn test_export_audit_unknown_format() {
        let app = admin_app();
        let req = Request::builder()
            .uri("/api/v1/audit/export?format=xml")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_consent_grant_and_check() {
        let state = Arc::new(AppState::new());
        let app = governance_router().with_state(state);

        // Grant consent.
        let grant_body = serde_json::json!({
            "subject_id": "subject-1",
            "purpose": "analytics"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/consent/grant")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(grant_body.to_string()))
            .expect("request");
        let resp = app.clone().oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Check consent.
        let req = Request::builder()
            .uri("/api/v1/consent/subject-1/check?purpose=analytics")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp).await;
        assert_eq!(json["data"]["consented"], true);
    }

    #[tokio::test]
    async fn test_consent_grant_validation() {
        let app = admin_app();
        let grant_body = serde_json::json!({ "subject_id": "", "purpose": "" });
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/consent/grant")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(grant_body.to_string()))
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_classify_field() {
        let app = admin_app();
        let body = serde_json::json!({
            "field_path": "author.email",
            "classification": "pii"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/governance/classifications")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_classify_field_invalid_level() {
        let app = admin_app();
        let body = serde_json::json!({
            "field_path": "x",
            "classification": "ultra-secret"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/governance/classifications")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_compliance_report() {
        let app = admin_app();
        let start = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();
        let end = (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339();
        let uri = format!(
            "/api/v1/reports/compliance?start={}&end={}",
            urlencoding(&start),
            urlencoding(&end)
        );
        let req = Request::builder()
            .uri(&uri)
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp).await;
        assert_eq!(json["data"]["total_events"], 0);
    }

    #[tokio::test]
    async fn test_abuse_status_empty() {
        let app = admin_app();
        let req = Request::builder()
            .uri("/api/v1/governance/abuse")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp).await;
        assert!(json["data"]["abusive_clients"].as_array().is_some());
    }

    #[tokio::test]
    async fn test_predictive_cache_stats() {
        let app = admin_app();
        let req = Request::builder()
            .uri("/api/v1/governance/predictive-cache")
            .header(header::AUTHORIZATION, ADMIN_BEARER)
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// Minimal percent-encoding for the RFC3339 timestamps used in test URIs
    /// (encodes `:` and `+`).
    fn urlencoding(s: &str) -> String {
        s.replace(':', "%3A").replace('+', "%2B")
    }
}
