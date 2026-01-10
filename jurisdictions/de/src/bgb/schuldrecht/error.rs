//! BGB Contract Law Error Types (Schuldrecht)
//!
//! Comprehensive error types for contract law validation with bilingual
//! messages (German primary, English secondary) and specific BGB article
//! references.

use thiserror::Error;

/// Result type for contract law operations
pub type Result<T> = std::result::Result<T, SchuldrechtError>;

/// Contract law validation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SchuldrechtError {
    // === Legal Capacity Errors (§§104-115 BGB) ===
    #[error(
        "Geschäftsunfähige Person kann keine wirksamen Willenserklärungen abgeben (§104 BGB)\n\
         Person without legal capacity cannot make valid declarations of intent (§104 BGB)"
    )]
    NoLegalCapacity { party_name: String },

    #[error(
        "Beschränkt geschäftsfähige Person '{party}' benötigt Zustimmung des gesetzlichen Vertreters (§107 BGB)\n\
         Person with limited capacity '{party}' requires consent of legal representative (§107 BGB)"
    )]
    RequiresRepresentativeConsent { party: String },

    #[error(
        "Gesetzlicher Vertreter nicht angegeben für '{party}' (§107 BGB)\n\
         Legal representative not specified for '{party}' (§107 BGB)"
    )]
    MissingLegalRepresentative { party: String },

    // === Declaration Errors (§§116-144 BGB) ===
    #[error(
        "Willenserklärung nicht zugegangen (§130 Abs. 1 BGB)\n\
         Declaration of intent not received (§130 para. 1 BGB)"
    )]
    DeclarationNotReceived,

    #[error(
        "Willenserklärung anfechtbar wegen {mistake_type} (§§119-122 BGB)\n\
         Declaration voidable due to {mistake_type} (§§119-122 BGB)"
    )]
    VoidableDueToMistake { mistake_type: String },

    #[error(
        "Willenserklärung anfechtbar wegen Drohung (§123 Abs. 1 BGB)\n\
         Declaration voidable due to duress (§123 para. 1 BGB)"
    )]
    VoidableDueToDuress,

    #[error(
        "Willenserklärung anfechtbar wegen arglistiger Täuschung (§123 Abs. 1 BGB)\n\
         Declaration voidable due to fraudulent misrepresentation (§123 para. 1 BGB)"
    )]
    VoidableDueToFraud,

    #[error(
        "Geheimer Vorbehalt unwirksam, da dem Empfänger bekannt (§116 S. 1 BGB)\n\
         Mental reservation ineffective as known to recipient (§116 sent. 1 BGB)"
    )]
    MentalReservationKnown,

    // === Offer and Acceptance Errors (§§145-157 BGB) ===
    #[error(
        "Angebot fehlt wesentliche Vertragsbestandteile (essentialia negotii) (§145 BGB)\n\
         Offer lacks essential terms (essentialia negotii) (§145 BGB)"
    )]
    OfferLacksEssentialTerms { missing_terms: Vec<String> },

    #[error(
        "Angebot ist nicht bindend (invitatio ad offerendum) (§145 BGB)\n\
         Offer is not binding (invitation to treat) (§145 BGB)"
    )]
    OfferNotBinding,

    #[error(
        "Angebot bereits widerrufen (§130 Abs. 1 S. 2 BGB)\n\
         Offer already revoked (§130 para. 1 sent. 2 BGB)"
    )]
    OfferRevoked,

    #[error(
        "Annahmefrist abgelaufen (§147 Abs. 2 BGB)\n\
         Acceptance deadline expired (§147 para. 2 BGB)"
    )]
    AcceptanceDeadlineExpired,

    #[error(
        "Verspätete Annahme (§150 Abs. 1 BGB) - gilt als neues Angebot\n\
         Late acceptance (§150 para. 1 BGB) - counts as new offer"
    )]
    LateAcceptance,

    #[error(
        "Annahme mit Änderungen (§150 Abs. 2 BGB) - gilt als Ablehnung und neues Angebot\n\
         Acceptance with modifications (§150 para. 2 BGB) - counts as rejection and counter-offer"
    )]
    AcceptanceWithModifications { modifications: Vec<String> },

    #[error(
        "Schweigen gilt nicht als Annahme (§151 BGB), außer bei Handelsbrauch\n\
         Silence does not constitute acceptance (§151 BGB), except by trade custom"
    )]
    SilenceNotAcceptance,

    // === Contract Formation Errors ===
    #[error(
        "Vertrag nicht zustande gekommen - fehlendes Angebot oder Annahme (§§145-157 BGB)\n\
         Contract not concluded - missing offer or acceptance (§§145-157 BGB)"
    )]
    ContractNotConcluded,

    #[error(
        "Vertragsinhalt fehlt (§154 Abs. 1 BGB)\n\
         Contract content missing (§154 para. 1 BGB)"
    )]
    ContractContentMissing,

    #[error(
        "Einigung über wesentlichen Punkt vorbehalten (§154 Abs. 1 S. 2 BGB)\n\
         Agreement on essential point reserved (§154 para. 1 sent. 2 BGB)"
    )]
    EssentialPointReserved,

    #[error(
        "Schriftform erforderlich aber nicht eingehalten (§§125, 126 BGB)\n\
         Written form required but not complied with (§§125, 126 BGB)"
    )]
    WrittenFormRequired,

    // === Performance Errors (Leistungsstörungen) ===
    #[error(
        "Leistung nicht erbracht - Pflichtverletzung (§280 Abs. 1 BGB)\n\
         Performance not rendered - breach of duty (§280 para. 1 BGB)"
    )]
    NonPerformance { obligor: String, obligation: String },

    #[error(
        "Leistung verspätet - Schuldnerverzug (§286 BGB)\n\
         Performance delayed - debtor in default (§286 BGB)"
    )]
    PerformanceDelayed {
        obligor: String,
        due_date: String,
        days_overdue: u32,
    },

    #[error(
        "Leistung unmöglich geworden (§275 BGB)\n\
         Performance has become impossible (§275 BGB)"
    )]
    PerformanceImpossible { reason: String },

    #[error(
        "Mangelhafte Leistung erbracht (Schlechterfüllung) (§280 BGB)\n\
         Defective performance rendered (§280 BGB)"
    )]
    DefectivePerformance { defects: Vec<String> },

    // === Fault and Causation Errors ===
    #[error(
        "Verschulden nicht nachgewiesen (§280 Abs. 1 S. 2 BGB)\n\
         Fault not proven (§280 para. 1 sent. 2 BGB)"
    )]
    FaultNotProven { party: String },

    #[error(
        "Kausalität zwischen Pflichtverletzung und Schaden nicht nachgewiesen (§280 BGB)\n\
         Causation between breach and damage not proven (§280 BGB)"
    )]
    CausationNotProven,

    #[error(
        "Mitverschulden des Gläubigers (§254 BGB) - Schadensersatz zu mindern\n\
         Contributory fault of creditor (§254 BGB) - damages to be reduced"
    )]
    ContributoryFault { percentage: u8 },

    // === Damages Errors (§§280-283 BGB) ===
    #[error(
        "Schadensersatzanspruch nicht gegeben - Voraussetzungen des §280 BGB nicht erfüllt\n\
         No claim for damages - requirements of §280 BGB not met"
    )]
    NoDamagesClaim { missing_elements: Vec<String> },

    #[error(
        "Schadensersatz statt der Leistung nur nach Fristsetzung (§281 Abs. 1 BGB)\n\
         Damages in lieu of performance only after setting grace period (§281 para. 1 BGB)"
    )]
    GracePeriodRequired,

    #[error(
        "Frist von {days} Tagen noch nicht abgelaufen (§281 Abs. 1 BGB)\n\
         Grace period of {days} days has not yet expired (§281 para. 1 BGB)"
    )]
    GracePeriodNotExpired { days: u32 },

    #[error(
        "Schadenshöhe nicht nachgewiesen (§249-252 BGB)\n\
         Amount of damages not proven (§§249-252 BGB)"
    )]
    DamageAmountNotProven,

    #[error(
        "Entgangener Gewinn nicht hinreichend wahrscheinlich (§252 BGB)\n\
         Lost profit not sufficiently probable (§252 BGB)"
    )]
    LostProfitNotSufficientlyProbable,

    // === Termination Errors (§§323-326 BGB) ===
    #[error(
        "Rücktritt nur nach erfolgloser Fristsetzung (§323 Abs. 1 BGB)\n\
         Termination only after unsuccessful grace period (§323 para. 1 BGB)"
    )]
    TerminationRequiresGracePeriod,

    #[error(
        "Rücktritt bei unerheblicher Pflichtverletzung ausgeschlossen (§323 Abs. 5 S. 2 BGB)\n\
         Termination excluded for minor breach (§323 para. 5 sent. 2 BGB)"
    )]
    MinorBreachNoTermination,

    #[error(
        "Rücktritt bei Fixgeschäft ohne Fristsetzung möglich (§323 Abs. 2 Nr. 2 BGB)\n\
         Termination for fixed-date transaction possible without grace period (§323 para. 2 no. 2 BGB)"
    )]
    FixedDateTransactionNoGracePeriodNeeded,

    #[error(
        "Rücktritt wegen ernsthafter und endgültiger Leistungsverweigerung (§323 Abs. 2 Nr. 1 BGB)\n\
         Termination due to serious and final refusal to perform (§323 para. 2 no. 1 BGB)"
    )]
    TerminationDueToRefusal,

    #[error(
        "Rücktritt bereits erklärt - Vertrag beendet (§§346 ff. BGB)\n\
         Termination already declared - contract terminated (§§346 ff. BGB)"
    )]
    AlreadyTerminated,

    // === Consumer Protection Errors (§§355-359 BGB) ===
    #[error(
        "Widerrufsfrist von {days} Tagen abgelaufen (§355 Abs. 2 BGB)\n\
         Withdrawal period of {days} days expired (§355 para. 2 BGB)"
    )]
    ConsumerWithdrawalPeriodExpired { days: u32 },

    #[error(
        "Verbraucher nicht ordnungsgemäß über Widerrufsrecht belehrt (§356 BGB)\n\
         Consumer not properly informed about withdrawal right (§356 BGB)"
    )]
    ConsumerNotInformed,

    // === Prescription/Limitation Errors (§§194-225 BGB) ===
    #[error(
        "Anspruch verjährt (§§194-195 BGB) - reguläre Verjährungsfrist: {years} Jahre\n\
         Claim prescribed (§§194-195 BGB) - standard limitation period: {years} years"
    )]
    ClaimPrescribed { years: u32 },

    #[error(
        "Verjährung gehemmt (§§203-206 BGB) - Frist läuft nicht\n\
         Prescription suspended (§§203-206 BGB) - period not running"
    )]
    PrescriptionSuspended { reason: String },

    // === General Validation Errors ===
    #[error(
        "Vertragspartei fehlt (§145 BGB)\n\
         Contract party missing (§145 BGB)"
    )]
    MissingParty,

    #[error(
        "Vertragsgegenstand fehlt oder unklar (§154 Abs. 1 BGB)\n\
         Subject matter missing or unclear (§154 para. 1 BGB)"
    )]
    SubjectMatterUnclear,

    #[error(
        "Gegenleistung nicht angegeben bei entgeltlichem Vertrag\n\
         Consideration not specified for contract with consideration"
    )]
    ConsiderationMissing,

    #[error(
        "Vertragstyp nicht erkennbar\n\
         Contract type not identifiable"
    )]
    UnknownContractType,

    #[error(
        "Mehrere Fehler: {errors:?}\n\
         Multiple errors: {errors:?}"
    )]
    MultipleErrors { errors: Vec<String> },
}

impl SchuldrechtError {
    /// Get the BGB article reference for this error
    pub fn article_reference(&self) -> &'static str {
        match self {
            Self::NoLegalCapacity { .. } => "§104 BGB",
            Self::RequiresRepresentativeConsent { .. } => "§107 BGB",
            Self::MissingLegalRepresentative { .. } => "§107 BGB",
            Self::DeclarationNotReceived => "§130 Abs. 1 BGB",
            Self::VoidableDueToMistake { .. } => "§§119-122 BGB",
            Self::VoidableDueToDuress => "§123 Abs. 1 BGB",
            Self::VoidableDueToFraud => "§123 Abs. 1 BGB",
            Self::MentalReservationKnown => "§116 S. 1 BGB",
            Self::OfferLacksEssentialTerms { .. } => "§145 BGB",
            Self::OfferNotBinding => "§145 BGB",
            Self::OfferRevoked => "§130 Abs. 1 S. 2 BGB",
            Self::AcceptanceDeadlineExpired => "§147 Abs. 2 BGB",
            Self::LateAcceptance => "§150 Abs. 1 BGB",
            Self::AcceptanceWithModifications { .. } => "§150 Abs. 2 BGB",
            Self::SilenceNotAcceptance => "§151 BGB",
            Self::ContractNotConcluded => "§§145-157 BGB",
            Self::ContractContentMissing => "§154 Abs. 1 BGB",
            Self::EssentialPointReserved => "§154 Abs. 1 S. 2 BGB",
            Self::WrittenFormRequired => "§§125, 126 BGB",
            Self::NonPerformance { .. } => "§280 Abs. 1 BGB",
            Self::PerformanceDelayed { .. } => "§286 BGB",
            Self::PerformanceImpossible { .. } => "§275 BGB",
            Self::DefectivePerformance { .. } => "§280 BGB",
            Self::FaultNotProven { .. } => "§280 Abs. 1 S. 2 BGB",
            Self::CausationNotProven => "§280 BGB",
            Self::ContributoryFault { .. } => "§254 BGB",
            Self::NoDamagesClaim { .. } => "§280 BGB",
            Self::GracePeriodRequired => "§281 Abs. 1 BGB",
            Self::GracePeriodNotExpired { .. } => "§281 Abs. 1 BGB",
            Self::DamageAmountNotProven => "§§249-252 BGB",
            Self::LostProfitNotSufficientlyProbable => "§252 BGB",
            Self::TerminationRequiresGracePeriod => "§323 Abs. 1 BGB",
            Self::MinorBreachNoTermination => "§323 Abs. 5 S. 2 BGB",
            Self::FixedDateTransactionNoGracePeriodNeeded => "§323 Abs. 2 Nr. 2 BGB",
            Self::TerminationDueToRefusal => "§323 Abs. 2 Nr. 1 BGB",
            Self::AlreadyTerminated => "§§346 ff. BGB",
            Self::ConsumerWithdrawalPeriodExpired { .. } => "§355 Abs. 2 BGB",
            Self::ConsumerNotInformed => "§356 BGB",
            Self::ClaimPrescribed { .. } => "§§194-195 BGB",
            Self::PrescriptionSuspended { .. } => "§§203-206 BGB",
            Self::MissingParty => "§145 BGB",
            Self::SubjectMatterUnclear => "§154 Abs. 1 BGB",
            Self::ConsiderationMissing => "General",
            Self::UnknownContractType => "General",
            Self::MultipleErrors { .. } => "Multiple",
        }
    }

    /// Check if this error renders a contract void (nichtig)
    pub fn makes_contract_void(&self) -> bool {
        matches!(
            self,
            Self::NoLegalCapacity { .. } | Self::WrittenFormRequired
        )
    }

    /// Check if this error makes a contract voidable (anfechtbar)
    pub fn makes_contract_voidable(&self) -> bool {
        matches!(
            self,
            Self::VoidableDueToMistake { .. }
                | Self::VoidableDueToDuress
                | Self::VoidableDueToFraud
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_article_references() {
        let error = SchuldrechtError::NoLegalCapacity {
            party_name: "Test".to_string(),
        };
        assert_eq!(error.article_reference(), "§104 BGB");

        let error2 = SchuldrechtError::GracePeriodRequired;
        assert_eq!(error2.article_reference(), "§281 Abs. 1 BGB");

        let error3 = SchuldrechtError::MinorBreachNoTermination;
        assert_eq!(error3.article_reference(), "§323 Abs. 5 S. 2 BGB");
    }

    #[test]
    fn test_void_vs_voidable() {
        let void_error = SchuldrechtError::NoLegalCapacity {
            party_name: "Child".to_string(),
        };
        assert!(void_error.makes_contract_void());
        assert!(!void_error.makes_contract_voidable());

        let voidable_error = SchuldrechtError::VoidableDueToDuress;
        assert!(!voidable_error.makes_contract_void());
        assert!(voidable_error.makes_contract_voidable());
    }

    #[test]
    fn test_bilingual_error_messages() {
        let error = SchuldrechtError::OfferRevoked;
        let message = error.to_string();
        assert!(message.contains("widerrufen"));
        assert!(message.contains("revoked"));
        assert!(message.contains("§130"));
    }

    #[test]
    fn test_performance_delay_error() {
        let error = SchuldrechtError::PerformanceDelayed {
            obligor: "Seller".to_string(),
            due_date: "2024-01-01".to_string(),
            days_overdue: 30,
        };
        let message = error.to_string();
        assert!(message.contains("verspätet"));
        assert!(message.contains("delayed"));
        assert!(message.contains("§286"));
    }

    #[test]
    fn test_grace_period_errors() {
        let error1 = SchuldrechtError::GracePeriodRequired;
        assert_eq!(error1.article_reference(), "§281 Abs. 1 BGB");

        let error2 = SchuldrechtError::GracePeriodNotExpired { days: 14 };
        assert!(error2.to_string().contains("14"));
    }

    #[test]
    fn test_consumer_protection_errors() {
        let error = SchuldrechtError::ConsumerWithdrawalPeriodExpired { days: 14 };
        assert!(error.to_string().contains("14"));
        assert_eq!(error.article_reference(), "§355 Abs. 2 BGB");
    }

    #[test]
    fn test_termination_errors() {
        let error1 = SchuldrechtError::MinorBreachNoTermination;
        assert!(!error1.makes_contract_void());
        assert!(!error1.makes_contract_voidable());

        let error2 = SchuldrechtError::TerminationDueToRefusal;
        assert_eq!(error2.article_reference(), "§323 Abs. 2 Nr. 1 BGB");
    }
}
