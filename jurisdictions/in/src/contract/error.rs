//! Indian Contract Act 1872 Error Types
//!
//! Error types for contract law compliance

use crate::citation::{Citation, cite};
use thiserror::Error;

/// Indian Contract Act errors
#[derive(Debug, Clone, Error)]
pub enum ContractActError {
    // Formation errors
    /// Contract void ab initio (Section 10)
    #[error("Contract Act Section 10: Contract void - essential element missing: {reason}")]
    VoidContract { reason: String },

    /// Agreement without consideration (Section 25)
    #[error("Contract Act Section 25: Agreement without consideration is void")]
    NoConsideration,

    /// Incompetent party (Section 11)
    #[error("Contract Act Section 11: Party incompetent to contract: {reason}")]
    IncompetentParty { reason: String },

    /// Minor's agreement (Mohori Bibee principle)
    #[error("Contract Act Section 11: Agreement with minor is void ab initio")]
    MinorAgreement,

    /// Person of unsound mind (Section 12)
    #[error("Contract Act Section 12: Party not of sound mind when contracting")]
    UnsoundMind,

    // Consent errors
    /// Consent obtained by coercion (Section 15)
    #[error("Contract Act Section 15: Consent obtained by coercion - contract voidable")]
    Coercion,

    /// Consent obtained by undue influence (Section 16)
    #[error("Contract Act Section 16: Consent obtained by undue influence - contract voidable")]
    UndueInfluence,

    /// Consent obtained by fraud (Section 17)
    #[error("Contract Act Section 17: Consent obtained by fraud - contract voidable")]
    Fraud,

    /// Consent obtained by misrepresentation (Section 18)
    #[error("Contract Act Section 18: Consent obtained by misrepresentation - contract voidable")]
    Misrepresentation,

    /// Bilateral mistake of fact (Section 20)
    #[error("Contract Act Section 20: Bilateral mistake as to matter of fact - contract void")]
    BilateralMistake,

    /// Free consent not obtained (Section 14)
    #[error("Contract Act Section 14: Free consent not obtained")]
    NoFreeConsent,

    // Legality errors
    /// Unlawful consideration (Section 23)
    #[error("Contract Act Section 23: Consideration unlawful: {reason}")]
    UnlawfulConsideration { reason: String },

    /// Unlawful object (Section 23)
    #[error("Contract Act Section 23: Object unlawful: {reason}")]
    UnlawfulObject { reason: String },

    /// Agreement in restraint of trade (Section 27)
    #[error("Contract Act Section 27: Agreement in restraint of trade is void")]
    RestraintOfTrade,

    /// Agreement in restraint of legal proceedings (Section 28)
    #[error("Contract Act Section 28: Agreement in restraint of legal proceedings is void")]
    RestraintOfLegalProceedings,

    /// Agreement in restraint of marriage (Section 26)
    #[error("Contract Act Section 26: Agreement in restraint of marriage is void")]
    RestraintOfMarriage,

    /// Wagering agreement (Section 30)
    #[error("Contract Act Section 30: Wagering agreements are void")]
    WageringAgreement,

    /// Uncertain agreement (Section 29)
    #[error("Contract Act Section 29: Agreement uncertain - meaning not ascertainable")]
    UncertainAgreement,

    // Performance errors
    /// Impossibility of performance (Section 56)
    #[error("Contract Act Section 56: Contract becomes impossible to perform")]
    ImpossibilityOfPerformance { reason: String },

    /// Failure to perform (Section 37)
    #[error("Contract Act Section 37: Promisor failed to perform obligation")]
    FailureToPerform,

    /// Time as essence (Section 55)
    #[error("Contract Act Section 55: Performance not made within specified time")]
    TimeNotKept,

    // Breach errors
    /// Breach of contract (Section 73)
    #[error("Contract Act Section 73: Breach of contract")]
    Breach { breach_type: String },

    /// Anticipatory breach (Section 39)
    #[error("Contract Act Section 39: Anticipatory breach - refusal to perform")]
    AnticipatoryBreach,

    // Agency errors
    /// Agent exceeded authority (Section 227)
    #[error("Contract Act Section 227: Agent acted beyond scope of authority")]
    AgentExceededAuthority,

    /// Unauthorized act not ratified (Section 196)
    #[error("Contract Act Section 196: Unauthorized act not ratified by principal")]
    NotRatified,

    /// Agent's duty breach (Section 211)
    #[error("Contract Act Section 211: Agent breached duty to principal")]
    AgentDutyBreach,

    // Contingent contract errors
    /// Contingent event impossible (Section 36)
    #[error("Contract Act Section 36: Contingent event becomes impossible")]
    ContingentEventImpossible,

    // Quasi-contract errors
    /// Unjust enrichment
    #[error("Contract Act: Unjust enrichment - party must compensate")]
    UnjustEnrichment,

    // General errors
    /// Validation error
    #[error("Contract Act validation error: {message}")]
    ValidationError { message: String },
}

impl ContractActError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::VoidContract { .. } => Some(cite::contract_act(10)),
            Self::NoConsideration => Some(cite::contract_act(25)),
            Self::IncompetentParty { .. } | Self::MinorAgreement => Some(cite::contract_act(11)),
            Self::UnsoundMind => Some(cite::contract_act(12)),
            Self::NoFreeConsent => Some(cite::contract_act(14)),
            Self::Coercion => Some(cite::contract_act(15)),
            Self::UndueInfluence => Some(cite::contract_act(16)),
            Self::Fraud => Some(cite::contract_act(17)),
            Self::Misrepresentation => Some(cite::contract_act(18)),
            Self::BilateralMistake => Some(cite::contract_act(20)),
            Self::UnlawfulConsideration { .. } | Self::UnlawfulObject { .. } => {
                Some(cite::contract_act(23))
            }
            Self::RestraintOfTrade => Some(cite::contract_act(27)),
            Self::RestraintOfLegalProceedings => Some(cite::contract_act(28)),
            Self::RestraintOfMarriage => Some(cite::contract_act(26)),
            Self::WageringAgreement => Some(cite::contract_act(30)),
            Self::UncertainAgreement => Some(cite::contract_act(29)),
            Self::ImpossibilityOfPerformance { .. } => Some(cite::contract_act(56)),
            Self::FailureToPerform => Some(cite::contract_act(37)),
            Self::TimeNotKept => Some(cite::contract_act(55)),
            Self::Breach { .. } => Some(cite::contract_act(73)),
            Self::AnticipatoryBreach => Some(cite::contract_act(39)),
            Self::AgentExceededAuthority => Some(cite::contract_act(227)),
            Self::NotRatified => Some(cite::contract_act(196)),
            Self::AgentDutyBreach => Some(cite::contract_act(211)),
            Self::ContingentEventImpossible => Some(cite::contract_act(36)),
            Self::UnjustEnrichment | Self::ValidationError { .. } => None,
        }
    }

    /// Get contract effect
    pub fn contract_effect(&self) -> &'static str {
        match self {
            Self::VoidContract { .. }
            | Self::NoConsideration
            | Self::MinorAgreement
            | Self::UnsoundMind
            | Self::BilateralMistake
            | Self::RestraintOfTrade
            | Self::RestraintOfLegalProceedings
            | Self::RestraintOfMarriage
            | Self::WageringAgreement
            | Self::UncertainAgreement
            | Self::ContingentEventImpossible => "Void",
            Self::Coercion
            | Self::UndueInfluence
            | Self::Fraud
            | Self::Misrepresentation
            | Self::NoFreeConsent => "Voidable at option of aggrieved party",
            Self::ImpossibilityOfPerformance { .. } => "Discharged",
            Self::Breach { .. } | Self::AnticipatoryBreach => "Breach - remedies available",
            _ => "Depends on circumstances",
        }
    }

    /// Get available remedy
    pub fn available_remedies(&self) -> Vec<&'static str> {
        match self {
            Self::Coercion | Self::UndueInfluence | Self::Fraud | Self::Misrepresentation => {
                vec!["Rescission of contract", "Damages"]
            }
            Self::Breach { .. } | Self::AnticipatoryBreach => {
                vec![
                    "Damages",
                    "Specific performance",
                    "Injunction",
                    "Quantum meruit",
                ]
            }
            Self::ImpossibilityOfPerformance { .. } => {
                vec!["Restoration of benefit received (Section 65)"]
            }
            Self::MinorAgreement => {
                vec!["Restitution of benefit to minor (if applicable)"]
            }
            _ => vec!["Depends on circumstances"],
        }
    }
}

/// Result type for Contract Act operations
pub type ContractActResult<T> = Result<T, ContractActError>;

/// Contract validity assessment
#[derive(Debug, Clone)]
pub struct ContractValidityReport {
    /// Overall validity
    pub valid: bool,
    /// Contract status
    pub status: &'static str,
    /// Issues found
    pub issues: Vec<ContractActError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl Default for ContractValidityReport {
    fn default() -> Self {
        Self {
            valid: true,
            status: "Valid",
            issues: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citations() {
        let error = ContractActError::Coercion;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 15);

        let error = ContractActError::NoConsideration;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 25);
    }

    #[test]
    fn test_contract_effect() {
        assert_eq!(ContractActError::MinorAgreement.contract_effect(), "Void");
        assert_eq!(
            ContractActError::Coercion.contract_effect(),
            "Voidable at option of aggrieved party"
        );
    }

    #[test]
    fn test_available_remedies() {
        let remedies = ContractActError::Fraud.available_remedies();
        assert!(remedies.contains(&"Rescission of contract"));
        assert!(remedies.contains(&"Damages"));
    }

    #[test]
    fn test_breach_remedies() {
        let error = ContractActError::Breach {
            breach_type: "Non-performance".to_string(),
        };
        let remedies = error.available_remedies();
        assert!(remedies.contains(&"Damages"));
        assert!(remedies.contains(&"Specific performance"));
    }
}
