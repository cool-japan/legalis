//! Contract Law - Validation and Analysis Logic
//!
//! Functions that apply the Singapore common law of contract to the typed models
//! in [`super::types`] and [`super::remedies`]:
//!
//! - [`validate_formation`] — tests the four formation requirements.
//! - [`classify_breach`] — determines the consequence of breach of a given term.
//! - [`assess_misrepresentation`], [`assess_mistake`], [`assess_duress`],
//!   [`assess_undue_influence`] — vitiating factors.
//! - [`assess_frustration`] — discharge by frustration.
//! - [`assess_damages`] — applies *Hadley v Baxendale* remoteness and the
//!   mitigation principle to the claimed heads of loss.
//! - [`assess_specific_performance`] — equitable discretionary relief.

use super::error::{ContractError, Result};
use super::remedies::{DamagesAward, DamagesMeasure, HeadOfLoss, SpecificPerformanceFactors};
use super::types::*;
use serde::{Deserialize, Serialize};

/// The outcome of breaching a particular term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachConsequence {
    /// Whether the innocent party may terminate (treat the contract as
    /// discharged) in addition to claiming damages.
    pub may_terminate: bool,
    /// Whether damages are available (always true for an established breach).
    pub damages_available: bool,
    /// Short explanation tied to the classification.
    pub rationale: String,
}

/// Validates the formation of a contract against the four requirements stated in
/// *Gay Choon Ing v Loh Sze Ti Terence Peter* \[2009\] SGCA 3: offer,
/// acceptance, consideration, and intention to create legal relations.
///
/// # Errors
///
/// Returns the specific [`ContractError`] for the first failed requirement.
pub fn validate_formation(contract: &Contract) -> Result<()> {
    // 1. There must be an acceptance.
    let acceptance = contract
        .acceptance
        .as_ref()
        .ok_or_else(|| ContractError::NoAgreement {
            reason: "no acceptance has been communicated".to_string(),
        })?;

    // The acceptance must relate to the offer.
    if acceptance.offer_id != contract.offer.id {
        return Err(ContractError::NoAgreement {
            reason: "acceptance does not correspond to the offer".to_string(),
        });
    }

    // 2. The acceptance must be unqualified (mirror-image rule); otherwise it is
    //    a counter-offer.
    if !acceptance.unqualified {
        return Err(ContractError::CounterOffer {
            detail: "acceptance introduced new or varied terms".to_string(),
        });
    }

    // 3. The offer must have been open when acceptance took effect.
    if !contract.offer.is_open() {
        return Err(ContractError::OfferNotOpen {
            reason: format!("offer status was {:?}", contract.offer.status),
        });
    }

    // 4. There must be good consideration moving from the promisee.
    if contract.considerations.is_empty() {
        return Err(ContractError::NoConsideration {
            reason: "no consideration recorded".to_string(),
        });
    }
    for consideration in &contract.considerations {
        if !consideration.moves_from_promisee {
            return Err(ContractError::NoConsideration {
                reason: format!(
                    "consideration from {} does not move from the promisee",
                    consideration.provider
                ),
            });
        }
        match consideration.kind {
            ConsiderationKind::Past => {
                return Err(ContractError::NoConsideration {
                    reason: format!("past consideration: {}", consideration.description),
                });
            }
            ConsiderationKind::ExistingDuty if !consideration.confers_practical_benefit => {
                return Err(ContractError::ExistingDutyConsideration {
                    reason: consideration.description.clone(),
                });
            }
            _ => {}
        }
    }

    // 5. The parties must intend to create legal relations.
    if !contract.intends_legal_relations() {
        return Err(ContractError::NoIntentionToCreateLegalRelations {
            reason: match contract.context {
                AgreementContext::SocialDomestic => {
                    "social/domestic agreement, presumption not rebutted".to_string()
                }
                AgreementContext::Commercial => {
                    "commercial presumption rebutted (e.g. honour clause)".to_string()
                }
            },
        });
    }

    Ok(())
}

/// Returns whether a contract is well-formed (does not surface the error).
pub fn is_formed(contract: &Contract) -> bool {
    validate_formation(contract).is_ok()
}

/// Determines the consequence of a breach of the given term, having regard to
/// its classification and (for innominate terms) the gravity of the breach.
///
/// For an innominate term the *Hongkong Fir* test asks whether the breach
/// deprives the innocent party of substantially the whole benefit of the
/// contract; `deprives_substantial_benefit` carries that finding.
pub fn classify_breach(
    term: &ContractTerm,
    deprives_substantial_benefit: bool,
) -> BreachConsequence {
    match term.classification {
        TermClassification::Condition => BreachConsequence {
            may_terminate: true,
            damages_available: true,
            rationale: format!(
                "breach of a condition — any breach permits termination ({})",
                term.classification.authority()
            ),
        },
        TermClassification::Warranty => BreachConsequence {
            may_terminate: false,
            damages_available: true,
            rationale: format!(
                "breach of a warranty — damages only ({})",
                term.classification.authority()
            ),
        },
        TermClassification::Innominate => BreachConsequence {
            may_terminate: deprives_substantial_benefit,
            damages_available: true,
            rationale: if deprives_substantial_benefit {
                format!(
                    "innominate term breached so as to deprive the innocent party of \
                     substantially the whole benefit — termination available ({})",
                    term.classification.authority()
                )
            } else {
                format!(
                    "innominate term breached without depriving the innocent party of \
                     substantially the whole benefit — damages only ({})",
                    term.classification.authority()
                )
            },
        },
    }
}

/// Asserts that a breach is repudiatory (gives a right to terminate), returning
/// an error describing the consequence so callers in a `?`-chain can branch.
///
/// # Errors
///
/// Returns [`ContractError::RepudiatoryBreach`] when the breach permits
/// termination, or [`ContractError::WarrantyBreach`] when it sounds in damages
/// only.
pub fn require_termination_right(
    term: &ContractTerm,
    deprives_substantial_benefit: bool,
) -> Result<()> {
    let consequence = classify_breach(term, deprives_substantial_benefit);
    if consequence.may_terminate {
        Err(ContractError::RepudiatoryBreach {
            detail: consequence.rationale,
        })
    } else {
        Err(ContractError::WarrantyBreach {
            detail: consequence.rationale,
        })
    }
}

/// Assesses a misrepresentation, returning the category-specific error when the
/// representation is actionable.
///
/// # Errors
///
/// Returns [`ContractError::Misrepresentation`] when the statement is a false
/// statement of fact that induced the contract.
pub fn assess_misrepresentation(misrep: &Misrepresentation) -> Result<()> {
    if misrep.is_actionable() {
        let category = match misrep.category {
            MisrepresentationCategory::Fraudulent => "fraudulent",
            MisrepresentationCategory::Negligent => "negligent",
            MisrepresentationCategory::Innocent => "innocent",
        };
        return Err(ContractError::Misrepresentation {
            category: category.to_string(),
            statement: misrep.statement.clone(),
            authority: misrep.category.authority().to_string(),
        });
    }
    Ok(())
}

/// Assesses an operative mistake.
///
/// # Errors
///
/// Returns [`ContractError::Mistake`] where the mistake is operative: a
/// fundamental common/mutual mistake, or a unilateral mistake of which the other
/// party had knowledge.
pub fn assess_mistake(mistake: &OperativeMistake) -> Result<()> {
    let operative = match mistake.kind {
        MistakeKind::Common | MistakeKind::Mutual => mistake.fundamental,
        MistakeKind::Unilateral => mistake.other_party_knew,
    };
    if operative {
        let kind = match mistake.kind {
            MistakeKind::Common => "common",
            MistakeKind::Mutual => "mutual",
            MistakeKind::Unilateral => "unilateral",
        };
        return Err(ContractError::Mistake {
            kind: kind.to_string(),
            detail: mistake.detail.clone(),
            authority: mistake.kind.authority().to_string(),
        });
    }
    Ok(())
}

/// Assesses a duress claim.
///
/// # Errors
///
/// Returns [`ContractError::Duress`] when duress is established.
pub fn assess_duress(claim: &DuressClaim) -> Result<()> {
    if claim.is_established() {
        let kind = match claim.kind {
            DuressKind::ToThePerson => "physical",
            DuressKind::ToGoods => "to-goods",
            DuressKind::Economic => "economic",
        };
        return Err(ContractError::Duress {
            kind: kind.to_string(),
            detail: claim.detail.clone(),
        });
    }
    Ok(())
}

/// Assesses an undue-influence claim.
///
/// # Errors
///
/// Returns [`ContractError::UndueInfluence`] when undue influence is established.
pub fn assess_undue_influence(claim: &UndueInfluenceClaim) -> Result<()> {
    if claim.is_established() {
        let kind = match claim.class {
            UndueInfluenceClass::Actual => "actual",
            UndueInfluenceClass::PresumedRecognised | UndueInfluenceClass::PresumedProved => {
                "presumed"
            }
        };
        return Err(ContractError::UndueInfluence {
            kind: kind.to_string(),
            detail: claim.detail.clone(),
        });
    }
    Ok(())
}

/// Assesses whether a supervening event frustrates the contract.
///
/// # Errors
///
/// Returns [`ContractError::FrustrationNotEstablished`] with the disqualifying
/// reason when frustration fails.
pub fn assess_frustration(event: &FrustratingEvent) -> Result<DischargeMode> {
    if event.frustrates() {
        return Ok(DischargeMode::Frustration);
    }
    let reason = if !event.radically_different {
        "performance is not impossible or radically different (mere hardship is insufficient — Davis Contractors v Fareham UDC [1956] AC 696)"
    } else if event.self_induced {
        "the event was self-induced (Maritime National Fish v Ocean Trawlers [1935] AC 524)"
    } else if event.risk_allocated_by_term {
        "the risk was allocated by an express term (e.g. force majeure)"
    } else {
        "the event was foreseeable / foreseen"
    };
    Err(ContractError::FrustrationNotEstablished {
        reason: reason.to_string(),
    })
}

/// Assesses damages for breach: aggregates the claimed heads, applies the
/// *Hadley v Baxendale* remoteness rule and the mitigation principle, and
/// returns the recoverable total.
///
/// # Errors
///
/// Returns [`ContractError::InvalidAmount`] if any head carries a negative
/// amount.
pub fn assess_damages(measure: DamagesMeasure, heads: &[HeadOfLoss]) -> Result<DamagesAward> {
    let mut claimed = 0i64;
    let mut recoverable = 0i64;
    let mut remote_heads = Vec::new();
    let mut unmitigated_heads = Vec::new();

    for head in heads {
        if head.amount_cents < 0 {
            return Err(ContractError::InvalidAmount {
                detail: format!("head '{}' has a negative amount", head.description),
            });
        }
        claimed = claimed.saturating_add(head.amount_cents);

        if head.avoidable_by_mitigation {
            unmitigated_heads.push(head.description.clone());
            continue;
        }
        if head.is_recoverable() {
            recoverable = recoverable.saturating_add(head.amount_cents);
        } else {
            remote_heads.push(head.description.clone());
        }
    }

    Ok(DamagesAward {
        measure,
        heads: heads.to_vec(),
        claimed_cents: claimed,
        recoverable_cents: recoverable,
        remote_heads,
        unmitigated_heads,
    })
}

/// Assesses whether specific performance is available on the given factors.
///
/// # Errors
///
/// Returns [`ContractError::SpecificPerformanceUnavailable`] with the
/// disqualifying reason where the equitable remedy is not available.
pub fn assess_specific_performance(factors: &SpecificPerformanceFactors) -> Result<()> {
    if factors.is_available() {
        return Ok(());
    }
    let reason = if factors.damages_adequate {
        "damages are an adequate remedy"
    } else if factors.personal_service {
        "the contract is one of personal service"
    } else {
        "performance would require constant supervision (Co-operative Insurance v Argyll Stores [1998] AC 1)"
    };
    Err(ContractError::SpecificPerformanceUnavailable {
        reason: reason.to_string(),
    })
}

/// A consolidated report on a contract: formation status and any vitiating
/// factors found.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractValidationReport {
    /// Identifier of the contract analysed.
    pub contract_id: String,
    /// Whether the contract is well-formed.
    pub formed: bool,
    /// The formation error, if any (as a display string).
    pub formation_issue: Option<String>,
    /// Vitiating factors found (as display strings).
    pub vitiating_factors: Vec<String>,
}

impl ContractValidationReport {
    /// Returns whether the contract is, on this analysis, valid and enforceable
    /// (well-formed and free of vitiating factors).
    pub fn is_enforceable(&self) -> bool {
        self.formed && self.vitiating_factors.is_empty()
    }
}

/// Builds a [`ContractValidationReport`] from a contract and optional vitiating
/// factors.
pub fn analyse_contract(
    contract: &Contract,
    misrep: Option<&Misrepresentation>,
    mistake: Option<&OperativeMistake>,
    duress: Option<&DuressClaim>,
    undue_influence: Option<&UndueInfluenceClaim>,
) -> ContractValidationReport {
    let formation_issue = validate_formation(contract).err().map(|e| e.to_string());
    let formed = formation_issue.is_none();

    let mut vitiating_factors = Vec::new();
    if let Some(m) = misrep
        && let Err(e) = assess_misrepresentation(m)
    {
        vitiating_factors.push(e.to_string());
    }
    if let Some(m) = mistake
        && let Err(e) = assess_mistake(m)
    {
        vitiating_factors.push(e.to_string());
    }
    if let Some(d) = duress
        && let Err(e) = assess_duress(d)
    {
        vitiating_factors.push(e.to_string());
    }
    if let Some(u) = undue_influence
        && let Err(e) = assess_undue_influence(u)
    {
        vitiating_factors.push(e.to_string());
    }

    ContractValidationReport {
        contract_id: contract.id.clone(),
        formed,
        formation_issue,
        vitiating_factors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_contract() -> Contract {
        let offer = Offer::new("o1", "Seller", "Buyer", "sale of machine");
        let mut k = Contract::new("k1", offer, AgreementContext::Commercial)
            .with_acceptance(Acceptance::new("o1", "Buyer", AcceptanceMode::Electronic));
        k.add_consideration(Consideration::promise("Seller", "deliver machine"));
        k.add_consideration(Consideration::promise("Buyer", "pay SGD 50,000"));
        k
    }

    #[test]
    fn well_formed_contract_passes() {
        assert!(is_formed(&valid_contract()));
    }

    #[test]
    fn missing_acceptance_is_no_agreement() {
        let offer = Offer::new("o1", "S", "B", "x");
        let mut k = Contract::new("k1", offer, AgreementContext::Commercial);
        k.add_consideration(Consideration::promise("S", "do x"));
        match validate_formation(&k) {
            Err(ContractError::NoAgreement { .. }) => {}
            other => panic!("expected NoAgreement, got {other:?}"),
        }
    }

    #[test]
    fn qualified_acceptance_is_counter_offer() {
        let offer = Offer::new("o1", "S", "B", "x");
        let mut k = Contract::new("k1", offer, AgreementContext::Commercial)
            .with_acceptance(Acceptance::new("o1", "B", AcceptanceMode::Postal).qualified());
        k.add_consideration(Consideration::promise("S", "do x"));
        match validate_formation(&k) {
            Err(ContractError::CounterOffer { .. }) => {}
            other => panic!("expected CounterOffer, got {other:?}"),
        }
    }

    #[test]
    fn social_agreement_without_intention_fails() {
        let offer = Offer::new("o1", "H", "W", "housekeeping allowance");
        let mut k = Contract::new("k1", offer, AgreementContext::SocialDomestic)
            .with_acceptance(Acceptance::new("o1", "W", AcceptanceMode::Instantaneous));
        k.add_consideration(Consideration::promise("W", "keep house"));
        match validate_formation(&k) {
            Err(ContractError::NoIntentionToCreateLegalRelations { .. }) => {}
            other => panic!("expected no-intention error, got {other:?}"),
        }
    }

    #[test]
    fn condition_breach_permits_termination() {
        let term = ContractTerm::new("t", "deliver on time", TermClassification::Condition);
        let c = classify_breach(&term, false);
        assert!(c.may_terminate);
    }

    #[test]
    fn innominate_term_depends_on_gravity() {
        let term = ContractTerm::new("t", "seaworthiness", TermClassification::Innominate);
        assert!(!classify_breach(&term, false).may_terminate);
        assert!(classify_breach(&term, true).may_terminate);
    }

    #[test]
    fn negligent_misrep_is_actionable_under_s2_1() {
        let m = Misrepresentation::new(
            "the car had one previous owner",
            MisrepresentationCategory::Negligent,
        );
        match assess_misrepresentation(&m) {
            Err(ContractError::Misrepresentation { authority, .. }) => {
                assert_eq!(authority, "Misrepresentation Act 1967 s. 2(1)");
            }
            other => panic!("expected misrep, got {other:?}"),
        }
    }

    #[test]
    fn frustration_succeeds_then_fails_when_self_induced() {
        let event = FrustratingEvent::new("government requisition of the vessel");
        assert_eq!(
            assess_frustration(&event).expect("frustrates"),
            DischargeMode::Frustration
        );

        let bad = event.self_induced();
        assert!(assess_frustration(&bad).is_err());
    }

    #[test]
    fn damages_apply_remoteness_and_mitigation() {
        let heads = vec![
            HeadOfLoss::ordinary("cost of replacement", 800_000),
            HeadOfLoss::special("lost government tender", 5_000_000, false),
            HeadOfLoss::ordinary("avoidable storage", 200_000).avoidable(),
        ];
        let award = assess_damages(DamagesMeasure::Expectation, &heads).expect("award");
        assert_eq!(award.claimed_cents, 6_000_000);
        // Only the ordinary, mitigated head is recoverable.
        assert_eq!(award.recoverable_cents, 800_000);
        assert_eq!(award.remote_heads.len(), 1);
        assert_eq!(award.unmitigated_heads.len(), 1);
        assert_eq!(award.disallowed_cents(), 5_200_000);
    }

    #[test]
    fn negative_head_is_rejected() {
        let heads = vec![HeadOfLoss::ordinary("bad", -1)];
        assert!(matches!(
            assess_damages(DamagesMeasure::Expectation, &heads),
            Err(ContractError::InvalidAmount { .. })
        ));
    }

    #[test]
    fn specific_performance_for_land_available() {
        let factors = SpecificPerformanceFactors::new().unique_subject_matter();
        assert!(assess_specific_performance(&factors).is_ok());
    }

    #[test]
    fn report_flags_unenforceable_when_vitiated() {
        let k = valid_contract();
        let m = Misrepresentation::new("fake provenance", MisrepresentationCategory::Fraudulent);
        let report = analyse_contract(&k, Some(&m), None, None, None);
        assert!(report.formed);
        assert!(!report.is_enforceable());
        assert_eq!(report.vitiating_factors.len(), 1);
    }
}
