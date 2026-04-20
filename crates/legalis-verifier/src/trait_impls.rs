//! # AmbiguityType - Trait Implementations
//!
//! This module contains trait implementations for `AmbiguityType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Display`
//! - `Display`
//! - `Display`
//! - `Default`
//! - `Default`
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BatchVerificationResult, ComplexityTrend, CrossReferenceErrorType, ImpactAssessment,
    OverlapArea, RiskLevel, StatuteVerifier, VerificationBudget,
};
use super::types_3::{
    AmbiguityType, CiPlatform, ConflictNature, ConflictType, CrossReferenceError, CtlFormula,
    GapType, LazyVerificationConfig, MechanismAnalysis, MechanismProperty, PreCommitHook,
    ProofCache, ProofStepType, ReportOutputFormat,
};
use super::types_4::{
    CtlStarFormula, CtlStarPathFormula, DependencyGraph, EvolutionTracker, ImpactLevel,
    IncrementalState, InteractionType, LtlFormula, RedundancyType, ReportScheduler, Severity,
    TransitionSystem,
};
use super::types_5::{ComplexityLevel, NotificationConfig, PatternType, StatuteChange};

impl std::fmt::Display for AmbiguityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VagueTerm => write!(f, "Vague Term"),
            Self::OverlappingConditions => write!(f, "Overlapping Conditions"),
            Self::UnclearEffect => write!(f, "Unclear Effect"),
            Self::MissingDiscretion => write!(f, "Missing Discretion"),
            Self::TemporalAmbiguity => write!(f, "Temporal Ambiguity"),
            Self::ImplicitAssumption => write!(f, "Implicit Assumption"),
            Self::QuantifierAmbiguity => write!(f, "Quantifier Ambiguity"),
        }
    }
}

impl Default for BatchVerificationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CiPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GitHubActions => write!(f, "GitHub Actions"),
            Self::GitLabCI => write!(f, "GitLab CI"),
            Self::Jenkins => write!(f, "Jenkins"),
            Self::CircleCI => write!(f, "CircleCI"),
            Self::TravisCI => write!(f, "Travis CI"),
        }
    }
}

impl std::fmt::Display for ComplexityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple => write!(f, "Simple"),
            Self::Moderate => write!(f, "Moderate"),
            Self::Complex => write!(f, "Complex"),
            Self::VeryComplex => write!(f, "Very Complex"),
        }
    }
}

impl std::fmt::Display for ComplexityTrend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Increasing => write!(f, "Increasing"),
            Self::Decreasing => write!(f, "Decreasing"),
            Self::Stable => write!(f, "Stable"),
        }
    }
}

impl std::fmt::Display for ConflictNature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectOpposition => write!(f, "Direct Opposition"),
            Self::ResourceCompetition => write!(f, "Resource Competition"),
            Self::InterpretationDifference => write!(f, "Interpretation Difference"),
            Self::JurisdictionalOverlap => write!(f, "Jurisdictional Overlap"),
            Self::PowerImbalance => write!(f, "Power Imbalance"),
        }
    }
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EffectConflict => write!(f, "Effect Conflict"),
            Self::JurisdictionalOverlap => write!(f, "Jurisdictional Overlap"),
            Self::TemporalConflict => write!(f, "Temporal Conflict"),
            Self::HierarchyViolation => write!(f, "Hierarchy Violation"),
            Self::IdCollision => write!(f, "ID Collision"),
        }
    }
}

impl std::fmt::Display for CrossReferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            CrossReferenceErrorType::NotFound => {
                write!(
                    f,
                    "Statute '{}' references non-existent statute '{}'",
                    self.source_statute_id, self.referenced_statute_id
                )
            }
            CrossReferenceErrorType::CircularReference => {
                write!(
                    f,
                    "Statute '{}' creates circular reference with '{}'",
                    self.source_statute_id, self.referenced_statute_id
                )
            }
            CrossReferenceErrorType::Ambiguous => {
                write!(
                    f,
                    "Statute '{}' has ambiguous reference to '{}'",
                    self.source_statute_id, self.referenced_statute_id
                )
            }
        }
    }
}

impl std::fmt::Display for CtlFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Implies(left, right) => write!(f, "({} → {})", left, right),
            Self::ExistsNext(formula) => write!(f, "EX({})", formula),
            Self::AllNext(formula) => write!(f, "AX({})", formula),
            Self::ExistsEventually(formula) => write!(f, "EF({})", formula),
            Self::AllEventually(formula) => write!(f, "AF({})", formula),
            Self::ExistsAlways(formula) => write!(f, "EG({})", formula),
            Self::AllAlways(formula) => write!(f, "AG({})", formula),
            Self::ExistsUntil(left, right) => write!(f, "E({} U {})", left, right),
            Self::AllUntil(left, right) => write!(f, "A({} U {})", left, right),
        }
    }
}

impl std::fmt::Display for CtlStarFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Implies(left, right) => write!(f, "({} → {})", left, right),
            Self::Exists(path) => write!(f, "E({})", path),
            Self::All(path) => write!(f, "A({})", path),
        }
    }
}

impl std::fmt::Display for CtlStarPathFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::State(formula) => write!(f, "{}", formula),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Next(formula) => write!(f, "X({})", formula),
            Self::Eventually(formula) => write!(f, "F({})", formula),
            Self::Always(formula) => write!(f, "G({})", formula),
            Self::Until(left, right) => write!(f, "({} U {})", left, right),
            Self::Release(left, right) => write!(f, "({} R {})", left, right),
        }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EvolutionTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AgeGap => write!(f, "Age Gap"),
            Self::IncomeGap => write!(f, "Income Gap"),
            Self::JurisdictionGap => write!(f, "Jurisdiction Gap"),
            Self::TemporalGap => write!(f, "Temporal Gap"),
            Self::EffectGap => write!(f, "Effect Gap"),
            Self::LogicalGap => write!(f, "Logical Gap"),
        }
    }
}

impl Default for ImpactAssessment {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ImpactLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
}

impl Default for IncrementalState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for InteractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Modification => write!(f, "Modification"),
            Self::Extension => write!(f, "Extension"),
            Self::Complementary => write!(f, "Complementary"),
            Self::Supersession => write!(f, "Supersession"),
            Self::MutualDependency => write!(f, "Mutual Dependency"),
            Self::Contradiction => write!(f, "Contradiction"),
            Self::Overlap => write!(f, "Overlap"),
        }
    }
}

impl Default for LazyVerificationConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for LtlFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Implies(left, right) => write!(f, "({} → {})", left, right),
            Self::Next(formula) => write!(f, "X({})", formula),
            Self::Eventually(formula) => write!(f, "F({})", formula),
            Self::Always(formula) => write!(f, "G({})", formula),
            Self::Until(left, right) => write!(f, "({} U {})", left, right),
            Self::Release(left, right) => write!(f, "({} R {})", left, right),
        }
    }
}

impl Default for MechanismAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MechanismProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncentiveCompatibility => write!(f, "Incentive Compatibility"),
            Self::IndividualRationality => write!(f, "Individual Rationality"),
            Self::BudgetBalance => write!(f, "Budget Balance"),
            Self::ParetoEfficiency => write!(f, "Pareto Efficiency"),
            Self::StrategyProofness => write!(f, "Strategy-Proofness"),
            Self::NonDictatorship => write!(f, "Non-Dictatorship"),
        }
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for OverlapArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jurisdiction => write!(f, "Jurisdiction"),
            Self::SubjectMatter => write!(f, "Subject Matter"),
            Self::Temporal => write!(f, "Temporal"),
            Self::Population => write!(f, "Population"),
            Self::Enforcement => write!(f, "Enforcement"),
        }
    }
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AgeEligibility => write!(f, "Age Eligibility"),
            Self::IncomeQualification => write!(f, "Income Qualification"),
            Self::AgeAndIncome => write!(f, "Age and Income"),
            Self::ProhibitionWithExceptions => write!(f, "Prohibition with Exceptions"),
            Self::TemporalRestriction => write!(f, "Temporal Restriction"),
            Self::JurisdictionalPattern => write!(f, "Jurisdictional Pattern"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}

impl Default for PreCommitHook {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ProofCache {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ProofStepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Premise => write!(f, "Premise"),
            Self::Deduction => write!(f, "Deduction"),
            Self::Contradiction => write!(f, "Contradiction"),
            Self::SmtResult => write!(f, "SMT Result"),
            Self::Simplification => write!(f, "Simplification"),
            Self::Conclusion => write!(f, "Conclusion"),
        }
    }
}

impl std::fmt::Display for RedundancyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Duplicate => write!(f, "Duplicate"),
            Self::Subsumed => write!(f, "Subsumed"),
            Self::OverlappingConditions => write!(f, "Overlapping Conditions"),
            Self::EquivalentEffects => write!(f, "Equivalent Effects"),
        }
    }
}

impl std::fmt::Display for ReportOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportOutputFormat::Markdown => write!(f, "markdown"),
            ReportOutputFormat::Html => write!(f, "html"),
            ReportOutputFormat::Json => write!(f, "json"),
            #[cfg(feature = "pdf")]
            ReportOutputFormat::Pdf => write!(f, "pdf"),
        }
    }
}

impl Default for ReportScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimal => write!(f, "Minimal"),
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Warning => write!(f, "Warning"),
            Self::Error => write!(f, "Error"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

impl std::fmt::Display for StatuteChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TitleChanged { old, new } => {
                write!(f, "Title changed from '{}' to '{}'", old, new)
            }
            Self::DescriptionChanged { old, new } => {
                write!(f, "Description changed from {:?} to {:?}", old, new)
            }
            Self::JurisdictionChanged { old, new } => {
                write!(f, "Jurisdiction changed from {:?} to {:?}", old, new)
            }
            Self::EffectChanged { old, new } => {
                write!(f, "Effect changed from '{}' to '{}'", old, new)
            }
            Self::PreconditionsChanged {
                old_count,
                new_count,
            } => {
                write!(
                    f,
                    "Preconditions changed from {} to {} conditions",
                    old_count, new_count
                )
            }
            Self::EnactmentDateChanged { old, new } => {
                write!(f, "Enactment date changed from {:?} to {:?}", old, new)
            }
            Self::EffectiveDateChanged { old, new } => {
                write!(f, "Effective date changed from {:?} to {:?}", old, new)
            }
        }
    }
}

impl Default for StatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TransitionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for VerificationBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}
