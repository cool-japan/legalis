//! UK Employment Law Module
//!
//! Comprehensive implementation of UK employment law covering:
//! - Employment Rights Act 1996 (ERA 1996)
//! - Working Time Regulations 1998 (WTR 1998)
//! - National Minimum Wage Act 1998 (NMWA 1998)
//! - Fixed-Term Employees Regulations 2002
//! - Part-Time Workers Regulations 2000
//!
//! # Key Legislation
//!
//! ## Employment Rights Act 1996
//!
//! ### Written Particulars (s.1)
//! Employers must provide written statement of employment particulars within 2 months of start date.
//! Must include: names, start date, pay, hours, holidays, notice periods, job title.
//!
//! ### Notice Periods (s.86)
//! Statutory minimum notice (given by employer):
//! - Less than 1 month service: No notice
//! - 1 month to 2 years: 1 week
//! - 2+ years: 1 week per year (max 12 weeks)
//!
//! ### Unfair Dismissal (s.98)
//! - Qualifying period: 2 years continuous employment
//! - Fair reasons: Capability, Conduct, Redundancy, Statutory restriction, Some Other Substantial Reason
//! - Automatically unfair (no qualifying period): Pregnancy, whistleblowing, trade union, etc.
//!
//! ### Redundancy Payments (s.162)
//! Age-based multipliers:
//! - Under 22: 0.5 week's pay per year
//! - 22-40: 1.0 week's pay per year
//! - 41+: 1.5 weeks' pay per year
//! - Maximum: 20 years, £700/week cap (April 2024)
//!
//! ## Working Time Regulations 1998
//!
//! - Maximum 48 hours per week (averaged over 17 weeks)
//! - Can opt out in writing
//! - 20-minute rest break if working 6+ hours
//! - 11 hours daily rest
//! - 24 hours weekly rest
//! - 5.6 weeks annual leave (28 days for 5-day week)
//!
//! ## National Minimum Wage Act 1998
//!
//! Rates as of April 2024:
//! - 21+: £11.44/hour (National Living Wage)
//! - 18-20: £8.60/hour
//! - Under 18: £6.40/hour
//! - Apprentice (first year): £6.40/hour
//!
//! # Example Usage
//!
//! ```rust
//! use legalis_uk::employment::*;
//! use chrono::NaiveDate;
//!
//! // Create employment contract
//! let contract = EmploymentContract::builder()
//!     .with_employee(Employee {
//!         name: "John Smith".to_string(),
//!         date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
//!         address: "123 High St, London".to_string(),
//!         national_insurance_number: Some("AB123456C".to_string()),
//!     })
//!     .with_employer(Employer {
//!         name: "Acme Ltd".to_string(),
//!         address: "456 Commercial Rd, London".to_string(),
//!         employee_count: Some(50),
//!     })
//!     .with_contract_type(ContractType::Permanent)
//!     .with_start_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
//!     .with_salary(Salary {
//!         gross_annual_gbp: 30000.0,
//!         payment_frequency: PaymentFrequency::Monthly,
//!         payment_day: 25,
//!     })
//!     .with_working_hours(WorkingHours {
//!         hours_per_week: 37,
//!         days_per_week: 5,
//!         opted_out_of_48h_limit: false,
//!         night_work_hours: None,
//!     })
//!     .with_written_particulars(true)
//!     .build();
//!
//! // Validate contract
//! match validate_employment_contract(&contract) {
//!     Ok(()) => println!("Contract complies with ERA 1996"),
//!     Err(e) => println!("Contract non-compliant: {}", e),
//! }
//!
//! // Calculate redundancy payment
//! let redundancy = RedundancyPayment {
//!     age: 45,
//!     years_of_service: 10,
//!     weekly_pay_gbp: 600.0,
//! };
//! let payment = redundancy.calculate_statutory_payment();
//! println!("Statutory redundancy: £{:.2}", payment); // £9,000
//!
//! // Check minimum wage compliance
//! let wage_check = MinimumWageAssessment {
//!     age: 25,
//!     hourly_rate_gbp: 12.00,
//!     apprentice: false,
//! };
//! assert!(wage_check.is_compliant());
//! ```
//!
//! # Module Structure
//!
//! - [`types`](types) - Core employment law types (contracts, dismissals, redundancy)
//! - [`error`](error) - Employment law error types with statute references
//! - [`validator`](validator) - Validation functions for ERA 1996, WTR 1998, NMWA 1998
//!
//! # Legal References
//!
//! - [Employment Rights Act 1996](https://www.legislation.gov.uk/ukpga/1996/18)
//! - [Working Time Regulations 1998](https://www.legislation.gov.uk/uksi/1998/1833)
//! - [National Minimum Wage Act 1998](https://www.legislation.gov.uk/ukpga/1998/39)
//! - [Fixed-Term Employees Regulations 2002](https://www.legislation.gov.uk/uksi/2002/2034)
//! - [Part-Time Workers Regulations 2000](https://www.legislation.gov.uk/uksi/2000/1551)

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{EmploymentError, Result};
pub use types::{
    AnnualLeaveEntitlement, AutomaticallyUnfairReason, ContractType, Dismissal, DismissalReason,
    DismissalType, Employee, Employer, EmploymentContract, EmploymentContractBuilder,
    FixedTermReason, MinimumWageAssessment, NoticePeriod, PaymentFrequency, PensionScheme,
    RedundancyPayment, RestEntitlement, Salary, WorkingHours,
};
pub use validator::{
    validate_annual_leave, validate_contract_type, validate_dismissal, validate_dismissal_reason,
    validate_employee, validate_employer, validate_employment_contract, validate_minimum_wage,
    validate_notice_period, validate_redundancy_payment, validate_rest_entitlements,
    validate_working_hours,
};
