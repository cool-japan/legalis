//! Labor Law Module (労働法モジュール)
//!
//! This module provides comprehensive support for Japanese labor law,
//! including the Labor Standards Act (労働基準法 - Rōdō Kijun-hō) and
//! Labor Contract Act (労働契約法 - Rōdō Keiyaku-hō).
//!
//! # Features
//!
//! - Employment contract validation (雇用契約検証)
//! - Working hours and overtime compliance (労働時間・残業管理)
//! - Wage payment validation (賃金支払検証)
//! - Termination and dismissal validation (解雇・退職検証)
//! - Harassment detection (ハラスメント検出)
//! - Fixed-term contract conversion rules (無期転換ルール)
//! - Type-safe validation and error handling
//!
//! # Legal Framework
//!
//! ## Labor Standards Act (労働基準法 - Act No. 49 of 1947)
//!
//! The Labor Standards Act establishes minimum standards for:
//! - Working hours (労働時間) - Article 32: 8 hours/day, 40 hours/week
//! - Rest periods (休憩時間) - Article 34
//! - Days off (休日) - Article 35: At least 1 day per week
//! - Wages (賃金) - Article 24: Payment principles
//! - Overtime premiums (割増賃金) - Article 37: 25%+ premiums
//! - Termination notice (解雇予告) - Article 20: 30 days or allowance
//!
//! ## Labor Contract Act (労働契約法 - Act No. 128 of 2007)
//!
//! The Labor Contract Act governs:
//! - Good faith principle (信義誠実の原則) - Article 3
//! - Abuse of dismissal rights (解雇権濫用) - Article 16
//! - Fixed-term conversion (無期転換ルール) - Article 18: 5-year rule
//! - Contract renewal rules - Article 19
//!
//! # Examples
//!
//! ## Validating an Employment Contract
//!
//! ```rust
//! use legalis_jp::labor_law::*;
//! use chrono::Utc;
//!
//! let contract = EmploymentContract {
//!     employee_name: "山田太郎".to_string(),
//!     employer_name: "テクノロジー株式会社".to_string(),
//!     employment_type: EmploymentType::IndefiniteTerm,
//!     work_pattern: WorkPattern::Regular,
//!     start_date: Utc::now(),
//!     end_date: None,
//!     base_wage_jpy: 300_000,
//!     hours_per_day: 8,
//!     days_per_week: 5,
//!     job_description: "Software Development".to_string(),
//!     work_location: "Tokyo Office".to_string(),
//!     probation_period_days: Some(90),
//!     renewal_count: 0,
//! };
//!
//! assert!(validate_employment_contract(&contract).is_ok());
//! ```
//!
//! ## Calculating Overtime Premium
//!
//! ```rust
//! use legalis_jp::labor_law::*;
//!
//! let summary = MonthlyWorkingSummary {
//!     year: 2026,
//!     month: 1,
//!     total_hours: 180.0,
//!     overtime_hours: 20.0,
//!     late_night_hours: 5.0,
//!     holiday_hours: 8.0,
//!     days_worked: 20,
//! };
//!
//! let hourly_rate = 2_000; // ¥2,000/hour
//! let total_wage = summary.calculate_total_wage(hourly_rate);
//! assert!(total_wage > 0);
//! ```
//!
//! ## Validating Termination Notice
//!
//! ```rust
//! use legalis_jp::labor_law::*;
//! use chrono::{Utc, Duration};
//!
//! let notice = TerminationNotice {
//!     employee_name: "佐藤花子".to_string(),
//!     termination_type: TerminationType::OrdinaryDismissal,
//!     notice_date: Utc::now(),
//!     effective_date: Utc::now() + Duration::days(30),
//!     reason: "Restructuring due to business conditions".to_string(),
//!     severance_pay_jpy: Some(500_000),
//!     notice_allowance_jpy: None,
//! };
//!
//! assert!(validate_termination_notice(&notice, 10_000).is_ok());
//! ```

pub mod builder;
pub mod conversion;
pub mod error;
pub mod minimum_wage;
pub mod non_compete;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use builder::EmploymentContractBuilder;
pub use conversion::IndefiniteConversionBuilder;
pub use error::{LaborLawError, Result};
pub use minimum_wage::Prefecture;
pub use non_compete::{
    NonCompeteClause, ReasonablenessReport, validate_non_compete_reasonableness,
};
pub use types::*;
pub use validator::*;
