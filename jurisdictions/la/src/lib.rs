//! Lao PDR (Laos) jurisdiction support for Legalis-RS.
//!
//! This crate provides structured representations of Lao law, focusing on:
//! - **Civil Code 2020** (Law No. 66/NA, effective July 9, 2021) - 6 Books, 1087 Articles
//! - **Criminal Code 2017** (Law No. 26/NA, effective May 27, 2018) - Penal law and procedures
//! - Comparative law analysis showing Japanese and French legal influences
//! - Support for Japan's ODA legal institutional development assistance programs
//! - Bilingual (Lao/English) statute handling
//!
//! ## Historical Context
//!
//! The Lao Civil Code (2020) represents a major milestone in Lao legal modernization,
//! developed with significant assistance from Japanese legal experts through JICA's
//! ODA programs. The code synthesizes influences from:
//! - **Japanese Civil Code** (明治民法・平成民法) - Structure, terminology, concepts
//! - **French Civil Code** (Code civil) - Historical colonial influence, basic principles
//! - **Socialist legal tradition** - Adapted to market economy transition
//!
//! ## Civil Code Structure (ປະມວນກົດໝາຍແພ່ງ)
//!
//! The 2020 Civil Code is organized into six books:
//!
//! ### Book I: General Provisions (ບົດບັນຍັດທົ່ວໄປ) - Articles 1-161
//! - Basic principles, legal capacity, juristic acts, agency, period of time
//! - Influenced by Japanese Civil Code Book I and French Code civil Book I
//!
//! ### Book II: Property (ຊັບສິນ) - Articles 162-431
//! - Real rights, ownership, possession, co-ownership, servitudes
//! - Based on Japanese property law with French droit réel influences
//!
//! ### Book III: Obligations (ພັນທະ) - Articles 432-672
//! - General obligations, contracts, torts, unjust enrichment
//! - Structure follows Japanese saiken-hō (債権法) with French obligations influence
//!
//! ### Book IV: Family Law (ກົດໝາຍຄອບຄົວ) - Articles 673-909
//! - Marriage, divorce, parent-child relations, adoption, guardianship
//! - Adapted to Lao cultural context while following civil law principles
//!
//! ### Book V: Inheritance (ມໍລະດົກ) - Articles 910-1078
//! - Succession, wills, forced heirship, estate administration
//! - Based on Japanese inheritance law with modifications for Lao customs
//!
//! ### Book VI: Miscellaneous Provisions (ບົດບັນຍັດເບັດເທື່ງ) - Articles 1079-1087
//! - Transitional provisions, effective dates
//!
//! ## Comparative Law Features
//!
//! This implementation includes:
//! - Cross-references to equivalent Japanese Civil Code articles
//! - Cross-references to equivalent French Code civil articles
//! - Analysis of legal transplantation and adaptation
//! - Documentation of ODA legal assistance contributions
//!
//! ## Example Usage
//!
//! ```
//! use legalis_la::obligations::{Contract, ContractType, validate_contract_formation};
//! use chrono::Utc;
//!
//! // Article 500: Requirements for contract formation
//! let contract = Contract {
//!     parties: vec!["Buyer".to_string(), "Seller".to_string()],
//!     contract_type: ContractType::Sale {
//!         price: 100_000_000,
//!         subject: "Land".to_string(),
//!     },
//!     offer: "Sale of land for 100,000,000 LAK".to_string(),
//!     acceptance: true,
//!     consideration: Some(100_000_000),
//!     lawful_purpose: true,
//!     capacity_verified: true,
//!     free_consent: true,
//!     concluded_at: Utc::now(),
//! };
//!
//! // Validate according to Lao Civil Code Book III
//! assert!(validate_contract_formation(&contract).is_ok());
//! ```
//!
//! ## Modules
//!
//! ### Constitution (ລັດຖະທຳມະນູນ)
//! - **constitution**: Constitution of Lao PDR (1991, amended 2003, 2015)
//!   - Fundamental rights and duties (Articles 34-51)
//!   - State structure (National Assembly, President, Government)
//!   - Judicial system (Courts and Prosecutors)
//!   - Constitutional amendment procedures
//!
//! ### Civil Code 2020
//! - **general_provisions**: Book I - Basic principles, capacity, juristic acts (Arts. 1-161)
//! - **property**: Book II - Real rights, ownership, possession (Arts. 162-431)
//! - **obligations**: Book III - Contracts, torts, unjust enrichment (Arts. 432-672)
//! - **family**: Book IV - Marriage, divorce, parent-child relations (Arts. 673-909)
//! - **inheritance**: Book V - Succession, wills, estate administration (Arts. 910-1078)
//!
//! ### Criminal Code 2017
//! - **criminal_code**: Criminal liability, penalties, homicide, sexual crimes, property crimes
//!   - Age of criminal responsibility: 16 years (general), 14 years (serious crimes)
//!   - Age of consent: 15 years (sexual crimes)
//!   - Penalties: Death, imprisonment, fines, re-education
//!
//! ### Commercial Law
//! - **commercial_law**: Enterprise Law 2013, Investment Promotion Law 2016
//!
//! ### Property & Land Law
//! - **land_law**: Land Law 2019 - State ownership, use rights, concessions, registration
//!
//! ### Labor Law
//! - **labor_law**: Labor Law 2013 - Employment, working hours, wages, leave, termination
//!
//! ### Administrative Law
//! - **administrative_law**: Administrative Procedure Law, State Liability Law
//!   - Administrative decisions and acts
//!   - Licensing and permit framework
//!   - Administrative sanctions
//!   - Administrative appeals and review
//!   - State liability claims
//!
//! ### Environmental Law
//! - **environmental_law**: Environmental Protection Law 2012 (Law No. 29/NA)
//!   - Environmental Impact Assessment (EIA) framework (Articles 18-24)
//!   - Pollution control standards - air, water, noise (Articles 28-35)
//!   - Protected area management (Articles 38-45)
//!   - Environmental permits and compliance (Articles 25-27)
//!   - Waste management regulations (Articles 34-35)
//!   - Bilingual support (Lao/English)
//!
//! ### Mining Law
//! - **mining_law**: Mining Law 2017 (Law No. 31/NA)
//!   - Mineral classifications (strategic, common, gemstones, rare earth)
//!   - Mining license types (exploration, mining, processing, small-scale)
//!   - Concession framework (20-30 years for strategic minerals)
//!   - Royalty rates (Gold 5%, Copper 3%, Potash 2%, Gemstones 10%)
//!   - Environmental requirements (EIA, rehabilitation bond, closure plan)
//!   - Foreign investment rules (joint venture for strategic minerals)
//!   - Community rights (prior consultation, compensation, employment quotas)
//!   - Bilingual support (Lao/English)
//!
//! ### Forestry Law
//! - **forestry_law**: Forestry Law 2019 (Law No. 64/NA)
//!   - Forest classification (protection, conservation, production, rehabilitation, village)
//!   - Timber harvesting regulations (AAC system, seasons, diameter limits)
//!   - Forest concessions (management and plantation, max 40-50 years)
//!   - Protected species (Category I-III, CITES compliance)
//!   - Community forestry (village forests, benefit sharing 50-30-20)
//!   - Log tracking and chain of custody (Article 51)
//!   - Permits (harvesting, transport, sawmill, export)
//!   - Penalties and enforcement (illegal logging, wildlife trafficking)
//!   - Bilingual support (Lao/English)
//!
//! ### Health Law
//! - **health_law**: Healthcare Law 2014 (Law No. 58/NA), Drug and Medical Products Law
//!   - Healthcare facility licensing and accreditation (Articles 12-16)
//!   - Medical professional licensing (Articles 20-27)
//!   - Patient rights including informed consent (Articles 30-35)
//!   - Drug registration and controlled substances
//!   - Public health measures and epidemic control
//!   - Health insurance schemes (SSO, CBHI, HEF)
//!   - Bilingual support (Lao/English)
//!
//! ### Education Law
//! - **education_law**: Education Law 2015 (Law No. 62/NA)
//!   - Education levels (pre-primary through higher education)
//!   - Compulsory education (ages 6-14, 9 years)
//!   - Educational institution licensing and accreditation
//!   - Teacher qualification and licensing
//!   - Student rights and protections
//!   - Scholarship and financial aid
//!   - National curriculum standards
//!
//! ### Water Law
//! - **water_law**: Water and Water Resources Law 2017 (Law No. 23/NA)
//!   - Water source classification (surface, groundwater, Mekong system, wetlands)
//!   - Water use rights and priority hierarchy (Articles 35-44)
//!   - Hydropower regulations and concessions (Articles 45-54)
//!   - Water quality standards (drinking, agricultural, industrial)
//!   - Mekong River Commission compliance (PNPCA procedures)
//!   - Irrigation districts and Water User Associations
//!   - Groundwater management and aquifer protection
//!   - Pollution prevention (polluter pays principle)
//!   - Bilingual support (Lao/English)
//!
//! ### Tourism Law
//! - **tourism_law**: Tourism Law 2013 (Law No. 32/NA)
//!   - Tourism enterprise categories (accommodation, tour operators, agencies)
//!   - Hotel classification (1-5 star ratings, boutique hotels, eco-lodges)
//!   - Tourism business licenses (3-year validity, renewal procedures)
//!   - Foreign ownership rules (49% limit for most activities, 100% for hotels)
//!   - Tour guide licensing (national, provincial, community guides)
//!   - Language proficiency requirements (Lao + foreign language)
//!   - Tourism zones (national, provincial, heritage, ecotourism, SEZ)
//!   - Tourist rights and protection (complaint mechanisms, insurance)
//!   - Sustainable tourism (environmental impact, CBT framework)
//!   - ASEAN tourism integration (MRA compliance, cross-border packages)
//!   - Bilingual support (Lao/English)
//!
//! ### Anti-Corruption Law
//! - **anti_corruption_law**: Anti-Corruption Law 2012 (Law No. 03/NA, amended 2019)
//!   - State Inspection Authority (SIA) structure and powers
//!   - Corruption offenses (bribery, embezzlement, abuse of position, nepotism)
//!   - Asset declaration requirements for public officials (Grade 5+)
//!   - Penalty framework based on corruption amount thresholds
//!   - Whistleblower protection mechanisms and rewards
//!   - Prevention measures (code of conduct, gift restrictions, cooling-off periods)
//!   - International cooperation (UNCAC compliance, mutual legal assistance)
//!   - Bilingual support (Lao/English)
//!
//! ### Banking & Financial Services Law
//! - **banking_law**: Commercial Bank Law 2006 (amended 2018), Bank of Lao PDR Law 2018
//!   - Bank of Lao PDR (central bank) - monetary policy, supervision, FX management
//!   - Bank types (state-owned, joint venture, foreign branches, MFIs)
//!   - License requirements (300B LAK commercial, 50B LAK foreign branch)
//!   - Capital adequacy (Basel III: CAR 8%, Tier 1 6%, leverage 3%)
//!   - Prudential regulations (single borrower 25%, related party 15%)
//!   - Liquidity requirements (LCR 100%, NSFR 100%)
//!   - Deposit protection (50M LAK per depositor coverage)
//!   - Foreign exchange regulations and capital controls
//!   - AML/CFT (CDD/KYC, STR 24hr deadline, 5yr record keeping, PEPs)
//!   - Interest rate regulations and usury prevention
//!   - Payment systems (RTGS, mobile banking, QR payments)
//!   - Bilingual support (Lao/English)
//!
//! ### Legal Development & Analysis
//! - **comparative**: Comparative law analysis and cross-references
//! - **oda**: Documentation of Japanese ODA legal assistance contributions

pub mod administrative_law;
pub mod anti_corruption_law;
pub mod banking_law;
pub mod commercial_law;
pub mod comparative;
pub mod constitution;
pub mod criminal_code;
pub mod education_law;
pub mod environmental_law;
pub mod family;
pub mod forestry_law;
pub mod general_provisions;
pub mod health_law;
pub mod inheritance;
pub mod labor_law;
pub mod land_law;
pub mod mining_law;
pub mod obligations;
pub mod oda;
pub mod property;
pub mod tax_law;
pub mod tourism_law;
pub mod water_law;

// Re-export general provisions
pub use general_provisions::{
    Agency, JuristicAct, LegalCapacity, Period, article1, article3, article20, article21,
    validate_juristic_act, validate_legal_capacity,
};

// Re-export property law
pub use property::{
    Ownership, Possession, Property, Servitude, article162, article163, article200,
    validate_ownership, validate_property_transaction,
};

// Re-export obligations
pub use obligations::{
    Contract, Tort, UnjustEnrichment, article432, article500, article600,
    validate_contract_formation, validate_tort_claim,
};

// Re-export family law
pub use family::{
    Adoption, AdoptionType, Divorce, DivorceType, Marriage, ParentChild, article673, article700,
    validate_adoption, validate_divorce, validate_marriage,
};

// Re-export inheritance law
pub use inheritance::{
    ForcedHeirship, Succession, Will, article910, article950, article1000, validate_succession,
    validate_will,
};

// Re-export comparative law analysis
pub use comparative::{
    ComparativeAnalysis, FrenchInfluence, JapaneseInfluence, LegalTransplant,
    compare_with_french_law, compare_with_japanese_law,
};

// Re-export ODA documentation
pub use oda::{
    JICAProject, LegalExpertMission, ODAContribution, get_legal_assistance_projects,
    get_oda_history,
};

// Re-export commercial law
pub use commercial_law::{
    BoardOfDirectors, BusinessSector, CommercialLawError, Director, DirectorPosition,
    DomesticInvestment, EnterpriseType, ForeignInvestment, IntellectualProperty,
    InvestmentIncentive, InvestmentType, LimitedCompany, Partnership, Patent, PublicCompany,
    Shareholder, Trademark, validate_board_composition, validate_enterprise_formation,
    validate_foreign_investment, validate_ip_registration,
};

// Re-export land law
pub use land_law::{
    CadastralSurvey, DisputeStatus, ForeignOwnershipStatus, LandCertificate, LandClassification,
    LandConcession, LandDispute, LandDisputeType, LandLawError, LandRegistrationStatus, LandTitle,
    LandTitleType, LandTransaction, LandTransactionType, LandUsePurpose, LandUseRight,
    ResolutionMethod, StateLand, SurveyMethod, validate_cadastral_survey,
    validate_foreign_ownership, validate_land_concession, validate_land_registration,
    validate_land_title, validate_land_transaction, validate_land_use_right,
};

// Re-export labor law
pub use labor_law::{
    Allowance, AllowanceType, DisputeType as LaborDisputeType, EmploymentContract, EmploymentType,
    LaborDispute, LaborLawError, LeaveRequest, LeaveType, MonthlyWorkingSummary, PaymentFrequency,
    PaymentMethod, SocialSecurityContribution, SocialSecurityType, TerminationNotice,
    TerminationType, WorkSchedule, WorkingHoursRecord, validate_annual_leave,
    validate_comprehensive, validate_employment_contract, validate_holiday_work_premium,
    validate_hourly_rate, validate_leave_request, validate_minimum_wage, validate_monthly_summary,
    validate_night_shift_premium, validate_overtime_premium, validate_severance_pay,
    validate_social_security_contribution, validate_social_security_enrollment,
    validate_termination_notice, validate_working_hours, validate_working_hours_record,
};

// Re-export criminal code
pub use criminal_code::{
    ActusReus, AgeError, BodilyHarmError, BodilyHarmType, Crime, CrimeType, CriminalCodeError,
    CriminalLiability, HomicideError, HomicideType, JustificationError, JustificationGround,
    LiabilityError, MensRea, MentalCapacity, MentalCapacityStatus, NegligenceType, Penalty,
    PenaltyError, PenaltySeverity, Perpetrator, PropertyCrime, PropertyCrimeError, Result,
    SexualCrime, SexualCrimeError, Victim, VictimCategory, validate_actus_reus,
    validate_age_for_serious_crime, validate_age_of_consent, validate_age_of_responsibility,
    validate_criminal_liability, validate_homicide, validate_justification, validate_mens_rea,
    validate_mental_capacity, validate_mental_capacity_status, validate_penalty,
    validate_property_crime, validate_sexual_crime,
};

// Re-export constitution
pub use constitution::{
    AdministrativeAuthority, AdministrativeLevel, AmendmentProposer, ConstitutionalAmendment,
    ConstitutionalError, ConstitutionalResult, CourtLevel, CourtPower, EconomicSystem, ElectedBy,
    ElectionMethod, FundamentalDuty, FundamentalRight, Government, GovernmentPower, Judge,
    LegitimateAim, LimitationFailure, LocalAdministration, LocalPower, Minister, NationalAssembly,
    NationalAssemblyPower, PeoplesCouncil, PeoplesCourt, PeoplesProsecutor, PoliticalRegime,
    President, PresidentialPower, ProsecutorLevel, ProsecutorPower, RightsLimitation, Sovereignty,
    StandingCommittee, StandingCommitteePower, StateForm, StateOrgan,
    validate_constitutional_amendment, validate_court_organization, validate_fundamental_right,
    validate_government, validate_local_administration, validate_na_candidacy,
    validate_national_assembly, validate_president, validate_prosecutor_organization,
    validate_rights_limitation, validate_state_structure, validate_voting_rights,
};

// Re-export tax law
pub use tax_law::{
    // Constants
    CORPORATE_INCOME_TAX_RATE,
    CUSTOMS_DUTY_RATE_MAX,
    CUSTOMS_DUTY_RATE_MIN,
    // Types
    CorporateEntityType,
    CorporateIncomeTax,
    CustomsDeclarationType,
    CustomsDuty,
    ExciseTax,
    ExciseTaxCategory,
    FuelType,
    INCOME_TAX_THRESHOLD,
    IncomeType,
    PERSONAL_INCOME_TAX_BRACKETS,
    PROPERTY_TAX_RATE_MAX,
    PROPERTY_TAX_RATE_MIN,
    PersonalIncomeTax,
    PersonalIncomeTaxBracket,
    PropertyTax,
    PropertyTaxType,
    PropertyType,
    TaxFiling,
    TaxFilingPeriod,
    TaxFilingStatus,
    TaxLawError,
    TaxPaymentMethod,
    TaxResidenceStatus,
    TaxType,
    VAT_REGISTRATION_THRESHOLD,
    VAT_STANDARD_RATE,
    VATExemptCategory,
    VATRateType,
    VATRegistrationStatus,
    VATReturn,
    VatExemption,
    VatRegistration,
    WithholdingPaymentType,
    WithholdingTax,
    // Validators
    calculate_corporate_income_tax,
    calculate_excise_tax,
    calculate_personal_income_tax,
    calculate_property_tax,
    calculate_vat,
    validate_corporate_income_tax,
    validate_customs_duty,
    validate_customs_duty_rate,
    validate_excise_tax,
    validate_hs_code,
    validate_personal_income_tax,
    validate_property_tax,
    validate_property_tax_rate,
    validate_tax_filing,
    validate_tax_id_format,
    validate_tax_residence,
    validate_vat_calculation,
    validate_vat_exemption,
    validate_vat_rate,
    validate_vat_registration,
};

// Re-export administrative law
pub use administrative_law::{
    // Constants
    ADMINISTRATIVE_APPEAL_DEADLINE_DAYS,
    // Types - Appeals
    AdministrativeAppeal,
    AdministrativeAppealBuilder,
    // Types - Decisions
    AdministrativeDecision,
    AdministrativeDecisionBuilder,
    // Errors
    AdministrativeLawError,
    AdministrativeLawResult,
    // Types - Administrative Levels
    AdministrativeLevel as AdminLevel,
    // Types - Sanctions
    AdministrativeSanction,
    AdministrativeSanctionBuilder,
    AffectedParty,
    AppealGround,
    AppealLevel,
    AppealOutcome,
    AppealStatus,
    COURT_APPEAL_DEADLINE_DAYS,
    ClaimStatus,
    DecisionType,
    // Types - Legal Basis and Parties
    LegalBasis,
    LiabilityType,
    // Types - Licenses and Permits
    LicenseType,
    MAXIMUM_SUSPENSION_DAYS,
    MINIMUM_FINE_AMOUNT_LAK,
    OrderType,
    PartyType,
    PermitType,
    STATE_LIABILITY_CLAIM_DEADLINE_YEARS,
    SanctionType,
    // Types - State Liability
    StateLiability,
    validate_administrative_appeal,
    // Validators
    validate_administrative_decision,
    validate_appeal_deadline,
    validate_authority_jurisdiction,
    validate_legal_basis,
    validate_license_application,
    validate_notification,
    validate_permit_application,
    validate_proportionality,
    validate_sanction,
    validate_state_liability_claim,
};

// Re-export environmental law
pub use environmental_law::{
    // Pollutant Types
    AirPollutant,
    EIA_VALIDITY_YEARS_CATEGORY_A,
    EIA_VALIDITY_YEARS_CATEGORY_B,
    EIAApprovalStatus,
    EIACategory,
    ENVIRONMENTAL_PERMIT_VALIDITY_YEARS,
    EmissionType,
    EnvironmentalImpact,
    // EIA Types
    EnvironmentalImpactAssessment,
    EnvironmentalImpactAssessmentBuilder,
    // Errors
    EnvironmentalLawError,
    // Permits
    EnvironmentalPermit,
    EnvironmentalPermitType,
    IUCNCategory,
    // Impact Types
    ImpactSeverity,
    ImplementationPhase,
    MAX_BOD_DISCHARGE,
    MAX_COD_DISCHARGE,
    MAX_NOISE_COMMERCIAL_DAY,
    MAX_NOISE_COMMERCIAL_NIGHT,
    MAX_NOISE_INDUSTRIAL,
    MAX_NOISE_RESIDENTIAL_DAY,
    MAX_NOISE_RESIDENTIAL_NIGHT,
    MAX_PH_DISCHARGE,
    MAX_PM10_ANNUAL,
    // Constants
    MAX_PM25_ANNUAL,
    MAX_TEMPERATURE_DISCHARGE,
    MAX_TSS_DISCHARGE,
    MIN_BUFFER_ZONE_METERS,
    MIN_MINING_DISTANCE_FROM_PROTECTED_AREA,
    MIN_PH_DISCHARGE,
    // Mitigation
    MitigationMeasure,
    PermitCondition,
    PermitStatus as EnvironmentalPermitStatus,
    // Pollution Sources
    PollutionSource,
    PollutionSourceType,
    // Project Types
    ProjectType,
    // Protected Areas
    ProtectedArea,
    ProtectedAreaActivity,
    ProtectedAreaType,
    RestrictionLevel,
    Result as EnvironmentalLawResult,
    WasteDisposalMethod,
    // Waste Types
    WasteType,
    WaterPollutant,
    // Zones
    ZoneType,
    // Air Quality Validators
    validate_air_quality,
    validate_air_quality_batch,
    validate_eia_approval,
    validate_eia_completeness,
    // EIA Validators
    validate_eia_requirement,
    validate_endangered_species_impact,
    // Comprehensive Validators
    validate_environmental_compliance,
    // Permit Validators
    validate_environmental_permit,
    validate_hazardous_waste_transport,
    validate_noise_impact,
    // Noise Validators
    validate_noise_level,
    validate_permit_for_activity,
    validate_pollution_monitoring,
    // Pollution Source Validators
    validate_pollution_source,
    // Protected Area Validators
    validate_protected_area_activity,
    validate_protected_area_distance,
    // Waste Validators
    validate_waste_disposal,
    validate_water_discharge_comprehensive,
    // Water Quality Validators
    validate_water_quality,
};

// Re-export health law
pub use health_law::{
    // Facility Types
    AccreditationStatus,
    // Public Health Types
    AuthorityLevel as HealthAuthorityLevel,
    // Health Insurance Types
    BeneficiaryCategory,
    // Constants
    CONTROLLED_SUBSTANCE_SCHEDULES,
    DRUG_REGISTRATION_VALIDITY_YEARS,
    // Drug Registration Types
    DrugCategory,
    DrugRegistration,
    EMERGENCY_RESPONSE_TIME_MINUTES,
    HEALTH_INSURANCE_COVERAGE_MINIMUM,
    HealthInsurance,
    HealthInsuranceScheme,
    // Error
    HealthLawError,
    HealthcareFacility,
    HealthcareFacilityType,
    HealthcareService,
    INFORMED_CONSENT_MINIMUM_AGE,
    // Informed Consent Types
    InformedConsent,
    InformedConsentStatus,
    // Medical Professional Types
    LicenseStatus as MedicalLicenseStatus,
    MAXIMUM_QUARANTINE_DAYS,
    MEDICAL_LICENSE_VALIDITY_YEARS,
    MINIMUM_HOSPITAL_BED_DISTRICT,
    MINIMUM_HOSPITAL_BED_PROVINCIAL,
    MedicalProfessionType,
    MedicalProfessional,
    // Patient Rights Types
    PatientRightType,
    PatientRights,
    PublicHealthMeasure,
    PublicHealthMeasureType,
    RegistrationStatus as DrugRegistrationStatus,
    Result as HealthLawResult,
    // Validators
    validate_drug_registration,
    validate_emergency_care_obligation,
    validate_emergency_response_time,
    validate_facility_comprehensive,
    validate_facility_license,
    validate_facility_services,
    validate_health_insurance_coverage,
    validate_informed_consent,
    validate_medical_license,
    validate_patient_privacy,
    validate_practice_scope,
    validate_prescription_requirements,
    validate_professional_comprehensive,
    validate_public_health_measure,
    validate_quarantine_compliance,
    validate_scheme_eligibility,
};

// Re-export education law
pub use education_law::{
    // Constants
    ACADEMIC_YEAR_END_MONTH,
    ACADEMIC_YEAR_START_MONTH,
    ACCREDITATION_VALIDITY_YEARS,
    // Institutions
    AccreditationStatus as EducationAccreditationStatus,
    COMPULSORY_EDUCATION_END_AGE,
    COMPULSORY_EDUCATION_START_AGE,
    COMPULSORY_EDUCATION_YEARS,
    // Students
    CompulsoryEducation,
    // Education Levels
    DegreeType,
    // Errors
    EducationLawError,
    EducationLevel,
    // Programs
    EducationProgram,
    EducationalInstitution,
    EducationalInstitutionBuilder,
    // Scholarships
    EligibilityCriterion,
    EnrollmentStatus,
    EntryRequirement,
    ErrorCategory as EducationErrorCategory,
    InstitutionType,
    InstructionLanguage,
    LICENSE_VALIDITY_YEARS,
    LOWER_SECONDARY_YEARS,
    MAX_CLASS_SIZE_PRIMARY,
    MAX_CLASS_SIZE_SECONDARY,
    MIN_CREDITS_BACHELOR,
    MIN_CREDITS_MASTER,
    MIN_TEACHER_QUALIFICATION_PRIMARY,
    MIN_TEACHER_QUALIFICATION_SECONDARY,
    // Curriculum
    NationalCurriculum,
    NonEnrollmentReason,
    OwnershipType,
    PRE_PRIMARY_MAX_AGE,
    PRE_PRIMARY_MIN_AGE,
    PRIMARY_EDUCATION_YEARS,
    Result as EducationLawResult,
    RightScope,
    Scholarship,
    ScholarshipCoverage,
    ScholarshipProvider,
    SpecialEducationType,
    StudentRight,
    StudentRightType,
    Subject,
    // Teachers
    Teacher,
    TeacherEmploymentStatus,
    TeacherLicenseStatus,
    TeacherQualification,
    UPPER_SECONDARY_YEARS,
    // Compulsory Education Validators
    check_enrollment_requirement,
    // Institution Validators
    validate_all_programs,
    validate_class_size,
    validate_compulsory_enrollment,
    // Curriculum Validators
    validate_curriculum_compliance,
    validate_institution_accreditation,
    validate_institution_comprehensive,
    validate_institution_license,
    validate_institution_programs,
    // Student Rights Validators
    validate_non_discrimination,
    // Age Validators
    validate_pre_primary_age,
    validate_primary_entry_age,
    // Program Validators
    validate_program_accreditation,
    validate_program_credits,
    validate_program_duration,
    validate_safe_environment,
    // Scholarship Validators
    validate_scholarship_eligibility,
    validate_student_age_for_level,
    validate_student_rights,
    validate_subject_hours,
    // Teacher Validators
    validate_teacher_license,
    validate_teacher_qualification,
    validate_teacher_student_ratio,
};

// Re-export water law
pub use water_law::{
    // Water Source Types
    AquiferProtectionZone,
    AquiferType,
    // Hydropower
    ConcessionStatus as HydropowerConcessionStatus,
    // Constants
    DRINKING_WATER_MAX_ARSENIC_MG_L,
    DRINKING_WATER_MAX_ECOLI,
    DRINKING_WATER_MAX_LEAD_MG_L,
    DRINKING_WATER_MAX_PH,
    DRINKING_WATER_MAX_TURBIDITY_NTU,
    DRINKING_WATER_MIN_PH,
    // Water Allocation
    DroughtLevel,
    DroughtRestrictions,
    EcologicalSignificance,
    // Pollution Prevention
    FacilityStatus,
    // Irrigation Districts
    FeePaymentStatus,
    GROUNDWATER_MONITORING_INTERVAL_DAYS,
    // Groundwater
    GroundwaterMonitoringRecord,
    HYDROPOWER_CONCESSION_MAX_YEARS,
    HYDROPOWER_CONCESSION_MIN_YEARS,
    HydropowerCategory,
    HydropowerConcession,
    HydropowerConcessionBuilder,
    INDUSTRIAL_DISCHARGE_MAX_BOD_MG_L,
    INDUSTRIAL_DISCHARGE_MAX_COD_MG_L,
    INDUSTRIAL_DISCHARGE_MAX_TSS_MG_L,
    IRRIGATION_FEE_PER_HECTARE_LAK,
    IrrigationServiceFee,
    MEDIUM_HYDROPOWER_THRESHOLD_MW,
    MRC_PRIOR_CONSULTATION_MONTHS,
    // MRC Compliance
    MRCComplianceRecord,
    MRCComplianceStatus,
    MRCProcedureType,
    MekongLocation,
    // Water Use Rights
    PermitCondition as WaterPermitCondition,
    PolluterRecord,
    PollutionType,
    PowerPurchaseAgreement,
    ProtectionLevel,
    RemediationStatus,
    ResettlementApprovalStatus,
    ResettlementPlan,
    // Error and Result types
    Result as WaterLawResult,
    SMALL_HYDROPOWER_THRESHOLD_MW,
    Season,
    SurfaceWaterBodyType,
    TreatmentType,
    WATER_PERMIT_VALIDITY_YEARS,
    WELL_PERMIT_DEPTH_THRESHOLD_M,
    WUAStatus,
    WastewaterTreatmentFacility,
    WaterAllocation,
    WaterLawError,
    WaterPermitStatus,
    // Water Quality
    WaterQualityClass,
    WaterQualityMeasurement,
    WaterQualityParameter,
    WaterSourceType,
    WaterUseRight,
    WaterUseRightBuilder,
    WaterUseType,
    WaterUserAssociation,
    WellPermit,
    WetlandType,
    // Validators
    calculate_irrigation_fee,
    validate_agricultural_runoff,
    validate_aquifer_zone_activity,
    validate_drinking_water_quality,
    validate_drought_protocol,
    validate_extraction_limit,
    validate_groundwater_extraction,
    validate_groundwater_monitoring,
    validate_hydropower_category,
    validate_hydropower_concession,
    validate_industrial_discharge,
    validate_irrigation_fee,
    validate_minimum_environmental_flow,
    validate_mrc_data_sharing,
    validate_mrc_notification,
    validate_mrc_prior_consultation,
    validate_polluter_pays,
    validate_seasonal_allocation,
    validate_transboundary_assessment,
    validate_wastewater_treatment,
    validate_water_law_compliance,
    validate_water_permit,
    validate_water_use_permit,
    validate_water_use_priority,
    validate_well_drilling_permit,
    validate_well_permit,
    validate_wetland_protection,
    validate_wua_registration,
};

// Re-export forestry law
pub use forestry_law::{
    BenefitSharingArrangement,
    ChainOfCustodyEntry,
    // Community Forestry
    CommunityForestEnterprise,
    ConcessionStatus,
    ConcessionType,
    // Constants
    DISTRICT_BENEFIT_SHARE_PERCENT,
    ExportProductType,
    // Forest Classification
    ForestClassification,
    // Forest Concessions
    ForestConcession,
    ForestConcessionBuilder,
    // Export
    ForestProductExportPermit,
    ForestProductExportPermitBuilder,
    // Error types
    ForestryLawError,
    // Penalties
    ForestryViolation,
    HARVESTING_SEASON_END_MONTH,
    HARVESTING_SEASON_START_MONTH,
    // Log Tracking
    LogEntry,
    LogEntryBuilder,
    MANAGEMENT_CONCESSION_BOND_PERCENT,
    MAX_MANAGEMENT_CONCESSION_HECTARES,
    MAX_MANAGEMENT_CONCESSION_YEARS,
    MAX_PLANTATION_CONCESSION_HECTARES,
    MAX_PLANTATION_CONCESSION_YEARS,
    MIN_DIAMETER_HARDWOOD_CM,
    MIN_DIAMETER_ROSEWOOD_CM,
    MIN_DIAMETER_TEAK_CM,
    NATIONAL_BENEFIT_SHARE_PERCENT,
    // NTFP
    NtfpPermit,
    NtfpPermitBuilder,
    NtfpType,
    PLANTATION_CONCESSION_BOND_PERCENT,
    PenaltyAssessment,
    PermitStatus as ForestryPermitStatus,
    ProcessingFacilityLicense,
    ProtectionCategory,
    REFORESTATION_MAINTENANCE_YEARS,
    Result as ForestryLawResult,
    // Sawmill and Processing
    SawmillLicense,
    SawmillLicenseBuilder,
    TRANSPORT_PERMIT_VALIDITY_DAYS,
    // Timber Harvesting
    TimberHarvestingPermit,
    TimberHarvestingPermitBuilder,
    TransportPermit,
    TransportPermitBuilder,
    // Tree Species
    TreeSpecies,
    VILLAGE_BENEFIT_SHARE_PERCENT,
    // Village Forests
    VillageForest,
    VillageForestAgreement,
    VillageForestBuilder,
    ViolationStatus,
    ViolationType,
    calculate_penalty,
    validate_benefit_sharing,
    validate_chain_of_custody,
    validate_cites_compliance,
    validate_concession_area,
    validate_concession_term,
    validate_export_permit,
    // Concession Validators
    validate_forest_concession,
    // Comprehensive Validators
    validate_forestry_compliance,
    // Penalty Validators
    validate_forestry_violation,
    validate_harvesting_season,
    // Log Tracking Validators
    validate_log_entry,
    validate_minimum_diameter,
    // NTFP Validators
    validate_ntfp_permit,
    validate_ntfp_sustainable_harvest,
    validate_performance_bond,
    validate_processing_facility_license,
    // License Validators
    validate_sawmill_license,
    validate_species_protection,
    // Timber Harvesting Validators
    validate_timber_harvesting_permit,
    validate_transport_permit,
    // Village Forest Validators
    validate_village_forest,
    validate_village_forest_agreement,
};

// Re-export mining law
pub use mining_law::{
    ARTISANAL_MINING_MAX_HECTARES,
    COMMUNITY_REVENUE_SHARE_MIN_PERCENT,
    CommunityCompensation,
    // Community Types
    CommunityConsultation,
    CompensationType,
    ConcessionStatus as MiningConcessionStatus,
    // Concession Types
    ConcessionType as MiningConcessionType,
    EXPLORATION_AREA_COMMON_MAX_HECTARES,
    // Constants - Area Limits
    EXPLORATION_AREA_STRATEGIC_MAX_HECTARES,
    // Constants - Concession Limits
    EXPLORATION_LICENSE_MAX_RENEWALS,
    EXPLORATION_LICENSE_MAX_YEARS,
    EnvironmentalViolation as MiningEnvironmentalViolation,
    FOREIGN_OWNERSHIP_COMMON_MAX_PERCENT,
    // Constants - Foreign Investment Limits
    FOREIGN_OWNERSHIP_STRATEGIC_MAX_PERCENT,
    // Foreign Investment Types
    ForeignInvestment as MiningForeignInvestment,
    LOCAL_CONTENT_MIN_PERCENT,
    // Constants - Community Requirements
    LOCAL_EMPLOYMENT_MIN_PERCENT,
    LicenseCondition as MiningLicenseCondition,
    LicenseStatus as MiningLicenseStatus,
    LocalEmployment,
    // Constants - Environmental Requirements
    MIN_DISTANCE_FROM_PROTECTED_AREA_METERS,
    MINING_CONCESSION_STRATEGIC_MAX_YEARS,
    MINING_CONCESSION_STRATEGIC_MIN_YEARS,
    // Mineral Types
    MineralClassification,
    MineralType,
    MiningConcession,
    MiningConcessionBuilder,
    // Environmental Types
    MiningEnvironmentalCompliance,
    // Error and Result types
    MiningLawError,
    MiningLicense,
    MiningLicenseBuilder,
    // License Types
    MiningLicenseType,
    PROCESSING_LICENSE_MAX_YEARS,
    // Royalty Types
    PaymentStatus as MiningPaymentStatus,
    REHABILITATION_BOND_MIN_PERCENT,
    // Constants - Royalty Rates
    ROYALTY_RATE_BAUXITE,
    ROYALTY_RATE_COMMON_MAX,
    ROYALTY_RATE_COMMON_MIN,
    ROYALTY_RATE_COPPER,
    ROYALTY_RATE_GEMSTONES,
    ROYALTY_RATE_GOLD,
    ROYALTY_RATE_POTASH,
    ROYALTY_RATE_RARE_EARTH,
    Result as MiningLawResult,
    RoyaltyPayment,
    SMALL_SCALE_MINING_MAX_HECTARES,
    SMALL_SCALE_MINING_MAX_YEARS,
    TechnologyTransfer,
    ViolationSeverity as MiningViolationSeverity,
    calculate_royalty_amount,
    validate_community_compensation,
    validate_concession_area as validate_mining_concession_area,
    validate_concession_duration,
    // Environmental Validators
    validate_environmental_compliance as validate_mining_environmental_compliance,
    // Foreign Investment Validators
    validate_foreign_investment as validate_mining_foreign_investment,
    validate_foreign_ownership as validate_mining_foreign_ownership,
    validate_license_for_activity,
    validate_local_content,
    validate_local_employment,
    // Mineral Validators
    validate_mineral_classification,
    validate_mineral_export,
    // Comprehensive Validators
    validate_mining_compliance,
    // Concession Validators
    validate_mining_concession,
    // License Validators
    validate_mining_license,
    // Community Validators
    validate_prior_consultation,
    validate_protected_area_distance as validate_mining_protected_area_distance,
    validate_rehabilitation_bond,
    validate_revenue_sharing,
    validate_royalty_payment,
    // Royalty Validators
    validate_royalty_rate,
    // Small-Scale Validators
    validate_small_scale_mining,
};

// Re-export anti-corruption law
pub use anti_corruption_law::{
    // Constants
    ANNUAL_DECLARATION_DEADLINE_MONTH,
    // Acquisition
    AcquisitionMethod,
    // Error Types
    AntiCorruptionLawError,
    AntiCorruptionLawResult,
    // Asset Declaration
    AssetDeclaration,
    AssetDeclarationBuilder,
    AssetDeclarationStatus,
    // Bribery
    BriberyDirection,
    COOLING_OFF_PERIOD_YEARS,
    // Code of Conduct
    CodeOfConductViolation,
    CodeOfConductViolationType,
    // Corruption Offense
    CorruptionOffense,
    CorruptionOffenseType,
    CorruptionSeverity,
    // Fund Source
    FundSource,
    GIFT_LIMIT_OFFICIAL_FUNCTION_LAK,
    // Gift
    Gift,
    GiftType,
    INVESTIGATION_FULL_DAYS,
    INVESTIGATION_PRELIMINARY_DAYS,
    // Income Source
    IncomeSource,
    IncomeSourceType,
    // International Cooperation
    InternationalCooperation,
    InternationalCooperationType,
    // Investigation
    Investigation,
    InvestigationStatus,
    InvestigationType,
    MEDIUM_CORRUPTION_THRESHOLD_LAK,
    MINOR_CORRUPTION_THRESHOLD_LAK,
    // Official Types
    OfficialCategory,
    OfficialType,
    PROSECUTION_REFERRAL_DAYS,
    // Penalty
    PenaltyRange,
    PenaltyType,
    // Position
    PositionGrade,
    // Prevention Measure
    PreventionMeasure,
    PreventionMeasureType,
    // Property
    PropertyType as AntiCorruptionPropertyType,
    // Prosecution
    ProsecutionReferral,
    ProsecutionStatus,
    // Real Estate
    RealEstate,
    SERIOUS_CORRUPTION_THRESHOLD_LAK,
    // SIA
    SIAOffice,
    SIAOfficeLevel,
    SIAPower,
    VERY_SERIOUS_CORRUPTION_THRESHOLD_LAK,
    // Vehicle
    Vehicle,
    VehicleType,
    // Verification
    VerificationResult,
    VerificationStatus,
    WHISTLEBLOWER_REWARD_MAX_PERCENT,
    WHISTLEBLOWER_REWARD_MIN_PERCENT,
    // Whistleblower
    WhistleblowerProtection,
    WhistleblowerProtectionType,
    WhistleblowerReport,
    WhistleblowerReportBuilder,
    WhistleblowerReportStatus,
    // Validators
    determine_penalty_range,
    validate_asset_declaration,
    validate_asset_declaration_completeness,
    validate_code_of_conduct_compliance,
    validate_cooling_off_period,
    validate_corruption_offense,
    validate_declaration_deadline,
    validate_declaration_required,
    validate_gift,
    validate_gift_limit,
    validate_international_cooperation,
    validate_investigation,
    validate_investigation_timeline,
    validate_official_category,
    validate_penalty as validate_anti_corruption_penalty,
    validate_prevention_measure,
    validate_sia_jurisdiction,
    validate_sia_powers,
    validate_whistleblower_protection,
    validate_whistleblower_report,
    validate_whistleblower_reward,
};

// Re-export tourism law
pub use tourism_law::{
    // Constants
    ASEAN_VISA_FREE_DAYS,
    // Accommodation Types
    Accommodation,
    AccommodationBuilder,
    AccommodationType,
    // ASEAN Integration Types
    AseanMraCertification,
    AseanTourismProfessional,
    COMPLAINT_RESPONSE_DEADLINE_DAYS,
    // Community-Based Tourism
    CommunityBasedTourism,
    // Complaint Types
    ComplaintStatus,
    FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
    FOREIGN_OWNERSHIP_HOTEL_MAX_PERCENT,
    GUIDE_LICENSE_VALIDITY_YEARS,
    // Tour Guide Types
    GuideLicenseCategory,
    HotelClassificationStatus,
    HotelFacility,
    LanguageProficiency,
    LanguageSkill,
    // License Status
    LicenseStatus as TourismLicenseStatus,
    MAX_ROOMS_BOUTIQUE,
    MAX_ROOMS_GUESTHOUSE,
    MIN_GUIDE_TRAINING_HOURS,
    MIN_ROOMS_1_STAR,
    MIN_ROOMS_2_STAR,
    MIN_ROOMS_3_STAR,
    MIN_ROOMS_4_STAR,
    MIN_ROOMS_5_STAR,
    MIN_ROOMS_BOUTIQUE,
    MIN_ROOMS_GUESTHOUSE,
    // Error Types
    Result as TourismLawResult,
    STAR_RATING_VALIDITY_YEARS,
    StarRating,
    TOURISM_DEVELOPMENT_FUND_RATE_PERCENT,
    TOURISM_LICENSE_VALIDITY_YEARS,
    TourGuide,
    TourismEnterprise,
    TourismEnterpriseCategory,
    TourismLawError,
    // Statistics Types
    TourismStatisticsReport,
    TourismVisaType,
    TourismZone,
    TourismZoneType,
    TouristComplaint,
    TravelInsurance,
    VISA_ON_ARRIVAL_VALIDITY_DAYS,
    // Validators
    validate_accommodation_comprehensive,
    validate_asean_mra_certification,
    validate_carrying_capacity,
    validate_cbt_project,
    validate_complaint_response,
    validate_enterprise_comprehensive,
    validate_enterprise_for_activity,
    validate_enterprise_license as validate_tourism_enterprise_license,
    validate_entrance_fee,
    validate_foreign_ownership as validate_tourism_foreign_ownership,
    validate_guide_comprehensive,
    validate_guide_language,
    validate_guide_license,
    validate_guide_scope,
    validate_guide_training,
    validate_hotel_classification,
    validate_hotel_facilities,
    validate_statistics_submission,
    validate_tourism_visa,
    validate_travel_insurance,
    validate_zone_access,
};

// Re-export banking law
pub use banking_law::{
    // Constants
    AML_RECORD_KEEPING_YEARS,
    // Capital adequacy types
    AssetRiskWeight,
    // Payment system types
    BOLReportType,
    // Bank types
    BankType,
    BankingActivity,
    // Error types
    BankingLawError,
    BankingLicense,
    // Prudential types
    BorrowerExposure,
    // AML/CFT types
    CDDLevel,
    CapitalAdequacyReport,
    // Deposit protection types
    ClaimStatus as DepositClaimStatus,
    CustomerDueDiligence,
    CustomerType,
    DEPOSIT_INSURANCE_LIMIT_LAK,
    DepositInsuranceClaim,
    // Interest rate types
    DepositRate,
    DepositType,
    // Foreign exchange types
    FXTransactionType,
    // BOL supervision types
    FitAndProperAssessment,
    ForeignExchangeTransaction,
    InterestRateStructure,
    LICENSE_VALIDITY_YEARS as BANKING_LICENSE_VALIDITY_YEARS,
    LargeExposureReport,
    LendingRate,
    LicenseStatus as BankingLicenseStatus,
    LiquidityReport,
    LoanType,
    MAX_LENDING_RATE_PERCENT,
    MIN_CAPITAL_ADEQUACY_RATIO_PERCENT,
    MIN_CAPITAL_COMMERCIAL_BANK_LAK,
    MIN_CAPITAL_FOREIGN_BRANCH_LAK,
    MIN_CAPITAL_MFI_DEPOSIT_LAK,
    MIN_CAPITAL_MFI_NON_DEPOSIT_LAK,
    MIN_CET1_RATIO_PERCENT,
    MIN_LCR_PERCENT,
    MIN_LEVERAGE_RATIO_PERCENT,
    MIN_NSFR_PERCENT,
    MIN_TIER1_CAPITAL_RATIO_PERCENT,
    MicrofinanceType,
    PEPStatus,
    PaymentService,
    PaymentServiceLicense,
    RELATED_PARTY_LIMIT_PERCENT,
    RESERVE_REQUIREMENT_PERCENT,
    RTGSStatus,
    RTGSTransaction,
    Result as BankingLawResult,
    RiskRating,
    RiskWeightedAssets,
    SINGLE_BORROWER_LIMIT_PERCENT,
    STR_REPORTING_DEADLINE_HOURS,
    STRStatus,
    SuspicionIndicator,
    SuspiciousTransactionReport,
    Tier1Capital,
    Tier2Capital,
    // Deposit protection validators
    calculate_insured_amount,
    // AML/CFT validators
    validate_aml_compliance,
    // BOL supervision validators
    validate_banking_compliance,
    // License validators
    validate_banking_license,
    validate_bol_reporting,
    // Capital adequacy validators
    validate_capital_adequacy,
    // Foreign exchange validators
    validate_capital_flow,
    validate_car,
    validate_cdd,
    validate_cdd_review,
    validate_deposit_claim,
    validate_deposit_insured,
    // Interest rate validators
    validate_deposit_rate,
    validate_exchange_rate,
    validate_fit_and_proper,
    validate_fx_account,
    // Prudential validators
    validate_large_exposures,
    validate_lcr,
    validate_lending_rate,
    validate_leverage_ratio,
    validate_license_for_activity as validate_banking_license_for_activity,
    validate_liquidity,
    validate_mfi_capital,
    validate_minimum_capital,
    // Payment system validators
    validate_mobile_banking_compliance,
    validate_nsfr,
    validate_payment_provider,
    validate_pep_identification,
    validate_record_keeping,
    validate_related_party_limit,
    validate_reserve_requirement,
    validate_risk_weight,
    validate_rtgs_transaction,
    validate_sanctions_screening,
    validate_single_borrower_limit,
    validate_str_reporting,
    validate_tier1_ratio,
    validate_usury,
};
