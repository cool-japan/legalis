//! Error types for French Intellectual Property Law
//!
//! This module defines comprehensive error types with bilingual (French/English)
//! error messages for intellectual property law violations and validation failures.

use std::fmt;

/// Result type for IP law operations
pub type IPLawResult<T> = Result<T, IPLawError>;

/// Comprehensive error type for French IP law violations
#[derive(Debug, Clone, PartialEq)]
pub enum IPLawError {
    /// Patent-related errors
    PatentError(PatentErrorKind),
    /// Copyright-related errors
    CopyrightError(CopyrightErrorKind),
    /// Trademark-related errors
    TrademarkError(TrademarkErrorKind),
    /// Design-related errors
    DesignError(DesignErrorKind),
    /// Validation errors
    ValidationError(String),
    /// Builder pattern errors
    BuilderError(String),
}

/// Patent-specific error kinds
#[derive(Debug, Clone, PartialEq)]
pub enum PatentErrorKind {
    /// Patent lacks novelty (Article L611-10)
    LackOfNovelty,
    /// Patent lacks inventive step (Article L611-10)
    LackOfInventiveStep,
    /// Patent lacks industrial applicability (Article L611-10)
    LackOfIndustrialApplicability,
    /// Patent has expired (20 years, Article L611-11)
    PatentExpired,
    /// Invalid filing date
    InvalidFilingDate,
    /// Invalid grant date
    InvalidGrantDate,
    /// Missing required field
    MissingField(String),
}

/// Copyright-specific error kinds
#[derive(Debug, Clone, PartialEq)]
pub enum CopyrightErrorKind {
    /// Copyright has expired (70 years post-mortem, Article L123-1)
    CopyrightExpired,
    /// Work lacks originality
    LackOfOriginality,
    /// Invalid creation date
    InvalidCreationDate,
    /// Invalid author death date
    InvalidDeathDate,
    /// Missing required field
    MissingField(String),
}

/// Trademark-specific error kinds
#[derive(Debug, Clone, PartialEq)]
pub enum TrademarkErrorKind {
    /// Trademark lacks distinctiveness (Article L711-1)
    LackOfDistinctiveness,
    /// Trademark has expired (10 years, Article L712-1)
    TrademarkExpired,
    /// Invalid registration date
    InvalidRegistrationDate,
    /// Invalid Nice classes
    InvalidClasses,
    /// Missing required field
    MissingField(String),
}

/// Design-specific error kinds
#[derive(Debug, Clone, PartialEq)]
pub enum DesignErrorKind {
    /// Design lacks novelty (Article L511-1)
    LackOfNovelty,
    /// Design lacks individual character (Article L511-1)
    LackOfIndividualCharacter,
    /// Design protection expired (25 years max, Article L513-1)
    DesignExpired,
    /// Invalid filing date
    InvalidFilingDate,
    /// Missing required field
    MissingField(String),
}

impl IPLawError {
    /// Get French error message
    pub fn message_fr(&self) -> String {
        match self {
            IPLawError::PatentError(kind) => kind.message_fr(),
            IPLawError::CopyrightError(kind) => kind.message_fr(),
            IPLawError::TrademarkError(kind) => kind.message_fr(),
            IPLawError::DesignError(kind) => kind.message_fr(),
            IPLawError::ValidationError(msg) => format!("Erreur de validation: {}", msg),
            IPLawError::BuilderError(msg) => format!("Erreur de construction: {}", msg),
        }
    }

    /// Get English error message
    pub fn message_en(&self) -> String {
        match self {
            IPLawError::PatentError(kind) => kind.message_en(),
            IPLawError::CopyrightError(kind) => kind.message_en(),
            IPLawError::TrademarkError(kind) => kind.message_en(),
            IPLawError::DesignError(kind) => kind.message_en(),
            IPLawError::ValidationError(msg) => format!("Validation error: {}", msg),
            IPLawError::BuilderError(msg) => format!("Builder error: {}", msg),
        }
    }
}

impl PatentErrorKind {
    /// Get French error message
    pub fn message_fr(&self) -> String {
        match self {
            PatentErrorKind::LackOfNovelty => {
                "Brevet invalide: absence de nouveauté (Article L611-10 CPI)".to_string()
            }
            PatentErrorKind::LackOfInventiveStep => {
                "Brevet invalide: absence d'activité inventive (Article L611-10 CPI)".to_string()
            }
            PatentErrorKind::LackOfIndustrialApplicability => {
                "Brevet invalide: absence d'application industrielle (Article L611-10 CPI)"
                    .to_string()
            }
            PatentErrorKind::PatentExpired => {
                "Brevet expiré: durée maximale de 20 ans dépassée (Article L611-11 CPI)".to_string()
            }
            PatentErrorKind::InvalidFilingDate => "Date de dépôt invalide".to_string(),
            PatentErrorKind::InvalidGrantDate => "Date de délivrance invalide".to_string(),
            PatentErrorKind::MissingField(field) => {
                format!("Champ obligatoire manquant: {}", field)
            }
        }
    }

    /// Get English error message
    pub fn message_en(&self) -> String {
        match self {
            PatentErrorKind::LackOfNovelty => {
                "Invalid patent: lack of novelty (Article L611-10 CPI)".to_string()
            }
            PatentErrorKind::LackOfInventiveStep => {
                "Invalid patent: lack of inventive step (Article L611-10 CPI)".to_string()
            }
            PatentErrorKind::LackOfIndustrialApplicability => {
                "Invalid patent: lack of industrial applicability (Article L611-10 CPI)".to_string()
            }
            PatentErrorKind::PatentExpired => {
                "Patent expired: maximum duration of 20 years exceeded (Article L611-11 CPI)"
                    .to_string()
            }
            PatentErrorKind::InvalidFilingDate => "Invalid filing date".to_string(),
            PatentErrorKind::InvalidGrantDate => "Invalid grant date".to_string(),
            PatentErrorKind::MissingField(field) => format!("Missing required field: {}", field),
        }
    }
}

impl CopyrightErrorKind {
    /// Get French error message
    pub fn message_fr(&self) -> String {
        match self {
            CopyrightErrorKind::CopyrightExpired => {
                "Droit d'auteur expiré: durée maximale de 70 ans après décès dépassée (Article L123-1 CPI)"
                    .to_string()
            }
            CopyrightErrorKind::LackOfOriginality => {
                "Oeuvre invalide: absence d'originalité (Article L111-1 CPI)".to_string()
            }
            CopyrightErrorKind::InvalidCreationDate => "Date de création invalide".to_string(),
            CopyrightErrorKind::InvalidDeathDate => "Date de décès invalide".to_string(),
            CopyrightErrorKind::MissingField(field) => {
                format!("Champ obligatoire manquant: {}", field)
            }
        }
    }

    /// Get English error message
    pub fn message_en(&self) -> String {
        match self {
            CopyrightErrorKind::CopyrightExpired => {
                "Copyright expired: maximum duration of 70 years post-mortem exceeded (Article L123-1 CPI)"
                    .to_string()
            }
            CopyrightErrorKind::LackOfOriginality => {
                "Invalid work: lack of originality (Article L111-1 CPI)".to_string()
            }
            CopyrightErrorKind::InvalidCreationDate => "Invalid creation date".to_string(),
            CopyrightErrorKind::InvalidDeathDate => "Invalid death date".to_string(),
            CopyrightErrorKind::MissingField(field) => format!("Missing required field: {}", field),
        }
    }
}

impl TrademarkErrorKind {
    /// Get French error message
    pub fn message_fr(&self) -> String {
        match self {
            TrademarkErrorKind::LackOfDistinctiveness => {
                "Marque invalide: absence de caractère distinctif (Article L711-1 CPI)".to_string()
            }
            TrademarkErrorKind::TrademarkExpired => {
                "Marque expirée: durée de 10 ans dépassée (Article L712-1 CPI)".to_string()
            }
            TrademarkErrorKind::InvalidRegistrationDate => {
                "Date d'enregistrement invalide".to_string()
            }
            TrademarkErrorKind::InvalidClasses => "Classes de Nice invalides".to_string(),
            TrademarkErrorKind::MissingField(field) => {
                format!("Champ obligatoire manquant: {}", field)
            }
        }
    }

    /// Get English error message
    pub fn message_en(&self) -> String {
        match self {
            TrademarkErrorKind::LackOfDistinctiveness => {
                "Invalid trademark: lack of distinctiveness (Article L711-1 CPI)".to_string()
            }
            TrademarkErrorKind::TrademarkExpired => {
                "Trademark expired: 10-year duration exceeded (Article L712-1 CPI)".to_string()
            }
            TrademarkErrorKind::InvalidRegistrationDate => "Invalid registration date".to_string(),
            TrademarkErrorKind::InvalidClasses => "Invalid Nice classes".to_string(),
            TrademarkErrorKind::MissingField(field) => format!("Missing required field: {}", field),
        }
    }
}

impl DesignErrorKind {
    /// Get French error message
    pub fn message_fr(&self) -> String {
        match self {
            DesignErrorKind::LackOfNovelty => {
                "Dessin ou modèle invalide: absence de nouveauté (Article L511-1 CPI)".to_string()
            }
            DesignErrorKind::LackOfIndividualCharacter => {
                "Dessin ou modèle invalide: absence de caractère propre (Article L511-1 CPI)"
                    .to_string()
            }
            DesignErrorKind::DesignExpired => {
                "Dessin ou modèle expiré: durée maximale de 25 ans dépassée (Article L513-1 CPI)"
                    .to_string()
            }
            DesignErrorKind::InvalidFilingDate => "Date de dépôt invalide".to_string(),
            DesignErrorKind::MissingField(field) => {
                format!("Champ obligatoire manquant: {}", field)
            }
        }
    }

    /// Get English error message
    pub fn message_en(&self) -> String {
        match self {
            DesignErrorKind::LackOfNovelty => {
                "Invalid design: lack of novelty (Article L511-1 CPI)".to_string()
            }
            DesignErrorKind::LackOfIndividualCharacter => {
                "Invalid design: lack of individual character (Article L511-1 CPI)".to_string()
            }
            DesignErrorKind::DesignExpired => {
                "Design expired: maximum duration of 25 years exceeded (Article L513-1 CPI)"
                    .to_string()
            }
            DesignErrorKind::InvalidFilingDate => "Invalid filing date".to_string(),
            DesignErrorKind::MissingField(field) => format!("Missing required field: {}", field),
        }
    }
}

impl fmt::Display for IPLawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.message_fr(), self.message_en())
    }
}

impl std::error::Error for IPLawError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_error_bilingual() {
        let error = IPLawError::PatentError(PatentErrorKind::LackOfNovelty);
        assert!(error.message_fr().contains("nouveauté"));
        assert!(error.message_en().contains("novelty"));
        assert!(error.message_fr().contains("L611-10"));
    }

    #[test]
    fn test_copyright_error_bilingual() {
        let error = IPLawError::CopyrightError(CopyrightErrorKind::CopyrightExpired);
        assert!(error.message_fr().contains("70 ans"));
        assert!(error.message_en().contains("70 years"));
        assert!(error.message_fr().contains("L123-1"));
    }

    #[test]
    fn test_trademark_error_bilingual() {
        let error = IPLawError::TrademarkError(TrademarkErrorKind::LackOfDistinctiveness);
        assert!(error.message_fr().contains("distinctif"));
        assert!(error.message_en().contains("distinctiveness"));
        assert!(error.message_fr().contains("L711-1"));
    }

    #[test]
    fn test_design_error_bilingual() {
        let error = IPLawError::DesignError(DesignErrorKind::LackOfIndividualCharacter);
        assert!(error.message_fr().contains("caractère propre"));
        assert!(error.message_en().contains("individual character"));
        assert!(error.message_fr().contains("L511-1"));
    }

    #[test]
    fn test_display_format() {
        let error = IPLawError::PatentError(PatentErrorKind::PatentExpired);
        let display = format!("{}", error);
        assert!(display.contains("Brevet expiré"));
        assert!(display.contains("Patent expired"));
        assert!(display.contains(" / "));
    }

    #[test]
    fn test_missing_field_errors() {
        let error = IPLawError::PatentError(PatentErrorKind::MissingField("title".to_string()));
        assert!(error.message_fr().contains("title"));
        assert!(error.message_en().contains("title"));
    }
}
