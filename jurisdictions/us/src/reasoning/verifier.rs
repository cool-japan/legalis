//! Legal statute verification for United States law.
//!
//! This module provides verification capabilities for US federal and state statutes,
//! checking constitutional compliance based on the US Constitution and legal hierarchy.
//!
//! # Constitutional Framework
//!
//! | Amendment/Clause | Protection |
//! |-----------------|------------|
//! | 1st Amendment | Free Speech, Religion |
//! | 4th Amendment | Search & Seizure |
//! | 5th Amendment | Due Process, Self-Incrimination |
//! | 14th Amendment | Equal Protection, Due Process |
//! | Supremacy Clause | Federal Preemption |
//!
//! # Legal Hierarchy
//!
//! 1. US Constitution
//! 2. Federal Statutes
//! 3. Federal Regulations (CFR)
//! 4. State Constitutions
//! 5. State Statutes
//! 6. State Regulations
//! 7. Local Ordinances
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_us::reasoning::verifier::{UsStatuteVerifier, us_constitutional_principles};
//! use legalis_us::reasoning::statute_adapter::all_federal_statutes;
//!
//! let verifier = UsStatuteVerifier::new();
//! let statutes = all_federal_statutes();
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

/// US statute verifier with Constitutional compliance checking.
///
/// This verifier integrates US Constitutional principles, federalism,
/// and the Supremacy Clause into the verification framework.
///
/// # Key Concepts
///
/// - **Federal Preemption**: Federal law overrides conflicting state law
/// - **Incorporation Doctrine**: Bill of Rights applies to states via 14th Amendment
/// - **Rational Basis Review**: Default standard for constitutional review
/// - **Strict Scrutiny**: Applied to suspect classifications
pub struct UsStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// US-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for UsStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// Legal hierarchy in United States law.
///
/// The US has a complex federalist structure with federal and state systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegalHierarchy {
    /// US Constitution (Supreme Law of the Land)
    UsConstitution,
    /// Federal Statutes (Acts of Congress)
    FederalStatute,
    /// Federal Regulations (Code of Federal Regulations)
    FederalRegulation,
    /// Executive Orders
    ExecutiveOrder,
    /// State Constitution
    StateConstitution,
    /// State Statute
    StateStatute,
    /// State Regulation
    StateRegulation,
    /// Local Ordinance (City/County)
    LocalOrdinance,
}

impl LegalHierarchy {
    /// Returns the hierarchy level name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::UsConstitution => "US Constitution",
            Self::FederalStatute => "Federal Statute",
            Self::FederalRegulation => "Federal Regulation",
            Self::ExecutiveOrder => "Executive Order",
            Self::StateConstitution => "State Constitution",
            Self::StateStatute => "State Statute",
            Self::StateRegulation => "State Regulation",
            Self::LocalOrdinance => "Local Ordinance",
        }
    }

    /// Returns a short abbreviation.
    #[must_use]
    pub const fn abbreviation(&self) -> &'static str {
        match self {
            Self::UsConstitution => "USC",
            Self::FederalStatute => "USC/PL",
            Self::FederalRegulation => "CFR",
            Self::ExecutiveOrder => "EO",
            Self::StateConstitution => "State Const.",
            Self::StateStatute => "State Code",
            Self::StateRegulation => "State Reg.",
            Self::LocalOrdinance => "Ord.",
        }
    }

    /// Determines hierarchy level from statute ID or title.
    #[must_use]
    pub fn from_statute(statute: &Statute) -> Self {
        let id = statute.id.to_lowercase();
        let title = statute.title.to_lowercase();

        // Check for US Constitution
        if id.contains("usconst")
            || id.contains("amendment")
            || title.contains("us constitution")
            || title.contains("amendment")
        {
            return Self::UsConstitution;
        }

        // Check for Executive Order
        if id.starts_with("eo_") || title.contains("executive order") {
            return Self::ExecutiveOrder;
        }

        // Check for Federal Regulations (CFR)
        if id.contains("cfr") || title.contains("cfr") || title.contains("code of federal") {
            return Self::FederalRegulation;
        }

        // Check for State Constitution
        if (id.contains("state") || title.contains("state"))
            && (id.contains("const") || title.contains("constitution"))
        {
            return Self::StateConstitution;
        }

        // Check for Local Ordinance
        if id.contains("ord_")
            || id.contains("local_")
            || title.contains("ordinance")
            || title.contains("municipal")
        {
            return Self::LocalOrdinance;
        }

        // Check for State Regulation
        if (id.contains("state") && id.contains("reg"))
            || title.contains("state regulation")
            || title.contains("administrative code")
        {
            return Self::StateRegulation;
        }

        // Check for State Statute
        if id.contains("state_")
            || id.starts_with("ca_")
            || id.starts_with("ny_")
            || id.starts_with("tx_")
            || title.contains("state code")
        {
            return Self::StateStatute;
        }

        // Default to Federal Statute for most US laws
        Self::FederalStatute
    }

    /// Checks if this hierarchy level is higher than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Checks if this is a federal-level law.
    #[must_use]
    pub fn is_federal(&self) -> bool {
        matches!(
            self,
            Self::UsConstitution
                | Self::FederalStatute
                | Self::FederalRegulation
                | Self::ExecutiveOrder
        )
    }

    /// Checks if this is a state-level law.
    #[must_use]
    pub fn is_state(&self) -> bool {
        matches!(
            self,
            Self::StateConstitution | Self::StateStatute | Self::StateRegulation
        )
    }
}

/// US State for state-specific rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
    DistrictOfColumbia,
}

impl UsState {
    /// Returns the two-letter state code.
    #[must_use]
    #[allow(dead_code)]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Alabama => "AL",
            Self::Alaska => "AK",
            Self::Arizona => "AZ",
            Self::Arkansas => "AR",
            Self::California => "CA",
            Self::Colorado => "CO",
            Self::Connecticut => "CT",
            Self::Delaware => "DE",
            Self::Florida => "FL",
            Self::Georgia => "GA",
            Self::Hawaii => "HI",
            Self::Idaho => "ID",
            Self::Illinois => "IL",
            Self::Indiana => "IN",
            Self::Iowa => "IA",
            Self::Kansas => "KS",
            Self::Kentucky => "KY",
            Self::Louisiana => "LA",
            Self::Maine => "ME",
            Self::Maryland => "MD",
            Self::Massachusetts => "MA",
            Self::Michigan => "MI",
            Self::Minnesota => "MN",
            Self::Mississippi => "MS",
            Self::Missouri => "MO",
            Self::Montana => "MT",
            Self::Nebraska => "NE",
            Self::Nevada => "NV",
            Self::NewHampshire => "NH",
            Self::NewJersey => "NJ",
            Self::NewMexico => "NM",
            Self::NewYork => "NY",
            Self::NorthCarolina => "NC",
            Self::NorthDakota => "ND",
            Self::Ohio => "OH",
            Self::Oklahoma => "OK",
            Self::Oregon => "OR",
            Self::Pennsylvania => "PA",
            Self::RhodeIsland => "RI",
            Self::SouthCarolina => "SC",
            Self::SouthDakota => "SD",
            Self::Tennessee => "TN",
            Self::Texas => "TX",
            Self::Utah => "UT",
            Self::Vermont => "VT",
            Self::Virginia => "VA",
            Self::Washington => "WA",
            Self::WestVirginia => "WV",
            Self::Wisconsin => "WI",
            Self::Wyoming => "WY",
            Self::DistrictOfColumbia => "DC",
        }
    }
}

/// Federal preemption type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreemptionType {
    /// Express preemption: Federal law explicitly preempts state law
    Express,
    /// Field preemption: Federal regulation is so pervasive it occupies the field
    Field,
    /// Conflict preemption: State law directly conflicts with federal law
    Conflict,
    /// Obstacle preemption: State law stands as obstacle to federal objectives
    Obstacle,
}

impl PreemptionType {
    /// Returns the preemption type name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Express => "Express Preemption",
            Self::Field => "Field Preemption",
            Self::Conflict => "Conflict Preemption",
            Self::Obstacle => "Obstacle Preemption",
        }
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies
    known_hierarchies: HashMap<String, LegalHierarchy>,
    /// Federal preemption relationships (federal_id -> set of preempted state statutes)
    preemption_rules: HashMap<String, HashSet<String>>,
    /// State savings clauses (statute_id that preserves state law)
    savings_clauses: HashSet<String>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: LegalHierarchy) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Registers a preemption rule.
    pub fn register_preemption(
        &mut self,
        federal_statute_id: impl Into<String>,
        state_statute_id: impl Into<String>,
    ) {
        self.preemption_rules
            .entry(federal_statute_id.into())
            .or_default()
            .insert(state_statute_id.into());
    }

    /// Registers a savings clause.
    pub fn register_savings_clause(&mut self, statute_id: impl Into<String>) {
        self.savings_clauses.insert(statute_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> LegalHierarchy {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| LegalHierarchy::from_statute(statute))
    }

    /// Checks if federal statute preempts state statute.
    #[must_use]
    pub fn is_preempted(&self, federal_id: &str, state_id: &str) -> bool {
        self.preemption_rules
            .get(federal_id)
            .is_some_and(|preempted| preempted.contains(state_id))
    }

    /// Checks if a statute has a savings clause.
    #[must_use]
    pub fn has_savings_clause(&self, statute_id: &str) -> bool {
        self.savings_clauses.contains(statute_id)
    }
}

impl Default for UsStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl UsStatuteVerifier {
    /// Creates a new US statute verifier with Constitutional principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = us_constitutional_principles();
        let inner = StatuteVerifier::with_principles(principles);

        let mut hierarchy_rules = HierarchyRules::new();
        setup_federal_hierarchy(&mut hierarchy_rules);

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

    /// Registers a preemption rule.
    pub fn register_preemption(
        &mut self,
        federal_statute_id: impl Into<String>,
        state_statute_id: impl Into<String>,
    ) {
        self.hierarchy_rules
            .register_preemption(federal_statute_id, state_statute_id);
    }

    /// Verifies a set of statutes with US-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        // Additional US-specific checks
        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_federal_preemption(statutes));
        result.merge(self.check_employment_law_consistency(statutes));

        result
    }

    /// Verifies a single statute.
    #[must_use]
    pub fn verify_single(&self, statute: &Statute) -> VerificationResult {
        self.verify(std::slice::from_ref(statute))
    }

    /// Checks legal hierarchy (Supremacy Clause) compliance.
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

                    // Check for federal-state conflicts (Supremacy Clause)
                    if higher_level.is_federal() && lower_level.is_state() {
                        if self.hierarchy_rules.has_savings_clause(&higher.id) {
                            result = result.with_suggestion(format!(
                                "Savings clause: {} preserves state law {} despite potential conflict",
                                higher.id, lower.id
                            ));
                        } else {
                            result = result.with_warning(format!(
                                "Supremacy Clause issue: {} ({}) may conflict with {} ({})",
                                lower.id,
                                lower_level.name(),
                                higher.id,
                                higher_level.name()
                            ));
                        }
                    } else {
                        result = result.with_warning(format!(
                            "Hierarchy conflict: {} ({}) may conflict with {} ({})",
                            lower.id,
                            lower_level.name(),
                            higher.id,
                            higher_level.name()
                        ));
                    }
                }
            }
        }

        result
    }

    /// Checks for federal preemption issues.
    fn check_federal_preemption(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let federal_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.get_hierarchy(s).is_federal())
            .collect();

        let state_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.hierarchy_rules.get_hierarchy(s).is_state())
            .collect();

        for federal in &federal_statutes {
            for state in &state_statutes {
                if self.hierarchy_rules.is_preempted(&federal.id, &state.id) {
                    result = result.with_warning(format!(
                        "Federal preemption: {} is preempted by {}",
                        state.id, federal.id
                    ));
                }
            }
        }

        result
    }

    /// Checks US employment law consistency.
    fn check_employment_law_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let employment_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.is_employment_law_statute(s))
            .collect();

        // Check for FLSA-related statutes
        let flsa_statutes: Vec<_> = employment_statutes
            .iter()
            .filter(|s| {
                s.id.contains("FLSA")
                    || s.title.contains("Fair Labor Standards")
                    || s.title.contains("minimum wage")
                    || s.title.contains("overtime")
            })
            .collect();

        if flsa_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "FLSA consistency check: {} wage/hour statutes found",
                flsa_statutes.len()
            ));
        }

        // Check for Title VII-related statutes
        let title_vii_statutes: Vec<_> = employment_statutes
            .iter()
            .filter(|s| {
                s.id.contains("TitleVII")
                    || s.title.contains("Title VII")
                    || s.title.contains("discrimination")
                    || s.title.contains("Civil Rights Act")
            })
            .collect();

        if title_vii_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "Title VII consistency check: {} discrimination statutes found",
                title_vii_statutes.len()
            ));
        }

        result
    }

    /// Checks if a statute is employment law related.
    fn is_employment_law_statute(&self, statute: &Statute) -> bool {
        let id = &statute.id;
        let title = statute.title.to_lowercase();

        id.contains("FLSA")
            || id.contains("FMLA")
            || id.contains("TitleVII")
            || id.contains("ADA")
            || id.contains("ADEA")
            || id.contains("OSHA")
            || id.contains("ERISA")
            || id.contains("NLRA")
            || title.contains("employment")
            || title.contains("labor")
            || title.contains("wage")
            || title.contains("discrimination")
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

/// Creates constitutional principles based on the US Constitution.
///
/// Returns principles for key Constitutional protections:
/// - 1st Amendment: Free Speech, Religion
/// - 4th Amendment: Search & Seizure
/// - 5th Amendment: Due Process, Self-Incrimination
/// - 14th Amendment: Equal Protection, Due Process
/// - Supremacy Clause: Federal Preemption
#[must_use]
pub fn us_constitutional_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        // 1st Amendment: Free Speech
        ConstitutionalPrinciple {
            id: "1st-amendment-speech".to_string(),
            name: "First Amendment - Free Speech".to_string(),
            description: "Congress shall make no law... abridging the freedom of speech, or of the press.".to_string(),
            check: PrincipleCheck::FreedomOfExpression,
        },
        // 1st Amendment: Religion
        ConstitutionalPrinciple {
            id: "1st-amendment-religion".to_string(),
            name: "First Amendment - Religion".to_string(),
            description: "Congress shall make no law respecting an establishment of religion, or prohibiting the free exercise thereof.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Religious freedom protection".to_string(),
            },
        },
        // 4th Amendment: Search & Seizure
        ConstitutionalPrinciple {
            id: "4th-amendment".to_string(),
            name: "Fourth Amendment - Search & Seizure".to_string(),
            description: "The right of the people to be secure in their persons, houses, papers, and effects, against unreasonable searches and seizures, shall not be violated.".to_string(),
            check: PrincipleCheck::PrivacyImpact,
        },
        // 5th Amendment: Due Process
        ConstitutionalPrinciple {
            id: "5th-amendment-due-process".to_string(),
            name: "Fifth Amendment - Due Process".to_string(),
            description: "No person shall... be deprived of life, liberty, or property, without due process of law.".to_string(),
            check: PrincipleCheck::DueProcess,
        },
        // 5th Amendment: Self-Incrimination
        ConstitutionalPrinciple {
            id: "5th-amendment-self-incrimination".to_string(),
            name: "Fifth Amendment - Self-Incrimination".to_string(),
            description: "No person shall be compelled in any criminal case to be a witness against himself.".to_string(),
            check: PrincipleCheck::RequiresProcedure,
        },
        // 5th Amendment: Takings
        ConstitutionalPrinciple {
            id: "5th-amendment-takings".to_string(),
            name: "Fifth Amendment - Takings Clause".to_string(),
            description: "Nor shall private property be taken for public use, without just compensation.".to_string(),
            check: PrincipleCheck::PropertyRights,
        },
        // 14th Amendment: Equal Protection
        ConstitutionalPrinciple {
            id: "14th-amendment-equal-protection".to_string(),
            name: "Fourteenth Amendment - Equal Protection".to_string(),
            description: "No State shall... deny to any person within its jurisdiction the equal protection of the laws.".to_string(),
            check: PrincipleCheck::EqualProtection,
        },
        // 14th Amendment: Due Process
        ConstitutionalPrinciple {
            id: "14th-amendment-due-process".to_string(),
            name: "Fourteenth Amendment - Due Process".to_string(),
            description: "Nor shall any State deprive any person of life, liberty, or property, without due process of law.".to_string(),
            check: PrincipleCheck::ProceduralDueProcess,
        },
        // Supremacy Clause
        ConstitutionalPrinciple {
            id: "supremacy-clause".to_string(),
            name: "Supremacy Clause (Article VI)".to_string(),
            description: "This Constitution, and the Laws of the United States... shall be the supreme Law of the Land.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Federal supremacy compliance".to_string(),
            },
        },
        // Commerce Clause
        ConstitutionalPrinciple {
            id: "commerce-clause".to_string(),
            name: "Commerce Clause (Article I, Section 8)".to_string(),
            description: "Congress shall have Power... To regulate Commerce with foreign Nations, and among the several States.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Interstate commerce regulation".to_string(),
            },
        },
    ]
}

/// Sets up federal hierarchy relationships.
fn setup_federal_hierarchy(rules: &mut HierarchyRules) {
    // Register Federal Statutes
    rules.register_hierarchy("FLSA", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("FMLA", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("TitleVII", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("ADA", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("ADEA", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("OSHA", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("ERISA", LegalHierarchy::FederalStatute);
    rules.register_hierarchy("NLRA", LegalHierarchy::FederalStatute);

    // Federal preemption examples
    // ERISA preempts most state employee benefit laws
    rules.register_preemption("ERISA", "state_benefit_law");

    // OSHA preempts state occupational safety in some areas
    // (but many states have approved state plans)

    // Savings clauses - FLSA has a savings clause for more protective state laws
    rules.register_savings_clause("FLSA");
}

/// Verification report for US law compliance.
#[derive(Debug, Clone)]
pub struct UsVerificationReport {
    /// Base verification result
    pub result: VerificationResult,
    /// Constitutional issues
    pub constitutional_issues: Vec<ConstitutionalIssue>,
    /// Preemption issues
    pub preemption_issues: Vec<PreemptionIssue>,
    /// Federal-state conflicts
    pub federalism_notes: Vec<String>,
}

/// Constitutional compliance issue.
#[derive(Debug, Clone)]
pub struct ConstitutionalIssue {
    /// Statute ID with the issue
    pub statute_id: String,
    /// Amendment or clause potentially violated
    pub amendment: String,
    /// Description of the issue
    pub description: String,
    /// Severity of the issue
    pub severity: Severity,
}

/// Federal preemption issue.
#[derive(Debug, Clone)]
pub struct PreemptionIssue {
    /// Federal statute that preempts
    pub federal_statute_id: String,
    /// State statute that is preempted
    pub state_statute_id: String,
    /// Type of preemption
    pub preemption_type: PreemptionType,
    /// Description
    pub description: String,
}

impl UsVerificationReport {
    /// Creates a new report from verification result.
    #[must_use]
    pub fn new(result: VerificationResult) -> Self {
        Self {
            result,
            constitutional_issues: Vec::new(),
            preemption_issues: Vec::new(),
            federalism_notes: Vec::new(),
        }
    }

    /// Checks if the verification passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.result.passed && self.constitutional_issues.is_empty()
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

        if !self.constitutional_issues.is_empty() {
            summary.push_str("\nConstitutional Issues:\n");
            for issue in &self.constitutional_issues {
                summary.push_str(&format!(
                    "  - {} ({}): {}\n",
                    issue.statute_id, issue.amendment, issue.description
                ));
            }
        }

        if !self.preemption_issues.is_empty() {
            summary.push_str("\nPreemption Issues:\n");
            for issue in &self.preemption_issues {
                summary.push_str(&format!(
                    "  - {} preempts {} ({}): {}\n",
                    issue.federal_statute_id,
                    issue.state_statute_id,
                    issue.preemption_type.name(),
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
    use crate::reasoning::statute_adapter::all_federal_statutes;
    use legalis_core::{Effect, EffectType, Statute};

    // ========================================================================
    // UsStatuteVerifier tests
    // ========================================================================

    #[test]
    fn test_verifier_creation() {
        let verifier = UsStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_federal_statutes() {
        let verifier = UsStatuteVerifier::new();
        let statutes = all_federal_statutes();

        let result = verifier.verify(&statutes);
        assert!(result.passed || !result.errors.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = UsStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = UsStatuteVerifier::new();
        let statute = Statute::new(
            "TEST_001",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = verifier.verify_single(&statute);
        assert!(result.passed);
    }

    // ========================================================================
    // LegalHierarchy tests
    // ========================================================================

    #[test]
    fn test_hierarchy_ordering() {
        assert!(LegalHierarchy::UsConstitution.is_higher_than(&LegalHierarchy::FederalStatute));
        assert!(LegalHierarchy::FederalStatute.is_higher_than(&LegalHierarchy::FederalRegulation));
        assert!(LegalHierarchy::FederalRegulation.is_higher_than(&LegalHierarchy::ExecutiveOrder));
        assert!(LegalHierarchy::ExecutiveOrder.is_higher_than(&LegalHierarchy::StateConstitution));
        assert!(LegalHierarchy::StateConstitution.is_higher_than(&LegalHierarchy::StateStatute));
        assert!(LegalHierarchy::StateStatute.is_higher_than(&LegalHierarchy::StateRegulation));
        assert!(LegalHierarchy::StateRegulation.is_higher_than(&LegalHierarchy::LocalOrdinance));
    }

    #[test]
    fn test_hierarchy_from_statute_federal() {
        let statute = Statute::new(
            "FLSA",
            "Fair Labor Standards Act",
            Effect::new(EffectType::Obligation, "Test"),
        );
        assert_eq!(
            LegalHierarchy::from_statute(&statute),
            LegalHierarchy::FederalStatute
        );
    }

    #[test]
    fn test_hierarchy_from_statute_state() {
        let statute = Statute::new(
            "CA_LaborCode",
            "California Labor Code",
            Effect::new(EffectType::Grant, "Test"),
        );
        assert_eq!(
            LegalHierarchy::from_statute(&statute),
            LegalHierarchy::StateStatute
        );
    }

    #[test]
    fn test_hierarchy_is_federal() {
        assert!(LegalHierarchy::UsConstitution.is_federal());
        assert!(LegalHierarchy::FederalStatute.is_federal());
        assert!(!LegalHierarchy::StateStatute.is_federal());
    }

    #[test]
    fn test_hierarchy_is_state() {
        assert!(LegalHierarchy::StateConstitution.is_state());
        assert!(LegalHierarchy::StateStatute.is_state());
        assert!(!LegalHierarchy::FederalStatute.is_state());
    }

    // ========================================================================
    // HierarchyRules tests
    // ========================================================================

    #[test]
    fn test_register_preemption() {
        let mut rules = HierarchyRules::new();
        rules.register_preemption("ERISA", "state_benefit_law");

        assert!(rules.is_preempted("ERISA", "state_benefit_law"));
        assert!(!rules.is_preempted("FLSA", "state_wage_law"));
    }

    #[test]
    fn test_register_savings_clause() {
        let mut rules = HierarchyRules::new();
        rules.register_savings_clause("FLSA");

        assert!(rules.has_savings_clause("FLSA"));
        assert!(!rules.has_savings_clause("ERISA"));
    }

    #[test]
    fn test_register_hierarchy() {
        let mut rules = HierarchyRules::new();
        rules.register_hierarchy("TEST_001", LegalHierarchy::StateStatute);

        let statute = Statute::new("TEST_001", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(rules.get_hierarchy(&statute), LegalHierarchy::StateStatute);
    }

    // ========================================================================
    // Constitutional principles tests
    // ========================================================================

    #[test]
    fn test_us_constitutional_principles() {
        let principles = us_constitutional_principles();

        assert!(!principles.is_empty());

        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"1st-amendment-speech"));
        assert!(principle_ids.contains(&"4th-amendment"));
        assert!(principle_ids.contains(&"5th-amendment-due-process"));
        assert!(principle_ids.contains(&"14th-amendment-equal-protection"));
        assert!(principle_ids.contains(&"supremacy-clause"));
    }

    #[test]
    fn test_constitutional_principles_have_descriptions() {
        let principles = us_constitutional_principles();

        for principle in &principles {
            assert!(!principle.name.is_empty());
            assert!(!principle.description.is_empty());
        }
    }

    // ========================================================================
    // UsVerificationReport tests
    // ========================================================================

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = UsVerificationReport::new(result);
        assert!(report.passed());
    }

    #[test]
    fn test_verification_report_with_issues() {
        let result = VerificationResult::pass();
        let mut report = UsVerificationReport::new(result);
        report.constitutional_issues.push(ConstitutionalIssue {
            statute_id: "TEST".to_string(),
            amendment: "14th Amendment".to_string(),
            description: "Test issue".to_string(),
            severity: Severity::Warning,
        });
        assert!(!report.passed());
    }

    #[test]
    fn test_verification_report_summary() {
        let result = VerificationResult::pass();
        let report = UsVerificationReport::new(result);
        let summary = report.summary();
        assert!(summary.contains("Verification Result: PASSED"));
    }

    // ========================================================================
    // Integration tests
    // ========================================================================

    #[test]
    fn test_federal_hierarchy_setup() {
        let verifier = UsStatuteVerifier::new();

        let statute = Statute::new("FLSA", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(
            verifier.hierarchy_rules.get_hierarchy(&statute),
            LegalHierarchy::FederalStatute
        );
    }

    #[test]
    fn test_savings_clause_setup() {
        let verifier = UsStatuteVerifier::new();
        assert!(verifier.hierarchy_rules.has_savings_clause("FLSA"));
    }

    #[test]
    fn test_preemption_setup() {
        let verifier = UsStatuteVerifier::new();
        assert!(
            verifier
                .hierarchy_rules
                .is_preempted("ERISA", "state_benefit_law")
        );
    }
}
