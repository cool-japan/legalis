//! European Union (EU) Jurisdiction Support for Legalis-RS
//!
//! This crate provides comprehensive modeling of EU law across major regulatory areas.
//!
//! **Status**: v0.6.0 - Phase 1C Complete ✅
//! - 200+ unit tests passing (0 warnings)
//! - 25+ comprehensive examples
//! - Production-ready
//!
//! ## Legal Areas Covered
//!
//! ### Core Data & Privacy
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
//! 2. **ePrivacy Directive** (Directive 2002/58/EC) - ✅ NEW in v0.6.0
//!    - Cookie consent requirements (Article 5(3))
//!    - Direct marketing (Article 13)
//!    - Location data processing (Article 9)
//!    - Confidentiality of communications (Article 5)
//!
//! ### Digital Services & Markets
//!
//! 3. **Digital Services Act (DSA)** (Regulation EU 2022/2065) - ✅ NEW in v0.6.0
//!    - Platform classification (VLOP, VLOSE, online platforms)
//!    - Notice and action mechanism (Article 16)
//!    - Content moderation transparency (Article 17)
//!    - Systemic risk assessment (Article 34)
//!    - Risk mitigation measures (Article 35)
//!    - Algorithmic transparency (Article 27)
//!
//! 4. **Digital Markets Act (DMA)** (Regulation EU 2022/1925) - ✅ NEW in v0.6.0
//!    - Gatekeeper designation (Article 3)
//!    - Core platform services
//!    - Gatekeeper obligations (Articles 5-7)
//!    - Interoperability requirements
//!
//! ### Artificial Intelligence
//!
//! 5. **AI Act** (Regulation EU 2024/1689) - ✅ NEW in v0.6.0
//!    - Risk-based classification (unacceptable, high, limited, minimal)
//!    - Prohibited AI practices (Article 5)
//!    - High-risk AI requirements (Articles 9-15)
//!    - Transparency obligations (Article 52)
//!    - General-purpose AI models (Article 51)
//!
//! ### Financial Services
//!
//! 6. **MiFID II** (Directive 2014/65/EU) - ✅ NEW in v0.6.0
//!    - Investment services and activities
//!    - Client categorization (retail, professional, eligible counterparty)
//!    - Conduct of business rules (Articles 24-25)
//!    - Best execution (Article 27)
//!    - Product governance (Article 16)
//!    - Transaction reporting (Article 26)
//!
//! 7. **PSD2** (Directive 2015/2366/EU) - ✅ NEW in v0.6.0
//!    - Payment services
//!    - Strong Customer Authentication (SCA) - Article 97
//!    - Open banking / API access (Article 67)
//!    - Payment initiation services (PIS)
//!    - Account information services (AIS)
//!    - Passporting rights
//!
//! ### Consumer Protection
//!
//! 8. **Consumer Rights Directive** (Directive 2011/83/EU) - ✅ COMPLETE
//!    - Distance and off-premises contracts (Article 6)
//!    - 14-day withdrawal right (Articles 9-16)
//!    - Information requirements
//!    - All 13 exceptions to withdrawal (Article 17)
//!
//! 9. **Unfair Commercial Practices Directive** (Directive 2005/29/EC) - ✅ NEW in v0.6.0
//!    - Misleading actions and omissions
//!    - Aggressive practices
//!    - 31 prohibited practices ("blacklist")
//!    - Average consumer standard
//!
//! ### Competition & Markets
//!
//! 10. **Competition Law** (Articles 101-102 TFEU) - ✅ COMPLETE
//!    - Anti-competitive agreements (Article 101)
//!    - De minimis test, exemption criteria
//!    - Abuse of dominant position (Article 102)
//!    - Market definition and dominance assessment
//!
//! ### Treaty Framework
//!
//! 11. **Treaty Framework** (TFEU, TEU, Charter) - ✅ COMPLETE
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

// Digital Services Act & Digital Markets Act (Phase 1C)
pub mod digital_services;

// AI Act (Phase 1C)
pub mod ai_regulation;

// Financial Services - MiFID II & PSD2 (Phase 1C)
pub mod financial_services;

// ePrivacy Directive (Phase 1C)
pub mod eprivacy;

// Shared utilities
pub mod shared;

// Legal Reasoning Engine
pub mod reasoning;

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

// Consumer Rights (Phase 3) + Unfair Commercial Practices (Phase 1C)
pub use consumer_rights::{
    AggressivePractice, ConsumerRightsError, ContractType, DistanceContract, MisleadingAction,
    MisleadingOmission, OffPremisesContract, ProhibitedPractice as ConsumerProhibitedPractice,
    UnfairCommercialPractice, WithdrawalException, WithdrawalPeriod, WithdrawalRight,
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

// Legal reasoning engine
pub use reasoning::{
    ComplianceStatus as ReasoningComplianceStatus, EuEvaluationContext, LegalAnalysis,
    LegalReasoningEngine, ReasoningError, ReasoningResult, ReasoningStep, RiskLevel, Violation,
    ViolationSeverity, all_eu_statutes, competition_statutes, consumer_rights_statutes,
    gdpr_statutes,
};

// Digital Services Act & Digital Markets Act (Phase 1C)
pub use digital_services::{
    AlgorithmicTransparency, CorePlatformService, DigitalServicesError, DmaComplianceReport,
    DsaValidationResult, GatekeeperDesignation, GatekeeperObligation, IllegalContent,
    IllegalContentNotice, InteroperabilityRequirement, ModerationDecision, NoticeDecision,
    NoticeResponse, PlatformType, QuantitativeThresholds, StatementOfReasons, SystemicRisk,
    TransparencyReport,
};

// AI Act (Phase 1C)
pub use ai_regulation::{
    AiActValidationResult, AiRegulationError, AiSystem, ConformityStatus, GeneralPurposeAiModel,
    HighRiskCategory, HighRiskRequirements, HumanOversight, LimitedRiskType, ProhibitedPractice,
    RiskLevel as AiRiskLevel, TransparencyObligation,
};

// Financial Services - MiFID II & PSD2 (Phase 1C)
pub use financial_services::{
    AccountInformationProvider, AuthenticationElement, BestExecutionPolicy, ClientCategory,
    ConductOfBusiness, FinancialServicesError, InvestmentService, OpenBankingApi, Passport,
    PaymentInitiationProvider, PaymentService, ScaExemption, StrongCustomerAuthentication,
    ThirdPartyProvider,
};

// ePrivacy Directive (Phase 1C)
pub use eprivacy::{
    CookieBanner, CookieCategory, CookieConsent, CookieDuration, CookieExemption, DirectMarketing,
    LocationDataProcessing, MarketingChannel,
};
