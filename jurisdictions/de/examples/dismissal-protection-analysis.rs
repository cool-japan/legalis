//! Dismissal Protection Analysis Example
//!
//! Demonstrates validation of dismissals under German labor law,
//! covering social justification (§1 KSchG), written form (§623 BGB),
//! notice periods (§622 BGB), and works council consultation (§102 BetrVG).

use chrono::NaiveDate;
use legalis_de::arbeitsrecht::*;

fn main() {
    println!("=== German Dismissal Protection Analysis ===\n");
    println!("Kündigungsschutzanalyse nach deutschem Arbeitsrecht\n");

    // Example 1: Valid Ordinary Dismissal - Operational Reasons
    println!(
        "📋 Example 1: Valid Ordinary Dismissal (Ordentliche Kündigung) - Operational Grounds"
    );

    let valid_dismissal = Dismissal {
        dismissed_by: DismissalParty::Employer,
        employee_name: "Max Mustermann".to_string(),
        dismissal_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        effective_date: NaiveDate::from_ymd_opt(2024, 4, 26).unwrap(), // 8 weeks later
        dismissal_type: DismissalType::Ordinary,
        grounds: DismissalGrounds::Operational {
            description: "Department closure due to business restructuring. Position eliminated as part of company-wide cost reduction measures.".to_string(),
        },
        notice_period_weeks: 8, // §622 BGB - 8 weeks for long employment
        written: true, // §623 BGB requires written form
        works_council_consulted: true, // §102 BetrVG
    };

    let company_size = CompanySize::Large; // 20+ employees - full KSchG protection

    match validate_dismissal(&valid_dismissal, company_size) {
        Ok(()) => {
            println!("✅ Dismissal Valid!");
            println!("   Employee: {}", valid_dismissal.employee_name);
            println!("   Type: Ordinary (Ordentliche Kündigung)");
            println!("   Grounds: Operational (Betriebsbedingt)");
            println!(
                "   Notice Period: {} weeks",
                valid_dismissal.notice_period_weeks
            );
            println!("   Written: ✅ (§623 BGB)");
            println!("   Works Council: ✅ Consulted (§102 BetrVG)");
            println!("\n   Legal Basis:");
            println!("   • §1 KSchG - Social justification (operational grounds)");
            println!("   • §622 BGB - Notice period compliance");
            println!("   • §623 BGB - Written form requirement");
            println!("   • §102 BetrVG - Works council consultation");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 2: Invalid - Dismissal Not Written
    println!("\n📋 Example 2: Invalid Dismissal - Missing Written Form");

    let mut invalid_not_written = valid_dismissal.clone();
    invalid_not_written.written = false; // Violates §623 BGB

    match validate_dismissal(&invalid_not_written, company_size) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §623 BGB requires dismissals in written form");
            println!("   Note: Oral dismissals are void (nichtig)");
        }
    }

    // Example 3: Invalid - Works Council Not Consulted
    println!("\n📋 Example 3: Invalid Dismissal - Works Council Not Consulted");

    let mut invalid_no_consultation = valid_dismissal.clone();
    invalid_no_consultation.works_council_consulted = false; // Violates §102 BetrVG

    match validate_dismissal(&invalid_no_consultation, company_size) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §102 BetrVG requires works council consultation");
            println!("   Note: Dismissal without consultation is invalid (unwirksam)");
        }
    }

    // Example 4: Invalid - Insufficient Notice Period
    println!("\n📋 Example 4: Invalid Dismissal - Insufficient Notice Period");

    let mut invalid_notice = valid_dismissal.clone();
    invalid_notice.notice_period_weeks = 2; // Below §622 BGB minimum of 4 weeks
    invalid_notice.effective_date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(); // Only 2 weeks

    match validate_dismissal(&invalid_notice, company_size) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §622 BGB requires minimum 4 weeks notice");
            println!("   Note: Notice period increases with tenure (up to 7 months)");
        }
    }

    // Example 5: Valid Conduct-Related Dismissal
    println!("\n📋 Example 5: Valid Conduct-Related Dismissal (Verhaltensbedingte Kündigung)");

    let conduct_dismissal = Dismissal {
        dismissed_by: DismissalParty::Employer,
        employee_name: "Erika Schmidt".to_string(),
        dismissal_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        effective_date: NaiveDate::from_ymd_opt(2024, 3, 29).unwrap(), // 4 weeks
        dismissal_type: DismissalType::Ordinary,
        grounds: DismissalGrounds::Conduct {
            description: "Repeated tardiness despite written warnings. Employee failed to arrive on time 15 times in past 3 months after two formal warnings.".to_string(),
            warnings: 2, // Prior warnings required
        },
        notice_period_weeks: 4, // Standard minimum
        written: true,
        works_council_consulted: true,
    };

    match validate_dismissal(&conduct_dismissal, CompanySize::Medium) {
        Ok(()) => {
            println!("✅ Conduct-Related Dismissal Valid!");
            println!("   Employee: {}", conduct_dismissal.employee_name);
            println!("   Type: Conduct-Related (Verhaltensbedingt)");
            println!("   Warnings: 2 prior warnings issued");
            println!(
                "   Notice Period: {} weeks",
                conduct_dismissal.notice_period_weeks
            );
            println!("\n   Legal Basis:");
            println!("   • §1 Abs. 2 KSchG - Conduct-related grounds");
            println!("   • Prior warnings required (Abmahnung principle)");
            println!("   • Progressive discipline followed");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 6: Invalid Conduct Dismissal - No Prior Warnings
    println!("\n📋 Example 6: Invalid Conduct Dismissal - No Prior Warnings");

    let mut invalid_no_warnings = conduct_dismissal.clone();
    invalid_no_warnings.grounds = DismissalGrounds::Conduct {
        description: "Repeated tardiness".to_string(),
        warnings: 0, // No warnings - generally insufficient
    };

    match validate_dismissal(&invalid_no_warnings, CompanySize::Medium) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: German labor law requires prior warnings (Abmahnung)");
            println!(
                "   Exception: Only severe misconduct allows immediate dismissal without warnings"
            );
        }
    }

    // Example 7: Valid Extraordinary Dismissal
    println!("\n📋 Example 7: Valid Extraordinary Dismissal (Außerordentliche Kündigung)");

    let extraordinary_dismissal = Dismissal {
        dismissed_by: DismissalParty::Employer,
        employee_name: "Thomas Wagner".to_string(),
        dismissal_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        effective_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(), // Immediate
        dismissal_type: DismissalType::Extraordinary,
        grounds: DismissalGrounds::ExtraordinaryCause {
            description: "Employee embezzled €50,000 from company accounts. Criminal investigation ongoing. Immediate termination for cause under §626 BGB due to severe breach of trust.".to_string(),
        },
        notice_period_weeks: 0, // No notice for extraordinary dismissal
        written: true,
        works_council_consulted: true,
    };

    match validate_dismissal(&extraordinary_dismissal, CompanySize::Large) {
        Ok(()) => {
            println!("✅ Extraordinary Dismissal Valid!");
            println!("   Employee: {}", extraordinary_dismissal.employee_name);
            println!("   Type: Extraordinary (Außerordentliche Kündigung)");
            println!("   Notice Period: 0 weeks (immediate)");
            println!("   Grounds: Severe breach of trust (embezzlement)");
            println!("\n   Legal Basis:");
            println!("   • §626 BGB - Extraordinary dismissal for good cause");
            println!("   • Must be issued within 2 weeks of knowledge");
            println!("   • Continuation of employment unreasonable");
            println!("   • No notice period required");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 8: Invalid Extraordinary Dismissal - Has Notice Period
    println!("\n📋 Example 8: Invalid Extraordinary Dismissal - Includes Notice Period");

    let mut invalid_extraordinary = extraordinary_dismissal.clone();
    invalid_extraordinary.notice_period_weeks = 4; // Extraordinary dismissals have no notice
    invalid_extraordinary.effective_date = NaiveDate::from_ymd_opt(2024, 3, 29).unwrap();

    match validate_dismissal(&invalid_extraordinary, CompanySize::Large) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!(
                "   Legal Basis: §626 BGB - Extraordinary dismissals are immediate (no notice)"
            );
        }
    }

    // Example 9: Valid Personal-Reasons Dismissal
    println!("\n📋 Example 9: Valid Personal-Reasons Dismissal (Personenbedingte Kündigung)");

    let personal_dismissal = Dismissal {
        dismissed_by: DismissalParty::Employer,
        employee_name: "Julia Becker".to_string(),
        dismissal_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        effective_date: NaiveDate::from_ymd_opt(2024, 3, 29).unwrap(),
        dismissal_type: DismissalType::Ordinary,
        grounds: DismissalGrounds::Personal {
            description: "Employee has lost driver's license permanently due to medical reasons. Position requires valid commercial driver's license. No alternative positions available in company.".to_string(),
        },
        notice_period_weeks: 4,
        written: true,
        works_council_consulted: true,
    };

    match validate_dismissal(&personal_dismissal, CompanySize::Small) {
        Ok(()) => {
            println!("✅ Personal-Reasons Dismissal Valid!");
            println!("   Employee: {}", personal_dismissal.employee_name);
            println!("   Type: Personal Reasons (Personenbedingt)");
            println!("   Grounds: Loss of required qualification (driver's license)");
            println!("\n   Legal Basis:");
            println!("   • §1 Abs. 2 KSchG - Personal grounds");
            println!("   • Inability to perform contracted duties");
            println!("   • No alternative positions available");
            println!("   • Future prognosis negative");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 10: Small Company - No KSchG Protection
    println!("\n📋 Example 10: Small Company Without KSchG Protection");

    let small_company_dismissal = Dismissal {
        dismissed_by: DismissalParty::Employer,
        employee_name: "Peter Klein".to_string(),
        dismissal_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        effective_date: NaiveDate::from_ymd_opt(2024, 3, 29).unwrap(),
        dismissal_type: DismissalType::Ordinary,
        grounds: DismissalGrounds::Operational {
            description: "Company downsizing".to_string(),
        },
        notice_period_weeks: 4,
        written: true,
        works_council_consulted: false, // No works council in small company
    };

    let small_company = CompanySize::Small; // < 10 employees - no KSchG

    match validate_dismissal(&small_company_dismissal, small_company) {
        Ok(()) => {
            println!("✅ Dismissal Valid for Small Company!");
            println!("   Employee: {}", small_company_dismissal.employee_name);
            println!("   Company Size: Small (< 10 employees)");
            println!("   KSchG Protection: ❌ Not applicable");
            println!("   Works Council: ❌ Not required (company too small)");
            println!("\n   Note: Companies with fewer than 10 employees are exempt from KSchG");
            println!("   However, dismissal still requires:");
            println!("   • Written form (§623 BGB)");
            println!("   • Proper notice period (§622 BGB)");
            println!("   • Good faith principle (§242 BGB)");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    println!("\n=== Summary ===");
    println!("✅ Dismissal protection validation covers:");
    println!("   • Social justification requirement (§1 KSchG)");
    println!("   • Three types of grounds: Conduct, Personal, Operational");
    println!("   • Written form requirement (§623 BGB - mandatory)");
    println!("   • Notice period validation (§622 BGB - minimum 4 weeks)");
    println!("   • Works council consultation (§102 BetrVG)");
    println!("   • Extraordinary dismissal (§626 BGB - immediate)");
    println!("   • Company size thresholds (KSchG applies 10+ employees)");
    println!("\n📚 Key German Dismissal Protection Statutes:");
    println!("   • KSchG - Protection Against Dismissal Act");
    println!("   • BGB §622 - Notice Periods");
    println!("   • BGB §623 - Written Form Requirement");
    println!("   • BGB §626 - Extraordinary Dismissal for Good Cause");
    println!("   • BetrVG §102 - Works Council Consultation");
    println!("\n⚖️  Three Types of Dismissal Grounds:");
    println!("   1. Conduct-Related (Verhaltensbedingt) - Employee's behavior");
    println!("   2. Personal Reasons (Personenbedingt) - Employee's inability");
    println!("   3. Operational Reasons (Betriebsbedingt) - Business needs");
    println!("\n⏱️  Notice Periods (§622 BGB):");
    println!("   • Minimum: 4 weeks to 15th or end of month");
    println!("   • Increases with tenure: 1 month (2 years), 2 months (5 years)");
    println!("   • Maximum: 7 months (20 years of service)");
}
