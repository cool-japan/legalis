//! Thai Labor Protection Act (LPA) - พ.ร.บ. คุ้มครองแรงงาน พ.ศ. 2541
//!
//! Thailand's LPA (B.E. 2541 / 1998 CE) provides comprehensive worker protection.
//!
//! ## Key Features
//!
//! - 48 hours/week maximum (Section 23)
//! - Generous severance pay (120-400 days) (Section 118)
//! - Minimum 13 public holidays (Section 29)
//! - Social Security Fund contributions

mod error;
mod types;
mod validator;

pub use error::{LpaError, LpaResult};
pub use types::{
    EmploymentContract, EmploymentType, JustCause, LaborRight, ScheduleType, Severance,
    TerminationType, WorkingHours,
};
pub use validator::{
    LaborCompliance, calculate_overtime, calculate_severance, get_labor_rights_checklist,
    get_minimum_wage_2024, validate_labor_compliance, validate_minimum_wage, validate_rest_period,
    validate_worker_age, validate_working_hours,
};
