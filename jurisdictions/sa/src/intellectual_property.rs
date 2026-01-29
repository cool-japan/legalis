//! Intellectual Property Laws (الملكية الفكرية)
//!
//! Saudi Arabia has comprehensive IP protection including:
//! - Patents Law (نظام براءات الاختراع)
//! - Trademarks Law (نظام العلامات التجارية)
//! - Copyright Law (نظام حماية حقوق المؤلف)
//!
//! Administered by SAIP (Saudi Authority for Intellectual Property)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for IP operations
pub type IpResult<T> = Result<T, IpError>;

/// IP errors
#[derive(Debug, Error)]
pub enum IpError {
    /// Invalid registration
    #[error("تسجيل غير صالح: {reason}")]
    InvalidRegistration { reason: String },

    /// Infringement detected
    #[error("انتهاك حقوق الملكية الفكرية: {description}")]
    Infringement { description: String },
}

/// Types of intellectual property
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpType {
    /// Patent (براءة اختراع)
    Patent,
    /// Trademark (علامة تجارية)
    Trademark,
    /// Copyright (حق مؤلف)
    Copyright,
    /// Industrial Design (تصميم صناعي)
    IndustrialDesign,
    /// Trade Secret (سر تجاري)
    TradeSecret,
}

impl IpType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Patent => "براءة اختراع",
            Self::Trademark => "علامة تجارية",
            Self::Copyright => "حق مؤلف",
            Self::IndustrialDesign => "تصميم صناعي",
            Self::TradeSecret => "سر تجاري",
        }
    }

    /// Get protection duration in years
    pub fn protection_duration_years(&self) -> Option<u32> {
        match self {
            Self::Patent => Some(20),    // 20 years from filing
            Self::Trademark => Some(10), // 10 years, renewable
            Self::Copyright => Some(50), // 50 years after author's death
            Self::IndustrialDesign => Some(10),
            Self::TradeSecret => None, // Indefinite if maintained
        }
    }
}

/// Patent types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatentType {
    /// Invention patent
    Invention,
    /// Utility model
    UtilityModel,
}

/// Trademark classes (Nice Classification)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrademarkClass {
    /// Class number (1-45)
    pub class_number: u32,
    /// Description
    pub description: String,
}

impl TrademarkClass {
    /// Create new trademark class
    pub fn new(class_number: u32, description: impl Into<String>) -> IpResult<Self> {
        if !(1..=45).contains(&class_number) {
            return Err(IpError::InvalidRegistration {
                reason: "Trademark class must be between 1 and 45".to_string(),
            });
        }
        Ok(Self {
            class_number,
            description: description.into(),
        })
    }
}

/// IP registration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRegistration {
    /// IP type
    pub ip_type: IpType,
    /// Registration number
    pub registration_number: Option<String>,
    /// Title/Name
    pub title: String,
    /// Owner name
    pub owner: String,
    /// Filing date
    pub filing_date: Option<chrono::NaiveDate>,
    /// Grant/Registration date
    pub grant_date: Option<chrono::NaiveDate>,
}

impl IpRegistration {
    /// Create new IP registration
    pub fn new(ip_type: IpType, title: impl Into<String>, owner: impl Into<String>) -> Self {
        Self {
            ip_type,
            registration_number: None,
            title: title.into(),
            owner: owner.into(),
            filing_date: None,
            grant_date: None,
        }
    }

    /// Set registration number
    pub fn with_registration_number(mut self, number: impl Into<String>) -> Self {
        self.registration_number = Some(number.into());
        self
    }

    /// Set filing date
    pub fn with_filing_date(mut self, date: chrono::NaiveDate) -> Self {
        self.filing_date = Some(date);
        self
    }

    /// Calculate expiry date
    pub fn expiry_date(&self) -> Option<chrono::NaiveDate> {
        let grant_date = self.grant_date?;
        let duration = self.ip_type.protection_duration_years()?;

        // Add years to grant date
        grant_date.checked_add_signed(chrono::Duration::days((duration * 365) as i64))
    }
}

/// Get IP registration checklist
pub fn get_ip_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("بحث الأسبقية", "Prior art/trademark search"),
        ("إعداد الطلب", "Prepare application"),
        (
            "التقديم لدى الهيئة السعودية للملكية الفكرية",
            "File with SAIP",
        ),
        ("دفع الرسوم", "Pay filing fees"),
        ("الفحص الشكلي", "Formal examination"),
        ("الفحص الموضوعي", "Substantive examination"),
        ("النشر", "Publication"),
        ("الاعتراضات", "Opposition period"),
        ("التسجيل النهائي", "Final registration"),
        ("الصيانة والتجديد", "Maintenance and renewal"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_types() {
        assert_eq!(IpType::Patent.name_ar(), "براءة اختراع");
        assert_eq!(IpType::Patent.protection_duration_years(), Some(20));
        assert_eq!(IpType::Trademark.protection_duration_years(), Some(10));
        assert_eq!(IpType::TradeSecret.protection_duration_years(), None);
    }

    #[test]
    fn test_trademark_class() {
        let class = TrademarkClass::new(25, "Clothing, footwear, headgear");
        assert!(class.is_ok());

        let invalid = TrademarkClass::new(50, "Invalid");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_ip_registration() {
        let registration =
            IpRegistration::new(IpType::Patent, "Innovative Device", "Inventor Name")
                .with_registration_number("SA12345");

        assert_eq!(
            registration.registration_number,
            Some("SA12345".to_string())
        );
        assert_eq!(registration.title, "Innovative Device");
    }

    #[test]
    fn test_checklist() {
        let checklist = get_ip_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
