//! Contract Risk Analyzer Example (契約リスク分析例)
//!
//! This example demonstrates comprehensive contract risk analysis,
//! detecting legal violations and unfair clauses.
//!
//! # Usage
//! ```bash
//! cargo run --example contract-risk-analyzer
//! ```

use legalis_jp::risk_analysis::*;

fn main() {
    println!("=== Contract Risk Analyzer Example ===\n");
    println!("契約リスク分析例 - Contract Risk Analysis System\n");

    // ========================================================================
    // PART 1: CONSUMER CONTRACT ANALYSIS
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: CONSUMER CONTRACT ANALYSIS (消費者契約分析)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut consumer_contract =
        ContractDocument::new("オンライン通販サービス利用規約", ContractType::Consumer);

    consumer_contract.add_clause(
        "第3条（免責事項）",
        "当社は、本サービスの利用に起因して発生した一切の損害について、\
         いかなる責任も負いません。",
    );

    consumer_contract.add_clause(
        "第5条（契約の変更）",
        "当社は、利用者の同意を得ることなく、本規約を一方的に変更できるものとします。",
    );

    consumer_contract.add_clause(
        "第8条（解約）",
        "利用者が解約する場合、契約金額の100%を違約金として支払うものとします。",
    );

    consumer_contract.add_clause(
        "第12条（管轄裁判所）",
        "本契約に関する一切の紛争については、当社本店所在地を管轄する裁判所を専属的合意管轄裁判所とします。",
    );

    println!("📋 Analyzing: {}\n", consumer_contract.title);
    println!(
        "Contract Type: {}",
        consumer_contract.contract_type.japanese_name()
    );
    println!("Total Clauses: {}\n", consumer_contract.clause_count());

    let detector = RiskDetector::new();
    let consumer_report = detector.analyze(&consumer_contract).unwrap();

    print_report(&consumer_report);

    // ========================================================================
    // PART 2: EMPLOYMENT CONTRACT ANALYSIS
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: EMPLOYMENT CONTRACT ANALYSIS (雇用契約分析)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut employment_contract =
        ContractDocument::new("正社員雇用契約書", ContractType::Employment);

    employment_contract.add_clause(
        "第7条（競業避止義務）",
        "従業員は、退職後永久に、当社と競合する全国の全ての事業に従事してはならない。",
    );

    employment_contract.add_clause(
        "第9条（違約金）",
        "従業員が契約に違反した場合、退職時には違約金を定め、\
         契約金額の50%を支払うものとする。",
    );

    employment_contract.add_clause(
        "第15条（個人情報）",
        "会社は従業員の個人情報を業務上必要な範囲で第三者に提供する。",
    );

    println!("📋 Analyzing: {}\n", employment_contract.title);
    println!(
        "Contract Type: {}",
        employment_contract.contract_type.japanese_name()
    );
    println!("Total Clauses: {}\n", employment_contract.clause_count());

    let employment_report = detector.analyze(&employment_contract).unwrap();

    print_report(&employment_report);

    // ========================================================================
    // PART 3: CLEAN CONTRACT (NO RISKS)
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 3: COMPLIANT CONTRACT ANALYSIS (適法契約分析)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut clean_contract = ContractDocument::new("業務委託契約書", ContractType::Service);

    clean_contract.add_clause(
        "第1条（契約の目的）",
        "本契約は、委託者と受託者が相互の信頼関係に基づき、\
         業務委託に関する事項を定めるものとする。",
    );

    clean_contract.add_clause(
        "第3条（誠実義務）",
        "双方は、本契約を誠実に履行するものとする。",
    );

    clean_contract.add_clause(
        "第7条（秘密保持）",
        "受託者は、業務遂行上知り得た秘密情報を第三者に開示してはならない。\
         ただし、委託者の事前の書面による同意を得た場合はこの限りではない。",
    );

    clean_contract.add_clause(
        "第12条（解除）",
        "双方は、相手方が本契約に違反し、相当の期間を定めて催告したにもかかわらず、\
         是正されない場合、本契約を解除することができる。",
    );

    println!("📋 Analyzing: {}\n", clean_contract.title);
    println!(
        "Contract Type: {}",
        clean_contract.contract_type.japanese_name()
    );
    println!("Total Clauses: {}\n", clean_contract.clause_count());

    let clean_report = detector.analyze(&clean_contract).unwrap();

    print_report(&clean_report);

    // ========================================================================
    // PART 4: QUICK ANALYSIS
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 4: QUICK TEXT ANALYSIS (クイック分析)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let problematic_text = "当社は一切の責任を負わず、解約時には全額を違約金として\
                            請求します。また、適宜相当の措置を講じることができ、\
                            本規約は当社の裁量により一方的に変更できるものとします。";

    println!("Analyzing text snippet...\n");
    println!("Text: \"{}\"\n", problematic_text);

    let quick_report = quick_analyze(
        "Quick Analysis Sample",
        ContractType::Consumer,
        problematic_text,
    )
    .unwrap();

    println!("Quick Analysis Results:");
    println!("  Risk Score: {}/100", quick_report.overall_risk_score);
    println!("  Findings: {}", quick_report.findings.len());
    println!("  Critical: {}", quick_report.critical_count());
    println!("  High: {}", quick_report.high_count());
    println!("  Medium: {}", quick_report.medium_count());
    println!("  Low: {}", quick_report.low_count());

    if quick_report.has_serious_risks() {
        println!("\n  ⚠️  WARNING: Serious legal risks detected!");
    }

    // ========================================================================
    // SUMMARY
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("SUMMARY (まとめ)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Contracts Analyzed: 4");
    println!("\n1. Consumer Contract (消費者契約):");
    println!("   Risk Score: {}/100", consumer_report.overall_risk_score);
    println!(
        "   Status: {}",
        if consumer_report.has_serious_risks() {
            "❌ SERIOUS RISKS"
        } else {
            "✅ ACCEPTABLE"
        }
    );

    println!("\n2. Employment Contract (雇用契約):");
    println!(
        "   Risk Score: {}/100",
        employment_report.overall_risk_score
    );
    println!(
        "   Status: {}",
        if employment_report.has_serious_risks() {
            "❌ SERIOUS RISKS"
        } else {
            "✅ ACCEPTABLE"
        }
    );

    println!("\n3. Service Contract (業務委託契約):");
    println!("   Risk Score: {}/100", clean_report.overall_risk_score);
    println!(
        "   Status: {}",
        if clean_report.has_serious_risks() {
            "❌ SERIOUS RISKS"
        } else {
            "✅ ACCEPTABLE"
        }
    );

    println!("\n4. Quick Analysis:");
    println!("   Risk Score: {}/100", quick_report.overall_risk_score);
    println!(
        "   Status: {}",
        if quick_report.has_serious_risks() {
            "❌ SERIOUS RISKS"
        } else {
            "✅ ACCEPTABLE"
        }
    );

    println!("\n✨ Risk Analysis Complete!\n");
    println!("Features Demonstrated:");
    println!("  ✅ Multi-contract type analysis (Consumer, Employment, Service)");
    println!("  ✅ Automated risk detection with severity classification");
    println!("  ✅ Legal reference tracking (Labor Standards Act, Consumer Contract Act)");
    println!("  ✅ Comprehensive reporting with recommendations");
    println!("  ✅ Quick analysis for text snippets");
    println!("  ✅ Clean contract validation (zero-risk detection)\n");
}

/// Prints a detailed risk analysis report
fn print_report(report: &RiskAnalysisReport) {
    println!("═══════════════════════════════════════════════════════════════════");
    println!("RISK ANALYSIS REPORT");
    println!("═══════════════════════════════════════════════════════════════════\n");

    println!("Document: {}", report.document_title);
    println!("Contract Type: {}", report.contract_type.japanese_name());
    println!(
        "Analysis Date: {}\n",
        report.analysis_date.format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Overall Score
    let risk_level = if report.overall_risk_score >= 75 {
        "CRITICAL"
    } else if report.overall_risk_score >= 50 {
        "HIGH"
    } else if report.overall_risk_score >= 25 {
        "MEDIUM"
    } else {
        "LOW"
    };

    println!("┌─────────────────────────────────────┐");
    println!(
        "│ OVERALL RISK SCORE: {}/100 ({:^8}) │",
        report.overall_risk_score, risk_level
    );
    println!("└─────────────────────────────────────┘\n");

    // Severity Breakdown
    println!("Severity Breakdown:");
    println!(
        "  {} Critical: {}",
        RiskSeverity::Critical.emoji(),
        report.critical_count()
    );
    println!(
        "  {} High:     {}",
        RiskSeverity::High.emoji(),
        report.high_count()
    );
    println!(
        "  {} Medium:   {}",
        RiskSeverity::Medium.emoji(),
        report.medium_count()
    );
    println!(
        "  {} Low:      {}",
        RiskSeverity::Low.emoji(),
        report.low_count()
    );
    println!();

    // Summary
    println!("Summary:");
    println!("  {}\n", report.summary.replace('\n', "\n  "));

    // Findings
    if !report.findings.is_empty() {
        println!("Detailed Findings ({}):", report.findings.len());
        println!();

        for (i, finding) in report.findings.iter().enumerate() {
            println!(
                "{}. {} {} [{}]",
                i + 1,
                finding.severity.emoji(),
                finding.severity.english_name(),
                finding.category.japanese_name()
            );
            println!("   Location: {}", finding.location);
            println!(
                "   Issue: {}",
                wrap_long_text(&finding.issue_description, 70, "   ")
            );

            if let Some(legal_ref) = &finding.legal_reference {
                println!("   Legal Reference: {}", legal_ref);
            }

            println!(
                "   Problematic Text: \"{}\"",
                truncate(&finding.problematic_text, 60)
            );
            println!(
                "   Recommendation: {}",
                wrap_long_text(&finding.recommendation, 70, "   ")
            );
            println!("   Confidence: {:.0}%", finding.confidence * 100.0);
            println!();
        }
    } else {
        println!("✅ No risks detected. Contract appears compliant.\n");
    }

    // Recommendations
    if !report.recommendations.is_empty() {
        println!("Recommendations:");
        for (i, rec) in report.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, wrap_long_text(rec, 68, "     "));
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════════\n");
}

/// Truncates text to max length
fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max - 3).collect();
        format!("{}...", truncated)
    }
}

/// Wraps long text
fn wrap_long_text(text: &str, width: usize, indent: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut result = Vec::new();

    for line in lines {
        if line.chars().count() <= width {
            result.push(line.to_string());
        } else {
            // Simple word wrap (not perfect for Japanese)
            result.push(line.to_string());
        }
    }

    result.join(&format!("\n{}", indent))
}
