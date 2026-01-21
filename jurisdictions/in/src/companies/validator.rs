//! Companies Act 2013 Validation
//!
//! Validation logic for Indian corporate law compliance

use super::error::{CompaniesActError, CompaniesActResult};
use super::types::*;
use chrono::{Datelike, NaiveDate};

/// Compliance report for company
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    /// Overall compliance status
    pub compliant: bool,
    /// List of violations
    pub violations: Vec<CompaniesActError>,
    /// List of warnings (non-critical)
    pub warnings: Vec<String>,
    /// List of recommendations
    pub recommendations: Vec<String>,
}

impl Default for ComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

/// Validate company formation
pub fn validate_company_formation(company: &Company) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // Validate CIN format (21 characters)
    if company.cin.len() != 21 {
        report.violations.push(CompaniesActError::InvalidCinFormat);
        report.compliant = false;
    }

    // Validate company status
    if !matches!(company.status, CompanyStatus::Active) {
        report.warnings.push(format!(
            "Company status is {:?}, some operations may be restricted",
            company.status
        ));
    }

    // Validate authorized vs paid-up capital
    if company.paid_up_capital > company.authorized_capital {
        report
            .violations
            .push(CompaniesActError::AuthorizedCapitalExceeded);
        report.compliant = false;
    }

    report
}

/// Validate board composition
pub fn validate_board_composition(company: &Company) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // Check minimum directors
    let eligible_directors: Vec<_> = company
        .directors
        .iter()
        .filter(|d| d.is_eligible())
        .collect();
    let director_count = eligible_directors.len() as u32;
    let min_required = company.company_type.min_directors();

    if director_count < min_required {
        report
            .violations
            .push(CompaniesActError::InsufficientDirectors {
                required: min_required,
                actual: director_count,
            });
        report.compliant = false;
    }

    // Check resident director requirement (Section 149(3))
    if !company.has_resident_director() {
        report
            .violations
            .push(CompaniesActError::NoResidentDirector);
        report.compliant = false;
    }

    // Check independent directors for listed companies
    if company.is_listed {
        let required_independent = company.required_independent_directors();
        let actual_independent = company.independent_director_count();

        if actual_independent < required_independent {
            report
                .violations
                .push(CompaniesActError::InsufficientIndependentDirectors {
                    required: required_independent,
                    actual: actual_independent,
                });
            report.compliant = false;
        }
    }

    // Check woman director requirement
    if company.requires_woman_director() && !company.has_woman_director() {
        report.violations.push(CompaniesActError::NoWomanDirector);
        report.compliant = false;
    }

    // Check each director's eligibility
    for director in &company.directors {
        if director.disqualified {
            report
                .violations
                .push(CompaniesActError::DirectorDisqualified {
                    din: director.din.clone(),
                });
            report.compliant = false;
        }

        if !matches!(director.din_status, DinStatus::Approved) {
            report.violations.push(CompaniesActError::DinNotApproved {
                din: director.din.clone(),
            });
            report.compliant = false;
        }

        // Check directorship limit
        let is_public = matches!(
            company.company_type,
            CompanyType::PublicLimited | CompanyType::Listed
        );
        if !director.within_directorship_limit(is_public) {
            report
                .violations
                .push(CompaniesActError::DirectorshipLimitExceeded {
                    din: director.din.clone(),
                });
            report.compliant = false;
        }
    }

    // Recommendations
    if director_count < 5 && company.is_listed {
        report
            .recommendations
            .push("Consider expanding board size for better governance".to_string());
    }

    report
}

/// Validate KMP appointments
pub fn validate_kmp_appointments(
    company: &Company,
    paid_up_capital: i64,
    turnover: i64,
) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // KMP requirements for prescribed class (Section 203)
    // Public companies with paid-up capital >= 10 crore OR
    // Companies with turnover >= 100 crore
    let kmp_required = matches!(
        company.company_type,
        CompanyType::PublicLimited | CompanyType::Listed
    ) && (paid_up_capital >= 100_000_000 || turnover >= 1_000_000_000);

    if kmp_required {
        // Check for Company Secretary
        let has_cs = company
            .kmps
            .iter()
            .any(|k| matches!(k.kmp_type, KmpType::CompanySecretary));
        if !has_cs {
            report
                .violations
                .push(CompaniesActError::NoCompanySecretary);
            report.compliant = false;
        }

        // Check for CFO
        let has_cfo = company
            .kmps
            .iter()
            .any(|k| matches!(k.kmp_type, KmpType::Cfo));
        if !has_cfo {
            report.violations.push(CompaniesActError::NoCfo);
            report.compliant = false;
        }

        // Check for Managing Director or CEO or Manager
        let has_md_ceo = company.kmps.iter().any(|k| {
            matches!(
                k.kmp_type,
                KmpType::ManagingDirector | KmpType::Ceo | KmpType::Manager
            )
        });
        if !has_md_ceo {
            report.warnings.push(
                "No Managing Director, CEO, or Manager appointed. At least one is recommended."
                    .to_string(),
            );
        }
    }

    report
}

/// Validate committee requirements
pub fn validate_committees(company: &Company) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // Audit Committee required for listed and prescribed public companies
    if company.is_listed
        || (matches!(company.company_type, CompanyType::PublicLimited)
            && company.paid_up_capital >= 100_000_000)
    {
        let has_audit = company
            .committees
            .iter()
            .any(|c| matches!(c.committee_type, CommitteeType::Audit));

        if !has_audit {
            report.violations.push(CompaniesActError::NoAuditCommittee);
            report.compliant = false;
        } else {
            // Validate composition
            let audit_committee = company
                .committees
                .iter()
                .find(|c| matches!(c.committee_type, CommitteeType::Audit));

            if let Some(committee) = audit_committee
                && !committee.is_valid_composition(&company.directors)
            {
                report
                    .violations
                    .push(CompaniesActError::InvalidAuditCommitteeComposition);
                report.compliant = false;
            }
        }

        // Nomination and Remuneration Committee
        let has_nrc = company
            .committees
            .iter()
            .any(|c| matches!(c.committee_type, CommitteeType::NominationRemuneration));

        if !has_nrc {
            report
                .violations
                .push(CompaniesActError::NoNominationRemunerationCommittee);
            report.compliant = false;
        }
    }

    // Stakeholders Relationship Committee (> 1000 shareholders)
    let total_shareholders: u64 = company.shareholders.len() as u64;
    if total_shareholders > 1000 {
        let has_src = company
            .committees
            .iter()
            .any(|c| matches!(c.committee_type, CommitteeType::StakeholdersRelationship));

        if !has_src {
            report
                .violations
                .push(CompaniesActError::NoStakeholdersCommittee);
            report.compliant = false;
        }
    }

    report
}

/// Validate CSR compliance
pub fn validate_csr_compliance(
    company: &Company,
    net_worth: i64,
    turnover: i64,
    net_profit: i64,
    csr_obligation: Option<&CsrObligation>,
) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // Check if CSR is applicable
    if !CsrObligation::is_csr_mandatory(net_worth, turnover, net_profit) {
        report
            .warnings
            .push("Company does not meet CSR threshold, CSR spending is voluntary".to_string());
        return report;
    }

    // CSR Committee required
    let has_csr_committee = company
        .committees
        .iter()
        .any(|c| matches!(c.committee_type, CommitteeType::Csr));

    if !has_csr_committee {
        report.violations.push(CompaniesActError::NoCsrCommittee);
        report.compliant = false;
    }

    // Check CSR spending
    if let Some(obligation) = csr_obligation
        && obligation.amount_spent < obligation.obligation_amount
    {
        report
            .violations
            .push(CompaniesActError::CsrSpendingShortfall {
                required: obligation.obligation_amount,
                spent: obligation.amount_spent,
            });
        report.compliant = false;

        // Check if unspent amount transferred to Unspent CSR Account
        if obligation.unspent_amount > 0 {
            report.recommendations.push(format!(
                "Unspent CSR amount of Rs. {} should be transferred to Unspent CSR Account within 30 days of FY end",
                obligation.unspent_amount
            ));
        }
    }

    report
}

/// Validate resolution requirements
pub fn validate_resolution(
    _matter: SpecialResolutionMatter,
    votes_for_percentage: f64,
) -> CompaniesActResult<()> {
    let required = ResolutionType::Special.required_majority();

    if votes_for_percentage < required {
        return Err(CompaniesActError::ResolutionNotPassed {
            required,
            actual: votes_for_percentage,
        });
    }

    Ok(())
}

/// Validate ordinary resolution
pub fn validate_ordinary_resolution(votes_for_percentage: f64) -> CompaniesActResult<()> {
    let required = ResolutionType::Ordinary.required_majority();

    if votes_for_percentage < required {
        return Err(CompaniesActError::ResolutionNotPassed {
            required,
            actual: votes_for_percentage,
        });
    }

    Ok(())
}

/// Validate board meeting compliance
pub fn validate_board_meetings(
    meeting_dates: &[NaiveDate],
    financial_year_start: NaiveDate,
    financial_year_end: NaiveDate,
) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // Filter meetings in the financial year
    let fy_meetings: Vec<_> = meeting_dates
        .iter()
        .filter(|d| **d >= financial_year_start && **d <= financial_year_end)
        .collect();

    // Minimum 4 meetings per year
    if fy_meetings.len() < 4 {
        report
            .violations
            .push(CompaniesActError::InsufficientBoardMeetings {
                count: fy_meetings.len() as u32,
            });
        report.compliant = false;
    }

    // Check gap between meetings (max 120 days)
    let mut sorted_meetings = fy_meetings.clone();
    sorted_meetings.sort();

    for i in 1..sorted_meetings.len() {
        let gap = (*sorted_meetings[i] - *sorted_meetings[i - 1]).num_days();
        if gap > 120 {
            report
                .violations
                .push(CompaniesActError::BoardMeetingGapExceeded { days: gap as u32 });
            report.compliant = false;
        }
    }

    report
}

/// Check filing deadlines
pub fn check_filing_deadline(
    filing_type: AnnualFilingType,
    fy_end: NaiveDate,
    agm_date: Option<NaiveDate>,
    current_date: NaiveDate,
) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    match filing_type {
        AnnualFilingType::AnnualReturn => {
            // Due within 60 days of AGM
            if let Some(agm) = agm_date {
                let deadline = agm + chrono::Days::new(60);
                if current_date > deadline {
                    let delay = (current_date - deadline).num_days();
                    report.violations.push(CompaniesActError::FilingDelayed {
                        form: "MGT-7".to_string(),
                        days: delay as u32,
                    });
                    report.compliant = false;
                }
            }
        }
        AnnualFilingType::FinancialStatements => {
            // Due within 30 days of AGM
            if let Some(agm) = agm_date {
                let deadline = agm + chrono::Days::new(30);
                if current_date > deadline {
                    let delay = (current_date - deadline).num_days();
                    report.violations.push(CompaniesActError::FilingDelayed {
                        form: "AOC-4".to_string(),
                        days: delay as u32,
                    });
                    report.compliant = false;
                }
            }
        }
        AnnualFilingType::DirectorKyc => {
            // Due by September 30 each year
            let fy_year = if fy_end.month() <= 3 {
                fy_end.year()
            } else {
                fy_end.year() + 1
            };
            if let Some(deadline) = NaiveDate::from_ymd_opt(fy_year, 9, 30)
                && current_date > deadline
            {
                let delay = (current_date - deadline).num_days();
                report.violations.push(CompaniesActError::FilingDelayed {
                    form: "DIR-3 KYC".to_string(),
                    days: delay as u32,
                });
                report.compliant = false;
            }
        }
        _ => {}
    }

    report
}

/// Validate AGM requirements
pub fn validate_agm(
    fy_end: NaiveDate,
    agm_date: Option<NaiveDate>,
    is_first_agm: bool,
) -> ComplianceReport {
    let mut report = ComplianceReport::default();

    // AGM must be held within 6 months of FY end (9 months for first AGM)
    let max_months = if is_first_agm { 9 } else { 6 };
    let deadline = fy_end + chrono::Months::new(max_months);

    match agm_date {
        Some(date) if date <= deadline => {
            // AGM held within time
        }
        Some(date) => {
            report.warnings.push(format!(
                "AGM held on {} is beyond the statutory deadline of {}",
                date, deadline
            ));
        }
        None => {
            report.violations.push(CompaniesActError::AgmNotHeld);
            report.compliant = false;
        }
    }

    report
}

/// Validate share buyback (Section 68)
pub fn validate_buyback(
    paid_up_capital: i64,
    free_reserves: i64,
    buyback_amount: i64,
    current_fy_buyback: i64,
) -> CompaniesActResult<()> {
    // Buyback cannot exceed 25% of paid-up capital and free reserves
    let max_buyback = ((paid_up_capital + free_reserves) as f64 * 0.25) as i64;

    if buyback_amount > max_buyback {
        return Err(CompaniesActError::BuybackLimitExceeded);
    }

    // Buyback in a FY cannot exceed 25% of paid-up capital
    let fy_max = (paid_up_capital as f64 * 0.25) as i64;
    if current_fy_buyback + buyback_amount > fy_max {
        return Err(CompaniesActError::BuybackLimitExceeded);
    }

    Ok(())
}

/// Get compliance checklist for company type
pub fn get_compliance_checklist(company_type: CompanyType, is_listed: bool) -> Vec<String> {
    let mut checklist = vec![
        "Maintain statutory registers".to_string(),
        "Hold minimum 4 board meetings per year".to_string(),
        "Hold AGM within 6 months of FY end".to_string(),
        "File annual return (MGT-7) within 60 days of AGM".to_string(),
        "File financial statements (AOC-4) within 30 days of AGM".to_string(),
        "Complete Director KYC (DIR-3 KYC) by September 30".to_string(),
        "Maintain at least one resident director".to_string(),
    ];

    if is_listed || matches!(company_type, CompanyType::PublicLimited) {
        checklist.extend([
            "Maintain Audit Committee".to_string(),
            "Maintain Nomination and Remuneration Committee".to_string(),
            "Appoint Company Secretary".to_string(),
            "Appoint CFO".to_string(),
        ]);
    }

    if is_listed {
        checklist.extend([
            "Maintain 1/3 independent directors".to_string(),
            "Appoint woman director".to_string(),
            "Maintain Risk Management Committee".to_string(),
            "Comply with LODR requirements".to_string(),
        ]);
    }

    checklist
}

/// Calculate penalty for delayed filing
pub fn calculate_filing_penalty(form: &str, delay_days: u32) -> (u64, u64) {
    // Normal additional fee structure for MCA filings
    let base_fee: u64 = match form {
        "MGT-7" | "MGT-7A" => 200,
        "AOC-4" => 300,
        _ => 100,
    };

    let multiplier: u64 = if delay_days <= 30 {
        2
    } else if delay_days <= 60 {
        4
    } else if delay_days <= 90 {
        6
    } else if delay_days <= 180 {
        10
    } else {
        12
    };

    let additional_fee = base_fee * multiplier;
    let penalty = if delay_days > 270 {
        // Additional penalty for serious delay
        (delay_days as u64 - 270) * 100
    } else {
        0
    };

    (additional_fee, penalty)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_company(is_listed: bool) -> Company {
        Company {
            cin: "U12345MH2020PTC123456".to_string(),
            name: "Test Company Ltd".to_string(),
            company_type: if is_listed {
                CompanyType::Listed
            } else {
                CompanyType::PrivateLimited
            },
            incorporation_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            registered_office: "Mumbai".to_string(),
            state: "Maharashtra".to_string(),
            roc: "RoC-Mumbai".to_string(),
            status: CompanyStatus::Active,
            authorized_capital: 10_000_000,
            paid_up_capital: 5_000_000,
            fy_end_month: 3,
            is_listed,
            directors: vec![
                Director {
                    din: "12345678".to_string(),
                    name: "Director 1".to_string(),
                    category: DirectorCategory::Executive,
                    appointment_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
                    term_end: None,
                    resident_in_india: true,
                    disqualified: false,
                    din_status: DinStatus::Approved,
                    other_directorships: 2,
                },
                Director {
                    din: "87654321".to_string(),
                    name: "Director 2".to_string(),
                    category: DirectorCategory::NonExecutive,
                    appointment_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
                    term_end: None,
                    resident_in_india: false,
                    disqualified: false,
                    din_status: DinStatus::Approved,
                    other_directorships: 1,
                },
            ],
            kmps: Vec::new(),
            shareholders: Vec::new(),
            committees: Vec::new(),
        }
    }

    #[test]
    fn test_board_composition_valid() {
        let company = create_test_company(false);
        let report = validate_board_composition(&company);
        assert!(report.compliant);
    }

    #[test]
    fn test_listed_company_needs_independent_directors() {
        let mut company = create_test_company(true);
        company.directors.push(Director {
            din: "11111111".to_string(),
            name: "Director 3".to_string(),
            category: DirectorCategory::NonExecutive,
            appointment_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            term_end: None,
            resident_in_india: true,
            disqualified: false,
            din_status: DinStatus::Approved,
            other_directorships: 0,
        });
        let report = validate_board_composition(&company);
        // Should fail - no independent directors
        assert!(!report.compliant);
    }

    #[test]
    fn test_resolution_validation() {
        // Special resolution requires 75%
        assert!(validate_resolution(SpecialResolutionMatter::AlterationOfArticles, 80.0).is_ok());
        assert!(validate_resolution(SpecialResolutionMatter::AlterationOfArticles, 70.0).is_err());

        // Ordinary resolution requires 50%
        assert!(validate_ordinary_resolution(60.0).is_ok());
        assert!(validate_ordinary_resolution(40.0).is_err());
    }

    #[test]
    fn test_board_meeting_validation() {
        let fy_start = NaiveDate::from_ymd_opt(2023, 4, 1).expect("valid date");
        let fy_end = NaiveDate::from_ymd_opt(2024, 3, 31).expect("valid date");

        let meetings = vec![
            NaiveDate::from_ymd_opt(2023, 5, 15).expect("valid date"),
            NaiveDate::from_ymd_opt(2023, 8, 20).expect("valid date"),
            NaiveDate::from_ymd_opt(2023, 11, 10).expect("valid date"),
            NaiveDate::from_ymd_opt(2024, 2, 5).expect("valid date"),
        ];

        let report = validate_board_meetings(&meetings, fy_start, fy_end);
        assert!(report.compliant);
    }

    #[test]
    fn test_board_meeting_gap_exceeded() {
        let fy_start = NaiveDate::from_ymd_opt(2023, 4, 1).expect("valid date");
        let fy_end = NaiveDate::from_ymd_opt(2024, 3, 31).expect("valid date");

        let meetings = vec![
            NaiveDate::from_ymd_opt(2023, 5, 1).expect("valid date"),
            NaiveDate::from_ymd_opt(2023, 10, 15).expect("valid date"), // Gap > 120 days
            NaiveDate::from_ymd_opt(2024, 1, 10).expect("valid date"),
            NaiveDate::from_ymd_opt(2024, 3, 5).expect("valid date"),
        ];

        let report = validate_board_meetings(&meetings, fy_start, fy_end);
        assert!(!report.compliant);
    }

    #[test]
    fn test_buyback_validation() {
        let paid_up = 100_000_000;
        let reserves = 50_000_000;

        // Valid buyback (within 25% of paid-up capital in a FY)
        // FY max = 25% of 100M = 25M
        assert!(validate_buyback(paid_up, reserves, 20_000_000, 0).is_ok());

        // Invalid - exceeds 25% of paid-up capital for FY
        assert!(validate_buyback(paid_up, reserves, 30_000_000, 0).is_err());

        // Invalid - exceeds 25% of paid-up + reserves overall
        assert!(validate_buyback(paid_up, reserves, 50_000_000, 0).is_err());
    }

    #[test]
    fn test_filing_penalty_calculation() {
        let (fee_30, _) = calculate_filing_penalty("MGT-7", 30);
        let (fee_90, _) = calculate_filing_penalty("MGT-7", 90);

        assert!(fee_90 > fee_30); // More delay = higher penalty
    }

    #[test]
    fn test_compliance_checklist() {
        let private_checklist = get_compliance_checklist(CompanyType::PrivateLimited, false);
        let listed_checklist = get_compliance_checklist(CompanyType::PublicLimited, true);

        assert!(listed_checklist.len() > private_checklist.len());
    }
}
