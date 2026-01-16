//! UK Judicial Review
//!
//! This module provides comprehensive analysis of judicial review under English law,
//! implementing the grounds, standing requirements, procedure, and remedies.
//!
//! # Legal Framework
//!
//! Judicial review is the mechanism by which courts supervise the exercise of
//! public power. Key sources include:
//!
//! - Senior Courts Act 1981 (s.31)
//! - Civil Procedure Rules Part 54
//! - Common law principles
//!
//! # Key Cases
//!
//! - Associated Provincial Picture Houses v Wednesbury [1948] (unreasonableness)
//! - Council of Civil Service Unions v Minister [1985] (GCHQ - grounds)
//! - Anisminic v Foreign Compensation Commission [1969] (error of law)
//! - R v Secretary of State ex p Daly [2001] (proportionality)
//! - R (Miller) v Secretary of State [2017, 2019] (constitutional review)

// Allow missing docs on enum variant struct fields
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::public_law::error::{JudicialReviewError, PublicLawResult};
use crate::public_law::types::{
    BiasType, DecisionNature, EchrArticle, ExpectationType, GroundOfReview, IllegalityType,
    InjunctionType, IrrationalityType, JrAnalysisResult, JrRemedy, JrTimeLimit, ProceduralType,
    PublicBodyType, PublicLawCitation, SpecificLimit, StandingType, SuccessLikelihood,
};

// ============================================================================
// Grounds Analysis
// ============================================================================

/// Facts for grounds analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroundsFacts {
    /// Nature of the decision
    pub decision_nature: DecisionNature,
    /// Alleged illegality
    pub illegality: Option<IllegalityFacts>,
    /// Alleged irrationality
    pub irrationality: Option<IrrationalityFacts>,
    /// Alleged procedural impropriety
    pub procedural: Option<ProceduralFacts>,
    /// Legitimate expectation claim
    pub legitimate_expectation: Option<ExpectationFacts>,
    /// Human rights claim
    pub human_rights: Option<HumanRightsFacts>,
}

/// Facts for illegality ground
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IllegalityFacts {
    /// Type of illegality alleged
    pub illegality_type: IllegalityType,
    /// Statutory power (if applicable)
    pub statutory_power: Option<StatutoryPower>,
    /// Evidence of illegality
    pub evidence: Vec<String>,
}

/// Statutory power details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatutoryPower {
    /// Statute name
    pub statute: String,
    /// Section
    pub section: String,
    /// Scope of power
    pub scope: String,
}

/// Facts for irrationality ground
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrrationalityFacts {
    /// Decision made
    pub decision: String,
    /// Why unreasonable
    pub unreasonableness: String,
    /// Standard to apply
    pub standard: IrrationalityType,
    /// Is this a rights case (anxious scrutiny)?
    pub rights_engaged: bool,
}

/// Facts for procedural impropriety ground
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProceduralFacts {
    /// Type of procedural breach
    pub procedural_type: ProceduralType,
    /// What procedure was required
    pub required_procedure: String,
    /// What actually happened
    pub actual_procedure: String,
    /// Statutory requirement?
    pub statutory_basis: Option<String>,
}

/// Facts for legitimate expectation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectationFacts {
    /// Type of expectation
    pub expectation_type: ExpectationType,
    /// Promise or practice
    pub promise_or_practice: PromiseOrPractice,
    /// Was claimant aware?
    pub claimant_aware: bool,
    /// Did claimant rely on it?
    pub reliance: bool,
    /// Any countervailing public interest?
    pub countervailing_interest: Option<String>,
}

/// Source of legitimate expectation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PromiseOrPractice {
    /// Express promise
    ExpressPromise { promise: String, by_whom: String },
    /// Published policy
    Policy { policy: String },
    /// Established practice
    Practice { practice: String, duration: String },
}

/// Facts for human rights ground
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanRightsFacts {
    /// Article engaged
    pub article: EchrArticle,
    /// Nature of interference
    pub interference: String,
    /// Justification offered
    pub justification: Option<String>,
}

/// Result of grounds analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroundsAnalysisResult {
    /// Grounds identified
    pub grounds: Vec<GroundOfReview>,
    /// Strength of each ground
    pub ground_strengths: Vec<(GroundOfReview, GroundStrength)>,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis
    pub analysis: String,
}

/// Strength of a ground
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroundStrength {
    /// Strong arguable case
    Strong,
    /// Arguable
    Arguable,
    /// Weak but arguable
    Weak,
    /// Unarguable
    Unarguable,
}

/// Analyzer for judicial review grounds
pub struct GroundsAnalyzer;

impl GroundsAnalyzer {
    /// Analyze grounds for judicial review
    pub fn analyze(facts: &GroundsFacts) -> PublicLawResult<GroundsAnalysisResult> {
        let mut grounds = Vec::new();
        let mut ground_strengths = Vec::new();
        let mut case_law = vec![PublicLawCitation::new(
            "Council of Civil Service Unions v Minister for the Civil Service",
            1985,
            "AC 374",
            "GCHQ case - three grounds: illegality, irrationality, procedural impropriety",
        )];

        // Analyze illegality
        if let Some(illegality) = &facts.illegality {
            let (ground, strength) = Self::analyze_illegality(illegality, &mut case_law);
            grounds.push(ground.clone());
            ground_strengths.push((ground, strength));
        }

        // Analyze irrationality
        if let Some(irrationality) = &facts.irrationality {
            let (ground, strength) = Self::analyze_irrationality(irrationality, &mut case_law);
            grounds.push(ground.clone());
            ground_strengths.push((ground, strength));
        }

        // Analyze procedural impropriety
        if let Some(procedural) = &facts.procedural {
            let (ground, strength) = Self::analyze_procedural(procedural, &mut case_law);
            grounds.push(ground.clone());
            ground_strengths.push((ground, strength));
        }

        // Analyze legitimate expectation
        if let Some(expectation) = &facts.legitimate_expectation {
            let (ground, strength) = Self::analyze_expectation(expectation, &mut case_law);
            grounds.push(ground.clone());
            ground_strengths.push((ground, strength));
        }

        // Analyze human rights
        if let Some(hr) = &facts.human_rights {
            let ground = GroundOfReview::HumanRightsViolation {
                article: hr.article.clone(),
            };
            let strength = if hr.justification.is_none() {
                GroundStrength::Strong
            } else {
                GroundStrength::Arguable
            };
            grounds.push(ground.clone());
            ground_strengths.push((ground, strength));
        }

        let analysis = Self::summarize_analysis(&grounds, &ground_strengths);

        Ok(GroundsAnalysisResult {
            grounds,
            ground_strengths,
            case_law,
            analysis,
        })
    }

    fn analyze_illegality(
        facts: &IllegalityFacts,
        case_law: &mut Vec<PublicLawCitation>,
    ) -> (GroundOfReview, GroundStrength) {
        case_law.push(PublicLawCitation::new(
            "Anisminic Ltd v Foreign Compensation Commission",
            1969,
            "2 AC 147",
            "Any error of law renders decision void - no distinction between jurisdictional and non-jurisdictional errors",
        ));

        let strength = match &facts.illegality_type {
            IllegalityType::UltraVires => {
                if facts.statutory_power.is_some() {
                    GroundStrength::Strong
                } else {
                    GroundStrength::Arguable
                }
            }
            IllegalityType::ErrorOfLaw => GroundStrength::Strong,
            IllegalityType::FetteringDiscretion => {
                case_law.push(PublicLawCitation::new(
                    "British Oxygen Co Ltd v Board of Trade",
                    1971,
                    "AC 610",
                    "Decision-maker may have policy but must be willing to hear individual case",
                ));
                GroundStrength::Arguable
            }
            IllegalityType::FailureToConsider { .. }
            | IllegalityType::IrrelevantConsideration { .. } => GroundStrength::Arguable,
            IllegalityType::ImproperPurpose { .. } => {
                case_law.push(PublicLawCitation::new(
                    "Padfield v Minister of Agriculture",
                    1968,
                    "AC 997",
                    "Discretion must be used for purpose for which it was conferred",
                ));
                GroundStrength::Arguable
            }
            _ => GroundStrength::Arguable,
        };

        (
            GroundOfReview::Illegality(facts.illegality_type.clone()),
            strength,
        )
    }

    fn analyze_irrationality(
        facts: &IrrationalityFacts,
        case_law: &mut Vec<PublicLawCitation>,
    ) -> (GroundOfReview, GroundStrength) {
        case_law.push(PublicLawCitation::new(
            "Associated Provincial Picture Houses v Wednesbury Corporation",
            1948,
            "1 KB 223",
            "Decision so unreasonable no reasonable authority could have made it",
        ));

        let (irr_type, strength) = if facts.rights_engaged {
            case_law.push(PublicLawCitation::new(
                "R v Ministry of Defence, ex p Smith",
                1996,
                "QB 517",
                "Anxious scrutiny - lower threshold where fundamental rights engaged",
            ));
            (IrrationalityType::AnxiousScrutiny, GroundStrength::Arguable)
        } else {
            match facts.standard {
                IrrationalityType::Wednesbury => {
                    (IrrationalityType::Wednesbury, GroundStrength::Weak)
                }
                IrrationalityType::Proportionality => {
                    case_law.push(PublicLawCitation::new(
                        "R (Daly) v Secretary of State for the Home Department",
                        2001,
                        "2 AC 532",
                        "Proportionality more intensive than Wednesbury",
                    ));
                    (IrrationalityType::Proportionality, GroundStrength::Arguable)
                }
                _ => (facts.standard.clone(), GroundStrength::Arguable),
            }
        };

        (GroundOfReview::Irrationality(irr_type), strength)
    }

    fn analyze_procedural(
        facts: &ProceduralFacts,
        case_law: &mut Vec<PublicLawCitation>,
    ) -> (GroundOfReview, GroundStrength) {
        let strength = match &facts.procedural_type {
            ProceduralType::Bias(bias_type) => {
                case_law.push(PublicLawCitation::new(
                    "Porter v Magill",
                    2002,
                    "2 AC 357",
                    "Fair-minded observer test for apparent bias",
                ));
                match bias_type {
                    BiasType::Actual | BiasType::AutomaticDisqualification { .. } => {
                        GroundStrength::Strong
                    }
                    BiasType::Apparent => GroundStrength::Arguable,
                }
            }
            ProceduralType::FairHearing { .. } => {
                case_law.push(PublicLawCitation::new(
                    "Ridge v Baldwin",
                    1964,
                    "AC 40",
                    "Right to be heard before adverse decision",
                ));
                GroundStrength::Arguable
            }
            ProceduralType::FailureToGiveReasons => {
                case_law.push(PublicLawCitation::new(
                    "R v Secretary of State for the Home Department, ex p Doody",
                    1994,
                    "1 AC 531",
                    "Common law duty to give reasons in some circumstances",
                ));
                GroundStrength::Arguable
            }
            ProceduralType::FailureToConsult => GroundStrength::Arguable,
            ProceduralType::StatutoryProcedure { .. } => {
                if facts.statutory_basis.is_some() {
                    GroundStrength::Strong
                } else {
                    GroundStrength::Arguable
                }
            }
        };

        (
            GroundOfReview::ProceduralImpropriety(facts.procedural_type.clone()),
            strength,
        )
    }

    fn analyze_expectation(
        facts: &ExpectationFacts,
        case_law: &mut Vec<PublicLawCitation>,
    ) -> (GroundOfReview, GroundStrength) {
        case_law.push(PublicLawCitation::new(
            "R v North and East Devon Health Authority, ex p Coughlan",
            2001,
            "QB 213",
            "Substantive legitimate expectation can be enforced where frustration would be abuse of power",
        ));

        let strength = match &facts.expectation_type {
            ExpectationType::Procedural { .. } => {
                if facts.claimant_aware && facts.reliance {
                    GroundStrength::Arguable
                } else {
                    GroundStrength::Weak
                }
            }
            ExpectationType::Substantive { .. } => {
                // Substantive expectations harder to establish
                if facts.claimant_aware && facts.reliance && facts.countervailing_interest.is_none()
                {
                    GroundStrength::Arguable
                } else {
                    GroundStrength::Weak
                }
            }
        };

        (
            GroundOfReview::LegitimateExpectation(facts.expectation_type.clone()),
            strength,
        )
    }

    fn summarize_analysis(
        grounds: &[GroundOfReview],
        strengths: &[(GroundOfReview, GroundStrength)],
    ) -> String {
        if grounds.is_empty() {
            return "No arguable grounds identified".to_string();
        }

        let strong_count = strengths
            .iter()
            .filter(|(_, s)| *s == GroundStrength::Strong)
            .count();
        let arguable_count = strengths
            .iter()
            .filter(|(_, s)| *s == GroundStrength::Arguable)
            .count();

        format!(
            "{} ground(s) identified: {} strong, {} arguable. {}",
            grounds.len(),
            strong_count,
            arguable_count,
            if strong_count > 0 {
                "Good prospects of obtaining permission"
            } else if arguable_count > 0 {
                "Arguable case - permission may be granted"
            } else {
                "Weak case - permission unlikely"
            }
        )
    }
}

// ============================================================================
// Standing Analysis
// ============================================================================

/// Facts for standing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingFacts {
    /// Claimant type
    pub claimant_type: ClaimantType,
    /// Relationship to decision
    pub relationship: String,
    /// Direct impact?
    pub direct_impact: bool,
    /// Interest in subject matter
    pub interest: String,
    /// Nature of the public interest (for NGOs etc.)
    pub public_interest_nature: Option<String>,
}

/// Type of claimant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimantType {
    /// Individual directly affected
    Individual,
    /// Company/corporate body
    Company,
    /// Interest group/NGO
    InterestGroup { name: String },
    /// Local authority
    LocalAuthority,
    /// Public body
    PublicBody,
    /// Representative claimant
    Representative { representing: String },
}

/// Result of standing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingAnalysisResult {
    /// Has standing?
    pub has_standing: bool,
    /// Type of standing
    pub standing_type: Option<StandingType>,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis
    pub analysis: String,
}

/// Analyzer for standing
pub struct StandingAnalyzer;

impl StandingAnalyzer {
    /// Analyze standing
    pub fn analyze(facts: &StandingFacts) -> PublicLawResult<StandingAnalysisResult> {
        let mut case_law = vec![PublicLawCitation::new(
            "R v Inland Revenue Commissioners, ex p National Federation of Self-Employed",
            1982,
            "AC 617",
            "Sufficient interest test - flexible, depends on relationship to subject matter",
        )];

        let (has_standing, standing_type) = if facts.direct_impact {
            (true, Some(StandingType::DirectVictim))
        } else {
            match &facts.claimant_type {
                ClaimantType::Individual | ClaimantType::Company => {
                    if !facts.interest.is_empty() {
                        (
                            true,
                            Some(StandingType::SufficientInterest {
                                basis: facts.interest.clone(),
                            }),
                        )
                    } else {
                        (false, None)
                    }
                }
                ClaimantType::InterestGroup { name } => {
                    case_law.push(PublicLawCitation::new(
                        "R v Secretary of State for Foreign Affairs, ex p World Development Movement",
                        1995,
                        "1 WLR 386",
                        "Interest groups may have standing where expert in area and no other challenger",
                    ));
                    if facts.public_interest_nature.is_some() {
                        (
                            true,
                            Some(StandingType::PublicInterest {
                                organization: name.clone(),
                            }),
                        )
                    } else {
                        (false, None)
                    }
                }
                ClaimantType::LocalAuthority | ClaimantType::PublicBody => (
                    true,
                    Some(StandingType::SufficientInterest {
                        basis: "Public body with interest in matter".into(),
                    }),
                ),
                ClaimantType::Representative { representing } => (
                    true,
                    Some(StandingType::Representative {
                        representing: representing.clone(),
                    }),
                ),
            }
        };

        let analysis = if has_standing {
            format!(
                "Claimant has standing as {:?}",
                standing_type
                    .as_ref()
                    .map(|s| format!("{:?}", s))
                    .unwrap_or_default()
            )
        } else {
            "Claimant lacks sufficient interest - no standing".to_string()
        };

        Ok(StandingAnalysisResult {
            has_standing,
            standing_type,
            case_law,
            analysis,
        })
    }
}

// ============================================================================
// Time Limit Analysis
// ============================================================================

/// Facts for time limit analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeLimitFacts {
    /// Date of decision
    pub decision_date: String,
    /// Date of claim
    pub claim_date: String,
    /// Days elapsed
    pub days_elapsed: u32,
    /// Type of decision (for specific limits)
    pub decision_type: DecisionType,
    /// Reason for any delay
    pub delay_reason: Option<String>,
    /// Good reason for extension?
    pub extension_grounds: Option<String>,
}

/// Type of decision for time limit purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    /// General administrative decision
    General,
    /// Planning decision
    Planning,
    /// Procurement decision
    Procurement,
    /// Asylum/immigration
    Immigration,
    /// Other with specific limit
    Other { days: u32 },
}

/// Result of time limit analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeLimitResult {
    /// In time?
    pub in_time: bool,
    /// Applicable limit
    pub limit: JrTimeLimit,
    /// Extension possible?
    pub extension_possible: bool,
    /// Analysis
    pub analysis: String,
}

/// Analyzer for time limits
pub struct TimeLimitAnalyzer;

impl TimeLimitAnalyzer {
    /// Analyze time limit
    pub fn analyze(facts: &TimeLimitFacts) -> PublicLawResult<TimeLimitResult> {
        let limit = match &facts.decision_type {
            DecisionType::General => JrTimeLimit::default(),
            DecisionType::Planning => JrTimeLimit {
                general_limit_days: 42, // 6 weeks
                promptness_required: true,
                specific_limit: Some(SpecificLimit::Planning { weeks: 6 }),
            },
            DecisionType::Procurement => JrTimeLimit {
                general_limit_days: 30,
                promptness_required: false,
                specific_limit: Some(SpecificLimit::Procurement { days: 30 }),
            },
            DecisionType::Immigration => JrTimeLimit::default(),
            DecisionType::Other { days } => JrTimeLimit {
                general_limit_days: *days,
                promptness_required: false,
                specific_limit: Some(SpecificLimit::Other {
                    days: *days,
                    basis: "Specific statutory limit".into(),
                }),
            },
        };

        let in_time = facts.days_elapsed <= limit.general_limit_days;

        // Even if in time, consider promptness
        let prompt_issue = limit.promptness_required && facts.days_elapsed > 30;

        let extension_possible = !in_time && facts.extension_grounds.is_some();

        let analysis = if in_time && !prompt_issue {
            "Claim within time".to_string()
        } else if in_time && prompt_issue {
            format!(
                "Claim within {} day limit but promptness requirement may be issue after {} days",
                limit.general_limit_days, facts.days_elapsed
            )
        } else if extension_possible {
            format!(
                "Out of time by {} days but extension may be granted: {}",
                facts.days_elapsed - limit.general_limit_days,
                facts.extension_grounds.as_ref().unwrap_or(&String::new())
            )
        } else {
            format!(
                "Out of time by {} days - claim likely time-barred",
                facts.days_elapsed - limit.general_limit_days
            )
        };

        Ok(TimeLimitResult {
            in_time,
            limit,
            extension_possible,
            analysis,
        })
    }
}

// ============================================================================
// Remedies Analysis
// ============================================================================

/// Facts for remedies analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediesFacts {
    /// Grounds established
    pub grounds: Vec<GroundOfReview>,
    /// What outcome sought
    pub outcome_sought: String,
    /// Decision still operative?
    pub decision_operative: bool,
    /// Interim relief needed?
    pub interim_relief_needed: bool,
    /// Damages claimed?
    pub damages_claimed: Option<DamagesClaimFacts>,
}

/// Facts for damages claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamagesClaimFacts {
    /// Basis for damages
    pub basis: DamagesBasis,
    /// Loss suffered
    pub loss: String,
    /// Quantum
    pub quantum: Option<String>,
}

/// Damages basis (re-exported for convenience)
pub use crate::public_law::types::DamagesBasis;

/// Result of remedies analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediesAnalysisResult {
    /// Recommended remedies
    pub remedies: Vec<JrRemedy>,
    /// Interim relief appropriate?
    pub interim_relief: Option<InjunctionType>,
    /// Damages available?
    pub damages_available: bool,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis
    pub analysis: String,
}

/// Analyzer for remedies
pub struct RemediesAnalyzer;

impl RemediesAnalyzer {
    /// Analyze available remedies
    pub fn analyze(facts: &RemediesFacts) -> PublicLawResult<RemediesAnalysisResult> {
        let mut remedies = Vec::new();
        let case_law = vec![PublicLawCitation::new(
            "R v Monopolies and Mergers Commission, ex p Argyll Group",
            1986,
            "1 WLR 763",
            "Court has discretion in granting remedies",
        )];

        // Primary remedy depends on what's sought
        if facts.decision_operative {
            remedies.push(JrRemedy::QuashingOrder);
        }

        // Add declaration as alternative/additional
        remedies.push(JrRemedy::Declaration {
            content: "Decision unlawful".into(),
        });

        // Check if mandatory order needed
        if facts.outcome_sought.contains("require") || facts.outcome_sought.contains("must") {
            remedies.push(JrRemedy::MandatoryOrder {
                action_required: facts.outcome_sought.clone(),
            });
        }

        // Interim relief
        let interim_relief = if facts.interim_relief_needed {
            Some(InjunctionType::Interim)
        } else {
            None
        };

        // Damages
        let damages_available = facts.damages_claimed.as_ref().is_some_and(|d| {
            matches!(
                d.basis,
                DamagesBasis::HraDamages | DamagesBasis::Misfeasance
            )
        });

        if let Some(damages) = &facts.damages_claimed {
            if damages_available {
                remedies.push(JrRemedy::Damages {
                    basis: damages.basis.clone(),
                });
            }
        }

        let analysis = format!(
            "Recommended remedies: {}. {}",
            remedies
                .iter()
                .map(|r| format!("{:?}", r))
                .collect::<Vec<_>>()
                .join(", "),
            if damages_available {
                "Damages may be available"
            } else {
                "Damages unlikely"
            }
        );

        Ok(RemediesAnalysisResult {
            remedies,
            interim_relief,
            damages_available,
            case_law,
            analysis,
        })
    }
}

// ============================================================================
// Full JR Analysis
// ============================================================================

/// Complete judicial review analysis facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JrFacts {
    /// Public body
    pub public_body: PublicBodyType,
    /// Standing facts
    pub standing: StandingFacts,
    /// Time limit facts
    pub time_limit: TimeLimitFacts,
    /// Grounds facts
    pub grounds: GroundsFacts,
    /// Remedies facts
    pub remedies: RemediesFacts,
}

/// Full JR analyzer
pub struct JudicialReviewAnalyzer;

impl JudicialReviewAnalyzer {
    /// Perform full judicial review analysis
    pub fn analyze(facts: &JrFacts) -> PublicLawResult<JrAnalysisResult> {
        // Analyze standing
        let standing_result = StandingAnalyzer::analyze(&facts.standing)?;
        if !standing_result.has_standing {
            return Err(JudicialReviewError::NoStanding {
                reason: standing_result.analysis,
            }
            .into());
        }

        // Analyze time
        let time_result = TimeLimitAnalyzer::analyze(&facts.time_limit)?;
        let in_time = time_result.in_time || time_result.extension_possible;

        // Analyze grounds
        let grounds_result = GroundsAnalyzer::analyze(&facts.grounds)?;
        if grounds_result.grounds.is_empty() {
            return Err(JudicialReviewError::NoArguableGround {
                analysis: "No grounds identified".into(),
            }
            .into());
        }

        // Analyze remedies
        let remedies_result = RemediesAnalyzer::analyze(&facts.remedies)?;

        // Determine success likelihood
        let success_likelihood = Self::assess_likelihood(&grounds_result);

        // Compile case law
        let mut case_law = Vec::new();
        case_law.extend(standing_result.case_law);
        case_law.extend(grounds_result.case_law);
        case_law.extend(remedies_result.case_law);

        let analysis = format!(
            "Standing: {}. Time: {}. Grounds: {}. Remedies: {}",
            standing_result.analysis,
            time_result.analysis,
            grounds_result.analysis,
            remedies_result.analysis
        );

        Ok(JrAnalysisResult {
            reviewable: true,
            standing: standing_result.has_standing,
            in_time,
            grounds: grounds_result.grounds,
            success_likelihood,
            remedies: remedies_result.remedies,
            case_law,
            analysis,
        })
    }

    fn assess_likelihood(grounds: &GroundsAnalysisResult) -> SuccessLikelihood {
        let strong_count = grounds
            .ground_strengths
            .iter()
            .filter(|(_, s)| *s == GroundStrength::Strong)
            .count();
        let arguable_count = grounds
            .ground_strengths
            .iter()
            .filter(|(_, s)| *s == GroundStrength::Arguable)
            .count();

        if strong_count >= 2 {
            SuccessLikelihood::Strong
        } else if strong_count >= 1 {
            SuccessLikelihood::GoodProspects
        } else if arguable_count >= 2 {
            SuccessLikelihood::ReasonableProspects
        } else if arguable_count >= 1 {
            SuccessLikelihood::Arguable
        } else {
            SuccessLikelihood::Weak
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
    fn test_grounds_analysis_illegality() {
        let facts = GroundsFacts {
            decision_nature: DecisionNature::Statutory {
                statute: "Immigration Act 1971".into(),
                section: "s.3".into(),
            },
            illegality: Some(IllegalityFacts {
                illegality_type: IllegalityType::ErrorOfLaw,
                statutory_power: Some(StatutoryPower {
                    statute: "Immigration Act 1971".into(),
                    section: "s.3".into(),
                    scope: "Power to refuse entry".into(),
                }),
                evidence: vec!["Misinterpreted statute".into()],
            }),
            irrationality: None,
            procedural: None,
            legitimate_expectation: None,
            human_rights: None,
        };

        let result = GroundsAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(!result.grounds.is_empty());
        assert!(matches!(result.grounds[0], GroundOfReview::Illegality(_)));
    }

    #[test]
    fn test_standing_direct_victim() {
        let facts = StandingFacts {
            claimant_type: ClaimantType::Individual,
            relationship: "Recipient of decision".into(),
            direct_impact: true,
            interest: "Directly affected by refusal".into(),
            public_interest_nature: None,
        };

        let result = StandingAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.has_standing);
        assert!(matches!(
            result.standing_type,
            Some(StandingType::DirectVictim)
        ));
    }

    #[test]
    fn test_standing_interest_group() {
        let facts = StandingFacts {
            claimant_type: ClaimantType::InterestGroup {
                name: "Environmental NGO".into(),
            },
            relationship: "Expert in environmental law".into(),
            direct_impact: false,
            interest: "Protection of environment".into(),
            public_interest_nature: Some("Environmental protection".into()),
        };

        let result = StandingAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.has_standing);
    }

    #[test]
    fn test_time_limit_in_time() {
        let facts = TimeLimitFacts {
            decision_date: "2024-01-01".into(),
            claim_date: "2024-02-15".into(),
            days_elapsed: 45,
            decision_type: DecisionType::General,
            delay_reason: None,
            extension_grounds: None,
        };

        let result = TimeLimitAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.in_time);
    }

    #[test]
    fn test_time_limit_planning() {
        let facts = TimeLimitFacts {
            decision_date: "2024-01-01".into(),
            claim_date: "2024-03-01".into(),
            days_elapsed: 60,
            decision_type: DecisionType::Planning,
            delay_reason: None,
            extension_grounds: None,
        };

        let result = TimeLimitAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(!result.in_time); // 6 weeks = 42 days
    }

    #[test]
    fn test_remedies_quashing_order() {
        let facts = RemediesFacts {
            grounds: vec![GroundOfReview::Illegality(IllegalityType::UltraVires)],
            outcome_sought: "Quash the decision".into(),
            decision_operative: true,
            interim_relief_needed: false,
            damages_claimed: None,
        };

        let result = RemediesAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.remedies.contains(&JrRemedy::QuashingOrder));
    }

    #[test]
    fn test_procedural_impropriety_bias() {
        let facts = GroundsFacts {
            decision_nature: DecisionNature::IndividualDetermination,
            illegality: None,
            irrationality: None,
            procedural: Some(ProceduralFacts {
                procedural_type: ProceduralType::Bias(BiasType::Apparent),
                required_procedure: "Impartial decision-maker".into(),
                actual_procedure: "Decision-maker had prior involvement".into(),
                statutory_basis: None,
            }),
            legitimate_expectation: None,
            human_rights: None,
        };

        let result = GroundsAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(matches!(
            result.grounds[0],
            GroundOfReview::ProceduralImpropriety(_)
        ));
    }
}
