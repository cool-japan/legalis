//! UK Constitutional Law Module
//!
//! This module provides analysis of UK constitutional principles including:
//! - Parliamentary sovereignty
//! - Rule of law
//! - Separation of powers
//! - Royal prerogative
//!
//! Key cases:
//! - R (Miller) v Secretary of State [2017] UKSC 5 (prorogation)
//! - R (Miller) v The Prime Minister [2019] UKSC 41 (Miller II)
//! - Entick v Carrington (1765) 19 St Tr 1029 (rule of law)
//! - Factortame (No 2) [1991] 1 AC 603 (EU law supremacy)
//! - Jackson v Attorney General [2005] UKHL 56 (manner and form)

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::error::PublicLawResult;
use super::types::{ConstitutionalPrinciple, PrerogativePower, RuleOfLawPrinciple};

// ============================================================================
// Key Constitutional Cases
// ============================================================================

/// Key constitutional case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCase {
    pub name: String,
    pub citation: String,
    pub year: u16,
    pub principle: String,
}

impl ConstitutionalCase {
    /// Dicey's rule of law
    pub fn dicey_rule_of_law() -> Self {
        Self {
            name: "Dicey's Introduction to the Study of the Law of the Constitution".into(),
            citation: "(1885)".into(),
            year: 1885,
            principle: "Three meanings of rule of law: supremacy of regular law, \
                equality before the law, constitutional law as consequence of individual rights"
                .into(),
        }
    }

    /// Entick v Carrington - foundational rule of law case
    pub fn entick_v_carrington() -> Self {
        Self {
            name: "Entick v Carrington".into(),
            citation: "(1765) 19 St Tr 1029".into(),
            year: 1765,
            principle: "State officials must show legal authority for their actions; \
                executive cannot claim inherent powers beyond the law"
                .into(),
        }
    }

    /// A v Secretary of State - Belmarsh detainees
    pub fn a_v_secretary_of_state() -> Self {
        Self {
            name: "A v Secretary of State for the Home Department".into(),
            citation: "[2004] UKHL 56".into(),
            year: 2004,
            principle: "Indefinite detention without trial incompatible with rule of law; \
                courts have constitutional role to protect fundamental rights"
                .into(),
        }
    }

    /// Miller I - Article 50 notification
    pub fn miller_i() -> Self {
        Self {
            name: "R (Miller) v Secretary of State for Exiting the European Union".into(),
            citation: "[2017] UKSC 5".into(),
            year: 2017,
            principle: "Government cannot use prerogative to nullify statutory rights; \
                Parliament must authorise triggering Article 50"
                .into(),
        }
    }

    /// Miller II - Prorogation case
    pub fn miller_ii() -> Self {
        Self {
            name: "R (Miller) v The Prime Minister".into(),
            citation: "[2019] UKSC 41".into(),
            year: 2019,
            principle: "Prerogative power to prorogue is justiciable and limited by \
                constitutional principles; prorogation preventing Parliament from \
                functioning is unlawful"
                .into(),
        }
    }

    /// GCHQ case - judicial review of prerogative
    pub fn gchq() -> Self {
        Self {
            name: "Council of Civil Service Unions v Minister for the Civil Service".into(),
            citation: "[1985] AC 374".into(),
            year: 1985,
            principle: "Prerogative powers are in principle subject to judicial review \
                unless excluded by subject matter (national security, foreign affairs)"
                .into(),
        }
    }

    /// Factortame No 2 - EU law supremacy
    pub fn factortame() -> Self {
        Self {
            name: "R v Secretary of State for Transport, ex parte Factortame (No 2)".into(),
            citation: "[1991] 1 AC 603".into(),
            year: 1991,
            principle: "Courts may disapply Acts of Parliament that conflict with EU law; \
                parliamentary sovereignty qualified during EU membership"
                .into(),
        }
    }

    /// Jackson v Attorney General - manner and form
    pub fn jackson() -> Self {
        Self {
            name: "Jackson v Attorney General".into(),
            citation: "[2005] UKHL 56".into(),
            year: 2005,
            principle: "Parliament may bind itself as to manner and form of legislation; \
                obiter on constitutional statutes and possible limits to sovereignty"
                .into(),
        }
    }

    /// Thoburn v Sunderland - constitutional statutes
    pub fn thoburn() -> Self {
        Self {
            name: "Thoburn v Sunderland City Council".into(),
            citation: "[2002] EWHC 195 (Admin)".into(),
            year: 2002,
            principle: "Constitutional statutes cannot be impliedly repealed; \
                require express words or necessary implication to amend"
                .into(),
        }
    }

    /// R (Evans) v Attorney General - override of FTT decision
    pub fn evans() -> Self {
        Self {
            name: "R (Evans) v Attorney General".into(),
            citation: "[2015] UKSC 21".into(),
            year: 2015,
            principle: "Executive cannot override judicial or tribunal decisions; \
                rule of law requires finality of court orders"
                .into(),
        }
    }

    /// Unison - access to justice
    pub fn unison() -> Self {
        Self {
            name: "R (UNISON) v Lord Chancellor".into(),
            citation: "[2017] UKSC 51".into(),
            year: 2017,
            principle: "Access to justice is a constitutional right; fees that \
                effectively prevent access to tribunals are unlawful"
                .into(),
        }
    }
}

// ============================================================================
// Parliamentary Sovereignty Analyzer
// ============================================================================

/// Result of parliamentary sovereignty analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyAnalysis {
    pub is_sovereign_act: bool,
    pub potential_limitations: Vec<SovereigntyLimitation>,
    pub constitutional_statute: bool,
    pub reasoning: String,
    pub key_cases: Vec<ConstitutionalCase>,
}

/// Potential limitation on parliamentary sovereignty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SovereigntyLimitation {
    /// EU/retained EU law (historical, pre-Brexit)
    EuLaw { retained: bool },
    /// Human Rights Act interpretation
    HumanRightsAct,
    /// Constitutional statute doctrine
    ConstitutionalStatute,
    /// Manner and form argument
    MannerAndForm,
    /// Devolution settlements
    DevolutionSettlement { nation: String },
    /// International treaty obligations
    InternationalTreaty { treaty: String },
}

/// Analyzes parliamentary sovereignty issues
pub struct SovereigntyAnalyzer;

impl SovereigntyAnalyzer {
    /// Check if Act is a constitutional statute
    pub fn is_constitutional_statute(act_name: &str) -> bool {
        let constitutional_statutes = [
            "Magna Carta",
            "Bill of Rights 1689",
            "Act of Settlement 1701",
            "Acts of Union",
            "Parliament Acts 1911",
            "Parliament Act 1949",
            "European Communities Act 1972",
            "Scotland Act 1998",
            "Government of Wales Act 1998",
            "Northern Ireland Act 1998",
            "Human Rights Act 1998",
            "Constitutional Reform Act 2005",
            "Fixed-term Parliaments Act",
            "European Union (Withdrawal) Act",
        ];

        let act_lower = act_name.to_lowercase();
        constitutional_statutes
            .iter()
            .any(|s| act_lower.contains(&s.to_lowercase()))
    }

    /// Analyze sovereignty implications
    pub fn analyze(
        legislation: &str,
        affects_eu_law: bool,
        affects_devolution: bool,
        affects_human_rights: bool,
    ) -> SovereigntyAnalysis {
        let mut limitations = Vec::new();
        let mut key_cases = Vec::new();

        // Check constitutional statute status
        let constitutional_statute = Self::is_constitutional_statute(legislation);
        if constitutional_statute {
            key_cases.push(ConstitutionalCase::thoburn());
        }

        // EU law considerations (historical but still relevant for retained EU law)
        if affects_eu_law {
            limitations.push(SovereigntyLimitation::EuLaw { retained: true });
            key_cases.push(ConstitutionalCase::factortame());
        }

        // Devolution considerations
        if affects_devolution {
            limitations.push(SovereigntyLimitation::DevolutionSettlement {
                nation: "Scotland/Wales/Northern Ireland".into(),
            });
        }

        // Human rights considerations
        if affects_human_rights {
            limitations.push(SovereigntyLimitation::HumanRightsAct);
        }

        let reasoning = if limitations.is_empty() {
            format!(
                "Parliament has unlimited legislative power. '{}' represents \
                 a valid exercise of parliamentary sovereignty with no identified \
                 limitations. Courts cannot strike down primary legislation.",
                legislation
            )
        } else {
            format!(
                "While Parliament remains sovereign, '{}' engages {} potential \
                 limitation(s) that may affect interpretation or application: {}",
                legislation,
                limitations.len(),
                Self::format_limitations(&limitations)
            )
        };

        SovereigntyAnalysis {
            is_sovereign_act: true, // UK has no formal unconstitutionality
            potential_limitations: limitations,
            constitutional_statute,
            reasoning,
            key_cases,
        }
    }

    fn format_limitations(limitations: &[SovereigntyLimitation]) -> String {
        limitations
            .iter()
            .map(|l| match l {
                SovereigntyLimitation::EuLaw { retained } => {
                    if *retained {
                        "retained EU law".to_string()
                    } else {
                        "EU law supremacy".to_string()
                    }
                }
                SovereigntyLimitation::HumanRightsAct => "HRA 1998 interpretation duty".into(),
                SovereigntyLimitation::ConstitutionalStatute => {
                    "constitutional statute doctrine".into()
                }
                SovereigntyLimitation::MannerAndForm => "manner and form requirements".into(),
                SovereigntyLimitation::DevolutionSettlement { nation } => {
                    format!("devolution settlement ({})", nation)
                }
                SovereigntyLimitation::InternationalTreaty { treaty } => {
                    format!("international obligations ({})", treaty)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

// ============================================================================
// Rule of Law Analyzer
// ============================================================================

/// Result of rule of law analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOfLawAnalysis {
    pub principles_engaged: Vec<RuleOfLawPrinciple>,
    pub potential_violations: Vec<RuleOfLawViolation>,
    pub overall_assessment: RuleOfLawAssessment,
    pub reasoning: String,
    pub key_cases: Vec<ConstitutionalCase>,
}

/// Potential rule of law violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOfLawViolation {
    pub principle: RuleOfLawPrinciple,
    pub description: String,
    pub severity: ViolationSeverity,
}

/// Severity of rule of law violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Formal technical issue
    Technical,
    /// Significant concern
    Significant,
    /// Fundamental breach
    Fundamental,
}

/// Overall rule of law assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleOfLawAssessment {
    /// Fully compliant
    Compliant,
    /// Some concerns but generally acceptable
    Concerns,
    /// Serious rule of law issues
    Problematic,
    /// Fundamental breach
    Breach,
}

/// Analyzes rule of law compliance
pub struct RuleOfLawAnalyzer;

impl RuleOfLawAnalyzer {
    /// Analyze action for rule of law compliance
    pub fn analyze(
        action_description: &str,
        has_legal_basis: bool,
        is_prospective: bool,
        is_accessible: bool,
        allows_court_access: bool,
        treats_equally: bool,
    ) -> RuleOfLawAnalysis {
        let mut principles_engaged = Vec::new();
        let mut violations = Vec::new();
        let mut key_cases = Vec::new();

        // Always cite Dicey as foundational
        key_cases.push(ConstitutionalCase::dicey_rule_of_law());

        // Principle 1: Legality - no punishment without law
        principles_engaged.push(RuleOfLawPrinciple::Legality);
        if !has_legal_basis {
            violations.push(RuleOfLawViolation {
                principle: RuleOfLawPrinciple::Legality,
                description: "Action lacks clear legal authority".into(),
                severity: ViolationSeverity::Fundamental,
            });
            key_cases.push(ConstitutionalCase::entick_v_carrington());
        }

        // Principle 2: Legal certainty - prospective, clear, accessible law
        principles_engaged.push(RuleOfLawPrinciple::LegalCertainty);
        if !is_prospective {
            violations.push(RuleOfLawViolation {
                principle: RuleOfLawPrinciple::LegalCertainty,
                description: "Retroactive application of law".into(),
                severity: ViolationSeverity::Significant,
            });
        }
        if !is_accessible {
            violations.push(RuleOfLawViolation {
                principle: RuleOfLawPrinciple::LegalCertainty,
                description: "Law not sufficiently accessible or clear".into(),
                severity: ViolationSeverity::Technical,
            });
        }

        // Principle 3: Equality before the law
        principles_engaged.push(RuleOfLawPrinciple::Equality);
        if !treats_equally {
            violations.push(RuleOfLawViolation {
                principle: RuleOfLawPrinciple::Equality,
                description: "Unequal treatment without justification".into(),
                severity: ViolationSeverity::Significant,
            });
        }

        // Principle 4: Access to justice
        principles_engaged.push(RuleOfLawPrinciple::AccessToJustice);
        if !allows_court_access {
            violations.push(RuleOfLawViolation {
                principle: RuleOfLawPrinciple::AccessToJustice,
                description: "Restriction on access to courts or tribunals".into(),
                severity: ViolationSeverity::Fundamental,
            });
            key_cases.push(ConstitutionalCase::unison());
        }

        // Determine overall assessment
        let overall_assessment = Self::determine_assessment(&violations);

        let reasoning = Self::build_reasoning(action_description, &violations, overall_assessment);

        RuleOfLawAnalysis {
            principles_engaged,
            potential_violations: violations,
            overall_assessment,
            reasoning,
            key_cases,
        }
    }

    fn determine_assessment(violations: &[RuleOfLawViolation]) -> RuleOfLawAssessment {
        let has_fundamental = violations
            .iter()
            .any(|v| v.severity == ViolationSeverity::Fundamental);
        let has_significant = violations
            .iter()
            .any(|v| v.severity == ViolationSeverity::Significant);
        let has_technical = violations
            .iter()
            .any(|v| v.severity == ViolationSeverity::Technical);

        if has_fundamental {
            RuleOfLawAssessment::Breach
        } else if has_significant {
            RuleOfLawAssessment::Problematic
        } else if has_technical {
            RuleOfLawAssessment::Concerns
        } else {
            RuleOfLawAssessment::Compliant
        }
    }

    fn build_reasoning(
        action: &str,
        violations: &[RuleOfLawViolation],
        assessment: RuleOfLawAssessment,
    ) -> String {
        match assessment {
            RuleOfLawAssessment::Compliant => {
                format!(
                    "The action '{}' appears consistent with rule of law principles. \
                     It has a clear legal basis, is prospective and accessible, \
                     preserves access to courts, and applies equally.",
                    action
                )
            }
            RuleOfLawAssessment::Concerns => {
                format!(
                    "The action '{}' raises minor rule of law concerns: {}. \
                     These are technical issues that do not fundamentally \
                     undermine legality.",
                    action,
                    Self::format_violations(violations)
                )
            }
            RuleOfLawAssessment::Problematic => {
                format!(
                    "The action '{}' raises significant rule of law concerns: {}. \
                     While not a fundamental breach, these issues require \
                     careful consideration and potential remediation.",
                    action,
                    Self::format_violations(violations)
                )
            }
            RuleOfLawAssessment::Breach => {
                format!(
                    "The action '{}' appears to breach fundamental rule of law \
                     principles: {}. This represents a serious constitutional \
                     concern that may render the action unlawful.",
                    action,
                    Self::format_violations(violations)
                )
            }
        }
    }

    fn format_violations(violations: &[RuleOfLawViolation]) -> String {
        violations
            .iter()
            .map(|v| v.description.clone())
            .collect::<Vec<_>>()
            .join("; ")
    }
}

// ============================================================================
// Prerogative Power Analyzer
// ============================================================================

/// Result of prerogative power analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrerogativeAnalysis {
    pub power: PrerogativePower,
    pub is_reviewable: bool,
    pub review_limitations: Vec<String>,
    pub has_been_abrogated: bool,
    pub abrogating_statute: Option<String>,
    pub reasoning: String,
    pub key_cases: Vec<ConstitutionalCase>,
}

/// Analyzes royal prerogative powers
pub struct PrerogativeAnalyzer;

impl PrerogativeAnalyzer {
    /// Analyze whether prerogative power is reviewable
    pub fn analyze(power: PrerogativePower) -> PrerogativeAnalysis {
        let mut key_cases = vec![ConstitutionalCase::gchq()];
        let mut review_limitations = Vec::new();

        // Determine reviewability based on GCHQ categories
        let (is_reviewable, base_reasoning): (bool, String) = match &power {
            // High policy - traditionally non-reviewable
            PrerogativePower::ForeignAffairs => {
                review_limitations.push("Matters of high foreign policy".into());
                review_limitations.push("Relations with foreign states".into());
                (
                    false,
                    "Foreign affairs prerogative traditionally non-reviewable as high policy"
                        .into(),
                )
            }
            PrerogativePower::DefenceAndArmedForces => {
                review_limitations.push("Operational military decisions".into());
                review_limitations.push("National security considerations".into());
                (
                    false,
                    "Defence prerogative involves operational and national security matters".into(),
                )
            }
            PrerogativePower::TreatyMaking => {
                review_limitations.push("Conduct of foreign relations".into());
                key_cases.push(ConstitutionalCase::miller_i());
                (
                    false,
                    "Treaty-making generally non-reviewable, but see Miller I on domestic effect"
                        .into(),
                )
            }
            PrerogativePower::NationalSecurity => {
                review_limitations.push("National security assessment".into());
                review_limitations.push("Intelligence matters".into());
                (
                    false,
                    "National security matters require judicial deference per GCHQ".into(),
                )
            }

            // Reviewable prerogatives
            PrerogativePower::AppointmentAndDismissal => (
                true,
                "Civil service matters reviewable on ordinary grounds".into(),
            ),
            PrerogativePower::MercyAndPardon => (
                true,
                "Mercy prerogative reviewable for procedural fairness and rationality".into(),
            ),
            PrerogativePower::HonoursAndTitles => (
                true,
                "Honours system subject to review for procedural propriety".into(),
            ),
            PrerogativePower::RoyalAssent => (
                false,
                "Royal assent is constitutional convention, not justiciable".into(),
            ),
            PrerogativePower::SummoningParliament => {
                key_cases.push(ConstitutionalCase::miller_ii());
                (
                    true,
                    "Miller II established prorogation is justiciable".into(),
                )
            }
            PrerogativePower::PassportIssuance => (
                true,
                "Passport decisions reviewable on normal JR grounds".into(),
            ),
            PrerogativePower::CrimeAndJustice => (
                true,
                "Exercise of prosecution discretion may be reviewable".into(),
            ),
            PrerogativePower::Other(desc) => (
                true,
                format!(
                    "Other prerogative powers ({}) presumptively reviewable post-GCHQ",
                    desc
                ),
            ),
        };

        // Check for statutory abrogation
        let (has_been_abrogated, abrogating_statute) = Self::check_abrogation(&power);

        let reasoning = if has_been_abrogated {
            format!(
                "The {} prerogative has been abrogated or limited by {}. \
                 Where statute covers the field, prerogative cannot be exercised \
                 (Attorney General v De Keyser's Royal Hotel [1920]).",
                Self::power_name(&power),
                abrogating_statute.as_deref().unwrap_or("statute")
            )
        } else if is_reviewable {
            format!(
                "The {} prerogative is reviewable on ordinary judicial review grounds. \
                 {}. Following GCHQ, the source of power (prerogative vs statutory) \
                 does not determine reviewability; the nature of the power does.",
                Self::power_name(&power),
                base_reasoning
            )
        } else {
            format!(
                "The {} prerogative falls within the category of non-reviewable \
                 matters identified in GCHQ. {}. However, this is not absolute; \
                 Miller II shows constitutional principles may still apply.",
                Self::power_name(&power),
                base_reasoning
            )
        };

        PrerogativeAnalysis {
            power,
            is_reviewable,
            review_limitations,
            has_been_abrogated,
            abrogating_statute,
            reasoning,
            key_cases,
        }
    }

    fn power_name(power: &PrerogativePower) -> &'static str {
        match power {
            PrerogativePower::ForeignAffairs => "foreign affairs",
            PrerogativePower::DefenceAndArmedForces => "defence and armed forces",
            PrerogativePower::TreatyMaking => "treaty-making",
            PrerogativePower::AppointmentAndDismissal => "appointment and dismissal",
            PrerogativePower::MercyAndPardon => "mercy and pardon",
            PrerogativePower::HonoursAndTitles => "honours and titles",
            PrerogativePower::NationalSecurity => "national security",
            PrerogativePower::RoyalAssent => "royal assent",
            PrerogativePower::SummoningParliament => "summoning/proroguing Parliament",
            PrerogativePower::PassportIssuance => "passport issuance",
            PrerogativePower::CrimeAndJustice => "crime and justice",
            PrerogativePower::Other(_) => "other prerogative",
        }
    }

    fn check_abrogation(power: &PrerogativePower) -> (bool, Option<String>) {
        match power {
            // Fixed-term Parliaments Act abrogated dissolution prerogative (now repealed)
            PrerogativePower::SummoningParliament => {
                // FTPA repealed by Dissolution and Calling of Parliament Act 2022
                (false, None)
            }
            // Immigration prerogative largely covered by statute
            PrerogativePower::PassportIssuance => {
                // Still prerogative but immigration generally statutory
                (false, None)
            }
            // Criminal justice largely statutory
            PrerogativePower::CrimeAndJustice => {
                (false, Some("Prosecution of Offences Act 1985 (CPS)".into()))
            }
            _ => (false, None),
        }
    }
}

// ============================================================================
// Separation of Powers Analyzer
// ============================================================================

/// Result of separation of powers analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparationAnalysis {
    pub branches_engaged: Vec<ConstitutionalBranch>,
    pub potential_conflicts: Vec<SeparationConflict>,
    pub is_constitutionally_proper: bool,
    pub reasoning: String,
}

/// Constitutional branch of government
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalBranch {
    /// Legislature (Parliament)
    Legislature,
    /// Executive (Government/Crown)
    Executive,
    /// Judiciary (Courts)
    Judiciary,
}

/// Potential separation of powers conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparationConflict {
    pub encroaching_branch: ConstitutionalBranch,
    pub encroached_branch: ConstitutionalBranch,
    pub description: String,
}

/// Analyzes separation of powers issues
pub struct SeparationAnalyzer;

impl SeparationAnalyzer {
    /// Analyze separation of powers compliance
    pub fn analyze(
        actor_branch: ConstitutionalBranch,
        action_description: &str,
        affects_legislative: bool,
        affects_executive: bool,
        affects_judicial: bool,
    ) -> SeparationAnalysis {
        let mut branches_engaged = vec![actor_branch];
        let mut conflicts = Vec::new();

        // Determine which branches are affected
        if affects_legislative && actor_branch != ConstitutionalBranch::Legislature {
            branches_engaged.push(ConstitutionalBranch::Legislature);
            // Executive legislating via orders
            if actor_branch == ConstitutionalBranch::Executive {
                conflicts.push(SeparationConflict {
                    encroaching_branch: ConstitutionalBranch::Executive,
                    encroached_branch: ConstitutionalBranch::Legislature,
                    description: "Executive action may usurp legislative function".into(),
                });
            }
        }

        if affects_executive && actor_branch != ConstitutionalBranch::Executive {
            branches_engaged.push(ConstitutionalBranch::Executive);
            // Courts directing executive
            if actor_branch == ConstitutionalBranch::Judiciary {
                // This is generally acceptable via JR
            }
        }

        if affects_judicial && actor_branch != ConstitutionalBranch::Judiciary {
            branches_engaged.push(ConstitutionalBranch::Judiciary);
            // Executive overriding courts
            if actor_branch == ConstitutionalBranch::Executive {
                conflicts.push(SeparationConflict {
                    encroaching_branch: ConstitutionalBranch::Executive,
                    encroached_branch: ConstitutionalBranch::Judiciary,
                    description: "Executive action may interfere with judicial independence".into(),
                });
            }
            // Legislature overriding specific judgments
            if actor_branch == ConstitutionalBranch::Legislature {
                conflicts.push(SeparationConflict {
                    encroaching_branch: ConstitutionalBranch::Legislature,
                    encroached_branch: ConstitutionalBranch::Judiciary,
                    description: "Legislative override of specific judicial decision".into(),
                });
            }
        }

        let is_constitutionally_proper =
            conflicts.is_empty() || conflicts.iter().all(Self::is_acceptable_overlap);

        let reasoning = if is_constitutionally_proper {
            format!(
                "The action '{}' by the {} is constitutionally proper. \
                 While the UK does not have strict separation of powers \
                 (unlike the US), the action respects institutional boundaries.",
                action_description,
                Self::branch_name(actor_branch)
            )
        } else {
            format!(
                "The action '{}' raises separation of powers concerns: {}. \
                 While UK constitution permits functional overlap between branches, \
                 core constitutional functions must be preserved.",
                action_description,
                Self::format_conflicts(&conflicts)
            )
        };

        SeparationAnalysis {
            branches_engaged,
            potential_conflicts: conflicts,
            is_constitutionally_proper,
            reasoning,
        }
    }

    fn branch_name(branch: ConstitutionalBranch) -> &'static str {
        match branch {
            ConstitutionalBranch::Legislature => "Legislature",
            ConstitutionalBranch::Executive => "Executive",
            ConstitutionalBranch::Judiciary => "Judiciary",
        }
    }

    fn is_acceptable_overlap(conflict: &SeparationConflict) -> bool {
        // Some overlaps are acceptable in UK constitution
        // e.g., judicial review of executive, delegated legislation
        matches!(
            (conflict.encroaching_branch, conflict.encroached_branch),
            (
                ConstitutionalBranch::Judiciary,
                ConstitutionalBranch::Executive
            ) | (
                ConstitutionalBranch::Legislature,
                ConstitutionalBranch::Executive
            )
        )
    }

    fn format_conflicts(conflicts: &[SeparationConflict]) -> String {
        conflicts
            .iter()
            .map(|c| c.description.clone())
            .collect::<Vec<_>>()
            .join("; ")
    }
}

// ============================================================================
// Constitutional Principles Analyzer
// ============================================================================

/// Full constitutional analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalAnalysis {
    pub principles_engaged: Vec<ConstitutionalPrinciple>,
    pub sovereignty_analysis: Option<SovereigntyAnalysis>,
    pub rule_of_law_analysis: Option<RuleOfLawAnalysis>,
    pub prerogative_analysis: Option<PrerogativeAnalysis>,
    pub separation_analysis: Option<SeparationAnalysis>,
    pub overall_assessment: ConstitutionalAssessment,
    pub recommendations: Vec<String>,
}

/// Overall constitutional assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalAssessment {
    /// No constitutional issues
    NoIssues,
    /// Minor concerns that do not affect validity
    MinorConcerns,
    /// Significant issues requiring attention
    SignificantIssues,
    /// Potentially unconstitutional
    PotentiallyUnconstitutional,
}

/// Comprehensive constitutional analyzer
pub struct ConstitutionalAnalyzer;

impl ConstitutionalAnalyzer {
    /// Perform comprehensive constitutional analysis
    pub fn analyze(
        matter_description: &str,
        involves_prerogative: Option<PrerogativePower>,
        affects_legislation: Option<&str>,
        actor: ConstitutionalBranch,
        rule_of_law_factors: Option<RuleOfLawFactors>,
    ) -> PublicLawResult<ConstitutionalAnalysis> {
        let mut principles_engaged = Vec::new();
        let mut recommendations = Vec::new();

        // Prerogative analysis if applicable
        let prerogative_analysis = involves_prerogative.map(|power| {
            principles_engaged.push(ConstitutionalPrinciple::RoyalPrerogative);
            PrerogativeAnalyzer::analyze(power)
        });

        // Sovereignty analysis if legislation involved
        let sovereignty_analysis = affects_legislation.map(|leg| {
            principles_engaged.push(ConstitutionalPrinciple::ParliamentarySovereignty);
            SovereigntyAnalyzer::analyze(leg, false, false, false)
        });

        // Rule of law analysis
        let rule_of_law_analysis = rule_of_law_factors.map(|factors| {
            principles_engaged.push(ConstitutionalPrinciple::RuleOfLaw);
            RuleOfLawAnalyzer::analyze(
                matter_description,
                factors.has_legal_basis,
                factors.is_prospective,
                factors.is_accessible,
                factors.allows_court_access,
                factors.treats_equally,
            )
        });

        // Separation of powers analysis
        let separation_analysis = Some(SeparationAnalyzer::analyze(
            actor,
            matter_description,
            affects_legislation.is_some(),
            true, // Assume affects executive
            false,
        ));
        principles_engaged.push(ConstitutionalPrinciple::SeparationOfPowers);

        // Determine overall assessment
        let overall_assessment = Self::determine_assessment(
            &prerogative_analysis,
            &rule_of_law_analysis,
            &separation_analysis,
        );

        // Generate recommendations
        if let Some(ref rol) = rule_of_law_analysis {
            if rol.overall_assessment != RuleOfLawAssessment::Compliant {
                recommendations.push("Review legal basis for action".into());
                recommendations.push("Ensure access to courts is preserved".into());
            }
        }

        if let Some(ref sep) = separation_analysis {
            if !sep.is_constitutionally_proper {
                recommendations.push("Consider institutional boundaries".into());
            }
        }

        Ok(ConstitutionalAnalysis {
            principles_engaged,
            sovereignty_analysis,
            rule_of_law_analysis,
            prerogative_analysis,
            separation_analysis,
            overall_assessment,
            recommendations,
        })
    }

    fn determine_assessment(
        prerogative: &Option<PrerogativeAnalysis>,
        rule_of_law: &Option<RuleOfLawAnalysis>,
        separation: &Option<SeparationAnalysis>,
    ) -> ConstitutionalAssessment {
        // Check rule of law first (most serious)
        if let Some(rol) = rule_of_law {
            match rol.overall_assessment {
                RuleOfLawAssessment::Breach => {
                    return ConstitutionalAssessment::PotentiallyUnconstitutional;
                }
                RuleOfLawAssessment::Problematic => {
                    return ConstitutionalAssessment::SignificantIssues;
                }
                RuleOfLawAssessment::Concerns => return ConstitutionalAssessment::MinorConcerns,
                RuleOfLawAssessment::Compliant => {}
            }
        }

        // Check separation of powers
        if let Some(sep) = separation {
            if !sep.is_constitutionally_proper {
                return ConstitutionalAssessment::SignificantIssues;
            }
        }

        // Check prerogative
        if let Some(pre) = prerogative {
            if pre.has_been_abrogated {
                return ConstitutionalAssessment::SignificantIssues;
            }
        }

        ConstitutionalAssessment::NoIssues
    }
}

/// Factors for rule of law analysis
#[derive(Debug, Clone)]
pub struct RuleOfLawFactors {
    pub has_legal_basis: bool,
    pub is_prospective: bool,
    pub is_accessible: bool,
    pub allows_court_access: bool,
    pub treats_equally: bool,
}

impl Default for RuleOfLawFactors {
    fn default() -> Self {
        Self {
            has_legal_basis: true,
            is_prospective: true,
            is_accessible: true,
            allows_court_access: true,
            treats_equally: true,
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
    fn test_constitutional_statutes() {
        assert!(SovereigntyAnalyzer::is_constitutional_statute(
            "Human Rights Act 1998"
        ));
        assert!(SovereigntyAnalyzer::is_constitutional_statute(
            "Scotland Act 1998"
        ));
        assert!(!SovereigntyAnalyzer::is_constitutional_statute(
            "Consumer Rights Act 2015"
        ));
    }

    #[test]
    fn test_sovereignty_analysis() {
        let analysis = SovereigntyAnalyzer::analyze("Human Rights Act 1998", false, false, true);
        assert!(analysis.constitutional_statute);
        assert!(!analysis.potential_limitations.is_empty());
    }

    #[test]
    fn test_rule_of_law_compliant() {
        let analysis = RuleOfLawAnalyzer::analyze(
            "Issue parking fine",
            true, // has legal basis
            true, // prospective
            true, // accessible
            true, // court access
            true, // equal treatment
        );
        assert_eq!(analysis.overall_assessment, RuleOfLawAssessment::Compliant);
    }

    #[test]
    fn test_rule_of_law_breach() {
        let analysis = RuleOfLawAnalyzer::analyze(
            "Detention without legal authority",
            false, // no legal basis
            true,
            true,
            false, // no court access
            true,
        );
        assert_eq!(analysis.overall_assessment, RuleOfLawAssessment::Breach);
    }

    #[test]
    fn test_prerogative_reviewable() {
        let analysis = PrerogativeAnalyzer::analyze(PrerogativePower::MercyAndPardon);
        assert!(analysis.is_reviewable);
    }

    #[test]
    fn test_prerogative_non_reviewable() {
        let analysis = PrerogativeAnalyzer::analyze(PrerogativePower::NationalSecurity);
        assert!(!analysis.is_reviewable);
    }

    #[test]
    fn test_prerogative_prorogation() {
        let analysis = PrerogativeAnalyzer::analyze(PrerogativePower::SummoningParliament);
        // Miller II established this is reviewable
        assert!(analysis.is_reviewable);
        assert!(analysis.key_cases.iter().any(|c| c.name.contains("Miller")));
    }

    #[test]
    fn test_separation_proper() {
        let analysis = SeparationAnalyzer::analyze(
            ConstitutionalBranch::Executive,
            "Issue policy guidance",
            false,
            true,
            false,
        );
        assert!(analysis.is_constitutionally_proper);
    }

    #[test]
    fn test_separation_conflict() {
        let analysis = SeparationAnalyzer::analyze(
            ConstitutionalBranch::Executive,
            "Override court judgment",
            false,
            false,
            true, // affects judicial
        );
        assert!(!analysis.is_constitutionally_proper);
        assert!(!analysis.potential_conflicts.is_empty());
    }

    #[test]
    fn test_constitutional_analysis() {
        let result = ConstitutionalAnalyzer::analyze(
            "Executive detention policy",
            Some(PrerogativePower::NationalSecurity),
            None,
            ConstitutionalBranch::Executive,
            Some(RuleOfLawFactors {
                has_legal_basis: true,
                is_prospective: true,
                is_accessible: true,
                allows_court_access: true,
                treats_equally: true,
            }),
        );
        assert!(result.is_ok());
        let analysis = result.expect("analysis should succeed");
        assert!(!analysis.principles_engaged.is_empty());
    }

    #[test]
    fn test_constitutional_cases() {
        let miller_i = ConstitutionalCase::miller_i();
        assert!(miller_i.principle.contains("Article 50"));
        assert_eq!(miller_i.year, 2017);

        let miller_ii = ConstitutionalCase::miller_ii();
        assert!(miller_ii.principle.contains("prorogation"));
        assert_eq!(miller_ii.year, 2019);
    }

    #[test]
    fn test_entick_v_carrington() {
        let entick = ConstitutionalCase::entick_v_carrington();
        assert!(entick.principle.contains("legal authority"));
        assert_eq!(entick.year, 1765);
    }

    #[test]
    fn test_violation_severity() {
        // Test that fundamental violations are correctly identified
        let violations = vec![RuleOfLawViolation {
            principle: RuleOfLawPrinciple::Legality,
            description: "No legal basis".into(),
            severity: ViolationSeverity::Fundamental,
        }];

        let assessment = RuleOfLawAnalyzer::determine_assessment(&violations);
        assert_eq!(assessment, RuleOfLawAssessment::Breach);
    }
}
