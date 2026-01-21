//! Indian Contract Act 1872 Validation
//!
//! Validation logic for contract formation and enforcement

use super::error::{ContractActError, ContractActResult, ContractValidityReport};
use super::types::*;
use chrono::NaiveDate;

/// Validate contract formation
pub fn validate_contract_formation(contract: &Contract) -> ContractValidityReport {
    let mut report = ContractValidityReport::default();

    // Check essential elements (Section 10)
    if !contract.essentials.is_valid() {
        report.valid = false;
        report.status = "Void";
        for missing in contract.essentials.missing_essentials() {
            report.issues.push(ContractActError::VoidContract {
                reason: missing.to_string(),
            });
        }
    }

    // Check party competency (Section 11)
    for party in &contract.parties {
        if !party.is_competent() {
            report.valid = false;
            if let Some(age) = party.age
                && age < 18
            {
                report.issues.push(ContractActError::MinorAgreement);
                report.status = "Void";
            }
            if !party.sound_mind {
                report.issues.push(ContractActError::UnsoundMind);
                report.status = "Void";
            }
            if let Some(disq) = &party.disqualification {
                report.issues.push(ContractActError::IncompetentParty {
                    reason: format!("{:?}", disq),
                });
            }
        }
    }

    // Check contract type specific rules
    match contract.contract_type {
        ContractType::Wagering => {
            report.valid = false;
            report.status = "Void";
            report.issues.push(ContractActError::WageringAgreement);
        }
        ContractType::Contingent => {
            report.warnings.push(
                "Contingent contract - enforceability depends on occurrence of contingent event"
                    .to_string(),
            );
        }
        ContractType::QuasiContract => {
            report.warnings.push(
                "Quasi-contract - obligations arise by operation of law, not agreement".to_string(),
            );
        }
        _ => {}
    }

    // Check for writing requirement
    if !contract.is_written {
        report.warnings.push(
            "Oral contract may be difficult to prove in court. Written contract recommended."
                .to_string(),
        );
    }

    // Check stamp duty (state-specific)
    if contract.is_written && !contract.stamp_duty_paid {
        report.warnings.push(
            "Stamp duty not paid - contract may not be admissible as evidence in court".to_string(),
        );
        report
            .recommendations
            .push("Pay applicable stamp duty as per state Stamp Act".to_string());
    }

    report
}

/// Validate free consent (Sections 13-14)
pub fn validate_consent(vitiating_factors: &[ConsentVitiator]) -> ContractActResult<()> {
    if vitiating_factors.is_empty() {
        return Ok(());
    }

    // Check for factors that make contract void
    for factor in vitiating_factors {
        if matches!(factor, ConsentVitiator::MistakeOfFact) {
            return Err(ContractActError::BilateralMistake);
        }
    }

    // Check for factors that make contract voidable
    for factor in vitiating_factors {
        match factor {
            ConsentVitiator::Coercion => return Err(ContractActError::Coercion),
            ConsentVitiator::UndueInfluence => return Err(ContractActError::UndueInfluence),
            ConsentVitiator::Fraud => return Err(ContractActError::Fraud),
            ConsentVitiator::Misrepresentation => return Err(ContractActError::Misrepresentation),
            _ => {}
        }
    }

    Ok(())
}

/// Validate consideration (Section 2(d), 25)
pub fn validate_consideration(consideration: &Consideration) -> ContractActResult<()> {
    // Check if consideration is present
    if consideration.description.is_empty()
        && consideration.monetary_value.is_none()
        && !matches!(
            consideration.consideration_type,
            ConsiderationType::Forbearance
        )
    {
        return Err(ContractActError::NoConsideration);
    }

    // Past consideration is valid in India (unlike English law)
    // Section 2(d): "has done or abstained from doing, or does or abstains from doing,
    // or promises to do or abstain from doing something"

    Ok(())
}

/// Legality check parameters for Section 23
#[derive(Debug, Clone, Default)]
pub struct LegalityCheck {
    /// Object of the contract
    pub object: String,
    /// Consideration description
    pub consideration: String,
    /// Is the object/consideration immoral
    pub is_immoral: bool,
    /// Is it opposed to public policy
    pub is_opposed_to_public_policy: bool,
    /// Is it forbidden by law
    pub is_forbidden_by_law: bool,
    /// Does it defeat provisions of law
    pub defeats_legal_provision: bool,
    /// Is it fraudulent
    pub fraudulent: bool,
    /// Does it cause injury
    pub causes_injury: bool,
}

/// Validate lawfulness of object and consideration (Section 23)
pub fn validate_legality(check: &LegalityCheck) -> ContractActResult<()> {
    // Section 23: The consideration or object of an agreement is lawful, unless—
    // (a) it is forbidden by law; or
    // (b) is of such a nature that, if permitted, it would defeat the provisions of any law; or
    // (c) is fraudulent; or
    // (d) involves or implies, injury to the person or property of another; or
    // (e) the Court regards it as immoral, or opposed to public policy.

    if check.is_forbidden_by_law {
        return Err(ContractActError::UnlawfulObject {
            reason: "Forbidden by law".to_string(),
        });
    }

    if check.defeats_legal_provision {
        return Err(ContractActError::UnlawfulObject {
            reason: "Would defeat provisions of law".to_string(),
        });
    }

    if check.fraudulent {
        return Err(ContractActError::UnlawfulConsideration {
            reason: "Fraudulent purpose".to_string(),
        });
    }

    if check.causes_injury {
        return Err(ContractActError::UnlawfulObject {
            reason: format!("Causes injury - object: {}", check.object),
        });
    }

    if check.is_immoral {
        return Err(ContractActError::UnlawfulObject {
            reason: "Court regards it as immoral".to_string(),
        });
    }

    if check.is_opposed_to_public_policy {
        return Err(ContractActError::UnlawfulObject {
            reason: "Opposed to public policy".to_string(),
        });
    }

    // Check consideration description for illegal elements
    if check.consideration.to_lowercase().contains("bribe")
        || check.consideration.to_lowercase().contains("illegal")
    {
        return Err(ContractActError::UnlawfulConsideration {
            reason: "Consideration appears to be unlawful".to_string(),
        });
    }

    Ok(())
}

/// Validate void agreements (Sections 24-30)
pub fn validate_void_agreements(
    agreement_type: VoidAgreementType,
    has_exception: bool,
) -> ContractActResult<()> {
    if has_exception {
        // Check if valid exception applies
        let exceptions = agreement_type.exceptions();
        if !exceptions.is_empty() {
            return Ok(());
        }
    }

    match agreement_type {
        VoidAgreementType::NoConsideration => Err(ContractActError::NoConsideration),
        VoidAgreementType::RestraintOfMarriage => Err(ContractActError::RestraintOfMarriage),
        VoidAgreementType::RestraintOfTrade => Err(ContractActError::RestraintOfTrade),
        VoidAgreementType::RestraintOfLegalProceedings => {
            Err(ContractActError::RestraintOfLegalProceedings)
        }
        VoidAgreementType::UncertainAgreement => Err(ContractActError::UncertainAgreement),
        VoidAgreementType::WageringAgreement => Err(ContractActError::WageringAgreement),
        VoidAgreementType::ImpossibleAct => Err(ContractActError::ImpossibilityOfPerformance {
            reason: "Act impossible from beginning".to_string(),
        }),
    }
}

/// Validate performance obligations (Section 37-38)
pub fn validate_performance(
    _contract: &Contract,
    performed: bool,
    on_time: bool,
    time_is_essence: bool,
) -> ContractActResult<()> {
    if !performed {
        if time_is_essence && !on_time {
            return Err(ContractActError::TimeNotKept);
        }
        return Err(ContractActError::FailureToPerform);
    }

    Ok(())
}

/// Validate contingent contract (Sections 31-36)
pub fn validate_contingent_contract(contingent: &ContingentContract) -> ContractActResult<()> {
    // Section 36: If event becomes impossible, contract becomes void
    if !contingent.event_possible {
        return Err(ContractActError::ContingentEventImpossible);
    }

    // Check enforceability
    if !contingent.is_enforceable() {
        // Not an error, just not yet enforceable
        return Ok(());
    }

    Ok(())
}

/// Calculate damages under Section 73
pub fn calculate_damages(
    contract_value: f64,
    actual_loss: f64,
    special_damages: f64,
    special_circumstances_known: bool,
) -> f64 {
    // Section 73: Compensation for loss or damage caused by breach
    // Only damages that arise naturally OR were known to parties at time of contract

    let ordinary_damages = actual_loss.min(contract_value * 2.0); // Natural consequence

    let special = if special_circumstances_known {
        special_damages
    } else {
        0.0
    };

    // Remote damages not recoverable (Hadley v. Baxendale principle)
    ordinary_damages + special
}

/// Validate liquidated damages vs penalty (Section 74)
pub fn validate_liquidated_damages(stipulated_amount: f64, actual_loss: f64) -> (f64, String) {
    // Section 74: Whether the sum named is by way of penalty or liquidated damages,
    // only reasonable compensation can be awarded

    // In India, no distinction made between penalty and liquidated damages
    // Court awards reasonable compensation not exceeding stipulated amount

    if actual_loss == 0.0 {
        // ONGC v. Saw Pipes: No actual loss, nominal damages may be awarded
        return (1.0, "Nominal damages as no actual loss proven".to_string());
    }

    let awarded = actual_loss.min(stipulated_amount);
    let explanation = if actual_loss < stipulated_amount {
        "Actual loss is less than stipulated - actual loss awarded".to_string()
    } else {
        "Actual loss exceeds stipulated amount - capped at stipulated amount".to_string()
    };

    (awarded, explanation)
}

/// Check if doctrine of frustration applies (Section 56)
pub fn check_frustration(
    contract_date: NaiveDate,
    frustrating_event_date: NaiveDate,
    event_description: &str,
    was_foreseeable: bool,
    caused_by_party: bool,
) -> ContractActResult<bool> {
    // Doctrine of frustration under Section 56

    // Event must occur after contract formation
    if frustrating_event_date <= contract_date {
        return Ok(false);
    }

    // Event must not be foreseeable at time of contract
    if was_foreseeable {
        return Ok(false);
    }

    // Event must not be caused by either party
    if caused_by_party {
        return Ok(false);
    }

    // Supervening impossibility
    let frustration_events = [
        "destruction of subject matter",
        "death or incapacity",
        "change of law",
        "outbreak of war",
        "government action",
    ];

    let is_frustrating = frustration_events
        .iter()
        .any(|e| event_description.to_lowercase().contains(e));

    if is_frustrating {
        return Err(ContractActError::ImpossibilityOfPerformance {
            reason: event_description.to_string(),
        });
    }

    Ok(false)
}

/// Validate agency authority (Chapter X)
pub fn validate_agent_authority(
    authority: AgentAuthority,
    _act_performed: &str,
    within_express_authority: bool,
    within_implied_authority: bool,
    ratified: bool,
) -> ContractActResult<()> {
    match authority {
        AgentAuthority::Express => {
            if !within_express_authority {
                return Err(ContractActError::AgentExceededAuthority);
            }
        }
        AgentAuthority::Implied => {
            if !within_implied_authority {
                return Err(ContractActError::AgentExceededAuthority);
            }
        }
        AgentAuthority::Ratification => {
            if !ratified {
                return Err(ContractActError::NotRatified);
            }
        }
        AgentAuthority::Apparent | AgentAuthority::Necessity | AgentAuthority::Estoppel => {
            // These create authority by operation of law
        }
    }

    Ok(())
}

/// Get quasi-contract obligations (Sections 68-72)
pub fn get_quasi_contract_obligation(
    quasi_type: QuasiContractType,
    value_received: f64,
) -> (f64, String) {
    match quasi_type {
        QuasiContractType::NecessariesToIncapable => {
            // Section 68: Reasonable price for necessaries
            (
                value_received,
                "Reasonable price for necessaries supplied".to_string(),
            )
        }
        QuasiContractType::PaymentOfAnothersDebt => {
            // Section 69: Reimbursement
            (value_received, "Reimbursement for payment made".to_string())
        }
        QuasiContractType::NonGratuitousAct => {
            // Section 70: Compensation for non-gratuitous act
            (
                value_received,
                "Compensation for benefit received".to_string(),
            )
        }
        QuasiContractType::FinderOfGoods => {
            // Section 71: Responsibility of finder
            (
                0.0,
                "Finder entitled to retain goods until reasonable reward paid".to_string(),
            )
        }
        QuasiContractType::MistakePayment => {
            // Section 72: Money paid by mistake must be repaid
            (
                value_received,
                "Repayment of money paid by mistake".to_string(),
            )
        }
    }
}

/// Get limitation period for contract claims
pub fn get_limitation_period(claim_type: &str) -> u32 {
    // As per Limitation Act, 1963
    match claim_type.to_lowercase().as_str() {
        "simple contract" => 3,      // 3 years
        "specific performance" => 3, // 3 years
        "partnership accounts" => 3, // 3 years
        "contract under seal" => 12, // 12 years (specialty contract)
        "contribution between co-debtors" => 3,
        _ => 3, // Default 3 years
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_essentials() -> ContractEssentials {
        ContractEssentials {
            free_consent: true,
            competent_parties: true,
            lawful_consideration: true,
            lawful_object: true,
            not_void: true,
        }
    }

    fn create_adult_party() -> ContractParty {
        ContractParty {
            name: "Adult Person".to_string(),
            party_type: PartyType::Individual,
            age: Some(25),
            sound_mind: true,
            disqualification: None,
        }
    }

    #[test]
    fn test_contract_formation_valid() {
        let contract = Contract {
            id: "C001".to_string(),
            contract_type: ContractType::Bilateral,
            status: ContractStatus::Valid,
            parties: vec![create_adult_party(), create_adult_party()],
            agreement_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            consideration: Consideration {
                consideration_type: ConsiderationType::Money,
                monetary_value: Some(100000.0),
                description: "Payment for services".to_string(),
                timing: ConsiderationTiming::Future,
            },
            essentials: create_valid_essentials(),
            subject_matter: "Software development".to_string(),
            performance_due: Some(NaiveDate::from_ymd_opt(2024, 6, 30).expect("valid date")),
            is_written: true,
            stamp_duty_paid: true,
            registered: None,
        };

        let report = validate_contract_formation(&contract);
        assert!(report.valid);
    }

    #[test]
    fn test_minor_agreement_void() {
        let contract = Contract {
            id: "C002".to_string(),
            contract_type: ContractType::Bilateral,
            status: ContractStatus::Valid,
            parties: vec![
                ContractParty {
                    age: Some(16), // Minor
                    ..create_adult_party()
                },
                create_adult_party(),
            ],
            agreement_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            consideration: Consideration {
                consideration_type: ConsiderationType::Money,
                monetary_value: Some(50000.0),
                description: "Payment".to_string(),
                timing: ConsiderationTiming::Future,
            },
            essentials: create_valid_essentials(),
            subject_matter: "Sale of goods".to_string(),
            performance_due: None,
            is_written: true,
            stamp_duty_paid: true,
            registered: None,
        };

        let report = validate_contract_formation(&contract);
        assert!(!report.valid);
        assert_eq!(report.status, "Void");
    }

    #[test]
    fn test_consent_validation() {
        assert!(validate_consent(&[]).is_ok());

        assert!(matches!(
            validate_consent(&[ConsentVitiator::Coercion]),
            Err(ContractActError::Coercion)
        ));

        assert!(matches!(
            validate_consent(&[ConsentVitiator::MistakeOfFact]),
            Err(ContractActError::BilateralMistake)
        ));
    }

    #[test]
    fn test_legality_validation() {
        let valid_check = LegalityCheck {
            object: "Sale of goods".to_string(),
            consideration: "Payment".to_string(),
            ..Default::default()
        };
        assert!(validate_legality(&valid_check).is_ok());

        let illegal_check = LegalityCheck {
            object: "Sale of goods".to_string(),
            consideration: "Payment".to_string(),
            is_forbidden_by_law: true,
            ..Default::default()
        };
        assert!(matches!(
            validate_legality(&illegal_check),
            Err(ContractActError::UnlawfulObject { .. })
        ));
    }

    #[test]
    fn test_damages_calculation() {
        let damages = calculate_damages(100000.0, 50000.0, 20000.0, true);
        assert_eq!(damages, 70000.0); // 50000 + 20000

        let damages_no_special = calculate_damages(100000.0, 50000.0, 20000.0, false);
        assert_eq!(damages_no_special, 50000.0); // Only ordinary
    }

    #[test]
    fn test_liquidated_damages() {
        let (awarded, _) = validate_liquidated_damages(100000.0, 50000.0);
        assert_eq!(awarded, 50000.0); // Actual loss < stipulated

        let (awarded, _) = validate_liquidated_damages(100000.0, 150000.0);
        assert_eq!(awarded, 100000.0); // Capped at stipulated
    }

    #[test]
    fn test_limitation_period() {
        assert_eq!(get_limitation_period("simple contract"), 3);
        assert_eq!(get_limitation_period("contract under seal"), 12);
    }

    #[test]
    fn test_quasi_contract_obligation() {
        let (amount, _) = get_quasi_contract_obligation(QuasiContractType::MistakePayment, 10000.0);
        assert_eq!(amount, 10000.0);
    }
}
