//! Common Law Contract Validation Functions
//!
//! Validates contract formation and terms under English common law,
//! incorporating case law precedents.

use super::error::{ContractError, Result};
use super::types::*;

/// Validate contract formation under common law
///
/// Checks five essential elements:
/// 1. Offer
/// 2. Acceptance
/// 3. Consideration
/// 4. Intention to create legal relations
/// 5. Capacity
///
/// # Example
/// ```ignore
/// let formation = ContractFormation { ... };
/// validate_contract_formation(&formation)?;
/// ```
pub fn validate_contract_formation(formation: &ContractFormation) -> Result<()> {
    let mut missing_elements = Vec::new();

    // 1. Valid offer
    if let Err(e) = validate_offer(&formation.offer) {
        missing_elements.push(format!("✗ Offer: {}", e));
    } else {
        missing_elements.push("✓ Offer: Valid".to_string());
    }

    // 2. Valid acceptance
    if let Some(acceptance) = &formation.acceptance {
        if let Err(e) = validate_acceptance(acceptance, &formation.offer) {
            missing_elements.push(format!("✗ Acceptance: {}", e));
        } else {
            missing_elements.push("✓ Acceptance: Valid".to_string());
        }
    } else {
        missing_elements.push("✗ Acceptance: No acceptance present".to_string());
    }

    // 3. Valid consideration
    if let Err(e) = validate_consideration(&formation.consideration) {
        missing_elements.push(format!("✗ Consideration: {}", e));
    } else {
        missing_elements.push("✓ Consideration: Valid".to_string());
    }

    // 4. Intention to create legal relations
    if !formation.intention.intention_exists {
        missing_elements.push(format!(
            "✗ Intention: No intention to create legal relations ({})",
            match formation.intention.context {
                AgreementContext::Domestic => "Balfour v Balfour [1919]",
                AgreementContext::Social => "Social agreement",
                AgreementContext::Commercial => "Presumed in commercial context",
            }
        ));
    } else {
        missing_elements.push("✓ Intention: Present".to_string());
    }

    // 5. Capacity
    if !formation.capacity.has_capacity {
        missing_elements.push(format!(
            "✗ Capacity: {} lacks capacity",
            formation.capacity.party.name
        ));
    } else {
        missing_elements.push("✓ Capacity: Present".to_string());
    }

    // Check if formation is complete
    let has_errors = missing_elements.iter().any(|e| e.starts_with('✗'));

    if has_errors {
        return Err(ContractError::FormationIncomplete {
            missing_elements: missing_elements.join("\n"),
        });
    }

    Ok(())
}

/// Validate offer
///
/// Requirements:
/// - Definite and certain terms
/// - Communicated to offeree
/// - Still open (not revoked, expired, or rejected)
/// - Distinguished from invitation to treat
pub fn validate_offer(offer: &Offer) -> Result<()> {
    // Check offer is still open
    if !offer.is_open() {
        return Err(ContractError::NoValidOffer {
            reason: "Offer has been revoked, expired, or rejected".to_string(),
        });
    }

    // Check for invitation to treat
    if matches!(offer.offer_type, OfferType::InvitationToTreat) {
        return Err(ContractError::NoValidOffer {
            reason: "This is an invitation to treat, not an offer. \
                     See: Pharmaceutical Society v Boots [1953] - \
                     goods displayed in shop are invitation to treat."
                .to_string(),
        });
    }

    // Check terms are present and definite
    if offer.terms.is_empty() {
        return Err(ContractError::NoValidOffer {
            reason: "Offer must have definite and certain terms".to_string(),
        });
    }

    Ok(())
}

/// Validate acceptance (mirror image rule)
///
/// Common law rule: Acceptance must be unqualified and match offer exactly.
///
/// Case law:
/// - Hyde v Wrench [1840]: Counter-offer destroys original offer
/// - Adams v Lindsell [1818]: Postal rule - acceptance complete when posted
pub fn validate_acceptance(acceptance: &Acceptance, offer: &Offer) -> Result<()> {
    // Mirror image rule: acceptance must be unqualified
    if !acceptance.is_valid_acceptance() {
        let modifications = acceptance.modifications.join("; ");
        let offer_terms = offer.terms.join("; ");

        return Err(ContractError::MirrorImageRuleViolated {
            offer_terms,
            modifications,
        });
    }

    // Note: Postal rule applies if acceptance by post
    // Acceptance is complete when posted, even if lost in post
    // Adams v Lindsell [1818]

    Ok(())
}

/// Validate consideration
///
/// Requirements:
/// - Must be sufficient (but need not be adequate) - Chappell v Nestlé [1960]
/// - Must not be past - Re McArdle [1951]
/// - Must move from promisee - Tweddle v Atkinson [1861]
///
/// Exception: Practical benefit may suffice - Williams v Roffey Bros [1991]
pub fn validate_consideration(consideration: &Consideration) -> Result<()> {
    // Check for past consideration
    if consideration.is_past {
        return Err(ContractError::PastConsideration {
            past_act: consideration.description.clone(),
            later_promise: "Promise made after act already performed".to_string(),
        });
    }

    // Check sufficiency
    if !consideration.sufficient {
        return Err(ContractError::NoConsideration {
            reason: format!(
                "Consideration '{}' is not sufficient. \
                 Consideration must be sufficient but need not be adequate \
                 (Chappell v Nestlé [1960]).",
                consideration.description
            ),
        });
    }

    Ok(())
}

/// Validate intention to create legal relations
///
/// Presumptions:
/// - Commercial context: Intention presumed (Esso v Commissioners [1976])
/// - Domestic/social context: No intention presumed (Balfour v Balfour [1919])
///
/// Presumptions are rebuttable with evidence
pub fn validate_intention(intention: &IntentionToCreateLegalRelations) -> Result<()> {
    if !intention.intention_exists {
        let context = match intention.context {
            AgreementContext::Commercial => "Commercial",
            AgreementContext::Domestic => "Domestic",
            AgreementContext::Social => "Social",
        };

        let presumption = match intention.presumption {
            IntentionPresumption::IntentionPresumed => {
                "Intention normally presumed in commercial context"
            }
            IntentionPresumption::NoIntentionPresumed => {
                "No intention presumed in domestic/social context (Balfour v Balfour [1919])"
            }
        };

        let rebuttal = if intention.rebuttal_evidence.is_empty() {
            "None".to_string()
        } else {
            intention.rebuttal_evidence.join("; ")
        };

        return Err(ContractError::NoIntention {
            context: context.to_string(),
            presumption: presumption.to_string(),
            rebuttal_evidence: rebuttal,
        });
    }

    Ok(())
}

/// Validate contractual capacity
///
/// Incapacity types:
/// - Minors (under 18): Minors' Contracts Act 1987
/// - Mental incapacity: Mental Capacity Act 2005
/// - Intoxication
/// - Companies acting ultra vires: Companies Act 2006
pub fn validate_capacity(capacity: &ContractualCapacity) -> Result<()> {
    if !capacity.has_capacity {
        let (incapacity_type, consequences) = match capacity.incapacity {
            Some(IncapacityType::Minor) => (
                "Minor (under 18)",
                "Minors' Contracts Act 1987:\n\
                 • Contracts for necessaries are binding\n\
                 • Employment contracts beneficial to minor are binding\n\
                 • Other contracts voidable at minor's option",
            ),
            Some(IncapacityType::MentalIncapacity) => (
                "Mental incapacity",
                "Mental Capacity Act 2005:\n\
                 • Contract voidable if other party knew of incapacity\n\
                 • Contracts for necessaries at fair price are binding",
            ),
            Some(IncapacityType::Intoxication) => (
                "Intoxication",
                "Contract voidable if:\n\
                 • Party so intoxicated they didn't know what they were doing, AND\n\
                 • Other party knew of intoxication",
            ),
            Some(IncapacityType::UltraVires) => (
                "Company acting ultra vires",
                "Companies Act 2006 s.39:\n\
                 • Validity of act not called into question on ultra vires grounds\n\
                 • Directors may be liable for breach of duty",
            ),
            None => ("Unknown", "Capacity issue present"),
        };

        return Err(ContractError::LackOfCapacity {
            party: capacity.party.name.clone(),
            incapacity_type: incapacity_type.to_string(),
            consequences: consequences.to_string(),
        });
    }

    Ok(())
}

/// Validate breach of contract and determine remedy
pub fn validate_breach(breach: &ContractBreach) -> Result<()> {
    match breach.term_breached.classification {
        TermClassification::Condition => Err(ContractError::BreachOfCondition {
            term: breach.term_breached.text.clone(),
            breach_description: breach.description.clone(),
        }),
        TermClassification::Warranty => Err(ContractError::BreachOfWarranty {
            term: breach.term_breached.text.clone(),
            breach_description: breach.description.clone(),
        }),
        TermClassification::InnominateTerm => {
            // Depends on consequences - treat as condition for serious breach
            if matches!(breach.breach_type, BreachType::Fundamental) {
                Err(ContractError::BreachOfCondition {
                    term: breach.term_breached.text.clone(),
                    breach_description: format!(
                        "{} (Innominate term - serious consequences)",
                        breach.description
                    ),
                })
            } else {
                Err(ContractError::BreachOfWarranty {
                    term: breach.term_breached.text.clone(),
                    breach_description: format!(
                        "{} (Innominate term - minor consequences)",
                        breach.description
                    ),
                })
            }
        }
    }
}

/// Validate remoteness of damages (Hadley v Baxendale test)
///
/// Case law: Hadley v Baxendale [1854]
/// Two limbs:
/// 1. Loss arising naturally in ordinary course
/// 2. Loss reasonably in contemplation of both parties
pub fn validate_remoteness(test: &RemotenessTest) -> Result<()> {
    if !test.passes_test {
        return Err(ContractError::LossTooRemote {
            loss_description: test.loss_description.clone(),
            loss_amount: test.loss_amount,
            reason: match test.limb {
                HadleyLimb::FirstLimb => {
                    "Loss does not arise naturally in ordinary course of things"
                }
                HadleyLimb::SecondLimb => {
                    "Special circumstances not communicated; \
                     loss not reasonably in contemplation of both parties"
                }
            }
            .to_string(),
            recoverable: 0.0,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_offer_invitation_to_treat() {
        let offer = Offer {
            offeror: Party {
                name: "Shop".to_string(),
                party_type: PartyType::Company,
                age: None,
            },
            offeree: Party {
                name: "Customer".to_string(),
                party_type: PartyType::Individual,
                age: Some(30),
            },
            terms: vec!["Item on shelf for £10".to_string()],
            offer_date: Utc::now(),
            expiry_date: None,
            still_open: true,
            offer_type: OfferType::InvitationToTreat,
        };

        let result = validate_offer(&offer);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContractError::NoValidOffer { .. }
        ));
    }

    #[test]
    fn test_validate_acceptance_mirror_image_violated() {
        let offer = Offer {
            offeror: Party {
                name: "Seller".to_string(),
                party_type: PartyType::Individual,
                age: Some(40),
            },
            offeree: Party {
                name: "Buyer".to_string(),
                party_type: PartyType::Individual,
                age: Some(30),
            },
            terms: vec!["Sell car for £5000".to_string()],
            offer_date: Utc::now(),
            expiry_date: None,
            still_open: true,
            offer_type: OfferType::Bilateral,
        };

        let acceptance = Acceptance {
            acceptance_date: Utc::now(),
            method: AcceptanceMethod::Written,
            unqualified: false,
            modifications: vec!["I'll pay £4500".to_string()],
        };

        let result = validate_acceptance(&acceptance, &offer);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContractError::MirrorImageRuleViolated { .. }
        ));
    }

    #[test]
    fn test_validate_past_consideration() {
        let consideration = Consideration {
            description: "Work completed last month".to_string(),
            provided_by: Party {
                name: "Worker".to_string(),
                party_type: PartyType::Individual,
                age: Some(25),
            },
            consideration_type: ConsiderationType::Act,
            sufficient: true,
            is_past: true,
        };

        let result = validate_consideration(&consideration);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContractError::PastConsideration { .. }
        ));
    }

    #[test]
    fn test_validate_intention_domestic_context() {
        let intention = IntentionToCreateLegalRelations {
            context: AgreementContext::Domestic,
            presumption: IntentionPresumption::NoIntentionPresumed,
            rebuttal_evidence: vec![],
            intention_exists: false,
        };

        let result = validate_intention(&intention);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ContractError::NoIntention { .. }
        ));
    }
}
