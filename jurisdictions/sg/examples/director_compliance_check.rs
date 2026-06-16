//! Director Compliance Check Example (Companies Act 1967)
//!
//! Demonstrates director eligibility checks under the Singapore Companies Act:
//!
//! - Resident director requirement (s. 145(1))
//! - Disqualification grounds (s. 148 bankruptcy, s. 149/155 Court order,
//!   s. 154 conviction)
//! - Expiry of time-limited disqualifications
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example director_compliance_check
//! ```

use chrono::{Duration, Utc};
use legalis_sg::companies::*;

fn main() {
    println!("== Singapore Director Compliance Check (Companies Act 1967) ==\n");

    check_resident_director_requirement();
    println!("\n{}\n", "-".repeat(64));
    check_disqualification_grounds();
    println!("\n{}\n", "-".repeat(64));
    check_expired_disqualification();
}

/// Section 145(1): every company must have at least one director ordinarily
/// resident in Singapore.
fn check_resident_director_requirement() {
    println!("Section 145(1): resident director requirement\n");

    let foreign_only = vec![
        Director::new("Alice Johnson", "P7654321", false),
        Director::new("Bob Williams", "P9876543", false),
    ];
    match validate_resident_director_requirement(&foreign_only) {
        Ok(()) => println!("  [foreign-only board] valid"),
        Err(e) => println!(
            "  [foreign-only board] rejected: {}",
            first_line(&e.to_string())
        ),
    }

    let with_resident = vec![
        Director::new("John Tan", "S1234567A", true),
        Director::new("Bob Williams", "P9876543", false),
    ];
    match validate_resident_director_requirement(&with_resident) {
        Ok(()) => println!("  [board with 1 resident] valid (s. 145(1) satisfied)"),
        Err(e) => println!(
            "  [board with 1 resident] rejected: {}",
            first_line(&e.to_string())
        ),
    }
}

/// Sections 148/149/154/155: disqualification grounds.
fn check_disqualification_grounds() {
    println!("Disqualification grounds (s. 148 / 149 / 154 / 155)\n");
    let now = Utc::now();

    let mut bankrupt = Director::new("Carol Lee", "S2222222B", true);
    bankrupt.disqualification_status = DisqualificationStatus::BankruptcyDisqualification {
        bankruptcy_date: now - Duration::days(30),
    };

    let mut convicted = Director::new("Dave Ng", "S3333333C", true);
    convicted.disqualification_status = DisqualificationStatus::ConvictionDisqualification {
        conviction_date: now - Duration::days(10),
        offense: "Cheating (Penal Code s. 420)".to_string(),
        disqualification_until: now + Duration::days(5 * 365),
    };

    for director in [&bankrupt, &convicted] {
        let section = director
            .disqualification_status
            .statute_section()
            .unwrap_or("n/a");
        match validate_director_disqualification(director, now) {
            Ok(()) => println!("  {} ({}): eligible", director.name, section),
            Err(e) => println!(
                "  {} ({}): {}",
                director.name,
                section,
                first_line(&e.to_string())
            ),
        }
    }
}

/// A time-limited disqualification (e.g. the 5-year period under s. 154) lapses
/// once the period has elapsed.
fn check_expired_disqualification() {
    println!("Expiry of a time-limited disqualification (s. 154)\n");
    let now = Utc::now();

    let mut director = Director::new("Eve Goh", "S4444444D", true);
    director.disqualification_status = DisqualificationStatus::ConvictionDisqualification {
        conviction_date: now - Duration::days(6 * 365),
        offense: "Fraud".to_string(),
        disqualification_until: now - Duration::days(1), // lapsed yesterday
    };

    println!(
        "  is_eligible (recorded status only): {}",
        director.is_eligible()
    );
    println!(
        "  is_eligible_as_of(now) (honours expiry): {}",
        director.is_eligible_as_of(now)
    );
    match validate_director_disqualification(&director, now) {
        Ok(()) => println!("  validate_director_disqualification(now): eligible again"),
        Err(e) => println!("  rejected: {}", first_line(&e.to_string())),
    }
}

/// Returns the first line of a (multilingual) error message for compact output.
fn first_line(message: &str) -> &str {
    message.lines().next().unwrap_or(message)
}
