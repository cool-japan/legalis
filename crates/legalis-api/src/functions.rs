//! Auto-generated module: functions for legalis-api.

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
use std::convert::Infallible;
use std::sync::Arc;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use tracing::info;

use super::functions_2::{
    check_compliance, compare_simulations, delete_saved_simulation, get_saved_simulation,
    list_saved_simulations, save_simulation, stream_simulation, visualize_statute, whatif_analysis,
};
use super::types::{
    ApiError, ApiKeyListResponse, ApiKeyResponse, ApiKeyRotationResponse, ApiResponse, AppState,
    AsyncVerifyStartResponse, BatchCreateStatutesRequest, BatchCreateStatutesResponse,
    BatchDeleteStatutesRequest, BatchDeleteStatutesResponse, BatchVerifyRequest,
    BatchVerifyResponse, BatchVerifyResult, ComparisonDifferences, ComparisonMatrixEntry,
    ComplexityResponse, ConflictDetectionRequest, ConflictDetectionResponse, ConflictInfo,
    CreateApiKeyRequest, CreateStatuteRequest, CreateVersionRequest, DetailedVerifyResponse,
    JobStatusResponse, ResponseMeta, SimulationRequest, SimulationResponse,
    StatuteComparisonMatrixRequest, StatuteComparisonMatrixResponse, StatuteComparisonRequest,
    StatuteComparisonResponse, StatuteListResponse, StatutePermissionEntry,
    StatutePermissionRequest, StatutePermissionsResponse, StatuteSearchQuery, StatuteSummary,
    StatuteVersionInfo, StatuteVersionListResponse, VerificationJobResult, VerifyRequest,
    VerifyResponse,
};

/// Get permissions for a specific statute.
async fn get_statute_permissions(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(statute_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    if !statutes.iter().any(|s| s.id == statute_id) {
        return Err(ApiError::NotFound(format!(
            "Statute not found: {}",
            statute_id
        )));
    }
    drop(statutes);
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(statute_id): Path<String>,
    Json(req): Json<StatutePermissionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageUsers)?;
    let statutes = state.statutes.read().await;
    if !statutes.iter().any(|s| s.id == statute_id) {
        return Err(ApiError::NotFound(format!(
            "Statute not found: {}",
            statute_id
        )));
    }
    drop(statutes);
    let target_user_id = uuid::Uuid::parse_str(&req.user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    statute_id.hash(&mut hasher);
    let hash_value = hasher.finish();
    let resource_uuid = uuid::Uuid::from_u128(hash_value as u128);
    let relation = match req.permission.as_str() {
        "owner" => crate::rebac::Relation::Owner,
        "editor" => crate::rebac::Relation::Editor,
        "viewer" => crate::rebac::Relation::Viewer,
        _ => {
            return Err(ApiError::BadRequest(format!(
                "Invalid permission type: {}. Must be one of: owner, editor, viewer",
                req.permission
            )));
        }
    };
    let mut rebac = state.rebac.write().await;
    let tuple = crate::rebac::RelationTuple::new(
        target_user_id,
        relation,
        crate::rebac::ResourceType::Statute,
        resource_uuid,
    );
    rebac.add_tuple(tuple);
    crate::metrics::PERMISSION_OPERATIONS
        .with_label_values(&["grant"])
        .inc();
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::PermissionGranted,
            user.id.to_string(),
            user.username.clone(),
            "grant_statute_permission".to_string(),
            Some(statute_id.clone()),
            Some("statute".to_string()),
            serde_json::json!(
                { "statute_id" : statute_id, "granted_to" : req.user_id, "permission" :
                req.permission }
            ),
        )
        .await;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(serde_json::json!(
            { "message" : "Permission granted successfully", "statute_id" :
            statute_id, "user_id" : req.user_id, "permission" : req.permission }
        ))),
    ))
}
/// Revoke permission on a statute from a user.
async fn revoke_statute_permission(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(statute_id): Path<String>,
    Json(req): Json<StatutePermissionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageUsers)?;
    let statutes = state.statutes.read().await;
    if !statutes.iter().any(|s| s.id == statute_id) {
        return Err(ApiError::NotFound(format!(
            "Statute not found: {}",
            statute_id
        )));
    }
    drop(statutes);
    let target_user_id = uuid::Uuid::parse_str(&req.user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    statute_id.hash(&mut hasher);
    let hash_value = hasher.finish();
    let resource_uuid = uuid::Uuid::from_u128(hash_value as u128);
    let relation = match req.permission.as_str() {
        "owner" => crate::rebac::Relation::Owner,
        "editor" => crate::rebac::Relation::Editor,
        "viewer" => crate::rebac::Relation::Viewer,
        _ => {
            return Err(ApiError::BadRequest(format!(
                "Invalid permission type: {}. Must be one of: owner, editor, viewer",
                req.permission
            )));
        }
    };
    let mut rebac = state.rebac.write().await;
    let tuple = crate::rebac::RelationTuple::new(
        target_user_id,
        relation,
        crate::rebac::ResourceType::Statute,
        resource_uuid,
    );
    rebac.remove_tuple(&tuple);
    crate::metrics::PERMISSION_OPERATIONS
        .with_label_values(&["revoke"])
        .inc();
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::PermissionRevoked,
            user.id.to_string(),
            user.username.clone(),
            "revoke_statute_permission".to_string(),
            Some(statute_id.clone()),
            Some("statute".to_string()),
            serde_json::json!(
                { "statute_id" : statute_id, "revoked_from" : req.user_id, "permission" :
                req.permission }
            ),
        )
        .await;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(serde_json::json!(
            { "message" : "Permission revoked successfully", "statute_id" :
            statute_id, "user_id" : req.user_id, "permission" : req.permission }
        ))),
    ))
}
/// Create a new API key.
async fn create_api_key(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageApiKeys)?;
    let scopes = if let Some(scope_strs) = req.scopes {
        let mut parsed_scopes = std::collections::HashSet::new();
        for scope_str in scope_strs {
            let permission = match scope_str.as_str() {
                "read_statutes" => crate::auth::Permission::ReadStatutes,
                "create_statutes" => crate::auth::Permission::CreateStatutes,
                "update_statutes" => crate::auth::Permission::UpdateStatutes,
                "delete_statutes" => crate::auth::Permission::DeleteStatutes,
                "verify_statutes" => crate::auth::Permission::VerifyStatutes,
                "run_simulations" => crate::auth::Permission::RunSimulations,
                "view_analytics" => crate::auth::Permission::ViewAnalytics,
                "manage_users" => crate::auth::Permission::ManageUsers,
                "manage_api_keys" => crate::auth::Permission::ManageApiKeys,
                "admin" => crate::auth::Permission::Admin,
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
    let api_key = if let Some(expires_in_days) = req.expires_in_days {
        crate::auth::ApiKey::with_expiration(req.name, user.id, req.role, expires_in_days)
    } else {
        crate::auth::ApiKey::with_scopes(req.name, user.id, req.role, scopes)
    };
    let key_id = api_key.id.to_string();
    let key_value = api_key.key.clone();
    let mut api_keys = state.api_keys.write().await;
    api_keys.push(api_key.clone());
    drop(api_keys);
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::ApiKeyCreated,
            user.id.to_string(),
            user.username.clone(),
            "create_api_key".to_string(),
            Some(key_id.clone()),
            Some("api_key".to_string()),
            serde_json::json!(
                { "key_id" : key_id, "name" : api_key.name, "role" : format!("{:?}",
                api_key.role) }
            ),
        )
        .await;
    let response = ApiKeyResponse {
        id: key_id,
        key: Some(key_value),
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageApiKeys)?;
    let api_keys = state.api_keys.read().await;
    let keys: Vec<ApiKeyResponse> = api_keys
        .iter()
        .filter(|key| {
            key.owner_id == user.id || user.has_permission(crate::auth::Permission::Admin)
        })
        .map(|key| ApiKeyResponse {
            id: key.id.to_string(),
            key: None,
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageApiKeys)?;
    let key_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid key ID format".to_string()))?;
    let api_keys = state.api_keys.read().await;
    let key = api_keys
        .iter()
        .find(|k| {
            k.id == key_id
                && (k.owner_id == user.id || user.has_permission(crate::auth::Permission::Admin))
        })
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;
    let response = ApiKeyResponse {
        id: key.id.to_string(),
        key: None,
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageApiKeys)?;
    let key_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid key ID format".to_string()))?;
    let mut api_keys = state.api_keys.write().await;
    let key_index = api_keys
        .iter()
        .position(|k| {
            k.id == key_id
                && (k.owner_id == user.id || user.has_permission(crate::auth::Permission::Admin))
        })
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;
    let key = api_keys.remove(key_index);
    drop(api_keys);
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::ApiKeyRevoked,
            user.id.to_string(),
            user.username.clone(),
            "revoke_api_key".to_string(),
            Some(key.id.to_string()),
            Some("api_key".to_string()),
            serde_json::json!({ "key_id" : key.id.to_string(), "name" : key.name }),
        )
        .await;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(serde_json::json!(
            { "message" : "API key revoked successfully", "key_id" : key.id
            .to_string() }
        ))),
    ))
}
/// Rotate an API key (creates a new key, deactivates the old one).
async fn rotate_api_key(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ManageApiKeys)?;
    let key_id = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid key ID format".to_string()))?;
    let mut api_keys = state.api_keys.write().await;
    let old_key = api_keys
        .iter_mut()
        .find(|k| {
            k.id == key_id
                && (k.owner_id == user.id || user.has_permission(crate::auth::Permission::Admin))
        })
        .ok_or_else(|| ApiError::NotFound("API key not found".to_string()))?;
    let new_key = old_key.rotate();
    let new_key_value = new_key.key.clone();
    old_key.active = false;
    api_keys.push(new_key.clone());
    drop(api_keys);
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::ApiKeyRotated,
            user.id.to_string(),
            user.username.clone(),
            "rotate_api_key".to_string(),
            Some(new_key.id.to_string()),
            Some("api_key".to_string()),
            serde_json::json!(
                { "old_key_id" : key_id.to_string(), "new_key_id" : new_key.id
                .to_string() }
            ),
        )
        .await;
    let response = ApiKeyRotationResponse {
        old_key_id: key_id.to_string(),
        new_key: ApiKeyResponse {
            id: new_key.id.to_string(),
            key: Some(new_key_value),
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Query(filter): Query<crate::audit::AuditQueryFilter>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::Admin)?;
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::Admin)?;
    let total_count = state.audit_log.count().await;
    let statute_created = state
        .audit_log
        .count_filtered(crate::audit::AuditQueryFilter {
            event_type: Some(crate::audit::AuditEventType::StatuteCreated),
            ..Default::default()
        })
        .await;
    let statute_deleted = state
        .audit_log
        .count_filtered(crate::audit::AuditQueryFilter {
            event_type: Some(crate::audit::AuditEventType::StatuteDeleted),
            ..Default::default()
        })
        .await;
    let simulations_saved = state
        .audit_log
        .count_filtered(crate::audit::AuditQueryFilter {
            event_type: Some(crate::audit::AuditEventType::SimulationSaved),
            ..Default::default()
        })
        .await;
    let stats = serde_json::json!(
        { "total_audit_entries" : total_count, "by_event_type" : { "statute_created" :
        statute_created, "statute_deleted" : statute_deleted, "simulations_saved" :
        simulations_saved } }
    );
    Ok(Json(ApiResponse::new(stats)))
}
/// GraphQL handler.
async fn graphql_handler(
    schema: axum::extract::Extension<crate::graphql::LegalisSchema>,
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
    crate::metrics::init();
    let graphql_state =
        crate::graphql::GraphQLState::with_broadcaster(state.ws_broadcaster.clone());
    let graphql_schema = crate::graphql::create_schema(graphql_state);
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
        .route("/ws", get(crate::websocket::ws_handler))
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
        .merge(crate::governance_routes::governance_router())
        .layer(Extension(graphql_schema))
        .layer(middleware::from_fn(crate::logging::log_request))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
/// Returns the OpenAPI 3.0 specification.
async fn openapi_spec() -> impl IntoResponse {
    Json(crate::openapi::generate_spec())
}
/// Returns the Swagger UI HTML page.
async fn swagger_ui() -> impl IntoResponse {
    axum::response::Html(crate::openapi::generate_swagger_ui_html())
}
/// Health check endpoint - liveness probe.
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!(
        { "status" : "healthy", "service" : "legalis-api", "version" :
        env!("CARGO_PKG_VERSION"), "timestamp" : chrono::Utc::now().to_rfc3339() }
    ))
}
/// Readiness check endpoint - checks if the service is ready to accept requests.
async fn readiness_check(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let statutes_available = state.statutes.try_read().is_ok();
    let rebac_available = state.rebac.try_read().is_ok();
    let is_ready = statutes_available && rebac_available;
    let response = serde_json::json!(
        { "status" : if is_ready { "ready" } else { "not_ready" }, "service" :
        "legalis-api", "version" : env!("CARGO_PKG_VERSION"), "timestamp" :
        chrono::Utc::now().to_rfc3339(), "checks" : { "statutes_store" : if
        statutes_available { "ok" } else { "unavailable" }, "rebac_engine" : if
        rebac_available { "ok" } else { "unavailable" } } }
    );
    if is_ready {
        Ok(Json(response))
    } else {
        Err(ApiError::Internal("Service not ready".to_string()))
    }
}
/// Prometheus metrics endpoint.
async fn metrics_endpoint() -> Result<String, ApiError> {
    crate::metrics::encode()
        .map_err(|e| ApiError::Internal(format!("Failed to encode metrics: {}", e)))
}
/// List all statutes.
async fn list_statutes(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    let summaries: Vec<StatuteSummary> = statutes.iter().map(StatuteSummary::from).collect();
    Ok(Json(ApiResponse::new(StatuteListResponse {
        statutes: summaries,
    })))
}
/// Search/filter statutes.
async fn search_statutes(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<StatuteSearchQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let _field_query = crate::field_selection::FieldsQuery {
        fields: query.fields.clone(),
    };
    let statutes = state.statutes.read().await;
    let mut filtered: Vec<&Statute> = statutes.iter().collect();
    if let Some(ref title_query) = query.title {
        let title_lower = title_query.to_lowercase();
        filtered.retain(|s| s.title.to_lowercase().contains(&title_lower));
    }
    if let Some(has_discretion) = query.has_discretion {
        filtered.retain(|s| s.discretion_logic.is_some() == has_discretion);
    }
    if let Some(min) = query.min_preconditions {
        filtered.retain(|s| s.preconditions.len() >= min);
    }
    if let Some(max) = query.max_preconditions {
        filtered.retain(|s| s.preconditions.len() <= max);
    }
    let total = filtered.len();
    let (paginated, meta) = if let Some(cursor) = query.cursor {
        let limit = query.limit.unwrap_or(100).min(1000);
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
        let cursor_pos = filtered
            .iter()
            .position(|s| s.id == cursor_id && s.version == cursor_version);
        let start_pos = cursor_pos.map(|p| p + 1).unwrap_or(0);
        let results: Vec<StatuteSummary> = filtered
            .iter()
            .skip(start_pos)
            .take(limit + 1)
            .map(|s| StatuteSummary::from(*s))
            .collect();
        let has_more = results.len() > limit;
        let mut final_results = results;
        if has_more {
            final_results.pop();
        }
        let next_cursor = if has_more && !final_results.is_empty() {
            let last = &final_results[final_results.len() - 1];
            Some(base64_encode(&format!("{}:{}", last.id, 1)))
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(request): Json<crate::ai_suggestions::SuggestionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    let statute_vec: Vec<_> = statutes.iter().cloned().collect();
    let engine = crate::ai_suggestions::SuggestionEngine::new();
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;
    Ok(Json(ApiResponse::new(statute.clone())))
}
/// Create a new statute.
async fn create_statute(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateStatuteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::CreateStatutes)?;
    let mut statutes = state.statutes.write().await;
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
    crate::metrics::STATUTE_OPERATIONS
        .with_label_values(&["create"])
        .inc();
    crate::metrics::STATUTES_TOTAL.inc();
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::StatuteCreated,
            user.id.to_string(),
            user.username.clone(),
            "create_statute".to_string(),
            Some(statute_id.clone()),
            Some("statute".to_string()),
            serde_json::json!({ "statute_id" : statute_id, "title" : statute_title }),
        )
        .await;
    state
        .ws_broadcaster
        .broadcast(crate::websocket::WsNotification::StatuteCreated {
            statute_id: statute_id.clone(),
            title: statute_title,
            created_by: user.username.clone(),
        });
    Ok((StatusCode::CREATED, Json(ApiResponse::new(req.statute))))
}
/// Compare multiple statutes in a matrix format.
async fn compare_statutes_matrix(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<StatuteComparisonMatrixRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
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
    let mut statute_list = Vec::new();
    for id in &req.statute_ids {
        if let Some(statute) = statutes.iter().find(|s| &s.id == id) {
            statute_list.push(statute.clone());
        } else {
            return Err(ApiError::NotFound(format!("Statute not found: {}", id)));
        }
    }
    let count = statute_list.len();
    let mut similarity_matrix = vec![vec![0.0; count]; count];
    let mut comparisons = Vec::new();
    for i in 0..count {
        for j in i..count {
            if i == j {
                similarity_matrix[i][j] = 100.0;
            } else {
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
                let mut similarity = 100.0;
                similarity -= (precondition_diff.abs() as f64) * 5.0;
                similarity -= (depth_diff.abs() as f64) * 10.0;
                if discretion_differs {
                    similarity -= 20.0;
                }
                similarity = similarity.clamp(0.0, 100.0);
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<StatuteComparisonRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
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
    let mut similarity_score = 100.0;
    let precond_diff = (precondition_count_b - precondition_count_a).abs() as f64;
    similarity_score -= precond_diff * 5.0;
    let depth_diff = (nesting_depth_b - nesting_depth_a).abs() as f64;
    similarity_score -= depth_diff * 10.0;
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchCreateStatutesRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::CreateStatutes)?;
    if req.statutes.is_empty() {
        return Err(ApiError::BadRequest("No statutes provided".to_string()));
    }
    let mut statutes = state.statutes.write().await;
    let mut created = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    let total_requested = req.statutes.len();
    for statute in req.statutes {
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
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::BatchStatutesCreated,
            user.id.to_string(),
            user.username.clone(),
            "batch_create_statutes".to_string(),
            None,
            Some("statute".to_string()),
            serde_json::json!(
                { "created" : created, "failed" : failed, "total" : total_requested }
            ),
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchDeleteStatutesRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::DeleteStatutes)?;
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
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::BatchStatutesDeleted,
            user.id.to_string(),
            user.username.clone(),
            "batch_delete_statutes".to_string(),
            None,
            Some("statute".to_string()),
            serde_json::json!(
                { "deleted" : deleted, "not_found" : not_found.len(), "total" :
                total_requested }
            ),
        )
        .await;
    Ok(Json(ApiResponse::new(BatchDeleteStatutesResponse {
        deleted,
        not_found,
    })))
}
/// Delete a statute.
async fn delete_statute(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::DeleteStatutes)?;
    let mut statutes = state.statutes.write().await;
    let initial_len = statutes.len();
    statutes.retain(|s| s.id != id);
    if statutes.len() == initial_len {
        return Err(ApiError::NotFound(format!("Statute not found: {}", id)));
    }
    info!("Deleted statute: {} by user {}", id, user.username);
    crate::metrics::STATUTE_OPERATIONS
        .with_label_values(&["delete"])
        .inc();
    crate::metrics::STATUTES_TOTAL.dec();
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::StatuteDeleted,
            user.id.to_string(),
            user.username.clone(),
            "delete_statute".to_string(),
            Some(id.clone()),
            Some("statute".to_string()),
            serde_json::json!({ "statute_id" : id }),
        )
        .await;
    state
        .ws_broadcaster
        .broadcast(crate::websocket::WsNotification::StatuteDeleted {
            statute_id: id.clone(),
            deleted_by: user.username.clone(),
        });
    Ok(StatusCode::NO_CONTENT)
}
/// Verify statutes.
async fn verify_statutes(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    crate::metrics::VERIFICATIONS_TOTAL.inc();
    crate::metrics::VERIFICATION_RESULTS
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ConflictDetectionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    for error in result.errors.iter() {
        let error_str = error.to_string();
        if error_str.contains("conflict") || error_str.contains("contradiction") {
            conflicts.push(ConflictInfo {
                statute_a_id: "statute-a".to_string(),
                statute_b_id: "statute-b".to_string(),
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
    let job_id = state.verification_jobs.create_job().await;
    let state_clone = Arc::clone(&state);
    let statute_ids = req.statute_ids.clone();
    let job_id_clone = job_id.clone();
    tokio::spawn(async move {
        let job_id = job_id_clone;
        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.set_running();
            })
            .await;
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
        state_clone
            .verification_jobs
            .update_job(&job_id, |job| {
                job.set_progress(30.0);
            })
            .await;
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
        state_clone.ws_broadcaster.broadcast(
            crate::websocket::WsNotification::VerificationCompleted {
                job_id: job_id.clone(),
                passed,
                errors_count,
                warnings_count,
            },
        );
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
    let job = state
        .verification_jobs
        .get_job(&job_id)
        .await
        .ok_or_else(|| ApiError::NotFound(format!("Job not found: {}", job_id)))?;
    let status_str = match job.status {
        crate::async_jobs::JobStatus::Pending => "pending",
        crate::async_jobs::JobStatus::Running => "running",
        crate::async_jobs::JobStatus::Completed => "completed",
        crate::async_jobs::JobStatus::Failed => "failed",
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchVerifyRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
    if req.jobs.is_empty() {
        return Err(ApiError::BadRequest(
            "No verification jobs provided".to_string(),
        ));
    }
    let statutes = state.statutes.read().await.clone();
    let stream = stream::unfold(
        (req.jobs, statutes, 0usize),
        |(mut jobs, statutes, processed)| async move {
            if processed == 0 {
                let event = Event::default()
                    .event("start")
                    .json_data(serde_json::json!(
                        { "total_jobs" : jobs.len(), "status" : "started" }
                    ))
                    .ok()?;
                return Some((Ok::<_, Infallible>(event), (jobs, statutes, processed)));
            }
            if jobs.is_empty() {
                let event = Event::default()
                    .event("complete")
                    .json_data(serde_json::json!(
                        { "status" : "completed", "total_processed" : processed }
                    ))
                    .ok()?;
                return Some((Ok::<_, Infallible>(event), (jobs, statutes, processed)));
            }
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
                .json_data(serde_json::json!(
                    { "job_index" : processed_count, "total_jobs" : processed_count +
                    jobs.len(), "result" : result, "progress" : (processed_count as
                    f64 / (processed_count + jobs.len()) as f64) * 100.0 }
                ))
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchVerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
    if req.jobs.is_empty() {
        return Err(ApiError::BadRequest(
            "No verification jobs provided".to_string(),
        ));
    }
    let statutes = state.statutes.read().await;
    let verifier = legalis_verifier::StatuteVerifier::new();
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;
    let precondition_count = statute.preconditions.len();
    let nesting_depth = calculate_nesting_depth(&statute.preconditions);
    let has_discretion = statute.discretion_logic.is_some();
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(base_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    let versions: Vec<StatuteVersionInfo> = statutes
        .iter()
        .filter(|s| s.id == base_id || s.id.starts_with(&format!("{}-v", base_id)))
        .map(|s| StatuteVersionInfo {
            id: s.id.clone(),
            version: s.version,
            title: s.title.clone(),
            created_at: None,
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
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<CreateVersionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::CreateStatutes)?;
    let mut statutes = state.statutes.write().await;
    let original = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?
        .clone();
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
    if statutes.iter().any(|s| s.id == new_id) {
        return Err(ApiError::BadRequest(format!(
            "Statute version already exists: {}",
            new_id
        )));
    }
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
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::StatuteVersionCreated,
            user.id.to_string(),
            user.username.clone(),
            "create_statute_version".to_string(),
            Some(new_id.clone()),
            Some("statute".to_string()),
            serde_json::json!(
                { "statute_id" : new_id, "version" : new_version, "base_id" : base_id }
            ),
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
pub(super) async fn run_simulation(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SimulationRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    use legalis_sim::SimEngine;
    let engine = SimEngine::new(to_simulate.clone(), population);
    let sim_metrics = engine.run_simulation().await;
    crate::metrics::SIMULATIONS_TOTAL.inc();
    crate::metrics::SIMULATION_OUTCOMES
        .with_label_values(&["deterministic"])
        .inc_by(sim_metrics.deterministic_count as u64);
    crate::metrics::SIMULATION_OUTCOMES
        .with_label_values(&["discretionary"])
        .inc_by(sim_metrics.discretion_count as u64);
    crate::metrics::SIMULATION_OUTCOMES
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
