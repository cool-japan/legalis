//! Code of Civil Procedure 1908 Validation
//!
//! Validation logic for civil procedure compliance

use super::error::{CpcComplianceReport, CpcError, CpcResult};
use super::types::*;
use chrono::NaiveDate;

/// Validate civil suit compliance
pub fn validate_suit_compliance(suit: &CivilSuit) -> CpcComplianceReport {
    let mut report = CpcComplianceReport::default();

    // Check limitation period
    if !suit.within_limitation {
        report.compliant = false;
        report.status = "Non-compliant".to_string();
        let limitation = LimitationPeriod::for_suit_type(suit.suit_type);
        report.errors.push(CpcError::BarredByLimitation {
            filing_date: suit.filing_date.to_string(),
            limitation_date: format!("{} years from cause of action", limitation.years),
        });
    }

    // Check court fees
    let required_fees = CourtFees::calculate_money_suit(suit.suit_value);
    if suit.court_fees_paid < required_fees.total_fees {
        report.compliant = false;
        report.errors.push(CpcError::InsufficientCourtFees {
            paid: suit.court_fees_paid,
            required: required_fees.total_fees,
        });
    }

    // Check jurisdiction
    if suit.jurisdiction_basis.is_empty() {
        report
            .warnings
            .push("No jurisdiction basis specified - may face challenge".to_string());
    }

    // Check pecuniary jurisdiction
    if let Err(e) = validate_pecuniary_jurisdiction(suit.court, suit.suit_value) {
        report.compliant = false;
        report.errors.push(e);
    }

    // Status-specific checks
    match suit.status {
        SuitStatus::Rejected => {
            report
                .warnings
                .push("Plaint rejected under Order 7 Rule 11".to_string());
        }
        SuitStatus::WrittenStatementFiled => {
            report
                .recommendations
                .push("Proceed to frame issues under Order 14".to_string());
        }
        _ => {}
    }

    report
}

/// Validate territorial jurisdiction (Section 15-20)
pub fn validate_territorial_jurisdiction(basis: &[JurisdictionBasis]) -> CpcResult<()> {
    if basis.is_empty() {
        return Err(CpcError::NoTerritorialJurisdiction {
            reason: "No jurisdiction basis established".to_string(),
        });
    }

    // At least one valid basis must exist
    Ok(())
}

/// Validate pecuniary jurisdiction
pub fn validate_pecuniary_jurisdiction(court: CourtType, suit_value: f64) -> CpcResult<()> {
    let jurisdiction = get_pecuniary_jurisdiction(court);

    if let Some(max) = jurisdiction.max_value
        && suit_value > max
    {
        return Err(CpcError::NoPecuniaryJurisdiction {
            suit_value,
            limit: max,
        });
    }

    if let Some(min) = jurisdiction.min_value
        && suit_value < min
    {
        return Err(CpcError::NoPecuniaryJurisdiction {
            suit_value,
            limit: min,
        });
    }

    Ok(())
}

/// Get pecuniary jurisdiction limits for court type
pub fn get_pecuniary_jurisdiction(court: CourtType) -> PecuniaryJurisdiction {
    match court {
        CourtType::SupremeCourt => PecuniaryJurisdiction {
            court_type: court,
            min_value: None,
            max_value: None,
        },
        CourtType::HighCourt => PecuniaryJurisdiction {
            court_type: court,
            min_value: Some(2000000.0), // Rs. 20 lakhs (varies by state)
            max_value: None,
        },
        CourtType::DistrictCourt => PecuniaryJurisdiction {
            court_type: court,
            min_value: Some(50000.0), // Rs. 50,000 (varies by state)
            max_value: None,
        },
        CourtType::SubJudge => PecuniaryJurisdiction {
            court_type: court,
            min_value: Some(10000.0),
            max_value: Some(500000.0), // Rs. 5 lakhs (varies by state)
        },
        CourtType::Munsif => PecuniaryJurisdiction {
            court_type: court,
            min_value: None,
            max_value: Some(50000.0), // Rs. 50,000 (varies by state)
        },
        CourtType::SmallCauses => PecuniaryJurisdiction {
            court_type: court,
            min_value: None,
            max_value: Some(20000.0), // Rs. 20,000 (varies by state)
        },
    }
}

/// Validate plaint under Order 7
pub fn validate_plaint(suit: &CivilSuit) -> CpcResult<()> {
    // Order 7 Rule 11 grounds for rejection
    if !suit.within_limitation {
        return Err(CpcError::PlaintRejected {
            ground: "Suit barred by limitation (Order 7 Rule 11(d))".to_string(),
        });
    }

    if suit.suit_value <= 0.0 {
        return Err(CpcError::PlaintRejected {
            ground: "Invalid suit valuation".to_string(),
        });
    }

    Ok(())
}

/// Validate appeal compliance
pub fn validate_appeal_compliance(appeal: &Appeal) -> CpcComplianceReport {
    let mut report = CpcComplianceReport::default();

    // Check limitation period
    if !appeal.within_limitation {
        report.compliant = false;
        report.status = "Appeal barred by limitation".to_string();
        let limitation_days = get_appeal_limitation(appeal.appeal_type);
        report.errors.push(CpcError::AppealBarredByLimitation {
            period: limitation_days,
        });
    }

    // Check court fees
    if !appeal.court_fee_paid {
        report.compliant = false;
        report.errors.push(CpcError::InsufficientCourtFees {
            paid: 0.0,
            required: 1.0, // Placeholder - actual calculation needed
        });
    }

    // Check security deposit (Order 41 Rule 1)
    if matches!(appeal.appeal_type, AppealType::FirstAppeal) && !appeal.security_deposited {
        report
            .warnings
            .push("Security not deposited - execution not stayed (Order 41 Rule 1)".to_string());
    }

    // Check second appeal grounds (Section 100)
    if matches!(appeal.appeal_type, AppealType::SecondAppeal) {
        report
            .recommendations
            .push("Ensure substantial question of law is framed for Second Appeal".to_string());
    }

    report
}

/// Get limitation period for appeal type (in days)
pub fn get_appeal_limitation(appeal_type: AppealType) -> u32 {
    match appeal_type {
        AppealType::FirstAppeal => 90,     // 90 days (Article 116)
        AppealType::SecondAppeal => 90,    // 90 days (Article 116)
        AppealType::AppealFromOrder => 30, // 30 days (Article 117)
        AppealType::Revision => 90,        // 90 days (varies by High Court Rules)
        AppealType::Review => 30,          // 30 days (Order 47 Rule 1)
    }
}

/// Check if order is appealable under Section 104
pub fn is_order_appealable(order_type: OrderType) -> bool {
    matches!(
        order_type,
        OrderType::TemporaryInjunction
            | OrderType::AttachmentBeforeJudgment
            | OrderType::AppointmentOfReceiver
    )
}

/// Validate execution proceedings
pub fn validate_execution_compliance(execution: &ExecutionProceeding) -> CpcComplianceReport {
    let mut report = CpcComplianceReport::default();

    // Check limitation (12 years from decree)
    if !execution.within_limitation {
        report.compliant = false;
        report.status = "Execution barred".to_string();
        report.errors.push(CpcError::ExecutionBarred);
    }

    // Check execution modes
    for mode in &execution.execution_mode {
        match mode {
            ExecutionMode::ArrestAndDetention => {
                report.warnings.push(
                    "Arrest in execution requires proof of subsisting debt and means to pay (Order 21 Rule 37)"
                        .to_string(),
                );
            }
            ExecutionMode::SalaryAttachment => {
                report.warnings.push(
                    "Salary attachment limited to 2/3 of gross salary (Order 21 Rule 48)"
                        .to_string(),
                );
            }
            _ => {}
        }
    }

    report
}

/// Calculate days between two dates
pub fn calculate_days_between(from: NaiveDate, to: NaiveDate) -> i64 {
    (to - from).num_days()
}

/// Check if appeal/execution is within limitation
pub fn is_within_limitation(
    decree_date: NaiveDate,
    filing_date: NaiveDate,
    limitation_days: u32,
) -> bool {
    let days_elapsed = calculate_days_between(decree_date, filing_date);
    days_elapsed <= limitation_days as i64
}

/// Get court fees for appeal
pub fn calculate_appeal_court_fees(decree_amount: f64) -> f64 {
    // Simplified calculation - typically percentage of decree amount
    if decree_amount <= 10000.0 {
        decree_amount * 0.05
    } else if decree_amount <= 100000.0 {
        500.0 + (decree_amount - 10000.0) * 0.04
    } else {
        4100.0 + (decree_amount - 100000.0) * 0.03
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pecuniary_jurisdiction() {
        // District court can hear suits above Rs. 50,000
        assert!(validate_pecuniary_jurisdiction(CourtType::DistrictCourt, 100000.0).is_ok());

        // Munsif court cannot hear suits above Rs. 50,000
        assert!(validate_pecuniary_jurisdiction(CourtType::Munsif, 60000.0).is_err());
    }

    #[test]
    fn test_appeal_limitation() {
        assert_eq!(get_appeal_limitation(AppealType::FirstAppeal), 90);
        assert_eq!(get_appeal_limitation(AppealType::AppealFromOrder), 30);
    }

    #[test]
    fn test_court_fees_calculation() {
        let fees = CourtFees::calculate_money_suit(10000.0);
        assert!(fees.total_fees > 0.0);
    }

    #[test]
    fn test_limitation_check() {
        let decree_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let filing_date = NaiveDate::from_ymd_opt(2024, 3, 1).expect("valid date");
        assert!(is_within_limitation(decree_date, filing_date, 90));

        let late_filing = NaiveDate::from_ymd_opt(2024, 5, 1).expect("valid date");
        assert!(!is_within_limitation(decree_date, late_filing, 90));
    }
}
