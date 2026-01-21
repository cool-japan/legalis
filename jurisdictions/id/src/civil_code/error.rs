//! Error types for Indonesian Civil Code

use thiserror::Error;

/// Result type for civil code operations
pub type CivilCodeResult<T> = Result<T, CivilCodeError>;

/// Errors related to KUHPerdata
#[derive(Debug, Error)]
pub enum CivilCodeError {
    /// Missing agreement (kesepakatan) - Pasal 1320 ayat 1
    #[error("Tidak ada kesepakatan yang sah (KUHPerdata Pasal 1320 ayat 1): {description}")]
    MissingAgreement { description: String },

    /// Agreement defect: fraud (penipuan) - Pasal 1328
    #[error("Kesepakatan cacat karena penipuan (KUHPerdata Pasal 1328): {description}")]
    Fraud { description: String },

    /// Agreement defect: duress (paksaan) - Pasal 1323-1327
    #[error("Kesepakatan cacat karena paksaan (KUHPerdata Pasal 1323-1327): {description}")]
    Duress { description: String },

    /// Agreement defect: mistake (kekhilafan) - Pasal 1322
    #[error("Kesepakatan cacat karena kekhilafan (KUHPerdata Pasal 1322): {description}")]
    Mistake { description: String },

    /// Incapacity to contract - Pasal 1320 ayat 2, Pasal 1330
    #[error("Pihak tidak cakap melakukan perbuatan hukum (KUHPerdata Pasal 1330): {description}")]
    Incapacity { description: String },

    /// No specific object - Pasal 1320 ayat 3
    #[error("Tidak ada hal tertentu sebagai objek perjanjian (KUHPerdata Pasal 1320 ayat 3)")]
    NoSpecificObject,

    /// Unlawful cause - Pasal 1320 ayat 4, Pasal 1337
    #[error("Sebab tidak halal (KUHPerdata Pasal 1337): {description}")]
    UnlawfulCause { description: String },

    /// Contract contrary to public order - Pasal 1337
    #[error("Perjanjian bertentangan dengan ketertiban umum (KUHPerdata Pasal 1337)")]
    ContraryToPublicOrder,

    /// Contract contrary to morality - Pasal 1337
    #[error("Perjanjian bertentangan dengan kesusilaan (KUHPerdata Pasal 1337)")]
    ContraryToMorality,

    /// Breach of contract - Pasal 1243
    #[error("Wanprestasi (KUHPerdata Pasal 1243): {description}")]
    BreachOfContract { description: String },

    /// Tort/unlawful act - Pasal 1365
    #[error("Perbuatan melawan hukum (KUHPerdata Pasal 1365): {description}")]
    Tort { description: String },

    /// Limitation period expired - Pasal 1967
    #[error("Daluwarsa (KUHPerdata Pasal 1967): tuntutan kedaluwarsa setelah {years} tahun")]
    LimitationExpired { years: u32 },

    /// Form requirement not met
    #[error("Syarat bentuk tidak terpenuhi: {requirement}")]
    FormRequirementNotMet { requirement: String },
}

impl CivilCodeError {
    /// Check if error results in void contract (batal demi hukum)
    pub fn results_in_void_contract(&self) -> bool {
        matches!(
            self,
            Self::NoSpecificObject
                | Self::UnlawfulCause { .. }
                | Self::ContraryToPublicOrder
                | Self::ContraryToMorality
        )
    }

    /// Check if error results in voidable contract (dapat dibatalkan)
    pub fn results_in_voidable_contract(&self) -> bool {
        matches!(
            self,
            Self::MissingAgreement { .. }
                | Self::Fraud { .. }
                | Self::Duress { .. }
                | Self::Mistake { .. }
                | Self::Incapacity { .. }
        )
    }

    /// Get limitation period for claims (in years)
    pub fn limitation_period(&self) -> Option<u32> {
        match self {
            Self::BreachOfContract { .. } => Some(30), // General contractual claims
            Self::Tort { .. } => Some(30),             // But with discovery rules
            Self::Fraud { .. } => Some(5),             // Voidability for fraud
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_void_vs_voidable() {
        let void_error = CivilCodeError::UnlawfulCause {
            description: "illegal".to_string(),
        };
        assert!(void_error.results_in_void_contract());
        assert!(!void_error.results_in_voidable_contract());

        let voidable_error = CivilCodeError::Fraud {
            description: "misrepresentation".to_string(),
        };
        assert!(!voidable_error.results_in_void_contract());
        assert!(voidable_error.results_in_voidable_contract());
    }

    #[test]
    fn test_error_messages() {
        let error = CivilCodeError::Incapacity {
            description: "Minor aged 17".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Pasal 1330"));
    }
}
