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

    // === §824 Errors (Kreditgefährdung / Credit Endangerment) ===
    #[error(
        "Äußerung ist ein Werturteil, keine Tatsachenbehauptung (§824 Abs. 1 BGB)\n\
         Statement is a value judgment, not an assertion of fact (§824 para. 1 BGB)"
    )]
    NotFactualAssertion,

    #[error(
        "Behauptete/verbreitete Tatsache ist nicht unwahr (§824 Abs. 1 BGB)\n\
         Asserted/disseminated fact is not untrue (§824 para. 1 BGB)"
    )]
    StatementNotUntrue,

    #[error(
        "Tatsache nicht geeignet, Kredit, Erwerb oder Fortkommen zu gefährden (§824 Abs. 1 BGB)\n\
         Fact not suitable to endanger credit, earnings or advancement (§824 para. 1 BGB)"
    )]
    NotSuitableToEndangerCredit,

    #[error(
        "Weder Kenntnis noch fahrlässige Unkenntnis der Unwahrheit (§824 Abs. 1 BGB)\n\
         Neither knowledge nor negligent ignorance of the untruth (§824 para. 1 BGB)"
    )]
    NoKnowledgeOrNegligenceOfUntruth,

    #[error(
        "Haftung entfällt wegen berechtigten Interesses an der Mitteilung (§824 Abs. 2 BGB)\n\
         Liability excluded due to legitimate interest in the communication (§824 para. 2 BGB)"
    )]
    LegitimateInterestPrivilege,

    // === §825 Errors (Bestimmung zu sexuellen Handlungen) ===
    #[error(
        "Keine Bestimmung zu sexuellen Handlungen nachgewiesen (§825 BGB)\n\
         No inducement to sexual acts established (§825 BGB)"
    )]
    NoSexualActInducement,

    #[error(
        "Kein qualifiziertes Bestimmungsmittel (Hinterlist, Drohung, Missbrauch eines \
         Abhängigkeitsverhältnisses) (§825 BGB)\n\
         No qualifying means (deception, threat, abuse of a relationship of dependence) (§825 BGB)"
    )]
    NoQualifyingInducementMeans,

    // === §832 Errors (Aufsichtspflicht / Supervision Liability) ===
    #[error(
        "Beaufsichtigte Person hat keinen widerrechtlichen Schaden zugefügt (§832 BGB)\n\
         Supervised person did not unlawfully cause damage (§832 BGB)"
    )]
    NoUnlawfulDamageBySupervised,

    #[error(
        "Aufsichtspflichtiger entlastet: {ground} (§832 Abs. 1 S. 2 BGB)\n\
         Supervisor exculpated: {ground} (§832 para. 1 sent. 2 BGB)"
    )]
    SupervisorExculpated { ground: String },

    // === §833/§834 Errors (Tierhalterhaftung / Animal Liability) ===
    #[error(
        "Schaden wurde nicht durch die spezifische Tiergefahr verursacht (§833 BGB)\n\
         Damage was not caused by the specific animal hazard (§833 BGB)"
    )]
    NotCausedByAnimal,

    #[error(
        "Tierhalter/Tieraufseher entlastet: {ground} (§833 S. 2 / §834 S. 2 BGB)\n\
         Animal keeper/supervisor exculpated: {ground} (§833 sent. 2 / §834 sent. 2 BGB)"
    )]
    AnimalKeeperExculpated { ground: String },

    // === §836-838 Errors (Gebäudehaftung / Building Liability) ===
    #[error(
        "Schaden nicht durch Einsturz/Ablösung infolge fehlerhafter Errichtung oder \
         mangelhafter Unterhaltung (§836 BGB)\n\
         Damage not caused by collapse/detachment resulting from faulty construction or \
         defective maintenance (§836 BGB)"
    )]
    NoStructuralDefectCausation,

    #[error(
        "Besitzer/Unterhaltungspflichtiger entlastet: {ground} (§836 Abs. 1 S. 2 BGB)\n\
         Possessor/maintenance obligor exculpated: {ground} (§836 para. 1 sent. 2 BGB)"
    )]
    BuildingPossessorExculpated { ground: String },

    // === §839 Errors (Amtshaftung / Public Official Liability) ===
    #[error(
        "Schädiger ist kein Beamter im haftungsrechtlichen Sinne (§839 BGB)\n\
         Wrongdoer is not an official in the liability sense (§839 BGB)"
    )]
    NotAnOfficial,

    #[error(
        "Keine drittbezogene Amtspflichtverletzung nachgewiesen (§839 Abs. 1 BGB)\n\
         No breach of an official duty established (§839 para. 1 BGB)"
    )]
    NoOfficialDutyBreach,

    #[error(
        "Amtspflicht bestand nicht gegenüber dem Geschädigten (Drittbezogenheit) (§839 Abs. 1 BGB)\n\
         Official duty was not owed to the injured party (third-party relation) (§839 para. 1 BGB)"
    )]
    NoDutyToThirdParty,

    #[error(
        "Kein Verschulden des Beamten (Vorsatz/Fahrlässigkeit) (§839 Abs. 1 BGB)\n\
         No fault of the official (intent/negligence) (§839 para. 1 BGB)"
    )]
    NoOfficialFault,

    #[error(
        "Subsidiarität: bei bloßer Fahrlässigkeit besteht anderweitige Ersatzmöglichkeit \
         (§839 Abs. 1 S. 2 BGB)\n\
         Subsidiarity: alternative compensation available for mere negligence \
         (§839 para. 1 sent. 2 BGB)"
    )]
    OfficialLiabilitySubsidiary,

    #[error(
        "Spruchrichterprivileg: Amtspflichtverletzung im Urteil ohne Straftat (§839 Abs. 2 BGB)\n\
         Judicial privilege: breach of duty in a judgment without a criminal offense (§839 para. 2 BGB)"
    )]
    JudicialPrivilege,

    #[error(
        "Haftung entfällt: Schaden war durch Gebrauch eines Rechtsmittels abwendbar (§839 Abs. 3 BGB)\n\
         Liability excluded: damage could have been averted by use of a legal remedy (§839 para. 3 BGB)"
    )]
    FailureToUseLegalRemedy,

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
            // §824 Kreditgefährdung
            Self::NotFactualAssertion => "§824 Abs. 1 BGB",
            Self::StatementNotUntrue => "§824 Abs. 1 BGB",
            Self::NotSuitableToEndangerCredit => "§824 Abs. 1 BGB",
            Self::NoKnowledgeOrNegligenceOfUntruth => "§824 Abs. 1 BGB",
            Self::LegitimateInterestPrivilege => "§824 Abs. 2 BGB",
            // §825 Bestimmung zu sexuellen Handlungen
            Self::NoSexualActInducement => "§825 BGB",
            Self::NoQualifyingInducementMeans => "§825 BGB",
            // §832 Aufsichtspflicht
            Self::NoUnlawfulDamageBySupervised => "§832 BGB",
            Self::SupervisorExculpated { .. } => "§832 Abs. 1 S. 2 BGB",
            // §833/§834 Tierhalterhaftung
            Self::NotCausedByAnimal => "§833 BGB",
            Self::AnimalKeeperExculpated { .. } => "§833 S. 2 / §834 S. 2 BGB",
            // §836-838 Gebäudehaftung
            Self::NoStructuralDefectCausation => "§836 BGB",
            Self::BuildingPossessorExculpated { .. } => "§836 Abs. 1 S. 2 BGB",
            // §839 Amtshaftung
            Self::NotAnOfficial => "§839 Abs. 1 BGB",
            Self::NoOfficialDutyBreach => "§839 Abs. 1 BGB",
            Self::NoDutyToThirdParty => "§839 Abs. 1 BGB",
            Self::NoOfficialFault => "§839 Abs. 1 BGB",
            Self::OfficialLiabilitySubsidiary => "§839 Abs. 1 S. 2 BGB",
            Self::JudicialPrivilege => "§839 Abs. 2 BGB",
            Self::FailureToUseLegalRemedy => "§839 Abs. 3 BGB",
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
