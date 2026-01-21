//! Income Tax (ITAA 1997)
//!
//! Implements income tax calculation for individuals and companies.
//!
//! ## Individual Tax Rates (2024-25)
//!
//! | Taxable Income | Rate |
//! |----------------|------|
//! | $0 - $18,200 | Nil |
//! | $18,201 - $45,000 | 19% |
//! | $45,001 - $120,000 | 32.5% |
//! | $120,001 - $180,000 | 37% |
//! | $180,001+ | 45% |
//!
//! ## Company Tax Rates (2024-25)
//!
//! - Base rate entities (turnover <$50M, ≤80% passive income): **25%**
//! - Other companies: **30%**

use super::error::{Result, TaxError};
use super::types::FinancialYear;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Individual tax brackets for 2024-25
const TAX_BRACKETS_2024_25: [(f64, f64, f64); 5] = [
    (0.0, 18_200.0, 0.0),         // 0%
    (18_200.0, 45_000.0, 0.19),   // 19%
    (45_000.0, 120_000.0, 0.325), // 32.5%
    (120_000.0, 180_000.0, 0.37), // 37%
    (180_000.0, f64::MAX, 0.45),  // 45%
];

/// Base tax amounts at each bracket threshold
const TAX_BASE_2024_25: [f64; 5] = [
    0.0,      // At $0
    0.0,      // At $18,200
    5_092.0,  // At $45,000
    29_467.0, // At $120,000
    51_667.0, // At $180,000
];

/// Taxable income details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxableIncome {
    /// Gross assessable income
    pub assessable_income: f64,
    /// Total deductions
    pub deductions: f64,
    /// Tax offsets
    pub tax_offsets: Vec<TaxOffset>,
    /// Financial year
    pub financial_year: FinancialYear,
    /// Medicare levy exemption (full or half)
    pub medicare_levy_exemption: MedicareLevyExemption,
}

impl TaxableIncome {
    /// Calculate taxable income (assessable income - deductions)
    pub fn taxable_income(&self) -> f64 {
        (self.assessable_income - self.deductions).max(0.0)
    }
}

/// Medicare levy exemption status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MedicareLevyExemption {
    /// No exemption (standard 2% levy)
    #[default]
    None,
    /// Full exemption
    Full,
    /// Half exemption
    Half,
}

impl MedicareLevyExemption {
    /// Get the Medicare levy rate
    pub fn levy_rate(&self) -> f64 {
        match self {
            MedicareLevyExemption::None => 0.02, // 2%
            MedicareLevyExemption::Full => 0.0,
            MedicareLevyExemption::Half => 0.01, // 1%
        }
    }
}

/// Tax offset (formerly rebate)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxOffset {
    /// Offset type
    pub offset_type: TaxOffsetType,
    /// Amount
    pub amount: f64,
    /// Whether refundable
    pub refundable: bool,
}

/// Types of tax offsets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxOffsetType {
    /// Low income tax offset (LITO)
    LowIncome,
    /// Low and middle income tax offset (LMITO) - no longer available from 2023
    LowAndMiddleIncome,
    /// Senior Australians and pensioners tax offset (SAPTO)
    SeniorAustralians,
    /// Private health insurance rebate
    PrivateHealthRebate,
    /// Franking credits (imputation)
    FrankingCredits,
    /// Foreign income tax offset
    ForeignIncomeTaxOffset,
    /// Other offset
    Other,
}

impl TaxOffsetType {
    /// Whether this offset is refundable
    pub fn is_refundable(&self) -> bool {
        matches!(self, TaxOffsetType::FrankingCredits)
    }
}

/// Deduction details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deduction {
    /// Deduction category
    pub category: DeductionCategory,
    /// Amount claimed
    pub amount: f64,
    /// Description
    pub description: String,
    /// Whether substantiated
    pub substantiated: bool,
    /// Date incurred
    pub date_incurred: Option<NaiveDate>,
    /// Apportionment (if partly private)
    pub work_percentage: Option<f64>,
}

/// Categories of deductions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeductionCategory {
    /// Work-related expenses (D1-D5)
    WorkRelated,
    /// Self-education expenses (D4)
    SelfEducation,
    /// Home office expenses (D5)
    HomeOffice,
    /// Motor vehicle expenses (D1)
    MotorVehicle,
    /// Travel expenses (D1)
    Travel,
    /// Uniform and clothing (D3)
    UniformClothing,
    /// Tools and equipment (D5)
    ToolsEquipment,
    /// Interest and dividend deductions (D7)
    InterestDividend,
    /// Gifts and donations (D9)
    GiftsDonations,
    /// Tax agent fees (D10)
    TaxAgentFees,
    /// Income protection insurance
    IncomeProtection,
    /// Other deductions
    Other,
}

impl DeductionCategory {
    /// Get the schedule/division reference
    pub fn schedule_reference(&self) -> &'static str {
        match self {
            DeductionCategory::WorkRelated => "D1-D5",
            DeductionCategory::SelfEducation => "D4",
            DeductionCategory::HomeOffice => "D5",
            DeductionCategory::MotorVehicle => "D1",
            DeductionCategory::Travel => "D1",
            DeductionCategory::UniformClothing => "D3",
            DeductionCategory::ToolsEquipment => "D5",
            DeductionCategory::InterestDividend => "D7",
            DeductionCategory::GiftsDonations => "D9",
            DeductionCategory::TaxAgentFees => "D10",
            DeductionCategory::IncomeProtection => "D12",
            DeductionCategory::Other => "D15",
        }
    }
}

/// Income category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncomeCategory {
    /// Salary and wages
    SalaryWages,
    /// Business income
    Business,
    /// Capital gains
    CapitalGains,
    /// Dividends
    Dividends,
    /// Interest
    Interest,
    /// Rental income
    Rental,
    /// Foreign income
    Foreign,
    /// Superannuation
    Superannuation,
    /// Government payments
    GovernmentPayments,
    /// Other income
    Other,
}

/// Tax calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxCalculation {
    /// Taxable income
    pub taxable_income: f64,
    /// Gross tax on taxable income
    pub gross_tax: f64,
    /// Medicare levy
    pub medicare_levy: f64,
    /// Total tax offsets applied
    pub offsets_applied: f64,
    /// Net tax payable
    pub net_tax: f64,
    /// Effective tax rate
    pub effective_rate: f64,
    /// Breakdown by bracket
    pub bracket_breakdown: Vec<BracketTax>,
}

/// Tax at each bracket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BracketTax {
    /// Income in this bracket
    pub income_in_bracket: f64,
    /// Tax rate
    pub rate: f64,
    /// Tax payable
    pub tax: f64,
}

/// Calculate individual income tax
pub fn calculate_individual_tax(income: &TaxableIncome) -> Result<TaxCalculation> {
    let taxable_income = income.taxable_income();

    // Calculate gross tax using brackets
    let (gross_tax, bracket_breakdown) = calculate_bracketed_tax(taxable_income);

    // Calculate Medicare levy
    let medicare_levy = taxable_income * income.medicare_levy_exemption.levy_rate();

    // Apply tax offsets
    let total_offsets: f64 = income.tax_offsets.iter().map(|o| o.amount).sum();
    let refundable_offsets: f64 = income
        .tax_offsets
        .iter()
        .filter(|o| o.refundable || o.offset_type.is_refundable())
        .map(|o| o.amount)
        .sum();

    // Non-refundable offsets can only reduce tax to zero
    let non_refundable = total_offsets - refundable_offsets;
    let tax_after_non_refundable = (gross_tax + medicare_levy - non_refundable).max(0.0);

    // Refundable offsets can create a refund
    let net_tax = tax_after_non_refundable - refundable_offsets;

    let effective_rate = if taxable_income > 0.0 {
        net_tax / taxable_income
    } else {
        0.0
    };

    Ok(TaxCalculation {
        taxable_income,
        gross_tax,
        medicare_levy,
        offsets_applied: total_offsets,
        net_tax,
        effective_rate,
        bracket_breakdown,
    })
}

/// Calculate tax using brackets
fn calculate_bracketed_tax(taxable_income: f64) -> (f64, Vec<BracketTax>) {
    let mut total_tax = 0.0;
    let mut breakdown = Vec::new();

    for (i, (lower, upper, rate)) in TAX_BRACKETS_2024_25.iter().enumerate() {
        if taxable_income > *lower {
            let income_in_bracket = (taxable_income.min(*upper) - lower).max(0.0);
            let tax_in_bracket = income_in_bracket * rate;

            if income_in_bracket > 0.0 {
                breakdown.push(BracketTax {
                    income_in_bracket,
                    rate: *rate,
                    tax: tax_in_bracket,
                });
            }

            // Use the base tax amount for accuracy
            if taxable_income > *upper && i < TAX_BASE_2024_25.len() - 1 {
                total_tax = TAX_BASE_2024_25[i + 1];
            } else {
                total_tax = if i > 0 {
                    TAX_BASE_2024_25[i] + income_in_bracket * rate
                } else {
                    income_in_bracket * rate
                };
            }
        }
    }

    (total_tax, breakdown)
}

/// Calculate company tax
pub fn calculate_company_tax(
    taxable_income: f64,
    is_base_rate_entity: bool,
) -> Result<TaxCalculation> {
    let rate = if is_base_rate_entity { 0.25 } else { 0.30 };
    let gross_tax = taxable_income * rate;

    Ok(TaxCalculation {
        taxable_income,
        gross_tax,
        medicare_levy: 0.0, // Companies don't pay Medicare levy
        offsets_applied: 0.0,
        net_tax: gross_tax,
        effective_rate: rate,
        bracket_breakdown: vec![BracketTax {
            income_in_bracket: taxable_income,
            rate,
            tax: gross_tax,
        }],
    })
}

/// Validate a deduction claim
pub fn validate_deduction(deduction: &Deduction) -> Result<()> {
    // Check amount is positive
    if deduction.amount <= 0.0 {
        return Err(TaxError::ValidationError {
            message: "Deduction amount must be positive".to_string(),
        });
    }

    // Check substantiation for amounts > $300 (work-related)
    if matches!(
        deduction.category,
        DeductionCategory::WorkRelated
            | DeductionCategory::ToolsEquipment
            | DeductionCategory::UniformClothing
    ) && deduction.amount > 300.0
        && !deduction.substantiated
    {
        return Err(TaxError::InsufficientSubstantiation {
            deduction: deduction.description.clone(),
            required: "Written evidence for amounts over $300".to_string(),
        });
    }

    // Check work percentage if provided
    if let Some(pct) = deduction.work_percentage
        && !(0.0..=100.0).contains(&pct)
    {
        return Err(TaxError::ValidationError {
            message: "Work percentage must be between 0 and 100".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_individual_tax_nil() {
        let income = TaxableIncome {
            assessable_income: 18_000.0,
            deductions: 0.0,
            tax_offsets: vec![],
            financial_year: FinancialYear::FY2024_25,
            medicare_levy_exemption: MedicareLevyExemption::Full,
        };

        let result = calculate_individual_tax(&income).unwrap();
        assert_eq!(result.gross_tax, 0.0);
    }

    #[test]
    fn test_calculate_individual_tax_19_percent() {
        let income = TaxableIncome {
            assessable_income: 45_000.0,
            deductions: 0.0,
            tax_offsets: vec![],
            financial_year: FinancialYear::FY2024_25,
            medicare_levy_exemption: MedicareLevyExemption::None,
        };

        let result = calculate_individual_tax(&income).unwrap();
        // Tax should be 19% of ($45,000 - $18,200) = 19% of $26,800 = $5,092
        assert!((result.gross_tax - 5_092.0).abs() < 1.0);
    }

    #[test]
    fn test_calculate_individual_tax_120k() {
        let income = TaxableIncome {
            assessable_income: 120_000.0,
            deductions: 0.0,
            tax_offsets: vec![],
            financial_year: FinancialYear::FY2024_25,
            medicare_levy_exemption: MedicareLevyExemption::None,
        };

        let result = calculate_individual_tax(&income).unwrap();
        // Tax should be $29,467
        assert!((result.gross_tax - 29_467.0).abs() < 1.0);
    }

    #[test]
    fn test_calculate_company_tax_base_rate() {
        let result = calculate_company_tax(100_000.0, true).unwrap();
        assert_eq!(result.gross_tax, 25_000.0);
        assert_eq!(result.effective_rate, 0.25);
    }

    #[test]
    fn test_calculate_company_tax_full_rate() {
        let result = calculate_company_tax(100_000.0, false).unwrap();
        assert_eq!(result.gross_tax, 30_000.0);
        assert_eq!(result.effective_rate, 0.30);
    }

    #[test]
    fn test_validate_deduction_valid() {
        let deduction = Deduction {
            category: DeductionCategory::WorkRelated,
            amount: 200.0,
            description: "Work supplies".to_string(),
            substantiated: false, // OK for < $300
            date_incurred: None,
            work_percentage: None,
        };

        assert!(validate_deduction(&deduction).is_ok());
    }

    #[test]
    fn test_validate_deduction_needs_substantiation() {
        let deduction = Deduction {
            category: DeductionCategory::WorkRelated,
            amount: 500.0,
            description: "Work supplies".to_string(),
            substantiated: false, // Not OK for > $300
            date_incurred: None,
            work_percentage: None,
        };

        let result = validate_deduction(&deduction);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TaxError::InsufficientSubstantiation { .. }
        ));
    }

    #[test]
    fn test_medicare_levy_rates() {
        assert_eq!(MedicareLevyExemption::None.levy_rate(), 0.02);
        assert_eq!(MedicareLevyExemption::Full.levy_rate(), 0.0);
        assert_eq!(MedicareLevyExemption::Half.levy_rate(), 0.01);
    }
}
