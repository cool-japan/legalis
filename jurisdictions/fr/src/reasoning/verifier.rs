//! Legal statute verification for French law (Droit français).
//!
//! This module provides verification capabilities for French legal statutes,
//! checking compliance with the Constitution and the Declaration of 1789.
//!
//! # French Legal Framework
//!
//! - Constitution of the Fifth Republic (1958)
//! - Declaration of the Rights of Man and of the Citizen (1789)
//! - Organic Laws (Lois organiques)
//! - Ordinary Laws (Lois ordinaires)
//! - Decrees (Décrets)
//! - Arrêtés
//!
//! # Key Principles
//!
//! - **Liberté**: Freedom of the individual
//! - **Égalité**: Equality before the law
//! - **Fraternité**: Social solidarity
//! - **Laïcité**: Secularism
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_fr::reasoning::verifier::{FrStatuteVerifier, fr_constitutional_principles};
//! use legalis_fr::reasoning::statute_adapter::all_french_statutes;
//!
//! let verifier = FrStatuteVerifier::new();
//! let statutes = all_french_statutes();
//!
//! let result = verifier.verify(&statutes);
//! println!("Vérification réussie: {}", result.passed);
//! ```

use legalis_core::Statute;
use legalis_verifier::{
    ConstitutionalPrinciple, PrincipleCheck, Severity, StatuteVerifier, VerificationError,
    VerificationResult,
};
use std::collections::{HashMap, HashSet};

/// French statute verifier with Constitutional compliance checking.
///
/// This verifier integrates French constitutional principles and the
/// legal hierarchy (hiérarchie des normes) into the verification framework.
pub struct FrStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// French-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for FrStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// French legal source hierarchy (Hiérarchie des normes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrLegalSource {
    /// Constitution (Bloc de constitutionnalité)
    Constitution,
    /// Organic Laws (Lois organiques)
    LoiOrganique,
    /// Ordinary Laws (Lois ordinaires)
    LoiOrdinaire,
    /// Ordinances (Ordonnances)
    Ordonnance,
    /// Decrees (Décrets)
    Decret,
    /// Arrêtés (Ministerial orders)
    Arrete,
    /// Circulaires (Circular letters - non-binding)
    Circulaire,
}

impl FrLegalSource {
    /// Returns the French name for this legal source.
    #[must_use]
    pub const fn french_name(&self) -> &'static str {
        match self {
            Self::Constitution => "Constitution",
            Self::LoiOrganique => "Loi organique",
            Self::LoiOrdinaire => "Loi ordinaire",
            Self::Ordonnance => "Ordonnance",
            Self::Decret => "Décret",
            Self::Arrete => "Arrêté",
            Self::Circulaire => "Circulaire",
        }
    }

    /// Returns the English name for this legal source.
    #[must_use]
    pub const fn english_name(&self) -> &'static str {
        match self {
            Self::Constitution => "Constitution",
            Self::LoiOrganique => "Organic Law",
            Self::LoiOrdinaire => "Ordinary Law",
            Self::Ordonnance => "Ordinance",
            Self::Decret => "Decree",
            Self::Arrete => "Ministerial Order",
            Self::Circulaire => "Circular Letter",
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

        if id.contains("org_") || title.contains("organique") {
            return Self::LoiOrganique;
        }

        if id.contains("ord_") || title.contains("ordonnance") {
            return Self::Ordonnance;
        }

        if id.contains("dec_") || title.contains("décret") || title.contains("decret") {
            return Self::Decret;
        }

        if id.contains("arr_") || title.contains("arrêté") || title.contains("arrete") {
            return Self::Arrete;
        }

        if title.contains("circulaire") {
            return Self::Circulaire;
        }

        Self::LoiOrdinaire
    }

    /// Checks if this level is higher than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Checks if this is binding law.
    #[must_use]
    pub fn is_binding(&self) -> bool {
        !matches!(self, Self::Circulaire)
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies
    known_hierarchies: HashMap<String, FrLegalSource>,
    /// Labor Code (Code du travail) statutes
    labor_statutes: HashSet<String>,
    /// Civil Code statutes
    civil_statutes: HashSet<String>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: FrLegalSource) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Marks a statute as labor-related.
    pub fn register_labor_statute(&mut self, statute_id: impl Into<String>) {
        self.labor_statutes.insert(statute_id.into());
    }

    /// Marks a statute as civil-related.
    pub fn register_civil_statute(&mut self, statute_id: impl Into<String>) {
        self.civil_statutes.insert(statute_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> FrLegalSource {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| FrLegalSource::from_statute(statute))
    }

    /// Checks if a statute is labor-related.
    #[must_use]
    pub fn is_labor_statute(&self, statute_id: &str) -> bool {
        self.labor_statutes.contains(statute_id) || statute_id.starts_with("L_")
    }

    /// Checks if a statute is civil-related.
    #[must_use]
    pub fn is_civil_statute(&self, statute_id: &str) -> bool {
        self.civil_statutes.contains(statute_id) || statute_id.starts_with("CC_")
    }
}

impl Default for FrStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl FrStatuteVerifier {
    /// Creates a new French statute verifier with constitutional principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = fr_constitutional_principles();
        let inner = StatuteVerifier::with_principles(principles);

        let mut hierarchy_rules = HierarchyRules::new();
        setup_fr_hierarchy(&mut hierarchy_rules);

        Self {
            inner,
            hierarchy_rules,
        }
    }

    /// Verifies a set of statutes with French-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_code_consistency(statutes));

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
                    let (higher, lower) = if level_i.is_higher_than(&level_j) {
                        (&statutes[i], &statutes[j])
                    } else {
                        (&statutes[j], &statutes[i])
                    };

                    let lower_level = self.hierarchy_rules.get_hierarchy(lower);
                    let higher_level = self.hierarchy_rules.get_hierarchy(higher);

                    result = result.with_warning(format!(
                        "Hiérarchie des normes: {} ({}) peut être en conflit avec {} ({})",
                        lower.id,
                        lower_level.french_name(),
                        higher.id,
                        higher_level.french_name()
                    ));
                }
            }
        }

        result
    }

    fn check_code_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let labor_count = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.is_labor_statute(&s.id))
            .count();

        if labor_count > 1 {
            result = result.with_suggestion(format!(
                "Cohérence du Code du travail: {} dispositions trouvées",
                labor_count
            ));
        }

        let civil_count = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.is_civil_statute(&s.id))
            .count();

        if civil_count > 1 {
            result = result.with_suggestion(format!(
                "Cohérence du Code civil: {} dispositions trouvées",
                civil_count
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

/// Creates constitutional principles based on the French Constitution and DDHC.
///
/// Includes principles from:
/// - Declaration of the Rights of Man and of the Citizen (1789)
/// - Preamble of the Constitution of 1946
/// - Constitution of the Fifth Republic (1958)
#[must_use]
pub fn fr_constitutional_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        // Article 1 DDHC: Liberty
        ConstitutionalPrinciple {
            id: "ddhc-art1-liberte".to_string(),
            name: "Liberté (DDHC Art. 1)".to_string(),
            description: "Les hommes naissent et demeurent libres et égaux en droits.\nMen are born and remain free and equal in rights.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Personal liberty protection".to_string(),
            },
        },
        // Article 1 DDHC: Equality
        ConstitutionalPrinciple {
            id: "ddhc-art1-egalite".to_string(),
            name: "Égalité (DDHC Art. 1)".to_string(),
            description: "Les distinctions sociales ne peuvent être fondées que sur l'utilité commune.\nSocial distinctions may be based only on common utility.".to_string(),
            check: PrincipleCheck::EqualProtection,
        },
        // Article 4 DDHC: Limits of Liberty
        ConstitutionalPrinciple {
            id: "ddhc-art4-limits".to_string(),
            name: "Limites de la liberté (DDHC Art. 4)".to_string(),
            description: "La liberté consiste à pouvoir faire tout ce qui ne nuit pas à autrui.\nLiberty consists in the ability to do whatever does not harm another.".to_string(),
            check: PrincipleCheck::Proportionality,
        },
        // Article 6 DDHC: Rule of Law
        ConstitutionalPrinciple {
            id: "ddhc-art6-loi".to_string(),
            name: "Égalité devant la loi (DDHC Art. 6)".to_string(),
            description: "La loi est l'expression de la volonté générale. Elle doit être la même pour tous.\nThe law is the expression of the general will. It must be the same for all.".to_string(),
            check: PrincipleCheck::NoDiscrimination,
        },
        // Article 17 DDHC: Property
        ConstitutionalPrinciple {
            id: "ddhc-art17-propriete".to_string(),
            name: "Propriété (DDHC Art. 17)".to_string(),
            description: "La propriété étant un droit inviolable et sacré.\nProperty being an inviolable and sacred right.".to_string(),
            check: PrincipleCheck::PropertyRights,
        },
        // Constitution Art. 1: Laïcité
        ConstitutionalPrinciple {
            id: "const-art1-laicite".to_string(),
            name: "Laïcité (Constitution Art. 1)".to_string(),
            description: "La France est une République indivisible, laïque, démocratique et sociale.\nFrance is an indivisible, secular, democratic and social Republic.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Secularism principle".to_string(),
            },
        },
        // Preamble 1946: Social Rights
        ConstitutionalPrinciple {
            id: "preambule46-social".to_string(),
            name: "Droits sociaux (Préambule 1946)".to_string(),
            description: "Chacun a le devoir de travailler et le droit d'obtenir un emploi.\nEveryone has the duty to work and the right to obtain employment.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Social rights protection".to_string(),
            },
        },
        // Legal Certainty
        ConstitutionalPrinciple {
            id: "fr-securite-juridique".to_string(),
            name: "Sécurité juridique".to_string(),
            description: "Le principe de sécurité juridique implique la clarté et la prévisibilité du droit.\nThe principle of legal certainty requires clarity and predictability of the law.".to_string(),
            check: PrincipleCheck::NoRetroactivity,
        },
    ]
}

/// Sets up French hierarchy relationships.
fn setup_fr_hierarchy(rules: &mut HierarchyRules) {
    // Civil Code articles
    rules.register_hierarchy("CC_Art1103", FrLegalSource::LoiOrdinaire);
    rules.register_hierarchy("CC_Art1240", FrLegalSource::LoiOrdinaire);
    rules.register_civil_statute("CC_Art1103");
    rules.register_civil_statute("CC_Art1240");

    // Labor Code articles
    rules.register_hierarchy("L_1221_1", FrLegalSource::LoiOrdinaire);
    rules.register_hierarchy("L_1232_1", FrLegalSource::LoiOrdinaire);
    rules.register_labor_statute("L_1221_1");
    rules.register_labor_statute("L_1232_1");

    // Commercial Code
    rules.register_hierarchy("ComC_L210_1", FrLegalSource::LoiOrdinaire);
}

/// Verification report for French law compliance.
#[derive(Debug, Clone)]
pub struct FrVerificationReport {
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

impl FrVerificationReport {
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
    pub fn summary_fr(&self) -> String {
        let mut summary = String::new();
        if self.passed() {
            summary.push_str("Résultat de vérification: RÉUSSI\n");
        } else {
            summary.push_str("Résultat de vérification: ÉCHEC\n");
        }
        summary.push_str(&format!("Erreurs: {}\n", self.result.errors.len()));
        summary.push_str(&format!("Avertissements: {}\n", self.result.warnings.len()));
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_verifier_creation() {
        let verifier = FrStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = FrStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = FrStatuteVerifier::new();
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
        assert!(FrLegalSource::Constitution.is_higher_than(&FrLegalSource::LoiOrganique));
        assert!(FrLegalSource::LoiOrganique.is_higher_than(&FrLegalSource::LoiOrdinaire));
        assert!(FrLegalSource::LoiOrdinaire.is_higher_than(&FrLegalSource::Ordonnance));
        assert!(FrLegalSource::Ordonnance.is_higher_than(&FrLegalSource::Decret));
    }

    #[test]
    fn test_legal_source_french_names() {
        assert_eq!(FrLegalSource::Constitution.french_name(), "Constitution");
        assert_eq!(FrLegalSource::LoiOrganique.french_name(), "Loi organique");
        assert_eq!(FrLegalSource::Decret.french_name(), "Décret");
    }

    #[test]
    fn test_fr_constitutional_principles() {
        let principles = fr_constitutional_principles();
        assert!(!principles.is_empty());

        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"ddhc-art1-liberte"));
        assert!(principle_ids.contains(&"ddhc-art1-egalite"));
        assert!(principle_ids.contains(&"const-art1-laicite"));
    }

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = FrVerificationReport::new(result);
        assert!(report.passed());
    }

    #[test]
    fn test_verification_report_summary() {
        let result = VerificationResult::pass();
        let report = FrVerificationReport::new(result);
        let summary = report.summary_fr();
        assert!(summary.contains("RÉUSSI"));
    }
}
