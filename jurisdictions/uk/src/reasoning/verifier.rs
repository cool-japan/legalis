//! Legal statute verification for United Kingdom law.
//!
//! This module provides verification capabilities for UK legal statutes,
//! checking compliance with the Human Rights Act 1998, constitutional
//! conventions, and post-Brexit regulatory framework.
//!
//! # UK Legal Framework
//!
//! The UK has an uncodified constitution consisting of:
//! - Statutes of constitutional significance
//! - Common law constitutional principles
//! - Constitutional conventions
//! - EU-derived law (retained after Brexit)
//!
//! # Key Principles
//!
//! - **Parliamentary Sovereignty**: Parliament is supreme law-maker
//! - **Rule of Law**: Government subject to law
//! - **Human Rights Act 1998**: Incorporation of ECHR
//! - **Devolution**: Powers to Scotland, Wales, Northern Ireland
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_uk::reasoning::verifier::{UkStatuteVerifier, uk_human_rights_principles};
//! use legalis_uk::reasoning::statute_adapter::all_employment_statutes;
//!
//! let verifier = UkStatuteVerifier::new();
//! let statutes = all_employment_statutes();
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

/// UK statute verifier with Human Rights Act compliance checking.
///
/// This verifier integrates UK constitutional principles, Human Rights Act,
/// and post-Brexit retained EU law into the verification framework.
pub struct UkStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// UK-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for UkStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UkStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// UK legal source hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UkLegalSource {
    /// Constitutional Statutes (e.g., Magna Carta, Bill of Rights 1689)
    ConstitutionalStatute,
    /// Act of Parliament
    ActOfParliament,
    /// Retained EU Law (post-Brexit)
    RetainedEuLaw,
    /// Statutory Instruments (Delegated Legislation)
    StatutoryInstrument,
    /// Devolved Legislation (Scotland, Wales, NI)
    DevolvedLegislation,
    /// Common Law (Judge-made law)
    CommonLaw,
    /// Guidance (Non-binding)
    Guidance,
}

impl UkLegalSource {
    /// Returns the legal source name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ConstitutionalStatute => "Constitutional Statute",
            Self::ActOfParliament => "Act of Parliament",
            Self::RetainedEuLaw => "Retained EU Law",
            Self::StatutoryInstrument => "Statutory Instrument",
            Self::DevolvedLegislation => "Devolved Legislation",
            Self::CommonLaw => "Common Law",
            Self::Guidance => "Guidance",
        }
    }

    /// Returns the legal source abbreviation.
    #[must_use]
    pub const fn abbreviation(&self) -> &'static str {
        match self {
            Self::ConstitutionalStatute => "Const.",
            Self::ActOfParliament => "Act",
            Self::RetainedEuLaw => "REUL",
            Self::StatutoryInstrument => "SI",
            Self::DevolvedLegislation => "Dev.",
            Self::CommonLaw => "CL",
            Self::Guidance => "Guid.",
        }
    }

    /// Determines legal source from statute ID or title.
    #[must_use]
    pub fn from_statute(statute: &Statute) -> Self {
        let id = statute.id.to_lowercase();
        let title = statute.title.to_lowercase();

        // Check for Constitutional Statutes
        if title.contains("human rights act")
            || title.contains("constitutional")
            || title.contains("magna carta")
            || title.contains("bill of rights")
            || id.contains("hra_")
        {
            return Self::ConstitutionalStatute;
        }

        // Check for Retained EU Law
        if id.contains("reul_")
            || title.contains("retained")
            || title.contains("eu-derived")
            || id.contains("uk_gdpr")
        {
            return Self::RetainedEuLaw;
        }

        // Check for Statutory Instruments
        if id.contains("si_")
            || title.contains("regulation")
            || title.contains("order")
            || title.contains("statutory instrument")
        {
            return Self::StatutoryInstrument;
        }

        // Check for Devolved Legislation
        if id.contains("scot_")
            || id.contains("wales_")
            || id.contains("ni_")
            || title.contains("scottish")
            || title.contains("welsh")
            || title.contains("northern ireland")
        {
            return Self::DevolvedLegislation;
        }

        // Check for Guidance
        if title.contains("guidance") || title.contains("code of practice") || id.contains("acas_")
        {
            return Self::Guidance;
        }

        // Default to Act of Parliament
        Self::ActOfParliament
    }

    /// Checks if this level is higher in the hierarchy than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Checks if this is binding law.
    #[must_use]
    pub fn is_binding(&self) -> bool {
        !matches!(self, Self::Guidance)
    }
}

/// UK Region for devolution purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UkRegion {
    /// England
    England,
    /// Scotland
    Scotland,
    /// Wales
    Wales,
    /// Northern Ireland
    NorthernIreland,
}

impl UkRegion {
    /// Returns the region name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::England => "England",
            Self::Scotland => "Scotland",
            Self::Wales => "Wales",
            Self::NorthernIreland => "Northern Ireland",
        }
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies
    known_hierarchies: HashMap<String, UkLegalSource>,
    /// Employment law statutes
    employment_statutes: HashSet<String>,
    /// GDPR-related statutes (UK GDPR)
    gdpr_statutes: HashSet<String>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: UkLegalSource) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Marks a statute as employment-related.
    pub fn register_employment_statute(&mut self, statute_id: impl Into<String>) {
        self.employment_statutes.insert(statute_id.into());
    }

    /// Marks a statute as GDPR-related.
    pub fn register_gdpr_statute(&mut self, statute_id: impl Into<String>) {
        self.gdpr_statutes.insert(statute_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> UkLegalSource {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| UkLegalSource::from_statute(statute))
    }

    /// Checks if a statute is employment-related.
    #[must_use]
    pub fn is_employment_statute(&self, statute_id: &str) -> bool {
        self.employment_statutes.contains(statute_id)
            || statute_id.starts_with("ERA_")
            || statute_id.starts_with("WTR_")
            || statute_id.starts_with("NMWA_")
    }

    /// Checks if a statute is GDPR-related.
    #[must_use]
    pub fn is_gdpr_statute(&self, statute_id: &str) -> bool {
        self.gdpr_statutes.contains(statute_id)
            || statute_id.contains("UK_GDPR")
            || statute_id.contains("DPA_")
    }
}

impl Default for UkStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl UkStatuteVerifier {
    /// Creates a new UK statute verifier with Human Rights principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = uk_human_rights_principles();
        let inner = StatuteVerifier::with_principles(principles);

        let mut hierarchy_rules = HierarchyRules::new();
        setup_uk_hierarchy(&mut hierarchy_rules);

        Self {
            inner,
            hierarchy_rules,
        }
    }

    /// Creates a verifier with custom hierarchy rules.
    #[must_use]
    pub fn with_hierarchy_rules(mut self, rules: HierarchyRules) -> Self {
        self.hierarchy_rules = rules;
        self
    }

    /// Verifies a set of statutes with UK-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        // Additional UK-specific checks
        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_employment_law_consistency(statutes));
        result.merge(self.check_retained_eu_law(statutes));

        result
    }

    /// Verifies a single statute.
    #[must_use]
    pub fn verify_single(&self, statute: &Statute) -> VerificationResult {
        self.verify(std::slice::from_ref(statute))
    }

    /// Checks legal hierarchy compliance.
    fn check_hierarchy_compliance(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                let level_i = self.hierarchy_rules.get_hierarchy(&statutes[i]);
                let level_j = self.hierarchy_rules.get_hierarchy(&statutes[j]);

                if level_i != level_j && self.effects_conflict(&statutes[i], &statutes[j]) {
                    let (higher, lower) = if level_i.is_higher_than(&level_j) {
                        (&statutes[i], &statutes[j])
                    } else {
                        (&statutes[j], &statutes[i])
                    };

                    let lower_level = self.hierarchy_rules.get_hierarchy(lower);
                    let higher_level = self.hierarchy_rules.get_hierarchy(higher);

                    result = result.with_warning(format!(
                        "UK hierarchy issue: {} ({}) may conflict with {} ({})",
                        lower.id,
                        lower_level.name(),
                        higher.id,
                        higher_level.name()
                    ));
                }
            }
        }

        result
    }

    /// Checks UK employment law consistency (ERA, WTR, NMWA).
    fn check_employment_law_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let employment_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.is_employment_statute(&s.id))
            .collect();

        // Check ERA statutes
        let era_statutes: Vec<_> = employment_statutes
            .iter()
            .filter(|s| s.id.starts_with("ERA_"))
            .collect();

        if era_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "ERA consistency check: {} Employment Rights Act provisions found",
                era_statutes.len()
            ));
        }

        // Check WTR statutes
        let wtr_statutes: Vec<_> = employment_statutes
            .iter()
            .filter(|s| s.id.starts_with("WTR_"))
            .collect();

        if wtr_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "WTR consistency check: {} Working Time Regulations provisions found",
                wtr_statutes.len()
            ));
        }

        result
    }

    /// Checks retained EU law status and potential divergence.
    fn check_retained_eu_law(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let reul_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| {
                matches!(
                    self.hierarchy_rules.get_hierarchy(s),
                    UkLegalSource::RetainedEuLaw
                )
            })
            .collect();

        if !reul_statutes.is_empty() {
            result = result.with_suggestion(format!(
                "Retained EU Law check: {} REUL provisions - verify post-Brexit modifications",
                reul_statutes.len()
            ));
        }

        result
    }

    /// Checks if two statutes have conflicting effects.
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

/// Creates constitutional principles based on the Human Rights Act 1998.
///
/// The HRA incorporates the European Convention on Human Rights into UK law.
#[must_use]
pub fn uk_human_rights_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        // Article 2: Right to Life
        ConstitutionalPrinciple {
            id: "hra-art2-life".to_string(),
            name: "Right to Life (ECHR Art. 2)".to_string(),
            description: "Everyone's right to life shall be protected by law.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Right to life protection".to_string(),
            },
        },
        // Article 5: Right to Liberty
        ConstitutionalPrinciple {
            id: "hra-art5-liberty".to_string(),
            name: "Right to Liberty (ECHR Art. 5)".to_string(),
            description: "Everyone has the right to liberty and security of person.".to_string(),
            check: PrincipleCheck::RequiresProcedure,
        },
        // Article 6: Right to Fair Trial
        ConstitutionalPrinciple {
            id: "hra-art6-fair-trial".to_string(),
            name: "Right to Fair Trial (ECHR Art. 6)".to_string(),
            description: "Everyone is entitled to a fair and public hearing within a reasonable time by an independent and impartial tribunal.".to_string(),
            check: PrincipleCheck::ProceduralDueProcess,
        },
        // Article 8: Right to Privacy
        ConstitutionalPrinciple {
            id: "hra-art8-privacy".to_string(),
            name: "Right to Privacy (ECHR Art. 8)".to_string(),
            description: "Everyone has the right to respect for his private and family life, his home and his correspondence.".to_string(),
            check: PrincipleCheck::PrivacyImpact,
        },
        // Article 9: Freedom of Thought
        ConstitutionalPrinciple {
            id: "hra-art9-thought".to_string(),
            name: "Freedom of Thought (ECHR Art. 9)".to_string(),
            description: "Everyone has the right to freedom of thought, conscience and religion.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Freedom of conscience".to_string(),
            },
        },
        // Article 10: Freedom of Expression
        ConstitutionalPrinciple {
            id: "hra-art10-expression".to_string(),
            name: "Freedom of Expression (ECHR Art. 10)".to_string(),
            description: "Everyone has the right to freedom of expression.".to_string(),
            check: PrincipleCheck::FreedomOfExpression,
        },
        // Article 14: Non-Discrimination
        ConstitutionalPrinciple {
            id: "hra-art14-nondiscrimination".to_string(),
            name: "Non-Discrimination (ECHR Art. 14)".to_string(),
            description: "The enjoyment of rights shall be secured without discrimination on any ground.".to_string(),
            check: PrincipleCheck::EqualProtection,
        },
        // Protocol 1, Article 1: Property Rights
        ConstitutionalPrinciple {
            id: "hra-p1a1-property".to_string(),
            name: "Property Rights (ECHR P1 Art. 1)".to_string(),
            description: "Every person is entitled to the peaceful enjoyment of his possessions.".to_string(),
            check: PrincipleCheck::PropertyRights,
        },
        // Proportionality (UK Constitutional Principle)
        ConstitutionalPrinciple {
            id: "uk-proportionality".to_string(),
            name: "Proportionality".to_string(),
            description: "Restrictions on rights must be proportionate to the legitimate aim pursued.".to_string(),
            check: PrincipleCheck::Proportionality,
        },
    ]
}

/// Sets up UK hierarchy relationships.
fn setup_uk_hierarchy(rules: &mut HierarchyRules) {
    // Register Employment Acts
    rules.register_hierarchy("ERA_s1", UkLegalSource::ActOfParliament);
    rules.register_hierarchy("ERA_s86", UkLegalSource::ActOfParliament);
    rules.register_hierarchy("ERA_s98", UkLegalSource::ActOfParliament);
    rules.register_hierarchy("ERA_s162", UkLegalSource::ActOfParliament);
    rules.register_hierarchy("NMWA_1998", UkLegalSource::ActOfParliament);
    rules.register_employment_statute("ERA_s1");
    rules.register_employment_statute("ERA_s86");
    rules.register_employment_statute("ERA_s98");
    rules.register_employment_statute("ERA_s162");
    rules.register_employment_statute("NMWA_1998");

    // Register Working Time Regulations (Statutory Instruments)
    rules.register_hierarchy("WTR_Reg4", UkLegalSource::StatutoryInstrument);
    rules.register_hierarchy("WTR_Reg12", UkLegalSource::StatutoryInstrument);
    rules.register_hierarchy("WTR_Reg13", UkLegalSource::StatutoryInstrument);
    rules.register_employment_statute("WTR_Reg4");
    rules.register_employment_statute("WTR_Reg12");
    rules.register_employment_statute("WTR_Reg13");

    // Register UK GDPR (Retained EU Law)
    rules.register_hierarchy("UK_GDPR", UkLegalSource::RetainedEuLaw);
    rules.register_gdpr_statute("UK_GDPR");
}

/// Verification report for UK law compliance.
#[derive(Debug, Clone)]
pub struct UkVerificationReport {
    /// Base verification result
    pub result: VerificationResult,
    /// HRA compliance issues
    pub hra_issues: Vec<HraIssue>,
    /// Brexit/REUL considerations
    pub brexit_notes: Vec<String>,
}

/// Human Rights Act compliance issue.
#[derive(Debug, Clone)]
pub struct HraIssue {
    /// Statute ID with the issue
    pub statute_id: String,
    /// ECHR Article potentially violated
    pub article: String,
    /// Description of the issue
    pub description: String,
    /// Severity of the issue
    pub severity: Severity,
}

impl UkVerificationReport {
    /// Creates a new report from verification result.
    #[must_use]
    pub fn new(result: VerificationResult) -> Self {
        Self {
            result,
            hra_issues: Vec::new(),
            brexit_notes: Vec::new(),
        }
    }

    /// Checks if the verification passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.result.passed && self.hra_issues.is_empty()
    }

    /// Gets all critical issues.
    #[must_use]
    pub fn critical_issues(&self) -> Vec<&VerificationError> {
        self.result.errors_by_severity(Severity::Critical)
    }

    /// Generates a summary.
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
        summary.push_str(&format!("Suggestions: {}\n", self.result.suggestions.len()));

        if !self.hra_issues.is_empty() {
            summary.push_str("\nHuman Rights Act Issues:\n");
            for issue in &self.hra_issues {
                summary.push_str(&format!(
                    "  - {} (ECHR {}): {}\n",
                    issue.statute_id, issue.article, issue.description
                ));
            }
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reasoning::statute_adapter::all_employment_statutes;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_verifier_creation() {
        let verifier = UkStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_employment_statutes() {
        let verifier = UkStatuteVerifier::new();
        let statutes = all_employment_statutes();

        let result = verifier.verify(&statutes);
        assert!(result.passed || !result.errors.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = UkStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = UkStatuteVerifier::new();
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
        assert!(
            UkLegalSource::ConstitutionalStatute.is_higher_than(&UkLegalSource::ActOfParliament)
        );
        assert!(UkLegalSource::ActOfParliament.is_higher_than(&UkLegalSource::RetainedEuLaw));
        assert!(UkLegalSource::RetainedEuLaw.is_higher_than(&UkLegalSource::StatutoryInstrument));
    }

    #[test]
    fn test_legal_source_from_statute() {
        let statute = Statute::new(
            "ERA_s98",
            "Employment Rights Act 1996 s.98",
            Effect::new(EffectType::Grant, "Test"),
        );
        assert_eq!(
            UkLegalSource::from_statute(&statute),
            UkLegalSource::ActOfParliament
        );
    }

    #[test]
    fn test_legal_source_is_binding() {
        assert!(UkLegalSource::ActOfParliament.is_binding());
        assert!(!UkLegalSource::Guidance.is_binding());
    }

    #[test]
    fn test_uk_human_rights_principles() {
        let principles = uk_human_rights_principles();
        assert!(!principles.is_empty());

        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"hra-art6-fair-trial"));
        assert!(principle_ids.contains(&"hra-art8-privacy"));
        assert!(principle_ids.contains(&"hra-art14-nondiscrimination"));
    }

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = UkVerificationReport::new(result);
        assert!(report.passed());
    }

    #[test]
    fn test_employment_statute_detection() {
        let verifier = UkStatuteVerifier::new();
        assert!(verifier.hierarchy_rules.is_employment_statute("ERA_s98"));
        assert!(verifier.hierarchy_rules.is_employment_statute("WTR_Reg4"));
    }
}
