//! Error types for French family law.
//!
//! Provides bilingual (French/English) error messages for family law violations.

use thiserror::Error;

/// Bilingual string with French and English versions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BilingualString {
    pub fr: String,
    pub en: String,
}

impl BilingualString {
    /// Create a new bilingual string.
    #[must_use]
    pub fn new(fr: String, en: String) -> Self {
        Self { fr, en }
    }
}

/// Result type for family law operations.
pub type FamilyLawResult<T> = Result<T, FamilyLawError>;

/// Errors that can occur in French family law analysis.
#[derive(Debug, Error)]
pub enum FamilyLawError {
    // Marriage Errors
    #[error("Insufficient age: {actual_age} years (required: {required_age})")]
    InsufficientAge { actual_age: u32, required_age: u32 },

    #[error("No consent from party: {party}")]
    NoConsent { party: String },

    #[error("Bigamy detected: existing marriage from {existing_marriage_date}")]
    Bigamy { existing_marriage_date: String },

    #[error("Proxy marriage prohibited for French nationals")]
    ProxyMarriageProhibited,

    #[error("Banns not published")]
    BannsNotPublished,

    #[error("Banns published too recently: {days_elapsed} days (required: 10)")]
    BannsPublishedTooRecently { days_elapsed: u32 },

    #[error("Marriage opposition filed")]
    MarriageOpposition { grounds: Vec<String> },

    #[error("Consanguinity violation: {relationship}")]
    Consanguinity { relationship: String },

    #[error("Incest prohibition: {relationship}")]
    Incest { relationship: String },

    // Divorce Errors
    #[error(
        "Insufficient separation duration: {months_elapsed} months (required: {required_months})"
    )]
    InsufficientSeparation {
        months_elapsed: u32,
        required_months: u32,
    },

    #[error("Child hearing required but not conducted")]
    ChildHearingRequired,

    #[error("No fault evidence provided")]
    NoFaultEvidence,

    #[error("Agreement not signed by both parties")]
    AgreementNotSigned,

    #[error("Notary filing required for mutual consent divorce")]
    NotaryFilingRequired,

    #[error("Both parties must accept divorce principle")]
    PrincipleNotAccepted,

    // Property Regime Errors
    #[error("Marriage contract required for {regime}")]
    MarriageContractRequired { regime: String },

    #[error("Invalid property classification")]
    InvalidPropertyClassification,

    // Parental Authority Errors
    #[error("Invalid child residence arrangement")]
    InvalidChildResidence,

    #[error("Maintenance obligation not specified")]
    MaintenanceNotSpecified,

    // PACS Errors
    #[error("PACS registration not completed")]
    PacsRegistrationIncomplete,

    #[error("PACS dissolution notice period insufficient: {days_elapsed} days")]
    PacsDissolutionNoticeInsufficient { days_elapsed: u32 },

    // General Errors
    #[error("Multiple errors occurred")]
    MultipleErrors(Vec<FamilyLawError>),

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
}

impl FamilyLawError {
    /// Get bilingual description of the error.
    #[must_use]
    pub fn description(&self) -> BilingualString {
        match self {
            Self::InsufficientAge {
                actual_age,
                required_age,
            } => BilingualString::new(
                format!(
                    "Âge insuffisant : {} ans (requis : {})",
                    actual_age, required_age
                ),
                format!(
                    "Insufficient age: {} years (required: {})",
                    actual_age, required_age
                ),
            ),
            Self::NoConsent { party } => BilingualString::new(
                format!("Consentement manquant de : {}", party),
                format!("Missing consent from: {}", party),
            ),
            Self::Bigamy {
                existing_marriage_date,
            } => BilingualString::new(
                format!(
                    "Bigamie détectée : mariage existant du {}",
                    existing_marriage_date
                ),
                format!(
                    "Bigamy detected: existing marriage from {}",
                    existing_marriage_date
                ),
            ),
            Self::ProxyMarriageProhibited => BilingualString::new(
                "Mariage par procuration interdit pour les nationaux français (Article 146-1)"
                    .to_string(),
                "Proxy marriage prohibited for French nationals (Article 146-1)".to_string(),
            ),
            Self::BannsNotPublished => BilingualString::new(
                "Publications des bans non effectuées (Article 161)".to_string(),
                "Banns not published (Article 161)".to_string(),
            ),
            Self::BannsPublishedTooRecently { days_elapsed } => BilingualString::new(
                format!(
                    "Publications des bans trop récentes : {} jours (requis : 10)",
                    days_elapsed
                ),
                format!(
                    "Banns published too recently: {} days (required: 10)",
                    days_elapsed
                ),
            ),
            Self::MarriageOpposition { grounds } => BilingualString::new(
                format!("Opposition au mariage déposée : {}", grounds.join(", ")),
                format!("Marriage opposition filed: {}", grounds.join(", ")),
            ),
            Self::Consanguinity { relationship } => BilingualString::new(
                format!("Violation de consanguinité : {}", relationship),
                format!("Consanguinity violation: {}", relationship),
            ),
            Self::Incest { relationship } => BilingualString::new(
                format!("Prohibition d'inceste : {}", relationship),
                format!("Incest prohibition: {}", relationship),
            ),
            Self::InsufficientSeparation {
                months_elapsed,
                required_months,
            } => BilingualString::new(
                format!(
                    "Durée de séparation insuffisante : {} mois (requis : {})",
                    months_elapsed, required_months
                ),
                format!(
                    "Insufficient separation duration: {} months (required: {})",
                    months_elapsed, required_months
                ),
            ),
            Self::ChildHearingRequired => BilingualString::new(
                "Audition de l'enfant requise mais non effectuée".to_string(),
                "Child hearing required but not conducted".to_string(),
            ),
            Self::NoFaultEvidence => BilingualString::new(
                "Aucune preuve de faute fournie".to_string(),
                "No fault evidence provided".to_string(),
            ),
            Self::AgreementNotSigned => BilingualString::new(
                "Convention non signée par les deux parties".to_string(),
                "Agreement not signed by both parties".to_string(),
            ),
            Self::NotaryFilingRequired => BilingualString::new(
                "Dépôt chez le notaire requis pour divorce par consentement mutuel".to_string(),
                "Notary filing required for mutual consent divorce".to_string(),
            ),
            Self::PrincipleNotAccepted => BilingualString::new(
                "Les deux parties doivent accepter le principe du divorce".to_string(),
                "Both parties must accept divorce principle".to_string(),
            ),
            Self::MarriageContractRequired { regime } => BilingualString::new(
                format!("Contrat de mariage requis pour : {}", regime),
                format!("Marriage contract required for: {}", regime),
            ),
            Self::InvalidPropertyClassification => BilingualString::new(
                "Classification de propriété invalide".to_string(),
                "Invalid property classification".to_string(),
            ),
            Self::InvalidChildResidence => BilingualString::new(
                "Arrangement de résidence de l'enfant invalide".to_string(),
                "Invalid child residence arrangement".to_string(),
            ),
            Self::MaintenanceNotSpecified => BilingualString::new(
                "Obligation alimentaire non spécifiée".to_string(),
                "Maintenance obligation not specified".to_string(),
            ),
            Self::PacsRegistrationIncomplete => BilingualString::new(
                "Enregistrement du PACS incomplet".to_string(),
                "PACS registration incomplete".to_string(),
            ),
            Self::PacsDissolutionNoticeInsufficient { days_elapsed } => BilingualString::new(
                format!(
                    "Délai de préavis de dissolution insuffisant : {} jours",
                    days_elapsed
                ),
                format!(
                    "PACS dissolution notice period insufficient: {} days",
                    days_elapsed
                ),
            ),
            Self::MultipleErrors(errors) => BilingualString::new(
                format!("{} erreurs détectées", errors.len()),
                format!("{} errors detected", errors.len()),
            ),
            Self::InvalidInput { message } => BilingualString::new(
                format!("Entrée invalide : {}", message),
                format!("Invalid input: {}", message),
            ),
        }
    }

    /// Get the French description.
    #[must_use]
    pub fn description_fr(&self) -> String {
        self.description().fr
    }

    /// Get the English description.
    #[must_use]
    pub fn description_en(&self) -> String {
        self.description().en
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insufficient_age_bilingual() {
        let error = FamilyLawError::InsufficientAge {
            actual_age: 17,
            required_age: 18,
        };
        let desc = error.description();
        assert!(desc.fr.contains("17"));
        assert!(desc.fr.contains("18"));
        assert!(desc.en.contains("17"));
        assert!(desc.en.contains("18"));
    }

    #[test]
    fn test_no_consent_bilingual() {
        let error = FamilyLawError::NoConsent {
            party: "Alice".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("Alice"));
        assert!(desc.en.contains("Alice"));
    }

    #[test]
    fn test_bigamy_bilingual() {
        let error = FamilyLawError::Bigamy {
            existing_marriage_date: "2020-01-15".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("2020-01-15"));
        assert!(desc.en.contains("2020-01-15"));
    }

    #[test]
    fn test_proxy_marriage_prohibited() {
        let error = FamilyLawError::ProxyMarriageProhibited;
        let desc = error.description();
        assert!(desc.fr.contains("procuration"));
        assert!(desc.en.contains("Proxy"));
    }

    #[test]
    fn test_insufficient_separation() {
        let error = FamilyLawError::InsufficientSeparation {
            months_elapsed: 18,
            required_months: 24,
        };
        let desc = error.description();
        assert!(desc.fr.contains("18"));
        assert!(desc.fr.contains("24"));
        assert!(desc.en.contains("18"));
        assert!(desc.en.contains("24"));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            FamilyLawError::NoConsent {
                party: "Party1".to_string(),
            },
            FamilyLawError::BannsNotPublished,
        ];
        let error = FamilyLawError::MultipleErrors(errors);
        let desc = error.description();
        assert!(desc.fr.contains("2"));
        assert!(desc.en.contains("2"));
    }
}
