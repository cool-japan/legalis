//! UAE Cybercrime Law - Federal Decree-Law No. 34/2021
//!
//! Combating Rumours and Cybercrimes
//!
//! ## Key Features
//!
//! - Criminalizes cybercrimes including hacking, identity theft, online fraud
//! - Penalties for spreading false information and rumors online
//! - Protection of information systems and networks
//! - Data privacy violations
//! - Electronic evidence procedures
//!
//! ## Penalties
//!
//! Penalties range from fines to imprisonment and deportation for non-citizens.
//! Serious cybercrimes can result in imprisonment up to 25 years.

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for cybercrime operations
pub type CybercrimeResult<T> = Result<T, CybercrimeError>;

/// Types of cybercrimes under UAE law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CybercrimeType {
    /// Unauthorized access to information system (Article 2)
    UnauthorizedAccess,
    /// Hacking (اختراق) (Article 3)
    Hacking,
    /// Phishing (تصيد) (Article 8)
    Phishing,
    /// Identity theft (سرقة الهوية) (Article 21)
    IdentityTheft,
    /// Online fraud (احتيال إلكتروني) (Article 11)
    OnlineFraud,
    /// Cyberstalking (مطاردة إلكترونية) (Article 18)
    Cyberstalking,
    /// Spreading false information (نشر معلومات كاذبة) (Article 44)
    FalseInformation,
    /// Online defamation (قذف إلكتروني) (Article 43)
    OnlineDefamation,
    /// Hate speech (خطاب كراهية) (Article 26)
    HateSpeech,
    /// Data breach (اختراق بيانات) (Article 15)
    DataBreach,
    /// DDoS attack (هجوم حجب الخدمة) (Article 4)
    DdosAttack,
    /// Malware distribution (نشر برامج ضارة) (Article 6)
    MalwareDistribution,
}

impl CybercrimeType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::UnauthorizedAccess => "الوصول غير المصرح به",
            Self::Hacking => "اختراق",
            Self::Phishing => "تصيد",
            Self::IdentityTheft => "سرقة الهوية",
            Self::OnlineFraud => "احتيال إلكتروني",
            Self::Cyberstalking => "مطاردة إلكترونية",
            Self::FalseInformation => "نشر معلومات كاذبة",
            Self::OnlineDefamation => "قذف إلكتروني",
            Self::HateSpeech => "خطاب كراهية",
            Self::DataBreach => "اختراق بيانات",
            Self::DdosAttack => "هجوم حجب الخدمة",
            Self::MalwareDistribution => "نشر برامج ضارة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::UnauthorizedAccess => "Unauthorized Access",
            Self::Hacking => "Hacking",
            Self::Phishing => "Phishing",
            Self::IdentityTheft => "Identity Theft",
            Self::OnlineFraud => "Online Fraud",
            Self::Cyberstalking => "Cyberstalking",
            Self::FalseInformation => "Spreading False Information",
            Self::OnlineDefamation => "Online Defamation",
            Self::HateSpeech => "Hate Speech",
            Self::DataBreach => "Data Breach",
            Self::DdosAttack => "DDoS Attack",
            Self::MalwareDistribution => "Malware Distribution",
        }
    }

    /// Get article reference
    pub fn article_reference(&self) -> u32 {
        match self {
            Self::UnauthorizedAccess => 2,
            Self::Hacking => 3,
            Self::Phishing => 8,
            Self::IdentityTheft => 21,
            Self::OnlineFraud => 11,
            Self::Cyberstalking => 18,
            Self::FalseInformation => 44,
            Self::OnlineDefamation => 43,
            Self::HateSpeech => 26,
            Self::DataBreach => 15,
            Self::DdosAttack => 4,
            Self::MalwareDistribution => 6,
        }
    }

    /// Get minimum fine (AED)
    pub fn minimum_fine(&self) -> Aed {
        match self {
            Self::UnauthorizedAccess => Aed::from_dirhams(100_000),
            Self::Hacking => Aed::from_dirhams(500_000),
            Self::Phishing => Aed::from_dirhams(250_000),
            Self::IdentityTheft => Aed::from_dirhams(200_000),
            Self::OnlineFraud => Aed::from_dirhams(250_000),
            Self::Cyberstalking => Aed::from_dirhams(100_000),
            Self::FalseInformation => Aed::from_dirhams(100_000),
            Self::OnlineDefamation => Aed::from_dirhams(200_000),
            Self::HateSpeech => Aed::from_dirhams(500_000),
            Self::DataBreach => Aed::from_dirhams(300_000),
            Self::DdosAttack => Aed::from_dirhams(500_000),
            Self::MalwareDistribution => Aed::from_dirhams(250_000),
        }
    }

    /// Get maximum imprisonment (years)
    pub fn maximum_imprisonment_years(&self) -> u32 {
        match self {
            Self::UnauthorizedAccess => 2,
            Self::Hacking => 10,
            Self::Phishing => 5,
            Self::IdentityTheft => 10,
            Self::OnlineFraud => 10,
            Self::Cyberstalking => 2,
            Self::FalseInformation => 3,
            Self::OnlineDefamation => 3,
            Self::HateSpeech => 10,
            Self::DataBreach => 15,
            Self::DdosAttack => 15,
            Self::MalwareDistribution => 10,
        }
    }
}

/// Electronic evidence types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectronicEvidence {
    /// Email messages
    Email,
    /// Social media posts
    SocialMedia,
    /// Chat messages
    ChatMessages,
    /// Website content
    WebsiteContent,
    /// Server logs
    ServerLogs,
    /// Digital images/videos
    DigitalMedia,
    /// Metadata
    Metadata,
    /// Blockchain records
    Blockchain,
}

impl ElectronicEvidence {
    /// Get evidence type name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Email => "Email Messages",
            Self::SocialMedia => "Social Media Posts",
            Self::ChatMessages => "Chat Messages",
            Self::WebsiteContent => "Website Content",
            Self::ServerLogs => "Server Logs",
            Self::DigitalMedia => "Digital Images/Videos",
            Self::Metadata => "Metadata",
            Self::Blockchain => "Blockchain Records",
        }
    }

    /// Check if admissible in UAE courts
    pub fn is_admissible(&self) -> bool {
        true // All electronic evidence is admissible under UAE law
    }
}

/// Cybersecurity requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CybersecurityCompliance {
    /// Has information security policy
    pub has_security_policy: bool,
    /// Regular security audits conducted
    pub security_audits: bool,
    /// Incident response plan exists
    pub incident_response_plan: bool,
    /// Data encryption implemented
    pub data_encryption: bool,
    /// Access control measures
    pub access_control: bool,
    /// Employee training
    pub employee_training: bool,
}

impl CybersecurityCompliance {
    /// Check if compliant with basic cybersecurity requirements
    pub fn is_compliant(&self) -> CybercrimeResult<()> {
        if !self.has_security_policy {
            return Err(CybercrimeError::SecurityPolicyRequired);
        }

        if !self.incident_response_plan {
            return Err(CybercrimeError::IncidentResponseRequired);
        }

        if !self.access_control {
            return Err(CybercrimeError::AccessControlRequired);
        }

        Ok(())
    }
}

/// Data breach notification requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBreachNotification {
    /// Breach occurred
    pub breach_occurred: bool,
    /// Notification to authorities (within 72 hours)
    pub authorities_notified: bool,
    /// Notification to affected individuals
    pub individuals_notified: bool,
    /// Hours since breach discovery
    pub hours_since_discovery: u32,
    /// Number of affected individuals
    pub affected_count: u32,
}

impl DataBreachNotification {
    /// Check if notification requirements are met
    pub fn is_compliant(&self) -> CybercrimeResult<()> {
        if self.breach_occurred && self.hours_since_discovery > 72 && !self.authorities_notified {
            return Err(CybercrimeError::BreachNotificationOverdue {
                hours: self.hours_since_discovery,
            });
        }

        Ok(())
    }
}

/// Cybercrime errors
#[derive(Debug, Error)]
pub enum CybercrimeError {
    /// Security policy required
    #[error("يجب وجود سياسة أمن المعلومات")]
    SecurityPolicyRequired,

    /// Incident response plan required
    #[error("يجب وجود خطة الاستجابة للحوادث")]
    IncidentResponseRequired,

    /// Access control required
    #[error("يجب تطبيق ضوابط الوصول")]
    AccessControlRequired,

    /// Breach notification overdue
    #[error("إخطار اختراق البيانات متأخر: {hours} ساعة (الحد الأقصى 72)")]
    BreachNotificationOverdue { hours: u32 },

    /// Cybercrime detected
    #[error("جريمة إلكترونية: {crime_type}")]
    CybercrimeDetected { crime_type: String },
}

/// Get cybersecurity compliance checklist
pub fn get_cybersecurity_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("سياسة أمن المعلومات", "Information security policy"),
        ("تقييم المخاطر", "Risk assessment"),
        ("تشفير البيانات", "Data encryption"),
        ("ضوابط الوصول", "Access control measures"),
        ("المصادقة متعددة العوامل", "Multi-factor authentication"),
        ("النسخ الاحتياطي", "Regular data backups"),
        ("خطة الاستجابة للحوادث", "Incident response plan"),
        ("التدريب الأمني", "Security awareness training"),
        ("المراقبة والتسجيل", "Monitoring and logging"),
        ("تحديثات الأمان", "Security updates and patches"),
    ]
}

/// Get common cybercrimes and penalties
pub fn get_common_cybercrimes() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("اختراق", "Hacking", "Up to 10 years + AED 500k+"),
        ("تصيد", "Phishing", "Up to 5 years + AED 250k+"),
        (
            "سرقة الهوية",
            "Identity Theft",
            "Up to 10 years + AED 200k+",
        ),
        (
            "احتيال إلكتروني",
            "Online Fraud",
            "Up to 10 years + AED 250k+",
        ),
        (
            "نشر معلومات كاذبة",
            "False Information",
            "Up to 3 years + AED 100k+",
        ),
        (
            "قذف إلكتروني",
            "Online Defamation",
            "Up to 3 years + AED 200k+",
        ),
        ("خطاب كراهية", "Hate Speech", "Up to 10 years + AED 500k+"),
        ("اختراق بيانات", "Data Breach", "Up to 15 years + AED 300k+"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cybercrime_types() {
        let hacking = CybercrimeType::Hacking;
        assert_eq!(hacking.name_ar(), "اختراق");
        assert_eq!(hacking.article_reference(), 3);
        assert_eq!(hacking.minimum_fine().dirhams(), 500_000);
        assert_eq!(hacking.maximum_imprisonment_years(), 10);
    }

    #[test]
    fn test_electronic_evidence() {
        let email = ElectronicEvidence::Email;
        assert!(email.is_admissible());
        assert_eq!(email.name_en(), "Email Messages");
    }

    #[test]
    fn test_cybersecurity_compliance() {
        let compliant = CybersecurityCompliance {
            has_security_policy: true,
            security_audits: true,
            incident_response_plan: true,
            data_encryption: true,
            access_control: true,
            employee_training: true,
        };

        assert!(compliant.is_compliant().is_ok());

        let non_compliant = CybersecurityCompliance {
            has_security_policy: false,
            security_audits: false,
            incident_response_plan: false,
            data_encryption: false,
            access_control: false,
            employee_training: false,
        };

        assert!(non_compliant.is_compliant().is_err());
    }

    #[test]
    fn test_data_breach_notification() {
        let timely = DataBreachNotification {
            breach_occurred: true,
            authorities_notified: true,
            individuals_notified: true,
            hours_since_discovery: 48,
            affected_count: 100,
        };

        assert!(timely.is_compliant().is_ok());

        let overdue = DataBreachNotification {
            breach_occurred: true,
            authorities_notified: false,
            individuals_notified: false,
            hours_since_discovery: 100,
            affected_count: 1000,
        };

        assert!(overdue.is_compliant().is_err());
    }

    #[test]
    fn test_cybersecurity_checklist() {
        let checklist = get_cybersecurity_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }

    #[test]
    fn test_common_cybercrimes() {
        let crimes = get_common_cybercrimes();
        assert!(!crimes.is_empty());
    }

    #[test]
    fn test_severe_cybercrimes() {
        let data_breach = CybercrimeType::DataBreach;
        assert_eq!(data_breach.maximum_imprisonment_years(), 15);

        let ddos = CybercrimeType::DdosAttack;
        assert_eq!(ddos.maximum_imprisonment_years(), 15);
    }
}
