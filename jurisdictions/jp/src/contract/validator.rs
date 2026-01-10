//! Validation logic for Article 415 breach of obligation claims
//!
//! This module implements the validation logic for determining whether
//! a breach claim satisfies the requirements of Article 415.

use crate::contract::article415::{ArticleReference, BreachLiability, LiabilityStatus};
use crate::contract::error::ContractLiabilityError;
use crate::contract::types::{AttributionType, BreachType, ObligationType};

/// Validate a breach of obligation claim under Article 415
///
/// Checks all five requirements of Article 415:
/// 1. Obligation existence (債務の存在)
/// 2. Breach/Non-performance (不履行)
/// 3. Attribution to debtor (帰責事由)
/// 4. Causation (因果関係)
/// 5. Damages (損害)
///
/// ## Example
///
/// ```rust
/// use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};
/// use legalis_jp::contract::validate_breach_claim;
/// use legalis_jp::tort::{Damage, CausalLink};
///
/// let claim = Article415::new()
///     .with_obligation(ObligationType::Monetary {
///         amount: 1_000_000,
///         currency: "JPY".to_string(),
///     })
///     .with_breach(BreachType::NonPerformance)
///     .with_attribution(Attribution::new(
///         AttributionType::Negligence,
///         "正当な理由なく履行を拒否"
///     ))
///     .with_damage(Damage::new(1_000_000, "契約金額"))
///     .with_causal_link(CausalLink::Direct)
///     .creditor("会社A")
///     .debtor("供給業者B");
///
/// let result = validate_breach_claim(&claim);
/// if let Ok(liability) = result {
///     assert!(liability.is_liability_established());
/// }
/// ```
pub fn validate_breach_claim(
    claim: &crate::contract::article415::Article415,
) -> Result<BreachLiability, ContractLiabilityError> {
    let mut validation_details = Vec::new();
    let mut errors = Vec::new();

    // Requirement 1: Obligation existence (債務の存在)
    match &claim.obligation {
        Some(obligation) => {
            let obligation_desc = match obligation {
                ObligationType::Monetary { amount, currency } => {
                    format!("Monetary obligation: {} {}", amount, currency)
                }
                ObligationType::Delivery { description } => {
                    format!("Delivery obligation: {}", description)
                }
                ObligationType::Service {
                    description,
                    duration,
                } => {
                    if let Some(dur) = duration {
                        format!("Service obligation: {} (duration: {})", description, dur)
                    } else {
                        format!("Service obligation: {}", description)
                    }
                }
                ObligationType::Other(desc) => format!("Other obligation: {}", desc),
            };
            validation_details.push(format!("Obligation exists: {}", obligation_desc));
        }
        None => {
            errors.push(ContractLiabilityError::NecessaryConditionMissing(
                "債務の存在 (Obligation existence)".to_string(),
            ));
            validation_details.push("No obligation specified".to_string());
        }
    }

    // Requirement 2: Breach/Non-performance (不履行)
    match &claim.breach {
        Some(breach) => {
            let breach_desc = match breach {
                BreachType::NonPerformance => "Complete non-performance (不履行)".to_string(),
                BreachType::DelayedPerformance { days_late } => {
                    format!("Delayed performance: {} days late (履行遅滞)", days_late)
                }
                BreachType::DefectivePerformance { description } => {
                    format!("Defective performance: {} (不完全履行)", description)
                }
            };
            validation_details.push(format!("Breach type: {}", breach_desc));
        }
        None => {
            errors.push(ContractLiabilityError::NecessaryConditionMissing(
                "不履行 (Breach/Non-performance)".to_string(),
            ));
            validation_details.push("No breach specified".to_string());
        }
    }

    // Requirement 3: Attribution to debtor (帰責事由)
    match &claim.attribution {
        Some(attribution) => {
            let attr_desc = match &attribution.attribution_type {
                AttributionType::Intentional => "Intentional breach (故意)",
                AttributionType::Negligence => "Negligent breach (過失)",
                AttributionType::StrictLiability => {
                    "Strict liability - no fault required (無過失責任)"
                }
            };
            validation_details.push(format!(
                "Attribution: {} - {}",
                attr_desc, attribution.explanation
            ));
        }
        None => {
            errors.push(ContractLiabilityError::NecessaryConditionMissing(
                "帰責事由 (Attribution)".to_string(),
            ));
            validation_details.push("No attribution to debtor".to_string());
        }
    }

    // Requirement 4: Causation (因果関係)
    match &claim.causal_link {
        Some(link) => {
            use crate::tort::types::CausalLink;
            let causation_desc = match link {
                CausalLink::Direct => "Direct causation (直接因果関係)".to_string(),
                CausalLink::Adequate(reason) => {
                    format!("Adequate causation (相当因果関係): {}", reason)
                }
                CausalLink::Conditional {
                    condition,
                    explanation,
                } => {
                    format!("Conditional causation: {} - {}", condition, explanation)
                }
            };
            validation_details.push(format!("Causation: {}", causation_desc));
        }
        None => {
            errors.push(ContractLiabilityError::NecessaryConditionMissing(
                "因果関係 (Causation)".to_string(),
            ));
            validation_details.push("No causal link established".to_string());
        }
    }

    // Requirement 5: Damages occurred (損害の発生)
    match &claim.damage {
        Some(damage) => {
            validation_details.push(format!(
                "Damages proven: ¥{} ({})",
                damage.amount, damage.description
            ));
        }
        None => {
            errors.push(ContractLiabilityError::NecessaryConditionMissing(
                "損害の発生 (Damages)".to_string(),
            ));
            validation_details.push("No damages proven".to_string());
        }
    }

    // Add creditor/debtor information if available
    if let Some(creditor) = &claim.creditor {
        validation_details.push(format!("Creditor: {}", creditor));
    }
    if let Some(debtor) = &claim.debtor {
        validation_details.push(format!("Debtor: {}", debtor));
    }

    // Add contract timing information if available
    if let Some(contract_date) = &claim.contract_date {
        validation_details.push(format!("Contract date: {}", contract_date));
    }
    if let Some(due_date) = &claim.due_date {
        validation_details.push(format!("Due date: {}", due_date));
    }

    // Determine liability status
    let status = if !errors.is_empty() {
        LiabilityStatus::NotEstablished
    } else {
        LiabilityStatus::Established
    };

    let compensation_basis = if status == LiabilityStatus::Established {
        claim
            .damage
            .as_ref()
            .map(|d| format!("Contract breach damages: ¥{} ({})", d.amount, d.description))
    } else {
        None
    };

    let result = BreachLiability {
        article: ArticleReference::default(),
        status: status.clone(),
        validation_details,
        compensation_basis,
    };

    // If there are errors, return error
    if !errors.is_empty() {
        if errors.len() == 1 {
            // Extract the single error without unwrap
            match errors.into_iter().next() {
                Some(err) => Err(err),
                None => Err(ContractLiabilityError::Multiple(vec![])),
            }
        } else {
            Err(ContractLiabilityError::Multiple(errors))
        }
    } else {
        Ok(result)
    }
}

/// Check if all Article 415 requirements are met
///
/// Returns true only if all five requirements are satisfied.
pub fn meets_all_requirements(claim: &crate::contract::article415::Article415) -> bool {
    claim.obligation.is_some()
        && claim.breach.is_some()
        && claim.attribution.is_some()
        && claim.causal_link.is_some()
        && claim.damage.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::article415::Article415;
    use crate::contract::types::*;
    use crate::tort::types::{CausalLink, Damage};

    #[test]
    fn test_validate_complete_claim() {
        let claim = Article415::new()
            .with_obligation(ObligationType::Monetary {
                amount: 1_000_000,
                currency: "JPY".to_string(),
            })
            .with_breach(BreachType::NonPerformance)
            .with_attribution(Attribution::new(
                AttributionType::Negligence,
                "正当な理由なく履行を拒否",
            ))
            .with_damage(Damage::new(1_000_000, "契約金額"))
            .with_causal_link(CausalLink::Direct)
            .creditor("会社A")
            .debtor("供給業者B");

        let result = validate_breach_claim(&claim);
        assert!(result.is_ok());

        if let Ok(liability) = result {
            assert_eq!(liability.status, LiabilityStatus::Established);
            assert!(liability.is_liability_established());
        }
    }

    #[test]
    fn test_validate_missing_obligation() {
        let claim = Article415::new()
            .with_breach(BreachType::NonPerformance)
            .with_attribution(Attribution::new(AttributionType::Negligence, "過失"))
            .with_damage(Damage::new(500_000, "損害"))
            .with_causal_link(CausalLink::Direct);

        let result = validate_breach_claim(&claim);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(
                err,
                ContractLiabilityError::NecessaryConditionMissing(_)
                    | ContractLiabilityError::Multiple(_)
            ));
        }
    }

    #[test]
    fn test_validate_missing_causation() {
        let claim = Article415::new()
            .with_obligation(ObligationType::Delivery {
                description: "商品の引渡し".to_string(),
            })
            .with_breach(BreachType::NonPerformance)
            .with_attribution(Attribution::new(AttributionType::Negligence, "過失"))
            .with_damage(Damage::new(500_000, "逸失利益"));

        let result = validate_breach_claim(&claim);
        assert!(result.is_err());
    }

    #[test]
    fn test_meets_all_requirements() {
        let complete_claim = Article415::new()
            .with_obligation(ObligationType::Service {
                description: "システム開発".to_string(),
                duration: Some("6ヶ月".to_string()),
            })
            .with_breach(BreachType::DelayedPerformance { days_late: 30 })
            .with_attribution(Attribution::new(AttributionType::Negligence, "人員不足"))
            .with_damage(Damage::new(2_000_000, "遅延損害金"))
            .with_causal_link(CausalLink::Direct);

        assert!(meets_all_requirements(&complete_claim));

        let incomplete_claim =
            Article415::new().with_obligation(ObligationType::Other("その他".to_string()));

        assert!(!meets_all_requirements(&incomplete_claim));
    }

    #[test]
    fn test_strict_liability_attribution() {
        let claim = Article415::new()
            .with_obligation(ObligationType::Delivery {
                description: "商品配達".to_string(),
            })
            .with_breach(BreachType::DefectivePerformance {
                description: "商品破損".to_string(),
            })
            .with_attribution(Attribution::new(
                AttributionType::StrictLiability,
                "運送業者の無過失責任",
            ))
            .with_damage(Damage::new(300_000, "商品価格"))
            .with_causal_link(CausalLink::Direct);

        let result = validate_breach_claim(&claim);
        assert!(result.is_ok());
        if let Ok(liability) = result {
            assert!(liability.is_liability_established());
        }
    }
}
