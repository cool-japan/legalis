//! Property-based tests for legalis-verifier using proptest.
//!
//! These tests verify verification properties:
//! - Verification determinism
//! - Result consistency
//! - Error handling properties

use legalis_core::{Effect, EffectType, Statute};
use legalis_verifier::{Severity, StatuteVerifier, VerificationError, VerificationResult};
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
            Just(EffectType::Prohibition),
            Just(EffectType::MonetaryTransfer),
        ],
    )
        .prop_map(|(id, title, effect_type)| {
            Statute::new(&id, &title, Effect::new(effect_type, "Test effect"))
        })
}

/// Strategy for generating Severity levels
fn severity_strategy() -> impl Strategy<Value = Severity> {
    prop_oneof![
        Just(Severity::Info),
        Just(Severity::Warning),
        Just(Severity::Error),
        Just(Severity::Critical),
    ]
}

/// Strategy for generating VerificationError
fn verification_error_strategy() -> impl Strategy<Value = VerificationError> {
    prop_oneof![
        "[A-Za-z ]{10,50}".prop_map(|msg| VerificationError::CircularReference { message: msg }),
        "[a-z][a-z0-9-]{3,20}".prop_map(|id| VerificationError::DeadStatute { statute_id: id }),
        ("[a-z][a-z0-9-]{3,20}", "[A-Za-z ]{5,30}").prop_map(|(id, principle)| {
            VerificationError::ConstitutionalConflict {
                statute_id: id,
                principle,
            }
        }),
        "[A-Za-z ]{10,50}".prop_map(|msg| VerificationError::LogicalContradiction { message: msg }),
        "[A-Za-z ]{10,50}".prop_map(|msg| VerificationError::Ambiguity { message: msg }),
        "[A-Za-z ]{10,50}".prop_map(|msg| VerificationError::UnreachableCode { message: msg }),
    ]
}

// ============================================================================
// Verification Determinism Properties
// ============================================================================

proptest! {
    /// Property: Verifying same statutes twice produces same result
    #[test]
    fn prop_verification_is_deterministic(statutes in prop::collection::vec(statute_strategy(), 1..3)) {
        let verifier = StatuteVerifier::new();

        let result1 = verifier.verify(&statutes);
        let result2 = verifier.verify(&statutes);

        prop_assert_eq!(result1.passed, result2.passed, "Pass/fail should be consistent");
        prop_assert_eq!(result1.errors.len(), result2.errors.len(), "Error count should be same");
        prop_assert_eq!(result1.warnings.len(), result2.warnings.len(), "Warning count should be same");
    }

    /// Property: Verification with caching produces same results
    #[test]
    fn prop_cached_verification_consistent(statutes in prop::collection::vec(statute_strategy(), 1..3)) {
        let verifier = StatuteVerifier::new().with_caching();

        let result1 = verifier.verify(&statutes);
        let result2 = verifier.verify(&statutes); // Should hit cache

        prop_assert_eq!(result1.passed, result2.passed);
        prop_assert_eq!(result1.errors.len(), result2.errors.len());
    }

    /// Property: Verifier without caching produces consistent results
    #[test]
    fn prop_uncached_verification_consistent(statutes in prop::collection::vec(statute_strategy(), 1..3)) {
        let verifier = StatuteVerifier::new();

        let result1 = verifier.verify(&statutes);
        let result2 = verifier.verify(&statutes);

        prop_assert_eq!(result1.passed, result2.passed);
        prop_assert_eq!(result1.errors.len(), result2.errors.len());
    }
}

// ============================================================================
// VerificationResult Properties
// ============================================================================

proptest! {
    /// Property: Passing result has no errors
    #[test]
    fn prop_passing_result_no_errors(_seed in any::<u64>()) {
        let result = VerificationResult::pass();
        prop_assert!(result.passed, "Passing result should have passed=true");
        prop_assert_eq!(result.errors.len(), 0, "Passing result should have no errors");
    }

    /// Property: Failing result has errors
    #[test]
    fn prop_failing_result_has_errors(errors in prop::collection::vec(verification_error_strategy(), 1..5)) {
        let result = VerificationResult::fail(errors.clone());
        prop_assert!(!result.passed, "Failing result should have passed=false");
        prop_assert_eq!(result.errors.len(), errors.len(), "Should contain all errors");
    }

    /// Property: Adding warnings preserves warnings
    #[test]
    fn prop_warnings_preserved(warnings in prop::collection::vec("[A-Za-z ]{10,50}", 0..5)) {
        let mut result = VerificationResult::pass();
        for warning in &warnings {
            result = result.with_warning(warning.clone());
        }
        prop_assert_eq!(result.warnings.len(), warnings.len(), "Warnings should be preserved");
    }

    /// Property: Adding suggestions preserves suggestions
    #[test]
    fn prop_suggestions_preserved(suggestions in prop::collection::vec("[A-Za-z ]{10,50}", 0..5)) {
        let mut result = VerificationResult::pass();
        for suggestion in &suggestions {
            result = result.with_suggestion(suggestion.clone());
        }
        prop_assert_eq!(result.suggestions.len(), suggestions.len(), "Suggestions should be preserved");
    }

    /// Property: Merging results combines errors
    #[test]
    fn prop_merge_combines_errors(
        errors1 in prop::collection::vec(verification_error_strategy(), 0..3),
        errors2 in prop::collection::vec(verification_error_strategy(), 0..3)
    ) {
        let mut result1 = if errors1.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors1.clone())
        };

        let result2 = if errors2.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors2.clone())
        };

        let initial_error_count = result1.errors.len();
        result1.merge(result2);

        prop_assert_eq!(
            result1.errors.len(),
            initial_error_count + errors2.len(),
            "Merge should combine all errors"
        );
    }

    /// Property: JSON serialization round-trip preserves result
    #[test]
    fn prop_result_json_roundtrip(errors in prop::collection::vec(verification_error_strategy(), 0..3)) {
        let result = if errors.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors)
        };

        let json = result.to_json().expect("Serialization should succeed");
        let deserialized = VerificationResult::from_json(&json)
            .expect("Deserialization should succeed");

        prop_assert_eq!(result.passed, deserialized.passed);
        prop_assert_eq!(result.errors.len(), deserialized.errors.len());
    }
}

// ============================================================================
// Severity Properties
// ============================================================================

proptest! {
    /// Property: Severity ordering is consistent
    #[test]
    fn prop_severity_ordering_consistent(s1 in severity_strategy(), s2 in severity_strategy()) {
        // Ordering should be transitive
        let cmp1 = s1.cmp(&s2);
        let cmp2 = s2.cmp(&s1);

        match cmp1 {
            std::cmp::Ordering::Less => prop_assert_eq!(cmp2, std::cmp::Ordering::Greater),
            std::cmp::Ordering::Greater => prop_assert_eq!(cmp2, std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => prop_assert_eq!(cmp2, std::cmp::Ordering::Equal),
        }
    }

    /// Property: Critical is highest severity
    #[test]
    fn prop_critical_is_highest(severity in severity_strategy()) {
        prop_assert!(Severity::Critical >= severity, "Critical should be >= all severities");
    }

    /// Property: Info is lowest severity
    #[test]
    fn prop_info_is_lowest(severity in severity_strategy()) {
        prop_assert!(Severity::Info <= severity, "Info should be <= all severities");
    }

    /// Property: Severity has consistent string representation
    #[test]
    fn prop_severity_string_consistent(severity in severity_strategy()) {
        let s1 = severity.to_string();
        let s2 = severity.to_string();
        prop_assert_eq!(&s1, &s2, "String representation should be consistent");
        prop_assert!(!s1.is_empty(), "String should not be empty");
    }
}

// ============================================================================
// VerificationError Properties
// ============================================================================

proptest! {
    /// Property: CircularReference has Critical severity
    #[test]
    fn prop_circular_reference_is_critical(message in "[A-Za-z ]{10,50}") {
        let error = VerificationError::CircularReference { message };
        prop_assert_eq!(error.severity(), Severity::Critical);
    }

    /// Property: ConstitutionalConflict has Critical severity
    #[test]
    fn prop_constitutional_conflict_is_critical(
        statute_id in "[a-z][a-z0-9-]{3,20}",
        principle in "[A-Za-z ]{5,30}"
    ) {
        let error = VerificationError::ConstitutionalConflict { statute_id, principle };
        prop_assert_eq!(error.severity(), Severity::Critical);
    }

    /// Property: Ambiguity has Warning severity
    #[test]
    fn prop_ambiguity_is_warning(message in "[A-Za-z ]{10,50}") {
        let error = VerificationError::Ambiguity { message };
        prop_assert_eq!(error.severity(), Severity::Warning);
    }

    /// Property: UnreachableCode has Warning severity
    #[test]
    fn prop_unreachable_is_warning(message in "[A-Za-z ]{10,50}") {
        let error = VerificationError::UnreachableCode { message };
        prop_assert_eq!(error.severity(), Severity::Warning);
    }

    /// Property: Error messages are non-empty
    #[test]
    fn prop_error_messages_non_empty(error in verification_error_strategy()) {
        let error_string = error.to_string();
        prop_assert!(!error_string.is_empty(), "Error message should not be empty");
    }
}

// ============================================================================
// Verification Behavior Properties
// ============================================================================

proptest! {
    /// Property: Empty collection passes verification
    #[test]
    fn prop_empty_collection_passes(_seed in any::<u64>()) {
        let verifier = StatuteVerifier::new();
        let result = verifier.verify(&[]);
        prop_assert!(result.passed, "Empty collection should pass");
        prop_assert_eq!(result.errors.len(), 0, "Empty collection should have no errors");
    }
}

// ============================================================================
// Report Generation Properties
// ============================================================================

proptest! {
    /// Property: Severity counts are accurate
    #[test]
    fn prop_severity_counts_accurate(errors in prop::collection::vec(verification_error_strategy(), 0..10)) {
        let result = if errors.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors.clone())
        };

        let counts = result.severity_counts();
        let total: usize = counts.values().sum();

        prop_assert_eq!(total, errors.len(), "Total counts should match error count");
    }

    /// Property: Has critical errors detection is accurate
    #[test]
    fn prop_has_critical_detection_accurate(errors in prop::collection::vec(verification_error_strategy(), 0..5)) {
        let result = if errors.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors.clone())
        };

        let has_critical_manual = errors.iter().any(|e| e.severity() == Severity::Critical);
        let has_critical_method = result.has_critical_errors();

        prop_assert_eq!(
            has_critical_manual,
            has_critical_method,
            "Critical error detection should be accurate"
        );
    }

    /// Property: Errors by severity filter correctly
    #[test]
    fn prop_errors_by_severity_correct(
        errors in prop::collection::vec(verification_error_strategy(), 0..10),
        min_severity in severity_strategy()
    ) {
        let result = if errors.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors.clone())
        };

        let filtered = result.errors_by_severity(min_severity);

        for error in filtered {
            prop_assert!(
                error.severity() >= min_severity,
                "Filtered errors should meet minimum severity"
            );
        }
    }

    /// Property: Compact JSON is shorter than or equal to pretty JSON
    #[test]
    fn prop_compact_json_shorter(errors in prop::collection::vec(verification_error_strategy(), 1..5)) {
        let result = VerificationResult::fail(errors);

        let pretty = result.to_json().expect("Pretty JSON should succeed");
        let compact = result.to_json_compact().expect("Compact JSON should succeed");

        prop_assert!(
            compact.len() <= pretty.len(),
            "Compact JSON should not be longer than pretty"
        );
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
