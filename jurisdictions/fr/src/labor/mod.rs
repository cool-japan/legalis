//! French labor law (Code du travail)
//!
//! This module provides structured representations of French employment law,
//! including contract formation, working hours (35-hour week), and dismissal regulations.
//!
//! ## Key Features
//!
//! - **CDI/CDD contracts**: Permanent and fixed-term employment
//! - **35-hour work week**: Article L3121-27 and overtime regulations
//! - **Strict dismissal protections**: Real and serious cause requirement
//! - **Economic dismissals**: Specialized rules for layoffs
//!
//! ## Modules
//!
//! - `types`: Employment contract types, working hours, dismissal categories
//! - `error`: Labor law error types with bilingual messages
//! - `validator`: Validation functions for contracts, hours, dismissals
//! - `formation`: Contract formation articles (L1221-1, L1242-2, etc.)
//! - `working_hours`: 35-hour week and working time articles (L3121-18, L3121-27, etc.)
//! - `termination`: Dismissal and termination articles (L1232-1, L1233-3, etc.)
//!
//! ## Example
//!
//! ```
//! use legalis_fr::labor::{
//!     EmploymentContract, EmploymentContractType, WorkingHours,
//!     validate_employment_contract,
//! };
//!
//! let contract = EmploymentContract::new(
//!     EmploymentContractType::CDI,
//!     "Jean Dupont".to_string(),
//!     "TechCorp SA".to_string(),
//! )
//! .with_hourly_rate(12.50) // €12.50/hour
//! .with_working_hours(WorkingHours {
//!     weekly_hours: 35.0,
//!     daily_hours: Some(7.0),
//! });
//!
//! assert!(validate_employment_contract(&contract, true).is_ok());
//! ```

pub mod error;
pub mod formation;
pub mod termination;
pub mod types;
pub mod validator;
pub mod working_hours;

// Re-export core types
pub use error::{LaborLawError, ValidationResult};
pub use types::{
    CDDReason, DismissalType, EmploymentContract, EmploymentContractType, PersonalCause,
    TrialPeriodCategory, WorkingHours,
};

// Re-export validation functions
pub use validator::{
    SMIC_HOURLY, validate_cdd, validate_dismissal, validate_employment_contract,
    validate_minimum_wage, validate_notice_period, validate_trial_period, validate_working_hours,
};

// Re-export formation articles
pub use formation::{
    article_l1221_1, article_l1221_19, article_l1242_2, article_l1242_8, article_l1242_12,
};

// Re-export working hours articles
pub use working_hours::{
    article_l3121_18, article_l3121_20, article_l3121_27, article_l3121_33, article_l3121_34,
};

// Re-export termination articles
pub use termination::{
    article_l1231_1, article_l1232_1, article_l1232_2, article_l1233_3, article_l1234_1,
};
