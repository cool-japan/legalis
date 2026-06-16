//! Uniform Probate Code (UPC) Tracker and Validators
//!
//! The Uniform Probate Code, first promulgated jointly by the Uniform Law
//! Commission and the American Law Institute in **1969** and comprehensively
//! revised in **1990** (with later amendments), standardizes the law of wills,
//! intestate succession, and the administration of decedents' estates.
//!
//! ## Adoption
//!
//! Unlike the UCC, the UPC has *not* been adopted nationwide. As of 2024 it has
//! been enacted in its entirety, or in substantial part, by **18 states** (per
//! the Uniform Law Commission). Many additional states have enacted individual
//! portions, most prominently Article VI (nonprobate transfers on death — the
//! pay-on-death and transfer-on-death rules) and the § 2-503 harmless-error
//! rule. Louisiana, a civil-law jurisdiction, governs successions through its
//! Civil Code (La. Civ. Code art. 870 et seq.) rather than the UPC.

use super::adoption_status::AdoptionStatus;
use super::error::{Result, UniformActError};
use super::model_act::{DraftingBody, ModelActMetadata, US_JURISDICTIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The 18 states that have enacted the UPC in whole or substantial part
/// (per the Uniform Law Commission, as of 2024).
const UPC_ENACTING_STATES: [&str; 18] = [
    "AK", "AZ", "CO", "FL", "HI", "ID", "ME", "MA", "MI", "MN", "MT", "NE", "NJ", "NM", "ND", "SC",
    "SD", "UT",
];

/// Minimum age to make a will (UPC § 2-501: "an individual 18 or more years of
/// age who is of sound mind may make a will").
pub const MINIMUM_TESTATOR_AGE: u8 = 18;

/// Number of attesting witnesses required for an attested will under
/// UPC § 2-502(a)(3)(A).
pub const REQUIRED_WITNESSES: u8 = 2;

/// Returns model-act metadata for the Uniform Probate Code.
#[must_use]
pub fn model_act() -> ModelActMetadata {
    ModelActMetadata::new("UPC", "Uniform Probate Code", DraftingBody::UlcAndAli, 1969)
        .with_revisions([1990, 1991, 1993, 1997, 2008, 2010, 2019])
        .with_summary(
            "Standardizes wills, intestate succession, and administration of decedents' estates.",
        )
}

/// Articles of the Uniform Probate Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpcArticle {
    /// Article I: General Provisions, Definitions, and Probate Jurisdiction.
    GeneralProvisions,
    /// Article II: Intestate Succession and Wills.
    IntestacyAndWills,
    /// Article III: Probate of Wills and Administration.
    ProbateAndAdministration,
    /// Article IV: Foreign Personal Representatives; Ancillary Administration.
    ForeignRepresentatives,
    /// Article V: Protection of Persons Under Disability and Their Property.
    ProtectionOfPersons,
    /// Article VI: Nonprobate Transfers on Death (POD / TOD).
    NonprobateTransfers,
    /// Article VII: Trust Administration (largely superseded by the UTC).
    TrustAdministration,
    /// Article VIII: Effective Dates and Repealer.
    EffectiveDates,
}

impl UpcArticle {
    /// Human-readable name of the article.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::GeneralProvisions => "Article I: General Provisions and Probate Jurisdiction",
            Self::IntestacyAndWills => "Article II: Intestate Succession and Wills",
            Self::ProbateAndAdministration => "Article III: Probate of Wills and Administration",
            Self::ForeignRepresentatives => "Article IV: Foreign Personal Representatives",
            Self::ProtectionOfPersons => "Article V: Protection of Persons Under Disability",
            Self::NonprobateTransfers => "Article VI: Nonprobate Transfers on Death",
            Self::TrustAdministration => "Article VII: Trust Administration",
            Self::EffectiveDates => "Article VIII: Effective Dates and Repealer",
        }
    }

    /// All articles in order.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::GeneralProvisions,
            Self::IntestacyAndWills,
            Self::ProbateAndAdministration,
            Self::ForeignRepresentatives,
            Self::ProtectionOfPersons,
            Self::NonprobateTransfers,
            Self::TrustAdministration,
            Self::EffectiveDates,
        ]
    }
}

/// Key, frequently litigated provisions of the Uniform Probate Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpcSection {
    /// § 2-102 - Intestate share of the surviving spouse.
    ShareOfSpouse,
    /// § 2-202 - Elective share of the surviving spouse.
    ElectiveShare,
    /// § 2-501 - Who may make a will.
    WhoMayMakeWill,
    /// § 2-502 - Execution; witnessed or notarized wills; holographic wills.
    Execution,
    /// § 2-503 - Harmless error (dispensing power).
    HarmlessError,
    /// § 2-603 - Antilapse; deceased devisee; class gifts.
    Antilapse,
    /// § 3-301 - Informal probate and appointment proceedings.
    InformalProbate,
}

impl UpcSection {
    /// Bluebook-style citation for the section.
    #[must_use]
    pub fn citation(&self) -> &'static str {
        match self {
            Self::ShareOfSpouse => "UPC § 2-102",
            Self::ElectiveShare => "UPC § 2-202",
            Self::WhoMayMakeWill => "UPC § 2-501",
            Self::Execution => "UPC § 2-502",
            Self::HarmlessError => "UPC § 2-503",
            Self::Antilapse => "UPC § 2-603",
            Self::InformalProbate => "UPC § 3-301",
        }
    }

    /// Short description of the section's rule.
    #[must_use]
    pub fn summary(&self) -> &'static str {
        match self {
            Self::ShareOfSpouse => {
                "Fixes the surviving spouse's intestate share: the entire estate when all of the \
                 decedent's descendants are also the spouse's and the spouse has no other \
                 descendants, with reduced first-dollar-plus-fraction shares in blended-family \
                 and surviving-parent situations."
            }
            Self::ElectiveShare => {
                "Allows a surviving spouse to elect a share of the augmented estate (a sliding \
                 percentage based on the length of the marriage under the 1990/2008 UPC) instead \
                 of taking under the will."
            }
            Self::WhoMayMakeWill => {
                "An individual 18 or more years of age who is of sound mind may make a will."
            }
            Self::Execution => {
                "An attested will must be in writing, signed by the testator (or in the testator's \
                 name by another at the testator's direction), and either signed by at least two \
                 witnesses or acknowledged before a notary; § 2-502(b) separately validates \
                 holographic wills whose signature and material portions are in the testator's hand."
            }
            Self::HarmlessError => {
                "A document not executed in compliance with § 2-502 may still be treated as a will \
                 if the proponent establishes by clear and convincing evidence that the decedent \
                 intended it to be the will."
            }
            Self::Antilapse => {
                "If a devisee related to the testator predeceases, the devise passes to the \
                 devisee's surviving descendants by representation rather than lapsing."
            }
            Self::InformalProbate => {
                "Permits probate and appointment of a personal representative by a registrar \
                 without formal notice or hearing, enabling streamlined estate administration."
            }
        }
    }
}

/// A state's adoption status for the Uniform Probate Code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpcAdoption {
    /// Two-letter state / jurisdiction code.
    pub state: String,
    /// Adoption status.
    pub status: AdoptionStatus,
    /// Year of enactment, when known.
    pub year_enacted: Option<u16>,
    /// Citation to the enacting state statute, when known.
    pub citation: Option<String>,
    /// Notable state-specific variations.
    pub variations: Vec<String>,
}

impl UpcAdoption {
    /// Create a new adoption record.
    #[must_use]
    pub fn new(state: impl Into<String>, status: AdoptionStatus) -> Self {
        Self {
            state: state.into(),
            status,
            year_enacted: None,
            citation: None,
            variations: vec![],
        }
    }

    /// Set the year of enactment.
    #[must_use]
    pub fn with_year(mut self, year: u16) -> Self {
        self.year_enacted = Some(year);
        self
    }

    /// Set the enacting statute citation.
    #[must_use]
    pub fn with_citation(mut self, citation: impl Into<String>) -> Self {
        self.citation = Some(citation.into());
        self
    }

    /// Add a notable state-specific variation.
    #[must_use]
    pub fn with_variation(mut self, variation: impl Into<String>) -> Self {
        self.variations.push(variation.into());
        self
    }

    /// Whether the jurisdiction has enacted the UPC in whole or substantial part.
    #[must_use]
    pub fn has_adopted(&self) -> bool {
        self.status.is_adopted()
    }
}

/// Tracks Uniform Probate Code adoption across the 51 US jurisdictions.
#[derive(Debug, Clone, Default)]
pub struct UpcTracker {
    adoptions: HashMap<String, UpcAdoption>,
}

impl UpcTracker {
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
            let status = if UPC_ENACTING_STATES.contains(&state) {
                AdoptionStatus::FullyAdopted
            } else {
                AdoptionStatus::NotAdopted
            };
            self.adoptions
                .insert(state.to_string(), UpcAdoption::new(state, status));
        }

        // Louisiana governs successions through its Civil Code, not the UPC.
        self.adoptions.insert(
            "LA".to_string(),
            UpcAdoption::new("LA", AdoptionStatus::CustomLaw)
                .with_citation("La. Civ. Code art. 870 et seq. (Successions)")
                .with_variation("Civil-law successions; UPC not adopted"),
        );

        self.annotate("CO", 1973, "Colo. Rev. Stat. § 15-10-101 et seq.");
        self.annotate("UT", 1975, "Utah Code § 75-1-101 et seq.");
        self.annotate("AZ", 1973, "Ariz. Rev. Stat. § 14-1101 et seq.");
        self.annotate("ME", 1979, "18-C M.R.S. § 1-101 et seq.");
        self.annotate("MN", 1975, "Minn. Stat. ch. 524");
        self.annotate("ND", 1973, "N.D. Cent. Code § 30.1-01-01 et seq.");
    }

    fn annotate(&mut self, state: &str, year: u16, citation: &str) {
        if let Some(record) = self.adoptions.get_mut(state) {
            record.year_enacted = Some(year);
            record.citation = Some(citation.to_string());
        }
    }

    /// Get the adoption record for a jurisdiction.
    #[must_use]
    pub fn get_adoption(&self, state: &str) -> Option<&UpcAdoption> {
        self.adoptions.get(state)
    }

    /// Whether a jurisdiction has enacted the UPC in whole or substantial part.
    #[must_use]
    pub fn has_adopted(&self, state: &str) -> bool {
        self.get_adoption(state)
            .is_some_and(UpcAdoption::has_adopted)
    }

    /// All jurisdictions that have enacted the UPC.
    #[must_use]
    pub fn adopting_states(&self) -> Vec<String> {
        let mut states: Vec<String> = self
            .adoptions
            .values()
            .filter(|a| a.has_adopted())
            .map(|a| a.state.clone())
            .collect();
        states.sort();
        states
    }

    /// Number of jurisdictions that have enacted the UPC.
    #[must_use]
    pub fn adoption_count(&self) -> usize {
        self.adoptions.values().filter(|a| a.has_adopted()).count()
    }

    /// Percentage of the 51 jurisdictions that have enacted the UPC.
    #[must_use]
    pub fn adoption_percentage(&self) -> f64 {
        let total = self.adoptions.len();
        if total == 0 {
            return 0.0;
        }
        (self.adoption_count() as f64 / total as f64) * 100.0
    }

    /// Add or replace an adoption record.
    pub fn add_adoption(&mut self, adoption: UpcAdoption) {
        self.adoptions.insert(adoption.state.clone(), adoption);
    }
}

/// Fact pattern describing the execution of a will, evaluated against
/// UPC §§ 2-501 and 2-502.
#[derive(Debug, Clone)]
pub struct WillExecution {
    /// Whether the will is in writing (§ 2-502(a)(1)).
    pub in_writing: bool,
    /// Testator's age in years (§ 2-501 requires at least 18).
    pub testator_age: u8,
    /// Whether the testator is of sound mind (§ 2-501).
    pub testator_of_sound_mind: bool,
    /// Whether the testator (or another at the testator's direction) signed (§ 2-502(a)(2)).
    pub signed_by_testator: bool,
    /// Number of attesting witnesses (§ 2-502(a)(3)(A) requires at least two).
    pub witness_count: u8,
    /// Whether the will was acknowledged before a notary (§ 2-502(a)(3)(B)).
    pub notarized: bool,
    /// Whether this is offered as a holographic will (§ 2-502(b)).
    pub is_holographic: bool,
    /// For holographic wills, whether the testator's signature is present.
    pub holographic_signature_present: bool,
    /// For holographic wills, whether the material portions are in the
    /// testator's handwriting.
    pub holographic_material_portions_handwritten: bool,
}

impl Default for WillExecution {
    fn default() -> Self {
        // A fully valid, attested will by default, so tests toggle one fault.
        Self {
            in_writing: true,
            testator_age: 40,
            testator_of_sound_mind: true,
            signed_by_testator: true,
            witness_count: REQUIRED_WITNESSES,
            notarized: false,
            is_holographic: false,
            holographic_signature_present: false,
            holographic_material_portions_handwritten: false,
        }
    }
}

/// Returns every UPC will-execution requirement that the fact pattern fails.
/// An empty vector means the will is validly executed.
#[must_use]
pub fn will_execution_issues(facts: &WillExecution) -> Vec<String> {
    let mut issues = Vec::new();

    if facts.testator_age < MINIMUM_TESTATOR_AGE {
        issues.push(format!(
            "testator is under {MINIMUM_TESTATOR_AGE} years of age (§ 2-501)"
        ));
    }
    if !facts.testator_of_sound_mind {
        issues.push("testator is not of sound mind (§ 2-501)".to_string());
    }
    if !facts.in_writing {
        issues.push("will is not in writing (§ 2-502(a)(1))".to_string());
    }

    if facts.is_holographic {
        // § 2-502(b): holographic wills need not be witnessed.
        if !facts.holographic_signature_present {
            issues.push("holographic will is not signed by the testator (§ 2-502(b))".to_string());
        }
        if !facts.holographic_material_portions_handwritten {
            issues.push(
                "material portions of the holographic will are not in the testator's handwriting \
                 (§ 2-502(b))"
                    .to_string(),
            );
        }
    } else {
        if !facts.signed_by_testator {
            issues.push("will is not signed by the testator (§ 2-502(a)(2))".to_string());
        }
        if facts.witness_count < REQUIRED_WITNESSES && !facts.notarized {
            issues.push(format!(
                "attested will has only {} witness(es) and was not notarized; § 2-502(a)(3) \
                 requires at least {REQUIRED_WITNESSES} witnesses or notarization",
                facts.witness_count
            ));
        }
    }

    issues
}

/// Validate that a will is validly executed under UPC §§ 2-501 and 2-502.
///
/// # Errors
///
/// Returns [`UniformActError::WillExecution`] listing every unsatisfied
/// formality.
pub fn validate_will_execution(facts: &WillExecution) -> Result<()> {
    let issues = will_execution_issues(facts);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(UniformActError::WillExecution(issues.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_act_metadata() {
        let meta = model_act();
        assert_eq!(meta.short_name, "UPC");
        assert_eq!(meta.promulgated_year, 1969);
        assert_eq!(meta.drafting_body, DraftingBody::UlcAndAli);
        assert!(meta.revision_years.contains(&1990));
    }

    #[test]
    fn test_articles() {
        assert_eq!(UpcArticle::all().len(), 8);
        assert!(UpcArticle::IntestacyAndWills.name().contains("Article II"));
    }

    #[test]
    fn test_section_citations() {
        assert_eq!(UpcSection::Execution.citation(), "UPC § 2-502");
        assert_eq!(UpcSection::HarmlessError.citation(), "UPC § 2-503");
        assert!(UpcSection::WhoMayMakeWill.summary().contains("18"));
    }

    #[test]
    fn test_tracker_coverage_and_counts() {
        let tracker = UpcTracker::new();
        assert_eq!(tracker.adoptions.len(), 51);
        assert_eq!(tracker.adoption_count(), 18);
        let pct = tracker.adoption_percentage();
        assert!(pct > 30.0 && pct < 40.0, "unexpected pct: {pct}");
    }

    #[test]
    fn test_known_adopters_and_non_adopters() {
        let tracker = UpcTracker::new();
        assert!(tracker.has_adopted("CO"));
        assert!(tracker.has_adopted("UT"));
        assert!(tracker.has_adopted("AZ"));
        // Large common-law states that kept their own probate law.
        assert!(!tracker.has_adopted("CA"));
        assert!(!tracker.has_adopted("NY"));
        assert!(!tracker.has_adopted("TX"));
    }

    #[test]
    fn test_louisiana_custom_successions() {
        let tracker = UpcTracker::new();
        let la = tracker.get_adoption("LA").expect("LA tracked");
        assert_eq!(la.status, AdoptionStatus::CustomLaw);
        assert!(
            la.citation
                .as_ref()
                .expect("LA citation")
                .contains("Successions")
        );
    }

    #[test]
    fn test_named_constants() {
        assert_eq!(MINIMUM_TESTATOR_AGE, 18);
        assert_eq!(REQUIRED_WITNESSES, 2);
    }

    #[test]
    fn test_valid_attested_will() {
        assert!(validate_will_execution(&WillExecution::default()).is_ok());
    }

    #[test]
    fn test_notarized_will_without_witnesses_is_valid() {
        let facts = WillExecution {
            witness_count: 0,
            notarized: true,
            ..WillExecution::default()
        };
        // § 2-502(a)(3)(B) accepts notarization in lieu of two witnesses.
        assert!(validate_will_execution(&facts).is_ok());
    }

    #[test]
    fn test_one_witness_not_notarized_fails() {
        let facts = WillExecution {
            witness_count: 1,
            notarized: false,
            ..WillExecution::default()
        };
        let err = validate_will_execution(&facts).expect_err("should fail");
        assert!(err.to_string().contains("§ 2-502(a)(3)"));
    }

    #[test]
    fn test_underage_testator_fails() {
        let facts = WillExecution {
            testator_age: 16,
            ..WillExecution::default()
        };
        let err = validate_will_execution(&facts).expect_err("should fail");
        assert!(err.to_string().contains("§ 2-501"));
    }

    #[test]
    fn test_valid_holographic_will_needs_no_witnesses() {
        let facts = WillExecution {
            witness_count: 0,
            notarized: false,
            is_holographic: true,
            holographic_signature_present: true,
            holographic_material_portions_handwritten: true,
            ..WillExecution::default()
        };
        assert!(validate_will_execution(&facts).is_ok());
    }

    #[test]
    fn test_holographic_missing_handwriting_fails() {
        let facts = WillExecution {
            is_holographic: true,
            holographic_signature_present: true,
            holographic_material_portions_handwritten: false,
            ..WillExecution::default()
        };
        let issues = will_execution_issues(&facts);
        assert!(issues.iter().any(|i| i.contains("handwriting")));
    }
}
