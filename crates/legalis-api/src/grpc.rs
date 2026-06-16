//! gRPC service implementation for Legalis API.
//!
//! This module provides a gRPC interface with:
//! - Bidirectional streaming for real-time collaboration
//! - Server streaming for simulation progress
//! - gRPC-web support for browser clients
//! - Reflection API for service discovery
//! - Health checking protocol

#[cfg(feature = "grpc")]
pub mod service {
    use futures::FutureExt;
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tokio_stream::{Stream, wrappers::ReceiverStream};
    use tonic::{Request, Response, Status}; // For now_or_never()

    use legalis_core::{Effect, EffectType, Statute};
    use legalis_verifier::StatuteVerifier;

    // Include the generated protobuf code
    pub mod pb {
        tonic::include_proto!("legalis.v1");

        // File descriptor set for reflection (loaded from OUT_DIR)
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/legalis_descriptor.bin"));
    }

    use pb::legalis_service_server::{LegalisService, LegalisServiceServer};
    use pb::*;

    /// gRPC service state
    #[derive(Clone)]
    pub struct GrpcServiceState {
        pub statutes: Arc<RwLock<Vec<Statute>>>,
    }

    impl GrpcServiceState {
        pub fn new() -> Self {
            Self {
                statutes: Arc::new(RwLock::new(Vec::new())),
            }
        }
    }

    impl Default for GrpcServiceState {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Legalis gRPC service implementation
    pub struct LegalisGrpcService {
        state: GrpcServiceState,
    }

    impl LegalisGrpcService {
        pub fn new(state: GrpcServiceState) -> Self {
            Self { state }
        }

        /// Convert internal Statute to protobuf Statute
        fn statute_to_proto(statute: &Statute) -> pb::Statute {
            pb::Statute {
                id: statute.id.clone(),
                title: statute.title.clone(),
                version: statute.version as i32,
                jurisdiction: statute.jurisdiction.clone().unwrap_or_default(),
                effect: Some(pb::Effect {
                    effect_type: match statute.effect.effect_type {
                        EffectType::Grant => pb::EffectType::Grant as i32,
                        EffectType::Revoke => pb::EffectType::Revoke as i32,
                        EffectType::Obligation => pb::EffectType::Obligation as i32,
                        EffectType::Prohibition => pb::EffectType::Prohibition as i32,
                        // Map additional effect types to closest equivalent or obligation
                        EffectType::MonetaryTransfer => pb::EffectType::Obligation as i32,
                        EffectType::StatusChange => pb::EffectType::Obligation as i32,
                        EffectType::Custom => pb::EffectType::Obligation as i32,
                    },
                    description: statute.effect.description.clone(),
                    parameters: statute.effect.parameters.clone(),
                }),
                preconditions: statute
                    .preconditions
                    .iter()
                    .map(|p| pb::Precondition {
                        description: format!("{:?}", p),
                        condition: format!("{:?}", p),
                    })
                    .collect(),
                discretion_logic: statute.discretion_logic.clone().unwrap_or_default(),
                exceptions: statute
                    .exceptions
                    .iter()
                    .map(|e| pb::Exception {
                        description: e.description.clone(),
                        condition: format!("{:?}", e.condition),
                    })
                    .collect(),
                applies_to: statute.applies_to.clone(),
                derives_from: statute.derives_from.clone(),
                temporal_validity: Some(pb::TemporalValidity {
                    effective_from: statute
                        .temporal_validity
                        .effective_date
                        .map(|d| {
                            d.and_hms_opt(0, 0, 0)
                                .expect("invariant: 0,0,0 is a valid time")
                                .and_utc()
                                .timestamp()
                        })
                        .unwrap_or(0),
                    effective_until: statute
                        .temporal_validity
                        .expiry_date
                        .map(|d| {
                            d.and_hms_opt(0, 0, 0)
                                .expect("invariant: 0,0,0 is a valid time")
                                .and_utc()
                                .timestamp()
                        })
                        .unwrap_or(0),
                    temporal_modifiers: vec![], // Simplified for now
                }),
            }
        }
    }

    #[tonic::async_trait]
    impl LegalisService for LegalisGrpcService {
        async fn list_statutes(
            &self,
            request: Request<ListStatutesRequest>,
        ) -> Result<Response<ListStatutesResponse>, Status> {
            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            // Filter by jurisdiction if provided
            let filtered: Vec<_> = if !req.jurisdiction.is_empty() {
                statutes
                    .iter()
                    .filter(|s| s.jurisdiction.as_ref() == Some(&req.jurisdiction))
                    .collect()
            } else {
                statutes.iter().collect()
            };

            let total_count = filtered.len() as i32;

            // Cursor-based pagination: page_token is base64(offset as decimal string)
            let page_size = if req.page_size > 0 {
                req.page_size as usize
            } else {
                usize::MAX
            };
            let offset = if req.page_token.is_empty() {
                0usize
            } else {
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.page_token)
                    .ok()
                    .and_then(|b| String::from_utf8(b).ok())
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0)
            };

            let page: Vec<_> = filtered
                .iter()
                .skip(offset)
                .take(page_size)
                .map(|s| Self::statute_to_proto(s))
                .collect();

            let next_offset = offset + page.len();
            let next_page_token = if next_offset < filtered.len() {
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    next_offset.to_string().as_bytes(),
                )
            } else {
                String::new()
            };

            Ok(Response::new(ListStatutesResponse {
                statutes: page,
                next_page_token,
                total_count,
            }))
        }

        async fn get_statute(
            &self,
            request: Request<GetStatuteRequest>,
        ) -> Result<Response<GetStatuteResponse>, Status> {
            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            let statute = statutes
                .iter()
                .find(|s| s.id == req.statute_id)
                .ok_or_else(|| {
                    Status::not_found(format!("Statute not found: {}", req.statute_id))
                })?;

            Ok(Response::new(GetStatuteResponse {
                statute_id: statute.id.clone(),
                title: statute.title.clone(),
                content: format!("{:?}", statute.effect),
            }))
        }

        async fn create_statute(
            &self,
            request: Request<CreateStatuteRequest>,
        ) -> Result<Response<pb::Statute>, Status> {
            let req = request.into_inner();
            let mut statutes = self.state.statutes.write().await;

            // Check if statute already exists
            if statutes.iter().any(|s| s.id == req.id) {
                return Err(Status::already_exists(format!(
                    "Statute already exists: {}",
                    req.id
                )));
            }

            // Parse effect type
            let effect_type = match pb::EffectType::try_from(req.effect_type) {
                Ok(pb::EffectType::Unspecified) => {
                    return Err(Status::invalid_argument("Effect type must be specified"));
                }
                Ok(pb::EffectType::Grant) => EffectType::Grant,
                Ok(pb::EffectType::Revoke) => EffectType::Revoke,
                Ok(pb::EffectType::Obligation) => EffectType::Obligation,
                Ok(pb::EffectType::Prohibition) => EffectType::Prohibition,
                Err(_) => {
                    return Err(Status::invalid_argument("Invalid effect type"));
                }
            };

            // Create statute
            let mut statute = Statute::new(
                req.id.clone(),
                req.title.clone(),
                Effect::new(effect_type, req.effect_description),
            );

            if !req.jurisdiction.is_empty() {
                statute = statute.with_jurisdiction(req.jurisdiction);
            }

            if req.has_version {
                statute = statute.with_version(req.version as u32);
            }

            let proto_statute = Self::statute_to_proto(&statute);
            statutes.push(statute);

            Ok(Response::new(proto_statute))
        }

        async fn update_statute(
            &self,
            request: Request<UpdateStatuteRequest>,
        ) -> Result<Response<pb::Statute>, Status> {
            let req = request.into_inner();
            let mut statutes = self.state.statutes.write().await;

            let statute = statutes
                .iter_mut()
                .find(|s| s.id == req.id)
                .ok_or_else(|| Status::not_found(format!("Statute not found: {}", req.id)))?;

            if !req.title.is_empty() {
                statute.title = req.title;
            }

            if !req.jurisdiction.is_empty() {
                statute.jurisdiction = Some(req.jurisdiction);
            }

            if req.has_version {
                statute.version = req.version as u32;
            }

            Ok(Response::new(Self::statute_to_proto(statute)))
        }

        async fn delete_statute(
            &self,
            request: Request<DeleteStatuteRequest>,
        ) -> Result<Response<DeleteStatuteResponse>, Status> {
            let req = request.into_inner();
            let mut statutes = self.state.statutes.write().await;

            let initial_len = statutes.len();
            statutes.retain(|s| s.id != req.id);

            if statutes.len() == initial_len {
                return Err(Status::not_found(format!("Statute not found: {}", req.id)));
            }

            Ok(Response::new(DeleteStatuteResponse {
                success: true,
                message: format!("Statute {} deleted successfully", req.id),
            }))
        }

        async fn batch_create_statutes(
            &self,
            request: Request<BatchCreateStatutesRequest>,
        ) -> Result<Response<BatchCreateStatutesResponse>, Status> {
            let req = request.into_inner();
            let mut statutes = self.state.statutes.write().await;

            let mut created_statutes = Vec::new();
            let mut errors = Vec::new();

            for create_req in req.statutes {
                // Check if statute already exists
                if statutes.iter().any(|s| s.id == create_req.id) {
                    errors.push(format!("Statute already exists: {}", create_req.id));
                    continue;
                }

                // Parse effect type
                let effect_type = match pb::EffectType::try_from(create_req.effect_type) {
                    Ok(pb::EffectType::Unspecified) => {
                        errors.push(format!(
                            "Effect type must be specified for statute: {}",
                            create_req.id
                        ));
                        continue;
                    }
                    Ok(pb::EffectType::Grant) => EffectType::Grant,
                    Ok(pb::EffectType::Revoke) => EffectType::Revoke,
                    Ok(pb::EffectType::Obligation) => EffectType::Obligation,
                    Ok(pb::EffectType::Prohibition) => EffectType::Prohibition,
                    Err(_) => {
                        errors.push(format!(
                            "Invalid effect type for statute: {}",
                            create_req.id
                        ));
                        continue;
                    }
                };

                // Create statute
                let mut statute = Statute::new(
                    create_req.id.clone(),
                    create_req.title.clone(),
                    Effect::new(effect_type, create_req.effect_description),
                );

                if !create_req.jurisdiction.is_empty() {
                    statute = statute.with_jurisdiction(create_req.jurisdiction);
                }

                if create_req.has_version {
                    statute = statute.with_version(create_req.version as u32);
                }

                created_statutes.push(Self::statute_to_proto(&statute));
                statutes.push(statute);
            }

            Ok(Response::new(BatchCreateStatutesResponse {
                statutes: created_statutes,
                errors,
            }))
        }

        async fn verify_statutes(
            &self,
            request: Request<VerifyStatutesRequest>,
        ) -> Result<Response<VerificationResult>, Status> {
            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            let statutes_to_verify: Vec<_> = statutes
                .iter()
                .filter(|s| req.statute_ids.contains(&s.id))
                .cloned()
                .collect();

            if statutes_to_verify.is_empty() {
                return Err(Status::not_found("No statutes found with provided IDs"));
            }

            let verifier = StatuteVerifier::new();
            let result = verifier.verify(&statutes_to_verify);

            Ok(Response::new(VerificationResult {
                passed: result.passed,
                errors: result.errors.iter().map(|e| e.to_string()).collect(),
                warnings: result.warnings,
                suggestions: result.suggestions,
            }))
        }

        type StreamVerifyStatutesStream =
            Pin<Box<dyn Stream<Item = Result<VerificationResult, Status>> + Send>>;

        async fn stream_verify_statutes(
            &self,
            request: Request<StreamVerifyStatutesRequest>,
        ) -> Result<Response<Self::StreamVerifyStatutesStream>, Status> {
            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            let statutes_to_verify: Vec<_> = statutes
                .iter()
                .filter(|s| req.statute_ids.contains(&s.id))
                .cloned()
                .collect();

            let (tx, rx) = tokio::sync::mpsc::channel(4);

            // Spawn a task to verify statutes and stream results
            tokio::spawn(async move {
                let verifier = StatuteVerifier::new();

                // Verify each statute individually and stream the results
                for statute in statutes_to_verify {
                    let result = verifier.verify(&[statute]);

                    let verification_result = VerificationResult {
                        passed: result.passed,
                        errors: result.errors.iter().map(|e| e.to_string()).collect(),
                        warnings: result.warnings,
                        suggestions: result.suggestions,
                    };

                    if tx.send(Ok(verification_result)).await.is_err() {
                        break;
                    }
                }
            });

            Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
        }

        async fn run_simulation(
            &self,
            request: Request<RunSimulationRequest>,
        ) -> Result<Response<SimulationResult>, Status> {
            use legalis_sim::{PopulationBuilder, SimEngine};

            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            let statute = statutes
                .iter()
                .find(|s| s.id == req.statute_id)
                .ok_or_else(|| Status::not_found(format!("Statute not found: {}", req.statute_id)))?
                .clone();

            drop(statutes);

            let population_size = req
                .parameters
                .get("population_size")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(10)
                .clamp(1, 10_000);

            let population = PopulationBuilder::new()
                .generate_random(population_size)
                .build();

            let engine = SimEngine::new(vec![statute], population);
            let metrics = engine.run_simulation().await;

            let mut results = HashMap::new();
            results.insert(
                "total_applications".to_string(),
                metrics.total_applications.to_string(),
            );
            results.insert(
                "deterministic_count".to_string(),
                metrics.deterministic_count.to_string(),
            );
            results.insert(
                "discretion_count".to_string(),
                metrics.discretion_count.to_string(),
            );
            results.insert("void_count".to_string(), metrics.void_count.to_string());
            results.insert(
                "statute_count".to_string(),
                metrics.statute_metrics.len().to_string(),
            );
            results.insert(
                "discretion_agents_count".to_string(),
                metrics.discretion_agents.len().to_string(),
            );

            Ok(Response::new(SimulationResult {
                simulation_id: uuid::Uuid::new_v4().to_string(),
                success: true,
                results,
                errors: vec![],
            }))
        }

        type CollaborateOnStatuteStream =
            Pin<Box<dyn Stream<Item = Result<CollaborationMessage, Status>> + Send>>;

        async fn collaborate_on_statute(
            &self,
            request: Request<tonic::Streaming<CollaborationMessage>>,
        ) -> Result<Response<Self::CollaborateOnStatuteStream>, Status> {
            let mut in_stream = request.into_inner();
            let (tx, rx) = tokio::sync::mpsc::channel(4);

            // Spawn a task to handle bidirectional streaming
            tokio::spawn(async move {
                while let Ok(Some(msg)) = in_stream.message().await {
                    // Echo the message back (in a real implementation, this would
                    // broadcast to other collaborators)
                    if tx.send(Ok(msg)).await.is_err() {
                        break;
                    }
                }
            });

            Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
        }

        type StreamSimulationStream =
            Pin<Box<dyn Stream<Item = Result<SimulationProgress, Status>> + Send>>;

        async fn stream_simulation(
            &self,
            request: Request<RunSimulationRequest>,
        ) -> Result<Response<Self::StreamSimulationStream>, Status> {
            use legalis_sim::{PopulationBuilder, SimEngine};

            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            let statute = statutes
                .iter()
                .find(|s| s.id == req.statute_id)
                .ok_or_else(|| Status::not_found(format!("Statute not found: {}", req.statute_id)))?
                .clone();

            drop(statutes);

            let population_size = req
                .parameters
                .get("population_size")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(10)
                .clamp(1, 10_000);

            let (tx, rx) = tokio::sync::mpsc::channel(8);

            tokio::spawn(async move {
                let simulation_id = uuid::Uuid::new_v4().to_string();

                // Phase 0 — notify that we are initialising
                let _ = tx
                    .send(Ok(SimulationProgress {
                        simulation_id: simulation_id.clone(),
                        progress_percent: 0,
                        current_step: "Initialising simulation".to_string(),
                        result: None,
                    }))
                    .await;

                // Phase 25 — building population
                let population = PopulationBuilder::new()
                    .generate_random(population_size)
                    .build();

                let _ = tx
                    .send(Ok(SimulationProgress {
                        simulation_id: simulation_id.clone(),
                        progress_percent: 25,
                        current_step: format!("Built population of {} agents", population.len()),
                        result: None,
                    }))
                    .await;

                // Phase 50 — running the engine
                let _ = tx
                    .send(Ok(SimulationProgress {
                        simulation_id: simulation_id.clone(),
                        progress_percent: 50,
                        current_step: "Running statute application engine".to_string(),
                        result: None,
                    }))
                    .await;

                let engine = SimEngine::new(vec![statute], population);
                let metrics = engine.run_simulation().await;

                // Phase 75 — aggregating results
                let _ = tx
                    .send(Ok(SimulationProgress {
                        simulation_id: simulation_id.clone(),
                        progress_percent: 75,
                        current_step: "Aggregating results".to_string(),
                        result: None,
                    }))
                    .await;

                let mut results = HashMap::new();
                results.insert(
                    "total_applications".to_string(),
                    metrics.total_applications.to_string(),
                );
                results.insert(
                    "deterministic_count".to_string(),
                    metrics.deterministic_count.to_string(),
                );
                results.insert(
                    "discretion_count".to_string(),
                    metrics.discretion_count.to_string(),
                );
                results.insert("void_count".to_string(), metrics.void_count.to_string());
                results.insert(
                    "statute_count".to_string(),
                    metrics.statute_metrics.len().to_string(),
                );

                // Phase 100 — done, send final result
                let _ = tx
                    .send(Ok(SimulationProgress {
                        simulation_id: simulation_id.clone(),
                        progress_percent: 100,
                        current_step: "Simulation complete".to_string(),
                        result: Some(SimulationResult {
                            simulation_id: simulation_id.clone(),
                            success: true,
                            results,
                            errors: vec![],
                        }),
                    }))
                    .await;
            });

            Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
        }

        async fn search_statutes(
            &self,
            request: Request<SearchStatutesRequest>,
        ) -> Result<Response<SearchStatutesResponse>, Status> {
            let req = request.into_inner();
            let statutes = self.state.statutes.read().await;

            let filtered: Vec<_> = statutes
                .iter()
                .filter(|s| {
                    let title_match = s.title.to_lowercase().contains(&req.query.to_lowercase())
                        || s.id.to_lowercase().contains(&req.query.to_lowercase());

                    let jurisdiction_match = if req.jurisdiction.is_empty() {
                        true
                    } else {
                        s.jurisdiction.as_ref() == Some(&req.jurisdiction)
                    };

                    title_match && jurisdiction_match
                })
                .collect();

            let total_count = filtered.len() as i32;

            // Cursor-based pagination: page_token is base64(offset as decimal string)
            let page_size = if req.page_size > 0 {
                req.page_size as usize
            } else {
                usize::MAX
            };
            let offset = if req.page_token.is_empty() {
                0usize
            } else {
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.page_token)
                    .ok()
                    .and_then(|b| String::from_utf8(b).ok())
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0)
            };

            let page: Vec<_> = filtered
                .iter()
                .skip(offset)
                .take(page_size)
                .map(|s| Self::statute_to_proto(s))
                .collect();

            let next_offset = offset + page.len();
            let next_page_token = if next_offset < filtered.len() {
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    next_offset.to_string().as_bytes(),
                )
            } else {
                String::new()
            };

            Ok(Response::new(SearchStatutesResponse {
                statutes: page,
                next_page_token,
                total_count,
            }))
        }

        async fn verify_condition(
            &self,
            request: Request<VerifyConditionRequest>,
        ) -> Result<Response<VerifyConditionResponse>, Status> {
            let req = request.into_inner();

            if req.condition.is_empty() {
                return Ok(Response::new(VerifyConditionResponse {
                    is_valid: false,
                    message: "Condition string is empty".to_string(),
                }));
            }

            // Wrap the condition in a minimal STATUTE block so the DSL parser can process it.
            // The DSL condition grammar is: WHEN <condition_expr> THEN GRANT "ok"
            let probe = format!(
                "STATUTE _verify_probe_: \"Probe\" {{ WHEN {} THEN GRANT \"ok\" }}",
                req.condition
            );

            let parser = legalis_dsl::LegalDslParser::new();
            match parser.parse_statute(&probe) {
                Ok(_) => Ok(Response::new(VerifyConditionResponse {
                    is_valid: true,
                    message: format!("Condition '{}' is syntactically valid", req.condition),
                })),
                Err(e) => Ok(Response::new(VerifyConditionResponse {
                    is_valid: false,
                    message: format!("Condition '{}' is invalid: {}", req.condition, e),
                })),
            }
        }

        async fn health_check(
            &self,
            _request: Request<HealthCheckRequest>,
        ) -> Result<Response<HealthCheckResponse>, Status> {
            Ok(Response::new(HealthCheckResponse {
                status: "healthy".to_string(),
            }))
        }
    }

    /// Create a new gRPC service server
    pub fn create_grpc_service(
        state: GrpcServiceState,
    ) -> LegalisServiceServer<LegalisGrpcService> {
        LegalisServiceServer::new(LegalisGrpcService::new(state))
    }

    /// Create a gRPC server with reflection support enabled.
    ///
    /// Reflection allows clients to discover service definitions at runtime,
    /// which is useful for development tools like grpcurl and gRPC UI.
    #[cfg(feature = "grpc")]
    pub fn create_grpc_server_with_reflection(
        state: GrpcServiceState,
    ) -> Result<tonic::transport::server::Router, Box<dyn std::error::Error + Send + Sync>> {
        use tonic::transport::Server;
        use tonic_reflection::server::Builder as ReflectionBuilder;

        let service = create_grpc_service(state);

        // Build reflection service
        let reflection_service = ReflectionBuilder::configure()
            .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
            .build_v1()
            .expect("gRPC reflection service build should not fail with valid file descriptor set");

        Ok(Server::builder()
            .add_service(service)
            .add_service(reflection_service))
    }

    /// Create a gRPC server with health checking support.
    ///
    /// Health checking allows load balancers and orchestrators to monitor
    /// service health and route traffic accordingly.
    #[cfg(feature = "grpc")]
    pub fn create_grpc_server_with_health(
        state: GrpcServiceState,
    ) -> tonic::transport::server::Router {
        use tonic::transport::Server;
        use tonic_health::server::health_reporter;

        let service = create_grpc_service(state);

        // Create health reporter
        let (health_reporter, health_service) = health_reporter();

        // Set the gRPC service as serving
        health_reporter
            .set_serving::<LegalisServiceServer<LegalisGrpcService>>()
            .now_or_never();

        Server::builder()
            .add_service(health_service)
            .add_service(service)
    }

    /// Create a gRPC server with both reflection and health checking.
    ///
    /// This is the recommended configuration for production deployments,
    /// providing both service discovery and health monitoring capabilities.
    #[cfg(feature = "grpc")]
    pub fn create_grpc_server_full(
        state: GrpcServiceState,
    ) -> Result<tonic::transport::server::Router, Box<dyn std::error::Error + Send + Sync>> {
        use tonic::transport::Server;
        use tonic_health::server::health_reporter;
        use tonic_reflection::server::Builder as ReflectionBuilder;

        let service = create_grpc_service(state);

        // Build reflection service
        let reflection_service = ReflectionBuilder::configure()
            .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
            .build_v1()?;

        // Create health reporter
        let (health_reporter, health_service) = health_reporter();

        // Set the gRPC service as serving
        health_reporter
            .set_serving::<LegalisServiceServer<LegalisGrpcService>>()
            .now_or_never();

        Ok(Server::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service(service))
    }

    /// Create a gRPC server with gRPC-web support for browser clients.
    ///
    /// gRPC-web allows browser-based applications to make gRPC calls,
    /// bridging the gap between web and native gRPC clients.
    #[cfg(feature = "grpc")]
    pub fn create_grpc_server_with_web(
        state: GrpcServiceState,
    ) -> tonic_web::GrpcWebService<impl tonic::server::NamedService + Clone + Send + 'static> {
        let service = create_grpc_service(state);

        // Wrap service with gRPC-web layer using tower ServiceBuilder
        tower::ServiceBuilder::new()
            .layer(tonic_web::GrpcWebLayer::new())
            .service(service)
    }
}

#[cfg(test)]
#[cfg(feature = "grpc")]
mod tests {
    use super::service::pb::legalis_service_server::LegalisService;
    use super::service::*;

    #[test]
    fn test_grpc_service_state_creation() {
        let state = GrpcServiceState::new();
        assert_eq!(state.statutes.try_read().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_verify_condition_valid() {
        use pb::VerifyConditionRequest;

        let state = GrpcServiceState::new();
        let service = LegalisGrpcService::new(state);

        let request = tonic::Request::new(VerifyConditionRequest {
            condition: "AGE >= 18".to_string(),
        });

        let response = service
            .verify_condition(request)
            .await
            .expect("verify_condition should not return an error");

        let result = response.into_inner();
        assert!(
            result.is_valid,
            "AGE >= 18 should be a valid DSL condition; message: {}",
            result.message
        );
    }

    #[tokio::test]
    async fn test_verify_condition_empty() {
        use pb::VerifyConditionRequest;

        let state = GrpcServiceState::new();
        let service = LegalisGrpcService::new(state);

        let request = tonic::Request::new(VerifyConditionRequest {
            condition: String::new(),
        });

        let response = service
            .verify_condition(request)
            .await
            .expect("verify_condition should not error on empty input");

        let result = response.into_inner();
        assert!(!result.is_valid, "Empty condition should be invalid");
    }

    #[tokio::test]
    async fn test_verify_condition_invalid_syntax() {
        use pb::VerifyConditionRequest;

        let state = GrpcServiceState::new();
        let service = LegalisGrpcService::new(state);

        // This condition is syntactically invalid: an unterminated string literal with no operator.
        // The DSL parser will reject it because the string is never closed.
        let request = tonic::Request::new(VerifyConditionRequest {
            condition: "AGE >= \"unclosed string".to_string(),
        });

        let response = service
            .verify_condition(request)
            .await
            .expect("verify_condition should not error on bad input");

        let result = response.into_inner();
        assert!(
            !result.is_valid,
            "Unclosed string literal should be invalid"
        );
    }

    #[tokio::test]
    async fn test_stream_simulation_happy_path() {
        use legalis_core::{Effect, EffectType, Statute};
        use pb::RunSimulationRequest;
        use std::collections::HashMap;
        use tokio_stream::StreamExt;

        let statute = Statute::new(
            "stream-test-statute",
            "Streaming Simulation Test",
            Effect::new(EffectType::Grant, "Grant test benefit"),
        );
        let statute_id = statute.id.clone();

        let state = GrpcServiceState::new();
        {
            let mut guard = state
                .statutes
                .try_write()
                .expect("write lock should not be contended");
            guard.push(statute);
        }

        let service = LegalisGrpcService::new(state);
        let mut params = HashMap::new();
        params.insert("population_size".to_string(), "3".to_string());

        let request = tonic::Request::new(RunSimulationRequest {
            statute_id: statute_id.clone(),
            parameters: params,
        });

        let response = service
            .stream_simulation(request)
            .await
            .expect("stream_simulation should succeed for known statute");

        let mut stream = response.into_inner();

        let mut progress_values = Vec::new();
        let mut final_result = None;

        while let Some(item) = stream.next().await {
            let msg = item.expect("stream item should not be an error");
            progress_values.push(msg.progress_percent);
            if let Some(r) = msg.result {
                final_result = Some(r);
            }
        }

        assert!(
            !progress_values.is_empty(),
            "Expected at least one progress message"
        );
        assert_eq!(
            *progress_values.last().unwrap_or(&0),
            100,
            "Final progress message should be 100"
        );

        let result = final_result.expect("Final SimulationResult should be emitted");
        assert!(result.success, "Simulation should complete successfully");
        assert!(
            result.results.contains_key("total_applications"),
            "Result should contain total_applications"
        );
    }

    #[tokio::test]
    async fn test_stream_simulation_not_found() {
        use pb::RunSimulationRequest;
        use std::collections::HashMap;

        let state = GrpcServiceState::new();
        let service = LegalisGrpcService::new(state);

        let request = tonic::Request::new(RunSimulationRequest {
            statute_id: "nonexistent-for-stream".to_string(),
            parameters: HashMap::new(),
        });

        let result = service.stream_simulation(request).await;

        assert!(
            result.is_err(),
            "stream_simulation should return Err for unknown statute"
        );
        // We can't use unwrap_err() because the Ok variant (Pin<Box<dyn Stream>>) is not Debug.
        // Extract the error via pattern matching instead.
        let error = match result {
            Err(e) => e,
            Ok(_) => panic!("Expected Err but got Ok"),
        };
        assert_eq!(
            error.code(),
            tonic::Code::NotFound,
            "Expected NotFound, got {:?}",
            error.code()
        );
    }
}
