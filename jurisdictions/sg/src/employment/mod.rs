//! Employment Act (Cap. 91) - Employment Standards
//!
//! This module implements Singapore's Employment Act (Cap. 91), which establishes
//! minimum employment standards for workers in Singapore.
//!
//! ## Coverage
//!
//! The Employment Act covers:
//!
//! - **Workmen** (manual labor): Earning ≤ SGD 4,500/month → Fully covered
//! - **Non-workmen** (administrative/clerical): Earning ≤ SGD 2,600/month → Fully covered
//! - **Earning SGD 2,601-4,500/month**: Partially covered (certain provisions)
//! - **Earning > SGD 4,500/month**: Not covered
//!
//! ## Key Provisions
//!
//! ### Working Hours (s. 38)
//! - Max 44 hours/week for non-shift workers
//! - Max 48 hours/week for shift workers
//! - Max 12 hours/day (including overtime)
//! - Minimum 1 rest day per week
//! - Overtime at minimum 1.5x regular rate (s. 38(4))
//!
//! ### Leave Entitlements
//!
//! **Annual Leave (s. 43):**
//! - Year 1: 7 days
//! - Year 2: 8 days
//! - Years 3-4: 9 days
//! - Years 5-6: 11 days
//! - Years 7-8: 12 days
//! - Year 8+: 14 days
//!
//! **Sick Leave (s. 89):**
//! - After 3 months: 14 days outpatient + 60 days hospitalization
//! - After 6 months: Full entitlement
//!
//! **Maternity Leave:**
//! - 16 weeks for Singapore citizens (under CDCA)
//!
//! ### CPF Contributions
//!
//! Central Provident Fund rates by age (2024):
//!
//! | Age | Employer | Employee | Total |
//! |-----|----------|----------|-------|
//! | ≤55 | 17% | 20% | 37% |
//! | 56-60 | 15.5% | 15% | 30.5% |
//! | 61-65 | 11.5% | 9.5% | 21% |
//! | 66-70 | 9% | 7.5% | 16.5% |
//! | 70+ | 7.5% | 5% | 12.5% |
//!
//! - Wage ceiling: SGD 6,000/month (Ordinary Wage)
//! - Applicable to Singapore citizens and PRs only
//!
//! ### Termination Notice (s. 10/11)
//!
//! - Less than 26 weeks: 1 day
//! - 26 weeks to 2 years: 1 week
//! - 2 years to 5 years: 2 weeks
//! - 5+ years: 4 weeks
//!
//! ## Module Structure
//!
//! - [`types`] - Core employment types (contracts, CPF, leave, termination)
//! - [`error`] - Bilingual error types with statute references
//! - [`validator`] - Validation functions for EA compliance
//!
//! ## Example: Complete Employment Contract Validation
//!
//! ```rust,ignore
//! use legalis_sg::employment::*;
//! use chrono::Utc;
//!
//! // Create employment contract
//! let contract = EmploymentContract {
//!     employee_name: "Jane Tan".to_string(),
//!     employee_nric: Some("S1234567A".to_string()),
//!     employer_name: "Tech Innovations Pte Ltd".to_string(),
//!     monthly_salary_cents: 500_000, // SGD 5,000
//!     contract_type: ContractType::Permanent,
//!     start_date: Utc::now(),
//!     end_date: None,
//!     working_hours: Some(WorkingHours {
//!         hours_per_week: 44.0,
//!         days_per_week: 5,
//!         is_shift_work: false,
//!         rest_days_per_week: 1,
//!     }),
//!     allowances: vec![
//!         Allowance::new("Transport", 20_000), // SGD 200
//!         Allowance::new("Meal", 15_000),      // SGD 150
//!     ],
//!     is_workman: false,
//!     is_singapore_citizen_or_pr: true,
//! };
//!
//! // Validate contract
//! match validate_employment_contract(&contract) {
//!     Ok(report) => {
//!         println!("✅ Contract valid");
//!         println!("EA Coverage: {:?}", report.ea_coverage);
//!         println!("CPF Applicable: {}", report.cpf_applicable);
//!     }
//!     Err(e) => eprintln!("❌ {}", e),
//! }
//!
//! // Calculate CPF contributions
//! let cpf = CpfContribution::new(30, 500_000); // Age 30, SGD 5,000
//! let breakdown = cpf.calculate();
//! println!("Employer CPF: SGD {:.2}", breakdown.employer_amount_sgd());
//! println!("Employee CPF: SGD {:.2}", breakdown.employee_amount_sgd());
//!
//! // Check leave entitlement
//! let leave = LeaveEntitlement::new(5); // 5 years service
//! println!("Annual leave: {} days", leave.annual_leave_days);
//! println!("Sick leave: {} outpatient + {} hospitalization",
//!          leave.sick_leave_outpatient_days,
//!          leave.sick_leave_hospitalization_days);
//!
//! // Calculate termination notice
//! let notice = TerminationNotice {
//!     service_weeks: 260, // 5 years
//!     notice_period_days: 28,
//!     termination_date: Utc::now(),
//!     reason: "Resignation".to_string(),
//!     initiated_by_employer: false,
//! };
//! println!("Required notice: {} days", notice.required_notice_days());
//! ```
//!
//! ## Singapore-Specific Features
//!
//! ### 1. CPF (Central Provident Fund)
//! Mandatory retirement savings scheme with age-based contribution rates and wage ceiling.
//!
//! ### 2. Employment Act Coverage Tiers
//! Different coverage levels based on salary thresholds and job type (workman vs non-workman).
//!
//! ### 3. Progressive Leave Entitlement
//! Annual leave increases with years of service (7→14 days).
//!
//! ### 4. Maternity Protection
//! Government-paid maternity leave scheme for citizens.
//!
//! ## Regulatory Body
//!
//! **Ministry of Manpower (MOM)**
//! - Employment standards enforcement
//! - Work pass administration
//! - CPF policy (jointly with CPF Board)
//! - Tripartite dispute resolution (TADM)
//!
//! ## Legal Citations
//!
//! All errors include Employment Act section references:
//! - "EA s. 38" - Working hours
//! - "EA s. 10/11" - Termination notice
//! - "EA s. 43" - Annual leave
//! - "EA s. 89" - Sick leave
//!
//! ## See Also
//!
//! - [Ministry of Manpower](https://www.mom.gov.sg/)
//! - [Employment Act Full Text](https://sso.agc.gov.sg/Act/EmPA1968)
//! - [CPF Contribution Rates](https://www.cpf.gov.sg/employer/employer-obligations/how-much-cpf-contributions-to-pay)

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{EmploymentError, ErrorSeverity, Result};
pub use types::{
    Allowance, ContractType, CpfContribution, EmploymentContract, LeaveEntitlement, LeaveType,
    TerminatingParty, TerminationNotice, WorkingHours,
};
pub use validator::{
    ValidationReport, calculate_hourly_rate, calculate_last_employment_day,
    calculate_prorated_leave, validate_cpf_calculation, validate_employment_contract,
    validate_leave_entitlement, validate_overtime_payment, validate_termination_notice,
    validate_working_hours,
};
