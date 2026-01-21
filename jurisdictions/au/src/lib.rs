//! Australian Jurisdiction Support for Legalis-RS
//!
//! Comprehensive implementation of Australian law including:
//! - Constitutional law (Commonwealth powers, implied rights)
//! - Competition law (Competition and Consumer Act 2010, Part IV)
//! - Contract law (with Australian Consumer Law)
//! - Tort law (with Civil Liability Act reforms)
//! - Employment law (Fair Work Act)
//! - Criminal law (Commonwealth and state)
//! - Family law (Family Law Act 1975)
//! - Property law (Torrens, native title)
//! - Corporate law (Corporations Act 2001)
//! - Financial services (Chapter 7, ASIC, AUSTRAC)
//! - Privacy law (Privacy Act 1988, APPs)
//! - Intellectual property (Patents, Trade Marks, Copyright, Designs)
//! - Tax law (Income tax, GST, CGT, FBT)
//! - Superannuation law (SG, SIS, SMSF)
//! - Mining and resources law (tenures, native title, environment)
//! - Immigration law (Migration Act 1958, visa system, citizenship)
//! - Consumer law (Product safety, ACCC enforcement, unsolicited agreements)
//!
//! ## Jurisdictional Structure
//!
//! Australia has a federal system with:
//! - Commonwealth (federal) law
//! - Six states (NSW, Vic, Qld, SA, WA, Tas)
//! - Two territories (NT, ACT)
//!
//! ## Key Legislation
//!
//! - Constitution (1901)
//! - Competition and Consumer Act 2010 (incl. ACL)
//! - Corporations Act 2001 (incl. Chapter 7 - Financial Services)
//! - Fair Work Act 2009
//! - Family Law Act 1975
//! - Native Title Act 1993
//! - Privacy Act 1988
//! - Anti-Money Laundering and Counter-Terrorism Financing Act 2006
//! - Banking Act 1959
//! - Income Tax Assessment Act 1997
//! - A New Tax System (Goods and Services Tax) Act 1999
//! - Fringe Benefits Tax Assessment Act 1986
//! - Superannuation Guarantee (Administration) Act 1992
//! - Superannuation Industry (Supervision) Act 1993
//! - Patents Act 1990
//! - Trade Marks Act 1995
//! - Copyright Act 1968
//! - Designs Act 2003
//! - Migration Act 1958
//! - Australian Citizenship Act 2007
//! - Civil Liability Acts (various states)
//!
//! ## Key Cases
//!
//! - Mabo v Queensland (No 2) (1992) - Native title
//! - Lange v ABC (1997) - Implied freedom
//! - Sullivan v Moody (2001) - Duty of care
//! - Work Choices (2006) - Corporations power
//! - ACCC v Visy (2007) - Cartel conduct
//! - ACCC v Flight Centre (2016) - Price fixing

#![allow(missing_docs)]

pub mod common;
pub mod competition;
pub mod constitution;
pub mod consumer_law;
pub mod contract;
pub mod corporate;
pub mod criminal;
pub mod employment;
pub mod family;
pub mod financial_services;
pub mod immigration;
pub mod intellectual_property;
pub mod mining_resources;
pub mod privacy;
pub mod property;
pub mod reasoning;
pub mod superannuation;
pub mod tax;
pub mod tort;

// Re-export commonly used types
pub use common::{AustralianCalendar, AustralianCase, AustralianHoliday, Court, StateTerritory};
pub use competition::{
    CartelAnalyzer, CartelType, CompetitionValidator, ExclusiveDealingAnalyzer,
    MarketPowerAnalyzer, MergerAnalyzer, RelevantMarket, Undertaking, ValidationResult,
};
pub use constitution::{
    CharacterizationAnalyzer, CommonwealthPower, ConstitutionalProvision, ExpressRight,
    ImpliedRight, InconsistencyAnalyzer, PoliticalCommunicationAnalyzer,
};
pub use consumer_law::{
    AclContravention,
    // Error types
    ConsumerLawError,
    CountryOfOriginAssessment,
    CountryOfOriginClaim,
    CountryOfOriginClaimDetails,
    CourtUndertaking,
    EnforcementActionType,
    InfringementNotice,
    InfringementNoticeAssessment,
    InfringementNoticeStatus,
    InjuryReport,
    InjuryType,
    LayByAgreement,
    LayByComplianceResult,
    LayByStatus,
    PermittedContactTime,
    ProductCategory,
    ProductRecall,
    ProductSafetyAssessment,
    ProductSafetyStatus,
    RecallType,
    RecipientType,
    SafetyStandard,
    UnsolicitedAgreementComplianceResult,
    UnsolicitedAgreementType,
    UnsolicitedConsumerAgreement,
    assess_infringement_notice,
    // Product safety
    assess_product_safety,
    calculate_cooling_off_end_date,
    // ACCC enforcement
    calculate_penalty,
    // Country of origin
    validate_country_of_origin_claim,
    validate_injury_report_timing,
    // Lay-by
    validate_layby_agreement,
    // Unsolicited agreements
    validate_unsolicited_agreement,
};
pub use contract::{
    ConsumerAnalyzer, ConsumerGuarantee, FormationAnalyzer, GuaranteeAnalyzer,
    MisleadingConductAnalyzer, UnfairTermsAnalyzer,
};
pub use corporate::{DirectorsDutiesAnalyzer, DirectorsDuty, InsolventTradingAnalyzer};
pub use criminal::{
    Defence, FaultElement, OffenceAnalyzer, OffenceCategory, SentenceType, SentencingAnalyzer,
};
pub use employment::{
    CompensationCalculator, EligibilityAnalyzer, GeneralProtectionsAnalyzer,
    NationalEmploymentStandard, NesAnalyzer, UnfairDismissalAnalyzer,
};
pub use family::{DivorceAnalyzer, ParentingAnalyzer, PropertyAnalyzer as FamilyPropertyAnalyzer};
pub use financial_services::{
    // Advice
    AdviceError,
    AdviceType,
    AfsLicensingError,
    // AFS Licensing
    AfslCondition,
    AfslLicense,
    // AML/CTF
    AmlCtfError,
    AuCustomerDueDiligence,
    AustracCompliance,
    // Banking
    AuthorizedDepositInstitution,
    AuthorizedRepresentative,
    AuthorizedService,
    BankingError,
    BestInterestsAssessment,
    CapitalRequirement,
    CddLevel,
    // Core types
    ClientClassification,
    ClientType,
    CompensationArrangement,
    // Managed Investments
    CompliancePlan,
    ConflictedRemuneration,
    DisputeResolution,
    FinancialServicesError,
    FinancialServicesGuide,
    FinancialServicesProvider,
    GeneralObligationsCompliance,
    LicenseStatus,
    LiquidityRequirement,
    ManagedInvestmentScheme,
    ManagedInvestmentsError,
    ProductDisclosureStatement,
    ProductType,
    ResponsibleEntity,
    StatementOfAdvice,
};
pub use immigration::{
    AssessmentOutcome,
    CITIZENSHIP_LAST_12_MONTHS_DAYS,
    CITIZENSHIP_RESIDENCE_DAYS,
    CharacterConcern,
    CharacterConcernSeverity,
    CharacterTestGround,
    CharacterTestResult,
    CitizenshipApplication,
    CitizenshipApplicationStatus,
    CitizenshipEligibilityResult,
    CitizenshipStream,
    EnglishLanguageLevel,
    EnglishLanguageTest,
    EnglishTestResult,
    // Error types
    ImmigrationError,
    ImmigrationStatus,
    OccupationAssessment,
    // Constants
    POINTS_PASS_MARK,
    PointsTestResult,
    ResidenceRequirement,
    SkilledOccupationList,
    Sponsor,
    SponsorComplianceHistory,
    SponsorType,
    SponsorValidationResult,
    VisaApplication,
    // Core types
    VisaCategory,
    VisaCondition,
    VisaEligibilityResult,
    VisaHolder,
    VisaStatus,
    VisaSubclass,
    // Character test
    assess_character_test,
    // Citizenship
    assess_citizenship_by_conferral,
    assess_citizenship_test,
    assess_employer_sponsored_eligibility,
    // Points test
    assess_points_test,
    // Visa eligibility
    assess_skilled_visa_eligibility,
    calculate_age_points,
    calculate_australian_employment_points,
    calculate_education_points,
    calculate_english_points,
    calculate_overseas_employment_points,
    // Sponsor validation
    validate_sponsor,
    validate_visa_condition_compliance,
};
pub use intellectual_property::{
    AbsoluteGrounds,
    CopyrightDuration,
    // Copyright
    CopyrightWork,
    // Designs
    Design,
    DesignApplication,
    DesignExamination,
    DesignType,
    FairDealingPurpose,
    // Error types
    IpError,
    IpOwner,
    // Core types
    IpRight,
    IpRightType,
    LikelihoodOfConfusion as TradeMarkConfusion,
    MannerOfManufacture,
    MoralRights,
    // Patents
    Patent,
    PatentApplication,
    PatentClaim,
    Patentability,
    RegistrationStatus as IpRegistrationStatus,
    RelativeGrounds,
    // Trade Marks
    TradeMark,
    TradeMarkApplication,
    TradeMarkClass,
    WorkType,
};
pub use mining_resources::{
    ClosurePlanValidationResult,
    EiaRequirement,
    EnvironmentalApproval,
    ExplorationValidationResult,
    HeritageSite,
    MineClosurePlan,
    MineralType,
    // Error types
    MiningError,
    MiningJurisdiction,
    MiningProject,
    // Core types
    MiningTenure,
    NativeTitleStatus,
    ProjectPhase,
    RoyaltyCalculation,
    RoyaltyType,
    TenureStatus,
    TenureType,
    TenureValidationResult,
    ValidationIssue,
    calculate_royalty,
    determine_eia_requirements,
    validate_closure_plan,
    validate_exploration_programme,
    // Validation
    validate_tenure,
};
pub use privacy::{
    App, AppAnalyzer, AppEntity, ComplianceReport, DataBreach, DataBreachAnalyzer, Organisation,
    PrivacyError, PrivacyPolicy, PrivacyValidator, SecurityMeasures,
};
pub use property::{IndefeasibilityAnalyzer, NativeTitleAnalyzer, TorrensPrinciple};
pub use reasoning::{AustralianReasoningEngine, ConstitutionalVerifier};
pub use superannuation::{
    BeneficiaryNomination,
    BeneficiaryRelationship,
    BenefitPaymentType,
    // Benefits
    BenefitReleaseAssessment,
    ConditionOfRelease,
    Contribution,
    // Contribution caps
    ContributionCapAssessment,
    ContributionCaps,
    ContributionType,
    DeathBenefitDistribution,
    EmployeeSgEligibility,
    EmploymentType,
    FundMember,
    FundType,
    InHouseAssetCalculation,
    InvestmentStrategy,
    MemberCategory,
    PensionDrawdown,
    // SG contributions
    SgCalculation,
    SgQuarter,
    SgShortfallCalculation,
    // SMSF
    SmsfComplianceAssessment,
    // Error types
    SuperannuationError,
    // Core types
    SuperannuationFund,
    assess_benefit_release,
    assess_contribution_caps,
    assess_smsf_compliance,
    calculate_pension_drawdown,
    calculate_sg_contribution,
    calculate_sg_shortfall,
    check_sg_eligibility,
    validate_benefit_payment,
    validate_contribution,
    validate_member_count,
    validate_trustee_eligibility,
};
pub use tax::{
    Abn,
    // CGT
    CgtAsset,
    CgtCalculation,
    CgtDiscount,
    CgtEvent,
    CgtExemption,
    Deduction,
    EntityType,
    // Core types
    FinancialYear,
    GstCalculation,
    GstRegistration,
    GstStatus,
    IncomeCategory,
    InputTaxCredit,
    SupplyType,
    TaxAgent,
    TaxCalculation,
    // Error types
    TaxError,
    TaxFileNumber,
    TaxOffset,
    TaxPayer,
    // Income tax
    TaxableIncome,
    // GST
    TaxableSupply,
    calculate_capital_gain,
    calculate_company_tax,
    calculate_gst,
    calculate_individual_tax,
    calculate_input_tax_credit,
    validate_bas_lodgement,
    validate_cgt_event,
    validate_deduction,
};
pub use tort::{
    BreachAnalyzer as TortBreachAnalyzer, CausationAnalyzer, DefamationAnalyzer,
    DutyOfCareAnalyzer, NegligenceAnalyzer,
};

use legalis_core::Statute;

// ============================================================================
// Main Statute Builders
// ============================================================================

/// Create Australian Constitution statute
pub fn create_constitution() -> Statute {
    constitution::create_constitution_statute()
}

/// Create Australian Consumer Law statute
pub fn create_acl() -> Statute {
    contract::create_acl_statute()
}

/// Create Civil Liability Act for a state
pub fn create_cla(state: &StateTerritory) -> Statute {
    tort::create_civil_liability_act(state)
}

/// Create Fair Work Act statute
pub fn create_fair_work_act() -> Statute {
    employment::create_fair_work_act()
}

/// Create Criminal Code Act statute
pub fn create_criminal_code_act() -> Statute {
    criminal::create_criminal_code_act()
}

/// Create Family Law Act statute
pub fn create_family_law_act() -> Statute {
    family::create_family_law_act()
}

/// Create Native Title Act statute
pub fn create_native_title_act() -> Statute {
    property::create_native_title_act()
}

/// Create Corporations Act statute
pub fn create_corporations_act() -> Statute {
    corporate::create_corporations_act()
}

/// Create Privacy Act 1988 statute
pub fn create_privacy_act() -> Statute {
    privacy::create_privacy_act()
}

/// Create Corporations Act 2001 Chapter 7 (Financial Services) statute
pub fn create_corporations_act_ch7() -> Statute {
    financial_services::create_corporations_act_chapter_7()
}

/// Create AML/CTF Act 2006 statute
pub fn create_aml_ctf_act() -> Statute {
    financial_services::create_aml_ctf_act()
}

/// Create Patents Act 1990 statute
pub fn create_patents_act() -> Statute {
    intellectual_property::create_patents_act()
}

/// Create Trade Marks Act 1995 statute
pub fn create_trade_marks_act() -> Statute {
    intellectual_property::create_trade_marks_act()
}

/// Create Copyright Act 1968 statute
pub fn create_copyright_act() -> Statute {
    intellectual_property::create_copyright_act()
}

/// Create Designs Act 2003 statute
pub fn create_designs_act() -> Statute {
    intellectual_property::create_designs_act()
}

/// Create Income Tax Assessment Act 1997 statute
pub fn create_itaa_1997() -> Statute {
    tax::create_itaa_1997()
}

/// Create GST Act 1999 statute
pub fn create_gst_act() -> Statute {
    tax::create_gst_act()
}

/// Create FBT Act 1986 statute
pub fn create_fbt_act() -> Statute {
    tax::create_fbt_act()
}

/// Create Superannuation Guarantee (Administration) Act 1992 statute
pub fn create_sg_act() -> Statute {
    superannuation::create_sg_act()
}

/// Create Superannuation Industry (Supervision) Act 1993 statute
pub fn create_sis_act() -> Statute {
    superannuation::create_sis_act()
}

/// Create Native Title Act 1993 (mining provisions) statute
pub fn create_native_title_act_mining() -> Statute {
    mining_resources::create_native_title_act_mining()
}

/// Create EPBC Act 1999 (mining provisions) statute
pub fn create_epbc_act_mining() -> Statute {
    mining_resources::create_epbc_act_mining()
}

/// Create state Mining Act statute
pub fn create_mining_act(jurisdiction: MiningJurisdiction) -> Statute {
    mining_resources::create_mining_act(jurisdiction)
}

/// Create Migration Act 1958 statute
pub fn create_migration_act() -> Statute {
    immigration::create_migration_act()
}

/// Create Australian Citizenship Act 2007 statute
pub fn create_citizenship_act() -> Statute {
    immigration::create_citizenship_act()
}

/// Create Product Safety provisions statute
pub fn create_product_safety_statute() -> Statute {
    consumer_law::create_product_safety_statute()
}

/// Create ACCC enforcement provisions statute
pub fn create_accc_enforcement_statute() -> Statute {
    consumer_law::create_accc_enforcement_statute()
}

/// Create Unsolicited Consumer Agreements statute
pub fn create_unsolicited_agreements_statute() -> Statute {
    consumer_law::create_unsolicited_agreements_statute()
}

/// Create Country of Origin statute
pub fn create_country_of_origin_statute() -> Statute {
    consumer_law::create_country_of_origin_statute()
}

/// Create all major Australian statutes
pub fn create_major_statutes() -> Vec<Statute> {
    let mut statutes = vec![
        // Constitutional
        create_constitution(),
        // Criminal
        create_criminal_code_act(),
        criminal::create_crimes_act(),
        // Employment
        create_fair_work_act(),
        // Family
        create_family_law_act(),
        family::create_child_support_act(),
        // Property
        create_native_title_act(),
        // Corporate
        create_corporations_act(),
        corporate::create_asic_act(),
        // Consumer/Competition
        contract::create_acl_statute(),
        contract::create_cca_statute(),
        // Privacy
        create_privacy_act(),
        // Financial Services
        create_corporations_act_ch7(),
        create_aml_ctf_act(),
        // Intellectual Property
        create_patents_act(),
        create_trade_marks_act(),
        create_copyright_act(),
        create_designs_act(),
        // Tax
        create_itaa_1997(),
        create_gst_act(),
        create_fbt_act(),
        // Superannuation
        create_sg_act(),
        create_sis_act(),
        // Mining (Commonwealth)
        create_native_title_act_mining(),
        create_epbc_act_mining(),
        // Immigration
        create_migration_act(),
        create_citizenship_act(),
        // Consumer Law (Product Safety, ACCC Enforcement)
        create_product_safety_statute(),
        create_accc_enforcement_statute(),
        create_unsolicited_agreements_statute(),
        create_country_of_origin_statute(),
    ];

    // Civil Liability Acts for each state
    for state in StateTerritory::all() {
        statutes.push(tort::create_civil_liability_act(state));
        statutes.push(tort::create_defamation_act(state));
        statutes.push(contract::create_sale_of_goods_statute(state));
    }

    statutes
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_constitution() {
        let statute = create_constitution();
        assert!(statute.id.contains("CONST"));
    }

    #[test]
    fn test_create_acl() {
        let statute = create_acl();
        assert!(statute.id.contains("ACL"));
    }

    #[test]
    fn test_create_major_statutes() {
        let statutes = create_major_statutes();
        // Constitution + ACL + CCA + (CLA + Defamation + SOG) * 8 states = 27
        assert!(statutes.len() >= 20);
    }

    #[test]
    fn test_state_territory_count() {
        assert_eq!(StateTerritory::all().len(), 8);
        assert_eq!(StateTerritory::states().len(), 6);
        assert_eq!(StateTerritory::territories().len(), 2);
    }
}
