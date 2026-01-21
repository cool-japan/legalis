//! Australian Privacy Law (Privacy Act 1988)
//!
//! This module implements Australia's privacy law framework, covering:
//! - **Australian Privacy Principles (APPs)** - 13 principles governing personal information
//! - **Notifiable Data Breaches (NDB)** - Mandatory breach notification scheme
//! - **Credit Reporting** - Part IIIA credit reporting provisions
//! - **APP Entities** - Organisations and agencies covered
//!
//! ## Regulatory Authority
//!
//! The Office of the Australian Information Commissioner (OAIC) administers the
//! Privacy Act, with powers to investigate complaints, conduct assessments, and
//! enforce compliance.
//!
//! ## Australian Privacy Principles
//!
//! The 13 APPs are grouped into five categories:
//!
//! ### Part 1: Consideration of personal information privacy
//! - **APP 1**: Open and transparent management of personal information
//! - **APP 2**: Anonymity and pseudonymity
//!
//! ### Part 2: Collection of personal information
//! - **APP 3**: Collection of solicited personal information
//! - **APP 4**: Dealing with unsolicited personal information
//! - **APP 5**: Notification of collection
//!
//! ### Part 3: Dealing with personal information
//! - **APP 6**: Use or disclosure of personal information
//! - **APP 7**: Direct marketing
//! - **APP 8**: Cross-border disclosure
//! - **APP 9**: Adoption, use or disclosure of government identifiers
//!
//! ### Part 4: Integrity of personal information
//! - **APP 10**: Quality of personal information
//! - **APP 11**: Security of personal information
//!
//! ### Part 5: Access to, and correction of, personal information
//! - **APP 12**: Access to personal information
//! - **APP 13**: Correction of personal information
//!
//! ## Notifiable Data Breaches (NDB) Scheme
//!
//! Since February 2018, APP entities must notify affected individuals and the
//! OAIC of eligible data breaches.
//!
//! **Eligible data breach** occurs when:
//! 1. Unauthorized access, disclosure, or loss of personal information
//! 2. Likely to result in serious harm to individuals
//! 3. Remedial action not possible
//!
//! **Notification timeline:**
//! - Assessment within 30 days
//! - Notification to OAIC and individuals "as soon as practicable"
//!
//! ## Comparison with GDPR/PDPA
//!
//! | Feature | Privacy Act (AU) | GDPR (EU) | PDPA (SG) |
//! |---------|-----------------|-----------|-----------|
//! | **Legal Basis** | Consent + exceptions | 6 lawful bases | Consent-centric |
//! | **DPO** | Not required | Mandatory for some | Recommended |
//! | **Breach Notification** | 30 days assessment | 72 hours | 3 days |
//! | **Fines** | Up to $50M+ | €20M/4% revenue | SGD 1M |
//! | **Cross-border** | Accountability approach | Adequacy/SCCs | Comparable protection |
//!
//! ## Key Amendments
//!
//! - Privacy Amendment (NDB) Act 2017 - Mandatory breach notification
//! - Privacy Legislation Amendment Act 2022 - Increased penalties
//! - Privacy Act Review 2023 - Proposed reforms (pending)
//!
//! ## Leading Cases
//!
//! - Privacy Commissioner v Telstra (2017) - Access rights
//! - OAIC v Australian Federal Police (2015) - Exemptions
//! - RI v Department of Human Services (2017) - Correction rights

pub mod apps;
pub mod breach;
pub mod credit;
pub mod entities;
pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use apps::{
    App, AppAnalyzer, AppCompliance, CollectionAnalysis, CrossBorderAnalysis,
    DirectMarketingAnalysis, DisclosureAnalysis, SecurityAnalysis,
};
pub use breach::{
    BreachAssessment, BreachNotification, DataBreach, DataBreachAnalyzer, EligibleBreach,
    NotificationRequirement, SeriousHarmAssessment,
};
pub use credit::{CreditProvider, CreditReportingAnalyzer, CreditReportingBody};
pub use entities::{Agency, AppEntity, EntityType, Organisation, SmallBusinessExemption};
pub use error::{PrivacyError, Result};
pub use types::{
    Consent, ConsentMethod, PersonalInformation, PersonalInformationType, PrivacyPolicy,
    SensitiveInformation,
};
pub use validator::{ComplianceReport, PrivacyValidator, SecurityMeasures};

use legalis_core::{Effect, EffectType, Statute};

/// Create Privacy Act 1988 statute
pub fn create_privacy_act() -> Statute {
    Statute::new(
        "AU-PA-1988",
        "Privacy Act 1988 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Privacy regulation including Australian Privacy Principles (APPs) and Notifiable Data Breaches (NDB) scheme",
        ),
    )
    .with_jurisdiction("AU")
}
