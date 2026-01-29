//! Federal Labor Law (Ley Federal del Trabajo)
//!
//! Mexican labor regulations including:
//! - Working hours (Jornada de trabajo)
//! - Christmas bonus (Aguinaldo)
//! - Vacation rights (Vacaciones)
//! - Profit sharing (PTU)

pub mod aguinaldo;
pub mod types;
pub mod vacation;
pub mod validator;
pub mod working_hours;

// Re-export main types
pub use aguinaldo::{MINIMUM_DAYS, calculate_aguinaldo, calculate_proportional};
pub use types::{EmploymentContract, EmploymentType, LaborRight, WorkDayType, WorkSchedule};
pub use vacation::{
    VACATION_PREMIUM_PERCENT, calculate_total_vacation_compensation, calculate_vacation_premium,
    get_vacation_days,
};
pub use validator::{LaborValidationError, validate_employment_contract};
pub use working_hours::{WorkingHoursError, calculate_overtime, limits, validate_schedule};
