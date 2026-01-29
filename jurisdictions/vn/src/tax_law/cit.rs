//! Corporate Income Tax (CIT) - Thuế Thu nhập Doanh nghiệp (TNDN)
//!
//! Law on Corporate Income Tax No. 14/2008/QH12 (amended by Law 32/2013).
//! Decree 218/2013/ND-CP and Circular 78/2014/TT-BTC for implementation.
//!
//! ## CIT Rates
//!
//! - **20%**: Standard rate (from 2016)
//! - **17%**: Small and medium enterprises (SMEs) with revenue < 50 billion VND
//! - **10%**: Education, healthcare, high-tech
//! - **Preferential rates**: Special economic zones, investment incentives

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// CIT rates in Vietnam (Thuế suất thuế TNDN) - Article 11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitRate {
    /// 20% - Standard rate (Thuế suất cơ bản)
    Standard,
    /// 17% - Small and medium enterprises (Doanh nghiệp nhỏ và vừa)
    Sme,
    /// 10% - Preferential sectors (Lĩnh vực ưu đãi)
    Preferential,
    /// Custom rate for special cases
    Custom(u8),
}

impl CitRate {
    /// Get rate as decimal (0.20, 0.17, 0.10)
    pub fn as_decimal(&self) -> f64 {
        match self {
            Self::Standard => 0.20,
            Self::Sme => 0.17,
            Self::Preferential => 0.10,
            Self::Custom(rate) => f64::from(*rate) / 100.0,
        }
    }

    /// Get rate as percentage (20, 17, 10)
    pub fn as_percentage(&self) -> u8 {
        match self {
            Self::Standard => 20,
            Self::Sme => 17,
            Self::Preferential => 10,
            Self::Custom(rate) => *rate,
        }
    }

    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::Standard => "Thuế suất 20% - Thuế suất cơ bản".to_string(),
            Self::Sme => "Thuế suất 17% - Doanh nghiệp nhỏ và vừa".to_string(),
            Self::Preferential => "Thuế suất 10% - Lĩnh vực ưu đãi".to_string(),
            Self::Custom(rate) => format!("Thuế suất {}% - Thuế suất đặc biệt", rate),
        }
    }
}

/// CIT incentive types (Ưu đãi thuế TNDN) - Article 16
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitIncentive {
    /// Tax exemption period (Miễn thuế)
    Exemption {
        /// Number of years
        years: u8,
    },
    /// Tax reduction period (Giảm thuế)
    Reduction {
        /// Number of years
        years: u8,
        /// Reduction percentage (50%, 75%)
        percent: u8,
    },
    /// Preferential rate (Thuế suất ưu đãi)
    PreferentialRate {
        /// Rate percentage
        rate: u8,
        /// Duration in years
        years: u8,
    },
}

impl CitIncentive {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::Exemption { years } => format!("Miễn thuế {} năm", years),
            Self::Reduction { years, percent } => {
                format!("Giảm {}% thuế trong {} năm", percent, years)
            }
            Self::PreferentialRate { rate, years } => {
                format!("Áp dụng thuế suất {}% trong {} năm", rate, years)
            }
        }
    }

    /// Get English description
    pub fn description_en(&self) -> String {
        match self {
            Self::Exemption { years } => format!("{}-year tax exemption", years),
            Self::Reduction { years, percent } => {
                format!("{}% tax reduction for {} years", percent, years)
            }
            Self::PreferentialRate { rate, years } => {
                format!("{}% preferential rate for {} years", rate, years)
            }
        }
    }
}

/// Eligible sectors for CIT incentives (Lĩnh vực được ưu đãi) - Article 15
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncentiveSector {
    /// High-tech (Công nghệ cao)
    HighTech,
    /// Scientific research (Nghiên cứu khoa học)
    ScientificResearch,
    /// Education (Giáo dục)
    Education,
    /// Healthcare (Y tế)
    Healthcare,
    /// Environmental protection (Bảo vệ môi trường)
    Environmental,
    /// Infrastructure (Cơ sở hạ tầng)
    Infrastructure,
    /// Renewable energy (Năng lượng tái tạo)
    RenewableEnergy,
    /// Software development (Phát triển phần mềm)
    Software,
    /// Other
    Other(String),
}

impl IncentiveSector {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::HighTech => "Công nghệ cao".to_string(),
            Self::ScientificResearch => "Nghiên cứu khoa học".to_string(),
            Self::Education => "Giáo dục, đào tạo".to_string(),
            Self::Healthcare => "Y tế, chăm sóc sức khỏe".to_string(),
            Self::Environmental => "Bảo vệ môi trường".to_string(),
            Self::Infrastructure => "Cơ sở hạ tầng".to_string(),
            Self::RenewableEnergy => "Năng lượng tái tạo".to_string(),
            Self::Software => "Phát triển phần mềm".to_string(),
            Self::Other(name) => name.clone(),
        }
    }

    /// Check if eligible for 10% preferential rate
    pub fn is_eligible_for_preferential_rate(&self) -> bool {
        matches!(
            self,
            Self::HighTech
                | Self::ScientificResearch
                | Self::Education
                | Self::Healthcare
                | Self::Software
        )
    }
}

/// CIT taxable income calculation (Thu nhập chịu thuế)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitTaxableIncome {
    /// Total revenue (Tổng doanh thu)
    pub revenue: i64,
    /// Deductible expenses (Chi phí được trừ)
    pub deductible_expenses: i64,
    /// Non-deductible expenses (Chi phí không được trừ)
    pub non_deductible_expenses: i64,
    /// Other income (Thu nhập khác)
    pub other_income: i64,
}

impl CitTaxableIncome {
    /// Calculate taxable income (Thu nhập chịu thuế)
    pub fn calculate_taxable_income(&self) -> i64 {
        (self.revenue + self.other_income - self.deductible_expenses).max(0)
    }

    /// Calculate CIT payable (Thuế TNDN phải nộp)
    pub fn calculate_cit_payable(&self, rate: CitRate) -> i64 {
        let taxable = self.calculate_taxable_income();
        (taxable as f64 * rate.as_decimal()) as i64
    }

    /// Check if qualifies as SME (revenue < 50 billion VND)
    pub fn is_sme(&self) -> bool {
        self.revenue < 50_000_000_000
    }
}

/// CIT declaration period (Kỳ khai thuế)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitDeclarationPeriod {
    /// Quarterly provisional (Tạm nộp theo quý)
    QuarterlyProvisional,
    /// Annual finalization (Quyết toán năm)
    AnnualFinalization,
}

impl CitDeclarationPeriod {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::QuarterlyProvisional => "Tạm nộp theo quý",
            Self::AnnualFinalization => "Quyết toán thuế năm",
        }
    }

    /// Get deadline (days after period end)
    pub fn deadline_days(&self) -> u8 {
        match self {
            Self::QuarterlyProvisional => 30, // 30 days after quarter end
            Self::AnnualFinalization => 90,   // 90 days after year end
        }
    }
}

/// Result type for CIT operations
pub type CitResult<T> = Result<T, CitError>;

/// Errors related to CIT
#[derive(Debug, Error)]
pub enum CitError {
    /// Invalid CIT rate
    #[error("Thuế suất thuế TNDN không hợp lệ: {rate}%")]
    InvalidRate { rate: u8 },

    /// Invalid taxable income
    #[error("Thu nhập chịu thuế không hợp lệ: {amount} VND")]
    InvalidTaxableIncome { amount: i64 },

    /// Ineligible for incentive
    #[error("Không đủ điều kiện ưu đãi thuế: {reason}")]
    IneligibleForIncentive { reason: String },

    /// Invalid declaration
    #[error("Khai báo thuế TNDN không hợp lệ: {reason}")]
    InvalidDeclaration { reason: String },
}

/// Validate CIT taxable income
pub fn validate_taxable_income(income: &CitTaxableIncome) -> CitResult<()> {
    if income.revenue < 0 {
        return Err(CitError::InvalidTaxableIncome {
            amount: income.revenue,
        });
    }

    if income.deductible_expenses < 0 {
        return Err(CitError::InvalidDeclaration {
            reason: "Chi phí được trừ không thể âm".to_string(),
        });
    }

    Ok(())
}

/// Determine applicable CIT rate based on business characteristics
pub fn determine_cit_rate(revenue: i64, sector: Option<&IncentiveSector>) -> CitRate {
    // Check if SME (revenue < 50 billion VND)
    if revenue < 50_000_000_000 {
        return CitRate::Sme;
    }

    // Check if eligible for preferential rate
    if let Some(sector) = sector
        && sector.is_eligible_for_preferential_rate()
    {
        return CitRate::Preferential;
    }

    // Default standard rate
    CitRate::Standard
}

/// Get CIT checklist
pub fn get_cit_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Đăng ký nộp thuế TNDN", "CIT registration", "Điều 4"),
        (
            "Xác định thu nhập chịu thuế",
            "Determine taxable income",
            "Điều 7",
        ),
        (
            "Xác định chi phí được trừ",
            "Determine deductible expenses",
            "Điều 8",
        ),
        (
            "Tạm nộp thuế hàng quý",
            "Quarterly provisional payment",
            "Điều 18",
        ),
        ("Quyết toán thuế năm", "Annual tax finalization", "Điều 19"),
        ("Ưu đãi đầu tư", "Investment incentives", "Điều 15-17"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cit_rates() {
        assert_eq!(CitRate::Standard.as_percentage(), 20);
        assert_eq!(CitRate::Sme.as_percentage(), 17);
        assert_eq!(CitRate::Preferential.as_percentage(), 10);

        assert_eq!(CitRate::Standard.as_decimal(), 0.20);
        assert_eq!(CitRate::Sme.as_decimal(), 0.17);
    }

    #[test]
    fn test_taxable_income_calculation() {
        let income = CitTaxableIncome {
            revenue: 1_000_000_000,
            deductible_expenses: 600_000_000,
            non_deductible_expenses: 50_000_000,
            other_income: 100_000_000,
        };

        assert_eq!(income.calculate_taxable_income(), 500_000_000);
        assert!(income.is_sme()); // Revenue 1B < 50B, so it IS SME

        let cit = income.calculate_cit_payable(CitRate::Standard);
        assert_eq!(cit, 100_000_000); // 20% of 500M
    }

    #[test]
    fn test_sme_qualification() {
        let sme_income = CitTaxableIncome {
            revenue: 30_000_000_000,
            deductible_expenses: 20_000_000_000,
            non_deductible_expenses: 0,
            other_income: 0,
        };

        assert!(sme_income.is_sme());

        let large_income = CitTaxableIncome {
            revenue: 100_000_000_000,
            deductible_expenses: 80_000_000_000,
            non_deductible_expenses: 0,
            other_income: 0,
        };

        assert!(!large_income.is_sme());
    }

    #[test]
    fn test_determine_cit_rate() {
        // SME
        let sme_rate = determine_cit_rate(30_000_000_000, None);
        assert_eq!(sme_rate, CitRate::Sme);

        // Large company - standard rate
        let standard_rate = determine_cit_rate(100_000_000_000, None);
        assert_eq!(standard_rate, CitRate::Standard);

        // Large company in preferential sector
        let preferential_rate =
            determine_cit_rate(100_000_000_000, Some(&IncentiveSector::HighTech));
        assert_eq!(preferential_rate, CitRate::Preferential);
    }

    #[test]
    fn test_incentive_sectors() {
        assert!(IncentiveSector::HighTech.is_eligible_for_preferential_rate());
        assert!(IncentiveSector::Software.is_eligible_for_preferential_rate());
        assert!(!IncentiveSector::Infrastructure.is_eligible_for_preferential_rate());
    }

    #[test]
    fn test_declaration_periods() {
        assert_eq!(
            CitDeclarationPeriod::QuarterlyProvisional.deadline_days(),
            30
        );
        assert_eq!(CitDeclarationPeriod::AnnualFinalization.deadline_days(), 90);
    }

    #[test]
    fn test_cit_incentives() {
        let exemption = CitIncentive::Exemption { years: 4 };
        assert!(exemption.description_vi().contains("Miễn thuế 4 năm"));

        let reduction = CitIncentive::Reduction {
            years: 9,
            percent: 50,
        };
        assert!(reduction.description_vi().contains("50%"));
    }

    #[test]
    fn test_validation() {
        let valid = CitTaxableIncome {
            revenue: 1_000_000_000,
            deductible_expenses: 600_000_000,
            non_deductible_expenses: 50_000_000,
            other_income: 0,
        };
        assert!(validate_taxable_income(&valid).is_ok());

        let invalid = CitTaxableIncome {
            revenue: -1_000_000_000,
            deductible_expenses: 600_000_000,
            non_deductible_expenses: 50_000_000,
            other_income: 0,
        };
        assert!(validate_taxable_income(&invalid).is_err());
    }

    #[test]
    fn test_cit_checklist() {
        let checklist = get_cit_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 6);
    }
}
