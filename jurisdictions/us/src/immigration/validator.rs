//! Immigration Law Validators
//!
//! Validation functions for immigration law compliance

#![allow(missing_docs)]

use super::types::*;
use chrono::Utc;

pub type Result<T> = std::result::Result<T, String>;

/// Validate naturalization eligibility (5-year rule)
pub fn validate_naturalization_5year(app: &NaturalizationApplication) -> Result<()> {
    let today = Utc::now().naive_utc().date();
    let residence_days = (today - app.continuous_residence_start).num_days();

    if residence_days < 365 * 5 {
        return Err("Must have 5 years continuous residence".to_string());
    }

    if app.physical_presence_days < 365 * 5 / 2 {
        return Err("Must have 30 months physical presence".to_string());
    }

    if !app.good_moral_character {
        return Err("Must demonstrate good moral character".to_string());
    }

    Ok(())
}

/// Validate H-1B visa requirements
pub fn validate_h1b_requirements(
    has_bachelors: bool,
    specialty_occupation: bool,
    labor_condition_approved: bool,
) -> Result<()> {
    if !has_bachelors {
        return Err("H-1B requires bachelor's degree or equivalent".to_string());
    }

    if !specialty_occupation {
        return Err("Position must be specialty occupation".to_string());
    }

    if !labor_condition_approved {
        return Err("Labor Condition Application must be approved".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_naturalization_validation() {
        let app = NaturalizationApplication {
            applicant_name: "Test".to_string(),
            green_card_date: NaiveDate::from_ymd_opt(2019, 1, 1).expect("valid"),
            continuous_residence_start: NaiveDate::from_ymd_opt(2019, 1, 1).expect("valid"),
            physical_presence_days: 915, // 30 months
            good_moral_character: true,
            english_proficiency: true,
            civics_knowledge: true,
        };

        assert!(validate_naturalization_5year(&app).is_ok());
    }
}
