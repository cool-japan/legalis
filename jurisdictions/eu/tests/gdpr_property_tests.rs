//! Property-based tests for GDPR module
//!
//! These tests use proptest to verify that GDPR validation logic holds
//! for arbitrary inputs, catching edge cases that unit tests might miss.

use legalis_core::LegalResult;
use legalis_eu::gdpr::article6::*;
use legalis_eu::gdpr::types::*;
use proptest::prelude::*;

/// Helper to extract boolean from LegalResult
fn legal_result_is_true(result: &LegalResult<bool>) -> bool {
    matches!(result, LegalResult::Deterministic(true))
}

/// Strategy for generating processing purposes
fn processing_purpose() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "Customer service".to_string(),
        "Marketing analytics".to_string(),
        "Security monitoring".to_string(),
        "Performance optimization".to_string(),
        "User authentication".to_string(),
        "Payment processing".to_string(),
    ])
}

/// Strategy for generating special categories
fn special_category() -> impl Strategy<Value = SpecialCategory> {
    prop::sample::select(vec![
        SpecialCategory::HealthData,
        SpecialCategory::GeneticData,
        SpecialCategory::BiometricData,
        SpecialCategory::RacialEthnicOrigin,
        SpecialCategory::PoliticalOpinions,
        SpecialCategory::ReligiousBeliefs,
        SpecialCategory::TradeUnionMembership,
        SpecialCategory::SexLifeOrOrientation,
    ])
}

#[cfg(test)]
mod article6_properties {
    use super::*;

    proptest! {
        /// Property: Any valid consent must have all Article 7 criteria
        #[test]
        fn consent_requires_all_criteria(
            controller in "[A-Z][a-z]+ (GmbH|Ltd|Inc|Corp)",
            purpose in processing_purpose(),
            freely_given in prop::bool::ANY,
            specific in prop::bool::ANY,
            informed in prop::bool::ANY,
            unambiguous in prop::bool::ANY
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given,
                    specific,
                    informed,
                    unambiguous,
                });

            // If all criteria are true, validation should succeed
            // If any is false, validation should fail with error
            let all_true = freely_given && specific && informed && unambiguous;

            let result = processing.validate();

            if all_true {
                // Valid consent should pass validation
                prop_assert!(result.is_ok(), "Valid consent should pass validation");
                let validation = result.unwrap();
                prop_assert!(
                    legal_result_is_true(&validation.lawful_basis_valid),
                    "Valid consent should have Deterministic(true) lawful basis"
                );
            } else {
                // Invalid consent should fail validation with error
                prop_assert!(result.is_err(), "Invalid consent should fail validation");
            }
        }

        /// Property: Contract processing requires purpose and controller
        #[test]
        fn contract_requires_essentials(
            controller in "[A-Z][a-z]+ (GmbH|Ltd|Inc|Corp)",
            purpose in processing_purpose()
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Contract {
                    necessary_for_performance: true,
                });

            let result = processing.validate();
            prop_assert!(result.is_ok());

            let validation = result.unwrap();
            prop_assert!(legal_result_is_true(&validation.lawful_basis_valid), "Valid contract basis should pass");
        }

        /// Property: Legitimate interest requires balancing test
        #[test]
        fn legitimate_interest_requires_balancing(
            controller in "[A-Z][a-z]+ (GmbH|Ltd|Inc|Corp)",
            purpose in processing_purpose(),
            balancing_passed in prop::bool::ANY
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("preferences".to_string()))
                .with_operation(ProcessingOperation::Use)
                .with_lawful_basis(LawfulBasis::LegitimateInterests {
                    controller_interest: "Service improvement".to_string(),
                    balancing_test_passed: balancing_passed,
                });

            let result = processing.validate();
            prop_assert!(result.is_ok());

            let validation = result.unwrap();

            // Balancing test result affects validity
            prop_assert_eq!(
                legal_result_is_true(&validation.lawful_basis_valid),
                balancing_passed,
                "Legitimate interest validity depends on balancing test"
            );
        }

        /// Property: Processing special categories requires Article 9 exception
        #[test]
        fn special_category_requires_exception(
            controller in "[A-Z][a-z]+ (GmbH|Ltd|Inc|Corp)",
            purpose in processing_purpose(),
            category in special_category()
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Special(category))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                });

            let result = processing.validate();
            prop_assert!(result.is_ok());

            let validation = result.unwrap();

            // Special categories should trigger Article 9 requirement
            prop_assert!(
                validation.requires_article9_exception,
                "Processing special category should require Article 9 exception"
            );
        }

        /// Property: Regular data categories don't require Article 9
        #[test]
        fn regular_data_no_article9(
            controller in "[A-Z][a-z]+ (GmbH|Ltd|Inc|Corp)",
            purpose in processing_purpose()
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .add_data_category(PersonalDataCategory::Regular("address".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                });

            let result = processing.validate();
            prop_assert!(result.is_ok());

            let validation = result.unwrap();

            // Regular categories don't require Article 9
            prop_assert!(
                !validation.requires_article9_exception,
                "Regular data should not require Article 9 exception"
            );
        }
    }
}

#[cfg(test)]
mod processing_invariants {
    use super::*;

    proptest! {
        /// Invariant: Validation should never panic on arbitrary strings
        #[test]
        fn validation_never_panics_on_strings(
            s1 in ".*",
            s2 in ".*"
        ) {
            // Try to create DataProcessing with arbitrary strings
            // Should not panic, only return validation errors

            let _ = DataProcessing::new()
                .with_controller(&s1)
                .with_purpose(&s2)
                .add_data_category(PersonalDataCategory::Regular(s1.clone()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                })
                .validate();
        }

        /// Invariant: Validation result should be deterministic
        #[test]
        fn validation_is_deterministic(
            controller in "[A-Z][a-z]+ Corp",
            purpose in processing_purpose(),
            freely_given in prop::bool::ANY,
            specific in prop::bool::ANY,
            informed in prop::bool::ANY,
            unambiguous in prop::bool::ANY
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given,
                    specific,
                    informed,
                    unambiguous,
                });

            // Validate twice
            let result1 = processing.validate();
            let result2 = processing.validate();

            // Results should be identical
            prop_assert_eq!(result1.is_ok(), result2.is_ok());

            if let (Ok(v1), Ok(v2)) = (result1, result2) {
                prop_assert_eq!(
                    legal_result_is_true(&v1.lawful_basis_valid),
                    legal_result_is_true(&v2.lawful_basis_valid)
                );
                prop_assert_eq!(v1.requires_article9_exception, v2.requires_article9_exception);
            }
        }

        /// Invariant: Missing controller should always fail
        #[test]
        fn missing_controller_fails(
            purpose in processing_purpose()
        ) {
            let processing = DataProcessing::new()
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                });

            let result = processing.validate();
            prop_assert!(result.is_err(), "Processing without controller should fail");
        }

        /// Invariant: Missing purpose should always fail
        #[test]
        fn missing_purpose_fails(
            controller in "[A-Z][a-z]+ Corp"
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                });

            let result = processing.validate();
            prop_assert!(result.is_err(), "Processing without purpose should fail");
        }

        /// Invariant: Missing lawful basis should always fail
        #[test]
        fn missing_lawful_basis_fails(
            controller in "[A-Z][a-z]+ Corp",
            purpose in processing_purpose()
        ) {
            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_operation(ProcessingOperation::Collection);

            let result = processing.validate();
            prop_assert!(result.is_err(), "Processing without lawful basis should fail");
        }
    }
}

#[cfg(test)]
mod lawful_basis_properties {
    use super::*;

    proptest! {
        /// Property: All 6 lawful bases should be processable
        #[test]
        fn all_lawful_bases_work(
            controller in "[A-Z][a-z]+ Corp",
            purpose in processing_purpose(),
            basis_type in 0u8..6
        ) {
            let lawful_basis = match basis_type {
                0 => LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                },
                1 => LawfulBasis::Contract {
                    necessary_for_performance: true,
                },
                2 => LawfulBasis::LegalObligation {
                    eu_law: Some("GDPR Art. 6(1)(c)".to_string()),
                    member_state_law: None,
                },
                3 => LawfulBasis::VitalInterests {
                    life_threatening: true,
                },
                4 => LawfulBasis::PublicTask {
                    task_basis: "Public health monitoring".to_string(),
                },
                5 => LawfulBasis::LegitimateInterests {
                    controller_interest: "Fraud prevention".to_string(),
                    balancing_test_passed: true,
                },
                _ => unreachable!(),
            };

            let processing = DataProcessing::new()
                .with_controller(&controller)
                .with_purpose(&purpose)
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_operation(ProcessingOperation::Collection)
                .with_lawful_basis(lawful_basis);

            let result = processing.validate();
            prop_assert!(result.is_ok(), "All 6 lawful bases should validate successfully");

            let validation = result.unwrap();
            prop_assert!(
                legal_result_is_true(&validation.lawful_basis_valid),
                "Valid lawful basis {} should pass",
                basis_type
            );
        }
    }
}
