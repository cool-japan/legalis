//! # Labor Law - CLT (Consolidação das Leis do Trabalho)
//!
//! Lei nº 5.452/1943 - Brazil's comprehensive labor code.
//!
//! ## Overview
//!
//! The CLT is one of the most protective labor laws globally, establishing
//! minimum standards for employment relationships.
//!
//! ## Key Provisions
//!
//! | Provision | Article | Details |
//! |-----------|---------|---------|
//! | Working Hours | Art. 58 | 44h/week, 8h/day maximum |
//! | Overtime | Art. 59 | +50% minimum, +100% Sundays/holidays |
//! | 13th Salary | Art. 7, CF | 1 month salary (Christmas bonus) |
//! | Vacation | Art. 129 | 30 days + 1/3 bonus |
//! | FGTS | Lei 8.036/90 | 8% monthly severance fund |
//! | Notice Period | Art. 487 | 30 days minimum, +3 days per year |
//!
//! ## Employment Types
//!
//! | Type | Characteristics |
//! |------|-----------------|
//! | CLT | Full protection, FGTS, 13th, vacation |
//! | Temporary | Max 180 days, limited benefits |
//! | Intermittent | On-call, hourly payment |
//! | Part-time | Max 30h/week, proportional benefits |
//!
//! ## Termination Types
//!
//! | Type | FGTS Penalty | Notice | Unemployment Insurance |
//! |------|--------------|--------|------------------------|
//! | Without Cause | 40% | Yes | Yes |
//! | For Cause | 0% | No | No |
//! | Resignation | 0% | Yes (or deduction) | No |
//! | Mutual Agreement | 20% | 50% | No |
//!
//! ## Severance Calculation
//!
//! ```rust,ignore
//! use legalis_br::labor_law::*;
//!
//! // Full severance calculation combines multiple components
//! let notice_period = calculate_notice_period(3);
//! let thirteenth = calculate_13th_salary(5000_00, 6);
//! let vacation = calculate_vacation_payment(5000_00, 12);
//! let fgts = calculate_fgts_deposit(5000_00);
//! ```
//!
//! ## Labor Courts
//!
//! - **Varas do Trabalho**: First instance
//! - **TRTs**: Regional Labor Courts (appeals)
//! - **TST**: Superior Labor Court (final instance)

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
