//! Vietnamese Competition Law 2018 (Luật Cạnh tranh 2018) - Law No. 23/2018/QH14
//!
//! Vietnam's law on competition and antitrust, effective from July 1, 2019.
//!
//! ## Key Provisions
//!
//! - **Market dominance** (Điều 25-28): >30% market share may be dominant
//! - **Monopoly** (Độc quyền): >50% market share
//! - **Merger control** (Điều 31-39): Large M&A requires notification
//! - **Anti-competitive agreements** (Điều 12-17): Cartels prohibited
//! - **Abuse of dominance** (Điều 18-24): Predatory pricing, refusal to deal

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Market share thresholds for competition law (Tỷ lệ thị phần) - Article 25
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketShareLevel {
    /// Less than 30% - Generally not dominant (Dưới 30%)
    BelowDominance,
    /// 30-50% - Potential market dominance (30-50%)
    PotentialDominance,
    /// 50-65% - Market dominance (50-65%)
    Dominance,
    /// Over 65% - Monopoly (Trên 65%)
    Monopoly,
}

impl MarketShareLevel {
    /// Determine level from market share percentage
    pub fn from_percentage(share: f64) -> Self {
        if share >= 65.0 {
            Self::Monopoly
        } else if share >= 50.0 {
            Self::Dominance
        } else if share >= 30.0 {
            Self::PotentialDominance
        } else {
            Self::BelowDominance
        }
    }

    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::BelowDominance => "Dưới ngưỡng thị phần chi phối (dưới 30%)",
            Self::PotentialDominance => "Có khả năng có thị phần chi phối (30-50%)",
            Self::Dominance => "Có thị phần chi phối thị trường (50-65%)",
            Self::Monopoly => "Độc quyền thị trường (trên 65%)",
        }
    }

    /// Check if subject to dominance regulations
    pub fn is_subject_to_dominance_rules(&self) -> bool {
        matches!(
            self,
            Self::PotentialDominance | Self::Dominance | Self::Monopoly
        )
    }

    /// Check if considered monopoly
    pub fn is_monopoly(&self) -> bool {
        matches!(self, Self::Monopoly)
    }
}

/// Market position (Vị trí thị trường)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPosition {
    /// Enterprise name
    pub enterprise: String,
    /// Market share percentage (0-100)
    pub market_share: f64,
    /// Relevant market definition
    pub relevant_market: String,
}

impl MarketPosition {
    /// Get market share level
    pub fn market_share_level(&self) -> MarketShareLevel {
        MarketShareLevel::from_percentage(self.market_share)
    }

    /// Check if has market dominance (Article 25)
    pub fn has_market_dominance(&self) -> bool {
        self.market_share_level().is_subject_to_dominance_rules()
    }
}

/// Types of anti-competitive agreements (Thỏa thuận hạn chế cạnh tranh) - Article 12-17
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AntiCompetitiveAgreement {
    /// Price fixing (Thỏa thuận về giá)
    PriceFixing,
    /// Market allocation (Phân chia thị trường)
    MarketAllocation,
    /// Output restriction (Hạn chế sản lượng)
    OutputRestriction,
    /// Bid rigging (Thỏa thuận trong đấu thầu)
    BidRigging,
    /// Collective boycott (Tẩy chay tập thể)
    CollectiveBoycott,
    /// Resale price maintenance (Giá bán lại cố định)
    ResalePriceMaintenance,
    /// Other restrictive agreements
    Other(String),
}

impl AntiCompetitiveAgreement {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::PriceFixing => "Thỏa thuận cố định giá, thao túng giá".to_string(),
            Self::MarketAllocation => "Thỏa thuận phân chia thị trường".to_string(),
            Self::OutputRestriction => "Thỏa thuận hạn chế sản lượng".to_string(),
            Self::BidRigging => "Thỏa thuận trong đấu thầu".to_string(),
            Self::CollectiveBoycott => "Thỏa thuận tẩy chay".to_string(),
            Self::ResalePriceMaintenance => "Quy định giá bán lại".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Get article reference
    pub fn article(&self) -> u32 {
        match self {
            Self::PriceFixing => 12,
            Self::MarketAllocation => 13,
            Self::OutputRestriction => 14,
            Self::BidRigging => 15,
            Self::CollectiveBoycott => 16,
            Self::ResalePriceMaintenance => 17,
            Self::Other(_) => 12,
        }
    }

    /// Check if per se illegal (always prohibited)
    pub fn is_per_se_illegal(&self) -> bool {
        matches!(
            self,
            Self::PriceFixing | Self::MarketAllocation | Self::BidRigging
        )
    }
}

/// Abuse of market dominance (Lạm dụng vị trí thống lĩnh) - Article 18-24
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbuseOfDominance {
    /// Predatory pricing (Bán giá dưới giá thành)
    PredatoryPricing,
    /// Refusal to deal (Từ chối giao dịch)
    RefusalToDeal,
    /// Exclusive dealing (Giao dịch độc quyền)
    ExclusiveDealing,
    /// Tying arrangement (Gắn kết sản phẩm)
    Tying,
    /// Discriminatory pricing (Phân biệt đối xử về giá)
    DiscriminatoryPricing,
    /// Margin squeeze (Ép giá)
    MarginSqueeze,
    /// Other abusive conduct
    Other(String),
}

impl AbuseOfDominance {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::PredatoryPricing => "Bán hàng hóa, dịch vụ dưới giá thành".to_string(),
            Self::RefusalToDeal => "Từ chối giao dịch không có lý do chính đáng".to_string(),
            Self::ExclusiveDealing => "Ép buộc giao dịch độc quyền".to_string(),
            Self::Tying => "Gắn kết sản phẩm, dịch vụ".to_string(),
            Self::DiscriminatoryPricing => "Phân biệt đối xử về giá cả".to_string(),
            Self::MarginSqueeze => "Ép giá đối với đối tác".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Get article reference
    pub fn article(&self) -> u32 {
        match self {
            Self::PredatoryPricing => 18,
            Self::RefusalToDeal => 19,
            Self::ExclusiveDealing => 20,
            Self::Tying => 21,
            Self::DiscriminatoryPricing => 22,
            Self::MarginSqueeze => 23,
            Self::Other(_) => 18,
        }
    }
}

/// Merger notification thresholds (Ngưỡng thông báo tập trung kinh tế) - Article 31
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergerThreshold;

impl MergerThreshold {
    /// Total assets threshold: 3,000 billion VND (Điều 31.1.a)
    pub const TOTAL_ASSETS: i64 = 3_000_000_000_000;

    /// Revenue threshold: 3,000 billion VND (Điều 31.1.b)
    pub const TOTAL_REVENUE: i64 = 3_000_000_000_000;

    /// Market share threshold: 20% (Điều 31.1.c)
    pub const MARKET_SHARE: f64 = 20.0;

    /// Check if merger requires notification
    pub fn requires_notification(
        total_assets: i64,
        total_revenue: i64,
        combined_market_share: f64,
    ) -> bool {
        total_assets >= Self::TOTAL_ASSETS
            || total_revenue >= Self::TOTAL_REVENUE
            || combined_market_share >= Self::MARKET_SHARE
    }
}

/// Merger transaction (Tập trung kinh tế)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergerTransaction {
    /// Acquiring entity
    pub acquirer: String,
    /// Target entity
    pub target: String,
    /// Combined total assets (VND)
    pub combined_assets: i64,
    /// Combined total revenue (VND)
    pub combined_revenue: i64,
    /// Combined market share percentage
    pub combined_market_share: f64,
    /// Relevant market
    pub relevant_market: String,
}

impl MergerTransaction {
    /// Check if requires notification to competition authority
    pub fn requires_notification(&self) -> bool {
        MergerThreshold::requires_notification(
            self.combined_assets,
            self.combined_revenue,
            self.combined_market_share,
        )
    }

    /// Check if likely to substantially lessen competition
    pub fn likely_reduces_competition(&self) -> bool {
        // Mergers resulting in >50% market share are presumed anticompetitive
        self.combined_market_share >= 50.0
    }
}

/// Result type for competition law operations
pub type CompetitionResult<T> = Result<T, CompetitionError>;

/// Errors related to Competition Law
#[derive(Debug, Error)]
pub enum CompetitionError {
    /// Anti-competitive agreement violation
    #[error("Vi phạm quy định về thỏa thuận hạn chế cạnh tranh (Điều {article}): {agreement}")]
    AntiCompetitiveAgreement { article: u32, agreement: String },

    /// Abuse of dominance violation
    #[error("Lạm dụng vị trí thống lĩnh thị trường (Điều {article}): {abuse}")]
    AbuseOfDominance { article: u32, abuse: String },

    /// Merger notification violation
    #[error("Vi phạm quy định thông báo tập trung kinh tế (Điều 31): {reason}")]
    MergerNotificationViolation { reason: String },

    /// Anticompetitive merger
    #[error("Tập trung kinh tế hạn chế cạnh tranh (Điều 32): thị phần {market_share}%")]
    AnticompetitiveMerger { market_share: f64 },

    /// Other competition violation
    #[error("Vi phạm Luật Cạnh tranh: {reason}")]
    CompetitionViolation { reason: String },
}

/// Validate market position for dominance abuse
pub fn validate_no_abuse_of_dominance(
    position: &MarketPosition,
    conduct: &AbuseOfDominance,
) -> CompetitionResult<()> {
    if position.has_market_dominance() {
        Err(CompetitionError::AbuseOfDominance {
            article: conduct.article(),
            abuse: format!(
                "{} (thị phần: {:.1}%)",
                conduct.description_vi(),
                position.market_share
            ),
        })
    } else {
        Ok(())
    }
}

/// Validate merger transaction
pub fn validate_merger_transaction(merger: &MergerTransaction) -> CompetitionResult<()> {
    if !merger.requires_notification() {
        return Ok(()); // Below thresholds, no review needed
    }

    if merger.likely_reduces_competition() {
        Err(CompetitionError::AnticompetitiveMerger {
            market_share: merger.combined_market_share,
        })
    } else {
        Ok(())
    }
}

/// Get Competition Law checklist
pub fn get_competition_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Xác định thị trường liên quan",
            "Define relevant market",
            "Điều 3",
        ),
        ("Tính thị phần", "Calculate market share", "Điều 25"),
        (
            "Kiểm tra thỏa thuận hạn chế cạnh tranh",
            "Check anti-competitive agreements",
            "Điều 12-17",
        ),
        (
            "Kiểm tra lạm dụng vị trí thống lĩnh",
            "Check abuse of dominance",
            "Điều 18-24",
        ),
        (
            "Thông báo tập trung kinh tế",
            "Merger notification",
            "Điều 31",
        ),
        (
            "Đánh giá ảnh hưởng cạnh tranh",
            "Competition impact assessment",
            "Điều 32",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_share_levels() {
        assert_eq!(
            MarketShareLevel::from_percentage(25.0),
            MarketShareLevel::BelowDominance
        );
        assert_eq!(
            MarketShareLevel::from_percentage(40.0),
            MarketShareLevel::PotentialDominance
        );
        assert_eq!(
            MarketShareLevel::from_percentage(55.0),
            MarketShareLevel::Dominance
        );
        assert_eq!(
            MarketShareLevel::from_percentage(70.0),
            MarketShareLevel::Monopoly
        );

        assert!(!MarketShareLevel::BelowDominance.is_subject_to_dominance_rules());
        assert!(MarketShareLevel::PotentialDominance.is_subject_to_dominance_rules());
        assert!(MarketShareLevel::Monopoly.is_monopoly());
    }

    #[test]
    fn test_market_position() {
        let dominant = MarketPosition {
            enterprise: "ABC Corp".to_string(),
            market_share: 55.0,
            relevant_market: "E-commerce".to_string(),
        };

        assert!(dominant.has_market_dominance());
        assert_eq!(dominant.market_share_level(), MarketShareLevel::Dominance);

        let non_dominant = MarketPosition {
            enterprise: "XYZ Ltd".to_string(),
            market_share: 15.0,
            relevant_market: "Retail".to_string(),
        };

        assert!(!non_dominant.has_market_dominance());
    }

    #[test]
    fn test_anti_competitive_agreements() {
        let price_fixing = AntiCompetitiveAgreement::PriceFixing;
        assert!(price_fixing.is_per_se_illegal());
        assert_eq!(price_fixing.article(), 12);

        let market_allocation = AntiCompetitiveAgreement::MarketAllocation;
        assert!(market_allocation.is_per_se_illegal());

        let resale_price = AntiCompetitiveAgreement::ResalePriceMaintenance;
        assert!(!resale_price.is_per_se_illegal());
    }

    #[test]
    fn test_abuse_of_dominance() {
        let predatory = AbuseOfDominance::PredatoryPricing;
        assert_eq!(predatory.article(), 18);
        assert!(predatory.description_vi().contains("giá thành"));

        let refusal = AbuseOfDominance::RefusalToDeal;
        assert_eq!(refusal.article(), 19);
    }

    #[test]
    fn test_merger_thresholds() {
        // Above asset threshold
        assert!(MergerThreshold::requires_notification(
            4_000_000_000_000,
            1_000_000_000_000,
            10.0
        ));

        // Above revenue threshold
        assert!(MergerThreshold::requires_notification(
            1_000_000_000_000,
            4_000_000_000_000,
            10.0
        ));

        // Above market share threshold
        assert!(MergerThreshold::requires_notification(
            1_000_000_000_000,
            1_000_000_000_000,
            25.0
        ));

        // Below all thresholds
        assert!(!MergerThreshold::requires_notification(
            1_000_000_000_000,
            1_000_000_000_000,
            10.0
        ));
    }

    #[test]
    fn test_merger_transaction() {
        let large_merger = MergerTransaction {
            acquirer: "BigCo".to_string(),
            target: "TargetCo".to_string(),
            combined_assets: 5_000_000_000_000,
            combined_revenue: 4_000_000_000_000,
            combined_market_share: 55.0,
            relevant_market: "Telecommunications".to_string(),
        };

        assert!(large_merger.requires_notification());
        assert!(large_merger.likely_reduces_competition());

        let small_merger = MergerTransaction {
            acquirer: "SmallCo".to_string(),
            target: "TinyCo".to_string(),
            combined_assets: 500_000_000_000,
            combined_revenue: 400_000_000_000,
            combined_market_share: 5.0,
            relevant_market: "Local retail".to_string(),
        };

        assert!(!small_merger.requires_notification());
        assert!(!small_merger.likely_reduces_competition());
    }

    #[test]
    fn test_validation() {
        let dominant_position = MarketPosition {
            enterprise: "DominantCo".to_string(),
            market_share: 60.0,
            relevant_market: "Market X".to_string(),
        };

        let predatory_pricing = AbuseOfDominance::PredatoryPricing;

        assert!(validate_no_abuse_of_dominance(&dominant_position, &predatory_pricing).is_err());

        let anticompetitive_merger = MergerTransaction {
            acquirer: "A".to_string(),
            target: "B".to_string(),
            combined_assets: 5_000_000_000_000,
            combined_revenue: 4_000_000_000_000,
            combined_market_share: 65.0,
            relevant_market: "Market Y".to_string(),
        };

        assert!(validate_merger_transaction(&anticompetitive_merger).is_err());
    }

    #[test]
    fn test_competition_checklist() {
        let checklist = get_competition_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 6);
    }
}
