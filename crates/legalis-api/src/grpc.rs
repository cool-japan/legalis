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
                        .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
                        .unwrap_or(0),
                    effective_until: statute
                        .temporal_validity
                        .expiry_date
                        .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
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

            // Convert to proto
            let proto_statutes: Vec<_> =
                filtered.iter().map(|s| Self::statute_to_proto(s)).collect();

            Ok(Response::new(ListStatutesResponse {
                statutes: proto_statutes,
                next_page_token: String::new(), // TODO: Implement pagination
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
            _request: Request<RunSimulationRequest>,
        ) -> Result<Response<SimulationResult>, Status> {
            // TODO: Implement actual simulation logic
            Ok(Response::new(SimulationResult {
                simulation_id: uuid::Uuid::new_v4().to_string(),
                success: true,
                results: HashMap::new(),
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
            let _req = request.into_inner();
            let (tx, rx) = tokio::sync::mpsc::channel(4);

            // Spawn a task to simulate progress updates
            tokio::spawn(async move {
                let simulation_id = uuid::Uuid::new_v4().to_string();

                for progress in [0, 25, 50, 75, 100] {
                    let progress_msg = SimulationProgress {
                        simulation_id: simulation_id.clone(),
                        progress_percent: progress,
                        current_step: format!("Processing step {}", progress / 25),
                        result: if progress == 100 {
                            Some(SimulationResult {
                                simulation_id: simulation_id.clone(),
                                success: true,
                                results: HashMap::new(),
                                errors: vec![],
                            })
                        } else {
                            None
                        },
                    };

                    if tx.send(Ok(progress_msg)).await.is_err() {
                        break;
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
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
            let proto_statutes: Vec<_> =
                filtered.iter().map(|s| Self::statute_to_proto(s)).collect();

            Ok(Response::new(SearchStatutesResponse {
                statutes: proto_statutes,
                next_page_token: String::new(), // TODO: Implement pagination
                total_count,
            }))
        }

        async fn verify_condition(
            &self,
            request: Request<VerifyConditionRequest>,
        ) -> Result<Response<VerifyConditionResponse>, Status> {
            let req = request.into_inner();

            // Basic validation logic - in a real implementation, this would parse and evaluate the condition
            let is_valid = !req.condition.is_empty() && !req.condition.contains("invalid");
            let message = if is_valid {
                format!("Condition '{}' is valid", req.condition)
            } else {
                format!("Condition '{}' is invalid", req.condition)
            };

            Ok(Response::new(VerifyConditionResponse { is_valid, message }))
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
            .unwrap();

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
    use super::service::*;

    #[test]
    fn test_grpc_service_state_creation() {
        let state = GrpcServiceState::new();
        assert_eq!(state.statutes.try_read().unwrap().len(), 0);
    }
}
