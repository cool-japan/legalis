//! Validation functions for Indonesian Labor Law

use super::error::{LaborError, LaborResult};
use super::types::*;
use crate::common::Province;
use serde::{Deserialize, Serialize};

/// Validate working hours - Pasal 77
pub fn validate_working_hours(hours: &WorkingHours) -> LaborResult<()> {
    let regular_hours = hours.hours_per_day * hours.days_per_week;

    // Regular hours max 40/week
    if regular_hours > 40 {
        return Err(LaborError::WorkingHoursExceeded {
            hours: regular_hours,
        });
    }

    // Overtime max 18 hours/week (PP 35/2021)
    if hours.overtime_hours_per_week > 18 {
        return Err(LaborError::OvertimeExceeded {
            hours: hours.overtime_hours_per_week,
        });
    }

    Ok(())
}

/// Validate minimum wage - Pasal 88
pub fn validate_minimum_wage(monthly_wage: i64, province: &Province) -> LaborResult<()> {
    let minimum = province.minimum_wage_2024();

    if monthly_wage < minimum.amount() {
        return Err(LaborError::MinimumWageViolation {
            actual: monthly_wage,
            minimum: minimum.amount(),
        });
    }

    Ok(())
}

/// Validate employment contract - Pasal 52-59
pub fn validate_contract(contract: &EmploymentContract) -> LaborResult<()> {
    // Check language requirement - Pasal 57
    if !contract.in_indonesian {
        return Err(LaborError::ContractNotInIndonesian);
    }

    // PKWT specific validations
    if let ContractType::Pkwt {
        duration_months,
        extensions,
    } = &contract.contract_type
    {
        // PKWT must be written - Pasal 57
        if !contract.is_written {
            return Err(LaborError::PkwtNotWritten);
        }

        // PKWT cannot have probation - Pasal 58
        if contract.probation_months.is_some() {
            return Err(LaborError::ProbationInPkwt);
        }

        // Check total duration - Omnibus Law max 5 years
        let total_months = duration_months * (extensions + 1);
        if total_months > ContractType::max_pkwt_duration_months() {
            return Err(LaborError::InvalidPkwtDuration {
                months: total_months,
            });
        }
    }

    // Validate minimum wage
    let province = match contract.province.to_lowercase().as_str() {
        "dki jakarta" | "jakarta" => Province::DkiJakarta,
        "jawa barat" | "west java" => Province::JawaBarat,
        "bali" => Province::Bali,
        _ => Province::Other(contract.province.clone()),
    };

    validate_minimum_wage(contract.monthly_salary, &province)?;

    // Validate working hours
    validate_working_hours(&contract.working_hours)?;

    Ok(())
}

/// Calculate overtime pay - PP 35/2021 Pasal 31
pub fn calculate_overtime_pay(
    monthly_wage: i64,
    overtime_hours: u32,
    is_holiday: bool,
    is_rest_day: bool,
) -> i64 {
    // Hourly rate = monthly wage / 173
    let hourly_rate = monthly_wage as f64 / 173.0;

    let multiplier = if is_holiday || is_rest_day {
        // First 8 hours: 2x, next hours: 3x or 4x
        if overtime_hours <= 8 {
            2.0 * overtime_hours as f64
        } else {
            (2.0 * 8.0) + (3.0 * (overtime_hours - 8) as f64)
        }
    } else {
        // Weekday: first hour 1.5x, subsequent hours 2x
        if overtime_hours == 0 {
            0.0
        } else if overtime_hours == 1 {
            1.5
        } else {
            1.5 + (2.0 * (overtime_hours - 1) as f64)
        }
    };

    (hourly_rate * multiplier) as i64
}

/// Calculate severance package
pub fn calculate_severance(
    years_of_service: u32,
    monthly_wage: i64,
    termination_type: TerminationType,
) -> Severance {
    Severance::calculate(years_of_service, monthly_wage, termination_type)
}

/// Calculate BPJS contribution
pub fn calculate_bpjs_contribution(monthly_wage: i64, risk_level: &str) -> BpjsContribution {
    // JKK rate based on risk level
    let jkk_rate = match risk_level.to_lowercase().as_str() {
        "sangat rendah" | "very low" => 0.0024,
        "rendah" | "low" => 0.0054,
        "sedang" | "medium" => 0.0089,
        "tinggi" | "high" => 0.0127,
        "sangat tinggi" | "very high" => 0.0174,
        _ => 0.0089, // Default medium
    };

    BpjsContribution::calculate(monthly_wage, jkk_rate)
}

/// Labor compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborCompliance {
    /// Overall compliance status
    pub compliant: bool,
    /// Working hours compliant
    pub working_hours_compliant: bool,
    /// Minimum wage met
    pub minimum_wage_met: bool,
    /// Contract valid
    pub contract_valid: bool,
    /// BPJS registered
    pub bpjs_registered: bool,
    /// Leave entitlements provided
    pub leave_provided: bool,
    /// List of issues
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive labor compliance check
pub fn validate_labor_compliance(
    contract: &EmploymentContract,
    bpjs_registered: bool,
    leave_provided: bool,
) -> LaborCompliance {
    let mut compliance = LaborCompliance {
        compliant: true,
        working_hours_compliant: true,
        minimum_wage_met: true,
        contract_valid: true,
        bpjs_registered,
        leave_provided,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Validate working hours
    if let Err(e) = validate_working_hours(&contract.working_hours) {
        compliance.compliant = false;
        compliance.working_hours_compliant = false;
        compliance.issues.push(format!("{}", e));
        compliance
            .recommendations
            .push("Sesuaikan jam kerja dengan ketentuan UU Ketenagakerjaan Pasal 77".to_string());
    }

    // Validate contract
    if let Err(e) = validate_contract(contract) {
        compliance.compliant = false;
        compliance.contract_valid = false;
        compliance.issues.push(format!("{}", e));
    }

    // Check BPJS
    if !bpjs_registered {
        compliance.compliant = false;
        compliance
            .issues
            .push("Pekerja belum terdaftar BPJS".to_string());
        compliance
            .recommendations
            .push("Daftarkan pekerja ke BPJS Kesehatan dan Ketenagakerjaan".to_string());
    }

    // Check leave
    if !leave_provided {
        compliance.compliant = false;
        compliance
            .issues
            .push("Hak cuti belum diberikan sesuai ketentuan".to_string());
        compliance
            .recommendations
            .push("Berikan hak cuti sesuai UU Ketenagakerjaan Pasal 79-84".to_string());
    }

    compliance
}

/// Get labor law compliance checklist
pub fn get_labor_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Perjanjian kerja tertulis",
            "Written employment contract",
            "Pasal 52",
        ),
        (
            "Bahasa Indonesia dalam perjanjian",
            "Contract in Indonesian",
            "Pasal 57",
        ),
        (
            "Upah minimal UMP/UMK",
            "Minimum wage compliance",
            "Pasal 88",
        ),
        (
            "Jam kerja maksimal 40 jam/minggu",
            "Max 40 hours/week",
            "Pasal 77",
        ),
        (
            "Lembur maksimal 18 jam/minggu",
            "Max 18 hours overtime/week",
            "PP 35/2021",
        ),
        (
            "Upah lembur sesuai ketentuan",
            "Overtime pay compliance",
            "PP 35/2021",
        ),
        ("Istirahat mingguan 1 hari", "Weekly rest 1 day", "Pasal 79"),
        ("Cuti tahunan 12 hari", "12 days annual leave", "Pasal 79"),
        ("Cuti melahirkan 3 bulan", "3 months maternity", "Pasal 82"),
        (
            "BPJS Kesehatan terdaftar",
            "BPJS Health registered",
            "UU BPJS",
        ),
        (
            "BPJS Ketenagakerjaan terdaftar",
            "BPJS Employment registered",
            "UU BPJS",
        ),
        (
            "K3 (Keselamatan Kerja)",
            "Occupational safety",
            "Pasal 86-87",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_contract() -> EmploymentContract {
        EmploymentContract {
            contract_type: ContractType::Pkwtt,
            employee_name: "Budi Santoso".to_string(),
            employee_nik: Some("3171234567890001".to_string()),
            employer_name: "PT Example Indonesia".to_string(),
            position: "Software Engineer".to_string(),
            start_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: None,
            monthly_salary: 10_000_000,
            province: "DKI Jakarta".to_string(),
            working_hours: WorkingHours::standard_5_day(),
            in_indonesian: true,
            is_written: true,
            probation_months: Some(3),
            signed_date: Utc::now(),
        }
    }

    #[test]
    fn test_validate_working_hours_ok() {
        let hours = WorkingHours::standard_5_day();
        assert!(validate_working_hours(&hours).is_ok());
    }

    #[test]
    fn test_validate_working_hours_exceeded() {
        let hours = WorkingHours {
            hours_per_day: 10,
            days_per_week: 6,
            overtime_hours_per_week: 0,
        };
        assert!(validate_working_hours(&hours).is_err());
    }

    #[test]
    fn test_validate_minimum_wage_ok() {
        let result = validate_minimum_wage(10_000_000, &Province::DkiJakarta);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_minimum_wage_violation() {
        let result = validate_minimum_wage(3_000_000, &Province::DkiJakarta);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_contract_valid() {
        let contract = sample_contract();
        assert!(validate_contract(&contract).is_ok());
    }

    #[test]
    fn test_validate_pkwt_with_probation() {
        let mut contract = sample_contract();
        contract.contract_type = ContractType::Pkwt {
            duration_months: 12,
            extensions: 0,
        };
        contract.probation_months = Some(3);
        let result = validate_contract(&contract);
        assert!(matches!(result, Err(LaborError::ProbationInPkwt)));
    }

    #[test]
    fn test_overtime_pay_calculation() {
        let monthly_wage = 10_000_000i64;

        // Weekday overtime: 3 hours
        let weekday_ot = calculate_overtime_pay(monthly_wage, 3, false, false);
        // First hour 1.5x + 2 hours 2x = 1.5 + 4 = 5.5x hourly rate
        let expected = ((monthly_wage as f64 / 173.0) * 5.5) as i64;
        assert_eq!(weekday_ot, expected);

        // Holiday overtime: 8 hours
        let holiday_ot = calculate_overtime_pay(monthly_wage, 8, true, false);
        let expected_holiday = ((monthly_wage as f64 / 173.0) * 16.0) as i64; // 2x * 8 hours
        assert_eq!(holiday_ot, expected_holiday);
    }

    #[test]
    fn test_bpjs_calculation() {
        let contribution = calculate_bpjs_contribution(10_000_000, "medium");

        assert!(contribution.total_employer > 0);
        assert!(contribution.total_employee > 0);
        assert_eq!(
            contribution.total,
            contribution.total_employer + contribution.total_employee
        );
    }

    #[test]
    fn test_labor_compliance_check() {
        let contract = sample_contract();
        let compliance = validate_labor_compliance(&contract, true, true);
        assert!(compliance.compliant);
        assert!(compliance.issues.is_empty());
    }

    #[test]
    fn test_labor_checklist() {
        let checklist = get_labor_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(id, _, _)| id.contains("BPJS")));
    }
}
