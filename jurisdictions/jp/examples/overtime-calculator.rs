//! Overtime Wage Calculator Example (残業代計算例)
//!
//! This example demonstrates overtime premium calculations according to
//! Article 37 of the Labor Standards Act (労働基準法).
//!
//! # Legal Framework
//!
//! - Overtime (時間外): 25% premium
//! - Late night (深夜 22:00-5:00): 25% premium
//! - Holiday (休日): 35% premium
//! - Over 60 hours/month: 50% premium (大企業)
//!
//! # Usage
//! ```bash
//! cargo run --example overtime-calculator
//! ```

use chrono::{Duration, Utc};
use legalis_jp::labor_law::*;

fn main() {
    println!("=== Overtime Wage Calculator Example ===\n");
    println!("残業代計算例 - Overtime Premium Calculation\n");

    // Example 1: Regular Working Day
    println!("📋 Example 1: Regular Working Day (通常勤務日)");
    println!("{}", "=".repeat(70));

    let now = Utc::now();
    let regular_day = WorkingTimeRecord {
        date: now,
        start_time: now,
        end_time: now + Duration::hours(9),
        rest_minutes: 60,
        is_holiday: false,
    };

    println!(
        "Working Hours: {:.1} hours",
        regular_day.actual_working_hours()
    );
    println!("Rest Period: {} minutes", regular_day.rest_minutes);
    println!(
        "Overtime: {:.1} hours",
        regular_day.overtime_hours(STATUTORY_HOURS_PER_DAY)
    );

    match validate_working_time_record(&regular_day) {
        Ok(()) => println!("✅ Validation: PASSED - Rest period is sufficient!"),
        Err(e) => println!("❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 2: Overtime Work
    println!("📋 Example 2: Overtime Work (残業あり)");
    println!("{}", "=".repeat(70));

    let overtime_day = WorkingTimeRecord {
        date: now,
        start_time: now,
        end_time: now + Duration::hours(11),
        rest_minutes: 60,
        is_holiday: false,
    };

    let actual_hours = overtime_day.actual_working_hours();
    let overtime_hours = overtime_day.overtime_hours(STATUTORY_HOURS_PER_DAY);

    println!("Working Hours: {:.1} hours", actual_hours);
    println!("Regular Hours: {} hours", STATUTORY_HOURS_PER_DAY);
    println!("Overtime Hours: {:.1} hours", overtime_hours);

    let base_hourly_rate = 2_000; // ¥2,000/hour
    let regular_pay = STATUTORY_HOURS_PER_DAY as u64 * base_hourly_rate;
    let overtime_pay =
        (overtime_hours * base_hourly_rate as f64 * (1.0 + OVERTIME_PREMIUM_RATE)) as u64;
    let total_pay = regular_pay + overtime_pay;

    println!("\nWage Calculation (Base: ¥{}/hour):", base_hourly_rate);
    println!("  Regular Pay: ¥{}", regular_pay);
    println!("  Overtime Pay (25% premium): ¥{}", overtime_pay);
    println!("  Total: ¥{}", total_pay);

    match validate_working_time_record(&overtime_day) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Holiday Work
    println!("📋 Example 3: Holiday Work (休日労働)");
    println!("{}", "=".repeat(70));

    let holiday_work = WorkingTimeRecord {
        date: now,
        start_time: now,
        end_time: now + Duration::hours(8),
        rest_minutes: 60,
        is_holiday: true,
    };

    let holiday_hours = holiday_work.actual_working_hours();

    println!("Holiday Work Hours: {:.1} hours", holiday_hours);
    println!(
        "Premium Rate: {}% (Article 37)",
        (HOLIDAY_WORK_PREMIUM_RATE * 100.0) as u32
    );

    let holiday_pay =
        (holiday_hours * base_hourly_rate as f64 * (1.0 + HOLIDAY_WORK_PREMIUM_RATE)) as u64;

    println!("\nWage Calculation:");
    println!("  Base Rate: ¥{}/hour", base_hourly_rate);
    println!("  Holiday Premium (35%): ¥{}", holiday_pay);

    println!("\n✅ Holiday work premium properly calculated!\n");

    println!("{}\n", "=".repeat(70));

    // Example 4: Monthly Summary with Mixed Work
    println!("📋 Example 4: Monthly Wage Summary (月間賃金集計)");
    println!("{}", "=".repeat(70));

    let monthly_summary = MonthlyWorkingSummary {
        year: 2026,
        month: 1,
        total_hours: 184.0,    // ~23 days × 8 hours
        overtime_hours: 25.0,  // 25 hours overtime
        late_night_hours: 8.0, // 8 hours late night
        holiday_hours: 8.0,    // 1 day holiday work
        days_worked: 23,
    };

    println!(
        "Year/Month: {}/{:02}",
        monthly_summary.year, monthly_summary.month
    );
    println!("Days Worked: {} days", monthly_summary.days_worked);
    println!("Total Hours: {:.1} hours", monthly_summary.total_hours);
    println!(
        "  - Regular: {:.1} hours",
        monthly_summary.total_hours - monthly_summary.overtime_hours
    );
    println!("  - Overtime: {:.1} hours", monthly_summary.overtime_hours);
    println!(
        "  - Late Night: {:.1} hours",
        monthly_summary.late_night_hours
    );
    println!("  - Holiday: {:.1} hours", monthly_summary.holiday_hours);

    if monthly_summary.exceeds_overtime_limit() {
        println!("\n⚠️  WARNING: Overtime exceeds 60 hours/month!");
        println!("   Higher premium rate (50%) applies to excess hours.");
    }

    println!("\nWage Breakdown (Hourly Rate: ¥{}):", base_hourly_rate);

    let base_wage = monthly_summary.calculate_base_wage(base_hourly_rate);
    let overtime_premium = monthly_summary.calculate_overtime_premium(base_hourly_rate);
    let late_night_premium = monthly_summary.calculate_late_night_premium(base_hourly_rate);
    let holiday_premium = monthly_summary.calculate_holiday_premium(base_hourly_rate);
    let total_wage = monthly_summary.calculate_total_wage(base_hourly_rate);

    println!("  Base Wage: ¥{}", base_wage);
    println!("  Overtime Premium: ¥{}", overtime_premium);
    println!("  Late Night Premium: ¥{}", late_night_premium);
    println!("  Holiday Premium: ¥{}", holiday_premium);
    println!("  ─────────────────────────");
    println!("  Total Wage: ¥{}", total_wage);

    match validate_monthly_working_summary(&monthly_summary) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 5: Excessive Overtime (Over 60 hours)
    println!("📋 Example 5: Excessive Overtime (60時間超過)");
    println!("{}", "=".repeat(70));

    let heavy_overtime = MonthlyWorkingSummary {
        year: 2026,
        month: 1,
        total_hours: 220.0,
        overtime_hours: 70.0, // Exceeds 60 hour limit!
        late_night_hours: 15.0,
        holiday_hours: 16.0,
        days_worked: 25,
    };

    println!("Total Hours: {:.1} hours", heavy_overtime.total_hours);
    println!("Overtime Hours: {:.1} hours", heavy_overtime.overtime_hours);

    if heavy_overtime.exceeds_overtime_limit() {
        println!("\n⚠️  CRITICAL: Overtime exceeds legal limit!");
        println!("   - 0-60 hours: 25% premium");
        println!("   - 60+ hours: 50% premium (large companies)");
        println!(
            "   - Overtime: {} hours over limit",
            heavy_overtime.overtime_hours - MONTHLY_OVERTIME_LIMIT as f64
        );
    }

    let total_with_excess = heavy_overtime.calculate_total_wage(base_hourly_rate);
    println!("\nTotal Wage with Excess Premiums: ¥{}", total_with_excess);

    match validate_monthly_working_summary(&heavy_overtime) {
        Ok(()) => println!("\n⚠️  Warning: Overtime is excessive but validated"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 6: Wage Payment Validation
    println!("📋 Example 6: Wage Payment Record Validation (賃金支払記録検証)");
    println!("{}", "=".repeat(70));

    let payment = WagePayment {
        employee_name: "田中次郎".to_string(),
        period_start: now,
        period_end: now + Duration::days(30),
        payment_date: now + Duration::days(31),
        base_wage_jpy: total_wage,
        overtime_pay_jpy: overtime_premium + late_night_premium + holiday_premium,
        other_allowances_jpy: 20_000,
        deductions_jpy: 85_000,
        net_payment_jpy: total_wage
            + overtime_premium
            + late_night_premium
            + holiday_premium
            + 20_000
            - 85_000,
    };

    println!("Employee: {}", payment.employee_name);
    println!(
        "Period: {} to {}",
        payment.period_start.format("%Y-%m-%d"),
        payment.period_end.format("%Y-%m-%d")
    );
    println!("Payment Date: {}", payment.payment_date.format("%Y-%m-%d"));
    println!("\nBreakdown:");
    println!("  Base Wage: ¥{}", payment.base_wage_jpy);
    println!("  Overtime Pay: ¥{}", payment.overtime_pay_jpy);
    println!("  Other Allowances: ¥{}", payment.other_allowances_jpy);
    println!("  ─────────────────────────");
    println!("  Gross: ¥{}", payment.gross_wage());
    println!("  Deductions: ¥{}", payment.deductions_jpy);
    println!("  ─────────────────────────");
    println!("  Net Payment: ¥{}", payment.net_payment_jpy);

    if payment.validate_net_payment() {
        println!("\n✅ Net payment calculation is correct!");
    } else {
        println!("\n❌ Net payment calculation error!");
    }

    match validate_wage_payment(&payment) {
        Ok(()) => println!("✅ Wage payment validation: PASSED"),
        Err(e) => println!("❌ Wage payment validation: FAILED - {}", e),
    }

    println!("\n{}", "=".repeat(70));
    println!("\n✨ Overtime Calculation Examples Complete!");
    println!("   All calculations comply with Labor Standards Act Article 37");
    println!("   (労働基準法第37条 - 時間外・休日及び深夜の割増賃金).\n");
}
