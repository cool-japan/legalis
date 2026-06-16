//! Zero-hours contract examples.
//!
//! Demonstrates the treatment of zero-hours contracts under UK employment law,
//! using the `legalis_uk::employment` library APIs.
//!
//! A zero-hours contract is a contract under which the employer is not obliged to
//! provide any minimum hours of work. Such contracts are lawful, but since the
//! Small Business, Enterprise and Employment Act 2015 (which inserted s.27A into
//! the Employment Rights Act 1996), any **exclusivity clause** — a term
//! prohibiting the worker from working for another employer — is **unenforceable**.
//! The Exclusivity Terms in Zero Hours Contracts (Redress) Regulations 2015 give
//! the worker the right not to be unfairly dismissed or subjected to a detriment
//! for working elsewhere.
//!
//! This example builds a zero-hours `EmploymentContract` both with and without an
//! unlawful exclusivity clause and runs it through `validate_employment_contract`.

use chrono::NaiveDate;
use legalis_uk::employment::{
    ContractType, Employee, Employer, EmploymentContract, NoticePeriod, Salary, WorkingHours,
    validate_contract_type, validate_employment_contract,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UK Zero-Hours Contract Validation ===");
    println!("(ERA 1996 s.27A; Exclusivity Terms Regulations 2015)\n");

    lawful_zero_hours()?;
    unlawful_exclusivity()?;
    raw_contract_type_check();

    Ok(())
}

/// Build a base employee/employer/contract scaffold shared by the examples.
fn build_contract(
    exclusivity_clause: bool,
) -> Result<EmploymentContract, Box<dyn std::error::Error>> {
    let dob = NaiveDate::from_ymd_opt(1995, 6, 15).ok_or("invalid date of birth")?;
    let start = NaiveDate::from_ymd_opt(2024, 1, 8).ok_or("invalid start date")?;

    let employee = Employee {
        name: "Alex Morgan".to_string(),
        date_of_birth: dob,
        address: "12 Bridge Street, Manchester, M1 1AA".to_string(),
        national_insurance_number: Some("QQ123456C".to_string()),
    };

    let employer = Employer {
        name: "FlexiStaff Hospitality Ltd".to_string(),
        address: "1 Market Square, Manchester, M2 2BB".to_string(),
        employee_count: Some(120),
    };

    let contract = EmploymentContract::builder()
        .with_employee(employee)
        .with_employer(employer)
        .with_contract_type(ContractType::ZeroHours { exclusivity_clause })
        .with_start_date(start)
        .with_salary(Salary {
            gross_annual_gbp: 0.0,
            payment_frequency: legalis_uk::employment::PaymentFrequency::Weekly,
            payment_day: 5,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 0,
            days_per_week: 0,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_duties("Bar and waiting duties as required.".to_string())
        .with_notice_period(NoticePeriod {
            employer_notice_weeks: 1,
            employee_notice_weeks: 1,
        })
        .with_written_particulars(true)
        .build();

    Ok(contract)
}

/// A lawful zero-hours contract with no exclusivity clause.
fn lawful_zero_hours() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Lawful zero-hours contract (no exclusivity)");
    println!("-----------------------------------------------------\n");

    let contract = build_contract(false)?;
    println!("  Contract type: {:?}", contract.contract_type);
    match validate_employment_contract(&contract) {
        Ok(()) => println!("  validate_employment_contract: OK — contract is valid\n"),
        Err(e) => println!("  validate_employment_contract: rejected -> {e}\n"),
    }
    Ok(())
}

/// A zero-hours contract carrying an unenforceable exclusivity clause.
fn unlawful_exclusivity() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Zero-hours contract with an exclusivity clause (unlawful)");
    println!("-------------------------------------------------------------------\n");

    let contract = build_contract(true)?;
    println!("  Contract type: {:?}", contract.contract_type);
    match validate_employment_contract(&contract) {
        Ok(()) => println!("  validate_employment_contract: OK (unexpected)\n"),
        Err(e) => {
            println!("  validate_employment_contract: rejected as expected");
            println!("    -> {e}");
            println!("    The exclusivity clause is unenforceable under ERA 1996 s.27A.\n");
        }
    }
    Ok(())
}

/// Demonstrate the standalone contract-type check on the exclusivity ban.
fn raw_contract_type_check() {
    println!("Example 3: Direct contract-type validation");
    println!("------------------------------------------\n");

    let lawful = ContractType::ZeroHours {
        exclusivity_clause: false,
    };
    let unlawful = ContractType::ZeroHours {
        exclusivity_clause: true,
    };

    println!(
        "  ZeroHours {{ exclusivity_clause: false }} -> {:?}",
        validate_contract_type(&lawful).map(|()| "valid")
    );
    println!(
        "  ZeroHours {{ exclusivity_clause: true  }} -> {}",
        match validate_contract_type(&unlawful) {
            Ok(()) => "valid".to_string(),
            Err(e) => format!("rejected: {e}"),
        }
    );
}
