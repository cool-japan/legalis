//! Sentencing Analysis
//!
//! Implementation of Australian sentencing principles.

use serde::{Deserialize, Serialize};

use super::types::{
    AggravatingFactor, MitigatingFactor, OffenceCategory, SentenceType, SentencingPurpose,
};
use crate::common::StateTerritory;

// ============================================================================
// Sentencing Analyzer
// ============================================================================

/// Analyzer for sentencing
pub struct SentencingAnalyzer;

impl SentencingAnalyzer {
    /// Analyze sentencing options
    pub fn analyze(facts: &SentencingFacts) -> SentencingResult {
        let purposes = Self::identify_purposes(facts);
        let range = Self::determine_range(facts);
        let available_sentences = Self::determine_available_sentences(facts);
        let recommendation = Self::recommend_sentence(facts, &available_sentences);

        let reasoning = Self::build_reasoning(facts, &purposes, &range, &recommendation);

        SentencingResult {
            offence_category: facts.offence_category.clone(),
            maximum_penalty: facts.maximum_penalty.clone(),
            standard_non_parole: facts.standard_non_parole_period,
            sentencing_range: range,
            purposes,
            aggravating_factors: facts.aggravating_factors.clone(),
            mitigating_factors: facts.mitigating_factors.clone(),
            available_sentences,
            recommended_sentence: recommendation,
            reasoning,
        }
    }

    /// Identify sentencing purposes
    fn identify_purposes(facts: &SentencingFacts) -> Vec<SentencingPurpose> {
        let mut purposes = Vec::new();

        // Always applicable
        purposes.push(SentencingPurpose::Punishment);
        purposes.push(SentencingPurpose::Denunciation);
        purposes.push(SentencingPurpose::Accountability);

        // Deterrence
        if facts.offence_prevalent {
            purposes.push(SentencingPurpose::GeneralDeterrence);
        }
        purposes.push(SentencingPurpose::SpecificDeterrence);

        // Community protection for serious offences
        if facts.risk_of_reoffending_high || facts.violent_offence {
            purposes.push(SentencingPurpose::CommunityProtection);
        }

        // Rehabilitation if appropriate
        if facts.rehabilitation_prospects_good && !facts.must_impose_imprisonment {
            purposes.push(SentencingPurpose::Rehabilitation);
        }

        // Recognition of harm
        if facts.victim_impact_statement {
            purposes.push(SentencingPurpose::RecognitionOfHarm);
        }

        purposes
    }

    /// Determine sentencing range
    fn determine_range(facts: &SentencingFacts) -> SentencingRange {
        let max = facts.maximum_penalty_months;

        // Starting point based on objective seriousness
        let starting_point = max as f64 * Self::objective_seriousness_factor(facts);

        // Adjust for aggravating factors (increase)
        let aggravation_increase = facts.aggravating_factors.len() as f64 * 0.1;
        let with_aggravation = starting_point * (1.0 + aggravation_increase);

        // Adjust for mitigating factors (decrease)
        let mitigation_decrease = Self::calculate_mitigation(facts);
        let adjusted = with_aggravation * (1.0 - mitigation_decrease);

        // Guilty plea discount (max 25%)
        let with_plea = if facts.early_guilty_plea {
            adjusted * 0.75
        } else if facts.guilty_plea {
            adjusted * 0.85
        } else {
            adjusted
        };

        // Non-parole period (typically 60-75% of head sentence)
        let min_months = (with_plea * 0.6).max(0.0);
        let max_months = with_plea.min(max as f64);

        SentencingRange {
            minimum_months: min_months as u32,
            maximum_months: max_months as u32,
            non_parole_minimum: (min_months * 0.6) as u32,
            non_parole_maximum: (max_months * 0.75) as u32,
        }
    }

    /// Calculate objective seriousness factor
    fn objective_seriousness_factor(facts: &SentencingFacts) -> f64 {
        // Low seriousness: 0.1-0.3
        // Mid-range: 0.3-0.6
        // High seriousness: 0.6-0.8
        // Worst category: 0.8-1.0

        if facts.worst_category_offence {
            0.9
        } else if facts.offence_category == OffenceCategory::Indictable && facts.violent_offence {
            0.6
        } else if facts.offence_category == OffenceCategory::Indictable {
            0.4
        } else {
            0.2
        }
    }

    /// Calculate mitigation discount
    fn calculate_mitigation(facts: &SentencingFacts) -> f64 {
        let mut discount: f64 = 0.0;

        for factor in &facts.mitigating_factors {
            discount += match factor {
                MitigatingFactor::EarlyGuiltyPlea => 0.0, // Handled separately
                MitigatingFactor::Remorse => 0.05,
                MitigatingFactor::GoodCharacter => 0.05,
                MitigatingFactor::Cooperation => 0.05,
                MitigatingFactor::Youth => 0.1,
                MitigatingFactor::MentalHealth => 0.05,
                MitigatingFactor::LowReoffendingRisk => 0.05,
                MitigatingFactor::HardshipToDependants => 0.05,
                MitigatingFactor::Reparations => 0.05,
                MitigatingFactor::Provocation => 0.1,
            };
        }

        discount.min(0.4) // Maximum 40% for all mitigating factors combined
    }

    /// Determine available sentences
    fn determine_available_sentences(facts: &SentencingFacts) -> Vec<SentenceType> {
        let mut sentences = Vec::new();

        // Based on offence category and jurisdiction
        match facts.offence_category {
            OffenceCategory::Summary => {
                sentences.push(SentenceType::Dismissal);
                sentences.push(SentenceType::NoConvictionRecorded);
                sentences.push(SentenceType::ConditionalReleaseOrder);
                sentences.push(SentenceType::Fine);
                sentences.push(SentenceType::GoodBehaviourBond);
                sentences.push(SentenceType::CommunityCorrectionOrder);
                if facts.maximum_penalty_months > 6 {
                    sentences.push(SentenceType::Imprisonment);
                }
            }
            OffenceCategory::IndictableSummarily => {
                sentences.push(SentenceType::Fine);
                sentences.push(SentenceType::CommunityCorrectionOrder);
                sentences.push(SentenceType::IntensiveCorrectionOrder);
                sentences.push(SentenceType::Imprisonment);
            }
            OffenceCategory::Indictable => {
                sentences.push(SentenceType::Fine);
                sentences.push(SentenceType::CommunityCorrectionOrder);
                sentences.push(SentenceType::IntensiveCorrectionOrder);
                sentences.push(SentenceType::Imprisonment);
            }
        }

        // Remove imprisonment if alternative appropriate
        if facts.rehabilitation_prospects_good
            && !facts.must_impose_imprisonment
            && !facts.violent_offence
        {
            sentences.retain(|s| *s != SentenceType::Imprisonment);
        }

        sentences
    }

    /// Recommend sentence
    fn recommend_sentence(
        facts: &SentencingFacts,
        available: &[SentenceType],
    ) -> Option<SentenceRecommendation> {
        if available.is_empty() {
            return None;
        }

        // Must impose imprisonment for certain offences
        if facts.must_impose_imprisonment {
            return Some(SentenceRecommendation {
                sentence_type: SentenceType::Imprisonment,
                duration_months: Some(facts.maximum_penalty_months / 2),
                conditions: Vec::new(),
                reasoning: "Full-time custody required by law or circumstances".to_string(),
            });
        }

        // Intensive correction order if available and appropriate
        if available.contains(&SentenceType::IntensiveCorrectionOrder)
            && facts.suitable_for_ico
            && facts.ico_assessment_positive
        {
            return Some(SentenceRecommendation {
                sentence_type: SentenceType::IntensiveCorrectionOrder,
                duration_months: Some(12),
                conditions: vec!["Community service".to_string(), "Supervision".to_string()],
                reasoning: "ICO appropriate given rehabilitation prospects".to_string(),
            });
        }

        // Community correction order for less serious matters
        if available.contains(&SentenceType::CommunityCorrectionOrder)
            && facts.offence_category != OffenceCategory::Indictable
        {
            return Some(SentenceRecommendation {
                sentence_type: SentenceType::CommunityCorrectionOrder,
                duration_months: Some(12),
                conditions: vec!["Supervision".to_string()],
                reasoning: "CCO appropriate for offence seriousness".to_string(),
            });
        }

        // Fine for minor matters
        if available.contains(&SentenceType::Fine) && facts.can_pay_fine {
            return Some(SentenceRecommendation {
                sentence_type: SentenceType::Fine,
                duration_months: None,
                conditions: Vec::new(),
                reasoning: "Fine sufficient penalty".to_string(),
            });
        }

        None
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &SentencingFacts,
        purposes: &[SentencingPurpose],
        range: &SentencingRange,
        recommendation: &Option<SentenceRecommendation>,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Sentencing analysis for {:?} offence",
            facts.offence_category
        ));
        parts.push(format!(
            "Maximum penalty: {} months imprisonment",
            facts.maximum_penalty_months
        ));

        // Purposes
        let purpose_names: Vec<String> = purposes.iter().map(|p| format!("{:?}", p)).collect();
        parts.push(format!("Sentencing purposes: {}", purpose_names.join(", ")));

        // Aggravating factors
        if !facts.aggravating_factors.is_empty() {
            parts.push(format!(
                "Aggravating factors ({}): {:?}",
                facts.aggravating_factors.len(),
                facts.aggravating_factors
            ));
        }

        // Mitigating factors
        if !facts.mitigating_factors.is_empty() {
            parts.push(format!(
                "Mitigating factors ({}): {:?}",
                facts.mitigating_factors.len(),
                facts.mitigating_factors
            ));
        }

        // Guilty plea discount
        if facts.early_guilty_plea {
            parts.push("25% discount for early guilty plea (utilitarian benefit)".to_string());
        } else if facts.guilty_plea {
            parts.push("15% discount for guilty plea".to_string());
        }

        // Range
        parts.push(format!(
            "Sentencing range: {}-{} months",
            range.minimum_months, range.maximum_months
        ));

        // Recommendation
        if let Some(rec) = recommendation {
            parts.push(format!(
                "Recommendation: {:?} - {}",
                rec.sentence_type, rec.reasoning
            ));
        }

        parts.join(". ")
    }
}

/// Facts for sentencing analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentencingFacts {
    /// State/Territory
    pub jurisdiction: Option<StateTerritory>,
    /// Offence category
    pub offence_category: OffenceCategory,
    /// Maximum penalty description
    pub maximum_penalty: String,
    /// Maximum penalty in months
    pub maximum_penalty_months: u32,
    /// Standard non-parole period (if applicable)
    pub standard_non_parole_period: Option<u32>,
    /// Worst category offence
    pub worst_category_offence: bool,
    /// Violent offence
    pub violent_offence: bool,
    /// Offence prevalent
    pub offence_prevalent: bool,
    /// Aggravating factors
    pub aggravating_factors: Vec<AggravatingFactor>,
    /// Mitigating factors
    pub mitigating_factors: Vec<MitigatingFactor>,
    /// Guilty plea
    pub guilty_plea: bool,
    /// Early guilty plea
    pub early_guilty_plea: bool,
    /// Prior record
    pub prior_record: bool,
    /// Risk of reoffending high
    pub risk_of_reoffending_high: bool,
    /// Rehabilitation prospects good
    pub rehabilitation_prospects_good: bool,
    /// Victim impact statement
    pub victim_impact_statement: bool,
    /// Must impose imprisonment
    pub must_impose_imprisonment: bool,
    /// Suitable for ICO
    pub suitable_for_ico: bool,
    /// ICO assessment positive
    pub ico_assessment_positive: bool,
    /// Can pay fine
    pub can_pay_fine: bool,
}

/// Sentencing range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentencingRange {
    /// Minimum sentence (months)
    pub minimum_months: u32,
    /// Maximum sentence (months)
    pub maximum_months: u32,
    /// Non-parole minimum
    pub non_parole_minimum: u32,
    /// Non-parole maximum
    pub non_parole_maximum: u32,
}

/// Sentence recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceRecommendation {
    /// Sentence type
    pub sentence_type: SentenceType,
    /// Duration in months (if applicable)
    pub duration_months: Option<u32>,
    /// Conditions
    pub conditions: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

/// Result of sentencing analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentencingResult {
    /// Offence category
    pub offence_category: OffenceCategory,
    /// Maximum penalty
    pub maximum_penalty: String,
    /// Standard non-parole period
    pub standard_non_parole: Option<u32>,
    /// Sentencing range
    pub sentencing_range: SentencingRange,
    /// Sentencing purposes
    pub purposes: Vec<SentencingPurpose>,
    /// Aggravating factors
    pub aggravating_factors: Vec<AggravatingFactor>,
    /// Mitigating factors
    pub mitigating_factors: Vec<MitigatingFactor>,
    /// Available sentences
    pub available_sentences: Vec<SentenceType>,
    /// Recommended sentence
    pub recommended_sentence: Option<SentenceRecommendation>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_offence_sentencing() {
        let facts = SentencingFacts {
            offence_category: OffenceCategory::Summary,
            maximum_penalty: "6 months imprisonment".to_string(),
            maximum_penalty_months: 6,
            guilty_plea: true,
            can_pay_fine: true,
            rehabilitation_prospects_good: true,
            ..Default::default()
        };

        let result = SentencingAnalyzer::analyze(&facts);
        assert!(result.available_sentences.contains(&SentenceType::Fine));
    }

    #[test]
    fn test_indictable_violent_offence() {
        let facts = SentencingFacts {
            offence_category: OffenceCategory::Indictable,
            maximum_penalty: "10 years imprisonment".to_string(),
            maximum_penalty_months: 120,
            violent_offence: true,
            aggravating_factors: vec![AggravatingFactor::UseOfWeapon],
            must_impose_imprisonment: true,
            ..Default::default()
        };

        let result = SentencingAnalyzer::analyze(&facts);
        assert!(
            result
                .purposes
                .contains(&SentencingPurpose::CommunityProtection)
        );
        assert!(result.recommended_sentence.is_some());
    }

    #[test]
    fn test_guilty_plea_discount() {
        let facts = SentencingFacts {
            offence_category: OffenceCategory::Indictable,
            maximum_penalty_months: 60,
            early_guilty_plea: true,
            ..Default::default()
        };

        let result = SentencingAnalyzer::analyze(&facts);
        assert!(result.reasoning.contains("25% discount"));
    }

    #[test]
    fn test_mitigating_factors() {
        let facts = SentencingFacts {
            offence_category: OffenceCategory::IndictableSummarily,
            maximum_penalty_months: 24,
            mitigating_factors: vec![
                MitigatingFactor::Remorse,
                MitigatingFactor::GoodCharacter,
                MitigatingFactor::Youth,
            ],
            ..Default::default()
        };

        let result = SentencingAnalyzer::analyze(&facts);
        assert!(!result.mitigating_factors.is_empty());
        // Range should be reduced
        assert!(result.sentencing_range.maximum_months < 24);
    }

    #[test]
    fn test_ico_recommendation() {
        let facts = SentencingFacts {
            offence_category: OffenceCategory::Indictable,
            maximum_penalty_months: 36,
            suitable_for_ico: true,
            ico_assessment_positive: true,
            rehabilitation_prospects_good: true,
            ..Default::default()
        };

        let result = SentencingAnalyzer::analyze(&facts);
        if let Some(rec) = result.recommended_sentence {
            assert_eq!(rec.sentence_type, SentenceType::IntensiveCorrectionOrder);
        }
    }
}
