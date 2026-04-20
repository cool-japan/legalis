//! Auto-generated module: functions_2 for legalis-api.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::stream::{self, Stream};
use legalis_core::Statute;
use legalis_viz::DecisionTree;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use super::types::{
    ApiError, ApiResponse, AppState, ComplianceCheckRequest, ComplianceCheckResponse,
    ListSavedSimulationsQuery, ResponseMeta, SaveSimulationRequest, SavedSimulation,
    SimulationComparisonRequest, SimulationComparisonResponse, SimulationDifferences,
    SimulationRequest, SimulationScenarioResult, VisualizationResponse, VizFormat, VizQuery,
    WhatIfRequest, WhatIfResponse,
};

/// Stream simulation results in real-time using Server-Sent Events.
pub(super) async fn stream_simulation(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SimulationRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
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
    drop(statutes);
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
                let event = Event::default()
                    .event("start")
                    .json_data(serde_json::json!(
                        { "simulation_id" : sim_id, "total_entities" : population
                        .len(), "status" : "started" }
                    ))
                    .ok()?;
                return Some((
                    Ok::<_, Infallible>(event),
                    (statutes, population, 10, sim_id, total_entities),
                ));
            }
            if progress < 100 {
                tokio::time::sleep(Duration::from_millis(100)).await;
                let event = Event::default()
                    .event("progress")
                    .json_data(serde_json::json!(
                        { "simulation_id" : sim_id, "progress" : progress, "status" :
                        "running" }
                    ))
                    .ok()?;
                return Some((
                    Ok::<_, Infallible>(event),
                    (statutes, population, progress + 10, sim_id, total_entities),
                ));
            }
            if progress == 100 {
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
                let event = Event::default()
                    .event("complete")
                    .json_data(serde_json::json!(
                        { "simulation_id" : sim_id, "status" : "completed",
                        "total_entities" : total_entities, "deterministic_outcomes" :
                        metrics.deterministic_count, "discretionary_outcomes" :
                        metrics.discretion_count, "void_outcomes" : metrics
                        .void_count, "deterministic_rate" : deterministic_rate,
                        "discretionary_rate" : discretionary_rate, "void_rate" :
                        void_rate, "completed_at" : chrono::Utc::now().to_rfc3339() }
                    ))
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
pub(super) async fn compare_simulations(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SimulationComparisonRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    use legalis_sim::SimEngine;
    let population_a = create_population(req.population_size);
    let engine_a = SimEngine::new(statutes_a, population_a);
    let metrics_a = engine_a.run_simulation().await;
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
pub(super) async fn check_compliance(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ComplianceCheckRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    use legalis_core::TypedEntity;
    let mut entity = TypedEntity::new();
    for (key, value) in &req.entity_attributes {
        if let Ok(num) = value.parse::<u32>() {
            entity.set_u32(key, num);
        } else if let Ok(num) = value.parse::<u64>() {
            entity.set_u64(key, num);
        } else {
            entity.set_string(key, value);
        }
    }
    use legalis_sim::SimEngine;
    let population: Vec<Box<dyn legalis_core::LegalEntity>> = vec![Box::new(entity)];
    let engine = SimEngine::new(to_check.clone(), population);
    let metrics = engine.run_simulation().await;
    let compliant = metrics.deterministic_count > 0;
    let requires_discretion = metrics.discretion_count > 0;
    let not_applicable = metrics.void_count > 0;
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
pub(super) async fn whatif_analysis(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<WhatIfRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::VerifyStatutes)?;
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
    let baseline_entity = create_entity(&req.baseline_attributes);
    let baseline_pop: Vec<Box<dyn legalis_core::LegalEntity>> = vec![Box::new(baseline_entity)];
    use legalis_sim::SimEngine;
    let baseline_engine = SimEngine::new(to_analyze.clone(), baseline_pop);
    let baseline_metrics = baseline_engine.run_simulation().await;
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
pub(super) async fn save_simulation(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SaveSimulationRequest>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::CreateStatutes)?;
    let saved = SavedSimulation {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        description: req.description,
        statute_ids: vec![],
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
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::SimulationSaved,
            user.id.to_string(),
            user.username.clone(),
            "save_simulation".to_string(),
            Some(saved.id.clone()),
            Some("simulation".to_string()),
            serde_json::json!({ "simulation_id" : saved.id, "name" : saved.name }),
        )
        .await;
    Ok((StatusCode::CREATED, Json(ApiResponse::new(saved))))
}
/// List all saved simulations.
pub(super) async fn list_saved_simulations(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListSavedSimulationsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
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
pub(super) async fn get_saved_simulation(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let simulations = state.saved_simulations.read().await;
    let simulation = simulations
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Saved simulation not found: {}", id)))?;
    Ok(Json(ApiResponse::new(simulation.clone())))
}
/// Delete a saved simulation.
pub(super) async fn delete_saved_simulation(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::DeleteStatutes)?;
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
    state
        .audit_log
        .log_success(
            crate::audit::AuditEventType::SimulationDeleted,
            user.id.to_string(),
            user.username.clone(),
            "delete_saved_simulation".to_string(),
            Some(id.clone()),
            Some("simulation".to_string()),
            serde_json::json!({ "simulation_id" : id }),
        )
        .await;
    Ok(StatusCode::NO_CONTENT)
}
/// Visualize a statute in various formats.
pub(super) async fn visualize_statute(
    user: crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<VizQuery>,
) -> Result<impl IntoResponse, ApiError> {
    user.require_permission(crate::auth::Permission::ReadStatutes)?;
    let statutes = state.statutes.read().await;
    let statute = statutes
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| ApiError::NotFound(format!("Statute not found: {}", id)))?;
    let tree = DecisionTree::from_statute(statute)
        .map_err(|e| ApiError::Internal(format!("Visualization error: {}", e)))?;
    let theme = match query.theme.as_deref() {
        Some("dark") => legalis_viz::Theme::dark(),
        Some("high_contrast") => legalis_viz::Theme::high_contrast(),
        Some("colorblind_friendly") => legalis_viz::Theme::colorblind_friendly(),
        _ => legalis_viz::Theme::light(),
    };
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_router;
    use axum::Router;
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
        let state = crate::graphql::GraphQLState::new();
        let schema = crate::graphql::create_schema(state);
        use crate::auth::{AuthMethod, AuthUser, Role};
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
