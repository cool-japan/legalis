//! BGB Tort Law Error Types (Unerlaubte Handlungen)
//!
//! Comprehensive error types for tort law validation with bilingual
//! messages (German primary, English secondary) and specific BGB article
//! references.

use thiserror::Error;

/// Result type for tort law operations
pub type Result<T> = std::result::Result<T, TortError>;

/// Tort law validation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TortError {
    // === §823 Abs. 1 Errors ===
    #[error(
        "Geschütztes Rechtsgut nicht verletzt (§823 Abs. 1 BGB)\n\
         Protected interest not violated (§823 para. 1 BGB)"
    )]
    NoProtectedInterestViolated,

    #[error(
        "Verschulden (Vorsatz oder Fahrlässigkeit) nicht nachgewiesen (§823 Abs. 1 BGB)\n\
         Fault (intent or negligence) not proven (§823 para. 1 BGB)"
    )]
    NoFaultProven,

    #[error(
        "Widerrechtlichkeit entfällt durch Rechtfertigungsgrund: {grund} (§823 Abs. 1 BGB)\n\
         Unlawfulness negated by justification ground: {grund} (§823 para. 1 BGB)"
    )]
    UnlawfulnessNegated { grund: String },

    #[error(
        "Kausalität zwischen Handlung und Schaden nicht nachgewiesen (§823 Abs. 1 BGB)\n\
         Causation between conduct and damage not proven (§823 para. 1 BGB)"
    )]
    CausationNotProven,

    #[error(
        "Schaden nicht nachgewiesen oder Schadenshöhe unklar (§§249-252 BGB)\n\
         Damage not proven or amount unclear (§§249-252 BGB)"
    )]
    DamageNotProven,

    // === §823 Abs. 2 Errors ===
    #[error(
        "Schutzgesetz '{gesetz}' nicht verletzt (§823 Abs. 2 BGB)\n\
         Protective statute '{gesetz}' not violated (§823 para. 2 BGB)"
    )]
    ProtectiveStatuteNotViolated { gesetz: String },

    #[error(
        "Geschädigter nicht im Schutzbereich des Gesetzes '{gesetz}' (§823 Abs. 2 BGB)\n\
         Injured party not within protective scope of statute '{gesetz}' (§823 para. 2 BGB)"
    )]
    NotWithinProtectiveScope { gesetz: String },

    // === §826 Errors ===
    #[error(
        "Schädigungsvorsatz nicht nachgewiesen (§826 BGB)\n\
         Intent to cause damage not proven (§826 BGB)"
    )]
    NoIntentToHarm,

    #[error(
        "Sittenwidrigkeit der Handlung nicht nachgewiesen (§826 BGB)\n\
         Conduct not proven contrary to good morals (§826 BGB)"
    )]
    NotContraryToGoodMorals,

    // === §831 Errors (Vicarious Liability) ===
    #[error(
        "Verrichtungsgehilfe '{name}' nicht im Auftrag des Geschäftsherrn tätig (§831 Abs. 1 BGB)\n\
         Agent '{name}' not acting on behalf of principal (§831 para. 1 BGB)"
    )]
    NotActingForPrincipal { name: String },

    #[error(
        "Geschäftsherr hat Entlastungsbeweis erbracht (§831 Abs. 1 S. 2 BGB)\n\
         Principal has proven exculpation (§831 para. 1 sent. 2 BGB)"
    )]
    PrincipalExculpated,

    // === Damage Calculation Errors ===
    #[error(
        "Schadensberechnung fehlerhaft: {fehler}\n\
         Damage calculation error: {fehler}"
    )]
    DamageCalculationError { fehler: String },

    #[error(
        "Entgangener Gewinn nicht hinreichend wahrscheinlich (§252 BGB)\n\
         Lost profit not sufficiently probable (§252 BGB)"
    )]
    LostProfitNotSufficientlyProbable,

    #[error(
        "Schmerzensgeld-Anspruch ohne Personenschaden nicht möglich (§253 Abs. 2 BGB)\n\
         Pain and suffering claim requires personal injury (§253 para. 2 BGB)"
    )]
    PainSufferingRequiresPersonalInjury,

    // === Causation Errors ===
    #[error(
        "Haftungsbegründende Kausalität fehlt (conditio sine qua non)\n\
         Factual causation missing (conditio sine qua non)"
    )]
    NoFactualCausation,

    #[error(
        "Haftungsausfüllende Kausalität fehlt (Adäquanztheorie)\n\
         Legal causation missing (adequacy theory)"
    )]
    NoLegalCausation,

    #[error(
        "Schutzzweck der Norm nicht erfüllt (Schutzzwecklehre)\n\
         Protective purpose of norm not fulfilled"
    )]
    ProtectivePurposeNotFulfilled,

    // === Contributory Negligence ===
    #[error(
        "Mitverschulden des Geschädigten beträgt {prozent}% (§254 BGB)\n\
         Contributory negligence of injured party: {prozent}% (§254 BGB)"
    )]
    ContributoryNegligence { prozent: u8 },

    // === Prescription/Limitation Errors ===
    #[error(
        "Anspruch verjährt nach {jahre} Jahren (§§195, 199 BGB)\n\
         Claim prescribed after {jahre} years (§§195, 199 BGB)"
    )]
    ClaimPrescribed { jahre: u32 },

    #[error(
        "Kenntniserlangung verjährungsrelevant (§199 Abs. 1 BGB)\n\
         Knowledge acquisition relevant for prescription (§199 para. 1 BGB)"
    )]
    KnowledgeRelevantForPrescription,

    // === General Validation Errors ===
    #[error(
        "Schädiger (tortfeasor) nicht angegeben\n\
         Tortfeasor not specified"
    )]
    TortfeasorMissing,

    #[error(
        "Geschädigter (injured party) nicht angegeben\n\
         Injured party not specified"
    )]
    InjuredPartyMissing,

    #[error(
        "Handlung/Verletzung nicht beschrieben\n\
         Conduct/violation not described"
    )]
    ConductNotDescribed,

    #[error(
        "Schadenshöhe muss größer als null sein\n\
         Damage amount must be greater than zero"
    )]
    ZeroDamage,

    #[error(
        "Mehrere Fehler: {errors:?}\n\
         Multiple errors: {errors:?}"
    )]
    MultipleErrors { errors: Vec<String> },
}

impl TortError {
    /// Get the BGB article reference for this error
    pub fn article_reference(&self) -> &'static str {
        match self {
            Self::NoProtectedInterestViolated => "§823 Abs. 1 BGB",
            Self::NoFaultProven => "§823 Abs. 1 BGB",
            Self::UnlawfulnessNegated { .. } => "§823 Abs. 1 BGB",
            Self::CausationNotProven => "§823 Abs. 1 BGB",
            Self::DamageNotProven => "§§249-252 BGB",
            Self::ProtectiveStatuteNotViolated { .. } => "§823 Abs. 2 BGB",
            Self::NotWithinProtectiveScope { .. } => "§823 Abs. 2 BGB",
            Self::NoIntentToHarm => "§826 BGB",
            Self::NotContraryToGoodMorals => "§826 BGB",
            Self::NotActingForPrincipal { .. } => "§831 Abs. 1 BGB",
            Self::PrincipalExculpated => "§831 Abs. 1 S. 2 BGB",
            Self::DamageCalculationError { .. } => "§§249-252 BGB",
            Self::LostProfitNotSufficientlyProbable => "§252 BGB",
            Self::PainSufferingRequiresPersonalInjury => "§253 Abs. 2 BGB",
            Self::NoFactualCausation => "General",
            Self::NoLegalCausation => "General (Adäquanztheorie)",
            Self::ProtectivePurposeNotFulfilled => "General (Schutzzwecklehre)",
            Self::ContributoryNegligence { .. } => "§254 BGB",
            Self::ClaimPrescribed { .. } => "§§195, 199 BGB",
            Self::KnowledgeRelevantForPrescription => "§199 Abs. 1 BGB",
            Self::TortfeasorMissing => "General",
            Self::InjuredPartyMissing => "General",
            Self::ConductNotDescribed => "General",
            Self::ZeroDamage => "General",
            Self::MultipleErrors { .. } => "Multiple",
        }
    }

    /// Check if this error relates to §823 Abs. 1
    pub fn is_section_823_1(&self) -> bool {
        matches!(
            self,
            Self::NoProtectedInterestViolated
                | Self::NoFaultProven
                | Self::UnlawfulnessNegated { .. }
                | Self::CausationNotProven
        )
    }

    /// Check if this error relates to §826
    pub fn is_section_826(&self) -> bool {
        matches!(self, Self::NoIntentToHarm | Self::NotContraryToGoodMorals)
    }

    /// Check if this error relates to causation analysis
    pub fn is_causation_error(&self) -> bool {
        matches!(
            self,
            Self::CausationNotProven
                | Self::NoFactualCausation
                | Self::NoLegalCausation
                | Self::ProtectivePurposeNotFulfilled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_article_references() {
        let error1 = TortError::NoProtectedInterestViolated;
        assert_eq!(error1.article_reference(), "§823 Abs. 1 BGB");

        let error2 = TortError::NoIntentToHarm;
        assert_eq!(error2.article_reference(), "§826 BGB");

        let error3 = TortError::ContributoryNegligence { prozent: 30 };
        assert_eq!(error3.article_reference(), "§254 BGB");
    }

    #[test]
    fn test_is_section_823_1() {
        let error1 = TortError::NoProtectedInterestViolated;
        assert!(error1.is_section_823_1());

        let error2 = TortError::NoIntentToHarm;
        assert!(!error2.is_section_823_1());
    }

    #[test]
    fn test_is_section_826() {
        let error1 = TortError::NoIntentToHarm;
        assert!(error1.is_section_826());

        let error2 = TortError::NoFaultProven;
        assert!(!error2.is_section_826());
    }

    #[test]
    fn test_is_causation_error() {
        let error1 = TortError::CausationNotProven;
        assert!(error1.is_causation_error());

        let error2 = TortError::NoFactualCausation;
        assert!(error2.is_causation_error());

        let error3 = TortError::NoIntentToHarm;
        assert!(!error3.is_causation_error());
    }

    #[test]
    fn test_bilingual_error_messages() {
        let error = TortError::NoProtectedInterestViolated;
        let message = error.to_string();
        assert!(message.contains("Geschütztes Rechtsgut"));
        assert!(message.contains("Protected interest"));
        assert!(message.contains("§823"));
    }

    #[test]
    fn test_contributory_negligence_error() {
        let error = TortError::ContributoryNegligence { prozent: 25 };
        let message = error.to_string();
        assert!(message.contains("25"));
        assert!(message.contains("Mitverschulden"));
        assert!(message.contains("§254"));
    }

    #[test]
    fn test_lost_profit_error() {
        let error = TortError::LostProfitNotSufficientlyProbable;
        assert_eq!(error.article_reference(), "§252 BGB");
        let message = error.to_string();
        assert!(message.contains("hinreichend wahrscheinlich"));
        assert!(message.contains("sufficiently probable"));
    }
}
