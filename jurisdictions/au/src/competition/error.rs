//! Error types for Australian Competition Law
//!
//! This module provides comprehensive error types with statutory references
//! for competition law violations and analysis failures.
//!
//! ## Statutory References
//!
//! Errors reference specific sections of the Competition and Consumer Act 2010:
//! - Part IV Division 1 (ss.45AD-45AS) - Cartel conduct
//! - Section 46 - Misuse of market power
//! - Section 47 - Exclusive dealing
//! - Section 50 - Mergers
//!
//! ## Penalty Structure
//!
//! The CCA provides for both criminal and civil penalties:
//!
//! ### Criminal Penalties (Cartel Conduct)
//! - Individuals: Up to 10 years imprisonment and/or $444,000 fine
//! - Corporations: Greater of $10M, 3x benefit, or 10% of turnover
//!
//! ### Civil Penalties
//! - Individuals: Up to $500,000 per contravention
//! - Corporations: Greater of $10M, 3x benefit, or 10% of turnover

use thiserror::Error;

use super::types::StateTerritory;

/// Result type for competition law operations
pub type Result<T> = std::result::Result<T, CompetitionError>;

/// Comprehensive error type for Australian competition law
#[derive(Debug, Error)]
pub enum CompetitionError {
    /// Cartel conduct violation (ss.45AD-45AS)
    #[error("Cartel conduct: {cartel_type:?} - CCA s.{section} - {description}")]
    CartelConduct {
        /// Type of cartel conduct
        cartel_type: CartelType,
        /// Relevant section
        section: String,
        /// Description
        description: String,
        /// Whether criminal liability applies
        criminal: bool,
    },

    /// Misuse of market power (s.46)
    #[error("Misuse of market power: CCA s.46 - {description}")]
    MisuseOfMarketPower {
        /// Description of the conduct
        description: String,
        /// Market share percentage
        market_share: Option<f64>,
        /// Effect on competition
        competition_effect: String,
    },

    /// Exclusive dealing violation (s.47)
    #[error("Exclusive dealing: CCA s.47 - {description}")]
    ExclusiveDealing {
        /// Description
        description: String,
        /// Specific subsection
        subsection: String,
        /// Effect on competition
        competition_effect: String,
    },

    /// Merger substantially lessens competition (s.50)
    #[error("Merger SLC: CCA s.50 - {description}")]
    MergerSLC {
        /// Description
        description: String,
        /// Post-merger market share
        post_merger_share: Option<f64>,
        /// Anti-competitive effects identified
        effects: Vec<String>,
    },

    /// Resale price maintenance (s.48)
    #[error("Resale price maintenance: CCA s.48 - {description}")]
    ResalePriceMaintenance {
        /// Description
        description: String,
        /// Product involved
        product: String,
    },

    /// Invalid market definition
    #[error("Invalid market definition: {reason}")]
    InvalidMarketDefinition {
        /// Reason for invalidity
        reason: String,
        /// Suggested corrections
        suggestions: Vec<String>,
    },

    /// Insufficient evidence
    #[error("Insufficient evidence for {element}: {details}")]
    InsufficientEvidence {
        /// Element lacking evidence
        element: String,
        /// Details
        details: String,
        /// Required evidence types
        required: Vec<String>,
    },

    /// Joint venture exception applies
    #[error("Joint venture exception: CCA ss.45AO-45AQ - {reason}")]
    JointVentureException {
        /// Reason exception applies
        reason: String,
    },

    /// Notification lodged (exclusive dealing)
    #[error("Notification lodged: conduct permitted unless ACCC objects within {days} days")]
    NotificationLodged {
        /// Notification number
        notification_number: String,
        /// Days until deemed allowed
        days: u32,
    },

    /// Authorisation granted
    #[error("Authorisation granted: {determination_number} - public benefit outweighs detriment")]
    AuthorisationGranted {
        /// ACCC determination number
        determination_number: String,
        /// Public benefits identified
        public_benefits: Vec<String>,
    },

    /// Clearance granted (merger)
    #[error("Merger clearance: {reason}")]
    MergerClearance {
        /// Clearance type
        clearance_type: ClearanceType,
        /// Reason for clearance
        reason: String,
    },

    /// State-specific competition matter
    #[error("State competition matter: {state:?} - {description}")]
    StateCompetitionMatter {
        /// State or territory
        state: StateTerritory,
        /// Description
        description: String,
        /// Relevant state legislation
        state_legislation: Option<String>,
    },

    /// Analysis error
    #[error("Competition analysis error: {message}")]
    AnalysisError {
        /// Error message
        message: String,
        /// Additional context
        context: Option<String>,
    },

    /// Data validation error
    #[error("Data validation error: {field} - {message}")]
    ValidationError {
        /// Field with error
        field: String,
        /// Error message
        message: String,
    },
}

/// Type of cartel conduct
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CartelType {
    /// Price fixing (s.45AD(1))
    PriceFixing,
    /// Output restriction (s.45AD(2))
    OutputRestriction,
    /// Market allocation (s.45AD(3))
    MarketAllocation,
    /// Bid rigging (s.45AD(4))
    BidRigging,
}

impl CartelType {
    /// Get the relevant CCA section
    pub fn section(&self) -> &'static str {
        match self {
            CartelType::PriceFixing => "45AD(1)",
            CartelType::OutputRestriction => "45AD(2)",
            CartelType::MarketAllocation => "45AD(3)",
            CartelType::BidRigging => "45AD(4)",
        }
    }

    /// Get maximum imprisonment term (years)
    pub fn max_imprisonment_years(&self) -> u32 {
        // All cartel offences carry up to 10 years
        10
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            CartelType::PriceFixing => "Agreement to fix, control, or maintain prices",
            CartelType::OutputRestriction => "Agreement to restrict output or supply",
            CartelType::MarketAllocation => {
                "Agreement to allocate customers, suppliers, or territories"
            }
            CartelType::BidRigging => "Agreement to rig bids or abstain from bidding",
        }
    }
}

/// Type of merger clearance
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClearanceType {
    /// Informal clearance (most common)
    Informal,
    /// Formal merger authorisation
    FormalAuthorisation,
    /// Clearance with undertakings
    WithUndertakings,
    /// No substantial lessening of competition found
    NoSLC,
}

impl CompetitionError {
    /// Create a cartel conduct error
    pub fn cartel(cartel_type: CartelType, description: impl Into<String>) -> Self {
        CompetitionError::CartelConduct {
            cartel_type,
            section: cartel_type.section().to_string(),
            description: description.into(),
            criminal: true,
        }
    }

    /// Create a misuse of market power error
    pub fn misuse_of_power(
        description: impl Into<String>,
        market_share: Option<f64>,
        effect: impl Into<String>,
    ) -> Self {
        CompetitionError::MisuseOfMarketPower {
            description: description.into(),
            market_share,
            competition_effect: effect.into(),
        }
    }

    /// Create a merger SLC error
    pub fn merger_slc(
        description: impl Into<String>,
        post_merger_share: Option<f64>,
        effects: Vec<String>,
    ) -> Self {
        CompetitionError::MergerSLC {
            description: description.into(),
            post_merger_share,
            effects,
        }
    }

    /// Create an analysis error
    pub fn analysis(message: impl Into<String>) -> Self {
        CompetitionError::AnalysisError {
            message: message.into(),
            context: None,
        }
    }

    /// Create a validation error
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        CompetitionError::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Check if this is a criminal matter
    pub fn is_criminal(&self) -> bool {
        matches!(self, CompetitionError::CartelConduct { criminal: true, .. })
    }

    /// Get the relevant CCA section if applicable
    pub fn cca_section(&self) -> Option<&str> {
        match self {
            CompetitionError::CartelConduct { section, .. } => Some(section),
            CompetitionError::MisuseOfMarketPower { .. } => Some("46"),
            CompetitionError::ExclusiveDealing { subsection, .. } => Some(subsection),
            CompetitionError::MergerSLC { .. } => Some("50"),
            CompetitionError::ResalePriceMaintenance { .. } => Some("48"),
            _ => None,
        }
    }

    /// Get maximum penalty description
    pub fn max_penalty_description(&self) -> Option<String> {
        match self {
            CompetitionError::CartelConduct { .. } => Some(
                "Individuals: Up to 10 years imprisonment and/or $444,000. \
                 Corporations: Greater of $10M, 3x benefit gained, or 10% of annual turnover"
                    .to_string(),
            ),
            CompetitionError::MisuseOfMarketPower { .. } => Some(
                "Individuals: Up to $500,000 per contravention. \
                 Corporations: Greater of $10M, 3x benefit gained, or 10% of annual turnover"
                    .to_string(),
            ),
            CompetitionError::MergerSLC { .. } => Some(
                "Corporations: Greater of $10M, 3x benefit gained, or 10% of annual turnover. \
                 Divestiture orders may also be made"
                    .to_string(),
            ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartel_type_sections() {
        assert_eq!(CartelType::PriceFixing.section(), "45AD(1)");
        assert_eq!(CartelType::BidRigging.section(), "45AD(4)");
    }

    #[test]
    fn test_cartel_error_creation() {
        let err = CompetitionError::cartel(
            CartelType::PriceFixing,
            "Competitors agreed to fix cement prices",
        );

        assert!(err.is_criminal());
        assert_eq!(err.cca_section(), Some("45AD(1)"));
    }

    #[test]
    fn test_misuse_of_power_error() {
        let err = CompetitionError::misuse_of_power(
            "Predatory pricing below cost",
            Some(0.65),
            "Substantially lessen competition in retail market",
        );

        assert!(!err.is_criminal());
        assert_eq!(err.cca_section(), Some("46"));
    }

    #[test]
    fn test_merger_slc_error() {
        let err = CompetitionError::merger_slc(
            "Horizontal merger in concentrated market",
            Some(0.55),
            vec![
                "Unilateral price increase".into(),
                "Reduced innovation".into(),
            ],
        );

        assert_eq!(err.cca_section(), Some("50"));
        assert!(err.max_penalty_description().is_some());
    }

    #[test]
    fn test_error_display() {
        let err = CompetitionError::cartel(CartelType::BidRigging, "Government tender bid rigging");

        let display = format!("{}", err);
        assert!(display.contains("Cartel conduct"));
        assert!(display.contains("BidRigging"));
        assert!(display.contains("45AD"));
    }
}
