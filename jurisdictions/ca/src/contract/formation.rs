//! Canada Contract Law - Formation Analysis
//!
//! Analyzers for contract formation under common law and Quebec civil law.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    Acceptance, CommunicationMethod, Consideration, ContractCase, FormationElement, Offer,
};
use crate::common::Province;

// ============================================================================
// Formation Analysis
// ============================================================================

/// Facts for analyzing contract formation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationFacts {
    /// Province where contract formed
    pub province: Province,
    /// The offer
    pub offer: Offer,
    /// The acceptance
    pub acceptance: Option<Acceptance>,
    /// Consideration (common law provinces)
    pub consideration: Option<Consideration>,
    /// Whether parties intended legal relations
    pub intention_evidence: IntentionEvidence,
    /// Whether parties have capacity
    pub capacity: CapacityStatus,
    /// Whether contract is legal
    pub legality: LegalityStatus,
    /// Whether terms are sufficiently certain
    pub certainty: bool,
}

/// Evidence of intention to create legal relations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentionEvidence {
    /// Commercial or social/domestic context
    pub context: ContractContext,
    /// Express statements about legal binding
    pub express_statements: Vec<String>,
    /// Objective indicators of intention
    pub objective_indicators: Vec<String>,
}

/// Context of the agreement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractContext {
    /// Commercial/business
    Commercial,
    /// Social or domestic
    SocialDomestic,
    /// Employment
    Employment,
    /// Government/public
    Government,
}

/// Capacity to contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityStatus {
    /// Whether all parties have capacity
    pub has_capacity: bool,
    /// Any capacity issues
    pub issues: Vec<CapacityIssue>,
}

/// Capacity issue
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityIssue {
    /// Minor (under 18/19 depending on province)
    Minor { age: u32, province: Province },
    /// Mental incapacity
    MentalIncapacity,
    /// Intoxication
    Intoxication,
    /// Corporation acting outside powers
    UltraVires,
}

/// Legality status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalityStatus {
    /// Whether purpose is legal
    pub is_legal: bool,
    /// Any illegality issues
    pub issues: Vec<String>,
}

/// Result of formation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationResult {
    /// Whether valid contract formed
    pub contract_formed: bool,
    /// Elements satisfied
    pub elements_satisfied: Vec<FormationElement>,
    /// Elements not satisfied
    pub elements_missing: Vec<FormationElement>,
    /// Whether Quebec civil law applies
    pub quebec_law_applies: bool,
    /// Key issues identified
    pub issues: Vec<String>,
    /// Key cases
    pub key_cases: Vec<ContractCase>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Formation Analyzer
// ============================================================================

/// Analyzer for contract formation
pub struct FormationAnalyzer;

impl FormationAnalyzer {
    /// Analyze whether a valid contract was formed
    pub fn analyze(facts: &FormationFacts) -> FormationResult {
        let quebec_law = facts.province.is_civil_law();
        let mut elements_satisfied = Vec::new();
        let mut elements_missing = Vec::new();
        let mut issues = Vec::new();
        let mut key_cases = Vec::new();

        // 1. Offer
        let offer_valid = Self::analyze_offer(&facts.offer);
        if offer_valid {
            elements_satisfied.push(FormationElement::Offer);
        } else {
            elements_missing.push(FormationElement::Offer);
            issues.push("Offer not valid or has lapsed/been revoked".to_string());
        }

        // 2. Acceptance
        let acceptance_valid = if let Some(acceptance) = &facts.acceptance {
            Self::analyze_acceptance(acceptance, &facts.offer)
        } else {
            false
        };
        if acceptance_valid {
            elements_satisfied.push(FormationElement::Acceptance);
        } else {
            elements_missing.push(FormationElement::Acceptance);
            issues.push("No valid acceptance of offer".to_string());
        }

        // 3. Consideration (common law) or Consent (Quebec)
        if quebec_law {
            // Quebec: consent is key, no consideration required
            elements_satisfied.push(FormationElement::Consent);
        } else {
            // Common law: consideration required
            let consideration_valid = if let Some(consideration) = &facts.consideration {
                Self::analyze_consideration(consideration)
            } else {
                false
            };
            if consideration_valid {
                elements_satisfied.push(FormationElement::Consideration);
            } else {
                elements_missing.push(FormationElement::Consideration);
                issues.push("No valid consideration provided".to_string());
            }
        }

        // 4. Intention
        let intention_valid = Self::analyze_intention(&facts.intention_evidence);
        if intention_valid {
            elements_satisfied.push(FormationElement::Intention);
        } else {
            elements_missing.push(FormationElement::Intention);
            issues.push("No intention to create legal relations".to_string());
        }

        // 5. Capacity
        if facts.capacity.has_capacity {
            elements_satisfied.push(FormationElement::Capacity);
        } else {
            elements_missing.push(FormationElement::Capacity);
            for issue in &facts.capacity.issues {
                issues.push(format!("Capacity issue: {:?}", issue));
            }
        }

        // 6. Legality
        if facts.legality.is_legal {
            elements_satisfied.push(FormationElement::Legality);
        } else {
            elements_missing.push(FormationElement::Legality);
            issues.extend(facts.legality.issues.clone());
        }

        // 7. Certainty
        if facts.certainty {
            elements_satisfied.push(FormationElement::Certainty);
        } else {
            elements_missing.push(FormationElement::Certainty);
            issues.push("Terms are uncertain or incomplete".to_string());
        }

        // Good faith (Bhasin v Hrynew)
        key_cases.push(ContractCase::bhasin());

        // Determine if contract formed
        let contract_formed = elements_missing.is_empty();

        let reasoning = Self::build_reasoning(
            contract_formed,
            &elements_satisfied,
            &elements_missing,
            quebec_law,
        );

        FormationResult {
            contract_formed,
            elements_satisfied,
            elements_missing,
            quebec_law_applies: quebec_law,
            issues,
            key_cases,
            reasoning,
        }
    }

    /// Analyze validity of offer
    fn analyze_offer(offer: &Offer) -> bool {
        offer.is_definite && !offer.lapsed && !offer.revoked
    }

    /// Analyze validity of acceptance
    fn analyze_acceptance(acceptance: &Acceptance, offer: &Offer) -> bool {
        // Mirror image rule
        if !acceptance.mirrors_offer {
            return false;
        }

        // Communication required (unless postal rule applies)
        if !acceptance.communicated {
            // Postal rule: acceptance effective on posting
            if acceptance.method != CommunicationMethod::Written {
                return false;
            }
        }

        // Prescribed method (if any)
        if offer.communication != acceptance.method && !acceptance.prescribed_method_followed {
            // Acceptance must use prescribed method if specified
            // But this is not always fatal
        }

        true
    }

    /// Analyze validity of consideration
    fn analyze_consideration(consideration: &Consideration) -> bool {
        // Consideration must be sufficient (need not be adequate)
        if !consideration.is_sufficient {
            return false;
        }

        // Consideration must not be past
        if consideration.is_past {
            return false;
        }

        // Consideration must move from promisee
        if !consideration.moves_from_promisee {
            return false;
        }

        true
    }

    /// Analyze intention to create legal relations
    fn analyze_intention(evidence: &IntentionEvidence) -> bool {
        match evidence.context {
            ContractContext::Commercial | ContractContext::Employment => {
                // Presumption of intention in commercial context
                // Rebutted only by clear contrary evidence
                true
            }
            ContractContext::SocialDomestic => {
                // Presumption against intention in domestic context
                // Must be rebutted by evidence
                !evidence.express_statements.is_empty() || !evidence.objective_indicators.is_empty()
            }
            ContractContext::Government => {
                // Depends on nature of agreement
                !evidence.express_statements.is_empty()
            }
        }
    }

    /// Build reasoning
    fn build_reasoning(
        formed: bool,
        satisfied: &[FormationElement],
        missing: &[FormationElement],
        quebec: bool,
    ) -> String {
        let jurisdiction = if quebec {
            "Under Quebec civil law (CCQ art. 1385)"
        } else {
            "Under common law"
        };

        if formed {
            format!(
                "{}, a valid contract has been formed. All required elements are satisfied: {:?}.",
                jurisdiction, satisfied
            )
        } else {
            format!(
                "{}, no valid contract has been formed. Missing elements: {:?}. \
                 Elements satisfied: {:?}.",
                jurisdiction, missing, satisfied
            )
        }
    }
}

// ============================================================================
// Offer Analyzer
// ============================================================================

/// Dedicated analyzer for offers
pub struct OfferAnalyzer;

impl OfferAnalyzer {
    /// Analyze whether communication is an offer or invitation to treat
    pub fn is_offer(facts: &OfferClassificationFacts) -> OfferClassificationResult {
        let is_offer = match facts.context {
            OfferContext::ShopDisplay => {
                // Pharmaceutical Society v Boots - display is invitation to treat
                false
            }
            OfferContext::Advertisement => {
                // Generally invitation to treat unless unilateral offer
                facts.is_unilateral
            }
            OfferContext::Auction => {
                // Auctioneer's call is invitation; bid is offer
                facts.is_bid
            }
            OfferContext::Tender => {
                // Invitation to tender is invitation; tender is offer
                // But process contract may form
                facts.is_tender_submission
            }
            OfferContext::DirectNegotiation => {
                // More likely to be offer if definite
                facts.is_definite
            }
        };

        OfferClassificationResult {
            is_offer,
            is_invitation_to_treat: !is_offer,
            reasoning: if is_offer {
                "Communication constitutes a valid offer".to_string()
            } else {
                "Communication is an invitation to treat, not an offer".to_string()
            },
        }
    }
}

/// Facts for classifying offer vs invitation to treat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferClassificationFacts {
    /// Context of communication
    pub context: OfferContext,
    /// Whether terms are definite
    pub is_definite: bool,
    /// Whether it's a unilateral offer
    pub is_unilateral: bool,
    /// For auctions - whether this is a bid
    pub is_bid: bool,
    /// For tenders - whether this is a tender submission
    pub is_tender_submission: bool,
}

/// Context for offer classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferContext {
    /// Goods displayed in shop
    ShopDisplay,
    /// Advertisement
    Advertisement,
    /// Auction
    Auction,
    /// Tender process
    Tender,
    /// Direct negotiation
    DirectNegotiation,
}

/// Result of offer classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferClassificationResult {
    /// Whether this is an offer
    pub is_offer: bool,
    /// Whether this is an invitation to treat
    pub is_invitation_to_treat: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_offer() -> Offer {
        Offer {
            description: "Sale of car".to_string(),
            offeror: "Seller".to_string(),
            offeree: "Buyer".to_string(),
            terms: vec!["Price: $10,000".to_string()],
            is_definite: true,
            communication: CommunicationMethod::Email,
            time_limit: None,
            lapsed: false,
            revoked: false,
        }
    }

    fn sample_acceptance() -> Acceptance {
        Acceptance {
            description: "Buyer accepts".to_string(),
            mirrors_offer: true,
            method: CommunicationMethod::Email,
            communicated: true,
            prescribed_method_followed: true,
        }
    }

    fn sample_consideration() -> Consideration {
        Consideration {
            benefit_to_promisor: "$10,000 payment".to_string(),
            detriment_to_promisee: "Payment of $10,000".to_string(),
            is_sufficient: true,
            is_past: false,
            moves_from_promisee: true,
        }
    }

    #[test]
    fn test_valid_common_law_formation() {
        let facts = FormationFacts {
            province: Province::Ontario,
            offer: sample_offer(),
            acceptance: Some(sample_acceptance()),
            consideration: Some(sample_consideration()),
            intention_evidence: IntentionEvidence {
                context: ContractContext::Commercial,
                express_statements: vec![],
                objective_indicators: vec!["Signed contract".to_string()],
            },
            capacity: CapacityStatus {
                has_capacity: true,
                issues: vec![],
            },
            legality: LegalityStatus {
                is_legal: true,
                issues: vec![],
            },
            certainty: true,
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(result.contract_formed);
        assert!(!result.quebec_law_applies);
    }

    #[test]
    fn test_quebec_no_consideration() {
        let facts = FormationFacts {
            province: Province::Quebec,
            offer: sample_offer(),
            acceptance: Some(sample_acceptance()),
            consideration: None, // No consideration in Quebec
            intention_evidence: IntentionEvidence {
                context: ContractContext::Commercial,
                express_statements: vec![],
                objective_indicators: vec![],
            },
            capacity: CapacityStatus {
                has_capacity: true,
                issues: vec![],
            },
            legality: LegalityStatus {
                is_legal: true,
                issues: vec![],
            },
            certainty: true,
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(result.contract_formed);
        assert!(result.quebec_law_applies);
    }

    #[test]
    fn test_missing_consideration() {
        let facts = FormationFacts {
            province: Province::BritishColumbia,
            offer: sample_offer(),
            acceptance: Some(sample_acceptance()),
            consideration: None,
            intention_evidence: IntentionEvidence {
                context: ContractContext::Commercial,
                express_statements: vec![],
                objective_indicators: vec![],
            },
            capacity: CapacityStatus {
                has_capacity: true,
                issues: vec![],
            },
            legality: LegalityStatus {
                is_legal: true,
                issues: vec![],
            },
            certainty: true,
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(!result.contract_formed);
        assert!(
            result
                .elements_missing
                .contains(&FormationElement::Consideration)
        );
    }

    #[test]
    fn test_offer_classification_shop_display() {
        let facts = OfferClassificationFacts {
            context: OfferContext::ShopDisplay,
            is_definite: true,
            is_unilateral: false,
            is_bid: false,
            is_tender_submission: false,
        };

        let result = OfferAnalyzer::is_offer(&facts);
        assert!(!result.is_offer);
        assert!(result.is_invitation_to_treat);
    }
}
