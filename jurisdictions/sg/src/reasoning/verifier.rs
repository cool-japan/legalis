//! Legal statute verification for Singapore law.
//!
//! This module provides verification capabilities for Singapore legal statutes,
//! checking compliance with the Constitution of Singapore and key principles.
//!
//! # Singapore Legal Framework
//!
//! - Constitution of the Republic of Singapore (supreme law)
//! - Acts of Parliament
//! - Subsidiary Legislation
//! - Common Law principles
//!
//! # Key Principles
//!
//! - **Rule of Law**: Government bound by law
//! - **Separation of Powers**: Legislature, Executive, Judiciary
//! - **Multi-racialism**: Equality regardless of race or religion
//! - **Fundamental Liberties**: Part IV of Constitution
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_sg::reasoning::verifier::{SgStatuteVerifier, sg_constitutional_principles};
//! use legalis_sg::reasoning::statute_adapter::all_singapore_statutes;
//!
//! let verifier = SgStatuteVerifier::new();
//! let statutes = all_singapore_statutes();
//!
//! let result = verifier.verify(&statutes);
//! println!("Verification passed: {}", result.passed);
//! ```

use legalis_core::Statute;
use legalis_verifier::{
    ConstitutionalPrinciple, PrincipleCheck, Severity, StatuteVerifier, VerificationError,
    VerificationResult,
};
use std::collections::{HashMap, HashSet};

/// Singapore statute verifier with Constitutional compliance checking.
pub struct SgStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// Singapore-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for SgStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SgStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// Singapore legal source hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SgLegalSource {
    /// Constitution of the Republic of Singapore
    Constitution,
    /// Act of Parliament
    ActOfParliament,
    /// Subsidiary Legislation
    SubsidiaryLegislation,
    /// Common Law
    CommonLaw,
    /// Guidelines (non-binding)
    Guidelines,
}

impl SgLegalSource {
    /// Returns the legal source name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Constitution => "Constitution",
            Self::ActOfParliament => "Act of Parliament",
            Self::SubsidiaryLegislation => "Subsidiary Legislation",
            Self::CommonLaw => "Common Law",
            Self::Guidelines => "Guidelines",
        }
    }

    /// Determines legal source from statute ID or title.
    #[must_use]
    pub fn from_statute(statute: &Statute) -> Self {
        let id = statute.id.to_lowercase();
        let title = statute.title.to_lowercase();

        if id.contains("const_") || title.contains("constitution") {
            return Self::Constitution;
        }

        if id.contains("reg_") || title.contains("regulation") || title.contains("rules") {
            return Self::SubsidiaryLegislation;
        }

        if title.contains("guideline") || title.contains("guidance") {
            return Self::Guidelines;
        }

        Self::ActOfParliament
    }

    /// Checks if this level is higher than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Checks if this is binding law.
    #[must_use]
    pub fn is_binding(&self) -> bool {
        !matches!(self, Self::Guidelines)
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies
    known_hierarchies: HashMap<String, SgLegalSource>,
    /// Employment-related statutes
    employment_statutes: HashSet<String>,
    /// PDPA-related statutes
    pdpa_statutes: HashSet<String>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: SgLegalSource) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Marks a statute as employment-related.
    pub fn register_employment_statute(&mut self, statute_id: impl Into<String>) {
        self.employment_statutes.insert(statute_id.into());
    }

    /// Marks a statute as PDPA-related.
    pub fn register_pdpa_statute(&mut self, statute_id: impl Into<String>) {
        self.pdpa_statutes.insert(statute_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> SgLegalSource {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| SgLegalSource::from_statute(statute))
    }

    /// Checks if a statute is employment-related.
    #[must_use]
    pub fn is_employment_statute(&self, statute_id: &str) -> bool {
        self.employment_statutes.contains(statute_id) || statute_id.starts_with("EA_")
    }

    /// Checks if a statute is PDPA-related.
    #[must_use]
    pub fn is_pdpa_statute(&self, statute_id: &str) -> bool {
        self.pdpa_statutes.contains(statute_id) || statute_id.contains("PDPA")
    }
}

impl Default for SgStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl SgStatuteVerifier {
    /// Creates a new Singapore statute verifier with constitutional principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = sg_constitutional_principles();
        let inner = StatuteVerifier::with_principles(principles);

        let mut hierarchy_rules = HierarchyRules::new();
        setup_sg_hierarchy(&mut hierarchy_rules);

        Self {
            inner,
            hierarchy_rules,
        }
    }

    /// Verifies a set of statutes with Singapore-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_employment_consistency(statutes));
        result.merge(self.check_pdpa_consistency(statutes));

        result
    }

    /// Verifies a single statute.
    #[must_use]
    pub fn verify_single(&self, statute: &Statute) -> VerificationResult {
        self.verify(std::slice::from_ref(statute))
    }

    fn check_hierarchy_compliance(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                let level_i = self.hierarchy_rules.get_hierarchy(&statutes[i]);
                let level_j = self.hierarchy_rules.get_hierarchy(&statutes[j]);

                if level_i != level_j && self.effects_conflict(&statutes[i], &statutes[j]) {
                    result = result.with_warning(format!(
                        "SG hierarchy issue: {} vs {}",
                        statutes[i].id, statutes[j].id
                    ));
                }
            }
        }

        result
    }

    fn check_employment_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let ea_count = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.is_employment_statute(&s.id))
            .count();

        if ea_count > 1 {
            result = result.with_suggestion(format!(
                "Employment Act consistency check: {} provisions found",
                ea_count
            ));
        }

        result
    }

    fn check_pdpa_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let pdpa_count = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.is_pdpa_statute(&s.id))
            .count();

        if pdpa_count > 1 {
            result = result.with_suggestion(format!(
                "PDPA consistency check: {} data protection provisions found",
                pdpa_count
            ));
        }

        result
    }

    fn effects_conflict(&self, statute1: &Statute, statute2: &Statute) -> bool {
        use legalis_core::EffectType;

        matches!(
            (&statute1.effect.effect_type, &statute2.effect.effect_type),
            (EffectType::Grant, EffectType::Revoke)
                | (EffectType::Revoke, EffectType::Grant)
                | (EffectType::Obligation, EffectType::Prohibition)
                | (EffectType::Prohibition, EffectType::Obligation)
        )
    }
}

/// Creates constitutional principles based on the Constitution of Singapore.
#[must_use]
pub fn sg_constitutional_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        ConstitutionalPrinciple {
            id: "sg-art9-liberty".to_string(),
            name: "Liberty of the Person (Art. 9)".to_string(),
            description: "No person shall be deprived of his life or personal liberty save in accordance with law.".to_string(),
            check: PrincipleCheck::RequiresProcedure,
        },
        ConstitutionalPrinciple {
            id: "sg-art12-equality".to_string(),
            name: "Equal Protection (Art. 12)".to_string(),
            description: "All persons are equal before the law and entitled to the equal protection of the law.".to_string(),
            check: PrincipleCheck::EqualProtection,
        },
        ConstitutionalPrinciple {
            id: "sg-art14-speech".to_string(),
            name: "Freedom of Speech (Art. 14)".to_string(),
            description: "Every citizen of Singapore has the right to freedom of speech and expression.".to_string(),
            check: PrincipleCheck::FreedomOfExpression,
        },
        ConstitutionalPrinciple {
            id: "sg-art15-religion".to_string(),
            name: "Freedom of Religion (Art. 15)".to_string(),
            description: "Every person has the right to profess and practise his religion.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Religious freedom protection".to_string(),
            },
        },
        ConstitutionalPrinciple {
            id: "sg-art16-education".to_string(),
            name: "Rights in respect of education (Art. 16)".to_string(),
            description: "No discrimination against any citizen on the grounds only of religion, race, descent or place of birth in any law relating to educational institutions.".to_string(),
            check: PrincipleCheck::NoDiscrimination,
        },
    ]
}

/// Sets up Singapore hierarchy relationships.
fn setup_sg_hierarchy(rules: &mut HierarchyRules) {
    // Employment Act
    rules.register_hierarchy("EA_s13", SgLegalSource::ActOfParliament);
    rules.register_hierarchy("EA_s14", SgLegalSource::ActOfParliament);
    rules.register_employment_statute("EA_s13");
    rules.register_employment_statute("EA_s14");

    // PDPA
    rules.register_hierarchy("PDPA_Art13", SgLegalSource::ActOfParliament);
    rules.register_pdpa_statute("PDPA_Art13");

    // Companies Act
    rules.register_hierarchy("CA_s145", SgLegalSource::ActOfParliament);

    // Banking Act
    rules.register_hierarchy("BA_s7", SgLegalSource::ActOfParliament);
}

/// Verification report for Singapore law compliance.
#[derive(Debug, Clone)]
pub struct SgVerificationReport {
    pub result: VerificationResult,
    pub constitutional_issues: Vec<ConstitutionalIssue>,
}

/// Constitutional compliance issue.
#[derive(Debug, Clone)]
pub struct ConstitutionalIssue {
    pub statute_id: String,
    pub article: String,
    pub description: String,
    pub severity: Severity,
}

impl SgVerificationReport {
    #[must_use]
    pub fn new(result: VerificationResult) -> Self {
        Self {
            result,
            constitutional_issues: Vec::new(),
        }
    }

    #[must_use]
    pub fn passed(&self) -> bool {
        self.result.passed && self.constitutional_issues.is_empty()
    }

    #[must_use]
    pub fn critical_issues(&self) -> Vec<&VerificationError> {
        self.result.errors_by_severity(Severity::Critical)
    }

    #[must_use]
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        if self.passed() {
            summary.push_str("Verification Result: PASSED\n");
        } else {
            summary.push_str("Verification Result: FAILED\n");
        }
        summary.push_str(&format!("Errors: {}\n", self.result.errors.len()));
        summary.push_str(&format!("Warnings: {}\n", self.result.warnings.len()));
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_verifier_creation() {
        let verifier = SgStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = SgStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = SgStatuteVerifier::new();
        let statute = Statute::new(
            "TEST_001",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = verifier.verify_single(&statute);
        assert!(result.passed);
    }

    #[test]
    fn test_legal_source_ordering() {
        assert!(SgLegalSource::Constitution.is_higher_than(&SgLegalSource::ActOfParliament));
        assert!(
            SgLegalSource::ActOfParliament.is_higher_than(&SgLegalSource::SubsidiaryLegislation)
        );
    }

    #[test]
    fn test_sg_constitutional_principles() {
        let principles = sg_constitutional_principles();
        assert!(!principles.is_empty());

        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"sg-art9-liberty"));
        assert!(principle_ids.contains(&"sg-art12-equality"));
    }

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = SgVerificationReport::new(result);
        assert!(report.passed());
    }
}
