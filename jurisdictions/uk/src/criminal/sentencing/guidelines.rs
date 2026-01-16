//! UK Sentencing Guidelines
//!
//! This module implements the Sentencing Council guidelines framework
//! for determining appropriate sentences in criminal cases.
//!
//! # Framework
//!
//! The Sentencing Council (est. 2010) produces guidelines that courts must follow
//! unless contrary to interests of justice (s.125 Coroners and Justice Act 2009).
//!
//! ## Nine-Step Process
//! 1. Determine offence category (culpability + harm)
//! 2. Identify starting point and category range
//! 3. Adjust within range for aggravating/mitigating factors
//! 4. Consider reduction for guilty plea
//! 5. Consider totality principle
//! 6. Consider ancillary orders
//! 7. Give reasons
//! 8. Consider time on remand/bail
//! 9. Pronounce sentence

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::types::{
    AggravatingFactor, CulpabilityCategory, GuiltyPleaReduction, HarmCategory, MitigatingFactor,
    PleaStage, SentenceRange, SentenceType,
};

// ============================================================================
// Guideline Application
// ============================================================================

/// Facts for sentencing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentencingFacts {
    /// The offence
    pub offence: String,
    /// Culpability factors
    pub culpability_factors: Vec<CulpabilityFactor>,
    /// Harm factors
    pub harm_factors: Vec<HarmFactor>,
    /// Aggravating factors
    pub aggravating: Vec<AggravatingFactor>,
    /// Mitigating factors
    pub mitigating: Vec<MitigatingFactor>,
    /// Guilty plea (if any)
    pub guilty_plea: Option<GuiltyPleaFacts>,
    /// Previous convictions
    pub previous_convictions: Vec<PreviousConviction>,
    /// Time on remand
    pub remand_time: Option<RemandTime>,
    /// Totality consideration (if multiple offences)
    pub totality: Option<TotalityFacts>,
}

/// Culpability factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CulpabilityFactor {
    /// Factor description
    pub factor: String,
    /// Which category this supports
    pub supports_category: CulpabilityCategory,
}

/// Harm factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmFactor {
    /// Factor description
    pub factor: String,
    /// Which category this supports
    pub supports_category: HarmCategory,
}

/// Guilty plea facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiltyPleaFacts {
    /// When was plea entered?
    pub stage: PleaStage,
    /// Any reasons for delay?
    pub delay_reasons: Option<String>,
}

/// Previous conviction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousConviction {
    /// Offence
    pub offence: String,
    /// Year
    pub year: u32,
    /// Relevance to current offence
    pub relevance: ConvictionRelevance,
    /// Sentence received
    pub sentence: String,
}

/// Relevance of previous conviction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvictionRelevance {
    /// Highly relevant (same/similar offence)
    HighlyRelevant,
    /// Moderately relevant
    ModeratelyRelevant,
    /// Not very relevant
    NotVeryRelevant,
    /// Spent/very old
    Spent,
}

/// Remand time
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemandTime {
    /// Days in custody on remand
    pub custody_days: u32,
    /// Days on qualifying curfew
    pub curfew_days: u32,
}

/// Totality facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TotalityFacts {
    /// Number of offences
    pub offence_count: u32,
    /// Individual sentences proposed
    pub individual_sentences: Vec<String>,
    /// Are offences connected?
    pub connected: bool,
}

/// Result of sentencing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentencingAnalysisResult {
    /// Offence category
    pub category: OffenceCategory,
    /// Starting point
    pub starting_point: String,
    /// Category range
    pub category_range: SentenceRange,
    /// Aggravating factors analysis
    pub aggravating_analysis: Vec<AggravatingAnalysis>,
    /// Mitigating factors analysis
    pub mitigating_analysis: Vec<MitigatingAnalysis>,
    /// Pre-reduction sentence
    pub pre_reduction_sentence: String,
    /// Guilty plea reduction (if applicable)
    pub plea_reduction: Option<GuiltyPleaReduction>,
    /// Final sentence recommendation
    pub recommended_sentence: SentenceRecommendation,
    /// Nine-step analysis
    pub nine_step_analysis: NineStepAnalysis,
}

/// Offence category (culpability + harm matrix)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffenceCategory {
    /// Culpability category
    pub culpability: CulpabilityCategory,
    /// Harm category
    pub harm: HarmCategory,
    /// Combined category label (e.g., "1A", "2B")
    pub label: String,
}

/// Aggravating factor analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggravatingAnalysis {
    /// The factor
    pub factor: AggravatingFactor,
    /// Weight given
    pub weight: FactorWeight,
    /// Impact on sentence
    pub impact: String,
}

/// Mitigating factor analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MitigatingAnalysis {
    /// The factor
    pub factor: MitigatingFactor,
    /// Weight given
    pub weight: FactorWeight,
    /// Impact on sentence
    pub impact: String,
}

/// Weight of sentencing factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorWeight {
    /// High weight
    High,
    /// Medium weight
    Medium,
    /// Low weight
    Low,
}

/// Sentence recommendation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentenceRecommendation {
    /// Recommended sentence type
    pub sentence_type: SentenceType,
    /// Reasoning
    pub reasoning: String,
    /// Suspended sentence appropriate?
    pub suspended_appropriate: Option<bool>,
}

/// Nine-step analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NineStepAnalysis {
    /// Step 1: Category determination
    pub step1_category: String,
    /// Step 2: Starting point
    pub step2_starting_point: String,
    /// Step 3: Aggravating/mitigating
    pub step3_factors: String,
    /// Step 4: Guilty plea reduction
    pub step4_plea: String,
    /// Step 5: Totality
    pub step5_totality: String,
    /// Step 6: Ancillary orders
    pub step6_ancillary: String,
    /// Step 7: Reasons
    pub step7_reasons: String,
    /// Step 8: Time served
    pub step8_time_served: String,
    /// Step 9: Final sentence
    pub step9_sentence: String,
}

/// Sentencing guideline analyzer
pub struct SentencingGuidelineAnalyzer;

impl SentencingGuidelineAnalyzer {
    /// Analyze sentencing using guidelines
    pub fn analyze(facts: &SentencingFacts) -> SentencingAnalysisResult {
        // Step 1: Determine category
        let category = Self::determine_category(&facts.culpability_factors, &facts.harm_factors);

        // Step 2: Get starting point and range (simplified - would look up actual guideline)
        let (starting_point, category_range) = Self::get_starting_point(&category, &facts.offence);

        // Step 3: Analyze aggravating and mitigating factors
        let aggravating_analysis = Self::analyze_aggravating(&facts.aggravating);
        let mitigating_analysis = Self::analyze_mitigating(&facts.mitigating);

        // Calculate pre-reduction sentence
        let pre_reduction_sentence = Self::calculate_pre_reduction(
            &starting_point,
            &aggravating_analysis,
            &mitigating_analysis,
        );

        // Step 4: Apply guilty plea reduction
        let plea_reduction = facts
            .guilty_plea
            .as_ref()
            .map(Self::calculate_plea_reduction);

        // Calculate final sentence
        let recommended_sentence =
            Self::calculate_final_sentence(&pre_reduction_sentence, &plea_reduction, facts);

        // Build nine-step analysis
        let nine_step_analysis = Self::build_nine_step_analysis(
            &category,
            &starting_point,
            &aggravating_analysis,
            &mitigating_analysis,
            &plea_reduction,
            facts,
            &recommended_sentence,
        );

        SentencingAnalysisResult {
            category,
            starting_point,
            category_range,
            aggravating_analysis,
            mitigating_analysis,
            pre_reduction_sentence,
            plea_reduction,
            recommended_sentence,
            nine_step_analysis,
        }
    }

    fn determine_category(
        culpability: &[CulpabilityFactor],
        harm: &[HarmFactor],
    ) -> OffenceCategory {
        // Count factors by category
        let culp_a = culpability
            .iter()
            .filter(|f| f.supports_category == CulpabilityCategory::A)
            .count();
        let culp_b = culpability
            .iter()
            .filter(|f| f.supports_category == CulpabilityCategory::B)
            .count();

        let culpability_cat = if culp_a > culp_b {
            CulpabilityCategory::A
        } else if culp_b > 0 {
            CulpabilityCategory::B
        } else {
            CulpabilityCategory::C
        };

        let harm_1 = harm
            .iter()
            .filter(|f| f.supports_category == HarmCategory::Category1)
            .count();
        let harm_2 = harm
            .iter()
            .filter(|f| f.supports_category == HarmCategory::Category2)
            .count();

        let harm_cat = if harm_1 > harm_2 {
            HarmCategory::Category1
        } else if harm_2 > 0 {
            HarmCategory::Category2
        } else {
            HarmCategory::Category3
        };

        let label = format!(
            "{}{}",
            match harm_cat {
                HarmCategory::Category1 => "1",
                HarmCategory::Category2 => "2",
                HarmCategory::Category3 => "3",
                HarmCategory::Category4 => "4",
            },
            match culpability_cat {
                CulpabilityCategory::A => "A",
                CulpabilityCategory::B => "B",
                CulpabilityCategory::C => "C",
                CulpabilityCategory::D => "D",
            }
        );

        OffenceCategory {
            culpability: culpability_cat,
            harm: harm_cat,
            label,
        }
    }

    fn get_starting_point(category: &OffenceCategory, _offence: &str) -> (String, SentenceRange) {
        // Simplified - would look up actual guideline for offence
        let (sp, min, max) = match (&category.harm, &category.culpability) {
            (HarmCategory::Category1, CulpabilityCategory::A) => {
                ("4 years' custody", "3 years", "6 years")
            }
            (HarmCategory::Category1, CulpabilityCategory::B) => {
                ("2 years' custody", "1 year", "4 years")
            }
            (HarmCategory::Category1, CulpabilityCategory::C) => {
                ("1 year's custody", "High community order", "2 years")
            }
            (HarmCategory::Category2, CulpabilityCategory::A) => {
                ("2 years' custody", "1 year", "4 years")
            }
            (HarmCategory::Category2, CulpabilityCategory::B) => {
                ("1 year's custody", "High community order", "2 years")
            }
            (HarmCategory::Category2, CulpabilityCategory::C) => (
                "Medium community order",
                "Low community order",
                "High community order",
            ),
            (HarmCategory::Category3, _) => {
                ("Low community order", "Discharge", "Medium community order")
            }
            _ => ("Fine", "Discharge", "Low community order"),
        };

        (
            sp.to_string(),
            SentenceRange {
                starting_point: sp.to_string(),
                range_min: min.to_string(),
                range_max: max.to_string(),
            },
        )
    }

    fn analyze_aggravating(factors: &[AggravatingFactor]) -> Vec<AggravatingAnalysis> {
        factors
            .iter()
            .map(|f| {
                let weight = match f {
                    AggravatingFactor::PreviousConvictions => FactorWeight::High,
                    AggravatingFactor::VulnerableVictim => FactorWeight::High,
                    AggravatingFactor::AbuseOfTrust => FactorWeight::High,
                    AggravatingFactor::Premeditation => FactorWeight::High,
                    AggravatingFactor::HateCrime { .. } => FactorWeight::High,
                    AggravatingFactor::GroupOffending => FactorWeight::Medium,
                    AggravatingFactor::UseOfWeapon => FactorWeight::Medium,
                    AggravatingFactor::OnBail => FactorWeight::Medium,
                    _ => FactorWeight::Low,
                };

                let impact = match weight {
                    FactorWeight::High => "Significant upward adjustment".to_string(),
                    FactorWeight::Medium => "Moderate upward adjustment".to_string(),
                    FactorWeight::Low => "Minor upward adjustment".to_string(),
                };

                AggravatingAnalysis {
                    factor: f.clone(),
                    weight,
                    impact,
                }
            })
            .collect()
    }

    fn analyze_mitigating(factors: &[MitigatingFactor]) -> Vec<MitigatingAnalysis> {
        factors
            .iter()
            .map(|f| {
                let weight = match f {
                    MitigatingFactor::NoPreviousConvictions => FactorWeight::High,
                    MitigatingFactor::MentalDisorder => FactorWeight::High,
                    MitigatingFactor::AgeOrMaturity => FactorWeight::Medium,
                    MitigatingFactor::Remorse => FactorWeight::Medium,
                    MitigatingFactor::Cooperation => FactorWeight::Medium,
                    MitigatingFactor::SelfReported => FactorWeight::Medium,
                    _ => FactorWeight::Low,
                };

                let impact = match weight {
                    FactorWeight::High => "Significant downward adjustment".to_string(),
                    FactorWeight::Medium => "Moderate downward adjustment".to_string(),
                    FactorWeight::Low => "Minor downward adjustment".to_string(),
                };

                MitigatingAnalysis {
                    factor: f.clone(),
                    weight,
                    impact,
                }
            })
            .collect()
    }

    fn calculate_pre_reduction(
        starting_point: &str,
        _aggravating: &[AggravatingAnalysis],
        _mitigating: &[MitigatingAnalysis],
    ) -> String {
        // Simplified - would adjust based on factors
        // For now just return starting point
        starting_point.to_string()
    }

    fn calculate_plea_reduction(plea: &GuiltyPleaFacts) -> GuiltyPleaReduction {
        let (fraction, description) = match plea.stage {
            PleaStage::FirstOpportunity => {
                ("1/3", "Maximum reduction for plea at first opportunity")
            }
            PleaStage::BeforeTrial => ("1/4", "Reduction for plea before trial"),
            PleaStage::DuringTrial => ("1/10", "Limited reduction for plea during trial"),
        };

        GuiltyPleaReduction {
            plea_stage: plea.stage.clone(),
            reduction_fraction: fraction.to_string(),
            reduction_description: description.to_string(),
        }
    }

    fn calculate_final_sentence(
        pre_reduction: &str,
        plea_reduction: &Option<GuiltyPleaReduction>,
        _facts: &SentencingFacts,
    ) -> SentenceRecommendation {
        let reasoning = if let Some(pr) = plea_reduction {
            format!(
                "Starting from {}, applying {} guilty plea reduction",
                pre_reduction, pr.reduction_fraction
            )
        } else {
            format!("Based on starting point of {}", pre_reduction)
        };

        // Simplified - would calculate actual sentence
        SentenceRecommendation {
            sentence_type: SentenceType::CommunityOrder(crate::criminal::types::CommunityOrder {
                length_months: 12,
                requirements: vec![],
            }),
            reasoning,
            suspended_appropriate: Some(true),
        }
    }

    fn build_nine_step_analysis(
        category: &OffenceCategory,
        starting_point: &str,
        aggravating: &[AggravatingAnalysis],
        mitigating: &[MitigatingAnalysis],
        plea_reduction: &Option<GuiltyPleaReduction>,
        facts: &SentencingFacts,
        recommendation: &SentenceRecommendation,
    ) -> NineStepAnalysis {
        let step1 = format!(
            "Category {}: Culpability {:?}, Harm {:?}",
            category.label, category.culpability, category.harm
        );

        let step2 = format!("Starting point: {}", starting_point);

        let step3 = format!(
            "{} aggravating factors, {} mitigating factors considered",
            aggravating.len(),
            mitigating.len()
        );

        let step4 = plea_reduction
            .as_ref()
            .map(|pr| format!("{} reduction applied", pr.reduction_fraction))
            .unwrap_or_else(|| "No guilty plea reduction".to_string());

        let step5 = facts
            .totality
            .as_ref()
            .map(|t| format!("Totality considered for {} offences", t.offence_count))
            .unwrap_or_else(|| "Single offence - no totality adjustment".to_string());

        let step6 = "Consider appropriate ancillary orders".to_string();

        let step7 = "Reasons given for sentence as required by law".to_string();

        let step8 = facts
            .remand_time
            .as_ref()
            .map(|rt| {
                format!(
                    "{} days custody, {} days curfew to count",
                    rt.custody_days, rt.curfew_days
                )
            })
            .unwrap_or_else(|| "No time to count".to_string());

        let step9 = recommendation.reasoning.clone();

        NineStepAnalysis {
            step1_category: step1,
            step2_starting_point: step2,
            step3_factors: step3,
            step4_plea: step4,
            step5_totality: step5,
            step6_ancillary: step6,
            step7_reasons: step7,
            step8_time_served: step8,
            step9_sentence: step9,
        }
    }
}

// ============================================================================
// Dangerous Offenders
// ============================================================================

/// Facts for dangerous offender assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DangerousOffenderFacts {
    /// Current offence
    pub current_offence: String,
    /// Is offence a specified offence?
    pub specified_offence: bool,
    /// Is offence a serious offence?
    pub serious_offence: bool,
    /// Previous offences
    pub previous_offences: Vec<PreviousOffence>,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
}

/// Previous offence for dangerousness
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousOffence {
    /// Offence description
    pub offence: String,
    /// Was it a specified offence?
    pub specified: bool,
    /// Sentence received
    pub sentence: String,
}

/// Risk assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Is there significant risk of serious harm?
    pub significant_risk: bool,
    /// Risk factors identified
    pub risk_factors: Vec<String>,
    /// Protective factors
    pub protective_factors: Vec<String>,
}

/// Result of dangerous offender assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DangerousOffenderResult {
    /// Is D a dangerous offender?
    pub dangerous: bool,
    /// Life sentence required/available?
    pub life_sentence: LifeSentenceAssessment,
    /// Extended sentence appropriate?
    pub extended_sentence: ExtendedSentenceAssessment,
    /// Reasoning
    pub reasoning: String,
}

/// Life sentence assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifeSentenceAssessment {
    /// Required (Schedule 21)?
    pub required: bool,
    /// Available (discretionary)?
    pub available: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Extended sentence assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedSentenceAssessment {
    /// Extended sentence appropriate?
    pub appropriate: bool,
    /// Extension period recommended
    pub extension_period: Option<u32>,
    /// Reasoning
    pub reasoning: String,
}

/// Dangerous offender analyzer
pub struct DangerousOffenderAnalyzer;

impl DangerousOffenderAnalyzer {
    /// Assess whether D is dangerous offender
    pub fn analyze(facts: &DangerousOffenderFacts) -> DangerousOffenderResult {
        let dangerous = facts.specified_offence && facts.risk_assessment.significant_risk;

        let life_sentence = if facts.serious_offence && dangerous {
            LifeSentenceAssessment {
                required: false, // Unless murder
                available: true,
                reasoning: "Discretionary life available for serious offence + dangerousness"
                    .into(),
            }
        } else {
            LifeSentenceAssessment {
                required: false,
                available: false,
                reasoning: "Life sentence not available".into(),
            }
        };

        let extended_sentence = if dangerous && !life_sentence.available {
            ExtendedSentenceAssessment {
                appropriate: true,
                extension_period: Some(5), // 5 years typical for specified violent offence
                reasoning: "Extended sentence appropriate given dangerousness".into(),
            }
        } else {
            ExtendedSentenceAssessment {
                appropriate: false,
                extension_period: None,
                reasoning: "Extended sentence not required".into(),
            }
        };

        let reasoning = if dangerous {
            "D assessed as dangerous offender: significant risk of serious harm from further \
             specified offences"
                .into()
        } else {
            "D not assessed as dangerous offender".into()
        };

        DangerousOffenderResult {
            dangerous,
            life_sentence,
            extended_sentence,
            reasoning,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_determination() {
        let culp = vec![CulpabilityFactor {
            factor: "High degree of planning".into(),
            supports_category: CulpabilityCategory::A,
        }];
        let harm = vec![HarmFactor {
            factor: "Serious physical harm".into(),
            supports_category: HarmCategory::Category1,
        }];

        let category = SentencingGuidelineAnalyzer::determine_category(&culp, &harm);
        assert_eq!(category.culpability, CulpabilityCategory::A);
        assert_eq!(category.harm, HarmCategory::Category1);
        assert_eq!(category.label, "1A");
    }

    #[test]
    fn test_guilty_plea_first_opportunity() {
        let plea = GuiltyPleaFacts {
            stage: PleaStage::FirstOpportunity,
            delay_reasons: None,
        };

        let reduction = SentencingGuidelineAnalyzer::calculate_plea_reduction(&plea);
        assert_eq!(reduction.reduction_fraction, "1/3");
    }

    #[test]
    fn test_guilty_plea_during_trial() {
        let plea = GuiltyPleaFacts {
            stage: PleaStage::DuringTrial,
            delay_reasons: Some("Hoped case would collapse".into()),
        };

        let reduction = SentencingGuidelineAnalyzer::calculate_plea_reduction(&plea);
        assert_eq!(reduction.reduction_fraction, "1/10");
    }

    #[test]
    fn test_sentencing_analysis() {
        let facts = SentencingFacts {
            offence: "Theft".into(),
            culpability_factors: vec![CulpabilityFactor {
                factor: "Sophisticated offence".into(),
                supports_category: CulpabilityCategory::A,
            }],
            harm_factors: vec![HarmFactor {
                factor: "High value goods".into(),
                supports_category: HarmCategory::Category1,
            }],
            aggravating: vec![AggravatingFactor::PreviousConvictions],
            mitigating: vec![MitigatingFactor::Remorse],
            guilty_plea: Some(GuiltyPleaFacts {
                stage: PleaStage::FirstOpportunity,
                delay_reasons: None,
            }),
            previous_convictions: vec![],
            remand_time: None,
            totality: None,
        };

        let result = SentencingGuidelineAnalyzer::analyze(&facts);
        assert_eq!(result.category.label, "1A");
        assert!(result.plea_reduction.is_some());
    }

    #[test]
    fn test_dangerous_offender() {
        let facts = DangerousOffenderFacts {
            current_offence: "s.18 GBH".into(),
            specified_offence: true,
            serious_offence: true,
            previous_offences: vec![PreviousOffence {
                offence: "s.20 GBH".into(),
                specified: true,
                sentence: "2 years".into(),
            }],
            risk_assessment: RiskAssessment {
                significant_risk: true,
                risk_factors: vec!["Pattern of violence".into()],
                protective_factors: vec![],
            },
        };

        let result = DangerousOffenderAnalyzer::analyze(&facts);
        assert!(result.dangerous);
        assert!(result.life_sentence.available);
    }

    #[test]
    fn test_not_dangerous() {
        let facts = DangerousOffenderFacts {
            current_offence: "Theft".into(),
            specified_offence: false,
            serious_offence: false,
            previous_offences: vec![],
            risk_assessment: RiskAssessment {
                significant_risk: false,
                risk_factors: vec![],
                protective_factors: vec!["Employment".into()],
            },
        };

        let result = DangerousOffenderAnalyzer::analyze(&facts);
        assert!(!result.dangerous);
    }
}
