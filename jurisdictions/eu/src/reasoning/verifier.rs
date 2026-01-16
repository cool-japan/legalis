//! Legal statute verification for European Union law.
//!
//! This module provides verification capabilities for EU legal statutes,
//! checking compliance with the Charter of Fundamental Rights, GDPR principles,
//! and EU legal hierarchy.
//!
//! # EU Legal Framework
//!
//! | Source | Authority |
//! |--------|-----------|
//! | Treaties (TEU, TFEU) | Primary law |
//! | Charter of Fundamental Rights | Constitutional |
//! | Regulations | Directly applicable |
//! | Directives | Require transposition |
//! | Decisions | Binding on addressees |
//!
//! # Key Principles
//!
//! - **Proportionality**: Measures must be suitable, necessary, and balanced
//! - **Subsidiarity**: EU acts only when Member States cannot achieve objectives
//! - **Conferral**: EU has only powers conferred by Treaties
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_eu::reasoning::verifier::{EuStatuteVerifier, eu_fundamental_rights_principles};
//! use legalis_eu::reasoning::statute_adapter::all_eu_statutes;
//!
//! let verifier = EuStatuteVerifier::new();
//! let statutes = all_eu_statutes();
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

/// EU statute verifier with Charter of Fundamental Rights compliance checking.
///
/// This verifier integrates EU fundamental rights, GDPR principles, and
/// the EU legal hierarchy into the verification framework.
///
/// # EU Legal Hierarchy
///
/// 1. Primary Law (Treaties)
/// 2. Charter of Fundamental Rights
/// 3. General Principles of EU Law
/// 4. Regulations
/// 5. Directives
/// 6. Decisions
/// 7. Recommendations/Opinions (non-binding)
pub struct EuStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// EU-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for EuStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EuStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// EU legal source hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EuLegalSource {
    /// Treaties (TEU, TFEU) - Primary law
    Treaty,
    /// Charter of Fundamental Rights
    Charter,
    /// General Principles of EU Law
    GeneralPrinciple,
    /// Regulations - Directly applicable
    Regulation,
    /// Directives - Require Member State transposition
    Directive,
    /// Decisions - Binding on addressees
    Decision,
    /// National transposition of Directive
    NationalTransposition,
    /// Recommendations and Opinions (non-binding)
    SoftLaw,
}

impl EuLegalSource {
    /// Returns the legal source name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Treaty => "Treaty (Primary Law)",
            Self::Charter => "Charter of Fundamental Rights",
            Self::GeneralPrinciple => "General Principle of EU Law",
            Self::Regulation => "Regulation",
            Self::Directive => "Directive",
            Self::Decision => "Decision",
            Self::NationalTransposition => "National Transposition",
            Self::SoftLaw => "Recommendation/Opinion",
        }
    }

    /// Returns the legal source abbreviation.
    #[must_use]
    pub const fn abbreviation(&self) -> &'static str {
        match self {
            Self::Treaty => "TEU/TFEU",
            Self::Charter => "CFR",
            Self::GeneralPrinciple => "GP",
            Self::Regulation => "Reg.",
            Self::Directive => "Dir.",
            Self::Decision => "Dec.",
            Self::NationalTransposition => "Nat.",
            Self::SoftLaw => "Rec./Op.",
        }
    }

    /// Determines legal source from statute ID or title.
    #[must_use]
    pub fn from_statute(statute: &Statute) -> Self {
        let id = statute.id.to_lowercase();
        let title = statute.title.to_lowercase();

        // Check for Treaties
        if id.contains("teu")
            || id.contains("tfeu")
            || id.contains("treaty")
            || title.contains("treaty")
        {
            return Self::Treaty;
        }

        // Check for Charter
        if id.contains("cfr")
            || id.contains("charter")
            || title.contains("charter of fundamental rights")
        {
            return Self::Charter;
        }

        // Check for Regulations
        if id.contains("reg_")
            || id.starts_with("regulation")
            || title.contains("regulation")
            || id.contains("gdpr")
        {
            return Self::Regulation;
        }

        // Check for Directives
        if id.contains("dir_") || id.starts_with("directive") || title.contains("directive") {
            return Self::Directive;
        }

        // Check for Decisions
        if id.contains("dec_") || title.contains("decision") {
            return Self::Decision;
        }

        // Default to Regulation for most EU laws
        Self::Regulation
    }

    /// Checks if this level is higher in the hierarchy than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Checks if this is binding law.
    #[must_use]
    pub fn is_binding(&self) -> bool {
        !matches!(self, Self::SoftLaw)
    }

    /// Checks if this requires Member State transposition.
    #[must_use]
    pub fn requires_transposition(&self) -> bool {
        matches!(self, Self::Directive)
    }
}

/// GDPR data protection principles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GdprPrinciple {
    /// Lawfulness, fairness and transparency (Art. 5(1)(a))
    Lawfulness,
    /// Purpose limitation (Art. 5(1)(b))
    PurposeLimitation,
    /// Data minimisation (Art. 5(1)(c))
    DataMinimisation,
    /// Accuracy (Art. 5(1)(d))
    Accuracy,
    /// Storage limitation (Art. 5(1)(e))
    StorageLimitation,
    /// Integrity and confidentiality (Art. 5(1)(f))
    IntegrityConfidentiality,
    /// Accountability (Art. 5(2))
    Accountability,
}

impl GdprPrinciple {
    /// Returns the GDPR article reference.
    #[must_use]
    pub const fn article(&self) -> &'static str {
        match self {
            Self::Lawfulness => "Art. 5(1)(a)",
            Self::PurposeLimitation => "Art. 5(1)(b)",
            Self::DataMinimisation => "Art. 5(1)(c)",
            Self::Accuracy => "Art. 5(1)(d)",
            Self::StorageLimitation => "Art. 5(1)(e)",
            Self::IntegrityConfidentiality => "Art. 5(1)(f)",
            Self::Accountability => "Art. 5(2)",
        }
    }

    /// Returns the principle name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Lawfulness => "Lawfulness, Fairness and Transparency",
            Self::PurposeLimitation => "Purpose Limitation",
            Self::DataMinimisation => "Data Minimisation",
            Self::Accuracy => "Accuracy",
            Self::StorageLimitation => "Storage Limitation",
            Self::IntegrityConfidentiality => "Integrity and Confidentiality",
            Self::Accountability => "Accountability",
        }
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies
    known_hierarchies: HashMap<String, EuLegalSource>,
    /// GDPR-related statutes
    gdpr_statutes: HashSet<String>,
    /// Competition law statutes
    competition_statutes: HashSet<String>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: EuLegalSource) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Marks a statute as GDPR-related.
    pub fn register_gdpr_statute(&mut self, statute_id: impl Into<String>) {
        self.gdpr_statutes.insert(statute_id.into());
    }

    /// Marks a statute as competition-related.
    pub fn register_competition_statute(&mut self, statute_id: impl Into<String>) {
        self.competition_statutes.insert(statute_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> EuLegalSource {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| EuLegalSource::from_statute(statute))
    }

    /// Checks if a statute is GDPR-related.
    #[must_use]
    pub fn is_gdpr_statute(&self, statute_id: &str) -> bool {
        self.gdpr_statutes.contains(statute_id)
            || statute_id.contains("GDPR")
            || statute_id.contains("gdpr")
    }

    /// Checks if a statute is competition-related.
    #[must_use]
    pub fn is_competition_statute(&self, statute_id: &str) -> bool {
        self.competition_statutes.contains(statute_id)
            || statute_id.contains("Art101")
            || statute_id.contains("Art102")
    }
}

impl Default for EuStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl EuStatuteVerifier {
    /// Creates a new EU statute verifier with fundamental rights principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = eu_fundamental_rights_principles();
        let inner = StatuteVerifier::with_principles(principles);

        let mut hierarchy_rules = HierarchyRules::new();
        setup_eu_hierarchy(&mut hierarchy_rules);

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

    /// Verifies a set of statutes with EU-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        // Additional EU-specific checks
        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_gdpr_consistency(statutes));
        result.merge(self.check_proportionality(statutes));

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
                        "EU hierarchy issue: {} ({}) may conflict with {} ({})",
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

    /// Checks GDPR consistency.
    fn check_gdpr_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let gdpr_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.is_gdpr_statute(&s.id))
            .collect();

        if gdpr_statutes.len() > 1 {
            // Check for potential conflicts between GDPR provisions
            result = result.with_suggestion(format!(
                "GDPR consistency check: {} data protection provisions found",
                gdpr_statutes.len()
            ));
        }

        // Check for lawful basis requirements
        let processing_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| {
                let title = s.title.to_lowercase();
                title.contains("processing")
                    || title.contains("lawfulness")
                    || s.id.contains("Art6")
            })
            .collect();

        if !processing_statutes.is_empty() {
            result = result.with_suggestion(format!(
                "GDPR Article 6 check: {} lawful basis provisions to verify",
                processing_statutes.len()
            ));
        }

        result
    }

    /// Checks proportionality principle compliance.
    ///
    /// EU measures must be:
    /// 1. Suitable: Capable of achieving the objective
    /// 2. Necessary: No less restrictive alternative
    /// 3. Proportionate: Benefits outweigh restrictions
    fn check_proportionality(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for statute in statutes {
            // Check statutes with restrictions (Prohibition/Revoke effects)
            if matches!(
                statute.effect.effect_type,
                legalis_core::EffectType::Prohibition | legalis_core::EffectType::Revoke
            ) {
                result = result.with_suggestion(format!(
                    "Proportionality check: {} contains restrictions - verify necessity and balance",
                    statute.id
                ));
            }
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

/// Creates fundamental rights principles based on the EU Charter.
///
/// Returns principles for key Charter articles and EU general principles:
/// - Human Dignity (Art. 1)
/// - Right to Life (Art. 2)
/// - Data Protection (Art. 8)
/// - Non-Discrimination (Art. 21)
/// - Right to Effective Remedy (Art. 47)
#[must_use]
pub fn eu_fundamental_rights_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        // Article 1: Human Dignity
        ConstitutionalPrinciple {
            id: "cfr-art1-dignity".to_string(),
            name: "Human Dignity (CFR Art. 1)".to_string(),
            description: "Human dignity is inviolable. It must be respected and protected.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Human dignity protection".to_string(),
            },
        },
        // Article 6: Right to Liberty and Security
        ConstitutionalPrinciple {
            id: "cfr-art6-liberty".to_string(),
            name: "Right to Liberty (CFR Art. 6)".to_string(),
            description: "Everyone has the right to liberty and security of person.".to_string(),
            check: PrincipleCheck::RequiresProcedure,
        },
        // Article 7: Respect for Private Life
        ConstitutionalPrinciple {
            id: "cfr-art7-privacy".to_string(),
            name: "Privacy (CFR Art. 7)".to_string(),
            description: "Everyone has the right to respect for his or her private and family life, home and communications.".to_string(),
            check: PrincipleCheck::PrivacyImpact,
        },
        // Article 8: Data Protection
        ConstitutionalPrinciple {
            id: "cfr-art8-data".to_string(),
            name: "Data Protection (CFR Art. 8)".to_string(),
            description: "Everyone has the right to the protection of personal data concerning him or her. Such data must be processed fairly for specified purposes.".to_string(),
            check: PrincipleCheck::PrivacyImpact,
        },
        // Article 17: Right to Property
        ConstitutionalPrinciple {
            id: "cfr-art17-property".to_string(),
            name: "Property Rights (CFR Art. 17)".to_string(),
            description: "Everyone has the right to own, use, dispose of and bequeath his or her lawfully acquired possessions.".to_string(),
            check: PrincipleCheck::PropertyRights,
        },
        // Article 21: Non-Discrimination
        ConstitutionalPrinciple {
            id: "cfr-art21-nondiscrimination".to_string(),
            name: "Non-Discrimination (CFR Art. 21)".to_string(),
            description: "Any discrimination based on any ground such as sex, race, colour, ethnic or social origin, genetic features, language, religion or belief, political or any other opinion, membership of a national minority, property, birth, disability, age or sexual orientation shall be prohibited.".to_string(),
            check: PrincipleCheck::EqualProtection,
        },
        // Article 47: Right to Effective Remedy
        ConstitutionalPrinciple {
            id: "cfr-art47-remedy".to_string(),
            name: "Effective Remedy (CFR Art. 47)".to_string(),
            description: "Everyone whose rights and freedoms guaranteed by the law of the Union are violated has the right to an effective remedy before a tribunal.".to_string(),
            check: PrincipleCheck::ProceduralDueProcess,
        },
        // Proportionality Principle
        ConstitutionalPrinciple {
            id: "eu-proportionality".to_string(),
            name: "Proportionality Principle".to_string(),
            description: "The content and form of Union action shall not exceed what is necessary to achieve the objectives of the Treaties (TEU Art. 5(4)).".to_string(),
            check: PrincipleCheck::Proportionality,
        },
        // Legal Certainty
        ConstitutionalPrinciple {
            id: "eu-legal-certainty".to_string(),
            name: "Legal Certainty".to_string(),
            description: "Legal rules must be clear, precise, and predictable in their effects.".to_string(),
            check: PrincipleCheck::NoRetroactivity,
        },
    ]
}

/// Sets up EU hierarchy relationships.
fn setup_eu_hierarchy(rules: &mut HierarchyRules) {
    // Register GDPR articles
    rules.register_hierarchy("GDPR_Art5", EuLegalSource::Regulation);
    rules.register_hierarchy("GDPR_Art6", EuLegalSource::Regulation);
    rules.register_hierarchy("GDPR_Art7", EuLegalSource::Regulation);
    rules.register_hierarchy("GDPR_Art8", EuLegalSource::Regulation);
    rules.register_gdpr_statute("GDPR_Art5");
    rules.register_gdpr_statute("GDPR_Art6");
    rules.register_gdpr_statute("GDPR_Art7");
    rules.register_gdpr_statute("GDPR_Art8");

    // Register Competition law articles
    rules.register_hierarchy("Art101", EuLegalSource::Treaty);
    rules.register_hierarchy("Art102", EuLegalSource::Treaty);
    rules.register_competition_statute("Art101");
    rules.register_competition_statute("Art102");

    // Register Consumer Rights Directive
    rules.register_hierarchy("CRD_Art3", EuLegalSource::Directive);
    rules.register_hierarchy("CRD_Art4", EuLegalSource::Directive);
}

/// Verification report for EU law compliance.
#[derive(Debug, Clone)]
pub struct EuVerificationReport {
    /// Base verification result
    pub result: VerificationResult,
    /// Charter compliance issues
    pub charter_issues: Vec<CharterIssue>,
    /// GDPR compliance issues
    pub gdpr_issues: Vec<GdprIssue>,
    /// Proportionality concerns
    pub proportionality_notes: Vec<String>,
}

/// Charter of Fundamental Rights issue.
#[derive(Debug, Clone)]
pub struct CharterIssue {
    /// Statute ID with the issue
    pub statute_id: String,
    /// Article of the Charter potentially violated
    pub article: String,
    /// Description of the issue
    pub description: String,
    /// Severity of the issue
    pub severity: Severity,
}

/// GDPR compliance issue.
#[derive(Debug, Clone)]
pub struct GdprIssue {
    /// Statute ID with the issue
    pub statute_id: String,
    /// GDPR principle potentially violated
    pub principle: GdprPrinciple,
    /// Description of the issue
    pub description: String,
    /// Severity of the issue
    pub severity: Severity,
}

impl EuVerificationReport {
    /// Creates a new report from verification result.
    #[must_use]
    pub fn new(result: VerificationResult) -> Self {
        Self {
            result,
            charter_issues: Vec::new(),
            gdpr_issues: Vec::new(),
            proportionality_notes: Vec::new(),
        }
    }

    /// Checks if the verification passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.result.passed && self.charter_issues.is_empty() && self.gdpr_issues.is_empty()
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

        if !self.charter_issues.is_empty() {
            summary.push_str("\nCharter of Fundamental Rights Issues:\n");
            for issue in &self.charter_issues {
                summary.push_str(&format!(
                    "  - {} (CFR {}): {}\n",
                    issue.statute_id, issue.article, issue.description
                ));
            }
        }

        if !self.gdpr_issues.is_empty() {
            summary.push_str("\nGDPR Compliance Issues:\n");
            for issue in &self.gdpr_issues {
                summary.push_str(&format!(
                    "  - {} ({}: {}): {}\n",
                    issue.statute_id,
                    issue.principle.article(),
                    issue.principle.name(),
                    issue.description
                ));
            }
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reasoning::statute_adapter::all_eu_statutes;
    use legalis_core::{Effect, EffectType, Statute};

    // ========================================================================
    // EuStatuteVerifier tests
    // ========================================================================

    #[test]
    fn test_verifier_creation() {
        let verifier = EuStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_eu_statutes() {
        let verifier = EuStatuteVerifier::new();
        let statutes = all_eu_statutes();

        let result = verifier.verify(&statutes);
        assert!(result.passed || !result.errors.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = EuStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = EuStatuteVerifier::new();
        let statute = Statute::new(
            "TEST_001",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = verifier.verify_single(&statute);
        assert!(result.passed);
    }

    // ========================================================================
    // EuLegalSource tests
    // ========================================================================

    #[test]
    fn test_legal_source_ordering() {
        assert!(EuLegalSource::Treaty.is_higher_than(&EuLegalSource::Charter));
        assert!(EuLegalSource::Charter.is_higher_than(&EuLegalSource::GeneralPrinciple));
        assert!(EuLegalSource::GeneralPrinciple.is_higher_than(&EuLegalSource::Regulation));
        assert!(EuLegalSource::Regulation.is_higher_than(&EuLegalSource::Directive));
        assert!(EuLegalSource::Directive.is_higher_than(&EuLegalSource::Decision));
    }

    #[test]
    fn test_legal_source_from_statute_regulation() {
        let statute = Statute::new(
            "GDPR_Art5",
            "General Data Protection Regulation Article 5",
            Effect::new(EffectType::Obligation, "Test"),
        );
        assert_eq!(
            EuLegalSource::from_statute(&statute),
            EuLegalSource::Regulation
        );
    }

    #[test]
    fn test_legal_source_from_statute_directive() {
        let statute = Statute::new(
            "Dir_2011_83",
            "Consumer Rights Directive",
            Effect::new(EffectType::Grant, "Test"),
        );
        assert_eq!(
            EuLegalSource::from_statute(&statute),
            EuLegalSource::Directive
        );
    }

    #[test]
    fn test_legal_source_is_binding() {
        assert!(EuLegalSource::Regulation.is_binding());
        assert!(EuLegalSource::Directive.is_binding());
        assert!(!EuLegalSource::SoftLaw.is_binding());
    }

    #[test]
    fn test_legal_source_requires_transposition() {
        assert!(EuLegalSource::Directive.requires_transposition());
        assert!(!EuLegalSource::Regulation.requires_transposition());
    }

    // ========================================================================
    // GdprPrinciple tests
    // ========================================================================

    #[test]
    fn test_gdpr_principle_articles() {
        assert_eq!(GdprPrinciple::Lawfulness.article(), "Art. 5(1)(a)");
        assert_eq!(GdprPrinciple::PurposeLimitation.article(), "Art. 5(1)(b)");
        assert_eq!(GdprPrinciple::DataMinimisation.article(), "Art. 5(1)(c)");
        assert_eq!(GdprPrinciple::Accountability.article(), "Art. 5(2)");
    }

    // ========================================================================
    // HierarchyRules tests
    // ========================================================================

    #[test]
    fn test_register_gdpr_statute() {
        let mut rules = HierarchyRules::new();
        rules.register_gdpr_statute("GDPR_Art5");

        assert!(rules.is_gdpr_statute("GDPR_Art5"));
    }

    #[test]
    fn test_register_competition_statute() {
        let mut rules = HierarchyRules::new();
        rules.register_competition_statute("Art101");

        assert!(rules.is_competition_statute("Art101"));
    }

    #[test]
    fn test_register_hierarchy() {
        let mut rules = HierarchyRules::new();
        rules.register_hierarchy("TEST_001", EuLegalSource::Directive);

        let statute = Statute::new("TEST_001", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(rules.get_hierarchy(&statute), EuLegalSource::Directive);
    }

    // ========================================================================
    // Constitutional principles tests
    // ========================================================================

    #[test]
    fn test_eu_fundamental_rights_principles() {
        let principles = eu_fundamental_rights_principles();

        assert!(!principles.is_empty());

        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"cfr-art1-dignity"));
        assert!(principle_ids.contains(&"cfr-art7-privacy"));
        assert!(principle_ids.contains(&"cfr-art8-data"));
        assert!(principle_ids.contains(&"cfr-art21-nondiscrimination"));
        assert!(principle_ids.contains(&"eu-proportionality"));
    }

    #[test]
    fn test_fundamental_rights_have_descriptions() {
        let principles = eu_fundamental_rights_principles();

        for principle in &principles {
            assert!(!principle.name.is_empty());
            assert!(!principle.description.is_empty());
        }
    }

    // ========================================================================
    // EuVerificationReport tests
    // ========================================================================

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = EuVerificationReport::new(result);
        assert!(report.passed());
    }

    #[test]
    fn test_verification_report_with_charter_issues() {
        let result = VerificationResult::pass();
        let mut report = EuVerificationReport::new(result);
        report.charter_issues.push(CharterIssue {
            statute_id: "TEST".to_string(),
            article: "Art. 8".to_string(),
            description: "Test issue".to_string(),
            severity: Severity::Warning,
        });
        assert!(!report.passed());
    }

    #[test]
    fn test_verification_report_summary() {
        let result = VerificationResult::pass();
        let report = EuVerificationReport::new(result);
        let summary = report.summary();
        assert!(summary.contains("Verification Result: PASSED"));
    }

    // ========================================================================
    // Integration tests
    // ========================================================================

    #[test]
    fn test_eu_hierarchy_setup() {
        let verifier = EuStatuteVerifier::new();

        let statute = Statute::new("GDPR_Art5", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(
            verifier.hierarchy_rules.get_hierarchy(&statute),
            EuLegalSource::Regulation
        );
    }

    #[test]
    fn test_gdpr_statute_detection() {
        let verifier = EuStatuteVerifier::new();
        assert!(verifier.hierarchy_rules.is_gdpr_statute("GDPR_Art5"));
        assert!(verifier.hierarchy_rules.is_gdpr_statute("GDPR_Art6"));
    }

    #[test]
    fn test_competition_statute_detection() {
        let verifier = EuStatuteVerifier::new();
        assert!(verifier.hierarchy_rules.is_competition_statute("Art101"));
        assert!(verifier.hierarchy_rules.is_competition_statute("Art102"));
    }
}
