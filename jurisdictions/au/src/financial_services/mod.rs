//! Financial Services Module (Corporations Act 2001 Chapter 7, ASIC Act 2001)
//!
//! Comprehensive implementation of Australian financial services regulation.
//!
//! # Key Legislation
//!
//! ## Corporations Act 2001 (Cth) - Chapter 7
//!
//! Chapter 7 is the primary Australian legislation regulating financial services. It establishes:
//! - The regulatory framework for financial services
//! - ASIC (Australian Securities and Investments Commission) as the regulator
//! - The requirement for an Australian Financial Services License (AFSL) to provide financial services
//! - The financial services disclosure regime
//!
//! ### Section 911A: The Licensing Requirement
//!
//! **"A person who carries on a financial services business in this jurisdiction must hold
//! an Australian financial services licence covering the provision of those services."**
//!
//! Breach can result in:
//! - **Criminal penalty**: Up to 5 years imprisonment and/or 500 penalty units ($156,500)
//! - **Civil penalty**: Up to $1.11 million for individuals, $11.1 million for corporations
//!
//! ### Section 761A: Financial Services Definitions
//!
//! A **financial service** includes:
//! - Providing financial product advice (s.766B)
//! - Dealing in a financial product (s.766C)
//! - Making a market for a financial product (s.766D)
//! - Operating a registered scheme (s.766E)
//! - Providing a custodial or depository service (s.766E)
//! - Providing traditional trustee company services
//!
//! ### Financial Products (s.764A)
//!
//! Financial products include:
//! - Securities (shares, bonds, debentures)
//! - Derivatives
//! - Managed investment scheme interests
//! - Superannuation products
//! - Insurance contracts
//! - Deposit products
//! - Foreign exchange contracts
//!
//! ## ASIC General Obligations (s.912A)
//!
//! AFSL holders must:
//!
//! 1. **Do all things necessary to ensure financial services are provided efficiently,
//!    honestly and fairly** (s.912A(1)(a))
//!
//! 2. **Have adequate arrangements** for managing conflicts of interest (s.912A(1)(aa))
//!
//! 3. **Comply with financial services laws** (s.912A(1)(c))
//!
//! 4. **Take reasonable steps** to ensure representatives comply (s.912A(1)(ca))
//!
//! 5. **Have adequate risk management systems** (s.912A(1)(h))
//!
//! 6. **Maintain competence** to provide financial services (s.912A(1)(e))
//!
//! 7. **Ensure representatives are adequately trained** and competent (s.912A(1)(f))
//!
//! 8. **Have adequate resources** (financial, technological, human) (s.912A(1)(d))
//!
//! 9. **Have a dispute resolution system** (s.912A(1)(g))
//!
//! 10. **Have adequate compensation arrangements** (s.912B)
//!
//! ## Client Classification (Wholesale vs Retail)
//!
//! ### Retail Clients (s.761G)
//!
//! Default category for most individuals. Retail clients receive:
//! - **Product Disclosure Statements (PDS)** before acquisition
//! - **Financial Services Guide (FSG)** from service provider
//! - **Statement of Advice (SOA)** for personal advice
//! - Access to **internal dispute resolution**
//! - Access to **external dispute resolution** (AFCA)
//!
//! ### Wholesale Clients (s.761G(7))
//!
//! Clients who are NOT retail clients. Includes:
//!
//! - **Product value test**: Acquiring product for $500,000+ consideration
//! - **Assets test**: Net assets $2.5M+
//! - **Income test**: Gross income $250,000+ for each of last 2 years
//! - **Professional investor**: s.708(11) sophisticated investor certificate
//! - **Authorized financial services licensee**
//! - **Listed entity or subsidiary**
//! - **Regulated superannuation fund** with >$10M in assets
//!
//! Wholesale clients have reduced protections (no PDS, limited dispute rights).
//!
//! ## Best Interests Duty (s.961B)
//!
//! When providing **personal advice** to retail clients, adviser must:
//!
//! 1. **Act in the best interests of the client** (s.961B(1))
//!
//! 2. **Safe harbour steps** (s.961B(2)):
//!    - (a) Identify the objectives, financial situation and needs
//!    - (b) Identify the subject matter of the advice
//!    - (c) Reasonable investigation of relevant financial products
//!    - (d) Reasonable steps to ensure advice is appropriate
//!    - (e) Base recommendations on reasonable assessment
//!    - (f) Consider whether to recommend product, and if so, which one
//!    - (g) Conduct any other relevant inquiries
//!
//! 3. **Priority rule** (s.961J): Where conflict, give priority to client's interests
//!
//! ## Product Disclosure (Division 2, Part 7.9)
//!
//! ### Financial Services Guide (FSG) (s.941A-942C)
//!
//! Licensee must give FSG to retail client before providing service:
//! - Name and contact details
//! - Services offered
//! - How advice is provided
//! - Remuneration and commissions
//! - Dispute resolution information
//! - Compensation arrangements
//!
//! ### Product Disclosure Statement (PDS) (s.1012A-1013L)
//!
//! Must be given to retail clients before they acquire a financial product:
//! - Product issuer details
//! - Significant product features
//! - Fees and charges
//! - Risks
//! - Cooling-off rights (where applicable)
//! - Complaints handling
//!
//! ### Statement of Advice (SOA) (s.946A)
//!
//! When providing personal advice to retail client:
//! - Advice and basis for advice
//! - Information relied on
//! - Warning if based on incomplete information
//! - Information about remuneration
//! - Associations or relationships
//!
//! ## Conflicted Remuneration (Division 4, Part 7.7A)
//!
//! ### Ban on Conflicted Remuneration (s.963E)
//!
//! Licensees and representatives must NOT accept **conflicted remuneration**:
//! - Volume-based benefits from product issuers
//! - Soft dollar benefits
//! - Asset-based fees on borrowed amounts in superannuation
//!
//! ### Permitted Benefits (s.963C):
//! - Flat-fee remuneration
//! - Fee-for-service arrangements
//! - Insurance commission under grandfathering
//!
//! ## AML/CTF Act 2006 (AUSTRAC)
//!
//! Separate but related legislation administered by AUSTRAC:
//! - Customer Identification Procedure (CIP)
//! - Know Your Customer (KYC)
//! - Transaction monitoring
//! - Suspicious matter reporting (SMR)
//! - International funds transfer instructions (IFTI)
//!
//! # ASIC Regulatory Guides
//!
//! Key regulatory guides:
//! - **RG 206**: Credit licensing
//! - **RG 244**: Giving information, general advice and scaled advice
//! - **RG 246**: Conflicted remuneration
//! - **RG 256**: Client money reporting rules
//! - **RG 259**: Risk management systems of responsible entities
//! - **RG 271**: Internal dispute resolution
//! - **RG 274**: Product design and distribution obligations
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_au::financial_services::*;
//! use chrono::NaiveDate;
//!
//! // Check AFSL authorization
//! let license = AfslLicense {
//!     license_number: "123456".to_string(),
//!     licensee_name: "Example Financial Services Pty Ltd".to_string(),
//!     abn: "12345678901".to_string(),
//!     status: LicenseStatus::Current,
//!     issue_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!     authorized_services: vec![
//!         AuthorizedService::ProvideFinancialProductAdvice {
//!             product_type: ProductType::Securities,
//!             client_type: ClientType::Retail,
//!         },
//!     ],
//!     conditions: vec![],
//! };
//!
//! let service = AuthorizedService::ProvideFinancialProductAdvice {
//!     product_type: ProductType::Securities,
//!     client_type: ClientType::Retail,
//! };
//!
//! validate_afsl_authorization(&license, &service)?;
//!
//! // Assess best interests duty compliance
//! let assessment: BestInterestsAssessment = /* ... */;
//! validate_best_interests_duty(&assessment)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Sub-modules for expanded financial services
pub mod advice;
pub mod afs_licensing;
pub mod aml_ctf;
pub mod banking;
pub mod managed_investments;

// Re-export key types
pub use error::{FinancialServicesError, Result};

// AFS Licensing re-exports
pub use afs_licensing::{
    AfsLicensingError, AfslCondition, AfslLicense, AuthorizedRepresentative, AuthorizedService,
    ClientType, LicenseStatus, ProductType, validate_afsl_authorization,
    validate_authorized_representative, validate_license_conditions,
};

// AML/CTF re-exports
pub use aml_ctf::{
    AmlCtfError, AuCustomerDueDiligence, AustracCompliance, CddLevel, CustomerType, EntityType,
    IdentityDocument, MonitoringFrequency, PepStatus, RiskRating, SuspiciousMatterReport,
    validate_austrac_compliance, validate_customer_identification, validate_smr,
};

// Advice re-exports
pub use advice::{
    AdviceDocument, AdviceError, AdviceType, BestInterestsAssessment, ConflictedRemuneration,
    FinancialServicesGuide, ProductDisclosureStatement, StatementOfAdvice,
    validate_best_interests_duty, validate_conflicted_remuneration, validate_fsg, validate_pds,
    validate_soa,
};

// Banking re-exports
pub use banking::{
    AdiStatus, ApraRequirement, AuthorizedDepositInstitution, BankingError, CapitalRequirement,
    LiquidityRequirement, validate_adi_compliance, validate_capital_adequacy,
};

// Managed Investments re-exports
pub use managed_investments::{
    CompliancePlan, ManagedInvestmentScheme, ManagedInvestmentsError, ResponsibleEntity,
    SchemeType, validate_compliance_plan, validate_responsible_entity,
};

// Core types re-exports
pub use types::{
    AsicObligation, ClientClassification, CompensationArrangement, DisputeResolution,
    FinancialServicesProvider, GeneralObligationsCompliance, ObligationCompliance,
    ResourceRequirement,
};

pub use validator::{
    validate_client_classification, validate_compensation_arrangements,
    validate_dispute_resolution, validate_general_obligations, validate_resources,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Corporations Act 2001 (Cth) - Chapter 7 statute
pub fn create_corporations_act_chapter_7() -> Statute {
    Statute::new(
        "AU-CA-2001-CH7",
        "Corporations Act 2001 (Cth) - Chapter 7: Financial Services and Markets",
        Effect::new(
            EffectType::Obligation,
            "Comprehensive regulation of financial services including licensing (AFSL), \
             disclosure obligations, best interests duty, and client protections",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create AML/CTF Act 2006 statute
pub fn create_aml_ctf_act() -> Statute {
    Statute::new(
        "AU-AMLCTF-2006",
        "Anti-Money Laundering and Counter-Terrorism Financing Act 2006 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "AML/CTF compliance requirements including customer identification, \
             transaction monitoring, and suspicious matter reporting administered by AUSTRAC",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_corporations_act_chapter_7() {
        let statute = create_corporations_act_chapter_7();
        assert_eq!(statute.id, "AU-CA-2001-CH7");
        assert!(statute.title.contains("Chapter 7"));
    }

    #[test]
    fn test_create_aml_ctf_act() {
        let statute = create_aml_ctf_act();
        assert_eq!(statute.id, "AU-AMLCTF-2006");
        assert!(statute.title.contains("Anti-Money Laundering"));
    }
}
