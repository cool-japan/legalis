//! Goods and Services Tax (GST Act 1999)
//!
//! Australian GST at 10% on taxable supplies.
//!
//! ## Key Concepts
//!
//! - **Taxable supply**: GST of 10% applies
//! - **GST-free supply**: No GST charged, but ITCs available
//! - **Input-taxed supply**: No GST charged, no ITCs available
//!
//! ## GST Registration
//!
//! Required if annual turnover exceeds $75,000 ($150,000 for non-profits).

use super::error::{Result, TaxError};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// GST rate (10%)
pub const GST_RATE: f64 = 0.10;

/// GST registration threshold
pub const GST_THRESHOLD: f64 = 75_000.0;

/// GST registration threshold for non-profits
pub const GST_THRESHOLD_NONPROFIT: f64 = 150_000.0;

/// Supply type for GST purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupplyType {
    /// Goods
    Goods,
    /// Services
    Services,
    /// Real property
    RealProperty,
    /// Financial supply
    FinancialSupply,
    /// Digital products
    Digital,
}

/// GST status of a supply
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GstStatus {
    /// Taxable supply (10% GST)
    Taxable,
    /// GST-free supply (Division 38)
    GstFree(GstFreeCategory),
    /// Input-taxed supply (Division 40)
    InputTaxed(InputTaxedCategory),
    /// Out of scope (not a supply)
    OutOfScope,
}

impl GstStatus {
    /// Whether ITC is available for acquisitions related to this supply
    pub fn itc_available(&self) -> bool {
        matches!(self, GstStatus::Taxable | GstStatus::GstFree(_))
    }

    /// Whether GST is charged on this supply
    pub fn gst_charged(&self) -> bool {
        matches!(self, GstStatus::Taxable)
    }
}

/// GST-free categories (Division 38)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GstFreeCategory {
    /// Food (basic) - Subdivision 38-A
    BasicFood,
    /// Health services - Subdivision 38-B
    Health,
    /// Education - Subdivision 38-C
    Education,
    /// Child care - Subdivision 38-D
    ChildCare,
    /// Exports - Subdivision 38-E
    Exports,
    /// Religious services - Subdivision 38-F
    Religious,
    /// Charitable activities - Subdivision 38-G
    Charitable,
    /// Water and sewerage - Subdivision 38-H
    WaterSewerage,
    /// International transport - Subdivision 38-I
    InternationalTransport,
    /// Precious metals - Subdivision 38-J
    PreciousMetals,
    /// Going concern - Subdivision 38-J
    GoingConcern,
    /// Farmland - Subdivision 38-K
    Farmland,
    /// Cars for disabled persons - Subdivision 38-L
    DisabledVehicles,
}

impl GstFreeCategory {
    /// Get the subdivision reference
    pub fn subdivision(&self) -> &'static str {
        match self {
            GstFreeCategory::BasicFood => "38-A",
            GstFreeCategory::Health => "38-B",
            GstFreeCategory::Education => "38-C",
            GstFreeCategory::ChildCare => "38-D",
            GstFreeCategory::Exports => "38-E",
            GstFreeCategory::Religious => "38-F",
            GstFreeCategory::Charitable => "38-G",
            GstFreeCategory::WaterSewerage => "38-H",
            GstFreeCategory::InternationalTransport => "38-I",
            GstFreeCategory::PreciousMetals => "38-J",
            GstFreeCategory::GoingConcern => "38-J",
            GstFreeCategory::Farmland => "38-K",
            GstFreeCategory::DisabledVehicles => "38-L",
        }
    }
}

/// Input-taxed categories (Division 40)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputTaxedCategory {
    /// Financial supplies - Subdivision 40-A
    FinancialSupplies,
    /// Residential rent - Subdivision 40-B
    ResidentialRent,
    /// Residential premises sale - Subdivision 40-C
    ResidentialPremisesSale,
}

impl InputTaxedCategory {
    /// Get the subdivision reference
    pub fn subdivision(&self) -> &'static str {
        match self {
            InputTaxedCategory::FinancialSupplies => "40-A",
            InputTaxedCategory::ResidentialRent => "40-B",
            InputTaxedCategory::ResidentialPremisesSale => "40-C",
        }
    }
}

/// Taxable supply details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxableSupply {
    /// Description
    pub description: String,
    /// Supply type
    pub supply_type: SupplyType,
    /// GST status
    pub gst_status: GstStatus,
    /// GST-exclusive amount
    pub gst_exclusive_amount: f64,
    /// GST amount (if taxable)
    pub gst_amount: f64,
    /// GST-inclusive amount
    pub gst_inclusive_amount: f64,
    /// Date of supply
    pub supply_date: NaiveDate,
    /// Tax invoice issued
    pub tax_invoice_issued: bool,
}

impl TaxableSupply {
    /// Create a new taxable supply (GST-exclusive price)
    pub fn new_taxable(
        description: impl Into<String>,
        supply_type: SupplyType,
        gst_exclusive: f64,
        supply_date: NaiveDate,
    ) -> Self {
        let gst = gst_exclusive * GST_RATE;
        Self {
            description: description.into(),
            supply_type,
            gst_status: GstStatus::Taxable,
            gst_exclusive_amount: gst_exclusive,
            gst_amount: gst,
            gst_inclusive_amount: gst_exclusive + gst,
            supply_date,
            tax_invoice_issued: false,
        }
    }

    /// Create a GST-free supply
    pub fn new_gst_free(
        description: impl Into<String>,
        supply_type: SupplyType,
        amount: f64,
        category: GstFreeCategory,
        supply_date: NaiveDate,
    ) -> Self {
        Self {
            description: description.into(),
            supply_type,
            gst_status: GstStatus::GstFree(category),
            gst_exclusive_amount: amount,
            gst_amount: 0.0,
            gst_inclusive_amount: amount,
            supply_date,
            tax_invoice_issued: false,
        }
    }
}

/// Input tax credit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputTaxCredit {
    /// Description of acquisition
    pub description: String,
    /// GST paid
    pub gst_paid: f64,
    /// Creditable percentage (0-100)
    pub creditable_percentage: f64,
    /// ITC amount
    pub itc_amount: f64,
    /// Tax invoice held
    pub tax_invoice_held: bool,
    /// Invoice date
    pub invoice_date: NaiveDate,
}

impl InputTaxCredit {
    /// Calculate ITC amount
    pub fn calculate(gst_paid: f64, creditable_percentage: f64) -> f64 {
        gst_paid * (creditable_percentage / 100.0)
    }
}

/// GST calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GstCalculation {
    /// GST-exclusive amount
    pub gst_exclusive: f64,
    /// GST amount
    pub gst_amount: f64,
    /// GST-inclusive amount
    pub gst_inclusive: f64,
    /// GST rate applied
    pub rate: f64,
}

/// Calculate GST on a supply
pub fn calculate_gst(supply: &TaxableSupply) -> Result<GstCalculation> {
    if !supply.gst_status.gst_charged() {
        return Ok(GstCalculation {
            gst_exclusive: supply.gst_exclusive_amount,
            gst_amount: 0.0,
            gst_inclusive: supply.gst_exclusive_amount,
            rate: 0.0,
        });
    }

    Ok(GstCalculation {
        gst_exclusive: supply.gst_exclusive_amount,
        gst_amount: supply.gst_amount,
        gst_inclusive: supply.gst_inclusive_amount,
        rate: GST_RATE,
    })
}

/// Calculate GST from GST-inclusive price
pub fn calculate_gst_from_inclusive(gst_inclusive: f64) -> GstCalculation {
    let gst_exclusive = gst_inclusive / (1.0 + GST_RATE);
    let gst_amount = gst_inclusive - gst_exclusive;

    GstCalculation {
        gst_exclusive,
        gst_amount,
        gst_inclusive,
        rate: GST_RATE,
    }
}

/// Calculate input tax credit
pub fn calculate_input_tax_credit(
    acquisition: &InputTaxCredit,
    supply_status: &GstStatus,
) -> Result<f64> {
    // Check if ITC is available
    if !supply_status.itc_available() {
        return Err(TaxError::ItcNotAvailable {
            reason: "Acquisitions for input-taxed supplies do not qualify for ITC".to_string(),
        });
    }

    // Check tax invoice
    if !acquisition.tax_invoice_held && acquisition.gst_paid > 82.50 {
        return Err(TaxError::InvalidTaxInvoice {
            missing: "Tax invoice required for acquisitions over $82.50".to_string(),
        });
    }

    Ok(acquisition.itc_amount)
}

/// Validate BAS lodgement
pub fn validate_bas_lodgement(
    period_end: NaiveDate,
    due_date: NaiveDate,
    lodgement_date: Option<NaiveDate>,
) -> Result<()> {
    match lodgement_date {
        Some(date) if date <= due_date => Ok(()),
        Some(_) => Err(TaxError::BasNotLodged {
            period: format!("Period ending {}", period_end),
            due_date: due_date.to_string(),
        }),
        None => Err(TaxError::BasNotLodged {
            period: format!("Period ending {}", period_end),
            due_date: due_date.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gst_taxable() {
        let supply = TaxableSupply::new_taxable(
            "Consulting services",
            SupplyType::Services,
            1000.0,
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        );

        let result = calculate_gst(&supply).unwrap();
        assert_eq!(result.gst_exclusive, 1000.0);
        assert_eq!(result.gst_amount, 100.0);
        assert_eq!(result.gst_inclusive, 1100.0);
    }

    #[test]
    fn test_calculate_gst_free() {
        let supply = TaxableSupply::new_gst_free(
            "Medical services",
            SupplyType::Services,
            500.0,
            GstFreeCategory::Health,
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        );

        let result = calculate_gst(&supply).unwrap();
        assert_eq!(result.gst_amount, 0.0);
        assert_eq!(result.gst_inclusive, 500.0);
    }

    #[test]
    fn test_calculate_gst_from_inclusive() {
        let result = calculate_gst_from_inclusive(110.0);

        assert!((result.gst_exclusive - 100.0).abs() < 0.01);
        assert!((result.gst_amount - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_gst_status_itc_available() {
        assert!(GstStatus::Taxable.itc_available());
        assert!(GstStatus::GstFree(GstFreeCategory::Exports).itc_available());
        assert!(!GstStatus::InputTaxed(InputTaxedCategory::FinancialSupplies).itc_available());
    }

    #[test]
    fn test_input_tax_credit_calculation() {
        let itc = InputTaxCredit {
            description: "Office supplies".to_string(),
            gst_paid: 100.0,
            creditable_percentage: 100.0,
            itc_amount: 100.0,
            tax_invoice_held: true,
            invoice_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        };

        let result = calculate_input_tax_credit(&itc, &GstStatus::Taxable);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100.0);
    }

    #[test]
    fn test_input_tax_credit_not_available() {
        let itc = InputTaxCredit {
            description: "Property for rental".to_string(),
            gst_paid: 50_000.0,
            creditable_percentage: 0.0,
            itc_amount: 0.0,
            tax_invoice_held: true,
            invoice_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        };

        let result = calculate_input_tax_credit(
            &itc,
            &GstStatus::InputTaxed(InputTaxedCategory::ResidentialRent),
        );
        assert!(result.is_err());
    }
}
