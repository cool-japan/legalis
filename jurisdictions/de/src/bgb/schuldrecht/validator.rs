//! BGB Contract Law Validators (Schuldrecht)
//!
//! Multi-stage validation implementing BGB contract law requirements.

use chrono::Utc;

use crate::bgb::schuldrecht::error::{Result, SchuldrechtError};
use crate::bgb::schuldrecht::types::{
    Acceptance, Breach, Contract, ContractStatus, ContractTerms, DamagesClaim, Declaration,
    LegalCapacity, Offer, Party, Remedy, RemedyType, Termination, TerminationGrounds,
};

/// Validate a party's legal capacity to enter contracts
///
/// Per §§104-115 BGB, legal capacity determines ability to enter binding
/// contracts. Natural persons gain full capacity at age 18.
///
/// # Legal Requirements
///
/// - **Full capacity (§105)**: Can enter all contracts independently
/// - **Limited capacity (§106-113)**: Requires representative consent except
///   for "purely beneficial" transactions and spending allowances
/// - **No capacity (§104)**: All declarations void
pub fn validate_party_capacity(party: &Party) -> Result<()> {
    match party.legal_capacity {
        LegalCapacity::None => Err(SchuldrechtError::NoLegalCapacity {
            party_name: party.name.clone(),
        }),
        LegalCapacity::Limited => {
            if party.legal_representative.is_none() {
                Err(SchuldrechtError::MissingLegalRepresentative {
                    party: party.name.clone(),
                })
            } else {
                // Note: Actual consent checking would require additional context
                // This validator only ensures representative is designated
                Ok(())
            }
        }
        LegalCapacity::Full => Ok(()),
    }
}

/// Validate a declaration of intent per §§116-144 BGB
///
/// A valid declaration requires:
/// 1. No invalidating mistakes (§§119-122 BGB)
/// 2. No duress or fraud (§123 BGB)
/// 3. Receipt by intended recipient (§130 BGB)
/// 4. Declarant has legal capacity
pub fn validate_declaration(declaration: &Declaration) -> Result<()> {
    // Check legal capacity
    validate_party_capacity(&declaration.declarant)?;

    // Check for voidability grounds
    if let Some(mistake_type) = declaration.mistake_type {
        return Err(SchuldrechtError::VoidableDueToMistake {
            mistake_type: format!("{:?}", mistake_type),
        });
    }

    if declaration.under_duress {
        return Err(SchuldrechtError::VoidableDueToDuress);
    }

    // Check receipt (§130 BGB - declaration effective upon receipt)
    if !declaration.received {
        return Err(SchuldrechtError::DeclarationNotReceived);
    }

    Ok(())
}

/// Validate an offer per §§145-157 BGB
///
/// A valid offer requires:
/// 1. Sufficiently specific terms (essentialia negotii)
/// 2. Intent to be bound
/// 3. Valid offeror
/// 4. Not revoked
/// 5. Within acceptance period (if specified)
pub fn validate_offer(offer: &Offer) -> Result<()> {
    // Validate offeror capacity
    validate_party_capacity(&offer.offeror)?;

    // Check if offer is still valid
    if offer.revoked {
        return Err(SchuldrechtError::OfferRevoked);
    }

    // Check acceptance deadline
    if let Some(deadline) = offer.acceptance_deadline
        && Utc::now() > deadline
    {
        return Err(SchuldrechtError::AcceptanceDeadlineExpired);
    }

    // Check if offer is binding
    if !offer.binding {
        return Err(SchuldrechtError::OfferNotBinding);
    }

    // Validate contract terms
    validate_contract_terms(&offer.terms)?;

    Ok(())
}

/// Validate contract terms have essential elements
///
/// Per §154 BGB, contract must have "essentialia negotii" (essential terms).
/// This typically includes:
/// - Subject matter (Vertragsgegenstand)
/// - Parties (implied in offer/acceptance)
/// - Price/consideration (for contracts with consideration)
pub fn validate_contract_terms(terms: &ContractTerms) -> Result<()> {
    // Subject matter must be specified
    if terms.subject_matter.trim().is_empty() {
        return Err(SchuldrechtError::SubjectMatterUnclear);
    }

    // Essential terms should not be empty for complex contracts
    if terms.essential_terms.is_empty() {
        let missing = vec!["Essential terms not specified".to_string()];
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: missing,
        });
    }

    Ok(())
}

/// Validate acceptance per §§147-150 BGB
///
/// A valid acceptance requires:
/// 1. Made by offeree or authorized person
/// 2. Timely (within deadline or reasonable period)
/// 3. Unconditional (no modifications)
/// 4. Acceptor has legal capacity
pub fn validate_acceptance(acceptance: &Acceptance, offer: &Offer) -> Result<()> {
    // Validate acceptor capacity
    validate_party_capacity(&acceptance.acceptor)?;

    // Check if acceptance is timely
    if !acceptance.timely {
        return Err(SchuldrechtError::LateAcceptance);
    }

    // Check for modifications (§150 Abs. 2 BGB - counts as rejection + counter-offer)
    if let Some(ref mods) = acceptance.modifications
        && !mods.is_empty()
    {
        return Err(SchuldrechtError::AcceptanceWithModifications {
            modifications: mods.clone(),
        });
    }

    // Ensure offer is still valid
    validate_offer(offer)?;

    Ok(())
}

/// Validate contract formation per §§145-157 BGB
///
/// A contract is concluded when:
/// 1. Valid offer exists
/// 2. Valid acceptance received
/// 3. Both parties have legal capacity
/// 4. Required form is met (if applicable)
pub fn validate_contract_formation(
    offer: &Offer,
    acceptance: &Acceptance,
    requires_written_form: bool,
    in_writing: bool,
) -> Result<()> {
    // Validate offer
    validate_offer(offer)?;

    // Validate acceptance
    validate_acceptance(acceptance, offer)?;

    // Check form requirements (§§125-129 BGB)
    if requires_written_form && !in_writing {
        return Err(SchuldrechtError::WrittenFormRequired);
    }

    Ok(())
}

/// Validate a concluded contract
///
/// Checks that contract has all required elements and is enforceable.
pub fn validate_contract(contract: &Contract) -> Result<()> {
    // Check parties
    if contract.parties.is_empty() {
        return Err(SchuldrechtError::MissingParty);
    }

    // Validate each party's capacity
    for party in &contract.parties {
        validate_party_capacity(party)?;
    }

    // Validate contract terms
    validate_contract_terms(&contract.terms)?;

    // Check contract status
    match contract.status {
        ContractStatus::Void => Err(SchuldrechtError::ContractNotConcluded),
        ContractStatus::Voidable => {
            // Voidable contracts are valid until voided
            Ok(())
        }
        ContractStatus::Concluded => Ok(()),
        ContractStatus::OfferPending => Err(SchuldrechtError::ContractNotConcluded),
        ContractStatus::Terminated | ContractStatus::Rescinded => Ok(()),
    }
}

/// Validate a breach of contract claim per §280 BGB
///
/// Requirements for damages under §280 Abs. 1 BGB:
/// 1. Schuldverhältnis (obligation relationship) exists
/// 2. Pflichtverletzung (breach of duty)
/// 3. Verschulden (fault) - unless proven otherwise by debtor
/// 4. Schaden (damage)
/// 5. Kausalität (causation)
pub fn validate_breach(breach: &Breach) -> Result<()> {
    // Contract must exist
    if breach.contract_id.is_empty() {
        return Err(SchuldrechtError::ContractNotConcluded);
    }

    // Breaching party must be identified
    if breach.breaching_party.is_empty() {
        return Err(SchuldrechtError::MissingParty);
    }

    // Description should be provided
    if breach.description.trim().is_empty() {
        return Err(SchuldrechtError::NoDamagesClaim {
            missing_elements: vec!["Breach description missing".to_string()],
        });
    }

    Ok(())
}

/// Validate damages claim per §§280-283 BGB
///
/// General damages claim (§280 Abs. 1 BGB) requires:
/// - Breach of obligation
/// - Fault (presumed unless debtor proves otherwise)
/// - Damage
/// - Causation
///
/// Damages in lieu of performance (§281 BGB) additionally requires:
/// - Grace period set and expired (unless exception applies)
pub fn validate_damages_claim(claim: &DamagesClaim) -> Result<()> {
    // Parties must be identified
    if claim.claimant.is_empty() || claim.respondent.is_empty() {
        return Err(SchuldrechtError::MissingParty);
    }

    // Fault must be proven (§280 Abs. 1 S. 2 BGB)
    if !claim.fault_proven {
        return Err(SchuldrechtError::FaultNotProven {
            party: claim.respondent.clone(),
        });
    }

    // Causation must be proven
    if !claim.causation_proven {
        return Err(SchuldrechtError::CausationNotProven);
    }

    // Damage types should be specified
    if claim.damage_types.is_empty() {
        return Err(SchuldrechtError::NoDamagesClaim {
            missing_elements: vec!["Damage types not specified".to_string()],
        });
    }

    // Amount must be positive
    if claim.amount_claimed.amount_cents == 0 {
        return Err(SchuldrechtError::DamageAmountNotProven);
    }

    Ok(())
}

/// Validate remedy claim
///
/// Different remedies have different requirements under BGB.
pub fn validate_remedy(remedy: &Remedy) -> Result<()> {
    // Contract must exist (for most remedies)
    if remedy.contract_id.is_empty() {
        return Err(SchuldrechtError::ContractNotConcluded);
    }

    // Parties must be identified
    if remedy.claimant.is_empty() || remedy.respondent.is_empty() {
        return Err(SchuldrechtError::MissingParty);
    }

    // Special validation for specific remedy types
    match remedy.remedy_type {
        RemedyType::DamagesInLieu => {
            // §281 BGB - requires grace period unless exception applies
            if let Some(days) = remedy.grace_period_days {
                if !remedy.grace_period_expired {
                    return Err(SchuldrechtError::GracePeriodNotExpired { days });
                }
            } else {
                return Err(SchuldrechtError::GracePeriodRequired);
            }
        }
        RemedyType::Damages => {
            // General damages (§280 BGB) - less strict
            // Fault and causation checked elsewhere
        }
        RemedyType::Termination => {
            // Termination validated separately
        }
        _ => {
            // Other remedies: basic validation sufficient
        }
    }

    Ok(())
}

/// Validate termination (Rücktritt) per §§323-326 BGB
///
/// Requirements for termination:
/// 1. Contract exists and is not yet fully performed
/// 2. Breach occurred
/// 3. Grace period set and expired (§323 Abs. 1) OR exception applies
/// 4. Breach is not minor (§323 Abs. 5 S. 2)
pub fn validate_termination(termination: &Termination) -> Result<()> {
    // Contract must exist
    if termination.contract_id.is_empty() {
        return Err(SchuldrechtError::ContractNotConcluded);
    }

    // Check grounds for termination
    match termination.grounds {
        TerminationGrounds::NonPerformanceAfterGracePeriod => {
            // Standard case: requires grace period (§323 Abs. 1 BGB)
            if !termination.grace_period_set_and_expired {
                Err(SchuldrechtError::TerminationRequiresGracePeriod)
            } else {
                Ok(())
            }
        }
        TerminationGrounds::MinorBreach => {
            // Minor breach excludes termination (§323 Abs. 5 S. 2 BGB)
            Err(SchuldrechtError::MinorBreachNoTermination)
        }
        TerminationGrounds::RefusalToPerform
        | TerminationGrounds::PerformanceImpossible
        | TerminationGrounds::SeriousBreach => {
            // These grounds allow termination without grace period
            // (§323 Abs. 2 BGB)
            Ok(())
        }
    }
}

/// Validate timeliness of acceptance
///
/// Per §§147-149 BGB:
/// - With deadline: acceptance must be received within deadline
/// - Without deadline: acceptance must be made within reasonable time
/// - Present parties: immediate acceptance required
pub fn validate_acceptance_timeliness(
    offer: &Offer,
    acceptance: &Acceptance,
    parties_present: bool,
) -> Result<()> {
    if let Some(deadline) = offer.acceptance_deadline {
        // Explicit deadline specified
        if acceptance.accepted_at > deadline {
            return Err(SchuldrechtError::LateAcceptance);
        }
    } else if parties_present {
        // Present parties - immediate acceptance required (§147 Abs. 1 BGB)
        // In practice, "immediate" means within the same conversation/meeting
        // We check if acceptance is same day as offer
        if acceptance.accepted_at.date_naive() != offer.offered_at.date_naive() {
            return Err(SchuldrechtError::LateAcceptance);
        }
    }
    // For absent parties without deadline, "reasonable time" is fact-specific
    // and cannot be validated programmatically

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgb::schuldrecht::types::{
        BreachType, DamageType, DamagesLegalBasis, FaultLevel, PartyType,
    };
    use crate::gmbhg::Capital;

    fn create_test_party(capacity: LegalCapacity) -> Party {
        Party {
            name: "Test Person".to_string(),
            address: "Test Address".to_string(),
            legal_capacity: capacity,
            legal_representative: if capacity == LegalCapacity::Limited {
                Some("Legal Rep".to_string())
            } else {
                None
            },
            party_type: PartyType::NaturalPerson,
        }
    }

    #[test]
    fn test_validate_party_full_capacity() {
        let party = create_test_party(LegalCapacity::Full);
        assert!(validate_party_capacity(&party).is_ok());
    }

    #[test]
    fn test_validate_party_no_capacity() {
        let party = create_test_party(LegalCapacity::None);
        let result = validate_party_capacity(&party);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::NoLegalCapacity { .. }
        ));
    }

    #[test]
    fn test_validate_party_limited_capacity_with_representative() {
        let party = create_test_party(LegalCapacity::Limited);
        assert!(validate_party_capacity(&party).is_ok());
    }

    #[test]
    fn test_validate_party_limited_capacity_without_representative() {
        let mut party = create_test_party(LegalCapacity::Limited);
        party.legal_representative = None;
        let result = validate_party_capacity(&party);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::MissingLegalRepresentative { .. }
        ));
    }

    #[test]
    fn test_validate_declaration_valid() {
        let declaration = Declaration {
            declarant: create_test_party(LegalCapacity::Full),
            content: "Test declaration".to_string(),
            declared_at: Utc::now(),
            received: true,
            received_at: Some(Utc::now()),
            mental_reservation: false,
            under_duress: false,
            mistake_type: None,
        };

        assert!(validate_declaration(&declaration).is_ok());
    }

    #[test]
    fn test_validate_declaration_not_received() {
        let mut declaration = Declaration {
            declarant: create_test_party(LegalCapacity::Full),
            content: "Test".to_string(),
            declared_at: Utc::now(),
            received: false,
            received_at: None,
            mental_reservation: false,
            under_duress: false,
            mistake_type: None,
        };

        let result = validate_declaration(&declaration);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::DeclarationNotReceived
        ));

        declaration.received = true;
        declaration.under_duress = true;
        let result2 = validate_declaration(&declaration);
        assert!(matches!(
            result2.unwrap_err(),
            SchuldrechtError::VoidableDueToDuress
        ));
    }

    #[test]
    fn test_validate_contract_terms() {
        let valid_terms = ContractTerms {
            subject_matter: "Sale of car".to_string(),
            consideration: Some(Capital::from_euros(10_000)),
            essential_terms: vec!["Car: VW Golf".to_string(), "Price: €10,000".to_string()],
            additional_terms: vec![],
            includes_gtc: false,
        };

        assert!(validate_contract_terms(&valid_terms).is_ok());
    }

    #[test]
    fn test_validate_contract_terms_missing_subject() {
        let invalid_terms = ContractTerms {
            subject_matter: "".to_string(),
            consideration: None,
            essential_terms: vec![],
            additional_terms: vec![],
            includes_gtc: false,
        };

        let result = validate_contract_terms(&invalid_terms);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::SubjectMatterUnclear
        ));
    }

    #[test]
    fn test_validate_offer_valid() {
        let offer = Offer {
            offeror: create_test_party(LegalCapacity::Full),
            offeree: create_test_party(LegalCapacity::Full),
            terms: ContractTerms {
                subject_matter: "Services".to_string(),
                consideration: Some(Capital::from_euros(1_000)),
                essential_terms: vec!["Service provision".to_string()],
                additional_terms: vec![],
                includes_gtc: false,
            },
            offered_at: Utc::now(),
            acceptance_deadline: Some(Utc::now() + chrono::Duration::days(7)),
            binding: true,
            revoked: false,
        };

        assert!(validate_offer(&offer).is_ok());
    }

    #[test]
    fn test_validate_offer_revoked() {
        let mut offer = Offer {
            offeror: create_test_party(LegalCapacity::Full),
            offeree: create_test_party(LegalCapacity::Full),
            terms: ContractTerms {
                subject_matter: "Test".to_string(),
                consideration: None,
                essential_terms: vec!["Test".to_string()],
                additional_terms: vec![],
                includes_gtc: false,
            },
            offered_at: Utc::now(),
            acceptance_deadline: None,
            binding: true,
            revoked: false,
        };

        assert!(validate_offer(&offer).is_ok());

        offer.revoked = true;
        let result = validate_offer(&offer);
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::OfferRevoked
        ));
    }

    #[test]
    fn test_validate_breach() {
        let breach = Breach {
            contract_id: "C001".to_string(),
            breaching_party: "Party A".to_string(),
            breach_type: BreachType::NonPerformance,
            occurred_at: Utc::now(),
            fault: FaultLevel::OrdinaryNegligence,
            description: "Failed to deliver goods".to_string(),
        };

        assert!(validate_breach(&breach).is_ok());
    }

    #[test]
    fn test_validate_damages_claim() {
        let claim = DamagesClaim {
            contract_id: Some("C001".to_string()),
            claimant: "Buyer".to_string(),
            respondent: "Seller".to_string(),
            legal_basis: DamagesLegalBasis::GeneralBreach,
            damage_types: vec![DamageType::Positive],
            amount_claimed: Capital::from_euros(5_000),
            fault_proven: true,
            causation_proven: true,
        };

        assert!(validate_damages_claim(&claim).is_ok());
    }

    #[test]
    fn test_validate_damages_claim_no_fault() {
        let claim = DamagesClaim {
            contract_id: Some("C001".to_string()),
            claimant: "Buyer".to_string(),
            respondent: "Seller".to_string(),
            legal_basis: DamagesLegalBasis::GeneralBreach,
            damage_types: vec![DamageType::Positive],
            amount_claimed: Capital::from_euros(5_000),
            fault_proven: false,
            causation_proven: true,
        };

        let result = validate_damages_claim(&claim);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::FaultNotProven { .. }
        ));
    }

    #[test]
    fn test_validate_termination_with_grace_period() {
        let termination = Termination {
            contract_id: "C001".to_string(),
            terminating_party: "Buyer".to_string(),
            grounds: TerminationGrounds::NonPerformanceAfterGracePeriod,
            grace_period_set_and_expired: true,
            declared_at: Utc::now(),
            effective: true,
        };

        assert!(validate_termination(&termination).is_ok());
    }

    #[test]
    fn test_validate_termination_minor_breach() {
        let termination = Termination {
            contract_id: "C001".to_string(),
            terminating_party: "Buyer".to_string(),
            grounds: TerminationGrounds::MinorBreach,
            grace_period_set_and_expired: false,
            declared_at: Utc::now(),
            effective: false,
        };

        let result = validate_termination(&termination);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::MinorBreachNoTermination
        ));
    }

    #[test]
    fn test_validate_termination_serious_breach_no_grace_period() {
        let termination = Termination {
            contract_id: "C001".to_string(),
            terminating_party: "Buyer".to_string(),
            grounds: TerminationGrounds::SeriousBreach,
            grace_period_set_and_expired: false,
            declared_at: Utc::now(),
            effective: true,
        };

        // Serious breach allows termination without grace period
        assert!(validate_termination(&termination).is_ok());
    }
}
