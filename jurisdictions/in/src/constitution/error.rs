//! Constitution of India Error Types
//!
//! Error types for constitutional violations and procedural errors.

use super::types::EmergencyType;
use crate::citation::Citation;
use thiserror::Error;

/// Constitutional error types
#[derive(Debug, Clone, Error)]
pub enum ConstitutionalError {
    // Fundamental Rights violations
    /// Article 14 violation - Equality
    #[error("Article 14: Violation of equality before law")]
    EqualityViolation { description: String },

    /// Article 19 violation - Freedoms
    #[error("Article 19: Violation of fundamental freedom")]
    FreedomViolation {
        freedom: String,
        restriction_invalid: bool,
    },

    /// Article 21 violation - Life and liberty
    #[error("Article 21: Violation of right to life and personal liberty")]
    LifeLibertyViolation { description: String },

    /// Article 21A violation - Education
    #[error("Article 21A: Violation of right to education")]
    EducationViolation { child_age: Option<u32> },

    /// Article 32/226 - Writ jurisdiction
    #[error("Article {article}: Writ petition dismissed")]
    WritRejected { article: u32, reason: String },

    // Procedural errors
    /// Emergency proclamation invalid
    #[error("Article {article}: Emergency proclamation invalid")]
    InvalidEmergency {
        article: u32,
        emergency_type: EmergencyType,
        reason: String,
    },

    /// Amendment unconstitutional
    #[error("Article 368: Amendment violates basic structure")]
    BasicStructureViolation {
        amendment_number: u32,
        feature_violated: String,
    },

    /// State ratification not obtained
    #[error("Article 368: State ratification not obtained")]
    RatificationNotObtained {
        states_required: u32,
        states_obtained: u32,
    },

    /// Special majority not achieved
    #[error("Article 368: Special majority not achieved")]
    SpecialMajorityFailed { required: u32, obtained: u32 },

    // Federal violations
    /// Centre-State relations violation
    #[error("Article {article}: Centre-State relations violated")]
    FederalViolation { article: u32, description: String },

    /// Legislative competence lacking
    #[error("Article 246: Legislature lacks competence to enact law")]
    LegislativeIncompetence { list: String, subject: String },

    /// Encroachment on state list
    #[error("Seventh Schedule: Union encroachment on State List")]
    StateListEncroachment { subject: String },

    // Judicial errors
    /// Judicial review violated
    #[error("Basic Structure: Judicial review cannot be excluded")]
    JudicialReviewExcluded,

    /// Contempt of court
    #[error("Article 129/215: Contempt of court")]
    ContemptOfCourt { court: String, nature: String },

    // Election violations
    /// Free and fair elections violated
    #[error("Basic Structure: Free and fair elections compromised")]
    ElectionViolation { description: String },

    /// Disqualification grounds
    #[error("Article 102/191: Disqualification ground exists")]
    MemberDisqualification { ground: String },

    // Miscellaneous
    /// Secular principle violated
    #[error("Basic Structure: Secular character violated")]
    SecularismViolation { description: String },

    /// Rule of law violated
    #[error("Basic Structure: Rule of law violated")]
    RuleOfLawViolation { description: String },

    /// Arbitrary state action
    #[error("Article 14: State action is arbitrary")]
    ArbitraryAction { action: String },

    /// Due process violation
    #[error("Article 21: Due process of law violated")]
    DueProcessViolation { stage: String },

    /// Validation error
    #[error("Constitutional validation error: {message}")]
    ValidationError { message: String },
}

impl ConstitutionalError {
    /// Get relevant article citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::EqualityViolation { .. } | Self::ArbitraryAction { .. } => {
                Some(Citation::statute("Constitution of India", 14, 1950))
            }
            Self::FreedomViolation { .. } => {
                Some(Citation::statute("Constitution of India", 19, 1950))
            }
            Self::LifeLibertyViolation { .. } | Self::DueProcessViolation { .. } => {
                Some(Citation::statute("Constitution of India", 21, 1950))
            }
            Self::EducationViolation { .. } => {
                Some(Citation::statute_full("Constitution of India", 21, 1, 1950))
            }
            Self::WritRejected { article, .. } => {
                Some(Citation::statute("Constitution of India", *article, 1950))
            }
            Self::InvalidEmergency { article, .. } => {
                Some(Citation::statute("Constitution of India", *article, 1950))
            }
            Self::BasicStructureViolation { .. }
            | Self::RatificationNotObtained { .. }
            | Self::SpecialMajorityFailed { .. } => {
                Some(Citation::statute("Constitution of India", 368, 1950))
            }
            Self::FederalViolation { article, .. } => {
                Some(Citation::statute("Constitution of India", *article, 1950))
            }
            Self::LegislativeIncompetence { .. } => {
                Some(Citation::statute("Constitution of India", 246, 1950))
            }
            _ => None,
        }
    }

    /// Get remedy for violation
    pub fn remedy(&self) -> &'static str {
        match self {
            Self::EqualityViolation { .. } => "File writ petition under Article 32/226",
            Self::FreedomViolation { .. } => "Challenge restriction through writ petition",
            Self::LifeLibertyViolation { .. } => "File habeas corpus petition",
            Self::EducationViolation { .. } => "Approach local authority or file PIL",
            Self::WritRejected { .. } => "Appeal to higher court",
            Self::InvalidEmergency { .. } => "Challenge in Supreme Court",
            Self::BasicStructureViolation { .. } => "Challenge amendment in Supreme Court",
            Self::RatificationNotObtained { .. } => "Amendment procedure must be restarted",
            Self::FederalViolation { .. } => {
                "Challenge through Article 131 (original jurisdiction)"
            }
            Self::LegislativeIncompetence { .. } => "Challenge law as ultra vires",
            Self::JudicialReviewExcluded => "Invoke basic structure doctrine",
            Self::ElectionViolation { .. } => "File election petition",
            Self::MemberDisqualification { .. } => "Petition to Speaker/Chairman",
            Self::SecularismViolation { .. } => "Challenge through PIL",
            Self::ArbitraryAction { .. } => "File writ of certiorari/mandamus",
            Self::DueProcessViolation { .. } => "File writ petition immediately",
            _ => "Seek appropriate constitutional remedy",
        }
    }

    /// Get leading case on point
    pub fn leading_case(&self) -> Option<&'static str> {
        match self {
            Self::EqualityViolation { .. } => Some("E.P. Royappa v. State of Tamil Nadu (1974)"),
            Self::LifeLibertyViolation { .. } => Some("Maneka Gandhi v. Union of India (1978)"),
            Self::BasicStructureViolation { .. } => {
                Some("Kesavananda Bharati v. State of Kerala (1973)")
            }
            Self::JudicialReviewExcluded => Some("L. Chandra Kumar v. Union of India (1997)"),
            Self::SecularismViolation { .. } => Some("S.R. Bommai v. Union of India (1994)"),
            Self::ArbitraryAction { .. } => Some("Ajay Hasia v. Khalid Mujib (1981)"),
            Self::DueProcessViolation { .. } => Some("Maneka Gandhi v. Union of India (1978)"),
            Self::InvalidEmergency { .. } => Some("Minerva Mills v. Union of India (1980)"),
            Self::EducationViolation { .. } => Some("Unni Krishnan v. State of A.P. (1993)"),
            _ => None,
        }
    }
}

/// Constitutional compliance report
#[derive(Debug, Clone, Default)]
pub struct ConstitutionalComplianceReport {
    /// Overall compliance
    pub compliant: bool,
    /// Fundamental rights compliant
    pub fundamental_rights_compliant: bool,
    /// Procedure compliant
    pub procedure_compliant: bool,
    /// Federal structure compliant
    pub federal_compliant: bool,
    /// Violations
    pub violations: Vec<ConstitutionalError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Result type for constitutional operations
pub type ConstitutionalResult<T> = Result<T, ConstitutionalError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = ConstitutionalError::EqualityViolation {
            description: "Discriminatory law".to_string(),
        };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 14);
    }

    #[test]
    fn test_basic_structure_remedy() {
        let error = ConstitutionalError::BasicStructureViolation {
            amendment_number: 99,
            feature_violated: "Judicial review".to_string(),
        };
        assert!(error.remedy().contains("Supreme Court"));
    }

    #[test]
    fn test_leading_case() {
        let error = ConstitutionalError::LifeLibertyViolation {
            description: "Arbitrary detention".to_string(),
        };
        let case = error.leading_case().expect("Should have leading case");
        assert!(case.contains("Maneka Gandhi"));
    }

    #[test]
    fn test_emergency_error() {
        let error = ConstitutionalError::InvalidEmergency {
            article: 352,
            emergency_type: EmergencyType::NationalEmergency,
            reason: "Not passed by cabinet".to_string(),
        };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 352);
    }
}
