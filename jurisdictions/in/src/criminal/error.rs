//! BNS 2023 Error Types
//!
//! Error types for criminal law proceedings under Bharatiya Nyaya Sanhita 2023.

use crate::citation::{Citation, cite};
use thiserror::Error;

/// BNS error types
#[derive(Debug, Clone, Error)]
pub enum BnsError {
    // Investigation errors
    /// FIR not registered
    #[error("BNS Section 173: FIR not registered within prescribed time")]
    FirNotRegistered { delay_hours: u32 },

    /// Investigation delay
    #[error("BNSS: Investigation not completed within prescribed period")]
    InvestigationDelay { offence: String, days_elapsed: u32 },

    /// Chargesheet not filed
    #[error("BNSS Section 193: Chargesheet not filed within 60/90 days")]
    ChargesheetDelay { custody_type: String, days: u32 },

    // Procedural errors
    /// Arrest without warrant
    #[error("BNSS Section 35: Arrest without warrant in non-cognizable case")]
    IllegalArrest { offence: String },

    /// Custody violation
    #[error("BNSS Section 187: Accused not produced before magistrate within 24 hours")]
    CustodyViolation { hours_elapsed: u32 },

    /// Remand exceeded
    #[error("BNSS Section 187: Police custody remand exceeded 15 days")]
    RemandExceeded { days: u32 },

    /// Bail denied wrongfully
    #[error("BNSS: Bail wrongfully denied for bailable offence")]
    BailDenied { offence: String },

    /// Section 41A compliance
    #[error("BNSS Section 41A: Notice of appearance not issued before arrest")]
    NoAppearanceNotice { offence: String },

    // Trial errors
    /// Trial delayed
    #[error("Article 21: Right to speedy trial violated")]
    TrialDelayed { years_pending: f64 },

    /// Witness tampering
    #[error("BNS Section 195: Witness tampering/intimidation")]
    WitnessTampering,

    /// Evidence tampering
    #[error("BNS Section 229: Destruction of evidence")]
    EvidenceTampering,

    /// Double jeopardy
    #[error("Article 20(2): Double jeopardy - previously prosecuted")]
    DoubleJeopardy { previous_case: String },

    // Sentencing errors
    /// Excessive sentence
    #[error("BNS: Sentence exceeds maximum prescribed for offence")]
    ExcessiveSentence {
        offence: String,
        max_years: u32,
        awarded_years: u32,
    },

    /// Mandatory minimum not met
    #[error("BNS: Mandatory minimum sentence not awarded")]
    MinimumNotMet {
        offence: String,
        min_years: u32,
        awarded_years: u32,
    },

    // Victim rights errors
    /// Victim not heard
    #[error("BNSS Section 397: Victim's right to be heard violated")]
    VictimNotHeard,

    /// Compensation not awarded
    #[error("BNS Section 395: Victim compensation not awarded")]
    CompensationNotAwarded { victim: String, amount_due: f64 },

    /// Witness protection failure
    #[error("Witness Protection Scheme 2018: Protection measures not implemented")]
    WitnessProtectionFailure,

    // Special category errors
    /// Juvenile not referred
    #[error("Juvenile Justice Act: Minor not referred to JJB")]
    JuvenileNotReferred { age: u32 },

    /// Woman accused special provisions
    #[error("BNSS Section 46: Special provisions for women not followed")]
    WomenProvisionViolation { provision: String },

    /// SC/ST atrocity
    #[error("SC/ST Prevention of Atrocities Act: Special court not constituted")]
    ScStAtrocityViolation,

    // Plea bargaining errors
    /// Plea bargaining denied
    #[error("BNSS: Plea bargaining denied for eligible offence")]
    PleaBargainingDenied { offence: String },

    // General errors
    /// Validation error
    #[error("BNS validation error: {message}")]
    ValidationError { message: String },

    /// Procedural irregularity
    #[error("BNSS procedural irregularity: {irregularity}")]
    ProceduralIrregularity { irregularity: String },
}

impl BnsError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::FirNotRegistered { .. } => Some(cite::bns(173)),
            Self::WitnessTampering => Some(cite::bns(195)),
            Self::EvidenceTampering => Some(cite::bns(229)),
            Self::CompensationNotAwarded { .. } => Some(cite::bns(395)),
            _ => None,
        }
    }

    /// Get remedial action
    pub fn remedial_action(&self) -> &'static str {
        match self {
            Self::FirNotRegistered { .. } => {
                "File complaint with senior police officer or magistrate"
            }
            Self::InvestigationDelay { .. } => "Apply to court for expediting investigation",
            Self::ChargesheetDelay { .. } => "Apply for statutory bail under BNSS",
            Self::IllegalArrest { .. } => "File habeas corpus petition",
            Self::CustodyViolation { .. } => "Report to magistrate immediately",
            Self::RemandExceeded { .. } => "Apply for release from custody",
            Self::BailDenied { .. } => "Appeal to higher court",
            Self::NoAppearanceNotice { .. } => "Challenge arrest legality",
            Self::TrialDelayed { .. } => "Apply for expeditious trial",
            Self::DoubleJeopardy { .. } => "Apply for quashing of FIR",
            Self::ExcessiveSentence { .. } => "File appeal against sentence",
            Self::MinimumNotMet { .. } => "State may appeal for enhanced sentence",
            Self::VictimNotHeard { .. } => "File application to be heard",
            Self::CompensationNotAwarded { .. } => "Apply under victim compensation scheme",
            Self::JuvenileNotReferred { .. } => "Immediately produce before JJB",
            _ => "Seek appropriate legal remedy",
        }
    }

    /// Get penalty for violation
    pub fn penalty_info(&self) -> BnsPenaltyInfo {
        match self {
            Self::WitnessTampering => BnsPenaltyInfo {
                imprisonment_years: Some(7),
                fine: Some(100_000.0),
                departmental_action: true,
            },
            Self::EvidenceTampering => BnsPenaltyInfo {
                imprisonment_years: Some(2),
                fine: Some(50_000.0),
                departmental_action: true,
            },
            Self::IllegalArrest { .. } | Self::CustodyViolation { .. } => BnsPenaltyInfo {
                imprisonment_years: None,
                fine: None,
                departmental_action: true,
            },
            _ => BnsPenaltyInfo {
                imprisonment_years: None,
                fine: None,
                departmental_action: false,
            },
        }
    }
}

/// Penalty information for BNS violations
#[derive(Debug, Clone)]
pub struct BnsPenaltyInfo {
    /// Imprisonment (years)
    pub imprisonment_years: Option<u32>,
    /// Fine (Rs.)
    pub fine: Option<f64>,
    /// Departmental action possible
    pub departmental_action: bool,
}

/// Criminal compliance report
#[derive(Debug, Clone, Default)]
pub struct CriminalComplianceReport {
    /// Overall compliance
    pub compliant: bool,
    /// Investigation compliance
    pub investigation_compliant: bool,
    /// Procedural compliance
    pub procedural_compliant: bool,
    /// Victim rights compliance
    pub victim_rights_compliant: bool,
    /// Violations
    pub violations: Vec<BnsError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Result type for BNS operations
pub type BnsResult<T> = Result<T, BnsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fir_error() {
        let error = BnsError::FirNotRegistered { delay_hours: 48 };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 173);
    }

    #[test]
    fn test_witness_tampering_penalty() {
        let error = BnsError::WitnessTampering;
        let penalty = error.penalty_info();
        assert_eq!(penalty.imprisonment_years, Some(7));
    }

    #[test]
    fn test_remedial_actions() {
        let error = BnsError::IllegalArrest {
            offence: "Defamation".to_string(),
        };
        assert!(error.remedial_action().contains("habeas corpus"));
    }

    #[test]
    fn test_custody_violation() {
        let error = BnsError::CustodyViolation { hours_elapsed: 30 };
        assert!(error.to_string().contains("24 hours"));
    }
}
