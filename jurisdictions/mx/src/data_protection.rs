//! Data Protection Law (Ley Federal de Protección de Datos Personales en Posesión de los Particulares - LFPDPPP)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Personal data processing (Tratamiento de datos personales)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDataProcessing {
    /// Data controller (Responsable)
    pub responsable: String,
    /// Data subjects
    pub titulares: Vec<DataSubject>,
    /// Purpose of processing
    pub finalidad: Vec<String>,
    /// Legal basis
    pub base_legal: LegalBasis,
    /// Consent obtained
    pub consentimiento: bool,
}

/// Data subject (Titular)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataSubject {
    /// Name
    pub nombre: String,
    /// Data categories being processed
    pub categorias_datos: Vec<DataCategory>,
}

/// Data categories (Categorías de datos)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCategory {
    /// Identification data
    Identification,
    /// Contact data
    Contact,
    /// Work data
    Work,
    /// Academic data
    Academic,
    /// Financial data
    Financial,
    /// Health data (sensitive)
    Health,
    /// Biometric data (sensitive)
    Biometric,
    /// Ideological data (sensitive)
    Ideological,
}

/// Legal basis for processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Consent (Consentimiento)
    Consent,
    /// Contract execution (Ejecución de contrato)
    Contract,
    /// Legal obligation (Obligación legal)
    LegalObligation,
    /// Vital interest (Interés vital)
    VitalInterest,
    /// Public interest (Interés público)
    PublicInterest,
}

/// Data subject rights (Derechos ARCO)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ARCORight {
    /// Access (Acceso)
    Access,
    /// Rectification (Rectificación)
    Rectification,
    /// Cancellation (Cancelación)
    Cancellation,
    /// Opposition (Oposición)
    Opposition,
}

/// Privacy notice (Aviso de privacidad)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivacyNotice {
    /// Controller identity
    pub identidad_responsable: String,
    /// Purpose of processing
    pub finalidades: Vec<String>,
    /// Data to be collected
    pub datos_recabados: Vec<DataCategory>,
    /// Data transfers
    pub transferencias: Vec<String>,
    /// Means to exercise ARCO rights
    pub medios_arco: String,
}

/// Data protection errors
#[derive(Debug, Error)]
pub enum DataProtectionError {
    #[error("Missing consent for processing")]
    MissingConsent,
    #[error("Invalid legal basis: {0}")]
    InvalidLegalBasis(String),
    #[error("Privacy notice violation: {0}")]
    PrivacyNoticeViolation(String),
}

impl PersonalDataProcessing {
    /// Validate data processing
    pub fn validate(&self) -> Result<(), DataProtectionError> {
        // Sensitive data requires explicit consent
        for titular in &self.titulares {
            let has_sensitive = titular
                .categorias_datos
                .iter()
                .any(|cat| matches!(cat, DataCategory::Health | DataCategory::Biometric));

            if has_sensitive && !self.consentimiento {
                return Err(DataProtectionError::MissingConsent);
            }
        }

        Ok(())
    }

    /// Check if processing involves sensitive data
    pub fn has_sensitive_data(&self) -> bool {
        self.titulares.iter().any(|titular| {
            titular.categorias_datos.iter().any(|cat| {
                matches!(
                    cat,
                    DataCategory::Health | DataCategory::Biometric | DataCategory::Ideological
                )
            })
        })
    }
}

impl PrivacyNotice {
    /// Validate privacy notice completeness
    pub fn validate(&self) -> Result<(), DataProtectionError> {
        if self.identidad_responsable.is_empty() {
            return Err(DataProtectionError::PrivacyNoticeViolation(
                "Controller identity required".to_string(),
            ));
        }

        if self.finalidades.is_empty() {
            return Err(DataProtectionError::PrivacyNoticeViolation(
                "At least one purpose required".to_string(),
            ));
        }

        if self.medios_arco.is_empty() {
            return Err(DataProtectionError::PrivacyNoticeViolation(
                "ARCO rights mechanism required".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_processing_with_consent() {
        let processing = PersonalDataProcessing {
            responsable: "Empresa SA".to_string(),
            titulares: vec![DataSubject {
                nombre: "Juan Pérez".to_string(),
                categorias_datos: vec![DataCategory::Identification, DataCategory::Contact],
            }],
            finalidad: vec!["Marketing".to_string()],
            base_legal: LegalBasis::Consent,
            consentimiento: true,
        };

        assert!(processing.validate().is_ok());
    }

    #[test]
    fn test_privacy_notice_validation() {
        let notice = PrivacyNotice {
            identidad_responsable: "Empresa SA".to_string(),
            finalidades: vec!["Prestación de servicios".to_string()],
            datos_recabados: vec![DataCategory::Identification],
            transferencias: vec![],
            medios_arco: "correo@empresa.com".to_string(),
        };

        assert!(notice.validate().is_ok());
    }
}
