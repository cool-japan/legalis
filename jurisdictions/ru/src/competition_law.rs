//! Protection of Competition Law (Federal Law 135-FZ).
//!
//! Federal Law No. 135-FZ of July 26, 2006
//! "On Protection of Competition" (О защите конкуренции)
//!
//! This module provides:
//! - Monopoly and dominant position rules
//! - Antitrust enforcement
//! - Merger control
//! - Unfair competition prohibition

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to competition law operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CompetitionError {
    /// Monopolistic practice violation
    #[error("Monopolistic practice violation: {0}")]
    MonopolyViolation(String),

    /// Dominant position abuse
    #[error("Dominant position abuse: {0}")]
    DominantPositionAbuse(String),

    /// Unfair competition
    #[error("Unfair competition: {0}")]
    UnfairCompetition(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Market share representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketShare {
    /// Company name
    pub company_name: String,
    /// Market share as percentage
    pub share_percentage: f64,
    /// Relevant market definition
    pub relevant_market: String,
    /// Revenue in relevant market
    pub revenue: crate::common::Currency,
}

impl MarketShare {
    /// Creates a new market share
    pub fn new(
        company_name: impl Into<String>,
        share_percentage: f64,
        relevant_market: impl Into<String>,
        revenue: crate::common::Currency,
    ) -> Self {
        Self {
            company_name: company_name.into(),
            share_percentage,
            relevant_market: relevant_market.into(),
            revenue,
        }
    }

    /// Checks if this market share constitutes dominant position
    pub fn is_dominant_position(&self) -> bool {
        // Article 5: Dominant position presumed if share > 50%
        // Can be dominant if share > 35% depending on market structure
        self.share_percentage > 50.0
    }

    /// Checks if this qualifies for simplified merger review
    pub fn qualifies_for_simplified_review(&self) -> bool {
        // Simplified if share < 35%
        self.share_percentage < 35.0
    }
}

/// Dominant position representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominantPosition {
    /// Company with dominant position
    pub company: String,
    /// Market shares
    pub market_shares: Vec<MarketShare>,
    /// Barriers to entry
    pub barriers_to_entry: Vec<String>,
}

impl DominantPosition {
    /// Creates a new dominant position
    pub fn new(company: impl Into<String>) -> Self {
        Self {
            company: company.into(),
            market_shares: Vec::new(),
            barriers_to_entry: Vec::new(),
        }
    }

    /// Adds a market share
    pub fn add_market_share(mut self, share: MarketShare) -> Self {
        self.market_shares.push(share);
        self
    }

    /// Adds a barrier to entry
    pub fn add_barrier(mut self, barrier: impl Into<String>) -> Self {
        self.barriers_to_entry.push(barrier.into());
        self
    }

    /// Validates if dominant position exists
    pub fn validate(&self) -> Result<(), CompetitionError> {
        if self.market_shares.is_empty() {
            return Err(CompetitionError::ValidationFailed(
                "Must specify at least one market share".to_string(),
            ));
        }

        // Check if any market share indicates dominance
        let has_dominant_share = self.market_shares.iter().any(|s| s.is_dominant_position());

        if !has_dominant_share {
            return Err(CompetitionError::ValidationFailed(
                "No market share indicates dominant position".to_string(),
            ));
        }

        Ok(())
    }
}

/// Monopoly representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monopoly {
    /// Monopolist company
    pub company: String,
    /// Market share (typically > 65%)
    pub market_share_percentage: f64,
    /// Type of monopoly
    pub monopoly_type: MonopolyType,
    /// Is state-regulated
    pub state_regulated: bool,
}

/// Types of monopolies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonopolyType {
    /// Natural monopoly (gas, electricity, railways)
    Natural,
    /// Economic monopoly
    Economic,
    /// State monopoly
    State,
}

impl Monopoly {
    /// Creates a new monopoly
    pub fn new(company: impl Into<String>, market_share: f64, monopoly_type: MonopolyType) -> Self {
        Self {
            company: company.into(),
            market_share_percentage: market_share,
            monopoly_type,
            state_regulated: false,
        }
    }

    /// Sets state regulation status
    pub fn state_regulated(mut self) -> Self {
        self.state_regulated = true;
        self
    }

    /// Checks if this is a natural monopoly requiring special regulation
    pub fn requires_special_regulation(&self) -> bool {
        matches!(self.monopoly_type, MonopolyType::Natural)
    }
}

/// Competition violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionViolation {
    /// Company committing violation
    pub company: String,
    /// Type of violation
    pub violation_type: ViolationType,
    /// Description
    pub description: String,
    /// Affected market
    pub affected_market: String,
    /// Estimated damage
    pub estimated_damage: Option<crate::common::Currency>,
}

/// Types of competition violations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    /// Abuse of dominant position (Article 10)
    AbuseDominantPosition,
    /// Price fixing cartel (Article 11)
    PriceFixing,
    /// Market division cartel (Article 11)
    MarketDivision,
    /// Bid rigging (Article 11)
    BidRigging,
    /// Unfair competition (Article 14)
    UnfairCompetition,
    /// Illegal merger (Article 27-32)
    IllegalMerger,
}

impl CompetitionViolation {
    /// Creates a new competition violation
    pub fn new(
        company: impl Into<String>,
        violation_type: ViolationType,
        description: impl Into<String>,
        affected_market: impl Into<String>,
    ) -> Self {
        Self {
            company: company.into(),
            violation_type,
            description: description.into(),
            affected_market: affected_market.into(),
            estimated_damage: None,
        }
    }

    /// Sets estimated damage
    pub fn with_damage(mut self, damage: crate::common::Currency) -> Self {
        self.estimated_damage = Some(damage);
        self
    }

    /// Gets penalty range based on violation type
    pub fn penalty_range(&self) -> (f64, f64) {
        match self.violation_type {
            ViolationType::AbuseDominantPosition => (1.0, 15.0), // 1-15% of revenue
            ViolationType::PriceFixing | ViolationType::MarketDivision => (3.0, 15.0), // 3-15%
            ViolationType::BidRigging => (3.0, 15.0),
            ViolationType::UnfairCompetition => (1.0, 5.0), // 1-5%
            ViolationType::IllegalMerger => (1.0, 10.0),
        }
    }
}

/// Article 10: Prohibited actions by dominant position holders
pub fn check_dominant_position_abuse(
    market_share: f64,
    action_type: DominantPositionAction,
) -> Result<(), CompetitionError> {
    if market_share < 35.0 {
        return Ok(()); // Not dominant
    }

    // Certain actions are prohibited for dominant companies
    match action_type {
        DominantPositionAction::ExcessivePricing => Err(CompetitionError::DominantPositionAbuse(
            "Excessive pricing by dominant company is prohibited".to_string(),
        )),
        DominantPositionAction::RefusalToDeal => Err(CompetitionError::DominantPositionAbuse(
            "Unjustified refusal to deal is prohibited".to_string(),
        )),
        DominantPositionAction::DifferentialPricing => {
            Err(CompetitionError::DominantPositionAbuse(
                "Discriminatory pricing is prohibited".to_string(),
            ))
        }
        DominantPositionAction::RegularBusiness => Ok(()),
    }
}

/// Actions by dominant position holders
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DominantPositionAction {
    /// Excessive pricing
    ExcessivePricing,
    /// Refusal to deal
    RefusalToDeal,
    /// Differential pricing
    DifferentialPricing,
    /// Regular business activity
    RegularBusiness,
}

/// Quick validation for market dominance
pub fn quick_validate_market_dominance(
    dominant_position: &DominantPosition,
) -> Result<(), CompetitionError> {
    dominant_position.validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_share() {
        let share = MarketShare::new(
            "Компания А",
            55.0,
            "Телекоммуникации",
            crate::common::Currency::from_rubles(10_000_000),
        );

        assert!(share.is_dominant_position());
        assert!(!share.qualifies_for_simplified_review());
    }

    #[test]
    fn test_dominant_position() {
        let position = DominantPosition::new("Газпром")
            .add_market_share(MarketShare::new(
                "Газпром",
                65.0,
                "Natural Gas",
                crate::common::Currency::from_rubles(1_000_000_000),
            ))
            .add_barrier("Pipeline infrastructure");

        assert!(position.validate().is_ok());
    }

    #[test]
    fn test_monopoly() {
        let monopoly = Monopoly::new("РЖД", 100.0, MonopolyType::Natural).state_regulated();

        assert!(monopoly.requires_special_regulation());
        assert!(monopoly.state_regulated);
    }

    #[test]
    fn test_competition_violation() {
        let violation = CompetitionViolation::new(
            "Компания X",
            ViolationType::PriceFixing,
            "Cartel agreement on pricing",
            "Construction materials",
        )
        .with_damage(crate::common::Currency::from_rubles(5_000_000));

        let (min, max) = violation.penalty_range();
        assert_eq!(min, 3.0);
        assert_eq!(max, 15.0);
    }

    #[test]
    fn test_dominant_position_abuse() {
        // Dominant company with excessive pricing
        assert!(
            check_dominant_position_abuse(60.0, DominantPositionAction::ExcessivePricing).is_err()
        );

        // Non-dominant company
        assert!(
            check_dominant_position_abuse(30.0, DominantPositionAction::ExcessivePricing).is_ok()
        );

        // Dominant company with regular business
        assert!(
            check_dominant_position_abuse(60.0, DominantPositionAction::RegularBusiness).is_ok()
        );
    }
}
