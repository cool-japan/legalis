//! GDPR Article 83 - Administrative Fines Calculator Example
//!
//! Demonstrates fine calculation with aggravating and mitigating factors

use legalis_eu::gdpr::*;

fn main() {
    println!("=== GDPR Article 83 - Administrative Fines Calculator ===\n");

    // Example 1: Upper tier violation (Article 6) - Large company
    println!("Example 1: Upper Tier - Article 6 Violation (Large Corporation)");
    let factors1 = Article83Factors {
        duration_months: Some(18),
        data_subjects_affected: Some(500_000),
        damage_suffered: Some(2_000_000.0),
        intentional: false, // Negligent
        mitigation_actions_taken: vec![
            "Immediately ceased processing".to_string(),
            "Notified all affected data subjects".to_string(),
        ],
        technical_organizational_measures: vec![
            "Implemented new data governance framework".to_string(),
        ],
        previous_violations: vec![],
        cooperated_with_authority: true,
        special_categories_involved: false,
        breach_notification_timely: None,
        certifications: vec![],
        other_aggravating: vec![],
        other_mitigating: vec!["First-time violation".to_string()],
        financial_benefit_gained: None,
    };

    let fine1 = AdministrativeFine::new()
        .with_controller("Tech Giant Corp")
        .with_violation(ViolatedArticle::Article6LawfulBasis)
        .with_turnover_eur(50_000_000_000.0) // €50 billion
        .with_factors(factors1);

    match fine1.calculate_maximum() {
        Ok(calc) => {
            println!("   Violation: No lawful basis for processing user data");
            println!("   Global annual turnover: €50B");
            println!("\n   Fine Calculation:");
            println!("   - Tier: {:?}", calc.tier);
            println!(
                "   - Statutory maximum: €{:.0}M",
                calc.statutory_maximum_eur / 1_000_000.0
            );
            println!(
                "   - Turnover-based maximum (4%): €{:.0}M",
                calc.turnover_based_maximum_eur.unwrap() / 1_000_000.0
            );
            println!(
                "   - Applicable maximum: €{:.0}M",
                calc.maximum_fine_eur / 1_000_000.0
            );
            println!("\n   Severity Assessment:");
            println!("   - Severity score: {:.1}%", calc.severity_score * 100.0);
            println!("   - Suggested fine: {}", calc.format_amount());
            println!("\n   Factors Considered:");
            for factor in &calc.factors_summary {
                println!("      {}", factor);
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Upper tier with aggravating factors
    println!("Example 2: Upper Tier - Article 9 Violation (Bad Actor)");
    let factors2 = Article83Factors {
        duration_months: Some(36), // 3 years
        data_subjects_affected: Some(2_000_000),
        damage_suffered: Some(10_000_000.0),
        intentional: true,                         // INTENTIONAL violation
        mitigation_actions_taken: vec![],          // No mitigation
        technical_organizational_measures: vec![], // No measures
        previous_violations: vec![
            "2021 Article 6 violation (€5M fine)".to_string(),
            "2022 Article 32 security violation".to_string(),
        ],
        cooperated_with_authority: false,  // Refused to cooperate
        special_categories_involved: true, // Health data
        breach_notification_timely: Some(false), // Late notification
        certifications: vec![],
        other_aggravating: vec!["Attempted to conceal violation from authority".to_string()],
        other_mitigating: vec![],
        financial_benefit_gained: Some(50_000_000.0), // €50M profit
    };

    let fine2 = AdministrativeFine::new()
        .with_controller("Bad Actor Inc")
        .with_violation(ViolatedArticle::Article9SpecialCategories)
        .with_turnover_eur(5_000_000_000.0) // €5 billion
        .with_factors(factors2);

    match fine2.calculate_maximum() {
        Ok(calc) => {
            println!("   Violation: Processing health data without Article 9 exception");
            println!("   Global annual turnover: €5B");
            println!("\n   Fine Calculation:");
            println!(
                "   - Maximum fine: €{:.0}M",
                calc.maximum_fine_eur / 1_000_000.0
            );
            println!("\n   ⚠️ SEVERE AGGRAVATING FACTORS:");
            println!("   - Intentional violation (not negligent)");
            println!("   - 3 years duration");
            println!("   - 2M data subjects affected");
            println!("   - Special categories (health data) involved");
            println!("   - Previous violations (2x)");
            println!("   - Refused to cooperate with authority");
            println!("   - Late breach notification");
            println!("   - Financial benefit: €50M");
            println!("\n   Severity Assessment:");
            println!("   - Severity score: {:.1}%", calc.severity_score * 100.0);
            println!(
                "   - Suggested fine: {} ({:.1}% of maximum)",
                calc.format_amount(),
                (calc.suggested_fine_eur / calc.maximum_fine_eur) * 100.0
            );
            println!("\n   ⚠️ This case may warrant maximum or near-maximum fine");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: Lower tier violation
    println!("Example 3: Lower Tier - Article 8 Child Consent (Medium Company)");
    let factors3 = Article83Factors {
        duration_months: Some(6),
        data_subjects_affected: Some(1_500), // Children
        intentional: false,
        mitigation_actions_taken: vec![
            "Immediately implemented age verification".to_string(),
            "Deleted data of affected minors".to_string(),
            "Notified parents/guardians".to_string(),
        ],
        cooperated_with_authority: true,
        ..Default::default()
    };

    let fine3 = AdministrativeFine::new()
        .with_controller("Social Media Startup")
        .with_violation(ViolatedArticle::Article8ChildConsent)
        .with_turnover_eur(50_000_000.0) // €50 million
        .with_factors(factors3);

    match fine3.calculate_maximum() {
        Ok(calc) => {
            println!("   Violation: Processing children's data without parental consent");
            println!("   Global annual turnover: €50M");
            println!("\n   Fine Calculation:");
            println!("   - Tier: {:?} (up to €10M or 2%)", calc.tier);
            println!(
                "   - Statutory maximum: €{:.0}M",
                calc.statutory_maximum_eur / 1_000_000.0
            );
            println!(
                "   - Turnover-based (2%): €{:.2}M",
                calc.turnover_based_maximum_eur.unwrap() / 1_000_000.0
            );
            println!(
                "   - Maximum fine: €{:.0}M",
                calc.maximum_fine_eur / 1_000_000.0
            );
            println!("\n   Mitigating Factors:");
            println!("   ✅ Immediate remediation");
            println!("   ✅ Proactive notification of affected parties");
            println!("   ✅ Full cooperation with authority");
            println!("   ✅ No previous violations");
            println!("\n   Severity score: {:.1}%", calc.severity_score * 100.0);
            println!("   Suggested fine: {}", calc.format_amount());
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Small company with mitigating factors
    println!("Example 4: Upper Tier - Data Subject Rights Violation (Small Business)");
    let factors4 = Article83Factors {
        duration_months: Some(3),
        data_subjects_affected: Some(20),
        intentional: false,
        mitigation_actions_taken: vec![
            "Apologized to affected individuals".to_string(),
            "Provided requested data access".to_string(),
            "Hired DPO to ensure compliance".to_string(),
        ],
        technical_organizational_measures: vec![
            "Implemented DSAR handling procedure".to_string(),
            "Staff training on data subject rights".to_string(),
        ],
        cooperated_with_authority: true,
        certifications: vec!["ISO 27001".to_string()],
        ..Default::default()
    };

    let fine4 = AdministrativeFine::new()
        .with_controller("Small E-commerce Shop")
        .with_violation(ViolatedArticle::DataSubjectRights)
        .with_turnover_eur(500_000.0) // €500K (no turnover-based maximum needed)
        .with_factors(factors4);

    match fine4.calculate_maximum() {
        Ok(calc) => {
            println!("   Violation: Failed to respond to data access requests (Article 15)");
            println!("   Global annual turnover: €500K (small business)");
            println!("\n   Fine Calculation:");
            println!(
                "   - Maximum fine: €{:.0}M (statutory, not turnover)",
                calc.maximum_fine_eur / 1_000_000.0
            );
            println!("\n   Strong Mitigating Factors:");
            for factor in &calc.factors_summary {
                if factor.contains("MITIGATING") {
                    println!("      {}", factor);
                }
            }
            println!("\n   Severity score: {:.1}%", calc.severity_score * 100.0);
            println!("   Suggested fine: {}", calc.format_amount());
            println!("\n   Note: Small businesses may receive proportionally lower fines");
            println!("   considering their economic situation (Recital 150)");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Cross-border transfer violation
    println!("Example 5: Upper Tier - Cross-Border Transfer Violation");
    let factors5 = Article83Factors {
        duration_months: Some(12),
        data_subjects_affected: Some(100_000),
        intentional: false,
        mitigation_actions_taken: vec![
            "Suspended transfers to non-adequate country".to_string(),
            "Implemented SCCs with all processors".to_string(),
        ],
        cooperated_with_authority: true,
        ..Default::default()
    };

    let fine5 = AdministrativeFine::new()
        .with_controller("Cloud Services Provider")
        .with_violation(ViolatedArticle::CrossBorderTransfers)
        .with_turnover_eur(2_000_000_000.0) // €2 billion
        .with_factors(factors5);

    match fine5.calculate_maximum() {
        Ok(calc) => {
            println!("   Violation: Transferred data to US without adequate safeguards");
            println!("   (Post-Schrems II - no Privacy Shield)");
            println!("\n   Fine Calculation:");
            println!(
                "   - Maximum: €{:.0}M (4% of €2B turnover)",
                calc.maximum_fine_eur / 1_000_000.0
            );
            println!("   - Severity: {:.1}%", calc.severity_score * 100.0);
            println!("   - Suggested fine: {}", calc.format_amount());
            println!("\n   Note: Post-Schrems II enforcement has been significant");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Summary
    println!("=== Article 83 Key Principles ===");
    println!("\n1. Two-Tier System:");
    println!("   Lower tier (Article 83(4)): Up to €10M or 2% of global turnover");
    println!("   Upper tier (Article 83(5)): Up to €20M or 4% of global turnover");
    println!("\n2. Effective, Proportionate, and Dissuasive:");
    println!("   Fines must be calibrated to each individual case");
    println!("\n3. Article 83(2) Assessment Factors:");
    println!("   (a) Nature, gravity, duration");
    println!("   (b) Intentional vs negligent");
    println!("   (c) Mitigation actions");
    println!("   (d) Technical/organizational measures (data protection by design)");
    println!("   (e) Previous violations");
    println!("   (f) Cooperation with supervisory authority");
    println!("   (g) Categories of data affected (special categories = aggravating)");
    println!("   (h) Breach notification compliance");
    println!("   (i) Certifications/approved codes of conduct");
    println!("   (j) Financial benefits gained");
    println!("   (k) Other aggravating/mitigating circumstances");
    println!("\n4. Notable Real Fines:");
    println!("   - Amazon (2021): €746M (Luxembourg DPA)");
    println!("   - Meta/WhatsApp (2021): €225M (Irish DPC)");
    println!("   - Google (2019): €50M (French CNIL)");
    println!("\n5. Calculation Note:");
    println!("   This tool provides illustrative estimates based on Article 83(2) factors.");
    println!("   Actual fines are determined by supervisory authorities on a case-by-case basis.");

    println!("\n=== End of Example ===");
}
