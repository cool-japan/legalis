//! Singapore Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of Singapore law across seven major domains:
//!
//! ## Legal Domains Covered
//!
//! 1. **Companies Act** (Cap. 50) - Company formation, ACRA registration, directors, shareholders
//! 2. **Employment Act** (Cap. 91) - Working hours, leave entitlements, dismissal protection, salary
//! 3. **PDPA** (Personal Data Protection Act 2012) - Data protection, consent, DNC Registry
//! 4. **Consumer Protection** - Sale of Goods Act, Consumer Protection (Fair Trading) Act
//! 5. **Intellectual Property** - Patents, Trademarks, Copyright, Registered Designs
//! 6. **Banking Act** (Cap. 19) - Basel III capital adequacy, AML/CFT, banking licenses
//! 7. **Payment Services Act 2019** - Payment services, e-wallets, digital payment tokens (DPT)
//!
//! ## Singapore Legal System Characteristics
//!
//! Singapore follows a **common law system** based on English law, with statutory modifications:
//!
//! ### Legal Hierarchy
//!
//! ```text
//! Constitution of Singapore (1965)
//!     ├── Fundamental Liberties (Part IV)
//!     └── Government Structure
//!          ↓
//! Statutes (Acts of Parliament)
//!     ├── Companies Act (Cap. 50)
//!     ├── Employment Act (Cap. 91)
//!     ├── Personal Data Protection Act 2012
//!     └── Sale of Goods Act (Cap. 393)
//!          ↓
//! Subsidiary Legislation
//!     ├── Regulations
//!     ├── Rules
//!     └── Orders
//!          ↓
//! Case Law (Judicial Precedents)
//!     ├── Court of Appeal (final authority)
//!     └── High Court
//! ```
//!
//! ### Key Regulatory Bodies
//!
//! - **ACRA**: Accounting and Corporate Regulatory Authority (company registration, BizFile+)
//! - **MOM**: Ministry of Manpower (employment, work passes, CPF)
//! - **PDPC**: Personal Data Protection Commission (data protection enforcement)
//! - **CASE**: Consumers Association of Singapore (consumer protection advocacy)
//!
//! ## Citation Format
//!
//! Singapore statutes use **chapter numbers** for older statutes, none for newer:
//!
//! - **Older Acts**: "Companies Act (Cap. 50), s. 145(1)" or "CA s. 145"
//! - **Modern Acts**: "PDPA s. 13" (no Cap. for post-2000 acts)
//! - **Subsidiary Legislation**: "Companies Regulations (Cap. 50, Rg 1)"
//! - **Case Law**: "\[2024\] SGCA 15" (Court of Appeal), "\[2023\] SGHC 150" (High Court)
//!
//! ## Domain 1: Companies Act (Cap. 50)
//!
//! The Companies Act governs company formation and governance in Singapore.
//!
//! ### Key Requirements
//!
//! - **UEN (Unique Entity Number)**: 9-10 digit identifier for all business entities
//! - **Resident Director**: At least 1 director ordinarily resident in Singapore (s. 145)
//! - **Company Secretary**: Mandatory within 6 months of incorporation (s. 171)
//! - **AGM**: Within 18 months of incorporation, then annually within 6 months of FYE (s. 175)
//! - **Annual Return**: Must be filed within 7 months of FYE (s. 197)
//!
//! ### Example: Company Formation
//!
//! ```rust,ignore
//! use legalis_sg::companies::*;
//!
//! // Create a Singapore Pte Ltd company
//! let company = Company::builder()
//!     .name("Tech Innovations Pte Ltd")
//!     .company_type(CompanyType::PrivateLimited)
//!     .share_capital(ShareCapital::new(100_000_00)) // SGD 100,000 in cents
//!     .add_director(Director::new("John Tan", "S1234567A", true)) // Resident director
//!     .registered_address(Address::singapore("1 Raffles Place", "048616"))
//!     .build()?;
//!
//! // Validate company formation
//! match validate_company_formation(&company) {
//!     Ok(report) => println!("✅ Company formation valid"),
//!     Err(e) => eprintln!("❌ Validation failed: {}", e),
//! }
//! ```
//!
//! ## Domain 2: Employment Act (Cap. 91)
//!
//! The Employment Act provides minimum employment standards for workers in Singapore.
//!
//! ### Coverage
//!
//! - **Workmen**: Earning ≤ SGD 4,500/month
//! - **Non-workmen**: Earning ≤ SGD 2,600/month
//!
//! ### Key Provisions
//!
//! - **Working Hours** (s. 38): Max 44 hours/week (non-shift), 48 hours/week (shift)
//! - **Overtime**: 1.5x regular rate (s. 38(4))
//! - **Annual Leave** (s. 43): 7 days (year 1) → 14 days (year 8+)
//! - **Sick Leave** (s. 89): 14 days outpatient + 60 days hospitalization (after 3 months)
//! - **Maternity Leave**: 16 weeks for citizens (CDCA)
//! - **CPF**: Employer 17%, Employee 20% (age ≤55), wage ceiling SGD 6,000/month
//!
//! ### Example: CPF Contribution
//!
//! ```rust,ignore
//! use legalis_sg::employment::*;
//!
//! // Calculate CPF contribution for a 30-year-old earning SGD 5,000/month
//! let cpf = CpfContribution::new(30, 5_000_00); // Age 30, SGD 5,000 in cents
//! let breakdown = cpf.calculate()?;
//!
//! println!("Employer contribution: SGD {:.2}", breakdown.employer_amount_sgd());
//! println!("Employee contribution: SGD {:.2}", breakdown.employee_amount_sgd());
//! // Output: Employer: SGD 850.00 (17%), Employee: SGD 1,000.00 (20%)
//! ```
//!
//! ## Domain 3: PDPA (Personal Data Protection Act 2012)
//!
//! Singapore's data protection framework, distinct from GDPR.
//!
//! ### PDPA vs GDPR Key Differences
//!
//! | Feature | GDPR | PDPA (Singapore) |
//! |---------|------|------------------|
//! | **Scope** | EU/EEA + extraterritorial | Singapore organizations |
//! | **Legal Basis** | 6 lawful bases | Consent-centric (with exceptions) |
//! | **DPO** | Mandatory for certain processing | Recommended (not mandatory) |
//! | **Breach Notification** | 72 hours to authority | 3 calendar days to PDPC (if notifiable) |
//! | **Fines** | Up to €20M or 4% revenue | Up to SGD 1M |
//! | **Right to be Forgotten** | Explicit (Art. 17) | Limited (correction/access only) |
//! | **DNC Registry** | No equivalent | Part IX - opt-out for marketing |
//!
//! ### Key Obligations
//!
//! - **Consent** (s. 13): Obtain valid consent for collection, use, disclosure
//! - **Purpose Limitation** (s. 18): Collect only for reasonable purposes
//! - **Data Breach Notification** (s. 26B/26C): Notify PDPC within 3 calendar days
//! - **DNC Compliance** (Part IX): Check Do Not Call Registry before marketing
//! - **Access Requests** (s. 21): Respond within 30 days
//!
//! ### Example: Consent Management
//!
//! ```rust,ignore
//! use legalis_sg::pdpa::*;
//!
//! // Record consent for marketing emails
//! let consent = ConsentRecord::builder()
//!     .data_subject_id("customer@example.com")
//!     .purpose(PurposeOfCollection::Marketing)
//!     .consent_method(ConsentMethod::Electronic)
//!     .add_data_category(PersonalDataCategory::Email)
//!     .timestamp_now()
//!     .build()?;
//!
//! // Validate consent
//! match validate_consent(&consent) {
//!     Ok(()) => println!("✅ Consent valid"),
//!     Err(PdpaError::MissingConsent { .. }) => eprintln!("❌ Consent not obtained"),
//! }
//! ```
//!
//! ## Domain 4: Consumer Protection
//!
//! ### Sale of Goods Act (Cap. 393)
//!
//! Implied terms in contracts for sale of goods:
//!
//! - **s. 13**: Goods must correspond to description
//! - **s. 14(2)**: Goods must be of merchantable quality
//! - **s. 14(3)**: Goods must be fit for stated purpose
//!
//! ### Consumer Protection (Fair Trading) Act (Cap. 52A)
//!
//! Prohibits unfair practices:
//!
//! - **s. 4**: False or misleading representation
//! - **s. 5**: Unconscionable conduct
//! - **s. 6**: Bait advertising
//! - **s. 7**: Harassment or coercion
//!
//! ### Example: Unfair Practice Detection
//!
//! ```rust,ignore
//! use legalis_sg::consumer::*;
//!
//! // Analyze contract for unfair practices
//! let contract = ConsumerContract::new()
//!     .business_name("Electronics Store")
//!     .consumer_name("Jane Lim")
//!     .add_term("No refunds under any circumstances")
//!     .add_term("Company not liable for defects");
//!
//! let practices = detect_unfair_practices(&contract);
//! for practice in practices {
//!     println!("⚠️  Unfair practice: {:?}", practice.practice_type);
//! }
//! ```
//!
//! ## Unique Singapore Features
//!
//! ### 1. UEN (Unique Entity Number)
//! All business entities in Singapore are assigned a 9-10 digit UEN by ACRA for identification.
//!
//! ### 2. CPF (Central Provident Fund)
//! Mandatory retirement savings scheme for Singapore citizens and PRs:
//! - Employer contribution: 17% (age ≤55)
//! - Employee contribution: 20% (age ≤55)
//! - Wage ceiling: SGD 6,000/month (Ordinary Wage)
//!
//! ### 3. DNC Registry
//! Singapore's Do Not Call Registry allows individuals to opt out of marketing calls/texts/faxes.
//!
//! ### 4. BizFile+
//! ACRA's electronic filing system for company registration and annual returns.
//!
//! ## Module Structure
//!
//! - [`banking`]: Banking Act (Cap. 19) - Basel III, AML/CFT, banking licenses
//! - [`payment`]: Payment Services Act 2019 - Payment services, e-wallets, DPT (crypto)
//! - [`companies`]: Companies Act (Cap. 50) - Formation, directors, governance
//! - [`employment`]: Employment Act (Cap. 91) - Contracts, CPF, leave, termination
//! - [`pdpa`]: Personal Data Protection Act 2012 - Consent, breach, DNC
//! - [`consumer`]: Consumer Protection - Sale of Goods, unfair practices
//! - [`ip`]: Intellectual Property - Patents, Trademarks, Copyright, Designs
//! - [`citation`]: Singapore legal citation system
//!
//! ## Examples
//!
//! This crate includes 16 comprehensive examples demonstrating real-world usage:
//!
//! ### Companies Act Examples
//! - `acra_company_registration` - Pte Ltd formation with ACRA
//! - `director_compliance_check` - Resident director validation (s. 145)
//! - `annual_compliance_checklist` - AGM and annual return deadlines
//! - `share_issuance` - Share allotment and dilution
//!
//! ### Employment Act Examples
//! - `employment_contract_validation` - Full contract validation
//! - `cpf_contribution_calculator` - CPF calculation by age groups
//! - `leave_entitlement_calculator` - Leave progression by service years
//! - `termination_notice_checker` - Notice period calculation
//!
//! ### PDPA Examples
//! - `consent_management` - Recording and withdrawing consent
//! - `data_breach_notification` - 3-day notification workflow
//! - `dnc_registry_check` - Checking DNC before marketing
//! - `dpo_requirement_assessment` - When to appoint DPO
//!
//! ### Consumer Protection Examples
//! - `consumer_contract_analysis` - Contract risk scoring
//! - `sale_of_goods_validation` - Implied terms checking
//! - `unfair_practice_detector` - Automated detection
//! - `online_transaction_compliance` - E-commerce rules
//!
//! ## Testing
//!
//! Run the comprehensive test suite:
//!
//! ```bash
//! # All tests
//! cargo nextest run --package legalis-sg
//!
//! # Specific domain
//! cargo test --package legalis-sg --test companies_validation_tests
//! cargo test --package legalis-sg --test employment_contract_tests
//! cargo test --package legalis-sg --test pdpa_consent_tests
//! cargo test --package legalis-sg --test consumer_protection_tests
//! ```
//!
//! ## License
//!
//! Licensed under either of:
//!
//! - MIT License
//! - Apache License, Version 2.0
//!
//! at your option.

pub mod banking;
pub mod citation;
pub mod common; // Common utilities - holidays, currency, names
pub mod companies;
pub mod consumer;
pub mod employment;
pub mod ip;
pub mod payment;
pub mod pdpa;
pub mod reasoning;

// Re-export commonly used types from each module

// Companies Act exports
pub use companies::{
    error::{CompaniesError, Result as CompaniesResult},
    types::{
        Address, Company, CompanySecretary, CompanyType, Director, DirectorQualification,
        DisqualificationStatus, DividendRights, MonthDay, ShareAllocation, ShareCapital,
        ShareClass, Shareholder,
    },
    validator::{
        ValidationReport, validate_agm_requirement, validate_annual_return_deadline,
        validate_company_formation, validate_director_eligibility,
        validate_resident_director_requirement,
    },
};

// Employment Act exports
pub use employment::{
    error::{EmploymentError, Result as EmploymentResult},
    types::{
        Allowance, ContractType, CpfContribution, EmploymentContract, LeaveEntitlement, LeaveType,
        TerminationNotice, WorkingHours,
    },
    validator::{
        validate_employment_contract, validate_leave_entitlement, validate_overtime_payment,
        validate_termination_notice, validate_working_hours,
    },
};

// PDPA exports
pub use pdpa::{
    error::{PdpaError, Result as PdpaResult},
    types::{
        BreachType, ConsentMethod, ConsentRecord, DataBreachNotification, DataTransfer,
        DncRegistry, DncType, DpoContact, OrganisationType, PdpaOrganisation, PersonalDataCategory,
        PurposeOfCollection,
    },
    validator::{
        validate_breach_notification, validate_consent, validate_cross_border_transfer,
        validate_dnc_compliance, validate_dpo_requirement, validate_purpose_limitation,
    },
};

// Consumer Protection exports
pub use consumer::{
    error::{ConsumerError, Result as ConsumerResult},
    types::{
        ConsumerContract, ContractTerm, ImpliedTerm, SaleOfGoods, TransactionType, UnfairPractice,
        UnfairPracticeType, WarrantyTerms, WarrantyType,
    },
    validator::{
        detect_unfair_practices, validate_consumer_contract, validate_implied_terms,
        validate_sale_of_goods,
    },
};

// Intellectual Property exports
pub use ip::{
    error::{IpError, IpType, Result as IpResult},
    types::{
        Author, ClaimType, Copyright, DesignStatus, FairDealingPurpose, Patent, PatentClaim,
        PatentStatus, PriorArt, PriorArtRelevance, RegisteredDesign, Trademark,
        TrademarkSpecification, TrademarkStatus, TrademarkType, WorkType,
    },
    validator::{
        CopyrightValidationReport, DesignValidationReport, PatentValidationReport,
        TrademarkConflict, TrademarkValidationReport, assess_distinctiveness, assess_fair_dealing,
        assess_patentability, validate_copyright, validate_design, validate_patent,
        validate_trademark,
    },
};

// Banking Act exports
pub use banking::{
    error::{BankingError, ErrorSeverity as BankingErrorSeverity, Result as BankingResult},
    types::{
        Bank, BankBuilder, BankLicenseType, CapitalAdequacy, CapitalType, CashTransactionReport,
        CashTransactionType, ComplianceOfficer, CustomerAccount as BankCustomerAccount,
        CustomerRiskCategory, LicenseStatus as BankLicenseStatus, SuspiciousTransactionReport,
    },
    validator::{
        AmlComplianceStatus, BankValidationReport, CapitalAdequacyStatus, assess_aml_compliance,
        calculate_capital_shortfall, calculate_required_capital, validate_bank,
        validate_cash_transaction, validate_customer_account as validate_bank_customer_account,
        validate_merchant_bank_activities, validate_str_filing, validate_wholesale_deposit,
    },
};

// Payment Services Act exports
pub use payment::{
    error::{ErrorSeverity as PaymentErrorSeverity, PaymentError, Result as PaymentResult},
    types::{
        CustomerPaymentAccount, DigitalPaymentToken, DptServiceType,
        LicenseStatus as PaymentLicenseStatus, PaymentAccountType, PaymentLicenseType,
        PaymentServiceProvider, PaymentServiceProviderBuilder, PaymentServiceType,
        PaymentTransaction, RiskCategory, SafeguardingArrangement, SafeguardingType,
    },
    validator::{
        AmlStatus, LicenseStatusReport, PaymentProviderValidationReport, SafeguardingStatus,
        assess_safeguarding_status, calculate_required_safeguarding, validate_customer_account,
        validate_dpt_service, validate_payment_provider, validate_safeguarding,
        validate_transaction,
    },
};

// Citation exports
pub use citation::{SingaporeCitation, Statute, StatuteSection};

// Reasoning engine exports
pub use reasoning::{
    ComplianceStatus, LegalAnalysis, LegalReasoningEngine, ReasoningError, ReasoningResult,
    RiskLevel, Violation, ViolationSeverity,
};

// Common utilities exports (legalis-i18n integration)
pub use common::{
    // Multi-ethnic name formatting
    EthnicGroup,
    // Currency formatting
    SingaporeCurrency,
    // Calendar and holidays
    SingaporeLegalCalendar,
    SingaporeNameFormatter,
    SingaporePersonName,
    calculate_legal_deadline,
    format_sgd,
    format_sgd_cents,
    is_singapore_holiday,
    is_working_day,
};
