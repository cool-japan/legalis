//! Personal Information Protection Act
//!
//! Implementation of Act on the Protection of Personal Information
//! (個人情報の保護に関する法律 Act No. 57 of 2003, amended 2020/2022).
//!
//! ## Key Provisions
//!
//! - **Article 15**: Purpose specification at collection (利用目的の特定)
//! - **Article 17**: Proper acquisition and consent for sensitive data (適正な取得)
//! - **Article 20**: Security management measures (安全管理措置)
//! - **Article 23**: Third-party provision restrictions (第三者提供の制限)
//! - **Article 24**: Cross-border transfer restrictions (外国にある第三者への提供の制限)
//! - **Article 25**: Record keeping for third-party provision (第三者提供に係る記録の作成等)
//! - **Article 28**: Disclosure requests (開示)
//! - **Article 29**: Correction requests (訂正等)
//! - **Article 30**: Usage stop/deletion requests (利用停止等)
//! - **Article 35-2**: Pseudonymous processed information (仮名加工情報 - 2020 amendment)
//! - **Article 36**: Anonymous processed information (匿名加工情報)
//!
//! ## 2020/2022 Amendments
//!
//! - Introduction of pseudonymous processing (仮名加工情報)
//! - Enhanced cross-border transfer requirements
//! - Data breach notification obligations
//! - Strengthened enforcement powers
//!
//! ## Examples
//!
//! ### Personal Information Handler Validation
//!
//! ```
//! use legalis_jp::personal_info_protection::{
//!     PersonalInformationHandler, BusinessType, DataHandlingVolume,
//!     UsagePurpose, PurposeType, validate_personal_info_handling
//! };
//!
//! let mut handler = PersonalInformationHandler::new(
//!     "株式会社テスト",
//!     BusinessType::StandardBusiness,
//!     DataHandlingVolume::Under100000,
//! );
//!
//! handler.purposes.push(UsagePurpose {
//!     purpose: "顧客管理".to_string(),
//!     purpose_type: PurposeType::CustomerManagement,
//!     specified_at_collection: true,
//!     consent_obtained: false,
//! });
//!
//! let report = validate_personal_info_handling(&handler)?;
//! assert!(report.is_valid());
//! # Ok::<(), legalis_jp::personal_info_protection::AppiError>(())
//! ```
//!
//! ### AI Risk Assessment
//!
//! ```
//! use legalis_jp::personal_info_protection::{
//!     AiRiskAssessment, assess_ai_risk, RiskLevel
//! };
//!
//! let assessment = AiRiskAssessment {
//!     ai_system_name: "顧客分析AI".to_string(),
//!     data_volume: 50_000,
//!     sensitive_data_included: false,
//!     automated_decision_making: true,
//!     profiling: true,
//!     high_risk_determination: false,
//!     risk_mitigation_measures: vec![],
//! };
//!
//! let report = assess_ai_risk(&assessment)?;
//! println!("Risk Level: {:?}", report.risk_level);
//! # Ok::<(), legalis_jp::personal_info_protection::AppiError>(())
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{AppiError, Result};
pub use types::{
    AiRiskAssessment, BusinessType, CrossBorderTransfer, DataHandlingVolume, DataSubject,
    DataSubjectRequest, PersonalInfoType, PersonalInformationHandler, ProvisionType, PurposeType,
    RequestType, RiskLevel, RiskReport, SecurityMeasure, SecurityMeasureType, ThirdPartyProvision,
    UsagePurpose,
};
pub use validator::{
    assess_ai_risk, quick_validate_handler, quick_validate_request, validate_data_subject_request,
    validate_personal_info_handling,
};
