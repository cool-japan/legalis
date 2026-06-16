//! Integration tests for the PDPA data breach notification regime (Part 6A,
//! ss. 26B-26D). These tests focus on:
//!
//! * notifiable-breach determination — the significant-harm limb (s. 26B(1)(a))
//!   and the significant-scale limb of 500 individuals (s. 26B(1)(b)/(3)(a));
//! * the internal-only carve-out (s. 26B(4));
//! * the **3-calendar-day** PDPC notification deadline (s. 26D(1)), measured in
//!   calendar days from the assessment date; and
//! * notification to affected individuals and the s. 26D(5) exceptions.

use chrono::{TimeZone, Utc};
use legalis_sg::pdpa::*;

// ---------------------------------------------------------------------------
// Notifiability (s. 26B)
// ---------------------------------------------------------------------------

#[test]
fn significant_scale_threshold_is_exactly_500() {
    let below = DataBreachBuilder::new("b-499", BreachType::AccidentalExposure, "x")
        .affected_individuals(499)
        .affected_category(PersonalDataCategory::Email)
        .build();
    assert!(
        !below.is_notifiable(),
        "499 individuals is below the threshold"
    );

    let at = DataBreachBuilder::new("b-500", BreachType::AccidentalExposure, "x")
        .affected_individuals(500)
        .affected_category(PersonalDataCategory::Email)
        .build();
    assert!(
        at.is_notifiable(),
        "500 individuals meets the significant-scale limb"
    );
    assert_eq!(SIGNIFICANT_SCALE_THRESHOLD, 500);
}

#[test]
fn significant_harm_requires_name_plus_prescribed_or_credentials() {
    // Name + financial -> significant harm (reg. 3(1)(a)).
    let name_financial = DataBreachBuilder::new("b-nf", BreachType::UnauthorizedAccess, "x")
        .affected_individuals(5)
        .affected_category(PersonalDataCategory::Name)
        .affected_category(PersonalDataCategory::Financial)
        .build();
    assert!(name_financial.meets_significant_harm());
    assert!(name_financial.is_notifiable());

    // Account credentials alone -> significant harm (reg. 3(1)(b)).
    let credentials = DataBreachBuilder::new("b-cred", BreachType::Theft, "x")
        .affected_individuals(1)
        .affected_category(PersonalDataCategory::AccountCredentials)
        .build();
    assert!(credentials.meets_significant_harm());

    // Name + email alone -> NOT significant harm, and below scale -> not notifiable.
    let benign = DataBreachBuilder::new("b-benign", BreachType::AccidentalExposure, "x")
        .affected_individuals(5)
        .affected_category(PersonalDataCategory::Name)
        .affected_category(PersonalDataCategory::Email)
        .build();
    assert!(!benign.meets_significant_harm());
    assert!(!benign.is_notifiable());
}

#[test]
fn internal_only_breach_is_not_notifiable_even_at_scale() {
    // s. 26B(4): a breach within an organisation only is excluded, regardless of
    // scale or sensitivity.
    let internal = DataBreachBuilder::new("b-internal", BreachType::UnauthorizedAccess, "x")
        .internal_only()
        .affected_individuals(5000)
        .affected_category(PersonalDataCategory::Name)
        .affected_category(PersonalDataCategory::Financial)
        .build();
    assert!(!internal.is_notifiable());
    // And validation passes because there is no notifiable breach to report.
    assert!(validate_breach_notification(&internal).is_ok());
}

#[test]
fn individual_notification_only_for_significant_harm_limb() {
    // Pure significant-scale breach (no harm category): no duty to notify
    // individuals (s. 26D(2)).
    let scale_only = DataBreachBuilder::new("b-scale-only", BreachType::AccidentalExposure, "x")
        .affected_individuals(600)
        .affected_category(PersonalDataCategory::Email)
        .build();
    let a = scale_only.assess_notifiability();
    assert!(a.is_notifiable());
    assert!(!a.requires_individual_notification());
}

// ---------------------------------------------------------------------------
// 3-calendar-day PDPC notification deadline (s. 26D(1))
// ---------------------------------------------------------------------------

/// Helper: builds a notifiable significant-scale breach assessed at `assessed`
/// and notified to the PDPC at `notified`.
fn breach_assessed_and_notified(
    assessed: chrono::DateTime<Utc>,
    notified: chrono::DateTime<Utc>,
) -> DataBreachNotification {
    let mut breach = DataBreachBuilder::new("b-timing", BreachType::Ransomware, "x")
        .affected_individuals(1000)
        .affected_category(PersonalDataCategory::Email)
        .build();
    breach.record_assessment(assessed);
    breach.notify_pdpc(notified);
    breach
}

#[test]
fn pdpc_notification_same_calendar_day_is_zero_days_and_timely() {
    let assessed = Utc
        .with_ymd_and_hms(2026, 1, 10, 9, 0, 0)
        .single()
        .expect("ts");
    let notified = Utc
        .with_ymd_and_hms(2026, 1, 10, 18, 30, 0)
        .single()
        .expect("ts");
    let breach = breach_assessed_and_notified(assessed, notified);
    assert_eq!(breach.days_to_pdpc_notification(), Some(0));
    assert!(breach.is_pdpc_notification_timely());
    assert!(validate_breach_notification(&breach).is_ok());
}

#[test]
fn pdpc_notification_on_day_three_is_timely() {
    // Assessed on the 10th, notified on the 13th = 3 calendar days -> within s. 26D(1).
    let assessed = Utc
        .with_ymd_and_hms(2026, 1, 10, 23, 0, 0)
        .single()
        .expect("ts");
    let notified = Utc
        .with_ymd_and_hms(2026, 1, 13, 1, 0, 0)
        .single()
        .expect("ts");
    let breach = breach_assessed_and_notified(assessed, notified);
    assert_eq!(breach.days_to_pdpc_notification(), Some(3));
    assert!(breach.is_pdpc_notification_timely());
    assert!(validate_breach_notification(&breach).is_ok());
}

#[test]
fn pdpc_notification_on_day_four_is_late() {
    // Assessed on the 10th, notified on the 14th = 4 calendar days -> contravention.
    let assessed = Utc
        .with_ymd_and_hms(2026, 1, 10, 9, 0, 0)
        .single()
        .expect("ts");
    let notified = Utc
        .with_ymd_and_hms(2026, 1, 14, 9, 0, 0)
        .single()
        .expect("ts");
    let breach = breach_assessed_and_notified(assessed, notified);
    assert_eq!(breach.days_to_pdpc_notification(), Some(4));
    assert!(!breach.is_pdpc_notification_timely());
    assert!(matches!(
        validate_breach_notification(&breach),
        Err(PdpaError::LateBreachNotification)
    ));
}

#[test]
fn deadline_runs_from_assessment_not_discovery() {
    // Discovery is the 1st, but assessment completes on the 10th; the 3-day
    // clock runs from the 10th (s. 26D(1)). Notified on the 12th -> timely,
    // even though that is 11 days after discovery.
    let discovery = Utc
        .with_ymd_and_hms(2026, 1, 1, 9, 0, 0)
        .single()
        .expect("ts");
    let assessed = Utc
        .with_ymd_and_hms(2026, 1, 10, 9, 0, 0)
        .single()
        .expect("ts");
    let notified = Utc
        .with_ymd_and_hms(2026, 1, 12, 9, 0, 0)
        .single()
        .expect("ts");

    let mut breach = DataBreachBuilder::new("b-disc", BreachType::DataLoss, "x")
        .affected_individuals(800)
        .affected_category(PersonalDataCategory::Email)
        .build();
    breach.discovery_date = discovery;
    breach.record_assessment(assessed);
    breach.notify_pdpc(notified);

    assert_eq!(breach.days_to_pdpc_notification(), Some(2));
    assert!(breach.is_pdpc_notification_timely());
    // The statutory deadline is assessment + 3 days = 13 Jan 2026.
    assert_eq!(
        breach
            .pdpc_notification_deadline()
            .expect("deadline")
            .date_naive(),
        Utc.with_ymd_and_hms(2026, 1, 13, 9, 0, 0)
            .single()
            .expect("ts")
            .date_naive()
    );
}

#[test]
fn notifiable_breach_never_notified_is_a_contravention() {
    let mut breach = DataBreachBuilder::new("b-none", BreachType::Theft, "x")
        .affected_individuals(700)
        .affected_category(PersonalDataCategory::Email)
        .build();
    breach.record_assessment(Utc::now());
    // No notify_pdpc call.
    assert!(!breach.is_pdpc_notification_timely());
    assert!(matches!(
        validate_breach_notification(&breach),
        Err(PdpaError::LateBreachNotification)
    ));
}

// ---------------------------------------------------------------------------
// Individual notification and exceptions (s. 26D(2)/(5))
// ---------------------------------------------------------------------------

#[test]
fn significant_harm_breach_requires_individual_notification() {
    let assessed = Utc::now();
    let mut breach = DataBreachBuilder::new("b-harm-ind", BreachType::UnauthorizedAccess, "x")
        .affected_individuals(20)
        .affected_category(PersonalDataCategory::IdentificationNumber)
        .affected_category(PersonalDataCategory::Health)
        .build();
    breach.record_assessment(assessed);
    breach.notify_pdpc(assessed);

    // PDPC notified, but individuals neither notified nor exempted.
    assert!(breach.individual_notification_outstanding());
    assert!(matches!(
        validate_breach_notification(&breach),
        Err(PdpaError::IndividualsNotNotified)
    ));

    // Notifying individuals discharges the duty.
    breach.notify_individuals(assessed);
    assert!(!breach.individual_notification_outstanding());
    assert!(validate_breach_notification(&breach).is_ok());
}

#[test]
fn encryption_exception_removes_individual_notification_duty() {
    let assessed = Utc::now();
    let mut breach = DataBreachBuilder::new("b-enc", BreachType::Theft, "x")
        .affected_individuals(30)
        .affected_category(PersonalDataCategory::Name)
        .affected_category(PersonalDataCategory::Financial)
        .build();
    breach.record_assessment(assessed);
    breach.notify_pdpc(assessed);
    breach.set_individual_notification_exemption(
        IndividualNotificationExemption::TechnologicalProtection,
    );
    assert!(!breach.individual_notification_outstanding());
    assert!(validate_breach_notification(&breach).is_ok());
    assert_eq!(
        IndividualNotificationExemption::TechnologicalProtection.statute_section(),
        "PDPA s. 26D(5)(b)"
    );
}

#[test]
fn calendar_days_helper_counts_dates_not_elapsed_hours() {
    // 23:59 on day 1 to 00:01 on day 2 is 1 calendar day, though only minutes
    // elapsed — matching the PDPA's "calendar days" wording.
    let start = Utc
        .with_ymd_and_hms(2026, 3, 1, 23, 59, 0)
        .single()
        .expect("ts");
    let end = Utc
        .with_ymd_and_hms(2026, 3, 2, 0, 1, 0)
        .single()
        .expect("ts");
    assert_eq!(calendar_days_between(start, end), 1);
}
