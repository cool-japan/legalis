//! Uniform Trust Code (UTC) Tracker and Validators
//!
//! The Uniform Trust Code, promulgated by the Uniform Law Commission in **2000**
//! (with amendments in 2001, 2003, 2004, 2005, and 2010), is the first national
//! codification of the law of trusts. Before the UTC, trust law was almost
//! entirely common law supplemented by scattered statutes; the UTC gathers the
//! default and mandatory rules of trust administration into a single code.
//!
//! ## Structure
//!
//! The UTC is organized into eleven articles, from general provisions and the
//! creation of trusts through the duties and powers of trustees and liability.
//!
//! ## Adoption
//!
//! As of 2024 the UTC has been enacted in **34 states and the District of
//! Columbia** (per the Uniform Law Commission). Several large trust
//! jurisdictions retain their own codes (for example, California's Probate Code,
//! New York's EPTL, Delaware's trust law, and the Texas Trust Code), and
//! Louisiana governs trusts through its own Louisiana Trust Code
//! (La. R.S. 9:1721 et seq.).

use super::adoption_status::AdoptionStatus;
use super::error::{Result, UniformActError};
use super::model_act::{DraftingBody, ModelActMetadata, US_JURISDICTIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The 34 states plus the District of Columbia that have enacted the UTC
/// (per the Uniform Law Commission, as of 2024).
const UTC_ENACTING_JURISDICTIONS: [&str; 35] = [
    "AL", "AZ", "AR", "CO", "CT", "DC", "FL", "IL", "KS", "KY", "ME", "MD", "MA", "MI", "MN", "MS",
    "MO", "MT", "NE", "NH", "NJ", "NM", "NC", "ND", "OH", "OR", "PA", "SC", "TN", "UT", "VT", "VA",
    "WV", "WI", "WY",
];

/// Returns model-act metadata for the Uniform Trust Code.
#[must_use]
pub fn model_act() -> ModelActMetadata {
    ModelActMetadata::new(
        "UTC",
        "Uniform Trust Code",
        DraftingBody::UniformLawCommission,
        2000,
    )
    .with_revisions([2001, 2003, 2004, 2005, 2010])
    .with_summary("First national codification of the law of trusts (creation, administration, trustee duties).")
}

/// Articles of the Uniform Trust Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UtcArticle {
    /// Article 1: General Provisions and Definitions (§§ 101-112).
    GeneralProvisions,
    /// Article 2: Judicial Proceedings (§§ 201-204).
    JudicialProceedings,
    /// Article 3: Representation (§§ 301-305).
    Representation,
    /// Article 4: Creation, Validity, Modification, and Termination (§§ 401-417).
    CreationAndModification,
    /// Article 5: Creditor's Claims; Spendthrift and Discretionary Trusts (§§ 501-507).
    CreditorClaims,
    /// Article 6: Revocable Trusts (§§ 601-604).
    RevocableTrusts,
    /// Article 7: Office of Trustee (§§ 701-709).
    OfficeOfTrustee,
    /// Article 8: Duties and Powers of Trustee (§§ 801-817).
    DutiesAndPowers,
    /// Article 9: Uniform Prudent Investor Act (incorporated, optional).
    PrudentInvestor,
    /// Article 10: Liability of Trustees; Rights of Persons Dealing With Trustee (§§ 1001-1013).
    Liability,
    /// Article 11: Miscellaneous Provisions.
    Miscellaneous,
}

impl UtcArticle {
    /// Human-readable name of the article.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::GeneralProvisions => "Article 1: General Provisions and Definitions",
            Self::JudicialProceedings => "Article 2: Judicial Proceedings",
            Self::Representation => "Article 3: Representation",
            Self::CreationAndModification => {
                "Article 4: Creation, Validity, Modification, and Termination"
            }
            Self::CreditorClaims => "Article 5: Creditor's Claims; Spendthrift Trusts",
            Self::RevocableTrusts => "Article 6: Revocable Trusts",
            Self::OfficeOfTrustee => "Article 7: Office of Trustee",
            Self::DutiesAndPowers => "Article 8: Duties and Powers of Trustee",
            Self::PrudentInvestor => "Article 9: Uniform Prudent Investor Act",
            Self::Liability => "Article 10: Liability of Trustees",
            Self::Miscellaneous => "Article 11: Miscellaneous Provisions",
        }
    }

    /// All articles in order.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::GeneralProvisions,
            Self::JudicialProceedings,
            Self::Representation,
            Self::CreationAndModification,
            Self::CreditorClaims,
            Self::RevocableTrusts,
            Self::OfficeOfTrustee,
            Self::DutiesAndPowers,
            Self::PrudentInvestor,
            Self::Liability,
            Self::Miscellaneous,
        ]
    }
}

/// Key, frequently litigated provisions of the Uniform Trust Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UtcSection {
    /// § 105 - Default and Mandatory Rules (terms of trust prevail except for
    /// the mandatory rules listed in § 105(b)).
    DefaultAndMandatoryRules,
    /// § 402 - Requirements for Creation of a Trust.
    RequirementsForCreation,
    /// § 404 - Trust Purposes (lawful, not contrary to public policy, possible).
    TrustPurposes,
    /// § 411 - Modification or Termination by Consent.
    ModificationByConsent,
    /// § 412 - Modification or Termination Because of Unanticipated Circumstances.
    ModificationUnanticipated,
    /// § 502 - Spendthrift Provision.
    SpendthriftProvision,
    /// § 801 - Duty to Administer Trust.
    DutyToAdminister,
    /// § 802 - Duty of Loyalty.
    DutyOfLoyalty,
    /// § 804 - Prudent Administration.
    PrudentAdministration,
    /// § 813 - Duty to Inform and Report.
    DutyToInformAndReport,
}

impl UtcSection {
    /// Bluebook-style citation for the section.
    #[must_use]
    pub fn citation(&self) -> &'static str {
        match self {
            Self::DefaultAndMandatoryRules => "UTC § 105",
            Self::RequirementsForCreation => "UTC § 402",
            Self::TrustPurposes => "UTC § 404",
            Self::ModificationByConsent => "UTC § 411",
            Self::ModificationUnanticipated => "UTC § 412",
            Self::SpendthriftProvision => "UTC § 502",
            Self::DutyToAdminister => "UTC § 801",
            Self::DutyOfLoyalty => "UTC § 802",
            Self::PrudentAdministration => "UTC § 804",
            Self::DutyToInformAndReport => "UTC § 813",
        }
    }

    /// Short description of the section's rule.
    #[must_use]
    pub fn summary(&self) -> &'static str {
        match self {
            Self::DefaultAndMandatoryRules => {
                "The terms of a trust prevail over the Code except for the mandatory rules \
                 enumerated in § 105(b) (e.g., the requirements for creation, a trustee's duty \
                 to act in good faith, and the court's power to modify or terminate a trust)."
            }
            Self::RequirementsForCreation => {
                "A trust is created only if the settlor has capacity and intent, there is a \
                 definite beneficiary (or a charitable, animal, or noncharitable purpose trust), \
                 the trustee has duties, and the sole trustee is not the sole beneficiary."
            }
            Self::TrustPurposes => {
                "A trust may be created only to the extent its purposes are lawful, not contrary \
                 to public policy, and possible to achieve."
            }
            Self::ModificationByConsent => {
                "A noncharitable irrevocable trust may be modified or terminated on consent of \
                 the settlor and all beneficiaries, or by all beneficiaries if consistent with a \
                 material purpose of the trust."
            }
            Self::ModificationUnanticipated => {
                "A court may modify the administrative or dispositive terms of a trust if, because \
                 of circumstances not anticipated by the settlor, modification will further the \
                 purposes of the trust."
            }
            Self::SpendthriftProvision => {
                "A spendthrift provision is valid only if it restrains both voluntary and \
                 involuntary transfer of a beneficiary's interest."
            }
            Self::DutyToAdminister => {
                "A trustee shall administer the trust in good faith, in accordance with its terms \
                 and purposes and the interests of the beneficiaries."
            }
            Self::DutyOfLoyalty => {
                "A trustee shall administer the trust solely in the interests of the \
                 beneficiaries; conflicted transactions are voidable (the no-further-inquiry rule)."
            }
            Self::PrudentAdministration => {
                "A trustee shall administer the trust as a prudent person would, exercising \
                 reasonable care, skill, and caution."
            }
            Self::DutyToInformAndReport => {
                "A trustee shall keep qualified beneficiaries reasonably informed and, on request, \
                 furnish a report of trust property, liabilities, receipts, and disbursements."
            }
        }
    }
}

/// A state's adoption status for the Uniform Trust Code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtcAdoption {
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

impl UtcAdoption {
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

    /// Whether the jurisdiction has enacted the UTC in some form.
    #[must_use]
    pub fn has_adopted(&self) -> bool {
        self.status.is_adopted()
    }
}

/// Tracks Uniform Trust Code adoption across the 51 US jurisdictions.
#[derive(Debug, Clone, Default)]
pub struct UtcTracker {
    adoptions: HashMap<String, UtcAdoption>,
}

impl UtcTracker {
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
            let status = if UTC_ENACTING_JURISDICTIONS.contains(&state) {
                AdoptionStatus::FullyAdopted
            } else {
                AdoptionStatus::NotAdopted
            };
            self.adoptions
                .insert(state.to_string(), UtcAdoption::new(state, status));
        }

        // Louisiana governs trusts through its own civil-law trust code.
        self.adoptions.insert(
            "LA".to_string(),
            UtcAdoption::new("LA", AdoptionStatus::CustomLaw)
                .with_citation("La. R.S. 9:1721 et seq. (Louisiana Trust Code)")
                .with_variation("Civil-law trust code; UTC not adopted"),
        );

        // Citations for several representative enacting jurisdictions.
        self.annotate("AL", 2006, "Ala. Code § 19-3B-101 et seq.");
        self.annotate("AZ", 2008, "Ariz. Rev. Stat. § 14-10101 et seq.");
        self.annotate("FL", 2006, "Fla. Stat. ch. 736");
        self.annotate("OH", 2006, "Ohio Rev. Code ch. 5801 et seq.");
        self.annotate("PA", 2006, "20 Pa.C.S. ch. 77");
        self.annotate("VA", 2005, "Va. Code § 64.2-700 et seq.");
        self.annotate("DC", 2004, "D.C. Code § 19-1301.01 et seq.");
    }

    fn annotate(&mut self, state: &str, year: u16, citation: &str) {
        if let Some(record) = self.adoptions.get_mut(state) {
            record.year_enacted = Some(year);
            record.citation = Some(citation.to_string());
        }
    }

    /// Get the adoption record for a jurisdiction.
    #[must_use]
    pub fn get_adoption(&self, state: &str) -> Option<&UtcAdoption> {
        self.adoptions.get(state)
    }

    /// Whether a jurisdiction has enacted the UTC.
    #[must_use]
    pub fn has_adopted(&self, state: &str) -> bool {
        self.get_adoption(state)
            .is_some_and(UtcAdoption::has_adopted)
    }

    /// All jurisdictions that have enacted the UTC.
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

    /// Number of jurisdictions that have enacted the UTC.
    #[must_use]
    pub fn adoption_count(&self) -> usize {
        self.adoptions.values().filter(|a| a.has_adopted()).count()
    }

    /// Percentage of the 51 jurisdictions that have enacted the UTC.
    #[must_use]
    pub fn adoption_percentage(&self) -> f64 {
        let total = self.adoptions.len();
        if total == 0 {
            return 0.0;
        }
        (self.adoption_count() as f64 / total as f64) * 100.0
    }

    /// Add or replace an adoption record.
    pub fn add_adoption(&mut self, adoption: UtcAdoption) {
        self.adoptions.insert(adoption.state.clone(), adoption);
    }
}

/// Fact pattern describing an attempt to create a trust, evaluated against
/// Uniform Trust Code § 402.
#[derive(Debug, Clone, Default)]
pub struct TrustCreation {
    /// Whether the settlor had capacity to create a trust (§ 402(a)(1)).
    pub settlor_has_capacity: bool,
    /// Whether the settlor indicated an intention to create a trust (§ 402(a)(2)).
    pub settlor_indicated_intent: bool,
    /// Whether the trust has a definite beneficiary (§ 402(a)(3)).
    pub has_definite_beneficiary: bool,
    /// Whether the trust is a charitable trust (§ 402(a)(3)(A)).
    pub is_charitable_trust: bool,
    /// Whether the trust is for the care of an animal or another noncharitable
    /// purpose (§§ 402(a)(3)(B)-(C), 408-409).
    pub is_animal_or_purpose_trust: bool,
    /// Whether the trustee has duties to perform (§ 402(a)(4)).
    pub trustee_has_duties: bool,
    /// Whether the same person is the sole trustee and the sole beneficiary
    /// (§ 402(a)(5)); if so, legal and equitable title merge.
    pub sole_trustee_is_sole_beneficiary: bool,
}

/// Returns every § 402 requirement that the given fact pattern fails, in
/// section order. An empty vector means a valid trust was created.
#[must_use]
pub fn trust_creation_issues(facts: &TrustCreation) -> Vec<String> {
    let mut issues = Vec::new();

    if !facts.settlor_has_capacity {
        issues.push("settlor lacks capacity to create a trust (§ 402(a)(1))".to_string());
    }
    if !facts.settlor_indicated_intent {
        issues.push(
            "settlor did not indicate an intention to create a trust (§ 402(a)(2))".to_string(),
        );
    }
    if !(facts.has_definite_beneficiary
        || facts.is_charitable_trust
        || facts.is_animal_or_purpose_trust)
    {
        issues.push(
            "trust has no definite beneficiary and is not a charitable, animal, or noncharitable \
             purpose trust (§ 402(a)(3))"
                .to_string(),
        );
    }
    if !facts.trustee_has_duties {
        issues.push("trustee has no duties to perform (§ 402(a)(4))".to_string());
    }
    if facts.sole_trustee_is_sole_beneficiary {
        issues.push(
            "the same person is the sole trustee and the sole beneficiary; legal and equitable \
             title merge (§ 402(a)(5))"
                .to_string(),
        );
    }

    issues
}

/// Validate that a fact pattern creates a valid trust under UTC § 402.
///
/// # Errors
///
/// Returns [`UniformActError::TrustCreation`] listing every unsatisfied
/// requirement of § 402(a).
pub fn validate_trust_creation(facts: &TrustCreation) -> Result<()> {
    let issues = trust_creation_issues(facts);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(UniformActError::TrustCreation(issues.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_trust() -> TrustCreation {
        TrustCreation {
            settlor_has_capacity: true,
            settlor_indicated_intent: true,
            has_definite_beneficiary: true,
            is_charitable_trust: false,
            is_animal_or_purpose_trust: false,
            trustee_has_duties: true,
            sole_trustee_is_sole_beneficiary: false,
        }
    }

    #[test]
    fn test_model_act_metadata() {
        let meta = model_act();
        assert_eq!(meta.short_name, "UTC");
        assert_eq!(meta.promulgated_year, 2000);
        assert_eq!(meta.latest_version_year(), 2010);
        assert_eq!(meta.drafting_body, DraftingBody::UniformLawCommission);
    }

    #[test]
    fn test_articles() {
        assert_eq!(UtcArticle::all().len(), 11);
        assert!(
            UtcArticle::CreationAndModification
                .name()
                .contains("Article 4")
        );
    }

    #[test]
    fn test_section_citations_and_summaries() {
        assert_eq!(UtcSection::RequirementsForCreation.citation(), "UTC § 402");
        assert_eq!(UtcSection::DutyOfLoyalty.citation(), "UTC § 802");
        assert!(
            UtcSection::SpendthriftProvision
                .summary()
                .contains("involuntary")
        );
    }

    #[test]
    fn test_tracker_jurisdiction_coverage() {
        let tracker = UtcTracker::new();
        // All 51 jurisdictions are tracked.
        assert_eq!(tracker.adoptions.len(), 51);
    }

    #[test]
    fn test_tracker_adoption_status() {
        let tracker = UtcTracker::new();
        // Enacting jurisdictions.
        assert!(tracker.has_adopted("FL"));
        assert!(tracker.has_adopted("OH"));
        assert!(tracker.has_adopted("DC"));
        // Non-enacting trust jurisdictions with their own codes.
        assert!(!tracker.has_adopted("CA"));
        assert!(!tracker.has_adopted("NY"));
        assert!(!tracker.has_adopted("DE"));
        assert!(!tracker.has_adopted("TX"));
    }

    #[test]
    fn test_louisiana_custom_trust_code() {
        let tracker = UtcTracker::new();
        let la = tracker.get_adoption("LA").expect("LA tracked");
        assert_eq!(la.status, AdoptionStatus::CustomLaw);
        assert!(!la.has_adopted());
        assert!(
            la.citation
                .as_ref()
                .expect("LA citation")
                .contains("Louisiana Trust Code")
        );
    }

    #[test]
    fn test_adoption_count_and_percentage() {
        let tracker = UtcTracker::new();
        assert_eq!(tracker.adoption_count(), 35);
        let pct = tracker.adoption_percentage();
        assert!(pct > 60.0 && pct < 75.0, "unexpected pct: {pct}");
    }

    #[test]
    fn test_adopting_states_sorted_and_complete() {
        let tracker = UtcTracker::new();
        let states = tracker.adopting_states();
        assert_eq!(states.len(), 35);
        // Sorted ascending.
        let mut sorted = states.clone();
        sorted.sort();
        assert_eq!(states, sorted);
    }

    #[test]
    fn test_annotated_citations() {
        let tracker = UtcTracker::new();
        let fl = tracker.get_adoption("FL").expect("FL tracked");
        assert_eq!(fl.year_enacted, Some(2006));
        assert!(fl.citation.as_ref().expect("FL citation").contains("736"));
    }

    #[test]
    fn test_valid_trust_creation() {
        assert!(validate_trust_creation(&valid_trust()).is_ok());
        assert!(trust_creation_issues(&valid_trust()).is_empty());
    }

    #[test]
    fn test_no_capacity_fails() {
        let mut facts = valid_trust();
        facts.settlor_has_capacity = false;
        let err = validate_trust_creation(&facts).expect_err("should fail");
        assert!(err.to_string().contains("capacity"));
        assert!(err.to_string().contains("§ 402"));
    }

    #[test]
    fn test_no_definite_beneficiary_unless_charitable() {
        let mut facts = valid_trust();
        facts.has_definite_beneficiary = false;
        // Without a charitable / purpose designation, creation fails.
        assert!(validate_trust_creation(&facts).is_err());
        // A charitable trust cures the missing definite beneficiary.
        facts.is_charitable_trust = true;
        assert!(validate_trust_creation(&facts).is_ok());
    }

    #[test]
    fn test_merger_of_title_fails() {
        let mut facts = valid_trust();
        facts.sole_trustee_is_sole_beneficiary = true;
        let issues = trust_creation_issues(&facts);
        assert!(issues.iter().any(|i| i.contains("§ 402(a)(5)")));
    }

    #[test]
    fn test_multiple_failures_reported_together() {
        let facts = TrustCreation::default();
        let issues = trust_creation_issues(&facts);
        // capacity, intent, beneficiary, trustee duties = 4 failures.
        assert_eq!(issues.len(), 4);
    }
}
