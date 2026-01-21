//! Indonesian Labor Law (UU Ketenagakerjaan) - UU No. 13 Tahun 2003
//!
//! As amended by Omnibus Law (UU Cipta Kerja) No. 6 Tahun 2023.
//!
//! ## Key Features
//!
//! - Maximum 40 hours/week (7 hours/day for 6 days, 8 hours/day for 5 days)
//! - PKWT (Fixed-term) and PKWTT (Permanent) contracts
//! - Complex severance formula (9-32 months depending on tenure)
//! - BPJS contributions (employment + health insurance)
//! - Minimum wage by province (UMP/UMK)
//!
//! ## Omnibus Law Changes (2023)
//!
//! - Simplified severance calculation
//! - Extended PKWT maximum to 5 years
//! - Outsourcing reforms
//! - Flexible working arrangements

mod error;
mod types;
mod validator;

pub use error::{LaborError, LaborResult};
pub use types::{
    BpjsContribution, ContractType, EmploymentContract, LeaveType, Severance, TerminationType,
    WorkingHours,
};
pub use validator::{
    LaborCompliance, calculate_bpjs_contribution, calculate_overtime_pay, calculate_severance,
    get_labor_checklist, validate_contract, validate_labor_compliance, validate_minimum_wage,
    validate_working_hours,
};
