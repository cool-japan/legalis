//! # India Jurisdiction - Legalis-RS
//!
//! Comprehensive modeling of Indian law for the Legalis legal reasoning framework.
//!
//! ## Legal System Overview
//!
//! India has a **common law** legal system inherited from British rule, with a
//! written constitution that is the supreme law of the land. The legal system
//! features a three-tier court structure: Supreme Court, High Courts, and
//! subordinate courts.
//!
//! ## Key Characteristics
//!
//! - **Constitution**: Constitution of India, 1950 (longest written constitution)
//! - **Legal Tradition**: Common Law with statutory modifications
//! - **Court System**: Supreme Court → High Courts → District Courts
//! - **Languages**: English (official for courts), Hindi (national)
//! - **Citation Formats**: AIR, SCC, SCR, High Court citations
//!
//! ## Module Coverage
//!
//! ### Data Protection
//!
//! The [`data_protection`] module implements India's Digital Personal Data
//! Protection Act, 2023 (DPDPA), which came into effect in 2024.
//!
//! **Key Concepts:**
//! - Data Principal (individual whose data is processed)
//! - Data Fiduciary (entity determining purpose of processing)
//! - Significant Data Fiduciary (SDF) with additional obligations
//! - Consent Manager (registered consent facilitator)
//!
//! ```rust
//! use legalis_in::data_protection::*;
//! use chrono::NaiveDate;
//!
//! // Validate SDF compliance
//! let fiduciary = DataFiduciary {
//!     registration_number: Some("REG001".to_string()),
//!     name: "TechCorp India".to_string(),
//!     category: DataFiduciaryCategory::Significant,
//!     principal_place: "Bengaluru".to_string(),
//!     contact_email: "privacy@techcorp.in".to_string(),
//!     dpo: Some(DataProtectionOfficer {
//!         name: "Rahul Sharma".to_string(),
//!         designation: "Chief Privacy Officer".to_string(),
//!         contact_email: "dpo@techcorp.in".to_string(),
//!         phone: "+91-9876543210".to_string(),
//!         based_in_india: true,
//!         appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     }),
//!     consent_manager: None,
//!     registration_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
//! };
//!
//! let report = validate_fiduciary_compliance(&fiduciary);
//! assert!(report.compliant);
//! ```
//!
//! ## Citation System
//!
//! The [`citation`] module provides Indian legal citation formatting:
//!
//! ```rust
//! use legalis_in::citation::{Citation, cite};
//!
//! // Statutory citation
//! let dpdpa_cite = cite::dpdpa(4);
//! assert_eq!(dpdpa_cite.format(), "Digital Personal Data Protection Act, 2023, s. 4");
//!
//! // Supreme Court AIR citation
//! let sc_cite = cite::air_sc(2024, 1234);
//! assert_eq!(sc_cite.format(), "AIR 2024 SC 1234");
//!
//! // SCC citation
//! let scc_cite = Citation::scc(2024, 5, 789);
//! assert_eq!(scc_cite.format(), "(2024) 5 SCC 789");
//! ```
//!
//! ## Currency Formatting
//!
//! The [`common`] module provides Indian currency formatting with lakhs and crores:
//!
//! ```rust
//! use legalis_in::common::{InrAmount, format_inr, to_lakhs, to_crores};
//!
//! let amount = InrAmount::new(25000000.0); // 2.5 crore
//! assert_eq!(amount.format(), "₹2,50,00,000");
//! assert_eq!(to_crores(25000000.0), 2.5);
//!
//! let lakhs_amount = InrAmount::new(500000.0); // 5 lakhs
//! assert_eq!(lakhs_amount.format_lakhs(), "₹5.00 lakhs");
//! ```
//!
//! ## Legal Framework Coverage
//!
//! | Domain | Primary Legislation | Module |
//! |--------|---------------------|--------|
//! | Data Protection | DPDPA 2023 | [`data_protection`] |
//! | Companies | Companies Act 2013 | [`companies`] |
//! | Contracts | Indian Contract Act 1872 | [`contract`] |
//! | IT/Cyber | IT Act 2000 | *Planned* |
//! | GST | CGST Act 2017 | *Planned* |
//! | Labour | Labour Codes 2020 | *Planned* |
//! | Criminal | BNS 2023 | *Planned* |
//! | Constitution | Constitution of India | *Planned* |
//!
//! ## DPDPA Penalty Structure (Section 33)
//!
//! | Tier | Maximum Penalty | Violations |
//! |------|-----------------|------------|
//! | Principal | Rs. 10,000 | Data principal duty breaches |
//! | Tier 1 | Rs. 50 crore | Security safeguard failures |
//! | Tier 2 | Rs. 200 crore | Breach notification, child data |
//! | Tier 3 | Rs. 250 crore | Processing without lawful grounds |
//!
//! ## Data Principal Rights (Chapter III, DPDPA)
//!
//! | Right | Section | Description |
//! |-------|---------|-------------|
//! | Access | 11 | Summary of personal data and processing |
//! | Correction | 12 | Correct inaccurate/misleading data |
//! | Erasure | 12 | Erase data no longer necessary |
//! | Grievance | 13 | Redressal mechanism |
//! | Nomination | 14 | Nominate for incapacity/death |
//!
//! ## Indian Number System
//!
//! India uses a unique numbering system for large amounts:
//!
//! | Term | Value | Example |
//! |------|-------|---------|
//! | Lakh (लाख) | 1,00,000 | Rs. 5 lakhs = Rs. 5,00,000 |
//! | Crore (करोड़) | 1,00,00,000 | Rs. 2 crore = Rs. 2,00,00,000 |
//! | Arab (अरब) | 1,00,00,00,000 | Rs. 1 arab = Rs. 100 crore |
//!
//! ## Court Hierarchy
//!
//! ```text
//! Supreme Court of India (Article 124-147)
//!        │
//!        ├── 25 High Courts (Article 214-231)
//!        │       │
//!        │       ├── District Courts (Article 233-237)
//!        │       │       │
//!        │       │       └── Subordinate Courts
//!        │       │
//!        │       └── Family Courts, Labour Courts, Consumer Forums
//!        │
//!        └── Tribunals (SAT, CAT, NCLT, NCLAT, etc.)
//! ```
//!
//! ## References
//!
//! - [Digital Personal Data Protection Act, 2023](https://www.meity.gov.in/writereaddata/files/Digital%20Personal%20Data%20Protection%20Act%202023.pdf)
//! - [Companies Act, 2013](https://www.mca.gov.in/content/mca/global/en/acts-rules/ebooks/acts.html)
//! - [Indian Contract Act, 1872](https://legislative.gov.in/actsofparliamentfromtheyear/indian-contract-act-1872)
//! - [Supreme Court of India](https://main.sci.gov.in/)

#![allow(missing_docs)]

// Core modules
pub mod citation;
pub mod common;
pub mod companies;
pub mod constitution;
pub mod contract;
pub mod criminal;
pub mod data_protection;
pub mod gst;
pub mod it_act;
pub mod labour_codes;

// Re-export citation types
pub use citation::{Citation, CitationType, Court, acts, cite};

// Re-export common utilities
pub use common::{
    // Types
    Address,
    DeadlineType,
    FinancialYear,
    // Names
    IndianName,
    IndianNameFormatter,
    InrAmount,
    NationalHoliday,
    State,
    Title,
    // Dates
    business_days_between,
    calculate_deadline,
    // Currency
    crores_to_rupees,
    format_crores,
    format_inr,
    format_inr_precision,
    format_lakhs,
    is_national_holiday,
    is_weekend,
    is_working_day,
    lakhs_to_rupees,
    national_holidays_in_year,
    next_working_day,
    parse_inr,
    previous_working_day,
    to_crores,
    to_lakhs,
    working_days_between,
};

// Re-export data protection types
pub use data_protection::{
    // Types
    ChildDataProcessing,
    ConsentManager,
    ConsentRecord,
    CrossBorderTransfer,
    DataFiduciary,
    DataFiduciaryCategory,
    DataPrincipalDuty,
    DataPrincipalRight,
    DataProtectionOfficer,
    // Report
    DpdpaComplianceReport,
    // Errors
    DpdpaError,
    DpdpaResult,
    LawfulPurpose,
    LegitimateUseType,
    PenaltyTier,
    ProcessingRecord,
    SdfCriteria,
    // Validators
    check_sdf_status,
    get_obligations,
    get_principal_duties,
    get_principal_rights,
    validate_child_processing,
    validate_consent,
    validate_cross_border_transfer,
    validate_fiduciary_compliance,
    validate_processing_record,
    validate_retention,
};

// Re-export companies module types
pub use companies::{
    // Types
    AnnualFilingType,
    Committee,
    CommitteeType,
    // Errors
    CompaniesActError,
    CompaniesActResult,
    Company,
    CompanyStatus,
    CompanyType,
    // Report
    ComplianceReport,
    CsrCategory,
    CsrObligation,
    DinStatus,
    Director,
    DirectorCategory,
    Kmp,
    KmpType,
    PenaltyInfo,
    RelatedPartyTransactionType,
    RelatedPartyType,
    ResolutionType,
    ShareCapitalType,
    ShareClass,
    Shareholder,
    ShareholderCategory,
    SpecialResolutionMatter,
    // Validators
    calculate_filing_penalty,
    check_filing_deadline,
    get_compliance_checklist,
    validate_agm,
    validate_board_composition,
    validate_board_meetings,
    validate_buyback,
    validate_committees,
    validate_company_formation,
    validate_csr_compliance,
    validate_kmp_appointments,
    validate_ordinary_resolution,
    validate_resolution,
};

// Re-export contract module types
pub use contract::{
    // Types
    AgentAuthority,
    AgentType,
    BreachType,
    ConsentVitiator,
    Consideration,
    ConsiderationTiming,
    ConsiderationType,
    ContingentContract,
    Contract,
    // Errors
    ContractActError,
    ContractActResult,
    ContractEssentials,
    ContractParty,
    ContractStatus,
    ContractType,
    ContractValidityReport,
    DamagesType,
    DischargeMode,
    IncompetentParty,
    LegalityCheck,
    PartyType,
    QuasiContractType,
    Remedy,
    VoidAgreementType,
    // Validators
    calculate_damages,
    check_frustration,
    get_limitation_period as get_contract_limitation_period,
    get_quasi_contract_obligation,
    validate_agent_authority,
    validate_consent as validate_contract_consent,
    validate_consideration,
    validate_contingent_contract,
    validate_contract_formation,
    validate_legality,
    validate_liquidated_damages,
    validate_performance,
    validate_void_agreements,
};

// Re-export IT Act module types
pub use it_act::{
    // Types
    AdjudicatingOfficerPower,
    BailStatus,
    CaFunction,
    CatJurisdiction,
    ComputerOffence,
    ConsentRequirement,
    CyberCrimeCategory,
    DigitalCertificate,
    DigitalSignatureType,
    EcommerceModel,
    ElectronicRecord,
    ElectronicRecordType,
    IntermediaryComplianceCheck,
    IntermediaryType,
    // Errors
    ItActError,
    ItActResult,
    ItComplianceReport,
    NetworkProviderLiability,
    PenaltyInfo as ItPenaltyInfo,
    Punishment,
    SafeHarborConditions,
    SensitivePersonalData,
    // Validators
    calculate_section43_compensation,
    classify_cyber_crime,
    get_jurisdiction,
    get_limitation_period as get_cyber_limitation_period,
    validate_certificate,
    validate_data_protection_compliance,
    validate_ecommerce_compliance,
    validate_intermediary_compliance,
    validate_safe_harbor,
    validate_takedown_compliance,
};

// Re-export GST module types
pub use gst::{
    // Types
    BlockedItcReason,
    CompensationCess,
    CompositionBusinessType,
    CompositionScheme,
    EwayBill,
    EwayDocType,
    FilingStatus,
    // Errors
    GstComplianceReport,
    GstError,
    GstRate,
    GstRegistration,
    GstResult,
    GstState,
    Gstin,
    HsnSacCode,
    HsnSacType,
    InputTaxCredit,
    Invoice,
    InvoiceType,
    ItcType,
    ItcUtilization,
    ItcUtilizationPlan,
    ItcUtilizationStep,
    NatureOfBusiness,
    PenaltyInfo as GstPenaltyInfo,
    PenaltyType as GstPenaltyType,
    RefundType,
    RegistrationStatus,
    RegistrationType,
    ReturnComplianceStatus,
    ReturnStatus,
    ReturnType,
    ReverseCharge,
    SupplyCategory,
    SupplyType,
    TaxLiability,
    TaxpayerCategory,
    TransportMode,
    // Validators
    calculate_interest as calculate_gst_interest,
    calculate_late_fee,
    calculate_tax,
    determine_supply_type,
    get_return_due_date,
    is_refund_within_time_limit,
    validate_composition_eligibility,
    validate_eway_bill_requirement,
    validate_gst_compliance,
    validate_gstin,
    validate_invoice,
    validate_itc_eligibility,
    validate_itc_utilization,
    validate_registration_requirement,
    validate_return_filing,
    validate_reverse_charge,
};

// Re-export Labour Codes module types
pub use labour_codes::{
    // Code on Wages types
    Bonus,
    ComplianceChecklistItem,
    // OSH Code types
    ContractLabour,
    ContractWorkNature,
    DeductionType,
    // Industrial Relations Code types
    DisputeStage,
    DisputeType,
    // Code on Social Security types
    EpfContribution,
    EsiContribution,
    EstablishmentType,
    GeographicalArea,
    GigWorker,
    Gratuity,
    IndustrialActionType,
    IndustrialDispute,
    InterStateMigrantWorker,
    // Errors
    LabourCodeError,
    LabourCodeResult,
    LabourComplianceCheck,
    LabourComplianceReport,
    LabourPenaltyInfo,
    Layoff,
    LayoffReason,
    LeaveProvisions,
    MaternityBenefit,
    MaternityBenefitType,
    MinimumWageFloor,
    Overtime,
    PaymentMode,
    PaymentPeriod,
    PlatformWorker,
    Retrenchment,
    SafetyCommittee,
    SkillLevel,
    SocialSecurityScheme,
    StandingOrders,
    StrikeLockout,
    TradeUnion,
    Wage,
    WageDeduction,
    WageExclusion,
    WagePayment,
    WeeklyOff,
    WorkerType,
    WorkingHours,
    // Validators
    calculate_gratuity,
    calculate_minimum_wage,
    get_compliance_checklist as get_labour_compliance_checklist,
    validate_bonus,
    validate_contract_labour,
    validate_deductions,
    validate_epf_compliance,
    validate_esi_compliance,
    validate_establishment_registration,
    validate_gratuity,
    validate_labour_compliance,
    validate_layoff,
    validate_leave,
    validate_maternity_benefit,
    validate_migrant_worker,
    validate_minimum_wage,
    validate_overtime,
    validate_retrenchment,
    validate_safety_committee,
    validate_standing_orders,
    validate_strike_lockout_notice,
    validate_trade_union,
    validate_wage_payment,
    validate_working_hours,
};

// Re-export Criminal (BNS) module types
pub use criminal::{
    // Types
    Accused,
    BailStatus as CriminalBailStatus,
    // Errors
    BnsError,
    BnsPenaltyInfo,
    BnsResult,
    CaseStatus,
    CommunityService,
    CommunityWorkType,
    Court as CriminalCourt,
    CriminalCase,
    CriminalComplianceReport,
    EFir,
    FineType,
    ImprisonmentType,
    InvestigationStatus,
    LimitationPeriod,
    Offence,
    OffenceCategory,
    PleaBargaining,
    Punishment as CriminalPunishment,
    ZeroFir,
    // Validators
    calculate_statutory_bail_eligibility,
    get_applicable_court,
    get_limitation_period as get_criminal_limitation_period,
    get_punishment_for_offence,
    validate_arrest_procedure,
    validate_bail_status,
    validate_criminal_compliance,
    validate_fir_registration,
    validate_investigation_timeline,
    validate_juvenile_handling,
    validate_plea_bargaining,
    validate_police_remand,
    validate_sentencing,
    validate_trial_timeline,
};

// Re-export Constitution module types
pub use constitution::{
    // Types
    AmendmentProcedure,
    AmendmentVoteCheck,
    Article19Freedom,
    BasicStructure,
    ConstitutionPart,
    ConstitutionalAmendment,
    // Errors
    ConstitutionalComplianceReport,
    ConstitutionalCourt,
    ConstitutionalError,
    ConstitutionalResult,
    DirectivePrinciple,
    EmergencyType,
    FundamentalDuty,
    FundamentalRight,
    HouseVotes,
    Legislature,
    PilPetitionerType,
    PublicInterestLitigation,
    Schedule7List,
    WritPetition,
    WritType,
    // Validators
    check_basic_structure_violation,
    get_appropriate_writ,
    get_constitutional_limitation,
    validate_amendment_procedure,
    validate_article19_restriction,
    validate_constitutional_compliance,
    validate_due_process,
    validate_emergency_proclamation,
    validate_equality,
    validate_fundamental_right_claim,
    validate_legislative_competence,
    validate_pil_maintainability,
    validate_writ_petition,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_dpdpa_citation() {
        let cite = cite::dpdpa(4);
        assert_eq!(
            cite.format(),
            "Digital Personal Data Protection Act, 2023, s. 4"
        );
    }

    #[test]
    fn test_air_citation() {
        let cite = cite::air_sc(2024, 1234);
        assert_eq!(cite.format(), "AIR 2024 SC 1234");
    }

    #[test]
    fn test_scc_citation() {
        let cite = Citation::scc(2024, 5, 789);
        assert_eq!(cite.format(), "(2024) 5 SCC 789");
    }

    #[test]
    fn test_inr_formatting() {
        let _amount = InrAmount::from_rupees(2500000.0);
        assert_eq!(to_lakhs(2500000.0), 25.0);
    }

    #[test]
    fn test_crore_conversion() {
        assert_eq!(to_crores(10000000.0), 1.0);
        assert_eq!(crores_to_rupees(2.5), 25000000.0);
    }

    #[test]
    fn test_data_fiduciary_validation() {
        let fiduciary = DataFiduciary {
            registration_number: Some("REG001".to_string()),
            name: "Test Corp".to_string(),
            category: DataFiduciaryCategory::Significant,
            principal_place: "Mumbai".to_string(),
            contact_email: "privacy@test.com".to_string(),
            dpo: Some(DataProtectionOfficer {
                name: "Test DPO".to_string(),
                designation: "CPO".to_string(),
                contact_email: "dpo@test.com".to_string(),
                phone: "+91-9999999999".to_string(),
                based_in_india: true,
                appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            }),
            consent_manager: None,
            registration_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
        };

        let report = validate_fiduciary_compliance(&fiduciary);
        assert!(report.compliant);
    }

    #[test]
    fn test_sdf_without_dpo_fails() {
        let fiduciary = DataFiduciary {
            registration_number: None,
            name: "Test Corp".to_string(),
            category: DataFiduciaryCategory::Significant,
            principal_place: "Delhi".to_string(),
            contact_email: "privacy@test.com".to_string(),
            dpo: None, // SDF must have DPO
            consent_manager: None,
            registration_date: None,
        };

        let report = validate_fiduciary_compliance(&fiduciary);
        assert!(!report.compliant);
    }

    #[test]
    fn test_penalty_tiers() {
        assert_eq!(PenaltyTier::Tier1.max_amount_rupees(), 500_000_000);
        assert_eq!(PenaltyTier::Tier2.max_amount_rupees(), 2_000_000_000);
        assert_eq!(PenaltyTier::Tier3.max_amount_rupees(), 2_500_000_000);
    }

    #[test]
    fn test_principal_rights() {
        let rights = get_principal_rights();
        assert_eq!(rights.len(), 5);
        assert!(rights.iter().any(|(r, _)| *r == DataPrincipalRight::Access));
    }

    #[test]
    fn test_sdf_obligations() {
        let sdf_obligations = get_obligations(DataFiduciaryCategory::Significant);
        let general_obligations = get_obligations(DataFiduciaryCategory::General);
        assert!(sdf_obligations.len() > general_obligations.len());
    }
}
