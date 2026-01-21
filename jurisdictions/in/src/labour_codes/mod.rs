//! Labour Codes 2020 Module
//!
//! # India's Consolidated Labour Codes
//!
//! This module implements India's four consolidated Labour Codes enacted in
//! 2019-2020, which replace 29 previous labour laws:
//!
//! ## The Four Labour Codes
//!
//! | Code | Year | Laws Replaced | Key Areas |
//! |------|------|---------------|-----------|
//! | Code on Wages | 2019 | 4 | Minimum wages, payment, bonus |
//! | Code on Social Security | 2020 | 9 | EPF, ESI, gratuity, maternity |
//! | Industrial Relations Code | 2020 | 3 | Unions, disputes, strikes |
//! | OSH Code | 2020 | 13 | Safety, working hours, leave |
//!
//! ## Code on Wages, 2019
//!
//! Subsumes:
//! - Payment of Wages Act, 1936
//! - Minimum Wages Act, 1948
//! - Payment of Bonus Act, 1965
//! - Equal Remuneration Act, 1976
//!
//! **Key Provisions:**
//! - National minimum wage floor (Section 9)
//! - Equal pay for equal work (Section 3)
//! - Timely payment within 7-10 days (Section 17)
//! - Maximum deductions: 50% of wages (Section 18)
//! - Bonus: 8.33% minimum to 20% maximum (Chapter III)
//!
//! ## Code on Social Security, 2020
//!
//! Subsumes:
//! - Employees' Provident Fund Act, 1952
//! - Employees' State Insurance Act, 1948
//! - Payment of Gratuity Act, 1972
//! - Maternity Benefit Act, 1961
//! - And 5 other acts
//!
//! **Social Security Contributions:**
//!
//! | Scheme | Employee | Employer | Ceiling |
//! |--------|----------|----------|---------|
//! | EPF | 12% | 12% | Rs. 15,000 |
//! | ESI | 0.75% | 3.25% | Rs. 21,000 |
//!
//! **Gratuity Formula:**
//! ```text
//! Gratuity = (Last Salary × 15 × Years) / 26
//! Minimum eligibility: 5 years
//! Maximum amount: Rs. 20 lakhs
//! ```
//!
//! **Maternity Leave:**
//! - First two children: 26 weeks
//! - Third child onwards: 12 weeks
//! - Miscarriage: 6 weeks
//! - Commissioning/adopting: 12 weeks
//!
//! ## Industrial Relations Code, 2020
//!
//! Subsumes:
//! - Industrial Disputes Act, 1947
//! - Trade Unions Act, 1926
//! - Industrial Employment (Standing Orders) Act, 1946
//!
//! **Key Thresholds:**
//!
//! | Provision | Old Threshold | New Threshold |
//! |-----------|---------------|---------------|
//! | Lay-off permission | 100 workers | 300 workers |
//! | Retrenchment permission | 100 workers | 300 workers |
//! | Closure permission | 100 workers | 300 workers |
//! | Standing orders | 100 workers | 300 workers |
//!
//! **Strike/Lockout Notice:**
//! - 14 days advance notice required
//! - Valid for 60 days from notice
//! - 14 days cooling-off period
//!
//! ## OSH Code, 2020
//!
//! Subsumes:
//! - Factories Act, 1948
//! - Mines Act, 1952
//! - Contract Labour Act, 1970
//! - Inter-State Migrant Workmen Act, 1979
//! - And 9 other acts
//!
//! **Working Hours:**
//! - Daily: 8 hours maximum
//! - Weekly: 48 hours maximum
//! - Spread over: 10.5 hours maximum
//! - Overtime rate: 2× ordinary wages
//!
//! **Leave Provisions:**
//! - Annual leave: 1 day per 20 days worked (minimum 15 days)
//! - Accumulation: Up to 30 days
//! - Encashment: Allowed on termination
//!
//! ## Example: Wage Compliance
//!
//! ```rust
//! use legalis_in::labour_codes::*;
//!
//! // Calculate minimum wage
//! let min_wage = calculate_minimum_wage(
//!     10000.0, // Floor wage
//!     GeographicalArea::Metropolitan,
//!     SkillLevel::Skilled,
//! );
//! assert_eq!(min_wage, 15600.0); // 10000 × 1.2 × 1.30
//! ```
//!
//! ## Example: EPF Contribution
//!
//! ```rust
//! use legalis_in::labour_codes::*;
//!
//! let contribution = EpfContribution::calculate(15000.0);
//! assert_eq!(contribution.employee, 1800.0); // 12%
//! assert_eq!(contribution.employer, 1800.0); // 12%
//! ```
//!
//! ## Example: Gratuity Calculation
//!
//! ```rust
//! use legalis_in::labour_codes::*;
//!
//! let gratuity = Gratuity::calculate(10.0, 30000.0);
//! // (30000 × 15 × 10) / 26 = Rs. 1,73,076.92
//! assert!(gratuity.amount > 173000.0);
//! ```
//!
//! ## Gig and Platform Workers
//!
//! The Code on Social Security, 2020 extends social security to gig and
//! platform workers for the first time:
//!
//! - **Gig Worker** (Section 2(35)): Works outside traditional employer-employee
//!   relationship
//! - **Platform Worker** (Section 2(61)): Work arrangement through online
//!   platform with algorithm-based assignments
//!
//! Social security schemes to be notified by Central Government (Section 114).
//!
//! ## Fixed Term Employment
//!
//! The IR Code recognizes fixed-term employment with:
//! - Same working conditions as permanent workers
//! - Pro-rata gratuity (without 5-year requirement)
//! - Pro-rata bonus and leave
//!
//! ## References
//!
//! - [Code on Wages, 2019](https://labour.gov.in/sites/default/files/THE%20CODE%20ON%20WAGES%2C%202019.pdf)
//! - [Code on Social Security, 2020](https://labour.gov.in/sites/default/files/ss_code_2020.pdf)
//! - [Industrial Relations Code, 2020](https://labour.gov.in/sites/default/files/IR_Code_2020.pdf)
//! - [OSH Code, 2020](https://labour.gov.in/sites/default/files/OSH_Code_2020.pdf)
//! - [Ministry of Labour & Employment](https://labour.gov.in/)

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
