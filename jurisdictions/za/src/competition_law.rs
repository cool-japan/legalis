//! South African Competition Law
//!
//! Regulation of competition, prevention of anti-competitive practices, and merger control.
//!
//! ## Key Legislation
//!
//! - Competition Act 89 of 1998
//!
//! ## Administered by
//!
//! - Competition Commission (investigation and prosecution)
//! - Competition Tribunal (adjudication)
//! - Competition Appeal Court (appeals)
//!
//! ## Prohibited Practices
//!
//! - Horizontal agreements (price-fixing, market division, bid-rigging)
//! - Vertical agreements (resale price maintenance, exclusivity)
//! - Abuse of dominance
//! - Prohibited mergers

use crate::common::Zar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for competition operations
pub type CompetitionResult<T> = Result<T, CompetitionError>;

/// Prohibited horizontal practices (s4)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HorizontalPractice {
    /// Price-fixing (per se prohibition)
    PriceFixing,
    /// Market division (per se prohibition)
    MarketDivision,
    /// Collusive tendering/bid-rigging (per se prohibition)
    CollusiveTendering,
    /// Limiting production (rule of reason)
    LimitingProduction,
    /// Limiting market access (rule of reason)
    LimitingMarketAccess,
}

impl HorizontalPractice {
    /// Check if per se prohibited (no justification possible)
    pub fn is_per_se_prohibited(&self) -> bool {
        matches!(
            self,
            Self::PriceFixing | Self::MarketDivision | Self::CollusiveTendering
        )
    }

    /// Maximum penalty (10% of annual turnover or R10 million)
    pub fn maximum_penalty_percent_turnover(&self) -> f64 {
        10.0
    }
}

/// Prohibited vertical practices (s5)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalPractice {
    /// Minimum resale price maintenance
    MinimumResalePriceMaintenance,
    /// Vertical market division
    VerticalMarketDivision,
    /// Exclusive dealing
    ExclusiveDealing,
    /// Tying and bundling
    TyingAndBundling,
}

impl VerticalPractice {
    /// Check if prohibited or subject to rule of reason
    pub fn analysis_standard(&self) -> &'static str {
        match self {
            Self::MinimumResalePriceMaintenance => "Per se prohibited",
            _ => "Rule of reason - assess anti-competitive effects",
        }
    }
}

/// Abuse of dominance (s8)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbuseOfDominance {
    /// Excessive pricing
    ExcessivePricing,
    /// Refusing to supply scarce goods
    RefusingToSupply,
    /// Requiring exclusive dealing
    RequiringExclusiveDealing,
    /// Predatory pricing
    PredatoryPricing,
    /// Price discrimination
    PriceDiscrimination,
    /// Margin squeeze
    MarginSqueeze,
}

impl AbuseOfDominance {
    /// Dominance threshold (market share)
    pub fn dominance_threshold_percent(&self) -> f64 {
        45.0 // s7 - firm with >45% market share is presumed dominant
    }

    /// Small firm exemption (< R75m turnover)
    pub fn small_firm_exemption_threshold() -> Zar {
        Zar::from_rands(75_000_000)
    }
}

/// Merger thresholds (s11-13)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergerThresholds {
    /// Combined annual turnover or assets in SA
    pub combined_value: Zar,
    /// Target firm annual turnover or assets
    pub target_value: Zar,
}

impl MergerThresholds {
    /// Get 2024 thresholds
    pub fn thresholds_2024() -> MergerCategory {
        MergerCategory::from_values(Zar::from_rands(0), Zar::from_rands(0))
    }

    /// Determine if notification required
    pub fn requires_notification(&self) -> MergerCategory {
        MergerCategory::from_values(self.combined_value, self.target_value)
    }
}

/// Merger categories based on thresholds
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MergerCategory {
    /// Small merger (no notification required)
    Small,
    /// Intermediate merger (notification to Commission)
    Intermediate,
    /// Large merger (notification to Commission + possible Tribunal referral)
    Large,
}

impl MergerCategory {
    /// Determine category from values (2024 thresholds)
    pub fn from_values(combined: Zar, target: Zar) -> Self {
        // Large: combined ≥ R6.6bn AND target ≥ R190m
        if combined.rands() >= 6_600_000_000 && target.rands() >= 190_000_000 {
            return Self::Large;
        }

        // Intermediate: combined ≥ R600m AND target ≥ R10m
        if combined.rands() >= 600_000_000 && target.rands() >= 10_000_000 {
            return Self::Intermediate;
        }

        Self::Small
    }

    /// Filing fee (2024)
    pub fn filing_fee(&self) -> Zar {
        match self {
            Self::Small => Zar::from_rands(0),
            Self::Intermediate => Zar::from_rands(165_000),
            Self::Large => Zar::from_rands(550_000),
        }
    }

    /// Review timeline (working days)
    pub fn review_timeline_days(&self) -> u32 {
        match self {
            Self::Small => 0,
            Self::Intermediate => 40, // Initial 20 days, possible 20 day extension
            Self::Large => 60,        // Initial 40 days, possible 20 day extension
        }
    }
}

/// Merger assessment factors (s12A)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergerAssessment {
    /// Market concentration increase
    pub increases_concentration: bool,
    /// Prevents/lessens competition
    pub prevents_competition: bool,
    /// Public interest factors
    pub public_interest_concerns: Vec<PublicInterestFactor>,
}

impl MergerAssessment {
    /// Should merger be prohibited
    pub fn should_prohibit(&self) -> bool {
        self.increases_concentration && self.prevents_competition
    }

    /// Can be approved with conditions
    pub fn can_approve_with_conditions(&self) -> bool {
        !self.public_interest_concerns.is_empty()
    }
}

/// Public interest factors in merger assessment (s12A(3))
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicInterestFactor {
    /// Effect on employment
    Employment,
    /// Ability of small/medium businesses to compete
    SmallBusinessCompetitiveness,
    /// Ability of HDI firms to compete
    HdiFirmCompetitiveness,
    /// Ability of worker-owned firms to compete
    WorkerOwnedFirms,
    /// Promotion of greater spread of ownership (B-BBEE)
    SpreadOfOwnership,
}

/// Leniency policy (immunity from prosecution)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeniencyApplication {
    /// Cartel participant
    pub applicant: String,
    /// First to report
    pub first_to_report: bool,
    /// Provides full cooperation
    pub full_cooperation: bool,
    /// Ceases participation
    pub ceased_participation: bool,
}

impl LeniencyApplication {
    /// Qualifies for immunity
    pub fn qualifies_for_immunity(&self) -> bool {
        self.first_to_report && self.full_cooperation && self.ceased_participation
    }

    /// Reduction percentage (if not first)
    pub fn penalty_reduction_percent(&self) -> f64 {
        if self.qualifies_for_immunity() {
            100.0
        } else if self.full_cooperation && self.ceased_participation {
            50.0 // Second applicant
        } else {
            25.0 // Third+ applicant
        }
    }
}

/// Competition errors
#[derive(Debug, Error)]
pub enum CompetitionError {
    /// Per se prohibited practice
    #[error("Per se prohibited practice (s4): {practice}")]
    PerSeProhibited { practice: String },

    /// Abuse of dominance
    #[error("Abuse of dominance (s8): {abuse} (market share {market_share}%)")]
    AbuseOfDominance { abuse: String, market_share: f64 },

    /// Merger not notified
    #[error("Merger not notified (required for {category} merger)")]
    MergerNotNotified { category: String },

    /// Merger prohibited
    #[error("Merger substantially prevents/lessens competition")]
    MergerProhibited,

    /// Administrative penalty
    #[error("Administrative penalty: {amount_zar} (up to 10% of annual turnover)")]
    AdministrativePenalty { amount_zar: i64 },
}

/// Validate horizontal agreement
pub fn validate_horizontal_agreement(practice: &HorizontalPractice) -> CompetitionResult<()> {
    if practice.is_per_se_prohibited() {
        return Err(CompetitionError::PerSeProhibited {
            practice: format!("{:?}", practice),
        });
    }
    Ok(())
}

/// Validate merger notification
pub fn validate_merger_notification(
    thresholds: &MergerThresholds,
    notified: bool,
) -> CompetitionResult<()> {
    let category = thresholds.requires_notification();

    if !matches!(category, MergerCategory::Small) && !notified {
        return Err(CompetitionError::MergerNotNotified {
            category: format!("{:?}", category),
        });
    }

    Ok(())
}

/// Get competition compliance checklist
pub fn get_competition_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("No price-fixing agreements", "s4(1)(b)(i)"),
        ("No market division", "s4(1)(b)(ii)"),
        ("No bid-rigging/collusive tendering", "s4(1)(b)(iii)"),
        ("No resale price maintenance", "s5(1)"),
        ("Market position assessed (dominance >45%)", "s7"),
        ("No abuse of dominance", "s8"),
        ("Merger thresholds calculated", "s11-13"),
        ("Merger notification filed (if required)", "s13A"),
        ("Filing fees paid", "Regulations"),
        ("Public interest factors considered", "s12A(3)"),
        ("Leniency policy understood", "Corporate Leniency Policy"),
        ("Competition compliance program", "Best practice"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_practice_per_se() {
        assert!(HorizontalPractice::PriceFixing.is_per_se_prohibited());
        assert!(HorizontalPractice::MarketDivision.is_per_se_prohibited());
        assert!(HorizontalPractice::CollusiveTendering.is_per_se_prohibited());
        assert!(!HorizontalPractice::LimitingProduction.is_per_se_prohibited());
    }

    #[test]
    fn test_validate_horizontal_price_fixing() {
        let result = validate_horizontal_agreement(&HorizontalPractice::PriceFixing);
        assert!(result.is_err());
    }

    #[test]
    fn test_abuse_dominance_threshold() {
        let abuse = AbuseOfDominance::ExcessivePricing;
        assert_eq!(abuse.dominance_threshold_percent(), 45.0);
    }

    #[test]
    fn test_merger_category_large() {
        let category = MergerCategory::from_values(
            Zar::from_rands(7_000_000_000),
            Zar::from_rands(200_000_000),
        );
        assert_eq!(category, MergerCategory::Large);
        assert_eq!(category.filing_fee().rands(), 550_000);
        assert_eq!(category.review_timeline_days(), 60);
    }

    #[test]
    fn test_merger_category_intermediate() {
        let category =
            MergerCategory::from_values(Zar::from_rands(800_000_000), Zar::from_rands(50_000_000));
        assert_eq!(category, MergerCategory::Intermediate);
        assert_eq!(category.filing_fee().rands(), 165_000);
    }

    #[test]
    fn test_merger_category_small() {
        let category =
            MergerCategory::from_values(Zar::from_rands(100_000_000), Zar::from_rands(5_000_000));
        assert_eq!(category, MergerCategory::Small);
        assert_eq!(category.filing_fee().rands(), 0);
    }

    #[test]
    fn test_merger_notification_required() {
        let thresholds = MergerThresholds {
            combined_value: Zar::from_rands(700_000_000),
            target_value: Zar::from_rands(20_000_000),
        };

        let result = validate_merger_notification(&thresholds, false);
        assert!(result.is_err());

        let result_notified = validate_merger_notification(&thresholds, true);
        assert!(result_notified.is_ok());
    }

    #[test]
    fn test_leniency_immunity() {
        let application = LeniencyApplication {
            applicant: "Company A".to_string(),
            first_to_report: true,
            full_cooperation: true,
            ceased_participation: true,
        };
        assert!(application.qualifies_for_immunity());
        assert_eq!(application.penalty_reduction_percent(), 100.0);
    }

    #[test]
    fn test_leniency_second_applicant() {
        let application = LeniencyApplication {
            applicant: "Company B".to_string(),
            first_to_report: false,
            full_cooperation: true,
            ceased_participation: true,
        };
        assert!(!application.qualifies_for_immunity());
        assert_eq!(application.penalty_reduction_percent(), 50.0);
    }

    #[test]
    fn test_merger_assessment() {
        let assessment = MergerAssessment {
            increases_concentration: true,
            prevents_competition: true,
            public_interest_concerns: vec![PublicInterestFactor::Employment],
        };
        assert!(assessment.should_prohibit());
        assert!(assessment.can_approve_with_conditions());
    }

    #[test]
    fn test_competition_checklist() {
        let checklist = get_competition_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
