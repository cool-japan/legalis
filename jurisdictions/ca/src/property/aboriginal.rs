//! Canada Property Law - Aboriginal Title
//!
//! Analysis of Aboriginal title and duty to consult.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{AboriginalTitleElement, AboriginalTitleStatus, ConsultationLevel};

// ============================================================================
// Aboriginal Title Analysis
// ============================================================================

/// Facts for Aboriginal title analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboriginalTitleFacts {
    /// Indigenous nation/group
    pub nation: String,
    /// Claimed territory description
    pub territory: String,
    /// Evidence of occupation
    pub occupation_evidence: OccupationEvidence,
    /// Treaty status
    pub treaty_status: TreatyStatus,
}

/// Evidence of occupation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupationEvidence {
    /// Evidence of sufficient occupation
    pub sufficient: Vec<OccupationFactor>,
    /// Evidence of continuity
    pub continuity: Vec<ContinuityFactor>,
    /// Evidence of exclusivity
    pub exclusivity: Vec<ExclusivityFactor>,
}

/// Occupation factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupationFactor {
    /// Physical presence (settlements, camps)
    PhysicalPresence,
    /// Regular use (hunting, fishing, gathering)
    RegularUse,
    /// Cultivation/farming
    Cultivation,
    /// Burial grounds
    BurialGrounds,
    /// Sacred sites
    SacredSites,
    /// Oral history evidence
    OralHistory,
    /// Archaeological evidence
    Archaeological,
}

/// Continuity factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinuityFactor {
    /// Continued use to present
    ContinuedUse,
    /// Oral tradition linking to ancestors
    OralTradition,
    /// Documentary evidence
    Documentary,
    /// Displacement by Crown (not break in continuity)
    CrownDisplacement,
}

/// Exclusivity factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExclusivityFactor {
    /// Intention and capacity to control
    IntentionToControl,
    /// Exclusion of others
    ExclusionOfOthers,
    /// Permission system for others' entry
    PermissionSystem,
    /// Shared with consent (doesn't defeat exclusivity)
    SharedWithConsent,
}

/// Treaty status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatyStatus {
    /// No treaty
    NoTreaty,
    /// Historic treaty
    HistoricTreaty { treaty_name: String },
    /// Modern treaty
    ModernTreaty { treaty_name: String },
    /// Treaty negotiations ongoing
    NegotiationsOngoing,
}

/// Result of Aboriginal title analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboriginalTitleResult {
    /// Title status
    pub status: AboriginalTitleStatus,
    /// Elements established
    pub elements_established: Vec<AboriginalTitleElement>,
    /// Elements not established
    pub elements_lacking: Vec<AboriginalTitleElement>,
    /// Strength of claim (0.0 to 1.0)
    pub claim_strength: f64,
    /// Reasoning
    pub reasoning: String,
}

/// Aboriginal title analyzer
pub struct AboriginalTitleAnalyzer;

impl AboriginalTitleAnalyzer {
    /// Analyze Aboriginal title claim
    pub fn analyze(facts: &AboriginalTitleFacts) -> AboriginalTitleResult {
        let mut elements_established = Vec::new();
        let mut elements_lacking = Vec::new();
        let mut claim_strength = 0.0;

        // Check sufficient occupation
        let occupation_score = Self::score_occupation(&facts.occupation_evidence.sufficient);
        if occupation_score >= 0.6 {
            elements_established.push(AboriginalTitleElement::SufficientOccupation);
            claim_strength += 0.33;
        } else {
            elements_lacking.push(AboriginalTitleElement::SufficientOccupation);
        }

        // Check continuity
        let continuity_score = Self::score_continuity(&facts.occupation_evidence.continuity);
        if continuity_score >= 0.5 {
            elements_established.push(AboriginalTitleElement::Continuity);
            claim_strength += 0.33;
        } else {
            elements_lacking.push(AboriginalTitleElement::Continuity);
        }

        // Check exclusivity
        let exclusivity_score = Self::score_exclusivity(&facts.occupation_evidence.exclusivity);
        if exclusivity_score >= 0.5 {
            elements_established.push(AboriginalTitleElement::Exclusivity);
            claim_strength += 0.34;
        } else {
            elements_lacking.push(AboriginalTitleElement::Exclusivity);
        }

        // Determine status
        let status = if elements_established.len() == 3 {
            AboriginalTitleStatus::Proven
        } else if !elements_established.is_empty() {
            AboriginalTitleStatus::Claimed
        } else {
            AboriginalTitleStatus::TraditionalTerritory
        };

        let reasoning = Self::build_reasoning(facts, &elements_established, claim_strength);

        AboriginalTitleResult {
            status,
            elements_established,
            elements_lacking,
            claim_strength,
            reasoning,
        }
    }

    /// Score occupation evidence
    fn score_occupation(factors: &[OccupationFactor]) -> f64 {
        let mut score: f64 = 0.0;
        for factor in factors {
            score += match factor {
                OccupationFactor::PhysicalPresence => 0.25,
                OccupationFactor::RegularUse => 0.2,
                OccupationFactor::Cultivation => 0.15,
                OccupationFactor::BurialGrounds => 0.15,
                OccupationFactor::SacredSites => 0.1,
                OccupationFactor::OralHistory => 0.15,
                OccupationFactor::Archaeological => 0.2,
            };
        }
        score.min(1.0)
    }

    /// Score continuity evidence
    fn score_continuity(factors: &[ContinuityFactor]) -> f64 {
        let mut score: f64 = 0.0;
        for factor in factors {
            score += match factor {
                ContinuityFactor::ContinuedUse => 0.4,
                ContinuityFactor::OralTradition => 0.3,
                ContinuityFactor::Documentary => 0.25,
                ContinuityFactor::CrownDisplacement => 0.2, // Doesn't break continuity
            };
        }
        score.min(1.0)
    }

    /// Score exclusivity evidence
    fn score_exclusivity(factors: &[ExclusivityFactor]) -> f64 {
        let mut score: f64 = 0.0;
        for factor in factors {
            score += match factor {
                ExclusivityFactor::IntentionToControl => 0.35,
                ExclusivityFactor::ExclusionOfOthers => 0.3,
                ExclusivityFactor::PermissionSystem => 0.25,
                ExclusivityFactor::SharedWithConsent => 0.1, // Doesn't defeat exclusivity
            };
        }
        score.min(1.0)
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &AboriginalTitleFacts,
        elements: &[AboriginalTitleElement],
        strength: f64,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Tsilhqot'in Nation v BC [2014] SCC 44 - Aboriginal title test".to_string());
        parts.push(format!(
            "Claim by {} to territory: {}",
            facts.nation, facts.territory
        ));
        parts.push(format!("Elements established: {}/3", elements.len()));
        parts.push(format!("Claim strength: {:.0}%", strength * 100.0));

        match &facts.treaty_status {
            TreatyStatus::NoTreaty => {
                parts.push("No treaty affecting this territory".to_string());
            }
            TreatyStatus::HistoricTreaty { treaty_name } => {
                parts.push(format!(
                    "Subject to {} - treaty interpretation required",
                    treaty_name
                ));
            }
            TreatyStatus::ModernTreaty { treaty_name } => {
                parts.push(format!(
                    "Modern treaty: {} may extinguish or define rights",
                    treaty_name
                ));
            }
            TreatyStatus::NegotiationsOngoing => {
                parts.push("Treaty negotiations ongoing - interim measures may apply".to_string());
            }
        }

        parts.join(". ")
    }
}

// ============================================================================
// Duty to Consult Analysis
// ============================================================================

/// Facts for duty to consult analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationFacts {
    /// Proposed Crown conduct
    pub crown_conduct: String,
    /// Affected Aboriginal group
    pub affected_group: String,
    /// Asserted/established rights
    pub rights_claimed: Vec<String>,
    /// Strength of claim
    pub claim_strength: ClaimStrength,
    /// Severity of potential impact
    pub impact_severity: ImpactSeverity,
    /// Consultation undertaken
    pub consultation_undertaken: Vec<ConsultationStep>,
}

/// Claim strength (Haida spectrum)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimStrength {
    /// Proven/established right
    Established,
    /// Strong prima facie claim
    Strong,
    /// Moderate claim
    Moderate,
    /// Weak/speculative claim
    Weak,
}

/// Impact severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactSeverity {
    /// High (irreversible, significant)
    High,
    /// Moderate
    Moderate,
    /// Low (minor, reversible)
    Low,
    /// Minimal
    Minimal,
}

/// Consultation step taken
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsultationStep {
    /// Notice provided
    NoticeProvided,
    /// Information disclosed
    InformationDisclosed,
    /// Meeting held
    MeetingHeld,
    /// Written submissions received
    WrittenSubmissions,
    /// Concerns addressed
    ConcernsAddressed,
    /// Accommodation offered
    AccommodationOffered,
    /// Agreement reached
    AgreementReached,
}

/// Result of consultation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationResult {
    /// Duty triggered
    pub duty_triggered: bool,
    /// Required consultation level
    pub required_level: ConsultationLevel,
    /// Consultation adequate
    pub consultation_adequate: bool,
    /// Steps missing
    pub steps_missing: Vec<ConsultationStep>,
    /// Accommodation required
    pub accommodation_required: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Consultation analyzer
pub struct ConsultationAnalyzer;

impl ConsultationAnalyzer {
    /// Analyze duty to consult
    pub fn analyze(facts: &ConsultationFacts) -> ConsultationResult {
        // Check if duty triggered
        let duty_triggered = !facts.rights_claimed.is_empty();

        // Determine required level based on Haida spectrum
        let required_level = Self::determine_level(&facts.claim_strength, &facts.impact_severity);

        // Determine required steps for this level
        let required_steps = Self::required_steps(&required_level);

        // Check what's missing
        let steps_missing: Vec<ConsultationStep> = required_steps
            .into_iter()
            .filter(|step| !facts.consultation_undertaken.contains(step))
            .collect();

        // Consultation adequate if no steps missing
        let consultation_adequate = steps_missing.is_empty();

        // Accommodation required at deep consultation level
        let accommodation_required = matches!(required_level, ConsultationLevel::Deep)
            && !facts
                .consultation_undertaken
                .contains(&ConsultationStep::AccommodationOffered);

        let reasoning = Self::build_reasoning(
            facts,
            &required_level,
            consultation_adequate,
            &steps_missing,
        );

        ConsultationResult {
            duty_triggered,
            required_level,
            consultation_adequate,
            steps_missing,
            accommodation_required,
            reasoning,
        }
    }

    /// Determine consultation level
    fn determine_level(claim: &ClaimStrength, impact: &ImpactSeverity) -> ConsultationLevel {
        match (claim, impact) {
            (ClaimStrength::Established, ImpactSeverity::High)
            | (ClaimStrength::Strong, ImpactSeverity::High) => ConsultationLevel::Deep,
            (ClaimStrength::Established, _) | (ClaimStrength::Strong, ImpactSeverity::Moderate) => {
                ConsultationLevel::Moderate
            }
            (ClaimStrength::Moderate, ImpactSeverity::High) => ConsultationLevel::Moderate,
            _ => ConsultationLevel::Low,
        }
    }

    /// Required steps for consultation level
    fn required_steps(level: &ConsultationLevel) -> Vec<ConsultationStep> {
        match level {
            ConsultationLevel::Low => {
                vec![
                    ConsultationStep::NoticeProvided,
                    ConsultationStep::InformationDisclosed,
                ]
            }
            ConsultationLevel::Moderate => {
                vec![
                    ConsultationStep::NoticeProvided,
                    ConsultationStep::InformationDisclosed,
                    ConsultationStep::MeetingHeld,
                    ConsultationStep::WrittenSubmissions,
                    ConsultationStep::ConcernsAddressed,
                ]
            }
            ConsultationLevel::Deep => {
                vec![
                    ConsultationStep::NoticeProvided,
                    ConsultationStep::InformationDisclosed,
                    ConsultationStep::MeetingHeld,
                    ConsultationStep::WrittenSubmissions,
                    ConsultationStep::ConcernsAddressed,
                    ConsultationStep::AccommodationOffered,
                ]
            }
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &ConsultationFacts,
        level: &ConsultationLevel,
        adequate: bool,
        missing: &[ConsultationStep],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Haida Nation v BC [2004] SCC 73 - Duty to consult framework".to_string());
        parts.push(format!("Crown conduct: {}", facts.crown_conduct));
        parts.push(format!("Affected group: {}", facts.affected_group));
        parts.push(format!(
            "Claim strength: {:?}, Impact: {:?}",
            facts.claim_strength, facts.impact_severity
        ));
        parts.push(format!("Required consultation level: {:?}", level));

        if adequate {
            parts.push("Consultation adequate - all required steps taken".to_string());
        } else {
            parts.push(format!(
                "Consultation inadequate - {} steps missing",
                missing.len()
            ));
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
    fn test_aboriginal_title_analysis() {
        let facts = AboriginalTitleFacts {
            nation: "Test Nation".to_string(),
            territory: "Test Territory".to_string(),
            occupation_evidence: OccupationEvidence {
                sufficient: vec![
                    OccupationFactor::PhysicalPresence,
                    OccupationFactor::RegularUse,
                    OccupationFactor::OralHistory,
                ],
                continuity: vec![
                    ContinuityFactor::ContinuedUse,
                    ContinuityFactor::OralTradition,
                ],
                exclusivity: vec![
                    ExclusivityFactor::IntentionToControl,
                    ExclusivityFactor::ExclusionOfOthers,
                ],
            },
            treaty_status: TreatyStatus::NoTreaty,
        };

        let result = AboriginalTitleAnalyzer::analyze(&facts);

        assert!(!result.elements_established.is_empty());
        assert!(result.claim_strength > 0.0);
    }

    #[test]
    fn test_consultation_deep_level() {
        let facts = ConsultationFacts {
            crown_conduct: "Major resource development".to_string(),
            affected_group: "Test First Nation".to_string(),
            rights_claimed: vec!["Hunting rights".to_string()],
            claim_strength: ClaimStrength::Established,
            impact_severity: ImpactSeverity::High,
            consultation_undertaken: vec![ConsultationStep::NoticeProvided],
        };

        let result = ConsultationAnalyzer::analyze(&facts);

        assert!(result.duty_triggered);
        assert_eq!(result.required_level, ConsultationLevel::Deep);
        assert!(!result.consultation_adequate);
    }

    #[test]
    fn test_consultation_low_level_adequate() {
        let facts = ConsultationFacts {
            crown_conduct: "Minor road repair".to_string(),
            affected_group: "Test First Nation".to_string(),
            rights_claimed: vec!["Traditional territory".to_string()],
            claim_strength: ClaimStrength::Weak,
            impact_severity: ImpactSeverity::Low,
            consultation_undertaken: vec![
                ConsultationStep::NoticeProvided,
                ConsultationStep::InformationDisclosed,
            ],
        };

        let result = ConsultationAnalyzer::analyze(&facts);

        assert_eq!(result.required_level, ConsultationLevel::Low);
        assert!(result.consultation_adequate);
    }
}
