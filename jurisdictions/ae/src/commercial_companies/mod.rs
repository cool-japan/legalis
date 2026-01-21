//! UAE Commercial Companies Law - Federal Decree-Law No. 32/2021
//!
//! Governs commercial companies in the UAE, replacing Federal Law No. 2/2015.
//!
//! ## Key Changes (2021)
//!
//! - 100% foreign ownership allowed in many sectors (previously 51% local required)
//! - Streamlined company formation process
//! - New governance requirements
//! - Enhanced shareholder protections
//!
//! ## Company Types (Article 8)
//!
//! - Limited Liability Company (LLC) - Sharikat Dhat Mas'uliya Mahduda
//! - Public Joint Stock Company (PJSC)
//! - Private Joint Stock Company (PrJSC)
//! - Partnership - Limited & Unlimited
//! - Sole Proprietorship

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for company law operations
pub type CompanyResult<T> = Result<T, CompanyError>;

/// Company types under UAE law - Article 8
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Limited Liability Company (شركة ذات مسؤولية محدودة)
    Llc,
    /// Public Joint Stock Company (شركة مساهمة عامة)
    Pjsc,
    /// Private Joint Stock Company (شركة مساهمة خاصة)
    PrJsc,
    /// General Partnership (شركة تضامن)
    GeneralPartnership,
    /// Limited Partnership (شركة التوصية البسيطة)
    LimitedPartnership,
    /// Partnership Limited by Shares (شركة التوصية بالأسهم)
    PartnershipLimitedByShares,
    /// Sole Proprietorship (مؤسسة فردية)
    SoleProprietorship,
    /// Branch of Foreign Company (فرع شركة أجنبية)
    ForeignBranch,
    /// Free Zone Company (شركة منطقة حرة)
    FreeZoneCompany { zone: String },
}

impl CompanyType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Llc => "شركة ذات مسؤولية محدودة",
            Self::Pjsc => "شركة مساهمة عامة",
            Self::PrJsc => "شركة مساهمة خاصة",
            Self::GeneralPartnership => "شركة تضامن",
            Self::LimitedPartnership => "شركة التوصية البسيطة",
            Self::PartnershipLimitedByShares => "شركة التوصية بالأسهم",
            Self::SoleProprietorship => "مؤسسة فردية",
            Self::ForeignBranch => "فرع شركة أجنبية",
            Self::FreeZoneCompany { .. } => "شركة منطقة حرة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Llc => "Limited Liability Company",
            Self::Pjsc => "Public Joint Stock Company",
            Self::PrJsc => "Private Joint Stock Company",
            Self::GeneralPartnership => "General Partnership",
            Self::LimitedPartnership => "Limited Partnership",
            Self::PartnershipLimitedByShares => "Partnership Limited by Shares",
            Self::SoleProprietorship => "Sole Proprietorship",
            Self::ForeignBranch => "Foreign Branch",
            Self::FreeZoneCompany { .. } => "Free Zone Company",
        }
    }

    /// Get standard abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::Llc => "LLC",
            Self::Pjsc => "PJSC",
            Self::PrJsc => "PrJSC",
            Self::GeneralPartnership => "GP",
            Self::LimitedPartnership => "LP",
            Self::PartnershipLimitedByShares => "PLS",
            Self::SoleProprietorship => "SP",
            Self::ForeignBranch => "Branch",
            Self::FreeZoneCompany { .. } => "FZC",
        }
    }

    /// Check if company type has limited liability
    pub fn has_limited_liability(&self) -> bool {
        matches!(
            self,
            Self::Llc
                | Self::Pjsc
                | Self::PrJsc
                | Self::LimitedPartnership
                | Self::FreeZoneCompany { .. }
        )
    }

    /// Minimum capital requirement (if any)
    pub fn minimum_capital(&self) -> Option<Aed> {
        match self {
            Self::Pjsc => Some(Aed::from_dirhams(30_000_000)), // 30 million
            Self::PrJsc => Some(Aed::from_dirhams(5_000_000)), // 5 million
            Self::Llc => None,                                 // No minimum specified in 2021 law
            _ => None,
        }
    }

    /// Minimum number of shareholders/partners
    pub fn minimum_shareholders(&self) -> u32 {
        match self {
            Self::Llc => 1,   // Single member LLC allowed
            Self::Pjsc => 5,  // Minimum 5 founders
            Self::PrJsc => 2, // Minimum 2 shareholders
            Self::GeneralPartnership => 2,
            Self::LimitedPartnership => 2, // 1 general + 1 limited
            Self::PartnershipLimitedByShares => 2,
            Self::SoleProprietorship => 1,
            Self::ForeignBranch => 0, // Branch, not separate entity
            Self::FreeZoneCompany { .. } => 1,
        }
    }

    /// Maximum number of shareholders (if limited)
    pub fn maximum_shareholders(&self) -> Option<u32> {
        match self {
            Self::Llc => Some(50),
            Self::PrJsc => Some(200),
            _ => None,
        }
    }
}

/// Company registration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyRegistration {
    /// Company name (must be in Arabic + optional English)
    pub name_ar: String,
    /// Company name in English (optional)
    pub name_en: Option<String>,
    /// Company type
    pub company_type: CompanyType,
    /// Registered capital (AED)
    pub capital: Aed,
    /// Number of shareholders/partners
    pub shareholder_count: u32,
    /// Foreign ownership percentage
    pub foreign_ownership_percent: Option<u32>,
    /// Business activities (license codes)
    pub activities: Vec<String>,
    /// Registered address emirate
    pub emirate: String,
    /// Is in free zone
    pub is_free_zone: bool,
}

impl CompanyRegistration {
    /// Check if registration meets basic requirements
    pub fn is_valid(&self) -> bool {
        // Check shareholder count
        let min_shareholders = self.company_type.minimum_shareholders();
        if self.shareholder_count < min_shareholders {
            return false;
        }

        if let Some(max) = self.company_type.maximum_shareholders()
            && self.shareholder_count > max
        {
            return false;
        }

        // Check minimum capital
        if let Some(min_capital) = self.company_type.minimum_capital()
            && self.capital.fils() < min_capital.fils()
        {
            return false;
        }

        // Check name
        if self.name_ar.is_empty() {
            return false;
        }

        true
    }
}

/// Company governance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRequirements {
    /// Minimum board members
    pub min_board_members: u32,
    /// Maximum board members
    pub max_board_members: u32,
    /// Requires independent directors
    pub requires_independent_directors: bool,
    /// Independent director percentage (if required)
    pub independent_director_percent: Option<u32>,
    /// Requires audit committee
    pub requires_audit_committee: bool,
    /// Requires annual general meeting
    pub requires_agm: bool,
    /// Financial reporting period (months)
    pub reporting_period_months: u32,
}

impl GovernanceRequirements {
    /// Get requirements for company type
    pub fn for_company_type(company_type: &CompanyType) -> Self {
        match company_type {
            CompanyType::Pjsc => Self {
                min_board_members: 5,
                max_board_members: 11,
                requires_independent_directors: true,
                independent_director_percent: Some(30),
                requires_audit_committee: true,
                requires_agm: true,
                reporting_period_months: 12,
            },
            CompanyType::PrJsc => Self {
                min_board_members: 3,
                max_board_members: 11,
                requires_independent_directors: false,
                independent_director_percent: None,
                requires_audit_committee: false,
                requires_agm: true,
                reporting_period_months: 12,
            },
            CompanyType::Llc => Self {
                min_board_members: 0, // Manager, not board
                max_board_members: 0,
                requires_independent_directors: false,
                independent_director_percent: None,
                requires_audit_committee: false,
                requires_agm: false, // Partners' meeting
                reporting_period_months: 12,
            },
            _ => Self {
                min_board_members: 0,
                max_board_members: 0,
                requires_independent_directors: false,
                independent_director_percent: None,
                requires_audit_committee: false,
                requires_agm: false,
                reporting_period_months: 12,
            },
        }
    }
}

/// Errors related to Commercial Companies Law
#[derive(Debug, Error)]
pub enum CompanyError {
    /// Insufficient capital - Article varies by company type
    #[error("رأس المال غير كافٍ: {actual} درهم (المطلوب {required} درهم)")]
    InsufficientCapital { actual: i64, required: i64 },

    /// Shareholder count violation
    #[error("عدد المساهمين غير صحيح: {actual} (المطلوب {min}-{max})")]
    InvalidShareholderCount { actual: u32, min: u32, max: u32 },

    /// Company name requirements - Article 12-15
    #[error("اسم الشركة غير مطابق للمتطلبات (المادة 12-15): {reason}")]
    InvalidCompanyName { reason: String },

    /// Foreign ownership restriction (for certain activities)
    #[error("نسبة الملكية الأجنبية تتجاوز الحد المسموح: {actual}% (الحد الأقصى {limit}%)")]
    ForeignOwnershipExceeded { actual: u32, limit: u32 },

    /// Activity restriction
    #[error("النشاط غير مسموح به لهذا النوع من الشركات: {activity}")]
    RestrictedActivity { activity: String },

    /// Governance violation
    #[error("مخالفة متطلبات الحوكمة (المادة {article}): {description}")]
    GovernanceViolation { article: u32, description: String },
}

/// Validate company registration
pub fn validate_registration(reg: &CompanyRegistration) -> CompanyResult<()> {
    // Check shareholder count
    let min = reg.company_type.minimum_shareholders();
    let max = reg.company_type.maximum_shareholders().unwrap_or(u32::MAX);

    if reg.shareholder_count < min || reg.shareholder_count > max {
        return Err(CompanyError::InvalidShareholderCount {
            actual: reg.shareholder_count,
            min,
            max,
        });
    }

    // Check minimum capital
    if let Some(min_capital) = reg.company_type.minimum_capital()
        && reg.capital.fils() < min_capital.fils()
    {
        return Err(CompanyError::InsufficientCapital {
            actual: reg.capital.dirhams(),
            required: min_capital.dirhams(),
        });
    }

    // Check name
    if reg.name_ar.is_empty() || reg.name_ar.len() < 2 {
        return Err(CompanyError::InvalidCompanyName {
            reason: "الاسم قصير جداً".to_string(),
        });
    }

    Ok(())
}

/// Get company law compliance checklist
pub fn get_company_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "اسم الشركة بالعربية",
            "Company name in Arabic",
            "Article 12",
        ),
        ("رأس المال المسجل", "Registered capital", "Article 73+"),
        ("عقد التأسيس والنظام الأساسي", "MoA and AoA", "Article 16-17"),
        ("الترخيص التجاري", "Trade license", "DED/Free Zone"),
        (
            "مجلس الإدارة (شركات المساهمة)",
            "Board of Directors (JSC)",
            "Article 142+",
        ),
        ("تعيين مراقب حسابات", "Auditor appointment", "Article 196"),
        (
            "الجمعية العمومية السنوية",
            "Annual General Meeting",
            "Article 169+",
        ),
        ("السجل التجاري", "Commercial Register", "Article 21"),
    ]
}

/// Sectors requiring local partner or special license
pub fn get_restricted_sectors() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("Oil & Gas", "النفط والغاز", 51), // 51% local ownership
        ("Defense", "الدفاع", 51),
        ("Banking", "البنوك", 0), // Central Bank license required
        ("Insurance", "التأمين", 0),
        ("Telecom", "الاتصالات", 0),
        ("Media", "الإعلام", 51),
        ("Education", "التعليم", 49), // 49% foreign max
        ("Healthcare", "الرعاية الصحية", 0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_types() {
        let llc = CompanyType::Llc;
        assert!(llc.has_limited_liability());
        assert_eq!(llc.minimum_shareholders(), 1);
        assert_eq!(llc.maximum_shareholders(), Some(50));
    }

    #[test]
    fn test_pjsc_capital_requirement() {
        let pjsc = CompanyType::Pjsc;
        assert_eq!(
            pjsc.minimum_capital().map(|c| c.dirhams()),
            Some(30_000_000)
        );
    }

    #[test]
    fn test_registration_valid() {
        let reg = CompanyRegistration {
            name_ar: "شركة الاختبار ذ.م.م".to_string(),
            name_en: Some("Test Company LLC".to_string()),
            company_type: CompanyType::Llc,
            capital: Aed::from_dirhams(100_000),
            shareholder_count: 2,
            foreign_ownership_percent: Some(100),
            activities: vec!["IT Services".to_string()],
            emirate: "Dubai".to_string(),
            is_free_zone: false,
        };

        assert!(reg.is_valid());
        assert!(validate_registration(&reg).is_ok());
    }

    #[test]
    fn test_registration_invalid_shareholders() {
        let reg = CompanyRegistration {
            name_ar: "شركة".to_string(),
            name_en: None,
            company_type: CompanyType::Llc,
            capital: Aed::from_dirhams(100_000),
            shareholder_count: 100, // Exceeds max 50
            foreign_ownership_percent: None,
            activities: vec![],
            emirate: "Abu Dhabi".to_string(),
            is_free_zone: false,
        };

        assert!(!reg.is_valid());
    }

    #[test]
    fn test_pjsc_insufficient_capital() {
        let reg = CompanyRegistration {
            name_ar: "شركة مساهمة عامة".to_string(),
            name_en: None,
            company_type: CompanyType::Pjsc,
            capital: Aed::from_dirhams(1_000_000), // Below 30M requirement
            shareholder_count: 5,
            foreign_ownership_percent: None,
            activities: vec![],
            emirate: "Dubai".to_string(),
            is_free_zone: false,
        };

        assert!(validate_registration(&reg).is_err());
    }

    #[test]
    fn test_governance_requirements() {
        let pjsc_gov = GovernanceRequirements::for_company_type(&CompanyType::Pjsc);
        assert!(pjsc_gov.requires_independent_directors);
        assert!(pjsc_gov.requires_audit_committee);
        assert_eq!(pjsc_gov.min_board_members, 5);

        let llc_gov = GovernanceRequirements::for_company_type(&CompanyType::Llc);
        assert!(!llc_gov.requires_agm);
    }

    #[test]
    fn test_company_checklist() {
        let checklist = get_company_checklist();
        assert!(!checklist.is_empty());
    }

    #[test]
    fn test_restricted_sectors() {
        let sectors = get_restricted_sectors();
        assert!(!sectors.is_empty());
    }
}
