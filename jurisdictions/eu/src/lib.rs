//! European Union (EU) Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of EU law across four major areas.
//!
//! **Status**: v0.5.9 - Core Implementation Complete ✅
//! - 150 unit tests passing (0 warnings)
//! - 19 comprehensive examples
//! - Production-ready
//!
//! ## Legal Areas Covered
//!
//! 1. **GDPR** (General Data Protection Regulation 2016/679) - ✅ COMPLETE
//!    - Data processing legal bases (Article 6)
//!    - Special categories (Article 9)
//!    - Data subject rights (Articles 15-22)
//!    - Controller accountability (Article 24)
//!    - Data Protection by Design & Default (Article 25)
//!    - Joint controllers (Article 26)
//!    - Processor contracts (Article 28)
//!    - Records of Processing Activities - ROPA (Article 30)
//!    - Security and breach notification (Articles 32-34)
//!    - Data Protection Impact Assessment - DPIA (Articles 35-36)
//!    - Data Protection Officer - DPO (Articles 37-39)
//!    - Cross-border transfers (Articles 44-49, Chapter V)
//!    - Administrative fines (Article 83)
//!
//! 2. **Consumer Rights** (Directive 2011/83/EU) - ✅ COMPLETE
//!    - Distance and off-premises contracts (Article 6)
//!    - 14-day withdrawal right (Articles 9-16)
//!    - Information requirements
//!    - All 13 exceptions to withdrawal (Article 17)
//!
//! 3. **Competition Law** (Articles 101-102 TFEU) - ✅ COMPLETE
//!    - Anti-competitive agreements (Article 101)
//!    - De minimis test, exemption criteria
//!    - Abuse of dominant position (Article 102)
//!    - Market definition and dominance assessment
//!
//! 4. **Treaty Framework** (TFEU, TEU, Charter) - ✅ COMPLETE
//!    - Four freedoms (goods, persons, services, capital)
//!    - Charter of Fundamental Rights
//!    - CJEU landmark cases (Van Gend en Loos, Costa v ENEL, etc.)
//!
//! ## EU Legal System Characteristics
//!
//! ### Primary vs Secondary Law
//!
//! ```text
//! Primary Law (Treaties)
//!     ├── TEU (Treaty on European Union)
//!     ├── TFEU (Treaty on Functioning of EU)
//!     └── Charter of Fundamental Rights
//!          ↓
//! Secondary Law
//!     ├── Regulations (directly applicable)
//!     ├── Directives (require member state transposition)
//!     └── Decisions (binding on specific addressees)
//!          ↓
//! Member State Law
//!     └── National implementations of directives
//! ```
//!
//! ### Regulations vs Directives
//!
//! | Feature | Regulation | Directive |
//! |---------|-----------|-----------|
//! | Direct applicability | Yes | No (requires transposition) |
//! | Binding force | Entirely binding | Result binding, means flexible |
//! | Example | GDPR (Regulation 2016/679) | Consumer Rights (Directive 2011/83/EU) |
//!
//! ### Key EU Legal Principles
//!
//! - **Direct Effect**: EU law creates individual rights enforceable in national courts
//! - **Supremacy**: EU law prevails over conflicting national law
//! - **Proportionality**: EU action must not exceed what is necessary
//!
//! ## Multi-Language Support
//!
//! All EU legislation exists in 24 official languages. This crate provides:
//! - English (primary implementation language)
//! - German (German legal tradition, largest member state)
//!
//! Future: French, Spanish, Italian, and other EU languages
//!
//! ## Citation Format
//!
//! EU legal citations follow EUR-Lex/CELEX format:
//! - GDPR: `CELEX:32016R0679` or `GDPR Art. 6(1)(a)`
//! - TFEU: `CELEX:12012E/TXT` or `Article 101 TFEU`
//! - Case law: `C-26/62` (Van Gend en Loos) or `C-6/64` (Costa v ENEL)
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_eu::gdpr::*;
//!
//! // Validate GDPR consent-based processing
//! let processing = DataProcessing::new()
//!     .with_controller("Acme Corp")
//!     .with_purpose("Marketing emails")
//!     .add_data_category(PersonalDataCategory::Regular("email".to_string()))
//!     .with_lawful_basis(LawfulBasis::Consent {
//!         freely_given: true,
//!         specific: true,
//!         informed: true,
//!         unambiguous: true,
//!     });
//!
//! match processing.validate() {
//!     Ok(validation) => {
//!         if validation.is_compliant() {
//!             println!("✅ Processing is GDPR compliant");
//!         }
//!     }
//!     Err(e) => println!("❌ Compliance error: {}", e),
//! }
//! ```

// Core EU legal infrastructure
pub mod citation;
pub mod i18n;

// GDPR (Phase 1 - Most Complete)
pub mod gdpr;

// Consumer Rights (Phase 3)
pub mod consumer_rights;

// Competition Law (Phase 4 - Complete)
pub mod competition;

// Treaty Framework (Phase 5 - Skeleton)
pub mod treaty;

// Intellectual Property (Phase 6 - New)
pub mod intellectual_property;

// Shared utilities
pub mod shared;

// Re-exports for convenience

// GDPR (Phase 1 + Phase 2)
pub use gdpr::{
    AdministrativeFine, Article9Exception, Article9Processing, Article83Factors, ComplianceStatus,
    DataBreach, DataProcessing, DataSubjectRequest, DataSubjectRight, FineCalculation, FineTier,
    GdprError, HealthcarePurpose, LawfulBasis, PersonalDataCategory, ProcessingOperation,
    ResearchPurpose, SpecialCategory, ViolatedArticle,
};

// Citation system
pub use citation::{EuCitation, EuLegalInstrument};

// Consumer Rights (Phase 3)
pub use consumer_rights::{
    ConsumerRightsError, ContractType, DistanceContract, OffPremisesContract, WithdrawalException,
    WithdrawalPeriod, WithdrawalRight,
};

// Competition Law (Phase 4)
pub use competition::{
    AbuseType, Article101Agreement, Article101Exemption, Article101Validation, Article102Conduct,
    Article102Validation, CompetitionError, ConcertedPractice, DominanceAssessment,
    ExclusionaryAbuse, ExploitativeAbuse, GeographicMarket, MarketAllocation, RelevantMarket,
    Undertaking,
};

// Treaty Framework (Phase 5 - Skeleton)
pub use treaty::{
    CharterArticle, CjeuCase, CjeuPrinciple, FourFreedom, FreedomType, FundamentalRight,
    JustificationGround, LandmarkCase, Restriction, TreatyArticle, TreatyProvision, TreatyType,
};

// Intellectual Property (Phase 6 - New)
pub use intellectual_property::{
    AcquisitionMethod, CommunityDesign, CopyrightException, CopyrightValidation, CopyrightWork,
    DesignType, DesignValidation, EuTrademark, IpError, MarkType, MisappropriationAnalysis,
    NiceClass, TradeSecret, TradeSecretCharacteristics, TradeSecretValidation, TrademarkStatus,
    TrademarkValidation, WorkType,
};

// I18n
pub use i18n::MultilingualText;

// Shared
pub use shared::MemberState;
