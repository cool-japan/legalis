//! Canada Employment Law - Termination Analysis
//!
//! Analyzers for termination, wrongful dismissal, and reasonable notice.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    BardalFactor, EmploymentCase, EmploymentJurisdiction, JustCauseGround, MitigationRequirement,
    TerminationType, WrongfulDismissalDamages,
};
use crate::common::Province;

// ============================================================================
// Reasonable Notice Analysis
// ============================================================================

/// Facts for reasonable notice analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonableNoticeFacts {
    /// Jurisdiction
    pub jurisdiction: EmploymentJurisdiction,
    /// Job title/position
    pub position: String,
    /// Length of service (months)
    pub service_months: u32,
    /// Age at termination
    pub age: u32,
    /// Annual salary (cents)
    pub annual_salary_cents: i64,
    /// Whether senior/managerial position
    pub is_senior_position: bool,
    /// Whether specialized skills
    pub specialized_skills: bool,
    /// Current job market conditions
    pub job_market: JobMarketConditions,
    /// Industry
    pub industry: String,
}

/// Job market conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobMarketConditions {
    /// Good - easy to find similar work
    Favourable,
    /// Average
    Average,
    /// Poor - difficult to find similar work
    Unfavourable,
    /// Very poor - specialized field with few opportunities
    VeryUnfavourable,
}

/// Result of reasonable notice analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonableNoticeResult {
    /// Statutory minimum notice (months)
    pub statutory_minimum_months: f64,
    /// Common law reasonable notice (months)
    pub common_law_months: f64,
    /// Recommended notice range (min, max months)
    pub notice_range: (f64, f64),
    /// Bardal factor analysis
    pub bardal_analysis: BardalAnalysis,
    /// Key cases
    pub key_cases: Vec<EmploymentCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of Bardal factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BardalAnalysis {
    /// Character of employment score (1-5)
    pub character_score: u8,
    /// Length of service score (1-5)
    pub service_score: u8,
    /// Age score (1-5)
    pub age_score: u8,
    /// Availability of employment score (1-5)
    pub availability_score: u8,
    /// Analysis
    pub analysis: Vec<(BardalFactor, String)>,
}

/// Analyzer for reasonable notice
pub struct ReasonableNoticeAnalyzer;

impl ReasonableNoticeAnalyzer {
    /// Analyze reasonable notice period
    pub fn analyze(facts: &ReasonableNoticeFacts) -> ReasonableNoticeResult {
        let mut key_cases = vec![EmploymentCase::bardal()];

        // Calculate statutory minimum
        let statutory_minimum = Self::calculate_statutory_minimum(facts);

        // Calculate common law notice using Bardal factors
        let bardal_analysis = Self::analyze_bardal(facts);

        // Calculate common law range
        let common_law = Self::calculate_common_law(&bardal_analysis, facts);
        let notice_range = Self::calculate_range(common_law);

        // Add Wallace/Keays if relevant
        if facts.is_senior_position {
            key_cases.push(EmploymentCase::keays_v_honda());
        }

        let reasoning = Self::build_reasoning(&bardal_analysis, statutory_minimum, common_law);

        ReasonableNoticeResult {
            statutory_minimum_months: statutory_minimum,
            common_law_months: common_law,
            notice_range,
            bardal_analysis,
            key_cases,
            reasoning,
        }
    }

    /// Calculate statutory minimum notice
    fn calculate_statutory_minimum(facts: &ReasonableNoticeFacts) -> f64 {
        // Varies by jurisdiction but generally 1 week per year up to 8 weeks
        let years = facts.service_months as f64 / 12.0;

        match &facts.jurisdiction {
            EmploymentJurisdiction::Federal => {
                // Canada Labour Code: 2 weeks after 3 months
                if years >= 0.25 {
                    0.5 // 2 weeks
                } else {
                    0.0
                }
            }
            EmploymentJurisdiction::Provincial { province } => {
                Self::provincial_statutory_minimum(province, years)
            }
        }
    }

    /// Provincial statutory minimums
    fn provincial_statutory_minimum(province: &Province, years: f64) -> f64 {
        // Most provinces: 1 week per year up to 8 weeks
        match province {
            Province::Ontario => {
                // ESA 2000: 1 week per year, max 8 weeks
                let weeks = years.min(8.0);
                weeks / 4.0 // Convert to months
            }
            Province::BritishColumbia => {
                // ESA: 1 week per year after 3 months, max 8 weeks
                if years < 0.25 {
                    0.0
                } else {
                    let weeks = years.min(8.0);
                    weeks / 4.0
                }
            }
            _ => {
                // Default: 1 week per year
                let weeks = years.min(8.0);
                weeks / 4.0
            }
        }
    }

    /// Analyze Bardal factors
    fn analyze_bardal(facts: &ReasonableNoticeFacts) -> BardalAnalysis {
        let mut analysis = Vec::new();

        // Character of employment (1-5)
        let character_score = if facts.is_senior_position {
            if facts.specialized_skills { 5 } else { 4 }
        } else if facts.specialized_skills {
            3
        } else {
            2
        };
        analysis.push((
            BardalFactor::CharacterOfEmployment,
            format!(
                "Position: {}. Senior: {}. Specialized: {}. Score: {}/5",
                facts.position, facts.is_senior_position, facts.specialized_skills, character_score
            ),
        ));

        // Length of service (1-5)
        let service_years = facts.service_months as f64 / 12.0;
        let service_score = if service_years >= 20.0 {
            5
        } else if service_years >= 10.0 {
            4
        } else if service_years >= 5.0 {
            3
        } else if service_years >= 2.0 {
            2
        } else {
            1
        };
        analysis.push((
            BardalFactor::LengthOfService,
            format!(
                "{:.1} years of service. Score: {}/5",
                service_years, service_score
            ),
        ));

        // Age (1-5)
        let age_score = if facts.age >= 55 {
            5
        } else if facts.age >= 50 {
            4
        } else if facts.age >= 45 {
            3
        } else if facts.age >= 35 {
            2
        } else {
            1
        };
        analysis.push((
            BardalFactor::AgeOfEmployee,
            format!("Age: {}. Score: {}/5", facts.age, age_score),
        ));

        // Availability of similar employment (1-5)
        let availability_score = match facts.job_market {
            JobMarketConditions::Favourable => 1,
            JobMarketConditions::Average => 2,
            JobMarketConditions::Unfavourable => 4,
            JobMarketConditions::VeryUnfavourable => 5,
        };
        analysis.push((
            BardalFactor::AvailabilityOfEmployment,
            format!(
                "Job market: {:?}. Industry: {}. Score: {}/5",
                facts.job_market, facts.industry, availability_score
            ),
        ));

        BardalAnalysis {
            character_score,
            service_score,
            age_score,
            availability_score,
            analysis,
        }
    }

    /// Calculate common law notice
    fn calculate_common_law(bardal: &BardalAnalysis, facts: &ReasonableNoticeFacts) -> f64 {
        // Base calculation: approximately 1 month per year of service
        let base_months = facts.service_months as f64 / 12.0;

        // Adjust based on Bardal factors (weighted average)
        let total_score = (bardal.character_score as f64 * 1.5
            + bardal.service_score as f64 * 1.0
            + bardal.age_score as f64 * 1.5
            + bardal.availability_score as f64 * 1.0)
            / 5.0;

        // Multiplier based on score (average score of 3 = 1.0x)
        let multiplier = total_score / 3.0;

        // Common law notice with adjustments
        let notice = base_months * multiplier;

        // Cap at 24 months (general guideline), minimum 1 month
        notice.clamp(1.0, 24.0)
    }

    /// Calculate notice range
    fn calculate_range(common_law: f64) -> (f64, f64) {
        let min = (common_law * 0.8).clamp(1.0, 24.0);
        let max = (common_law * 1.2).clamp(1.0, 24.0);
        (min, max)
    }

    /// Build reasoning
    fn build_reasoning(bardal: &BardalAnalysis, statutory: f64, common_law: f64) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Statutory minimum: {:.1} months. Common law estimate: {:.1} months.",
            statutory, common_law
        ));

        parts.push(format!(
            "Bardal factors: Character {}/5, Service {}/5, Age {}/5, Availability {}/5.",
            bardal.character_score,
            bardal.service_score,
            bardal.age_score,
            bardal.availability_score
        ));

        parts.join(" ")
    }
}

// ============================================================================
// Just Cause Analysis
// ============================================================================

/// Facts for just cause analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JustCauseFacts {
    /// Alleged misconduct
    pub misconduct: Vec<JustCauseGround>,
    /// Description of conduct
    pub conduct_description: String,
    /// Whether progressive discipline used
    pub progressive_discipline: bool,
    /// Prior warnings
    pub prior_warnings: u32,
    /// Length of service (months)
    pub service_months: u32,
    /// Employee's explanation
    pub employee_explanation: Option<String>,
    /// Whether conduct isolated
    pub isolated_incident: bool,
    /// Impact on workplace
    pub workplace_impact: String,
    /// Whether employer contributed
    pub employer_contribution: bool,
}

/// Result of just cause analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JustCauseResult {
    /// Whether just cause established
    pub just_cause_established: bool,
    /// Grounds analysis
    pub grounds_analysis: Vec<GroundAnalysis>,
    /// Proportionality assessment
    pub proportionality: ProportionalityAssessment,
    /// Recommended outcome
    pub recommended_outcome: JustCauseOutcome,
    /// Key cases
    pub key_cases: Vec<EmploymentCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of specific ground
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundAnalysis {
    /// Ground
    pub ground: JustCauseGround,
    /// Whether established on facts
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Proportionality assessment (McKinley)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProportionalityAssessment {
    /// Whether proportionate response
    pub is_proportionate: bool,
    /// Mitigating factors
    pub mitigating_factors: Vec<String>,
    /// Aggravating factors
    pub aggravating_factors: Vec<String>,
    /// Assessment
    pub assessment: String,
}

/// Just cause outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustCauseOutcome {
    /// Just cause established - no notice required
    JustCauseEstablished,
    /// Insufficient for just cause - termination without cause
    InsufficientForJustCause,
    /// Suspension more appropriate
    SuspensionAppropriate,
    /// Warning more appropriate
    WarningAppropriate,
}

/// Analyzer for just cause
pub struct JustCauseAnalyzer;

impl JustCauseAnalyzer {
    /// Analyze just cause
    pub fn analyze(facts: &JustCauseFacts) -> JustCauseResult {
        let key_cases = vec![EmploymentCase::mckinley()];

        // Analyze each ground
        let grounds_analysis: Vec<_> = facts
            .misconduct
            .iter()
            .map(|g| Self::analyze_ground(g, facts))
            .collect();

        // At least one ground must be established
        let any_established = grounds_analysis.iter().any(|a| a.established);

        // Proportionality assessment
        let proportionality = Self::assess_proportionality(facts);

        // Determine outcome
        let recommended_outcome = if any_established && proportionality.is_proportionate {
            JustCauseOutcome::JustCauseEstablished
        } else if any_established && !proportionality.is_proportionate {
            if facts.prior_warnings == 0 {
                JustCauseOutcome::WarningAppropriate
            } else {
                JustCauseOutcome::SuspensionAppropriate
            }
        } else {
            JustCauseOutcome::InsufficientForJustCause
        };

        let just_cause_established =
            matches!(recommended_outcome, JustCauseOutcome::JustCauseEstablished);

        let reasoning =
            Self::build_reasoning(&grounds_analysis, &proportionality, just_cause_established);

        JustCauseResult {
            just_cause_established,
            grounds_analysis,
            proportionality,
            recommended_outcome,
            key_cases,
            reasoning,
        }
    }

    /// Analyze specific ground
    fn analyze_ground(ground: &JustCauseGround, facts: &JustCauseFacts) -> GroundAnalysis {
        let established = match ground {
            JustCauseGround::Dishonesty | JustCauseGround::Theft => {
                // High threshold - must be fundamental incompatibility
                !facts.isolated_incident || facts.prior_warnings > 0
            }
            JustCauseGround::Insubordination => {
                // Requires clear refusal and usually warnings
                facts.prior_warnings > 0 || !facts.isolated_incident
            }
            JustCauseGround::Incompetence => {
                // Requires opportunity to improve
                facts.progressive_discipline && facts.prior_warnings >= 2
            }
            JustCauseGround::Violence | JustCauseGround::Harassment => {
                // May warrant immediate dismissal
                true
            }
            _ => {
                // Context-dependent
                facts.prior_warnings > 0 && !facts.isolated_incident
            }
        };

        let reasoning = if established {
            format!(
                "{:?} established - sufficient for just cause analysis",
                ground
            )
        } else {
            format!(
                "{:?} - insufficient evidence or mitigating circumstances",
                ground
            )
        };

        GroundAnalysis {
            ground: ground.clone(),
            established,
            reasoning,
        }
    }

    /// Assess proportionality (McKinley contextual approach)
    fn assess_proportionality(facts: &JustCauseFacts) -> ProportionalityAssessment {
        let mut mitigating = Vec::new();
        let mut aggravating = Vec::new();

        // Mitigating factors
        if facts.service_months > 120 {
            mitigating.push(format!(
                "Long service ({:.1} years)",
                facts.service_months as f64 / 12.0
            ));
        }
        if facts.isolated_incident {
            mitigating.push("Isolated incident".to_string());
        }
        if facts.prior_warnings == 0 {
            mitigating.push("No prior discipline".to_string());
        }
        if facts.employee_explanation.is_some() {
            mitigating.push("Employee provided explanation".to_string());
        }
        if facts.employer_contribution {
            mitigating.push("Employer contributed to situation".to_string());
        }

        // Aggravating factors
        if facts.prior_warnings > 2 {
            aggravating.push(format!("{} prior warnings", facts.prior_warnings));
        }
        if !facts.isolated_incident {
            aggravating.push("Pattern of conduct".to_string());
        }
        if !facts.workplace_impact.is_empty() {
            aggravating.push(format!("Workplace impact: {}", facts.workplace_impact));
        }

        // Balance
        let is_proportionate = aggravating.len() >= mitigating.len()
            || facts.misconduct.iter().any(|g| {
                matches!(
                    g,
                    JustCauseGround::Violence
                        | JustCauseGround::Theft
                        | JustCauseGround::Harassment
                )
            });

        let assessment = if is_proportionate {
            "Dismissal is proportionate response given circumstances (McKinley)".to_string()
        } else {
            "Dismissal may not be proportionate - consider lesser discipline".to_string()
        };

        ProportionalityAssessment {
            is_proportionate,
            mitigating_factors: mitigating,
            aggravating_factors: aggravating,
            assessment,
        }
    }

    /// Build reasoning
    fn build_reasoning(
        grounds: &[GroundAnalysis],
        proportionality: &ProportionalityAssessment,
        established: bool,
    ) -> String {
        let established_grounds: Vec<_> = grounds.iter().filter(|g| g.established).collect();

        if established {
            format!(
                "Just cause established. Grounds: {:?}. {}",
                established_grounds
                    .iter()
                    .map(|g| format!("{:?}", g.ground))
                    .collect::<Vec<_>>(),
                proportionality.assessment
            )
        } else {
            format!("Just cause not established. {}", proportionality.assessment)
        }
    }
}

// ============================================================================
// Wrongful Dismissal Analysis
// ============================================================================

/// Facts for wrongful dismissal analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongfulDismissalFacts {
    /// Type of termination
    pub termination_type: TerminationType,
    /// Notice provided (months)
    pub notice_provided_months: f64,
    /// Reasonable notice facts
    pub notice_facts: ReasonableNoticeFacts,
    /// Whether just cause asserted
    pub just_cause_asserted: bool,
    /// Just cause facts (if asserted)
    pub just_cause_facts: Option<JustCauseFacts>,
    /// Mitigation
    pub mitigation: MitigationRequirement,
    /// Bad faith in manner of dismissal
    pub bad_faith: bool,
    /// Bad faith conduct
    pub bad_faith_conduct: Vec<String>,
}

/// Result of wrongful dismissal analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongfulDismissalResult {
    /// Whether wrongful dismissal occurred
    pub wrongful_dismissal: bool,
    /// Reasonable notice analysis
    pub notice_analysis: ReasonableNoticeResult,
    /// Just cause analysis (if applicable)
    pub just_cause_analysis: Option<JustCauseResult>,
    /// Damages available
    pub damages: Vec<WrongfulDismissalDamages>,
    /// Estimated damages (cents)
    pub estimated_damages_cents: Option<i64>,
    /// Key cases
    pub key_cases: Vec<EmploymentCase>,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for wrongful dismissal
pub struct WrongfulDismissalAnalyzer;

impl WrongfulDismissalAnalyzer {
    /// Analyze wrongful dismissal claim
    pub fn analyze(facts: &WrongfulDismissalFacts) -> WrongfulDismissalResult {
        let mut key_cases = Vec::new();

        // Analyze just cause if asserted
        let just_cause_analysis = facts
            .just_cause_facts
            .as_ref()
            .map(JustCauseAnalyzer::analyze);

        // If just cause established, no wrongful dismissal
        let just_cause_established = just_cause_analysis
            .as_ref()
            .is_some_and(|r| r.just_cause_established);

        // Analyze reasonable notice
        let notice_analysis = ReasonableNoticeAnalyzer::analyze(&facts.notice_facts);
        key_cases.extend(notice_analysis.key_cases.clone());

        // Determine if wrongful dismissal
        let wrongful_dismissal = !just_cause_established
            && facts.notice_provided_months < notice_analysis.common_law_months;

        // Determine damages
        let (damages, estimated_damages) = if wrongful_dismissal {
            Self::calculate_damages(facts, &notice_analysis)
        } else {
            (vec![], None)
        };

        // Add relevant cases
        if facts.bad_faith {
            key_cases.push(EmploymentCase::keays_v_honda());
            key_cases.push(EmploymentCase::wallace());
        }

        let reasoning = Self::build_reasoning(
            wrongful_dismissal,
            just_cause_established,
            &notice_analysis,
            facts,
        );

        WrongfulDismissalResult {
            wrongful_dismissal,
            notice_analysis,
            just_cause_analysis,
            damages,
            estimated_damages_cents: estimated_damages,
            key_cases,
            reasoning,
        }
    }

    /// Calculate damages
    fn calculate_damages(
        facts: &WrongfulDismissalFacts,
        notice: &ReasonableNoticeResult,
    ) -> (Vec<WrongfulDismissalDamages>, Option<i64>) {
        let mut damages = vec![WrongfulDismissalDamages::PayInLieuOfNotice];

        // Calculate pay in lieu
        let shortfall_months = notice.common_law_months - facts.notice_provided_months;
        let monthly_salary = facts.notice_facts.annual_salary_cents / 12;
        let mut total = (monthly_salary as f64 * shortfall_months) as i64;

        // Benefits continuation
        damages.push(WrongfulDismissalDamages::BenefitsContinuation);

        // Bad faith damages
        if facts.bad_faith {
            damages.push(WrongfulDismissalDamages::AggravatedDamages);
            // Add approximately 10% for bad faith
            total = (total as f64 * 1.1) as i64;
        }

        // Mitigation reduces damages
        if let Some(new_income) = facts.mitigation.new_income_cents {
            total -= new_income;
        }

        (damages, Some(total.max(0)))
    }

    /// Build reasoning
    fn build_reasoning(
        wrongful: bool,
        just_cause: bool,
        notice: &ReasonableNoticeResult,
        facts: &WrongfulDismissalFacts,
    ) -> String {
        if just_cause {
            "Just cause established - no wrongful dismissal claim.".to_string()
        } else if wrongful {
            format!(
                "Wrongful dismissal established. Reasonable notice: {:.1} months. \
                Notice provided: {:.1} months. Shortfall: {:.1} months.",
                notice.common_law_months,
                facts.notice_provided_months,
                notice.common_law_months - facts.notice_provided_months
            )
        } else {
            format!(
                "No wrongful dismissal. Notice provided ({:.1} months) meets or exceeds \
                reasonable notice ({:.1} months).",
                facts.notice_provided_months, notice.common_law_months
            )
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
    fn test_reasonable_notice_senior() {
        let facts = ReasonableNoticeFacts {
            jurisdiction: EmploymentJurisdiction::Provincial {
                province: Province::Ontario,
            },
            position: "Vice President".to_string(),
            service_months: 120, // 10 years
            age: 55,
            annual_salary_cents: 20_000_000,
            is_senior_position: true,
            specialized_skills: true,
            job_market: JobMarketConditions::Unfavourable,
            industry: "Technology".to_string(),
        };

        let result = ReasonableNoticeAnalyzer::analyze(&facts);
        assert!(result.common_law_months > result.statutory_minimum_months);
        assert!(result.common_law_months >= 12.0);
    }

    #[test]
    fn test_reasonable_notice_junior() {
        let facts = ReasonableNoticeFacts {
            jurisdiction: EmploymentJurisdiction::Provincial {
                province: Province::Ontario,
            },
            position: "Clerk".to_string(),
            service_months: 24, // 2 years
            age: 25,
            annual_salary_cents: 4_000_000,
            is_senior_position: false,
            specialized_skills: false,
            job_market: JobMarketConditions::Favourable,
            industry: "Retail".to_string(),
        };

        let result = ReasonableNoticeAnalyzer::analyze(&facts);
        assert!(result.common_law_months >= 1.0);
        assert!(result.common_law_months <= 6.0);
    }

    #[test]
    fn test_just_cause_violence() {
        let facts = JustCauseFacts {
            misconduct: vec![JustCauseGround::Violence],
            conduct_description: "Physical assault of coworker".to_string(),
            progressive_discipline: false,
            prior_warnings: 0,
            service_months: 60,
            employee_explanation: None,
            isolated_incident: true,
            workplace_impact: "Other employees afraid".to_string(),
            employer_contribution: false,
        };

        let result = JustCauseAnalyzer::analyze(&facts);
        assert!(result.just_cause_established);
    }

    #[test]
    fn test_just_cause_minor_misconduct() {
        let facts = JustCauseFacts {
            misconduct: vec![JustCauseGround::Insubordination],
            conduct_description: "Disagreed with manager".to_string(),
            progressive_discipline: false,
            prior_warnings: 0,
            service_months: 120, // 10 years
            employee_explanation: Some("Misunderstanding".to_string()),
            isolated_incident: true,
            workplace_impact: String::new(),
            employer_contribution: true,
        };

        let result = JustCauseAnalyzer::analyze(&facts);
        assert!(!result.just_cause_established);
        assert!(matches!(
            result.recommended_outcome,
            JustCauseOutcome::WarningAppropriate | JustCauseOutcome::InsufficientForJustCause
        ));
    }

    #[test]
    fn test_wrongful_dismissal() {
        let notice_facts = ReasonableNoticeFacts {
            jurisdiction: EmploymentJurisdiction::Provincial {
                province: Province::Ontario,
            },
            position: "Manager".to_string(),
            service_months: 60, // 5 years
            age: 45,
            annual_salary_cents: 10_000_000,
            is_senior_position: true,
            specialized_skills: false,
            job_market: JobMarketConditions::Average,
            industry: "Finance".to_string(),
        };

        let facts = WrongfulDismissalFacts {
            termination_type: TerminationType::WithoutCause,
            notice_provided_months: 1.0, // Only 1 month provided
            notice_facts,
            just_cause_asserted: false,
            just_cause_facts: None,
            mitigation: MitigationRequirement {
                required: true,
                efforts: vec!["Applied to jobs".to_string()],
                efforts_reasonable: true,
                new_income_cents: None,
            },
            bad_faith: false,
            bad_faith_conduct: vec![],
        };

        let result = WrongfulDismissalAnalyzer::analyze(&facts);
        assert!(result.wrongful_dismissal);
        assert!(result.estimated_damages_cents.is_some());
    }
}
