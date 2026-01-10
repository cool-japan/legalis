//! Error types for French evidence law

use std::error::Error;
use std::fmt;

/// Bilingual string for French/English error messages
#[derive(Debug, Clone, PartialEq)]
pub struct BilingualString {
    pub fr: String,
    pub en: String,
}

impl BilingualString {
    pub fn new(fr: impl Into<String>, en: impl Into<String>) -> Self {
        Self {
            fr: fr.into(),
            en: en.into(),
        }
    }
}

/// Errors that can occur when applying French evidence law
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceLawError {
    /// Invalid evidence type for the claim
    InvalidEvidenceType {
        evidence_type: String,
        reason: String,
    },

    /// Burden of proof not satisfied
    BurdenNotMet {
        party: String,
        missing_facts: Vec<String>,
    },

    /// Inadmissible evidence
    InadmissibleEvidence { reason: String },

    /// Electronic evidence lacks required formalities
    ElectronicEvidenceDefect { defect: String },

    /// Witness testimony issues
    WitnessCredibilityIssue { witness: String, issue: String },

    /// Expert report issues
    ExpertReportDefect { expert: String, defect: String },

    /// Presumption cannot be applied
    PresumptionNotApplicable { presumption: String, reason: String },

    /// Confession invalidity
    InvalidConfession { reason: String },

    /// Oath procedure violation
    OathProcedureViolation { violation: String },

    /// Res judicata violation (Article 1355)
    ResJudicata { previous_judgment: String },

    /// Multiple errors occurred
    MultipleErrors(Vec<EvidenceLawError>),
}

impl EvidenceLawError {
    /// Returns bilingual description of the error
    pub fn description(&self) -> BilingualString {
        match self {
            Self::InvalidEvidenceType {
                evidence_type,
                reason,
            } => BilingualString::new(
                format!("Type de preuve invalide '{}': {}", evidence_type, reason),
                format!("Invalid evidence type '{}': {}", evidence_type, reason),
            ),

            Self::BurdenNotMet {
                party,
                missing_facts,
            } => BilingualString::new(
                format!(
                    "Charge de la preuve non satisfaite pour {} (Article 1353): faits manquants: {:?}",
                    party, missing_facts
                ),
                format!(
                    "Burden of proof not met for {} (Article 1353): missing facts: {:?}",
                    party, missing_facts
                ),
            ),

            Self::InadmissibleEvidence { reason } => BilingualString::new(
                format!("Preuve irrecevable: {}", reason),
                format!("Inadmissible evidence: {}", reason),
            ),

            Self::ElectronicEvidenceDefect { defect } => BilingualString::new(
                format!(
                    "Preuve électronique défectueuse (Articles 1366-1378): {}",
                    defect
                ),
                format!(
                    "Electronic evidence defect (Articles 1366-1378): {}",
                    defect
                ),
            ),

            Self::WitnessCredibilityIssue { witness, issue } => BilingualString::new(
                format!("Problème de crédibilité du témoin '{}': {}", witness, issue),
                format!("Witness credibility issue '{}': {}", witness, issue),
            ),

            Self::ExpertReportDefect { expert, defect } => BilingualString::new(
                format!(
                    "Rapport d'expertise défectueux de '{}' (CPC 227-229): {}",
                    expert, defect
                ),
                format!(
                    "Expert report defect from '{}' (CPC 227-229): {}",
                    expert, defect
                ),
            ),

            Self::PresumptionNotApplicable {
                presumption,
                reason,
            } => BilingualString::new(
                format!(
                    "Présomption '{}' non applicable (Article 1354): {}",
                    presumption, reason
                ),
                format!(
                    "Presumption '{}' not applicable (Article 1354): {}",
                    presumption, reason
                ),
            ),

            Self::InvalidConfession { reason } => BilingualString::new(
                format!("Aveu invalide: {}", reason),
                format!("Invalid confession: {}", reason),
            ),

            Self::OathProcedureViolation { violation } => BilingualString::new(
                format!("Violation de la procédure de serment: {}", violation),
                format!("Oath procedure violation: {}", violation),
            ),

            Self::ResJudicata { previous_judgment } => BilingualString::new(
                format!(
                    "Autorité de la chose jugée (Article 1355): jugement antérieur '{}'",
                    previous_judgment
                ),
                format!(
                    "Res judicata (Article 1355): previous judgment '{}'",
                    previous_judgment
                ),
            ),

            Self::MultipleErrors(errors) => BilingualString::new(
                format!("Plusieurs erreurs de droit de la preuve ({})", errors.len()),
                format!("Multiple evidence law errors ({})", errors.len()),
            ),
        }
    }

    /// Returns French description
    pub fn description_fr(&self) -> String {
        self.description().fr
    }

    /// Returns English description
    pub fn description_en(&self) -> String {
        self.description().en
    }
}

impl fmt::Display for EvidenceLawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description().en)
    }
}

impl Error for EvidenceLawError {}

/// Result type for evidence law operations
pub type EvidenceLawResult<T> = Result<T, EvidenceLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_evidence_type_error() {
        let error = EvidenceLawError::InvalidEvidenceType {
            evidence_type: "hearsay".to_string(),
            reason: "not admissible".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("invalide"));
        assert!(desc.en.contains("Invalid"));
    }

    #[test]
    fn test_burden_not_met_error() {
        let error = EvidenceLawError::BurdenNotMet {
            party: "Plaintiff".to_string(),
            missing_facts: vec!["contract".to_string(), "breach".to_string()],
        };
        let desc = error.description();
        assert!(desc.fr.contains("Charge de la preuve"));
        assert!(desc.en.contains("Burden of proof"));
    }

    #[test]
    fn test_electronic_evidence_defect() {
        let error = EvidenceLawError::ElectronicEvidenceDefect {
            defect: "No digital signature".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("1366-1378"));
        assert!(desc.en.contains("1366-1378"));
    }

    #[test]
    fn test_res_judicata_error() {
        let error = EvidenceLawError::ResJudicata {
            previous_judgment: "TGI Paris 2024-123".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("chose jugée"));
        assert!(desc.en.contains("Res judicata"));
        assert!(desc.fr.contains("1355"));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            EvidenceLawError::InadmissibleEvidence {
                reason: "Test".to_string(),
            },
            EvidenceLawError::InvalidConfession {
                reason: "Coerced".to_string(),
            },
        ];
        let error = EvidenceLawError::MultipleErrors(errors);
        let desc = error.description();
        assert!(desc.fr.contains("Plusieurs"));
        assert!(desc.en.contains("Multiple"));
    }

    #[test]
    fn test_display_trait() {
        let error = EvidenceLawError::InadmissibleEvidence {
            reason: "Test reason".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Inadmissible"));
    }
}
