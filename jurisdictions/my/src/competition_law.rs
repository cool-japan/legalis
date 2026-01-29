//! Competition Act 2010
//!
//! Malaysian competition law prohibiting anti-competitive practices.
//!
//! # Key Prohibitions
//!
//! - **Section 4**: Anti-competitive agreements (horizontal and vertical)
//! - **Section 10**: Abuse of dominant position
//! - **Section 11**: Merger control
//!
//! # Administration
//!
//! - **MyCC**: Malaysia Competition Commission

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Competition law error types.
#[derive(Debug, Error)]
pub enum CompetitionError {
    /// Anti-competitive agreement detected.
    #[error("Anti-competitive agreement: {description}")]
    AntiCompetitiveAgreement { description: String },

    /// Abuse of dominant position.
    #[error("Abuse of dominant position: {description}")]
    AbuseDominantPosition { description: String },

    /// Merger raises competition concerns.
    #[error("Merger raises competition concerns: {description}")]
    MergerConcerns { description: String },
}

/// Result type for competition law operations.
pub type Result<T> = std::result::Result<T, CompetitionError>;

/// Type of anti-competitive agreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementType {
    /// Horizontal agreement (between competitors).
    Horizontal,
    /// Vertical agreement (along supply chain).
    Vertical,
}

/// Anti-competitive practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiCompetitivePractice {
    /// Price fixing.
    PriceFixing,
    /// Market sharing/allocation.
    MarketSharing,
    /// Bid rigging.
    BidRigging,
    /// Output limitation.
    OutputLimitation,
    /// Exclusive dealing.
    ExclusiveDealing,
    /// Resale price maintenance.
    ResalePriceMaintenance,
}

/// Market position assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketPosition {
    /// Entity ID.
    pub entity_id: Uuid,
    /// Entity name.
    pub entity_name: String,
    /// Market share (percentage).
    pub market_share: f64,
    /// Whether entity has dominant position (>50% market share or significant market power).
    pub dominant: bool,
}

impl MarketPosition {
    /// Creates a new market position assessment.
    #[must_use]
    pub fn new(entity_name: impl Into<String>, market_share: f64) -> Self {
        let dominant = market_share > 50.0;
        Self {
            entity_id: Uuid::new_v4(),
            entity_name: entity_name.into(),
            market_share,
            dominant,
        }
    }

    /// Checks if entity has dominant position.
    #[must_use]
    pub fn is_dominant(&self) -> bool {
        self.dominant
    }
}

/// Merger notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerNotification {
    /// Notification ID.
    pub id: Uuid,
    /// Acquiring entity.
    pub acquirer: String,
    /// Target entity.
    pub target: String,
    /// Combined market share after merger.
    pub combined_market_share: f64,
    /// Whether merger exceeds notification threshold.
    pub exceeds_threshold: bool,
}

impl MergerNotification {
    /// Creates a new merger notification.
    #[must_use]
    pub fn new(
        acquirer: impl Into<String>,
        target: impl Into<String>,
        combined_market_share: f64,
    ) -> Self {
        // Threshold: Combined market share > 20% or creates dominant position
        let exceeds_threshold = combined_market_share > 20.0;

        Self {
            id: Uuid::new_v4(),
            acquirer: acquirer.into(),
            target: target.into(),
            combined_market_share,
            exceeds_threshold,
        }
    }

    /// Checks if MyCC notification is required.
    #[must_use]
    pub fn requires_notification(&self) -> bool {
        self.exceeds_threshold
    }
}

/// Validates whether an agreement is anti-competitive.
pub fn assess_agreement(
    agreement_type: AgreementType,
    practice: AntiCompetitivePractice,
) -> Result<Assessment> {
    let mut concerns = Vec::new();

    // Horizontal agreements are generally more serious
    if agreement_type == AgreementType::Horizontal {
        match practice {
            AntiCompetitivePractice::PriceFixing
            | AntiCompetitivePractice::MarketSharing
            | AntiCompetitivePractice::BidRigging
            | AntiCompetitivePractice::OutputLimitation => {
                concerns.push(format!(
                    "Hardcore cartel activity: {:?} is per se prohibited under Section 4",
                    practice
                ));
            }
            _ => {}
        }
    }

    // Vertical agreements may be permissible under certain conditions
    if agreement_type == AgreementType::Vertical {
        match practice {
            AntiCompetitivePractice::ResalePriceMaintenance => {
                concerns.push("Resale price maintenance may violate Section 4".to_string());
            }
            AntiCompetitivePractice::ExclusiveDealing => {
                concerns.push(
                    "Exclusive dealing may be anti-competitive depending on market power"
                        .to_string(),
                );
            }
            _ => {}
        }
    }

    Ok(Assessment {
        compliant: concerns.is_empty(),
        concerns,
    })
}

/// Assessment result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    /// Whether practice is compliant.
    pub compliant: bool,
    /// Concerns identified.
    pub concerns: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dominant_position() {
        let position = MarketPosition::new("Tech Giant Sdn Bhd", 60.0);
        assert!(position.is_dominant());

        let position2 = MarketPosition::new("Small Player Sdn Bhd", 15.0);
        assert!(!position2.is_dominant());
    }

    #[test]
    fn test_merger_notification() {
        let merger = MergerNotification::new("Acquirer Bhd", "Target Sdn Bhd", 35.0);
        assert!(merger.requires_notification());

        let merger2 = MergerNotification::new("Small Co", "Tiny Co", 5.0);
        assert!(!merger2.requires_notification());
    }

    #[test]
    fn test_horizontal_price_fixing() {
        let assessment = assess_agreement(
            AgreementType::Horizontal,
            AntiCompetitivePractice::PriceFixing,
        )
        .expect("Assessment succeeds");

        assert!(!assessment.compliant);
        assert!(!assessment.concerns.is_empty());
    }
}
