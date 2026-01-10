//! Error types for French inheritance law
//!
//! This module provides comprehensive error handling for inheritance law
//! violations with bilingual (French/English) support.

use thiserror::Error;

/// Bilingual string for error messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BilingualString {
    pub fr: String,
    pub en: String,
}

impl BilingualString {
    /// Creates a new bilingual string
    pub fn new(fr: impl Into<String>, en: impl Into<String>) -> Self {
        Self {
            fr: fr.into(),
            en: en.into(),
        }
    }
}

/// Result type for inheritance law operations
pub type InheritanceLawResult<T> = Result<T, InheritanceLawError>;

/// Errors that can occur in inheritance law operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum InheritanceLawError {
    /// Succession not yet opened (Article 720)
    #[error("Succession not opened: death date required")]
    SuccessionNotOpened,

    /// Invalid domicile
    #[error("Invalid last domicile: {domicile}")]
    InvalidDomicile { domicile: String },

    /// Invalid will - missing required elements
    #[error("Invalid will: {reason}")]
    InvalidWill { reason: String },

    /// Holographic will not properly handwritten (Article 970)
    #[error("Holographic will must be entirely handwritten, dated, and signed")]
    HolographicWillNotHandwritten,

    /// Authentic will missing notary or witnesses (Article 971)
    #[error("Authentic will requires notary and two witnesses")]
    AuthenticWillMissingFormalities,

    /// Mystic will not properly sealed (Article 976)
    #[error("Mystic will must be sealed and presented to notary")]
    MysticWillNotSealed,

    /// Will has been revoked
    #[error("Will has been revoked")]
    WillRevoked,

    /// Reserved portion violated (Article 912-913)
    #[error("Reserved portion violated: allocated {allocated}, required {required}")]
    ReservedPortionViolation { allocated: f64, required: f64 },

    /// Disposition exceeds available portion (quotité disponible)
    #[error("Disposition exceeds available portion: {amount} > {available}")]
    ExceedsAvailablePortion { amount: u64, available: u64 },

    /// No heirs found
    #[error("No heirs found for succession")]
    NoHeirs,

    /// Invalid heir relationship
    #[error("Invalid heir relationship: {relationship}")]
    InvalidHeirRelationship { relationship: String },

    /// Heir has renounced succession
    #[error("Heir {heir} has renounced the succession")]
    HeirRenounced { heir: String },

    /// Estate insolvent (debts exceed assets)
    #[error("Estate insolvent: debts {debts} exceed assets {assets}")]
    EstateInsolvent { debts: u64, assets: u64 },

    /// Invalid share distribution
    #[error("Invalid share distribution: total shares {total} must equal 1.0")]
    InvalidShareDistribution { total: f64 },

    /// Missing testator
    #[error("Missing testator in will")]
    MissingTestator,

    /// Invalid date
    #[error("Invalid date: {reason}")]
    InvalidDate { reason: String },

    /// Disposition to ineligible beneficiary
    #[error("Beneficiary {beneficiary} is ineligible")]
    IneligibleBeneficiary { beneficiary: String },

    /// Multiple errors occurred
    #[error("Multiple errors: {0:?}")]
    MultipleErrors(Vec<InheritanceLawError>),
}

impl InheritanceLawError {
    /// Returns a bilingual description of the error
    pub fn description(&self) -> BilingualString {
        match self {
            Self::SuccessionNotOpened => BilingualString::new(
                "La succession n'est pas encore ouverte (Article 720)",
                "Succession not yet opened (Article 720)",
            ),

            Self::InvalidDomicile { domicile } => BilingualString::new(
                format!("Domicile invalide : {}", domicile),
                format!("Invalid domicile: {}", domicile),
            ),

            Self::InvalidWill { reason } => BilingualString::new(
                format!("Testament invalide : {}", reason),
                format!("Invalid will: {}", reason),
            ),

            Self::HolographicWillNotHandwritten => BilingualString::new(
                "Le testament olographe doit être entièrement écrit de la main du testateur, daté et signé (Article 970)",
                "Holographic will must be entirely handwritten, dated, and signed by testator (Article 970)",
            ),

            Self::AuthenticWillMissingFormalities => BilingualString::new(
                "Le testament authentique nécessite un notaire et deux témoins (Article 971)",
                "Authentic will requires a notary and two witnesses (Article 971)",
            ),

            Self::MysticWillNotSealed => BilingualString::new(
                "Le testament mystique doit être scellé et présenté au notaire (Article 976)",
                "Mystic will must be sealed and presented to notary (Article 976)",
            ),

            Self::WillRevoked => {
                BilingualString::new("Le testament a été révoqué", "The will has been revoked")
            }

            Self::ReservedPortionViolation {
                allocated,
                required,
            } => BilingualString::new(
                format!(
                    "Violation de la réserve héréditaire : alloué {:.2}, requis {:.2} (Articles 912-913)",
                    allocated, required
                ),
                format!(
                    "Reserved portion violated: allocated {:.2}, required {:.2} (Articles 912-913)",
                    allocated, required
                ),
            ),

            Self::ExceedsAvailablePortion { amount, available } => BilingualString::new(
                format!(
                    "La disposition excède la quotité disponible : {} > {}",
                    amount, available
                ),
                format!(
                    "Disposition exceeds available portion: {} > {}",
                    amount, available
                ),
            ),

            Self::NoHeirs => BilingualString::new(
                "Aucun héritier trouvé pour la succession",
                "No heirs found for succession",
            ),

            Self::InvalidHeirRelationship { relationship } => BilingualString::new(
                format!("Relation héritière invalide : {}", relationship),
                format!("Invalid heir relationship: {}", relationship),
            ),

            Self::HeirRenounced { heir } => BilingualString::new(
                format!("L'héritier {} a renoncé à la succession", heir),
                format!("Heir {} has renounced the succession", heir),
            ),

            Self::EstateInsolvent { debts, assets } => BilingualString::new(
                format!(
                    "Succession insolvable : dettes {} dépassent les actifs {} (Article 873)",
                    debts, assets
                ),
                format!(
                    "Estate insolvent: debts {} exceed assets {} (Article 873)",
                    debts, assets
                ),
            ),

            Self::InvalidShareDistribution { total } => BilingualString::new(
                format!(
                    "Distribution de parts invalide : total {:.2} doit égaler 1.0",
                    total
                ),
                format!(
                    "Invalid share distribution: total {:.2} must equal 1.0",
                    total
                ),
            ),

            Self::MissingTestator => BilingualString::new(
                "Testateur manquant dans le testament",
                "Missing testator in will",
            ),

            Self::InvalidDate { reason } => BilingualString::new(
                format!("Date invalide : {}", reason),
                format!("Invalid date: {}", reason),
            ),

            Self::IneligibleBeneficiary { beneficiary } => BilingualString::new(
                format!("Le bénéficiaire {} n'est pas éligible", beneficiary),
                format!("Beneficiary {} is ineligible", beneficiary),
            ),

            Self::MultipleErrors(errors) => {
                let fr_msgs: Vec<String> = errors.iter().map(|e| e.description().fr).collect();
                let en_msgs: Vec<String> = errors.iter().map(|e| e.description().en).collect();

                BilingualString::new(
                    format!("Erreurs multiples : {}", fr_msgs.join("; ")),
                    format!("Multiple errors: {}", en_msgs.join("; ")),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_succession_not_opened_error() {
        let error = InheritanceLawError::SuccessionNotOpened;
        let desc = error.description();
        assert!(desc.fr.contains("succession"));
        assert!(desc.en.contains("Succession"));
    }

    #[test]
    fn test_reserved_portion_violation_error() {
        let error = InheritanceLawError::ReservedPortionViolation {
            allocated: 0.3,
            required: 0.5,
        };
        let desc = error.description();
        assert!(desc.fr.contains("réserve héréditaire"));
        assert!(desc.en.contains("Reserved portion"));
        assert!(desc.fr.contains("0.30"));
        assert!(desc.fr.contains("0.50"));
    }

    #[test]
    fn test_holographic_will_error() {
        let error = InheritanceLawError::HolographicWillNotHandwritten;
        let desc = error.description();
        assert!(desc.fr.contains("olographe"));
        assert!(desc.en.contains("Holographic"));
        assert!(desc.fr.contains("Article 970"));
    }

    #[test]
    fn test_estate_insolvent_error() {
        let error = InheritanceLawError::EstateInsolvent {
            debts: 500_000,
            assets: 300_000,
        };
        let desc = error.description();
        assert!(desc.fr.contains("insolvable"));
        assert!(desc.en.contains("insolvent"));
        assert!(desc.fr.contains("500000"));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            InheritanceLawError::SuccessionNotOpened,
            InheritanceLawError::NoHeirs,
        ];
        let error = InheritanceLawError::MultipleErrors(errors);
        let desc = error.description();
        assert!(desc.fr.contains("multiples"));
        assert!(desc.en.contains("Multiple"));
    }

    #[test]
    fn test_will_revoked_error() {
        let error = InheritanceLawError::WillRevoked;
        let desc = error.description();
        assert!(desc.fr.contains("révoqué"));
        assert!(desc.en.contains("revoked"));
    }
}
