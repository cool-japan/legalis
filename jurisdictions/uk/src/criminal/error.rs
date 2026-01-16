//! UK Criminal Law - Error Types
//!
//! This module provides comprehensive error types for UK criminal law analysis,
//! covering offence analysis, mens rea/actus reus determination, causation,
//! defences, and sentencing.

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Primary error type for UK criminal law operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CriminalError {
    /// Error in offence analysis
    OffenceAnalysis(OffenceAnalysisError),
    /// Error in mens rea analysis
    MensRea(MensReaError),
    /// Error in actus reus analysis
    ActusReus(ActusReusError),
    /// Error in causation analysis
    Causation(CausationError),
    /// Error in defence analysis
    Defence(DefenceError),
    /// Error in sentencing analysis
    Sentencing(SentencingError),
    /// Error in procedure analysis
    Procedure(ProcedureError),
    /// Error in party liability analysis
    PartyLiability(PartyLiabilityError),
    /// Invalid input data
    InvalidInput(String),
    /// Missing required information
    MissingInformation(String),
    /// Internal analysis error
    InternalError(String),
}

impl fmt::Display for CriminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffenceAnalysis(e) => write!(f, "Offence analysis error: {e}"),
            Self::MensRea(e) => write!(f, "Mens rea error: {e}"),
            Self::ActusReus(e) => write!(f, "Actus reus error: {e}"),
            Self::Causation(e) => write!(f, "Causation error: {e}"),
            Self::Defence(e) => write!(f, "Defence error: {e}"),
            Self::Sentencing(e) => write!(f, "Sentencing error: {e}"),
            Self::Procedure(e) => write!(f, "Procedure error: {e}"),
            Self::PartyLiability(e) => write!(f, "Party liability error: {e}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::MissingInformation(msg) => write!(f, "Missing information: {msg}"),
            Self::InternalError(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for CriminalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OffenceAnalysis(e) => Some(e),
            Self::MensRea(e) => Some(e),
            Self::ActusReus(e) => Some(e),
            Self::Causation(e) => Some(e),
            Self::Defence(e) => Some(e),
            Self::Sentencing(e) => Some(e),
            Self::Procedure(e) => Some(e),
            Self::PartyLiability(e) => Some(e),
            _ => None,
        }
    }
}

// ============================================================================
// Offence Analysis Errors
// ============================================================================

/// Errors in offence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OffenceAnalysisError {
    /// Offence not recognized
    UnrecognizedOffence { offence_name: String },
    /// Offence elements incomplete
    IncompleteElements {
        offence: String,
        missing_elements: Vec<String>,
    },
    /// Statutory interpretation issue
    StatutoryInterpretation { statute: String, issue: String },
    /// Classification error
    ClassificationError { offence: String, reason: String },
    /// Offence abolished or repealed
    OffenceAbolished {
        offence: String,
        repealed_by: String,
    },
    /// Attempt not applicable
    AttemptNotApplicable { offence: String, reason: String },
}

impl fmt::Display for OffenceAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnrecognizedOffence { offence_name } => {
                write!(f, "Offence not recognized: {offence_name}")
            }
            Self::IncompleteElements {
                offence,
                missing_elements,
            } => {
                write!(
                    f,
                    "Incomplete elements for {offence}: missing {}",
                    missing_elements.join(", ")
                )
            }
            Self::StatutoryInterpretation { statute, issue } => {
                write!(f, "Statutory interpretation issue in {statute}: {issue}")
            }
            Self::ClassificationError { offence, reason } => {
                write!(f, "Classification error for {offence}: {reason}")
            }
            Self::OffenceAbolished {
                offence,
                repealed_by,
            } => {
                write!(f, "Offence {offence} abolished by {repealed_by}")
            }
            Self::AttemptNotApplicable { offence, reason } => {
                write!(f, "Attempt not applicable to {offence}: {reason}")
            }
        }
    }
}

impl std::error::Error for OffenceAnalysisError {}

// ============================================================================
// Mens Rea Errors
// ============================================================================

/// Errors in mens rea analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MensReaError {
    /// Mens rea type not applicable to offence
    NotApplicable {
        mens_rea_type: String,
        offence: String,
    },
    /// Insufficient evidence for mens rea
    InsufficientEvidence {
        mens_rea_type: String,
        required_evidence: Vec<String>,
    },
    /// Conflicting evidence
    ConflictingEvidence { issue: String },
    /// Strict liability determination failed
    StrictLiabilityUnclear { offence: String, reason: String },
    /// Transferred malice issue
    TransferredMaliceIssue { issue: String },
    /// Contemporaneity issue (coincidence of AR and MR)
    ContemporaneityIssue { issue: String },
}

impl fmt::Display for MensReaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotApplicable {
                mens_rea_type,
                offence,
            } => {
                write!(f, "{mens_rea_type} not applicable to {offence}")
            }
            Self::InsufficientEvidence {
                mens_rea_type,
                required_evidence,
            } => {
                write!(
                    f,
                    "Insufficient evidence for {mens_rea_type}: need {}",
                    required_evidence.join(", ")
                )
            }
            Self::ConflictingEvidence { issue } => {
                write!(f, "Conflicting evidence: {issue}")
            }
            Self::StrictLiabilityUnclear { offence, reason } => {
                write!(f, "Strict liability unclear for {offence}: {reason}")
            }
            Self::TransferredMaliceIssue { issue } => {
                write!(f, "Transferred malice issue: {issue}")
            }
            Self::ContemporaneityIssue { issue } => {
                write!(f, "Contemporaneity issue: {issue}")
            }
        }
    }
}

impl std::error::Error for MensReaError {}

// ============================================================================
// Actus Reus Errors
// ============================================================================

/// Errors in actus reus analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActusReusError {
    /// No voluntary act identified
    NoVoluntaryAct { reason: String },
    /// Omission without duty
    OmissionNoDuty {
        claimed_duty: String,
        rejection_reason: String,
    },
    /// Conduct element unclear
    ConductUnclear { issue: String },
    /// Circumstances element not met
    CircumstancesNotMet { required: String, actual: String },
    /// Result element not met
    ResultNotMet { required: String, actual: String },
    /// State of affairs issue
    StateOfAffairsIssue { issue: String },
}

impl fmt::Display for ActusReusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoVoluntaryAct { reason } => {
                write!(f, "No voluntary act: {reason}")
            }
            Self::OmissionNoDuty {
                claimed_duty,
                rejection_reason,
            } => {
                write!(
                    f,
                    "Omission without valid duty: claimed {claimed_duty}, rejected because {rejection_reason}"
                )
            }
            Self::ConductUnclear { issue } => {
                write!(f, "Conduct element unclear: {issue}")
            }
            Self::CircumstancesNotMet { required, actual } => {
                write!(
                    f,
                    "Circumstances not met: required {required}, found {actual}"
                )
            }
            Self::ResultNotMet { required, actual } => {
                write!(f, "Result not met: required {required}, found {actual}")
            }
            Self::StateOfAffairsIssue { issue } => {
                write!(f, "State of affairs issue: {issue}")
            }
        }
    }
}

impl std::error::Error for ActusReusError {}

// ============================================================================
// Causation Errors
// ============================================================================

/// Errors in causation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CausationError {
    /// Factual causation not established
    FactualCausationFailed { reason: String },
    /// Legal causation not established
    LegalCausationFailed { reason: String },
    /// Chain of causation broken
    ChainBroken {
        intervening_act: String,
        analysis: String,
    },
    /// Causation unclear due to multiple causes
    MultipleCauses { causes: Vec<String>, issue: String },
    /// Thin skull rule application issue
    ThinSkullIssue { issue: String },
    /// Medical treatment causation issue
    MedicalTreatmentIssue { treatment: String, issue: String },
}

impl fmt::Display for CausationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FactualCausationFailed { reason } => {
                write!(f, "Factual causation failed: {reason}")
            }
            Self::LegalCausationFailed { reason } => {
                write!(f, "Legal causation failed: {reason}")
            }
            Self::ChainBroken {
                intervening_act,
                analysis,
            } => {
                write!(f, "Causal chain broken by {intervening_act}: {analysis}")
            }
            Self::MultipleCauses { causes, issue } => {
                write!(f, "Multiple causes issue: {} - {issue}", causes.join(", "))
            }
            Self::ThinSkullIssue { issue } => {
                write!(f, "Thin skull rule issue: {issue}")
            }
            Self::MedicalTreatmentIssue { treatment, issue } => {
                write!(f, "Medical treatment causation: {treatment} - {issue}")
            }
        }
    }
}

impl std::error::Error for CausationError {}

// ============================================================================
// Defence Errors
// ============================================================================

/// Errors in defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DefenceError {
    /// Defence not available for this offence
    NotAvailable {
        defence: String,
        offence: String,
        reason: String,
    },
    /// Defence elements not met
    ElementsNotMet {
        defence: String,
        missing: Vec<String>,
    },
    /// Defence withdrawn/lost
    DefenceWithdrawn { defence: String, reason: String },
    /// Evidential burden not met
    EvidentialBurdenNotMet { defence: String, required: String },
    /// Legal burden not discharged
    LegalBurdenNotDischarged { defence: String, standard: String },
    /// Partial defence inapplicable
    PartialDefenceInapplicable { defence: String, reason: String },
    /// Intoxication defence issue
    IntoxicationIssue { issue: String },
    /// Self-defence proportionality issue
    ProportionalityIssue { force_used: String, threat: String },
    /// Duress unavailable (murder/attempted murder)
    DuressUnavailable { offence: String },
    /// Necessity not recognized for offence
    NecessityNotRecognized { offence: String },
}

impl fmt::Display for DefenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAvailable {
                defence,
                offence,
                reason,
            } => {
                write!(f, "{defence} not available for {offence}: {reason}")
            }
            Self::ElementsNotMet { defence, missing } => {
                write!(
                    f,
                    "{defence} elements not met: missing {}",
                    missing.join(", ")
                )
            }
            Self::DefenceWithdrawn { defence, reason } => {
                write!(f, "{defence} withdrawn: {reason}")
            }
            Self::EvidentialBurdenNotMet { defence, required } => {
                write!(f, "Evidential burden for {defence} not met: {required}")
            }
            Self::LegalBurdenNotDischarged { defence, standard } => {
                write!(f, "Legal burden for {defence} not discharged to {standard}")
            }
            Self::PartialDefenceInapplicable { defence, reason } => {
                write!(f, "Partial defence {defence} inapplicable: {reason}")
            }
            Self::IntoxicationIssue { issue } => {
                write!(f, "Intoxication defence issue: {issue}")
            }
            Self::ProportionalityIssue { force_used, threat } => {
                write!(f, "Force disproportionate: {force_used} against {threat}")
            }
            Self::DuressUnavailable { offence } => {
                write!(f, "Duress unavailable for {offence}")
            }
            Self::NecessityNotRecognized { offence } => {
                write!(f, "Necessity not recognized for {offence}")
            }
        }
    }
}

impl std::error::Error for DefenceError {}

// ============================================================================
// Sentencing Errors
// ============================================================================

/// Errors in sentencing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SentencingError {
    /// Guideline not found
    GuidelineNotFound { offence: String },
    /// Category determination failed
    CategoryDeterminationFailed { reason: String },
    /// Invalid sentence for offence
    InvalidSentence {
        sentence: String,
        offence: String,
        reason: String,
    },
    /// Sentence exceeds maximum
    ExceedsMaximum {
        proposed: String,
        maximum: String,
        offence: String,
    },
    /// Minimum term issue
    MinimumTermIssue { issue: String },
    /// Dangerous offender criteria unclear
    DangerousOffenderUnclear { reason: String },
    /// Totality principle issue
    TotalityIssue {
        sentences: Vec<String>,
        issue: String,
    },
    /// Guilty plea reduction error
    GuiltyPleaReductionError { reason: String },
}

impl fmt::Display for SentencingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GuidelineNotFound { offence } => {
                write!(f, "Sentencing guideline not found for {offence}")
            }
            Self::CategoryDeterminationFailed { reason } => {
                write!(f, "Category determination failed: {reason}")
            }
            Self::InvalidSentence {
                sentence,
                offence,
                reason,
            } => {
                write!(f, "Invalid sentence {sentence} for {offence}: {reason}")
            }
            Self::ExceedsMaximum {
                proposed,
                maximum,
                offence,
            } => {
                write!(
                    f,
                    "Sentence {proposed} exceeds maximum {maximum} for {offence}"
                )
            }
            Self::MinimumTermIssue { issue } => {
                write!(f, "Minimum term issue: {issue}")
            }
            Self::DangerousOffenderUnclear { reason } => {
                write!(f, "Dangerous offender status unclear: {reason}")
            }
            Self::TotalityIssue { sentences, issue } => {
                write!(
                    f,
                    "Totality issue with {} sentences: {issue}",
                    sentences.len()
                )
            }
            Self::GuiltyPleaReductionError { reason } => {
                write!(f, "Guilty plea reduction error: {reason}")
            }
        }
    }
}

impl std::error::Error for SentencingError {}

// ============================================================================
// Procedure Errors
// ============================================================================

/// Errors in criminal procedure analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcedureError {
    /// PACE breach
    PaceBreach {
        code: String,
        provision: String,
        breach: String,
    },
    /// Unlawful arrest
    UnlawfulArrest { reason: String },
    /// Detention time limit exceeded
    DetentionExceeded { limit_hours: u32, actual_hours: u32 },
    /// Improper interview
    ImproperInterview { issue: String },
    /// Identification procedure error
    IdentificationError { procedure: String, error: String },
    /// Disclosure failure
    DisclosureFailure { missing: String },
    /// Abuse of process
    AbuseOfProcess { grounds: String },
    /// Mode of trial error
    ModeOfTrialError { issue: String },
    /// Jury irregularity
    JuryIrregularity { issue: String },
}

impl fmt::Display for ProcedureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PaceBreach {
                code,
                provision,
                breach,
            } => {
                write!(f, "PACE Code {code} {provision} breach: {breach}")
            }
            Self::UnlawfulArrest { reason } => {
                write!(f, "Unlawful arrest: {reason}")
            }
            Self::DetentionExceeded {
                limit_hours,
                actual_hours,
            } => {
                write!(
                    f,
                    "Detention exceeded: {actual_hours}h exceeds {limit_hours}h limit"
                )
            }
            Self::ImproperInterview { issue } => {
                write!(f, "Improper interview: {issue}")
            }
            Self::IdentificationError { procedure, error } => {
                write!(f, "Identification error in {procedure}: {error}")
            }
            Self::DisclosureFailure { missing } => {
                write!(f, "Disclosure failure: {missing}")
            }
            Self::AbuseOfProcess { grounds } => {
                write!(f, "Abuse of process: {grounds}")
            }
            Self::ModeOfTrialError { issue } => {
                write!(f, "Mode of trial error: {issue}")
            }
            Self::JuryIrregularity { issue } => {
                write!(f, "Jury irregularity: {issue}")
            }
        }
    }
}

impl std::error::Error for ProcedureError {}

// ============================================================================
// Party Liability Errors
// ============================================================================

/// Errors in party/accessory liability analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PartyLiabilityError {
    /// Joint enterprise analysis failed
    JointEnterpriseFailed { reason: String },
    /// Secondary participation unclear
    SecondaryParticipationUnclear { issue: String },
    /// Withdrawal ineffective
    WithdrawalIneffective { reason: String },
    /// Principal not identified
    PrincipalNotIdentified { issue: String },
    /// Innocent agency issue
    InnocentAgencyIssue { issue: String },
    /// Post-Jogee intent issue
    JogeeIntentIssue { issue: String },
}

impl fmt::Display for PartyLiabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JointEnterpriseFailed { reason } => {
                write!(f, "Joint enterprise analysis failed: {reason}")
            }
            Self::SecondaryParticipationUnclear { issue } => {
                write!(f, "Secondary participation unclear: {issue}")
            }
            Self::WithdrawalIneffective { reason } => {
                write!(f, "Withdrawal ineffective: {reason}")
            }
            Self::PrincipalNotIdentified { issue } => {
                write!(f, "Principal not identified: {issue}")
            }
            Self::InnocentAgencyIssue { issue } => {
                write!(f, "Innocent agency issue: {issue}")
            }
            Self::JogeeIntentIssue { issue } => {
                write!(f, "Post-Jogee intent requirement issue: {issue}")
            }
        }
    }
}

impl std::error::Error for PartyLiabilityError {}

// ============================================================================
// Conversion Traits
// ============================================================================

impl From<OffenceAnalysisError> for CriminalError {
    fn from(err: OffenceAnalysisError) -> Self {
        Self::OffenceAnalysis(err)
    }
}

impl From<MensReaError> for CriminalError {
    fn from(err: MensReaError) -> Self {
        Self::MensRea(err)
    }
}

impl From<ActusReusError> for CriminalError {
    fn from(err: ActusReusError) -> Self {
        Self::ActusReus(err)
    }
}

impl From<CausationError> for CriminalError {
    fn from(err: CausationError) -> Self {
        Self::Causation(err)
    }
}

impl From<DefenceError> for CriminalError {
    fn from(err: DefenceError) -> Self {
        Self::Defence(err)
    }
}

impl From<SentencingError> for CriminalError {
    fn from(err: SentencingError) -> Self {
        Self::Sentencing(err)
    }
}

impl From<ProcedureError> for CriminalError {
    fn from(err: ProcedureError) -> Self {
        Self::Procedure(err)
    }
}

impl From<PartyLiabilityError> for CriminalError {
    fn from(err: PartyLiabilityError) -> Self {
        Self::PartyLiability(err)
    }
}

// ============================================================================
// Result Type Alias
// ============================================================================

/// Result type for criminal law operations
pub type CriminalResult<T> = Result<T, CriminalError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_criminal_error_display() {
        let err = CriminalError::InvalidInput("test input".into());
        assert!(err.to_string().contains("Invalid input"));
    }

    #[test]
    fn test_offence_analysis_error() {
        let err = OffenceAnalysisError::UnrecognizedOffence {
            offence_name: "Unknown offence".into(),
        };
        assert!(err.to_string().contains("not recognized"));
    }

    #[test]
    fn test_mens_rea_error() {
        let err = MensReaError::InsufficientEvidence {
            mens_rea_type: "intention".into(),
            required_evidence: vec!["planning".into(), "motive".into()],
        };
        assert!(err.to_string().contains("Insufficient"));
    }

    #[test]
    fn test_defence_error() {
        let err = DefenceError::DuressUnavailable {
            offence: "murder".into(),
        };
        assert!(err.to_string().contains("Duress unavailable"));
    }

    #[test]
    fn test_sentencing_error() {
        let err = SentencingError::ExceedsMaximum {
            proposed: "15 years".into(),
            maximum: "10 years".into(),
            offence: "theft".into(),
        };
        assert!(err.to_string().contains("exceeds maximum"));
    }

    #[test]
    fn test_procedure_error() {
        let err = ProcedureError::PaceBreach {
            code: "C".into(),
            provision: "s.11".into(),
            breach: "No appropriate adult".into(),
        };
        assert!(err.to_string().contains("PACE"));
    }

    #[test]
    fn test_error_conversion() {
        let pace_err = ProcedureError::UnlawfulArrest {
            reason: "No grounds".into(),
        };
        let criminal_err: CriminalError = pace_err.into();
        assert!(matches!(criminal_err, CriminalError::Procedure(_)));
    }

    #[test]
    fn test_error_source() {
        let inner = DefenceError::DuressUnavailable {
            offence: "murder".into(),
        };
        let outer = CriminalError::Defence(inner);
        assert!(outer.source().is_some());
    }
}
