//! Example: Employment contract creation and validation.

use legalis_my::common::format_myr_cents;
use legalis_my::employment_law::*;

fn main() {
    println!("=== Malaysian Employment Contract Demo ===\n");

    // Create employment contract
    let contract = EmploymentContract::builder()
        .employer("Tech Innovations Sdn Bhd")
        .employee("Ahmad bin Ali", "850123-01-5678")
        .job_title("Software Engineer")
        .monthly_salary_sen(500_000) // RM 5,000
        .working_hours(WorkingHours::standard())
        .leave_entitlement(LeaveEntitlement::calculate(3)) // 3 years of service
        .notice_period_days(30)
        .build()
        .expect("Valid contract");

    println!("Employer: {}", contract.employer);
    println!("Employee: {} ({})", contract.employee, contract.employee_ic);
    println!("Job Title: {}", contract.job_title);
    println!("Salary: {}", format_myr_cents(contract.monthly_salary_sen));
    println!(
        "Working Hours: {}h/day, {}h/week",
        contract.working_hours.hours_per_day, contract.working_hours.hours_per_week
    );
    println!(
        "Annual Leave: {} days",
        contract.leave_entitlement.annual_leave_days
    );
    println!("Notice Period: {} days", contract.notice_period_days);

    // Calculate EPF contribution
    println!("\n--- EPF Contribution (Age 30) ---");
    let epf = contract.calculate_epf(30);
    let breakdown = epf.calculate();

    println!(
        "Employer ({}%): {}",
        epf.employer_rate,
        format_myr_cents(breakdown.employer_amount_sen)
    );
    println!(
        "Employee ({}%): {}",
        epf.employee_rate,
        format_myr_cents(breakdown.employee_amount_sen)
    );
    println!(
        "Total EPF: {}",
        format_myr_cents(breakdown.total_amount_sen)
    );

    // Validate contract
    println!("\n--- Contract Validation ---");
    match contract.validate() {
        Ok(report) => {
            if report.valid {
                println!("✅ Employment contract is compliant with Employment Act 1955");
            } else {
                println!("❌ Contract has compliance issues:");
                for issue in report.issues {
                    println!("  - {}", issue);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Invalid contract example
    println!("\n--- Invalid Contract Example (Excessive Working Hours) ---");
    let invalid_hours = WorkingHours {
        hours_per_day: 10,
        hours_per_week: 60,
        shift_work: false,
    };

    let invalid_contract = EmploymentContract::builder()
        .employer("Bad Employer Sdn Bhd")
        .employee("Worker", "900101-01-1234")
        .job_title("Worker")
        .monthly_salary_sen(150_000) // RM 1,500 (minimum wage)
        .working_hours(invalid_hours)
        .leave_entitlement(LeaveEntitlement::calculate(0))
        .notice_period_days(14)
        .build()
        .expect("Contract built");

    match invalid_contract.validate() {
        Ok(report) => {
            println!("Valid: {}", report.valid);
            for issue in report.issues {
                println!("  - {}", issue);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
