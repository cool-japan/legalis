#![allow(clippy::large_enum_variant)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::new_ret_no_self)]

//! United Kingdom Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of UK law (England & Wales) across five major areas.
//!
//! **Status**: v0.2.0 - Initial Implementation 🚧
//! - Foundation complete
//! - Modules in development
//!
//! ## Legal Areas Covered
//!
//! 1. **Employment Law** (Employment Rights Act 1996) - 🚧 IN PROGRESS
//!    - Employment contracts and written particulars (s.1)
//!    - Statutory notice periods (s.86)
//!    - Unfair dismissal (s.98 - 2-year qualifying period)
//!    - Redundancy payments (s.162)
//!    - Working Time Regulations 1998 (48-hour week)
//!    - National Minimum Wage Act 1998 (age-based rates)
//!
//! 2. **Data Protection** (UK GDPR / Data Protection Act 2018) - ⏳ PLANNED
//!    - Reuses 80% from EU GDPR implementation
//!    - UK-specific: ICO enforcement, adequacy decisions
//!    - DPA 2018 exemptions (journalism, research, national security)
//!    - UK international data transfers (IDTA, SCCs with addendum)
//!
//! 3. **Consumer Rights** (Consumer Rights Act 2015) - ⏳ PLANNED
//!    - Goods contracts (s.9-11: quality, purpose, description)
//!    - Services contracts (s.49-52: care, skill, time, price)
//!    - Digital content (s.34-47)
//!    - Tiered remedies (short-term reject → repair/replace → price reduction/final reject)
//!    - Unfair terms (Part 2)
//!
//! 4. **Contract Law** (Common Law Principles) - ⏳ PLANNED
//!    - Contract formation (offer, acceptance, consideration, intention)
//!    - Common law rules: mirror image rule, postal rule
//!    - Case law integration: Hadley v Baxendale, Adams v Lindsell, etc.
//!    - Terms classification (condition, warranty, innominate)
//!    - Remedies (damages, specific performance, injunction)
//!
//! 5. **Company Law** (Companies Act 2006) - ⏳ PLANNED
//!    - Company formation (Part 2)
//!    - Seven statutory director duties (ss.171-177)
//!    - Share capital requirements (£50k minimum for plc)
//!    - Company name restrictions (ss.53-81)
//!    - Corporate governance
//!
//! ## UK Legal System Characteristics
//!
//! ### Common Law vs Civil Law
//!
//! The UK (England & Wales) follows the **Common Law** tradition, fundamentally different
//! from civil law systems (Germany, France, Japan):
//!
//! ```text
//! Common Law (UK)              Civil Law (DE, FR, JP)
//! ├── Primary source           ├── Primary source
//! │   └── Case law (precedent) │   └── Codified statutes
//! ├── Court role               ├── Court role
//! │   └── Law-making           │   └── Law-applying
//! ├── Reasoning                ├── Reasoning
//! │   └── Inductive            │   └── Deductive
//! └── Binding force            └── Binding force
//!     └── Stare decisis            └── Statutory text
//! ```
//!
//! ### Stare Decisis (Binding Precedent)
//!
//! UK courts follow precedent from higher courts:
//! - Supreme Court (binds all lower courts)
//! - Court of Appeal (binds High Court and below)
//! - High Court (binds County Court and tribunals)
//!
//! ### Statute Referencing
//!
//! UK statutes use section (s.) notation, not articles:
//! - `ERA 1996 s.86` (not "Article 86")
//! - `CA 2006 ss.171-177` (sections 171 to 177)
//! - `CRA 2015 s.9` (section 9)
//!
//! ### Contract Formation Requirements
//!
//! Unlike civil law, UK common law requires **consideration**:
//! - Must move from promisee (Tweddle v Atkinson 1861)
//! - Must not be past consideration (Re McArdle 1951)
//! - Must be sufficient but need not be adequate (Chappell v Nestlé 1960)
//!
//! ## Regional Coverage
//!
//! This crate currently covers **England & Wales** only:
//! - **Scotland**: Different legal system (hybrid civil/common law) - not yet implemented
//! - **Northern Ireland**: Separate but similar to E&W - not yet implemented
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::employment::{EmploymentContract, ContractType};
//!
//! let contract = EmploymentContract::builder()
//!     .with_employee_name("John Smith")
//!     .with_employer_name("Acme Ltd")
//!     .with_contract_type(ContractType::Permanent);
//!
//! // Validate against ERA 1996
//! contract.validate()?;
//! ```
//!
//! ## Architecture
//!
//! Each module follows the standard Legalis-RS pattern:
//! - `types.rs` - Core data structures
//! - `error.rs` - Error types with statute references
//! - `validator.rs` - Validation logic
//! - `mod.rs` - Module documentation and re-exports
//!
//! ## Dependencies
//!
//! - `legalis-core` - Core legal framework
//! - `legalis-eu` - EU GDPR implementation (reused for UK GDPR)
//! - `chrono` - Date/time handling
//! - `thiserror` - Error handling
//! - `uuid` - Unique identifiers
//! - `serde` - Serialization (optional feature)

#![deny(missing_docs)]
#![warn(clippy::all)]

/// Common utilities (legalis-i18n integration)
///
/// Covers:
/// - UK bank holidays (England & Wales, Scotland, Northern Ireland)
/// - Working day calculations
/// - Legal deadline calculations
/// - UK timezone handling (GMT/BST)
pub mod common;

/// Employment Law (Employment Rights Act 1996, Working Time Regulations 1998, NMWA 1998)
///
/// Covers:
/// - Employment contracts and written particulars
/// - Statutory notice periods
/// - Unfair dismissal (2-year qualifying period)
/// - Redundancy payments (age-based multipliers)
/// - Working time regulations (48-hour week)
/// - National minimum wage (age-based rates)
pub mod employment;

/// Intellectual Property Law (Patents Act 1977, Trade Marks Act 1994, CDPA 1988)
///
/// Covers:
/// - Patents (novelty, inventive step, industrial application)
/// - Trade marks (distinctiveness, likelihood of confusion)
/// - Copyright (originality, duration, fair dealing)
/// - Designs (registered and unregistered design rights)
/// - UK IPO proceedings and enforcement
pub mod intellectual_property;

/// Data Protection (UK GDPR, Data Protection Act 2018)
///
/// 80% reuse from EU GDPR with UK-specific adaptations:
/// - ICO enforcement (not EDPB)
/// - UK adequacy decisions (post-Brexit)
/// - DPA 2018 exemptions (journalism, research, national security)
/// - UK international data transfers (IDTA, SCCs with addendum)
pub mod data_protection;

/// Consumer Rights (Consumer Rights Act 2015)
///
/// Covers:
/// - Goods contracts (satisfactory quality, fit for purpose, as described)
/// - Services contracts (reasonable care and skill)
/// - Digital content
/// - Tiered remedies (short-term reject → repair/replace → price reduction/final reject)
/// - Unfair terms test (Part 2)
pub mod consumer_rights;

/// Contract Law (Common Law Principles)
///
/// Covers:
/// - Contract formation (offer, acceptance, consideration, intention)
/// - Common law rules (mirror image rule, postal rule)
/// - Case law integration (Hadley v Baxendale, Adams v Lindsell, etc.)
/// - Terms classification (condition, warranty, innominate)
/// - Breach and remedies
pub mod contract;

/// Company Law (Companies Act 2006)
///
/// Covers:
/// - Company formation (Part 2)
/// - Seven statutory director duties (ss.171-177)
/// - Share capital requirements
/// - Company name restrictions (ss.53-81)
/// - Corporate governance
pub mod company;

/// Financial Services (FSMA 2000, FCA Rules)
///
/// Covers:
/// - FCA authorization and regulated activities
/// - 11 Principles for Businesses (PRIN)
/// - Client categorization (COBS 3)
/// - Suitability and appropriateness (COBS 9, 10)
/// - Client assets protection (CASS 6, 7)
/// - Financial promotions (FSMA s.21)
/// - Market abuse (UK MAR)
/// - Best execution (COBS 11)
/// - Senior Managers Regime (SM&CR)
pub mod financial_services;

/// Trust Law (Equity and Trustee Acts)
///
/// Covers:
/// - Three certainties (Knight v Knight [1840])
/// - Trust constitution (Milroy v Lord [1862])
/// - Trustee duties (Trustee Act 2000)
/// - Breach of trust and remedies (tracing)
/// - Third party liability (dishonest assistance, knowing receipt)
/// - Charitable trusts (Charities Act 2011)
/// - Cy-pres doctrine
pub mod trusts;

/// Family Law (Matrimonial Causes Act 1973, Children Act 1989, Family Law Act 1996)
///
/// Covers:
/// - Divorce and dissolution (DDSA 2020 no-fault divorce)
/// - Children law (CA 1989: PR, s.8 orders, welfare principle)
/// - Financial remedies (MCA 1973 s.25 factors, clean break)
/// - Domestic abuse protection (FLA 1996 Part IV, DAA 2021)
/// - Forced marriage and FGM protection orders
pub mod family;

/// Tort Law (Common Law, OLA 1957/1984, Defamation Act 2013)
///
/// Covers:
/// - Negligence (Donoghue v Stevenson, Caparo three-stage test)
/// - Professional negligence (Bolam test, Bolitho logical basis)
/// - Psychiatric injury (Alcock control mechanisms)
/// - Pure economic loss (Hedley Byrne, Murphy v Brentwood)
/// - Occupiers' liability (OLA 1957 visitors, OLA 1984 trespassers)
/// - Nuisance (private, public, Rylands v Fletcher)
/// - Defamation (Defamation Act 2013, serious harm threshold)
/// - Economic torts (OBG v Allan, inducing breach, conspiracy)
pub mod tort;

/// Criminal Law (Common Law, Criminal Justice Acts, PACE 1984)
///
/// Covers:
/// - Homicide (murder, manslaughter - R v Woollin, R v Adomako)
/// - Non-fatal offences (OAPA 1861 ss.47, 20, 18)
/// - Property offences (Theft Act 1968, Fraud Act 2006)
/// - General defences (self-defence, duress, intoxication - DPP v Majewski)
/// - Insanity (M'Naghten Rules, diminished responsibility)
/// - Sentencing (Sentencing Council guidelines, dangerous offenders)
/// - Criminal procedure (PACE 1984, arrest, detention, interview)
pub mod criminal;

/// Public Law (Senior Courts Act 1981, Human Rights Act 1998, Constitutional Principles)
///
/// Covers:
/// - Judicial review (CPR Part 54, Senior Courts Act 1981 s.31)
///   - Grounds: illegality, irrationality (Wednesbury), procedural impropriety
///   - Key cases: GCHQ, Anisminic, Daly
///   - Remedies: quashing, mandatory, prohibiting orders, declarations
/// - Human Rights Act 1998
///   - Section 3: interpretive duty (Ghaidan v Godin-Mendoza)
///   - Section 4: declarations of incompatibility
///   - Section 6: public authority duty (YL v Birmingham)
///   - Proportionality analysis (Bank Mellat)
/// - Constitutional principles
///   - Parliamentary sovereignty (Factortame, Jackson, Miller I)
///   - Rule of law (Dicey, Entick v Carrington, UNISON)
///   - Separation of powers (Miller II, R (Evans))
///   - Royal prerogative (GCHQ, Miller I, Miller II)
pub mod public_law;

/// Land Law (LPA 1925, LRA 2002, LTA 1954, TOLATA 1996)
///
/// Covers:
/// - Estates: freehold (fee simple absolute), leasehold (term of years)
/// - Lease vs licence (Street v Mountford exclusive possession test)
/// - Easements (Re Ellenborough Park four requirements)
/// - Restrictive covenants (Tulk v Moxhay, running of burden/benefit)
/// - Mortgages (legal charges, remedies, undue influence - Etridge)
/// - Registration (LRA 2002, overriding interests, priority)
/// - Unregistered land (Land Charges Act 1972)
/// - Co-ownership and trusts of land (TOLATA 1996)
pub mod land_law;

/// Legal Reasoning Engine (legalis-core Integration)
///
/// Provides automated legal analysis using legalis-core framework:
/// - Statute-based reasoning with EvaluationContext
/// - Compliance analysis for employment contracts
/// - Violation detection and remediation recommendations
/// - Risk level assessment
pub mod reasoning;

// Re-exports for convenience
pub use employment::{
    EmploymentContract, EmploymentError, MinimumWageAssessment, RedundancyPayment,
    validate_employment_contract,
};

// Data protection re-exports
pub use data_protection::{
    Article9Processing, DataController, DataProcessing, DataSubject, Dpa2018Exemption,
    IcoEnforcement, LawfulBasis, PersonalDataCategory, SpecialCategory, UkAdequacyDecision,
    UkDataProtectionError, is_adequate_country_uk,
};

// Consumer rights re-exports
pub use consumer_rights::{
    ConsumerRightsError, DigitalContentContract, GoodsContract, GoodsStatutoryRight,
    ServicesContract, ServicesStatutoryRight, UnfairTermAssessment, validate_as_described,
    validate_digital_content_contract, validate_fit_for_purpose, validate_goods_contract,
    validate_satisfactory_quality, validate_services_contract, validate_unfair_term,
};

// Contract law re-exports
pub use contract::{
    Acceptance, AcceptanceMethod, Consideration, ConsiderationType, ContractError,
    ContractFormation, IntentionToCreateLegalRelations, Offer, OfferType, validate_acceptance,
    validate_capacity, validate_consideration, validate_contract_formation, validate_intention,
    validate_offer,
};

// Company law re-exports
pub use company::{
    AnnualAccountsRequirement, CompanyFormation, CompanyLawError, CompanyType, Director,
    DirectorDutiesCompliance, RegisteredOffice, ShareCapital, Shareholder,
    validate_company_formation, validate_company_name, validate_director_duties,
};

// Financial services re-exports
pub use financial_services::{
    AuthorizationStatus, ClientCategory, FcaAuthorization, FinancialServicesError, InvestmentType,
    PrinciplesCompliance, RegulatedActivity, SuitabilityAssessment, validate_fca_authorization,
    validate_principles_compliance, validate_suitability_assessment,
};

// Reasoning engine re-exports (legalis-core integration)
pub use reasoning::{
    ComplianceStatus, LegalAnalysis, LegalReasoningEngine, ReasoningError, ReasoningResult,
    RiskLevel, UkEvaluationContext, Violation, ViolationSeverity,
};

// Common utilities re-exports (legalis-i18n integration)
pub use common::{
    // Calendar and bank holidays
    UkLegalCalendar,
    UkRegion,
    // Timezone
    UkTimeZone,
    calculate_legal_deadline,
    convert_to_uk_local,
    current_uk_offset,
    is_bank_holiday,
    is_bst,
    is_working_day,
};

// Trust law re-exports
pub use trusts::{
    // Types and errors
    Beneficiary,
    BeneficiaryType,
    // Breach
    BreachOfTrust,
    BreachRemedy,
    BreachSeverity,
    // Creation and certainties
    CertaintyOfIntention,
    CertaintyOfObjects,
    CertaintyOfSubjectMatter,
    // Charitable
    CharitablePurpose,
    CharitableTrust,
    // Trustees
    ConflictOfInterest,
    ConstitutionMethod,
    CyPresScheme,
    DishonestAssistance,
    DutyOfCare,
    InvestmentDecision,
    KnowingReceipt,
    PublicBenefitTest,
    ThreeCertainties,
    TracingMethod,
    Trust,
    TrustConstitution,
    TrustDeclaration,
    TrustError,
    TrustProperty,
    TrustResult,
    TrustType,
    Trustee,
    TrusteeAppointment,
    TrusteeDuty,
    TrusteePower,
    assess_breach_of_trust,
    assess_conflict_of_interest,
    calculate_tracing_remedy,
    check_certainty_intention,
    check_certainty_objects,
    check_certainty_subject_matter,
    check_duty_of_care,
    check_three_certainties,
    validate_charitable_purpose,
    validate_cy_pres,
    validate_dishonest_assistance,
    validate_investment_decision,
    validate_knowing_receipt,
    validate_public_benefit,
    validate_trust_constitution,
    validate_trustee_appointment,
};

// Family law re-exports
pub use family::{
    // Core types
    AbuseType,
    // Children
    ApplicantCategory,
    // Divorce
    ApplicationType,
    // Financial
    AssetSchedule,
    // Protection
    AssociatedPersonAnalysis,
    AssociatedPersonRelationship,
    ChildDetails,
    CleanBreakAnalysis,
    ConditionalOrderAnalysis,
    DivorceApplication,
    DivorceApplicationAnalysis,
    DivorceStage,
    DivorceTimeline,
    DomesticAbuseAnalysis,
    FamilyLawError,
    FinalOrderAnalysis,
    JurisdictionBasis,
    Marriage,
    NonMolestationOrder,
    NonMolestationOrderAnalysis,
    OccupationOrder,
    OccupationOrderAnalysis,
    ParentalResponsibility,
    ParentalResponsibilityAnalysis,
    PensionAnalysis,
    PersonDetails,
    PrenupAnalysis,
    Section8OrderType,
    Section25Factor,
    SharingAnalysis,
    StandingAnalysis,
    ThreeStrandsAnalysis,
    UndertakingAnalysis,
    WelfareChecklistAnalysis,
    WelfareChecklistFactor,
    perform_balance_of_harm_test,
    validate_divorce_application,
    validate_section25_factors,
    validate_welfare_checklist,
};

// Tort law re-exports
pub use tort::{
    AgreementAnalysis,
    AlcockControl,
    Apportionment,
    BolamTest,
    BreachAnalyzer,
    BreachEvidence,
    BreachFacts,
    BreachOfDuty,
    CaparoAnalyzer,
    CaseContext,
    // Negligence - Causation
    CausationAnalysis,
    ChaseLevel,
    ChildConsiderations,
    ChildStandard,
    ClaimantType,
    CloseTie,
    CommonDutyAnalysis,
    ConspiracyAnalysis,
    ConspiracyFacts,
    ConspiracyIntention,
    ConspiracyType,
    ContractAnalysis,
    ContractType,
    ContributoryNegligence,
    ContributoryNegligenceFacts,
    ControlDegree,
    Damage,
    DamageType,
    DangerType,
    DangerousThing,
    DealingAnalysis,
    DefamationAnalyzer,
    DefamationClaimAnalysis,
    DefamationDefence,
    DefamationDefenceType,
    DefamationFacts,
    DefamationHarmType,
    DefamationRemedy,
    DefamationRemedyType,
    // Defamation
    DefamationType,
    DefenceAnalysis as DefamationDefenceAnalysis,
    DefenceAnalyzer,
    DefenceEffect,
    DefenceType,
    // Negligence - Caparo test
    DutyOfCareAnalysis,
    EconomicDefenceType,
    EconomicLossClaimType,
    EconomicLossFacts,
    EconomicTortAnalyzer,
    EconomicTortDefence,
    // Economic torts
    EconomicTortType,
    // Occupiers' liability
    EntrantStatus,
    EscapeAnalysis,
    EstablishedDutyCategory,
    ExTurpiCausa,
    ExtendedHedleyByrne,
    FactualCausation,
    FairJustReasonable,
    Foreseeability,
    ForeseeabilityFacts,
    HarmType,
    HedleyByrneAnalysis,
    HonestOpinionAnalysis,
    HonestOpinionFacts,
    IndependentContractorAnalysis,
    InducementAnalysis,
    InducementMethod,
    InducingBreachAnalysis,
    InducingBreachFacts,
    InjurySeverity,
    IntentionAnalysis,
    InterferenceAnalysis,
    InterferenceDuration,
    InterferenceSeverity,
    InterferenceType,
    InterveningAct,
    InterveningActType,
    JustificationAnalysis,
    KnowledgeAnalysis,
    LandInterest,
    LegalCausation,
    LimitationAnalysis,
    LocalityCharacter,
    LossAnalysis,
    LossOfChance,
    MaliceAnalysis,
    MaterialContribution,
    MaterialIncrease,
    MeaningAnalysis,
    MischiefAnalysis,
    // Negligence - Claim analysis
    NegligenceClaimAnalysis,
    // Negligence - Defences
    NegligenceDefence,
    NonNaturalUseAnalysis,
    NonVisitorType,
    NuisanceAnalyzer,
    NuisanceDefence,
    NuisanceDefenceType,
    NuisanceRemedy,
    // Nuisance
    NuisanceType,
    OLA1957Analysis,
    OLA1957Facts,
    OLA1984Analysis,
    OLA1984Defence,
    OLA1984DefenceType,
    OLA1984Facts,
    OLADefence,
    OLADefenceType,
    OccupationBasis,
    Occupier,
    OccupierAnalysis,
    OccupiersLiabilityAnalyzer,
    PartyRole,
    PartyType,
    PolicyConsideration,
    PolicyFacts,
    PremisesDanger,
    PremisesInfo,
    PremisesType,
    PrivateNuisanceAnalysis,
    PrivateNuisanceFacts,
    PrivilegeAnalysis,
    PrivilegeFacts,
    PrivilegeType,
    ProfessionalCapacity,
    Proximity,
    ProximityFacts,
    ProximityType,
    // Negligence - Psychiatric injury
    PsychiatricInjuryAnalysis,
    PsychiatricInjuryAnalyzer,
    PsychiatricInjuryFacts,
    PsychiatricVictimType,
    PublicInterestAnalysis,
    PublicInterestFacts,
    PublicNuisanceAnalysis,
    PublicNuisanceFacts,
    PublicNuisanceTest,
    PublicRight,
    PublicationAnalysis,
    PublicationMedium,
    PublisherRole,
    // Negligence - Pure economic loss
    PureEconomicLossAnalysis,
    PureEconomicLossAnalyzer,
    ReasonablePersonTest,
    ReasonablenessAnalysis,
    Relationship,
    ResIpsaEffect,
    ResIpsaFacts,
    ResIpsaLoquitur,
    RylandsDefence,
    RylandsDefenceType,
    RylandsFacts,
    RylandsStanding,
    RylandsVFletcherAnalysis,
    Section1_3Analysis,
    Section1_4Analysis,
    SensitivityAnalysis,
    SeriousHarmAnalysis,
    SkilledVisitorAnalysis,
    SpecialDamageAnalysis,
    // Negligence - Breach
    StandardOfCare,
    StandardType,
    StatementAnalysis,
    StatementType,
    ThingBroughtAnalysis,
    TortError,
    TortParty,
    // Core types
    TortType,
    TruthAnalysis,
    TruthDefenceFacts,
    UnlawfulMeansAnalysis,
    UnlawfulMeansDetail,
    UnlawfulMeansFacts,
    UnlawfulMeansIntention,
    UnlawfulMeansType,
    VisitorAnalysis,
    Volenti,
    VolentiFacts,
    Warning,
    WarningAnalysis,
    WarningType,
    WebsiteOperatorAnalysis,
    WebsiteOperatorFacts,
};

// Criminal law re-exports
pub use criminal::{
    ABHFacts,
    AbuseOfPositionAnalyzer,
    AbuseOfPositionFacts,
    AbuseOfPositionResult,
    ActType,
    // Core types - Actus reus
    ActusReusElement,
    // Error types
    ActusReusError,
    AdmissibilityRisk,
    AggravatedAssaultAnalyzer,
    AggravatedAssaultResult,
    AggravatingFactor,
    AllocationFactor,
    AppealRoute,
    AppropriateAdult,
    ArrestAnalysisResult,
    // Procedure - Arrest
    ArrestAnalyzer,
    ArrestFacts,
    ArrestGrounds,
    ArrestingOfficer,
    // Offence analyzers - Assault
    AssaultBatteryAnalyzer,
    AssaultBatteryResult,
    AssaultFacts,
    AutomatismAnalyzer,
    AutomatismCause,
    AutomatismFacts,
    AutomatismResult,
    AutomatismType,
    BatteryFacts,
    BurglaryAnalysisResult,
    BurglaryAnalyzer,
    BurglaryFacts,
    // Core types - Case law
    CaseCitation,
    // Core types - Causation
    CausationAnalysis as CriminalCausationAnalysis,
    CausationError,
    CautionStatus,
    CommunityOrder,
    CommunityRequirement,
    ConsentDefenceAnalyzer,
    ConsentDefenceFacts,
    ConsentDefenceResult,
    ConsentHarmLevel,
    CorporateManslaughterAnalyzer,
    CorporateManslaughterFacts,
    CorporateManslaughterResult,
    CriminalError,
    CriminalResult,
    // Core types - Procedure
    CriminalStage,
    CulpabilityAssessment,
    CulpabilityCategory,
    CulpabilityFactor,
    CustodialSentence,
    CustodyType,
    DangerousOffenderAnalyzer,
    DangerousOffenderFacts,
    DangerousOffenderResult,
    // Core types - Defences
    DefenceCategory,
    DefenceEffect as CriminalDefenceEffect,
    DefenceError,
    DefenceResult as CriminalDefenceResult,
    DefenceType as CriminalDefenceType,
    DepartureAnalysis,
    DetentionAnalysisResult,
    // Procedure - Detention
    DetentionAnalyzer,
    DetentionDuration,
    DetentionExtension,
    DetentionFacts,
    DetentionReview,
    DetentionRights,
    DirectIntentionFacts,
    DischargeType,
    DiseaseTransmissionAnalyzer,
    DiseaseTransmissionFacts,
    DiseaseTransmissionResult,
    DishonestyAnalysis,
    DuressAnalyzer,
    DuressFacts,
    DuressResult,
    DuressTestFindings,
    DuressType,
    ExclusionGround,
    ExtendedSentence,
    ExtendedSentenceAssessment,
    ExtensionAuthority,
    FactorWeight,
    FactualCausation as CriminalFactualCausation,
    FailingToDiscloseAnalyzer,
    FailingToDiscloseFacts,
    FailingToDiscloseResult,
    // Offence analyzers - Fraud
    FalseRepresentationAnalyzer,
    FalseRepresentationFacts,
    FalseRepresentationResult,
    FineDetails,
    FraudType,
    GrossNegligenceManslaughterAnalyzer,
    GrossNegligenceManslaughterFacts,
    GrossNegligenceManslaughterResult,
    GuiltyPleaFacts,
    GuiltyPleaReduction,
    HandlingAnalysisResult,
    HandlingAnalyzer,
    HandlingFacts,
    HarmAssessment,
    HarmCategory,
    HarmFactor,
    HomicideVerdict,
    InfanticideAnalyzer,
    InfanticideFacts,
    InfanticideResult,
    // Defence analyzers - Insanity
    InsanityAnalyzer,
    InsanityDisposal,
    InsanityFacts,
    InsanityResult,
    InsanityVerdict,
    InterveningAct as CriminalInterveningAct,
    InterveningActType as CriminalInterveningActType,
    InterviewAnalysisResult,
    // Procedure - Interview
    InterviewAnalyzer,
    InterviewConduct,
    InterviewFacts,
    IntoxicationAnalyzer,
    IntoxicationFacts,
    IntoxicationOffenceType,
    IntoxicationResult,
    IntoxicationType,
    InvoluntaryCause,
    JointEnterpriseAnalysis,
    LegalCausation as CriminalLegalCausation,
    LegalRepresentation,
    LifeSentenceAssessment,
    LifeSentenceType,
    MaximumSentence,
    McNaghtenLimbs,
    McNaghtenLimbsResult,
    MensReaAnalysis,
    MensReaError,
    // Core types - Mens rea
    MensReaType,
    MindDiseaseCause,
    MistakeAnalyzer,
    MistakeFacts,
    MistakeResult,
    MistakeType,
    MitigatingFactor,
    // Procedure - Mode of trial
    ModeOfTrialAnalyzer,
    ModeOfTrialFacts,
    ModeOfTrialResult,
    ModePreference,
    MurderAnalysisResult,
    // Offence analyzers - Homicide
    MurderAnalyzer,
    MurderFacts,
    MurderSentencing,
    NecessityCriteria,
    NecessityCriterion,
    NineStepAnalysis,
    NonFatalOffence,
    ObliqueIntentionFacts,
    ObtainingServicesAnalyzer,
    ObtainingServicesFacts,
    ObtainingServicesResult,
    // Core types - Offence classification
    Offence,
    OffenceAnalysisError,
    OffenceBuilder,
    OffenceCategory,
    OffenceClassification,
    OffenceSeverity,
    OmissionDuty,
    PartyLiabilityError,
    // Core types - Parties
    PartyRole as CriminalPartyRole,
    PleaStage,
    PreviousConviction,
    PritchardCriteria,
    ProcedureError,
    PropertyOffence,
    RecklessnessAnalysis,
    RecordingStatus,
    RemandTime,
    ReviewOutcome,
    RightStatus,
    RiskLevel as CriminalRiskLevel,
    RobberyAnalysisResult,
    RobberyAnalyzer,
    RobberyFacts,
    Schedule21StartingPoint,
    SecondaryParticipation,
    Section18Facts,
    Section20Facts,
    // Defence analyzers - General
    SelfDefenceAnalyzer,
    SelfDefenceFacts,
    SelfDefenceResult,
    SelfDefenceType,
    SentenceRange,
    SentenceRecommendation,
    // Core types - Sentencing
    SentenceType,
    SentencingAnalysisResult,
    SentencingCategory,
    SentencingError,
    SentencingFacts,
    // Core types - Sentencing guidelines
    SentencingGuideline,
    // Sentencing
    SentencingGuidelineAnalyzer,
    SpecialWarning,
    StatutorySource,
    SuspendedSentence,
    TheftAnalysisResult,
    // Offence analyzers - Theft
    TheftAnalyzer,
    TheftFacts,
    ThinSkullAnalysis,
    TotalityFacts,
    TrialVenue,
    UnfitnessAnalyzer,
    UnfitnessRecommendation,
    UnfitnessResult,
    UnfitnessToPlead,
    UnlawfulActManslaughterAnalyzer,
    UnlawfulActManslaughterFacts,
    UnlawfulActManslaughterResult,
    UnlawfulArrestConsequences,
    Verdict,
    VoluntarinessAnalysis,
    WithdrawalAnalysis,
    abh_offence,
    abuse_of_position_offence,
    analyze_defence,
    battery_offence,
    burglary_offence,
    common_assault_offence,
    corporate_manslaughter_offence,
    failing_to_disclose_offence,
    false_representation_offence,
    gross_negligence_manslaughter_offence,
    handling_offence,
    insanity_to_defence_result,
    // Offence definitions
    murder_offence,
    obtaining_services_offence,
    robbery_offence,
    section18_offence,
    section20_offence,
    theft_offence,
    unlawful_act_manslaughter_offence,
    voluntary_manslaughter_offence,
};

// Public law re-exports
pub use public_law::{
    // HRA result types
    ArticleAnalysisResult,
    // Human rights analyzers
    ArticleAnalyzer,
    ArticleEngagement,
    BiasType,
    BodyType,
    ClaimantFacts,
    ClaimantType as JrClaimantType,
    ConstitutionalAnalysis,
    ConstitutionalAnalyzer,
    ConstitutionalAssessment,
    ConstitutionalBranch,
    ConstitutionalCase,
    ConstitutionalError,
    // Constitutional principles
    ConstitutionalPrinciple,
    DamagesBasis,
    DamagesClaimFacts,
    DecisionNature,
    DecisionType as JrDecisionType,
    DeclarationOfIncompatibility,
    // ECHR articles
    EchrArticle,
    ExpectationFacts,
    ExpectationType,
    // Judicial review grounds
    GroundOfReview,
    // JR supporting types
    GroundStrength,
    // JR result types
    GroundsAnalysisResult,
    // Judicial review analyzers
    GroundsAnalyzer,
    GroundsFacts,
    HraAnalysisResult,
    HraAnalyzer,
    // HRA duties
    HraDuty,
    // HRA supporting types
    HraFacts,
    HumanRightsError,
    HumanRightsFacts,
    IllegalityFacts,
    IllegalityType,
    InjunctionType,
    InterferenceFacts,
    InterferenceSeverity as HraInterferenceSeverity,
    IrrationalityFacts,
    IrrationalityType,
    // Analysis results
    JrAnalysisResult,
    JrFacts,
    // Remedies
    JrRemedy,
    // Time limits
    JrTimeLimit,
    JudicialReviewAnalyzer,
    JudicialReviewError,
    JustificationFacts,
    LegitimateAim,
    PrerogativeAnalysis,
    PrerogativeAnalyzer,
    PrerogativePower,
    ProceduralFacts,
    ProceduralType,
    PromiseOrPractice,
    // Proportionality
    ProportionalityAnalysis,
    // Core types - Public bodies and decisions
    PublicBodyType,
    // Case citations
    PublicLawCitation,
    // Error types
    PublicLawError,
    PublicLawResult,
    RemediesAnalysisResult,
    RemediesAnalyzer,
    RemediesFacts,
    RespondentFacts,
    RuleOfLawAnalysis,
    RuleOfLawAnalyzer,
    RuleOfLawAssessment,
    RuleOfLawFactors,
    RuleOfLawPrinciple,
    RuleOfLawViolation,
    Section3Analyzer,
    Section3Facts,
    Section3Outcome,
    Section3Result,
    Section4Analyzer,
    Section4Facts,
    Section4Result,
    Section6Analyzer,
    Section6Authority,
    Section6Result,
    SeparationAnalysis,
    SeparationAnalyzer,
    SeparationConflict,
    // Constitutional analysis types
    SovereigntyAnalysis,
    // Constitutional analyzers
    SovereigntyAnalyzer,
    SovereigntyLimitation,
    SpecificLimit,
    StandingAnalysisResult,
    StandingAnalyzer as JrStandingAnalyzer,
    StandingFacts,
    // Standing
    StandingType,
    StatutoryPower,
    SuccessLikelihood,
    TimeLimitAnalyzer,
    TimeLimitFacts,
    TimeLimitResult,
    VictimType,
    ViolationSeverity as ConstitutionalViolationSeverity,
    create_cpr_54_statute,
    // Statute builders
    create_hra_statute,
    create_sca_1981_statute,
};

// Land law re-exports
pub use land_law::{
    AcquisitionType,
    AlterationAnalyzer,
    AlterationFacts,
    AlterationResult,
    AlterationType,
    BreachType,
    // Core types - Estates
    CoOwnershipType,
    // Error types
    ConveyancingError,
    // Conveyancing
    ConveyancingSearch,
    ConveyancingStage,
    // Core types - Interests
    Covenant,
    CovenantAnalysisResult,
    CovenantAnalyzer,
    CovenantFacts,
    CovenantNature,
    // Interests analyzers
    CreationFacts,
    DefaultType,
    Easement,
    EasementAnalysisResult,
    EasementAnalyzer,
    EasementBenefit,
    EasementCreation,
    EasementFacts,
    EasementType,
    EnforcementMethod,
    EstateError,
    EstateType,
    // Registration analyzers
    FirstRegistrationAnalyzer,
    FirstRegistrationFacts,
    FirstRegistrationResult,
    // Core types - Registration
    FirstRegistrationTrigger,
    ForfeitureAnalysisResult,
    ForfeitureAnalyzer,
    ForfeitureFacts,
    ForfeitureRisk,
    FreeholdAnalysisResult,
    FreeholdAnalyzer,
    FreeholdEstate,
    FreeholdFacts,
    InterestCategory,
    InterestError,
    InterestType,
    LandChargeClass,
    LandContract,
    LandLawCase,
    LandLawError,
    LandLawResult,
    LeaseDuration,
    // Estates analyzers
    LeaseOrLicenceAnalyzer,
    LeaseOrLicenceFacts,
    LeaseOrLicenceResult,
    LeaseUseType,
    LeaseholdAnalysisResult,
    LeaseholdAnalyzer,
    LeaseholdEstate,
    LeaseholdFacts,
    LegalEasementFacts,
    LegalOrEquitable,
    LicenceException,
    Lta1954Analyzer,
    Lta1954Facts,
    Lta1954Ground,
    Lta1954Result,
    Mortgage,
    MortgageAnalysisResult,
    MortgageAnalyzer,
    MortgageError,
    MortgageFacts,
    MortgageRemedy,
    OccupationFacts,
    OverridingInterest,
    OverridingInterestAnalyzer,
    OverridingInterestFacts,
    OverridingInterestResult,
    Owner,
    OwnerType,
    PeriodicTenancy,
    PriorityAnalyzer,
    PriorityBasis,
    PriorityFacts,
    PriorityResult,
    PropertyAddress,
    RegisterEntry,
    RegisterProtection,
    RegistrationError,
    RegistrationStatus,
    ReliefLikelihood,
    RestrictionType,
    // Trusts of land
    Section15Factor,
    ShortLeaseFacts,
    TitleClass,
    TitleGuarantee,
    TitleQuality,
    TolataClaim,
    TolataOrder,
    TransferDeed,
    TrustOfLandType,
    UndueInfluenceRisk,
    UnregisteredLandAnalyzer,
    UnregisteredLandFacts,
    UnregisteredLandResult,
    // Statute builders
    create_lpa_1925_statute,
    create_lra_2002_statute,
    create_lta_1954_statute,
    create_tolata_statute,
};
