//! UK Defamation Law
//!
//! This module implements defamation law under:
//! - Defamation Act 2013
//! - Common law principles
//!
//! Key cases:
//! - Lachaux v Independent Print \[2019\] UKSC 27 (serious harm)
//! - Stocker v Stocker \[2019\] UKSC 17 (meaning)
//! - Serafin v Malkiewicz \[2020\] UKSC 23 (honest opinion)

use serde::{Deserialize, Serialize};

use super::error::TortError;

// ============================================================================
// Core Types for Defamation
// ============================================================================

/// Type of defamatory publication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationType {
    /// Libel - permanent form (written, broadcast, etc.)
    Libel,
    /// Slander - transient form (spoken)
    Slander,
}

/// Type of claimant in defamation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimantType {
    /// Natural person
    Individual,
    /// Company (must show serious financial loss under s.1(2))
    Company,
    /// Charity
    Charity,
    /// Public figure
    PublicFigure,
    /// Government body (cannot sue - Derbyshire CC v Times)
    GovernmentBody,
}

/// Type of statement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementType {
    /// Statement of fact
    Fact,
    /// Statement of opinion
    Opinion,
    /// Mixed fact and opinion
    Mixed,
}

/// Publication medium
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationMedium {
    /// Printed publication (newspaper, book, etc.)
    Print,
    /// Online publication (website, social media)
    Online,
    /// Broadcast (TV, radio)
    Broadcast,
    /// Social media post
    SocialMedia,
    /// Email
    Email,
    /// Spoken words
    Spoken,
    /// Letter
    Letter,
    /// Other
    Other(String),
}

/// Role in publication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublisherRole {
    /// Author/primary publisher
    Author,
    /// Editor
    Editor,
    /// Commercial publisher
    CommercialPublisher,
    /// Website operator
    WebsiteOperator,
    /// Secondary publisher (e.g., newsagent)
    SecondaryPublisher,
    /// Platform/intermediary
    Platform,
}

// ============================================================================
// Defamation Claim Analysis
// ============================================================================

/// Full defamation claim analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefamationClaimAnalysis {
    /// Defamation type (libel/slander)
    pub defamation_type: DefamationType,
    /// Claimant analysis
    pub claimant: ClaimantAnalysis,
    /// Statement analysis
    pub statement: StatementAnalysis,
    /// Publication analysis
    pub publication: PublicationAnalysis,
    /// Serious harm analysis (s.1 DA 2013)
    pub serious_harm: SeriousHarmAnalysis,
    /// Defences
    pub defences: Vec<DefamationDefence>,
    /// Remedies
    pub remedies: Vec<DefamationRemedy>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Claimant analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimantAnalysis {
    /// Claimant name
    pub name: String,
    /// Claimant type
    pub claimant_type: ClaimantType,
    /// Is claimant identifiable from publication?
    pub identifiable: bool,
    /// Can claimant sue?
    pub can_sue: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Statement analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatementAnalysis {
    /// The statement in question
    pub statement: String,
    /// Meaning of statement (Stocker v Stocker)
    pub meaning: MeaningAnalysis,
    /// Is statement defamatory at common law?
    pub defamatory_at_common_law: bool,
    /// Does it refer to claimant?
    pub refers_to_claimant: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Meaning analysis (Chase levels)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeaningAnalysis {
    /// Natural and ordinary meaning
    pub natural_meaning: String,
    /// Innuendo meaning (if any)
    pub innuendo_meaning: Option<InuendoMeaning>,
    /// Chase level (1 = guilty, 2 = reasonable grounds, 3 = grounds for investigation)
    pub chase_level: Option<ChaseLevel>,
    /// Statement type
    pub statement_type: StatementType,
    /// Reasoning (reasonable reader test)
    pub reasoning: String,
}

/// Innuendo meaning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InuendoMeaning {
    /// Type of innuendo
    pub innuendo_type: InnuendoType,
    /// Extrinsic facts known
    pub extrinsic_facts: String,
    /// Meaning derived
    pub meaning: String,
}

/// Type of innuendo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InnuendoType {
    /// False/popular innuendo (extended meaning, no extrinsic facts)
    FalseInnuendo,
    /// True/legal innuendo (requires extrinsic facts)
    TrueInnuendo,
}

/// Chase levels of meaning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChaseLevel {
    /// Level 1: Claimant is guilty of act
    Guilty,
    /// Level 2: Reasonable grounds to suspect claimant
    ReasonableGrounds,
    /// Level 3: Grounds for investigation
    GroundsForInvestigation,
}

/// Publication analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationAnalysis {
    /// Medium of publication
    pub medium: PublicationMedium,
    /// Publisher role
    pub publisher_role: PublisherRole,
    /// Was there publication to third party?
    pub published_to_third_party: bool,
    /// Number of publishees (extent)
    pub extent_of_publication: PublicationExtent,
    /// Is defendant the publisher?
    pub defendant_is_publisher: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Extent of publication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationExtent {
    /// Single person
    Single,
    /// Small group (<10)
    SmallGroup,
    /// Limited group (10-100)
    LimitedGroup,
    /// Large audience (100-10000)
    LargeAudience,
    /// Mass publication (>10000)
    MassPublication,
}

/// Serious harm analysis (s.1 Defamation Act 2013)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriousHarmAnalysis {
    /// Type of harm
    pub harm_type: HarmType,
    /// Evidence of serious harm
    pub evidence: Vec<HarmEvidence>,
    /// For companies: serious financial loss (s.1(2))
    pub serious_financial_loss: Option<FinancialLossAnalysis>,
    /// Serious harm threshold met? (Lachaux)
    pub serious_harm_met: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of harm to reputation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmType {
    /// Harm to reputation in general community
    GeneralReputation,
    /// Harm to professional reputation
    ProfessionalReputation,
    /// Harm to business reputation
    BusinessReputation,
    /// Social harm (ostracism, etc.)
    SocialHarm,
}

/// Evidence of harm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmEvidence {
    /// Type of evidence
    pub evidence_type: HarmEvidenceType,
    /// Description
    pub description: String,
    /// Strength
    pub strength: EvidenceStrength,
}

/// Type of harm evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmEvidenceType {
    /// Witness evidence of reputation damage
    WitnessEvidence,
    /// Loss of clients/business
    LossOfBusiness,
    /// Loss of employment/opportunities
    LossOfOpportunities,
    /// Social ostracism
    SocialOstracism,
    /// Distress and hurt feelings
    DistressAndHurtFeelings,
    /// Evidence from grapevine effect
    GrapevineEffect,
}

/// Evidence strength
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStrength {
    /// Weak
    Weak,
    /// Moderate
    Moderate,
    /// Strong
    Strong,
}

/// Financial loss analysis for companies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialLossAnalysis {
    /// Evidence of financial loss
    pub evidence: Vec<String>,
    /// Amount of loss (if quantifiable)
    pub amount: Option<f64>,
    /// Is loss serious?
    pub serious: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Defences
// ============================================================================

/// Defence to defamation claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefamationDefence {
    /// Type of defence
    pub defence_type: DefamationDefenceType,
    /// Analysis
    pub analysis: DefenceAnalysis,
    /// Does defence apply?
    pub applies: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Types of defence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationDefenceType {
    /// Truth (s.2 DA 2013)
    Truth,
    /// Honest opinion (s.3 DA 2013)
    HonestOpinion,
    /// Public interest (s.4 DA 2013)
    PublicInterest,
    /// Absolute privilege
    AbsolutePrivilege,
    /// Qualified privilege (common law)
    QualifiedPrivilege,
    /// Qualified privilege (s.6 DA 2013 - peer-reviewed journals)
    QualifiedPrivilegePeerReview,
    /// Qualified privilege (s.7 DA 2013 - reports)
    QualifiedPrivilegeReports,
    /// Website operator (s.5 DA 2013)
    WebsiteOperator,
    /// Innocent dissemination
    InnocentDissemination,
    /// Offer of amends (ss.2-4 Defamation Act 1996)
    OfferOfAmends,
    /// Consent
    Consent,
    /// Limitation (1 year under s.4A LA 1980)
    Limitation,
}

/// Defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DefenceAnalysis {
    /// Truth defence analysis
    Truth(TruthAnalysis),
    /// Honest opinion analysis
    HonestOpinion(HonestOpinionAnalysis),
    /// Public interest analysis
    PublicInterest(PublicInterestAnalysis),
    /// Privilege analysis
    Privilege(PrivilegeAnalysis),
    /// Website operator analysis
    WebsiteOperator(WebsiteOperatorAnalysis),
    /// Generic analysis
    Generic(GenericDefenceAnalysis),
}

/// Truth defence analysis (s.2 DA 2013)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TruthAnalysis {
    /// Imputation(s) being defended
    pub imputations: Vec<String>,
    /// Evidence of truth
    pub evidence: Vec<String>,
    /// Are imputations substantially true?
    pub substantially_true: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Honest opinion defence analysis (s.3 DA 2013)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HonestOpinionAnalysis {
    /// s.3(2): Statement complained of was opinion
    pub is_opinion: bool,
    /// s.3(3): Statement indicated basis of opinion
    pub indicated_basis: bool,
    /// s.3(4): Honest person could hold opinion on basis of fact
    pub honest_person_could_hold: bool,
    /// s.3(4)(a): Fact existed at time of publication
    pub fact_existed: bool,
    /// s.3(4)(b): Privileged statement fact existed
    pub privileged_statement: bool,
    /// s.3(5): Did defendant not hold opinion? (defeats defence)
    pub defendant_did_not_hold: bool,
    /// All requirements met?
    pub requirements_met: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Public interest defence analysis (s.4 DA 2013)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicInterestAnalysis {
    /// s.4(1)(a): Statement on matter of public interest
    pub public_interest_matter: bool,
    /// Description of public interest
    pub public_interest_description: String,
    /// s.4(1)(b): Defendant reasonably believed publication in public interest
    pub reasonable_belief: bool,
    /// s.4(2): Factors considered
    pub factors_considered: Vec<PublicInterestFactor>,
    /// s.4(3): Reportage?
    pub reportage: bool,
    /// Requirements met?
    pub requirements_met: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Factors in public interest assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInterestFactor {
    /// Seriousness of allegation
    SeriousnessOfAllegation,
    /// Nature of information
    NatureOfInformation,
    /// Source of information
    SourceOfInformation,
    /// Steps taken to verify
    StepsToVerify,
    /// Status of information
    StatusOfInformation,
    /// Urgency of publication
    Urgency,
    /// Claimant's response sought
    ClaimantResponseSought,
    /// Editorial discretion
    EditorialDiscretion,
    /// Tone of publication
    ToneOfPublication,
}

/// Privilege analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeAnalysis {
    /// Type of privilege
    pub privilege_type: PrivilegeType,
    /// Context of publication
    pub context: String,
    /// Was there malice? (defeats qualified privilege)
    pub malice: bool,
    /// Is privilege established?
    pub privilege_established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of privilege
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivilegeType {
    /// Parliamentary proceedings (Article 9 Bill of Rights)
    Parliamentary,
    /// Judicial proceedings
    Judicial,
    /// Reports of court proceedings (s.14 DA 1996)
    CourtReports,
    /// Communications between solicitor and client
    LegalAdvice,
    /// Duty/interest qualified privilege
    DutyInterest,
    /// Peer-reviewed journal (s.6 DA 2013)
    PeerReviewedJournal,
    /// Reports of public proceedings (Sch 1 DA 1996)
    PublicProceedings,
}

/// Website operator analysis (s.5 DA 2013)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebsiteOperatorAnalysis {
    /// Is defendant a website operator?
    pub is_website_operator: bool,
    /// Is statement posted by author (not operator)?
    pub posted_by_author: bool,
    /// s.5(3): Can claimant identify author?
    pub can_identify_author: bool,
    /// s.5(3): Has claimant been given notice?
    pub notice_given: bool,
    /// s.5(3): Operator response compliant with regulations?
    pub complied_with_regulations: bool,
    /// s.5(11): Did operator act with malice?
    pub acted_with_malice: bool,
    /// Defence applies?
    pub defence_applies: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Generic defence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericDefenceAnalysis {
    /// Description
    pub description: String,
    /// Evidence
    pub evidence: Vec<String>,
    /// Defence established?
    pub established: bool,
}

// ============================================================================
// Remedies
// ============================================================================

/// Remedy for defamation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefamationRemedy {
    /// Type of remedy
    pub remedy_type: DefamationRemedyType,
    /// Appropriate?
    pub appropriate: bool,
    /// Quantum (for damages)
    pub quantum: Option<f64>,
    /// Reasoning
    pub reasoning: String,
}

/// Types of remedy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationRemedyType {
    /// General damages (for harm to reputation)
    GeneralDamages,
    /// Special damages (proved pecuniary loss)
    SpecialDamages,
    /// Aggravated damages
    AggravatedDamages,
    /// Exemplary damages (rare)
    ExemplaryDamages,
    /// Injunction
    Injunction,
    /// Declaration of falsity (s.12 DA 2013)
    DeclarationOfFalsity,
    /// Summary disposal order for retraction
    SummaryDisposal,
    /// Publication of correction
    PublicationOfCorrection,
}

// ============================================================================
// Defamation Analyzer
// ============================================================================

/// Analyzer for defamation claims
#[derive(Debug, Clone)]
pub struct DefamationAnalyzer;

impl DefamationAnalyzer {
    /// Analyze a defamation claim
    pub fn analyze(facts: &DefamationFacts) -> Result<DefamationClaimAnalysis, TortError> {
        // 1. Claimant analysis
        let claimant = Self::analyze_claimant(facts);
        if !claimant.can_sue {
            return Err(TortError::InvalidParties {
                issue: claimant.reasoning.clone(),
            });
        }

        // 2. Statement analysis
        let statement = Self::analyze_statement(facts);
        if !statement.defamatory_at_common_law {
            return Err(TortError::NotDefamatory {
                reason: statement.reasoning.clone(),
            });
        }

        // 3. Publication analysis
        let publication = Self::analyze_publication(facts);
        if !publication.defendant_is_publisher {
            return Err(TortError::NotPublisher {
                role: format!("{:?}", publication.publisher_role),
            });
        }

        // 4. Serious harm analysis (s.1 DA 2013)
        let serious_harm = Self::analyze_serious_harm(facts, &claimant);
        if !serious_harm.serious_harm_met {
            return Err(TortError::NoSeriousHarm {
                harm_level: serious_harm.reasoning.clone(),
            });
        }

        // 5. Analyze defences
        let defences = Self::analyze_defences(facts, &statement);

        // 6. Check if any complete defence applies
        let defence_succeeds = defences.iter().any(|d| d.applies);

        // 7. Determine remedies
        let remedies = if !defence_succeeds {
            Self::determine_remedies(facts, &serious_harm)
        } else {
            Vec::new()
        };

        let claim_succeeds = !defence_succeeds;

        Ok(DefamationClaimAnalysis {
            defamation_type: facts.defamation_type.clone(),
            claimant,
            statement,
            publication,
            serious_harm,
            defences: defences.clone(),
            remedies,
            claim_succeeds,
            reasoning: if claim_succeeds {
                "Defamation claim succeeds - no defence applies".to_string()
            } else {
                format!(
                    "Claim fails - defence of {:?} applies",
                    defences.iter().find(|d| d.applies).map(|d| &d.defence_type)
                )
            },
        })
    }

    fn analyze_claimant(facts: &DefamationFacts) -> ClaimantAnalysis {
        let can_sue = match facts.claimant_type {
            ClaimantType::GovernmentBody => false, // Derbyshire CC v Times
            ClaimantType::Company => facts.company_financial_loss_evidence.is_some(),
            _ => facts.claimant_identifiable,
        };

        ClaimantAnalysis {
            name: facts.claimant_name.clone(),
            claimant_type: facts.claimant_type.clone(),
            identifiable: facts.claimant_identifiable,
            can_sue,
            reasoning: match facts.claimant_type {
                ClaimantType::GovernmentBody => {
                    "Government bodies cannot sue in defamation (Derbyshire CC v Times)".to_string()
                }
                ClaimantType::Company => {
                    if facts.company_financial_loss_evidence.is_some() {
                        "Company with evidence of serious financial loss can sue (s.1(2) DA 2013)"
                            .to_string()
                    } else {
                        "Company must show serious financial loss under s.1(2) DA 2013".to_string()
                    }
                }
                _ => {
                    if facts.claimant_identifiable {
                        "Claimant identifiable from publication".to_string()
                    } else {
                        "Claimant not identifiable from publication".to_string()
                    }
                }
            },
        }
    }

    fn analyze_statement(facts: &DefamationFacts) -> StatementAnalysis {
        let meaning = MeaningAnalysis {
            natural_meaning: facts.natural_meaning.clone(),
            innuendo_meaning: facts.innuendo_meaning.clone(),
            chase_level: facts.chase_level.clone(),
            statement_type: facts.statement_type.clone(),
            reasoning: "Meaning determined by reasonable reader test (Stocker v Stocker)"
                .to_string(),
        };

        let defamatory = facts.would_lower_estimation || facts.would_cause_shunning;

        StatementAnalysis {
            statement: facts.statement.clone(),
            meaning,
            defamatory_at_common_law: defamatory,
            refers_to_claimant: facts.claimant_identifiable,
            reasoning: if defamatory {
                "Statement would tend to lower claimant in estimation of right-thinking members of society".to_string()
            } else {
                "Statement not defamatory at common law".to_string()
            },
        }
    }

    fn analyze_publication(facts: &DefamationFacts) -> PublicationAnalysis {
        let defendant_is_publisher = facts.defendant_published || facts.defendant_republished;

        PublicationAnalysis {
            medium: facts.publication_medium.clone(),
            publisher_role: facts.publisher_role.clone(),
            published_to_third_party: facts.published_to_third_party,
            extent_of_publication: facts.publication_extent.clone(),
            defendant_is_publisher,
            reasoning: if defendant_is_publisher {
                "Defendant is publisher of statement".to_string()
            } else {
                "Defendant is not the publisher".to_string()
            },
        }
    }

    fn analyze_serious_harm(
        facts: &DefamationFacts,
        claimant: &ClaimantAnalysis,
    ) -> SeriousHarmAnalysis {
        let mut evidence = Vec::new();

        for ev in &facts.harm_evidence {
            evidence.push(HarmEvidence {
                evidence_type: ev.0.clone(),
                description: ev.1.clone(),
                strength: EvidenceStrength::Moderate,
            });
        }

        let financial_loss = if matches!(claimant.claimant_type, ClaimantType::Company) {
            facts
                .company_financial_loss_evidence
                .as_ref()
                .map(|e| FinancialLossAnalysis {
                    evidence: e.clone(),
                    amount: facts.company_financial_loss_amount,
                    serious: facts
                        .company_financial_loss_amount
                        .is_some_and(|a| a > 10000.0),
                    reasoning: "Company must show serious financial loss (s.1(2) DA 2013)"
                        .to_string(),
                })
        } else {
            None
        };

        // Lachaux factors
        let extent_serious = matches!(
            facts.publication_extent,
            PublicationExtent::LargeAudience | PublicationExtent::MassPublication
        );
        let allegation_serious = matches!(
            facts.chase_level,
            Some(ChaseLevel::Guilty) | Some(ChaseLevel::ReasonableGrounds)
        );
        let evidence_of_harm = !evidence.is_empty();

        let serious_harm_met = if matches!(claimant.claimant_type, ClaimantType::Company) {
            financial_loss.as_ref().is_some_and(|f| f.serious)
        } else {
            (extent_serious && allegation_serious) || evidence_of_harm
        };

        SeriousHarmAnalysis {
            harm_type: facts.harm_type.clone(),
            evidence,
            serious_financial_loss: financial_loss,
            serious_harm_met,
            reasoning: if serious_harm_met {
                "Serious harm threshold met (Lachaux v Independent Print)".to_string()
            } else {
                "Serious harm not established - s.1 DA 2013 threshold not met".to_string()
            },
        }
    }

    fn analyze_defences(
        facts: &DefamationFacts,
        statement: &StatementAnalysis,
    ) -> Vec<DefamationDefence> {
        let mut defences = Vec::new();

        // Truth (s.2 DA 2013)
        if let Some(truth) = &facts.truth_defence {
            let substantially_true = truth.evidence.len() >= 2;
            defences.push(DefamationDefence {
                defence_type: DefamationDefenceType::Truth,
                analysis: DefenceAnalysis::Truth(TruthAnalysis {
                    imputations: truth.imputations.clone(),
                    evidence: truth.evidence.clone(),
                    substantially_true,
                    reasoning: if substantially_true {
                        "Imputations substantially true (s.2 DA 2013)".to_string()
                    } else {
                        "Insufficient evidence of substantial truth".to_string()
                    },
                }),
                applies: substantially_true,
                reasoning: if substantially_true {
                    "Truth defence succeeds under s.2 DA 2013".to_string()
                } else {
                    "Truth defence fails - not substantially true".to_string()
                },
            });
        }

        // Honest opinion (s.3 DA 2013)
        if let Some(ho) = &facts.honest_opinion_defence {
            let is_opinion = matches!(statement.meaning.statement_type, StatementType::Opinion);
            let requirements_met = is_opinion && ho.indicated_basis && ho.honest_person_could_hold;

            defences.push(DefamationDefence {
                defence_type: DefamationDefenceType::HonestOpinion,
                analysis: DefenceAnalysis::HonestOpinion(HonestOpinionAnalysis {
                    is_opinion,
                    indicated_basis: ho.indicated_basis,
                    honest_person_could_hold: ho.honest_person_could_hold,
                    fact_existed: ho.fact_existed,
                    privileged_statement: ho.privileged_statement,
                    defendant_did_not_hold: ho.defendant_did_not_hold,
                    requirements_met,
                    reasoning: if requirements_met {
                        "Honest opinion defence requirements met (s.3 DA 2013)".to_string()
                    } else {
                        "Honest opinion requirements not satisfied".to_string()
                    },
                }),
                applies: requirements_met && !ho.defendant_did_not_hold,
                reasoning: if requirements_met && !ho.defendant_did_not_hold {
                    "Honest opinion defence succeeds (Serafin v Malkiewicz)".to_string()
                } else if ho.defendant_did_not_hold {
                    "Defence defeated - defendant did not hold opinion".to_string()
                } else {
                    "Honest opinion defence fails".to_string()
                },
            });
        }

        // Public interest (s.4 DA 2013)
        if let Some(pi) = &facts.public_interest_defence {
            let requirements_met = pi.public_interest_matter && pi.reasonable_belief;

            defences.push(DefamationDefence {
                defence_type: DefamationDefenceType::PublicInterest,
                analysis: DefenceAnalysis::PublicInterest(PublicInterestAnalysis {
                    public_interest_matter: pi.public_interest_matter,
                    public_interest_description: pi.description.clone(),
                    reasonable_belief: pi.reasonable_belief,
                    factors_considered: pi.factors.clone(),
                    reportage: pi.reportage,
                    requirements_met,
                    reasoning: if requirements_met {
                        "Publication on matter of public interest with reasonable belief"
                            .to_string()
                    } else {
                        "Public interest requirements not met".to_string()
                    },
                }),
                applies: requirements_met,
                reasoning: if requirements_met {
                    "Public interest defence succeeds (s.4 DA 2013)".to_string()
                } else {
                    "Public interest defence fails".to_string()
                },
            });
        }

        // Privilege
        if let Some(priv_facts) = &facts.privilege_defence {
            let is_absolute = matches!(
                priv_facts.privilege_type,
                PrivilegeType::Parliamentary | PrivilegeType::Judicial
            );
            let applies = is_absolute || (priv_facts.privilege_established && !priv_facts.malice);

            defences.push(DefamationDefence {
                defence_type: if is_absolute {
                    DefamationDefenceType::AbsolutePrivilege
                } else {
                    DefamationDefenceType::QualifiedPrivilege
                },
                analysis: DefenceAnalysis::Privilege(PrivilegeAnalysis {
                    privilege_type: priv_facts.privilege_type.clone(),
                    context: priv_facts.context.clone(),
                    malice: priv_facts.malice,
                    privilege_established: applies,
                    reasoning: if is_absolute {
                        "Absolute privilege applies".to_string()
                    } else if applies {
                        "Qualified privilege applies - no malice".to_string()
                    } else {
                        "Privilege defeated by malice".to_string()
                    },
                }),
                applies,
                reasoning: if applies {
                    format!("{:?} defence succeeds", priv_facts.privilege_type)
                } else {
                    "Privilege defence fails".to_string()
                },
            });
        }

        // Website operator (s.5 DA 2013)
        if let Some(wo) = &facts.website_operator_defence {
            let applies = wo.is_website_operator
                && wo.posted_by_author
                && (!wo.can_identify_author || wo.complied_with_regulations)
                && !wo.acted_with_malice;

            defences.push(DefamationDefence {
                defence_type: DefamationDefenceType::WebsiteOperator,
                analysis: DefenceAnalysis::WebsiteOperator(WebsiteOperatorAnalysis {
                    is_website_operator: wo.is_website_operator,
                    posted_by_author: wo.posted_by_author,
                    can_identify_author: wo.can_identify_author,
                    notice_given: wo.notice_given,
                    complied_with_regulations: wo.complied_with_regulations,
                    acted_with_malice: wo.acted_with_malice,
                    defence_applies: applies,
                    reasoning: if applies {
                        "Website operator defence applies (s.5 DA 2013)".to_string()
                    } else {
                        "Website operator defence not available".to_string()
                    },
                }),
                applies,
                reasoning: if applies {
                    "Website operator defence succeeds".to_string()
                } else {
                    "Website operator defence fails".to_string()
                },
            });
        }

        defences
    }

    fn determine_remedies(
        facts: &DefamationFacts,
        serious_harm: &SeriousHarmAnalysis,
    ) -> Vec<DefamationRemedy> {
        let mut remedies = Vec::new();

        // General damages
        let general_quantum = match facts.publication_extent {
            PublicationExtent::MassPublication => Some(50000.0),
            PublicationExtent::LargeAudience => Some(20000.0),
            PublicationExtent::LimitedGroup => Some(10000.0),
            PublicationExtent::SmallGroup => Some(5000.0),
            PublicationExtent::Single => Some(1000.0),
        };

        remedies.push(DefamationRemedy {
            remedy_type: DefamationRemedyType::GeneralDamages,
            appropriate: true,
            quantum: general_quantum,
            reasoning: "General damages for harm to reputation".to_string(),
        });

        // Special damages if financial loss proved
        if let Some(ref fin_loss) = serious_harm.serious_financial_loss {
            remedies.push(DefamationRemedy {
                remedy_type: DefamationRemedyType::SpecialDamages,
                appropriate: true,
                quantum: fin_loss.amount,
                reasoning: "Special damages for proved financial loss".to_string(),
            });
        }

        // Injunction
        if facts.publication_ongoing {
            remedies.push(DefamationRemedy {
                remedy_type: DefamationRemedyType::Injunction,
                appropriate: true,
                quantum: None,
                reasoning: "Injunction to prevent continued publication".to_string(),
            });
        }

        // Declaration of falsity
        remedies.push(DefamationRemedy {
            remedy_type: DefamationRemedyType::DeclarationOfFalsity,
            appropriate: true,
            quantum: None,
            reasoning: "Declaration of falsity under s.12 DA 2013".to_string(),
        });

        remedies
    }
}

/// Facts for defamation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefamationFacts {
    /// Type of defamation
    pub defamation_type: DefamationType,
    /// Claimant name
    pub claimant_name: String,
    /// Claimant type
    pub claimant_type: ClaimantType,
    /// Is claimant identifiable?
    pub claimant_identifiable: bool,
    /// The statement
    pub statement: String,
    /// Statement type
    pub statement_type: StatementType,
    /// Natural meaning
    pub natural_meaning: String,
    /// Innuendo meaning
    pub innuendo_meaning: Option<InuendoMeaning>,
    /// Chase level
    pub chase_level: Option<ChaseLevel>,
    /// Would lower estimation?
    pub would_lower_estimation: bool,
    /// Would cause shunning?
    pub would_cause_shunning: bool,
    /// Publication medium
    pub publication_medium: PublicationMedium,
    /// Publisher role
    pub publisher_role: PublisherRole,
    /// Published to third party?
    pub published_to_third_party: bool,
    /// Publication extent
    pub publication_extent: PublicationExtent,
    /// Defendant published?
    pub defendant_published: bool,
    /// Defendant republished?
    pub defendant_republished: bool,
    /// Harm type
    pub harm_type: HarmType,
    /// Harm evidence
    pub harm_evidence: Vec<(HarmEvidenceType, String)>,
    /// Company financial loss evidence (for companies)
    pub company_financial_loss_evidence: Option<Vec<String>>,
    /// Company financial loss amount
    pub company_financial_loss_amount: Option<f64>,
    /// Publication ongoing?
    pub publication_ongoing: bool,
    /// Truth defence facts
    pub truth_defence: Option<TruthDefenceFacts>,
    /// Honest opinion defence facts
    pub honest_opinion_defence: Option<HonestOpinionFacts>,
    /// Public interest defence facts
    pub public_interest_defence: Option<PublicInterestFacts>,
    /// Privilege defence facts
    pub privilege_defence: Option<PrivilegeFacts>,
    /// Website operator defence facts
    pub website_operator_defence: Option<WebsiteOperatorFacts>,
}

/// Truth defence facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TruthDefenceFacts {
    /// Imputations
    pub imputations: Vec<String>,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Honest opinion facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HonestOpinionFacts {
    /// Indicated basis
    pub indicated_basis: bool,
    /// Honest person could hold
    pub honest_person_could_hold: bool,
    /// Fact existed
    pub fact_existed: bool,
    /// Privileged statement
    pub privileged_statement: bool,
    /// Defendant did not hold opinion
    pub defendant_did_not_hold: bool,
}

/// Public interest facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicInterestFacts {
    /// Public interest matter
    pub public_interest_matter: bool,
    /// Description
    pub description: String,
    /// Reasonable belief
    pub reasonable_belief: bool,
    /// Factors
    pub factors: Vec<PublicInterestFactor>,
    /// Reportage
    pub reportage: bool,
}

/// Privilege facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeFacts {
    /// Privilege type
    pub privilege_type: PrivilegeType,
    /// Context
    pub context: String,
    /// Privilege established
    pub privilege_established: bool,
    /// Malice
    pub malice: bool,
}

/// Website operator facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebsiteOperatorFacts {
    /// Is website operator
    pub is_website_operator: bool,
    /// Posted by author
    pub posted_by_author: bool,
    /// Can identify author
    pub can_identify_author: bool,
    /// Notice given
    pub notice_given: bool,
    /// Complied with regulations
    pub complied_with_regulations: bool,
    /// Acted with malice
    pub acted_with_malice: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defamation_claim_succeeds() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            claimant_name: "John Smith".to_string(),
            claimant_type: ClaimantType::Individual,
            claimant_identifiable: true,
            statement: "John Smith is a thief".to_string(),
            statement_type: StatementType::Fact,
            natural_meaning: "John Smith has committed theft".to_string(),
            innuendo_meaning: None,
            chase_level: Some(ChaseLevel::Guilty),
            would_lower_estimation: true,
            would_cause_shunning: true,
            publication_medium: PublicationMedium::Online,
            publisher_role: PublisherRole::Author,
            published_to_third_party: true,
            publication_extent: PublicationExtent::LargeAudience,
            defendant_published: true,
            defendant_republished: false,
            harm_type: HarmType::GeneralReputation,
            harm_evidence: vec![(
                HarmEvidenceType::WitnessEvidence,
                "Friends have distanced themselves".to_string(),
            )],
            company_financial_loss_evidence: None,
            company_financial_loss_amount: None,
            publication_ongoing: false,
            truth_defence: None,
            honest_opinion_defence: None,
            public_interest_defence: None,
            privilege_defence: None,
            website_operator_defence: None,
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
        assert!(analysis.serious_harm.serious_harm_met);
    }

    #[test]
    fn test_truth_defence() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            claimant_name: "John Smith".to_string(),
            claimant_type: ClaimantType::Individual,
            claimant_identifiable: true,
            statement: "John Smith was convicted of fraud".to_string(),
            statement_type: StatementType::Fact,
            natural_meaning: "John Smith is a convicted fraudster".to_string(),
            innuendo_meaning: None,
            chase_level: Some(ChaseLevel::Guilty),
            would_lower_estimation: true,
            would_cause_shunning: false,
            publication_medium: PublicationMedium::Print,
            publisher_role: PublisherRole::CommercialPublisher,
            published_to_third_party: true,
            publication_extent: PublicationExtent::MassPublication,
            defendant_published: true,
            defendant_republished: false,
            harm_type: HarmType::GeneralReputation,
            harm_evidence: vec![],
            company_financial_loss_evidence: None,
            company_financial_loss_amount: None,
            publication_ongoing: false,
            truth_defence: Some(TruthDefenceFacts {
                imputations: vec!["convicted of fraud".to_string()],
                evidence: vec![
                    "Court records".to_string(),
                    "Judgment transcript".to_string(),
                ],
            }),
            honest_opinion_defence: None,
            public_interest_defence: None,
            privilege_defence: None,
            website_operator_defence: None,
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.claim_succeeds); // Truth defence applies
        assert!(
            analysis
                .defences
                .iter()
                .any(|d| matches!(d.defence_type, DefamationDefenceType::Truth) && d.applies)
        );
    }

    #[test]
    fn test_honest_opinion_defence() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            claimant_name: "John Smith".to_string(),
            claimant_type: ClaimantType::Individual,
            claimant_identifiable: true,
            statement: "In my view, John Smith's business practices are unethical".to_string(),
            statement_type: StatementType::Opinion,
            natural_meaning: "Opinion that practices are unethical".to_string(),
            innuendo_meaning: None,
            chase_level: Some(ChaseLevel::ReasonableGrounds), // Serious allegation
            would_lower_estimation: true,
            would_cause_shunning: false,
            publication_medium: PublicationMedium::SocialMedia,
            publisher_role: PublisherRole::Author,
            published_to_third_party: true,
            publication_extent: PublicationExtent::LargeAudience, // Wide publication
            defendant_published: true,
            defendant_republished: false,
            harm_type: HarmType::ProfessionalReputation,
            harm_evidence: vec![(
                HarmEvidenceType::WitnessEvidence,
                "Reputation damaged".to_string(),
            )],
            company_financial_loss_evidence: None,
            company_financial_loss_amount: None,
            publication_ongoing: false,
            truth_defence: None,
            honest_opinion_defence: Some(HonestOpinionFacts {
                indicated_basis: true,
                honest_person_could_hold: true,
                fact_existed: true,
                privileged_statement: false,
                defendant_did_not_hold: false,
            }),
            public_interest_defence: None,
            privilege_defence: None,
            website_operator_defence: None,
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.claim_succeeds);
        assert!(
            analysis
                .defences
                .iter()
                .any(
                    |d| matches!(d.defence_type, DefamationDefenceType::HonestOpinion) && d.applies
                )
        );
    }

    #[test]
    fn test_no_serious_harm_company() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            claimant_name: "ABC Ltd".to_string(),
            claimant_type: ClaimantType::Company,
            claimant_identifiable: true,
            statement: "ABC Ltd provides poor service".to_string(),
            statement_type: StatementType::Opinion,
            natural_meaning: "Poor service provider".to_string(),
            innuendo_meaning: None,
            chase_level: None,
            would_lower_estimation: true,
            would_cause_shunning: false,
            publication_medium: PublicationMedium::Online,
            publisher_role: PublisherRole::Author,
            published_to_third_party: true,
            publication_extent: PublicationExtent::SmallGroup,
            defendant_published: true,
            defendant_republished: false,
            harm_type: HarmType::BusinessReputation,
            harm_evidence: vec![],
            company_financial_loss_evidence: None, // No financial loss evidence
            company_financial_loss_amount: None,
            publication_ongoing: false,
            truth_defence: None,
            honest_opinion_defence: None,
            public_interest_defence: None,
            privilege_defence: None,
            website_operator_defence: None,
        };

        let result = DefamationAnalyzer::analyze(&facts);
        // Company needs evidence of serious financial loss
        assert!(result.is_err());
        assert!(matches!(result, Err(TortError::InvalidParties { .. })));
    }

    #[test]
    fn test_government_body_cannot_sue() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            claimant_name: "Local Council".to_string(),
            claimant_type: ClaimantType::GovernmentBody,
            claimant_identifiable: true,
            statement: "The Council is corrupt".to_string(),
            statement_type: StatementType::Fact,
            natural_meaning: "Council is corrupt".to_string(),
            innuendo_meaning: None,
            chase_level: Some(ChaseLevel::Guilty),
            would_lower_estimation: true,
            would_cause_shunning: false,
            publication_medium: PublicationMedium::Print,
            publisher_role: PublisherRole::CommercialPublisher,
            published_to_third_party: true,
            publication_extent: PublicationExtent::MassPublication,
            defendant_published: true,
            defendant_republished: false,
            harm_type: HarmType::GeneralReputation,
            harm_evidence: vec![],
            company_financial_loss_evidence: None,
            company_financial_loss_amount: None,
            publication_ongoing: false,
            truth_defence: None,
            honest_opinion_defence: None,
            public_interest_defence: None,
            privilege_defence: None,
            website_operator_defence: None,
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.is_err());
        // Derbyshire CC v Times
    }

    #[test]
    fn test_absolute_privilege() {
        let facts = DefamationFacts {
            defamation_type: DefamationType::Libel,
            claimant_name: "John Smith".to_string(),
            claimant_type: ClaimantType::Individual,
            claimant_identifiable: true,
            statement: "Statement made in Parliament".to_string(),
            statement_type: StatementType::Fact,
            natural_meaning: "Defamatory meaning".to_string(),
            innuendo_meaning: None,
            chase_level: Some(ChaseLevel::Guilty),
            would_lower_estimation: true,
            would_cause_shunning: false,
            publication_medium: PublicationMedium::Spoken,
            publisher_role: PublisherRole::Author,
            published_to_third_party: true,
            publication_extent: PublicationExtent::LargeAudience,
            defendant_published: true,
            defendant_republished: false,
            harm_type: HarmType::GeneralReputation,
            harm_evidence: vec![(HarmEvidenceType::WitnessEvidence, "Evidence".to_string())],
            company_financial_loss_evidence: None,
            company_financial_loss_amount: None,
            publication_ongoing: false,
            truth_defence: None,
            honest_opinion_defence: None,
            public_interest_defence: None,
            privilege_defence: Some(PrivilegeFacts {
                privilege_type: PrivilegeType::Parliamentary,
                context: "Parliamentary debate".to_string(),
                privilege_established: true,
                malice: true, // Even with malice, absolute privilege applies
            }),
            website_operator_defence: None,
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.claim_succeeds); // Absolute privilege applies
    }
}
