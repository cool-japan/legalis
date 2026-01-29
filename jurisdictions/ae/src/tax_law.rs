//! UAE Tax Law
//!
//! The UAE introduced two major taxes in recent years:
//! - **VAT** (Value Added Tax) - 5% since 2018
//! - **Corporate Tax** - 9% on profits above AED 375,000 since June 2023
//!
//! ## VAT Law
//!
//! Federal Decree-Law No. 8/2017 on Value Added Tax
//! - Standard rate: 5%
//! - Zero-rated: Exports, international transport, certain healthcare/education
//! - Exempt: Residential property, bare land, local passenger transport
//!
//! ## Corporate Tax Law
//!
//! Federal Decree-Law No. 47/2022 on Taxation of Corporations and Businesses
//! - 0% on taxable income up to AED 375,000
//! - 9% on taxable income above AED 375,000
//! - Effective from June 1, 2023
//! - Free zones: 0% on "qualifying income" (conditions apply)

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for tax law operations
pub type TaxResult<T> = Result<T, TaxError>;

/// VAT registration thresholds
pub const VAT_MANDATORY_THRESHOLD: i64 = 375_000; // AED per annum
pub const VAT_VOLUNTARY_THRESHOLD: i64 = 187_500; // AED per annum

/// Corporate tax threshold
pub const CORPORATE_TAX_THRESHOLD: i64 = 375_000; // AED per annum

/// VAT rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRate {
    /// Standard rate (5%)
    Standard,
    /// Zero-rated (0%)
    ZeroRated,
    /// Exempt (no VAT)
    Exempt,
    /// Out of scope (not subject to UAE VAT)
    OutOfScope,
}

impl VatRate {
    /// Get percentage rate
    pub fn percentage(&self) -> f64 {
        match self {
            Self::Standard => 5.0,
            Self::ZeroRated => 0.0,
            Self::Exempt => 0.0,
            Self::OutOfScope => 0.0,
        }
    }

    /// Calculate VAT amount
    pub fn calculate_vat(&self, amount: Aed) -> Aed {
        match self {
            Self::Standard => Aed::from_fils(amount.fils() * 5 / 100),
            Self::ZeroRated | Self::Exempt | Self::OutOfScope => Aed::from_fils(0),
        }
    }

    /// Get name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Standard => "النسبة الأساسية (5%)",
            Self::ZeroRated => "نسبة الصفر (0%)",
            Self::Exempt => "معفى",
            Self::OutOfScope => "خارج النطاق",
        }
    }

    /// Get name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Standard => "Standard Rate (5%)",
            Self::ZeroRated => "Zero-Rated (0%)",
            Self::Exempt => "Exempt",
            Self::OutOfScope => "Out of Scope",
        }
    }
}

/// VAT supply categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatSupplyCategory {
    /// Goods (tangible movable property)
    Goods,
    /// Services
    Services,
    /// Real property (land and buildings)
    RealProperty,
    /// Financial services
    FinancialServices,
    /// Healthcare
    Healthcare,
    /// Education
    Education,
    /// International transport
    InternationalTransport,
    /// Exports
    Exports,
}

impl VatSupplyCategory {
    /// Get applicable VAT rate for category
    pub fn default_vat_rate(&self) -> VatRate {
        match self {
            Self::Goods | Self::Services => VatRate::Standard,
            Self::RealProperty => VatRate::Exempt, // Residential property
            Self::FinancialServices => VatRate::Exempt,
            Self::Healthcare => VatRate::ZeroRated, // Certain healthcare
            Self::Education => VatRate::ZeroRated,  // Certain education
            Self::InternationalTransport => VatRate::ZeroRated,
            Self::Exports => VatRate::ZeroRated,
        }
    }

    /// Get category name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Goods => "Goods",
            Self::Services => "Services",
            Self::RealProperty => "Real Property",
            Self::FinancialServices => "Financial Services",
            Self::Healthcare => "Healthcare",
            Self::Education => "Education",
            Self::InternationalTransport => "International Transport",
            Self::Exports => "Exports",
        }
    }
}

/// VAT registration status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRegistrationStatus {
    /// Mandatory registration required
    Mandatory,
    /// Voluntary registration allowed
    Voluntary,
    /// Not required
    NotRequired,
}

/// VAT registration assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRegistration {
    /// Annual taxable supplies (AED)
    pub annual_taxable_supplies: Aed,
    /// Is business in free zone
    pub is_free_zone: bool,
    /// Registration status
    pub status: VatRegistrationStatus,
    /// TRN (Tax Registration Number) if registered
    pub trn: Option<String>,
}

impl VatRegistration {
    /// Assess VAT registration requirement
    pub fn assess(annual_taxable_supplies: Aed, is_free_zone: bool) -> Self {
        let status = if annual_taxable_supplies.dirhams() >= VAT_MANDATORY_THRESHOLD {
            VatRegistrationStatus::Mandatory
        } else if annual_taxable_supplies.dirhams() >= VAT_VOLUNTARY_THRESHOLD {
            VatRegistrationStatus::Voluntary
        } else {
            VatRegistrationStatus::NotRequired
        };

        Self {
            annual_taxable_supplies,
            is_free_zone,
            status,
            trn: None,
        }
    }

    /// Check if VAT registration is required
    pub fn is_required(&self) -> bool {
        matches!(self.status, VatRegistrationStatus::Mandatory)
    }
}

/// Corporate tax rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorporateTaxRate {
    /// Small business relief (0% up to AED 375,000)
    SmallBusinessRelief,
    /// Standard rate (9% above AED 375,000)
    Standard,
    /// Free zone qualifying income (0%)
    FreeZoneQualifying,
}

impl CorporateTaxRate {
    /// Get percentage rate
    pub fn percentage(&self) -> f64 {
        match self {
            Self::SmallBusinessRelief | Self::FreeZoneQualifying => 0.0,
            Self::Standard => 9.0,
        }
    }

    /// Get name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::SmallBusinessRelief => "إعفاء المؤسسات الصغيرة (0%)",
            Self::Standard => "النسبة الأساسية (9%)",
            Self::FreeZoneQualifying => "دخل المنطقة الحرة المؤهل (0%)",
        }
    }

    /// Get name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::SmallBusinessRelief => "Small Business Relief (0%)",
            Self::Standard => "Standard Rate (9%)",
            Self::FreeZoneQualifying => "Free Zone Qualifying Income (0%)",
        }
    }
}

/// Corporate tax calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateTax {
    /// Taxable income (AED)
    pub taxable_income: Aed,
    /// Is qualifying free zone person
    pub is_qualifying_free_zone_person: bool,
    /// Tax rate applied
    pub tax_rate: CorporateTaxRate,
    /// Tax payable
    pub tax_payable: Aed,
}

impl CorporateTax {
    /// Calculate corporate tax
    pub fn calculate(taxable_income: Aed, is_qualifying_free_zone_person: bool) -> Self {
        if is_qualifying_free_zone_person {
            // Qualifying free zone income is taxed at 0%
            Self {
                taxable_income,
                is_qualifying_free_zone_person: true,
                tax_rate: CorporateTaxRate::FreeZoneQualifying,
                tax_payable: Aed::from_fils(0),
            }
        } else if taxable_income.dirhams() <= CORPORATE_TAX_THRESHOLD {
            // Small business relief
            Self {
                taxable_income,
                is_qualifying_free_zone_person: false,
                tax_rate: CorporateTaxRate::SmallBusinessRelief,
                tax_payable: Aed::from_fils(0),
            }
        } else {
            // Standard rate (9% on full amount above threshold)
            let taxable_above_threshold = taxable_income.dirhams() - CORPORATE_TAX_THRESHOLD;
            let tax = (taxable_above_threshold * 9) / 100;

            Self {
                taxable_income,
                is_qualifying_free_zone_person: false,
                tax_rate: CorporateTaxRate::Standard,
                tax_payable: Aed::from_dirhams(tax),
            }
        }
    }

    /// Get effective tax rate
    pub fn effective_rate(&self) -> f64 {
        if self.taxable_income.dirhams() == 0 {
            return 0.0;
        }

        (self.tax_payable.dirhams() as f64 / self.taxable_income.dirhams() as f64) * 100.0
    }
}

/// Tax errors
#[derive(Debug, Error)]
pub enum TaxError {
    /// VAT registration violation
    #[error("مخالفة تسجيل ضريبة القيمة المضافة: {description}")]
    VatRegistrationViolation { description: String },

    /// Corporate tax filing error
    #[error("خطأ في تقديم الإقرار الضريبي للشركات: {description}")]
    CorporateTaxFilingError { description: String },

    /// Tax calculation error
    #[error("خطأ في حساب الضريبة: {reason}")]
    CalculationError { reason: String },

    /// Invalid TRN
    #[error("رقم التسجيل الضريبي غير صالح: {trn}")]
    InvalidTrn { trn: String },
}

/// Validate TRN (Tax Registration Number) format
///
/// UAE TRN is 15 digits
pub fn validate_trn(trn: &str) -> TaxResult<()> {
    if trn.len() != 15 {
        return Err(TaxError::InvalidTrn {
            trn: trn.to_string(),
        });
    }

    if !trn.chars().all(|c| c.is_ascii_digit()) {
        return Err(TaxError::InvalidTrn {
            trn: trn.to_string(),
        });
    }

    Ok(())
}

/// Get VAT compliance checklist
pub fn get_vat_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("التسجيل الضريبي", "VAT Registration (if required)"),
        ("فواتير ضريبية", "Tax invoices with TRN"),
        ("إقرار ضريبي", "VAT return (quarterly/monthly)"),
        ("السجلات المحاسبية", "Accounting records (5 years)"),
        ("فاتورة إلكترونية", "Electronic invoicing (from 2026)"),
        ("دفتر الضرائب", "Tax account at FTA"),
        ("احتساب الضريبة", "Correct VAT calculation"),
        ("سداد الضريبة", "Timely tax payment"),
    ]
}

/// Get corporate tax compliance checklist
pub fn get_corporate_tax_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("التسجيل الضريبي", "Tax registration"),
        ("السنة المالية", "Financial year determination"),
        ("الإقرار الضريبي", "Tax return filing (9 months after FYE)"),
        ("دفع الضريبة", "Tax payment"),
        ("نقل الأسعار", "Transfer pricing documentation"),
        ("العقود المرتبطة", "Related party transactions"),
        ("الحوافز الحرة", "Free zone election (if applicable)"),
        ("السجلات", "Accounting records maintenance"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_rates() {
        assert_eq!(VatRate::Standard.percentage(), 5.0);
        assert_eq!(VatRate::ZeroRated.percentage(), 0.0);

        let amount = Aed::from_dirhams(1000);
        let vat = VatRate::Standard.calculate_vat(amount);
        assert_eq!(vat.dirhams(), 50); // 5% of 1000
    }

    #[test]
    fn test_vat_supply_categories() {
        assert_eq!(
            VatSupplyCategory::Exports.default_vat_rate(),
            VatRate::ZeroRated
        );
        assert_eq!(
            VatSupplyCategory::FinancialServices.default_vat_rate(),
            VatRate::Exempt
        );
        assert_eq!(
            VatSupplyCategory::Goods.default_vat_rate(),
            VatRate::Standard
        );
    }

    #[test]
    fn test_vat_registration_mandatory() {
        let reg = VatRegistration::assess(Aed::from_dirhams(500_000), false);
        assert!(reg.is_required());
        assert_eq!(reg.status, VatRegistrationStatus::Mandatory);
    }

    #[test]
    fn test_vat_registration_voluntary() {
        let reg = VatRegistration::assess(Aed::from_dirhams(200_000), false);
        assert!(!reg.is_required());
        assert_eq!(reg.status, VatRegistrationStatus::Voluntary);
    }

    #[test]
    fn test_vat_registration_not_required() {
        let reg = VatRegistration::assess(Aed::from_dirhams(100_000), false);
        assert!(!reg.is_required());
        assert_eq!(reg.status, VatRegistrationStatus::NotRequired);
    }

    #[test]
    fn test_corporate_tax_small_business() {
        let tax = CorporateTax::calculate(Aed::from_dirhams(300_000), false);
        assert_eq!(tax.tax_payable.dirhams(), 0);
        assert_eq!(tax.effective_rate(), 0.0);
    }

    #[test]
    fn test_corporate_tax_standard() {
        let tax = CorporateTax::calculate(Aed::from_dirhams(1_000_000), false);
        // (1,000,000 - 375,000) * 9% = 56,250
        assert_eq!(tax.tax_payable.dirhams(), 56_250);
        assert!(tax.effective_rate() > 0.0);
        assert!(tax.effective_rate() < 9.0); // Effective rate is less than marginal
    }

    #[test]
    fn test_corporate_tax_free_zone() {
        let tax = CorporateTax::calculate(Aed::from_dirhams(5_000_000), true);
        assert_eq!(tax.tax_payable.dirhams(), 0);
        assert_eq!(tax.tax_rate, CorporateTaxRate::FreeZoneQualifying);
    }

    #[test]
    fn test_validate_trn_valid() {
        assert!(validate_trn("100000000000003").is_ok());
    }

    #[test]
    fn test_validate_trn_invalid_length() {
        assert!(validate_trn("12345").is_err());
        assert!(validate_trn("1234567890123456").is_err());
    }

    #[test]
    fn test_validate_trn_invalid_characters() {
        assert!(validate_trn("10000000000000A").is_err());
    }

    #[test]
    fn test_vat_checklist() {
        let checklist = get_vat_checklist();
        assert!(!checklist.is_empty());
    }

    #[test]
    fn test_corporate_tax_checklist() {
        let checklist = get_corporate_tax_checklist();
        assert!(!checklist.is_empty());
    }

    #[test]
    fn test_corporate_tax_at_threshold() {
        let tax = CorporateTax::calculate(Aed::from_dirhams(CORPORATE_TAX_THRESHOLD), false);
        assert_eq!(tax.tax_payable.dirhams(), 0);
    }

    #[test]
    fn test_corporate_tax_just_above_threshold() {
        let tax = CorporateTax::calculate(Aed::from_dirhams(CORPORATE_TAX_THRESHOLD + 100), false);
        assert_eq!(tax.tax_payable.dirhams(), 9); // 9% of 100
    }
}
