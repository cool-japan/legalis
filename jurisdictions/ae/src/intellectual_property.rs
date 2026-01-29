//! UAE Intellectual Property Law
//!
//! Comprehensive IP protection in the UAE covering:
//! - **Trademarks** - Federal Law No. 37/1992
//! - **Patents** - Federal Law No. 31/2006
//! - **Copyright** - Federal Law No. 7/2002
//! - **Industrial Designs** - Federal Law No. 31/2006
//!
//! ## IP Authorities
//!
//! - **Ministry of Economy** - Trademarks, Patents, Industrial Designs
//! - **Ministry of Economy** - Copyright registration
//! - **UAE Courts** - IP enforcement

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for IP operations
pub type IpResult<T> = Result<T, IpError>;

/// Types of intellectual property
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpType {
    /// Trademark (علامة تجارية)
    Trademark,
    /// Patent (براءة اختراع)
    Patent,
    /// Copyright (حق المؤلف)
    Copyright,
    /// Industrial design (تصميم صناعي)
    IndustrialDesign,
    /// Trade secret (سر تجاري)
    TradeSecret,
}

impl IpType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Trademark => "علامة تجارية",
            Self::Patent => "براءة اختراع",
            Self::Copyright => "حق المؤلف",
            Self::IndustrialDesign => "تصميم صناعي",
            Self::TradeSecret => "سر تجاري",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Trademark => "Trademark",
            Self::Patent => "Patent",
            Self::Copyright => "Copyright",
            Self::IndustrialDesign => "Industrial Design",
            Self::TradeSecret => "Trade Secret",
        }
    }

    /// Get protection duration (years, 0 = varies)
    pub fn protection_duration_years(&self) -> u32 {
        match self {
            Self::Trademark => 10, // Renewable
            Self::Patent => 20,    // From filing date
            Self::Copyright => 50, // After author's death
            Self::IndustrialDesign => 10,
            Self::TradeSecret => 0, // As long as kept secret
        }
    }

    /// Check if registration is required
    pub fn requires_registration(&self) -> bool {
        !matches!(self, Self::Copyright | Self::TradeSecret)
    }
}

/// Trademark classification - Nice Classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trademark {
    /// Trademark name/sign
    pub mark: String,
    /// Nice classification classes (1-45)
    pub classes: Vec<u32>,
    /// Registration number (if registered)
    pub registration_number: Option<String>,
    /// Filing date
    pub filing_date: Option<String>,
    /// Registration date
    pub registration_date: Option<String>,
    /// Expiry date (10 years from registration)
    pub expiry_date: Option<String>,
}

impl Trademark {
    /// Check if trademark is active
    pub fn is_active(&self) -> bool {
        self.registration_number.is_some()
    }

    /// Calculate renewal fee (per class)
    pub fn renewal_fee_per_class() -> Aed {
        Aed::from_dirhams(3_000) // Approximate
    }

    /// Calculate total renewal fee
    pub fn total_renewal_fee(&self) -> Aed {
        let fee_per_class = Self::renewal_fee_per_class();
        Aed::from_fils(fee_per_class.fils() * self.classes.len() as i64)
    }
}

/// Patent types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatentType {
    /// Invention patent (براءة اختراع)
    Invention,
    /// Utility model (نموذج منفعة)
    UtilityModel,
}

impl PatentType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Invention => "براءة اختراع",
            Self::UtilityModel => "نموذج منفعة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Invention => "Invention Patent",
            Self::UtilityModel => "Utility Model",
        }
    }

    /// Protection duration (years)
    pub fn duration_years(&self) -> u32 {
        match self {
            Self::Invention => 20,
            Self::UtilityModel => 10,
        }
    }
}

/// Patent requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patent {
    /// Patent title
    pub title: String,
    /// Patent type
    pub patent_type: PatentType,
    /// Is novel (جديد)
    pub is_novel: bool,
    /// Involves inventive step (نشاط ابتكاري)
    pub inventive_step: bool,
    /// Is industrially applicable (قابل للتطبيق الصناعي)
    pub industrially_applicable: bool,
    /// Application number
    pub application_number: Option<String>,
    /// Grant date
    pub grant_date: Option<String>,
}

impl Patent {
    /// Check if patent requirements are met
    pub fn meets_requirements(&self) -> IpResult<()> {
        if !self.is_novel {
            return Err(IpError::NoveltyRequired);
        }

        if !self.inventive_step {
            return Err(IpError::InventiveStepRequired);
        }

        if !self.industrially_applicable {
            return Err(IpError::IndustrialApplicabilityRequired);
        }

        Ok(())
    }

    /// Get filing fee
    pub fn filing_fee() -> Aed {
        Aed::from_dirhams(1_000)
    }

    /// Get annual maintenance fee (increases over time)
    pub fn annual_maintenance_fee(year: u32) -> Aed {
        let base_fee = 500;
        let increase = year * 100;
        Aed::from_dirhams((base_fee + increase) as i64)
    }
}

/// Copyright subject matter
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CopyrightWork {
    /// Literary work (مصنف أدبي)
    Literary,
    /// Artistic work (مصنف فني)
    Artistic,
    /// Musical work (مصنف موسيقي)
    Musical,
    /// Audiovisual work (مصنف سمعي بصري)
    Audiovisual,
    /// Software (برنامج حاسوبي)
    Software,
    /// Database (قاعدة بيانات)
    Database,
}

impl CopyrightWork {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Literary => "مصنف أدبي",
            Self::Artistic => "مصنف فني",
            Self::Musical => "مصنف موسيقي",
            Self::Audiovisual => "مصنف سمعي بصري",
            Self::Software => "برنامج حاسوبي",
            Self::Database => "قاعدة بيانات",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Literary => "Literary Work",
            Self::Artistic => "Artistic Work",
            Self::Musical => "Musical Work",
            Self::Audiovisual => "Audiovisual Work",
            Self::Software => "Software",
            Self::Database => "Database",
        }
    }

    /// Protection duration (years after author's death, or from creation)
    pub fn protection_duration(&self) -> u32 {
        match self {
            Self::Software => 50, // From creation
            Self::Database => 25, // From creation
            _ => 50,              // After author's death
        }
    }
}

/// IP enforcement actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnforcementAction {
    /// Cease and desist letter
    CeaseAndDesist,
    /// Customs recordation
    CustomsRecordation,
    /// Civil lawsuit
    CivilLawsuit,
    /// Criminal complaint
    CriminalComplaint,
    /// Administrative action
    AdministrativeAction,
}

impl EnforcementAction {
    /// Get action name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::CeaseAndDesist => "Cease and Desist Letter",
            Self::CustomsRecordation => "Customs Recordation",
            Self::CivilLawsuit => "Civil Lawsuit",
            Self::CriminalComplaint => "Criminal Complaint",
            Self::AdministrativeAction => "Administrative Action",
        }
    }
}

/// Intellectual property errors
#[derive(Debug, Error)]
pub enum IpError {
    /// Novelty required for patent
    #[error("يجب أن يكون الاختراع جديداً (حداثة)")]
    NoveltyRequired,

    /// Inventive step required
    #[error("يجب أن يتضمن نشاطاً ابتكارياً")]
    InventiveStepRequired,

    /// Industrial applicability required
    #[error("يجب أن يكون قابلاً للتطبيق الصناعي")]
    IndustrialApplicabilityRequired,

    /// Trademark already registered
    #[error("العلامة التجارية مسجلة بالفعل: {mark}")]
    TrademarkAlreadyRegistered { mark: String },

    /// Invalid Nice class
    #[error("فئة نيس غير صالحة: {class}")]
    InvalidNiceClass { class: u32 },

    /// Registration required
    #[error("التسجيل مطلوب لحماية {ip_type}")]
    RegistrationRequired { ip_type: String },
}

/// Get IP registration checklist
pub fn get_ip_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("البحث عن العلامة", "Trademark search", "Before filing"),
        ("تقديم الطلب", "File application", "Ministry of Economy"),
        ("فحص رسمي", "Formal examination", "1-2 months"),
        ("النشر في الجريدة", "Publication", "Opposition period"),
        ("التسجيل", "Registration", "If no opposition"),
        ("دفع الرسوم", "Pay fees", "Initial + renewal"),
        ("التجديد", "Renewal", "Every 10 years (TM)"),
        ("الإنفاذ", "Enforcement", "Customs/Courts"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_types() {
        let trademark = IpType::Trademark;
        assert_eq!(trademark.name_ar(), "علامة تجارية");
        assert_eq!(trademark.protection_duration_years(), 10);
        assert!(trademark.requires_registration());

        let copyright = IpType::Copyright;
        assert!(!copyright.requires_registration());
    }

    #[test]
    fn test_trademark() {
        let tm = Trademark {
            mark: "TestBrand".to_string(),
            classes: vec![9, 35, 42],
            registration_number: Some("TM12345".to_string()),
            filing_date: Some("2024-01-15".to_string()),
            registration_date: Some("2024-06-15".to_string()),
            expiry_date: Some("2034-06-15".to_string()),
        };

        assert!(tm.is_active());
        assert_eq!(tm.total_renewal_fee().dirhams(), 9_000); // 3 classes * 3000
    }

    #[test]
    fn test_patent_types() {
        let invention = PatentType::Invention;
        assert_eq!(invention.duration_years(), 20);
        assert_eq!(invention.name_ar(), "براءة اختراع");

        let utility = PatentType::UtilityModel;
        assert_eq!(utility.duration_years(), 10);
    }

    #[test]
    fn test_patent_requirements() {
        let valid_patent = Patent {
            title: "Novel Device".to_string(),
            patent_type: PatentType::Invention,
            is_novel: true,
            inventive_step: true,
            industrially_applicable: true,
            application_number: None,
            grant_date: None,
        };

        assert!(valid_patent.meets_requirements().is_ok());

        let invalid_patent = Patent {
            title: "Device".to_string(),
            patent_type: PatentType::Invention,
            is_novel: false,
            inventive_step: true,
            industrially_applicable: true,
            application_number: None,
            grant_date: None,
        };

        assert!(invalid_patent.meets_requirements().is_err());
    }

    #[test]
    fn test_copyright_works() {
        let software = CopyrightWork::Software;
        assert_eq!(software.name_ar(), "برنامج حاسوبي");
        assert_eq!(software.protection_duration(), 50);

        let database = CopyrightWork::Database;
        assert_eq!(database.protection_duration(), 25);
    }

    #[test]
    fn test_patent_fees() {
        assert_eq!(Patent::filing_fee().dirhams(), 1_000);
        assert_eq!(Patent::annual_maintenance_fee(1).dirhams(), 600);
        assert_eq!(Patent::annual_maintenance_fee(10).dirhams(), 1_500);
    }

    #[test]
    fn test_enforcement_actions() {
        let action = EnforcementAction::CivilLawsuit;
        assert_eq!(action.name_en(), "Civil Lawsuit");
    }

    #[test]
    fn test_ip_checklist() {
        let checklist = get_ip_checklist();
        assert!(!checklist.is_empty());
    }
}
