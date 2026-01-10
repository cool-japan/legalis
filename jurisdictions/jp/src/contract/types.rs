//! Core types for Japanese contract law (契約法 - Book 3: Claims/債権)
//!
//! This module defines the fundamental types used in Article 415 breach of obligation claims.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of obligation under a contract (債務の種類)
///
/// Different types of contractual obligations that can be breached.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObligationType {
    /// Monetary obligation (金銭債務)
    Monetary {
        /// Amount owed (債務額)
        amount: u64,
        /// Currency (通貨)
        currency: String,
    },

    /// Delivery obligation (給付債務・引渡債務)
    Delivery {
        /// Description of what must be delivered (給付物の説明)
        description: String,
    },

    /// Service obligation (役務債務)
    Service {
        /// Description of service to be performed (役務の説明)
        description: String,
        /// Duration of service if applicable (期間)
        duration: Option<String>,
    },

    /// Other obligation
    Other(String),
}

/// Type of breach/non-performance (不履行の種類)
///
/// Categories of contract breach under Japanese law.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BreachType {
    /// Complete non-performance (不履行)
    NonPerformance,

    /// Delayed performance (履行遅滞)
    DelayedPerformance {
        /// Number of days late (遅延日数)
        days_late: u32,
    },

    /// Defective/incomplete performance (不完全履行)
    DefectivePerformance {
        /// Description of defect (瑕疵の説明)
        description: String,
    },
}

/// Attribution type for breach (帰責事由の種類)
///
/// Basis for holding the debtor liable for breach.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AttributionType {
    /// Intentional breach (故意)
    Intentional,

    /// Negligent breach (過失)
    Negligence,

    /// Strict liability - no fault required (無過失責任)
    StrictLiability,
}

/// Attribution for breach of obligation (帰責事由)
///
/// The reason why the debtor is held liable for the breach.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attribution {
    /// Type of attribution (帰責事由の種類)
    pub attribution_type: AttributionType,

    /// Explanation of attribution (説明)
    pub explanation: String,
}

impl Attribution {
    /// Create a new attribution
    pub fn new(attribution_type: AttributionType, explanation: impl Into<String>) -> Self {
        Self {
            attribution_type,
            explanation: explanation.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligation_type_variants() {
        let monetary = ObligationType::Monetary {
            amount: 1_000_000,
            currency: "JPY".to_string(),
        };
        assert!(matches!(monetary, ObligationType::Monetary { .. }));

        let delivery = ObligationType::Delivery {
            description: "商品の引渡し".to_string(),
        };
        assert!(matches!(delivery, ObligationType::Delivery { .. }));
    }

    #[test]
    fn test_breach_type_variants() {
        let non_perf = BreachType::NonPerformance;
        assert!(matches!(non_perf, BreachType::NonPerformance));

        let delayed = BreachType::DelayedPerformance { days_late: 30 };
        assert!(matches!(delayed, BreachType::DelayedPerformance { .. }));
    }

    #[test]
    fn test_attribution_creation() {
        let attr = Attribution::new(AttributionType::Negligence, "正当な理由なく履行を拒否");
        assert_eq!(attr.attribution_type, AttributionType::Negligence);
        assert_eq!(attr.explanation, "正当な理由なく履行を拒否");
    }
}
