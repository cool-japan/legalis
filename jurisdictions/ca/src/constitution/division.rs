//! Canada Constitutional Law - Division of Powers
//!
//! Analyzers for determining constitutional validity under the division of powers
//! between federal and provincial governments (Constitution Act, 1867).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    ConstitutionalDoctrine, FederalPower, HeadOfPower, PithAndSubstance, ProvincialPower,
};
use crate::common::Province;

// ============================================================================
// Division of Powers Analysis
// ============================================================================

/// Facts for analyzing division of powers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionFacts {
    /// The law or measure being challenged
    pub law_name: String,
    /// Enacting body
    pub enacting_body: EnactingBody,
    /// Purpose of the law
    pub stated_purpose: String,
    /// Effects of the law
    pub effects: Vec<String>,
    /// Subject matter the law regulates
    pub subject_matter: String,
    /// Whether there is conflicting federal/provincial law
    pub conflicting_law: Option<ConflictingLaw>,
}

/// Body that enacted the law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnactingBody {
    /// Parliament of Canada
    Parliament,
    /// Provincial legislature
    ProvincialLegislature(Province),
    /// Municipal government (delegated power)
    Municipal { province: Province },
}

/// Conflicting law from other level of government
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictingLaw {
    /// Name of conflicting law
    pub name: String,
    /// Level of government
    pub level: EnactingBody,
    /// Nature of conflict
    pub conflict_type: ConflictType,
}

/// Type of conflict between laws
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Express contradiction (compliance with one means violation of other)
    ExpressContradiction,
    /// Frustration of federal purpose
    FrustrationOfPurpose,
    /// Operational conflict
    OperationalConflict,
    /// Covering the field (federal law occupies the field)
    CoveringTheField,
}

/// Result of division of powers analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionResult {
    /// Pith and substance analysis
    pub pith_and_substance: PithAndSubstance,
    /// Whether the law is valid (intra vires)
    pub is_valid: bool,
    /// Applicable constitutional doctrines
    pub doctrines_applied: Vec<ConstitutionalDoctrine>,
    /// If invalid, whether it's ultra vires the enacting body
    pub ultra_vires: bool,
    /// Paramountcy analysis (if conflict exists)
    pub paramountcy: Option<ParamountcyAnalysis>,
    /// Interjurisdictional immunity analysis
    pub iji_analysis: Option<IjiAnalysis>,
    /// Key reasoning
    pub reasoning: String,
}

/// Federal paramountcy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamountcyAnalysis {
    /// Whether there is a genuine conflict
    pub conflict_exists: bool,
    /// Whether federal law prevails
    pub federal_prevails: bool,
    /// Provincial law inoperative to extent of conflict
    pub provincial_inoperative: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Interjurisdictional immunity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IjiAnalysis {
    /// Whether provincial law affects core of federal power
    pub affects_core: bool,
    /// Whether impairment is significant
    pub significant_impairment: bool,
    /// Federal undertaking or matter at issue
    pub federal_matter: String,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Division of Powers Analyzer
// ============================================================================

/// Analyzer for division of powers issues
pub struct DivisionAnalyzer;

impl DivisionAnalyzer {
    /// Analyze whether a law is constitutionally valid
    pub fn analyze(facts: &DivisionFacts) -> DivisionResult {
        let mut doctrines = Vec::new();

        // Step 1: Pith and substance analysis
        doctrines.push(ConstitutionalDoctrine::PithAndSubstance);
        let pith_and_substance = Self::analyze_pith_and_substance(facts);

        // Step 2: Determine if valid under head of power
        let (is_valid, ultra_vires) = Self::check_validity(facts, &pith_and_substance);

        // Step 3: Check for double aspect if needed
        if Self::has_double_aspect(facts) {
            doctrines.push(ConstitutionalDoctrine::DoubleAspect);
        }

        // Step 4: Paramountcy analysis if conflict exists
        let paramountcy = if let Some(conflict) = &facts.conflicting_law {
            doctrines.push(ConstitutionalDoctrine::FederalParamountcy);
            Some(Self::analyze_paramountcy(facts, conflict))
        } else {
            None
        };

        // Step 5: Interjurisdictional immunity (if provincial affecting federal)
        let iji_analysis = if matches!(facts.enacting_body, EnactingBody::ProvincialLegislature(_))
            && Self::may_affect_federal_core(facts)
        {
            doctrines.push(ConstitutionalDoctrine::InterjurisdictionalImmunity);
            Some(Self::analyze_iji(facts))
        } else {
            None
        };

        // Build reasoning
        let reasoning = Self::build_reasoning(
            &facts.law_name,
            is_valid,
            &pith_and_substance,
            paramountcy.as_ref(),
        );

        DivisionResult {
            pith_and_substance,
            is_valid,
            doctrines_applied: doctrines,
            ultra_vires,
            paramountcy,
            iji_analysis,
            reasoning,
        }
    }

    /// Analyze pith and substance
    fn analyze_pith_and_substance(facts: &DivisionFacts) -> PithAndSubstance {
        // Determine dominant characteristic from purpose and effects
        let dominant_characteristic = format!(
            "The dominant purpose is '{}' with primary effects on '{}'",
            facts.stated_purpose, facts.subject_matter
        );

        // Classify under head of power based on subject matter
        let head_of_power = Self::classify_head_of_power(facts);

        // Check if valid
        let is_valid = Self::matches_enacting_authority(facts, &head_of_power);

        let reasoning = format!(
            "Applying pith and substance analysis: The law's dominant characteristic is {}. \
             This relates to {}, which falls under {}.",
            dominant_characteristic,
            facts.subject_matter,
            match &head_of_power {
                HeadOfPower::Federal(p) => format!("federal power s.{}", p.section()),
                HeadOfPower::Provincial(p) => format!("provincial power s.{}", p.section()),
                HeadOfPower::DoubleAspect {
                    federal,
                    provincial,
                } => format!(
                    "both federal s.{} and provincial s.{} (double aspect)",
                    federal.section(),
                    provincial.section()
                ),
            }
        );

        PithAndSubstance {
            dominant_characteristic,
            matter: facts.subject_matter.clone(),
            head_of_power,
            is_valid,
            reasoning,
        }
    }

    /// Classify subject matter under head of power
    fn classify_head_of_power(facts: &DivisionFacts) -> HeadOfPower {
        let subject = facts.subject_matter.to_lowercase();

        // Check for federal matters
        if subject.contains("criminal")
            || subject.contains("offence")
            || subject.contains("penalty")
        {
            return HeadOfPower::Federal(FederalPower::CriminalLaw);
        }
        if subject.contains("bank") || subject.contains("currency") {
            return HeadOfPower::Federal(FederalPower::Banking);
        }
        if subject.contains("trade") && subject.contains("interprovincial") {
            return HeadOfPower::Federal(FederalPower::TradeAndCommerce);
        }
        if subject.contains("telecom") || subject.contains("broadcast") {
            return HeadOfPower::Federal(FederalPower::Telecommunications);
        }
        if subject.contains("aboriginal") || subject.contains("indigenous") {
            return HeadOfPower::Federal(FederalPower::IndiansAndLands);
        }
        if subject.contains("divorce") {
            return HeadOfPower::Federal(FederalPower::MarriageDivorce);
        }
        if subject.contains("bankruptcy") || subject.contains("insolvency") {
            return HeadOfPower::Federal(FederalPower::BankruptcyInsolvency);
        }

        // Check for provincial matters
        if subject.contains("property") || subject.contains("contract") {
            return HeadOfPower::Provincial(ProvincialPower::PropertyAndCivilRights);
        }
        if subject.contains("health") || subject.contains("hospital") {
            return HeadOfPower::Provincial(ProvincialPower::HealthCare);
        }
        if subject.contains("education") || subject.contains("school") {
            return HeadOfPower::Provincial(ProvincialPower::Education);
        }
        if subject.contains("labour") || subject.contains("employment") {
            return HeadOfPower::Provincial(ProvincialPower::LabourRelations);
        }
        if subject.contains("municipal") || subject.contains("local") {
            return HeadOfPower::Provincial(ProvincialPower::LocalMatters);
        }
        if subject.contains("highway") || subject.contains("road") {
            return HeadOfPower::Provincial(ProvincialPower::LocalWorks);
        }
        if subject.contains("natural resource") || subject.contains("mining") {
            return HeadOfPower::Provincial(ProvincialPower::NaturalResources);
        }
        if subject.contains("securities") {
            return HeadOfPower::Provincial(ProvincialPower::SecuritiesRegulation);
        }

        // Check for double aspect (environmental regulation is classic example)
        if subject.contains("environment") || subject.contains("pollution") {
            return HeadOfPower::DoubleAspect {
                federal: FederalPower::CriminalLaw, // Environmental offences
                provincial: ProvincialPower::PropertyAndCivilRights,
            };
        }

        // Default to property and civil rights (catch-all provincial)
        HeadOfPower::Provincial(ProvincialPower::PropertyAndCivilRights)
    }

    /// Check if enacting body matches head of power
    fn matches_enacting_authority(facts: &DivisionFacts, head: &HeadOfPower) -> bool {
        match (&facts.enacting_body, head) {
            (EnactingBody::Parliament, HeadOfPower::Federal(_)) => true,
            (EnactingBody::ProvincialLegislature(_), HeadOfPower::Provincial(_)) => true,
            (_, HeadOfPower::DoubleAspect { .. }) => true, // Both can legislate
            _ => false,
        }
    }

    /// Check validity of law
    fn check_validity(_facts: &DivisionFacts, pith: &PithAndSubstance) -> (bool, bool) {
        let valid = pith.is_valid;
        let ultra_vires = !valid;
        (valid, ultra_vires)
    }

    /// Check for double aspect doctrine
    fn has_double_aspect(facts: &DivisionFacts) -> bool {
        let subject = facts.subject_matter.to_lowercase();
        subject.contains("environment")
            || subject.contains("securities")
            || subject.contains("health")
    }

    /// Check if provincial law may affect core federal matter
    fn may_affect_federal_core(facts: &DivisionFacts) -> bool {
        let subject = facts.subject_matter.to_lowercase();
        subject.contains("bank")
            || subject.contains("aeronaut")
            || subject.contains("telecom")
            || subject.contains("aboriginal")
    }

    /// Analyze federal paramountcy
    fn analyze_paramountcy(
        facts: &DivisionFacts,
        conflict: &ConflictingLaw,
    ) -> ParamountcyAnalysis {
        let conflict_exists = match conflict.conflict_type {
            ConflictType::ExpressContradiction => true,
            ConflictType::FrustrationOfPurpose => true,
            ConflictType::OperationalConflict => true,
            ConflictType::CoveringTheField => false, // Not sufficient alone anymore
        };

        let federal_prevails = conflict_exists
            && matches!(conflict.level, EnactingBody::Parliament)
            && matches!(facts.enacting_body, EnactingBody::ProvincialLegislature(_));

        let provincial_inoperative = federal_prevails;

        let reasoning = if conflict_exists && federal_prevails {
            format!(
                "Federal paramountcy applies: There is an operational conflict between {} and {}. \
                 The provincial law is inoperative to the extent of the inconsistency.",
                facts.law_name, conflict.name
            )
        } else if conflict_exists {
            format!(
                "Although conflict exists, federal paramountcy does not apply as {} is not \
                 provincial law challenged against federal law.",
                facts.law_name
            )
        } else {
            format!(
                "No genuine conflict exists between {} and {}. Both laws can operate together.",
                facts.law_name, conflict.name
            )
        };

        ParamountcyAnalysis {
            conflict_exists,
            federal_prevails,
            provincial_inoperative,
            reasoning,
        }
    }

    /// Analyze interjurisdictional immunity
    fn analyze_iji(facts: &DivisionFacts) -> IjiAnalysis {
        let subject = facts.subject_matter.to_lowercase();

        let federal_matter = if subject.contains("bank") {
            "Banking under s.91(15)".to_string()
        } else if subject.contains("aeronaut") {
            "Aeronautics (POGG)".to_string()
        } else if subject.contains("telecom") {
            "Telecommunications (s.92(10)(a))".to_string()
        } else if subject.contains("aboriginal") {
            "Indians and lands reserved for Indians (s.91(24))".to_string()
        } else {
            "Federal undertaking".to_string()
        };

        // Modern IJI is narrowly applied (Canadian Western Bank)
        let affects_core = false; // Conservative approach
        let significant_impairment = false;

        IjiAnalysis {
            affects_core,
            significant_impairment,
            federal_matter,
            reasoning: "Following Canadian Western Bank v Alberta [2007] 2 SCR 3, \
                interjurisdictional immunity applies only where provincial law \
                significantly impairs the core of an exclusive federal power. \
                This doctrine is applied narrowly."
                .to_string(),
        }
    }

    /// Build reasoning
    fn build_reasoning(
        law_name: &str,
        is_valid: bool,
        pith: &PithAndSubstance,
        paramountcy: Option<&ParamountcyAnalysis>,
    ) -> String {
        let mut reasoning = format!("Constitutional analysis of '{}': ", law_name);

        reasoning.push_str(&pith.reasoning);

        if let Some(p) = paramountcy {
            reasoning.push(' ');
            reasoning.push_str(&p.reasoning);
        }

        if is_valid {
            reasoning.push_str(" The law is constitutionally valid (intra vires).");
        } else {
            reasoning.push_str(" The law is constitutionally invalid (ultra vires).");
        }

        reasoning
    }
}

// ============================================================================
// POGG Analyzer
// ============================================================================

/// Analyzer for Peace, Order, and Good Government power
pub struct PoggAnalyzer;

impl PoggAnalyzer {
    /// Analyze whether matter falls under POGG
    pub fn analyze(matter: &str, claimed_branch: PoggBranch, evidence: &[String]) -> PoggAnalysis {
        match claimed_branch {
            PoggBranch::Emergency => Self::analyze_emergency(matter, evidence),
            PoggBranch::NationalConcern => Self::analyze_national_concern(matter, evidence),
            PoggBranch::Residual => Self::analyze_residual(matter, evidence),
        }
    }

    fn analyze_emergency(matter: &str, evidence: &[String]) -> PoggAnalysis {
        let temporary = evidence.iter().any(|e| e.contains("temporary"));
        let is_valid = !evidence.is_empty() && temporary;

        PoggAnalysis {
            branch: PoggBranch::Emergency,
            is_valid,
            reasoning: format!(
                "POGG emergency branch: The matter '{}' {} constitute a national emergency \
                 requiring temporary federal intervention. Emergency power is valid only \
                 for duration of emergency.",
                matter,
                if is_valid { "may" } else { "does not" }
            ),
        }
    }

    fn analyze_national_concern(matter: &str, evidence: &[String]) -> PoggAnalysis {
        // Crown Zellerbach test
        let is_valid = !evidence.is_empty();

        PoggAnalysis {
            branch: PoggBranch::NationalConcern,
            is_valid,
            reasoning: format!(
                "POGG national concern branch (R v Crown Zellerbach): The matter '{}' must \
                 have singleness, distinctiveness, and indivisibility that clearly distinguishes \
                 it from matters of provincial concern. Impact on provincial jurisdiction must \
                 be reconcilable with distribution of powers.",
                matter
            ),
        }
    }

    fn analyze_residual(matter: &str, _evidence: &[String]) -> PoggAnalysis {
        PoggAnalysis {
            branch: PoggBranch::Residual,
            is_valid: true,
            reasoning: format!(
                "POGG residual branch: The matter '{}' does not fall within any enumerated \
                 head of provincial power and therefore falls to federal Parliament under \
                 the residual clause.",
                matter
            ),
        }
    }
}

/// POGG branch claimed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoggBranch {
    /// Emergency (temporary)
    Emergency,
    /// National concern (permanent)
    NationalConcern,
    /// Residual gap
    Residual,
}

/// Result of POGG analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoggAnalysis {
    /// Branch of POGG
    pub branch: PoggBranch,
    /// Whether valid under POGG
    pub is_valid: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federal_criminal_law() {
        let facts = DivisionFacts {
            law_name: "Criminal Code Amendment".to_string(),
            enacting_body: EnactingBody::Parliament,
            stated_purpose: "Prohibit certain conduct".to_string(),
            effects: vec!["Creates new criminal offence".to_string()],
            subject_matter: "Criminal law - new offence".to_string(),
            conflicting_law: None,
        };

        let result = DivisionAnalyzer::analyze(&facts);
        assert!(result.is_valid);
        assert!(!result.ultra_vires);
    }

    #[test]
    fn test_provincial_health() {
        let facts = DivisionFacts {
            law_name: "Health Services Act".to_string(),
            enacting_body: EnactingBody::ProvincialLegislature(Province::Ontario),
            stated_purpose: "Regulate health care delivery".to_string(),
            effects: vec!["Hospital funding".to_string()],
            subject_matter: "Health care services".to_string(),
            conflicting_law: None,
        };

        let result = DivisionAnalyzer::analyze(&facts);
        assert!(result.is_valid);
    }

    #[test]
    fn test_ultra_vires_provincial_criminal() {
        let facts = DivisionFacts {
            law_name: "Provincial Criminal Offence Act".to_string(),
            enacting_body: EnactingBody::ProvincialLegislature(Province::Quebec),
            stated_purpose: "Create new criminal offence".to_string(),
            effects: vec!["Imprisonment for violation".to_string()],
            subject_matter: "Criminal law - offence creation".to_string(),
            conflicting_law: None,
        };

        let result = DivisionAnalyzer::analyze(&facts);
        assert!(!result.is_valid);
        assert!(result.ultra_vires);
    }

    #[test]
    fn test_paramountcy() {
        let facts = DivisionFacts {
            law_name: "Provincial Banking Regulation".to_string(),
            enacting_body: EnactingBody::ProvincialLegislature(Province::Alberta),
            stated_purpose: "Regulate bank operations".to_string(),
            effects: vec!["Restrict bank hours".to_string()],
            subject_matter: "Banking regulation".to_string(),
            conflicting_law: Some(ConflictingLaw {
                name: "Bank Act".to_string(),
                level: EnactingBody::Parliament,
                conflict_type: ConflictType::OperationalConflict,
            }),
        };

        let result = DivisionAnalyzer::analyze(&facts);
        assert!(result.paramountcy.is_some());
    }

    #[test]
    fn test_double_aspect() {
        let facts = DivisionFacts {
            law_name: "Environmental Protection Act".to_string(),
            enacting_body: EnactingBody::ProvincialLegislature(Province::BritishColumbia),
            stated_purpose: "Protect environment".to_string(),
            effects: vec!["Pollution limits".to_string()],
            subject_matter: "Environment and pollution control".to_string(),
            conflicting_law: None,
        };

        let result = DivisionAnalyzer::analyze(&facts);
        assert!(
            result
                .doctrines_applied
                .contains(&ConstitutionalDoctrine::DoubleAspect)
        );
    }
}
