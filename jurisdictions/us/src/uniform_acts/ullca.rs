//! Uniform Limited Liability Company Act (ULLCA / RULLCA) Tracker and Validators
//!
//! The Uniform Law Commission first promulgated the Uniform Limited Liability
//! Company Act (**ULLCA**) in **1996**. It was not widely adopted, and the ULC
//! replaced it with the Revised Uniform Limited Liability Company Act
//! (**RULLCA**) in **2006** (amended 2011 and 2013, and reorganized into the
//! Harmonized series). RULLCA supplies default and mandatory rules governing the
//! formation, management, fiduciary duties, and dissolution of LLCs.
//!
//! ## Adoption
//!
//! As of 2024, RULLCA has been enacted in roughly **20 jurisdictions** (per the
//! Uniform Law Commission), including several large states such as California
//! (the California Revised Uniform Limited Liability Company Act, Cal. Corp.
//! Code § 17701.01 et seq.) and Florida (Fla. Stat. ch. 605). Delaware — by far
//! the most popular state of LLC formation — retains its own non-uniform
//! Delaware Limited Liability Company Act (6 Del. C. ch. 18).

use super::adoption_status::AdoptionStatus;
use super::error::{Result, UniformActError};
use super::model_act::{DraftingBody, ModelActMetadata, US_JURISDICTIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Jurisdictions that have enacted RULLCA (per the Uniform Law Commission, as
/// of 2024). The precise set continues to evolve as more states enact the
/// Harmonized acts.
const RULLCA_ENACTING_JURISDICTIONS: [&str; 20] = [
    "AL", "AZ", "CA", "CT", "DC", "FL", "ID", "IL", "IA", "MN", "NE", "NJ", "ND", "PA", "SD", "UT",
    "VT", "WA", "WV", "WY",
];

/// Version of the uniform LLC act a jurisdiction has enacted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UllcaVersion {
    /// Original Uniform Limited Liability Company Act (1996).
    Ullca1996,
    /// Revised Uniform Limited Liability Company Act (2006).
    Rullca2006,
    /// Revised Uniform Limited Liability Company Act (2006), as amended in 2013
    /// (the Harmonized version).
    Rullca2013,
    /// A non-uniform, state-specific LLC act (e.g., Delaware).
    NonUniform,
}

impl UllcaVersion {
    /// Human-readable name of the version.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ullca1996 => "Uniform Limited Liability Company Act (1996)",
            Self::Rullca2006 => "Revised Uniform Limited Liability Company Act (2006)",
            Self::Rullca2013 => "Revised Uniform Limited Liability Company Act (2006/2013)",
            Self::NonUniform => "Non-Uniform State LLC Act",
        }
    }

    /// Whether this is a revised (RULLCA) version rather than the 1996 ULLCA or
    /// a non-uniform act.
    #[must_use]
    pub fn is_revised(&self) -> bool {
        matches!(self, Self::Rullca2006 | Self::Rullca2013)
    }
}

/// Default management structure of an LLC (RULLCA § 407(a) makes LLCs
/// member-managed unless the operating agreement provides otherwise).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LlcManagementStructure {
    /// Member-managed (the RULLCA default).
    MemberManaged,
    /// Manager-managed (requires an election in the operating agreement).
    ManagerManaged,
}

/// The default management structure under RULLCA § 407(a).
#[must_use]
pub fn default_management_structure() -> LlcManagementStructure {
    LlcManagementStructure::MemberManaged
}

/// Returns model-act metadata for RULLCA.
#[must_use]
pub fn model_act() -> ModelActMetadata {
    ModelActMetadata::new(
        "RULLCA",
        "Revised Uniform Limited Liability Company Act",
        DraftingBody::UniformLawCommission,
        2006,
    )
    .with_revisions([2011, 2013])
    .with_summary("Default and mandatory rules for LLC formation, management, fiduciary duties, and dissolution.")
}

/// Key, frequently litigated provisions of RULLCA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RullcaSection {
    /// § 105 - Operating Agreement; Scope, Function, and Limitations.
    OperatingAgreement,
    /// § 108 - Name (must contain an LLC designator).
    Name,
    /// § 201 - Formation; Certificate of Organization.
    Formation,
    /// § 304 - Liability of Members and Managers (limited liability shield).
    LimitedLiabilityShield,
    /// § 407 - Management of Limited Liability Company.
    Management,
    /// § 409 - Standards of Conduct for Members and Managers (fiduciary duties).
    StandardsOfConduct,
    /// § 503 - Charging Order (exclusive remedy of a member's creditor).
    ChargingOrder,
    /// § 701 - Events Causing Dissolution.
    Dissolution,
}

impl RullcaSection {
    /// Bluebook-style citation for the section.
    #[must_use]
    pub fn citation(&self) -> &'static str {
        match self {
            Self::OperatingAgreement => "RULLCA § 105",
            Self::Name => "RULLCA § 108",
            Self::Formation => "RULLCA § 201",
            Self::LimitedLiabilityShield => "RULLCA § 304",
            Self::Management => "RULLCA § 407",
            Self::StandardsOfConduct => "RULLCA § 409",
            Self::ChargingOrder => "RULLCA § 503",
            Self::Dissolution => "RULLCA § 701",
        }
    }

    /// Short description of the section's rule.
    #[must_use]
    pub fn summary(&self) -> &'static str {
        match self {
            Self::OperatingAgreement => {
                "The operating agreement governs relations among the members and the company; \
                 § 105(c)-(d) lists the mandatory rules the agreement may not override (e.g., it \
                 may not eliminate the duty of loyalty entirely or unreasonably restrict access to \
                 records)."
            }
            Self::Name => {
                "The name of an LLC must contain the words 'limited liability company' or the \
                 abbreviation 'LLC' or 'L.L.C.', and must be distinguishable on the records of the \
                 filing office."
            }
            Self::Formation => {
                "An LLC is formed when the certificate of organization filed with the filing office \
                 becomes effective; one or more persons may act as organizers."
            }
            Self::LimitedLiabilityShield => {
                "The debts, obligations, and liabilities of an LLC are solely those of the company; \
                 a member or manager is not personally liable merely by reason of being a member or \
                 manager."
            }
            Self::Management => {
                "An LLC is member-managed unless the operating agreement expressly provides that it \
                 is manager-managed; in a member-managed LLC each member has equal rights in \
                 management."
            }
            Self::StandardsOfConduct => {
                "Members of a member-managed LLC (and managers of a manager-managed LLC) owe the \
                 duties of loyalty and care and the contractual obligation of good faith and fair \
                 dealing."
            }
            Self::ChargingOrder => {
                "A charging order against a member's transferable interest is the exclusive remedy \
                 by which a judgment creditor of a member may satisfy the judgment from the \
                 member's interest."
            }
            Self::Dissolution => {
                "An LLC is dissolved upon the occurrence of an event in the operating agreement, the \
                 consent of all members, the passage of 90 consecutive days with no members, or a \
                 judicial decree."
            }
        }
    }
}

/// A jurisdiction's adoption status for the uniform LLC act.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UllcaAdoption {
    /// Two-letter state / jurisdiction code.
    pub state: String,
    /// Adoption status.
    pub status: AdoptionStatus,
    /// Version of the uniform act in force, when applicable.
    pub version: Option<UllcaVersion>,
    /// Year of enactment, when known.
    pub year_enacted: Option<u16>,
    /// Citation to the enacting state statute, when known.
    pub citation: Option<String>,
}

impl UllcaAdoption {
    /// Create a new adoption record.
    #[must_use]
    pub fn new(state: impl Into<String>, status: AdoptionStatus) -> Self {
        Self {
            state: state.into(),
            status,
            version: None,
            year_enacted: None,
            citation: None,
        }
    }

    /// Set the version of the uniform act in force.
    #[must_use]
    pub fn with_version(mut self, version: UllcaVersion) -> Self {
        self.version = Some(version);
        self
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

    /// Whether the jurisdiction has enacted RULLCA.
    #[must_use]
    pub fn has_rullca(&self) -> bool {
        self.version.is_some_and(|v| v.is_revised())
    }
}

/// Tracks uniform LLC act adoption across the 51 US jurisdictions.
#[derive(Debug, Clone, Default)]
pub struct UllcaTracker {
    adoptions: HashMap<String, UllcaAdoption>,
}

impl UllcaTracker {
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
            if RULLCA_ENACTING_JURISDICTIONS.contains(&state) {
                self.adoptions.insert(
                    state.to_string(),
                    UllcaAdoption::new(state, AdoptionStatus::AdoptedWithVariations)
                        .with_version(UllcaVersion::Rullca2013),
                );
            } else {
                // Non-RULLCA jurisdictions operate under their own LLC statutes.
                self.adoptions.insert(
                    state.to_string(),
                    UllcaAdoption::new(state, AdoptionStatus::CustomLaw)
                        .with_version(UllcaVersion::NonUniform),
                );
            }
        }

        // Notable annotations.
        self.annotate("CA", 2014, "Cal. Corp. Code § 17701.01 et seq.");
        self.annotate("FL", 2015, "Fla. Stat. ch. 605");
        self.annotate("PA", 2017, "15 Pa.C.S. ch. 88");
        self.annotate("WA", 2016, "Wash. Rev. Code ch. 25.15");

        // Delaware is the dominant formation state but is non-uniform.
        if let Some(de) = self.adoptions.get_mut("DE") {
            de.citation = Some("6 Del. C. ch. 18 (Delaware LLC Act)".to_string());
        }
    }

    fn annotate(&mut self, state: &str, year: u16, citation: &str) {
        if let Some(record) = self.adoptions.get_mut(state) {
            record.year_enacted = Some(year);
            record.citation = Some(citation.to_string());
        }
    }

    /// Get the adoption record for a jurisdiction.
    #[must_use]
    pub fn get_adoption(&self, state: &str) -> Option<&UllcaAdoption> {
        self.adoptions.get(state)
    }

    /// Whether a jurisdiction has enacted RULLCA.
    #[must_use]
    pub fn has_rullca(&self, state: &str) -> bool {
        self.get_adoption(state)
            .is_some_and(UllcaAdoption::has_rullca)
    }

    /// All jurisdictions that have enacted RULLCA.
    #[must_use]
    pub fn rullca_states(&self) -> Vec<String> {
        let mut states: Vec<String> = self
            .adoptions
            .values()
            .filter(|a| a.has_rullca())
            .map(|a| a.state.clone())
            .collect();
        states.sort();
        states
    }

    /// Number of jurisdictions that have enacted RULLCA.
    #[must_use]
    pub fn rullca_count(&self) -> usize {
        self.adoptions.values().filter(|a| a.has_rullca()).count()
    }

    /// Percentage of the 51 jurisdictions that have enacted RULLCA.
    #[must_use]
    pub fn rullca_percentage(&self) -> f64 {
        let total = self.adoptions.len();
        if total == 0 {
            return 0.0;
        }
        (self.rullca_count() as f64 / total as f64) * 100.0
    }

    /// Add or replace an adoption record.
    pub fn add_adoption(&mut self, adoption: UllcaAdoption) {
        self.adoptions.insert(adoption.state.clone(), adoption);
    }
}

/// Fact pattern describing the formation of an LLC, evaluated against
/// RULLCA §§ 108, 113, and 201.
#[derive(Debug, Clone)]
pub struct LlcFormation {
    /// Whether the name contains a required designator such as "LLC" (§ 108(a)).
    pub name_contains_required_designator: bool,
    /// Whether the name is distinguishable on the records of the filing office
    /// (§ 108(c)).
    pub name_distinguishable: bool,
    /// Whether a certificate of organization has been filed (§ 201).
    pub certificate_of_organization_filed: bool,
    /// Whether the LLC has designated a registered agent (§ 113).
    pub has_registered_agent: bool,
}

impl Default for LlcFormation {
    fn default() -> Self {
        // A validly formed LLC by default, so tests toggle one fault.
        Self {
            name_contains_required_designator: true,
            name_distinguishable: true,
            certificate_of_organization_filed: true,
            has_registered_agent: true,
        }
    }
}

/// Returns every RULLCA formation requirement that the fact pattern fails. An
/// empty vector means the LLC was validly formed.
#[must_use]
pub fn llc_formation_issues(facts: &LlcFormation) -> Vec<String> {
    let mut issues = Vec::new();

    if !facts.name_contains_required_designator {
        issues.push(
            "name does not contain 'limited liability company', 'LLC', or 'L.L.C.' (§ 108(a))"
                .to_string(),
        );
    }
    if !facts.name_distinguishable {
        issues.push(
            "name is not distinguishable on the records of the filing office (§ 108(c))"
                .to_string(),
        );
    }
    if !facts.certificate_of_organization_filed {
        issues.push("certificate of organization has not been filed (§ 201)".to_string());
    }
    if !facts.has_registered_agent {
        issues.push("no registered agent has been designated (§ 113)".to_string());
    }

    issues
}

/// Validate that an LLC was validly formed under RULLCA §§ 108, 113, and 201.
///
/// # Errors
///
/// Returns [`UniformActError::LlcFormation`] listing every unsatisfied
/// requirement.
pub fn validate_llc_formation(facts: &LlcFormation) -> Result<()> {
    let issues = llc_formation_issues(facts);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(UniformActError::LlcFormation(issues.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_act_metadata() {
        let meta = model_act();
        assert_eq!(meta.short_name, "RULLCA");
        assert_eq!(meta.promulgated_year, 2006);
        assert_eq!(meta.latest_version_year(), 2013);
    }

    #[test]
    fn test_version_helpers() {
        assert!(UllcaVersion::Rullca2006.is_revised());
        assert!(UllcaVersion::Rullca2013.is_revised());
        assert!(!UllcaVersion::Ullca1996.is_revised());
        assert!(!UllcaVersion::NonUniform.is_revised());
    }

    #[test]
    fn test_default_management_is_member_managed() {
        assert_eq!(
            default_management_structure(),
            LlcManagementStructure::MemberManaged
        );
    }

    #[test]
    fn test_section_citations() {
        assert_eq!(RullcaSection::Formation.citation(), "RULLCA § 201");
        assert_eq!(RullcaSection::ChargingOrder.citation(), "RULLCA § 503");
        assert!(
            RullcaSection::LimitedLiabilityShield
                .summary()
                .contains("not personally liable")
        );
    }

    #[test]
    fn test_tracker_coverage_and_counts() {
        let tracker = UllcaTracker::new();
        assert_eq!(tracker.adoptions.len(), 51);
        assert_eq!(tracker.rullca_count(), 20);
        let pct = tracker.rullca_percentage();
        assert!(pct > 35.0 && pct < 45.0, "unexpected pct: {pct}");
    }

    #[test]
    fn test_known_rullca_states() {
        let tracker = UllcaTracker::new();
        assert!(tracker.has_rullca("CA"));
        assert!(tracker.has_rullca("FL"));
        assert!(tracker.has_rullca("PA"));
        assert!(tracker.has_rullca("DC"));
    }

    #[test]
    fn test_delaware_is_non_uniform() {
        let tracker = UllcaTracker::new();
        let de = tracker.get_adoption("DE").expect("DE tracked");
        assert!(!de.has_rullca());
        assert_eq!(de.version, Some(UllcaVersion::NonUniform));
        assert!(
            de.citation
                .as_ref()
                .expect("DE citation")
                .contains("Delaware LLC Act")
        );
    }

    #[test]
    fn test_california_annotation() {
        let tracker = UllcaTracker::new();
        let ca = tracker.get_adoption("CA").expect("CA tracked");
        assert_eq!(ca.year_enacted, Some(2014));
        assert!(ca.citation.as_ref().expect("CA citation").contains("17701"));
    }

    #[test]
    fn test_valid_formation() {
        assert!(validate_llc_formation(&LlcFormation::default()).is_ok());
        assert!(llc_formation_issues(&LlcFormation::default()).is_empty());
    }

    #[test]
    fn test_missing_designator_fails() {
        let facts = LlcFormation {
            name_contains_required_designator: false,
            ..LlcFormation::default()
        };
        let err = validate_llc_formation(&facts).expect_err("should fail");
        assert!(err.to_string().contains("§ 108"));
    }

    #[test]
    fn test_no_certificate_fails() {
        let facts = LlcFormation {
            certificate_of_organization_filed: false,
            ..LlcFormation::default()
        };
        let err = validate_llc_formation(&facts).expect_err("should fail");
        assert!(err.to_string().contains("§ 201"));
    }

    #[test]
    fn test_multiple_formation_failures() {
        let facts = LlcFormation {
            name_contains_required_designator: false,
            name_distinguishable: false,
            certificate_of_organization_filed: false,
            has_registered_agent: false,
        };
        assert_eq!(llc_formation_issues(&facts).len(), 4);
    }
}
