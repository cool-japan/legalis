//! Uniform Electronic Transactions Act (UETA) Tracker and Validators
//!
//! The Uniform Electronic Transactions Act (**UETA**) was promulgated by the
//! Uniform Law Commission in **1999**. It is the most widely enacted of the
//! electronic-commerce uniform acts: as of 2024 it has been adopted in **49
//! jurisdictions** — every state except New York, plus the District of Columbia,
//! Puerto Rico, and the U.S. Virgin Islands. New York instead enacted its own
//! Electronic Signatures and Records Act (ESRA, N.Y. State Tech. Law §§ 301-309).
//!
//! ## Purpose and Core Principle
//!
//! UETA does **not** itself compel anyone to use electronic records or
//! signatures. Its purpose is to remove barriers to electronic commerce by
//! providing that an electronic record or signature has the **same legal effect**
//! as its paper equivalent. The central operative rules are in § 7:
//!
//! - § 7(a): a record or signature may not be denied legal effect *solely*
//!   because it is in electronic form;
//! - § 7(b): a contract may not be denied legal effect *solely* because an
//!   electronic record was used in its formation;
//! - § 7(c): if a law requires a *record* in writing, an electronic record
//!   satisfies the law;
//! - § 7(d): if a law requires a *signature*, an electronic signature satisfies
//!   the law.
//!
//! ## Scope (§§ 3, 5)
//!
//! UETA applies only to transactions between parties each of which has **agreed**
//! to conduct the transaction by electronic means (§ 5(b)); agreement is
//! determined from the context and surrounding circumstances. § 3(b) **excludes**
//! the law of wills, codicils, and testamentary trusts, and (except for UCC
//! §§ 1-107 and 1-206 and Articles 2 and 2A) the Uniform Commercial Code.
//!
//! ## Relationship to E-SIGN
//!
//! The federal Electronic Signatures in Global and National Commerce Act
//! (**E-SIGN**, 15 U.S.C. §§ 7001-7031, 2000) establishes a national floor for
//! the validity of electronic records and signatures. Under 15 U.S.C. § 7002, a
//! state may modify, limit, or supersede E-SIGN by enacting **the official text
//! of UETA**. Thus a state's enactment of uniform UETA generally avoids E-SIGN
//! preemption, whereas non-uniform variations are measured against the E-SIGN
//! floor.

use super::adoption_status::AdoptionStatus;
use super::error::{Result, UniformActError};
use super::model_act::{DraftingBody, ModelActMetadata, US_JURISDICTIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The single jurisdiction in the 51-jurisdiction set that has **not** enacted
/// UETA and instead uses its own electronic-records statute.
const NON_UETA_JURISDICTIONS: [&str; 1] = ["NY"];

/// Returns model-act metadata for the Uniform Electronic Transactions Act.
#[must_use]
pub fn model_act() -> ModelActMetadata {
    ModelActMetadata::new(
        "UETA",
        "Uniform Electronic Transactions Act",
        DraftingBody::UniformLawCommission,
        1999,
    )
    .with_summary(
        "Gives electronic records and signatures the same legal effect as paper, for transactions \
         the parties have agreed to conduct electronically.",
    )
}

/// Key operative provisions of the Uniform Electronic Transactions Act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UetaSection {
    /// § 3 - Scope (exclusions: wills/codicils/testamentary trusts; most of the UCC).
    Scope,
    /// § 5 - Use of Electronic Records and Signatures; Variation by Agreement.
    UseAndAgreement,
    /// § 7 - Legal Recognition of Electronic Records, Signatures, and Contracts.
    LegalRecognition,
    /// § 8 - Provision of Information in Writing; Presentation of Records.
    InformationInWriting,
    /// § 9 - Attribution and Effect of Electronic Record and Electronic Signature.
    Attribution,
    /// § 10 - Effect of Change or Error.
    ChangeOrError,
    /// § 11 - Notarization and Acknowledgment.
    Notarization,
    /// § 12 - Retention of Electronic Records; Originals.
    Retention,
    /// § 13 - Admissibility in Evidence.
    Admissibility,
    /// § 14 - Automated Transaction (electronic agents).
    AutomatedTransaction,
    /// § 15 - Time and Place of Sending and Receipt.
    SendingAndReceipt,
    /// § 16 - Transferable Records.
    TransferableRecords,
}

impl UetaSection {
    /// Bluebook-style citation for the section.
    #[must_use]
    pub fn citation(&self) -> &'static str {
        match self {
            Self::Scope => "UETA § 3",
            Self::UseAndAgreement => "UETA § 5",
            Self::LegalRecognition => "UETA § 7",
            Self::InformationInWriting => "UETA § 8",
            Self::Attribution => "UETA § 9",
            Self::ChangeOrError => "UETA § 10",
            Self::Notarization => "UETA § 11",
            Self::Retention => "UETA § 12",
            Self::Admissibility => "UETA § 13",
            Self::AutomatedTransaction => "UETA § 14",
            Self::SendingAndReceipt => "UETA § 15",
            Self::TransferableRecords => "UETA § 16",
        }
    }

    /// Short description of the section's rule.
    #[must_use]
    pub fn summary(&self) -> &'static str {
        match self {
            Self::Scope => {
                "UETA applies to electronic records and signatures relating to a transaction, but \
                 does not apply to wills, codicils, or testamentary trusts, nor (with narrow \
                 exceptions) to transactions governed by the Uniform Commercial Code."
            }
            Self::UseAndAgreement => {
                "UETA applies only to parties who have agreed to conduct the transaction by \
                 electronic means, determined from the context and surrounding circumstances; the \
                 effect of its provisions may be varied by agreement (§ 5(b), (d))."
            }
            Self::LegalRecognition => {
                "A record or signature may not be denied legal effect solely because it is \
                 electronic; a contract may not be denied effect solely because an electronic \
                 record was used in its formation; an electronic record satisfies a writing \
                 requirement and an electronic signature satisfies a signature requirement \
                 (§ 7(a)-(d))."
            }
            Self::InformationInWriting => {
                "If a law requires that information be provided in writing or that a record be \
                 retained, an electronic record satisfies the law, subject to capability-of-\
                 retention requirements; a record may not be required in a specific format unless \
                 otherwise provided by law."
            }
            Self::Attribution => {
                "An electronic record or signature is attributable to a person if it was the act of \
                 the person, which may be shown in any manner, including a showing of the efficacy \
                 of any security procedure; the legal effect is determined from the context and \
                 surrounding circumstances at the time of creation, execution, or adoption (§ 9)."
            }
            Self::ChangeOrError => {
                "If a change or error in an electronic record occurs in a transmission between \
                 parties, an agreed security procedure that one party conformed to and the other \
                 did not lets the conforming party avoid the effect of the changed or erroneous \
                 record; in automated transactions an individual may avoid the effect of a keying \
                 error if no opportunity to prevent or correct it was provided and prompt notice \
                 and return are given (§ 10)."
            }
            Self::Notarization => {
                "If a law requires a signature or record to be notarized, acknowledged, verified, \
                 or made under oath, the requirement is satisfied if the electronic signature of \
                 the authorized person, together with all other required information, is attached \
                 to or logically associated with the record (§ 11)."
            }
            Self::Retention => {
                "A requirement to retain a record is satisfied by retaining an electronic record \
                 that accurately reflects the information and remains accessible for later \
                 reference; this requirement may not be imposed for information whose sole purpose \
                 was to enable transmission (§ 12). An electronic record so retained satisfies a \
                 requirement to retain an original."
            }
            Self::Admissibility => {
                "Evidence of a record or signature may not be excluded in a proceeding solely \
                 because it is in electronic form (§ 13)."
            }
            Self::AutomatedTransaction => {
                "A contract may be formed by the interaction of electronic agents, or by the \
                 interaction of an electronic agent and an individual, even if no individual was \
                 aware of or reviewed the agent's actions or the resulting terms (§ 14)."
            }
            Self::SendingAndReceipt => {
                "Unless otherwise agreed, an electronic record is sent when it is properly \
                 addressed, in a form capable of being processed by the recipient's system, and \
                 enters an information processing system outside the sender's control; it is \
                 received when it enters the recipient's designated system in a processable form, \
                 regardless of whether any individual is aware of its receipt (§ 15)."
            }
            Self::TransferableRecords => {
                "A transferable record is an electronic record that would be a note under UCC \
                 Article 3 or a document under Article 7 if it were in writing; a person having \
                 control of a transferable record is the holder and has the rights of a holder \
                 (§ 16)."
            }
        }
    }
}

/// A jurisdiction's adoption status for the Uniform Electronic Transactions Act.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UetaAdoption {
    /// Two-letter state / jurisdiction code.
    pub state: String,
    /// Adoption status.
    pub status: AdoptionStatus,
    /// Year the jurisdiction enacted its electronic-transactions statute.
    pub adopted_year: Option<u16>,
    /// Citation to the state statute, when known.
    pub citation: Option<String>,
}

impl UetaAdoption {
    /// Create a new adoption record with the given status.
    #[must_use]
    pub fn new(state: impl Into<String>, status: AdoptionStatus) -> Self {
        Self {
            state: state.into(),
            status,
            adopted_year: None,
            citation: None,
        }
    }

    /// Set the year of enactment.
    #[must_use]
    pub fn with_year(mut self, year: u16) -> Self {
        self.adopted_year = Some(year);
        self
    }

    /// Set the state statute citation.
    #[must_use]
    pub fn with_citation(mut self, citation: impl Into<String>) -> Self {
        self.citation = Some(citation.into());
        self
    }

    /// Whether the jurisdiction has enacted the uniform UETA (as opposed to a
    /// non-uniform electronic-records statute).
    #[must_use]
    pub fn has_ueta(&self) -> bool {
        matches!(
            self.status,
            AdoptionStatus::FullyAdopted | AdoptionStatus::AdoptedWithVariations
        )
    }
}

/// Tracks UETA adoption across the 51 US jurisdictions.
#[derive(Debug, Clone, Default)]
pub struct UetaTracker {
    adoptions: HashMap<String, UetaAdoption>,
}

impl UetaTracker {
    /// Create a tracker pre-populated with current adoption data.
    #[must_use]
    pub fn new() -> Self {
        let mut tracker = Self {
            adoptions: HashMap::new(),
        };
        tracker.initialize();
        tracker
    }

    fn initialize(&mut self) {
        for state in US_JURISDICTIONS {
            let adoption = if NON_UETA_JURISDICTIONS.contains(&state) {
                // New York: non-uniform Electronic Signatures and Records Act.
                UetaAdoption::new(state, AdoptionStatus::CustomLaw)
                    .with_year(1999)
                    .with_citation("N.Y. State Tech. Law §§ 301-309 (ESRA)")
            } else {
                UetaAdoption::new(state, AdoptionStatus::FullyAdopted)
            };
            self.adoptions.insert(state.to_string(), adoption);
        }

        // Representative state enactment citations.
        self.annotate("CA", 1999, "Cal. Civ. Code §§ 1633.1-1633.17");
        self.annotate("TX", 2001, "Tex. Bus. & Com. Code ch. 322");
        self.annotate("FL", 2000, "Fla. Stat. §§ 668.50");
        self.annotate("IL", 2021, "5 ILCS 175 (replaced by UETA, 815 ILCS 333)");
        self.annotate("WA", 2020, "Wash. Rev. Code ch. 1.80");
        self.annotate("PA", 1999, "73 Pa. Stat. §§ 2260.101-2260.5101");
    }

    fn annotate(&mut self, state: &str, year: u16, citation: &str) {
        if let Some(record) = self.adoptions.get_mut(state) {
            record.adopted_year = Some(year);
            record.citation = Some(citation.to_string());
        }
    }

    /// Get the adoption record for a jurisdiction.
    #[must_use]
    pub fn get_adoption(&self, state: &str) -> Option<&UetaAdoption> {
        self.adoptions.get(state)
    }

    /// Whether a jurisdiction has enacted the uniform UETA.
    #[must_use]
    pub fn has_adopted(&self, state: &str) -> bool {
        self.get_adoption(state).is_some_and(UetaAdoption::has_ueta)
    }

    /// All jurisdictions that have enacted the uniform UETA, sorted.
    #[must_use]
    pub fn ueta_states(&self) -> Vec<String> {
        let mut states: Vec<String> = self
            .adoptions
            .values()
            .filter(|a| a.has_ueta())
            .map(|a| a.state.clone())
            .collect();
        states.sort();
        states
    }

    /// Number of jurisdictions that have enacted the uniform UETA.
    #[must_use]
    pub fn ueta_count(&self) -> usize {
        self.adoptions.values().filter(|a| a.has_ueta()).count()
    }

    /// Jurisdictions that use a non-uniform electronic-records statute.
    #[must_use]
    pub fn non_uniform_states(&self) -> Vec<String> {
        let mut states: Vec<String> = self
            .adoptions
            .values()
            .filter(|a| matches!(a.status, AdoptionStatus::CustomLaw))
            .map(|a| a.state.clone())
            .collect();
        states.sort();
        states
    }

    /// Percentage of the 51 jurisdictions that have enacted the uniform UETA.
    #[must_use]
    pub fn ueta_percentage(&self) -> f64 {
        let total = self.adoptions.len();
        if total == 0 {
            return 0.0;
        }
        (self.ueta_count() as f64 / total as f64) * 100.0
    }

    /// Add or replace an adoption record.
    pub fn add_adoption(&mut self, adoption: UetaAdoption) {
        self.adoptions.insert(adoption.state.clone(), adoption);
    }
}

/// How an electronic signature was effected, used for attribution under § 9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignatureMethod {
    /// A typed name in a record (e.g., "/s/ Jane Doe").
    TypedName,
    /// A click-through "I agree" / "I accept" assent.
    ClickWrap,
    /// A digital signature backed by public-key cryptography.
    DigitalCryptographic,
    /// A biometric or hand-drawn signature captured electronically.
    Biometric,
    /// An action of an electronic agent attributable to its principal (§ 14).
    ElectronicAgent,
}

impl SignatureMethod {
    /// Human-readable description of the method.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::TypedName => "Typed name in a record",
            Self::ClickWrap => "Click-through assent",
            Self::DigitalCryptographic => "Cryptographic digital signature",
            Self::Biometric => "Biometric / captured handwritten signature",
            Self::ElectronicAgent => "Action of an electronic agent",
        }
    }
}

/// A fact pattern describing an electronic record or signature, evaluated for
/// legal recognition under UETA §§ 5 and 7.
#[derive(Debug, Clone)]
pub struct ElectronicRecord {
    /// Whether both parties agreed to transact electronically (§ 5(b)).
    pub parties_agreed_electronic: bool,
    /// Whether the record relates to a transaction (the act's threshold; § 3(a)).
    pub relates_to_transaction: bool,
    /// Whether the subject matter is a will, codicil, or testamentary trust,
    /// which is excluded from the act (§ 3(b)(1)).
    pub is_testamentary: bool,
    /// Whether the subject matter is governed by the UCC outside the act's
    /// carve-ins (§ 3(b)(2)).
    pub is_excluded_ucc: bool,
    /// Whether the record bears an electronic signature, and if so, by what method.
    pub signature_method: Option<SignatureMethod>,
}

impl Default for ElectronicRecord {
    fn default() -> Self {
        // A non-testamentary, non-UCC, agreed electronic transaction record.
        Self {
            parties_agreed_electronic: true,
            relates_to_transaction: true,
            is_testamentary: false,
            is_excluded_ucc: false,
            signature_method: Some(SignatureMethod::TypedName),
        }
    }
}

/// Returns every reason the electronic record or signature would fail to receive
/// legal recognition under UETA. An empty vector means it is legally recognized
/// to the same extent as a paper record/signature.
#[must_use]
pub fn electronic_record_issues(record: &ElectronicRecord) -> Vec<String> {
    let mut issues = Vec::new();

    if !record.relates_to_transaction {
        issues.push(
            "UETA applies only to records relating to a transaction between parties (§ 3(a))"
                .to_string(),
        );
    }
    if record.is_testamentary {
        issues.push(
            "wills, codicils, and testamentary trusts are excluded from UETA (§ 3(b)(1))"
                .to_string(),
        );
    }
    if record.is_excluded_ucc {
        issues.push(
            "transactions governed by the UCC (other than §§ 1-107, 1-206 and Articles 2, 2A) are \
             excluded from UETA (§ 3(b)(2))"
                .to_string(),
        );
    }
    if !record.parties_agreed_electronic {
        issues.push(
            "UETA applies only where the parties have agreed to conduct the transaction by \
             electronic means (§ 5(b))"
                .to_string(),
        );
    }

    issues
}

/// Validate that an electronic record or signature receives legal recognition
/// under UETA §§ 5 and 7.
///
/// # Errors
///
/// Returns [`UniformActError::ElectronicTransaction`] listing every reason the
/// record falls outside the act's recognition rule.
pub fn validate_electronic_record(record: &ElectronicRecord) -> Result<()> {
    let issues = electronic_record_issues(record);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(UniformActError::ElectronicTransaction(issues.join("; ")))
    }
}

/// Whether an electronic signature is attributable to the purported signer under
/// UETA § 9(a). Attribution requires that the signature was the *act of the
/// person*, shown in any manner. This helper returns `true` when a recognized
/// signature method is present on an otherwise in-scope record.
#[must_use]
pub fn signature_attributable(record: &ElectronicRecord) -> bool {
    record.signature_method.is_some() && electronic_record_issues(record).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_act_metadata() {
        let ueta = model_act();
        assert_eq!(ueta.short_name, "UETA");
        assert_eq!(ueta.promulgated_year, 1999);
        assert_eq!(ueta.drafting_body, DraftingBody::UniformLawCommission);
    }

    #[test]
    fn test_section_citations_and_summaries() {
        assert_eq!(UetaSection::LegalRecognition.citation(), "UETA § 7");
        assert_eq!(UetaSection::Attribution.citation(), "UETA § 9");
        assert_eq!(UetaSection::ChangeOrError.citation(), "UETA § 10");
        assert_eq!(UetaSection::Retention.citation(), "UETA § 12");
        assert_eq!(UetaSection::AutomatedTransaction.citation(), "UETA § 14");
        assert_eq!(UetaSection::SendingAndReceipt.citation(), "UETA § 15");
        // § 14 is about electronic agents.
        assert!(
            UetaSection::AutomatedTransaction
                .summary()
                .contains("electronic agent")
        );
        // § 7 is the core legal-recognition rule.
        assert!(
            UetaSection::LegalRecognition
                .summary()
                .contains("electronic")
        );
    }

    #[test]
    fn test_tracker_full_coverage() {
        let tracker = UetaTracker::new();
        assert_eq!(tracker.adoptions.len(), 51);
    }

    #[test]
    fn test_ueta_adoption_counts() {
        let tracker = UetaTracker::new();
        // 49 states + DC adopted; only New York is non-uniform within our 51-set.
        assert_eq!(tracker.ueta_count(), 50);
        assert!(!tracker.has_adopted("NY"));
        assert!(tracker.has_adopted("CA"));
        assert!(tracker.has_adopted("DC"));
        let pct = tracker.ueta_percentage();
        assert!(pct > 95.0, "unexpected pct: {pct}");
    }

    #[test]
    fn test_new_york_is_non_uniform() {
        let tracker = UetaTracker::new();
        let ny = tracker.get_adoption("NY").expect("NY tracked");
        assert!(matches!(ny.status, AdoptionStatus::CustomLaw));
        assert!(ny.citation.as_ref().expect("NY citation").contains("ESRA"));
        assert_eq!(tracker.non_uniform_states(), vec!["NY".to_string()]);
    }

    #[test]
    fn test_state_citations_annotated() {
        let tracker = UetaTracker::new();
        let ca = tracker.get_adoption("CA").expect("CA tracked");
        assert!(ca.citation.as_ref().expect("CA citation").contains("1633"));
        assert_eq!(ca.adopted_year, Some(1999));
    }

    #[test]
    fn test_valid_electronic_record() {
        assert!(validate_electronic_record(&ElectronicRecord::default()).is_ok());
        assert!(signature_attributable(&ElectronicRecord::default()));
    }

    #[test]
    fn test_testamentary_excluded() {
        let record = ElectronicRecord {
            is_testamentary: true,
            ..ElectronicRecord::default()
        };
        let err = validate_electronic_record(&record).expect_err("should fail");
        assert!(err.to_string().contains("§ 3(b)(1)"));
        // An excluded record is not attributable for UETA purposes.
        assert!(!signature_attributable(&record));
    }

    #[test]
    fn test_no_agreement_to_transact_electronically() {
        let record = ElectronicRecord {
            parties_agreed_electronic: false,
            ..ElectronicRecord::default()
        };
        let issues = electronic_record_issues(&record);
        assert!(issues.iter().any(|i| i.contains("§ 5(b)")));
    }

    #[test]
    fn test_ucc_excluded() {
        let record = ElectronicRecord {
            is_excluded_ucc: true,
            ..ElectronicRecord::default()
        };
        let issues = electronic_record_issues(&record);
        assert!(issues.iter().any(|i| i.contains("§ 3(b)(2)")));
    }

    #[test]
    fn test_multiple_defects() {
        let record = ElectronicRecord {
            parties_agreed_electronic: false,
            relates_to_transaction: false,
            is_testamentary: true,
            is_excluded_ucc: true,
            signature_method: None,
        };
        assert_eq!(electronic_record_issues(&record).len(), 4);
    }

    #[test]
    fn test_signature_methods() {
        assert!(
            SignatureMethod::ClickWrap
                .description()
                .contains("Click-through")
        );
        assert!(
            SignatureMethod::DigitalCryptographic
                .description()
                .contains("digital")
        );
        // Unsigned record is not attributable.
        let record = ElectronicRecord {
            signature_method: None,
            ..ElectronicRecord::default()
        };
        assert!(!signature_attributable(&record));
    }
}
