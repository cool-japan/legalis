//! Canada Criminal Law - Sentencing
//!
//! Sentencing principles and analysis under Part XXIII of the Criminal Code.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    AggravatingFactor, GladueFactor, MitigatingFactor, OffenceType, SentenceType,
    SentencingPrinciple,
};

// ============================================================================
// Sentencing Facts
// ============================================================================

/// Facts for sentencing analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentencingFacts {
    /// Offence
    pub offence: String,
    /// Offence type
    pub offence_type: OffenceType,
    /// Maximum sentence (months)
    pub max_sentence_months: Option<u32>,
    /// Mandatory minimum (months)
    pub mandatory_minimum_months: Option<u32>,
    /// Aggravating factors
    pub aggravating: Vec<AggravatingFactor>,
    /// Mitigating factors
    pub mitigating: Vec<MitigatingFactor>,
    /// Offender information
    pub offender: OffenderInfo,
    /// Victim impact
    pub victim_impact: Option<VictimImpact>,
    /// Pre-sentence custody days
    pub pre_sentence_custody_days: u32,
}

/// Offender information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffenderInfo {
    /// Age
    pub age: u32,
    /// First offender
    pub first_offender: bool,
    /// Prior criminal record
    pub prior_record: Vec<PriorConviction>,
    /// Indigenous offender (Gladue applies)
    pub indigenous: bool,
    /// Gladue factors
    pub gladue_factors: Vec<GladueFactor>,
    /// Employment status
    pub employed: bool,
    /// Family support
    pub family_support: bool,
    /// Rehabilitation efforts
    pub rehabilitation_efforts: Vec<String>,
}

/// Prior conviction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorConviction {
    /// Offence
    pub offence: String,
    /// Year
    pub year: u32,
    /// Sentence received
    pub sentence: SentenceType,
}

/// Victim impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimImpact {
    /// Physical harm
    pub physical_harm: HarmLevel,
    /// Psychological harm
    pub psychological_harm: HarmLevel,
    /// Financial loss (cents)
    pub financial_loss_cents: Option<i64>,
    /// Ongoing impact
    pub ongoing_impact: bool,
}

/// Level of harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmLevel {
    /// None
    None,
    /// Minor
    Minor,
    /// Moderate
    Moderate,
    /// Serious
    Serious,
    /// Severe/permanent
    Severe,
}

// ============================================================================
// Sentencing Result
// ============================================================================

/// Result of sentencing analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentencingResult {
    /// Recommended sentence type
    pub recommended_sentence: SentenceType,
    /// Sentence range (low, mid, high in months)
    pub range: SentenceRange,
    /// Primary principles
    pub primary_principles: Vec<SentencingPrinciple>,
    /// Aggravating factors weighted
    pub aggravating_analysis: Vec<(AggravatingFactor, f64)>,
    /// Mitigating factors weighted
    pub mitigating_analysis: Vec<(MitigatingFactor, f64)>,
    /// Gladue considerations
    pub gladue_analysis: Option<GladueAnalysis>,
    /// Pre-sentence credit
    pub pre_sentence_credit_days: u32,
    /// Reasoning
    pub reasoning: String,
}

/// Sentence range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceRange {
    /// Low end (months)
    pub low_months: f64,
    /// Mid-range (months)
    pub mid_months: f64,
    /// High end (months)
    pub high_months: f64,
}

/// Gladue analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GladueAnalysis {
    /// Factors present
    pub factors: Vec<GladueFactor>,
    /// Background circumstances
    pub background_summary: String,
    /// Alternative sanctions considered
    pub alternative_sanctions: Vec<String>,
    /// Reduction factor
    pub reduction_factor: f64,
}

// ============================================================================
// Sentencing Analyzer
// ============================================================================

/// Sentencing analyzer
pub struct SentencingAnalyzer;

impl SentencingAnalyzer {
    /// Analyze sentencing
    pub fn analyze(facts: &SentencingFacts) -> SentencingResult {
        // Determine primary principles
        let principles = Self::determine_principles(facts);

        // Calculate starting point
        let starting_point = Self::calculate_starting_point(facts);

        // Analyze aggravating factors
        let aggravating_analysis = Self::analyze_aggravating(&facts.aggravating, facts);
        let aggravating_total: f64 = aggravating_analysis.iter().map(|(_, w)| w).sum();

        // Analyze mitigating factors
        let mitigating_analysis = Self::analyze_mitigating(&facts.mitigating, facts);
        let mitigating_total: f64 = mitigating_analysis.iter().map(|(_, w)| w).sum();

        // Gladue analysis if applicable
        let gladue_analysis = if facts.offender.indigenous {
            Some(Self::analyze_gladue(facts))
        } else {
            None
        };
        let gladue_reduction = gladue_analysis.as_ref().map_or(0.0, |g| g.reduction_factor);

        // Calculate range
        let net_adjustment = aggravating_total - mitigating_total - gladue_reduction;
        let adjusted_months = starting_point * (1.0 + net_adjustment);

        // Apply limits
        let min_months = facts.mandatory_minimum_months.unwrap_or(0) as f64;
        let max_months = facts.max_sentence_months.map_or(999.0, |m| m as f64);
        let final_months = adjusted_months.clamp(min_months, max_months);

        // Calculate range
        let range = SentenceRange {
            low_months: (final_months * 0.7).max(min_months),
            mid_months: final_months,
            high_months: (final_months * 1.3).min(max_months),
        };

        // Calculate pre-sentence credit (typically 1.5:1)
        let pre_sentence_credit = (facts.pre_sentence_custody_days as f64 * 1.5) as u32;

        // Determine recommended sentence type
        let recommended = Self::determine_sentence_type(&range, facts);

        // Build reasoning
        let reasoning = Self::build_reasoning(
            facts,
            &principles,
            starting_point,
            aggravating_total,
            mitigating_total,
            gladue_reduction,
        );

        SentencingResult {
            recommended_sentence: recommended,
            range,
            primary_principles: principles,
            aggravating_analysis,
            mitigating_analysis,
            gladue_analysis,
            pre_sentence_credit_days: pre_sentence_credit,
            reasoning,
        }
    }

    /// Determine primary sentencing principles
    fn determine_principles(facts: &SentencingFacts) -> Vec<SentencingPrinciple> {
        let mut principles = Vec::new();

        // Serious offences emphasize denunciation and deterrence
        if matches!(facts.offence_type, OffenceType::Indictable) {
            principles.push(SentencingPrinciple::Denunciation);
            principles.push(SentencingPrinciple::Deterrence);
        }

        // First offenders - rehabilitation
        if facts.offender.first_offender {
            principles.push(SentencingPrinciple::Rehabilitation);
        }

        // Victim impact - reparations
        if facts.victim_impact.is_some() {
            principles.push(SentencingPrinciple::Reparations);
        }

        // Repeated offenders - incapacitation
        if facts.offender.prior_record.len() > 2 {
            principles.push(SentencingPrinciple::Incapacitation);
        }

        if principles.is_empty() {
            principles.push(SentencingPrinciple::Denunciation);
            principles.push(SentencingPrinciple::Rehabilitation);
        }

        principles
    }

    /// Calculate starting point (months)
    fn calculate_starting_point(facts: &SentencingFacts) -> f64 {
        // This is a simplified model - actual starting points vary by offence
        let base = match facts.offence_type {
            OffenceType::Summary => 2.0,
            OffenceType::Hybrid => 6.0,
            OffenceType::Indictable => 12.0,
        };

        // Adjust based on max sentence
        let max_factor = facts.max_sentence_months.map_or(1.0, |max| {
            if max > 120 {
                2.0
            } else if max > 60 {
                1.5
            } else {
                1.0
            }
        });

        base * max_factor
    }

    /// Analyze aggravating factors
    fn analyze_aggravating(
        factors: &[AggravatingFactor],
        _facts: &SentencingFacts,
    ) -> Vec<(AggravatingFactor, f64)> {
        factors
            .iter()
            .map(|f| {
                let weight = match f {
                    AggravatingFactor::HateCrime => 0.3,
                    AggravatingFactor::DomesticViolence => 0.25,
                    AggravatingFactor::AbuseOfTrust => 0.25,
                    AggravatingFactor::VulnerableVictim => 0.2,
                    AggravatingFactor::BreachCourtOrder => 0.2,
                    AggravatingFactor::CriminalOrganization => 0.3,
                    AggravatingFactor::Terrorism => 0.4,
                    AggravatingFactor::PriorRecord => 0.15,
                };
                (f.clone(), weight)
            })
            .collect()
    }

    /// Analyze mitigating factors
    fn analyze_mitigating(
        factors: &[MitigatingFactor],
        _facts: &SentencingFacts,
    ) -> Vec<(MitigatingFactor, f64)> {
        factors
            .iter()
            .map(|f| {
                let weight = match f {
                    MitigatingFactor::FirstOffender => 0.2,
                    MitigatingFactor::GuiltyPlea => 0.15,
                    MitigatingFactor::Remorse => 0.1,
                    MitigatingFactor::GoodCharacter => 0.1,
                    MitigatingFactor::MentalHealth => 0.15,
                    MitigatingFactor::Addiction => 0.1,
                    MitigatingFactor::GladueFactor => 0.2,
                    MitigatingFactor::Youth => 0.15,
                    MitigatingFactor::Cooperation => 0.1,
                };
                (f.clone(), weight)
            })
            .collect()
    }

    /// Analyze Gladue factors
    fn analyze_gladue(facts: &SentencingFacts) -> GladueAnalysis {
        let factors = &facts.offender.gladue_factors;

        // Build background summary
        let background = factors
            .iter()
            .map(|f| match f {
                GladueFactor::ResidentialSchool => {
                    "Impact of residential school system on offender/family"
                }
                GladueFactor::SixtiesScoop => "Removal from family through Sixties Scoop",
                GladueFactor::ChildWelfare => "Child welfare system involvement",
                GladueFactor::SystemicRacism => "Experience of systemic racism",
                GladueFactor::IntergenerationalTrauma => "Intergenerational trauma",
                GladueFactor::SocioeconomicDisadvantage => "Socioeconomic disadvantage",
                GladueFactor::CulturalDislocation => "Cultural and language loss",
                GladueFactor::EducationalDisadvantage => "Educational barriers",
                GladueFactor::FamilyViolence => "Family violence",
            })
            .collect::<Vec<_>>()
            .join("; ");

        // Alternative sanctions
        let alternatives = vec![
            "Community-based healing program".to_string(),
            "Restorative justice circle".to_string(),
            "Conditional sentence with Indigenous programming".to_string(),
        ];

        // Reduction based on number of factors
        let reduction = (factors.len() as f64 * 0.05).min(0.25);

        GladueAnalysis {
            factors: factors.clone(),
            background_summary: background,
            alternative_sanctions: alternatives,
            reduction_factor: reduction,
        }
    }

    /// Determine sentence type based on range
    fn determine_sentence_type(range: &SentenceRange, facts: &SentencingFacts) -> SentenceType {
        let months = range.mid_months;

        // Less serious - discharge possible
        if months < 1.0 && facts.offender.first_offender {
            return SentenceType::ConditionalDischarge;
        }

        // Short sentence - conditional sentence possible (if not excluded)
        if months < 24.0 && !Self::is_cso_excluded(facts) {
            return SentenceType::ConditionalSentence;
        }

        // Less than 2 years - provincial
        if months < 24.0 {
            return SentenceType::Imprisonment {
                months: months as u32,
            };
        }

        // 2 years or more - federal penitentiary
        SentenceType::Imprisonment {
            months: months as u32,
        }
    }

    /// Check if conditional sentence order is excluded
    fn is_cso_excluded(facts: &SentencingFacts) -> bool {
        // CSO excluded for serious personal injury offences, terrorism, etc.
        let offence_lower = facts.offence.to_lowercase();
        let excluded_terms = [
            "murder",
            "sexual assault",
            "terrorism",
            "criminal organization",
        ];
        excluded_terms.iter().any(|t| offence_lower.contains(t))
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &SentencingFacts,
        principles: &[SentencingPrinciple],
        starting: f64,
        aggravating: f64,
        mitigating: f64,
        gladue: f64,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Offence: {} ({:?})",
            facts.offence, facts.offence_type
        ));

        let principle_names: Vec<_> = principles.iter().map(|p| format!("{:?}", p)).collect();
        parts.push(format!(
            "Primary principles: {}",
            principle_names.join(", ")
        ));

        parts.push(format!("Starting point: {:.1} months", starting));

        if aggravating > 0.0 {
            parts.push(format!(
                "Aggravating factors (+{:.0}%): {} factors",
                aggravating * 100.0,
                facts.aggravating.len()
            ));
        }

        if mitigating > 0.0 {
            parts.push(format!(
                "Mitigating factors (-{:.0}%): {} factors",
                mitigating * 100.0,
                facts.mitigating.len()
            ));
        }

        if gladue > 0.0 {
            parts.push(format!(
                "Gladue reduction (-{:.0}%): {} factors considered per s.718.2(e)",
                gladue * 100.0,
                facts.offender.gladue_factors.len()
            ));
        }

        if facts.pre_sentence_custody_days > 0 {
            let credit = (facts.pre_sentence_custody_days as f64 * 1.5) as u32;
            parts.push(format!(
                "Pre-sentence custody: {} days x 1.5 = {} days credit",
                facts.pre_sentence_custody_days, credit
            ));
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

    fn create_basic_facts() -> SentencingFacts {
        SentencingFacts {
            offence: "Theft over $5,000".to_string(),
            offence_type: OffenceType::Hybrid,
            max_sentence_months: Some(120),
            mandatory_minimum_months: None,
            aggravating: vec![],
            mitigating: vec![MitigatingFactor::FirstOffender],
            offender: OffenderInfo {
                age: 35,
                first_offender: true,
                prior_record: vec![],
                indigenous: false,
                gladue_factors: vec![],
                employed: true,
                family_support: true,
                rehabilitation_efforts: vec![],
            },
            victim_impact: None,
            pre_sentence_custody_days: 0,
        }
    }

    #[test]
    fn test_first_offender_sentence() {
        let facts = create_basic_facts();
        let result = SentencingAnalyzer::analyze(&facts);

        assert!(
            result
                .mitigating_analysis
                .iter()
                .any(|(f, _)| matches!(f, MitigatingFactor::FirstOffender))
        );
        assert!(
            result
                .primary_principles
                .contains(&SentencingPrinciple::Rehabilitation)
        );
    }

    #[test]
    fn test_aggravating_increases_sentence() {
        let mut facts = create_basic_facts();
        let baseline = SentencingAnalyzer::analyze(&facts);

        facts.aggravating = vec![
            AggravatingFactor::AbuseOfTrust,
            AggravatingFactor::VulnerableVictim,
        ];
        let with_aggravating = SentencingAnalyzer::analyze(&facts);

        assert!(with_aggravating.range.mid_months > baseline.range.mid_months);
    }

    #[test]
    fn test_gladue_analysis() {
        let mut facts = create_basic_facts();
        facts.offender.indigenous = true;
        facts.offender.gladue_factors = vec![
            GladueFactor::ResidentialSchool,
            GladueFactor::IntergenerationalTrauma,
        ];

        let result = SentencingAnalyzer::analyze(&facts);

        assert!(result.gladue_analysis.is_some());
        let gladue = result.gladue_analysis.as_ref().expect("should have Gladue");
        assert!(!gladue.factors.is_empty());
        assert!(gladue.reduction_factor > 0.0);
    }

    #[test]
    fn test_pre_sentence_credit() {
        let mut facts = create_basic_facts();
        facts.pre_sentence_custody_days = 60;

        let result = SentencingAnalyzer::analyze(&facts);

        assert_eq!(result.pre_sentence_credit_days, 90); // 60 * 1.5
    }

    #[test]
    fn test_indictable_principles() {
        let mut facts = create_basic_facts();
        facts.offence_type = OffenceType::Indictable;

        let result = SentencingAnalyzer::analyze(&facts);

        assert!(
            result
                .primary_principles
                .contains(&SentencingPrinciple::Denunciation)
        );
        assert!(
            result
                .primary_principles
                .contains(&SentencingPrinciple::Deterrence)
        );
    }
}
