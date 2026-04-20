//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Change;

/// Effect scope change analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectScopeChange {
    /// The effect scope has expanded (more people/situations affected).
    Expanded,
    /// The effect scope has narrowed (fewer people/situations affected).
    Narrowed,
    /// The effect scope has changed in incomparable ways.
    Changed,
    /// The effect scope is unchanged.
    Unchanged,
}
/// Types of relationships between statutes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatuteRelationship {
    /// One statute references another (e.g., "as defined in...")
    References,
    /// Statutes have overlapping conditions
    OverlappingConditions,
    /// Statutes have related effects
    RelatedEffects,
    /// One statute is a special case of another
    SpecialCase,
    /// Statutes are mutually exclusive
    MutuallyExclusive,
    /// Part of the same legislative package
    SamePackage,
}
/// Complete regulatory compliance analysis.
#[derive(Debug, Clone)]
pub struct RegulatoryComplianceAnalysis {
    /// Overall compliance impact level.
    pub overall_impact: ComplianceImpactLevel,
    /// Specific compliance impacts.
    pub impacts: Vec<ComplianceImpact>,
    /// Compliance summary.
    pub summary: String,
}
/// Impact on a specific stakeholder group.
#[derive(Debug, Clone)]
pub struct StakeholderImpact {
    /// Type of stakeholder.
    pub stakeholder_type: StakeholderType,
    /// Impact level (0-100).
    pub impact_level: u8,
    /// Description of the impact.
    pub description: String,
    /// Estimated number affected.
    pub estimated_affected: Option<u64>,
    /// Recommended actions for this stakeholder.
    pub recommended_actions: Vec<String>,
}
/// Backward compatibility score (0-100, where 100 is fully compatible).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BackwardCompatibilityScore {
    /// Overall compatibility score (0-100).
    pub overall: u8,
    /// Data compatibility (0-100).
    pub data: u8,
    /// API compatibility (0-100).
    pub api: u8,
    /// Behavioral compatibility (0-100).
    pub behavioral: u8,
}
/// Classification of change impact on compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeCompatibility {
    /// Change does not affect behavior (e.g., renaming, formatting).
    NonBreaking,
    /// Change relaxes requirements (backward compatible).
    BackwardCompatible,
    /// Change tightens requirements (forward compatible).
    ForwardCompatible,
    /// Change breaks compatibility in both directions.
    Breaking,
}
/// Complete stakeholder impact analysis.
#[derive(Debug, Clone)]
pub struct StakeholderAnalysis {
    /// Impacts by stakeholder type.
    pub impacts: Vec<StakeholderImpact>,
    /// Overall stakeholder impact summary.
    pub summary: String,
}
/// Overall impact level of cross-statute changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrossStatuteImpactLevel {
    /// No cross-statute impact detected
    None,
    /// Minor impact (informational)
    Low,
    /// Moderate impact (review recommended)
    Medium,
    /// High impact (coordination required)
    High,
    /// Critical impact (simultaneous amendment needed)
    Critical,
}
/// Change impact score (0-100 scale).
///
/// This provides a quantitative measure of how impactful a change is.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ImpactScore {
    /// Overall impact score (0-100).
    pub overall: u8,
    /// Eligibility impact (0-100).
    pub eligibility: u8,
    /// Outcome impact (0-100).
    pub outcome: u8,
    /// Process impact (0-100).
    pub process: u8,
    /// Population impact (0-100).
    pub population: u8,
}
impl ImpactScore {
    /// Creates a new impact score with all values set to zero.
    pub fn new() -> Self {
        Self {
            overall: 0,
            eligibility: 0,
            outcome: 0,
            process: 0,
            population: 0,
        }
    }
}
/// Cross-statute impact analysis.
///
/// Analyzes how changes to one statute might affect other related statutes.
#[derive(Debug, Clone)]
pub struct CrossStatuteImpact {
    /// The statute being changed
    pub source_statute_id: String,
    /// Potentially affected statutes
    pub affected_statutes: Vec<AffectedStatute>,
    /// Overall impact level
    pub impact_level: CrossStatuteImpactLevel,
}
/// Migration complexity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationComplexity {
    /// No migration needed.
    None,
    /// Trivial migration (documentation update only).
    Trivial,
    /// Simple migration (minor code changes).
    Simple,
    /// Moderate migration (significant changes required).
    Moderate,
    /// Complex migration (major refactoring required).
    Complex,
    /// Very complex migration (complete redesign may be needed).
    VeryComplex,
}
/// Summary of compatibility analysis.
#[derive(Debug)]
pub struct CompatibilitySummary {
    pub total_changes: usize,
    pub breaking_changes: usize,
    pub backward_compatible_changes: usize,
    pub forward_compatible_changes: usize,
    pub non_breaking_changes: usize,
    pub overall_compatibility: ChangeCompatibility,
}
/// Regulatory compliance area.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComplianceArea {
    /// Data protection and privacy.
    DataProtection,
    /// Financial regulations.
    Financial,
    /// Labor and employment.
    Labor,
    /// Environmental regulations.
    Environmental,
    /// Health and safety.
    HealthSafety,
    /// Consumer protection.
    ConsumerProtection,
    /// General administrative compliance.
    Administrative,
}
/// Result of effect scope analysis.
#[derive(Debug, Clone)]
pub struct EffectScopeAnalysis {
    /// Overall scope change
    pub scope_change: EffectScopeChange,
    /// Estimated impact on population (percentage)
    pub population_impact: Option<f64>,
    /// Explanation
    pub explanation: String,
}
/// Stakeholder type affected by statute changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StakeholderType {
    /// Individual citizens/residents.
    Citizens,
    /// Businesses and corporations.
    Businesses,
    /// Government agencies.
    GovernmentAgencies,
    /// Legal professionals.
    LegalProfessionals,
    /// Social service providers.
    ServiceProviders,
    /// Advocacy groups.
    AdvocacyGroups,
}
/// Analysis result for a change.
#[derive(Debug, Clone)]
pub struct ChangeAnalysis {
    /// The change being analyzed.
    pub change: Change,
    /// Compatibility classification.
    pub compatibility: ChangeCompatibility,
    /// Whether this change relaxes conditions.
    pub relaxes_conditions: bool,
    /// Whether this change tightens conditions.
    pub tightens_conditions: bool,
    /// Explanation of the analysis.
    pub explanation: String,
}
/// Result of logical equivalence analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquivalenceResult {
    /// Changes are logically equivalent.
    Equivalent,
    /// Changes are not equivalent.
    NotEquivalent,
    /// Cannot determine equivalence.
    Unknown,
}
/// A statute potentially affected by changes to another.
#[derive(Debug, Clone)]
pub struct AffectedStatute {
    /// ID of the affected statute
    pub statute_id: String,
    /// Type of relationship
    pub relationship: StatuteRelationship,
    /// Reason for potential impact
    pub impact_reason: String,
    /// Recommended action
    pub recommended_action: String,
}
/// Impact on regulatory compliance.
#[derive(Debug, Clone)]
pub struct ComplianceImpact {
    /// Affected compliance area.
    pub area: ComplianceArea,
    /// Impact level.
    pub impact_level: ComplianceImpactLevel,
    /// Description of compliance requirements.
    pub requirements: Vec<String>,
    /// Deadline for compliance (if applicable).
    pub deadline_days: Option<u32>,
}
/// Regulatory compliance impact level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplianceImpactLevel {
    /// No compliance impact.
    None,
    /// Minor compliance adjustments needed.
    Minor,
    /// Moderate compliance changes required.
    Moderate,
    /// Major compliance overhaul needed.
    Major,
    /// Critical compliance risk.
    Critical,
}
/// Migration effort estimation.
#[derive(Debug, Clone)]
pub struct MigrationEffort {
    /// Complexity level.
    pub complexity: MigrationComplexity,
    /// Estimated effort in person-hours.
    pub estimated_hours: f64,
    /// Migration steps required.
    pub migration_steps: Vec<String>,
    /// Risks associated with migration.
    pub risks: Vec<String>,
    /// Recommended migration strategy.
    pub strategy: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionComparison {
    /// Conditions are logically equivalent.
    Equivalent,
    /// New condition is more relaxed (easier to satisfy).
    Relaxed,
    /// New condition is tightened (harder to satisfy).
    Tightened,
    /// Conditions are different in incomparable ways.
    Different,
}
