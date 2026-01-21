//! Error types for EU Financial Services regulation compliance

use crate::i18n::MultilingualText;
use thiserror::Error;

/// Errors for MiFID II and PSD2 compliance validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FinancialServicesError {
    /// Missing authorization for investment services
    #[error("Investment firm not authorized to provide service: {service}")]
    MissingAuthorization { service: String },

    /// Best execution policy inadequate
    #[error("Best execution policy inadequate: {reason}")]
    InadequateBestExecution { reason: String },

    /// Client categorization error
    #[error("Client categorization error: {reason}")]
    ClientCategorizationError { reason: String },

    /// Product governance violation
    #[error("Product governance requirements not met (Article 16): {reason}")]
    ProductGovernanceViolation { reason: String },

    /// Inducements not properly disclosed
    #[error("Inducements must be disclosed to client and enhance quality of service")]
    InducementsViolation,

    /// Conflicts of interest not managed
    #[error("Conflicts of interest not adequately managed: {conflicts}")]
    ConflictsNotManaged { conflicts: String },

    /// Suitability assessment missing or inadequate
    #[error("Suitability assessment required for investment advice/portfolio management")]
    SuitabilityAssessmentMissing,

    /// Appropriateness assessment missing
    #[error("Appropriateness assessment required for this service")]
    AppropriatenessAssessmentMissing,

    /// Strong customer authentication not implemented
    #[error("Strong Customer Authentication (SCA) required under PSD2 Article 97")]
    ScaNotImplemented,

    /// SCA elements insufficient
    #[error(
        "SCA requires 2+ elements from different categories (knowledge, possession, inherence)"
    )]
    InsufficientScaElements,

    /// Dynamic linking missing for payment
    #[error("Payment transactions require dynamic linking (RTS on SCA)")]
    MissingDynamicLinking,

    /// Payment initiation provider not authorized
    #[error("Payment initiation provider must be authorized under PSD2")]
    PispNotAuthorized,

    /// Account information provider not authorized
    #[error("Account information provider must be authorized under PSD2")]
    AispNotAuthorized,

    /// User consent missing for account information service
    #[error("Account information service requires explicit user consent")]
    MissingUserConsent,

    /// Open banking API requirements not met
    #[error("Open banking API requirements not met (PSD2 Article 67): {reason}")]
    OpenBankingViolation { reason: String },

    /// Passporting notification not made
    #[error("Passporting requires notification to home state competent authority")]
    PassportingNotificationMissing,

    /// Transaction reporting violation
    #[error("Transaction reporting requirements not met (MiFID II Article 26): {reason}")]
    TransactionReportingViolation { reason: String },

    /// Missing field
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid value
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    /// Multiple violations
    #[error("Multiple financial services violations: {count}")]
    MultipleViolations { count: usize },
}

impl FinancialServicesError {
    /// Create error for missing authorization
    pub fn missing_authorization(service: impl Into<String>) -> Self {
        Self::MissingAuthorization {
            service: service.into(),
        }
    }

    /// Create error for inadequate best execution
    pub fn inadequate_best_execution(reason: impl Into<String>) -> Self {
        Self::InadequateBestExecution {
            reason: reason.into(),
        }
    }

    /// Create error for client categorization
    pub fn client_categorization_error(reason: impl Into<String>) -> Self {
        Self::ClientCategorizationError {
            reason: reason.into(),
        }
    }

    /// Create error for product governance violation
    pub fn product_governance_violation(reason: impl Into<String>) -> Self {
        Self::ProductGovernanceViolation {
            reason: reason.into(),
        }
    }

    /// Create error for conflicts not managed
    pub fn conflicts_not_managed(conflicts: impl Into<String>) -> Self {
        Self::ConflictsNotManaged {
            conflicts: conflicts.into(),
        }
    }

    /// Create error for open banking violation
    pub fn open_banking_violation(reason: impl Into<String>) -> Self {
        Self::OpenBankingViolation {
            reason: reason.into(),
        }
    }

    /// Create error for transaction reporting violation
    pub fn transaction_reporting_violation(reason: impl Into<String>) -> Self {
        Self::TransactionReportingViolation {
            reason: reason.into(),
        }
    }

    /// Create error for missing field
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create error for invalid value
    pub fn invalid_value(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Get localized error message
    pub fn message(&self, lang: &str) -> String {
        let ml_text = self.to_multilingual();
        ml_text.in_language(lang).to_string()
    }

    /// Convert error to multilingual text
    fn to_multilingual(&self) -> MultilingualText {
        match self {
            Self::MissingAuthorization { service } => MultilingualText::new(format!(
                "Investment firm not authorized to provide service: {}",
                service
            ))
            .with_de(format!(
                "Wertpapierfirma nicht zur Erbringung der Dienstleistung autorisiert: {}",
                service
            ))
            .with_fr(format!(
                "L'entreprise d'investissement n'est pas autorisée à fournir le service: {}",
                service
            )),

            Self::InadequateBestExecution { reason } => MultilingualText::new(format!(
                "Best execution policy inadequate: {}",
                reason
            ))
            .with_de(format!("Best-Execution-Politik unzureichend: {}", reason))
            .with_fr(format!(
                "Politique de meilleure exécution inadéquate: {}",
                reason
            )),

            Self::ClientCategorizationError { reason } => MultilingualText::new(format!(
                "Client categorization error: {}",
                reason
            ))
            .with_de(format!("Fehler bei Kundenkategorisierung: {}", reason))
            .with_fr(format!("Erreur de catégorisation du client: {}", reason)),

            Self::ProductGovernanceViolation { reason } => MultilingualText::new(format!(
                "Product governance requirements not met (Article 16): {}",
                reason
            ))
            .with_de(format!(
                "Anforderungen an Product Governance nicht erfüllt (Artikel 16): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de gouvernance des produits non satisfaites (Article 16): {}",
                reason
            )),

            Self::InducementsViolation => MultilingualText::new(
                "Inducements must be disclosed to client and enhance quality of service"
            )
            .with_de("Zuwendungen müssen dem Kunden offengelegt werden und Qualität der Dienstleistung verbessern")
            .with_fr("Les incitations doivent être divulguées au client et améliorer la qualité du service"),

            Self::ConflictsNotManaged { conflicts } => MultilingualText::new(format!(
                "Conflicts of interest not adequately managed: {}",
                conflicts
            ))
            .with_de(format!(
                "Interessenkonflikte nicht angemessen gemanagt: {}",
                conflicts
            ))
            .with_fr(format!(
                "Conflits d'intérêts non gérés de manière adéquate: {}",
                conflicts
            )),

            Self::SuitabilityAssessmentMissing => MultilingualText::new(
                "Suitability assessment required for investment advice/portfolio management"
            )
            .with_de("Eignungsprüfung erforderlich für Anlageberatung/Vermögensverwaltung")
            .with_fr("Évaluation de l'adéquation requise pour le conseil en investissement/gestion de portefeuille"),

            Self::AppropriatenessAssessmentMissing => MultilingualText::new(
                "Appropriateness assessment required for this service"
            )
            .with_de("Angemessenheitsprüfung erforderlich für diese Dienstleistung")
            .with_fr("Évaluation du caractère approprié requise pour ce service"),

            Self::ScaNotImplemented => MultilingualText::new(
                "Strong Customer Authentication (SCA) required under PSD2 Article 97"
            )
            .with_de("Starke Kundenauthentifizierung (SCA) erforderlich gemäß PSD2 Artikel 97")
            .with_fr("Authentification forte du client (SCA) requise en vertu de l'article 97 de la DSP2"),

            Self::InsufficientScaElements => MultilingualText::new(
                "SCA requires 2+ elements from different categories (knowledge, possession, inherence)"
            )
            .with_de("SCA erfordert 2+ Elemente aus verschiedenen Kategorien (Wissen, Besitz, Inhärenz)")
            .with_fr("La SCA nécessite 2+ éléments de catégories différentes (connaissance, possession, inhérence)"),

            Self::MissingDynamicLinking => MultilingualText::new(
                "Payment transactions require dynamic linking (RTS on SCA)"
            )
            .with_de("Zahlungstransaktionen erfordern dynamische Verknüpfung (RTS zu SCA)")
            .with_fr("Les transactions de paiement nécessitent un lien dynamique (RTS sur la SCA)"),

            Self::PispNotAuthorized => MultilingualText::new(
                "Payment initiation provider must be authorized under PSD2"
            )
            .with_de("Zahlungsauslösedienstleister muss gemäß PSD2 autorisiert sein")
            .with_fr("Le prestataire de services d'initiation de paiement doit être autorisé en vertu de la DSP2"),

            Self::AispNotAuthorized => MultilingualText::new(
                "Account information provider must be authorized under PSD2"
            )
            .with_de("Kontoinformationsdienstleister muss gemäß PSD2 autorisiert sein")
            .with_fr("Le prestataire de services d'information sur les comptes doit être autorisé en vertu de la DSP2"),

            Self::MissingUserConsent => MultilingualText::new(
                "Account information service requires explicit user consent"
            )
            .with_de("Kontoinformationsdienst erfordert ausdrückliche Nutzereinwilligung")
            .with_fr("Le service d'information sur les comptes nécessite le consentement explicite de l'utilisateur"),

            Self::OpenBankingViolation { reason } => MultilingualText::new(format!(
                "Open banking API requirements not met (PSD2 Article 67): {}",
                reason
            ))
            .with_de(format!(
                "Open-Banking-API-Anforderungen nicht erfüllt (PSD2 Artikel 67): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de l'API d'open banking non satisfaites (DSP2 Article 67): {}",
                reason
            )),

            Self::PassportingNotificationMissing => MultilingualText::new(
                "Passporting requires notification to home state competent authority"
            )
            .with_de("Passporting erfordert Benachrichtigung der zuständigen Behörde des Herkunftsmitgliedstaats")
            .with_fr("Le passeport européen nécessite une notification à l'autorité compétente de l'État d'origine"),

            Self::TransactionReportingViolation { reason } => MultilingualText::new(format!(
                "Transaction reporting requirements not met (MiFID II Article 26): {}",
                reason
            ))
            .with_de(format!(
                "Anforderungen an Transaktionsmeldungen nicht erfüllt (MiFID II Artikel 26): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de déclaration des transactions non satisfaites (MiFID II Article 26): {}",
                reason
            )),

            Self::MissingField { field } => MultilingualText::new(format!(
                "Missing required field: {}",
                field
            ))
            .with_de(format!("Fehlendes Pflichtfeld: {}", field))
            .with_fr(format!("Champ obligatoire manquant: {}", field)),

            Self::InvalidValue { field, reason } => MultilingualText::new(format!(
                "Invalid value for field '{}': {}",
                field, reason
            ))
            .with_de(format!("Ungültiger Wert für Feld '{}': {}", field, reason))
            .with_fr(format!("Valeur invalide pour le champ '{}': {}", field, reason)),

            Self::MultipleViolations { count } => MultilingualText::new(format!(
                "Multiple financial services violations: {}",
                count
            ))
            .with_de(format!("Mehrere Verstöße gegen Finanzdienstleistungsvorschriften: {}", count))
            .with_fr(format!("Violations multiples des services financiers: {}", count)),
        }
    }
}
