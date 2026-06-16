//! PDPA Do Not Call (DNC) Registry Example (Personal Data Protection Act 2012)
//!
//! Demonstrates the check-before-marketing rule under Part 9 of the PDPA:
//!
//! - The three registers (s. 39): No Voice Call, No Text Message, No Fax Message
//! - The duty to check the relevant register before sending a specified message
//!   (s. 43(1))
//! - The 21-day validity of a confirmation of non-registration (s. 43(2) read
//!   with reg. 15 of the PDP (Do Not Call Registry) Regulations 2013)
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example dnc_registry_check
//! ```

use chrono::{Duration, Utc};
use legalis_sg::pdpa::*;

fn main() {
    println!("== Singapore PDPA Do Not Call Registry Check (PDPA 2012, Part 9) ==\n");

    the_three_registers();
    println!("\n{}\n", "-".repeat(68));
    check_before_marketing();
    println!("\n{}\n", "-".repeat(68));
    confirmation_validity_window();
}

/// Section 39: the DNC Registry comprises three registers, one per kind of
/// specified message.
fn the_three_registers() {
    println!("The three DNC registers (s. 39)\n");
    for kind in [
        DncRegisterKind::VoiceCall,
        DncRegisterKind::TextMessage,
        DncRegisterKind::Fax,
    ] {
        println!("  {:?} -> {}", kind, kind.register_name());
    }
}

/// Section 43(1): a marketing message may not be sent to a Singapore number
/// listed on the relevant register; and a valid prior check is required.
fn check_before_marketing() {
    println!("Check before sending a specified message (s. 43)\n");

    // A number listed on the No Voice Call Register.
    let mut listed = DncRegistration::new("+6591111111");
    listed.register(DncRegisterKind::VoiceCall);

    let now = Utc::now();
    let conf = DncCheckConfirmation::at("+6591111111", DncRegisterKind::VoiceCall, now);
    match validate_dnc_before_marketing(
        "+6591111111",
        DncRegisterKind::VoiceCall,
        &listed,
        Some(&conf),
        now,
    ) {
        Ok(()) => println!("  [+6591111111 voice] may call"),
        Err(e) => println!(
            "  [+6591111111 voice] blocked: {}",
            first_line(&e.to_string())
        ),
    }

    // An unlisted number, with a fresh non-registration confirmation -> may send.
    let unlisted = DncRegistration::new("+6592222222");
    let conf2 = DncCheckConfirmation::at("+6592222222", DncRegisterKind::TextMessage, now);
    match validate_dnc_before_marketing(
        "+6592222222",
        DncRegisterKind::TextMessage,
        &unlisted,
        Some(&conf2),
        now,
    ) {
        Ok(()) => println!("  [+6592222222 SMS] may send (unlisted + valid check)"),
        Err(e) => println!(
            "  [+6592222222 SMS] blocked: {}",
            first_line(&e.to_string())
        ),
    }

    // Unlisted but no prior check -> still blocked (s. 43(2)).
    match validate_dnc_before_marketing(
        "+6592222222",
        DncRegisterKind::TextMessage,
        &unlisted,
        None,
        now,
    ) {
        Ok(()) => println!("  [+6592222222 SMS, no check] sent"),
        Err(e) => println!(
            "  [+6592222222 SMS, no check] blocked: {}",
            first_line(&e.to_string())
        ),
    }
}

/// Section 43(2) / reg. 15: a confirmation of non-registration is valid for
/// 21 days. A check obtained 22 days ago is stale.
fn confirmation_validity_window() {
    println!(
        "Confirmation validity window: {} days (reg. 15)\n",
        DNC_CONFIRMATION_VALIDITY_DAYS
    );

    let unlisted = DncRegistration::new("+6593333333");
    let now = Utc::now();

    let fresh = DncCheckConfirmation::at(
        "+6593333333",
        DncRegisterKind::VoiceCall,
        now - Duration::days(20),
    );
    println!(
        "  check 20 days old -> still valid? {}",
        fresh.is_valid_for("+6593333333", DncRegisterKind::VoiceCall, now)
    );

    let stale = DncCheckConfirmation::at(
        "+6593333333",
        DncRegisterKind::VoiceCall,
        now - Duration::days(22),
    );
    println!(
        "  check 22 days old -> still valid? {}",
        stale.is_valid_for("+6593333333", DncRegisterKind::VoiceCall, now)
    );
    match validate_dnc_before_marketing(
        "+6593333333",
        DncRegisterKind::VoiceCall,
        &unlisted,
        Some(&stale),
        now,
    ) {
        Ok(()) => println!("  marketing with a 22-day-old check: allowed"),
        Err(e) => println!(
            "  marketing with a 22-day-old check: blocked — {}",
            first_line(&e.to_string())
        ),
    }
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or(s)
}
