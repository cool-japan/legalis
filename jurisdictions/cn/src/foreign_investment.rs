//! Foreign Investment Law Module (外商投资法)
//!
//! # 中华人民共和国外商投资法 / Foreign Investment Law of the PRC
//!
//! Implements the Foreign Investment Law effective January 1, 2020.
//!
//! ## Key Concepts
//!
//! - **外商投资 (Foreign Investment)**: Investment activities by foreign investors in China
//! - **外商投资企业 (Foreign-Invested Enterprise, FIE)**: Enterprise with foreign investment
//! - **负面清单 (Negative List)**: List of prohibited or restricted investment sectors
//! - **国民待遇 (National Treatment)**: Equal treatment with domestic investors
//!
//! ## Legal Framework
//!
//! ### Pre-establishment National Treatment + Negative List (准入前国民待遇+负面清单)
//!
//! Article 4: Foreign investors enjoy national treatment in areas outside the negative list.
//!
//! ### Investment Forms (Article 2)
//!
//! 1. Establishing FIE
//! 2. Acquiring shares/equity in Chinese companies
//! 3. Investing in new projects
//! 4. Other forms prescribed by law
//!
//! ### Protection Measures
//!
//! - Prohibition of forced technology transfer (Article 22)
//! - Protection of trade secrets and intellectual property (Article 23)
//! - Equal participation in standardization work (Article 15)
//! - Equal access to government procurement (Article 16)
//!
//! ## Security Review
//!
//! Article 35: Security review for foreign investment affecting national security

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Foreign investor type (外国投资者类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForeignInvestorType {
    /// Foreign natural person (外国自然人)
    NaturalPerson,
    /// Foreign enterprise (外国企业)
    Enterprise,
    /// Foreign organization (外国其他组织)
    Organization,
}

impl ForeignInvestorType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::NaturalPerson => BilingualText::new("外国自然人", "Foreign natural person"),
            Self::Enterprise => BilingualText::new("外国企业", "Foreign enterprise"),
            Self::Organization => BilingualText::new("外国其他组织", "Foreign organization"),
        }
    }
}

/// Foreign investor (外国投资者)
///
/// Article 2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignInvestor {
    /// Name
    pub name: String,
    /// Type
    pub investor_type: ForeignInvestorType,
    /// Jurisdiction of incorporation/nationality
    pub jurisdiction: String,
}

/// Investment form (投资方式)
///
/// Article 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestmentForm {
    /// Establish FIE (设立外商投资企业)
    EstablishFIE,
    /// Acquire shares/equity (取得中国境内企业的股份、股权)
    AcquireEquity,
    /// Invest in new projects (投资新建项目)
    NewProject,
    /// Other forms (其他方式)
    Other,
}

impl InvestmentForm {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::EstablishFIE => BilingualText::new("设立外商投资企业", "Establish FIE"),
            Self::AcquireEquity => BilingualText::new("取得股份、股权", "Acquire equity"),
            Self::NewProject => BilingualText::new("投资新建项目", "New project investment"),
            Self::Other => BilingualText::new("其他投资方式", "Other forms"),
        }
    }
}

/// Negative list category (负面清单类别)
///
/// Article 28
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegativeListCategory {
    /// Prohibited (禁止投资)
    Prohibited,
    /// Restricted (限制投资)
    Restricted,
}

impl NegativeListCategory {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Prohibited => BilingualText::new("禁止投资", "Prohibited"),
            Self::Restricted => BilingualText::new("限制投资", "Restricted"),
        }
    }
}

/// Sector classification for negative list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sector {
    /// Sector name
    pub name: BilingualText,
    /// Industry code (if applicable)
    pub industry_code: Option<String>,
    /// Negative list status
    pub negative_list_status: Option<NegativeListCategory>,
    /// Additional requirements
    pub requirements: Vec<BilingualText>,
}

impl Sector {
    /// Check if sector is prohibited for foreign investment
    pub fn is_prohibited(&self) -> bool {
        matches!(
            self.negative_list_status,
            Some(NegativeListCategory::Prohibited)
        )
    }

    /// Check if sector is restricted
    pub fn is_restricted(&self) -> bool {
        matches!(
            self.negative_list_status,
            Some(NegativeListCategory::Restricted)
        )
    }

    /// Check if sector allows foreign investment without restrictions
    pub fn is_open(&self) -> bool {
        self.negative_list_status.is_none()
    }
}

/// Foreign investment project (外商投资项目)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignInvestmentProject {
    /// Project name
    pub project_name: BilingualText,
    /// Foreign investor(s)
    pub foreign_investors: Vec<String>,
    /// Sector
    pub sector: Sector,
    /// Investment form
    pub investment_form: InvestmentForm,
    /// Total investment amount (USD)
    pub investment_amount_usd: f64,
    /// Foreign ownership percentage
    pub foreign_ownership_percentage: f64,
    /// Registration date
    pub registration_date: Option<DateTime<Utc>>,
    /// Affects national security
    pub affects_national_security: bool,
}

impl ForeignInvestmentProject {
    /// Check if security review is required
    ///
    /// Article 35
    pub fn requires_security_review(&self) -> bool {
        self.affects_national_security
    }

    /// Check if investment is permitted
    pub fn is_permitted(&self) -> bool {
        !self.sector.is_prohibited()
    }
}

/// Foreign-invested enterprise (外商投资企业, FIE)
///
/// Article 31
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignInvestedEnterprise {
    /// Enterprise name
    pub name: String,
    /// Enterprise type (LLC, JSC, Partnership, etc.)
    pub enterprise_type: BilingualText,
    /// Sector
    pub sector: Sector,
    /// Foreign investors
    pub foreign_investors: Vec<ForeignInvestor>,
    /// Total foreign ownership percentage
    pub foreign_ownership_percentage: f64,
    /// Registration date
    pub registration_date: DateTime<Utc>,
    /// Registered capital (CNY)
    pub registered_capital_cny: f64,
}

impl ForeignInvestedEnterprise {
    /// Check if subject to special reporting requirements
    ///
    /// Some sectors require special reporting
    pub fn requires_special_reporting(&self) -> bool {
        self.sector.is_restricted()
    }
}

/// Security review (安全审查)
///
/// Article 35
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReview {
    /// Project under review
    pub project_name: BilingualText,
    /// Foreign investor
    pub foreign_investor: String,
    /// Review initiated date
    pub review_initiated: DateTime<Utc>,
    /// Review status
    pub status: SecurityReviewStatus,
    /// Review result
    pub result: Option<SecurityReviewResult>,
}

/// Security review status (审查状态)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityReviewStatus {
    /// Pending (待审查)
    Pending,
    /// Under review (审查中)
    UnderReview,
    /// Completed (已完成)
    Completed,
}

/// Security review result (审查结果)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityReviewResult {
    /// Approved (批准)
    Approved,
    /// Approved with conditions (附条件批准)
    ApprovedWithConditions,
    /// Prohibited (禁止)
    Prohibited,
}

impl SecurityReviewResult {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Approved => BilingualText::new("批准", "Approved"),
            Self::ApprovedWithConditions => {
                BilingualText::new("附条件批准", "Approved with conditions")
            }
            Self::Prohibited => BilingualText::new("禁止", "Prohibited"),
        }
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate foreign investment project
///
/// Articles 4, 28
pub fn validate_foreign_investment_project(
    project: &ForeignInvestmentProject,
) -> Result<(), ForeignInvestmentError> {
    // Check if sector is prohibited
    if project.sector.is_prohibited() {
        return Err(ForeignInvestmentError::ProhibitedSector {
            sector: project.sector.name.clone(),
        });
    }

    // Check if security review is required but not completed
    if project.requires_security_review() && project.registration_date.is_none() {
        return Err(ForeignInvestmentError::SecurityReviewRequired {
            project: project.project_name.clone(),
        });
    }

    // Check ownership percentage is valid
    if project.foreign_ownership_percentage < 0.0 || project.foreign_ownership_percentage > 100.0 {
        return Err(ForeignInvestmentError::InvalidOwnershipPercentage {
            percentage: project.foreign_ownership_percentage,
        });
    }

    Ok(())
}

/// Check if sector is open to foreign investment
///
/// Article 4: National treatment applies outside negative list
pub fn check_sector_openness(sector: &Sector) -> Result<(), ForeignInvestmentError> {
    if sector.is_prohibited() {
        Err(ForeignInvestmentError::ProhibitedSector {
            sector: sector.name.clone(),
        })
    } else if sector.is_restricted() {
        Err(ForeignInvestmentError::RestrictedSector {
            sector: sector.name.clone(),
            requirements: sector.requirements.clone(),
        })
    } else {
        Ok(())
    }
}

/// Validate FIE compliance
///
/// Article 31: FIEs subject to company law, partnership law, etc.
pub fn validate_fie_compliance(
    fie: &ForeignInvestedEnterprise,
) -> Result<(), ForeignInvestmentError> {
    // Check if registered capital is positive
    if fie.registered_capital_cny <= 0.0 {
        return Err(ForeignInvestmentError::InvalidRegisteredCapital {
            capital: fie.registered_capital_cny,
        });
    }

    // Check ownership percentage
    if fie.foreign_ownership_percentage < 0.0 || fie.foreign_ownership_percentage > 100.0 {
        return Err(ForeignInvestmentError::InvalidOwnershipPercentage {
            percentage: fie.foreign_ownership_percentage,
        });
    }

    // Check sector compliance
    check_sector_openness(&fie.sector)?;

    Ok(())
}

// ============================================================================
// Common Sectors
// ============================================================================

/// Create common sector definitions
pub mod sectors {
    use super::*;

    /// Manufacturing sector (generally open)
    pub fn manufacturing() -> Sector {
        Sector {
            name: BilingualText::new("制造业", "Manufacturing"),
            industry_code: None,
            negative_list_status: None,
            requirements: Vec::new(),
        }
    }

    /// Internet information services (restricted in some areas)
    pub fn internet_information_services() -> Sector {
        Sector {
            name: BilingualText::new("互联网信息服务", "Internet information services"),
            industry_code: None,
            negative_list_status: Some(NegativeListCategory::Restricted),
            requirements: vec![BilingualText::new(
                "外资比例不超过50%",
                "Foreign ownership max 50%",
            )],
        }
    }

    /// Telecommunications (restricted)
    pub fn telecommunications() -> Sector {
        Sector {
            name: BilingualText::new("电信业务", "Telecommunications"),
            industry_code: None,
            negative_list_status: Some(NegativeListCategory::Restricted),
            requirements: vec![BilingualText::new(
                "外资比例限制",
                "Foreign ownership restrictions apply",
            )],
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Foreign Investment Law
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ForeignInvestmentError {
    /// Prohibited sector
    #[error("Foreign investment prohibited in sector: {sector}")]
    ProhibitedSector {
        /// Sector name
        sector: BilingualText,
    },

    /// Restricted sector
    #[error("Foreign investment restricted in sector: {sector}. Requirements: {requirements:?}")]
    RestrictedSector {
        /// Sector name
        sector: BilingualText,
        /// Requirements
        requirements: Vec<BilingualText>,
    },

    /// Security review required
    #[error("Security review required for project: {project}")]
    SecurityReviewRequired {
        /// Project name
        project: BilingualText,
    },

    /// Invalid ownership percentage
    #[error("Invalid ownership percentage: {percentage}%")]
    InvalidOwnershipPercentage {
        /// Percentage
        percentage: f64,
    },

    /// Invalid registered capital
    #[error("Invalid registered capital: {capital} CNY")]
    InvalidRegisteredCapital {
        /// Capital amount
        capital: f64,
    },
}

/// Result type for Foreign Investment operations
pub type ForeignInvestmentResult<T> = Result<T, ForeignInvestmentError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sector_openness() {
        let open_sector = sectors::manufacturing();
        assert!(open_sector.is_open());
        assert!(!open_sector.is_prohibited());
        assert!(!open_sector.is_restricted());

        let restricted_sector = sectors::internet_information_services();
        assert!(!restricted_sector.is_open());
        assert!(restricted_sector.is_restricted());
        assert!(!restricted_sector.is_prohibited());
    }

    #[test]
    fn test_foreign_investment_project_validation() {
        let project = ForeignInvestmentProject {
            project_name: BilingualText::new("制造项目", "Manufacturing project"),
            foreign_investors: vec!["Foreign Corp".to_string()],
            sector: sectors::manufacturing(),
            investment_form: InvestmentForm::EstablishFIE,
            investment_amount_usd: 10_000_000.0,
            foreign_ownership_percentage: 100.0,
            registration_date: Some(Utc::now()),
            affects_national_security: false,
        };

        assert!(project.is_permitted());
        assert!(!project.requires_security_review());
        assert!(validate_foreign_investment_project(&project).is_ok());
    }

    #[test]
    fn test_prohibited_sector() {
        let prohibited_sector = Sector {
            name: BilingualText::new("禁止行业", "Prohibited sector"),
            industry_code: None,
            negative_list_status: Some(NegativeListCategory::Prohibited),
            requirements: Vec::new(),
        };

        let project = ForeignInvestmentProject {
            project_name: BilingualText::new("项目", "Project"),
            foreign_investors: vec!["Foreign Corp".to_string()],
            sector: prohibited_sector,
            investment_form: InvestmentForm::EstablishFIE,
            investment_amount_usd: 1_000_000.0,
            foreign_ownership_percentage: 100.0,
            registration_date: None,
            affects_national_security: false,
        };

        assert!(!project.is_permitted());
        assert!(validate_foreign_investment_project(&project).is_err());
    }

    #[test]
    fn test_security_review_requirement() {
        let project = ForeignInvestmentProject {
            project_name: BilingualText::new("安全敏感项目", "Security-sensitive project"),
            foreign_investors: vec!["Foreign Corp".to_string()],
            sector: sectors::manufacturing(),
            investment_form: InvestmentForm::EstablishFIE,
            investment_amount_usd: 100_000_000.0,
            foreign_ownership_percentage: 100.0,
            registration_date: None,
            affects_national_security: true,
        };

        assert!(project.requires_security_review());
        assert!(validate_foreign_investment_project(&project).is_err());
    }

    #[test]
    fn test_fie_validation() {
        let fie = ForeignInvestedEnterprise {
            name: "某外商投资企业".to_string(),
            enterprise_type: BilingualText::new("有限责任公司", "Limited Liability Company"),
            sector: sectors::manufacturing(),
            foreign_investors: vec![ForeignInvestor {
                name: "Foreign Corp".to_string(),
                investor_type: ForeignInvestorType::Enterprise,
                jurisdiction: "US".to_string(),
            }],
            foreign_ownership_percentage: 100.0,
            registration_date: Utc::now(),
            registered_capital_cny: 10_000_000.0,
        };

        assert!(!fie.requires_special_reporting());
        assert!(validate_fie_compliance(&fie).is_ok());
    }

    #[test]
    fn test_invalid_ownership_percentage() {
        let project = ForeignInvestmentProject {
            project_name: BilingualText::new("项目", "Project"),
            foreign_investors: vec!["Foreign Corp".to_string()],
            sector: sectors::manufacturing(),
            investment_form: InvestmentForm::EstablishFIE,
            investment_amount_usd: 1_000_000.0,
            foreign_ownership_percentage: 150.0, // Invalid
            registration_date: Some(Utc::now()),
            affects_national_security: false,
        };

        assert!(validate_foreign_investment_project(&project).is_err());
    }
}
