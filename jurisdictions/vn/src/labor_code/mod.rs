//! Vietnamese Labor Code 2019 (Bộ luật Lao động 2019) - Law No. 45/2019/QH14
//!
//! Vietnam's comprehensive labor law, effective from January 1, 2021.
//!
//! ## Key Features
//!
//! - 48 hours/week maximum (Article 105)
//! - Minimum 12 days annual leave (Article 113)
//! - Severance: 0.5 months per year of service (Article 46)
//! - Social insurance (BHXH) mandatory
//! - Trade union rights enshrined
//!
//! ## Contract Types
//!
//! - Indefinite-term contract (Hợp đồng lao động không xác định thời hạn)
//! - Fixed-term contract: max 36 months (Hợp đồng lao động xác định thời hạn)
//! - Seasonal contract: max 12 months (Hợp đồng lao động theo mùa vụ)

mod error;
mod types;
mod validator;

pub use error::{LaborCodeError, LaborCodeResult};
pub use types::{
    ContractType, EmploymentContract, LeaveType, Severance, SocialInsurance, TerminationType,
    WorkingHours,
};
pub use validator::{
    LaborCompliance, calculate_severance, calculate_social_insurance, get_labor_checklist,
    validate_contract, validate_labor_compliance, validate_minimum_wage, validate_working_hours,
};
