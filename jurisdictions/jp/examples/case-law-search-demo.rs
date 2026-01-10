//! Case Law Database Search Demo (判例データベース検索デモ)
//!
//! This example demonstrates the case law database system with search,
//! citation formatting, and database operations.
//!
//! # Usage
//! ```bash
//! cargo run --example case-law-search-demo
//! ```

use chrono::{TimeZone, Utc};
use legalis_jp::case_law::*;

fn main() {
    println!("=== Case Law Database Search Demo ===\n");
    println!("判例データベース検索デモ - Case Law Database System\n");

    // ========================================================================
    // PART 1: BUILDING A CASE LAW DATABASE
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: BUILDING A CASE LAW DATABASE");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut db = InMemoryCaseDatabase::new();

    // Example 1: Supreme Court Tort Case
    println!("📚 Adding Case 1: Supreme Court Tort Decision\n");

    let mut metadata1 = CaseMetadata::new(
        "令和2年(受)第1234号",
        Utc.with_ymd_and_hms(2021, 3, 15, 0, 0, 0).unwrap(),
        Court::new(CourtLevel::Supreme)
            .with_location("Tokyo")
            .with_division("First Petty Bench"),
        LegalArea::Civil,
        CaseOutcome::AppealGranted,
    );
    metadata1.add_keyword("不法行為");
    metadata1.add_keyword("損害賠償");
    metadata1.add_keyword("因果関係");
    metadata1.add_cited_statute("民法第709条");
    metadata1.add_cited_statute("民法第710条");

    let mut decision1 = CourtDecision::new(
        "supreme-2021-001",
        metadata1,
        "不法行為に基づく損害賠償請求事件。被告の過失により原告が負傷した事案について、\
         因果関係の立証責任および損害額の認定が争点となった。",
    );

    decision1.add_party(Party {
        party_type: "原告・被上告人".to_string(),
        name: "X".to_string(),
        representative: Some("弁護士 A".to_string()),
    });

    decision1.add_party(Party {
        party_type: "被告・上告人".to_string(),
        name: "Y株式会社".to_string(),
        representative: Some("弁護士 B".to_string()),
    });

    decision1.add_holding(Holding {
        principle: "不法行為における因果関係の立証は、通常人が疑いを差し挟まない程度の\
                    高度の蓋然性の証明で足りる。"
            .to_string(),
        reasoning: "因果関係の存否は、自然科学的証明ではなく、経験則に照らし、\
                    全証拠を総合的に検討して判断すべきである。"
            .to_string(),
        related_statutes: vec!["民法第709条".to_string()],
        is_leading_case: true,
    });

    decision1 = decision1.with_source_url("https://courts.go.jp/example/2021-001");

    db.add_case(decision1).unwrap();

    println!("✅ Added: Supreme Court tort case with leading precedent");
    println!("   Case Number: 令和2年(受)第1234号");
    println!("   Keywords: 不法行為, 損害賠償, 因果関係");
    println!("   Statutes: 民法第709条, 民法第710条\n");

    // Example 2: High Court Contract Case
    println!("📚 Adding Case 2: High Court Contract Breach Decision\n");

    let mut metadata2 = CaseMetadata::new(
        "令和3年(ネ)第5678号",
        Utc.with_ymd_and_hms(2022, 6, 20, 0, 0, 0).unwrap(),
        Court::new(CourtLevel::High)
            .with_location("Osaka")
            .with_division("Civil Division"),
        LegalArea::Commercial,
        CaseOutcome::PartiallyGranted,
    );
    metadata2.add_keyword("契約違反");
    metadata2.add_keyword("債務不履行");
    metadata2.add_keyword("履行遅滞");
    metadata2.add_cited_statute("民法第415条");
    metadata2.add_cited_statute("民法第541条");

    let decision2 = CourtDecision::new(
        "high-2022-002",
        metadata2,
        "売買契約における売主の債務不履行に基づく損害賠償請求事件。\
         履行遅滞による損害の範囲が争点。",
    )
    .with_source_url("https://courts.go.jp/example/2022-002");

    db.add_case(decision2).unwrap();

    println!("✅ Added: High Court contract breach case");
    println!("   Case Number: 令和3年(ネ)第5678号");
    println!("   Keywords: 契約違反, 債務不履行, 履行遅滞\n");

    // Example 3: District Court Labor Case
    println!("📚 Adding Case 3: District Court Labor Dispute\n");

    let mut metadata3 = CaseMetadata::new(
        "令和4年(ワ)第9012号",
        Utc.with_ymd_and_hms(2023, 9, 10, 0, 0, 0).unwrap(),
        Court::new(CourtLevel::District)
            .with_location("Tokyo")
            .with_division("Labor Division"),
        LegalArea::Labor,
        CaseOutcome::PlaintiffWins,
    );
    metadata3.add_keyword("不当解雇");
    metadata3.add_keyword("解雇権濫用");
    metadata3.add_keyword("労働契約");
    metadata3.add_cited_statute("労働契約法第16条");
    metadata3.add_cited_statute("労働基準法第20条");

    let decision3 = CourtDecision::new(
        "district-2023-003",
        metadata3,
        "解雇無効確認請求事件。使用者による解雇が解雇権の濫用に該当するか否かが争点。",
    );

    db.add_case(decision3).unwrap();

    println!("✅ Added: District Court labor case");
    println!("   Case Number: 令和4年(ワ)第9012号");
    println!("   Keywords: 不当解雇, 解雇権濫用\n");

    println!("Database now contains {} cases\n", db.count());
    println!("{}\n", "=".repeat(70));

    // ========================================================================
    // PART 2: SEARCHING THE DATABASE
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: SEARCHING THE DATABASE");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let engine = CaseLawSearchEngine::new(db);

    // Search 1: Keyword Search
    println!("🔍 Search 1: Keyword Search for \"不法行為\"");
    println!("{}", "=".repeat(70));

    let query1 = CaseSearchQuery::new()
        .with_keyword("不法行為")
        .with_limit(10);

    match engine.search(&query1) {
        Ok(results) => {
            println!("Found {} case(s)\n", results.len());
            for (i, result) in results.iter().enumerate() {
                println!(
                    "Result {}: {} (Relevance: {:.2})",
                    i + 1,
                    result.decision.metadata.case_number,
                    result.relevance_score
                );
                println!("  Court: {}", result.decision.metadata.court.full_name());
                println!(
                    "  Date: {}",
                    result.decision.metadata.decision_date.format("%Y-%m-%d")
                );
                println!("  Summary: {}", truncate(&result.decision.summary, 80));
                println!("  Matching Keywords: {:?}", result.matching_keywords);
                println!();
            }
        }
        Err(e) => println!("Search error: {}\n", e),
    }

    println!("{}\n", "=".repeat(70));

    // Search 2: Filtered Search
    println!("🔍 Search 2: Supreme Court Cases Only");
    println!("{}", "=".repeat(70));

    let query2 = CaseSearchQuery::new()
        .with_court_level(CourtLevel::Supreme)
        .with_limit(10);

    match engine.search(&query2) {
        Ok(results) => {
            println!("Found {} Supreme Court case(s)\n", results.len());
            for result in &results {
                println!(
                    "- {} ({})",
                    result.decision.metadata.case_number,
                    result.decision.metadata.legal_area.japanese_name()
                );
                println!(
                    "  Precedent Weight: {} (0=highest)",
                    result.decision.precedent_weight()
                );
            }
        }
        Err(e) => println!("Search error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Search 3: Legal Area Filter
    println!("🔍 Search 3: All Civil Law Cases");
    println!("{}", "=".repeat(70));

    let query3 = CaseSearchQuery::new()
        .with_legal_area(LegalArea::Civil)
        .with_limit(10);

    match engine.search(&query3) {
        Ok(results) => {
            println!("Found {} civil law case(s)\n", results.len());
            for result in &results {
                println!("- {}", result.decision.metadata.case_number);
                println!("  Outcome: {:?}", result.decision.metadata.outcome);
            }
        }
        Err(e) => println!("Search error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // ========================================================================
    // PART 3: CITATION FORMATTING
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 3: CITATION FORMATTING (引用形式)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Get a case for citation examples
    let case = engine.get("supreme-2021-001").unwrap();

    println!("📝 Citation Examples for: {}\n", case.metadata.case_number);

    // Short citation
    let short = CitationFormatter::format(&case, CitationStyle::Short).unwrap();
    println!("Short Citation (短縮引用):");
    println!("  {}\n", short);

    // Standard citation
    let standard = CitationFormatter::format(&case, CitationStyle::Standard).unwrap();
    println!("Standard Citation (標準引用):");
    println!("  {}\n", standard);

    // Full citation with URL
    let full = CitationFormatter::format(&case, CitationStyle::Full).unwrap();
    println!("Full Citation (完全引用):");
    println!("  {}\n", full);

    // Blue Book style
    let bluebook = CitationFormatter::format(&case, CitationStyle::BlueBook).unwrap();
    println!("Blue Book Style:");
    println!("  {}\n", bluebook);

    println!("{}\n", "=".repeat(70));

    // ========================================================================
    // PART 4: CASE DETAILS AND HOLDINGS
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 4: DETAILED CASE INFORMATION");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📋 Case Details: {}\n", case.metadata.case_number);

    println!("Court Information:");
    println!("  Full Name: {}", case.metadata.court.full_name());
    println!("  Level: {}", case.metadata.court.level.japanese_name());
    println!("  Precedent Weight: {}", case.precedent_weight());
    println!();

    println!("Case Information:");
    println!("  Legal Area: {}", case.metadata.legal_area.japanese_name());
    println!("  Outcome: {:?}", case.metadata.outcome);
    println!(
        "  Decision Date: {}",
        case.metadata.decision_date.format("%Y年%m月%d日")
    );
    println!("  Decision Year: {}", case.decision_year());
    println!();

    println!("Parties:");
    for party in &case.parties {
        println!("  - {} ({})", party.name, party.party_type);
        if let Some(rep) = &party.representative {
            println!("    Representative: {}", rep);
        }
    }
    println!();

    println!("Summary:");
    println!("  {}\n", wrap_text(&case.summary, 70));

    println!("Legal Holdings ({}):", case.holdings.len());
    for (i, holding) in case.holdings.iter().enumerate() {
        println!("  Holding {}:", i + 1);
        println!("    Principle: {}", wrap_text(&holding.principle, 66));
        println!("    Reasoning: {}", wrap_text(&holding.reasoning, 66));
        println!("    Leading Case: {}", holding.is_leading_case);
        if !holding.related_statutes.is_empty() {
            println!(
                "    Related Statutes: {}",
                holding.related_statutes.join(", ")
            );
        }
        println!();
    }

    println!("Cited Statutes:");
    for statute in &case.metadata.cited_statutes {
        println!("  - {}", statute);
    }
    println!();

    println!("Keywords:");
    for keyword in &case.metadata.keywords {
        println!("  - {}", keyword);
    }
    println!();

    if let Some(url) = &case.source_url {
        println!("Official Source: {}", url);
        println!();
    }

    println!("{}\n", "=".repeat(70));

    // ========================================================================
    // SUMMARY
    // ========================================================================

    let stats = engine.stats();

    println!("✨ Case Law Database Demo Complete!\n");
    println!("Database Statistics:");
    println!("  Total Cases: {}", stats.total_cases);
    println!("\nFeatures Demonstrated:");
    println!("  ✅ Case creation with comprehensive metadata");
    println!("  ✅ Multi-level court system (Supreme, High, District)");
    println!("  ✅ Keyword-based search with relevance scoring");
    println!("  ✅ Court level and legal area filtering");
    println!("  ✅ Multiple citation formats (Japanese & English)");
    println!("  ✅ Holdings and legal principles tracking");
    println!("  ✅ Statute citation tracking");
    println!("  ✅ Party information management\n");
}

/// Truncates text to specified length with ellipsis (UTF-8 safe)
fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

/// Wraps text to specified width (simple implementation)
fn wrap_text(text: &str, width: usize) -> String {
    if text.len() <= width {
        text.to_string()
    } else {
        let mut result = String::new();
        let mut current_len = 0;

        for word in text.split_whitespace() {
            if current_len + word.len() + 1 > width && current_len > 0 {
                result.push_str("\n    ");
                current_len = 4;
            }

            if current_len > 4 {
                result.push(' ');
                current_len += 1;
            }

            result.push_str(word);
            current_len += word.len();
        }

        result
    }
}
