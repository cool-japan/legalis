//! Contract Breach and Termination
//!
//! This module implements breach of contract law under English common law.
//!
//! ## Types of Breach
//!
//! ### Actual Breach
//! Breach occurs when performance is due:
//! - Non-performance
//! - Defective performance
//! - Incomplete performance
//!
//! ### Anticipatory Breach (Hochster v De La Tour [1853])
//! Breach occurs before performance due when party:
//! - Expressly renounces obligations
//! - By conduct renders performance impossible
//!
//! ## Repudiatory Breach
//!
//! A breach that goes to the root of the contract, entitling innocent party to terminate.
//!
//! ### When is Breach Repudiatory?
//! 1. **Breach of condition** - automatically repudiatory (Poussard v Spiers)
//! 2. **Breach of innominate term** - if deprives of substantially whole benefit (Hong Kong Fir)
//! 3. **Renunciation** - clear refusal to perform
//! 4. **Impossibility** - self-induced impossibility
//!
//! ## Election
//!
//! When repudiatory breach occurs, innocent party must elect (Vitol v Norelf [1996]):
//!
//! ### Terminate
//! - Accept repudiation
//! - Contract discharged for future performance
//! - Both parties released from future obligations
//! - Innocent party can claim damages
//!
//! ### Affirm
//! - Continue with contract
//! - Await performance/further breach
//! - Contract remains alive for both parties (White & Carter v McGregor [1962])
//!
//! ## Affirmation
//!
//! Once elected to affirm, cannot later terminate for same breach:
//! - Must have knowledge of breach and right to terminate
//! - Conduct must be unequivocal
//! - Mere lapse of time may indicate affirmation (Yukong Line v Rendsburg [1998])

use serde::{Deserialize, Serialize};

use super::terms::TermType;
use super::types::Party;

// ============================================================================
// Breach Types
// ============================================================================

/// Type of contract breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachType {
    /// Actual breach - occurs when performance due
    Actual,
    /// Anticipatory breach - before performance due (Hochster v De La Tour)
    Anticipatory,
}

impl BreachType {
    /// Get description with case law reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::Actual => {
                "Breach occurring at time performance is due. Party fails to perform, \
                 performs defectively, or performs incompletely."
            }
            Self::Anticipatory => {
                "Breach before performance due. Following Hochster v De La Tour [1853], \
                 party clearly indicates they will not perform when due."
            }
        }
    }
}

/// Category of breach based on how it occurs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachCategory {
    /// Complete non-performance
    NonPerformance,
    /// Performance rendered but defective
    DefectivePerformance,
    /// Partial/incomplete performance
    IncompletePerformance,
    /// Express renunciation of obligations
    ExpressRenunciation,
    /// Implied renunciation by conduct
    ImpliedRenunciation,
    /// Self-induced impossibility
    SelfInducedImpossibility,
}

impl BreachCategory {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::NonPerformance => "Complete failure to perform contracted obligations",
            Self::DefectivePerformance => {
                "Performance rendered but not in accordance with contract"
            }
            Self::IncompletePerformance => "Partial performance only",
            Self::ExpressRenunciation => {
                "Clear and unequivocal statement that party will not perform"
            }
            Self::ImpliedRenunciation => {
                "Conduct evincing intention not to perform (Frost v Knight [1872])"
            }
            Self::SelfInducedImpossibility => {
                "Party renders own performance impossible (Universal Cargo Carriers v Citati [1957])"
            }
        }
    }
}

// ============================================================================
// Repudiatory Breach Analysis
// ============================================================================

/// Severity assessment of breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachSeverity {
    /// Repudiatory - goes to root, entitles termination
    Repudiatory,
    /// Non-repudiatory - damages only
    NonRepudiatory,
}

/// Basis for breach being repudiatory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepudiationBasis {
    /// Breach of condition (automatically repudiatory)
    BreachOfCondition {
        /// The condition breached
        condition_description: String,
    },
    /// Substantial deprivation from innominate term breach
    SubstantialDeprivation {
        /// Expected benefit
        expected_benefit: String,
        /// Actual receipt
        actual_receipt: String,
    },
    /// Express renunciation
    ExpressRenunciation {
        /// Words/conduct constituting renunciation
        renunciation_details: String,
    },
    /// Self-induced impossibility
    SelfInducedImpossibility {
        /// How impossibility was caused
        impossibility_cause: String,
    },
}

/// Analysis of whether breach is repudiatory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepudiationAnalysis {
    /// The breach being analyzed
    pub breach_description: String,
    /// Type of term breached (if applicable)
    pub term_type: Option<TermType>,
    /// Basis for repudiation claim
    pub basis: RepudiationBasis,
    /// Is the breach repudiatory?
    pub is_repudiatory: bool,
    /// Analysis
    pub analysis: String,
}

impl RepudiationAnalysis {
    /// Analyze breach of condition
    pub fn analyze_condition_breach(breach_description: &str, condition: &str) -> Self {
        Self {
            breach_description: breach_description.to_string(),
            term_type: Some(TermType::Condition),
            basis: RepudiationBasis::BreachOfCondition {
                condition_description: condition.to_string(),
            },
            is_repudiatory: true,
            analysis: format!(
                "Breach of condition '{}'. Following Poussard v Spiers [1876], breach of \
                 condition is automatically repudiatory. Innocent party entitled to terminate \
                 and claim damages.",
                condition
            ),
        }
    }

    /// Analyze breach of innominate term
    pub fn analyze_innominate_breach(
        breach_description: &str,
        expected_benefit: &str,
        actual_receipt: &str,
        is_substantial_deprivation: bool,
    ) -> Self {
        let is_repudiatory = is_substantial_deprivation;

        let analysis = if is_substantial_deprivation {
            format!(
                "Breach of innominate term. Expected benefit: '{}'. Actual receipt: '{}'. \
                 Following Hong Kong Fir Shipping v Kawasaki [1962], breach deprives innocent \
                 party of substantially the whole benefit of the contract. Breach is \
                 repudiatory - right to terminate.",
                expected_benefit, actual_receipt
            )
        } else {
            format!(
                "Breach of innominate term. Expected benefit: '{}'. Actual receipt: '{}'. \
                 Following Hong Kong Fir Shipping v Kawasaki [1962], breach does NOT deprive \
                 innocent party of substantially the whole benefit. Breach is non-repudiatory \
                 - damages only.",
                expected_benefit, actual_receipt
            )
        };

        Self {
            breach_description: breach_description.to_string(),
            term_type: Some(TermType::Innominate),
            basis: RepudiationBasis::SubstantialDeprivation {
                expected_benefit: expected_benefit.to_string(),
                actual_receipt: actual_receipt.to_string(),
            },
            is_repudiatory,
            analysis,
        }
    }

    /// Analyze express renunciation
    pub fn analyze_renunciation(
        renunciation_details: &str,
        is_clear_and_unequivocal: bool,
    ) -> Self {
        let is_repudiatory = is_clear_and_unequivocal;

        let analysis = if is_clear_and_unequivocal {
            format!(
                "Express renunciation: '{}'. Renunciation is clear and unequivocal, evincing \
                 intention not to be bound by the contract. Following Hochster v De La Tour \
                 [1853], this constitutes anticipatory repudiatory breach.",
                renunciation_details
            )
        } else {
            format!(
                "Alleged renunciation: '{}'. However, statement is NOT sufficiently clear \
                 and unequivocal to constitute renunciation. A party must evince intention \
                 not to perform that is clear and absolute (Woodar Investment v Wimpey [1980]).",
                renunciation_details
            )
        };

        Self {
            breach_description: format!("Alleged renunciation: {}", renunciation_details),
            term_type: None,
            basis: RepudiationBasis::ExpressRenunciation {
                renunciation_details: renunciation_details.to_string(),
            },
            is_repudiatory,
            analysis,
        }
    }

    /// Analyze self-induced impossibility
    pub fn analyze_impossibility(impossibility_cause: &str, was_deliberate: bool) -> Self {
        let analysis = format!(
            "Self-induced impossibility: '{}'. Following Universal Cargo Carriers v Citati \
             [1957], where party renders performance impossible by own act, this constitutes \
             repudiatory breach.{}",
            impossibility_cause,
            if was_deliberate {
                " Impossibility was deliberate."
            } else {
                " Though not deliberate, party is responsible for making performance impossible."
            }
        );

        Self {
            breach_description: format!("Self-induced impossibility: {}", impossibility_cause),
            term_type: None,
            basis: RepudiationBasis::SelfInducedImpossibility {
                impossibility_cause: impossibility_cause.to_string(),
            },
            is_repudiatory: true,
            analysis,
        }
    }
}

// ============================================================================
// Election Doctrine
// ============================================================================

/// Election made by innocent party
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Election {
    /// Terminate the contract
    Terminate,
    /// Affirm the contract
    Affirm,
    /// No election made yet
    NotYetMade,
}

/// Analysis of election doctrine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElectionAnalysis {
    /// Is there a repudiatory breach giving rise to election?
    pub repudiatory_breach: bool,
    /// Party making election
    pub electing_party: Party,
    /// Election made
    pub election: Election,
    /// Manner of election (express or conduct)
    pub election_manner: Option<ElectionManner>,
    /// Consequences of election
    pub consequences: Vec<String>,
    /// Analysis
    pub analysis: String,
}

/// How election was made
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElectionManner {
    /// Express communication
    ExpressCommunication {
        /// Details of communication
        details: String,
    },
    /// Conduct evincing election
    Conduct {
        /// Conduct description
        conduct: String,
    },
    /// Silence/inaction (may be affirmation)
    Silence {
        /// Duration of silence
        duration: String,
    },
}

impl ElectionAnalysis {
    /// Create analysis for termination election
    pub fn terminate(
        electing_party: Party,
        manner: ElectionManner,
        repudiatory_breach: bool,
    ) -> Self {
        if !repudiatory_breach {
            return Self {
                repudiatory_breach: false,
                electing_party,
                election: Election::NotYetMade,
                election_manner: None,
                consequences: vec![
                    "No right to terminate - breach is not repudiatory".to_string(),
                    "Purported termination may itself be repudiatory breach".to_string(),
                ],
                analysis: "Cannot terminate for non-repudiatory breach. If innocent party \
                          purports to terminate without justification, this may itself \
                          constitute repudiatory breach (Woodar Investment v Wimpey [1980])."
                    .to_string(),
            };
        }

        Self {
            repudiatory_breach: true,
            electing_party,
            election: Election::Terminate,
            election_manner: Some(manner),
            consequences: vec![
                "Contract discharged for future performance".to_string(),
                "Both parties released from future obligations".to_string(),
                "Accrued rights preserved".to_string(),
                "Innocent party entitled to damages".to_string(),
            ],
            analysis: "Election to terminate accepted. Following Vitol SA v Norelf Ltd \
                      [1996], acceptance of repudiation need not be communicated - comes to \
                      other party's attention in due course suffices. Contract discharged \
                      for the future."
                .to_string(),
        }
    }

    /// Create analysis for affirmation election
    pub fn affirm(electing_party: Party, manner: ElectionManner, repudiatory_breach: bool) -> Self {
        if !repudiatory_breach {
            return Self {
                repudiatory_breach: false,
                electing_party,
                election: Election::NotYetMade,
                election_manner: None,
                consequences: vec![
                    "No election required - breach not repudiatory".to_string(),
                    "Contract continues".to_string(),
                    "Damages available for breach".to_string(),
                ],
                analysis: "Non-repudiatory breach does not give rise to election. Contract \
                          continues and innocent party has claim for damages."
                    .to_string(),
            };
        }

        Self {
            repudiatory_breach: true,
            electing_party,
            election: Election::Affirm,
            election_manner: Some(manner),
            consequences: vec![
                "Contract remains alive for both parties".to_string(),
                "Cannot later terminate for same breach".to_string(),
                "May still terminate for subsequent breach".to_string(),
                "White & Carter v McGregor [1962] - can continue performance".to_string(),
            ],
            analysis: "Election to affirm. Contract remains alive for benefit of both parties \
                      (White & Carter v McGregor [1962]). Innocent party loses right to \
                      terminate for this breach but retains damages claim. Note: subsequent \
                      repudiatory breach may give fresh right to terminate."
                .to_string(),
        }
    }
}

// ============================================================================
// Affirmation Analysis
// ============================================================================

/// Requirements for valid affirmation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffirmationRequirements {
    /// Did party have knowledge of breach?
    pub knowledge_of_breach: bool,
    /// Did party have knowledge of right to terminate?
    pub knowledge_of_right: bool,
    /// Was conduct unequivocal?
    pub unequivocal_conduct: bool,
    /// Details of affirming conduct
    pub conduct_details: String,
}

/// Analysis of affirmation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffirmationAnalysis {
    /// Requirements assessment
    pub requirements: AffirmationRequirements,
    /// Time elapsed since breach discovered
    pub time_elapsed: Option<String>,
    /// Has affirmation occurred?
    pub affirmation_occurred: bool,
    /// Is right to terminate lost?
    pub right_to_terminate_lost: bool,
    /// Analysis
    pub analysis: String,
}

impl AffirmationAnalysis {
    /// Analyze whether affirmation has occurred
    pub fn analyze(requirements: AffirmationRequirements, time_elapsed: Option<&str>) -> Self {
        let valid_affirmation = requirements.knowledge_of_breach
            && requirements.knowledge_of_right
            && requirements.unequivocal_conduct;

        // Even without explicit affirmation, lapse of time may bar termination
        let lost_by_delay = time_elapsed
            .map(|t| t.contains("month") || t.contains("year"))
            .unwrap_or(false);

        let affirmation_occurred = valid_affirmation || lost_by_delay;
        let right_to_terminate_lost = affirmation_occurred;

        let analysis = if valid_affirmation {
            format!(
                "Affirmation has occurred. Party had knowledge of breach and right to \
                 terminate. Conduct was unequivocal: '{}'. Following Peyman v Lanjani \
                 [1985], valid affirmation requires knowledge of both breach and right \
                 to terminate. Right to terminate for this breach is LOST.",
                requirements.conduct_details
            )
        } else if lost_by_delay {
            format!(
                "Right to terminate likely lost by delay. {} elapsed since breach discovered. \
                 Following Yukong Line v Rendsburg [1998], lapse of time may indicate \
                 affirmation. Party should have acted promptly to preserve right to terminate.",
                time_elapsed.unwrap_or("Significant time")
            )
        } else if !requirements.knowledge_of_breach {
            "No affirmation - party did not have knowledge of breach. Cannot affirm what \
             is not known."
                .to_string()
        } else if !requirements.knowledge_of_right {
            "No affirmation - party did not know of right to terminate. Following Peyman v \
             Lanjani [1985], affirmation requires knowledge of right as well as breach. \
             Right to terminate preserved."
                .to_string()
        } else {
            format!(
                "Conduct '{}' not sufficiently unequivocal to constitute affirmation. \
                 Right to terminate may still be available.",
                requirements.conduct_details
            )
        };

        Self {
            requirements,
            time_elapsed: time_elapsed.map(|s| s.to_string()),
            affirmation_occurred,
            right_to_terminate_lost,
            analysis,
        }
    }
}

// ============================================================================
// Anticipatory Breach
// ============================================================================

/// Analysis of anticipatory breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnticipatoryBreachAnalysis {
    /// Date performance was due
    pub performance_due_date: String,
    /// Date of alleged anticipatory breach
    pub anticipatory_breach_date: String,
    /// What constituted the anticipatory breach?
    pub breach_constitutent: AnticipatoryBreachType,
    /// Is this a valid anticipatory breach?
    pub is_valid_anticipatory_breach: bool,
    /// Options available to innocent party
    pub innocent_party_options: Vec<InnocentPartyOption>,
    /// Analysis
    pub analysis: String,
}

/// Type of anticipatory breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnticipatoryBreachType {
    /// Express renunciation
    ExpressRenunciation {
        /// Statement made
        statement: String,
    },
    /// Conduct rendering performance impossible
    ImpossibilityByConduct {
        /// Conduct description
        conduct: String,
    },
}

/// Options available to innocent party on anticipatory breach
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnocentPartyOption {
    /// Accept repudiation and sue immediately
    AcceptAndSueImmediately,
    /// Await performance date then sue if not performed
    AwaitPerformance,
    /// Affirm and continue with contract
    AffirmAndContinue,
}

impl AnticipatoryBreachAnalysis {
    /// Analyze anticipatory breach situation
    pub fn analyze(
        performance_due_date: &str,
        anticipatory_breach_date: &str,
        breach_type: AnticipatoryBreachType,
        is_clear_and_unequivocal: bool,
    ) -> Self {
        let is_valid = is_clear_and_unequivocal;

        let options = if is_valid {
            vec![
                InnocentPartyOption::AcceptAndSueImmediately,
                InnocentPartyOption::AwaitPerformance,
                InnocentPartyOption::AffirmAndContinue,
            ]
        } else {
            vec![InnocentPartyOption::AwaitPerformance]
        };

        let analysis = match &breach_type {
            AnticipatoryBreachType::ExpressRenunciation { statement } => {
                if is_valid {
                    format!(
                        "Anticipatory breach by express renunciation: '{}'. Performance due: {}, \
                         breach: {}. Following Hochster v De La Tour [1853], innocent party may: \
                         (1) accept repudiation and sue immediately; (2) await performance date; \
                         (3) affirm. If awaiting, remains subject to frustration/breach by \
                         innocent party (Avery v Bowden [1855]).",
                        statement, performance_due_date, anticipatory_breach_date
                    )
                } else {
                    format!(
                        "Alleged anticipatory breach: '{}'. Statement is NOT sufficiently clear \
                         and unequivocal. Following Woodar Investment v Wimpey [1980], intention \
                         not to perform must be clear and absolute. Recommend awaiting performance \
                         date.",
                        statement
                    )
                }
            }
            AnticipatoryBreachType::ImpossibilityByConduct { conduct } => {
                format!(
                    "Anticipatory breach by conduct: '{}'. Party has rendered own performance \
                     impossible before due date. Following Universal Cargo Carriers v Citati \
                     [1957], this constitutes anticipatory repudiatory breach.",
                    conduct
                )
            }
        };

        Self {
            performance_due_date: performance_due_date.to_string(),
            anticipatory_breach_date: anticipatory_breach_date.to_string(),
            breach_constitutent: breach_type,
            is_valid_anticipatory_breach: is_valid,
            innocent_party_options: options,
            analysis,
        }
    }
}

// ============================================================================
// Comprehensive Breach Analysis
// ============================================================================

/// Full breach analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachAnalysis {
    /// Parties involved
    pub breaching_party: Party,
    /// Innocent party
    pub innocent_party: Party,
    /// Type of breach
    pub breach_type: BreachType,
    /// Category of breach
    pub category: BreachCategory,
    /// Description of breach
    pub description: String,
    /// Repudiation analysis
    pub repudiation: Option<RepudiationAnalysis>,
    /// Anticipatory breach analysis (if applicable)
    pub anticipatory: Option<AnticipatoryBreachAnalysis>,
    /// Is breach repudiatory?
    pub is_repudiatory: bool,
    /// Available remedies
    pub available_remedies: Vec<AvailableRemedy>,
    /// Recommended action for innocent party
    pub recommended_action: String,
}

/// Remedies available for breach
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AvailableRemedy {
    /// Damages (always available)
    Damages,
    /// Termination (if repudiatory)
    Termination,
    /// Specific performance (if applicable)
    SpecificPerformance,
    /// Injunction (if applicable)
    Injunction,
}

impl BreachAnalysis {
    /// Create comprehensive breach analysis
    pub fn analyze(
        breaching_party: Party,
        innocent_party: Party,
        breach_type: BreachType,
        category: BreachCategory,
        description: &str,
        term_type: Option<TermType>,
        is_substantial_deprivation: bool,
    ) -> Self {
        // Determine if repudiatory
        let is_repudiatory = match category {
            BreachCategory::ExpressRenunciation | BreachCategory::SelfInducedImpossibility => true,
            BreachCategory::ImpliedRenunciation => true,
            _ => match term_type {
                Some(TermType::Condition) => true,
                Some(TermType::Innominate) => is_substantial_deprivation,
                Some(TermType::Warranty) => false,
                None => false,
            },
        };

        // Determine available remedies
        let mut remedies = vec![AvailableRemedy::Damages];
        if is_repudiatory {
            remedies.push(AvailableRemedy::Termination);
        }

        // Build repudiation analysis if relevant
        let repudiation = term_type.map(|tt| match tt {
            TermType::Condition => {
                RepudiationAnalysis::analyze_condition_breach(description, description)
            }
            TermType::Innominate => RepudiationAnalysis::analyze_innominate_breach(
                description,
                "contracted benefit",
                "actual receipt",
                is_substantial_deprivation,
            ),
            TermType::Warranty => RepudiationAnalysis {
                breach_description: description.to_string(),
                term_type: Some(TermType::Warranty),
                basis: RepudiationBasis::BreachOfCondition {
                    condition_description: description.to_string(),
                },
                is_repudiatory: false,
                analysis: format!(
                    "Breach of warranty '{}'. Breach of warranty is never repudiatory \
                     (Bettini v Gye [1876]). Damages only.",
                    description
                ),
            },
        });

        let recommended_action = if is_repudiatory {
            "Consider whether to: (1) Accept repudiation and terminate, then claim damages; \
             or (2) Affirm and claim damages while requiring continued performance. \
             Decision should be made promptly to avoid loss of right to terminate."
                .to_string()
        } else {
            "Breach is non-repudiatory. Contract continues. Claim damages for breach.".to_string()
        };

        Self {
            breaching_party,
            innocent_party,
            breach_type,
            category,
            description: description.to_string(),
            repudiation,
            anticipatory: None,
            is_repudiatory,
            available_remedies: remedies,
            recommended_action,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_party(name: &str) -> Party {
        Party {
            name: name.to_string(),
            party_type: super::super::types::PartyType::Individual,
            age: Some(30),
        }
    }

    #[test]
    fn test_breach_of_condition_repudiatory() {
        let analysis = RepudiationAnalysis::analyze_condition_breach(
            "Failed to deliver on time",
            "Time is of the essence",
        );
        assert!(analysis.is_repudiatory);
        assert!(analysis.analysis.contains("Poussard"));
    }

    #[test]
    fn test_breach_of_innominate_substantial() {
        let analysis = RepudiationAnalysis::analyze_innominate_breach(
            "Defective machinery",
            "Fully functional machinery for production",
            "Machinery cannot be used at all",
            true,
        );
        assert!(analysis.is_repudiatory);
        assert!(analysis.analysis.contains("Hong Kong Fir"));
    }

    #[test]
    fn test_breach_of_innominate_not_substantial() {
        let analysis = RepudiationAnalysis::analyze_innominate_breach(
            "Minor defect in paint",
            "Pristine finish",
            "Small scratch on underside",
            false,
        );
        assert!(!analysis.is_repudiatory);
    }

    #[test]
    fn test_express_renunciation() {
        let analysis =
            RepudiationAnalysis::analyze_renunciation("I will not perform this contract", true);
        assert!(analysis.is_repudiatory);
        assert!(analysis.analysis.contains("Hochster"));
    }

    #[test]
    fn test_vague_statement_not_renunciation() {
        let analysis = RepudiationAnalysis::analyze_renunciation(
            "I'm having second thoughts about this",
            false,
        );
        assert!(!analysis.is_repudiatory);
        assert!(analysis.analysis.contains("Woodar"));
    }

    #[test]
    fn test_election_to_terminate() {
        let analysis = ElectionAnalysis::terminate(
            test_party("Buyer"),
            ElectionManner::ExpressCommunication {
                details: "Written notice of termination".to_string(),
            },
            true,
        );
        assert_eq!(analysis.election, Election::Terminate);
        assert!(
            analysis
                .consequences
                .contains(&"Contract discharged for future performance".to_string())
        );
    }

    #[test]
    fn test_cannot_terminate_non_repudiatory() {
        let analysis = ElectionAnalysis::terminate(
            test_party("Buyer"),
            ElectionManner::ExpressCommunication {
                details: "Notice".to_string(),
            },
            false, // not repudiatory
        );
        assert_eq!(analysis.election, Election::NotYetMade);
        assert!(analysis.analysis.contains("Cannot terminate"));
    }

    #[test]
    fn test_affirmation_with_knowledge() {
        let requirements = AffirmationRequirements {
            knowledge_of_breach: true,
            knowledge_of_right: true,
            unequivocal_conduct: true,
            conduct_details: "Continued to accept deliveries".to_string(),
        };
        let analysis = AffirmationAnalysis::analyze(requirements, None);
        assert!(analysis.affirmation_occurred);
        assert!(analysis.right_to_terminate_lost);
    }

    #[test]
    fn test_no_affirmation_without_knowledge_of_right() {
        let requirements = AffirmationRequirements {
            knowledge_of_breach: true,
            knowledge_of_right: false,
            unequivocal_conduct: true,
            conduct_details: "Continued performance".to_string(),
        };
        let analysis = AffirmationAnalysis::analyze(requirements, None);
        assert!(!analysis.affirmation_occurred);
        assert!(analysis.analysis.contains("Peyman v Lanjani"));
    }

    #[test]
    fn test_affirmation_by_delay() {
        let requirements = AffirmationRequirements {
            knowledge_of_breach: true,
            knowledge_of_right: true,
            unequivocal_conduct: false,
            conduct_details: "No action taken".to_string(),
        };
        let analysis = AffirmationAnalysis::analyze(requirements, Some("6 months"));
        assert!(analysis.affirmation_occurred);
        assert!(analysis.analysis.contains("delay"));
    }

    #[test]
    fn test_anticipatory_breach_express() {
        let analysis = AnticipatoryBreachAnalysis::analyze(
            "1 January 2025",
            "1 December 2024",
            AnticipatoryBreachType::ExpressRenunciation {
                statement: "I will not deliver the goods".to_string(),
            },
            true,
        );
        assert!(analysis.is_valid_anticipatory_breach);
        assert!(
            analysis
                .innocent_party_options
                .contains(&InnocentPartyOption::AcceptAndSueImmediately)
        );
    }

    #[test]
    fn test_comprehensive_breach_analysis() {
        let analysis = BreachAnalysis::analyze(
            test_party("Seller"),
            test_party("Buyer"),
            BreachType::Actual,
            BreachCategory::NonPerformance,
            "Failed to deliver goods",
            Some(TermType::Condition),
            false,
        );
        assert!(analysis.is_repudiatory);
        assert!(
            analysis
                .available_remedies
                .contains(&AvailableRemedy::Termination)
        );
        assert!(
            analysis
                .available_remedies
                .contains(&AvailableRemedy::Damages)
        );
    }

    #[test]
    fn test_warranty_breach_not_repudiatory() {
        let analysis = BreachAnalysis::analyze(
            test_party("Seller"),
            test_party("Buyer"),
            BreachType::Actual,
            BreachCategory::DefectivePerformance,
            "Minor scratch on goods",
            Some(TermType::Warranty),
            false,
        );
        assert!(!analysis.is_repudiatory);
        assert!(
            !analysis
                .available_remedies
                .contains(&AvailableRemedy::Termination)
        );
    }
}
