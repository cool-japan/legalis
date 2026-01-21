//! Labor Law Module (ກົດໝາຍແຮງງານ)
//!
//! This module provides comprehensive support for Lao labor law based on
//! **Labor Law 2013** (Law No. 43/NA, dated December 24, 2013).
//!
//! # Legal Framework
//!
//! The Labor Law 2013 is the primary legislation governing employment relationships
//! in the Lao People's Democratic Republic. It establishes minimum standards for:
//!
//! ## Key Provisions
//!
//! ### Employment Contracts (ສັນຍາຈ້າງງານ)
//! - **Article 15-16**: Essential contract terms and types
//! - **Article 17**: Fixed-term contracts (maximum 3 years)
//! - **Article 20**: Probation period (maximum 60 days)
//!
//! ### Working Hours (ເວລາເຮັດວຽກ)
//! - **Article 51**: Statutory hours (8 hours/day, 48 hours/week, max 6 days/week)
//! - **Article 52**: Overtime limits (4 hours/day, 45 hours/month)
//! - **Article 53**: Premium rates (overtime 50%, night 20%, holiday 100%)
//! - **Article 54**: Rest periods (minimum 1 hour for 6+ hour shifts)
//! - **Article 55**: Weekly rest (minimum 1 day per week)
//!
//! ### Leave Entitlements (ການລາພັກ)
//! - **Article 58**: Annual leave (minimum 15 days)
//! - **Article 61**: Sick leave (30 days per year with full pay)
//! - **Article 62**: Maternity leave (105 days / 15 weeks)
//! - **Article 62**: Paternity leave (3 days)
//!
//! ### Wages (ຄ່າຈ້າງ)
//! - **Article 79**: Payment in Lao Kip currency
//! - Minimum wage set by decree
//! - Premium payments for overtime, night shift, and holiday work
//!
//! ### Termination (ການເລີກຈ້າງ)
//! - **Article 73**: Voluntary resignation
//! - **Article 74**: Termination by employer (30 days notice)
//! - **Article 75**: Termination for cause (immediate)
//! - **Article 77**: Severance pay (based on years of service)
//!
//! ### Labor Protection (ການປົກປ້ອງແຮງງານ)
//! - **Article 38**: Minimum working age (prohibition of child labor)
//! - **Article 94**: Occupational safety and health
//! - Prohibition of discrimination and forced labor
//!
//! ### Social Security (ປະກັນສັງຄົມ)
//! - Mandatory enrollment for all employees
//! - Coverage for work injury, sickness, maternity, old age, disability, death
//! - Employee contribution: typically 5.5%
//! - Employer contribution: typically 6.0%
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for labor law compliance
//! - **Comprehensive Coverage**: All major aspects of Labor Law 2013
//! - **Worker Protection**: Strong emphasis on employee rights and protections
//!
//! # Examples
//!
//! ## Validating an Employment Contract
//!
//! ```rust
//! use legalis_la::labor_law::*;
//! use chrono::Utc;
//!
//! let contract = EmploymentContract {
//!     employee_name: "ສົມຊາຍ ວົງສະຫວັນ".to_string(),
//!     employee_name_lao: Some("ສົມຊາຍ ວົງສະຫວັນ".to_string()),
//!     employee_id: "P1234567".to_string(),
//!     employer_name: "ບໍລິສັດເທັກໂນໂລຊີ ຈຳກັດ".to_string(),
//!     employer_name_lao: Some("ບໍລິສັດເທັກໂນໂລຊີ ຈຳກັດ".to_string()),
//!     employer_registration: "ENT-2024-001".to_string(),
//!     employment_type: EmploymentType::IndefiniteTerm,
//!     work_schedule: WorkSchedule::Regular,
//!     start_date: Utc::now(),
//!     end_date: None,
//!     probation_period_days: Some(60),
//!     job_title: "Software Developer".to_string(),
//!     job_title_lao: Some("ນັກພັດທະນາຊອບແວ".to_string()),
//!     job_description: "Develop and maintain software applications".to_string(),
//!     work_location: "Vientiane Capital".to_string(),
//!     work_location_lao: Some("ນະຄອນຫຼວງວຽງຈັນ".to_string()),
//!     hours_per_day: 8,
//!     days_per_week: 6,
//!     start_time: "08:00".to_string(),
//!     end_time: "17:00".to_string(),
//!     rest_period_minutes: 60,
//!     base_wage_lak: 3_000_000, // 3 million LAK per month
//!     hourly_rate_lak: None,
//!     allowances: vec![],
//!     payment_frequency: PaymentFrequency::Monthly,
//!     payment_method: PaymentMethod::BankTransfer,
//!     annual_leave_days: 15,
//!     sick_leave_days: 30,
//!     social_security_enrolled: true,
//!     social_security_number: Some("SS-2024-123456".to_string()),
//!     special_conditions: vec![],
//!     renewal_count: 0,
//! };
//!
//! // Validate the contract
//! match validate_employment_contract(&contract) {
//!     Ok(()) => println!("Contract is valid! / ສັນຍາຖືກຕ້ອງ!"),
//!     Err(e) => {
//!         println!("English: {}", e.english_message());
//!         println!("Lao: {}", e.lao_message());
//!     }
//! }
//! ```
//!
//! ## Calculating Overtime Premium
//!
//! ```rust
//! use legalis_la::labor_law::*;
//!
//! let summary = MonthlyWorkingSummary {
//!     year: 2026,
//!     month: 1,
//!     employee_name: "ສົມຊາຍ ວົງສະຫວັນ".to_string(),
//!     total_regular_hours: 192.0, // 8 hours * 24 days
//!     total_overtime_hours: 30.0,
//!     total_night_shift_hours: 10.0,
//!     total_holiday_work_hours: 8.0,
//!     days_worked: 24,
//!     days_absent: 0,
//!     days_on_leave: 2,
//! };
//!
//! let hourly_rate = 15_625; // LAK per hour (3M LAK / 192 hours)
//! let total_wage = summary.calculate_total_wage(hourly_rate);
//!
//! println!("Total wage: {} LAK", total_wage);
//! // Includes:
//! // - Regular: 192 * 15,625 = 3,000,000 LAK
//! // - Overtime: 30 * 15,625 * 1.5 = 703,125 LAK (50% premium)
//! // - Night shift: 10 * 15,625 * 1.2 = 187,500 LAK (20% premium)
//! // - Holiday: 8 * 15,625 * 2.0 = 250,000 LAK (100% premium)
//! // Total: 4,140,625 LAK
//! ```
//!
//! ## Validating Termination Notice
//!
//! ```rust
//! use legalis_la::labor_law::*;
//! use chrono::{Utc, Duration};
//!
//! let notice = TerminationNotice {
//!     employee_name: "ສົມຍິງ ພິມມະລາດ".to_string(),
//!     termination_type: TerminationType::EmployerTermination,
//!     notice_date: Utc::now(),
//!     effective_date: Utc::now() + Duration::days(30),
//!     reason: "Business restructuring".to_string(),
//!     reason_lao: Some("ການປັບໂຄງສ້າງທຸລະກິດ".to_string()),
//!     severance_pay_lak: Some(6_000_000), // 2 months salary
//!     notice_allowance_lak: None,
//!     years_of_service: 3.5,
//! };
//!
//! // Validate termination (3.5 years = 2 months severance)
//! let monthly_wage = 3_000_000;
//! match validate_termination_notice(&notice, monthly_wage) {
//!     Ok(()) => println!("Termination notice is valid"),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! ## Checking Social Security
//!
//! ```rust
//! use legalis_la::labor_law::*;
//!
//! let contribution = SocialSecurityContribution {
//!     employee_name: "ສົມຊາຍ ວົງສະຫວັນ".to_string(),
//!     social_security_number: "SS-2024-123456".to_string(),
//!     base_wage_lak: 3_000_000,
//!     employee_rate: 0.055, // 5.5%
//!     employer_rate: 0.060, // 6.0%
//!     coverage_types: vec![
//!         SocialSecurityType::WorkInjury,
//!         SocialSecurityType::Sickness,
//!         SocialSecurityType::Maternity,
//!         SocialSecurityType::OldAge,
//!     ],
//! };
//!
//! println!("Employee contribution: {} LAK", contribution.employee_contribution());
//! // 3,000,000 * 0.055 = 165,000 LAK
//!
//! println!("Employer contribution: {} LAK", contribution.employer_contribution());
//! // 3,000,000 * 0.060 = 180,000 LAK
//!
//! println!("Total contribution: {} LAK", contribution.total_contribution());
//! // 345,000 LAK
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::labor_law::*;
//!
//! let error = LaborLawError::ExceedsStatutoryDailyHours {
//!     actual: 10,
//!     statutory: 8,
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Working hours 10 hours/day exceeds statutory limit of 8 hours (Article 51)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ເກີນຊົ່ວໂມງເຮັດວຽກຕາມກົດໝາຍ: 10 ຊົ່ວໂມງ/ມື້ > 8 ຊົ່ວໂມງ (ມາດຕາ 51)"
//! ```
//!
//! # Legal Context
//!
//! The Labor Law 2013 was enacted to modernize Lao labor regulations and align them
//! with international labor standards while respecting Lao cultural context. It:
//!
//! - Protects worker rights and establishes minimum standards
//! - Balances employer flexibility with employee protection
//! - Promotes social dialogue between employers and employees
//! - Ensures safe and healthy working conditions
//! - Prohibits discrimination and forced labor
//! - Establishes dispute resolution mechanisms
//!
//! # Compliance Notes
//!
//! When implementing labor law compliance in Laos:
//!
//! 1. **Contract Requirements**: All employment relationships must have written contracts
//! 2. **Working Hours**: Strictly enforce 8/48 hour limits unless properly authorized
//! 3. **Leave**: Ensure minimum 15 days annual leave for all employees
//! 4. **Wages**: Pay at least minimum wage, always in Lao Kip
//! 5. **Overtime**: Pay proper premiums (50% for regular, 20% night, 100% holiday)
//! 6. **Social Security**: Mandatory enrollment for all employees
//! 7. **Termination**: Follow proper procedures and provide required severance
//! 8. **Safety**: Maintain safe working conditions per Article 94

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{LaborLawError, Result};
pub use types::*;
pub use validator::*;
