//! BGB Property Law Errors (Sachenrecht)
//!
//! Comprehensive bilingual error types for property law validation.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, PropertyError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PropertyError {
    // ========================================================================
    // Transfer Errors (Übertragung)
    // ========================================================================
    #[error(
        "Einigung fehlt oder ist ungültig (§929 BGB)\n\
         Agreement missing or invalid (§929 BGB)"
    )]
    NoAgreement,

    #[error(
        "Übergabe fehlt bei §929 S. 1 BGB\n\
         Delivery missing for §929 sentence 1 BGB"
    )]
    NoDelivery,

    #[error(
        "Übertragende Person '{transferor}' ist nicht Eigentümer (§929 BGB)\n\
         Transferor '{transferor}' is not owner (§929 BGB)"
    )]
    TransferorNotOwner { transferor: String },

    #[error(
        "Übertragende Person fehlt\n\
         Transferor missing"
    )]
    TransferorMissing,

    #[error(
        "Erwerbende Person fehlt\n\
         Transferee missing"
    )]
    TransfereeMissing,

    #[error(
        "Sache fehlt oder ist nicht beschrieben\n\
         Thing missing or not described"
    )]
    ThingMissing,

    // ========================================================================
    // Immovable Transfer Errors (Grundstücksübertragung)
    // ========================================================================
    #[error(
        "Grundbucheintragung fehlt (§873 Abs. 1 BGB)\n\
         Land registry entry missing (§873 para. 1 BGB)"
    )]
    NoLandRegistryEntry,

    #[error(
        "Flurstücksnummer fehlt oder ungültig\n\
         Parcel number missing or invalid"
    )]
    InvalidParcelNumber,

    #[error(
        "Grundbuchbezirk fehlt\n\
         Land registry district missing"
    )]
    LandRegistryDistrictMissing,

    #[error(
        "Grundstücksgröße muss größer als 0 sein\n\
         Land parcel size must be greater than 0"
    )]
    InvalidParcelSize,

    // ========================================================================
    // Possession Errors (Besitz)
    // ========================================================================
    #[error(
        "Tatsächliche Gewalt fehlt (§854 BGB)\n\
         Factual control missing (§854 BGB)"
    )]
    NoFactualControl,

    #[error(
        "Besitzwille fehlt (§854 BGB)\n\
         Possession will missing (§854 BGB)"
    )]
    NoPossessionWill,

    #[error(
        "Besitzstörung nicht nachgewiesen (§862 BGB)\n\
         Possession interference not proven (§862 BGB)"
    )]
    NoInterference,

    #[error(
        "Jahresfrist überschritten (§864 BGB)\n\
         One-year limitation exceeded (§864 BGB)"
    )]
    OneyearLimitationExceeded,

    // ========================================================================
    // Easement Errors (Dienstbarkeiten)
    // ========================================================================
    #[error(
        "Dienendes Grundstück fehlt\n\
         Servient land missing"
    )]
    ServientLandMissing,

    #[error(
        "Herrschendes Grundstück fehlt bei Grunddienstbarkeit (§1018 BGB)\n\
         Dominant land missing for predial easement (§1018 BGB)"
    )]
    DominantLandMissing,

    #[error(
        "Berechtigte Person fehlt bei persönlicher Dienstbarkeit (§1090 BGB)\n\
         Beneficiary missing for personal easement (§1090 BGB)"
    )]
    EasementBeneficiaryMissing,

    #[error(
        "Dienstbarkeit nicht im Grundbuch eingetragen (§1018 BGB)\n\
         Easement not registered in land registry (§1018 BGB)"
    )]
    EasementNotRegistered,

    // ========================================================================
    // Mortgage and Land Charge Errors (Hypotheken und Grundschulden)
    // ========================================================================
    #[error(
        "Hypothekenbetrag muss größer als 0 sein (§1113 BGB)\n\
         Mortgage amount must be greater than 0 (§1113 BGB)"
    )]
    InvalidMortgageAmount,

    #[error(
        "Gesicherte Forderung fehlt bei Hypothek (§1113 BGB)\n\
         Secured claim missing for mortgage (§1113 BGB)"
    )]
    SecuredClaimMissing,

    #[error(
        "Gesicherte Forderung existiert nicht (Hypothek ohne Forderung)\n\
         Secured claim does not exist (mortgage without claim)"
    )]
    SecuredClaimNonexistent,

    #[error(
        "Grundschuldbetrag muss größer als 0 sein (§1191 BGB)\n\
         Land charge amount must be greater than 0 (§1191 BGB)"
    )]
    InvalidLandChargeAmount,

    #[error(
        "Rangverhältnis ungültig (Rang muss ≥ 1 sein)\n\
         Priority rank invalid (rank must be ≥ 1)"
    )]
    InvalidPriorityRank,

    #[error(
        "Gläubiger fehlt\n\
         Creditor missing"
    )]
    CreditorMissing,

    #[error(
        "Schuldner fehlt\n\
         Debtor missing"
    )]
    DebtorMissing,

    // ========================================================================
    // Pledge Errors (Pfandrecht)
    // ========================================================================
    #[error(
        "Besitzübertragung fehlt bei Pfandrecht (§1205 BGB)\n\
         Possession transfer missing for pledge (§1205 BGB)"
    )]
    PledgePossessionNotTransferred,

    #[error(
        "Verpfändete Sache fehlt\n\
         Pledged thing missing"
    )]
    PledgedThingMissing,

    #[error(
        "Verpfändetes Recht fehlt oder ungültig\n\
         Pledged right missing or invalid"
    )]
    PledgedRightInvalid,

    #[error(
        "Pfandgläubiger fehlt\n\
         Pledgee missing"
    )]
    PledgeeMissing,

    #[error(
        "Pfandschuldner fehlt\n\
         Pledgor missing"
    )]
    PledgorMissing,

    // ========================================================================
    // Good Faith Acquisition Errors (Gutgläubiger Erwerb)
    // ========================================================================
    #[error(
        "Guter Glaube fehlt (§932 BGB)\n\
         Good faith missing (§932 BGB)"
    )]
    NoGoodFaith,

    #[error(
        "Grob fahrlässige Unkenntnis (§932 Abs. 2 BGB)\n\
         Grossly negligent lack of knowledge (§932 para. 2 BGB)"
    )]
    GrosslyNegligentLackOfKnowledge,

    #[error(
        "Sache abhanden gekommen - §935 BGB schließt gutgläubigen Erwerb aus\n\
         Thing lost or stolen - §935 BGB excludes good faith acquisition"
    )]
    ThingLostOrStolen,

    #[error(
        "Freiwillige Übertragung fehlt (§935 BGB)\n\
         Voluntary transfer missing (§935 BGB)"
    )]
    NoVoluntaryTransfer,

    // ========================================================================
    // Ownership Errors (Eigentum)
    // ========================================================================
    #[error(
        "Eigentümer fehlt oder ungültig\n\
         Owner missing or invalid"
    )]
    OwnerMissing,

    #[error(
        "Erwerbsmethode nicht spezifiziert\n\
         Acquisition method not specified"
    )]
    AcquisitionMethodMissing,

    #[error(
        "Eigentum kann nicht übertragen werden: {reason}\n\
         Ownership cannot be transferred: {reason}"
    )]
    TransferNotPossible { reason: String },

    // ========================================================================
    // General Validation Errors
    // ========================================================================
    #[error(
        "Wert muss größer als 0 sein\n\
         Value must be greater than 0"
    )]
    InvalidValue,

    #[error(
        "Beschreibung fehlt oder zu kurz (Minimum: 3 Zeichen)\n\
         Description missing or too short (minimum: 3 characters)"
    )]
    InvalidDescription,

    #[error(
        "Partei '{party}' fehlt oder ungültig\n\
         Party '{party}' missing or invalid"
    )]
    InvalidParty { party: String },

    #[error(
        "Datum liegt in der Zukunft\n\
         Date is in the future"
    )]
    FutureDate,

    #[error(
        "Validierung fehlgeschlagen: {reason}\n\
         Validation failed: {reason}"
    )]
    ValidationFailed { reason: String },
}

impl PropertyError {
    /// Get the BGB article reference for this error
    pub fn article_reference(&self) -> &'static str {
        match self {
            Self::NoAgreement | Self::NoDelivery | Self::TransferorNotOwner { .. } => "§929 BGB",
            Self::NoLandRegistryEntry => "§873 BGB",
            Self::NoFactualControl | Self::NoPossessionWill => "§854 BGB",
            Self::NoInterference => "§862 BGB",
            Self::OneyearLimitationExceeded => "§864 BGB",
            Self::DominantLandMissing | Self::EasementNotRegistered => "§1018 BGB",
            Self::EasementBeneficiaryMissing => "§1090 BGB",
            Self::InvalidMortgageAmount
            | Self::SecuredClaimMissing
            | Self::SecuredClaimNonexistent => "§1113 BGB",
            Self::InvalidLandChargeAmount => "§1191 BGB",
            Self::PledgePossessionNotTransferred => "§1205 BGB",
            Self::NoGoodFaith | Self::GrosslyNegligentLackOfKnowledge => "§932 BGB",
            Self::ThingLostOrStolen | Self::NoVoluntaryTransfer => "§935 BGB",
            _ => "BGB Sachenrecht",
        }
    }

    /// Check if error is related to movable transfers
    pub fn is_movable_transfer_error(&self) -> bool {
        matches!(
            self,
            Self::NoAgreement
                | Self::NoDelivery
                | Self::TransferorNotOwner { .. }
                | Self::TransferorMissing
                | Self::TransfereeMissing
                | Self::ThingMissing
        )
    }

    /// Check if error is related to immovable transfers
    pub fn is_immovable_transfer_error(&self) -> bool {
        matches!(
            self,
            Self::NoLandRegistryEntry
                | Self::InvalidParcelNumber
                | Self::LandRegistryDistrictMissing
                | Self::InvalidParcelSize
        )
    }

    /// Check if error is related to possession
    pub fn is_possession_error(&self) -> bool {
        matches!(
            self,
            Self::NoFactualControl
                | Self::NoPossessionWill
                | Self::NoInterference
                | Self::OneyearLimitationExceeded
        )
    }

    /// Check if error is related to good faith acquisition
    pub fn is_good_faith_error(&self) -> bool {
        matches!(
            self,
            Self::NoGoodFaith
                | Self::GrosslyNegligentLackOfKnowledge
                | Self::ThingLostOrStolen
                | Self::NoVoluntaryTransfer
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_article_reference() {
        let error = PropertyError::NoAgreement;
        assert_eq!(error.article_reference(), "§929 BGB");

        let error = PropertyError::NoLandRegistryEntry;
        assert_eq!(error.article_reference(), "§873 BGB");

        let error = PropertyError::NoGoodFaith;
        assert_eq!(error.article_reference(), "§932 BGB");
    }

    #[test]
    fn test_movable_transfer_error_detection() {
        assert!(PropertyError::NoAgreement.is_movable_transfer_error());
        assert!(PropertyError::NoDelivery.is_movable_transfer_error());
        assert!(!PropertyError::NoLandRegistryEntry.is_movable_transfer_error());
    }

    #[test]
    fn test_immovable_transfer_error_detection() {
        assert!(PropertyError::NoLandRegistryEntry.is_immovable_transfer_error());
        assert!(PropertyError::InvalidParcelNumber.is_immovable_transfer_error());
        assert!(!PropertyError::NoAgreement.is_immovable_transfer_error());
    }

    #[test]
    fn test_possession_error_detection() {
        assert!(PropertyError::NoFactualControl.is_possession_error());
        assert!(PropertyError::NoPossessionWill.is_possession_error());
        assert!(!PropertyError::NoAgreement.is_possession_error());
    }

    #[test]
    fn test_good_faith_error_detection() {
        assert!(PropertyError::NoGoodFaith.is_good_faith_error());
        assert!(PropertyError::ThingLostOrStolen.is_good_faith_error());
        assert!(!PropertyError::NoAgreement.is_good_faith_error());
    }
}
