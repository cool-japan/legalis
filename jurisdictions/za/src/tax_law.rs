//! South African Tax Law
//!
//! Comprehensive tax system administered by the South African Revenue Service (SARS).
//!
//! ## Key Legislation
//!
//! - Income Tax Act 58 of 1962
//! - Value-Added Tax Act 89 of 1991
//! - Tax Administration Act 28 of 2011
//! - Customs and Excise Act 91 of 1964
//!
//! ## Tax Types
//!
//! - Personal Income Tax (progressive rates)
//! - Corporate Income Tax (27% standard rate)
//! - VAT (15% standard rate)
//! - Capital Gains Tax
//! - Dividends Tax (20%)
//! - Transfer Duty (property)
//! - Estate Duty

use crate::common::Zar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for tax operations
pub type TaxResult<T> = Result<T, TaxError>;

/// Tax types in South Africa
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxType {
    /// Personal Income Tax (PIT)
    PersonalIncomeTax,
    /// Corporate Income Tax (CIT) - 27% standard
    CorporateIncomeTax,
    /// Value-Added Tax (VAT) - 15%
    Vat,
    /// Capital Gains Tax (CGT) - 40% inclusion rate for individuals
    CapitalGainsTax,
    /// Dividends Tax - 20%
    DividendsTax,
    /// Transfer Duty (property transfer)
    TransferDuty,
    /// Estate Duty - 20%/25%
    EstateDuty,
    /// Donations Tax - 20%/25%
    DonationsTax,
    /// Securities Transfer Tax (STT) - 0.25%
    SecuritiesTransferTax,
    /// Skills Development Levy - 1%
    SkillsDevelopmentLevy,
    /// Unemployment Insurance Fund (UIF) - 2%
    UnemploymentInsurance,
}

/// VAT registration requirements (s23 VAT Act)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRegistration {
    /// Annual taxable supplies
    pub annual_taxable_supplies: Zar,
    /// Mandatory registration threshold (R1 million)
    pub mandatory_threshold: Zar,
    /// Voluntary registration threshold (R50,000)
    pub voluntary_threshold: Zar,
    /// Is registered
    pub is_registered: bool,
    /// VAT number
    pub vat_number: Option<String>,
}

impl VatRegistration {
    /// Create new VAT registration status
    pub fn new(annual_taxable_supplies: Zar) -> Self {
        Self {
            annual_taxable_supplies,
            mandatory_threshold: Zar::from_rands(1_000_000),
            voluntary_threshold: Zar::from_rands(50_000),
            is_registered: false,
            vat_number: None,
        }
    }

    /// Check if mandatory registration required
    pub fn requires_mandatory_registration(&self) -> bool {
        self.annual_taxable_supplies.cents() >= self.mandatory_threshold.cents()
    }

    /// Check if eligible for voluntary registration
    pub fn eligible_for_voluntary_registration(&self) -> bool {
        self.annual_taxable_supplies.cents() >= self.voluntary_threshold.cents()
    }

    /// Calculate VAT on amount (15%)
    pub fn calculate_vat(amount: Zar) -> Zar {
        Zar::from_cents((amount.cents() as f64 * 0.15) as i64)
    }

    /// Calculate amount including VAT
    pub fn amount_including_vat(exclusive_amount: Zar) -> Zar {
        Zar::from_cents(((exclusive_amount.cents() as f64 * 1.15).round()) as i64)
    }

    /// Extract VAT from VAT-inclusive amount
    pub fn extract_vat(inclusive_amount: Zar) -> Zar {
        let exclusive = (inclusive_amount.cents() as f64 / 1.15) as i64;
        Zar::from_cents(inclusive_amount.cents() - exclusive)
    }
}

/// VAT supply types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatSupplyType {
    /// Standard rated (15%)
    StandardRated,
    /// Zero-rated (0%) - Exports, basic foodstuffs
    ZeroRated,
    /// Exempt - Financial services, residential rent
    Exempt,
}

impl VatSupplyType {
    /// Get VAT rate percentage
    pub fn rate(&self) -> f64 {
        match self {
            Self::StandardRated => 15.0,
            Self::ZeroRated => 0.0,
            Self::Exempt => 0.0,
        }
    }

    /// Can claim input VAT
    pub fn can_claim_input_vat(&self) -> bool {
        matches!(self, Self::StandardRated | Self::ZeroRated)
    }
}

/// Personal income tax brackets (2024/2025 tax year)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalIncomeTaxBracket {
    /// Lower threshold (inclusive)
    pub threshold_from: Zar,
    /// Upper threshold (exclusive)
    pub threshold_to: Option<Zar>,
    /// Marginal tax rate (percentage)
    pub rate: f64,
    /// Base tax amount
    pub base_tax: Zar,
}

impl PersonalIncomeTaxBracket {
    /// Get 2024/2025 tax brackets
    pub fn tax_brackets_2024() -> Vec<Self> {
        vec![
            Self {
                threshold_from: Zar::from_rands(0),
                threshold_to: Some(Zar::from_rands(237_100)),
                rate: 18.0,
                base_tax: Zar::from_rands(0),
            },
            Self {
                threshold_from: Zar::from_rands(237_100),
                threshold_to: Some(Zar::from_rands(370_500)),
                rate: 26.0,
                base_tax: Zar::from_rands(42_678),
            },
            Self {
                threshold_from: Zar::from_rands(370_500),
                threshold_to: Some(Zar::from_rands(512_800)),
                rate: 31.0,
                base_tax: Zar::from_rands(77_362),
            },
            Self {
                threshold_from: Zar::from_rands(512_800),
                threshold_to: Some(Zar::from_rands(673_000)),
                rate: 36.0,
                base_tax: Zar::from_rands(121_475),
            },
            Self {
                threshold_from: Zar::from_rands(673_000),
                threshold_to: Some(Zar::from_rands(857_900)),
                rate: 39.0,
                base_tax: Zar::from_rands(179_147),
            },
            Self {
                threshold_from: Zar::from_rands(857_900),
                threshold_to: Some(Zar::from_rands(1_817_000)),
                rate: 41.0,
                base_tax: Zar::from_rands(251_258),
            },
            Self {
                threshold_from: Zar::from_rands(1_817_000),
                threshold_to: None,
                rate: 45.0,
                base_tax: Zar::from_rands(644_489),
            },
        ]
    }

    /// Calculate personal income tax
    pub fn calculate_tax(taxable_income: Zar) -> Zar {
        let brackets = Self::tax_brackets_2024();

        for bracket in &brackets {
            let in_bracket = if let Some(to) = bracket.threshold_to {
                taxable_income.cents() >= bracket.threshold_from.cents()
                    && taxable_income.cents() < to.cents()
            } else {
                taxable_income.cents() >= bracket.threshold_from.cents()
            };

            if in_bracket {
                let taxable_in_bracket = taxable_income.cents() - bracket.threshold_from.cents();
                let tax_on_bracket = (taxable_in_bracket as f64 * bracket.rate / 100.0) as i64;
                return Zar::from_cents(bracket.base_tax.cents() + tax_on_bracket);
            }
        }

        Zar::from_rands(0)
    }
}

/// Tax rebates (2024/2025)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRebate {
    /// Primary rebate (under 65)
    pub primary_rebate: Zar,
    /// Secondary rebate (65-75)
    pub secondary_rebate: Zar,
    /// Tertiary rebate (75+)
    pub tertiary_rebate: Zar,
}

impl TaxRebate {
    /// Get 2024/2025 rebates
    pub fn rebates_2024() -> Self {
        Self {
            primary_rebate: Zar::from_rands(17_235),
            secondary_rebate: Zar::from_rands(9_444),
            tertiary_rebate: Zar::from_rands(3_145),
        }
    }

    /// Calculate total rebate based on age
    pub fn total_rebate(age: u8) -> Zar {
        let rebates = Self::rebates_2024();
        if age >= 75 {
            Zar::from_cents(
                rebates.primary_rebate.cents()
                    + rebates.secondary_rebate.cents()
                    + rebates.tertiary_rebate.cents(),
            )
        } else if age >= 65 {
            Zar::from_cents(rebates.primary_rebate.cents() + rebates.secondary_rebate.cents())
        } else {
            rebates.primary_rebate
        }
    }
}

/// SARS filing requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarsFilingRequirement {
    /// Must file tax return
    pub must_file: bool,
    /// Tax clearance required
    pub tax_clearance_required: bool,
    /// Provisional tax required
    pub provisional_tax: bool,
    /// VAT returns required (monthly/bi-monthly)
    pub vat_returns: bool,
    /// PAYE registered
    pub paye_registered: bool,
}

impl SarsFilingRequirement {
    /// Check if individual must file
    pub fn individual_must_file(gross_income: Zar, age: u8, other_factors: bool) -> bool {
        // Thresholds for 2024/2025
        let threshold = if age >= 75 {
            Zar::from_rands(157_900)
        } else if age >= 65 {
            Zar::from_rands(135_150)
        } else {
            Zar::from_rands(95_750)
        };

        gross_income.cents() > threshold.cents() || other_factors
    }
}

/// Capital Gains Tax inclusion rates
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CgtInclusionRate {
    /// Individuals and special trusts - 40%
    Individual,
    /// Companies - 80%
    Company,
    /// Other trusts - 80%
    Trust,
}

impl CgtInclusionRate {
    /// Get inclusion rate percentage
    pub fn rate(&self) -> f64 {
        match self {
            Self::Individual => 40.0,
            Self::Company => 80.0,
            Self::Trust => 80.0,
        }
    }

    /// Calculate taxable capital gain
    pub fn calculate_taxable_gain(&self, capital_gain: Zar) -> Zar {
        Zar::from_cents((capital_gain.cents() as f64 * self.rate() / 100.0) as i64)
    }

    /// Annual exclusion for individuals (R40,000)
    pub fn annual_exclusion() -> Zar {
        Zar::from_rands(40_000)
    }
}

/// Tax errors
#[derive(Debug, Error)]
pub enum TaxError {
    /// VAT registration required but not done
    #[error("VAT registration required (annual supplies R{supplies} exceed R1,000,000)")]
    VatRegistrationRequired { supplies: i64 },

    /// Tax return not filed
    #[error("Tax return filing required for {tax_year}")]
    ReturnNotFiled { tax_year: String },

    /// Underpayment of tax
    #[error("Tax underpaid: R{underpaid} (owed R{owed}, paid R{paid})")]
    TaxUnderpaid {
        owed: i64,
        paid: i64,
        underpaid: i64,
    },

    /// Late filing penalty
    #[error("Late filing penalty: {description}")]
    LateFilingPenalty { description: String },

    /// Interest on late payment
    #[error("Interest on late payment: {rate}% per month")]
    LatePaymentInterest { rate: f64 },

    /// Invalid tax number
    #[error("Invalid tax number: {tax_number}")]
    InvalidTaxNumber { tax_number: String },
}

/// Validate VAT registration compliance
pub fn validate_vat_registration(registration: &VatRegistration) -> TaxResult<()> {
    if registration.requires_mandatory_registration() && !registration.is_registered {
        return Err(TaxError::VatRegistrationRequired {
            supplies: registration.annual_taxable_supplies.rands(),
        });
    }
    Ok(())
}

/// Get tax compliance checklist
pub fn get_tax_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Register for Income Tax (IRP5/IT3)", "Income Tax Act"),
        ("VAT registration (if turnover > R1m)", "s23 VAT Act"),
        ("File annual tax return", "Tax Admin Act s25"),
        ("Pay provisional tax (if applicable)", "4th Schedule"),
        ("File VAT returns (monthly/bi-monthly)", "VAT Act"),
        ("Register for PAYE (if employer)", "4th Schedule"),
        ("Submit PAYE/SDL/UIF monthly", "PAYE"),
        ("Keep records for 5 years", "s29 Tax Admin Act"),
        ("Obtain tax clearance certificate", "s256 TAA"),
        ("Report capital gains", "8th Schedule"),
        ("Pay dividends tax", "Dividends Tax Act"),
        ("Transfer duty on property", "Transfer Duty Act"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_calculation() {
        let amount = Zar::from_rands(1000);
        let vat = VatRegistration::calculate_vat(amount);
        assert_eq!(vat.rands(), 150); // 15%
    }

    #[test]
    fn test_vat_inclusive() {
        let exclusive = Zar::from_rands(1000);
        let inclusive = VatRegistration::amount_including_vat(exclusive);
        assert_eq!(inclusive.rands(), 1150);
    }

    #[test]
    fn test_vat_extraction() {
        let inclusive = Zar::from_rands(1150);
        let vat = VatRegistration::extract_vat(inclusive);
        assert_eq!(vat.rands(), 150);
    }

    #[test]
    fn test_vat_mandatory_registration() {
        let registration = VatRegistration::new(Zar::from_rands(1_500_000));
        assert!(registration.requires_mandatory_registration());
    }

    #[test]
    fn test_vat_voluntary_registration() {
        let registration = VatRegistration::new(Zar::from_rands(100_000));
        assert!(!registration.requires_mandatory_registration());
        assert!(registration.eligible_for_voluntary_registration());
    }

    #[test]
    fn test_vat_supply_types() {
        assert_eq!(VatSupplyType::StandardRated.rate(), 15.0);
        assert_eq!(VatSupplyType::ZeroRated.rate(), 0.0);
        assert!(VatSupplyType::StandardRated.can_claim_input_vat());
        assert!(!VatSupplyType::Exempt.can_claim_input_vat());
    }

    #[test]
    fn test_personal_tax_calculation() {
        let income = Zar::from_rands(500_000);
        let tax = PersonalIncomeTaxBracket::calculate_tax(income);
        assert!(tax.cents() > 0);
    }

    #[test]
    fn test_tax_rebates() {
        let rebate_under_65 = TaxRebate::total_rebate(40);
        let rebate_65_75 = TaxRebate::total_rebate(70);
        let rebate_over_75 = TaxRebate::total_rebate(80);

        assert!(rebate_65_75.cents() > rebate_under_65.cents());
        assert!(rebate_over_75.cents() > rebate_65_75.cents());
    }

    #[test]
    fn test_cgt_inclusion_rates() {
        assert_eq!(CgtInclusionRate::Individual.rate(), 40.0);
        assert_eq!(CgtInclusionRate::Company.rate(), 80.0);

        let gain = Zar::from_rands(100_000);
        let taxable = CgtInclusionRate::Individual.calculate_taxable_gain(gain);
        assert_eq!(taxable.rands(), 40_000);
    }

    #[test]
    fn test_filing_requirements() {
        assert!(SarsFilingRequirement::individual_must_file(
            Zar::from_rands(200_000),
            30,
            false
        ));
        assert!(!SarsFilingRequirement::individual_must_file(
            Zar::from_rands(50_000),
            30,
            false
        ));
    }

    #[test]
    fn test_validate_vat_registration() {
        let mut registration = VatRegistration::new(Zar::from_rands(1_500_000));
        assert!(validate_vat_registration(&registration).is_err());

        registration.is_registered = true;
        assert!(validate_vat_registration(&registration).is_ok());
    }

    #[test]
    fn test_tax_checklist() {
        let checklist = get_tax_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
