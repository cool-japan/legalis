//! Legal Reasoning Engine demonstration.
//!
//! This example demonstrates the automated legal analysis capabilities of the
//! French law reasoning engine, showing how it can analyze contracts, employment
//! contracts, and company formations for compliance violations.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example legal-reasoning-demo --features=serde
//! ```

use chrono::{Duration, Utc};
use legalis_fr::{
    ArticlesOfIncorporation, BreachType, CDDReason, Capital, CompanyType, ComplianceStatus,
    Contract, ContractType, EmploymentContract, EmploymentContractType, FrenchLawAnalyzer,
    Shareholder, ViolationSeverity,
};

fn main() {
    println!("\n🏛️  Legal Reasoning Engine - Comprehensive Demo");
    println!("═══════════════════════════════════════════════════\n");

    // Create unified analyzer
    let analyzer = FrenchLawAnalyzer::new();

    // Demo 1: Valid Contract Analysis
    println!("📋 Demo 1: Valid Contract Analysis");
    println!("───────────────────────────────────────\n");
    analyze_valid_contract(&analyzer);

    // Demo 2: Invalid Contract Analysis
    println!("\n📋 Demo 2: Invalid Contract Analysis (Consent Missing)");
    println!("───────────────────────────────────────────────────────\n");
    analyze_invalid_contract(&analyzer);

    // Demo 3: Contract Breach Analysis
    println!("\n📋 Demo 3: Contract Breach Analysis");
    println!("───────────────────────────────────────\n");
    analyze_contract_breach(&analyzer);

    // Demo 4: Employment Contract Analysis (Working Hours Violation)
    println!("\n👔 Demo 4: Employment Contract - Working Hours Violation");
    println!("─────────────────────────────────────────────────────────\n");
    analyze_employment_working_hours(&analyzer);

    // Demo 5: CDD Duration Violation
    println!("\n👔 Demo 5: CDD Duration Violation (>18 months)");
    println!("─────────────────────────────────────────────────\n");
    analyze_cdd_violation(&analyzer);

    // Demo 6: Company Formation (SA with insufficient capital)
    println!("\n🏢 Demo 6: SA Formation - Capital Violation");
    println!("─────────────────────────────────────────────────\n");
    analyze_sa_capital_violation(&analyzer);

    // Demo 7: Compliant SA Formation
    println!("\n🏢 Demo 7: Compliant SA Formation");
    println!("─────────────────────────────────────────\n");
    analyze_compliant_sa(&analyzer);

    println!("\n═══════════════════════════════════════════════════");
    println!("✅ Legal Reasoning Engine Demo Complete!");
    println!("═══════════════════════════════════════════════════\n");
}

fn analyze_valid_contract(analyzer: &FrenchLawAnalyzer) {
    let contract = Contract::new()
        .with_type(ContractType::Sale {
            price: 50_000,
            subject: "Industrial Machine".to_string(),
        })
        .with_parties(vec!["Buyer Corp".to_string(), "Seller SA".to_string()])
        .with_consent(true)
        .with_good_faith(true);

    println!("Contract Type: Sale (€50,000)");
    println!("Parties: Buyer Corp ↔ Seller SA");
    println!("Consent: Yes, Good Faith: Yes\n");

    match analyzer.contract.analyze_comprehensive(&contract) {
        Ok(analysis) => {
            print_analysis(&analysis);
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn analyze_invalid_contract(analyzer: &FrenchLawAnalyzer) {
    let contract = Contract::new()
        .with_type(ContractType::Service {
            description: "Consulting services".to_string(),
            remuneration: 60_000,
        })
        .with_parties(vec!["Client".to_string(), "Consultant".to_string()])
        .with_consent(false) // Missing consent!
        .with_good_faith(true);

    println!("Contract Type: Service (€60,000)");
    println!("Parties: Client ↔ Consultant");
    println!("❌ Consent: NO (violation of Article 1128)\n");

    match analyzer.contract.analyze_validity(&contract) {
        Ok(violations) => {
            if violations.is_empty() {
                println!("✅ No validity violations found");
            } else {
                println!("⚠️  Found {} validity violation(s):\n", violations.len());
                for (i, violation) in violations.iter().enumerate() {
                    print_violation(i + 1, violation);
                }
            }
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn analyze_contract_breach(analyzer: &FrenchLawAnalyzer) {
    let contract = Contract::new()
        .with_type(ContractType::Sale {
            price: 100_000,
            subject: "Equipment".to_string(),
        })
        .with_parties(vec!["Buyer".to_string(), "Seller".to_string()])
        .with_consent(true)
        .with_good_faith(true)
        .with_breach(BreachType::NonPerformance)
        .with_actual_loss(50_000)
        .with_contract_value(100_000);

    println!("Contract Type: Sale (€100,000)");
    println!("❌ Breach Type: Non-Performance");
    println!("Actual Loss: €50,000\n");

    match analyzer.contract.analyze_breach(&contract) {
        Ok(violations) => {
            if violations.is_empty() {
                println!("✅ No breach violations detected");
            } else {
                println!("⚠️  Found {} breach violation(s):\n", violations.len());
                for (i, violation) in violations.iter().enumerate() {
                    print_violation(i + 1, violation);
                }

                // Calculate damages
                if let Ok(Some(damages)) = analyzer.contract.calculate_damages(&contract) {
                    println!("\n💶 Estimated Damages: €{}", damages);
                }
            }
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn analyze_employment_working_hours(analyzer: &FrenchLawAnalyzer) {
    let mut contract = EmploymentContract::new(
        EmploymentContractType::CDI,
        "Jean Dupont".to_string(),
        "Tech Solutions SA".to_string(),
    );

    // Override working hours to exceed legal limit
    contract.working_hours.weekly_hours = 50.0; // Exceeds 35 hours

    println!("Contract Type: CDI (Permanent)");
    println!("Employee: Jean Dupont");
    println!("Employer: Tech Solutions SA");
    println!("❌ Weekly Hours: 50 (exceeds 35-hour legal limit)\n");

    match analyzer.labor.analyze_working_hours(&contract) {
        Ok(violations) => {
            if violations.is_empty() {
                println!("✅ No working hours violations");
            } else {
                println!(
                    "⚠️  Found {} working hours violation(s):\n",
                    violations.len()
                );
                for (i, violation) in violations.iter().enumerate() {
                    print_violation(i + 1, violation);
                }
            }

            // Calculate overtime premium
            if let Some(premium) = analyzer
                .labor
                .calculate_overtime_premium(50.0, contract.hourly_rate as u64)
            {
                println!("\n💶 Weekly Overtime Premium Owed: €{}", premium);
            }
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn analyze_cdd_violation(analyzer: &FrenchLawAnalyzer) {
    let end_date = Utc::now().naive_utc().date() + Duration::days(600); // ~20 months
    let contract = EmploymentContract::new(
        EmploymentContractType::CDD {
            duration_months: 20, // Exceeds 18-month maximum!
            reason: CDDReason::TemporaryIncreaseActivity,
            end_date,
        },
        "Marie Martin".to_string(),
        "Retail Corp".to_string(),
    );

    println!("Contract Type: CDD (Fixed-term)");
    println!("Employee: Marie Martin");
    println!("Employer: Retail Corp");
    println!("❌ Duration: 20 months (exceeds 18-month maximum - Article L1242-8)\n");

    match analyzer.labor.analyze_cdd_validity(&contract) {
        Ok(violations) => {
            if violations.is_empty() {
                println!("✅ CDD compliant with regulations");
            } else {
                println!("⚠️  Found {} CDD violation(s):\n", violations.len());
                for (i, violation) in violations.iter().enumerate() {
                    print_violation(i + 1, violation);
                }
            }
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn analyze_sa_capital_violation(analyzer: &FrenchLawAnalyzer) {
    let capital = Capital::new(30_000); // Below €37,000 minimum
    let shareholder = Shareholder::new("Founder A".to_string(), 1000, 30_000);

    let articles =
        ArticlesOfIncorporation::new("Innovation Tech SA".to_string(), CompanyType::SA, capital)
            .with_shareholder(shareholder);

    println!("Company: Innovation Tech SA");
    println!("Type: SA (Société Anonyme)");
    println!("❌ Capital: €30,000 (below €37,000 minimum - Article L225-1)\n");

    match analyzer.company.analyze_sa_formation(&articles) {
        Ok(violations) => {
            if violations.is_empty() {
                println!("✅ SA formation compliant");
            } else {
                println!(
                    "⚠️  Found {} SA formation violation(s):\n",
                    violations.len()
                );
                for (i, violation) in violations.iter().enumerate() {
                    print_violation(i + 1, violation);
                }
            }
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn analyze_compliant_sa(analyzer: &FrenchLawAnalyzer) {
    let capital = Capital::new(50_000); // Above €37,000 minimum
    let shareholder1 = Shareholder::new("Founder A".to_string(), 600, 30_000);
    let shareholder2 = Shareholder::new("Investor B".to_string(), 400, 20_000);

    let articles =
        ArticlesOfIncorporation::new("Tech Solutions SA".to_string(), CompanyType::SA, capital)
            .with_shareholder(shareholder1)
            .with_shareholder(shareholder2)
            .with_business_purpose("Software development and consulting".to_string())
            .with_head_office("15 Rue de la République, 75001 Paris".to_string());

    println!("Company: Tech Solutions SA");
    println!("Type: SA (Société Anonyme)");
    println!("✅ Capital: €50,000 (compliant)");
    println!("Shareholders: 2 (60% Founder A, 40% Investor B)\n");

    match analyzer.company.analyze_comprehensive(&articles) {
        Ok(analysis) => {
            print_analysis(&analysis);
        }
        Err(e) => println!("❌ Analysis Error: {}", e),
    }
}

fn print_analysis(analysis: &legalis_fr::LegalAnalysis) {
    println!("📊 Analysis Results:");
    println!("   Entity Type: {:?}", analysis.entity_type);
    println!(
        "   Applicable Statutes: {} article(s)",
        analysis.applicable_statutes.len()
    );

    match &analysis.compliance_status {
        ComplianceStatus::Compliant => {
            println!("   ✅ Compliance Status: COMPLIANT");
        }
        ComplianceStatus::MinorIssues(issues) => {
            println!("   ⚠️  Compliance Status: Minor Issues ({})", issues.len());
        }
        ComplianceStatus::MajorViolations(violations) => {
            println!(
                "   ❌ Compliance Status: Major Violations ({})",
                violations.len()
            );
        }
        ComplianceStatus::Invalid => {
            println!("   ❌ Compliance Status: INVALID");
        }
    }

    println!("   Confidence: {:.1}%", analysis.confidence * 100.0);
    println!("   Risk Level: {:?}", analysis.legal_opinion.risk_level);

    if !analysis.violations.is_empty() {
        println!("\n⚠️  Violations Found: {}", analysis.violations.len());
        for (i, violation) in analysis.violations.iter().enumerate() {
            print_violation(i + 1, violation);
        }
    }

    if !analysis.legal_opinion.summary_en.is_empty() {
        println!("\n📝 Legal Opinion:");
        println!("   EN: {}", analysis.legal_opinion.summary_en);
        if !analysis.legal_opinion.recommendations_en.is_empty() {
            println!("\n   Recommendations:");
            for rec in &analysis.legal_opinion.recommendations_en {
                println!("   • {}", rec);
            }
        }
    }
}

fn print_violation(index: usize, violation: &legalis_fr::Violation) {
    let severity_emoji = match violation.severity {
        ViolationSeverity::Critical => "🔴",
        ViolationSeverity::High => "🟠",
        ViolationSeverity::Medium => "🟡",
        ViolationSeverity::Low => "🟢",
    };

    println!(
        "   {} Violation #{}: {}",
        severity_emoji, index, violation.article_id
    );
    println!("      Severity: {:?}", violation.severity);
    println!("      EN: {}", violation.description_en);

    if !violation.remedies.is_empty() {
        println!("      Remedies: {} available", violation.remedies.len());
        for (i, remedy) in violation.remedies.iter().enumerate() {
            println!(
                "         {}. {:?} - {}",
                i + 1,
                remedy.remedy_type,
                remedy.description_en
            );
            if let Some(damages) = remedy.estimated_damages {
                println!("            💶 Estimated Damages: €{}", damages);
            }
        }
    }
    println!();
}
