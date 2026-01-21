//! Vietnamese Investment Law 2020 (Luật Đầu tư 2020) - Law No. 61/2020/QH14
//!
//! Vietnam's comprehensive investment law governing domestic and foreign investment.
//!
//! ## Key Features
//!
//! - State ownership of all land (land use rights only)
//! - Conditional investment sectors (Ngành nghề đầu tư có điều kiện)
//! - Prohibited investment sectors
//! - Investment incentives (ưu đãi đầu tư)
//! - Special Economic Zones (Khu kinh tế)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Investment sector classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentSector {
    /// Open sector (no restrictions)
    Open,
    /// Conditional sector (ngành nghề có điều kiện)
    Conditional(Vec<String>),
    /// Prohibited sector (ngành nghề cấm đầu tư)
    Prohibited,
}

/// Types of investment incentives - Article 15-16
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentIncentive {
    /// Corporate income tax exemption/reduction
    TaxExemption { years: u32 },
    /// Reduced CIT rate
    ReducedTaxRate { rate_percent: u32, years: u32 },
    /// Import duty exemption
    ImportDutyExemption,
    /// Land rent exemption/reduction
    LandRentExemption { years: u32 },
    /// Accelerated depreciation
    AcceleratedDepreciation,
}

/// Investment project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentProject {
    /// Project name
    pub name: String,
    /// Investor name
    pub investor: String,
    /// Is foreign investor
    pub is_foreign_investor: bool,
    /// Investment capital (VND)
    pub investment_capital: i64,
    /// Project location (province)
    pub location: String,
    /// In special economic zone
    pub in_sez: bool,
    /// Business lines (VSIC codes)
    pub business_lines: Vec<String>,
    /// Land use area (m2)
    pub land_area: Option<f64>,
    /// Employment (Vietnamese workers)
    pub planned_employment: u32,
    /// Investment incentives applied
    pub incentives: Vec<InvestmentIncentive>,
}

impl InvestmentProject {
    /// Check if project qualifies for special incentives
    /// Based on location, sector, capital amount
    pub fn qualifies_for_incentives(&self) -> bool {
        // High-tech, R&D, infrastructure, disadvantaged areas
        self.in_sez || self.investment_capital >= 6_000_000_000_000 // 6 trillion VND
    }
}

/// Result type for investment operations
pub type InvestmentResult<T> = Result<T, InvestmentError>;

/// Errors related to Investment Law
#[derive(Debug, Error)]
pub enum InvestmentError {
    /// Prohibited sector - Article 6
    #[error("Ngành nghề cấm đầu tư (Điều 6 LĐT): {sector}")]
    ProhibitedSector { sector: String },

    /// Conditional sector without conditions met - Article 7
    #[error("Chưa đáp ứng điều kiện đầu tư (Điều 7 LĐT): {sector} - {condition}")]
    ConditionNotMet { sector: String, condition: String },

    /// Foreign ownership restriction
    #[error("Hạn chế sở hữu nước ngoài: {sector} - tối đa {limit}%")]
    ForeignOwnershipLimit { sector: String, limit: u32 },

    /// Land use rights issue
    #[error("Vấn đề quyền sử dụng đất: {description}")]
    LandUseRightsIssue { description: String },

    /// Investment registration required
    #[error("Cần đăng ký đầu tư: {reason}")]
    RegistrationRequired { reason: String },
}

/// Check investment sector eligibility
pub fn check_sector_eligibility(
    business_line: &str,
    is_foreign_investor: bool,
) -> InvestmentResult<InvestmentSector> {
    // Simplified check - in reality would check full list
    match business_line {
        // Prohibited sectors
        "drugs" | "prostitution" | "human_trafficking" => Err(InvestmentError::ProhibitedSector {
            sector: business_line.to_string(),
        }),
        // Conditional sectors (examples)
        "banking" | "insurance" | "securities" => {
            if is_foreign_investor {
                Ok(InvestmentSector::Conditional(vec![
                    "Foreign ownership cap".to_string(),
                    "License required".to_string(),
                ]))
            } else {
                Ok(InvestmentSector::Conditional(vec![
                    "License required".to_string(),
                ]))
            }
        }
        // Open sectors
        _ => Ok(InvestmentSector::Open),
    }
}

/// Validate investment project
pub fn validate_investment_project(project: &InvestmentProject) -> InvestmentResult<()> {
    // Check business lines
    for line in &project.business_lines {
        check_sector_eligibility(line, project.is_foreign_investor)?;
    }

    Ok(())
}

/// Get investment law checklist
pub fn get_investment_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Ngành nghề không thuộc danh mục cấm",
            "Sector not prohibited",
            "Điều 6",
        ),
        (
            "Đáp ứng điều kiện đầu tư (nếu có)",
            "Investment conditions met",
            "Điều 7",
        ),
        (
            "Đăng ký đầu tư / Giấy chứng nhận đầu tư",
            "Investment registration / IRC",
            "Điều 37",
        ),
        ("Đăng ký doanh nghiệp", "Enterprise registration", "Điều 22"),
        (
            "Quyền sử dụng đất hợp pháp",
            "Legal land use rights",
            "Luật Đất đai",
        ),
        (
            "Đánh giá tác động môi trường (nếu cần)",
            "Environmental impact assessment",
            "Luật BVMT",
        ),
    ]
}

/// Special Economic Zones in Vietnam
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialEconomicZone {
    /// Khu công nghiệp (Industrial Zone)
    IndustrialZone(String),
    /// Khu chế xuất (Export Processing Zone)
    ExportProcessingZone(String),
    /// Khu kinh tế (Economic Zone)
    EconomicZone(String),
    /// Khu công nghệ cao (High-Tech Zone)
    HighTechZone(String),
}

impl SpecialEconomicZone {
    /// Get name in Vietnamese
    pub fn name_vi(&self) -> String {
        match self {
            Self::IndustrialZone(name) => format!("Khu công nghiệp {}", name),
            Self::ExportProcessingZone(name) => format!("Khu chế xuất {}", name),
            Self::EconomicZone(name) => format!("Khu kinh tế {}", name),
            Self::HighTechZone(name) => format!("Khu công nghệ cao {}", name),
        }
    }

    /// Get CIT incentive rate
    pub fn cit_incentive_rate(&self) -> u32 {
        match self {
            Self::HighTechZone(_) => 10, // 10% for 15 years
            Self::EconomicZone(_) => 10,
            Self::ExportProcessingZone(_) => 10,
            Self::IndustrialZone(_) => 17, // Standard incentive rate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sector_eligibility_open() {
        let result = check_sector_eligibility("software", false);
        assert!(matches!(result, Ok(InvestmentSector::Open)));
    }

    #[test]
    fn test_sector_eligibility_prohibited() {
        let result = check_sector_eligibility("drugs", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_sector_eligibility_conditional() {
        let result = check_sector_eligibility("banking", true);
        assert!(matches!(result, Ok(InvestmentSector::Conditional(_))));
    }

    #[test]
    fn test_investment_project() {
        let project = InvestmentProject {
            name: "Tech Project".to_string(),
            investor: "Foreign Corp".to_string(),
            is_foreign_investor: true,
            investment_capital: 1_000_000_000_000,
            location: "Hanoi".to_string(),
            in_sez: true,
            business_lines: vec!["software".to_string()],
            land_area: Some(10000.0),
            planned_employment: 100,
            incentives: vec![],
        };

        assert!(project.qualifies_for_incentives());
        assert!(validate_investment_project(&project).is_ok());
    }

    #[test]
    fn test_sez_incentives() {
        let htz = SpecialEconomicZone::HighTechZone("Hòa Lạc".to_string());
        assert_eq!(htz.cit_incentive_rate(), 10);
    }

    #[test]
    fn test_investment_checklist() {
        let checklist = get_investment_checklist();
        assert!(!checklist.is_empty());
    }
}
