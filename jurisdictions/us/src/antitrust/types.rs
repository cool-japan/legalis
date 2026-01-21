//! Antitrust Law Types (Sherman Act, Clayton Act, FTC Act)
//!
//! This module provides types for US antitrust law.

#![allow(missing_docs)]

// Date types not currently used but reserved for future merger timeline tracking
use serde::{Deserialize, Serialize};

/// Antitrust violation type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AntitrustViolation {
    /// Sherman Act Section 1: Anticompetitive agreements
    Section1Sherman {
        violation_type: Section1ViolationType,
        parties: Vec<String>,
        market: String,
    },

    /// Sherman Act Section 2: Monopolization
    Section2Sherman {
        monopolist: String,
        market: String,
        market_share_percentage: f64,
        exclusionary_conduct: String,
    },

    /// Clayton Act Section 7: Merger/acquisition substantially lessening competition
    ClaytonAct7Merger {
        acquiring_company: String,
        target_company: String,
        relevant_market: String,
        hhi_increase: f64,
    },

    /// Robinson-Patman Act: Price discrimination
    RobinsonPatmanPriceDiscrimination {
        seller: String,
        favored_buyer: String,
        disfavored_buyer: String,
    },

    /// FTC Act Section 5: Unfair methods of competition
    FtcAct5 { conduct: String, market: String },
}

/// Sherman Act Section 1 violation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Section1ViolationType {
    /// Per se illegal: Price fixing
    PriceFixing,

    /// Per se illegal: Market allocation
    MarketAllocation,

    /// Per se illegal: Bid rigging
    BidRigging,

    /// Per se illegal: Group boycott
    GroupBoycott,

    /// Rule of reason: Vertical restraints
    VerticalRestraint { restraint_type: String },

    /// Rule of reason: Horizontal restraints
    HorizontalRestraint { restraint_type: String },
}

/// Market power analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketPower {
    /// Relevant product market
    pub product_market: String,

    /// Relevant geographic market
    pub geographic_market: String,

    /// Market share (percentage)
    pub market_share: f64,

    /// Barriers to entry
    pub barriers_to_entry: Vec<String>,

    /// Whether firm has substantial market power
    pub has_market_power: bool,
}

/// Merger analysis under Clayton Act and HSR Act
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerAnalysis {
    /// Acquiring company
    pub acquirer: String,

    /// Target company
    pub target: String,

    /// Transaction value
    pub transaction_value: f64,

    /// Relevant market definition
    pub relevant_market: String,

    /// Pre-merger HHI (Herfindahl-Hirschman Index)
    pub pre_merger_hhi: f64,

    /// Post-merger HHI
    pub post_merger_hhi: f64,

    /// Change in HHI
    pub hhi_delta: f64,

    /// Concentration level
    pub concentration_level: ConcentrationLevel,

    /// Whether HSR filing required
    pub hsr_filing_required: bool,

    /// Competitive effects
    pub competitive_effects: Vec<CompetitiveEffect>,

    /// Efficiencies claimed
    pub efficiencies: Vec<String>,

    /// Likely to substantially lessen competition
    pub likely_anticompetitive: bool,
}

impl MergerAnalysis {
    /// Determine concentration level based on HHI
    pub fn determine_concentration(&self) -> ConcentrationLevel {
        if self.post_merger_hhi < 1500.0 {
            ConcentrationLevel::Unconcentrated
        } else if self.post_merger_hhi < 2500.0 {
            ConcentrationLevel::ModeratelyConcentrated
        } else {
            ConcentrationLevel::HighlyConcentrated
        }
    }

    /// Assess competitive concern level
    pub fn competitive_concern_level(&self) -> CompetitiveConcern {
        match self.determine_concentration() {
            ConcentrationLevel::Unconcentrated => CompetitiveConcern::Low,
            ConcentrationLevel::ModeratelyConcentrated => {
                if self.hhi_delta > 100.0 {
                    CompetitiveConcern::Moderate
                } else {
                    CompetitiveConcern::Low
                }
            }
            ConcentrationLevel::HighlyConcentrated => {
                if self.hhi_delta > 200.0 {
                    CompetitiveConcern::High
                } else if self.hhi_delta > 100.0 {
                    CompetitiveConcern::Moderate
                } else {
                    CompetitiveConcern::Low
                }
            }
        }
    }
}

/// Market concentration level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcentrationLevel {
    /// HHI < 1500: Unconcentrated market
    Unconcentrated,

    /// 1500 ≤ HHI < 2500: Moderately concentrated
    ModeratelyConcentrated,

    /// HHI ≥ 2500: Highly concentrated
    HighlyConcentrated,
}

/// Competitive concern level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompetitiveConcern {
    /// Unlikely to raise competitive concerns
    Low,

    /// Potentially raises significant competitive concerns
    Moderate,

    /// Presumed likely to enhance market power
    High,
}

/// Competitive effect of merger
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompetitiveEffect {
    /// Unilateral effects (loss of competition between merging parties)
    UnilateralEffects,

    /// Coordinated effects (increased likelihood of coordination/collusion)
    CoordinatedEffects,

    /// Vertical foreclosure
    VerticalForeclosure,

    /// Elimination of potential competition
    EliminationOfPotentialCompetition,
}

/// HSR (Hart-Scott-Rodino) filing requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HsrFiling {
    /// Whether filing is required
    pub filing_required: bool,

    /// Size of transaction threshold met
    pub transaction_size: f64,

    /// Size of persons threshold met
    pub size_of_persons_met: bool,

    /// Filing fee tier
    pub filing_fee: f64,

    /// Waiting period (15 or 30 days)
    pub waiting_period_days: u32,

    /// Early termination requested
    pub early_termination_requested: bool,

    /// Second request issued
    pub second_request_issued: bool,
}

impl HsrFiling {
    /// Determine if HSR filing required (simplified 2024 thresholds)
    pub fn is_required(transaction_value: f64, acquirer_size: f64, target_size: f64) -> bool {
        // Size of transaction test
        if transaction_value >= 111_400_000.0 {
            return true;
        }

        // Size of transaction and size of persons test
        if transaction_value >= 445_500_000.0
            && ((acquirer_size >= 222_700_000.0 && target_size >= 22_300_000.0)
                || (acquirer_size >= 22_300_000.0 && target_size >= 222_700_000.0))
        {
            return true;
        }

        false
    }

    /// Calculate filing fee tier
    pub fn calculate_filing_fee(transaction_value: f64) -> f64 {
        if transaction_value < 161_500_000.0 {
            45_000.0
        } else if transaction_value < 500_000_000.0 {
            125_000.0
        } else if transaction_value < 1_000_000_000.0 {
            280_000.0
        } else if transaction_value < 2_000_000_000.0 {
            400_000.0
        } else if transaction_value < 5_000_000_000.0 {
            800_000.0
        } else {
            2_250_000.0
        }
    }
}

/// Monopoly power analysis (Sherman Act Section 2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonopolyAnalysis {
    /// Alleged monopolist
    pub monopolist: String,

    /// Relevant market
    pub relevant_market: String,

    /// Market share
    pub market_share: f64,

    /// Possession of monopoly power
    pub has_monopoly_power: bool,

    /// Exclusionary conduct
    pub exclusionary_conduct: Vec<ExclusionaryConduct>,

    /// Willful acquisition or maintenance
    pub willful_maintenance: bool,

    /// Procompetitive justifications
    pub procompetitive_justifications: Vec<String>,

    /// Violation established
    pub violation: bool,
}

/// Types of exclusionary conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExclusionaryConduct {
    /// Predatory pricing (below cost to drive out competitors)
    PredatoryPricing,

    /// Refusal to deal / Essential facilities
    RefusalToDeal { facility: String },

    /// Tying arrangement
    Tying {
        tying_product: String,
        tied_product: String,
    },

    /// Exclusive dealing
    ExclusiveDealing,

    /// Vertical integration to foreclose rivals
    VerticalForeclosure,

    /// Raising rivals' costs
    RaisingRivalsCosts,

    /// Other exclusionary conduct
    Other { description: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merger_concentration() {
        let merger = MergerAnalysis {
            acquirer: "Company A".to_string(),
            target: "Company B".to_string(),
            transaction_value: 1_000_000_000.0,
            relevant_market: "Widget Market".to_string(),
            pre_merger_hhi: 2000.0,
            post_merger_hhi: 2800.0,
            hhi_delta: 800.0,
            concentration_level: ConcentrationLevel::HighlyConcentrated,
            hsr_filing_required: true,
            competitive_effects: vec![],
            efficiencies: vec![],
            likely_anticompetitive: true,
        };

        assert_eq!(
            merger.determine_concentration(),
            ConcentrationLevel::HighlyConcentrated
        );
        assert_eq!(merger.competitive_concern_level(), CompetitiveConcern::High);
    }

    #[test]
    fn test_hsr_filing_required() {
        // Large transaction - filing required
        assert!(HsrFiling::is_required(
            200_000_000.0,
            500_000_000.0,
            100_000_000.0
        ));

        // Small transaction - no filing required
        assert!(!HsrFiling::is_required(
            50_000_000.0,
            500_000_000.0,
            100_000_000.0
        ));
    }

    #[test]
    fn test_hsr_filing_fee() {
        assert_eq!(HsrFiling::calculate_filing_fee(100_000_000.0), 45_000.0);
        assert_eq!(HsrFiling::calculate_filing_fee(300_000_000.0), 125_000.0);
        assert_eq!(
            HsrFiling::calculate_filing_fee(6_000_000_000.0),
            2_250_000.0
        );
    }
}
