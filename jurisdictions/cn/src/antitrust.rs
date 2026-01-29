//! Anti-Monopoly Law Module (反垄断法)
//!
//! # 中华人民共和国反垄断法 / Anti-Monopoly Law of the PRC
//!
//! Implements the Anti-Monopoly Law (2022 Revision) effective August 1, 2022.
//!
//! ## Key Concepts
//!
//! - **垄断行为 (Monopolistic Conduct)**: Prohibited anticompetitive conduct
//! - **市场支配地位 (Market Dominance)**: Position allowing control over market
//! - **经营者集中 (Concentration of Undertakings)**: Mergers and acquisitions
//! - **行政性垄断 (Administrative Monopoly)**: Government-imposed restrictions on competition
//!
//! ## Prohibited Conduct (Article 9-55)
//!
//! ### 1. Monopoly Agreements (垄断协议, Articles 16-20)
//!
//! #### Horizontal Agreements (Article 17)
//! - Price fixing
//! - Output restriction
//! - Market division
//! - Bid rigging
//!
//! #### Vertical Agreements (Article 18)
//! - Resale price maintenance
//! - Minimum resale price
//!
//! ### 2. Abuse of Market Dominance (滥用市场支配地位, Articles 22-25)
//!
//! - Unfair pricing
//! - Below-cost selling
//! - Refusal to deal
//! - Tying and bundling
//! - Discriminatory treatment
//!
//! ### 3. Concentration Review (经营者集中审查, Articles 26-33)
//!
//! SAMR (State Administration for Market Regulation) merger review
//!
//! ### 4. Administrative Monopoly (行政性垄断, Articles 34-42)
//!
//! Government actions restricting competition
//!
//! ## Merger Filing Thresholds (Article 26)
//!
//! Filing required if:
//! - Combined global turnover > 10 billion RMB AND at least 2 parties > 400 million RMB in China
//! - Combined China turnover > 2 billion RMB AND at least 2 parties > 400 million RMB in China

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Monopolistic conduct type (垄断行为类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonopolisticConductType {
    /// Monopoly agreement (垄断协议)
    MonopolyAgreement,
    /// Abuse of market dominance (滥用市场支配地位)
    AbuseOfDominance,
    /// Prohibited concentration (禁止的经营者集中)
    ProhibitedConcentration,
    /// Administrative monopoly (行政性垄断)
    AdministrativeMonopoly,
}

impl MonopolisticConductType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::MonopolyAgreement => BilingualText::new("垄断协议", "Monopoly agreement"),
            Self::AbuseOfDominance => {
                BilingualText::new("滥用市场支配地位", "Abuse of market dominance")
            }
            Self::ProhibitedConcentration => {
                BilingualText::new("禁止的经营者集中", "Prohibited concentration")
            }
            Self::AdministrativeMonopoly => {
                BilingualText::new("行政性垄断", "Administrative monopoly")
            }
        }
    }
}

/// Monopoly agreement type (垄断协议类型)
///
/// Articles 17-18
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonopolyAgreementType {
    /// Horizontal - Price fixing (固定价格)
    HorizontalPriceFixing,
    /// Horizontal - Output restriction (限制产量)
    HorizontalOutputRestriction,
    /// Horizontal - Market division (分割市场)
    HorizontalMarketDivision,
    /// Horizontal - Bid rigging (联合抵制交易)
    HorizontalBidRigging,
    /// Vertical - Resale price maintenance (固定转售价格)
    VerticalResalePriceMaintenance,
    /// Vertical - Minimum resale price (限定最低转售价格)
    VerticalMinimumResalePrice,
}

impl MonopolyAgreementType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::HorizontalPriceFixing => {
                BilingualText::new("横向价格固定", "Horizontal price fixing")
            }
            Self::HorizontalOutputRestriction => {
                BilingualText::new("限制产量", "Output restriction")
            }
            Self::HorizontalMarketDivision => BilingualText::new("分割市场", "Market division"),
            Self::HorizontalBidRigging => BilingualText::new("联合抵制交易", "Bid rigging"),
            Self::VerticalResalePriceMaintenance => BilingualText::new("固定转售价格", "RPM"),
            Self::VerticalMinimumResalePrice => {
                BilingualText::new("限定最低转售价格", "Minimum RPM")
            }
        }
    }

    /// Check if horizontal agreement (Article 17)
    pub fn is_horizontal(&self) -> bool {
        matches!(
            self,
            Self::HorizontalPriceFixing
                | Self::HorizontalOutputRestriction
                | Self::HorizontalMarketDivision
                | Self::HorizontalBidRigging
        )
    }

    /// Check if vertical agreement (Article 18)
    pub fn is_vertical(&self) -> bool {
        matches!(
            self,
            Self::VerticalResalePriceMaintenance | Self::VerticalMinimumResalePrice
        )
    }
}

/// Monopoly agreement (垄断协议)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonopolyAgreement {
    /// Agreement parties
    pub parties: Vec<String>,
    /// Agreement type
    pub agreement_type: MonopolyAgreementType,
    /// Description
    pub description: BilingualText,
    /// Agreement date
    pub agreement_date: DateTime<Utc>,
    /// Relevant market
    pub relevant_market: BilingualText,
}

/// Abuse of dominance type (滥用支配地位类型)
///
/// Article 22
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseType {
    /// Unfair pricing (不公平高价)
    UnfairPricing,
    /// Below-cost selling (低于成本销售)
    BelowCostSelling,
    /// Refusal to deal (拒绝交易)
    RefusalToDeal,
    /// Exclusive dealing (限定交易)
    ExclusiveDealing,
    /// Tying and bundling (搭售)
    TyingAndBundling,
    /// Discriminatory treatment (差别待遇)
    DiscriminatoryTreatment,
}

impl AbuseType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::UnfairPricing => BilingualText::new("不公平高价", "Unfair pricing"),
            Self::BelowCostSelling => BilingualText::new("低于成本销售", "Below-cost selling"),
            Self::RefusalToDeal => BilingualText::new("拒绝交易", "Refusal to deal"),
            Self::ExclusiveDealing => BilingualText::new("限定交易", "Exclusive dealing"),
            Self::TyingAndBundling => BilingualText::new("搭售", "Tying and bundling"),
            Self::DiscriminatoryTreatment => {
                BilingualText::new("差别待遇", "Discriminatory treatment")
            }
        }
    }
}

/// Market dominance (市场支配地位)
///
/// Articles 22-25
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDominance {
    /// Undertaking with dominance
    pub undertaking: String,
    /// Relevant market
    pub relevant_market: BilingualText,
    /// Market share percentage
    pub market_share: f64,
    /// Presumption of dominance
    pub presumed_dominant: bool,
}

impl MarketDominance {
    /// Check if presumed dominant based on market share
    ///
    /// Article 23:
    /// - 1 undertaking with ≥50% = presumed dominant
    /// - 2 undertakings with combined ≥2/3 (each ≥10%) = presumed dominant
    /// - 3 undertakings with combined ≥3/4 (each ≥10%) = presumed dominant
    pub fn check_presumption_single(&self) -> bool {
        self.market_share >= 50.0
    }
}

/// Abuse of market dominance (滥用市场支配地位)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseOfDominance {
    /// Dominant undertaking
    pub dominant_undertaking: String,
    /// Market dominance
    pub dominance: MarketDominance,
    /// Abuse type
    pub abuse_type: AbuseType,
    /// Description
    pub description: BilingualText,
    /// Date of abuse
    pub abuse_date: DateTime<Utc>,
}

/// Concentration transaction (经营者集中)
///
/// Articles 26-33
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationTransaction {
    /// Transaction parties
    pub parties: Vec<String>,
    /// Transaction type
    pub transaction_type: ConcentrationType,
    /// Combined global turnover (RMB)
    pub combined_global_turnover_rmb: f64,
    /// Combined China turnover (RMB)
    pub combined_china_turnover_rmb: f64,
    /// Each party China turnover (RMB)
    pub each_party_china_turnover_rmb: Vec<f64>,
    /// Relevant market
    pub relevant_market: BilingualText,
    /// Filing made
    pub filing_made: bool,
    /// Filing date
    pub filing_date: Option<DateTime<Utc>>,
    /// Review decision
    pub review_decision: Option<ConcentrationReviewDecision>,
}

impl ConcentrationTransaction {
    /// Check if filing is required
    ///
    /// Article 26
    pub fn requires_filing(&self) -> bool {
        // Threshold 1: Combined global > 10B RMB AND at least 2 parties > 400M RMB in China
        let threshold1 = self.combined_global_turnover_rmb > 10_000_000_000.0
            && self
                .each_party_china_turnover_rmb
                .iter()
                .filter(|&&t| t > 400_000_000.0)
                .count()
                >= 2;

        // Threshold 2: Combined China > 2B RMB AND at least 2 parties > 400M RMB in China
        let threshold2 = self.combined_china_turnover_rmb > 2_000_000_000.0
            && self
                .each_party_china_turnover_rmb
                .iter()
                .filter(|&&t| t > 400_000_000.0)
                .count()
                >= 2;

        threshold1 || threshold2
    }

    /// Check if transaction is compliant (filed if required)
    pub fn is_compliant(&self) -> bool {
        !self.requires_filing() || self.filing_made
    }
}

/// Concentration type (集中类型)
///
/// Article 26
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcentrationType {
    /// Merger (合并)
    Merger,
    /// Acquisition of control through equity/assets (取得控制权)
    AcquisitionOfControl,
    /// Control through contract or other means (通过合同等方式取得控制权)
    ControlThroughContract,
}

impl ConcentrationType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Merger => BilingualText::new("合并", "Merger"),
            Self::AcquisitionOfControl => {
                BilingualText::new("取得控制权", "Acquisition of control")
            }
            Self::ControlThroughContract => {
                BilingualText::new("合同控制", "Control through contract")
            }
        }
    }
}

/// Concentration review decision (审查决定)
///
/// Article 30-33
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcentrationReviewDecision {
    /// Approved (批准)
    Approved,
    /// Approved with conditions (附条件批准)
    ApprovedWithConditions,
    /// Prohibited (禁止)
    Prohibited,
}

impl ConcentrationReviewDecision {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Approved => BilingualText::new("批准", "Approved"),
            Self::ApprovedWithConditions => {
                BilingualText::new("附条件批准", "Approved with conditions")
            }
            Self::Prohibited => BilingualText::new("禁止", "Prohibited"),
        }
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate monopoly agreement
///
/// Articles 17-18
pub fn validate_monopoly_agreement(agreement: &MonopolyAgreement) -> Result<(), AntitrustError> {
    // Monopoly agreements are generally prohibited
    // (subject to exemptions under Article 20, not implemented here)

    Err(AntitrustError::ProhibitedMonopolyAgreement {
        agreement_type: agreement.agreement_type.description(),
        parties: agreement.parties.clone(),
    })
}

/// Validate abuse of market dominance
///
/// Articles 22-25
pub fn validate_abuse_of_dominance(abuse: &AbuseOfDominance) -> Result<(), AntitrustError> {
    // Check if undertaking has market dominance
    if !abuse.dominance.presumed_dominant && !abuse.dominance.check_presumption_single() {
        return Err(AntitrustError::NoDominance {
            undertaking: abuse.dominant_undertaking.clone(),
            market_share: abuse.dominance.market_share,
        });
    }

    // Abuse of dominance is prohibited
    Err(AntitrustError::AbuseOfDominance {
        undertaking: abuse.dominant_undertaking.clone(),
        abuse_type: abuse.abuse_type.description(),
    })
}

/// Validate concentration transaction
///
/// Articles 26-33
pub fn validate_concentration_transaction(
    transaction: &ConcentrationTransaction,
) -> Result<(), AntitrustError> {
    // Check if filing is required but not made
    if transaction.requires_filing() && !transaction.filing_made {
        return Err(AntitrustError::FilingRequired {
            parties: transaction.parties.clone(),
            combined_turnover: transaction.combined_global_turnover_rmb,
        });
    }

    // Check if decision is prohibited
    if matches!(
        transaction.review_decision,
        Some(ConcentrationReviewDecision::Prohibited)
    ) {
        return Err(AntitrustError::ConcentrationProhibited {
            parties: transaction.parties.clone(),
        });
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Anti-Monopoly Law
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum AntitrustError {
    /// Prohibited monopoly agreement
    #[error("Prohibited monopoly agreement ({agreement_type}): parties {parties:?}")]
    ProhibitedMonopolyAgreement {
        /// Agreement type
        agreement_type: BilingualText,
        /// Parties
        parties: Vec<String>,
    },

    /// Abuse of market dominance
    #[error("Abuse of market dominance by {undertaking}: {abuse_type}")]
    AbuseOfDominance {
        /// Undertaking
        undertaking: String,
        /// Abuse type
        abuse_type: BilingualText,
    },

    /// No market dominance
    #[error("Undertaking {undertaking} does not have market dominance (share: {market_share}%)")]
    NoDominance {
        /// Undertaking
        undertaking: String,
        /// Market share
        market_share: f64,
    },

    /// Filing required
    #[error(
        "Merger filing required for parties {parties:?} (combined turnover: {combined_turnover} RMB)"
    )]
    FilingRequired {
        /// Parties
        parties: Vec<String>,
        /// Combined turnover
        combined_turnover: f64,
    },

    /// Concentration prohibited
    #[error("Concentration prohibited for parties: {parties:?}")]
    ConcentrationProhibited {
        /// Parties
        parties: Vec<String>,
    },
}

/// Result type for Antitrust operations
pub type AntitrustResult<T> = Result<T, AntitrustError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monopoly_agreement_types() {
        assert!(MonopolyAgreementType::HorizontalPriceFixing.is_horizontal());
        assert!(!MonopolyAgreementType::HorizontalPriceFixing.is_vertical());
        assert!(MonopolyAgreementType::VerticalResalePriceMaintenance.is_vertical());
        assert!(!MonopolyAgreementType::VerticalResalePriceMaintenance.is_horizontal());
    }

    #[test]
    fn test_market_dominance_presumption() {
        let dominance = MarketDominance {
            undertaking: "Company A".to_string(),
            relevant_market: BilingualText::new("相关市场", "Relevant market"),
            market_share: 60.0,
            presumed_dominant: false,
        };

        assert!(dominance.check_presumption_single());
    }

    #[test]
    fn test_concentration_filing_threshold() {
        let transaction = ConcentrationTransaction {
            parties: vec!["Company A".to_string(), "Company B".to_string()],
            transaction_type: ConcentrationType::Merger,
            combined_global_turnover_rmb: 15_000_000_000.0, // 15B RMB
            combined_china_turnover_rmb: 5_000_000_000.0,
            each_party_china_turnover_rmb: vec![3_000_000_000.0, 2_000_000_000.0], // Both > 400M
            relevant_market: BilingualText::new("相关市场", "Relevant market"),
            filing_made: true,
            filing_date: Some(Utc::now()),
            review_decision: Some(ConcentrationReviewDecision::Approved),
        };

        assert!(transaction.requires_filing());
        assert!(transaction.is_compliant());
        assert!(validate_concentration_transaction(&transaction).is_ok());
    }

    #[test]
    fn test_concentration_no_filing_required() {
        let transaction = ConcentrationTransaction {
            parties: vec!["Small Company A".to_string(), "Small Company B".to_string()],
            transaction_type: ConcentrationType::Merger,
            combined_global_turnover_rmb: 100_000_000.0, // 100M RMB
            combined_china_turnover_rmb: 50_000_000.0,
            each_party_china_turnover_rmb: vec![30_000_000.0, 20_000_000.0],
            relevant_market: BilingualText::new("相关市场", "Relevant market"),
            filing_made: false,
            filing_date: None,
            review_decision: None,
        };

        assert!(!transaction.requires_filing());
        assert!(transaction.is_compliant());
    }

    #[test]
    fn test_concentration_filing_not_made() {
        let transaction = ConcentrationTransaction {
            parties: vec!["Company A".to_string(), "Company B".to_string()],
            transaction_type: ConcentrationType::Merger,
            combined_global_turnover_rmb: 15_000_000_000.0,
            combined_china_turnover_rmb: 5_000_000_000.0,
            each_party_china_turnover_rmb: vec![3_000_000_000.0, 2_000_000_000.0],
            relevant_market: BilingualText::new("相关市场", "Relevant market"),
            filing_made: false, // Filing not made
            filing_date: None,
            review_decision: None,
        };

        assert!(transaction.requires_filing());
        assert!(!transaction.is_compliant());
        assert!(validate_concentration_transaction(&transaction).is_err());
    }

    #[test]
    fn test_monopoly_agreement_validation() {
        let agreement = MonopolyAgreement {
            parties: vec!["Company A".to_string(), "Company B".to_string()],
            agreement_type: MonopolyAgreementType::HorizontalPriceFixing,
            description: BilingualText::new("价格垄断协议", "Price fixing agreement"),
            agreement_date: Utc::now(),
            relevant_market: BilingualText::new("相关市场", "Relevant market"),
        };

        assert!(validate_monopoly_agreement(&agreement).is_err());
    }

    #[test]
    fn test_abuse_validation() {
        let abuse = AbuseOfDominance {
            dominant_undertaking: "Dominant Corp".to_string(),
            dominance: MarketDominance {
                undertaking: "Dominant Corp".to_string(),
                relevant_market: BilingualText::new("相关市场", "Relevant market"),
                market_share: 70.0,
                presumed_dominant: true,
            },
            abuse_type: AbuseType::UnfairPricing,
            description: BilingualText::new("不公平高价", "Unfair pricing"),
            abuse_date: Utc::now(),
        };

        assert!(validate_abuse_of_dominance(&abuse).is_err());
    }
}
