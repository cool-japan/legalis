//! Canada Tort Law - Types
//!
//! Core types for Canadian tort law.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::{CaseCitation, Province};

// ============================================================================
// Duty of Care
// ============================================================================

/// Stages of duty of care analysis (Anns/Cooper test)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DutyOfCareStage {
    /// Stage 1: Prima facie duty (foreseeability + proximity)
    PrimaFacie,
    /// Stage 2: Policy considerations that may negate duty
    PolicyConsiderations,
}

/// Proximity factors for duty of care
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProximityFactor {
    /// Physical proximity
    Physical,
    /// Circumstantial proximity (relationship)
    Circumstantial,
    /// Causal proximity
    Causal,
    /// Assumed responsibility
    AssumedResponsibility,
    /// Reliance
    Reliance,
    /// Representation/advice relationship
    RepresentationAdvice,
    /// Regulatory relationship
    Regulatory,
    /// Employer-employee
    Employment,
    /// Property-based
    Property,
    /// Contractual nexus
    ContractualNexus,
}

/// Categories of recognized duty relationships
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecognizedDutyCategory {
    /// Manufacturer to consumer (Donoghue v Stevenson)
    ManufacturerConsumer,
    /// Motorist to other road users
    MotoristRoadUser,
    /// Employer to employee
    EmployerEmployee,
    /// Occupier to visitor (see OLA)
    OccupierVisitor,
    /// Doctor to patient
    DoctorPatient,
    /// Lawyer to client
    LawyerClient,
    /// Financial advisor to client
    FinancialAdvisor,
    /// Regulator to regulated (limited - Cooper v Hobart)
    RegulatorRegulated,
    /// Educational institution to student
    EducationalStudent,
    /// Parent to child
    ParentChild,
    /// Host to guest (social host liability - Childs v Desormeaux)
    SocialHost,
    /// Commercial host to patron
    CommercialHost,
    /// Police to public (limited)
    PolicePublic,
    /// Novel category
    Novel { description: String },
}

/// Policy considerations that may negate duty (Stage 2)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyNegation {
    /// Indeterminate liability (floodgates)
    IndeterminateLiability,
    /// Conflict with other legal duties
    ConflictWithOtherDuties,
    /// Chilling effect on beneficial activity
    ChillingEffect,
    /// Already adequately addressed by other remedies
    OtherRemedies,
    /// Constitutional/Charter considerations
    Constitutional,
    /// Statute specifically precludes duty
    StatutoryExclusion,
    /// Public policy against imposing duty
    PublicPolicy { reason: String },
}

// ============================================================================
// Standard of Care
// ============================================================================

/// Standard of care
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardOfCare {
    /// Reasonable person (objective standard)
    ReasonablePerson,
    /// Professional standard
    Professional { profession: String },
    /// Higher standard (children in adult activity)
    Heightened { reason: String },
    /// Lower standard (emergency)
    Emergency,
    /// Statutory standard
    Statutory { statute: String, section: String },
}

/// Factors relevant to breach of standard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachFactor {
    /// Likelihood of harm
    LikelihoodOfHarm,
    /// Severity of potential harm
    SeverityOfHarm,
    /// Cost of precautions
    CostOfPrecautions,
    /// Social utility of defendant's conduct
    SocialUtility,
    /// Industry practice/custom
    IndustryPractice,
    /// Statutory compliance (not conclusive)
    StatutoryCompliance,
    /// Expert evidence
    ExpertEvidence,
}

// ============================================================================
// Causation
// ============================================================================

/// Causation test
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausationTest {
    /// But-for test (default)
    ButFor,
    /// Material contribution to risk (Clements)
    MaterialContribution,
    /// Material increase in risk (McGhee/Fairchild exception)
    MaterialIncreaseInRisk,
}

/// Remoteness test
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemotenessTest {
    /// Reasonable foreseeability of type of harm
    ReasonableForeseeability,
    /// Direct consequences
    DirectConsequences,
}

/// Intervening cause analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterveningCause {
    /// Act of third party
    ThirdPartyAct,
    /// Act of claimant
    ClaimantAct,
    /// Natural event
    NaturalEvent,
    /// Medical treatment (generally doesn't break chain)
    MedicalTreatment,
}

// ============================================================================
// Defences
// ============================================================================

/// Defence to negligence claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegligenceDefence {
    /// Contributory negligence (apportionment)
    ContributoryNegligence { percentage: u8 },
    /// Voluntary assumption of risk (volenti)
    VolentiNonFitInjuria,
    /// Ex turpi causa (illegality)
    ExTurpiCausa,
    /// Inevitable accident
    InevitableAccident,
    /// Statutory authority
    StatutoryAuthority,
    /// Limitation period expired
    LimitationPeriod,
    /// Waiver/exclusion clause
    WaiverExclusion,
}

// ============================================================================
// Damages
// ============================================================================

/// Type of damages in tort
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortDamages {
    /// Compensatory - pecuniary
    Pecuniary,
    /// Compensatory - non-pecuniary (pain and suffering)
    NonPecuniary,
    /// Future care costs
    FutureCare,
    /// Loss of earning capacity
    LossOfEarningCapacity,
    /// Loss of housekeeping capacity
    LossOfHousekeeping,
    /// Cost of future care
    CostOfFutureCare,
    /// Aggravated damages
    Aggravated,
    /// Punitive/exemplary damages
    Punitive,
    /// Family Law Act claims (derivative)
    FamilyLawAct,
}

/// Non-pecuniary damages cap (trilogy cases)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonPecuniaryCap {
    /// Base amount (1978 dollars)
    pub base_amount_1978: i64,
    /// Current indexed amount
    pub current_indexed: Option<i64>,
    /// Whether catastrophic injury
    pub is_catastrophic: bool,
}

impl Default for NonPecuniaryCap {
    fn default() -> Self {
        Self {
            base_amount_1978: 10_000_000, // $100,000 in 1978 (in cents)
            current_indexed: None,
            is_catastrophic: false,
        }
    }
}

// ============================================================================
// Occupiers' Liability
// ============================================================================

/// Status of entrant (common law)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommonLawEntrantStatus {
    /// Invitee (business benefit)
    Invitee,
    /// Licensee (permission, no benefit)
    Licensee,
    /// Trespasser
    Trespasser,
    /// Child trespasser (allurement doctrine)
    ChildTrespasser,
}

/// Duty under Occupiers' Liability Act
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OlaDuty {
    /// Common duty of care (to visitors)
    CommonDuty,
    /// Limited duty to trespassers
    TrespasserDuty,
    /// Duty where activity on premises
    ActivityDuty,
    /// Duty to independent contractors
    ContractorDuty,
}

/// Type of hazard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HazardType {
    /// Unusual danger
    UnusualDanger,
    /// Known danger
    KnownDanger,
    /// Obvious danger
    ObviousDanger,
    /// Concealed danger
    ConcealedDanger,
    /// Activity-created danger
    ActivityCreated,
}

// ============================================================================
// Defamation
// ============================================================================

/// Type of defamation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationType {
    /// Libel (permanent form)
    Libel,
    /// Slander (transitory form)
    Slander,
    /// Internet/online defamation
    OnlineDefamation,
}

/// Defence to defamation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationDefence {
    /// Truth/justification
    Truth,
    /// Absolute privilege
    AbsolutePrivilege,
    /// Qualified privilege
    QualifiedPrivilege,
    /// Fair comment (honest opinion on public interest)
    FairComment,
    /// Responsible communication (Grant v Torstar)
    ResponsibleCommunication,
    /// Innocent dissemination
    InnocentDissemination,
    /// Consent
    Consent,
    /// Limitation period
    LimitationPeriod,
}

/// Responsible communication factors (Grant v Torstar)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsibleCommunicationFactors {
    /// Whether matter of public interest
    pub public_interest: bool,
    /// Seriousness of allegation
    pub seriousness: String,
    /// Urgency of matter
    pub urgency: bool,
    /// Status and reliability of source
    pub source_reliability: String,
    /// Whether claimant's side sought
    pub claimant_side_sought: bool,
    /// Whether inclusion of defamatory statement justifiable
    pub inclusion_justifiable: bool,
    /// Overall responsible journalism
    pub responsible_journalism: bool,
}

// ============================================================================
// Nuisance
// ============================================================================

/// Type of nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceType {
    /// Private nuisance (interference with land use)
    Private,
    /// Public nuisance (interference with public right)
    Public,
}

/// Factors in private nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceFactor {
    /// Severity of interference
    Severity,
    /// Duration and frequency
    Duration,
    /// Character of neighbourhood
    NeighbourhoodCharacter,
    /// Sensitivity of claimant
    ClaimantSensitivity,
    /// Utility of defendant's conduct
    DefendantUtility,
    /// Whether malicious
    Malice,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Canadian tort law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TortCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of tort law
    pub area: TortArea,
}

/// Area of tort law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortArea {
    /// Duty of care
    DutyOfCare,
    /// Standard of care
    StandardOfCare,
    /// Causation
    Causation,
    /// Remoteness
    Remoteness,
    /// Defences
    Defences,
    /// Damages
    Damages,
    /// Occupiers' liability
    OccupiersLiability,
    /// Defamation
    Defamation,
    /// Nuisance
    Nuisance,
}

impl TortCase {
    /// Donoghue v Stevenson \[1932\] - neighbour principle
    pub fn donoghue_v_stevenson() -> Self {
        Self {
            citation: CaseCitation {
                name: "Donoghue v Stevenson".to_string(),
                year: 1932,
                neutral_citation: None,
                report_citation: Some("[1932] AC 562 (HL)".to_string()),
                court: crate::common::Court::SupremeCourt, // Applied in Canada
                principle: "Neighbour principle - duty owed to those closely and directly affected"
                    .to_string(),
            },
            principle: "You must take reasonable care to avoid acts or omissions which you can \
                reasonably foresee would be likely to injure your neighbour"
                .to_string(),
            area: TortArea::DutyOfCare,
        }
    }

    /// Anns v Merton \[1978\] - two-stage test
    pub fn anns_v_merton() -> Self {
        Self {
            citation: CaseCitation {
                name: "Anns v Merton London Borough Council".to_string(),
                year: 1978,
                neutral_citation: None,
                report_citation: Some("[1978] AC 728 (HL)".to_string()),
                court: crate::common::Court::SupremeCourt, // Applied in Canada
                principle: "Two-stage test for duty of care".to_string(),
            },
            principle: "Stage 1: Prima facie duty (foreseeability + proximity). \
                Stage 2: Policy considerations that may negate duty"
                .to_string(),
            area: TortArea::DutyOfCare,
        }
    }

    /// Cooper v Hobart \[2001\] SCC 79 - refined Anns test
    pub fn cooper_v_hobart() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Cooper v Hobart",
                2001,
                79,
                "Refined Anns/Cooper test for duty of care in Canada",
            ),
            principle: "Two-stage Anns/Cooper test: (1) Foreseeability + proximity (recognized \
                category or analogous); (2) Residual policy considerations. Proximity is the \
                controlling concept."
                .to_string(),
            area: TortArea::DutyOfCare,
        }
    }

    /// Childs v Desormeaux \[2006\] SCC 18 - social host liability
    pub fn childs_v_desormeaux() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Childs v Desormeaux",
                2006,
                18,
                "Social host liability - no general duty to monitor guests",
            ),
            principle: "Social hosts do not owe a duty of care to public users of highways to \
                monitor guests' alcohol consumption. No positive duty without special relationship."
                .to_string(),
            area: TortArea::DutyOfCare,
        }
    }

    /// Clements v Clements \[2012\] SCC 32 - material contribution
    pub fn clements_v_clements() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Clements v Clements",
                2012,
                32,
                "Material contribution to risk causation test",
            ),
            principle: "But-for test is default. Material contribution applies only where: \
                (1) impossible to prove causation on but-for due to current limits of science, \
                and (2) breach of duty materially contributed to risk of injury."
                .to_string(),
            area: TortArea::Causation,
        }
    }

    /// Mustapha v Culligan \[2008\] SCC 27 - psychological harm
    pub fn mustapha_v_culligan() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Mustapha v Culligan of Canada Ltd",
                2008,
                27,
                "Remoteness and foreseeability for psychological injury",
            ),
            principle: "Psychological injury must be foreseeable in person of ordinary fortitude. \
                Thin skull rule applies once some harm foreseeable, but not to initial threshold."
                .to_string(),
            area: TortArea::Remoteness,
        }
    }

    /// Andrews v Grand & Toy \[1978\] - non-pecuniary cap
    pub fn andrews_v_grand_toy() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Andrews v Grand & Toy Alberta Ltd",
                1978,
                5,
                "Cap on non-pecuniary damages for catastrophic injury",
            ),
            principle: "Non-pecuniary damages capped at $100,000 (1978 dollars, indexed) for \
                catastrophic injuries. Functional approach to damages."
                .to_string(),
            area: TortArea::Damages,
        }
    }

    /// Grant v Torstar \[2009\] SCC 61 - responsible communication
    pub fn grant_v_torstar() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Grant v Torstar Corp",
                2009,
                61,
                "Defence of responsible communication on matters of public interest",
            ),
            principle: "New defence of responsible communication on matters of public interest. \
                Defendant must show: (1) publication on matter of public interest, and \
                (2) responsible journalism despite defamatory content."
                .to_string(),
            area: TortArea::Defamation,
        }
    }
}

// ============================================================================
// Provincial Variations
// ============================================================================

/// Provincial occupiers' liability statute reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlaStatute {
    /// Province
    pub province: Province,
    /// Statute name
    pub statute: String,
    /// Whether statutory regime replaces common law
    pub replaces_common_law: bool,
}

impl OlaStatute {
    /// Get OLA statute for province
    pub fn for_province(province: &Province) -> Option<Self> {
        match province {
            Province::Ontario => Some(Self {
                province: Province::Ontario,
                statute: "Occupiers' Liability Act, RSO 1990, c O.2".to_string(),
                replaces_common_law: true,
            }),
            Province::BritishColumbia => Some(Self {
                province: Province::BritishColumbia,
                statute: "Occupiers Liability Act, RSBC 1996, c 337".to_string(),
                replaces_common_law: true,
            }),
            Province::Alberta => Some(Self {
                province: Province::Alberta,
                statute: "Occupiers' Liability Act, RSA 2000, c O-4".to_string(),
                replaces_common_law: true,
            }),
            Province::Manitoba => Some(Self {
                province: Province::Manitoba,
                statute: "Occupiers' Liability Act, CCSM c O8".to_string(),
                replaces_common_law: true,
            }),
            // Other provinces may use common law or have different statutes
            _ => None,
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
    fn test_duty_of_care_stages() {
        let stage1 = DutyOfCareStage::PrimaFacie;
        let stage2 = DutyOfCareStage::PolicyConsiderations;
        assert_ne!(stage1, stage2);
    }

    #[test]
    fn test_recognized_duty_category() {
        let category = RecognizedDutyCategory::ManufacturerConsumer;
        assert_eq!(category, RecognizedDutyCategory::ManufacturerConsumer);
    }

    #[test]
    fn test_causation_test() {
        let but_for = CausationTest::ButFor;
        let material = CausationTest::MaterialContribution;
        assert_ne!(but_for, material);
    }

    #[test]
    fn test_non_pecuniary_cap() {
        let cap = NonPecuniaryCap::default();
        assert_eq!(cap.base_amount_1978, 10_000_000);
        assert!(!cap.is_catastrophic);
    }

    #[test]
    fn test_donoghue_v_stevenson() {
        let case = TortCase::donoghue_v_stevenson();
        assert_eq!(case.citation.year, 1932);
        assert_eq!(case.area, TortArea::DutyOfCare);
    }

    #[test]
    fn test_cooper_v_hobart() {
        let case = TortCase::cooper_v_hobart();
        assert_eq!(case.citation.year, 2001);
        assert!(case.principle.contains("Anns/Cooper"));
    }

    #[test]
    fn test_grant_v_torstar() {
        let case = TortCase::grant_v_torstar();
        assert_eq!(case.area, TortArea::Defamation);
        assert!(case.principle.contains("responsible communication"));
    }

    #[test]
    fn test_ola_statute_ontario() {
        let ola = OlaStatute::for_province(&Province::Ontario);
        assert!(ola.is_some());
        assert!(ola.as_ref().is_some_and(|o| o.replaces_common_law));
    }

    #[test]
    fn test_ola_statute_quebec() {
        let ola = OlaStatute::for_province(&Province::Quebec);
        assert!(ola.is_none()); // Quebec uses civil code
    }
}
