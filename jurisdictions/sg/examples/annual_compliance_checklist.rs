//! Annual Compliance Checklist Example (Companies Act 1967)
//!
//! Computes the key recurring statutory deadlines for a Singapore company under
//! the current (post-2018) financial-year-end-based regime:
//!
//! - Annual General Meeting (AGM) — s. 175(1)
//! - Annual return filing — s. 197(1)
//! - Company secretary requirement — s. 171
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example annual_compliance_checklist
//! ```

use chrono::{DateTime, Datelike, TimeZone, Utc};
use legalis_sg::companies::*;

fn main() {
    println!("== Singapore Annual Compliance Checklist (Companies Act 1967) ==\n");

    // A private company with a 31 December financial year end.
    let fye = ymd_or_now(2024, 12, 31);

    let mut company = Company::new(
        "202401234A",
        "Tech Innovations Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("1 Raffles Place", "048616"),
    );
    company.registration_date = ymd_or_now(2024, 1, 15);
    company
        .directors
        .push(Director::new("John Tan", "S1234567A", true));
    company.company_secretary = Some(CompanySecretary::new("Mary Lim", "S7654321B"));

    println!("Company: {} ({})", company.name, company.company_type);
    println!("Financial year end: {}\n", company.financial_year_end);

    agm_deadlines(fye);
    println!();
    annual_return_deadlines(fye);
    println!();
    secretary_check(&company);
}

/// Section 175(1): hold the AGM within 4 months of FYE (listed) or 6 months (other).
fn agm_deadlines(fye: DateTime<Utc>) {
    println!("AGM deadline (s. 175(1)) from FYE {}:", fmt_date(fye));
    for (label, is_listed) in [("non-listed", false), ("listed", true)] {
        match calculate_agm_deadline_from_fye(fye, is_listed) {
            Some(deadline) => println!(
                "  {} company: hold AGM by {} ({} months)",
                label,
                fmt_date(deadline),
                agm_deadline_months(is_listed)
            ),
            None => println!("  {} company: deadline unavailable", label),
        }
    }
}

/// Section 197(1): file the annual return within 5 months of FYE (listed) or 7 months (other).
fn annual_return_deadlines(fye: DateTime<Utc>) {
    println!(
        "Annual return deadline (s. 197(1)) from FYE {}:",
        fmt_date(fye)
    );
    for (label, is_listed) in [("non-listed", false), ("listed", true)] {
        match calculate_annual_return_deadline_from_fye(fye, is_listed) {
            Some(deadline) => println!("  {} company: file by {}", label, fmt_date(deadline)),
            None => println!("  {} company: deadline unavailable", label),
        }
    }
}

/// Section 171: a private company must have a secretary, with the office not
/// vacant for more than 6 months.
fn secretary_check(company: &Company) {
    println!("Company secretary requirement (s. 171):");
    match validate_company_secretary_requirement(company, Utc::now()) {
        Ok(()) => println!("  compliant (secretary appointed and within the rules)"),
        Err(e) => println!("  issue: {}", first_line(&e.to_string())),
    }
}

fn ymd_or_now(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    match Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single() {
        Some(dt) => dt,
        None => Utc::now(),
    }
}

fn fmt_date(dt: DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", dt.year(), dt.month(), dt.day())
}

fn first_line(message: &str) -> &str {
    message.lines().next().unwrap_or(message)
}
