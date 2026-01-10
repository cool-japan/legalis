//! Employment contract example - CDI and CDD validation
//!
//! Demonstrates French labor law for employment contracts, working hours, and dismissals

use chrono::{Duration, Utc};
use legalis_fr::labor::*;

fn main() {
    println!("=== French Labor Law - Employment Contracts Example ===\n");
    println!("Code du travail - Contrat de travail (Employment contracts)\n");

    // Example 1: Valid CDI (permanent contract)
    println!("📋 Example 1: Valid CDI Contract (Article L1221-1)");
    println!("────────────────────────────────────────────────");

    let cdi_contract = EmploymentContract::new(
        EmploymentContractType::CDI,
        "Marie Dupont".to_string(),
        "TechCorp SA".to_string(),
    )
    .with_hourly_rate(15.50) // €15.50/hour (above SMIC)
    .with_working_hours(WorkingHours {
        weekly_hours: 35.0, // Legal 35-hour week
        daily_hours: Some(7.0),
    })
    .with_start_date(Utc::now().naive_utc().date());

    match validate_employment_contract(&cdi_contract, true) {
        Ok(_) => {
            println!("✅ CDI contract is valid!");
            println!("   Employee: {}", cdi_contract.employee);
            println!("   Employer: {}", cdi_contract.employer);
            println!("   Type: CDI (Contrat à Durée Indéterminée)");
            println!("   Hourly rate: €{}", cdi_contract.hourly_rate);
            println!(
                "   Weekly hours: {} hours",
                cdi_contract.working_hours.weekly_hours
            );
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!();

    // Example 2: Valid CDD (fixed-term contract)
    println!("📄 Example 2: Valid CDD Contract (Article L1242-2)");
    println!("────────────────────────────────────────────────");

    let cdd_contract_type = EmploymentContractType::CDD {
        duration_months: 12,
        reason: CDDReason::ReplacementAbsentEmployee,
        end_date: (Utc::now() + Duration::days(365)).naive_utc().date(),
    };

    let cdd_contract = EmploymentContract::new(
        cdd_contract_type.clone(),
        "Pierre Martin".to_string(),
        "HealthCare SAS".to_string(),
    )
    .with_hourly_rate(14.00)
    .with_working_hours(WorkingHours {
        weekly_hours: 35.0,
        daily_hours: Some(7.0),
    });

    match validate_cdd(&cdd_contract_type, true) {
        Ok(_) => {
            println!("✅ CDD contract is valid!");
            println!("   Duration: 12 months (max 18 months - Article L1242-8)");
            println!("   Reason: Replacement of absent employee");
            println!("   Written contract: Yes (mandatory - Article L1242-12)");
            println!("   Hourly rate: €{}", cdd_contract.hourly_rate);
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!();

    // Example 3: Invalid CDD - too long
    println!("❌ Example 3: Invalid CDD - Duration Exceeded");
    println!("─────────────────────────────────────────────");

    let invalid_cdd = EmploymentContractType::CDD {
        duration_months: 24, // Exceeds 18 months!
        reason: CDDReason::TemporaryIncreaseActivity,
        end_date: (Utc::now() + Duration::days(730)).naive_utc().date(),
    };

    match validate_cdd(&invalid_cdd, true) {
        Ok(_) => println!("✅ Valid"),
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            println!("\nExplanation: CDD cannot exceed 18 months (Article L1242-8)");
            println!("Consequence: Contract would be requalified as CDI");
        }
    }

    println!();

    // Example 4: Working hours with overtime
    println!("⏰ Example 4: Working Hours with Overtime (Article L3121-27)");
    println!("────────────────────────────────────────────────────────");

    let contract_with_overtime = EmploymentContract::new(
        EmploymentContractType::CDI,
        "Jean Lefebvre".to_string(),
        "Factory SARL".to_string(),
    )
    .with_hourly_rate(13.00)
    .with_working_hours(WorkingHours {
        weekly_hours: 40.0, // 5 hours overtime
        daily_hours: Some(8.0),
    });

    match validate_working_hours(&contract_with_overtime.working_hours) {
        Ok(_) => {
            println!("✅ Working hours are within legal limits!");
            println!("   Weekly hours: 40 hours");
            println!("   Legal duration: 35 hours (Article L3121-27)");
            println!("   Overtime: 5 hours/week");

            let base_rate = contract_with_overtime.hourly_rate;
            let overtime = 5.0;
            let overtime_pay = WorkingHours::calculate_overtime_premium(overtime, base_rate);

            println!("\n   💰 Overtime calculation:");
            println!(
                "   First 5 hours at +25%: {} × €{} × 1.25 = €{:.2}",
                overtime, base_rate, overtime_pay
            );
            println!(
                "   Total weekly: (35 × €{}) + €{:.2} = €{:.2}",
                base_rate,
                overtime_pay,
                (35.0 * base_rate) + overtime_pay
            );
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!();

    // Example 5: Excessive working hours
    println!("❌ Example 5: Excessive Working Hours");
    println!("─────────────────────────────────────");

    let excessive_hours = WorkingHours {
        weekly_hours: 55.0, // Exceeds 48 hours!
        daily_hours: Some(8.0),
    };

    match validate_working_hours(&excessive_hours) {
        Ok(_) => println!("✅ Valid"),
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            println!("\nExplanation: Weekly hours cannot exceed 48 (Article L3121-20)");
            println!("Average over 12 weeks must not exceed 44 hours");
        }
    }

    println!();

    // Example 6: Trial period validation
    println!("🔍 Example 6: Trial Period Validation (Article L1221-19)");
    println!("────────────────────────────────────────────────────");

    let trial_scenarios = vec![
        (TrialPeriodCategory::WorkersEmployees, 2, "✅"),
        (TrialPeriodCategory::SupervisorsTechnicians, 3, "✅"),
        (TrialPeriodCategory::Executives, 4, "✅"),
        (TrialPeriodCategory::Executives, 5, "❌"),
    ];

    for (category, months, _expected) in trial_scenarios {
        let result = validate_trial_period(category, months);
        let status = if result.is_ok() { "✅" } else { "❌" };
        let max = match category {
            TrialPeriodCategory::WorkersEmployees => 2,
            TrialPeriodCategory::SupervisorsTechnicians => 3,
            TrialPeriodCategory::Executives => 4,
        };
        println!("{} {:?}: {} months (max {})", status, category, months, max);
    }

    println!();

    // Example 7: Dismissal validation
    println!("⚖️  Example 7: Dismissal Validation (Article L1232-1)");
    println!("────────────────────────────────────────────────");

    let valid_dismissal = DismissalType::Personal {
        cause: PersonalCause::SimpleFault,
        serious_misconduct: false,
    };

    match validate_dismissal(&valid_dismissal, true) {
        Ok(_) => {
            println!("✅ Dismissal is procedurally valid!");
            println!("   Type: Personal dismissal (Licenciement personnel)");
            println!("   Cause: Misconduct");
            println!("   Interview held: Yes (mandatory - Article L1232-2)");
            println!("   Notice period: Required (Article L1234-1)");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!();

    // Example 8: Invalid dismissal - no interview
    println!("❌ Example 8: Invalid Dismissal - Missing Interview");
    println!("─────────────────────────────────────────────────");

    match validate_dismissal(&valid_dismissal, false) {
        Ok(_) => println!("✅ Valid"),
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            println!("\nExplanation: Pre-dismissal interview is mandatory (Article L1232-2)");
            println!("Consequence: Procedural irregularity → Damages (~1 month salary)");
        }
    }

    println!();

    // Example 9: Economic dismissal
    println!("💼 Example 9: Economic Dismissal (Article L1233-3)");
    println!("────────────────────────────────────────────────");

    let economic_dismissal = DismissalType::Economic {
        economic_difficulties: true,
        job_eliminated: true,
        affected_count: 1,
    };

    match validate_dismissal(&economic_dismissal, true) {
        Ok(_) => {
            println!("✅ Economic dismissal is valid!");
            println!("   Economic difficulties: Yes");
            println!("   Job eliminated: Yes");
            println!("   Interview held: Yes");
            println!("\n   Required steps:");
            println!("   1. Prove economic difficulties");
            println!("   2. Demonstrate job elimination necessity");
            println!("   3. Search for reclassification positions");
            println!("   4. Consult employee representatives");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!();

    // Example 10: Notice period requirements
    println!("📅 Example 10: Notice Period Requirements (Article L1234-1)");
    println!("────────────────────────────────────────────────────────");

    let notice_scenarios = vec![
        (3, 0, "✅", "< 6 months: no legal minimum"),
        (12, 1, "✅", "6-24 months: 1 month"),
        (36, 2, "✅", "≥ 24 months: 2 months"),
        (36, 1, "❌", "Insufficient notice for seniority"),
    ];

    for (seniority, notice, _expected, explanation) in notice_scenarios {
        let result = validate_notice_period(seniority, notice);
        let status = if result.is_ok() { "✅" } else { "❌" };
        println!(
            "{} {} months seniority, {} months notice: {}",
            status, seniority, notice, explanation
        );
    }

    println!();

    // Example 11: Minimum wage validation
    println!("💶 Example 11: Minimum Wage Validation (SMIC)");
    println!("───────────────────────────────────────────");

    println!("Current SMIC: €{}/hour (2024)", SMIC_HOURLY);

    let wage_scenarios = vec![(15.00, "✅"), (SMIC_HOURLY, "✅"), (10.00, "❌")];

    for (wage, _expected) in wage_scenarios {
        let result = validate_minimum_wage(wage);
        let status = if result.is_ok() { "✅" } else { "❌" };
        let comparison = if wage >= SMIC_HOURLY {
            "above"
        } else {
            "BELOW"
        };
        println!("{} €{}/hour ({} SMIC)", status, wage, comparison);
    }

    println!();

    // Summary
    println!("════════════════════════════════════════════════════════");
    println!("📚 Summary - French Labor Law Key Rules");
    println!("════════════════════════════════════════════════════════");
    println!();
    println!("📝 Contract Formation:");
    println!("   • CDI: No written form required (but recommended)");
    println!("   • CDD: Written mandatory (Article L1242-12)");
    println!("   • CDD max duration: 18 months (Article L1242-8)");
    println!();
    println!("⏰ Working Hours:");
    println!("   • Legal duration: 35 hours/week (Article L3121-27)");
    println!("   • Maximum daily: 10 hours (Article L3121-18)");
    println!("   • Maximum weekly: 48 hours (Article L3121-20)");
    println!("   • Overtime premium: +25% (first 8h), +50% (beyond)");
    println!();
    println!("⚖️  Dismissal:");
    println!("   • Requires real and serious cause (Article L1232-1)");
    println!("   • Pre-dismissal interview mandatory (Article L1232-2)");
    println!("   • Notice period: 1-2 months based on seniority (Article L1234-1)");
    println!("   • Economic dismissal: strict requirements (Article L1233-3)");
    println!();
    println!("💶 Minimum Wage:");
    println!("   • SMIC: €{}/hour (2024)", SMIC_HOURLY);
    println!();
    println!("🇯🇵 Comparison with Japanese Labor Law:");
    println!("  ┌────────────────────┬────────────────┬──────────────────┐");
    println!("  │ Aspect             │ 🇫🇷 France     │ 🇯🇵 Japan        │");
    println!("  ├────────────────────┼────────────────┼──────────────────┤");
    println!("  │ Legal work week    │ 35 hours       │ 40 hours         │");
    println!("  │ Max weekly hours   │ 48 hours       │ No absolute max  │");
    println!("  │ Fixed-term max     │ 18 months      │ 5 years          │");
    println!("  │ Dismissal cause    │ Real & serious │ Objective reason │");
    println!("  │ Notice period      │ 1-2 months     │ 30 days          │");
    println!("  └────────────────────┴────────────────┴──────────────────┘");
    println!();
    println!("Both France and Japan have strong employee protections,");
    println!("but France's 35-hour week and 18-month CDD limit are stricter.");
    println!();
}
