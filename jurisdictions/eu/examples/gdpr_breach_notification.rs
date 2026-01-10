//! GDPR Data Breach Notification Example
//!
//! This example demonstrates GDPR Articles 33 & 34 - Data breach notification requirements.

use chrono::{Duration, Utc};
use legalis_eu::gdpr::*;

fn main() {
    println!("=== GDPR Data Breach Notification Example ===\n");

    // Example 1: Breach within 72-hour window
    println!("Example 1: Breach Discovered 48 Hours Ago (Within Deadline)");
    let recent_breach = DataBreach::new()
        .with_controller("Online Retailer Ltd")
        .with_breach_category(BreachCategory::ConfidentialityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(48))
        .with_affected_data_subjects(500)
        .with_severity(BreachSeverity::Medium)
        .with_description("Unauthorized access to customer database via SQL injection")
        .add_mitigation_measure("Patched SQL injection vulnerability")
        .add_mitigation_measure("Reset affected user passwords")
        .add_mitigation_measure("Enhanced database access logging");

    match recent_breach.validate_notification_requirements() {
        Ok(requirements) => {
            println!(
                "   Hours since discovery: {}",
                requirements.hours_since_discovery
            );
            println!(
                "   Supervisory authority deadline: {}",
                requirements.supervisory_authority_deadline
            );

            if requirements.supervisory_authority_notification_required {
                if requirements.supervisory_authority_deadline_passed {
                    println!("   ❌ 72-hour deadline EXCEEDED - Non-compliant!");
                } else {
                    let remaining = 72 - requirements.hours_since_discovery;
                    println!(
                        "   ✅ {} hours remaining to notify supervisory authority",
                        remaining
                    );
                }
            }

            if requirements.data_subject_notification_required {
                println!("   ⚠️  Must notify data subjects (high risk breach)");
            } else {
                println!("   ℹ️  Data subject notification not required (medium risk)");
            }

            println!("   Compliance: {:?}", requirements.compliance_status);
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Deadline exceeded
    println!("Example 2: Breach Discovered 80 Hours Ago (Deadline Exceeded)");
    let late_breach = DataBreach::new()
        .with_controller("Financial Services Inc")
        .with_breach_category(BreachCategory::ConfidentialityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(80))
        .with_affected_data_subjects(10000)
        .with_severity(BreachSeverity::High)
        .with_description("Ransomware attack encrypting customer financial records");

    match late_breach.validate_notification_requirements() {
        Ok(requirements) => {
            println!(
                "   Hours since discovery: {}",
                requirements.hours_since_discovery
            );

            if requirements.supervisory_authority_deadline_passed {
                println!("   ❌ 72-hour notification deadline EXCEEDED!");
                println!("   ⚠️  Late notification may result in Article 83 fines");
                println!("   ⚠️  Must still notify immediately + explain delay");
            }

            if requirements.data_subject_notification_required {
                println!("   ❌ Must notify affected data subjects (HIGH RISK)");
                println!(
                    "   Affected: {} individuals",
                    late_breach.affected_count.unwrap()
                );
            }

            match requirements.compliance_status {
                BreachComplianceStatus::NonCompliant { ref violation } => {
                    println!("   Status: NON-COMPLIANT");
                    println!("   Violation: {}", violation);
                }
                BreachComplianceStatus::Compliant => println!("   Status: COMPLIANT"),
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: Critical breach (immediate notification)
    println!("Example 3: Critical Breach (Just Discovered)");
    let critical_breach = DataBreach::new()
        .with_controller("Healthcare Provider")
        .with_breach_category(BreachCategory::ConfidentialityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(2))
        .with_affected_data_subjects(50000)
        .with_severity(BreachSeverity::Critical)
        .with_description("Massive data leak - patient medical records exposed on dark web")
        .with_affected_data_categories(vec![
            "Names".to_string(),
            "Addresses".to_string(),
            "Social security numbers".to_string(),
            "Medical diagnoses".to_string(),
            "Treatment records".to_string(),
        ]);

    match critical_breach.validate_notification_requirements() {
        Ok(requirements) => {
            println!("   🚨 CRITICAL BREACH DETECTED");
            println!(
                "   Hours since discovery: {}",
                requirements.hours_since_discovery
            );
            println!(
                "   Affected: {} individuals",
                critical_breach.affected_count.unwrap()
            );

            let remaining = 72 - requirements.hours_since_discovery;
            println!("   ⏰ {} hours remaining for SA notification", remaining);

            if requirements.data_subject_notification_required {
                println!("\n   IMMEDIATE ACTIONS REQUIRED:");
                println!("   1. ✅ Notify supervisory authority within 72 hours");
                println!("   2. ✅ Notify all affected data subjects WITHOUT DELAY");
                println!("   3. ✅ Document breach in Article 33(5) register");
                println!("   4. ✅ Implement containment measures");
                println!("\n   Special categories involved:");
                for category in &critical_breach.affected_data_categories {
                    println!("      - {}", category);
                }
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Low severity breach
    println!("Example 4: Low Severity Breach (Unlikely to Result in Risk)");
    let low_breach = DataBreach::new()
        .with_controller("Small Business")
        .with_breach_category(BreachCategory::AvailabilityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(24))
        .with_affected_data_subjects(10)
        .with_severity(BreachSeverity::Low)
        .with_description("Temporary server outage - no data accessed or modified")
        .add_mitigation_measure("Server restored from backup")
        .add_mitigation_measure("No evidence of unauthorized access");

    match low_breach.validate_notification_requirements() {
        Ok(requirements) => {
            println!(
                "   Hours since discovery: {}",
                requirements.hours_since_discovery
            );

            if !requirements.data_subject_notification_required {
                println!("   ℹ️  Data subject notification NOT required (low risk)");
            }

            if requirements.supervisory_authority_notification_required {
                println!("   ⚠️  SA notification still required (Article 33)");
                println!("      Even if unlikely to result in risk, document in internal register");
            }

            println!("   Note: If unlikely to result in risk, SA notification may not be needed");
            println!("         However, must still document breach internally (Article 33(5))");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Summary
    println!("=== Key Takeaways ===");
    println!("1. Article 33(1): Notify supervisory authority within 72 hours");
    println!("2. Article 34: Notify data subjects if HIGH RISK to rights/freedoms");
    println!("3. Article 33(5): Document ALL breaches in internal register");
    println!("4. Late notification increases Article 83 fine risk");
    println!("5. Mitigation measures can reduce severity assessment");

    println!("\n=== End of Example ===");
}
