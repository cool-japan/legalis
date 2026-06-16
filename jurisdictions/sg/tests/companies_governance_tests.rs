//! Integration tests for Companies Act corporate governance: AGM deadlines
//! (s. 175), annual return deadlines (s. 197), meeting notice, resolutions and
//! board quorum.

use chrono::{DateTime, Duration, TimeZone, Utc};
use legalis_sg::companies::governance::{
    AGM_DEADLINE_MONTHS_LISTED, AGM_DEADLINE_MONTHS_NON_LISTED,
    ANNUAL_RETURN_DEADLINE_MONTHS_NON_LISTED,
};
use legalis_sg::companies::*;

fn ymd(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
        .single()
        .expect("valid date")
}

#[test]
fn test_legacy_first_agm_deadline_18_months() {
    let inc = Utc::now();
    let deadline = calculate_first_agm_deadline(inc);
    assert_eq!((deadline - inc).num_days(), 548); // ~18 months
}

#[test]
fn test_legacy_subsequent_agm_deadline_15_months() {
    let last = Utc::now();
    let deadline = calculate_subsequent_agm_deadline(last);
    assert_eq!((deadline - last).num_days(), 456); // ~15 months
}

#[test]
fn test_modern_agm_deadline_from_fye() {
    let fye = ymd(2024, 12, 31);

    let non_listed = calculate_agm_deadline_from_fye(fye, false).expect("deadline");
    assert_eq!(non_listed.date_naive(), ymd(2025, 6, 30).date_naive()); // 6 months
    assert_eq!(agm_deadline_months(false), AGM_DEADLINE_MONTHS_NON_LISTED);

    let listed = calculate_agm_deadline_from_fye(fye, true).expect("deadline");
    assert_eq!(listed.date_naive(), ymd(2025, 4, 30).date_naive()); // 4 months
    assert_eq!(agm_deadline_months(true), AGM_DEADLINE_MONTHS_LISTED);
}

#[test]
fn test_modern_annual_return_deadline_from_fye() {
    let fye = ymd(2024, 6, 30);
    let deadline = calculate_annual_return_deadline_from_fye(fye, false).expect("deadline");
    // 30 Jun + 7 months = 30 Jan next year (calendar-accurate month arithmetic).
    assert_eq!(deadline.date_naive(), ymd(2025, 1, 30).date_naive());
    assert_eq!(ANNUAL_RETURN_DEADLINE_MONTHS_NON_LISTED, 7);
}

#[test]
fn test_agm_overdue_from_fye() {
    let fye = ymd(2024, 12, 31);
    assert!(!is_agm_overdue_from_fye(fye, false, ymd(2025, 6, 1)));
    assert!(is_agm_overdue_from_fye(fye, false, ymd(2025, 7, 1)));
}

#[test]
fn test_agm_validator_overdue() {
    let company = Company::new(
        "202401234A",
        "Test Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("1 Raffles Place", "048616"),
    );
    assert!(validate_agm_requirement(&company, Utc::now() - Duration::days(300)).is_ok());
    assert!(validate_agm_requirement(&company, Utc::now() - Duration::days(500)).is_err());
}

#[test]
fn test_annual_return_deadline_validator() {
    let company = Company::new(
        "202401234A",
        "Test Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("1 Raffles Place", "048616"),
    );
    let deadline = validate_annual_return_deadline(&company).expect("deadline");
    // The most recent FYE is at most a year ago, so the deadline is recent/future.
    assert!(deadline > Utc::now() - Duration::days(365));
}

#[test]
fn test_meeting_notice_requirement() {
    let notice = Utc::now();
    assert!(is_sufficient_notice(
        notice,
        notice + Duration::days(14),
        NoticeRequirement::AgmNotice
    ));
    assert!(!is_sufficient_notice(
        notice,
        notice + Duration::days(10),
        NoticeRequirement::AgmNotice
    ));
}

#[test]
fn test_resolution_majorities() {
    assert_eq!(ResolutionType::Ordinary.required_majority(), 50.0);
    assert_eq!(ResolutionType::Special.required_majority(), 75.0);

    let result = VotingResult::new(80, 20, 0);
    assert!(result.passed_with_majority(50.0)); // ordinary
    assert!(result.passed_with_majority(75.0)); // special
    assert!(!result.passed_with_majority(90.0));
}

#[test]
fn test_board_meeting_quorum() {
    let meeting = BoardMeeting {
        meeting_date: Utc::now(),
        directors_present: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        quorum_met: true,
        resolutions: vec![],
        minutes: None,
    };
    assert!(meeting.check_quorum(5)); // 3 of 5 → quorum
    assert!(!meeting.check_quorum(6)); // 3 of 6 → no quorum
}
