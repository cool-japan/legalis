//! Working hours regulations (Jornada de trabajo)
//!
//! Federal Labor Law Article 61-68

use super::types::{WorkDayType, WorkSchedule};
use thiserror::Error;

/// Maximum working hours per day and week
pub mod limits {
    /// Day shift maximum (Article 61)
    pub const DAY_SHIFT_MAX_HOURS: u8 = 8;
    pub const DAY_SHIFT_MAX_WEEKLY: u8 = 48;

    /// Night shift maximum (Article 61)
    pub const NIGHT_SHIFT_MAX_HOURS: u8 = 7;
    pub const NIGHT_SHIFT_MAX_WEEKLY: u8 = 42;

    /// Mixed shift maximum (Article 61)
    pub const MIXED_SHIFT_MAX_HOURS: u8 = 7;
    pub const MIXED_SHIFT_MAX_WEEKLY: u8 = 45;
}

/// Working hours error
#[derive(Debug, Error)]
pub enum WorkingHoursError {
    #[error("Exceeds maximum daily hours for {0:?} shift")]
    ExceedsDailyLimit(WorkDayType),
    #[error("Exceeds maximum weekly hours for {0:?} shift")]
    ExceedsWeeklyLimit(WorkDayType),
}

/// Validate working hours schedule
pub fn validate_schedule(schedule: &WorkSchedule) -> Result<(), WorkingHoursError> {
    let weekly_hours = schedule.weekly_hours();

    match schedule.tipo_jornada {
        WorkDayType::Day => {
            if schedule.horas_diarias > limits::DAY_SHIFT_MAX_HOURS {
                return Err(WorkingHoursError::ExceedsDailyLimit(WorkDayType::Day));
            }
            if weekly_hours > limits::DAY_SHIFT_MAX_WEEKLY {
                return Err(WorkingHoursError::ExceedsWeeklyLimit(WorkDayType::Day));
            }
        }
        WorkDayType::Night => {
            if schedule.horas_diarias > limits::NIGHT_SHIFT_MAX_HOURS {
                return Err(WorkingHoursError::ExceedsDailyLimit(WorkDayType::Night));
            }
            if weekly_hours > limits::NIGHT_SHIFT_MAX_WEEKLY {
                return Err(WorkingHoursError::ExceedsWeeklyLimit(WorkDayType::Night));
            }
        }
        WorkDayType::Mixed => {
            if schedule.horas_diarias > limits::MIXED_SHIFT_MAX_HOURS {
                return Err(WorkingHoursError::ExceedsDailyLimit(WorkDayType::Mixed));
            }
            if weekly_hours > limits::MIXED_SHIFT_MAX_WEEKLY {
                return Err(WorkingHoursError::ExceedsWeeklyLimit(WorkDayType::Mixed));
            }
        }
    }

    Ok(())
}

/// Calculate overtime hours
pub fn calculate_overtime(_regular_hours: u8, actual_hours: u8, shift_type: WorkDayType) -> u8 {
    let max_hours = match shift_type {
        WorkDayType::Day => limits::DAY_SHIFT_MAX_HOURS,
        WorkDayType::Night => limits::NIGHT_SHIFT_MAX_HOURS,
        WorkDayType::Mixed => limits::MIXED_SHIFT_MAX_HOURS,
    };

    actual_hours.saturating_sub(max_hours)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_schedule() {
        let schedule = WorkSchedule::standard_day();
        assert!(validate_schedule(&schedule).is_ok());
    }

    #[test]
    fn test_calculate_overtime() {
        let overtime = calculate_overtime(8, 10, WorkDayType::Day);
        assert_eq!(overtime, 2);
    }
}
