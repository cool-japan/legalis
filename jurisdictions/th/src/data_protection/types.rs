//! PDPA Types and Structures

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Legal bases for data processing under PDPA (Section 24)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Consent of the data subject (Section 19)
    Consent,

    /// Necessary for the performance of a contract (Section 24(3))
    ContractPerformance,

    /// Compliance with a legal obligation (Section 24(6))
    LegalObligation,

    /// Protection of vital interests (Section 24(2))
    VitalInterests,

    /// Performance of a task in public interest (Section 24(4))
    PublicTask,

    /// Legitimate interests (Section 24(5))
    LegitimateInterests,
}

impl LegalBasis {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Consent => "ความยินยอม",
            Self::ContractPerformance => "การปฏิบัติตามสัญญา",
            Self::LegalObligation => "การปฏิบัติตามกฎหมาย",
            Self::VitalInterests => "ประโยชน์สำคัญต่อชีวิต",
            Self::PublicTask => "การปฏิบัติหน้าที่สาธารณะ",
            Self::LegitimateInterests => "ประโยชน์โดยชอบด้วยกฎหมาย",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Consent => "Consent",
            Self::ContractPerformance => "Contract Performance",
            Self::LegalObligation => "Legal Obligation",
            Self::VitalInterests => "Vital Interests",
            Self::PublicTask => "Public Task",
            Self::LegitimateInterests => "Legitimate Interests",
        }
    }

    /// Get PDPA citation
    pub fn citation(&self) -> String {
        let pdpa = ThaiAct::new(
            "คุ้มครองข้อมูลส่วนบุคคล",
            "Personal Data Protection Act",
            BuddhistYear::from_be(2562),
        );

        let section = match self {
            Self::Consent => 19,
            Self::ContractPerformance
            | Self::LegalObligation
            | Self::VitalInterests
            | Self::PublicTask
            | Self::LegitimateInterests => 24,
        };

        pdpa.section(section).format_th()
    }
}

/// Data subject rights under PDPA (Sections 30-36)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSubjectRight {
    /// Right to be informed (Section 23)
    Information,

    /// Right of access (Section 30)
    Access,

    /// Right to rectification (Section 35)
    Rectification,

    /// Right to erasure (Section 33)
    Erasure,

    /// Right to restriction of processing (Section 34)
    Restriction,

    /// Right to data portability (Section 31)
    Portability,

    /// Right to object (Section 32)
    Objection,

    /// Rights related to automated decision-making (Section 36)
    AutomatedDecision,
}

impl DataSubjectRight {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Information => "สิทธิในการได้รับแจ้งข้อมูล",
            Self::Access => "สิทธิในการเข้าถึงข้อมูล",
            Self::Rectification => "สิทธิในการแก้ไขข้อมูล",
            Self::Erasure => "สิทธิในการลบข้อมูล",
            Self::Restriction => "สิทธิในการระงับการประมวลผล",
            Self::Portability => "สิทธิในการโอนย้ายข้อมูล",
            Self::Objection => "สิทธิในการคัดค้าน",
            Self::AutomatedDecision => "สิทธิเกี่ยวกับการตัดสินใจอัตโนมัติ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Information => "Right to Information",
            Self::Access => "Right of Access",
            Self::Rectification => "Right to Rectification",
            Self::Erasure => "Right to Erasure",
            Self::Restriction => "Right to Restriction",
            Self::Portability => "Right to Data Portability",
            Self::Objection => "Right to Object",
            Self::AutomatedDecision => "Automated Decision-Making Rights",
        }
    }

    /// Get PDPA section number
    pub fn section(&self) -> u32 {
        match self {
            Self::Information => 23,
            Self::Access => 30,
            Self::Rectification => 35,
            Self::Erasure => 33,
            Self::Restriction => 34,
            Self::Portability => 31,
            Self::Objection => 32,
            Self::AutomatedDecision => 36,
        }
    }

    /// Get response deadline in days
    pub fn response_deadline_days(&self) -> u32 {
        // PDPA generally requires response within 30 days
        30
    }
}

/// Categories of personal data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCategory {
    /// Basic identification data
    Identification,

    /// Contact information
    Contact,

    /// Financial data
    Financial,

    /// Location data
    Location,

    /// Online identifiers
    OnlineIdentifiers,

    /// Health data (sensitive)
    Health,

    /// Biometric data (sensitive)
    Biometric,

    /// Racial or ethnic origin (sensitive)
    RaceEthnicity,

    /// Religious beliefs (sensitive)
    Religion,

    /// Political opinions (sensitive)
    Political,

    /// Criminal records (sensitive)
    Criminal,

    /// Sexual orientation (sensitive)
    Sexual,

    /// Trade union membership (sensitive)
    TradeUnion,

    /// Genetic data (sensitive)
    Genetic,
}

impl DataCategory {
    /// Check if this is sensitive data under PDPA (Section 26)
    pub fn is_sensitive(&self) -> bool {
        matches!(
            self,
            Self::Health
                | Self::Biometric
                | Self::RaceEthnicity
                | Self::Religion
                | Self::Political
                | Self::Criminal
                | Self::Sexual
                | Self::TradeUnion
                | Self::Genetic
        )
    }

    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Identification => "ข้อมูลระบุตัวตน",
            Self::Contact => "ข้อมูลติดต่อ",
            Self::Financial => "ข้อมูลทางการเงิน",
            Self::Location => "ข้อมูลตำแหน่งที่ตั้ง",
            Self::OnlineIdentifiers => "ข้อมูลระบุตัวตนออนไลน์",
            Self::Health => "ข้อมูลสุขภาพ",
            Self::Biometric => "ข้อมูลชีวมิติ",
            Self::RaceEthnicity => "ข้อมูลเชื้อชาติหรือเผ่าพันธุ์",
            Self::Religion => "ข้อมูลความเชื่อทางศาสนา",
            Self::Political => "ข้อมูลความคิดเห็นทางการเมือง",
            Self::Criminal => "ข้อมูลประวัติอาชญากรรม",
            Self::Sexual => "ข้อมูลเกี่ยวกับเพศ",
            Self::TradeUnion => "ข้อมูลการเป็นสมาชิกสหภาพแรงงาน",
            Self::Genetic => "ข้อมูลพันธุกรรม",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Identification => "Identification",
            Self::Contact => "Contact",
            Self::Financial => "Financial",
            Self::Location => "Location",
            Self::OnlineIdentifiers => "Online Identifiers",
            Self::Health => "Health",
            Self::Biometric => "Biometric",
            Self::RaceEthnicity => "Race/Ethnicity",
            Self::Religion => "Religion",
            Self::Political => "Political Opinions",
            Self::Criminal => "Criminal Records",
            Self::Sexual => "Sexual Orientation",
            Self::TradeUnion => "Trade Union Membership",
            Self::Genetic => "Genetic Data",
        }
    }
}

/// Processing purpose with legal basis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingPurpose {
    /// Purpose description in Thai
    pub purpose_th: String,

    /// Purpose description in English
    pub purpose_en: String,

    /// Legal basis for this processing
    pub legal_basis: LegalBasis,
}

impl ProcessingPurpose {
    /// Create a new processing purpose
    pub fn new(
        purpose_th: impl Into<String>,
        purpose_en: impl Into<String>,
        legal_basis: LegalBasis,
    ) -> Self {
        Self {
            purpose_th: purpose_th.into(),
            purpose_en: purpose_en.into(),
            legal_basis,
        }
    }
}

/// Personal data processing activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDataProcessing {
    /// Unique identifier
    pub id: String,

    /// Data categories involved
    pub data_categories: Vec<DataCategory>,

    /// Processing purposes
    pub purposes: Vec<ProcessingPurpose>,

    /// Data subjects (groups)
    pub data_subjects: Vec<String>,

    /// Retention period in months
    pub retention_months: Option<u32>,

    /// Whether data is transferred internationally
    pub international_transfer: bool,

    /// Transfer destination countries
    pub transfer_countries: Vec<String>,

    /// Whether processing includes sensitive data
    pub has_sensitive_data: bool,

    /// Whether automated decision-making is used
    pub automated_decision: bool,
}

impl PersonalDataProcessing {
    /// Check if processing has sensitive data
    pub fn contains_sensitive_data(&self) -> bool {
        self.has_sensitive_data || self.data_categories.iter().any(|c| c.is_sensitive())
    }
}

/// Consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Data subject identifier
    pub subject_id: String,

    /// Date consent was given
    pub consent_date: NaiveDate,

    /// Purpose of consent (Thai)
    pub purpose_th: String,

    /// Purpose of consent (English)
    pub purpose_en: String,

    /// Consent text shown to subject
    pub consent_text: String,

    /// Method of obtaining consent
    pub method: String,

    /// Whether consent is currently active
    pub active: bool,

    /// Date consent was withdrawn (if applicable)
    pub withdrawal_date: Option<NaiveDate>,
}

impl ConsentRecord {
    /// Check if consent is valid
    pub fn is_valid(&self) -> bool {
        self.active && self.withdrawal_date.is_none()
    }
}

/// Security incident (breach) record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    /// Incident identifier
    pub id: String,

    /// Date and time of detection
    pub detection_time: DateTime<Utc>,

    /// Date and time incident occurred (if known)
    pub occurrence_time: Option<DateTime<Utc>>,

    /// Description in Thai
    pub description_th: String,

    /// Description in English
    pub description_en: String,

    /// Data categories affected
    pub affected_data_categories: Vec<DataCategory>,

    /// Estimated number of subjects affected
    pub estimated_affected_subjects: u32,

    /// Risk level (1-5)
    pub risk_level: u8,

    /// Whether PDPC has been notified
    pub pdpc_notified: bool,

    /// Whether affected subjects have been notified
    pub subjects_notified: bool,
}

impl SecurityIncident {
    /// Check if PDPC notification is required (high risk)
    pub fn requires_pdpc_notification(&self) -> bool {
        self.risk_level >= 3
            || self
                .affected_data_categories
                .iter()
                .any(|c| c.is_sensitive())
    }

    /// Check if subject notification is required
    pub fn requires_subject_notification(&self) -> bool {
        self.risk_level >= 4 || (self.risk_level >= 3 && self.estimated_affected_subjects > 100)
    }

    /// Get hours since detection
    pub fn hours_since_detection(&self) -> i64 {
        let now = Utc::now();
        (now - self.detection_time).num_hours()
    }

    /// Check if 72-hour notification deadline has passed
    pub fn notification_deadline_passed(&self) -> bool {
        self.hours_since_detection() > 72
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_basis_citation() {
        let basis = LegalBasis::Consent;
        let citation = basis.citation();
        assert!(citation.contains("มาตรา 19"));
    }

    #[test]
    fn test_data_category_sensitive() {
        assert!(DataCategory::Health.is_sensitive());
        assert!(DataCategory::Biometric.is_sensitive());
        assert!(!DataCategory::Contact.is_sensitive());
        assert!(!DataCategory::Identification.is_sensitive());
    }

    #[test]
    fn test_data_subject_right_section() {
        assert_eq!(DataSubjectRight::Access.section(), 30);
        assert_eq!(DataSubjectRight::Erasure.section(), 33);
    }

    #[test]
    fn test_consent_record() {
        let consent = ConsentRecord {
            subject_id: "123".to_string(),
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            purpose_th: "การตลาด".to_string(),
            purpose_en: "Marketing".to_string(),
            consent_text: "ยอมรับ".to_string(),
            method: "Click".to_string(),
            active: true,
            withdrawal_date: None,
        };

        assert!(consent.is_valid());
    }

    #[test]
    fn test_consent_record_withdrawn() {
        let consent = ConsentRecord {
            subject_id: "123".to_string(),
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            purpose_th: "การตลาด".to_string(),
            purpose_en: "Marketing".to_string(),
            consent_text: "ยอมรับ".to_string(),
            method: "Click".to_string(),
            active: false,
            withdrawal_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date")),
        };

        assert!(!consent.is_valid());
    }

    #[test]
    fn test_processing_sensitive_data() {
        let processing = PersonalDataProcessing {
            id: "1".to_string(),
            data_categories: vec![DataCategory::Health, DataCategory::Contact],
            purposes: vec![],
            data_subjects: vec!["Patients".to_string()],
            retention_months: Some(24),
            international_transfer: false,
            transfer_countries: vec![],
            has_sensitive_data: false,
            automated_decision: false,
        };

        assert!(processing.contains_sensitive_data());
    }

    #[test]
    fn test_legal_basis_names() {
        let basis = LegalBasis::ContractPerformance;
        assert_eq!(basis.name_th(), "การปฏิบัติตามสัญญา");
        assert_eq!(basis.name_en(), "Contract Performance");
    }

    #[test]
    fn test_data_category_names() {
        let category = DataCategory::Biometric;
        assert_eq!(category.name_th(), "ข้อมูลชีวมิติ");
        assert_eq!(category.name_en(), "Biometric");
    }
}
