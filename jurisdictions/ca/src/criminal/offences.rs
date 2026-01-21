//! Canada Criminal Law - Offence Analysis
//!
//! Analyzers for criminal offences under the Criminal Code of Canada.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    AssaultType, BodilyHarmLevel, CriminalDefence, DuressElements, FirstDegreeFactor, HomicideType,
    MentalDisorderElements, NecessityElements, OffenceType, SelfDefenceElements,
};

// ============================================================================
// Homicide Analysis
// ============================================================================

/// Facts for homicide analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomicideFacts {
    /// Death occurred
    pub death_occurred: bool,
    /// Accused's act caused death
    pub causation: CausationFacts,
    /// Mental state
    pub mental_state: MentalStateFacts,
    /// Whether planned and deliberate
    pub planned_deliberate: Option<PlannedDeliberateFacts>,
    /// First degree factors
    pub first_degree_factors: Vec<FirstDegreeFactor>,
    /// Provocation facts (if any)
    pub provocation: Option<ProvocationFacts>,
}

/// Causation facts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausationFacts {
    /// Accused's act
    pub act_description: String,
    /// Act was significant contributing cause
    pub significant_contributing_cause: bool,
    /// Intervening acts (if any)
    pub intervening_acts: Vec<InterveningAct>,
}

/// Intervening act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterveningAct {
    /// Description
    pub description: String,
    /// Whether reasonably foreseeable
    pub foreseeable: bool,
    /// Whether breaks chain of causation
    pub breaks_chain: bool,
}

/// Mental state facts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalStateFacts {
    /// Intended to kill
    pub intent_to_kill: bool,
    /// Intended serious bodily harm
    pub intent_serious_harm: bool,
    /// Knew death likely
    pub knew_death_likely: bool,
    /// Reckless as to death
    pub reckless_as_to_death: bool,
    /// Criminal negligence
    pub criminal_negligence: bool,
}

/// Planned and deliberate facts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedDeliberateFacts {
    /// Evidence of planning
    pub planning_evidence: Vec<String>,
    /// Evidence of deliberation
    pub deliberation_evidence: Vec<String>,
    /// Time to reflect
    pub time_to_reflect: bool,
}

/// Provocation facts (s.232)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvocationFacts {
    /// Wrongful act or insult
    pub wrongful_act_or_insult: String,
    /// Sudden (before passion cooled)
    pub sudden: bool,
    /// Would deprive ordinary person of self-control
    pub objective_test: bool,
    /// Accused actually deprived of self-control
    pub subjective_test: bool,
}

/// Result of homicide analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomicideResult {
    /// Offence type
    pub offence: Option<HomicideType>,
    /// Reasoning
    pub reasoning: String,
    /// Actus reus established
    pub actus_reus: bool,
    /// Mens rea established
    pub mens_rea: bool,
    /// Applicable defences
    pub possible_defences: Vec<CriminalDefence>,
}

/// Homicide analyzer
pub struct HomicideAnalyzer;

impl HomicideAnalyzer {
    /// Analyze homicide facts
    pub fn analyze(facts: &HomicideFacts) -> HomicideResult {
        // Check actus reus
        let actus_reus = facts.death_occurred && facts.causation.significant_contributing_cause;

        // Check if chain broken by intervening act
        let chain_broken = facts
            .causation
            .intervening_acts
            .iter()
            .any(|act| act.breaks_chain && !act.foreseeable);

        if !actus_reus || chain_broken {
            return HomicideResult {
                offence: None,
                reasoning: if chain_broken {
                    "Causation broken by unforeseeable intervening act".to_string()
                } else {
                    "Actus reus not established".to_string()
                },
                actus_reus: false,
                mens_rea: false,
                possible_defences: vec![],
            };
        }

        // Determine type based on mens rea
        let (offence, mens_rea, reasoning) = Self::classify_homicide(facts);

        // Determine possible defences
        let mut possible_defences = Vec::new();
        if facts.provocation.is_some() {
            possible_defences.push(CriminalDefence::Provocation);
        }
        possible_defences.push(CriminalDefence::SelfDefence);
        possible_defences.push(CriminalDefence::MentalDisorder);

        HomicideResult {
            offence,
            reasoning,
            actus_reus: true,
            mens_rea,
            possible_defences,
        }
    }

    /// Classify type of homicide
    fn classify_homicide(facts: &HomicideFacts) -> (Option<HomicideType>, bool, String) {
        let ms = &facts.mental_state;

        // Murder: intent to kill OR intent serious harm + know death likely
        let murder_mens_rea = ms.intent_to_kill || (ms.intent_serious_harm && ms.knew_death_likely);

        if murder_mens_rea {
            // Check provocation (reduces murder to manslaughter)
            if let Some(prov) = &facts.provocation
                && prov.sudden
                && prov.objective_test
                && prov.subjective_test
            {
                return (
                    Some(HomicideType::Manslaughter),
                    true,
                    "Murder reduced to manslaughter by provocation (s.232)".to_string(),
                );
            }

            // Check for first degree murder
            if Self::is_first_degree(facts) {
                return (
                    Some(HomicideType::FirstDegreeMurder),
                    true,
                    "First degree murder - planned and deliberate or statutory factor".to_string(),
                );
            }

            return (
                Some(HomicideType::SecondDegreeMurder),
                true,
                "Second degree murder - intent to kill or know death likely from harm".to_string(),
            );
        }

        // Manslaughter by unlawful act or criminal negligence
        if ms.criminal_negligence {
            return (
                Some(HomicideType::CriminalNegligenceDeath),
                true,
                "Criminal negligence causing death - marked departure from reasonable standard"
                    .to_string(),
            );
        }

        if ms.reckless_as_to_death {
            return (
                Some(HomicideType::Manslaughter),
                true,
                "Manslaughter - unlawful act with objective foreseeability of harm".to_string(),
            );
        }

        (
            None,
            false,
            "Insufficient mens rea for culpable homicide".to_string(),
        )
    }

    /// Determine if first degree murder
    fn is_first_degree(facts: &HomicideFacts) -> bool {
        // Planned and deliberate
        if let Some(pd) = &facts.planned_deliberate
            && pd.time_to_reflect
            && !pd.planning_evidence.is_empty()
        {
            return true;
        }

        // Statutory first degree factors
        !facts.first_degree_factors.is_empty()
    }
}

// ============================================================================
// Assault Analysis
// ============================================================================

/// Facts for assault analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssaultFacts {
    /// Force applied
    pub force_applied: bool,
    /// Type of force
    pub force_description: String,
    /// Without consent
    pub without_consent: bool,
    /// Intention to apply force
    pub intentional: bool,
    /// Bodily harm level
    pub harm_level: Option<BodilyHarmLevel>,
    /// Weapon used
    pub weapon_used: Option<String>,
    /// Victim was peace officer
    pub victim_peace_officer: bool,
    /// Sexual nature
    pub sexual_nature: bool,
}

/// Result of assault analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssaultResult {
    /// Type of assault
    pub assault_type: Option<AssaultType>,
    /// Offence classification
    pub offence_type: OffenceType,
    /// Maximum sentence (months)
    pub max_sentence_months: Option<u32>,
    /// Reasoning
    pub reasoning: String,
}

/// Assault analyzer
pub struct AssaultAnalyzer;

impl AssaultAnalyzer {
    /// Analyze assault facts
    pub fn analyze(facts: &AssaultFacts) -> AssaultResult {
        // Check basic assault elements
        if !facts.force_applied || !facts.without_consent || !facts.intentional {
            return AssaultResult {
                assault_type: None,
                offence_type: OffenceType::Summary,
                max_sentence_months: None,
                reasoning: "Elements of assault not established".to_string(),
            };
        }

        // Sexual assault
        if facts.sexual_nature {
            return Self::classify_sexual_assault(facts);
        }

        // Non-sexual assault classification
        Self::classify_assault(facts)
    }

    /// Classify sexual assault
    fn classify_sexual_assault(facts: &AssaultFacts) -> AssaultResult {
        // Aggravated sexual assault (s.273)
        if matches!(facts.harm_level, Some(BodilyHarmLevel::Grievous)) {
            return AssaultResult {
                assault_type: Some(AssaultType::AggravatedSexual),
                offence_type: OffenceType::Indictable,
                max_sentence_months: Some(999), // Life
                reasoning: "Aggravated sexual assault (s.273) - wounds, maims, disfigures"
                    .to_string(),
            };
        }

        // Sexual assault with weapon (s.272)
        if facts.weapon_used.is_some()
            || matches!(facts.harm_level, Some(BodilyHarmLevel::BodilyHarm))
        {
            return AssaultResult {
                assault_type: Some(AssaultType::SexualWithWeapon),
                offence_type: OffenceType::Indictable,
                max_sentence_months: Some(168), // 14 years
                reasoning: "Sexual assault with weapon or causing bodily harm (s.272)".to_string(),
            };
        }

        // Basic sexual assault (s.271)
        AssaultResult {
            assault_type: Some(AssaultType::Sexual),
            offence_type: OffenceType::Hybrid,
            max_sentence_months: Some(120), // 10 years
            reasoning: "Sexual assault (s.271)".to_string(),
        }
    }

    /// Classify non-sexual assault
    fn classify_assault(facts: &AssaultFacts) -> AssaultResult {
        // Aggravated assault (s.268)
        if matches!(facts.harm_level, Some(BodilyHarmLevel::Grievous)) {
            return AssaultResult {
                assault_type: Some(AssaultType::Aggravated),
                offence_type: OffenceType::Indictable,
                max_sentence_months: Some(168), // 14 years
                reasoning:
                    "Aggravated assault (s.268) - wounds, maims, disfigures or endangers life"
                        .to_string(),
            };
        }

        // Assault peace officer (s.270)
        if facts.victim_peace_officer {
            return AssaultResult {
                assault_type: Some(AssaultType::PeaceOfficer),
                offence_type: OffenceType::Hybrid,
                max_sentence_months: Some(60), // 5 years
                reasoning: "Assault peace officer (s.270)".to_string(),
            };
        }

        // Assault causing bodily harm (s.267(b))
        if matches!(
            facts.harm_level,
            Some(BodilyHarmLevel::BodilyHarm) | Some(BodilyHarmLevel::Serious)
        ) {
            return AssaultResult {
                assault_type: Some(AssaultType::CausingBodilyHarm),
                offence_type: OffenceType::Hybrid,
                max_sentence_months: Some(120), // 10 years
                reasoning: "Assault causing bodily harm (s.267(b))".to_string(),
            };
        }

        // Assault with weapon (s.267(a))
        if facts.weapon_used.is_some() {
            return AssaultResult {
                assault_type: Some(AssaultType::WithWeapon),
                offence_type: OffenceType::Hybrid,
                max_sentence_months: Some(120), // 10 years
                reasoning: "Assault with weapon (s.267(a))".to_string(),
            };
        }

        // Common assault (s.266)
        AssaultResult {
            assault_type: Some(AssaultType::Common),
            offence_type: OffenceType::Hybrid,
            max_sentence_months: Some(60), // 5 years indictable, 2 years less day summary
            reasoning: "Common assault (s.266)".to_string(),
        }
    }
}

// ============================================================================
// Defence Analysis
// ============================================================================

/// Facts for defence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenceFacts {
    /// Type of defence claimed
    pub defence_type: CriminalDefence,
    /// Self-defence elements (if applicable)
    pub self_defence: Option<SelfDefenceElements>,
    /// Necessity elements (if applicable)
    pub necessity: Option<NecessityElements>,
    /// Duress elements (if applicable)
    pub duress: Option<DuressElements>,
    /// Mental disorder elements (if applicable)
    pub mental_disorder: Option<MentalDisorderElements>,
    /// Offence charged
    pub offence_charged: String,
}

/// Result of defence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenceResult {
    /// Defence available
    pub defence_available: bool,
    /// Type of defence
    pub defence: CriminalDefence,
    /// Whether complete defence
    pub complete_defence: bool,
    /// Outcome if successful
    pub outcome: DefenceOutcome,
    /// Reasoning
    pub reasoning: String,
}

/// Defence outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenceOutcome {
    /// Acquittal
    Acquittal,
    /// Reduced charge
    ReducedCharge { from: String, to: String },
    /// Not criminally responsible (NCR)
    NotCriminallyResponsible,
    /// Defence fails
    NoDefence,
}

/// Defence analyzer
pub struct DefenceAnalyzer;

impl DefenceAnalyzer {
    /// Analyze defence
    pub fn analyze(facts: &DefenceFacts) -> DefenceResult {
        match facts.defence_type {
            CriminalDefence::SelfDefence | CriminalDefence::DefenceOfAnother => {
                Self::analyze_self_defence(facts)
            }
            CriminalDefence::Necessity => Self::analyze_necessity(facts),
            CriminalDefence::Duress => Self::analyze_duress(facts),
            CriminalDefence::MentalDisorder => Self::analyze_mental_disorder(facts),
            CriminalDefence::Provocation => Self::analyze_provocation(facts),
            _ => DefenceResult {
                defence_available: false,
                defence: facts.defence_type.clone(),
                complete_defence: false,
                outcome: DefenceOutcome::NoDefence,
                reasoning: "Defence analysis not implemented for this type".to_string(),
            },
        }
    }

    /// Analyze self-defence (s.34)
    fn analyze_self_defence(facts: &DefenceFacts) -> DefenceResult {
        let elements = match &facts.self_defence {
            Some(e) => e,
            None => {
                return DefenceResult {
                    defence_available: false,
                    defence: CriminalDefence::SelfDefence,
                    complete_defence: false,
                    outcome: DefenceOutcome::NoDefence,
                    reasoning: "No self-defence elements provided".to_string(),
                };
            }
        };

        let available = elements.reasonable_belief_threat
            && elements.defensive_purpose
            && elements.reasonable_response;

        DefenceResult {
            defence_available: available,
            defence: CriminalDefence::SelfDefence,
            complete_defence: available,
            outcome: if available {
                DefenceOutcome::Acquittal
            } else {
                DefenceOutcome::NoDefence
            },
            reasoning: if available {
                "Self-defence established under s.34: reasonable belief of threat, \
                 defensive purpose, and reasonable response"
                    .to_string()
            } else {
                let mut missing = Vec::new();
                if !elements.reasonable_belief_threat {
                    missing.push("reasonable belief of threat");
                }
                if !elements.defensive_purpose {
                    missing.push("defensive purpose");
                }
                if !elements.reasonable_response {
                    missing.push("reasonable response");
                }
                format!(
                    "Self-defence not established - missing: {}",
                    missing.join(", ")
                )
            },
        }
    }

    /// Analyze necessity (Perka)
    fn analyze_necessity(facts: &DefenceFacts) -> DefenceResult {
        let elements = match &facts.necessity {
            Some(e) => e,
            None => {
                return DefenceResult {
                    defence_available: false,
                    defence: CriminalDefence::Necessity,
                    complete_defence: false,
                    outcome: DefenceOutcome::NoDefence,
                    reasoning: "No necessity elements provided".to_string(),
                };
            }
        };

        let available =
            elements.imminent_peril && elements.no_legal_alternative && elements.proportional;

        DefenceResult {
            defence_available: available,
            defence: CriminalDefence::Necessity,
            complete_defence: available,
            outcome: if available {
                DefenceOutcome::Acquittal
            } else {
                DefenceOutcome::NoDefence
            },
            reasoning: if available {
                "Necessity established under Perka: imminent peril, no legal alternative, \
                 and proportional response"
                    .to_string()
            } else {
                let mut missing = Vec::new();
                if !elements.imminent_peril {
                    missing.push("imminent peril");
                }
                if !elements.no_legal_alternative {
                    missing.push("no legal alternative");
                }
                if !elements.proportional {
                    missing.push("proportionality");
                }
                format!(
                    "Necessity not established - missing: {}",
                    missing.join(", ")
                )
            },
        }
    }

    /// Analyze duress
    fn analyze_duress(facts: &DefenceFacts) -> DefenceResult {
        let elements = match &facts.duress {
            Some(e) => e,
            None => {
                return DefenceResult {
                    defence_available: false,
                    defence: CriminalDefence::Duress,
                    complete_defence: false,
                    outcome: DefenceOutcome::NoDefence,
                    reasoning: "No duress elements provided".to_string(),
                };
            }
        };

        // Note: Duress not available for certain offences (murder, attempted murder, etc.)
        let excluded_offences = ["murder", "attempted murder", "robbery", "kidnapping"];
        let offence_lower = facts.offence_charged.to_lowercase();
        let offence_excluded = excluded_offences.iter().any(|o| offence_lower.contains(o));

        if offence_excluded {
            return DefenceResult {
                defence_available: false,
                defence: CriminalDefence::Duress,
                complete_defence: false,
                outcome: DefenceOutcome::NoDefence,
                reasoning: format!(
                    "Duress not available for {} under s.17",
                    facts.offence_charged
                ),
            };
        }

        let available = elements.threat_death_or_harm
            && elements.present_threat
            && elements.no_escape
            && elements.proportional_response;

        DefenceResult {
            defence_available: available,
            defence: CriminalDefence::Duress,
            complete_defence: available,
            outcome: if available {
                DefenceOutcome::Acquittal
            } else {
                DefenceOutcome::NoDefence
            },
            reasoning: if available {
                "Duress established: threat of death/harm, present threat, no escape, proportional"
                    .to_string()
            } else {
                "Duress elements not all established".to_string()
            },
        }
    }

    /// Analyze mental disorder (s.16 NCR)
    fn analyze_mental_disorder(facts: &DefenceFacts) -> DefenceResult {
        let elements = match &facts.mental_disorder {
            Some(e) => e,
            None => {
                return DefenceResult {
                    defence_available: false,
                    defence: CriminalDefence::MentalDisorder,
                    complete_defence: false,
                    outcome: DefenceOutcome::NoDefence,
                    reasoning: "No mental disorder elements provided".to_string(),
                };
            }
        };

        let available = elements.mental_disorder
            && (elements.incapable_appreciating || elements.incapable_knowing_wrong);

        DefenceResult {
            defence_available: available,
            defence: CriminalDefence::MentalDisorder,
            complete_defence: available,
            outcome: if available {
                DefenceOutcome::NotCriminallyResponsible
            } else {
                DefenceOutcome::NoDefence
            },
            reasoning: if available {
                "NCR established under s.16: mental disorder rendering incapable of \
                 appreciating nature/quality of act or knowing it was wrong"
                    .to_string()
            } else {
                "Mental disorder defence not established".to_string()
            },
        }
    }

    /// Analyze provocation (partial defence - murder to manslaughter)
    fn analyze_provocation(facts: &DefenceFacts) -> DefenceResult {
        // Provocation only reduces murder to manslaughter
        let is_murder = facts.offence_charged.to_lowercase().contains("murder");

        if !is_murder {
            return DefenceResult {
                defence_available: false,
                defence: CriminalDefence::Provocation,
                complete_defence: false,
                outcome: DefenceOutcome::NoDefence,
                reasoning: "Provocation only applies to murder charges".to_string(),
            };
        }

        DefenceResult {
            defence_available: true,
            defence: CriminalDefence::Provocation,
            complete_defence: false, // Partial defence
            outcome: DefenceOutcome::ReducedCharge {
                from: "Murder".to_string(),
                to: "Manslaughter".to_string(),
            },
            reasoning:
                "Provocation (s.232) is a partial defence that reduces murder to manslaughter"
                    .to_string(),
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
    fn test_second_degree_murder() {
        let facts = HomicideFacts {
            death_occurred: true,
            causation: CausationFacts {
                act_description: "Stabbing".to_string(),
                significant_contributing_cause: true,
                intervening_acts: vec![],
            },
            mental_state: MentalStateFacts {
                intent_to_kill: true,
                intent_serious_harm: false,
                knew_death_likely: false,
                reckless_as_to_death: false,
                criminal_negligence: false,
            },
            planned_deliberate: None,
            first_degree_factors: vec![],
            provocation: None,
        };

        let result = HomicideAnalyzer::analyze(&facts);
        assert_eq!(result.offence, Some(HomicideType::SecondDegreeMurder));
        assert!(result.mens_rea);
    }

    #[test]
    fn test_first_degree_murder_planned() {
        let facts = HomicideFacts {
            death_occurred: true,
            causation: CausationFacts {
                act_description: "Shooting".to_string(),
                significant_contributing_cause: true,
                intervening_acts: vec![],
            },
            mental_state: MentalStateFacts {
                intent_to_kill: true,
                intent_serious_harm: false,
                knew_death_likely: false,
                reckless_as_to_death: false,
                criminal_negligence: false,
            },
            planned_deliberate: Some(PlannedDeliberateFacts {
                planning_evidence: vec!["Purchased weapon".to_string()],
                deliberation_evidence: vec!["Waited for victim".to_string()],
                time_to_reflect: true,
            }),
            first_degree_factors: vec![],
            provocation: None,
        };

        let result = HomicideAnalyzer::analyze(&facts);
        assert_eq!(result.offence, Some(HomicideType::FirstDegreeMurder));
    }

    #[test]
    fn test_manslaughter_provocation() {
        let facts = HomicideFacts {
            death_occurred: true,
            causation: CausationFacts {
                act_description: "Punching".to_string(),
                significant_contributing_cause: true,
                intervening_acts: vec![],
            },
            mental_state: MentalStateFacts {
                intent_to_kill: true,
                intent_serious_harm: false,
                knew_death_likely: false,
                reckless_as_to_death: false,
                criminal_negligence: false,
            },
            planned_deliberate: None,
            first_degree_factors: vec![],
            provocation: Some(ProvocationFacts {
                wrongful_act_or_insult: "Found spouse in affair".to_string(),
                sudden: true,
                objective_test: true,
                subjective_test: true,
            }),
        };

        let result = HomicideAnalyzer::analyze(&facts);
        assert_eq!(result.offence, Some(HomicideType::Manslaughter));
        assert!(result.reasoning.contains("provocation"));
    }

    #[test]
    fn test_common_assault() {
        let facts = AssaultFacts {
            force_applied: true,
            force_description: "Pushed".to_string(),
            without_consent: true,
            intentional: true,
            harm_level: Some(BodilyHarmLevel::Minor),
            weapon_used: None,
            victim_peace_officer: false,
            sexual_nature: false,
        };

        let result = AssaultAnalyzer::analyze(&facts);
        assert_eq!(result.assault_type, Some(AssaultType::Common));
    }

    #[test]
    fn test_aggravated_assault() {
        let facts = AssaultFacts {
            force_applied: true,
            force_description: "Attacked with knife".to_string(),
            without_consent: true,
            intentional: true,
            harm_level: Some(BodilyHarmLevel::Grievous),
            weapon_used: Some("Knife".to_string()),
            victim_peace_officer: false,
            sexual_nature: false,
        };

        let result = AssaultAnalyzer::analyze(&facts);
        assert_eq!(result.assault_type, Some(AssaultType::Aggravated));
        assert_eq!(result.max_sentence_months, Some(168)); // 14 years
    }

    #[test]
    fn test_self_defence_established() {
        let facts = DefenceFacts {
            defence_type: CriminalDefence::SelfDefence,
            self_defence: Some(SelfDefenceElements {
                reasonable_belief_threat: true,
                defensive_purpose: true,
                reasonable_response: true,
            }),
            necessity: None,
            duress: None,
            mental_disorder: None,
            offence_charged: "Assault".to_string(),
        };

        let result = DefenceAnalyzer::analyze(&facts);
        assert!(result.defence_available);
        assert_eq!(result.outcome, DefenceOutcome::Acquittal);
    }

    #[test]
    fn test_ncr_defence() {
        let facts = DefenceFacts {
            defence_type: CriminalDefence::MentalDisorder,
            self_defence: None,
            necessity: None,
            duress: None,
            mental_disorder: Some(MentalDisorderElements {
                mental_disorder: true,
                incapable_appreciating: true,
                incapable_knowing_wrong: false,
            }),
            offence_charged: "Murder".to_string(),
        };

        let result = DefenceAnalyzer::analyze(&facts);
        assert!(result.defence_available);
        assert_eq!(result.outcome, DefenceOutcome::NotCriminallyResponsible);
    }

    #[test]
    fn test_duress_excluded_for_murder() {
        let facts = DefenceFacts {
            defence_type: CriminalDefence::Duress,
            self_defence: None,
            necessity: None,
            duress: Some(DuressElements {
                threat_death_or_harm: true,
                present_threat: true,
                no_escape: true,
                proportional_response: true,
            }),
            mental_disorder: None,
            offence_charged: "Murder".to_string(),
        };

        let result = DefenceAnalyzer::analyze(&facts);
        assert!(!result.defence_available);
        assert!(result.reasoning.contains("not available"));
    }
}
