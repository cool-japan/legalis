//! Competition Act 2002 Types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AntiCompetitiveAgreementType {
    Horizontal,
    Vertical,
    Cartel,
    BidRigging,
    PriceFixing,
    MarketAllocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntiCompetitiveAgreement {
    pub agreement_type: AntiCompetitiveAgreementType,
    pub parties: Vec<String>,
    pub description: String,
    pub appreciable_adverse_effect: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbuseOfDominance {
    UnfairPricing,
    LimitingProduction,
    DenialOfMarketAccess,
    PredatoryPricing,
    TyingArrangement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombinationNotification {
    pub acquirer: String,
    pub target: String,
    pub transaction_value: f64,
    pub assets_value: Option<f64>,
    pub turnover: Option<f64>,
    pub requires_cci_approval: bool,
}

impl CombinationNotification {
    pub fn check_notification_requirement(&self) -> bool {
        self.transaction_value > 20_000_000_000.0
            || self.assets_value.is_some_and(|v| v > 10_000_000_000.0)
            || self.turnover.is_some_and(|v| v > 30_000_000_000.0)
    }
}
