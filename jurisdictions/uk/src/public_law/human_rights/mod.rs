//! UK Human Rights Act 1998
//!
//! This module provides analysis of human rights claims under the Human Rights Act 1998,
//! which incorporated the European Convention on Human Rights into UK law.
//!
//! # Legal Framework
//!
//! ## Key Provisions
//!
//! - **s.2**: Duty to take into account ECtHR jurisprudence
//! - **s.3**: Duty to interpret legislation compatibly "so far as possible"
//! - **s.4**: Power to make declaration of incompatibility
//! - **s.6**: Unlawful for public authority to act incompatibly with Convention
//! - **s.7**: Victim can bring proceedings against public authority
//! - **s.8**: Court may grant "just satisfaction" remedy
//!
//! # Key Cases
//!
//! - Ghaidan v Godin-Mendoza \[2004\] (s.3 interpretation)
//! - R (Daly) v Secretary of State \[2001\] (proportionality)
//! - YL v Birmingham City Council \[2007\] (s.6 public authority)
//! - Rabone v Pennine Care NHS Trust \[2012\] (operational duty)

// Allow missing docs on enum variant struct fields
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::public_law::error::{HumanRightsError, PublicLawResult};
use crate::public_law::types::{
    DeclarationOfIncompatibility, EchrArticle, HraAnalysisResult, HraDuty, LegitimateAim,
    ProportionalityAnalysis, PublicLawCitation, Section3Outcome, Section6Authority,
};

// ============================================================================
// HRA Analysis Facts
// ============================================================================

/// Facts for HRA analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HraFacts {
    /// Claimant facts
    pub claimant: ClaimantFacts,
    /// Respondent facts
    pub respondent: RespondentFacts,
    /// Article engagement
    pub article_engagement: Vec<ArticleEngagement>,
    /// Time since act complained of
    pub days_since_act: u32,
}

/// Claimant facts for HRA
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimantFacts {
    /// Is claimant a victim?
    pub is_victim: bool,
    /// How affected
    pub how_affected: String,
    /// Direct or indirect victim
    pub victim_type: VictimType,
}

/// Type of victim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictimType {
    /// Direct victim
    Direct,
    /// Indirect victim (family member of deceased etc.)
    Indirect { relationship: String },
    /// Potential victim (facing deportation etc.)
    Potential { risk: String },
}

/// Respondent facts for HRA
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RespondentFacts {
    /// Name of respondent
    pub name: String,
    /// Type of body
    pub body_type: BodyType,
    /// Function being exercised
    pub function: String,
}

/// Type of body for s.6 analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyType {
    /// Core public authority (courts, government, police)
    CorePublicAuthority,
    /// Hybrid body exercising public functions
    HybridBody { private_functions_too: bool },
    /// Private body
    Private,
}

/// Facts about article engagement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticleEngagement {
    /// Which article
    pub article: EchrArticle,
    /// Nature of interference
    pub interference: InterferenceFacts,
    /// Justification (for qualified rights)
    pub justification: Option<JustificationFacts>,
}

/// Facts about interference with rights
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterferenceFacts {
    /// Description of interference
    pub description: String,
    /// Duty type (positive/negative/procedural)
    pub duty_type: HraDuty,
    /// Severity of interference
    pub severity: InterferenceSeverity,
}

/// Severity of interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterferenceSeverity {
    /// Minor interference
    Minor,
    /// Significant interference
    Significant,
    /// Severe interference
    Severe,
    /// Total denial of right
    Total,
}

/// Facts about justification for interference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JustificationFacts {
    /// Legal basis for interference
    pub legal_basis: String,
    /// Legitimate aim pursued
    pub legitimate_aim: LegitimateAim,
    /// Necessary in democratic society?
    pub necessity_argument: String,
    /// Proportionality factors
    pub proportionality_factors: Vec<String>,
}

// ============================================================================
// Article Analysis
// ============================================================================

/// Result of article analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticleAnalysisResult {
    /// Article analyzed
    pub article: EchrArticle,
    /// Is article engaged?
    pub engaged: bool,
    /// Is there an interference?
    pub interference_found: bool,
    /// Is interference justified? (for qualified rights)
    pub justified: Option<bool>,
    /// Proportionality analysis (if applicable)
    pub proportionality: Option<ProportionalityAnalysis>,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis reasoning
    pub analysis: String,
}

/// Analyzer for individual articles
pub struct ArticleAnalyzer;

impl ArticleAnalyzer {
    /// Analyze engagement and breach of article
    pub fn analyze(engagement: &ArticleEngagement) -> PublicLawResult<ArticleAnalysisResult> {
        let mut case_law = Vec::new();

        // Determine if engaged based on article type
        let engaged = true; // Assume engaged if facts provided

        // Check if interference exists
        let interference_found = !engagement.interference.description.is_empty();

        // For qualified rights, analyze justification
        let (justified, proportionality) = if engagement.article.is_qualified() {
            Self::analyze_qualified_right(engagement, &mut case_law)
        } else if engagement.article.is_absolute() {
            // Absolute rights - no justification possible
            case_law.push(PublicLawCitation::new(
                "Chahal v United Kingdom",
                1996,
                "23 EHRR 413",
                "Article 3 is absolute - no balancing of state interests permitted",
            ));
            (Some(false), None) // Any interference is breach
        } else {
            // Limited rights
            Self::analyze_limited_right(engagement, &mut case_law)
        };

        let analysis = Self::build_analysis(&engagement.article, interference_found, justified);

        Ok(ArticleAnalysisResult {
            article: engagement.article.clone(),
            engaged,
            interference_found,
            justified,
            proportionality,
            case_law,
            analysis,
        })
    }

    fn analyze_qualified_right(
        engagement: &ArticleEngagement,
        case_law: &mut Vec<PublicLawCitation>,
    ) -> (Option<bool>, Option<ProportionalityAnalysis>) {
        case_law.push(PublicLawCitation::new(
            "R (Daly) v Secretary of State for the Home Department",
            2001,
            "2 AC 532",
            "Proportionality requires examining whether measure is suitable, necessary, and strikes fair balance",
        ));

        if let Some(justification) = &engagement.justification {
            // Check if legal basis exists
            let has_legal_basis = !justification.legal_basis.is_empty();

            // Check legitimate aim
            let has_legitimate_aim = true; // Aim provided

            // Assess proportionality
            let proportionate = Self::assess_proportionality(engagement, justification);

            let proportionality_analysis = ProportionalityAnalysis {
                legitimate_aim: justification.legitimate_aim.clone(),
                rational_connection: has_legal_basis,
                necessary: !justification.necessity_argument.is_empty(),
                fair_balance: proportionate,
                proportionate,
                reasoning: format!(
                    "Legal basis: {}. Aim: {:?}. {}",
                    if has_legal_basis { "Yes" } else { "No" },
                    justification.legitimate_aim,
                    if proportionate {
                        "Fair balance struck"
                    } else {
                        "Disproportionate interference"
                    }
                ),
            };

            let justified = has_legal_basis && has_legitimate_aim && proportionate;

            (Some(justified), Some(proportionality_analysis))
        } else {
            // No justification offered - interference unjustified
            (Some(false), None)
        }
    }

    fn analyze_limited_right(
        engagement: &ArticleEngagement,
        case_law: &mut Vec<PublicLawCitation>,
    ) -> (Option<bool>, Option<ProportionalityAnalysis>) {
        match engagement.article {
            EchrArticle::Article5 => {
                case_law.push(PublicLawCitation::new(
                    "Secretary of State for the Home Department v JJ",
                    2007,
                    "UKHL 45",
                    "Article 5 deprivation vs restriction of liberty",
                ));
            }
            EchrArticle::Article6 => {
                case_law.push(PublicLawCitation::new(
                    "Brown v Stott",
                    2003,
                    "1 AC 681",
                    "Article 6 rights not absolute - implicit limitations permitted",
                ));
            }
            _ => {}
        }

        // For limited rights, check if within permitted limitations
        if let Some(justification) = &engagement.justification {
            let within_limits = !justification.legal_basis.is_empty();
            (Some(!within_limits), None)
        } else {
            (Some(false), None)
        }
    }

    fn assess_proportionality(
        engagement: &ArticleEngagement,
        justification: &JustificationFacts,
    ) -> bool {
        // Simple proportionality assessment based on severity
        match engagement.interference.severity {
            InterferenceSeverity::Minor => true,
            InterferenceSeverity::Significant => justification.proportionality_factors.len() >= 2,
            InterferenceSeverity::Severe => justification.proportionality_factors.len() >= 3,
            InterferenceSeverity::Total => false, // Total denial rarely proportionate
        }
    }

    fn build_analysis(
        article: &EchrArticle,
        interference: bool,
        justified: Option<bool>,
    ) -> String {
        if !interference {
            return format!("{:?} not engaged - no interference", article);
        }

        match justified {
            Some(true) => format!("{:?} engaged but interference justified", article),
            Some(false) => format!("{:?} violated - unjustified interference", article),
            None => format!("{:?} engaged - further analysis needed", article),
        }
    }
}

// ============================================================================
// Section 6 Analysis
// ============================================================================

/// Analyzer for s.6 public authority status
pub struct Section6Analyzer;

impl Section6Analyzer {
    /// Analyze whether respondent is a public authority
    pub fn analyze(respondent: &RespondentFacts) -> PublicLawResult<Section6Result> {
        let mut case_law = vec![PublicLawCitation::new(
            "YL v Birmingham City Council",
            2007,
            "UKHL 27",
            "Private care homes not s.6 public authorities even when performing publicly funded care",
        )];

        let authority = match &respondent.body_type {
            BodyType::CorePublicAuthority => Section6Authority::Core {
                name: respondent.name.clone(),
            },
            BodyType::HybridBody {
                private_functions_too,
            } => {
                case_law.push(PublicLawCitation::new(
                    "Aston Cantlow v Wallbank",
                    2003,
                    "UKHL 37",
                    "Hybrid bodies only bound when exercising public functions",
                ));
                if *private_functions_too {
                    Section6Authority::Hybrid {
                        name: respondent.name.clone(),
                        public_function: respondent.function.clone(),
                    }
                } else {
                    Section6Authority::Core {
                        name: respondent.name.clone(),
                    }
                }
            }
            BodyType::Private => Section6Authority::NotPublicAuthority,
        };

        let is_public_authority = !matches!(authority, Section6Authority::NotPublicAuthority);

        let analysis = if is_public_authority {
            format!("{} is a public authority under s.6 HRA", respondent.name)
        } else {
            format!(
                "{} is not a public authority - HRA claim not available",
                respondent.name
            )
        };

        Ok(Section6Result {
            authority,
            is_public_authority,
            case_law,
            analysis,
        })
    }
}

/// Result of s.6 analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section6Result {
    /// Authority classification
    pub authority: Section6Authority,
    /// Is public authority?
    pub is_public_authority: bool,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis
    pub analysis: String,
}

// ============================================================================
// Section 3 Analysis
// ============================================================================

/// Facts for s.3 interpretation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section3Facts {
    /// Provision to be interpreted
    pub provision: String,
    /// Natural meaning
    pub natural_meaning: String,
    /// Article engaged
    pub article: EchrArticle,
    /// Incompatibility if read naturally
    pub incompatibility: String,
    /// Possible compatible reading?
    pub possible_reading: Option<String>,
}

/// Analyzer for s.3 interpretation
pub struct Section3Analyzer;

impl Section3Analyzer {
    /// Analyze whether s.3 compatible interpretation possible
    pub fn analyze(facts: &Section3Facts) -> PublicLawResult<Section3Result> {
        let mut case_law = vec![
            PublicLawCitation::new(
                "Ghaidan v Godin-Mendoza",
                2004,
                "2 AC 557",
                "s.3 requires courts to go far beyond normal interpretation to achieve compatibility",
            ),
            PublicLawCitation::new(
                "R v A (No 2)",
                2001,
                "UKHL 25",
                "s.3 can require reading in words or reading down provisions",
            ),
        ];

        let outcome = if let Some(reading) = &facts.possible_reading {
            // Check if reading would be too radical
            let too_radical = Self::is_too_radical(facts, reading);

            if too_radical {
                case_law.push(PublicLawCitation::new(
                    "Bellinger v Bellinger",
                    2003,
                    "UKHL 21",
                    "s.3 cannot be used to make decisions requiring legislative deliberation",
                ));
                Section3Outcome::IncompatibleReading
            } else {
                Section3Outcome::CompatibleReading {
                    interpretation: reading.clone(),
                }
            }
        } else {
            Section3Outcome::IncompatibleReading
        };

        let s3_possible = matches!(outcome, Section3Outcome::CompatibleReading { .. });

        let analysis = if s3_possible {
            format!(
                "s.3 compatible reading possible: {}",
                facts.possible_reading.as_ref().unwrap_or(&String::new())
            )
        } else {
            format!(
                "s.3 interpretation not possible - provision incompatible with {:?}",
                facts.article
            )
        };

        Ok(Section3Result {
            outcome,
            s3_possible,
            case_law,
            analysis,
        })
    }

    fn is_too_radical(facts: &Section3Facts, reading: &str) -> bool {
        // Check if interpretation would fundamentally change scheme
        let natural_len = facts.natural_meaning.len();
        let reading_len = reading.len();

        // Rough heuristic - if reading adds lots of text, may be too radical
        reading_len > natural_len * 3
    }
}

/// Result of s.3 analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section3Result {
    /// Outcome
    pub outcome: Section3Outcome,
    /// s.3 reading possible?
    pub s3_possible: bool,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis
    pub analysis: String,
}

// ============================================================================
// Section 4 Declaration Analysis
// ============================================================================

/// Facts for s.4 declaration analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section4Facts {
    /// Provision incompatible
    pub provision: String,
    /// Article violated
    pub article: EchrArticle,
    /// Nature of incompatibility
    pub incompatibility: String,
    /// s.3 reading attempted?
    pub s3_attempted: bool,
    /// Why s.3 not possible
    pub why_s3_impossible: String,
}

/// Analyzer for s.4 declaration
pub struct Section4Analyzer;

impl Section4Analyzer {
    /// Analyze whether s.4 declaration appropriate
    pub fn analyze(facts: &Section4Facts) -> PublicLawResult<Section4Result> {
        let case_law = vec![PublicLawCitation::new(
            "R (Anderson) v Secretary of State for the Home Department",
            2002,
            "UKHL 46",
            "Declaration of incompatibility - Home Secretary setting tariff incompatible with Art 6",
        )];

        // Declaration appropriate if s.3 not possible
        let declaration_appropriate = facts.s3_attempted && !facts.why_s3_impossible.is_empty();

        let declaration = if declaration_appropriate {
            Some(DeclarationOfIncompatibility {
                provision: facts.provision.clone(),
                article: facts.article.clone(),
                incompatibility: facts.incompatibility.clone(),
            })
        } else {
            None
        };

        let analysis = if declaration_appropriate {
            format!(
                "s.4 declaration appropriate - {} incompatible with {:?}",
                facts.provision, facts.article
            )
        } else if !facts.s3_attempted {
            "s.3 interpretation should be attempted before s.4 declaration".to_string()
        } else {
            "No declaration appropriate".to_string()
        };

        Ok(Section4Result {
            declaration_appropriate,
            declaration,
            case_law,
            analysis,
        })
    }
}

/// Result of s.4 analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section4Result {
    /// Declaration appropriate?
    pub declaration_appropriate: bool,
    /// Declaration if appropriate
    pub declaration: Option<DeclarationOfIncompatibility>,
    /// Key case law
    pub case_law: Vec<PublicLawCitation>,
    /// Analysis
    pub analysis: String,
}

// ============================================================================
// Full HRA Analysis
// ============================================================================

/// Full HRA analyzer
pub struct HraAnalyzer;

impl HraAnalyzer {
    /// Perform full HRA analysis
    pub fn analyze(facts: &HraFacts) -> PublicLawResult<HraAnalysisResult> {
        // Check victim status
        if !facts.claimant.is_victim {
            return Err(HumanRightsError::NoVictimStatus {
                reason: "Claimant not a victim".into(),
            }
            .into());
        }

        // Check time limit (1 year under s.7(5))
        if facts.days_since_act > 365 {
            return Err(HumanRightsError::OutOfHraTime {
                days_late: facts.days_since_act - 365,
            }
            .into());
        }

        // Check public authority status
        let s6_result = Section6Analyzer::analyze(&facts.respondent)?;
        if !s6_result.is_public_authority {
            return Err(HumanRightsError::NotPublicAuthority {
                reason: s6_result.analysis,
            }
            .into());
        }

        // Analyze each article engagement
        let mut articles_engaged = Vec::new();
        let mut interference = false;
        let mut justified = None;
        let mut proportionality = None;
        let mut case_law = s6_result.case_law;

        for engagement in &facts.article_engagement {
            let result = ArticleAnalyzer::analyze(engagement)?;
            if result.engaged {
                articles_engaged.push(engagement.article.clone());
            }
            if result.interference_found {
                interference = true;
            }
            if result.justified.is_some() {
                justified = result.justified;
            }
            if result.proportionality.is_some() {
                proportionality = result.proportionality;
            }
            case_law.extend(result.case_law);
        }

        let analysis = format!(
            "Articles engaged: {:?}. Interference: {}. Justified: {:?}",
            articles_engaged, interference, justified
        );

        Ok(HraAnalysisResult {
            articles_engaged,
            public_authority: s6_result.authority,
            interference,
            justified,
            proportionality,
            section_3: None,
            section_4: None,
            case_law,
            analysis,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section6_core_authority() {
        let facts = RespondentFacts {
            name: "Home Office".into(),
            body_type: BodyType::CorePublicAuthority,
            function: "Immigration control".into(),
        };

        let result = Section6Analyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.is_public_authority);
        assert!(matches!(result.authority, Section6Authority::Core { .. }));
    }

    #[test]
    fn test_section6_private_body() {
        let facts = RespondentFacts {
            name: "Private Company Ltd".into(),
            body_type: BodyType::Private,
            function: "Commercial services".into(),
        };

        let result = Section6Analyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(!result.is_public_authority);
    }

    #[test]
    fn test_article_absolute_right() {
        let engagement = ArticleEngagement {
            article: EchrArticle::Article3,
            interference: InterferenceFacts {
                description: "Inhuman treatment".into(),
                duty_type: HraDuty::Negative,
                severity: InterferenceSeverity::Severe,
            },
            justification: None,
        };

        let result = ArticleAnalyzer::analyze(&engagement).expect("Analysis should succeed");
        assert!(result.interference_found);
        assert_eq!(result.justified, Some(false)); // Article 3 absolute
    }

    #[test]
    fn test_article_qualified_right_justified() {
        let engagement = ArticleEngagement {
            article: EchrArticle::Article8,
            interference: InterferenceFacts {
                description: "Search of home".into(),
                duty_type: HraDuty::Negative,
                severity: InterferenceSeverity::Minor,
            },
            justification: Some(JustificationFacts {
                legal_basis: "Police and Criminal Evidence Act 1984".into(),
                legitimate_aim: LegitimateAim::PreventionOfCrime,
                necessity_argument: "Evidence of serious crime".into(),
                proportionality_factors: vec!["Warrant obtained".into(), "Limited scope".into()],
            }),
        };

        let result = ArticleAnalyzer::analyze(&engagement).expect("Analysis should succeed");
        assert!(result.interference_found);
        assert_eq!(result.justified, Some(true));
    }

    #[test]
    fn test_section3_compatible_reading() {
        let facts = Section3Facts {
            provision: "spouse".into(),
            natural_meaning: "married partner".into(),
            article: EchrArticle::Article8,
            incompatibility: "Excludes same-sex partners".into(),
            possible_reading: Some("spouse or civil partner".into()),
        };

        let result = Section3Analyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.s3_possible);
    }

    #[test]
    fn test_section4_declaration() {
        let facts = Section4Facts {
            provision: "s.82 Crime (Sentences) Act 1997".into(),
            article: EchrArticle::Article6,
            incompatibility: "Allows Home Secretary to set tariff".into(),
            s3_attempted: true,
            why_s3_impossible: "Would require rewriting statute".into(),
        };

        let result = Section4Analyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.declaration_appropriate);
        assert!(result.declaration.is_some());
    }

    #[test]
    fn test_full_hra_analysis() {
        let facts = HraFacts {
            claimant: ClaimantFacts {
                is_victim: true,
                how_affected: "Decision affects claimant directly".into(),
                victim_type: VictimType::Direct,
            },
            respondent: RespondentFacts {
                name: "Secretary of State".into(),
                body_type: BodyType::CorePublicAuthority,
                function: "Immigration decision".into(),
            },
            article_engagement: vec![ArticleEngagement {
                article: EchrArticle::Article8,
                interference: InterferenceFacts {
                    description: "Removal from UK".into(),
                    duty_type: HraDuty::Negative,
                    severity: InterferenceSeverity::Severe,
                },
                justification: None,
            }],
            days_since_act: 30,
        };

        let result = HraAnalyzer::analyze(&facts).expect("Analysis should succeed");
        assert!(result.interference);
        assert!(result.articles_engaged.contains(&EchrArticle::Article8));
    }
}
