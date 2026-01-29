//! Personal Income Tax (PIT) - Thuế Thu nhập Cá nhân (TNCN)
//!
//! Law on Personal Income Tax No. 04/2007/QH12 (amended by Laws 26/2012, 71/2014).
//! Circular 111/2013/TT-BTC for implementation.
//!
//! ## PIT Rates (Progressive)
//!
//! - 0-5 million: 5%
//! - 5-10 million: 10%
//! - 10-18 million: 15%
//! - 18-32 million: 20%
//! - 32-52 million: 25%
//! - 52-80 million: 30%
//! - Over 80 million: 35%

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// PIT tax brackets (Bậc thuế TNCN) - Article 22, Circular 111/2013
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PitBracket {
    /// Lower bound (VND per month)
    pub lower: i64,
    /// Upper bound (VND per month), None for top bracket
    pub upper: Option<i64>,
    /// Tax rate (percentage)
    pub rate: u8,
}

impl PitBracket {
    /// Get all PIT brackets (2024)
    pub fn all_brackets() -> Vec<PitBracket> {
        vec![
            PitBracket {
                lower: 0,
                upper: Some(5_000_000),
                rate: 5,
            },
            PitBracket {
                lower: 5_000_000,
                upper: Some(10_000_000),
                rate: 10,
            },
            PitBracket {
                lower: 10_000_000,
                upper: Some(18_000_000),
                rate: 15,
            },
            PitBracket {
                lower: 18_000_000,
                upper: Some(32_000_000),
                rate: 20,
            },
            PitBracket {
                lower: 32_000_000,
                upper: Some(52_000_000),
                rate: 25,
            },
            PitBracket {
                lower: 52_000_000,
                upper: Some(80_000_000),
                rate: 30,
            },
            PitBracket {
                lower: 80_000_000,
                upper: None,
                rate: 35,
            },
        ]
    }

    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self.upper {
            Some(upper) => format!("Từ {} đến {} VND: {}%", self.lower, upper, self.rate),
            None => format!("Trên {} VND: {}%", self.lower, self.rate),
        }
    }

    /// Check if amount falls in this bracket
    pub fn contains(&self, amount: i64) -> bool {
        amount >= self.lower && self.upper.is_none_or(|upper| amount < upper)
    }
}

/// Personal deductions (Giảm trừ gia cảnh) - Article 9, Circular 111/2013
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonalDeduction {
    /// Deduction for taxpayer (Bản thân)
    pub self_deduction: i64,
    /// Deduction per dependent (Người phụ thuộc)
    pub dependent_deduction: i64,
}

impl PersonalDeduction {
    /// Personal deduction amounts for 2024 (updated July 2020)
    pub fn current_2024() -> Self {
        Self {
            self_deduction: 11_000_000,     // 11 million VND per month
            dependent_deduction: 4_400_000, // 4.4 million VND per dependent per month
        }
    }

    /// Calculate total monthly deduction
    pub fn total_monthly(&self, num_dependents: u8) -> i64 {
        self.self_deduction + (self.dependent_deduction * i64::from(num_dependents))
    }
}

/// PIT income types (Loại thu nhập chịu thuế) - Article 3
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PitIncomeType {
    /// Employment income (Thu nhập từ tiền lương, tiền công)
    Employment,
    /// Business income (Thu nhập từ kinh doanh)
    Business,
    /// Capital investment income (Thu nhập từ đầu tư vốn)
    CapitalInvestment,
    /// Capital transfer income (Thu nhập từ chuyển nhượng vốn)
    CapitalTransfer,
    /// Real estate transfer income (Thu nhập từ chuyển nhượng bất động sản)
    RealEstateTransfer,
    /// Lottery and prizes (Thu nhập từ trúng thưởng)
    LotteryPrizes,
    /// Royalties (Thu nhập từ bản quyền)
    Royalties,
    /// Inheritance and gifts (Thu nhập từ thừa kế, quà tặng)
    InheritanceGifts,
    /// Other income (Thu nhập khác)
    Other(String),
}

impl PitIncomeType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::Employment => "Thu nhập từ tiền lương, tiền công".to_string(),
            Self::Business => "Thu nhập từ kinh doanh".to_string(),
            Self::CapitalInvestment => "Thu nhập từ đầu tư vốn".to_string(),
            Self::CapitalTransfer => "Thu nhập từ chuyển nhượng vốn".to_string(),
            Self::RealEstateTransfer => "Thu nhập từ chuyển nhượng bất động sản".to_string(),
            Self::LotteryPrizes => "Thu nhập từ trúng thưởng".to_string(),
            Self::Royalties => "Thu nhập từ bản quyền".to_string(),
            Self::InheritanceGifts => "Thu nhập từ thừa kế, quà tặng".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Check if progressive tax applies (vs flat rate)
    pub fn uses_progressive_tax(&self) -> bool {
        matches!(self, Self::Employment | Self::Business)
    }

    /// Get flat tax rate if applicable
    pub fn flat_rate(&self) -> Option<u8> {
        match self {
            Self::CapitalInvestment => Some(5),  // 5% on dividends
            Self::CapitalTransfer => Some(20),   // 20% on capital gains
            Self::RealEstateTransfer => Some(2), // 2% on transfer value
            Self::LotteryPrizes => Some(10),     // 10% on prizes
            Self::Royalties => Some(5),          // 5% on royalties
            _ => None,
        }
    }
}

/// PIT calculation for employment income
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitCalculation {
    /// Monthly gross income (Thu nhập trước thuế)
    pub gross_income: i64,
    /// Compulsory insurance deductions (BHXH, BHYT, BHTN)
    pub insurance_deductions: i64,
    /// Number of dependents (Số người phụ thuộc)
    pub num_dependents: u8,
    /// Other deductible expenses
    pub other_deductions: i64,
}

impl PitCalculation {
    /// Calculate taxable income (Thu nhập tính thuế)
    pub fn calculate_taxable_income(&self) -> i64 {
        let deduction = PersonalDeduction::current_2024();
        let personal_deduction = deduction.total_monthly(self.num_dependents);

        (self.gross_income - self.insurance_deductions - personal_deduction - self.other_deductions)
            .max(0)
    }

    /// Calculate PIT payable using progressive rates (Thuế TNCN phải nộp)
    pub fn calculate_pit_payable(&self) -> i64 {
        let taxable_income = self.calculate_taxable_income();
        calculate_progressive_pit(taxable_income)
    }

    /// Calculate net income after tax (Thu nhập sau thuế)
    pub fn calculate_net_income(&self) -> i64 {
        self.gross_income - self.insurance_deductions - self.calculate_pit_payable()
    }
}

/// Calculate PIT using progressive tax brackets
pub fn calculate_progressive_pit(taxable_income: i64) -> i64 {
    if taxable_income <= 0 {
        return 0;
    }

    let brackets = PitBracket::all_brackets();
    let mut total_tax = 0i64;
    let mut remaining_income = taxable_income;

    for bracket in &brackets {
        if remaining_income <= 0 {
            break;
        }

        let bracket_size = match bracket.upper {
            Some(upper) => upper - bracket.lower,
            None => remaining_income, // Top bracket - tax all remaining
        };

        let taxable_in_bracket = remaining_income.min(bracket_size);
        let tax_in_bracket = (taxable_in_bracket as f64 * bracket.rate as f64 / 100.0) as i64;

        total_tax += tax_in_bracket;
        remaining_income -= taxable_in_bracket;
    }

    total_tax
}

/// Calculate flat rate PIT for specific income types
pub fn calculate_flat_rate_pit(income: i64, income_type: &PitIncomeType) -> Result<i64, PitError> {
    match income_type.flat_rate() {
        Some(rate) => Ok((income as f64 * rate as f64 / 100.0) as i64),
        None => Err(PitError::InvalidIncomeType {
            reason: format!("{} không áp dụng thuế suất cố định", income_type.name_vi()),
        }),
    }
}

/// Result type for PIT operations
pub type PitResult<T> = Result<T, PitError>;

/// Errors related to PIT
#[derive(Debug, Error)]
pub enum PitError {
    /// Invalid income amount
    #[error("Số tiền thu nhập không hợp lệ: {amount} VND")]
    InvalidIncome { amount: i64 },

    /// Invalid income type
    #[error("Loại thu nhập không hợp lệ: {reason}")]
    InvalidIncomeType { reason: String },

    /// Invalid deduction
    #[error("Giảm trừ không hợp lệ: {reason}")]
    InvalidDeduction { reason: String },

    /// Invalid declaration
    #[error("Khai báo thuế TNCN không hợp lệ: {reason}")]
    InvalidDeclaration { reason: String },
}

/// Validate PIT calculation
pub fn validate_pit_calculation(calc: &PitCalculation) -> PitResult<()> {
    if calc.gross_income < 0 {
        return Err(PitError::InvalidIncome {
            amount: calc.gross_income,
        });
    }

    if calc.insurance_deductions < 0 || calc.insurance_deductions > calc.gross_income {
        return Err(PitError::InvalidDeduction {
            reason: "Giảm trừ bảo hiểm không hợp lệ".to_string(),
        });
    }

    Ok(())
}

/// Get PIT checklist
pub fn get_pit_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Đăng ký mã số thuế cá nhân",
            "PIT registration (tax code)",
            "Điều 6",
        ),
        (
            "Xác định thu nhập chịu thuế",
            "Determine taxable income",
            "Điều 3",
        ),
        ("Đăng ký người phụ thuộc", "Register dependents", "Điều 9"),
        ("Giảm trừ gia cảnh", "Personal deductions", "Điều 9"),
        (
            "Tính thuế TNCN theo bậc",
            "Calculate progressive PIT",
            "Điều 22",
        ),
        (
            "Quyết toán thuế cuối năm",
            "Annual tax finalization",
            "Điều 15",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pit_brackets() {
        let brackets = PitBracket::all_brackets();
        assert_eq!(brackets.len(), 7);
        assert_eq!(brackets[0].rate, 5);
        assert_eq!(brackets[6].rate, 35);

        assert!(brackets[0].contains(3_000_000));
        assert!(brackets[6].contains(100_000_000));
    }

    #[test]
    fn test_personal_deduction() {
        let deduction = PersonalDeduction::current_2024();
        assert_eq!(deduction.self_deduction, 11_000_000);
        assert_eq!(deduction.dependent_deduction, 4_400_000);

        // Self + 2 dependents
        assert_eq!(deduction.total_monthly(2), 19_800_000);
    }

    #[test]
    fn test_progressive_pit_calculation() {
        // 5 million - 5% bracket
        let tax1 = calculate_progressive_pit(5_000_000);
        assert_eq!(tax1, 250_000); // 5M * 5%

        // 10 million - spans 2 brackets
        let tax2 = calculate_progressive_pit(10_000_000);
        assert_eq!(tax2, 750_000); // 5M*5% + 5M*10%

        // 20 million - spans 3 brackets
        let tax3 = calculate_progressive_pit(20_000_000);
        assert_eq!(tax3, 2_350_000); // 5M*5% + 5M*10% + 8M*15% + 2M*20%
    }

    #[test]
    fn test_pit_calculation() {
        let calc = PitCalculation {
            gross_income: 30_000_000,
            insurance_deductions: 3_000_000, // ~10% for social insurance
            num_dependents: 1,
            other_deductions: 0,
        };

        // Taxable = 30M - 3M - 11M - 4.4M = 11.6M
        let taxable = calc.calculate_taxable_income();
        assert_eq!(taxable, 11_600_000);

        // Tax should be in 15% bracket
        let tax = calc.calculate_pit_payable();
        assert!(tax > 0);
        assert!(tax < 2_000_000); // Rough sanity check

        // Net income
        let net = calc.calculate_net_income();
        assert_eq!(net, calc.gross_income - calc.insurance_deductions - tax);
    }

    #[test]
    fn test_flat_rate_pit() {
        // Dividend income - 5%
        let dividend_tax = calculate_flat_rate_pit(10_000_000, &PitIncomeType::CapitalInvestment);
        assert_eq!(dividend_tax.ok(), Some(500_000));

        // Capital transfer - 20%
        let capital_tax = calculate_flat_rate_pit(10_000_000, &PitIncomeType::CapitalTransfer);
        assert_eq!(capital_tax.ok(), Some(2_000_000));

        // Employment should fail (uses progressive)
        let employment_tax = calculate_flat_rate_pit(10_000_000, &PitIncomeType::Employment);
        assert!(employment_tax.is_err());
    }

    #[test]
    fn test_income_types() {
        assert!(PitIncomeType::Employment.uses_progressive_tax());
        assert!(!PitIncomeType::CapitalInvestment.uses_progressive_tax());

        assert_eq!(PitIncomeType::CapitalInvestment.flat_rate(), Some(5));
        assert_eq!(PitIncomeType::Employment.flat_rate(), None);
    }

    #[test]
    fn test_validation() {
        let valid = PitCalculation {
            gross_income: 20_000_000,
            insurance_deductions: 2_000_000,
            num_dependents: 2,
            other_deductions: 0,
        };
        assert!(validate_pit_calculation(&valid).is_ok());

        let invalid = PitCalculation {
            gross_income: -20_000_000,
            insurance_deductions: 2_000_000,
            num_dependents: 2,
            other_deductions: 0,
        };
        assert!(validate_pit_calculation(&invalid).is_err());
    }

    #[test]
    fn test_pit_checklist() {
        let checklist = get_pit_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 6);
    }

    #[test]
    fn test_zero_taxable_income() {
        // High deductions, no tax
        let calc = PitCalculation {
            gross_income: 15_000_000,
            insurance_deductions: 1_500_000,
            num_dependents: 3, // 11M + 3*4.4M = 24.2M deduction
            other_deductions: 0,
        };

        let taxable = calc.calculate_taxable_income();
        assert_eq!(taxable, 0); // Deductions exceed income

        let tax = calc.calculate_pit_payable();
        assert_eq!(tax, 0);
    }
}
