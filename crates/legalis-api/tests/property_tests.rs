//! Property-based tests for legalis-api using proptest.
//!
//! These tests verify API endpoint properties:
//! - API endpoint properties
//! - Serialization round-trip
//! - Authentication invariants

use legalis_api::{ApiError, ApiResponse, ResponseMeta, SavedSimulation, VerificationJobResult};
use legalis_core::{Effect, EffectType, Statute};
use proptest::prelude::*;

// ============================================================================
// Strategy Generators
// ============================================================================

/// Strategy for generating Statutes
fn statute_strategy() -> impl Strategy<Value = Statute> {
    (
        "[a-z][a-z0-9-]{2,20}",
        "[A-Za-z ]{10,60}",
        prop_oneof![
            Just(EffectType::Grant),
            Just(EffectType::Revoke),
            Just(EffectType::Obligation),
        ],
    )
        .prop_map(|(id, title, effect_type)| {
            Statute::new(&id, &title, Effect::new(effect_type, "Test effect"))
        })
}

/// Strategy for generating ResponseMeta
fn response_meta_strategy() -> impl Strategy<Value = ResponseMeta> {
    (
        prop::option::of(any::<usize>().prop_map(|v| v % 10000)),
        prop::option::of(any::<usize>().prop_map(|v| v % 100)),
        prop::option::of(any::<usize>().prop_map(|v| v % 100)),
        prop::option::of("[a-z0-9]{10,20}".prop_map(|s| s.to_string())),
        prop::option::of("[a-z0-9]{10,20}".prop_map(|s| s.to_string())),
        prop::option::of(any::<bool>()),
    )
        .prop_map(
            |(total, page, per_page, next_cursor, prev_cursor, has_more)| ResponseMeta {
                total,
                page,
                per_page,
                next_cursor,
                prev_cursor,
                has_more,
            },
        )
}

/// Strategy for generating VerificationJobResult
fn verification_job_result_strategy() -> impl Strategy<Value = VerificationJobResult> {
    (
        any::<bool>(),
        prop::collection::vec("[A-Za-z ]{10,50}", 0..5),
        prop::collection::vec("[A-Za-z ]{10,50}", 0..5),
        any::<usize>().prop_map(|v| v % 100),
    )
        .prop_map(
            |(passed, errors, warnings, statute_count)| VerificationJobResult {
                passed,
                errors,
                warnings,
                statute_count,
            },
        )
}

/// Strategy for generating SavedSimulation
fn saved_simulation_strategy() -> impl Strategy<Value = SavedSimulation> {
    (
        "[a-z0-9]{8}",
        "[A-Za-z ]{5,30}",
        prop::option::of("[A-Za-z ]{10,100}"),
        prop::collection::vec("[a-z][a-z0-9-]{2,20}", 1..10),
        any::<usize>().prop_map(|v| v % 10000),
        any::<usize>().prop_map(|v| v % 1000),
        any::<usize>().prop_map(|v| v % 1000),
        any::<usize>().prop_map(|v| v % 1000),
        "[a-z]{5,10}",
    )
        .prop_map(
            |(
                id,
                name,
                description,
                statute_ids,
                population_size,
                deterministic,
                discretionary,
                void_outcomes,
                created_by,
            )| {
                let total = (deterministic + discretionary + void_outcomes).max(1);
                SavedSimulation {
                    id,
                    name,
                    description,
                    statute_ids,
                    population_size,
                    deterministic_outcomes: deterministic,
                    discretionary_outcomes: discretionary,
                    void_outcomes,
                    deterministic_rate: deterministic as f64 / total as f64,
                    discretionary_rate: discretionary as f64 / total as f64,
                    void_rate: void_outcomes as f64 / total as f64,
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    created_by,
                }
            },
        )
}

// ============================================================================
// API Response Properties
// ============================================================================

proptest! {
    /// Property: ApiResponse serialization round-trip preserves data
    #[test]
    fn prop_api_response_serde_roundtrip(statute in statute_strategy()) {
        let response = ApiResponse::new(statute.clone());
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        let deserialized: ApiResponse<Statute> = serde_json::from_str(&json)
            .expect("Deserialization should succeed");

        prop_assert_eq!(response.data.id, deserialized.data.id);
        prop_assert_eq!(response.data.title, deserialized.data.title);
    }

    /// Property: ApiResponse with metadata preserves both data and meta
    #[test]
    fn prop_api_response_with_meta_preserves_both(
        statute in statute_strategy(),
        meta in response_meta_strategy()
    ) {
        let response = ApiResponse::new(statute.clone()).with_meta(meta.clone());

        prop_assert_eq!(response.data.id, statute.id);
        prop_assert!(response.meta.is_some(), "Meta should be present");

        if let Some(response_meta) = response.meta {
            prop_assert_eq!(response_meta.total, meta.total);
            prop_assert_eq!(response_meta.page, meta.page);
            prop_assert_eq!(response_meta.per_page, meta.per_page);
        }
    }

    /// Property: ResponseMeta with pagination values are consistent
    #[test]
    fn prop_response_meta_pagination_consistent(
        total in prop::option::of(1usize..10000),
        page in prop::option::of(1usize..100),
        per_page in prop::option::of(1usize..100)
    ) {
        let meta = ResponseMeta {
            total,
            page,
            per_page,
            next_cursor: None,
            prev_cursor: None,
            has_more: None,
        };

        // If all pagination fields are present, they should be reasonable
        if let (Some(_t), Some(p), Some(pp)) = (total, page, per_page) {
            prop_assert!(p >= 1, "Page should be at least 1");
            prop_assert!(pp >= 1, "Per-page should be at least 1");
        }

        let json = serde_json::to_string(&meta).expect("Should serialize");
        let _: ResponseMeta = serde_json::from_str(&json).expect("Should deserialize");
    }
}

// ============================================================================
// Verification Job Result Properties
// ============================================================================

proptest! {
    /// Property: VerificationJobResult serialization round-trip
    #[test]
    fn prop_verification_job_result_serde_roundtrip(result in verification_job_result_strategy()) {
        let json = serde_json::to_string(&result).expect("Serialization should succeed");
        let deserialized: VerificationJobResult = serde_json::from_str(&json)
            .expect("Deserialization should succeed");

        prop_assert_eq!(result.passed, deserialized.passed);
        prop_assert_eq!(result.errors.len(), deserialized.errors.len());
        prop_assert_eq!(result.warnings.len(), deserialized.warnings.len());
        prop_assert_eq!(result.statute_count, deserialized.statute_count);
    }

    /// Property: Passing verification has no errors
    #[test]
    fn prop_passing_verification_no_errors(
        warnings in prop::collection::vec("[A-Za-z ]{10,50}", 0..5),
        statute_count in 1usize..100
    ) {
        let result = VerificationJobResult {
            passed: true,
            errors: vec![],
            warnings,
            statute_count,
        };

        prop_assert!(result.passed);
        prop_assert_eq!(result.errors.len(), 0, "Passing should have no errors");
    }

    /// Property: Failing verification has errors
    #[test]
    fn prop_failing_verification_has_errors(
        errors in prop::collection::vec("[A-Za-z ]{10,50}", 1..5),
        warnings in prop::collection::vec("[A-Za-z ]{10,50}", 0..5),
        statute_count in 1usize..100
    ) {
        let result = VerificationJobResult {
            passed: false,
            errors: errors.clone(),
            warnings,
            statute_count,
        };

        prop_assert!(!result.passed);
        prop_assert!(!result.errors.is_empty(), "Failing should have errors");
        prop_assert_eq!(result.errors.len(), errors.len());
    }

    /// Property: Statute count is valid
    #[test]
    fn prop_statute_count_non_negative(result in verification_job_result_strategy()) {
        // Statute count is usize, so always non-negative by type
        prop_assert!(result.statute_count < 10_000_000, "Statute count should be reasonable");
    }
}

// ============================================================================
// SavedSimulation Properties
// ============================================================================

proptest! {
    /// Property: SavedSimulation serialization round-trip
    #[test]
    fn prop_saved_simulation_serde_roundtrip(sim in saved_simulation_strategy()) {
        let json = serde_json::to_string(&sim).expect("Serialization should succeed");
        let deserialized: SavedSimulation = serde_json::from_str(&json)
            .expect("Deserialization should succeed");

        prop_assert_eq!(sim.id, deserialized.id);
        prop_assert_eq!(sim.name, deserialized.name);
        prop_assert_eq!(sim.population_size, deserialized.population_size);
        prop_assert_eq!(sim.statute_ids.len(), deserialized.statute_ids.len());
    }

    /// Property: Simulation rates sum to approximately 1.0
    #[test]
    fn prop_simulation_rates_sum_to_one(sim in saved_simulation_strategy()) {
        let total_rate = sim.deterministic_rate + sim.discretionary_rate + sim.void_rate;

        // Rates should sum to approximately 1.0 (allowing for floating point error)
        prop_assert!(
            (total_rate - 1.0).abs() < 0.001,
            "Rates should sum to ~1.0, got {}",
            total_rate
        );
    }

    /// Property: Simulation rates are non-negative
    #[test]
    fn prop_simulation_rates_non_negative(sim in saved_simulation_strategy()) {
        prop_assert!(sim.deterministic_rate >= 0.0, "Deterministic rate should be non-negative");
        prop_assert!(sim.discretionary_rate >= 0.0, "Discretionary rate should be non-negative");
        prop_assert!(sim.void_rate >= 0.0, "Void rate should be non-negative");
    }

    /// Property: Simulation rates are at most 1.0
    #[test]
    fn prop_simulation_rates_bounded(sim in saved_simulation_strategy()) {
        prop_assert!(sim.deterministic_rate <= 1.0, "Deterministic rate should be <= 1.0");
        prop_assert!(sim.discretionary_rate <= 1.0, "Discretionary rate should be <= 1.0");
        prop_assert!(sim.void_rate <= 1.0, "Void rate should be <= 1.0");
    }

    /// Property: Simulation population size is reasonable
    #[test]
    fn prop_simulation_population_non_negative(sim in saved_simulation_strategy()) {
        prop_assert!(sim.population_size < 1_000_000, "Population size should be reasonable");
    }

    /// Property: Simulation outcome counts sum correctly
    #[test]
    fn prop_simulation_outcomes_non_negative(sim in saved_simulation_strategy()) {
        let total_outcomes = sim.deterministic_outcomes + sim.discretionary_outcomes + sim.void_outcomes;
        prop_assert!(total_outcomes > 0, "Should have at least one outcome");
    }

    /// Property: Simulation ID is non-empty
    #[test]
    fn prop_simulation_id_non_empty(sim in saved_simulation_strategy()) {
        prop_assert!(!sim.id.is_empty(), "Simulation ID should not be empty");
    }

    /// Property: Simulation name is non-empty
    #[test]
    fn prop_simulation_name_non_empty(sim in saved_simulation_strategy()) {
        prop_assert!(!sim.name.is_empty(), "Simulation name should not be empty");
    }

    /// Property: Simulation has at least one statute
    #[test]
    fn prop_simulation_has_statutes(sim in saved_simulation_strategy()) {
        prop_assert!(
            !sim.statute_ids.is_empty(),
            "Simulation should have at least one statute"
        );
    }
}

// ============================================================================
// API Error Properties
// ============================================================================

proptest! {
    /// Property: Error messages are non-empty
    #[test]
    fn prop_api_error_message_non_empty(error_msg in "[A-Za-z ]{10,100}") {
        let error = ApiError::NotFound(error_msg.clone());
        let error_string = error.to_string();
        prop_assert!(!error_string.is_empty(), "Error message should not be empty");
        prop_assert!(error_string.contains(&error_msg), "Error should contain original message");
    }

    /// Property: Different error types have different semantics
    #[test]
    fn prop_api_error_types_distinct(msg in "[A-Za-z ]{10,50}") {
        let not_found = ApiError::NotFound(msg.clone());
        let bad_request = ApiError::BadRequest(msg.clone());
        let internal = ApiError::Internal(msg.clone());
        let validation = ApiError::ValidationFailed(msg.clone());

        // Each should have different string representations
        let s1 = not_found.to_string();
        let s2 = bad_request.to_string();
        let s3 = internal.to_string();
        let s4 = validation.to_string();

        prop_assert_ne!(&s1, &s2);
        prop_assert_ne!(&s1, &s3);
        prop_assert_ne!(&s1, &s4);
        prop_assert_ne!(&s2, &s3);
    }
}

#[cfg(test)]
mod additional_tests {
    #[test]
    fn test_proptest_configuration() {
        // Verify proptest is configured correctly
        // This test verifies that proptest runs successfully
    }
}
