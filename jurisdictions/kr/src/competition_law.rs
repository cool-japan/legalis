//! Competition Law (공정거래법)
//!
//! # 독점규제 및 공정거래에 관한 법률 / Monopoly Regulation and Fair Trade Act
//!
//! Enacted: 1980
//! Enforced by: Korea Fair Trade Commission (KFTC)

use crate::common::KrwAmount;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Competition law errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CompetitionLawError {
    /// Anti-competitive conduct
    #[error("Anti-competitive conduct: {0}")]
    AntiCompetitive(String),

    /// Merger review violation
    #[error("Merger review violation: {0}")]
    MergerViolation(String),
}

/// Result type for competition law operations
pub type CompetitionLawResult<T> = Result<T, CompetitionLawError>;

/// Merger filing threshold
/// Combined turnover: 300B KRW
/// Target turnover: 30B KRW
pub fn merger_filing_threshold() -> (KrwAmount, KrwAmount) {
    (
        KrwAmount::from_eok(3_000.0), // 300B KRW
        KrwAmount::from_eok(300.0),   // 30B KRW
    )
}

/// Check if merger filing is required
pub fn requires_merger_filing(
    acquiring_party_turnover: &KrwAmount,
    target_turnover: &KrwAmount,
) -> bool {
    let (threshold_combined, threshold_target) = merger_filing_threshold();

    acquiring_party_turnover.won >= threshold_combined.won
        && target_turnover.won >= threshold_target.won
}

/// Abuse of market dominance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseType {
    /// Refusal to deal (거래 거절)
    RefusalToDeal,
    /// Discriminatory treatment (차별 대우)
    DiscriminatoryTreatment,
    /// Predatory pricing (부당 염매)
    PredatoryPricing,
    /// Tying (끼워팔기)
    Tying,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merger_filing_threshold() {
        let (combined, target) = merger_filing_threshold();
        assert!((combined.won - 300_000_000_000.0).abs() < 0.01);
        assert!((target.won - 30_000_000_000.0).abs() < 0.01);
    }

    #[test]
    fn test_requires_merger_filing() {
        let acquiring = KrwAmount::from_eok(5_000.0);
        let target = KrwAmount::from_eok(500.0);

        assert!(requires_merger_filing(&acquiring, &target));
    }

    #[test]
    fn test_requires_merger_filing_below_threshold() {
        let acquiring = KrwAmount::from_eok(1_000.0);
        let target = KrwAmount::from_eok(10.0);

        assert!(!requires_merger_filing(&acquiring, &target));
    }
}
