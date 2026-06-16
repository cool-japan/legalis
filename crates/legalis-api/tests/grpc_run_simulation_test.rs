//! Integration tests for the gRPC `run_simulation` handler.

#[cfg(feature = "grpc")]
mod tests {
    use legalis_api::grpc::service::{
        GrpcServiceState, LegalisGrpcService, pb::legalis_service_server::LegalisService,
    };
    use legalis_core::{Effect, EffectType, Statute};
    use tonic::Request;

    fn make_state_with_statute(statute: Statute) -> GrpcServiceState {
        let state = GrpcServiceState::new();
        {
            let mut guard = state
                .statutes
                .try_write()
                .expect("RwLock should not be contended during test setup");
            guard.push(statute);
        }
        state
    }

    fn test_statute() -> Statute {
        Statute::new(
            "test-statute-sim",
            "Test Statute for Simulation",
            Effect::new(EffectType::Grant, "Grant full legal capacity"),
        )
    }

    #[tokio::test]
    async fn test_run_simulation_happy_path() {
        use legalis_api::grpc::service::pb::RunSimulationRequest;
        use std::collections::HashMap;

        let statute = test_statute();
        let statute_id = statute.id.clone();
        let state = make_state_with_statute(statute);
        let service = LegalisGrpcService::new(state);

        let mut parameters = HashMap::new();
        parameters.insert("population_size".to_string(), "5".to_string());

        let request = Request::new(RunSimulationRequest {
            statute_id: statute_id.clone(),
            parameters,
        });

        let response = service
            .run_simulation(request)
            .await
            .expect("run_simulation should succeed for a known statute");

        let result = response.into_inner();

        assert!(result.success, "result.success must be true");
        assert!(result.errors.is_empty(), "errors must be empty on success");

        let total_applications: u64 = result
            .results
            .get("total_applications")
            .expect("results must contain 'total_applications'")
            .parse()
            .expect("'total_applications' must be a valid u64");

        assert!(
            total_applications >= 1,
            "total_applications must be >= 1, got {}",
            total_applications
        );
    }

    #[tokio::test]
    async fn test_run_simulation_not_found() {
        use legalis_api::grpc::service::pb::RunSimulationRequest;
        use std::collections::HashMap;

        let state = GrpcServiceState::new(); // empty — no statutes
        let service = LegalisGrpcService::new(state);

        let request = Request::new(RunSimulationRequest {
            statute_id: "nonexistent-statute-xyz".to_string(),
            parameters: HashMap::new(),
        });

        let error = service
            .run_simulation(request)
            .await
            .expect_err("run_simulation must return Err for an unknown statute_id");

        assert_eq!(
            error.code(),
            tonic::Code::NotFound,
            "error code must be NotFound, got {:?}",
            error.code()
        );
    }
}
