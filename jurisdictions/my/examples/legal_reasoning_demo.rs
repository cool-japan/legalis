//! Example: Legal reasoning engine demonstration.

use legalis_my::reasoning::*;

fn main() {
    println!("=== Malaysian Legal Reasoning Engine Demo ===\n");

    let engine = LegalReasoningEngine::new();

    // Employment law scenario
    println!("--- Scenario 1: Employment Law ---");
    let scenario1 = "Employee works 9 hours per day, 50 hours per week, earning RM 1,200/month";
    let analysis1 = engine
        .analyze(scenario1, LegalDomain::Employment)
        .expect("Analysis succeeds");

    print_analysis(&analysis1);

    // Data protection scenario
    println!("\n--- Scenario 2: Data Protection (PDPA) ---");
    let scenario2 =
        "Company processes customer personal data without consent for marketing purposes";
    let analysis2 = engine
        .analyze(scenario2, LegalDomain::DataProtection)
        .expect("Analysis succeeds");

    print_analysis(&analysis2);

    // Islamic law scenario
    println!("\n--- Scenario 3: Islamic Finance ---");
    let scenario3 = "Bank offers home loan with 5% interest per annum";
    let analysis3 = engine
        .analyze(scenario3, LegalDomain::Islamic)
        .expect("Analysis succeeds");

    print_analysis(&analysis3);

    // Competition law scenario
    println!("\n--- Scenario 4: Competition Law ---");
    let scenario4 = "Companies in the same industry agree on price fixing";
    let analysis4 = engine
        .analyze(scenario4, LegalDomain::Competition)
        .expect("Analysis succeeds");

    print_analysis(&analysis4);
}

fn print_analysis(analysis: &LegalAnalysis) {
    println!("Compliance Status: {:?}", analysis.compliance_status);
    println!("Risk Level: {:?}", analysis.risk_level);

    if !analysis.applicable_laws.is_empty() {
        println!("\nApplicable Laws:");
        for law in &analysis.applicable_laws {
            println!("  - {}", law);
        }
    }

    if !analysis.issues.is_empty() {
        println!("\nIssues:");
        for issue in &analysis.issues {
            println!("  - {}", issue);
        }
    }

    if !analysis.recommendations.is_empty() {
        println!("\nRecommendations:");
        for rec in &analysis.recommendations {
            println!("  - {}", rec);
        }
    }
}
