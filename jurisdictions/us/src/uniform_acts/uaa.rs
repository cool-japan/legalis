//! Uniform Arbitration Act (UAA / RUAA) Tracker and Validators
//!
//! The Uniform Arbitration Act (**UAA**) was promulgated by the Uniform Law
//! Commission in **1955** and became one of the most successful uniform acts,
//! enacted in 49 jurisdictions. The ULC comprehensively revised it as the
//! Revised Uniform Arbitration Act (**RUAA**) in **2000** to address modern
//! issues such as arbitrator disclosure, provisional remedies, and
//! consolidation.
//!
//! ## Relationship to the Federal Arbitration Act
//!
//! State arbitration law operates against the backdrop of the Federal
//! Arbitration Act (FAA, 9 U.S.C. §§ 1-16). Under *Southland Corp. v. Keating*,
//! 465 U.S. 1 (1984), the FAA's substantive rule of arbitrability preempts
//! conflicting state law in contracts involving interstate commerce. The UAA /
//! RUAA principally supply the *procedural* framework (appointment of
//! arbitrators, provisional remedies, confirmation, vacatur) for arbitrations
//! seated in the enacting state.
//!
//! ## Adoption
//!
//! As of 2024, RUAA (2000) has been enacted in roughly **23 jurisdictions**
//! (per the Uniform Law Commission). Most other states remain on the 1955 UAA.
//! A few large states never adopted the uniform act and use their own arbitration
//! statutes — notably California (Cal. Civ. Proc. Code §§ 1280-1294.2) and New
//! York (N.Y. C.P.L.R. Article 75).

use super::adoption_status::AdoptionStatus;
use super::error::{Result, UniformActError};
use super::model_act::{DraftingBody, ModelActMetadata, US_JURISDICTIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Jurisdictions that have enacted the Revised Uniform Arbitration Act (2000)
/// (per the Uniform Law Commission, as of 2024).
const RUAA_ENACTING_JURISDICTIONS: [&str; 23] = [
    "AK", "AZ", "AR", "CO", "DC", "FL", "GA", "HI", "ID", "KS", "MA", "MI", "MN", "NV", "NJ", "NM",
    "NC", "ND", "OK", "OR", "UT", "WA", "WV",
];

/// States that use their own non-uniform arbitration statutes (never adopted the
/// UAA / RUAA).
const NON_UNIFORM_ARBITRATION_STATES: [&str; 2] = ["CA", "NY"];

/// Version of the uniform arbitration act in force in a jurisdiction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArbitrationActVersion {
    /// Original Uniform Arbitration Act (1955).
    Uaa1955,
    /// Revised Uniform Arbitration Act (2000).
    Ruaa2000,
    /// A non-uniform, state-specific arbitration statute (e.g., California, New York).
    NonUniform,
}

impl ArbitrationActVersion {
    /// Human-readable name of the version.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Uaa1955 => "Uniform Arbitration Act (1955)",
            Self::Ruaa2000 => "Revised Uniform Arbitration Act (2000)",
            Self::NonUniform => "Non-Uniform State Arbitration Statute",
        }
    }

    /// Whether this is the revised (2000) act.
    #[must_use]
    pub fn is_revised(&self) -> bool {
        matches!(self, Self::Ruaa2000)
    }
}

/// Returns model-act metadata for the Revised Uniform Arbitration Act.
#[must_use]
pub fn model_act() -> ModelActMetadata {
    ModelActMetadata::new(
        "RUAA",
        "Revised Uniform Arbitration Act",
        DraftingBody::UniformLawCommission,
        2000,
    )
    .with_summary("Procedural framework for arbitration: agreements, arbitrators, awards, vacatur and confirmation.")
}

/// Returns model-act metadata for the original Uniform Arbitration Act (1955).
#[must_use]
pub fn original_uaa_metadata() -> ModelActMetadata {
    ModelActMetadata::new(
        "UAA",
        "Uniform Arbitration Act",
        DraftingBody::UniformLawCommission,
        1955,
    )
    .with_revisions([1956])
    .with_summary("Original uniform arbitration act, enacted in 49 jurisdictions.")
}

/// Key provisions of the Revised Uniform Arbitration Act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuaaSection {
    /// § 4 - Effect of Agreement to Arbitrate; Nonwaivable Provisions.
    NonwaivableProvisions,
    /// § 6 - Validity of Agreement to Arbitrate.
    Validity,
    /// § 8 - Provisional Remedies.
    ProvisionalRemedies,
    /// § 12 - Disclosure by Arbitrator.
    ArbitratorDisclosure,
    /// § 21 - Remedies; Fees and Expenses of Arbitration Proceeding.
    Remedies,
    /// § 23 - Vacating an Award.
    VacatingAward,
    /// § 25 - Confirmation of Award; Judgment.
    ConfirmationAndJudgment,
}

impl RuaaSection {
    /// Bluebook-style citation for the section.
    #[must_use]
    pub fn citation(&self) -> &'static str {
        match self {
            Self::NonwaivableProvisions => "RUAA § 4",
            Self::Validity => "RUAA § 6",
            Self::ProvisionalRemedies => "RUAA § 8",
            Self::ArbitratorDisclosure => "RUAA § 12",
            Self::Remedies => "RUAA § 21",
            Self::VacatingAward => "RUAA § 23",
            Self::ConfirmationAndJudgment => "RUAA § 25",
        }
    }

    /// Short description of the section's rule.
    #[must_use]
    pub fn summary(&self) -> &'static str {
        match self {
            Self::NonwaivableProvisions => {
                "Before a controversy arises, parties may not waive or vary the rights to judicial \
                 relief and to vacate an award under §§ 8, 17(a), 17(b), 23, and others; these are \
                 the mandatory core of the act."
            }
            Self::Validity => {
                "An agreement contained in a record to submit to arbitration is valid, enforceable, \
                 and irrevocable except upon a ground that exists at law or in equity for the \
                 revocation of a contract; the court decides whether an agreement exists."
            }
            Self::ProvisionalRemedies => {
                "Before an arbitrator is appointed, a court may grant provisional remedies; \
                 thereafter the arbitrator may do so to protect the effectiveness of the award."
            }
            Self::ArbitratorDisclosure => {
                "An arbitrator must, before accepting appointment, disclose known facts that a \
                 reasonable person would consider likely to affect impartiality, including financial \
                 or personal interests and existing relationships; nondisclosure can support \
                 vacatur for evident partiality."
            }
            Self::Remedies => {
                "An arbitrator may award punitive damages or other exemplary relief if authorized \
                 by law in a civil action, and may award reasonable attorney's fees and other costs."
            }
            Self::VacatingAward => {
                "A court shall vacate an award only on the limited statutory grounds in § 23(a) \
                 (corruption, fraud, evident partiality, arbitrator misconduct, exceeding powers, \
                 no agreement, or improper notice)."
            }
            Self::ConfirmationAndJudgment => {
                "After a party moves to confirm, the court shall confirm the award unless it is \
                 modified, corrected, or vacated, and shall enter judgment on the confirmed award."
            }
        }
    }
}

/// Statutory grounds on which a court must vacate an arbitration award under
/// RUAA § 23(a). These grounds are exclusive; courts may not review the merits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VacaturGround {
    /// § 23(a)(1) - Award procured by corruption, fraud, or other undue means.
    ProcuredByCorruptionOrFraud,
    /// § 23(a)(2) - Evident partiality, corruption, or prejudicial misconduct by an arbitrator.
    ArbitratorPartialityOrMisconduct,
    /// § 23(a)(3) - Arbitrator refused to postpone, refused material evidence, or otherwise
    /// conducted the hearing contrary to § 15, prejudicing a party.
    RefusalToHearEvidence,
    /// § 23(a)(4) - Arbitrator exceeded the arbitrator's powers.
    ArbitratorExceededPowers,
    /// § 23(a)(5) - No agreement to arbitrate (and the objection was preserved).
    NoArbitrationAgreement,
    /// § 23(a)(6) - Arbitration conducted without proper notice of initiation under § 9.
    ImproperNotice,
}

impl VacaturGround {
    /// Citation for the specific § 23(a) ground.
    #[must_use]
    pub fn citation(&self) -> &'static str {
        match self {
            Self::ProcuredByCorruptionOrFraud => "RUAA § 23(a)(1)",
            Self::ArbitratorPartialityOrMisconduct => "RUAA § 23(a)(2)",
            Self::RefusalToHearEvidence => "RUAA § 23(a)(3)",
            Self::ArbitratorExceededPowers => "RUAA § 23(a)(4)",
            Self::NoArbitrationAgreement => "RUAA § 23(a)(5)",
            Self::ImproperNotice => "RUAA § 23(a)(6)",
        }
    }

    /// All statutory vacatur grounds.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::ProcuredByCorruptionOrFraud,
            Self::ArbitratorPartialityOrMisconduct,
            Self::RefusalToHearEvidence,
            Self::ArbitratorExceededPowers,
            Self::NoArbitrationAgreement,
            Self::ImproperNotice,
        ]
    }
}

/// A jurisdiction's adoption status for the uniform arbitration act.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UaaAdoption {
    /// Two-letter state / jurisdiction code.
    pub state: String,
    /// Adoption status.
    pub status: AdoptionStatus,
    /// Version of the arbitration act in force.
    pub version: ArbitrationActVersion,
    /// Citation to the state statute, when known.
    pub citation: Option<String>,
}

impl UaaAdoption {
    /// Create a new adoption record.
    #[must_use]
    pub fn new(state: impl Into<String>, version: ArbitrationActVersion) -> Self {
        let status = match version {
            ArbitrationActVersion::Uaa1955 => AdoptionStatus::AdoptedWithVariations,
            ArbitrationActVersion::Ruaa2000 => AdoptionStatus::FullyAdopted,
            ArbitrationActVersion::NonUniform => AdoptionStatus::CustomLaw,
        };
        Self {
            state: state.into(),
            status,
            version,
            citation: None,
        }
    }

    /// Set the state statute citation.
    #[must_use]
    pub fn with_citation(mut self, citation: impl Into<String>) -> Self {
        self.citation = Some(citation.into());
        self
    }

    /// Whether the jurisdiction has enacted the revised (2000) act.
    #[must_use]
    pub fn has_ruaa(&self) -> bool {
        self.version.is_revised()
    }
}

/// Tracks uniform arbitration act adoption across the 51 US jurisdictions.
#[derive(Debug, Clone, Default)]
pub struct UaaTracker {
    adoptions: HashMap<String, UaaAdoption>,
}

impl UaaTracker {
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
            let version = if RUAA_ENACTING_JURISDICTIONS.contains(&state) {
                ArbitrationActVersion::Ruaa2000
            } else if NON_UNIFORM_ARBITRATION_STATES.contains(&state) {
                ArbitrationActVersion::NonUniform
            } else {
                // Historical baseline: the 1955 UAA was enacted in 49 jurisdictions.
                ArbitrationActVersion::Uaa1955
            };
            self.adoptions
                .insert(state.to_string(), UaaAdoption::new(state, version));
        }

        self.annotate("CA", "Cal. Civ. Proc. Code §§ 1280-1294.2");
        self.annotate("NY", "N.Y. C.P.L.R. Article 75");
        self.annotate(
            "FL",
            "Fla. Stat. ch. 682 (Revised Florida Arbitration Code)",
        );
        self.annotate("WA", "Wash. Rev. Code ch. 7.04A");
        self.annotate("NV", "Nev. Rev. Stat. §§ 38.206-38.248");
    }

    fn annotate(&mut self, state: &str, citation: &str) {
        if let Some(record) = self.adoptions.get_mut(state) {
            record.citation = Some(citation.to_string());
        }
    }

    /// Get the adoption record for a jurisdiction.
    #[must_use]
    pub fn get_adoption(&self, state: &str) -> Option<&UaaAdoption> {
        self.adoptions.get(state)
    }

    /// Version of the arbitration act in force in a jurisdiction.
    #[must_use]
    pub fn state_version(&self, state: &str) -> Option<ArbitrationActVersion> {
        self.get_adoption(state).map(|a| a.version)
    }

    /// Whether a jurisdiction has enacted RUAA (2000).
    #[must_use]
    pub fn has_ruaa(&self, state: &str) -> bool {
        self.get_adoption(state).is_some_and(UaaAdoption::has_ruaa)
    }

    /// All jurisdictions that have enacted RUAA (2000).
    #[must_use]
    pub fn ruaa_states(&self) -> Vec<String> {
        let mut states: Vec<String> = self
            .adoptions
            .values()
            .filter(|a| a.has_ruaa())
            .map(|a| a.state.clone())
            .collect();
        states.sort();
        states
    }

    /// Number of jurisdictions that have enacted RUAA (2000).
    #[must_use]
    pub fn ruaa_count(&self) -> usize {
        self.adoptions.values().filter(|a| a.has_ruaa()).count()
    }

    /// Percentage of the 51 jurisdictions that have enacted RUAA (2000).
    #[must_use]
    pub fn ruaa_percentage(&self) -> f64 {
        let total = self.adoptions.len();
        if total == 0 {
            return 0.0;
        }
        (self.ruaa_count() as f64 / total as f64) * 100.0
    }

    /// Add or replace an adoption record.
    pub fn add_adoption(&mut self, adoption: UaaAdoption) {
        self.adoptions.insert(adoption.state.clone(), adoption);
    }
}

/// Fact pattern describing an agreement to arbitrate, evaluated against
/// RUAA § 6.
#[derive(Debug, Clone)]
pub struct ArbitrationAgreement {
    /// Whether the agreement is contained in a record (§ 6(a)).
    pub in_record: bool,
    /// Whether the parties mutually assented to a contract to arbitrate (§ 6(a)).
    pub mutual_assent: bool,
    /// Whether the subject matter is one that may be submitted to arbitration.
    pub subject_matter_arbitrable: bool,
    /// Whether the agreement is unconscionable — a ground at law or in equity
    /// for revocation of a contract (§ 6(a)).
    pub unconscionable: bool,
}

impl Default for ArbitrationAgreement {
    fn default() -> Self {
        // A valid, enforceable arbitration agreement by default.
        Self {
            in_record: true,
            mutual_assent: true,
            subject_matter_arbitrable: true,
            unconscionable: false,
        }
    }
}

/// Returns every RUAA § 6 requirement that the fact pattern fails. An empty
/// vector means the agreement is valid and enforceable.
#[must_use]
pub fn arbitration_agreement_issues(agreement: &ArbitrationAgreement) -> Vec<String> {
    let mut issues = Vec::new();

    if !agreement.in_record {
        issues.push("agreement to arbitrate is not contained in a record (§ 6(a))".to_string());
    }
    if !agreement.mutual_assent {
        issues.push("no mutual assent to a contract to arbitrate (§ 6(a))".to_string());
    }
    if !agreement.subject_matter_arbitrable {
        issues.push("the subject matter is not arbitrable (§ 6)".to_string());
    }
    if agreement.unconscionable {
        issues.push(
            "agreement is unconscionable — a ground at law or in equity for revocation of a \
             contract (§ 6(a))"
                .to_string(),
        );
    }

    issues
}

/// Validate that an arbitration agreement is valid and enforceable under
/// RUAA § 6.
///
/// # Errors
///
/// Returns [`UniformActError::ArbitrationAgreement`] listing every defect.
pub fn validate_arbitration_agreement(agreement: &ArbitrationAgreement) -> Result<()> {
    let issues = arbitration_agreement_issues(agreement);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(UniformActError::ArbitrationAgreement(issues.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_act_metadata() {
        let ruaa = model_act();
        assert_eq!(ruaa.short_name, "RUAA");
        assert_eq!(ruaa.promulgated_year, 2000);

        let uaa = original_uaa_metadata();
        assert_eq!(uaa.promulgated_year, 1955);
    }

    #[test]
    fn test_version_helpers() {
        assert!(ArbitrationActVersion::Ruaa2000.is_revised());
        assert!(!ArbitrationActVersion::Uaa1955.is_revised());
        assert!(!ArbitrationActVersion::NonUniform.is_revised());
    }

    #[test]
    fn test_section_citations() {
        assert_eq!(RuaaSection::Validity.citation(), "RUAA § 6");
        assert_eq!(RuaaSection::VacatingAward.citation(), "RUAA § 23");
        assert!(RuaaSection::Remedies.summary().contains("punitive"));
    }

    #[test]
    fn test_vacatur_grounds() {
        assert_eq!(VacaturGround::all().len(), 6);
        assert_eq!(
            VacaturGround::ArbitratorExceededPowers.citation(),
            "RUAA § 23(a)(4)"
        );
    }

    #[test]
    fn test_tracker_full_coverage() {
        let tracker = UaaTracker::new();
        assert_eq!(tracker.adoptions.len(), 51);
    }

    #[test]
    fn test_ruaa_counts() {
        let tracker = UaaTracker::new();
        assert_eq!(tracker.ruaa_count(), 23);
        let pct = tracker.ruaa_percentage();
        assert!(pct > 40.0 && pct < 50.0, "unexpected pct: {pct}");
    }

    #[test]
    fn test_version_classification() {
        let tracker = UaaTracker::new();
        assert_eq!(
            tracker.state_version("FL"),
            Some(ArbitrationActVersion::Ruaa2000)
        );
        assert_eq!(
            tracker.state_version("CA"),
            Some(ArbitrationActVersion::NonUniform)
        );
        assert_eq!(
            tracker.state_version("NY"),
            Some(ArbitrationActVersion::NonUniform)
        );
        // A non-RUAA, non-CA/NY state defaults to the historical 1955 UAA.
        assert_eq!(
            tracker.state_version("TX"),
            Some(ArbitrationActVersion::Uaa1955)
        );
    }

    #[test]
    fn test_non_uniform_citations() {
        let tracker = UaaTracker::new();
        let ca = tracker.get_adoption("CA").expect("CA tracked");
        assert_eq!(ca.version, ArbitrationActVersion::NonUniform);
        assert!(ca.citation.as_ref().expect("CA citation").contains("1280"));
    }

    #[test]
    fn test_valid_arbitration_agreement() {
        assert!(validate_arbitration_agreement(&ArbitrationAgreement::default()).is_ok());
    }

    #[test]
    fn test_not_in_record_fails() {
        let agreement = ArbitrationAgreement {
            in_record: false,
            ..ArbitrationAgreement::default()
        };
        let err = validate_arbitration_agreement(&agreement).expect_err("should fail");
        assert!(err.to_string().contains("§ 6"));
    }

    #[test]
    fn test_unconscionable_fails() {
        let agreement = ArbitrationAgreement {
            unconscionable: true,
            ..ArbitrationAgreement::default()
        };
        let issues = arbitration_agreement_issues(&agreement);
        assert!(issues.iter().any(|i| i.contains("unconscionable")));
    }

    #[test]
    fn test_multiple_defects() {
        let agreement = ArbitrationAgreement {
            in_record: false,
            mutual_assent: false,
            subject_matter_arbitrable: false,
            unconscionable: true,
        };
        assert_eq!(arbitration_agreement_issues(&agreement).len(), 4);
    }
}
