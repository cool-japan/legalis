//! CLT Validation Functions

use super::error::{CltError, CltResult};
use super::types::*;
use chrono::NaiveDate;

/// Minimum wage for 2024 (in centavos)
pub const MINIMUM_WAGE_2024: i64 = 141200; // R$ 1,412.00

/// Validate working hours compliance (Art. 58)
pub fn validate_working_hours(hours: &WorkingHours) -> CltResult<()> {
    // Standard: max 8h/day, 44h/week
    if hours.daily_hours > 10 {
        return Err(CltError::WorkingHoursViolation {
            description: "Jornada diária excede limite de 10 horas".to_string(),
            actual_hours: hours.daily_hours,
            max_hours: 10,
        });
    }

    if hours.weekly_hours > 44 && !hours.time_bank {
        return Err(CltError::WorkingHoursViolation {
            description: "Jornada semanal excede 44 horas sem banco de horas".to_string(),
            actual_hours: hours.weekly_hours,
            max_hours: 44,
        });
    }

    Ok(())
}

/// Validate minimum wage compliance
pub fn validate_minimum_wage(salary_centavos: i64, minimum_wage: i64) -> CltResult<()> {
    if salary_centavos < minimum_wage {
        return Err(CltError::MinimumWageViolation {
            actual: salary_centavos,
            minimum: minimum_wage,
        });
    }
    Ok(())
}

/// Validate rest period between shifts (Art. 66)
pub fn validate_rest_period(rest_hours: u32) -> CltResult<()> {
    if rest_hours < 11 {
        return Err(CltError::RestPeriodViolation { hours: rest_hours });
    }
    Ok(())
}

/// Validate worker age (Art. 403)
pub fn validate_worker_age(age: u32, is_apprentice: bool) -> CltResult<()> {
    let minimum_age = if is_apprentice { 14 } else { 16 };

    if age < minimum_age {
        return Err(CltError::ChildLaborViolation { age });
    }

    // Night work prohibited for under 18
    // Hazardous work prohibited for under 18
    // These would be additional checks with more context

    Ok(())
}

/// Calculate overtime payment
pub fn calculate_overtime_payment(
    hourly_rate_centavos: i64,
    overtime_hours: u32,
    is_sunday_holiday: bool,
) -> i64 {
    let multiplier = if is_sunday_holiday { 200 } else { 150 }; // 100% or 50%
    (hourly_rate_centavos * multiplier / 100) * overtime_hours as i64
}

/// Validate vacation compliance (Art. 134)
pub fn validate_vacation_compliance(
    employment_start: NaiveDate,
    last_vacation: Option<NaiveDate>,
    reference_date: NaiveDate,
) -> CltResult<()> {
    let months_employed = ((reference_date - employment_start).num_days() / 30) as u32;

    // First vacation after 12 months (período aquisitivo)
    if months_employed < 12 {
        return Ok(()); // Not yet entitled
    }

    // Must grant vacation within 12 months after earning (período concessivo)
    let acquisition_date = if let Some(last) = last_vacation {
        last
    } else {
        employment_start + chrono::Duration::days(365)
    };

    let grant_deadline = acquisition_date + chrono::Duration::days(365);

    if reference_date > grant_deadline {
        let months_overdue = ((reference_date - grant_deadline).num_days() / 30) as u32;
        return Err(CltError::VacationNotGranted { months_overdue });
    }

    Ok(())
}

/// Validate termination procedure
pub fn validate_termination(
    termination_type: TerminationType,
    just_cause_ground: Option<JustCauseGround>,
    notice_given_days: u32,
    required_notice_days: u32,
) -> CltResult<()> {
    // Just cause requires valid ground
    if termination_type == TerminationType::ForCause && just_cause_ground.is_none() {
        return Err(CltError::FalseJustCause {
            claimed_ground: "Nenhum motivo especificado".to_string(),
        });
    }

    // Check notice period
    if termination_type.requires_notice() && notice_given_days < required_notice_days {
        return Err(CltError::NoticePeriodViolation {
            days_required: required_notice_days,
            days_given: notice_given_days,
        });
    }

    Ok(())
}

/// Validate CTPS registration
pub fn validate_ctps_registration(
    is_registered: bool,
    employment_type: EmploymentType,
) -> CltResult<()> {
    // All CLT employees must have CTPS registered
    if !is_registered && !matches!(employment_type, EmploymentType::Intermittent) {
        return Err(CltError::CtpsNotRegistered);
    }
    Ok(())
}

/// Calculate notice period (Art. 487)
pub fn calculate_notice_period(years_worked: u32) -> u32 {
    // 30 days base + 3 days per year worked, max 90 days
    30 + (years_worked * 3).min(60)
}

/// Calculate 13th salary (décimo terceiro)
pub fn calculate_13th_salary(monthly_salary_centavos: i64, months_worked_in_year: u32) -> i64 {
    (monthly_salary_centavos * months_worked_in_year as i64) / 12
}

/// Calculate vacation payment (férias + 1/3)
pub fn calculate_vacation_payment(monthly_salary_centavos: i64, vacation_days: u32) -> i64 {
    let base = (monthly_salary_centavos * vacation_days as i64) / 30;
    base + (base / 3) // +1/3 constitutional bonus
}

/// Calculate FGTS monthly deposit
pub fn calculate_fgts_deposit(monthly_salary_centavos: i64) -> i64 {
    (monthly_salary_centavos * 8) / 100 // 8%
}

/// Validate insalubrity/danger premium
pub fn validate_hazard_premium(
    is_unhealthy: bool,
    unhealthy_level: Option<&str>,
    is_dangerous: bool,
    receives_premium: bool,
) -> CltResult<()> {
    if is_unhealthy && !receives_premium {
        return Err(CltError::UnhealthyConditionsNoPremium {
            level: unhealthy_level.unwrap_or("não especificado").to_string(),
        });
    }

    if is_dangerous && !receives_premium {
        return Err(CltError::DangerousConditionsNoPremium);
    }

    Ok(())
}

/// Calculate unhealthy premium (Art. 192)
pub fn calculate_unhealthy_premium(minimum_wage: i64, level: &str) -> i64 {
    let percent = match level.to_lowercase().as_str() {
        "mínimo" | "minimo" | "minimum" => 10,
        "médio" | "medio" | "medium" => 20,
        "máximo" | "maximo" | "maximum" => 40,
        _ => 10, // Default to minimum
    };
    (minimum_wage * percent) / 100
}

/// Calculate dangerous premium (Art. 193)
pub fn calculate_dangerous_premium(base_salary_centavos: i64) -> i64 {
    (base_salary_centavos * 30) / 100 // 30%
}

/// Comprehensive labor compliance check
pub fn validate_labor_compliance(
    contract: &EmploymentContract,
    fgts_deposited: bool,
    vacation_up_to_date: bool,
    overtime_paid: bool,
    reference_date: NaiveDate,
) -> LaborCompliance {
    let mut compliance = LaborCompliance {
        compliant: true,
        ctps_registered: contract.ctps_registrada,
        hours_compliant: true,
        wage_compliant: true,
        fgts_compliant: fgts_deposited,
        vacation_compliant: vacation_up_to_date,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check CTPS
    if !contract.ctps_registrada {
        compliance.compliant = false;
        compliance.issues.push("CTPS não registrada".to_string());
        compliance
            .recommendations
            .push("Regularizar registro na CTPS".to_string());
    }

    // Check working hours
    if let Err(e) = validate_working_hours(&contract.jornada) {
        compliance.compliant = false;
        compliance.hours_compliant = false;
        compliance.issues.push(e.to_string());
    }

    // Check minimum wage
    if let Err(e) = validate_minimum_wage(contract.salario_centavos, MINIMUM_WAGE_2024) {
        compliance.compliant = false;
        compliance.wage_compliant = false;
        compliance.issues.push(e.to_string());
    }

    // Check FGTS
    if !fgts_deposited && contract.employment_type.has_fgts() {
        compliance.compliant = false;
        compliance.fgts_compliant = false;
        compliance.issues.push("FGTS não depositado".to_string());
        compliance
            .recommendations
            .push("Regularizar depósitos do FGTS".to_string());
    }

    // Check vacation
    if !vacation_up_to_date {
        let _ = reference_date; // Used for completeness
        compliance.compliant = false;
        compliance.vacation_compliant = false;
        compliance.issues.push("Férias vencidas".to_string());
        compliance
            .recommendations
            .push("Conceder férias em até 12 meses após período aquisitivo".to_string());
    }

    // Check overtime
    if !overtime_paid && contract.jornada.weekly_overtime() > 0 {
        compliance.compliant = false;
        compliance.issues.push("Horas extras não pagas".to_string());
        compliance
            .recommendations
            .push("Pagar horas extras com adicional de 50% (100% domingos/feriados)".to_string());
    }

    compliance
}

/// Get worker rights checklist
pub fn get_worker_rights_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Registro CTPS", "Art. 29 CLT"),
        ("Salário mínimo", "CF Art. 7, IV"),
        ("Jornada 44h/semana", "Art. 58 CLT"),
        ("13º salário", "CF Art. 7, VIII"),
        ("Férias + 1/3", "Art. 129 CLT"),
        ("FGTS 8%", "Lei 8.036/90"),
        ("Hora extra +50%", "Art. 59 CLT"),
        ("Intervalo 11h", "Art. 66 CLT"),
        ("DSR", "Art. 67 CLT"),
        ("Vale-transporte", "Lei 7.418/85"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BrazilianState;

    #[test]
    fn test_validate_working_hours_ok() {
        let hours = WorkingHours::standard();
        assert!(validate_working_hours(&hours).is_ok());
    }

    #[test]
    fn test_validate_working_hours_exceeded() {
        let hours = WorkingHours {
            daily_hours: 12,
            weekly_hours: 60,
            time_bank: false,
            schedule_type: ScheduleType::Standard,
        };
        assert!(validate_working_hours(&hours).is_err());
    }

    #[test]
    fn test_validate_minimum_wage() {
        assert!(validate_minimum_wage(MINIMUM_WAGE_2024, MINIMUM_WAGE_2024).is_ok());
        assert!(validate_minimum_wage(100000, MINIMUM_WAGE_2024).is_err());
    }

    #[test]
    fn test_validate_rest_period() {
        assert!(validate_rest_period(11).is_ok());
        assert!(validate_rest_period(8).is_err());
    }

    #[test]
    fn test_validate_worker_age() {
        assert!(validate_worker_age(18, false).is_ok());
        assert!(validate_worker_age(14, true).is_ok());
        assert!(validate_worker_age(13, false).is_err());
    }

    #[test]
    fn test_calculate_overtime() {
        let hourly = 2000; // R$ 20/hour
        let overtime = calculate_overtime_payment(hourly, 10, false);
        assert_eq!(overtime, 30000); // 10 * 20 * 1.5 = R$ 300

        let overtime_sunday = calculate_overtime_payment(hourly, 10, true);
        assert_eq!(overtime_sunday, 40000); // 10 * 20 * 2.0 = R$ 400
    }

    #[test]
    fn test_calculate_notice_period() {
        assert_eq!(calculate_notice_period(0), 30);
        assert_eq!(calculate_notice_period(5), 45);
        assert_eq!(calculate_notice_period(20), 90);
        assert_eq!(calculate_notice_period(30), 90); // Max 90
    }

    #[test]
    fn test_calculate_13th_salary() {
        let salary = 500000; // R$ 5,000
        let thirteenth = calculate_13th_salary(salary, 6);
        assert_eq!(thirteenth, 250000); // 6/12 = R$ 2,500
    }

    #[test]
    fn test_calculate_vacation_payment() {
        let salary = 300000; // R$ 3,000
        let vacation = calculate_vacation_payment(salary, 30);
        assert_eq!(vacation, 400000); // 3,000 + 1,000 (1/3) = R$ 4,000
    }

    #[test]
    fn test_calculate_fgts() {
        let salary = 500000; // R$ 5,000
        let fgts = calculate_fgts_deposit(salary);
        assert_eq!(fgts, 40000); // 8% = R$ 400
    }

    #[test]
    fn test_calculate_unhealthy_premium() {
        let min_wage = MINIMUM_WAGE_2024;
        assert_eq!(
            calculate_unhealthy_premium(min_wage, "mínimo"),
            min_wage / 10
        );
        assert_eq!(
            calculate_unhealthy_premium(min_wage, "máximo"),
            min_wage * 40 / 100
        );
    }

    #[test]
    fn test_calculate_dangerous_premium() {
        let salary = 500000;
        let premium = calculate_dangerous_premium(salary);
        assert_eq!(premium, 150000); // 30%
    }

    #[test]
    fn test_labor_compliance_check() {
        let contract = EmploymentContract {
            employment_type: EmploymentType::Standard,
            data_inicio: NaiveDate::from_ymd_opt(2023, 1, 1).expect("valid date"),
            data_fim: None,
            salario_centavos: 300000,
            funcao: "Auxiliar".to_string(),
            estado: BrazilianState::SP,
            jornada: WorkingHours::standard(),
            ctps_registrada: true,
        };

        let reference = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
        let compliance = validate_labor_compliance(&contract, true, true, true, reference);
        assert!(compliance.compliant);
    }

    #[test]
    fn test_worker_rights_checklist() {
        let checklist = get_worker_rights_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(name, _)| name.contains("FGTS")));
    }
}
