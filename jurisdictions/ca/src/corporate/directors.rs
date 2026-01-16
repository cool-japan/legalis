//! Canada Corporate Law - Director Duties Analysis
//!
//! Analysis of director duties under CBCA s.122 and case law.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    BusinessJudgmentElement, DutyBreach, FiduciaryBreachType, StakeholderInterest, StakeholderType,
};

// ============================================================================
// Director Duty Analysis
// ============================================================================

/// Facts for director duty analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorDutyFacts {
    /// Director name
    pub director: String,
    /// Corporation name
    pub corporation: String,
    /// Decision or conduct under scrutiny
    pub conduct: String,
    /// Context of decision
    pub context: DecisionContext,
    /// Business judgment factors
    pub business_judgment: BusinessJudgmentFactors,
    /// Conflict of interest details
    pub conflict: Option<ConflictDetails>,
    /// Stakeholders affected
    pub stakeholders_affected: Vec<StakeholderImpact>,
}

/// Decision context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    /// Was decision made by board or individual
    pub decision_maker: DecisionMakerType,
    /// Was proper process followed
    pub proper_process: bool,
    /// Was there a special committee
    pub special_committee: bool,
    /// Was independent advice obtained
    pub independent_advice: bool,
    /// Was there disclosure
    pub full_disclosure: bool,
}

/// Decision maker type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionMakerType {
    /// Full board decision
    Board,
    /// Committee decision
    Committee,
    /// Individual director
    Individual,
    /// Officer (not director)
    Officer,
}

/// Business judgment factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessJudgmentFactors {
    /// Was decision informed
    pub informed: InformationLevel,
    /// Was decision made in good faith
    pub good_faith: bool,
    /// Was there a conflict of interest
    pub no_conflict: bool,
    /// Was there a rational business purpose
    pub rational_purpose: bool,
    /// Was decision within range of reasonable alternatives
    pub reasonable_alternative: bool,
}

/// Information level for decision
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InformationLevel {
    /// No investigation
    None,
    /// Minimal inquiry
    Minimal,
    /// Reasonable inquiry
    Reasonable,
    /// Thorough investigation
    Thorough,
}

/// Conflict of interest details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetails {
    /// Nature of conflict
    pub nature: ConflictNature,
    /// Was conflict disclosed (s.120 CBCA)
    pub disclosed: bool,
    /// Did director abstain from voting
    pub abstained: bool,
    /// Was conflict material
    pub material: bool,
}

/// Nature of conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictNature {
    /// Direct financial interest
    DirectFinancial,
    /// Indirect financial interest
    IndirectFinancial,
    /// Family relationship
    Family,
    /// Interest in contracting party
    ContractingParty,
    /// Competing business
    CompetingBusiness,
    /// Corporate opportunity
    CorporateOpportunity,
}

/// Stakeholder impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderImpact {
    /// Stakeholder type
    pub stakeholder: StakeholderType,
    /// Nature of impact
    pub impact: ImpactNature,
    /// Severity of impact
    pub severity: ImpactSeverity,
    /// Were reasonable expectations violated
    pub expectations_violated: bool,
}

/// Nature of impact
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactNature {
    /// Financial loss
    FinancialLoss,
    /// Dilution
    Dilution,
    /// Loss of rights
    LossOfRights,
    /// Loss of opportunity
    LossOfOpportunity,
    /// Employment termination
    EmploymentLoss,
    /// Breach of contract
    ContractBreach,
}

/// Severity of impact
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ImpactSeverity {
    /// Minor impact
    Minor,
    /// Moderate impact
    Moderate,
    /// Significant impact
    Significant,
    /// Severe impact
    Severe,
}

/// Result of director duty analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorDutyResult {
    /// Whether duty of care met
    pub duty_of_care_met: bool,
    /// Whether fiduciary duty met
    pub fiduciary_duty_met: bool,
    /// Business judgment rule applies
    pub business_judgment_applies: bool,
    /// Elements of business judgment satisfied
    pub business_judgment_elements: Vec<BusinessJudgmentElement>,
    /// Potential breaches identified
    pub breaches: Vec<DutyBreach>,
    /// Stakeholder analysis
    pub stakeholder_analysis: StakeholderAnalysis,
    /// Reasoning
    pub reasoning: String,
}

/// Stakeholder analysis (BCE framework)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderAnalysis {
    /// Were all affected stakeholders considered
    pub stakeholders_considered: bool,
    /// Was treatment fair (not necessarily equal)
    pub fair_treatment: bool,
    /// Stakeholder interests balanced
    pub interests_balanced: Vec<StakeholderInterest>,
}

/// Director duty analyzer
pub struct DirectorDutyAnalyzer;

impl DirectorDutyAnalyzer {
    /// Analyze director duties
    pub fn analyze(facts: &DirectorDutyFacts) -> DirectorDutyResult {
        let mut breaches = Vec::new();
        let mut business_judgment_elements = Vec::new();

        // Analyze business judgment rule
        let business_judgment_applies = Self::check_business_judgment(
            &facts.business_judgment,
            &mut business_judgment_elements,
        );

        // Analyze duty of care (Peoples standard)
        let duty_of_care_met = Self::check_duty_of_care(facts, business_judgment_applies);
        if !duty_of_care_met {
            breaches.push(DutyBreach::DutyOfCareBreach);
        }

        // Analyze fiduciary duty
        let fiduciary_duty_met = Self::check_fiduciary_duty(facts, &mut breaches);

        // Analyze stakeholder interests (BCE framework)
        let stakeholder_analysis = Self::analyze_stakeholders(&facts.stakeholders_affected);

        let reasoning = Self::build_reasoning(
            facts,
            duty_of_care_met,
            fiduciary_duty_met,
            business_judgment_applies,
            &stakeholder_analysis,
        );

        DirectorDutyResult {
            duty_of_care_met,
            fiduciary_duty_met,
            business_judgment_applies,
            business_judgment_elements,
            breaches,
            stakeholder_analysis,
            reasoning,
        }
    }

    /// Check business judgment rule
    fn check_business_judgment(
        factors: &BusinessJudgmentFactors,
        elements: &mut Vec<BusinessJudgmentElement>,
    ) -> bool {
        let mut applies = true;

        if factors.good_faith {
            elements.push(BusinessJudgmentElement::GoodFaith);
        } else {
            applies = false;
        }

        if factors.informed >= InformationLevel::Reasonable {
            elements.push(BusinessJudgmentElement::InformedDecision);
        } else {
            applies = false;
        }

        if factors.no_conflict {
            elements.push(BusinessJudgmentElement::NoConflict);
        } else {
            applies = false;
        }

        if factors.rational_purpose {
            elements.push(BusinessJudgmentElement::RationalPurpose);
        } else {
            applies = false;
        }

        if factors.reasonable_alternative {
            elements.push(BusinessJudgmentElement::ReasonableAlternatives);
        }

        applies
    }

    /// Check duty of care (Peoples v Wise standard)
    fn check_duty_of_care(facts: &DirectorDutyFacts, bjr_applies: bool) -> bool {
        // If business judgment rule applies, duty of care presumptively met
        if bjr_applies {
            return true;
        }

        // Otherwise, check objective standard
        // Did director exercise care, diligence, and skill of reasonably prudent person?
        facts.context.proper_process
            && facts.business_judgment.informed >= InformationLevel::Reasonable
    }

    /// Check fiduciary duty (s.122(1)(a) CBCA)
    fn check_fiduciary_duty(facts: &DirectorDutyFacts, breaches: &mut Vec<DutyBreach>) -> bool {
        let mut met = true;

        // Check for conflict of interest
        if let Some(conflict) = &facts.conflict {
            if conflict.material && !conflict.disclosed {
                breaches.push(DutyBreach::FiduciaryBreach(
                    FiduciaryBreachType::UndisclosedConflict,
                ));
                met = false;
            }

            if conflict.nature == ConflictNature::CorporateOpportunity {
                breaches.push(DutyBreach::CorporateOpportunity);
                met = false;
            }

            if conflict.nature == ConflictNature::CompetingBusiness {
                breaches.push(DutyBreach::FiduciaryBreach(
                    FiduciaryBreachType::CompetingBusiness,
                ));
                met = false;
            }

            if conflict.material && conflict.disclosed && !conflict.abstained {
                // Disclosed but didn't abstain - may still be issue
                breaches.push(DutyBreach::ConflictOfInterest);
            }
        }

        // Check good faith
        if !facts.business_judgment.good_faith {
            breaches.push(DutyBreach::FiduciaryBreach(FiduciaryBreachType::BadFaith));
            met = false;
        }

        met
    }

    /// Analyze stakeholder interests (BCE framework)
    fn analyze_stakeholders(impacts: &[StakeholderImpact]) -> StakeholderAnalysis {
        let stakeholders_considered = !impacts.is_empty();

        // Check if any stakeholder had expectations violated
        let expectations_violated = impacts.iter().any(|i| i.expectations_violated);

        // Fair treatment if no severe impacts with violated expectations
        let fair_treatment = !impacts
            .iter()
            .any(|i| i.expectations_violated && i.severity >= ImpactSeverity::Significant);

        let interests_balanced = if fair_treatment {
            vec![
                StakeholderInterest::FairTreatment,
                StakeholderInterest::ReasonableExpectations,
            ]
        } else if !expectations_violated {
            vec![StakeholderInterest::FairTreatment]
        } else {
            vec![]
        };

        StakeholderAnalysis {
            stakeholders_considered,
            fair_treatment,
            interests_balanced,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &DirectorDutyFacts,
        duty_of_care: bool,
        fiduciary: bool,
        bjr: bool,
        stakeholder: &StakeholderAnalysis,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(
            "Peoples Department Stores v Wise [2004] SCC 68 - Director duties framework"
                .to_string(),
        );
        parts.push(format!(
            "Conduct by {} regarding: {}",
            facts.director, facts.conduct
        ));

        // Business judgment rule
        if bjr {
            parts.push(
                "Business judgment rule applies - decision within range of reasonable alternatives"
                    .to_string(),
            );
        } else {
            parts.push(
                "Business judgment rule does not apply - decision subject to scrutiny".to_string(),
            );
        }

        // Duty of care
        if duty_of_care {
            parts.push("Duty of care (s.122(1)(b)) - Met".to_string());
        } else {
            parts.push("Duty of care (s.122(1)(b)) - NOT met (failed to act with care, diligence, skill of reasonably prudent person)".to_string());
        }

        // Fiduciary duty
        if fiduciary {
            parts.push("Fiduciary duty (s.122(1)(a)) - Met".to_string());
        } else {
            parts.push("Fiduciary duty (s.122(1)(a)) - NOT met (breach identified)".to_string());
        }

        // Stakeholder analysis (BCE framework)
        parts.push(
            "BCE Inc v 1976 Debentureholders [2008] SCC 69 - Stakeholder analysis".to_string(),
        );
        if stakeholder.fair_treatment {
            parts.push("Fair treatment of stakeholders - satisfied".to_string());
        } else {
            parts.push("Fair treatment of stakeholders - concerns identified".to_string());
        }

        parts.join(". ")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_director_duty_met() {
        let facts = DirectorDutyFacts {
            director: "Jane Director".to_string(),
            corporation: "Test Corp".to_string(),
            conduct: "Approved acquisition".to_string(),
            context: DecisionContext {
                decision_maker: DecisionMakerType::Board,
                proper_process: true,
                special_committee: true,
                independent_advice: true,
                full_disclosure: true,
            },
            business_judgment: BusinessJudgmentFactors {
                informed: InformationLevel::Thorough,
                good_faith: true,
                no_conflict: true,
                rational_purpose: true,
                reasonable_alternative: true,
            },
            conflict: None,
            stakeholders_affected: vec![],
        };

        let result = DirectorDutyAnalyzer::analyze(&facts);

        assert!(result.duty_of_care_met);
        assert!(result.fiduciary_duty_met);
        assert!(result.business_judgment_applies);
        assert!(result.breaches.is_empty());
    }

    #[test]
    fn test_conflict_of_interest() {
        let facts = DirectorDutyFacts {
            director: "John Conflicted".to_string(),
            corporation: "Test Corp".to_string(),
            conduct: "Approved contract with related party".to_string(),
            context: DecisionContext {
                decision_maker: DecisionMakerType::Board,
                proper_process: true,
                special_committee: false,
                independent_advice: false,
                full_disclosure: false,
            },
            business_judgment: BusinessJudgmentFactors {
                informed: InformationLevel::Reasonable,
                good_faith: true,
                no_conflict: false,
                rational_purpose: true,
                reasonable_alternative: true,
            },
            conflict: Some(ConflictDetails {
                nature: ConflictNature::DirectFinancial,
                disclosed: false,
                abstained: false,
                material: true,
            }),
            stakeholders_affected: vec![],
        };

        let result = DirectorDutyAnalyzer::analyze(&facts);

        assert!(!result.business_judgment_applies);
        assert!(!result.fiduciary_duty_met);
        assert!(!result.breaches.is_empty());
    }

    #[test]
    fn test_stakeholder_impact() {
        let facts = DirectorDutyFacts {
            director: "Board".to_string(),
            corporation: "Test Corp".to_string(),
            conduct: "Major restructuring".to_string(),
            context: DecisionContext {
                decision_maker: DecisionMakerType::Board,
                proper_process: true,
                special_committee: false,
                independent_advice: true,
                full_disclosure: true,
            },
            business_judgment: BusinessJudgmentFactors {
                informed: InformationLevel::Thorough,
                good_faith: true,
                no_conflict: true,
                rational_purpose: true,
                reasonable_alternative: true,
            },
            conflict: None,
            stakeholders_affected: vec![StakeholderImpact {
                stakeholder: StakeholderType::Employees,
                impact: ImpactNature::EmploymentLoss,
                severity: ImpactSeverity::Significant,
                expectations_violated: false,
            }],
        };

        let result = DirectorDutyAnalyzer::analyze(&facts);

        assert!(result.stakeholder_analysis.stakeholders_considered);
        assert!(result.stakeholder_analysis.fair_treatment);
    }

    #[test]
    fn test_bad_faith_breach() {
        let facts = DirectorDutyFacts {
            director: "Bad Actor".to_string(),
            corporation: "Test Corp".to_string(),
            conduct: "Self-dealing transaction".to_string(),
            context: DecisionContext {
                decision_maker: DecisionMakerType::Individual,
                proper_process: false,
                special_committee: false,
                independent_advice: false,
                full_disclosure: false,
            },
            business_judgment: BusinessJudgmentFactors {
                informed: InformationLevel::None,
                good_faith: false,
                no_conflict: false,
                rational_purpose: false,
                reasonable_alternative: false,
            },
            conflict: Some(ConflictDetails {
                nature: ConflictNature::DirectFinancial,
                disclosed: false,
                abstained: false,
                material: true,
            }),
            stakeholders_affected: vec![],
        };

        let result = DirectorDutyAnalyzer::analyze(&facts);

        assert!(!result.duty_of_care_met);
        assert!(!result.fiduciary_duty_met);
        assert!(!result.business_judgment_applies);
        assert!(result.breaches.len() >= 2);
    }
}
