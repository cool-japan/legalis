//! Capital Gains Tax (ITAA 1997 Part 3-1)
//!
//! CGT on disposal of CGT assets.
//!
//! ## Key Concepts
//!
//! - **CGT asset** (s.108-5): Any kind of property or legal/equitable right
//! - **CGT event** (Division 104): Event that results in capital gain/loss
//! - **Cost base** (Division 110): Original cost plus incidental costs
//! - **CGT discount** (s.115-25): 50% discount for assets held 12+ months
//!
//! ## Main Residence Exemption
//!
//! Full exemption for dwelling used as main residence (Subdivision 118-B).

use super::error::{Result, TaxError};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// CGT discount rate for individuals and trusts
pub const CGT_DISCOUNT_INDIVIDUAL: f64 = 0.50; // 50%

/// CGT discount rate for superannuation funds
pub const CGT_DISCOUNT_SUPER: f64 = 0.333; // 33.33%

/// Minimum holding period for CGT discount (12 months)
pub const CGT_DISCOUNT_HOLDING_PERIOD_MONTHS: u32 = 12;

/// CGT asset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CgtAsset {
    /// Asset identifier/description
    pub description: String,
    /// Asset type
    pub asset_type: CgtAssetType,
    /// Acquisition date
    pub acquisition_date: NaiveDate,
    /// Cost base elements
    pub cost_base: CostBase,
    /// Main residence
    pub is_main_residence: bool,
    /// Days used as main residence (if partial)
    pub main_residence_days: Option<u32>,
    /// Total days owned
    pub total_days_owned: Option<u32>,
}

impl CgtAsset {
    /// Calculate holding period in months
    pub fn holding_period_months(&self, disposal_date: NaiveDate) -> u32 {
        let years = disposal_date.year() - self.acquisition_date.year();
        let months = disposal_date.month() as i32 - self.acquisition_date.month() as i32;
        let days = disposal_date.day() as i32 - self.acquisition_date.day() as i32;

        let total_months = years * 12 + months;
        if days < 0 {
            (total_months - 1).max(0) as u32
        } else {
            total_months.max(0) as u32
        }
    }

    /// Check if eligible for CGT discount
    pub fn eligible_for_discount(&self, disposal_date: NaiveDate) -> bool {
        self.holding_period_months(disposal_date) >= CGT_DISCOUNT_HOLDING_PERIOD_MONTHS
    }
}

/// CGT asset type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CgtAssetType {
    /// Real property (land/buildings)
    RealProperty,
    /// Shares
    Shares,
    /// Units in a trust
    Units,
    /// Collectables
    Collectables,
    /// Personal use assets
    PersonalUse,
    /// Goodwill
    Goodwill,
    /// Cryptocurrency
    Cryptocurrency,
    /// Other intangible
    Intangible,
    /// Other
    Other,
}

/// Cost base elements (Division 110)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostBase {
    /// Element 1: Money paid or property given (s.110-25)
    pub acquisition_cost: f64,
    /// Element 2: Incidental costs (s.110-35)
    pub incidental_costs: f64,
    /// Element 3: Ownership costs for non-income assets (s.110-40)
    pub ownership_costs: f64,
    /// Element 4: Capital expenditure (s.110-45)
    pub capital_expenditure: f64,
    /// Element 5: Title costs (s.110-50)
    pub title_costs: f64,
}

impl CostBase {
    /// Calculate total cost base
    pub fn total(&self) -> f64 {
        self.acquisition_cost
            + self.incidental_costs
            + self.ownership_costs
            + self.capital_expenditure
            + self.title_costs
    }
}

/// CGT event (Division 104)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CgtEvent {
    /// A1: Disposal of CGT asset
    A1Disposal,
    /// B1: Use and enjoyment before title passes
    B1UseAndEnjoyment,
    /// C1: Loss or destruction of CGT asset
    C1LossDestruction,
    /// C2: Cancellation, surrender, etc.
    C2Cancellation,
    /// D1: Creating rights in asset
    D1CreatingRights,
    /// E1: Trust: creating rights
    E1TrustCreatingRights,
    /// H2: CGT asset becoming trading stock
    H2TradingStock,
    /// K3: Asset passing to tax-exempt entity
    K3TaxExemptEntity,
}

impl CgtEvent {
    /// Get the event reference (e.g., "CGT event A1")
    pub fn reference(&self) -> &'static str {
        match self {
            CgtEvent::A1Disposal => "CGT event A1",
            CgtEvent::B1UseAndEnjoyment => "CGT event B1",
            CgtEvent::C1LossDestruction => "CGT event C1",
            CgtEvent::C2Cancellation => "CGT event C2",
            CgtEvent::D1CreatingRights => "CGT event D1",
            CgtEvent::E1TrustCreatingRights => "CGT event E1",
            CgtEvent::H2TradingStock => "CGT event H2",
            CgtEvent::K3TaxExemptEntity => "CGT event K3",
        }
    }
}

/// CGT exemption
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CgtExemption {
    /// Main residence exemption (Subdivision 118-B)
    MainResidence,
    /// Small business 15-year exemption (s.152-110)
    SmallBusiness15Year,
    /// Small business 50% active asset reduction (s.152-205)
    SmallBusiness50Percent,
    /// Small business retirement exemption (s.152-305)
    SmallBusinessRetirement,
    /// Small business rollover (s.152-410)
    SmallBusinessRollover,
    /// Personal use asset under $10,000 (s.118-10)
    PersonalUseUnder10k,
    /// Collectable under $500 (s.118-10)
    CollectableUnder500,
    /// Motor vehicle (s.118-5)
    MotorVehicle,
}

impl CgtExemption {
    /// Get the statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            CgtExemption::MainResidence => "Subdivision 118-B",
            CgtExemption::SmallBusiness15Year => "s.152-110",
            CgtExemption::SmallBusiness50Percent => "s.152-205",
            CgtExemption::SmallBusinessRetirement => "s.152-305",
            CgtExemption::SmallBusinessRollover => "s.152-410",
            CgtExemption::PersonalUseUnder10k => "s.118-10",
            CgtExemption::CollectableUnder500 => "s.118-10",
            CgtExemption::MotorVehicle => "s.118-5",
        }
    }
}

/// CGT discount type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CgtDiscount {
    /// Individual discount (50%)
    Individual,
    /// Superannuation fund discount (33.33%)
    SuperannuationFund,
    /// No discount (company or < 12 months)
    None,
}

impl CgtDiscount {
    /// Get the discount rate
    pub fn rate(&self) -> f64 {
        match self {
            CgtDiscount::Individual => CGT_DISCOUNT_INDIVIDUAL,
            CgtDiscount::SuperannuationFund => CGT_DISCOUNT_SUPER,
            CgtDiscount::None => 0.0,
        }
    }
}

/// CGT calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CgtCalculation {
    /// CGT event
    pub event: CgtEvent,
    /// Capital proceeds
    pub capital_proceeds: f64,
    /// Cost base
    pub cost_base: f64,
    /// Gross capital gain (before discount)
    pub gross_gain: f64,
    /// Capital loss (if applicable)
    pub capital_loss: f64,
    /// Discount applied
    pub discount: CgtDiscount,
    /// Net capital gain (after discount)
    pub net_gain: f64,
    /// Exemptions applied
    pub exemptions: Vec<CgtExemption>,
    /// Exempt portion (0.0 to 1.0)
    pub exempt_portion: f64,
}

/// Validate CGT event
pub fn validate_cgt_event(
    asset: &CgtAsset,
    _event: CgtEvent,
    disposal_date: NaiveDate,
) -> Result<()> {
    // Check asset is identified
    if asset.description.trim().is_empty() {
        return Err(TaxError::CgtAssetNotIdentified);
    }

    // Check disposal is after acquisition
    if disposal_date < asset.acquisition_date {
        return Err(TaxError::ValidationError {
            message: "Disposal date cannot be before acquisition date".to_string(),
        });
    }

    Ok(())
}

/// Calculate capital gain
pub fn calculate_capital_gain(
    asset: &CgtAsset,
    capital_proceeds: f64,
    disposal_date: NaiveDate,
    discount: CgtDiscount,
) -> Result<CgtCalculation> {
    let cost_base = asset.cost_base.total();
    let mut exemptions = Vec::new();
    let mut exempt_portion = 0.0;

    // Check main residence exemption
    if asset.is_main_residence {
        if let (Some(mr_days), Some(total_days)) =
            (asset.main_residence_days, asset.total_days_owned)
        {
            // Partial exemption
            exempt_portion = mr_days as f64 / total_days as f64;
        } else {
            // Full exemption
            exempt_portion = 1.0;
        }
        exemptions.push(CgtExemption::MainResidence);
    }

    // Check personal use asset exemption
    if matches!(asset.asset_type, CgtAssetType::PersonalUse) && capital_proceeds < 10_000.0 {
        exempt_portion = 1.0;
        exemptions.push(CgtExemption::PersonalUseUnder10k);
    }

    // Check collectable exemption
    if matches!(asset.asset_type, CgtAssetType::Collectables) && capital_proceeds < 500.0 {
        exempt_portion = 1.0;
        exemptions.push(CgtExemption::CollectableUnder500);
    }

    // Calculate gross gain/loss
    let taxable_proceeds = capital_proceeds * (1.0 - exempt_portion);
    let taxable_cost_base = cost_base * (1.0 - exempt_portion);

    let (gross_gain, capital_loss) = if taxable_proceeds > taxable_cost_base {
        (taxable_proceeds - taxable_cost_base, 0.0)
    } else {
        (0.0, taxable_cost_base - taxable_proceeds)
    };

    // Apply discount if applicable
    let eligible = asset.eligible_for_discount(disposal_date);
    let actual_discount = if eligible && gross_gain > 0.0 {
        discount
    } else {
        CgtDiscount::None
    };

    let net_gain = gross_gain * (1.0 - actual_discount.rate());

    Ok(CgtCalculation {
        event: CgtEvent::A1Disposal,
        capital_proceeds,
        cost_base,
        gross_gain,
        capital_loss,
        discount: actual_discount,
        net_gain,
        exemptions,
        exempt_portion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_asset() -> CgtAsset {
        CgtAsset {
            description: "Investment property".to_string(),
            asset_type: CgtAssetType::RealProperty,
            acquisition_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            cost_base: CostBase {
                acquisition_cost: 500_000.0,
                incidental_costs: 20_000.0, // Stamp duty, legal
                ownership_costs: 0.0,
                capital_expenditure: 50_000.0, // Renovations
                title_costs: 2_000.0,
            },
            is_main_residence: false,
            main_residence_days: None,
            total_days_owned: None,
        }
    }

    #[test]
    fn test_cost_base_total() {
        let asset = create_test_asset();
        assert_eq!(asset.cost_base.total(), 572_000.0);
    }

    #[test]
    fn test_holding_period() {
        let asset = create_test_asset();
        let disposal = NaiveDate::from_ymd_opt(2022, 6, 15).unwrap();

        let months = asset.holding_period_months(disposal);
        assert_eq!(months, 29); // 2 years 5 months
        assert!(asset.eligible_for_discount(disposal));
    }

    #[test]
    fn test_holding_period_under_12_months() {
        let asset = create_test_asset();
        let disposal = NaiveDate::from_ymd_opt(2020, 10, 1).unwrap();

        let months = asset.holding_period_months(disposal);
        assert_eq!(months, 9);
        assert!(!asset.eligible_for_discount(disposal));
    }

    #[test]
    fn test_calculate_capital_gain_with_discount() {
        let asset = create_test_asset();
        let disposal_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let result =
            calculate_capital_gain(&asset, 800_000.0, disposal_date, CgtDiscount::Individual)
                .unwrap();

        // Gross gain = 800,000 - 572,000 = 228,000
        assert!((result.gross_gain - 228_000.0).abs() < 0.01);

        // Net gain with 50% discount = 114,000
        assert!((result.net_gain - 114_000.0).abs() < 0.01);
        assert_eq!(result.discount, CgtDiscount::Individual);
    }

    #[test]
    fn test_calculate_capital_loss() {
        let asset = create_test_asset();
        let disposal_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let result =
            calculate_capital_gain(&asset, 500_000.0, disposal_date, CgtDiscount::Individual)
                .unwrap();

        assert_eq!(result.gross_gain, 0.0);
        assert!((result.capital_loss - 72_000.0).abs() < 0.01);
    }

    #[test]
    fn test_main_residence_exemption() {
        let mut asset = create_test_asset();
        asset.is_main_residence = true;

        let disposal_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let result =
            calculate_capital_gain(&asset, 800_000.0, disposal_date, CgtDiscount::Individual)
                .unwrap();

        assert_eq!(result.exempt_portion, 1.0);
        assert_eq!(result.net_gain, 0.0);
        assert!(result.exemptions.contains(&CgtExemption::MainResidence));
    }

    #[test]
    fn test_cgt_discount_rates() {
        assert_eq!(CgtDiscount::Individual.rate(), 0.50);
        assert!((CgtDiscount::SuperannuationFund.rate() - 0.333).abs() < 0.001);
        assert_eq!(CgtDiscount::None.rate(), 0.0);
    }

    #[test]
    fn test_validate_cgt_event() {
        let asset = create_test_asset();
        let disposal_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        assert!(validate_cgt_event(&asset, CgtEvent::A1Disposal, disposal_date).is_ok());
    }
}
