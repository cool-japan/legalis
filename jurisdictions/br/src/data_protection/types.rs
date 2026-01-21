//! LGPD Data Protection Types

use crate::citation::{RomanNumeral, format_lgpd_citation};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Legal bases for processing personal data (Art. 7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Consent of data subject (Art. 7, I)
    Consent,
    /// Legal or regulatory obligation (Art. 7, II)
    LegalObligation,
    /// Public administration (Art. 7, III)
    PublicAdministration,
    /// Research (anonymized) (Art. 7, IV)
    Research,
    /// Contract execution (Art. 7, V)
    ContractExecution,
    /// Legal proceedings (Art. 7, VI)
    LegalProceedings,
    /// Life/safety protection (Art. 7, VII)
    VitalInterests,
    /// Health protection by professionals (Art. 7, VIII)
    HealthProtection,
    /// Legitimate interest (Art. 7, IX)
    LegitimateInterest,
    /// Credit protection (Art. 7, X)
    CreditProtection,
}

impl LegalBasis {
    /// Get legal citation
    pub fn citation(&self) -> String {
        let inciso = match self {
            Self::Consent => RomanNumeral::I,
            Self::LegalObligation => RomanNumeral::II,
            Self::PublicAdministration => RomanNumeral::III,
            Self::Research => RomanNumeral::IV,
            Self::ContractExecution => RomanNumeral::V,
            Self::LegalProceedings => RomanNumeral::VI,
            Self::VitalInterests => RomanNumeral::VII,
            Self::HealthProtection => RomanNumeral::VIII,
            Self::LegitimateInterest => RomanNumeral::IX,
            Self::CreditProtection => RomanNumeral::X,
        };
        format_lgpd_citation(7, None, Some(inciso))
    }

    /// Get description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::Consent => "Consentimento do titular",
            Self::LegalObligation => "Cumprimento de obrigação legal ou regulatória",
            Self::PublicAdministration => "Execução de políticas públicas",
            Self::Research => "Realização de estudos por órgão de pesquisa",
            Self::ContractExecution => "Execução de contrato",
            Self::LegalProceedings => "Exercício de direitos em processo",
            Self::VitalInterests => "Proteção da vida ou incolumidade física",
            Self::HealthProtection => "Tutela da saúde por profissionais de saúde",
            Self::LegitimateInterest => "Interesse legítimo do controlador",
            Self::CreditProtection => "Proteção do crédito",
        }
    }

    /// Check if requires specific documentation
    pub fn requires_documentation(&self) -> bool {
        matches!(
            self,
            Self::Consent | Self::LegitimateInterest | Self::ContractExecution
        )
    }
}

/// Data subject rights (Art. 18)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSubjectRight {
    /// Confirmation of processing (Art. 18, I)
    ConfirmationOfProcessing,
    /// Access to data (Art. 18, II)
    Access,
    /// Correction of inaccurate data (Art. 18, III)
    Correction,
    /// Anonymization/blocking/deletion of unnecessary data (Art. 18, IV)
    AnonymizationBlockingDeletion,
    /// Portability (Art. 18, V)
    Portability,
    /// Deletion with consent (Art. 18, VI)
    Deletion,
    /// Information about sharing (Art. 18, VII)
    SharingInformation,
    /// Refusal information (Art. 18, VIII)
    RefusalInformation,
    /// Consent revocation (Art. 18, IX)
    ConsentRevocation,
}

impl DataSubjectRight {
    /// Get legal citation
    pub fn citation(&self) -> String {
        let inciso = match self {
            Self::ConfirmationOfProcessing => RomanNumeral::I,
            Self::Access => RomanNumeral::II,
            Self::Correction => RomanNumeral::III,
            Self::AnonymizationBlockingDeletion => RomanNumeral::IV,
            Self::Portability => RomanNumeral::V,
            Self::Deletion => RomanNumeral::VI,
            Self::SharingInformation => RomanNumeral::VII,
            Self::RefusalInformation => RomanNumeral::VIII,
            Self::ConsentRevocation => RomanNumeral::IX,
        };
        format_lgpd_citation(18, None, Some(inciso))
    }

    /// Get description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::ConfirmationOfProcessing => "Confirmação da existência de tratamento",
            Self::Access => "Acesso aos dados",
            Self::Correction => "Correção de dados incompletos ou inexatos",
            Self::AnonymizationBlockingDeletion => "Anonimização, bloqueio ou eliminação",
            Self::Portability => "Portabilidade dos dados",
            Self::Deletion => "Eliminação dos dados com consentimento",
            Self::SharingInformation => "Informação sobre compartilhamento",
            Self::RefusalInformation => "Informação sobre não fornecimento de consentimento",
            Self::ConsentRevocation => "Revogação do consentimento",
        }
    }

    /// Get response deadline in days (Art. 18, §4)
    pub fn response_deadline_days(&self) -> u32 {
        match self {
            Self::ConfirmationOfProcessing => 15, // Simplified format
            Self::Access => 15,
            _ => 15, // Default ANPD guideline
        }
    }
}

/// Processing purpose
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessingPurpose {
    /// Purpose description
    pub descricao_pt: String,
    /// English description
    pub description_en: String,
    /// Legal basis
    pub legal_basis: LegalBasis,
    /// Whether purpose is specific
    pub is_specific: bool,
}

impl ProcessingPurpose {
    /// Create a new processing purpose
    pub fn new(
        descricao_pt: impl Into<String>,
        description_en: impl Into<String>,
        legal_basis: LegalBasis,
    ) -> Self {
        Self {
            descricao_pt: descricao_pt.into(),
            description_en: description_en.into(),
            legal_basis,
            is_specific: true,
        }
    }
}

/// Personal data processing activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDataProcessing {
    /// Processing ID
    pub id: String,
    /// Data categories
    pub data_categories: Vec<DataCategory>,
    /// Processing purposes
    pub purposes: Vec<ProcessingPurpose>,
    /// Data subjects affected
    pub data_subjects: Vec<String>,
    /// Retention period in months
    pub retention_months: Option<u32>,
    /// International transfer
    pub international_transfer: bool,
    /// Transfer destination countries
    pub transfer_countries: Vec<String>,
    /// Sensitive data involved
    pub has_sensitive_data: bool,
    /// Automated decision-making
    pub automated_decision: bool,
}

/// Data category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCategory {
    /// Identification data
    Identification,
    /// Contact data
    Contact,
    /// Financial data
    Financial,
    /// Location data
    Location,
    /// Health data (sensitive)
    Health,
    /// Biometric data (sensitive)
    Biometric,
    /// Genetic data (sensitive)
    Genetic,
    /// Religious belief (sensitive)
    Religious,
    /// Political opinion (sensitive)
    Political,
    /// Union membership (sensitive)
    UnionMembership,
    /// Sexual orientation (sensitive)
    SexualOrientation,
    /// Racial/ethnic origin (sensitive)
    RacialEthnic,
    /// Criminal data
    Criminal,
    /// Children's data
    ChildrensData,
}

impl DataCategory {
    /// Check if category is sensitive (Art. 11)
    pub fn is_sensitive(&self) -> bool {
        matches!(
            self,
            Self::Health
                | Self::Biometric
                | Self::Genetic
                | Self::Religious
                | Self::Political
                | Self::UnionMembership
                | Self::SexualOrientation
                | Self::RacialEthnic
        )
    }

    /// Get name in Portuguese
    pub fn nome_pt(&self) -> &'static str {
        match self {
            Self::Identification => "Dados de identificação",
            Self::Contact => "Dados de contato",
            Self::Financial => "Dados financeiros",
            Self::Location => "Dados de localização",
            Self::Health => "Dados de saúde",
            Self::Biometric => "Dados biométricos",
            Self::Genetic => "Dados genéticos",
            Self::Religious => "Convicção religiosa",
            Self::Political => "Opinião política",
            Self::UnionMembership => "Filiação sindical",
            Self::SexualOrientation => "Orientação sexual",
            Self::RacialEthnic => "Origem racial/étnica",
            Self::Criminal => "Dados criminais",
            Self::ChildrensData => "Dados de menores",
        }
    }
}

/// Security incident (Art. 48)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityIncident {
    /// Incident date
    pub data_incidente: DateTime<Utc>,
    /// Detection date
    pub data_deteccao: DateTime<Utc>,
    /// Incident type
    pub incident_type: IncidentType,
    /// Affected data subjects count
    pub affected_count: u64,
    /// Data categories affected
    pub data_affected: Vec<DataCategory>,
    /// Risk level
    pub risk_level: RiskLevel,
    /// ANPD notified
    pub anpd_notified: bool,
    /// Notification date
    pub notification_date: Option<DateTime<Utc>>,
    /// Data subjects notified
    pub subjects_notified: bool,
}

/// Incident type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentType {
    /// Unauthorized access
    UnauthorizedAccess,
    /// Data breach
    DataBreach,
    /// Accidental disclosure
    AccidentalDisclosure,
    /// Data loss
    DataLoss,
    /// Ransomware attack
    Ransomware,
    /// Phishing
    Phishing,
    /// Other
    Other,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

impl SecurityIncident {
    /// Check if ANPD notification required (reasonable time)
    pub fn requires_anpd_notification(&self) -> bool {
        self.risk_level >= RiskLevel::Medium || self.has_sensitive_data()
    }

    /// Check if data subjects must be notified
    pub fn requires_subject_notification(&self) -> bool {
        self.risk_level >= RiskLevel::High
    }

    /// Check if sensitive data involved
    pub fn has_sensitive_data(&self) -> bool {
        self.data_affected.iter().any(|c| c.is_sensitive())
    }
}

/// LGPD compliance status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LgpdCompliance {
    /// Overall compliance
    pub compliant: bool,
    /// Has DPO appointed
    pub has_dpo: bool,
    /// Has privacy policy
    pub has_privacy_policy: bool,
    /// Has data mapping
    pub has_data_mapping: bool,
    /// Has consent mechanisms
    pub has_consent_mechanism: bool,
    /// Has data subject request process
    pub has_dsr_process: bool,
    /// Has incident response plan
    pub has_incident_plan: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Consent record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Data subject identifier
    pub titular_id: String,
    /// Consent date
    pub data_consentimento: NaiveDate,
    /// Purpose consented to
    pub finalidade: String,
    /// Consent text shown
    pub texto_consentimento: String,
    /// Consent method (click, signature, etc.)
    pub metodo: String,
    /// Is consent active
    pub ativo: bool,
    /// Revocation date if revoked
    pub data_revogacao: Option<NaiveDate>,
}

impl ConsentRecord {
    /// Check if consent is valid
    pub fn is_valid(&self) -> bool {
        self.ativo && self.data_revogacao.is_none()
    }

    /// Revoke consent
    pub fn revoke(&mut self, date: NaiveDate) {
        self.ativo = false;
        self.data_revogacao = Some(date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_basis_citation() {
        let basis = LegalBasis::Consent;
        let citation = basis.citation();
        assert!(citation.contains("Art. 7"));
        assert!(citation.contains("inciso I"));
    }

    #[test]
    fn test_data_subject_right_citation() {
        let right = DataSubjectRight::Access;
        let citation = right.citation();
        assert!(citation.contains("Art. 18"));
    }

    #[test]
    fn test_data_category_sensitive() {
        assert!(DataCategory::Health.is_sensitive());
        assert!(DataCategory::Biometric.is_sensitive());
        assert!(!DataCategory::Contact.is_sensitive());
        assert!(!DataCategory::Identification.is_sensitive());
    }

    #[test]
    fn test_security_incident_notification() {
        let incident = SecurityIncident {
            data_incidente: Utc::now(),
            data_deteccao: Utc::now(),
            incident_type: IncidentType::DataBreach,
            affected_count: 1000,
            data_affected: vec![DataCategory::Health],
            risk_level: RiskLevel::High,
            anpd_notified: false,
            notification_date: None,
            subjects_notified: false,
        };

        assert!(incident.requires_anpd_notification());
        assert!(incident.requires_subject_notification());
        assert!(incident.has_sensitive_data());
    }

    #[test]
    fn test_consent_record() {
        let mut consent = ConsentRecord {
            titular_id: "123".to_string(),
            data_consentimento: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            finalidade: "Marketing".to_string(),
            texto_consentimento: "Aceito receber emails".to_string(),
            metodo: "Click".to_string(),
            ativo: true,
            data_revogacao: None,
        };

        assert!(consent.is_valid());

        consent.revoke(NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date"));
        assert!(!consent.is_valid());
    }

    #[test]
    fn test_processing_purpose() {
        let purpose = ProcessingPurpose::new(
            "Envio de marketing",
            "Marketing emails",
            LegalBasis::Consent,
        );
        assert!(purpose.is_specific);
        assert!(purpose.legal_basis.requires_documentation());
    }

    #[test]
    fn test_response_deadline() {
        let right = DataSubjectRight::Access;
        assert_eq!(right.response_deadline_days(), 15);
    }
}
