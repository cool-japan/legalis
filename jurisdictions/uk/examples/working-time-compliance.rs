//! Working Time Regulations 1998 compliance examples.
//!
//! Demonstrates the core limits of the Working Time Regulations 1998 (WTR 1998),
//! using the `legalis_uk::employment` library APIs:
//!
//! - **48-hour average working week** (WTR 1998 Reg 4) — averaged over a 17-week
//!   reference period, unless the worker has signed an individual opt-out.
//! - **Rest breaks and rest periods** (WTR 1998 Regs 10-12) — a 20-minute rest
//!   break where the working day exceeds 6 hours, 11 hours' daily rest, and
//!   24 hours' weekly rest.
//! - **Paid annual leave** (WTR 1998 Reg 13 / 13A) — 5.6 weeks (28 days for a
//!   five-day week, pro-rated for part-timers).

use legalis_uk::employment::{
    AnnualLeaveEntitlement, RestEntitlement, WorkingHours, validate_working_hours,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UK Working Time Regulations 1998 Compliance ===\n");

    weekly_hours_examples()?;
    rest_break_examples();
    annual_leave_examples()?;

    Ok(())
}

/// WTR 1998 Reg 4 — the 48-hour average weekly limit and the individual opt-out.
fn weekly_hours_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("WTR 1998 Reg 4 — 48-hour average weekly limit");
    println!("---------------------------------------------\n");

    let compliant = WorkingHours {
        hours_per_week: 40,
        days_per_week: 5,
        opted_out_of_48h_limit: false,
        night_work_hours: None,
    };
    report_hours("Standard 40-hour week", &compliant)?;

    let at_limit = WorkingHours {
        hours_per_week: 48,
        days_per_week: 6,
        opted_out_of_48h_limit: false,
        night_work_hours: None,
    };
    report_hours("Exactly at the 48-hour limit", &at_limit)?;

    let over_limit = WorkingHours {
        hours_per_week: 55,
        days_per_week: 6,
        opted_out_of_48h_limit: false,
        night_work_hours: None,
    };
    report_hours("55 hours with no opt-out (breach)", &over_limit)?;

    let over_with_optout = WorkingHours {
        hours_per_week: 55,
        days_per_week: 6,
        opted_out_of_48h_limit: true,
        night_work_hours: None,
    };
    report_hours("55 hours with a signed opt-out", &over_with_optout)?;

    Ok(())
}

/// Validate a single set of working hours and print the outcome.
fn report_hours(label: &str, hours: &WorkingHours) -> Result<(), Box<dyn std::error::Error>> {
    println!("{label}:");
    println!("  Hours/week: {}", hours.hours_per_week);
    println!(
        "  Within 48h limit (method): {}",
        hours.complies_with_48h_limit()
    );
    match validate_working_hours(hours) {
        Ok(()) => println!("  validate_working_hours: OK\n"),
        Err(e) => println!("  validate_working_hours: BREACH -> {e}\n"),
    }
    Ok(())
}

/// WTR 1998 Regs 10-12 — rest breaks and daily/weekly rest periods.
fn rest_break_examples() {
    println!("WTR 1998 Regs 10-12 — rest breaks and rest periods");
    println!("--------------------------------------------------\n");

    for daily_hours in [5_u8, 6, 9] {
        let rest = RestEntitlement {
            daily_hours,
            days_per_week: 5,
        };
        println!("Working day of {daily_hours} hours:");
        println!("  Rest break: {} minutes", rest.rest_break_minutes());
        println!(
            "  Daily rest: {} hours; weekly rest: {} hours\n",
            rest.daily_rest_hours(),
            rest.weekly_rest_hours()
        );
    }
}

/// WTR 1998 Reg 13 — 5.6 weeks' statutory paid annual leave.
fn annual_leave_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("WTR 1998 Reg 13 — statutory annual leave (5.6 weeks)");
    println!("----------------------------------------------------\n");

    let leave_year_start =
        chrono::NaiveDate::from_ymd_opt(2024, 1, 1).ok_or("invalid leave-year start date")?;

    let full_time = AnnualLeaveEntitlement {
        days_per_week: 5,
        leave_year_start,
    };
    println!(
        "Full-time (5 days/week): {:.1} days (28-day statutory minimum)",
        full_time.statutory_minimum_days()
    );

    let part_time = AnnualLeaveEntitlement {
        days_per_week: 3,
        leave_year_start,
    };
    println!(
        "Part-time (3 days/week): {:.1} days (pro-rated)\n",
        part_time.statutory_minimum_days()
    );

    Ok(())
}
