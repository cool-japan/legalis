//! Personal Income Tax (НДФЛ - Налог на доходы физических лиц) implementation.
//!
//! Chapter 23 of Tax Code Part 2
//!
//! Standard rate: 13% (for income up to 5 million RUB per year)
//! Progressive rate: 15% (for income exceeding 5 million RUB per year)

use serde::{Deserialize, Serialize};

use super::TaxCodeError;

/// Personal income tax calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeTaxCalculation {
    /// Gross income
    pub gross_income: crate::common::Currency,
    /// Tax deductions applied
    pub deductions: crate::common::Currency,
    /// Taxable income (gross - deductions)
    pub taxable_income: crate::common::Currency,
    /// Tax rate applied
    pub tax_rate: f64,
    /// Tax amount
    pub tax_amount: crate::common::Currency,
    /// Net income (gross - tax)
    pub net_income: crate::common::Currency,
}

impl IncomeTaxCalculation {
    /// Creates a new income tax calculation
    pub fn new(gross_income: crate::common::Currency, deductions: crate::common::Currency) -> Self {
        let taxable_income = gross_income.subtract(&deductions);

        // Determine tax rate based on annual income
        // Standard: 13% for income up to 5,000,000 RUB
        // Progressive: 15% for income exceeding 5,000,000 RUB
        let threshold = crate::common::Currency::from_rubles(5_000_000);

        let (tax_rate, tax_amount) = if taxable_income.kopecks > threshold.kopecks {
            // Progressive taxation
            let base_tax = threshold.multiply_percentage(13.0);
            let excess = taxable_income.subtract(&threshold);
            let excess_tax = excess.multiply_percentage(15.0);
            (15.0, base_tax.add(&excess_tax))
        } else {
            // Standard rate
            (13.0, taxable_income.multiply_percentage(13.0))
        };

        let net_income = gross_income.subtract(&tax_amount);

        Self {
            gross_income,
            deductions,
            taxable_income,
            tax_rate,
            tax_amount,
            net_income,
        }
    }

    /// Validates the income tax calculation
    pub fn validate(&self) -> Result<(), TaxCodeError> {
        if !self.gross_income.is_positive() {
            return Err(TaxCodeError::InvalidCalculation(
                "Gross income must be positive".to_string(),
            ));
        }

        if self.deductions.kopecks > self.gross_income.kopecks {
            return Err(TaxCodeError::InvalidCalculation(
                "Deductions cannot exceed gross income".to_string(),
            ));
        }

        Ok(())
    }
}

/// Calculates personal income tax
pub fn calculate_income_tax(
    gross_income: crate::common::Currency,
    deductions: crate::common::Currency,
) -> IncomeTaxCalculation {
    IncomeTaxCalculation::new(gross_income, deductions)
}

/// Article 218: Standard tax deductions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardDeduction {
    /// Monthly deduction amount
    pub monthly_amount: crate::common::Currency,
    /// Description
    pub description: String,
}

impl StandardDeduction {
    /// Creates standard deduction for taxpayer (500 RUB/month)
    pub fn for_taxpayer() -> Self {
        Self {
            monthly_amount: crate::common::Currency::from_rubles(500),
            description: "Стандартный вычет на налогоплательщика".to_string(),
        }
    }

    /// Creates standard deduction for child (1,400 RUB/month for first two children)
    pub fn for_child(child_number: u32) -> Self {
        let amount = match child_number {
            1 | 2 => 1400,
            _ => 3000,
        };

        Self {
            monthly_amount: crate::common::Currency::from_rubles(amount),
            description: format!("Стандартный вычет на {} ребенка", child_number),
        }
    }

    /// Gets annual deduction amount
    pub fn annual_amount(&self) -> crate::common::Currency {
        crate::common::Currency {
            kopecks: self.monthly_amount.kopecks * 12,
        }
    }
}

/// Article 219: Social tax deductions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDeduction {
    /// Type of social deduction
    pub deduction_type: SocialDeductionType,
    /// Amount spent
    pub amount_spent: crate::common::Currency,
    /// Maximum deductible amount
    pub max_deductible: crate::common::Currency,
}

/// Types of social deductions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialDeductionType {
    /// Education expenses
    Education,
    /// Medical treatment expenses
    MedicalTreatment,
    /// Voluntary pension contributions
    PensionContributions,
    /// Charitable donations
    Charity,
}

impl SocialDeduction {
    /// Creates education deduction (max 120,000 RUB)
    pub fn education(amount_spent: crate::common::Currency) -> Self {
        Self {
            deduction_type: SocialDeductionType::Education,
            amount_spent,
            max_deductible: crate::common::Currency::from_rubles(120_000),
        }
    }

    /// Creates medical treatment deduction (max 120,000 RUB, except expensive treatment)
    pub fn medical_treatment(amount_spent: crate::common::Currency, is_expensive: bool) -> Self {
        let max_deductible = if is_expensive {
            amount_spent // No limit for expensive treatment
        } else {
            crate::common::Currency::from_rubles(120_000)
        };

        Self {
            deduction_type: SocialDeductionType::MedicalTreatment,
            amount_spent,
            max_deductible,
        }
    }

    /// Gets the deductible amount
    pub fn deductible_amount(&self) -> crate::common::Currency {
        if self.amount_spent.kopecks < self.max_deductible.kopecks {
            self.amount_spent
        } else {
            self.max_deductible
        }
    }
}

/// Article 220: Property tax deductions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDeduction {
    /// Type of property deduction
    pub deduction_type: PropertyDeductionType,
    /// Purchase or sale price
    pub amount: crate::common::Currency,
    /// Maximum deductible amount
    pub max_deductible: crate::common::Currency,
}

/// Types of property deductions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyDeductionType {
    /// Purchase of residential property
    PropertyPurchase,
    /// Mortgage interest
    MortgageInterest,
    /// Sale of property
    PropertySale,
}

impl PropertyDeduction {
    /// Creates property purchase deduction (max 2,000,000 RUB)
    pub fn property_purchase(purchase_price: crate::common::Currency) -> Self {
        Self {
            deduction_type: PropertyDeductionType::PropertyPurchase,
            amount: purchase_price,
            max_deductible: crate::common::Currency::from_rubles(2_000_000),
        }
    }

    /// Creates mortgage interest deduction (max 3,000,000 RUB)
    pub fn mortgage_interest(interest_paid: crate::common::Currency) -> Self {
        Self {
            deduction_type: PropertyDeductionType::MortgageInterest,
            amount: interest_paid,
            max_deductible: crate::common::Currency::from_rubles(3_000_000),
        }
    }

    /// Gets the deductible amount
    pub fn deductible_amount(&self) -> crate::common::Currency {
        if self.amount.kopecks < self.max_deductible.kopecks {
            self.amount
        } else {
            self.max_deductible
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_income_tax_standard_rate() {
        let gross = crate::common::Currency::from_rubles(1_000_000);
        let deductions = crate::common::Currency::from_rubles(0);

        let calc = calculate_income_tax(gross, deductions);

        assert_eq!(calc.gross_income.rubles(), 1_000_000);
        assert_eq!(calc.tax_rate, 13.0);
        assert_eq!(calc.tax_amount.rubles(), 130_000); // 13% of 1,000,000
        assert!(calc.validate().is_ok());
    }

    #[test]
    fn test_income_tax_progressive_rate() {
        let gross = crate::common::Currency::from_rubles(6_000_000);
        let deductions = crate::common::Currency::from_rubles(0);

        let calc = calculate_income_tax(gross, deductions);

        assert_eq!(calc.gross_income.rubles(), 6_000_000);
        assert_eq!(calc.tax_rate, 15.0);
        // 5M at 13% = 650,000 + 1M at 15% = 150,000 = 800,000
        assert_eq!(calc.tax_amount.rubles(), 800_000);
    }

    #[test]
    fn test_standard_deductions() {
        let taxpayer = StandardDeduction::for_taxpayer();
        assert_eq!(taxpayer.monthly_amount.rubles(), 500);
        assert_eq!(taxpayer.annual_amount().rubles(), 6_000);

        let child1 = StandardDeduction::for_child(1);
        assert_eq!(child1.monthly_amount.rubles(), 1_400);

        let child3 = StandardDeduction::for_child(3);
        assert_eq!(child3.monthly_amount.rubles(), 3_000);
    }

    #[test]
    fn test_social_deductions() {
        let education = SocialDeduction::education(crate::common::Currency::from_rubles(100_000));
        assert_eq!(education.deductible_amount().rubles(), 100_000);

        let expensive_education =
            SocialDeduction::education(crate::common::Currency::from_rubles(200_000));
        assert_eq!(expensive_education.deductible_amount().rubles(), 120_000); // Capped

        let medical =
            SocialDeduction::medical_treatment(crate::common::Currency::from_rubles(50_000), false);
        assert_eq!(medical.deductible_amount().rubles(), 50_000);

        let expensive_medical =
            SocialDeduction::medical_treatment(crate::common::Currency::from_rubles(500_000), true);
        assert_eq!(expensive_medical.deductible_amount().rubles(), 500_000); // No cap
    }

    #[test]
    fn test_property_deductions() {
        let purchase =
            PropertyDeduction::property_purchase(crate::common::Currency::from_rubles(3_000_000));
        assert_eq!(purchase.deductible_amount().rubles(), 2_000_000); // Capped

        let mortgage =
            PropertyDeduction::mortgage_interest(crate::common::Currency::from_rubles(500_000));
        assert_eq!(mortgage.deductible_amount().rubles(), 500_000);
    }
}
