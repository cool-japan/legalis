//! Competition Act 2004 - Error Types
//!
//! This module defines error types for breaches of Singapore's Competition Act 2004,
//! enforced by the Competition and Consumer Commission of Singapore (CCCS).
//!
//! All error messages are bilingual, in Singapore's primary languages:
//! - English (official business and administration language)
//! - Chinese/华语 (Chinese community, ~74% of the resident population)
//!
//! The three substantive prohibitions are:
//! - **Section 34**: anti-competitive agreements, decisions and concerted practices
//! - **Section 47**: abuse of a dominant position
//! - **Section 54**: mergers resulting in a substantial lessening of competition (SLC)
//!
//! Each error exposes a [`CompetitionError::statute_reference`] and a numeric
//! [`CompetitionError::severity`] (1-5) to support downstream triage and reporting.

use thiserror::Error;

/// Result type for Competition Act operations.
pub type Result<T> = std::result::Result<T, CompetitionError>;

/// Errors arising from the Competition Act 2004 and CCCS enforcement.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CompetitionError {
    /// Section 34 prohibition infringed - anti-competitive agreement.
    #[error(
        "Section 34 prohibition infringed: {conduct} (Competition Act s. 34)\n\
         违反第34条禁令: {conduct} (竞争法第34条)"
    )]
    Section34Infringement {
        /// Human-readable description of the offending conduct.
        conduct: String,
    },

    /// Hardcore "by object" restriction (cartel) - infringement regardless of effect.
    #[error(
        "Hardcore restriction by object: {conduct} - infringement irrespective of market share (Competition Act s. 34)\n\
         核心限制(以限制竞争为目的): {conduct} - 无论市场份额如何均构成违法 (竞争法第34条)"
    )]
    HardcoreRestriction {
        /// Description of the cartel conduct (e.g. price fixing, bid rigging).
        conduct: String,
    },

    /// Section 47 prohibition infringed - abuse of a dominant position.
    #[error(
        "Section 47 prohibition infringed: {abuse} (Competition Act s. 47)\n\
         违反第47条禁令: {abuse} (竞争法第47条)"
    )]
    Section47Abuse {
        /// Description of the abusive conduct.
        abuse: String,
    },

    /// Section 47 invoked but the undertaking is not (likely) dominant.
    #[error(
        "Abuse of dominance cannot arise: market share {market_share_percent}% does not indicate dominance (Competition Act s. 47)\n\
         不构成滥用市场支配地位: 市场份额{market_share_percent}%不足以表明支配地位 (竞争法第47条)"
    )]
    NotDominant {
        /// The undertaking's market share, as a percentage.
        market_share_percent: u8,
    },

    /// Section 54 prohibition infringed - merger leading to a substantial lessening of competition.
    #[error(
        "Merger may result in a substantial lessening of competition (Competition Act s. 54)\n\
         合并可能导致竞争大幅减少 (竞争法第54条)"
    )]
    SubstantialLesseningOfCompetition {
        /// Post-merger market share of the merged entity, as a percentage.
        merged_share_percent: u8,
    },

    /// A financial penalty exceeding the statutory cap was proposed.
    #[error(
        "Financial penalty exceeds statutory cap of 10% of turnover per year (max 3 years) (Competition Act s. 69(4))\n\
         罚款超过法定上限(每年营业额的10%,最多3年) (竞争法第69(4)条)"
    )]
    PenaltyExceedsCap {
        /// The proposed penalty, in SGD cents.
        proposed_cents: u64,
        /// The statutory maximum, in SGD cents.
        maximum_cents: u64,
    },

    /// An undertaking with no Singapore nexus was assessed.
    ///
    /// The prohibitions bite on competition *within Singapore* (ss. 34(1), 47(1), 54(1)).
    #[error(
        "Conduct has no effect on competition within Singapore (Competition Act s. 33)\n\
         行为对新加坡境内竞争没有影响 (竞争法第33条)"
    )]
    NoSingaporeNexus,

    /// Invalid market share value supplied (must be 0-100).
    #[error(
        "Invalid market share: {value}% (must be between 0 and 100)\n\
         无效的市场份额: {value}% (必须介于0至100之间)"
    )]
    InvalidMarketShare {
        /// The offending value.
        value: u16,
    },

    /// Generic validation error.
    #[error(
        "Competition Act validation error: {message}\n\
         竞争法验证错误: {message}"
    )]
    ValidationError {
        /// Free-form description of the problem.
        message: String,
    },
}

impl CompetitionError {
    /// Returns the primary statutory reference for this error, if any.
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            CompetitionError::Section34Infringement { .. } => Some("Competition Act s. 34"),
            CompetitionError::HardcoreRestriction { .. } => Some("Competition Act s. 34"),
            CompetitionError::Section47Abuse { .. } => Some("Competition Act s. 47"),
            CompetitionError::NotDominant { .. } => Some("Competition Act s. 47"),
            CompetitionError::SubstantialLesseningOfCompetition { .. } => {
                Some("Competition Act s. 54")
            }
            CompetitionError::PenaltyExceedsCap { .. } => Some("Competition Act s. 69(4)"),
            CompetitionError::NoSingaporeNexus => Some("Competition Act s. 33"),
            CompetitionError::InvalidMarketShare { .. } => None,
            CompetitionError::ValidationError { .. } => None,
        }
    }

    /// Returns the severity level (1 = informational, 5 = most serious).
    ///
    /// Hardcore cartels (s. 34 "by object") and abuse of dominance attract the
    /// highest severity, consistent with CCCS's enforcement priorities.
    pub fn severity(&self) -> u8 {
        match self {
            CompetitionError::HardcoreRestriction { .. } => 5,
            CompetitionError::Section47Abuse { .. } => 5,
            CompetitionError::Section34Infringement { .. } => 4,
            CompetitionError::SubstantialLesseningOfCompetition { .. } => 4,
            CompetitionError::PenaltyExceedsCap { .. } => 3,
            CompetitionError::NotDominant { .. } => 2,
            CompetitionError::NoSingaporeNexus => 2,
            CompetitionError::InvalidMarketShare { .. } => 1,
            CompetitionError::ValidationError { .. } => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = CompetitionError::Section34Infringement {
            conduct: "Price fixing".to_string(),
        };
        assert_eq!(error.statute_reference(), Some("Competition Act s. 34"));

        let abuse = CompetitionError::Section47Abuse {
            abuse: "Predatory pricing".to_string(),
        };
        assert_eq!(abuse.statute_reference(), Some("Competition Act s. 47"));

        let merger = CompetitionError::SubstantialLesseningOfCompetition {
            merged_share_percent: 55,
        };
        assert_eq!(merger.statute_reference(), Some("Competition Act s. 54"));
    }

    #[test]
    fn test_error_severity_ordering() {
        let hardcore = CompetitionError::HardcoreRestriction {
            conduct: "Bid rigging".to_string(),
        };
        let not_dominant = CompetitionError::NotDominant {
            market_share_percent: 20,
        };
        assert_eq!(hardcore.severity(), 5);
        assert_eq!(not_dominant.severity(), 2);
        assert!(hardcore.severity() > not_dominant.severity());
    }

    #[test]
    fn test_error_display_is_bilingual() {
        let error = CompetitionError::HardcoreRestriction {
            conduct: "Market sharing cartel".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Hardcore restriction by object"));
        assert!(display.contains("Market sharing cartel"));
        assert!(display.contains("Competition Act s. 34"));
        // Chinese rendering present.
        assert!(display.contains("核心限制"));
    }

    #[test]
    fn test_penalty_cap_error() {
        let error = CompetitionError::PenaltyExceedsCap {
            proposed_cents: 5_000_000,
            maximum_cents: 3_000_000,
        };
        assert_eq!(error.statute_reference(), Some("Competition Act s. 69(4)"));
        assert_eq!(error.severity(), 3);
    }

    #[test]
    fn test_no_singapore_nexus() {
        let error = CompetitionError::NoSingaporeNexus;
        assert_eq!(error.statute_reference(), Some("Competition Act s. 33"));
        assert_eq!(error.severity(), 2);
    }
}
