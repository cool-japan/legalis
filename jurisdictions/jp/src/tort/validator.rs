//! Validation logic for Article 709 tort claims
//!
//! This module implements the validation logic for determining whether
//! a tort claim satisfies the requirements of Article 709.

use crate::tort::article709::{Article709, ArticleReference, LiabilityStatus, TortLiability};
use crate::tort::error::ValidationError;
use crate::tort::types::{Intent, ProtectedInterest};

/// Validate a tort claim under Article 709
///
/// Checks all four requirements of Article 709:
/// 1. Intent (故意) or Negligence (過失)
/// 2. Infringement of rights or legally protected interests (権利侵害)
/// 3. Causation (因果関係)
/// 4. Damages (損害)
///
/// ## Example
///
/// ```rust
/// use legalis_jp::tort::{Article709, Intent, Damage, CausalLink, ProtectedInterest, validate_tort_claim};
///
/// let claim = Article709::new()
///     .with_act("交通事故で相手の車に衝突")
///     .with_intent(Intent::Negligence)
///     .with_victim_interest(ProtectedInterest::Property("車両所有権"))
///     .with_damage(Damage::new(500_000, "修理費"))
///     .with_causal_link(CausalLink::Direct);
///
/// let result = validate_tort_claim(&claim);
/// if let Ok(liability) = result {
///     assert!(liability.is_liability_established());
/// }
/// ```
pub fn validate_tort_claim(claim: &Article709) -> Result<TortLiability, ValidationError> {
    let mut validation_details = Vec::new();
    let mut errors = Vec::new();

    // Requirement 1: Intent or Negligence (故意・過失)
    match &claim.intent {
        Some(Intent::Intentional { age }) => {
            if *age < 12 {
                validation_details.push(
                    "Intent exists but responsibility capacity questionable (age < 12)".to_string(),
                );
                // Note: In Japanese law, minors under 12 generally lack tort liability
                // This would require parental liability under Article 714
            } else {
                validation_details.push("Intent (故意) established".to_string());
            }
        }
        Some(Intent::Negligence) | Some(Intent::NegligenceWithDuty { .. }) => {
            validation_details.push("Negligence (過失) established".to_string());
        }
        None => {
            errors.push(ValidationError::NoIntentOrNegligence);
            validation_details.push("No intent or negligence proven".to_string());
        }
    }

    // Check responsibility capacity
    if !claim.has_full_capacity() {
        errors.push(ValidationError::NoResponsibilityCapacity);
        validation_details.push("Tortfeasor lacks responsibility capacity".to_string());
    }

    // Requirement 2: Infringement of rights or legally protected interests (権利侵害)
    match &claim.victim_interest {
        Some(interest) => {
            let interest_desc = match interest {
                ProtectedInterest::Property(desc) => format!("Property right: {}", desc),
                ProtectedInterest::BodyAndHealth => {
                    "Body and health (Article 710 may apply)".to_string()
                }
                ProtectedInterest::Liberty => "Liberty interest".to_string(),
                ProtectedInterest::Privacy => "Privacy interest".to_string(),
                ProtectedInterest::Reputation => "Reputation interest".to_string(),
                ProtectedInterest::Other(desc) => format!("Other interest: {}", desc),
            };
            validation_details.push(format!("Protected interest infringed: {}", interest_desc));
        }
        None => {
            errors.push(ValidationError::NoInfringement);
            validation_details.push("No protected interest identified".to_string());
        }
    }

    // Requirement 3: Causation (因果関係)
    match &claim.causal_link {
        Some(link) => {
            let causation_desc = match link {
                crate::tort::types::CausalLink::Direct => "Direct causation established",
                crate::tort::types::CausalLink::Adequate(reason) => {
                    validation_details.push(format!("Adequate causation: {}", reason));
                    "Adequate causation established"
                }
                crate::tort::types::CausalLink::Conditional {
                    condition,
                    explanation,
                } => {
                    validation_details.push(format!(
                        "Conditional causation - Condition: {}, Explanation: {}",
                        condition, explanation
                    ));
                    "Conditional causation established"
                }
            };
            validation_details.push(causation_desc.to_string());
        }
        None => {
            errors.push(ValidationError::NoCausalLink);
            validation_details.push("No causal link established".to_string());
        }
    }

    // Requirement 4: Damages (損害)
    match &claim.damage {
        Some(damage) => {
            validation_details.push(format!(
                "Damages proven: ¥{} ({})",
                damage.amount, damage.description
            ));
        }
        None => {
            errors.push(ValidationError::NoDamage);
            validation_details.push("No damages proven".to_string());
        }
    }

    // Determine liability status
    let status = if !errors.is_empty() {
        // Check if any critical errors exist
        let has_critical_error = errors
            .iter()
            .any(|e| !matches!(e, ValidationError::InsufficientEvidence(_)));

        if has_critical_error {
            LiabilityStatus::NotEstablished
        } else {
            LiabilityStatus::InsufficientEvidence("See validation details".to_string())
        }
    } else {
        // All requirements satisfied
        LiabilityStatus::Established
    };

    let result = TortLiability {
        article: ArticleReference::default(),
        status: status.clone(),
        validation_details,
    };

    // If there are critical errors, return error
    if !errors.is_empty() {
        if errors.len() == 1 {
            // Extract the single error without unwrap/expect
            match errors.into_iter().next() {
                Some(err) => Err(err),
                None => {
                    // This should never happen as we checked !errors.is_empty()
                    // But to satisfy no-unwrap policy, we return a generic error
                    Err(ValidationError::Multiple(vec![]))
                }
            }
        } else {
            Err(ValidationError::Multiple(errors))
        }
    } else {
        Ok(result)
    }
}

/// Additional helper to check if all Article 709 requirements are met
///
/// Returns true only if all four requirements are satisfied.
pub fn meets_all_requirements(claim: &Article709) -> bool {
    claim.intent.is_some()
        && claim.victim_interest.is_some()
        && claim.causal_link.is_some()
        && claim.damage.is_some()
        && claim.has_full_capacity()
}

/// Calculate recommended compensation (placeholder for complex judicial determination)
///
/// In reality, compensation calculation involves:
/// - Actual damages (現実の損害)
/// - Lost profits (逸失利益)
/// - Pain and suffering (慰謝料)
/// - Comparative negligence (過失相殺)
///
/// This is a simplified version.
pub fn calculate_compensation(claim: &Article709) -> Option<u64> {
    if !meets_all_requirements(claim) {
        return None;
    }

    claim.damage.as_ref().map(|d| {
        // In a real implementation, this would involve complex calculations
        // including comparative negligence, lost profits, etc.
        d.amount
    })
}

/// Validate an Article 710 non-pecuniary damages claim
///
/// Checks that:
/// 1. Article 709 liability is established (precondition)
/// 2. Non-pecuniary damage type is specified
/// 3. Harm severity is categorized
/// 4. Emotional distress is described
///
/// Returns Article710Liability with status and recommended compensation.
pub fn validate_article_710(
    claim: &crate::tort::article710::Article710,
) -> Result<crate::tort::article710::Article710Liability, ValidationError> {
    use crate::tort::article709::ArticleReference;
    use crate::tort::article710::{Article710Liability, NonPecuniaryLiabilityStatus};

    let mut validation_details = Vec::new();
    let mut calculation_factors = Vec::new();
    let mut errors = Vec::new();

    // Requirement 1: Article 709 liability must be established
    match &claim.article_709_claim {
        Some(article_709) => {
            // Validate the Article 709 claim
            match validate_tort_claim(article_709) {
                Ok(liability) => {
                    if liability.is_liability_established() {
                        validation_details
                            .push("Article 709 liability established (前提条件充足)".to_string());
                        calculation_factors.push("Based on Article 709 tort liability".to_string());
                    } else {
                        errors.push(ValidationError::Article709NotEstablished);
                        validation_details
                            .push("Article 709 liability not established".to_string());
                    }
                }
                Err(_) => {
                    errors.push(ValidationError::Article709NotEstablished);
                    validation_details.push("Article 709 validation failed".to_string());
                }
            }
        }
        None => {
            errors.push(ValidationError::Article709NotEstablished);
            validation_details.push("No Article 709 claim provided".to_string());
        }
    }

    // Requirement 2: Non-pecuniary damage type
    match &claim.damage_type {
        Some(dtype) => {
            use crate::tort::types::NonPecuniaryDamageType;
            let type_desc = match dtype {
                NonPecuniaryDamageType::BodyAndHealth => "Body and health (身体・健康)",
                NonPecuniaryDamageType::ReputationDamage => "Reputation damage (名誉毀損)",
                NonPecuniaryDamageType::LibertyInfringement => "Liberty infringement (自由侵害)",
                NonPecuniaryDamageType::PropertyRelatedDistress => {
                    "Property-related distress (財産侵害に伴う精神的損害)"
                }
            };
            validation_details.push(format!("Non-pecuniary damage type: {}", type_desc));
            calculation_factors.push(format!("Damage type: {}", type_desc));
        }
        None => {
            errors.push(ValidationError::InsufficientEvidence(
                "No damage type specified".to_string(),
            ));
        }
    }

    // Requirement 3: Harm severity
    let mut base_compensation = 0u64;
    match &claim.harm_severity {
        Some(severity) => {
            use crate::tort::types::HarmSeverity;
            let (severity_desc, amount) = match severity {
                HarmSeverity::Minor => ("Minor (軽度)", 100_000u64),
                HarmSeverity::Moderate => ("Moderate (中度)", 500_000u64),
                HarmSeverity::Severe => ("Severe (重度)", 1_500_000u64),
                HarmSeverity::Catastrophic => ("Catastrophic (最重度)", 5_000_000u64),
            };
            validation_details.push(format!("Harm severity: {}", severity_desc));
            calculation_factors.push(format!("Severity level: {}", severity_desc));
            base_compensation = amount;
        }
        None => {
            errors.push(ValidationError::InsufficientEvidence(
                "No harm severity specified".to_string(),
            ));
        }
    }

    // Emotional distress description (recommended but not required)
    if let Some(desc) = &claim.emotional_distress_description {
        validation_details.push(format!("Emotional distress: {}", desc));
        calculation_factors.push("Detailed emotional distress described".to_string());
    }

    // Apply victim comparative fault if present
    let mut final_compensation = claim.consolation_money.unwrap_or(base_compensation);
    if let Some(fault_pct) = claim.victim_comparative_fault {
        if fault_pct > 0 && fault_pct <= 100 {
            let reduction = (final_compensation as f64 * fault_pct as f64 / 100.0) as u64;
            final_compensation = final_compensation.saturating_sub(reduction);
            validation_details.push(format!(
                "Victim comparative fault: {}% (reduction applied)",
                fault_pct
            ));
            calculation_factors.push(format!("Comparative fault reduction: {}%", fault_pct));
        }
    }

    // Determine status
    let status = if !errors.is_empty() {
        NonPecuniaryLiabilityStatus::NotEstablished
    } else {
        NonPecuniaryLiabilityStatus::Established
    };

    let result = Article710Liability {
        article: ArticleReference {
            number: "710".to_string(),
            title: "財産以外の損害の賠償 (Non-Pecuniary Damages)".to_string(),
        },
        based_on_article_709: claim.article_709_claim.is_some(),
        status: status.clone(),
        recommended_consolation_money: final_compensation,
        calculation_factors,
        validation_details,
    };

    if !errors.is_empty() {
        if errors.len() == 1 {
            match errors.into_iter().next() {
                Some(err) => Err(err),
                None => Err(ValidationError::Multiple(vec![])),
            }
        } else {
            Err(ValidationError::Multiple(errors))
        }
    } else {
        Ok(result)
    }
}

/// Validate an Article 715 employer liability claim
///
/// Checks that:
/// 1. Employment relationship exists
/// 2. Employee committed an Article 709 tort
/// 3. Tort occurred during business execution (事業執行について)
/// 4. Evaluates employer's defenses (reasonable care, unavoidable damage)
///
/// Returns Article715Liability with vicarious liability status.
pub fn validate_article_715(
    claim: &crate::tort::article715::Article715,
) -> Result<crate::tort::article715::Article715Liability, ValidationError> {
    use crate::tort::article709::ArticleReference;
    use crate::tort::article715::{Article715Liability, VicariousLiabilityStatus};
    use crate::tort::types::EmploymentType;

    let mut validation_details = Vec::new();
    let mut reasoning = Vec::new();
    let mut applicable_defenses = Vec::new();
    let mut errors = Vec::new();

    // Requirement 1: Employment relationship
    match &claim.employment_relationship {
        Some(relationship) => {
            validation_details.push(format!(
                "Employment relationship: {} (employer) - {} (employee)",
                relationship.employer_name, relationship.employee_name
            ));

            // Check for independent contractor (excludes Article 715)
            if relationship.employment_type == EmploymentType::Independent {
                errors.push(ValidationError::IndependentContractor);
                validation_details.push(
                    "Independent contractor relationship - Article 715 does not apply".to_string(),
                );
            } else {
                let etype_desc = match relationship.employment_type {
                    EmploymentType::FullTime => "Full-time employee",
                    EmploymentType::PartTime => "Part-time employee",
                    EmploymentType::Contract => "Contract employee",
                    EmploymentType::Dispatch => "Dispatch worker",
                    EmploymentType::Agent => "Agent/Representative",
                    EmploymentType::Independent => "Independent", // already handled
                };
                reasoning.push(format!("Employment type: {}", etype_desc));
            }
        }
        None => {
            errors.push(ValidationError::NoEmploymentRelationship);
            validation_details.push("No employment relationship established".to_string());
        }
    }

    // Requirement 2: Employee's Article 709 tort
    match &claim.employee_tort {
        Some(employee_tort) => match validate_tort_claim(employee_tort) {
            Ok(liability) => {
                if liability.is_liability_established() {
                    validation_details
                        .push("Employee's Article 709 tort liability established".to_string());
                    reasoning.push("Employee committed tortious act".to_string());
                } else {
                    errors.push(ValidationError::NoIntentOrNegligence);
                    validation_details
                        .push("Employee's tort liability not established".to_string());
                }
            }
            Err(_) => {
                errors.push(ValidationError::NoIntentOrNegligence);
                validation_details.push("Employee's tort validation failed".to_string());
            }
        },
        None => {
            errors.push(ValidationError::NoIntentOrNegligence);
            validation_details.push("No employee tort provided".to_string());
        }
    }

    // Requirement 3: During business execution (事業執行について)
    match claim.during_business_execution {
        Some(true) => {
            validation_details
                .push("Tort occurred during business execution (事業執行について)".to_string());
            reasoning
                .push("External appearance doctrine: act appeared business-related".to_string());

            if let Some(context) = &claim.business_context {
                validation_details.push(format!("Business context: {}", context));
            }
        }
        Some(false) => {
            errors.push(ValidationError::NotDuringBusinessExecution);
            validation_details.push("Tort did not occur during business execution".to_string());
        }
        None => {
            errors.push(ValidationError::NotDuringBusinessExecution);
            validation_details.push("Business execution context not specified".to_string());
        }
    }

    // Defense 1: Reasonable care in appointment
    let mut has_appointment_defense = false;
    if let Some(true) = claim.reasonable_care_appointment {
        applicable_defenses.push("Reasonable care in appointment (選任の際の相当注意)".to_string());
        has_appointment_defense = true;
    }

    // Defense 2: Reasonable care in supervision
    let mut has_supervision_defense = false;
    if let Some(true) = claim.reasonable_care_supervision {
        applicable_defenses.push("Reasonable care in supervision (監督の際の相当注意)".to_string());
        has_supervision_defense = true;
    }

    // Defense 3: Unavoidable damage
    if let Some(true) = claim.unavoidable_damage {
        applicable_defenses.push("Damage unavoidable despite reasonable care".to_string());
    }

    // Evidence of defenses
    if let Some(evidence) = &claim.care_evidence {
        validation_details.push(format!("Defense evidence: {}", evidence));
    }

    // Determine liability status
    let (employer_liable, status) = if !errors.is_empty() {
        (
            false,
            VicariousLiabilityStatus::NotLiable {
                defense_reason: "Essential requirements not met".to_string(),
            },
        )
    } else if has_appointment_defense && has_supervision_defense {
        // Both defenses present - employer may not be liable
        (
            false,
            VicariousLiabilityStatus::NotLiable {
                defense_reason:
                    "Employer exercised reasonable care in both appointment and supervision"
                        .to_string(),
            },
        )
    } else if claim.unavoidable_damage == Some(true) {
        (
            false,
            VicariousLiabilityStatus::NotLiable {
                defense_reason: "Damage would have occurred despite reasonable care".to_string(),
            },
        )
    } else {
        // Employer is liable
        reasoning.push("All requirements met; no complete defense established".to_string());
        (true, VicariousLiabilityStatus::Liable)
    };

    let result = Article715Liability {
        article: ArticleReference {
            number: "715(1)".to_string(),
            title: "使用者責任 (Employer's Vicarious Liability)".to_string(),
        },
        employer_liable,
        status: status.clone(),
        reasoning,
        applicable_defenses,
        compensation_basis: if employer_liable {
            Some("Based on employee's Article 709 tort damages".to_string())
        } else {
            None
        },
        validation_details,
    };

    if !errors.is_empty() && !employer_liable {
        if errors.len() == 1 {
            match errors.into_iter().next() {
                Some(err) => Err(err),
                None => Err(ValidationError::Multiple(vec![])),
            }
        } else {
            Err(ValidationError::Multiple(errors))
        }
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tort::types::*;

    #[test]
    fn test_validate_complete_claim() {
        let claim = Article709::new()
            .with_act("交通事故で相手の車に衝突")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両所有権"))
            .with_damage(Damage::new(500_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        let result = validate_tort_claim(&claim);
        assert!(result.is_ok());

        if let Ok(liability) = result {
            assert_eq!(liability.status, LiabilityStatus::Established);
            assert!(liability.is_liability_established());
        }
    }

    #[test]
    fn test_validate_missing_intent() {
        let claim = Article709::new()
            .with_act("交通事故")
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(500_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        let result = validate_tort_claim(&claim);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(
                err,
                ValidationError::NoIntentOrNegligence | ValidationError::Multiple(_)
            ));
        }
    }

    #[test]
    fn test_validate_missing_causation() {
        let claim = Article709::new()
            .with_act("交通事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(500_000, "修理費"));

        let result = validate_tort_claim(&claim);
        assert!(result.is_err());
    }

    #[test]
    fn test_meets_all_requirements() {
        let complete_claim = Article709::new()
            .with_act("交通事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(500_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        assert!(meets_all_requirements(&complete_claim));

        let incomplete_claim = Article709::new().with_act("交通事故");

        assert!(!meets_all_requirements(&incomplete_claim));
    }

    #[test]
    fn test_calculate_compensation() {
        let claim = Article709::new()
            .with_act("交通事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(500_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        let compensation = calculate_compensation(&claim);
        assert_eq!(compensation, Some(500_000));
    }

    #[test]
    fn test_responsibility_capacity_child() {
        let claim = Article709::new()
            .with_act("小学生がボールで窓ガラス破損")
            .with_intent(Intent::Intentional { age: 10 })
            .with_victim_interest(ProtectedInterest::Property("窓ガラス"))
            .with_damage(Damage::new(50_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        let result = validate_tort_claim(&claim);
        // Should fail due to lack of responsibility capacity
        assert!(result.is_err());
    }
}
