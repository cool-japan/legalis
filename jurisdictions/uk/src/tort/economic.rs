//! UK Economic Torts
//!
//! This module implements economic torts under common law:
//! - Inducing breach of contract (OBG v Allan [2008] 1 AC 1)
//! - Causing loss by unlawful means (OBG v Allan)
//! - Unlawful means conspiracy (Revenue v Total Network [2008])
//! - Lawful means conspiracy (Lonrho v Shell)
//!
//! Key cases:
//! - OBG Ltd v Allan [2008] 1 AC 1 (restructuring of economic torts)
//! - Revenue v Total Network [2008] 1 AC 1174 (unlawful means conspiracy)
//! - Lumley v Gye (1853) 2 E&B 216 (inducing breach)

use serde::{Deserialize, Serialize};

use super::error::TortError;

// ============================================================================
// Core Types for Economic Torts
// ============================================================================

/// Type of economic tort
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicTortType {
    /// Inducing breach of contract
    InducingBreachOfContract,
    /// Causing loss by unlawful means
    UnlawfulMeans,
    /// Unlawful means conspiracy
    UnlawfulMeansConspiracy,
    /// Lawful means conspiracy
    LawfulMeansConspiracy,
    /// Intimidation
    Intimidation,
    /// Passing off
    PassingOff,
}

/// Type of unlawful means
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnlawfulMeansType {
    /// Crime
    Crime,
    /// Tort against third party
    TortAgainstThirdParty,
    /// Breach of contract with third party
    BreachOfContractWithThirdParty,
    /// Breach of statutory duty
    BreachOfStatutoryDuty,
    /// Fraud/deceit
    Fraud,
    /// Breach of equitable obligation
    BreachOfEquitableObligation,
    /// Other actionable wrong
    OtherActionableWrong(String),
}

/// Type of contract allegedly breached
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Employment contract
    Employment,
    /// Commercial contract
    Commercial,
    /// Service contract
    Service,
    /// Supply contract
    Supply,
    /// Agency contract
    Agency,
    /// Other
    Other(String),
}

// ============================================================================
// Inducing Breach of Contract Analysis
// ============================================================================

/// Inducing breach of contract analysis (OBG v Allan)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InducingBreachAnalysis {
    /// Contract analysis
    pub contract: ContractAnalysis,
    /// Defendant's knowledge of contract
    pub knowledge: KnowledgeAnalysis,
    /// Defendant's intention
    pub intention: IntentionAnalysis,
    /// Inducement analysis
    pub inducement: InducementAnalysis,
    /// Breach occurred
    pub breach: BreachAnalysis,
    /// Resulting loss
    pub loss: LossAnalysis,
    /// Defences
    pub defences: Vec<EconomicTortDefence>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Contract analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractAnalysis {
    /// Parties to contract
    pub parties: Vec<String>,
    /// Type of contract
    pub contract_type: ContractType,
    /// Was there a valid contract?
    pub valid_contract: bool,
    /// Is contract relevant to inducement?
    pub relevant_to_inducement: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Knowledge analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeAnalysis {
    /// Did defendant know of contract?
    pub knew_of_contract: bool,
    /// Did defendant know of contractual terms?
    pub knew_of_terms: bool,
    /// Actual or constructive knowledge
    pub knowledge_type: KnowledgeType,
    /// Sufficient knowledge?
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of knowledge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// Actual knowledge
    Actual,
    /// Constructive knowledge (should have known)
    Constructive,
    /// Reckless (didn't care)
    Reckless,
    /// No knowledge
    None,
}

/// Intention analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentionAnalysis {
    /// Did defendant intend to procure breach?
    pub intended_breach: bool,
    /// Was breach an end in itself or means to an end?
    pub breach_as_end: bool,
    /// Did defendant intend to harm claimant?
    pub intended_harm: bool,
    /// Sufficient intention? (OBG v Allan)
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Inducement analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InducementAnalysis {
    /// Method of inducement
    pub method: InducementMethod,
    /// Did defendant act on contracting party's will?
    pub acted_on_will: bool,
    /// Was inducement effective?
    pub effective: bool,
    /// Sufficient inducement?
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Method of inducement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InducementMethod {
    /// Persuasion
    Persuasion,
    /// Pressure
    Pressure,
    /// Bribery
    Bribery,
    /// Threats
    Threats,
    /// Deceit
    Deceit,
    /// Financial incentive
    FinancialIncentive,
    /// Other
    Other(String),
}

/// Breach analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachAnalysis {
    /// Did breach occur?
    pub breach_occurred: bool,
    /// Nature of breach
    pub nature: String,
    /// Was breach caused by defendant?
    pub caused_by_defendant: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Loss analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LossAnalysis {
    /// Did claimant suffer loss?
    pub loss_suffered: bool,
    /// Type of loss
    pub loss_type: EconomicLossType,
    /// Amount of loss
    pub amount: Option<f64>,
    /// Was loss caused by breach?
    pub caused_by_breach: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of economic loss
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicLossType {
    /// Loss of profit
    LossOfProfit,
    /// Loss of contract
    LossOfContract,
    /// Wasted expenditure
    WastedExpenditure,
    /// Damage to business
    DamageToBusiness,
    /// Loss of customers
    LossOfCustomers,
    /// Other financial loss
    OtherFinancial(String),
}

// ============================================================================
// Unlawful Means Tort Analysis
// ============================================================================

/// Causing loss by unlawful means analysis (OBG v Allan)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulMeansAnalysis {
    /// Unlawful means used
    pub unlawful_means: UnlawfulMeansDetail,
    /// Intention to cause loss
    pub intention: UnlawfulMeansIntention,
    /// Loss suffered
    pub loss: LossAnalysis,
    /// Dealing requirement
    pub dealing_requirement: DealingAnalysis,
    /// Defences
    pub defences: Vec<EconomicTortDefence>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Detail of unlawful means
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulMeansDetail {
    /// Type of unlawful means
    pub means_type: UnlawfulMeansType,
    /// Description
    pub description: String,
    /// Is it actionable by third party?
    pub actionable_by_third_party: bool,
    /// Would it give third party cause of action?
    pub gives_cause_of_action: bool,
    /// Sufficient unlawful means? (OBG narrow approach)
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Intention for unlawful means tort
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulMeansIntention {
    /// Did defendant intend to cause loss to claimant?
    pub intended_loss: bool,
    /// Was harm to claimant an end in itself?
    pub harm_as_end: bool,
    /// Was harm a means to another end?
    pub harm_as_means: bool,
    /// Sufficient intention?
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Dealing requirement analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DealingAnalysis {
    /// Did claimant deal with third party?
    pub claimant_dealt_with_third_party: bool,
    /// Was dealing affected by unlawful means?
    pub dealing_affected: bool,
    /// Requirement satisfied?
    pub satisfied: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Conspiracy Analysis
// ============================================================================

/// Conspiracy analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConspiracyAnalysis {
    /// Type of conspiracy
    pub conspiracy_type: ConspiracyType,
    /// Agreement analysis
    pub agreement: AgreementAnalysis,
    /// Intention analysis
    pub intention: ConspiracyIntention,
    /// Unlawful means (for unlawful means conspiracy)
    pub unlawful_means: Option<UnlawfulMeansDetail>,
    /// Damage analysis
    pub damage: DamageAnalysis,
    /// Justification (for lawful means conspiracy)
    pub justification: Option<JustificationAnalysis>,
    /// Defences
    pub defences: Vec<EconomicTortDefence>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of conspiracy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConspiracyType {
    /// Unlawful means conspiracy
    UnlawfulMeans,
    /// Lawful means conspiracy (simple conspiracy to injure)
    LawfulMeans,
}

/// Agreement analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementAnalysis {
    /// Number of conspirators
    pub number_of_conspirators: usize,
    /// Names/description of conspirators
    pub conspirators: Vec<String>,
    /// Was there an agreement/combination?
    pub agreement_exists: bool,
    /// Nature of agreement
    pub agreement_nature: String,
    /// Sufficient agreement?
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Intention for conspiracy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConspiracyIntention {
    /// For lawful means: was predominant purpose to injure?
    pub predominant_purpose_to_injure: bool,
    /// For unlawful means: was injury intended?
    pub injury_intended: bool,
    /// Was self-interest legitimate justification?
    pub self_interest_justifies: bool,
    /// Sufficient intention?
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Damage analysis for conspiracy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageAnalysis {
    /// Was damage suffered?
    pub damage_suffered: bool,
    /// Type of damage
    pub damage_type: EconomicLossType,
    /// Amount
    pub amount: Option<f64>,
    /// Was damage caused by conspiracy?
    pub caused_by_conspiracy: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Justification analysis (for lawful means conspiracy)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JustificationAnalysis {
    /// Was there a legitimate interest being protected?
    pub legitimate_interest: bool,
    /// Description of interest
    pub interest_description: String,
    /// Were means proportionate?
    pub proportionate: bool,
    /// Justification established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Defences
// ============================================================================

/// Defence to economic tort
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicTortDefence {
    /// Type of defence
    pub defence_type: EconomicDefenceType,
    /// Evidence supporting defence
    pub evidence: Vec<String>,
    /// Does defence apply?
    pub applies: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Types of defence to economic torts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicDefenceType {
    /// Justification
    Justification,
    /// Lack of knowledge
    LackOfKnowledge,
    /// Lack of intention
    LackOfIntention,
    /// No breach occurred
    NoBreachOccurred,
    /// No loss suffered
    NoLossSuffered,
    /// Equal or superior right
    EqualOrSuperiorRight,
    /// Trade union immunity (TULRCA 1992)
    TradeUnionImmunity,
    /// Limitation
    Limitation,
}

// ============================================================================
// Economic Tort Analyzer
// ============================================================================

/// Analyzer for economic torts
#[derive(Debug, Clone)]
pub struct EconomicTortAnalyzer;

impl EconomicTortAnalyzer {
    /// Analyze inducing breach of contract claim
    pub fn analyze_inducing_breach(
        facts: &InducingBreachFacts,
    ) -> Result<InducingBreachAnalysis, TortError> {
        // 1. Contract analysis
        let contract = ContractAnalysis {
            parties: facts.contract_parties.clone(),
            contract_type: facts.contract_type.clone(),
            valid_contract: facts.valid_contract,
            relevant_to_inducement: facts.valid_contract,
            reasoning: if facts.valid_contract {
                "Valid contract existed between parties".to_string()
            } else {
                "No valid contract established".to_string()
            },
        };

        if !contract.valid_contract {
            return Err(TortError::InducingBreachFails {
                missing_element: "No valid contract".to_string(),
            });
        }

        // 2. Knowledge analysis
        let knowledge = KnowledgeAnalysis {
            knew_of_contract: facts.knew_of_contract,
            knew_of_terms: facts.knew_of_terms,
            knowledge_type: if facts.knew_of_contract {
                KnowledgeType::Actual
            } else {
                KnowledgeType::None
            },
            sufficient: facts.knew_of_contract,
            reasoning: if facts.knew_of_contract {
                "Defendant had knowledge of the contract".to_string()
            } else {
                "Defendant lacked knowledge of contract".to_string()
            },
        };

        if !knowledge.sufficient {
            return Err(TortError::InducingBreachFails {
                missing_element: "No knowledge of contract".to_string(),
            });
        }

        // 3. Intention analysis (OBG v Allan)
        let intention = IntentionAnalysis {
            intended_breach: facts.intended_breach,
            breach_as_end: facts.breach_as_end,
            intended_harm: facts.intended_harm_to_claimant,
            sufficient: facts.intended_breach,
            reasoning: if facts.intended_breach {
                "Defendant intended to procure breach of contract (OBG v Allan)".to_string()
            } else {
                "No intention to procure breach - OBG v Allan requires intention".to_string()
            },
        };

        if !intention.sufficient {
            return Err(TortError::NoIntentionToHarm {
                explanation: "Did not intend to procure breach".to_string(),
            });
        }

        // 4. Inducement analysis
        let inducement = InducementAnalysis {
            method: facts.inducement_method.clone(),
            acted_on_will: facts.acted_on_will,
            effective: facts.inducement_effective,
            sufficient: facts.acted_on_will && facts.inducement_effective,
            reasoning: if facts.acted_on_will && facts.inducement_effective {
                "Defendant's actions induced the breach".to_string()
            } else {
                "No effective inducement established".to_string()
            },
        };

        if !inducement.sufficient {
            return Err(TortError::InducingBreachFails {
                missing_element: "No effective inducement".to_string(),
            });
        }

        // 5. Breach analysis
        let breach = BreachAnalysis {
            breach_occurred: facts.breach_occurred,
            nature: facts.breach_nature.clone(),
            caused_by_defendant: facts.breach_caused_by_defendant,
            reasoning: if facts.breach_occurred && facts.breach_caused_by_defendant {
                "Breach occurred as a result of defendant's inducement".to_string()
            } else {
                "No breach or breach not caused by defendant".to_string()
            },
        };

        if !breach.breach_occurred || !breach.caused_by_defendant {
            return Err(TortError::InducingBreachFails {
                missing_element: "No breach occurred".to_string(),
            });
        }

        // 6. Loss analysis
        let loss = LossAnalysis {
            loss_suffered: facts.loss_suffered,
            loss_type: facts.loss_type.clone(),
            amount: facts.loss_amount,
            caused_by_breach: facts.loss_caused_by_breach,
            reasoning: if facts.loss_suffered && facts.loss_caused_by_breach {
                "Claimant suffered loss as a result of breach".to_string()
            } else {
                "No loss or loss not caused by breach".to_string()
            },
        };

        // 7. Defences
        let defences = Self::analyze_inducing_breach_defences(facts);
        let defence_applies = defences.iter().any(|d| d.applies);

        let claim_succeeds = !defence_applies;

        Ok(InducingBreachAnalysis {
            contract,
            knowledge,
            intention,
            inducement,
            breach,
            loss,
            defences,
            claim_succeeds,
            reasoning: if claim_succeeds {
                "Inducing breach of contract established (OBG v Allan)".to_string()
            } else {
                "Defence applies to defeat claim".to_string()
            },
        })
    }

    /// Analyze causing loss by unlawful means claim
    pub fn analyze_unlawful_means(
        facts: &UnlawfulMeansFacts,
    ) -> Result<UnlawfulMeansAnalysis, TortError> {
        // 1. Unlawful means analysis (OBG narrow approach)
        let unlawful_means = UnlawfulMeansDetail {
            means_type: facts.means_type.clone(),
            description: facts.means_description.clone(),
            actionable_by_third_party: facts.actionable_by_third_party,
            gives_cause_of_action: facts.gives_cause_of_action,
            sufficient: facts.actionable_by_third_party && facts.gives_cause_of_action,
            reasoning: if facts.actionable_by_third_party && facts.gives_cause_of_action {
                "Unlawful means satisfy OBG requirement - actionable by affected third party"
                    .to_string()
            } else {
                "Means do not satisfy OBG narrow requirement for unlawful means".to_string()
            },
        };

        if !unlawful_means.sufficient {
            return Err(TortError::UnlawfulMeansFails {
                missing_requirement: "Means not unlawful per OBG v Allan".to_string(),
            });
        }

        // 2. Intention analysis
        let intention = UnlawfulMeansIntention {
            intended_loss: facts.intended_loss,
            harm_as_end: facts.harm_as_end,
            harm_as_means: facts.harm_as_means,
            sufficient: facts.intended_loss && (facts.harm_as_end || facts.harm_as_means),
            reasoning: if facts.intended_loss {
                "Defendant intended to cause loss to claimant".to_string()
            } else {
                "No intention to cause loss to claimant".to_string()
            },
        };

        if !intention.sufficient {
            return Err(TortError::NoIntentionToHarm {
                explanation: "Did not intend to cause loss".to_string(),
            });
        }

        // 3. Loss analysis
        let loss = LossAnalysis {
            loss_suffered: facts.loss_suffered,
            loss_type: facts.loss_type.clone(),
            amount: facts.loss_amount,
            caused_by_breach: facts.loss_caused_by_unlawful_means,
            reasoning: if facts.loss_suffered {
                "Loss suffered by claimant".to_string()
            } else {
                "No loss suffered".to_string()
            },
        };

        // 4. Dealing requirement
        let dealing = DealingAnalysis {
            claimant_dealt_with_third_party: facts.claimant_dealt_with_third_party,
            dealing_affected: facts.dealing_affected,
            satisfied: facts.claimant_dealt_with_third_party && facts.dealing_affected,
            reasoning: if facts.claimant_dealt_with_third_party && facts.dealing_affected {
                "Claimant's dealing with third party affected by unlawful means".to_string()
            } else {
                "Dealing requirement not satisfied".to_string()
            },
        };

        // 5. Defences
        let defences = Self::analyze_unlawful_means_defences(facts);
        let defence_applies = defences.iter().any(|d| d.applies);

        let claim_succeeds = unlawful_means.sufficient
            && intention.sufficient
            && loss.loss_suffered
            && dealing.satisfied
            && !defence_applies;

        Ok(UnlawfulMeansAnalysis {
            unlawful_means,
            intention,
            loss,
            dealing_requirement: dealing,
            defences,
            claim_succeeds,
            reasoning: if claim_succeeds {
                "Causing loss by unlawful means established (OBG v Allan)".to_string()
            } else {
                "Claim fails - element missing or defence applies".to_string()
            },
        })
    }

    /// Analyze conspiracy claim
    pub fn analyze_conspiracy(facts: &ConspiracyFacts) -> Result<ConspiracyAnalysis, TortError> {
        // 1. Agreement analysis
        let agreement = AgreementAnalysis {
            number_of_conspirators: facts.conspirators.len(),
            conspirators: facts.conspirators.clone(),
            agreement_exists: facts.agreement_exists,
            agreement_nature: facts.agreement_nature.clone(),
            sufficient: facts.agreement_exists && facts.conspirators.len() >= 2,
            reasoning: if facts.agreement_exists && facts.conspirators.len() >= 2 {
                "Agreement/combination between two or more parties established".to_string()
            } else {
                "No agreement or insufficient parties".to_string()
            },
        };

        if !agreement.sufficient {
            return Err(TortError::ConspiracyFails {
                conspiracy_type: format!("{:?}", facts.conspiracy_type),
                missing_element: "No agreement".to_string(),
            });
        }

        // 2. Unlawful means (if unlawful means conspiracy)
        let unlawful_means = if matches!(facts.conspiracy_type, ConspiracyType::UnlawfulMeans) {
            if let Some(ref means) = facts.unlawful_means {
                Some(UnlawfulMeansDetail {
                    means_type: means.means_type.clone(),
                    description: means.description.clone(),
                    actionable_by_third_party: means.actionable,
                    gives_cause_of_action: means.actionable,
                    sufficient: means.actionable,
                    reasoning: if means.actionable {
                        "Unlawful means established".to_string()
                    } else {
                        "Means not sufficiently unlawful".to_string()
                    },
                })
            } else {
                return Err(TortError::ConspiracyFails {
                    conspiracy_type: "Unlawful means".to_string(),
                    missing_element: "No unlawful means".to_string(),
                });
            }
        } else {
            None
        };

        // 3. Intention analysis
        let intention = ConspiracyIntention {
            predominant_purpose_to_injure: facts.predominant_purpose_to_injure,
            injury_intended: facts.injury_intended,
            self_interest_justifies: facts.self_interest_justifies,
            sufficient: match facts.conspiracy_type {
                ConspiracyType::UnlawfulMeans => facts.injury_intended,
                ConspiracyType::LawfulMeans => {
                    facts.predominant_purpose_to_injure && !facts.self_interest_justifies
                }
            },
            reasoning: match facts.conspiracy_type {
                ConspiracyType::UnlawfulMeans => {
                    if facts.injury_intended {
                        "Intention to injure claimant established".to_string()
                    } else {
                        "No intention to injure".to_string()
                    }
                }
                ConspiracyType::LawfulMeans => {
                    if facts.predominant_purpose_to_injure && !facts.self_interest_justifies {
                        "Predominant purpose to injure without justification".to_string()
                    } else {
                        "Self-interest justifies combination".to_string()
                    }
                }
            },
        };

        if !intention.sufficient {
            return Err(TortError::ConspiracyFails {
                conspiracy_type: format!("{:?}", facts.conspiracy_type),
                missing_element: if matches!(facts.conspiracy_type, ConspiracyType::LawfulMeans)
                    && facts.self_interest_justifies
                {
                    "Self-interest justifies combination".to_string()
                } else {
                    "Insufficient intention".to_string()
                },
            });
        }

        // 4. Damage analysis
        let damage = DamageAnalysis {
            damage_suffered: facts.damage_suffered,
            damage_type: facts.damage_type.clone(),
            amount: facts.damage_amount,
            caused_by_conspiracy: facts.caused_by_conspiracy,
            reasoning: if facts.damage_suffered && facts.caused_by_conspiracy {
                "Damage suffered as result of conspiracy".to_string()
            } else {
                "No damage or causation issue".to_string()
            },
        };

        // 5. Justification (for lawful means only)
        let justification = if matches!(facts.conspiracy_type, ConspiracyType::LawfulMeans) {
            Some(JustificationAnalysis {
                legitimate_interest: facts.legitimate_interest,
                interest_description: facts.interest_description.clone().unwrap_or_default(),
                proportionate: facts.proportionate,
                established: facts.self_interest_justifies || facts.legitimate_interest,
                reasoning: if facts.self_interest_justifies {
                    "Self-interest justifies the combination (Lonrho v Shell)".to_string()
                } else {
                    "No justification established".to_string()
                },
            })
        } else {
            None
        };

        // 6. Defences
        let defences = Self::analyze_conspiracy_defences(facts);
        let defence_applies = defences.iter().any(|d| d.applies);

        let claim_succeeds = agreement.sufficient
            && intention.sufficient
            && damage.damage_suffered
            && damage.caused_by_conspiracy
            && (matches!(facts.conspiracy_type, ConspiracyType::UnlawfulMeans)
                || !justification.as_ref().is_some_and(|j| j.established))
            && !defence_applies;

        Ok(ConspiracyAnalysis {
            conspiracy_type: facts.conspiracy_type.clone(),
            agreement,
            intention,
            unlawful_means,
            damage,
            justification,
            defences,
            claim_succeeds,
            reasoning: if claim_succeeds {
                format!("{:?} conspiracy established", facts.conspiracy_type)
            } else {
                "Conspiracy claim fails".to_string()
            },
        })
    }

    fn analyze_inducing_breach_defences(facts: &InducingBreachFacts) -> Vec<EconomicTortDefence> {
        let mut defences = Vec::new();

        // Justification defence
        if let Some(just) = &facts.justification {
            let applies = just.legitimate_interest && just.proportionate;
            defences.push(EconomicTortDefence {
                defence_type: EconomicDefenceType::Justification,
                evidence: just.evidence.clone(),
                applies,
                reasoning: if applies {
                    "Justification established - legitimate interest protected proportionately"
                        .to_string()
                } else {
                    "Justification not established".to_string()
                },
            });
        }

        // Trade union immunity
        if facts.trade_dispute {
            defences.push(EconomicTortDefence {
                defence_type: EconomicDefenceType::TradeUnionImmunity,
                evidence: vec!["In contemplation or furtherance of trade dispute".to_string()],
                applies: facts.tulrca_immunity,
                reasoning: if facts.tulrca_immunity {
                    "Trade union immunity applies (TULRCA 1992 s.219)".to_string()
                } else {
                    "Trade union immunity not available".to_string()
                },
            });
        }

        defences
    }

    fn analyze_unlawful_means_defences(facts: &UnlawfulMeansFacts) -> Vec<EconomicTortDefence> {
        let mut defences = Vec::new();

        // Lack of intention
        if !facts.intended_loss {
            defences.push(EconomicTortDefence {
                defence_type: EconomicDefenceType::LackOfIntention,
                evidence: vec![],
                applies: true,
                reasoning: "No intention to cause loss to claimant".to_string(),
            });
        }

        defences
    }

    fn analyze_conspiracy_defences(facts: &ConspiracyFacts) -> Vec<EconomicTortDefence> {
        let mut defences = Vec::new();

        // For lawful means conspiracy - self-interest defence
        if matches!(facts.conspiracy_type, ConspiracyType::LawfulMeans)
            && facts.self_interest_justifies
        {
            defences.push(EconomicTortDefence {
                defence_type: EconomicDefenceType::Justification,
                evidence: vec!["Legitimate self-interest".to_string()],
                applies: true,
                reasoning: "Self-interest justifies combination (Lonrho v Shell)".to_string(),
            });
        }

        // Trade union immunity
        if facts.trade_dispute {
            defences.push(EconomicTortDefence {
                defence_type: EconomicDefenceType::TradeUnionImmunity,
                evidence: vec!["In furtherance of trade dispute".to_string()],
                applies: facts.tulrca_immunity,
                reasoning: if facts.tulrca_immunity {
                    "Trade union immunity applies (TULRCA 1992)".to_string()
                } else {
                    "Trade union immunity not available".to_string()
                },
            });
        }

        defences
    }
}

/// Facts for inducing breach of contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InducingBreachFacts {
    /// Contract parties
    pub contract_parties: Vec<String>,
    /// Contract type
    pub contract_type: ContractType,
    /// Valid contract?
    pub valid_contract: bool,
    /// Defendant knew of contract?
    pub knew_of_contract: bool,
    /// Defendant knew of terms?
    pub knew_of_terms: bool,
    /// Defendant intended breach?
    pub intended_breach: bool,
    /// Breach as end in itself?
    pub breach_as_end: bool,
    /// Intended harm to claimant?
    pub intended_harm_to_claimant: bool,
    /// Method of inducement
    pub inducement_method: InducementMethod,
    /// Did defendant act on will of contracting party?
    pub acted_on_will: bool,
    /// Was inducement effective?
    pub inducement_effective: bool,
    /// Did breach occur?
    pub breach_occurred: bool,
    /// Nature of breach
    pub breach_nature: String,
    /// Breach caused by defendant?
    pub breach_caused_by_defendant: bool,
    /// Loss suffered?
    pub loss_suffered: bool,
    /// Type of loss
    pub loss_type: EconomicLossType,
    /// Amount of loss
    pub loss_amount: Option<f64>,
    /// Loss caused by breach?
    pub loss_caused_by_breach: bool,
    /// Justification
    pub justification: Option<JustificationFacts>,
    /// Trade dispute?
    pub trade_dispute: bool,
    /// TULRCA immunity applies?
    pub tulrca_immunity: bool,
}

/// Justification facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JustificationFacts {
    /// Legitimate interest?
    pub legitimate_interest: bool,
    /// Interest description
    pub interest_description: String,
    /// Proportionate?
    pub proportionate: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Facts for unlawful means tort
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulMeansFacts {
    /// Type of unlawful means
    pub means_type: UnlawfulMeansType,
    /// Description of means
    pub means_description: String,
    /// Actionable by third party?
    pub actionable_by_third_party: bool,
    /// Gives cause of action?
    pub gives_cause_of_action: bool,
    /// Intended loss?
    pub intended_loss: bool,
    /// Harm as end?
    pub harm_as_end: bool,
    /// Harm as means?
    pub harm_as_means: bool,
    /// Loss suffered?
    pub loss_suffered: bool,
    /// Loss type
    pub loss_type: EconomicLossType,
    /// Loss amount
    pub loss_amount: Option<f64>,
    /// Loss caused by unlawful means?
    pub loss_caused_by_unlawful_means: bool,
    /// Claimant dealt with third party?
    pub claimant_dealt_with_third_party: bool,
    /// Dealing affected?
    pub dealing_affected: bool,
}

/// Facts for conspiracy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConspiracyFacts {
    /// Type of conspiracy
    pub conspiracy_type: ConspiracyType,
    /// Conspirators
    pub conspirators: Vec<String>,
    /// Agreement exists?
    pub agreement_exists: bool,
    /// Nature of agreement
    pub agreement_nature: String,
    /// Unlawful means (for unlawful means conspiracy)
    pub unlawful_means: Option<UnlawfulMeansFact>,
    /// Predominant purpose to injure? (lawful means)
    pub predominant_purpose_to_injure: bool,
    /// Injury intended?
    pub injury_intended: bool,
    /// Self-interest justifies?
    pub self_interest_justifies: bool,
    /// Legitimate interest?
    pub legitimate_interest: bool,
    /// Interest description
    pub interest_description: Option<String>,
    /// Proportionate?
    pub proportionate: bool,
    /// Damage suffered?
    pub damage_suffered: bool,
    /// Damage type
    pub damage_type: EconomicLossType,
    /// Damage amount
    pub damage_amount: Option<f64>,
    /// Caused by conspiracy?
    pub caused_by_conspiracy: bool,
    /// Trade dispute?
    pub trade_dispute: bool,
    /// TULRCA immunity?
    pub tulrca_immunity: bool,
}

/// Unlawful means fact
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlawfulMeansFact {
    /// Type
    pub means_type: UnlawfulMeansType,
    /// Description
    pub description: String,
    /// Actionable?
    pub actionable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inducing_breach_of_contract() {
        let facts = InducingBreachFacts {
            contract_parties: vec!["Claimant".to_string(), "Third Party".to_string()],
            contract_type: ContractType::Commercial,
            valid_contract: true,
            knew_of_contract: true,
            knew_of_terms: true,
            intended_breach: true,
            breach_as_end: false,
            intended_harm_to_claimant: true,
            inducement_method: InducementMethod::FinancialIncentive,
            acted_on_will: true,
            inducement_effective: true,
            breach_occurred: true,
            breach_nature: "Third party terminated contract".to_string(),
            breach_caused_by_defendant: true,
            loss_suffered: true,
            loss_type: EconomicLossType::LossOfContract,
            loss_amount: Some(100000.0),
            loss_caused_by_breach: true,
            justification: None,
            trade_dispute: false,
            tulrca_immunity: false,
        };

        let result = EconomicTortAnalyzer::analyze_inducing_breach(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
    }

    #[test]
    fn test_inducing_breach_no_intention() {
        let facts = InducingBreachFacts {
            contract_parties: vec!["Claimant".to_string(), "Third Party".to_string()],
            contract_type: ContractType::Employment,
            valid_contract: true,
            knew_of_contract: true,
            knew_of_terms: true,
            intended_breach: false, // No intention
            breach_as_end: false,
            intended_harm_to_claimant: false,
            inducement_method: InducementMethod::Persuasion,
            acted_on_will: true,
            inducement_effective: true,
            breach_occurred: true,
            breach_nature: "Employee left".to_string(),
            breach_caused_by_defendant: true,
            loss_suffered: true,
            loss_type: EconomicLossType::LossOfProfit,
            loss_amount: Some(50000.0),
            loss_caused_by_breach: true,
            justification: None,
            trade_dispute: false,
            tulrca_immunity: false,
        };

        let result = EconomicTortAnalyzer::analyze_inducing_breach(&facts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TortError::NoIntentionToHarm { .. })));
    }

    #[test]
    fn test_unlawful_means_tort() {
        let facts = UnlawfulMeansFacts {
            means_type: UnlawfulMeansType::TortAgainstThirdParty,
            means_description: "Deceit of third party".to_string(),
            actionable_by_third_party: true,
            gives_cause_of_action: true,
            intended_loss: true,
            harm_as_end: true,
            harm_as_means: false,
            loss_suffered: true,
            loss_type: EconomicLossType::DamageToBusiness,
            loss_amount: Some(75000.0),
            loss_caused_by_unlawful_means: true,
            claimant_dealt_with_third_party: true,
            dealing_affected: true,
        };

        let result = EconomicTortAnalyzer::analyze_unlawful_means(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
    }

    #[test]
    fn test_unlawful_means_obg_requirement() {
        let facts = UnlawfulMeansFacts {
            means_type: UnlawfulMeansType::BreachOfContractWithThirdParty,
            means_description: "Breach of contract".to_string(),
            actionable_by_third_party: false, // Not independently actionable
            gives_cause_of_action: false,
            intended_loss: true,
            harm_as_end: true,
            harm_as_means: false,
            loss_suffered: true,
            loss_type: EconomicLossType::LossOfProfit,
            loss_amount: Some(25000.0),
            loss_caused_by_unlawful_means: true,
            claimant_dealt_with_third_party: true,
            dealing_affected: true,
        };

        let result = EconomicTortAnalyzer::analyze_unlawful_means(&facts);
        assert!(result.is_err());
        // OBG requires means to be independently actionable
        assert!(matches!(result, Err(TortError::UnlawfulMeansFails { .. })));
    }

    #[test]
    fn test_unlawful_means_conspiracy() {
        let facts = ConspiracyFacts {
            conspiracy_type: ConspiracyType::UnlawfulMeans,
            conspirators: vec!["Defendant A".to_string(), "Defendant B".to_string()],
            agreement_exists: true,
            agreement_nature: "Agreement to harm claimant's business".to_string(),
            unlawful_means: Some(UnlawfulMeansFact {
                means_type: UnlawfulMeansType::Fraud,
                description: "Fraudulent statements to customers".to_string(),
                actionable: true,
            }),
            predominant_purpose_to_injure: false,
            injury_intended: true,
            self_interest_justifies: false,
            legitimate_interest: false,
            interest_description: None,
            proportionate: false,
            damage_suffered: true,
            damage_type: EconomicLossType::LossOfCustomers,
            damage_amount: Some(200000.0),
            caused_by_conspiracy: true,
            trade_dispute: false,
            tulrca_immunity: false,
        };

        let result = EconomicTortAnalyzer::analyze_conspiracy(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
    }

    #[test]
    fn test_lawful_means_conspiracy_self_interest() {
        // In lawful means conspiracy, when self-interest justifies the combination,
        // there is no tort liability - Lonrho v Shell Petroleum
        let facts = ConspiracyFacts {
            conspiracy_type: ConspiracyType::LawfulMeans,
            conspirators: vec!["Competitor A".to_string(), "Competitor B".to_string()],
            agreement_exists: true,
            agreement_nature: "Joint marketing campaign".to_string(),
            unlawful_means: None,
            predominant_purpose_to_injure: true,
            injury_intended: true,
            self_interest_justifies: true, // Self-interest defence
            legitimate_interest: true,
            interest_description: Some("Legitimate business competition".to_string()),
            proportionate: true,
            damage_suffered: true,
            damage_type: EconomicLossType::LossOfCustomers,
            damage_amount: Some(50000.0),
            caused_by_conspiracy: true,
            trade_dispute: false,
            tulrca_immunity: false,
        };

        let result = EconomicTortAnalyzer::analyze_conspiracy(&facts);
        // Self-interest justifies means no tort liability, so error is returned
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(TortError::ConspiracyFails { missing_element, .. })
                if missing_element.contains("Self-interest")
        ));
    }

    #[test]
    fn test_inducing_breach_trade_union_immunity() {
        let facts = InducingBreachFacts {
            contract_parties: vec!["Employer".to_string(), "Employee".to_string()],
            contract_type: ContractType::Employment,
            valid_contract: true,
            knew_of_contract: true,
            knew_of_terms: true,
            intended_breach: true,
            breach_as_end: false,
            intended_harm_to_claimant: true,
            inducement_method: InducementMethod::Persuasion,
            acted_on_will: true,
            inducement_effective: true,
            breach_occurred: true,
            breach_nature: "Industrial action".to_string(),
            breach_caused_by_defendant: true,
            loss_suffered: true,
            loss_type: EconomicLossType::LossOfProfit,
            loss_amount: Some(100000.0),
            loss_caused_by_breach: true,
            justification: None,
            trade_dispute: true,
            tulrca_immunity: true, // Trade union immunity applies
        };

        let result = EconomicTortAnalyzer::analyze_inducing_breach(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.claim_succeeds); // Trade union immunity
        assert!(
            analysis.defences.iter().any(|d| matches!(
                d.defence_type,
                EconomicDefenceType::TradeUnionImmunity
            ) && d.applies)
        );
    }
}
