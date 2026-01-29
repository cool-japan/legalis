//! Criminal Procedure Code Validation

use super::error::{CrpcComplianceReport, CrpcError, CrpcResult};
use super::types::*;

pub fn validate_police_remand(remand: &PoliceRemand) -> CrpcResult<()> {
    if !remand.within_24_hours {
        return Err(CrpcError::Not24HoursProduction);
    }

    if !remand.is_valid_period() {
        return Err(CrpcError::ExcessiveRemand);
    }

    Ok(())
}

pub fn validate_chargesheet_timing(
    chargesheet: &Chargesheet,
    max_punishment_years: u32,
) -> CrpcResult<()> {
    if !chargesheet.check_statutory_period(max_punishment_years) {
        let days = (chargesheet.filing_date - chargesheet.fir_date).num_days() as u32;
        return Err(CrpcError::ChargesheetDelayed { days });
    }

    Ok(())
}

pub fn validate_bail_application(application: &BailApplication) -> CrpcComplianceReport {
    let mut report = CrpcComplianceReport {
        compliant: true,
        status: "Compliant".to_string(),
        ..Default::default()
    };

    if application.is_bailable {
        report
            .recommendations
            .push("Bailable offence - bail as of right under Section 436".to_string());
    } else {
        report.warnings.push(
            "Non-bailable offence - bail at court's discretion under Section 437".to_string(),
        );
    }

    report
}

pub fn check_default_bail_eligibility(
    fir_date: chrono::NaiveDate,
    max_punishment_years: u32,
    chargesheet_filed: bool,
) -> bool {
    let days_elapsed = (chrono::Utc::now().date_naive() - fir_date).num_days();
    let limit = if max_punishment_years >= 10 { 90 } else { 60 };

    days_elapsed > limit && !chargesheet_filed
}

pub fn validate_appeal_limitation(appeal: &CriminalAppeal) -> CrpcResult<()> {
    if !appeal.within_limitation {
        let days = (appeal.filing_date - appeal.judgment_date).num_days();
        return Err(CrpcError::AppealBarred { days });
    }

    Ok(())
}

pub fn get_appeal_limitation_days(appeal_type: AppealType) -> u32 {
    match appeal_type {
        AppealType::ToSessionsCourt => 30,
        AppealType::ToHighCourt => 60,
        AppealType::Revision => 90,
        AppealType::Slp => 90,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_chargesheet_timing() {
        let chargesheet = Chargesheet {
            fir_number: "FIR-001/2024".to_string(),
            fir_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid"),
            accused: vec!["John Doe".to_string()],
            sections: vec![302],
            filing_date: NaiveDate::from_ymd_opt(2024, 3, 1).expect("valid"),
            within_time_limit: true,
            io_name: "Inspector A".to_string(),
            police_station: "PS-1".to_string(),
        };

        assert!(chargesheet.check_statutory_period(15));
    }
}
