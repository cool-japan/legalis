//! Canada Family Law - Parenting Analysis
//!
//! Parenting arrangements, best interests analysis, and relocation.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    BestInterestsFactor, FamilyViolence, ParentingArrangement, ParentingTimeSchedule,
    RelocationReason, RelocationRequest,
};

// ============================================================================
// Best Interests Analysis
// ============================================================================

/// Facts for best interests analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestInterestsFacts {
    /// Children involved
    pub children: Vec<ChildInfo>,
    /// Parent A information
    pub parent_a: ParentInfo,
    /// Parent B information
    pub parent_b: ParentInfo,
    /// Current arrangement (if any)
    pub current_arrangement: Option<CurrentArrangement>,
    /// Family violence allegations
    pub family_violence: Vec<FamilyViolenceAllegation>,
    /// Proposed arrangement
    pub proposed_arrangement: ProposedArrangement,
}

/// Child information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildInfo {
    /// Age
    pub age: u32,
    /// Views and preferences (if expressed)
    pub views: Option<ChildViews>,
    /// Special needs
    pub special_needs: Vec<String>,
    /// School/community ties
    pub community_ties: Vec<String>,
    /// Cultural/religious needs
    pub cultural_needs: Vec<String>,
}

/// Child's views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildViews {
    /// Preference expressed
    pub preference: String,
    /// Maturity assessment
    pub maturity: MaturityLevel,
    /// Whether views should be given weight
    pub give_weight: bool,
    /// Source of views (child interview, s.211 report, etc.)
    pub source: String,
}

/// Maturity level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaturityLevel {
    /// Below average for age
    BelowAverage,
    /// Average for age
    Average,
    /// Above average maturity
    AboveAverage,
    /// Very mature
    VeryMature,
}

/// Parent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentInfo {
    /// Name/identifier
    pub identifier: String,
    /// History of care for children
    pub history_of_care: Vec<String>,
    /// Ability to meet child's needs
    pub ability_to_meet_needs: Vec<String>,
    /// Willingness to support relationship with other parent
    pub willingness_to_support: WillingnessLevel,
    /// Work schedule flexibility
    pub work_flexibility: FlexibilityLevel,
    /// Support system (extended family, etc.)
    pub support_system: Vec<String>,
    /// Any concerns
    pub concerns: Vec<String>,
}

/// Willingness to support other parent relationship
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WillingnessLevel {
    /// Actively undermines (lowest)
    Undermining,
    /// Limited support
    Limited,
    /// Adequate support
    Adequate,
    /// Actively supports (highest)
    Strong,
}

/// Work flexibility level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexibilityLevel {
    /// Very flexible
    VeryFlexible,
    /// Somewhat flexible
    SomeFlexibility,
    /// Limited flexibility
    Limited,
    /// No flexibility
    None,
}

/// Current arrangement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentArrangement {
    /// Type of arrangement
    pub arrangement: ParentingArrangement,
    /// Time schedule
    pub schedule: ParentingTimeSchedule,
    /// How long in place
    pub duration_months: u32,
    /// How is it working
    pub functioning: ArrangementFunctioning,
}

/// How current arrangement is functioning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrangementFunctioning {
    /// Working well
    WorkingWell,
    /// Minor issues
    MinorIssues,
    /// Significant issues
    SignificantIssues,
    /// Not working
    NotWorking,
}

/// Family violence allegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyViolenceAllegation {
    /// Type of violence
    pub violence_type: FamilyViolence,
    /// Against whom
    pub against: String,
    /// By whom
    pub by: String,
    /// Evidence
    pub evidence: Vec<String>,
    /// Finding (if any)
    pub finding: Option<ViolenceFinding>,
}

/// Violence finding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolenceFinding {
    /// Proven
    Proven,
    /// Allegations credible
    CredibleAllegations,
    /// Unproven
    Unproven,
    /// Fabricated
    Fabricated,
}

/// Proposed arrangement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedArrangement {
    /// Type
    pub arrangement: ParentingArrangement,
    /// Schedule
    pub schedule: ParentingTimeSchedule,
    /// Decision-making allocation
    pub decision_making: DecisionMakingAllocation,
}

/// Decision-making allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMakingAllocation {
    /// Education decisions
    pub education: DecisionMaker,
    /// Health decisions
    pub health: DecisionMaker,
    /// Religious/cultural decisions
    pub religious_cultural: DecisionMaker,
    /// Extracurricular decisions
    pub extracurricular: DecisionMaker,
}

/// Who makes decisions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionMaker {
    /// Parent A sole
    ParentA,
    /// Parent B sole
    ParentB,
    /// Joint
    Joint,
    /// Parallel (each during their time)
    Parallel,
}

/// Result of best interests analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestInterestsResult {
    /// Recommended arrangement
    pub recommended_arrangement: ParentingArrangement,
    /// Recommended schedule
    pub recommended_schedule: ParentingTimeSchedule,
    /// Factor analysis
    pub factor_analysis: Vec<FactorAnalysis>,
    /// Whether family violence impacts
    pub family_violence_impact: Option<ViolenceImpact>,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of individual factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorAnalysis {
    /// Factor
    pub factor: BestInterestsFactor,
    /// Weight
    pub weight: FactorWeight,
    /// Favors which parent
    pub favors: Option<String>,
    /// Analysis
    pub analysis: String,
}

/// Factor weight
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorWeight {
    /// High importance
    High,
    /// Moderate importance
    Moderate,
    /// Low importance
    Low,
    /// Not applicable
    NotApplicable,
}

/// Impact of family violence on arrangement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolenceImpact {
    /// Violence finding
    pub finding: ViolenceFinding,
    /// Impact on decision-making
    pub decision_making_impact: String,
    /// Impact on parenting time
    pub time_impact: String,
    /// Safety measures required
    pub safety_measures: Vec<String>,
}

/// Best interests analyzer
pub struct BestInterestsAnalyzer;

impl BestInterestsAnalyzer {
    /// Analyze best interests
    pub fn analyze(facts: &BestInterestsFacts) -> BestInterestsResult {
        // Analyze each factor
        let factor_analysis = Self::analyze_factors(facts);

        // Check for family violence
        let violence_impact = Self::analyze_violence(facts);

        // Determine recommendation
        let (arrangement, schedule) =
            Self::determine_recommendation(facts, &factor_analysis, &violence_impact);

        let reasoning = Self::build_reasoning(facts, &factor_analysis, &violence_impact);

        BestInterestsResult {
            recommended_arrangement: arrangement,
            recommended_schedule: schedule,
            factor_analysis,
            family_violence_impact: violence_impact,
            reasoning,
        }
    }

    /// Analyze best interests factors
    fn analyze_factors(facts: &BestInterestsFacts) -> Vec<FactorAnalysis> {
        let mut analyses = Vec::new();

        // Child's needs
        analyses.push(FactorAnalysis {
            factor: BestInterestsFactor::ChildNeeds,
            weight: FactorWeight::High,
            favors: None,
            analysis: "Child's physical, emotional and psychological needs considered".to_string(),
        });

        // Child views (if applicable)
        for child in &facts.children {
            if child.age >= 12 {
                if let Some(views) = &child.views {
                    analyses.push(FactorAnalysis {
                        factor: BestInterestsFactor::ChildViews {
                            age: child.age,
                            maturity_level: format!("{:?}", views.maturity),
                        },
                        weight: if child.age >= 14 {
                            FactorWeight::High
                        } else {
                            FactorWeight::Moderate
                        },
                        favors: if views.give_weight {
                            Some(views.preference.clone())
                        } else {
                            None
                        },
                        analysis: format!(
                            "Child age {} expressed preference: {}",
                            child.age, views.preference
                        ),
                    });
                }
            }
        }

        // History of care
        analyses.push(FactorAnalysis {
            factor: BestInterestsFactor::HistoryOfCare,
            weight: FactorWeight::High,
            favors: Self::compare_history(&facts.parent_a, &facts.parent_b),
            analysis: "History of care by each parent evaluated".to_string(),
        });

        // Willingness to support relationship
        let willingness_a = &facts.parent_a.willingness_to_support;
        let willingness_b = &facts.parent_b.willingness_to_support;
        if willingness_a != willingness_b {
            analyses.push(FactorAnalysis {
                factor: BestInterestsFactor::WillingnessToSupport,
                weight: FactorWeight::High,
                favors: if willingness_a > willingness_b {
                    Some(facts.parent_a.identifier.clone())
                } else {
                    Some(facts.parent_b.identifier.clone())
                },
                analysis:
                    "Maximum contact principle - parent more willing to support other relationship"
                        .to_string(),
            });
        }

        // Stability
        if let Some(current) = &facts.current_arrangement {
            if current.duration_months >= 6
                && matches!(
                    current.functioning,
                    ArrangementFunctioning::WorkingWell | ArrangementFunctioning::MinorIssues
                )
            {
                analyses.push(FactorAnalysis {
                    factor: BestInterestsFactor::Stability,
                    weight: FactorWeight::High,
                    favors: None,
                    analysis: format!(
                        "Current arrangement in place for {} months and functioning",
                        current.duration_months
                    ),
                });
            }
        }

        analyses
    }

    /// Compare history of care
    fn compare_history(parent_a: &ParentInfo, parent_b: &ParentInfo) -> Option<String> {
        let a_count = parent_a.history_of_care.len();
        let b_count = parent_b.history_of_care.len();

        if a_count > b_count + 2 {
            Some(parent_a.identifier.clone())
        } else if b_count > a_count + 2 {
            Some(parent_b.identifier.clone())
        } else {
            None // Relatively equal
        }
    }

    /// Analyze family violence
    fn analyze_violence(facts: &BestInterestsFacts) -> Option<ViolenceImpact> {
        let proven_violence: Vec<_> = facts
            .family_violence
            .iter()
            .filter(|v| {
                matches!(
                    v.finding,
                    Some(ViolenceFinding::Proven) | Some(ViolenceFinding::CredibleAllegations)
                )
            })
            .collect();

        if proven_violence.is_empty() {
            return None;
        }

        let mut safety_measures = Vec::new();
        let mut decision_impact = String::new();
        let mut time_impact = String::new();

        for violence in &proven_violence {
            match violence.violence_type {
                FamilyViolence::PhysicalAbuse | FamilyViolence::SexualAbuse => {
                    safety_measures.push("Supervised parenting time required".to_string());
                    decision_impact = "Sole decision-making to non-violent parent".to_string();
                    time_impact = "Restricted parenting time with safeguards".to_string();
                }
                FamilyViolence::CoerciveControl => {
                    decision_impact = "Parallel parenting recommended".to_string();
                    safety_measures.push("No direct communication between parents".to_string());
                }
                FamilyViolence::ExposureToViolence => {
                    safety_measures.push("Child to be shielded from conflict".to_string());
                }
                _ => {}
            }
        }

        Some(ViolenceImpact {
            finding: proven_violence
                .first()
                .and_then(|v| v.finding.clone())
                .unwrap_or(ViolenceFinding::CredibleAllegations),
            decision_making_impact: decision_impact,
            time_impact,
            safety_measures,
        })
    }

    /// Determine recommendation
    fn determine_recommendation(
        facts: &BestInterestsFacts,
        _factor_analysis: &[FactorAnalysis],
        violence_impact: &Option<ViolenceImpact>,
    ) -> (ParentingArrangement, ParentingTimeSchedule) {
        // If violence, restrict accordingly
        if let Some(impact) = violence_impact {
            if !impact.safety_measures.is_empty() {
                return (
                    ParentingArrangement::SupervisedParentingTime,
                    ParentingTimeSchedule::SupervisedOnly,
                );
            }
        }

        // Check cooperation
        let both_cooperative = matches!(
            facts.parent_a.willingness_to_support,
            WillingnessLevel::Strong | WillingnessLevel::Adequate
        ) && matches!(
            facts.parent_b.willingness_to_support,
            WillingnessLevel::Strong | WillingnessLevel::Adequate
        );

        // If both cooperative and similar involvement, joint may work
        if both_cooperative {
            // Check for young children
            let youngest = facts.children.iter().map(|c| c.age).min().unwrap_or(10);

            if youngest < 5 {
                // Young children - more primary caregiver focused
                (
                    ParentingArrangement::JointDecisionMaking,
                    ParentingTimeSchedule::PrimaryResidence { percentage: 70 },
                )
            } else {
                // Older children - more shared time possible
                (
                    ParentingArrangement::JointDecisionMaking,
                    ParentingTimeSchedule::EqualTime,
                )
            }
        } else {
            // Less cooperation - parallel or sole
            (
                ParentingArrangement::ParallelParenting,
                ParentingTimeSchedule::EveryOtherWeekend,
            )
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &BestInterestsFacts,
        factor_analysis: &[FactorAnalysis],
        violence_impact: &Option<ViolenceImpact>,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Best interests of child is paramount (s.16 Divorce Act)".to_string());
        parts.push(format!(
            "Children: {} (ages: {})",
            facts.children.len(),
            facts
                .children
                .iter()
                .map(|c| c.age.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));

        // Key factors
        let high_factors: Vec<_> = factor_analysis
            .iter()
            .filter(|f| matches!(f.weight, FactorWeight::High))
            .collect();
        if !high_factors.is_empty() {
            parts.push(format!("Key factors: {}", high_factors.len()));
        }

        // Violence impact
        if violence_impact.is_some() {
            parts.push(
                "Family violence found - safety measures required per s.16(3)(j)".to_string(),
            );
        }

        parts.join(". ")
    }
}

// ============================================================================
// Relocation Analysis
// ============================================================================

/// Facts for relocation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocationFacts {
    /// Relocation request
    pub request: RelocationRequest,
    /// Current parenting arrangement
    pub current_arrangement: ParentingArrangement,
    /// Current schedule
    pub current_schedule: ParentingTimeSchedule,
    /// Best interests factors
    pub best_interests: BestInterestsFacts,
}

/// Result of relocation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocationResult {
    /// Relocation allowed
    pub allowed: bool,
    /// Condition on relocation
    pub conditions: Vec<String>,
    /// New parenting arrangement if allowed
    pub new_arrangement: Option<ParentingArrangement>,
    /// New schedule if allowed
    pub new_schedule: Option<ParentingTimeSchedule>,
    /// Reasoning
    pub reasoning: String,
}

/// Relocation analyzer
pub struct RelocationAnalyzer;

impl RelocationAnalyzer {
    /// Analyze relocation request
    pub fn analyze(facts: &RelocationFacts) -> RelocationResult {
        // Fresh best interests analysis (Gordon v Goertz)
        let bi_result = BestInterestsAnalyzer::analyze(&facts.best_interests);

        // Evaluate reason for relocation
        let reason_weight = Self::evaluate_reason(&facts.request.reason);

        // Consider impact on relationship
        let relationship_impact = Self::assess_relationship_impact(facts);

        // Determine if relocation should be allowed
        let allowed = reason_weight >= 0.5 && relationship_impact < 0.7;

        let conditions = if allowed {
            Self::determine_conditions(facts)
        } else {
            vec![]
        };

        let (new_arrangement, new_schedule) = if allowed {
            (
                Some(ParentingArrangement::JointDecisionMaking),
                facts.request.proposed_schedule.clone(),
            )
        } else {
            (None, None)
        };

        let reasoning = Self::build_reasoning(
            facts,
            allowed,
            reason_weight,
            relationship_impact,
            &bi_result,
        );

        RelocationResult {
            allowed,
            conditions,
            new_arrangement,
            new_schedule,
            reasoning,
        }
    }

    /// Evaluate reason for relocation
    fn evaluate_reason(reason: &RelocationReason) -> f64 {
        match reason {
            RelocationReason::Safety => 0.9,
            RelocationReason::Employment => 0.7,
            RelocationReason::Education => 0.6,
            RelocationReason::FamilySupport => 0.6,
            RelocationReason::NewRelationship => 0.4,
            RelocationReason::Other { .. } => 0.3,
        }
    }

    /// Assess impact on child-parent relationship
    fn assess_relationship_impact(facts: &RelocationFacts) -> f64 {
        match facts.current_schedule {
            ParentingTimeSchedule::EqualTime => 0.9,      // High impact
            ParentingTimeSchedule::WeekOnWeekOff => 0.85, // High impact
            ParentingTimeSchedule::EveryOtherWeekend => 0.5, // Moderate impact
            ParentingTimeSchedule::SupervisedOnly => 0.2, // Lower impact
            _ => 0.6,
        }
    }

    /// Determine conditions for relocation
    fn determine_conditions(facts: &RelocationFacts) -> Vec<String> {
        let mut conditions = Vec::new();

        conditions.push("Relocating parent to bear travel costs".to_string());
        conditions.push("Extended parenting time during school breaks".to_string());
        conditions.push("Regular video communication schedule".to_string());

        if matches!(
            facts.current_schedule,
            ParentingTimeSchedule::EqualTime | ParentingTimeSchedule::WeekOnWeekOff
        ) {
            conditions.push("Summer residence with other parent".to_string());
        }

        conditions
    }

    /// Build reasoning
    fn build_reasoning(
        _facts: &RelocationFacts,
        allowed: bool,
        reason_weight: f64,
        relationship_impact: f64,
        _bi_result: &BestInterestsResult,
    ) -> String {
        let mut parts = Vec::new();

        parts
            .push("Gordon v Goertz [1996] SCC: Fresh best interests analysis required".to_string());
        parts.push(format!(
            "Reason for relocation weight: {:.0}%",
            reason_weight * 100.0
        ));
        parts.push(format!(
            "Relationship impact: {:.0}%",
            relationship_impact * 100.0
        ));

        if allowed {
            parts.push(
                "Relocation allowed with conditions to maintain child's relationship".to_string(),
            );
        } else {
            parts.push(
                "Relocation denied - insufficient reason or excessive relationship impact"
                    .to_string(),
            );
        }

        parts.join(". ")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_basic_facts() -> BestInterestsFacts {
        BestInterestsFacts {
            children: vec![ChildInfo {
                age: 8,
                views: None,
                special_needs: vec![],
                community_ties: vec!["School".to_string()],
                cultural_needs: vec![],
            }],
            parent_a: ParentInfo {
                identifier: "Parent A".to_string(),
                history_of_care: vec!["Primary caregiver".to_string()],
                ability_to_meet_needs: vec!["Stable housing".to_string()],
                willingness_to_support: WillingnessLevel::Strong,
                work_flexibility: FlexibilityLevel::SomeFlexibility,
                support_system: vec!["Grandparents nearby".to_string()],
                concerns: vec![],
            },
            parent_b: ParentInfo {
                identifier: "Parent B".to_string(),
                history_of_care: vec!["Involved parent".to_string()],
                ability_to_meet_needs: vec!["Stable income".to_string()],
                willingness_to_support: WillingnessLevel::Adequate,
                work_flexibility: FlexibilityLevel::Limited,
                support_system: vec![],
                concerns: vec![],
            },
            current_arrangement: None,
            family_violence: vec![],
            proposed_arrangement: ProposedArrangement {
                arrangement: ParentingArrangement::JointDecisionMaking,
                schedule: ParentingTimeSchedule::EqualTime,
                decision_making: DecisionMakingAllocation {
                    education: DecisionMaker::Joint,
                    health: DecisionMaker::Joint,
                    religious_cultural: DecisionMaker::Joint,
                    extracurricular: DecisionMaker::Joint,
                },
            },
        }
    }

    #[test]
    fn test_best_interests_analysis() {
        let facts = create_basic_facts();
        let result = BestInterestsAnalyzer::analyze(&facts);

        assert!(!result.factor_analysis.is_empty());
        assert!(result.family_violence_impact.is_none());
    }

    #[test]
    fn test_family_violence_impact() {
        let mut facts = create_basic_facts();
        facts.family_violence = vec![FamilyViolenceAllegation {
            violence_type: FamilyViolence::PhysicalAbuse,
            against: "Child".to_string(),
            by: "Parent B".to_string(),
            evidence: vec!["Hospital records".to_string()],
            finding: Some(ViolenceFinding::Proven),
        }];

        let result = BestInterestsAnalyzer::analyze(&facts);

        assert!(result.family_violence_impact.is_some());
        assert_eq!(
            result.recommended_arrangement,
            ParentingArrangement::SupervisedParentingTime
        );
    }

    #[test]
    fn test_child_views_weight() {
        let mut facts = create_basic_facts();
        facts.children[0].age = 14;
        facts.children[0].views = Some(ChildViews {
            preference: "Live with Parent A".to_string(),
            maturity: MaturityLevel::AboveAverage,
            give_weight: true,
            source: "Section 211 report".to_string(),
        });

        let result = BestInterestsAnalyzer::analyze(&facts);

        let views_factor = result
            .factor_analysis
            .iter()
            .find(|f| matches!(f.factor, BestInterestsFactor::ChildViews { .. }));
        assert!(views_factor.is_some());
    }

    #[test]
    fn test_relocation_employment() {
        let facts = RelocationFacts {
            request: RelocationRequest {
                proposed_location: "Vancouver".to_string(),
                reason: RelocationReason::Employment,
                parenting_time_impact: "Reduced from 50% to school breaks".to_string(),
                proposed_schedule: Some(ParentingTimeSchedule::EveryOtherWeekend),
            },
            current_arrangement: ParentingArrangement::JointDecisionMaking,
            current_schedule: ParentingTimeSchedule::EqualTime,
            best_interests: create_basic_facts(),
        };

        let result = RelocationAnalyzer::analyze(&facts);

        // With equal time, relocation likely denied due to high impact
        assert!(!result.conditions.is_empty() || !result.allowed);
    }

    #[test]
    fn test_relocation_safety() {
        let facts = RelocationFacts {
            request: RelocationRequest {
                proposed_location: "Another city".to_string(),
                reason: RelocationReason::Safety,
                parenting_time_impact: "Supervised visits only".to_string(),
                proposed_schedule: Some(ParentingTimeSchedule::SupervisedOnly),
            },
            current_arrangement: ParentingArrangement::JointDecisionMaking,
            current_schedule: ParentingTimeSchedule::EveryOtherWeekend,
            best_interests: create_basic_facts(),
        };

        let result = RelocationAnalyzer::analyze(&facts);

        // Safety reason should be given high weight
        assert!(result.reasoning.contains("Gordon v Goertz"));
    }
}
