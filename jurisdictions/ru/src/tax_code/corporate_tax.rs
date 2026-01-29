//! Corporate Income Tax (Налог на прибыль организаций) implementation.
//!
//! Chapter 25 of Tax Code Part 2
//!
//! Standard rate: 20% (3% to federal budget, 17% to regional budget)

use serde::{Deserialize, Serialize};

use super::TaxCodeError;

/// Corporate tax calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateTaxCalculation {
    /// Total revenue
    pub revenue: crate::common::Currency,
    /// Deductible expenses
    pub expenses: crate::common::Currency,
    /// Taxable profit (revenue - expenses)
    pub taxable_profit: crate::common::Currency,
    /// Tax rate (20%)
    pub tax_rate: f64,
    /// Tax amount to federal budget (3%)
    pub federal_tax: crate::common::Currency,
    /// Tax amount to regional budget (17%)
    pub regional_tax: crate::common::Currency,
    /// Total tax amount
    pub total_tax: crate::common::Currency,
    /// Net profit after tax
    pub net_profit: crate::common::Currency,
}

impl CorporateTaxCalculation {
    /// Creates a new corporate tax calculation
    pub fn new(
        revenue: crate::common::Currency,
        expenses: crate::common::Currency,
    ) -> Result<Self, TaxCodeError> {
        if expenses.kopecks > revenue.kopecks {
            return Err(TaxCodeError::InvalidCalculation(
                "Expenses cannot exceed revenue".to_string(),
            ));
        }

        let taxable_profit = revenue.subtract(&expenses);

        // Corporate tax rate is 20% total:
        // - 3% to federal budget
        // - 17% to regional budget (can be reduced by regions to min 12.5%)
        let federal_tax = taxable_profit.multiply_percentage(3.0);
        let regional_tax = taxable_profit.multiply_percentage(17.0);
        let total_tax = federal_tax.add(&regional_tax);

        let net_profit = taxable_profit.subtract(&total_tax);

        Ok(Self {
            revenue,
            expenses,
            taxable_profit,
            tax_rate: 20.0,
            federal_tax,
            regional_tax,
            total_tax,
            net_profit,
        })
    }

    /// Validates the corporate tax calculation
    pub fn validate(&self) -> Result<(), TaxCodeError> {
        if !self.revenue.is_positive() && !self.revenue.is_zero() {
            return Err(TaxCodeError::InvalidCalculation(
                "Revenue must be non-negative".to_string(),
            ));
        }

        // Verify calculation
        let expected_profit = self.revenue.subtract(&self.expenses);
        if self.taxable_profit.kopecks != expected_profit.kopecks {
            return Err(TaxCodeError::InvalidCalculation(
                "Taxable profit calculation mismatch".to_string(),
            ));
        }

        let expected_total = self.federal_tax.add(&self.regional_tax);
        if (self.total_tax.kopecks - expected_total.kopecks).abs() > 1 {
            return Err(TaxCodeError::InvalidCalculation(
                "Total tax calculation mismatch".to_string(),
            ));
        }

        Ok(())
    }
}

/// Calculates corporate income tax
pub fn calculate_corporate_tax(
    revenue: crate::common::Currency,
    expenses: crate::common::Currency,
) -> Result<CorporateTaxCalculation, TaxCodeError> {
    CorporateTaxCalculation::new(revenue, expenses)
}

/// Article 252: Deductible expenses criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeductibleExpense {
    /// Expense description
    pub description: String,
    /// Expense amount
    pub amount: crate::common::Currency,
    /// Is economically justified
    pub economically_justified: bool,
    /// Is documented
    pub documented: bool,
    /// Is related to income generation
    pub income_related: bool,
}

impl DeductibleExpense {
    /// Creates a new deductible expense
    pub fn new(description: impl Into<String>, amount: crate::common::Currency) -> Self {
        Self {
            description: description.into(),
            amount,
            economically_justified: false,
            documented: false,
            income_related: false,
        }
    }

    /// Sets whether expense is economically justified
    pub fn economically_justified(mut self, justified: bool) -> Self {
        self.economically_justified = justified;
        self
    }

    /// Sets whether expense is documented
    pub fn documented(mut self, documented: bool) -> Self {
        self.documented = documented;
        self
    }

    /// Sets whether expense is related to income generation
    pub fn income_related(mut self, related: bool) -> Self {
        self.income_related = related;
        self
    }

    /// Validates if the expense is deductible
    pub fn is_deductible(&self) -> Result<bool, TaxCodeError> {
        // Article 252: Expenses must be:
        // 1. Economically justified
        // 2. Documented
        // 3. Related to income generation

        if !self.economically_justified {
            return Ok(false);
        }

        if !self.documented {
            return Ok(false);
        }

        if !self.income_related {
            return Ok(false);
        }

        Ok(true)
    }
}

/// Article 253: Labor expenses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborExpense {
    /// Salaries and wages
    pub salaries: crate::common::Currency,
    /// Social contributions
    pub social_contributions: crate::common::Currency,
    /// Other labor-related expenses
    pub other_labor_expenses: crate::common::Currency,
}

impl LaborExpense {
    /// Creates new labor expense
    pub fn new(salaries: crate::common::Currency) -> Self {
        // Social contributions are typically around 30% of salaries
        let social_contributions = salaries.multiply_percentage(30.0);

        Self {
            salaries,
            social_contributions,
            other_labor_expenses: crate::common::Currency::from_rubles(0),
        }
    }

    /// Adds other labor expenses
    pub fn with_other_expenses(mut self, amount: crate::common::Currency) -> Self {
        self.other_labor_expenses = amount;
        self
    }

    /// Gets total labor expenses
    pub fn total(&self) -> crate::common::Currency {
        self.salaries
            .add(&self.social_contributions)
            .add(&self.other_labor_expenses)
    }
}

/// Article 256: Depreciable property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepreciableProperty {
    /// Initial cost
    pub initial_cost: crate::common::Currency,
    /// Useful life in months
    pub useful_life_months: u32,
    /// Depreciation method
    pub depreciation_method: DepreciationMethod,
}

/// Depreciation methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepreciationMethod {
    /// Linear (линейный)
    Linear,
    /// Non-linear (нелинейный)
    NonLinear,
}

impl DepreciableProperty {
    /// Calculates monthly depreciation
    pub fn monthly_depreciation(&self) -> crate::common::Currency {
        match self.depreciation_method {
            DepreciationMethod::Linear => {
                // Linear: cost / useful life
                crate::common::Currency {
                    kopecks: self.initial_cost.kopecks / self.useful_life_months as i64,
                }
            }
            DepreciationMethod::NonLinear => {
                // Simplified non-linear calculation
                // Actual calculation is more complex
                let rate = 2.0 / self.useful_life_months as f64;
                self.initial_cost.multiply_percentage(rate * 100.0)
            }
        }
    }

    /// Calculates annual depreciation
    pub fn annual_depreciation(&self) -> crate::common::Currency {
        let monthly = self.monthly_depreciation();
        crate::common::Currency {
            kopecks: monthly.kopecks * 12,
        }
    }
}

/// Loss carryforward (Article 283)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossCarryforward {
    /// Loss amount
    pub loss_amount: crate::common::Currency,
    /// Year of loss
    pub loss_year: u32,
    /// Remaining amount to carry forward
    pub remaining_amount: crate::common::Currency,
}

impl LossCarryforward {
    /// Creates new loss carryforward
    pub fn new(loss_amount: crate::common::Currency, loss_year: u32) -> Self {
        Self {
            loss_amount,
            loss_year,
            remaining_amount: loss_amount,
        }
    }

    /// Applies loss to current year profit
    pub fn apply_to_profit(
        &mut self,
        current_profit: &crate::common::Currency,
    ) -> crate::common::Currency {
        // Can offset up to 50% of current profit (since 2017)
        let max_offset = current_profit.multiply_percentage(50.0);

        let offset = if self.remaining_amount.kopecks < max_offset.kopecks {
            self.remaining_amount
        } else {
            max_offset
        };

        self.remaining_amount = self.remaining_amount.subtract(&offset);
        offset
    }

    /// Checks if loss can still be carried forward (max 10 years)
    pub fn is_valid(&self, current_year: u32) -> bool {
        (current_year - self.loss_year) <= 10 && self.remaining_amount.is_positive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corporate_tax_calculation() {
        let revenue = crate::common::Currency::from_rubles(10_000_000);
        let expenses = crate::common::Currency::from_rubles(7_000_000);

        let calc = calculate_corporate_tax(revenue, expenses).expect("Should succeed");

        assert_eq!(calc.revenue.rubles(), 10_000_000);
        assert_eq!(calc.expenses.rubles(), 7_000_000);
        assert_eq!(calc.taxable_profit.rubles(), 3_000_000);
        assert_eq!(calc.federal_tax.rubles(), 90_000); // 3% of 3M
        assert_eq!(calc.regional_tax.rubles(), 510_000); // 17% of 3M
        assert_eq!(calc.total_tax.rubles(), 600_000); // 20% of 3M
        assert!(calc.validate().is_ok());
    }

    #[test]
    fn test_deductible_expense() {
        let expense =
            DeductibleExpense::new("Office rent", crate::common::Currency::from_rubles(100_000))
                .economically_justified(true)
                .documented(true)
                .income_related(true);

        assert!(expense.is_deductible().expect("Should succeed"));

        let non_deductible = DeductibleExpense::new(
            "Personal expenses",
            crate::common::Currency::from_rubles(50_000),
        )
        .economically_justified(false)
        .documented(true)
        .income_related(false);

        assert!(!non_deductible.is_deductible().expect("Should succeed"));
    }

    #[test]
    fn test_labor_expense() {
        let salaries = crate::common::Currency::from_rubles(1_000_000);
        let labor = LaborExpense::new(salaries);

        assert_eq!(labor.salaries.rubles(), 1_000_000);
        assert_eq!(labor.social_contributions.rubles(), 300_000); // 30%
        assert_eq!(labor.total().rubles(), 1_300_000);
    }

    #[test]
    fn test_depreciation() {
        let property = DepreciableProperty {
            initial_cost: crate::common::Currency::from_rubles(1_200_000),
            useful_life_months: 60,
            depreciation_method: DepreciationMethod::Linear,
        };

        let monthly = property.monthly_depreciation();
        assert_eq!(monthly.rubles(), 20_000); // 1,200,000 / 60

        let annual = property.annual_depreciation();
        assert_eq!(annual.rubles(), 240_000); // 20,000 * 12
    }

    #[test]
    fn test_loss_carryforward() {
        let loss = crate::common::Currency::from_rubles(1_000_000);
        let mut carryforward = LossCarryforward::new(loss, 2020);

        let profit = crate::common::Currency::from_rubles(800_000);
        let offset = carryforward.apply_to_profit(&profit);

        assert_eq!(offset.rubles(), 400_000); // 50% of profit
        assert_eq!(carryforward.remaining_amount.rubles(), 600_000);

        assert!(carryforward.is_valid(2025)); // Within 10 years
        assert!(!carryforward.is_valid(2031)); // Beyond 10 years
    }
}
