//! Trade Competition Act - พ.ร.บ. การแข่งขันทางการค้า พ.ศ. 2560

use serde::{Deserialize, Serialize};

/// Prohibited practices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProhibitedPractice {
    /// Price fixing (การกำหนดราคา)
    PriceFixing,
    /// Market allocation (การแบ่งแยกตลาด)
    MarketAllocation,
    /// Bid rigging (การสมคบเสนอราคา)
    BidRigging,
    /// Resale price maintenance (การกำหนดราคาขายต่อ)
    ResalePriceMaintenance,
    /// Predatory pricing (การขายทุน)
    PredatoryPricing,
    /// Refusal to deal (การปฏิเสธการค้า)
    RefusalToDeal,
}

impl ProhibitedPractice {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::PriceFixing => "การกำหนดราคา",
            Self::MarketAllocation => "การแบ่งแยกตลาด",
            Self::BidRigging => "การสมคบเสนอราคา",
            Self::ResalePriceMaintenance => "การกำหนดราคาขายต่อ",
            Self::PredatoryPricing => "การขายทุน",
            Self::RefusalToDeal => "การปฏิเสธการค้า",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PriceFixing => "Price Fixing",
            Self::MarketAllocation => "Market Allocation",
            Self::BidRigging => "Bid Rigging",
            Self::ResalePriceMaintenance => "Resale Price Maintenance",
            Self::PredatoryPricing => "Predatory Pricing",
            Self::RefusalToDeal => "Refusal to Deal",
        }
    }
}

/// Market dominance threshold
pub const MARKET_DOMINANCE_THRESHOLD_PERCENT: u32 = 50;

/// Merger control thresholds
pub const MERGER_NOTIFICATION_THRESHOLD_THB: u64 = 1_000_000_000; // 1B THB

/// Abuse of dominance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseOfDominance {
    /// Excessive pricing
    ExcessivePricing,
    /// Margin squeeze
    MarginSqueeze,
    /// Exclusionary conduct
    ExclusionaryConduct,
    /// Tying and bundling
    TyingBundling,
}

impl AbuseOfDominance {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::ExcessivePricing => "การตั้งราคาสูงเกินควร",
            Self::MarginSqueeze => "การบีบอัตรากำไร",
            Self::ExclusionaryConduct => "การกระทำที่กีดกันคู่แข่ง",
            Self::TyingBundling => "การขายผูกมัด",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prohibited_practices() {
        assert_eq!(ProhibitedPractice::PriceFixing.name_en(), "Price Fixing");
    }

    #[test]
    fn test_thresholds() {
        assert_eq!(MARKET_DOMINANCE_THRESHOLD_PERCENT, 50);
        assert_eq!(MERGER_NOTIFICATION_THRESHOLD_THB, 1_000_000_000);
    }
}
