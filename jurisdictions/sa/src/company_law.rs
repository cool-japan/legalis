//! Saudi Companies Law (نظام الشركات)
//!
//! Royal Decree No. M/3 dated 28/1/1437H (2015)
//!
//! Regulates company formation, governance, and dissolution in Saudi Arabia.
//! Key features include 100% foreign ownership in many sectors post-Vision 2030.

use crate::common::Sar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for company law operations
pub type CompanyResult<T> = Result<T, CompanyError>;

/// Company law errors
#[derive(Debug, Error)]
pub enum CompanyError {
    /// Invalid registration
    #[error("تسجيل غير صالح: {reason}")]
    InvalidRegistration { reason: String },

    /// Insufficient capital
    #[error("رأس المال غير كافٍ: required {required}, provided {provided}")]
    InsufficientCapital { required: Sar, provided: Sar },

    /// Governance violation
    #[error("انتهاك حوكمة الشركات: {description}")]
    GovernanceViolation { description: String },
}

/// Types of companies under Saudi law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Limited Liability Company (شركة ذات مسؤولية محدودة)
    Llc,
    /// Joint Stock Company (شركة مساهمة)
    JointStock,
    /// Simplified Joint Stock Company (شركة مساهمة مبسطة)
    SimplifiedJointStock,
    /// General Partnership (شركة التضامن)
    GeneralPartnership,
    /// Limited Partnership (شركة التوصية البسيطة)
    LimitedPartnership,
    /// Professional Company (شركة مهنية)
    Professional,
}

impl CompanyType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Llc => "شركة ذات مسؤولية محدودة",
            Self::JointStock => "شركة مساهمة",
            Self::SimplifiedJointStock => "شركة مساهمة مبسطة",
            Self::GeneralPartnership => "شركة التضامن",
            Self::LimitedPartnership => "شركة التوصية البسيطة",
            Self::Professional => "شركة مهنية",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Llc => "Limited Liability Company (LLC)",
            Self::JointStock => "Joint Stock Company (JSC)",
            Self::SimplifiedJointStock => "Simplified Joint Stock Company",
            Self::GeneralPartnership => "General Partnership",
            Self::LimitedPartnership => "Limited Partnership",
            Self::Professional => "Professional Company",
        }
    }

    /// Get minimum capital requirement
    pub fn minimum_capital(&self) -> Sar {
        match self {
            Self::Llc => Sar::from_riyals(0), // No minimum as of 2023 reform
            Self::JointStock => Sar::from_riyals(500_000),
            Self::SimplifiedJointStock => Sar::from_riyals(0), // Flexible
            Self::GeneralPartnership => Sar::from_riyals(0),
            Self::LimitedPartnership => Sar::from_riyals(0),
            Self::Professional => Sar::from_riyals(0),
        }
    }

    /// Get minimum number of shareholders
    pub fn minimum_shareholders(&self) -> u32 {
        match self {
            Self::Llc => 1,        // Single-person LLC allowed
            Self::JointStock => 2, // Reduced from 5 in 2023
            Self::SimplifiedJointStock => 1,
            Self::GeneralPartnership => 2,
            Self::LimitedPartnership => 2,
            Self::Professional => 2,
        }
    }

    /// Check if limited liability applies
    pub fn has_limited_liability(&self) -> bool {
        matches!(
            self,
            Self::Llc | Self::JointStock | Self::SimplifiedJointStock | Self::Professional
        )
    }
}

/// Company registration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyRegistration {
    /// Company type
    pub company_type: CompanyType,
    /// Company name (Arabic)
    pub name_ar: String,
    /// Company name (English)
    pub name_en: String,
    /// Capital amount
    pub capital: Sar,
    /// Number of shareholders
    pub shareholders_count: u32,
    /// Sector/activity
    pub sector: String,
    /// Foreign ownership percentage (0-100)
    pub foreign_ownership_pct: f64,
}

impl CompanyRegistration {
    /// Create new company registration
    pub fn new(
        company_type: CompanyType,
        name_ar: impl Into<String>,
        name_en: impl Into<String>,
        capital: Sar,
    ) -> Self {
        Self {
            company_type,
            name_ar: name_ar.into(),
            name_en: name_en.into(),
            capital,
            shareholders_count: 1,
            sector: String::new(),
            foreign_ownership_pct: 0.0,
        }
    }

    /// Set shareholders count
    pub fn with_shareholders(mut self, count: u32) -> Self {
        self.shareholders_count = count;
        self
    }

    /// Set sector
    pub fn with_sector(mut self, sector: impl Into<String>) -> Self {
        self.sector = sector.into();
        self
    }

    /// Set foreign ownership percentage
    pub fn with_foreign_ownership(mut self, percentage: f64) -> Self {
        self.foreign_ownership_pct = percentage;
        self
    }
}

/// Corporate governance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRequirements {
    /// Board of directors required
    pub requires_board: bool,
    /// Audit committee required
    pub requires_audit_committee: bool,
    /// External auditor required
    pub requires_external_auditor: bool,
    /// Shareholder meeting frequency (months)
    pub shareholder_meeting_frequency: u32,
}

impl GovernanceRequirements {
    /// Get governance requirements for company type
    pub fn for_company_type(company_type: &CompanyType) -> Self {
        match company_type {
            CompanyType::JointStock => Self {
                requires_board: true,
                requires_audit_committee: true,
                requires_external_auditor: true,
                shareholder_meeting_frequency: 12,
            },
            CompanyType::Llc => Self {
                requires_board: false,
                requires_audit_committee: false,
                requires_external_auditor: true,
                shareholder_meeting_frequency: 12,
            },
            CompanyType::SimplifiedJointStock => Self {
                requires_board: true,
                requires_audit_committee: false,
                requires_external_auditor: true,
                shareholder_meeting_frequency: 12,
            },
            _ => Self {
                requires_board: false,
                requires_audit_committee: false,
                requires_external_auditor: false,
                shareholder_meeting_frequency: 12,
            },
        }
    }
}

/// Validate company registration
pub fn validate_registration(registration: &CompanyRegistration) -> CompanyResult<()> {
    // Check minimum capital
    let min_capital = registration.company_type.minimum_capital();
    if registration.capital < min_capital {
        return Err(CompanyError::InsufficientCapital {
            required: min_capital,
            provided: registration.capital,
        });
    }

    // Check minimum shareholders
    let min_shareholders = registration.company_type.minimum_shareholders();
    if registration.shareholders_count < min_shareholders {
        return Err(CompanyError::InvalidRegistration {
            reason: format!(
                "Minimum {} shareholders required, provided {}",
                min_shareholders, registration.shareholders_count
            ),
        });
    }

    // Check foreign ownership (most sectors allow 100% as of Vision 2030)
    if registration.foreign_ownership_pct < 0.0 || registration.foreign_ownership_pct > 100.0 {
        return Err(CompanyError::InvalidRegistration {
            reason: "Foreign ownership must be between 0% and 100%".to_string(),
        });
    }

    // Check names
    if registration.name_ar.is_empty() || registration.name_en.is_empty() {
        return Err(CompanyError::InvalidRegistration {
            reason: "Company name required in both Arabic and English".to_string(),
        });
    }

    Ok(())
}

/// Get company registration checklist
pub fn get_company_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("اختيار نوع الشركة", "Choose company type"),
        ("تحديد رأس المال", "Determine capital amount"),
        ("إعداد عقد التأسيس", "Prepare Articles of Association"),
        (
            "تسجيل في وزارة التجارة",
            "Register with Ministry of Commerce",
        ),
        ("الحصول على السجل التجاري", "Obtain Commercial Registration"),
        ("فتح حساب بنكي", "Open bank account"),
        ("التسجيل الضريبي", "Tax registration (ZATCA)"),
        ("التسجيل في التأمينات الاجتماعية", "Register with GOSI"),
        ("الحصول على الرخص اللازمة", "Obtain necessary licenses"),
        ("توثيق العقود", "Notarize documents"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_types() {
        assert_eq!(CompanyType::Llc.name_ar(), "شركة ذات مسؤولية محدودة");
        assert!(CompanyType::Llc.has_limited_liability());
        assert_eq!(CompanyType::Llc.minimum_shareholders(), 1);
    }

    #[test]
    fn test_minimum_capital() {
        assert_eq!(CompanyType::Llc.minimum_capital().riyals(), 0);
        assert_eq!(CompanyType::JointStock.minimum_capital().riyals(), 500_000);
    }

    #[test]
    fn test_valid_registration() {
        let registration = CompanyRegistration::new(
            CompanyType::Llc,
            "شركة الاختبار",
            "Test Company LLC",
            Sar::from_riyals(100_000),
        )
        .with_shareholders(2)
        .with_sector("Technology")
        .with_foreign_ownership(50.0);

        assert!(validate_registration(&registration).is_ok());
    }

    #[test]
    fn test_insufficient_capital() {
        let registration = CompanyRegistration::new(
            CompanyType::JointStock,
            "شركة مساهمة",
            "JSC Test",
            Sar::from_riyals(100_000), // Less than 500,000 required
        );

        assert!(validate_registration(&registration).is_err());
    }

    #[test]
    fn test_governance_requirements() {
        let jsc_gov = GovernanceRequirements::for_company_type(&CompanyType::JointStock);
        assert!(jsc_gov.requires_board);
        assert!(jsc_gov.requires_audit_committee);

        let llc_gov = GovernanceRequirements::for_company_type(&CompanyType::Llc);
        assert!(!llc_gov.requires_board);
        assert!(!llc_gov.requires_audit_committee);
    }

    #[test]
    fn test_checklist() {
        let checklist = get_company_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
