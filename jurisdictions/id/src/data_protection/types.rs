//! Types for Indonesian Personal Data Protection Law (UU PDP)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Category of personal data under UU PDP
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCategory {
    /// Data Pribadi Umum (General Personal Data) - Pasal 4(2)
    General,
    /// Data Pribadi Spesifik (Specific/Sensitive Personal Data) - Pasal 4(3)
    Specific(SpecificDataType),
}

/// Types of specific (sensitive) personal data - Pasal 4(3)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecificDataType {
    /// Data kesehatan (Health data)
    Health,
    /// Data biometrik (Biometric data)
    Biometric,
    /// Data genetika (Genetic data)
    Genetic,
    /// Data catatan kejahatan (Criminal record)
    CriminalRecord,
    /// Data anak (Children's data)
    ChildData,
    /// Data keuangan pribadi (Personal financial data)
    Financial,
    /// Agama/kepercayaan (Religion/belief)
    Religion,
    /// Pandangan politik (Political opinion)
    PoliticalOpinion,
    /// Kehidupan/orientasi seksual (Sexual life/orientation)
    SexualOrientation,
    /// Keanggotaan serikat pekerja (Trade union membership)
    TradeUnionMembership,
    /// Data lain sesuai ketentuan peraturan (Other as regulated)
    Other(String),
}

/// Legal basis for processing personal data - Pasal 20
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Persetujuan eksplisit (Explicit consent) - Pasal 20(2)(a)
    Consent,
    /// Pelaksanaan perjanjian (Contract performance) - Pasal 20(2)(b)
    ContractPerformance,
    /// Kewajiban hukum (Legal obligation) - Pasal 20(2)(c)
    LegalObligation,
    /// Kepentingan vital (Vital interests) - Pasal 20(2)(d)
    VitalInterests,
    /// Kepentingan umum (Public interest) - Pasal 20(2)(e)
    PublicInterest,
    /// Kepentingan sah (Legitimate interests) - Pasal 20(2)(f)
    LegitimateInterests,
}

impl LegalBasis {
    /// Get description in Indonesian
    pub fn description_id(&self) -> &'static str {
        match self {
            Self::Consent => "Persetujuan eksplisit dari Subjek Data Pribadi",
            Self::ContractPerformance => "Pelaksanaan perjanjian dengan Subjek Data Pribadi",
            Self::LegalObligation => "Pemenuhan kewajiban hukum Pengendali Data Pribadi",
            Self::VitalInterests => "Pelindungan kepentingan vital Subjek Data Pribadi",
            Self::PublicInterest => {
                "Pelaksanaan tugas dalam kepentingan umum atau kewenangan publik"
            }
            Self::LegitimateInterests => "Pemenuhan kepentingan sah Pengendali Data Pribadi",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Consent => "Explicit consent from the Data Subject",
            Self::ContractPerformance => "Performance of a contract with the Data Subject",
            Self::LegalObligation => "Compliance with legal obligation of the Data Controller",
            Self::VitalInterests => "Protection of vital interests of the Data Subject",
            Self::PublicInterest => "Performance of task in public interest or public authority",
            Self::LegitimateInterests => "Legitimate interests of the Data Controller",
        }
    }

    /// Get statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::Consent => "UU PDP Pasal 20 ayat (2) huruf a",
            Self::ContractPerformance => "UU PDP Pasal 20 ayat (2) huruf b",
            Self::LegalObligation => "UU PDP Pasal 20 ayat (2) huruf c",
            Self::VitalInterests => "UU PDP Pasal 20 ayat (2) huruf d",
            Self::PublicInterest => "UU PDP Pasal 20 ayat (2) huruf e",
            Self::LegitimateInterests => "UU PDP Pasal 20 ayat (2) huruf f",
        }
    }

    /// Check if this basis requires explicit consent
    pub fn requires_explicit_consent(&self) -> bool {
        matches!(self, Self::Consent)
    }
}

/// Data subject rights under UU PDP - Pasal 5-13
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSubjectRight {
    /// Hak mendapat informasi (Right to information) - Pasal 5-6
    Information,
    /// Hak akses (Right of access) - Pasal 7
    Access,
    /// Hak pembetulan (Right to rectification) - Pasal 8
    Rectification,
    /// Hak penghapusan (Right to erasure) - Pasal 9
    Erasure,
    /// Hak pembatasan pemrosesan (Right to restrict processing) - Pasal 10
    RestrictionOfProcessing,
    /// Hak portabilitas data (Right to data portability) - Pasal 11
    DataPortability,
    /// Hak keberatan (Right to object) - Pasal 12
    Objection,
}

impl DataSubjectRight {
    /// Get the name in Indonesian
    pub fn name_id(&self) -> &'static str {
        match self {
            Self::Information => "Hak mendapat informasi",
            Self::Access => "Hak akses",
            Self::Rectification => "Hak pembetulan",
            Self::Erasure => "Hak penghapusan",
            Self::RestrictionOfProcessing => "Hak pembatasan pemrosesan",
            Self::DataPortability => "Hak portabilitas data",
            Self::Objection => "Hak keberatan",
        }
    }

    /// Get the name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Information => "Right to information",
            Self::Access => "Right of access",
            Self::Rectification => "Right to rectification",
            Self::Erasure => "Right to erasure",
            Self::RestrictionOfProcessing => "Right to restrict processing",
            Self::DataPortability => "Right to data portability",
            Self::Objection => "Right to object",
        }
    }

    /// Get statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::Information => "UU PDP Pasal 5-6",
            Self::Access => "UU PDP Pasal 7",
            Self::Rectification => "UU PDP Pasal 8",
            Self::Erasure => "UU PDP Pasal 9",
            Self::RestrictionOfProcessing => "UU PDP Pasal 10",
            Self::DataPortability => "UU PDP Pasal 11",
            Self::Objection => "UU PDP Pasal 12",
        }
    }

    /// Get response deadline in working days
    pub fn response_deadline_days(&self) -> u32 {
        // UU PDP Pasal 13 - generally 3x24 hours for acknowledgment,
        // 14 days for substantive response
        14
    }
}

/// Purpose of personal data processing
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    /// Service delivery (penyediaan layanan)
    ServiceDelivery,
    /// Marketing and promotion (pemasaran dan promosi)
    Marketing,
    /// Human resources (sumber daya manusia)
    HumanResources,
    /// Legal compliance (kepatuhan hukum)
    LegalCompliance,
    /// Research and statistics (penelitian dan statistik)
    Research,
    /// Security and fraud prevention (keamanan dan pencegahan penipuan)
    Security,
    /// Profiling and automated decision making (pembuatan profil)
    Profiling,
    /// Custom purpose
    Custom(String),
}

/// Consent record for data processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Unique identifier
    pub id: String,
    /// Data subject identifier
    pub data_subject_id: String,
    /// Data controller identifier
    pub data_controller_id: String,
    /// Purposes consented to
    pub purposes: Vec<ProcessingPurpose>,
    /// Data categories consented
    pub data_categories: Vec<DataCategory>,
    /// Consent given date
    pub consent_date: DateTime<Utc>,
    /// Consent expiry date (if applicable)
    pub expiry_date: Option<DateTime<Utc>>,
    /// Whether consent was explicit (Pasal 20)
    pub is_explicit: bool,
    /// Whether consent is for specific data (Pasal 25)
    pub is_for_specific_data: bool,
    /// Method of obtaining consent
    pub consent_method: ConsentMethod,
    /// Whether consent can be withdrawn
    pub is_withdrawable: bool,
    /// Language of consent (Bahasa Indonesia required)
    pub language: String,
    /// Withdrawal date if withdrawn
    pub withdrawal_date: Option<DateTime<Utc>>,
}

/// Method of obtaining consent
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentMethod {
    /// Written consent (tertulis)
    Written,
    /// Electronic consent (elektronik)
    Electronic,
    /// Verbal consent (lisan) - recorded
    Verbal,
    /// Implied consent (where permitted)
    Implied,
}

/// Personal data record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalData {
    /// Category of data
    pub category: DataCategory,
    /// Type of data (e.g., "nama", "email", "NIK")
    pub data_type: String,
    /// Purpose of collection
    pub purpose: ProcessingPurpose,
    /// Legal basis for processing
    pub legal_basis: LegalBasis,
    /// Retention period in days
    pub retention_days: Option<u32>,
    /// Is data encrypted
    pub is_encrypted: bool,
    /// Is data pseudonymized
    pub is_pseudonymized: bool,
}

/// Personal data processing activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDataProcessing {
    /// Data controller name (Pengendali Data Pribadi)
    pub controller_name: String,
    /// Data processor name if applicable (Prosesor Data Pribadi)
    pub processor_name: Option<String>,
    /// Processing purposes
    pub purposes: Vec<ProcessingPurpose>,
    /// Categories of data processed
    pub data_categories: Vec<DataCategory>,
    /// Legal basis for each processing activity
    pub legal_bases: Vec<LegalBasis>,
    /// Whether processing involves cross-border transfer
    pub involves_cross_border: bool,
    /// Destination countries for cross-border transfer
    pub transfer_destinations: Vec<String>,
    /// Data retention period in days
    pub retention_period_days: u32,
    /// Whether automated decision making is used
    pub uses_automated_decisions: bool,
    /// Whether profiling is performed
    pub uses_profiling: bool,
    /// Whether Data Protection Impact Assessment is required
    pub requires_dpia: bool,
    /// Whether DPO is appointed
    pub has_dpo: bool,
}

impl PersonalDataProcessing {
    /// Check if DPIA is required based on processing characteristics
    pub fn should_require_dpia(&self) -> bool {
        // DPIA required for high-risk processing (Pasal 34)
        self.data_categories
            .iter()
            .any(|c| matches!(c, DataCategory::Specific(_)))
            || self.uses_automated_decisions
            || self.uses_profiling
            || self.involves_cross_border
    }
}

/// Security incident/data breach record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    /// Incident identifier
    pub id: String,
    /// Time incident was discovered
    pub discovery_time: DateTime<Utc>,
    /// Time incident actually occurred (if known)
    pub incident_time: Option<DateTime<Utc>>,
    /// Categories of data affected
    pub affected_data_categories: Vec<DataCategory>,
    /// Number of data subjects affected
    pub affected_subjects_count: u64,
    /// Description of the incident
    pub description: String,
    /// Whether incident was notified to authority
    pub notified_authority: bool,
    /// Authority notification time
    pub authority_notification_time: Option<DateTime<Utc>>,
    /// Whether data subjects were notified
    pub notified_subjects: bool,
    /// Subject notification time
    pub subject_notification_time: Option<DateTime<Utc>>,
    /// Remedial actions taken
    pub remedial_actions: Vec<String>,
    /// Risk level assessment
    pub risk_level: RiskLevel,
}

/// Risk level for security incidents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Risiko rendah (Low risk)
    Low,
    /// Risiko sedang (Medium risk)
    Medium,
    /// Risiko tinggi (High risk)
    High,
    /// Risiko kritis (Critical risk)
    Critical,
}

impl SecurityIncident {
    /// Calculate hours since discovery
    pub fn hours_since_discovery(&self) -> i64 {
        let now = Utc::now();
        (now - self.discovery_time).num_hours()
    }

    /// Check if notification deadline exceeded (3x24 hours = 72 hours)
    pub fn is_notification_overdue(&self) -> bool {
        self.hours_since_discovery() > 72 && !self.notified_authority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_basis_descriptions() {
        let consent = LegalBasis::Consent;
        assert!(consent.description_id().contains("Persetujuan"));
        assert!(consent.description_en().contains("consent"));
        assert!(consent.requires_explicit_consent());
    }

    #[test]
    fn test_data_subject_rights() {
        let access = DataSubjectRight::Access;
        assert_eq!(access.name_id(), "Hak akses");
        assert_eq!(access.name_en(), "Right of access");
        assert_eq!(access.response_deadline_days(), 14);
    }

    #[test]
    fn test_security_incident_deadline() {
        use chrono::Duration;

        let now = Utc::now();
        let incident = SecurityIncident {
            id: "INC-001".to_string(),
            discovery_time: now - Duration::hours(80),
            incident_time: None,
            affected_data_categories: vec![DataCategory::General],
            affected_subjects_count: 1000,
            description: "Data breach".to_string(),
            notified_authority: false,
            authority_notification_time: None,
            notified_subjects: false,
            subject_notification_time: None,
            remedial_actions: vec![],
            risk_level: RiskLevel::High,
        };

        assert!(incident.is_notification_overdue());
    }

    #[test]
    fn test_processing_dpia_requirement() {
        let processing = PersonalDataProcessing {
            controller_name: "PT Example".to_string(),
            processor_name: None,
            purposes: vec![ProcessingPurpose::ServiceDelivery],
            data_categories: vec![DataCategory::Specific(SpecificDataType::Health)],
            legal_bases: vec![LegalBasis::Consent],
            involves_cross_border: false,
            transfer_destinations: vec![],
            retention_period_days: 365,
            uses_automated_decisions: false,
            uses_profiling: false,
            requires_dpia: false,
            has_dpo: false,
        };

        assert!(processing.should_require_dpia());
    }
}
