//! Consumer Rights Act 2015 Error Types

use thiserror::Error;

/// Consumer rights errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConsumerRightsError {
    /// Goods not of satisfactory quality (CRA 2015 s.9)
    #[error(
        "Goods not of satisfactory quality.\n\
         \n\
         CRA 2015 s.9: Goods must be of quality that reasonable person would regard as satisfactory.\n\
         Factors include:\n\
         • Fitness for common purposes\n\
         • Appearance and finish\n\
         • Freedom from minor defects\n\
         • Safety\n\
         • Durability\n\
         \n\
         Defect: {description}\n\
         \n\
         Remedy: {remedy}"
    )]
    GoodsNotSatisfactoryQuality { description: String, remedy: String },

    /// Goods not fit for particular purpose (CRA 2015 s.10)
    #[error(
        "Goods not fit for particular purpose.\n\
         \n\
         CRA 2015 s.10: Where consumer makes known particular purpose, goods must be fit for that purpose.\n\
         \n\
         Purpose made known: {purpose}\n\
         Reason not fit: {reason}\n\
         \n\
         Remedy: {remedy}"
    )]
    GoodsNotFitForPurpose {
        purpose: String,
        reason: String,
        remedy: String,
    },

    /// Goods not as described (CRA 2015 s.11)
    #[error(
        "Goods not as described.\n\
         \n\
         CRA 2015 s.11: Goods must match description given.\n\
         \n\
         Description: {description}\n\
         Actual: {actual}\n\
         \n\
         Remedy: {remedy}"
    )]
    GoodsNotAsDescribed {
        description: String,
        actual: String,
        remedy: String,
    },

    /// Service not performed with reasonable care and skill (CRA 2015 s.49)
    #[error(
        "Service not performed with reasonable care and skill.\n\
         \n\
         CRA 2015 s.49: Trader must perform service with reasonable care and skill.\n\
         \n\
         Service: {service}\n\
         Failure: {failure}\n\
         \n\
         Remedy: Right to require repeat performance (s.55) or price reduction (s.56)"
    )]
    ServiceNotReasonableCareAndSkill { service: String, failure: String },

    /// Service not completed in reasonable time (CRA 2015 s.52)
    #[error(
        "Service not performed within reasonable time.\n\
         \n\
         CRA 2015 s.52: Where time not agreed, service must be performed within reasonable time.\n\
         \n\
         Service: {service}\n\
         Time elapsed: {time_elapsed_days} days\n\
         \n\
         Remedy: Right to price reduction (s.56)"
    )]
    ServiceNotReasonableTime {
        service: String,
        time_elapsed_days: u32,
    },

    /// Digital content not of satisfactory quality (CRA 2015 s.34)
    #[error(
        "Digital content not of satisfactory quality.\n\
         \n\
         CRA 2015 s.34: Digital content must be of satisfactory quality.\n\
         Factors include:\n\
         • Fit for purposes for which digital content of that kind normally supplied\n\
         • Freedom from minor defects\n\
         • Other relevant circumstances\n\
         \n\
         Content: {content}\n\
         Defect: {defect}\n\
         \n\
         Remedy: Right to repair/replacement (s.43) or price reduction (s.44)"
    )]
    DigitalContentNotSatisfactoryQuality { content: String, defect: String },

    /// Unfair contract term (CRA 2015 Part 2)
    #[error(
        "Unfair contract term (not binding on consumer).\n\
         \n\
         CRA 2015 s.62: Term unfair if:\n\
         • Contrary to requirement of good faith, AND\n\
         • Causes significant imbalance in parties' rights/obligations, AND\n\
         • To detriment of consumer\n\
         \n\
         Term: \"{term}\"\n\
         \n\
         Assessment:\n\
         • Contrary to good faith: {contrary_to_good_faith}\n\
         • Significant imbalance: {significant_imbalance}\n\
         • Detriment to consumer: {detriment_to_consumer}\n\
         {grey_list}\n\
         \n\
         CRA 2015 s.62(1): Unfair term is NOT BINDING on consumer.\n\
         Rest of contract continues if possible without unfair term."
    )]
    UnfairContractTerm {
        term: String,
        contrary_to_good_faith: bool,
        significant_imbalance: bool,
        detriment_to_consumer: bool,
        grey_list: String,
    },

    /// Remedy time limit expired
    #[error(
        "Remedy time limit expired.\n\
         \n\
         Remedy: {remedy}\n\
         Deadline: {deadline}\n\
         \n\
         Note: Different remedies have different time limits under CRA 2015."
    )]
    RemedyTimeLimitExpired { remedy: String, deadline: String },

    /// Short-term right to reject expired (30 days)
    #[error(
        "Short-term right to reject expired.\n\
         \n\
         CRA 2015 s.22: Short-term right to reject available for 30 days.\n\
         \n\
         Purchase date: {purchase_date}\n\
         Days elapsed: {days_elapsed}\n\
         \n\
         Available remedies:\n\
         • Repair (s.23(2)(a))\n\
         • Replacement (s.23(2)(b))\n\
         • After failed repair/replacement: Price reduction or final rejection (s.24)"
    )]
    ShortTermRejectExpired {
        purchase_date: String,
        days_elapsed: u32,
    },

    /// Repair/replacement not possible
    #[error(
        "Repair or replacement not possible.\n\
         \n\
         CRA 2015 s.23(3): Repair/replacement must be:\n\
         • Possible (not impossible)\n\
         • Not disproportionate compared to other remedy\n\
         \n\
         Reason: {reason}\n\
         \n\
         Alternative remedy: {alternative}"
    )]
    RepairReplacementNotPossible { reason: String, alternative: String },

    /// Invalid contract type
    #[error("Invalid contract type: {0}")]
    InvalidContractType(String),

    /// Missing field
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid value
    #[error("Invalid value for {field}: {reason}")]
    InvalidValue { field: String, reason: String },
}

/// Result type for consumer rights operations
pub type Result<T> = std::result::Result<T, ConsumerRightsError>;

impl ConsumerRightsError {
    /// Create missing field error
    pub fn missing_field(field: &str) -> Self {
        Self::MissingField {
            field: field.to_string(),
        }
    }

    /// Create invalid value error
    pub fn invalid_value(field: &str, reason: &str) -> Self {
        Self::InvalidValue {
            field: field.to_string(),
            reason: reason.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ConsumerRightsError::GoodsNotSatisfactoryQuality {
            description: "Screen has dead pixels".to_string(),
            remedy: "Short-term right to reject or repair".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("CRA 2015 s.9"));
        assert!(display.contains("dead pixels"));
    }

    #[test]
    fn test_unfair_term_error() {
        let err = ConsumerRightsError::UnfairContractTerm {
            term: "Trader may change price at any time".to_string(),
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            grey_list: "• On grey list: Para 20 (price increase without exit)".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("CRA 2015 s.62"));
        assert!(display.contains("NOT BINDING"));
    }
}
