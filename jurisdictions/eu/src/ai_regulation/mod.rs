//! EU AI Act (Regulation EU 2024/1689) Implementation
//!
//! This module provides comprehensive modeling of the EU's Artificial Intelligence Act,
//! the world's first comprehensive AI regulation framework.
//!
//! ## Overview
//!
//! The AI Act establishes a risk-based regulatory framework for AI systems:
//!
//! ### Risk-Based Classification
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │ UNACCEPTABLE RISK → PROHIBITED (Article 5)      │
//! │ - Social scoring                                 │
//! │ - Subliminal manipulation                        │
//! │ - Real-time biometric ID (limited exceptions)   │
//! └─────────────────────────────────────────────────┘
//!          ↓
//! ┌─────────────────────────────────────────────────┐
//! │ HIGH RISK → STRICT REQUIREMENTS (Articles 6-51) │
//! │ - Biometric identification                       │
//! │ - Critical infrastructure                        │
//! │ - Education/Employment                           │
//! │ - Law enforcement/Justice                        │
//! └─────────────────────────────────────────────────┘
//!          ↓
//! ┌─────────────────────────────────────────────────┐
//! │ LIMITED RISK → TRANSPARENCY (Article 52)        │
//! │ - Chatbots (must inform users)                  │
//! │ - Emotion recognition                            │
//! │ - Deep fakes (must mark content)                │
//! └─────────────────────────────────────────────────┘
//!          ↓
//! ┌─────────────────────────────────────────────────┐
//! │ MINIMAL RISK → NO OBLIGATIONS                   │
//! │ - Most AI applications                           │
//! │ - Voluntary codes of conduct                    │
//! └─────────────────────────────────────────────────┘
//! ```
//!
//! ## Prohibited AI Practices (Article 5)
//!
//! The following AI systems are banned in the EU:
//!
//! 1. **Subliminal manipulation** causing harm
//! 2. **Exploitation of vulnerabilities** (age, disability, socioeconomic status)
//! 3. **Social scoring** by public authorities
//! 4. **Real-time remote biometric identification** in public spaces (with narrow exceptions)
//! 5. **Biometric categorization** based on sensitive attributes (race, politics, etc.)
//! 6. **Emotion recognition** in workplace and education
//! 7. **Indiscriminate scraping** of facial images from internet/CCTV
//!
//! ## High-Risk AI Systems (Articles 6-51)
//!
//! ### Categories (Annex III)
//!
//! - **Biometric identification and categorization**
//! - **Critical infrastructure** (transport, energy, water)
//! - **Education and vocational training**
//! - **Employment** (recruitment, promotion, monitoring)
//! - **Essential services** (credit scoring, emergency dispatch)
//! - **Law enforcement** (crime prediction, evidence evaluation)
//! - **Migration and border control**
//! - **Justice and democratic processes**
//!
//! ### Requirements Before Market Placement
//!
//! High-risk AI systems must comply with:
//!
//! - **Article 9**: Risk management system (continuous, iterative)
//! - **Article 10**: Data governance (quality, representativeness, bias examination)
//! - **Article 11**: Technical documentation (Annex IV)
//! - **Article 12**: Record-keeping (automatic logging)
//! - **Article 13**: Transparency and information to deployers
//! - **Article 14**: Human oversight (HITL, HOTL, HIC)
//! - **Article 15**: Accuracy, robustness, cybersecurity
//! - **Article 43**: Conformity assessment
//! - **Article 71**: EU database registration
//!
//! ## Limited Risk - Transparency Obligations (Article 52)
//!
//! Systems must inform users they are interacting with AI:
//!
//! - **Chatbots**: Clear disclosure of AI nature
//! - **Emotion recognition**: Users must be informed
//! - **Deep fakes**: Content must be marked as artificially generated/manipulated
//!
//! ## General-Purpose AI (Article 51)
//!
//! Providers of general-purpose AI models (e.g., foundation models) must:
//!
//! - Provide technical documentation
//! - Provide information about training data
//! - Comply with EU copyright law
//! - Publish summary of training data
//!
//! **Systemic Risk Models** (large-scale models) have additional requirements:
//! - Model evaluation
//! - Adversarial testing
//! - Tracking and reporting serious incidents
//! - Cybersecurity measures
//!
//! ## Example Usage
//!
//! ### Validate High-Risk Employment AI
//!
//! ```rust
//! use legalis_eu::ai_regulation::*;
//! use chrono::Utc;
//!
//! let system = AiSystem {
//!     system_id: "HR-AI-001".to_string(),
//!     name: "Resume Screening AI".to_string(),
//!     description: "AI system for automated resume screening".to_string(),
//!     provider: "HRTech Inc".to_string(),
//!     deployer: Some("BigCorp".to_string()),
//!     intended_purpose: "Screen and rank job applicants".to_string(),
//!     risk_level: RiskLevel::HighRisk {
//!         category: HighRiskCategory::Employment {
//!             use_case: "recruitment".to_string(),
//!         },
//!     },
//!     adaptive: true,
//!     market_placement_date: None, // Not yet on market
//!     conformity_status: ConformityStatus::InProgress {
//!         expected_completion: Utc::now(),
//!     },
//! };
//!
//! let validation = validate_ai_system(&system)
//!     .expect("Validation failed");
//!
//! // High-risk system requires multiple obligations
//! assert!(validation.applicable_requirements.len() >= 8);
//! assert!(validation.applicable_requirements.iter()
//!     .any(|r| r.contains("Risk management")));
//! ```
//!
//! ### Detect Prohibited Practice
//!
//! ```rust
//! use legalis_eu::ai_regulation::*;
//!
//! let system = AiSystem {
//!     system_id: "SC-001".to_string(),
//!     name: "Social Credit System".to_string(),
//!     description: "Citizen behavior scoring".to_string(),
//!     provider: "BadAI Corp".to_string(),
//!     deployer: None,
//!     intended_purpose: "Rate citizens based on behavior".to_string(),
//!     risk_level: RiskLevel::Unacceptable {
//!         prohibited_practice: ProhibitedPractice::SocialScoring,
//!     },
//!     adaptive: false,
//!     market_placement_date: None,
//!     conformity_status: ConformityStatus::NotAssessed,
//! };
//!
//! // This will return an error - social scoring is prohibited
//! let result = validate_ai_system(&system);
//! assert!(result.is_err());
//! ```
//!
//! ### Validate Chatbot Transparency
//!
//! ```rust
//! use legalis_eu::ai_regulation::*;
//!
//! let transparency = TransparencyObligation {
//!     system_type: LimitedRiskType::Chatbot,
//!     users_informed: true,
//!     notification_method: "Displayed at start: 'You are chatting with an AI'".to_string(),
//!     content_marked: false, // Not applicable for chatbots
//! };
//!
//! let validation = validate_transparency_obligation(&transparency)
//!     .expect("Validation failed");
//! assert!(validation.is_compliant());
//! ```
//!
//! ## Penalties
//!
//! - **Prohibited practices (Article 5)**: Up to €35M or 7% of global turnover
//! - **High-risk non-compliance**: Up to €15M or 3% of global turnover
//! - **Incorrect/misleading information**: Up to €7.5M or 1% of global turnover
//!
//! ## Timeline
//!
//! - **2024**: Regulation enters into force
//! - **2025**: Prohibited practices ban takes effect
//! - **2026**: General-purpose AI rules apply
//! - **2027**: High-risk AI requirements fully applicable

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::AiRegulationError;
pub use types::{
    AccuracyRobustness, AiLiteracy, AiSystem, BiasExamination, ConformityStatus, DataGovernance,
    DataQuality, GeneralPurposeAiModel, HighRiskCategory, HighRiskRequirements, HumanOversight,
    HumanOversightMeasure, IdentifiedRisk, LimitedRiskType, LoggedEvent, Modification,
    ProhibitedPractice, RecordKeeping, RiskLevel, RiskLikelihood, RiskManagementSystem,
    RiskSeverity, TechnicalDocumentation, TransparencyObligation, TransparencyRequirements,
};
pub use validator::{
    AiActValidationResult, validate_ai_system, validate_general_purpose_ai,
    validate_high_risk_requirements, validate_transparency_obligation,
};
