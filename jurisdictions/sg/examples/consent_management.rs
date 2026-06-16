//! PDPA Consent Management Example (Personal Data Protection Act 2012)
//!
//! Demonstrates Singapore's **consent-centric** model under the PDPA:
//!
//! - Express consent (s. 14) vs deemed consent (s. 15 conduct / s. 15A notification)
//! - The contrast with the GDPR's six lawful bases
//! - Purpose limitation (s. 18) — re-purposing operational data for marketing
//! - Withdrawal of consent (s. 16), including the s. 16(2) duty to explain
//!   the likely consequences of withdrawal
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example consent_management
//! ```

use chrono::Duration;
use legalis_sg::pdpa::*;

fn main() {
    println!("== Singapore PDPA Consent Management (PDPA 2012) ==\n");

    express_vs_deemed_consent();
    println!("\n{}\n", "-".repeat(68));
    deemed_consent_by_notification();
    println!("\n{}\n", "-".repeat(68));
    purpose_limitation();
    println!("\n{}\n", "-".repeat(68));
    withdrawal_of_consent();
}

/// Section 13 default rule: consent is required. Unlike the GDPR (6 lawful
/// bases), the PDPA is consent-centric, with deemed consent (s. 15) and the
/// Schedule exceptions as the main alternatives.
fn express_vs_deemed_consent() {
    println!("Express consent (s. 14) vs deemed consent by conduct (s. 15(1))\n");

    let express = ConsentRecordBuilder::express(
        "consent-express-001",
        "customer@example.com",
        PurposeOfCollection::Marketing,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .build();
    match express {
        Ok(c) => println!(
            "  express marketing consent valid (method={:?}, {})",
            c.consent_method,
            c.consent_method.statute_section()
        ),
        Err(e) => println!("  express consent rejected: {}", first_line(&e.to_string())),
    }

    // A customer who voluntarily hands over their phone number to receive a
    // delivery is deemed to consent to that use (s. 15(1)).
    let deemed = ConsentRecordBuilder::deemed(
        "consent-deemed-001",
        "customer@example.com",
        PurposeOfCollection::ServiceDelivery,
        DeemedConsentBasis::ByConduct,
    )
    .data_category(PersonalDataCategory::Phone)
    .build();
    match deemed {
        Ok(c) => println!(
            "  deemed-by-conduct consent valid for delivery ({})",
            c.deemed_basis.map(|b| b.statute_section()).unwrap_or("n/a")
        ),
        Err(e) => println!("  deemed consent rejected: {}", first_line(&e.to_string())),
    }
}

/// Deemed consent by notification (s. 15A) is the most demanding limb: it
/// requires a prior assessment that the use is not likely to have an adverse
/// effect, plus a reasonable opt-out window.
fn deemed_consent_by_notification() {
    println!("Deemed consent by notification (s. 15A) — assessment + opt-out required\n");

    // Without the s. 15A(4)(a) assessment and opt-out window the consent is invalid.
    let incomplete = ConsentRecordBuilder::deemed(
        "consent-15a-bad",
        "customer@example.com",
        PurposeOfCollection::Analytics,
        DeemedConsentBasis::ByNotification,
    )
    .data_category(PersonalDataCategory::Email)
    .build();
    match incomplete {
        Ok(_) => println!("  [no assessment] unexpectedly accepted"),
        Err(e) => println!("  [no assessment] rejected: {}", first_line(&e.to_string())),
    }

    // With both preconditions satisfied it is valid.
    let complete = ConsentRecordBuilder::deemed(
        "consent-15a-good",
        "customer@example.com",
        PurposeOfCollection::Analytics,
        DeemedConsentBasis::ByNotification,
    )
    .data_category(PersonalDataCategory::Email)
    .notification_assessment(Duration::days(30))
    .build();
    match complete {
        Ok(_) => println!("  [assessment + 30-day opt-out] valid (s. 15A satisfied)"),
        Err(e) => println!(
            "  [assessment + opt-out] rejected: {}",
            first_line(&e.to_string())
        ),
    }
}

/// Purpose limitation (s. 18): data collected for one purpose may not be freely
/// re-purposed. Marketing in particular always needs fresh consent.
fn purpose_limitation() {
    println!("Purpose limitation (s. 18)\n");

    let consent = ConsentRecordBuilder::express(
        "consent-service-001",
        "customer@example.com",
        PurposeOfCollection::ServiceDelivery,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .data_category(PersonalDataCategory::Address)
    .build()
    .expect("valid service-delivery consent");

    for intended in [
        PurposeOfCollection::OrderProcessing,
        PurposeOfCollection::Marketing,
    ] {
        match validate_purpose_limitation(&consent, intended) {
            Ok(()) => println!(
                "  use for {:?}: permitted (compatible with collection purpose)",
                intended
            ),
            Err(e) => println!(
                "  use for {:?}: blocked — {}",
                intended,
                first_line(&e.to_string())
            ),
        }
    }
}

/// Withdrawal of consent (s. 16): permitted at any time on reasonable notice;
/// the organisation must explain the likely consequences (s. 16(2)) and then
/// cease the relevant collection/use/disclosure (s. 16(4)).
fn withdrawal_of_consent() {
    println!("Withdrawal of consent (s. 16)\n");

    let mut consent = ConsentRecordBuilder::express(
        "consent-withdraw-001",
        "customer@example.com",
        PurposeOfCollection::Marketing,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .build()
    .expect("valid marketing consent");

    // Non-compliant withdrawal: consequences not explained (s. 16(2)).
    consent.withdraw(Some("No longer interested".to_string()), false);
    match validate_withdrawal(&consent) {
        Ok(()) => println!("  [consequences not explained] unexpectedly accepted"),
        Err(e) => println!(
            "  [consequences not explained] flagged: {}",
            first_line(&e.to_string())
        ),
    }

    // Compliant withdrawal flow.
    consent.consequences_of_withdrawal_explained = true;
    match validate_withdrawal(&consent) {
        Ok(()) => println!("  [consequences explained] compliant withdrawal recorded (s. 16)"),
        Err(e) => println!(
            "  [consequences explained] rejected: {}",
            first_line(&e.to_string())
        ),
    }
    // After withdrawal the consent no longer authorises any purpose (s. 16(4)).
    println!(
        "  marketing now authorised? {} (processing must cease, s. 16(4))",
        consent.authorises_purpose(PurposeOfCollection::Marketing)
    );
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or(s)
}
