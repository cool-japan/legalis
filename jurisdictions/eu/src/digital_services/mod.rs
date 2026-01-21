//! Digital Services Act (DSA) and Digital Markets Act (DMA) Implementation
//!
//! This module provides comprehensive modeling of the EU's digital services regulation.
//!
//! ## Digital Services Act (Regulation EU 2022/2065)
//!
//! The DSA establishes harmonized rules for digital services, with obligations varying by platform size:
//!
//! ### Platform Classification
//!
//! 1. **Intermediary Services** - Basic obligations (e.g., notice and action)
//! 2. **Hosting Services** - Additional transparency requirements
//! 3. **Online Platforms** - Full platform obligations including internal complaints
//! 4. **Very Large Online Platforms (VLOPs)** - 45M+ users, systemic risk assessments
//! 5. **Very Large Online Search Engines (VLOSEs)** - 45M+ users, algorithmic transparency
//!
//! ### Key DSA Obligations
//!
//! - **Article 16**: Notice and action mechanism for illegal content
//! - **Article 17**: Statement of reasons for content moderation decisions
//! - **Article 20-21**: Internal complaint systems and out-of-court dispute settlement
//! - **Article 22**: Trusted flagger framework
//! - **Article 27**: Algorithmic transparency (VLOPs/VLOSEs)
//! - **Article 34**: Systemic risk assessment (VLOPs/VLOSEs)
//! - **Article 35**: Risk mitigation measures (VLOPs/VLOSEs)
//! - **Article 42**: Enhanced transparency reporting (VLOPs/VLOSEs)
//!
//! ## Digital Markets Act (Regulation EU 2022/1925)
//!
//! The DMA regulates "gatekeepers" - large platforms providing core platform services.
//!
//! ### Gatekeeper Designation (Article 3)
//!
//! Platforms are designated as gatekeepers if they meet quantitative thresholds:
//! - €7.5B annual EEA turnover OR €75B market capitalization
//! - 45M+ monthly active end users AND 10k+ yearly active business users
//! - Operates in at least 3 Member States
//! - Met thresholds for 3 consecutive years (entrenched position)
//!
//! ### Core Platform Services (Article 2)
//!
//! - Online intermediation services (marketplaces)
//! - Online search engines
//! - Online social networking services
//! - Video-sharing platforms
//! - Interpersonal communications (messaging)
//! - Operating systems
//! - Web browsers
//! - Virtual assistants
//! - Cloud computing
//! - Online advertising
//!
//! ### Key DMA Obligations
//!
//! **Article 5 (Self-Executing)**:
//! - No combining personal data without consent
//! - Allow un-installation of pre-installed software
//! - Allow third-party app stores and sideloading
//! - No leveraging business user data for own services
//! - No tying of services
//! - No self-preferencing in ranking
//!
//! **Article 6 (Specification Required)**:
//! - Third-party interoperability (especially messaging)
//! - Effective data portability tools
//! - Business user data access
//! - Effective unsubscribe mechanisms
//! - No tracking outside core platform without consent
//! - User choice of default browser/search engine
//! - Advertising performance data for advertisers/publishers
//! - FRAND access to platform features
//! - Fair and non-discriminatory app store terms
//!
//! ## Example Usage
//!
//! ### DSA - Validate VLOP Status
//!
//! ```rust
//! use legalis_eu::digital_services::*;
//! use chrono::Utc;
//!
//! let platform = PlatformType::VeryLargeOnlinePlatform {
//!     monthly_active_recipients: 50_000_000, // 50M users
//!     designation_date: Utc::now(),
//!     systemic_risk_designation: true,
//! };
//!
//! let validation = validate_platform_classification(&platform)
//!     .expect("Validation failed");
//!
//! assert!(validation.is_compliant());
//! // VLOP must conduct systemic risk assessments
//! assert!(validation.applicable_obligations.iter()
//!     .any(|o| o.contains("Systemic risk assessment")));
//! ```
//!
//! ### DSA - Process Illegal Content Notice
//!
//! ```rust
//! use legalis_eu::digital_services::*;
//! use chrono::Utc;
//!
//! let notice = IllegalContentNotice {
//!     notice_id: "N12345".to_string(),
//!     submission_date: Utc::now(),
//!     content_type: IllegalContent::TerroristContent,
//!     content_location: "https://example.com/content".to_string(),
//!     explanation: "Content promotes terrorism".to_string(),
//!     notifier_contact: "user@example.com".to_string(),
//!     is_trusted_flagger: false,
//! };
//!
//! let validation = validate_illegal_content_notice(&notice)
//!     .expect("Validation failed");
//! assert!(validation.is_compliant());
//! ```
//!
//! ### DMA - Validate Gatekeeper Designation
//!
//! ```rust
//! use legalis_eu::digital_services::*;
//! use chrono::Utc;
//!
//! let designation = GatekeeperDesignation {
//!     company_name: "BigTech Corp".to_string(),
//!     designated_services: vec![CorePlatformService::OnlineSearchEngines],
//!     designation_date: Utc::now(),
//!     meets_quantitative_thresholds: QuantitativeThresholds {
//!         significant_impact_on_internal_market: true,
//!         operates_in_multiple_member_states: true,
//!         substantial_user_base: true,
//!         entrenched_and_durable_position: true,
//!     },
//!     contested: false,
//! };
//!
//! let validation = validate_gatekeeper_designation(&designation)
//!     .expect("Validation failed");
//! assert!(validation.is_compliant());
//! ```
//!
//! ## Penalties
//!
//! - **DSA**: Up to 6% of global annual turnover
//! - **DMA**: Up to 10% of global annual turnover (20% for repeated infringements)

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::DigitalServicesError;
pub use types::{
    AlgorithmicTransparency, AutomatedDecisionInfo, CorePlatformService, DmaComplianceReport,
    GatekeeperDesignation, GatekeeperObligation, IllegalContent, IllegalContentNotice,
    InteroperabilityAccessTerms, InteroperabilityRequirement, MitigationMeasureType,
    ModerationDecision, ModerationStatistics, NoticeDecision, NoticeResponse, NoticeStatistics,
    ObligationCompliance, PlatformType, QuantitativeThresholds, RedressMechanism,
    RiskMitigationMeasure, StatementOfReasons, SystemicRisk, TransparencyReport,
};
pub use validator::{
    DsaValidationResult, validate_dma_compliance_report, validate_gatekeeper_designation,
    validate_illegal_content_notice, validate_interoperability_requirement,
    validate_notice_response, validate_platform_classification, validate_statement_of_reasons,
    validate_transparency_report,
};
