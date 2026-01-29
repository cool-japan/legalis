//! Malaysia Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of Malaysian law across multiple major domains:
//!
//! ## Legal Domains Covered
//!
//! 1. **Federal Constitution** (1957) - Constitutional law, fundamental liberties, federalism
//! 2. **Companies Act 2016** - Company formation, SSM registration, directors, shareholders
//! 3. **Employment Act 1955** - Working hours (8h/day, 48h/week), leave, termination
//! 4. **Personal Data Protection Act (PDPA) 2010** - Data protection, consent
//! 5. **Contracts Act 1950** - Contract formation, validity, remedies
//! 6. **Islamic Law** - Syariah law for Muslims (family law, finance)
//! 7. **Tax Law** - Income Tax, SST (Sales and Service Tax), Stamp Duty
//! 8. **Intellectual Property** - Patents, Trademarks, Copyright
//! 9. **Competition Act 2010** - Anti-competitive practices, market dominance
//! 10. **Capital Markets and Services Act 2007** - Securities regulation
//!
//! ## Malaysian Legal System Characteristics
//!
//! Malaysia follows a **dual legal system**:
//!
//! ### Legal Hierarchy
//!
//! ```text
//! Federal Constitution (1957)
//!     ├── Fundamental Liberties (Part II)
//!     ├── Federation (13 states + 3 federal territories)
//!     └── Islamic Law for Muslims (List II, Schedule 9)
//!          ↓
//! Federal Laws (Acts of Parliament)
//!     ├── Companies Act 2016
//!     ├── Employment Act 1955
//!     ├── PDPA 2010
//!     └── Contracts Act 1950
//!          ↓
//! State Laws
//!     ├── Islamic Family Law (States)
//!     └── Land Law (States)
//!          ↓
//! Case Law (Judicial Precedents)
//!     ├── Federal Court (apex court)
//!     ├── Court of Appeal
//!     └── High Courts (Malaya & Sabah/Sarawak)
//!
//! Syariah Courts (parallel system for Muslims)
//!     ├── Syariah Appeal Court
//!     ├── Syariah High Court
//!     └── Syariah Subordinate Court
//! ```
//!
//! ### Key Regulatory Bodies
//!
//! - **SSM**: Suruhanjaya Syarikat Malaysia (Companies Commission of Malaysia)
//! - **LHDN**: Lembaga Hasil Dalam Negeri (Inland Revenue Board)
//! - **PDPD**: Personal Data Protection Department (under KKMM)
//! - **BNM**: Bank Negara Malaysia (Central Bank)
//! - **SC**: Securities Commission Malaysia
//! - **MyCC**: Malaysia Competition Commission
//!
//! ## Citation Format
//!
//! Malaysian statutes and case law:
//!
//! - **Statutes**: "Companies Act 2016, s. 241(1)" or "Employment Act 1955, s. 60D"
//! - **Case Law**: "\[2024\] 1 MLJ 123" (Malayan Law Journal), "\[2023\] 5 CLJ 456" (Current Law Journal)
//! - **Federal Court**: "\[2024\] 1 FC 789"
//! - **Syariah Cases**: "\[2024\] JH 1" (Jurnal Hukum)
//!
//! ## Unique Malaysian Features
//!
//! ### 1. Dual Legal System
//! - **Civil Law**: Common law system for all citizens (contracts, torts, property)
//! - **Islamic Law**: Syariah law for Muslims only (family, inheritance, Islamic finance)
//!
//! ### 2. Trilingual Legal System
//! - **Malay (Bahasa Malaysia)**: Official language, authoritative for statutes
//! - **English**: Widely used in courts and commerce
//! - **Chinese**: Commercial documentation in Chinese communities
//!
//! ### 3. SSM (Suruhanjaya Syarikat Malaysia)
//! Companies Commission registration system for all business entities.
//!
//! ### 4. EPF (Employees Provident Fund)
//! Mandatory retirement savings:
//! - Employer: 12% or 13% (depends on salary threshold)
//! - Employee: 11%
//! - Wage ceiling: RM 5,000/month
//!
//! ### 5. SST (Sales and Service Tax)
//! - Sales Tax: 5-10% on manufactured goods
//! - Service Tax: 6% on prescribed services
//! - Replaced GST in 2018
//!
//! ### 6. Islamic Banking & Finance
//! Syariah-compliant financial products governed by Islamic Banking Act 1983.
//!
//! ## Module Structure
//!
//! - [`citation`]: Malaysian legal citation system
//! - [`common`]: Common utilities - MYR currency, holidays, dates
//! - [`constitution`]: Federal Constitution 1957
//! - [`contract_law`]: Contracts Act 1950
//! - [`company_law`]: Companies Act 2016
//! - [`employment_law`]: Employment Act 1955
//! - [`data_protection`]: PDPA 2010
//! - [`islamic_law`]: Syariah law (family, finance)
//! - [`tax_law`]: Income Tax, SST, Stamp Duty
//! - [`intellectual_property`]: IP laws (Patents, Trademarks, Copyright)
//! - [`competition_law`]: Competition Act 2010
//! - [`securities_law`]: Capital Markets and Services Act 2007
//! - [`reasoning`]: Legal reasoning engine for Malaysian law
//!
//! ## Examples
//!
//! ### Company Formation
//!
//! ```rust,ignore
//! use legalis_my::company_law::*;
//!
//! let company = Company::builder()
//!     .name("Tech Innovations Sdn Bhd")
//!     .company_type(CompanyType::PrivateLimited) // Sdn Bhd
//!     .share_capital(ShareCapital::new(10000000)) // RM 100,000
//!     .add_director(Director::new("Ahmad bin Ali", "850123-01-5678", true))
//!     .registered_address("Kuala Lumpur")
//!     .build()?;
//! ```
//!
//! ### Employment Law
//!
//! ```rust,ignore
//! use legalis_my::employment_law::*;
//!
//! // Calculate EPF contribution
//! let epf = EpfContribution::new(30, 300000); // Age 30, RM 3,000
//! let breakdown = epf.calculate()?;
//! println!("Employer: RM {:.2}", breakdown.employer_amount());
//! println!("Employee: RM {:.2}", breakdown.employee_amount());
//! ```
//!
//! ### PDPA Consent
//!
//! ```rust,ignore
//! use legalis_my::data_protection::*;
//!
//! let consent = ConsentRecord::builder()
//!     .data_subject_id("customer@example.com")
//!     .purpose(PurposeOfCollection::Marketing)
//!     .consent_method(ConsentMethod::Written)
//!     .build()?;
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

pub mod citation;
pub mod common;
pub mod company_law;
pub mod competition_law;
pub mod constitution;
pub mod contract_law;
pub mod data_protection;
pub mod employment_law;
pub mod intellectual_property;
pub mod islamic_law;
pub mod reasoning;
pub mod securities_law;
pub mod tax_law;

// Re-export commonly used types

// Company Law exports
pub use company_law::{
    Company, CompanyType, Director, ShareCapital, Shareholder, validate_company_formation,
};

// Employment Law exports
pub use employment_law::{
    EmploymentContract, EpfContribution, LeaveEntitlement, WorkingHours,
    validate_employment_contract,
};

// PDPA exports
pub use data_protection::{
    ConsentMethod, ConsentRecord, DataBreachNotification, PdpaOrganisation, PurposeOfCollection,
    validate_consent,
};

// Contract Law exports
pub use contract_law::{Contract, ContractType, validate_contract};

// Islamic Law exports
pub use islamic_law::{IslamicContract, IslamicFinanceProduct, validate_shariah_compliance};

// Tax Law exports
pub use tax_law::{IncomeTax, SalesTax, ServiceTax, StampDuty};

// Common utilities exports
pub use common::{
    MalaysianCurrency, MalaysianLegalCalendar, format_myr, format_myr_cents, is_malaysian_holiday,
    is_working_day,
};
