//! Canada Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of Canadian law across multiple areas.
//!
//! **Status**: v0.1.1 - Initial Implementation
//!
//! ## Legal Areas Covered
//!
//! 1. **Constitutional Law** (Charter of Rights, Division of Powers)
//!    - Canadian Charter of Rights and Freedoms (s.1 Oakes test)
//!    - Division of powers (ss.91-92 Constitution Act, 1867)
//!    - Aboriginal and treaty rights (s.35)
//!    - Constitutional doctrines (pith and substance, paramountcy, IJI)
//!
//! 2. **Contract Law** (Common Law + Quebec Civil Law)
//!    - Formation (offer, acceptance, consideration)
//!    - Terms and interpretation
//!    - Breach and remedies
//!    - Quebec obligations (Civil Code)
//!
//! 3. **Tort Law** (Common Law)
//!    - Negligence (Cooper v Hobart, Anns/Cooper test)
//!    - Occupiers' liability (provincial OLA statutes)
//!    - Defamation (Grant v Torstar responsible communication)
//!
//! 4. **Employment Law** (Federal + Provincial)
//!    - Provincial employment standards (ESA)
//!    - Human Rights Codes (duty to accommodate)
//!    - Reasonable notice (Bardal factors)
//!    - Just cause (McKinley contextual approach)
//!    - Wrongful dismissal (Wallace, Keays v Honda)
//!
//! 5. **Criminal Law** (Criminal Code)
//!    - Homicide (murder, manslaughter)
//!    - Assault and sexual offences
//!    - Defences (self-defence, necessity, duress, NCR)
//!    - Sentencing (s.718 principles, Gladue factors)
//!    - Charter rights in criminal process
//!
//! 6. **Family Law** (Divorce Act + Provincial)
//!    - Divorce Act (grounds, parenting, support)
//!    - Parenting arrangements (decision-making, parenting time)
//!    - Child Support Guidelines
//!    - Spousal Support Advisory Guidelines (SSAG)
//!
//! 7. **Property Law** (Real Property + Aboriginal Title)
//!    - Land titles and Torrens system
//!    - Aboriginal title (Tsilhqot'in Nation)
//!    - Duty to consult (Haida Nation)
//!    - Interests in land (easements, covenants, mortgages)
//!    - Conveyancing
//!
//! 8. **Corporate Law** (CBCA + Provincial)
//!    - Canada Business Corporations Act
//!    - Provincial incorporation
//!    - Director duties (BCE Inc, Peoples v Wise)
//!    - Oppression remedy (s.241 CBCA)
//!    - Derivative actions (s.239 CBCA)
//!
//! ## Canadian Legal System
//!
//! ### Bijural System
//!
//! Canada has a unique bijural legal system:
//! - **Common Law**: All provinces except Quebec (English tradition)
//! - **Civil Law**: Quebec (French tradition, Civil Code of Québec)
//! - **Federal**: Bijural - federal laws apply both traditions
//!
//! ### Division of Powers
//!
//! The Constitution Act, 1867 divides legislative authority:
//! - **Section 91**: Federal exclusive powers (criminal law, banking, divorce, etc.)
//! - **Section 92**: Provincial exclusive powers (property, civil rights, health, etc.)
//! - **POGG**: Federal residual power for national concerns
//!
//! ### Charter of Rights and Freedoms
//!
//! The Charter (1982) protects fundamental rights:
//! - Rights may be limited under s.1 (Oakes test: pressing objective, proportionality)
//! - Notwithstanding clause (s.33) allows override of some rights
//! - Applies to government action only (s.32)
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_ca::constitution::{
//!     CharterAnalyzer, CharterClaimFacts, CharterRight,
//! };
//! use legalis_ca::common::Province;
//!
//! // Charter analysis
//! let claim = CharterClaimFacts { /* ... */ };
//! let result = CharterAnalyzer::analyze(&claim);
//!
//! // Division of powers
//! use legalis_ca::constitution::{DivisionAnalyzer, DivisionFacts};
//! let facts = DivisionFacts { /* ... */ };
//! let result = DivisionAnalyzer::analyze(&facts);
//! ```
//!
//! ## Key Cases Implemented
//!
//! - **R v Oakes** \[1986\] 1 SCR 103 (s.1 test)
//! - **Haida Nation v BC** \[2004\] 3 SCR 511 (duty to consult)
//! - **Tsilhqot'in Nation v BC** \[2014\] 2 SCR 256 (Aboriginal title)
//! - **Reference re Secession of Quebec** \[1998\] 2 SCR 217 (constitutional principles)
//! - **Carter v Canada** \[2015\] 1 SCR 331 (s.7 security of person)
//! - **R v Jordan** \[2016\] 1 SCR 631 (s.11(b) trial delay)
//! - **Bhasin v Hrynew** \[2014\] 3 SCR 494 (good faith in contracts)
//! - **Tercon v BC** \[2010\] 1 SCR 69 (exclusion clauses)
//! - **Hunter Engineering v Syncrude** \[1989\] 1 SCR 426 (fundamental breach)
//! - **Cooper v Hobart** \[2001\] SCC 79 (Anns/Cooper duty of care test)
//! - **Clements v Clements** \[2012\] SCC 32 (material contribution causation)
//! - **Mustapha v Culligan** \[2008\] SCC 27 (psychological harm remoteness)
//! - **Grant v Torstar** \[2009\] SCC 61 (responsible communication defence)
//! - **Andrews v Grand & Toy** \[1978\] SCC (non-pecuniary damages cap)
//! - **Bardal v Globe & Mail** \[1960\] (reasonable notice factors)
//! - **671122 Ontario v Sagaz** \[2001\] SCC 59 (employee vs. contractor)
//! - **McKinley v BC Tel** \[2001\] SCC 38 (contextual just cause)
//! - **Wallace v United Grain Growers** \[1997\] SCC 46 (bad faith dismissal)
//! - **Keays v Honda** \[2008\] SCC 39 (aggravated damages)
//! - **Potter v NB Legal Aid** \[2015\] SCC 10 (constructive dismissal)
//! - **BC v BCGSEU (Meiorin)** \[1999\] SCC 48 (BFOR test)

#![deny(missing_docs)]
#![warn(clippy::all)]

/// Common utilities (provinces, calendar, court hierarchy)
///
/// Covers:
/// - Province and territory enumeration
/// - Canadian statutory holidays by province
/// - Court hierarchy and precedent
/// - Statute references
/// - Official languages
pub mod common;

/// Constitutional Law (Charter, Division of Powers)
///
/// Covers:
/// - Canadian Charter of Rights and Freedoms
/// - Section 1 Oakes test justification
/// - Division of powers (ss.91-92)
/// - POGG (Peace, Order, Good Government)
/// - Constitutional doctrines (pith and substance, paramountcy, IJI)
/// - Aboriginal and treaty rights (s.35)
pub mod constitution;

/// Contract Law (Common Law + Quebec Civil Law)
///
/// Covers:
/// - Contract formation (offer, acceptance, consideration)
/// - Contract terms (conditions, warranties, innominate terms)
/// - Breach and remedies (Hadley v Baxendale, Tercon framework)
/// - Quebec obligations (Civil Code of Quebec)
/// - Good faith (Bhasin v Hrynew, CM Callow v Zollinger)
/// - Consumer protection (provincial statutes)
pub mod contract;

/// Tort Law (Common Law)
///
/// Covers:
/// - Negligence (Anns/Cooper two-stage test)
/// - Duty of care, standard of care, causation, remoteness
/// - Damages (non-pecuniary cap from trilogy)
/// - Occupiers' liability (provincial OLA statutes)
/// - Defamation (responsible communication - Grant v Torstar)
pub mod tort;

/// Employment Law (Federal + Provincial)
///
/// Covers:
/// - Employment status (Sagaz test)
/// - Provincial employment standards (ESA)
/// - Federal employment (Canada Labour Code)
/// - Human Rights Codes (duty to accommodate, Meiorin)
/// - Reasonable notice (Bardal factors)
/// - Just cause (McKinley contextual approach)
/// - Wrongful dismissal (Wallace, Keays v Honda)
/// - Constructive dismissal (Potter)
pub mod employment;

/// Criminal Law (Criminal Code)
///
/// Covers:
/// - Homicide offences (murder, manslaughter, criminal negligence)
/// - Assault offences (common to aggravated)
/// - Sexual offences
/// - Property offences (theft, fraud, break and enter)
/// - Defences (self-defence, necessity, duress, NCR)
/// - Sentencing principles (s.718)
/// - Gladue factors for Indigenous offenders (s.718.2(e))
/// - Charter rights in criminal process
pub mod criminal;

/// Family Law (Divorce Act + Provincial)
///
/// Covers:
/// - Divorce (grounds under s.8 Divorce Act)
/// - Parenting arrangements (decision-making, parenting time)
/// - Best interests of child analysis (s.16)
/// - Family violence considerations
/// - Child Support Guidelines
/// - Spousal Support Advisory Guidelines (SSAG)
/// - Relocation (Gordon v Goertz)
pub mod family;

/// Property Law (Real Property + Aboriginal Title)
///
/// Covers:
/// - Land classification (freehold, leasehold, condominium)
/// - Land registration (Torrens, Registry, Land Titles)
/// - Interests in land (easements, covenants, mortgages)
/// - Aboriginal title (Tsilhqot'in test: occupation, continuity, exclusivity)
/// - Duty to consult (Haida Nation spectrum)
/// - Conveyancing process
/// - Expropriation
pub mod property;

/// Corporate Law (CBCA + Provincial)
///
/// Covers:
/// - Federal incorporation (CBCA)
/// - Provincial incorporation
/// - Director duties (fiduciary duty, duty of care)
/// - Business judgment rule (Peoples v Wise)
/// - Stakeholder interests (BCE Inc)
/// - Oppression remedy (s.241 CBCA)
/// - Derivative action (s.239 CBCA)
/// - Fundamental changes (amalgamation, arrangement)
pub mod corporate;

/// Reasoning Engine (legalis-core integration)
///
/// Covers:
/// - Canadian reasoning engine with statute loading
/// - Constitutional verification (Charter, division of powers)
/// - Inter-provincial conflict of laws
/// - Bijural interoperability (common law / civil law)
pub mod reasoning;

// Re-exports for convenience
pub use common::{
    BilingualRequirement, CanadianCalendar, CanadianTimeZone, CaseCitation, Court, Holiday,
    JurisdictionalLevel, LegalSystem, OfficialLanguage, Province, StatuteReference,
};

pub use constitution::{
    // Aboriginal Rights
    AboriginalRight,
    // Charter
    CharterAnalyzer,
    CharterClaimFacts,
    CharterClaimResult,
    CharterRemedy,
    CharterRight,
    // Division of Powers
    ConflictType,
    ConflictingLaw,
    // Cases
    ConstitutionalCase,
    ConstitutionalDoctrine,
    // Errors
    ConstitutionalError,
    ConstitutionalResult,
    DivisionAnalyzer,
    DivisionFacts,
    DivisionResult,
    EnactingBody,
    FederalPower,
    GovernmentAction,
    HeadOfPower,
    IjiAnalysis,
    MinimalImpairment,
    OakesAnalyzer,
    OakesTest,
    ParamountcyAnalysis,
    PithAndSubstance,
    PoggAnalysis,
    PoggAnalyzer,
    PoggBranch,
    PressAndSubstantial,
    ProportionalityAnalysis,
    ProportionalityStrictoSensu,
    ProvincialPower,
    RationalConnection,
    // Statute builders
    create_charter_statute,
    create_constitution_1867_statute,
    create_constitution_1982_statute,
};

pub use contract::{
    // Formation
    Acceptance,
    // Breach
    BreachAnalyzer,
    BreachFacts,
    BreachResult,
    BreachType,
    CapacityIssue,
    CapacityStatus,
    // Quebec Civil Law
    CcqConcept,
    CcqContractType,
    CommunicationMethod,
    Consideration,
    // Cases
    ContractArea,
    ContractCase,
    ContractContext,
    // Errors
    ContractError,
    ContractRemedy,
    ContractResult,
    // Terms
    ContractTerm,
    DamagesAnalyzer,
    DamagesCalculation,
    DamagesFacts,
    DamagesResult,
    // Vitiating Factors
    DuressType,
    ExclusionClause,
    FormationAnalyzer,
    FormationElement,
    FormationFacts,
    FormationResult,
    IntentionEvidence,
    LegalityStatus,
    MisrepresentationType,
    MistakeType,
    Offer,
    OfferAnalyzer,
    OfferClassificationFacts,
    OfferClassificationResult,
    OfferContext,
    TermClassification,
    TermType,
    VitiatingFactor,
    // Statute builders
    create_ccq_consent,
    create_ccq_obligations,
    create_ccq_warranty_quality,
    create_consumer_protection_act,
    create_contract_statutes,
    create_sale_of_goods_act,
};

pub use tort::{
    // Occupiers' liability
    ApplicableLaw,
    // Standard of care
    BreachFactor,
    // Causation
    CausationAnalyzer,
    CausationFacts,
    CausationResult,
    CausationTest,
    CommonLawEntrantStatus,
    // Defamation
    DamagesAssessment,
    DamagesType,
    DefamationAnalyzer,
    DefamationDefence,
    DefamationDefenceClaim,
    DefamationElements,
    DefamationFacts,
    DefamationResult,
    DefamationType,
    DefenceAnalysis,
    // Duty of care
    DutyOfCareAnalyzer,
    DutyOfCareFacts,
    DutyOfCareResult,
    DutyOfCareStage,
    EntrantStatus,
    EntryPurpose,
    HazardDescription,
    HazardType,
    InterveningCause,
    // Full negligence
    NegligenceAnalyzer,
    NegligenceDamagesFacts,
    NegligenceDamagesResult,
    // Defences
    NegligenceDefence,
    NegligenceFacts,
    NegligenceResult,
    // Damages
    NonPecuniaryCap,
    // Nuisance
    NuisanceFactor,
    NuisanceType,
    OccupierStatus,
    OccupiersLiabilityAnalyzer,
    OccupiersLiabilityFacts,
    OccupiersLiabilityResult,
    OlaDefence,
    OlaDuty,
    OlaStatute,
    PolicyNegation,
    ProximityFactor,
    PublicationMedium,
    PublicationReach,
    RecognizedDutyCategory,
    // Remoteness
    RemotenessAnalyzer,
    RemotenessFacts,
    RemotenessResult,
    RemotenessTest,
    ResponsibleCommunicationFactors,
    StandardOfCare,
    StandardOfCareAnalyzer,
    StandardOfCareFacts,
    StandardOfCareResult,
    StatementContext,
    // Cases
    TortArea,
    TortCase,
    TortDamages,
    TortDamagesAnalyzer,
    // Errors
    TortError,
    TortResult,
    // Statute builders
    create_ccq_civil_liability,
    create_negligence_statute,
    create_ola_statute,
};

pub use employment::{
    AccommodationType,
    BardalFactor,
    DiscriminationType,
    DutyToAccommodate,
    // Cases
    EmploymentArea,
    EmploymentCase,
    // Errors
    EmploymentError,
    EmploymentJurisdiction,
    EmploymentResult,
    // Employment standards
    EmploymentStandards,
    // Employment status
    EmploymentStatus,
    FederalIndustry,
    HardshipFactor,
    JustCauseAnalyzer,
    JustCauseFacts,
    JustCauseGround,
    JustCauseResult,
    MitigationRequirement,
    // Human rights
    ProtectedGround,
    // Analyzers
    ReasonableNoticeAnalyzer,
    ReasonableNoticeFacts,
    ReasonableNoticeResult,
    SagazFactor,
    StandardType,
    // Termination
    TerminationType,
    WrongfulDismissalAnalyzer,
    // Wrongful dismissal
    WrongfulDismissalDamages,
    WrongfulDismissalFacts,
    WrongfulDismissalResult,
    // Statute builders
    create_canada_labour_code,
    create_canadian_human_rights_act,
    create_employment_standards_act,
    create_employment_statutes,
    create_federal_employment_statutes,
    create_human_rights_code,
};

pub use criminal::{
    AccusedElection,
    ActusReus,
    AggravatingFactor,
    AssaultAnalyzer,
    AssaultFacts,
    AssaultResult,
    AssaultType,
    BailType,
    BodilyHarmLevel,
    BreachSeriousness,
    BreakEnterType,
    // Supporting types
    CausationFacts as CriminalCausationFacts,
    CharterRemedy as CriminalCharterRemedy,
    // Cases
    CriminalArea,
    CriminalCase,
    // Charter rights
    CriminalCharterRight,
    // Defences
    CriminalDefence,
    // Errors
    CriminalError,
    CriminalResult,
    CrownElection,
    DefenceAnalyzer,
    DefenceFacts,
    DefenceOutcome,
    DefenceResult,
    DetentionGround,
    DuressElements,
    DutySource,
    // Homicide specifics
    FirstDegreeFactor,
    FraudType,
    GladueAnalysis,
    GladueFactor,
    GrantAnalysis,
    HarmLevel,
    // Analyzers
    HomicideAnalyzer,
    HomicideFacts,
    HomicideResult,
    HomicideType,
    ImpactLevel,
    IntentionType,
    InterveningAct,
    IntoxicationDefence,
    ManslaughterType,
    // Mens rea
    MensRea,
    MentalDisorderElements,
    MentalStateFacts,
    MitigatingFactor,
    // Procedure
    ModeOfTrial,
    NecessityElements,
    OffenceCategory,
    // Offence types
    OffenceType,
    OffenderInfo,
    PlannedDeliberateFacts,
    PriorConviction,
    ProvocationFacts,
    RecklessnessType,
    SelfDefenceElements,
    SentenceRange,
    // Sentencing
    SentenceType,
    SentencingAnalyzer,
    SentencingFacts,
    SentencingPrinciple,
    SentencingResult,
    SocietalInterest,
    TheftType,
    VictimImpact,
    // Statute builders
    create_cdsa,
    create_charter_criminal_rights,
    create_criminal_code,
    create_criminal_statutes,
    create_ycja,
};

pub use family::{
    ArrangementFunctioning,
    // Best interests analysis
    BestInterestsAnalyzer,
    BestInterestsFactor,
    BestInterestsFacts,
    BestInterestsResult,
    ChildInfo,
    // Child support
    ChildSupportAnalyzer,
    ChildSupportCalculationType,
    ChildSupportFacts,
    ChildSupportResult,
    ChildSupportType,
    ChildViews,
    CurrentArrangement,
    DecisionMaker,
    DecisionMakingAllocation,
    DivorceGround,
    DivorceStage,
    DurationRange,
    ExcludedPropertyType,
    FactorAnalysis,
    FactorWeight,
    // Cases
    FamilyArea,
    FamilyCase,
    // Errors
    FamilyError,
    FamilyResult,
    FamilyViolence,
    FamilyViolenceAllegation,
    FlexibilityLevel,
    // Marriage/Divorce
    MarriageStatus,
    MaturityLevel,
    ParentInfo,
    // Parenting
    ParentingArrangement,
    ParentingTimeSchedule,
    // Property
    PropertyClassification,
    ProposedArrangement,
    // Relocation
    RelocationAnalyzer,
    RelocationFacts,
    RelocationReason,
    RelocationRequest,
    RelocationResult,
    Section7Expense,
    Section7ExpenseItem,
    // Spousal support
    SpousalSupportAnalyzer,
    SpousalSupportBasis,
    SpousalSupportFacts,
    SpousalSupportRange,
    SpousalSupportResult,
    SpousalSupportType,
    SsagFormula,
    SupportDuration,
    UndueHardshipClaim,
    UndueHardshipFactor,
    ValuationDate,
    ViolenceFinding,
    ViolenceImpact,
    WillingnessLevel,
    // Statute builders
    create_child_support_guidelines,
    create_divorce_act,
    create_family_law_act,
    create_family_statutes,
    create_ssag,
};

pub use property::{
    // Aboriginal title analysis
    AboriginalTitleAnalyzer,
    AboriginalTitleElement,
    AboriginalTitleFacts,
    AboriginalTitleResult,
    // Aboriginal title types
    AboriginalTitleStatus,
    ClaimStrength,
    CoOwnershipType,
    // Consultation analysis
    ConsultationAnalyzer,
    ConsultationFacts,
    ConsultationLevel,
    ConsultationResult,
    ConsultationStep,
    ConsultationTrigger,
    ContinuityFactor,
    // Conveyancing
    ConveyancingStage,
    EasementCreation,
    EasementType,
    EstateType,
    ExclusivityFactor,
    ImpactSeverity,
    InfringementJustification,
    // Interests in land
    InterestInLand,
    // Registration
    LandRegistrationSystem,
    LienType,
    OccupationEvidence,
    OccupationFactor,
    // Cases
    PropertyArea,
    PropertyCase,
    // Errors
    PropertyError,
    PropertyResult,
    // Land types
    PropertyType,
    StandardCondition,
    TenancyPeriod,
    TitleAssurance,
    TitleDefect,
    TitleException,
    TreatyStatus,
    // Statute builders
    create_condominium_act,
    create_construction_lien_act,
    create_expropriation_act,
    create_fnlma,
    create_indian_act,
    create_land_titles_act,
    create_planning_act,
    create_property_statutes,
};

pub use corporate::{
    AllegedConduct,
    AmalgamationType,
    ApprovalRequirement,
    BusinessJudgmentElement,
    BusinessJudgmentFactors,
    CompensationType,
    ComplainantInfo,
    ComplainantType,
    ConductType,
    ConflictDetails,
    ConflictNature,
    ContinuanceDirection,
    // Cases
    CorporateArea,
    CorporateCase,
    // Errors
    CorporateError,
    CorporateResult,
    CorporateStatus,
    CorporateType,
    DecisionContext,
    DecisionMakerType,
    DerivativeRequirement,
    DirectorDisqualification,
    DirectorDuty,
    // Director duty analysis
    DirectorDutyAnalyzer,
    DirectorDutyFacts,
    DirectorDutyResult,
    // Director duties
    DirectorQualification,
    DutyBreach,
    ExitOffer,
    ExpectationSource,
    ExpectationStrength,
    FiduciaryBreachType,
    // Fundamental changes
    FundamentalChange,
    ImpactNature,
    ImpactSeverity as CorporateImpactSeverity,
    // Incorporation types
    IncorporationJurisdiction,
    InformationLevel,
    // Oppression analysis
    OppressionAnalyzer,
    OppressionConduct,
    OppressionContext,
    OppressionElement,
    OppressionFacts,
    OppressionRemedy,
    OppressionResult,
    ProspectusExemption,
    ReasonableExpectation,
    ReportingIssuerStatus,
    // Securities
    SecurityType,
    ShareClass,
    SharePurchaser,
    ShareRight,
    ShareStructure,
    // Shareholder remedies
    ShareholderRemedy,
    StakeholderAnalysis,
    StakeholderImpact,
    StakeholderInterest,
    // Stakeholder types
    StakeholderType,
    ValuationBasis,
    // Statute builders
    create_cbca,
    create_cnca,
    create_competition_act,
    create_corporate_statutes,
    create_investment_canada_act,
    create_provincial_corporations_act,
    create_securities_act,
};

pub use reasoning::{
    ApplicableLawResult,
    // Interoperability
    CanadianInterop,
    // Reasoning engine
    CanadianReasoningEngine,
    CivilLawConcept,
    CommonLawConcept,
    ConflictType as ReasoningConflictType,
    ConstitutionalIssue,
    // Constitutional verifier
    ConstitutionalVerifier,
    GoverningLaw,
    InterProvincialFacts,
    IssueType,
    LegalAreaType,
    ReasoningJurisdiction,
    ReasoningQuery,
    ReasoningResult,
    StatuteConflict,
    VerificationContext,
    VerificationResult,
    create_federal_engine,
    // Convenience functions
    create_provincial_engine,
    determine_applicable_law,
    verify_statute,
};
