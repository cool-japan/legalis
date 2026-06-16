//! PDPA Data Breach Notification Example (Personal Data Protection Act 2012)
//!
//! Demonstrates the mandatory data breach notification regime (Part 6A):
//!
//! - Notifiable-breach determination (s. 26B): the significant-harm limb
//!   (s. 26B(1)(a)) and the significant-scale limb of 500 individuals
//!   (s. 26B(1)(b)/(3)(a))
//! - The internal-only carve-out (s. 26B(4))
//! - The duty to notify the PDPC within **3 calendar days** of assessment
//!   (s. 26D(1))
//! - The duty to notify affected individuals (s. 26D(2)) and the exceptions
//!   (s. 26D(5), e.g. encryption / remedial action)
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example data_breach_notification
//! ```

use chrono::{Duration, Utc};
use legalis_sg::pdpa::*;

fn main() {
    println!("== Singapore PDPA Data Breach Notification (PDPA 2012, Part 6A) ==\n");

    notifiability_determination();
    println!("\n{}\n", "-".repeat(68));
    three_calendar_day_deadline();
    println!("\n{}\n", "-".repeat(68));
    individual_notification_and_exceptions();
}

/// Section 26B: a breach is notifiable if it causes (or is likely to cause)
/// significant harm OR is of a significant scale (>= 500 individuals).
fn notifiability_determination() {
    println!("Notifiable-breach determination (s. 26B)\n");

    // Significant scale: 800 individuals (>= 500), basic contact data only.
    let scale = DataBreachBuilder::new(
        "breach-scale",
        BreachType::AccidentalExposure,
        "Mailing list of 800 customers exposed",
    )
    .affected_individuals(800)
    .affected_category(PersonalDataCategory::Email)
    .build();
    report_assessment(&scale);

    // Significant harm: 12 individuals, but name + financial data (reg. 3(1)(a)).
    let harm = DataBreachBuilder::new(
        "breach-harm",
        BreachType::UnauthorizedAccess,
        "12 customer records with NRIC and bank account numbers accessed",
    )
    .affected_individuals(12)
    .affected_category(PersonalDataCategory::IdentificationNumber)
    .affected_category(PersonalDataCategory::Financial)
    .build();
    report_assessment(&harm);

    // Internal-only: excluded from notification (s. 26B(4)).
    let internal = DataBreachBuilder::new(
        "breach-internal",
        BreachType::UnauthorizedAccess,
        "Employee viewed colleagues' records; no external disclosure",
    )
    .internal_only()
    .affected_individuals(2000)
    .affected_category(PersonalDataCategory::Financial)
    .affected_category(PersonalDataCategory::IdentificationNumber)
    .build();
    report_assessment(&internal);
}

fn report_assessment(breach: &DataBreachNotification) {
    let a = breach.assess_notifiability();
    println!(
        "  [{}] notifiable={} (harm={}, scale={}, internal_only={}) | individuals required={}",
        breach.breach_id,
        a.is_notifiable(),
        a.significant_harm,
        a.significant_scale,
        a.internal_only,
        a.requires_individual_notification(),
    );
}

/// Section 26D(1): the PDPC must be notified no later than 3 CALENDAR days after
/// the day the organisation assesses the breach to be notifiable.
fn three_calendar_day_deadline() {
    println!("3-calendar-day PDPC notification deadline (s. 26D(1))\n");

    let assessed = Utc::now() - Duration::days(1);

    // Timely: notified 2 calendar days after assessment.
    let mut timely = DataBreachBuilder::new(
        "breach-timely",
        BreachType::Ransomware,
        "Ransomware affecting 1,500 customer records",
    )
    .affected_individuals(1500)
    .affected_category(PersonalDataCategory::Financial)
    .affected_category(PersonalDataCategory::Name)
    .build();
    timely.record_assessment(assessed);
    timely.notify_pdpc(assessed + Duration::days(2));
    println!(
        "  [timely] assessed -> PDPC in {} calendar day(s); within deadline? {}",
        timely.days_to_pdpc_notification().unwrap_or(-1),
        timely.is_pdpc_notification_timely(),
    );
    if let Some(deadline) = timely.pdpc_notification_deadline() {
        println!(
            "           statutory deadline was {} (assessment + 3 days)",
            deadline.date_naive()
        );
    }

    // Late: notified 4 calendar days after assessment.
    let mut late = DataBreachBuilder::new(
        "breach-late",
        BreachType::Theft,
        "Stolen laptop with 700 customer records",
    )
    .affected_individuals(700)
    .affected_category(PersonalDataCategory::Email)
    .build();
    late.record_assessment(assessed);
    late.notify_pdpc(assessed + Duration::days(4));
    println!(
        "  [late]   assessed -> PDPC in {} calendar day(s); within deadline? {}",
        late.days_to_pdpc_notification().unwrap_or(-1),
        late.is_pdpc_notification_timely(),
    );
    match validate_breach_notification(&late) {
        Ok(()) => println!("           validation: compliant"),
        Err(e) => println!("           validation: {}", first_line(&e.to_string())),
    }
}

/// Section 26D(2): affected individuals must be notified where the
/// significant-harm limb applies — subject to the s. 26D(5) exceptions, e.g.
/// the data was strongly encrypted before the breach.
fn individual_notification_and_exceptions() {
    println!("Individual notification and exceptions (s. 26D(2)/(5))\n");

    let assessed = Utc::now();

    let mut breach = DataBreachBuilder::new(
        "breach-individuals",
        BreachType::UnauthorizedAccess,
        "50 records with NRIC + health information accessed",
    )
    .affected_individuals(50)
    .affected_category(PersonalDataCategory::IdentificationNumber)
    .affected_category(PersonalDataCategory::Health)
    .build();
    breach.record_assessment(assessed);
    breach.notify_pdpc(assessed); // PDPC notified same day

    // Before notifying individuals or claiming an exception -> outstanding.
    println!(
        "  individual notification outstanding (before action)? {}",
        breach.individual_notification_outstanding()
    );
    match validate_breach_notification(&breach) {
        Ok(()) => println!("  validation: compliant"),
        Err(e) => println!("  validation: {}", first_line(&e.to_string())),
    }

    // Apply the technological-protection (encryption) exception (s. 26D(5)(b)).
    breach.set_individual_notification_exemption(
        IndividualNotificationExemption::TechnologicalProtection,
    );
    println!(
        "  after claiming encryption exception ({}): outstanding? {}",
        IndividualNotificationExemption::TechnologicalProtection.statute_section(),
        breach.individual_notification_outstanding()
    );
    match validate_breach_notification(&breach) {
        Ok(()) => println!("  validation: compliant (individuals need not be notified)"),
        Err(e) => println!("  validation: {}", first_line(&e.to_string())),
    }
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or(s)
}
