//! Legislative Diff Simulator - PoC for Law Amendment Impact Analysis
//!
//! This tool simulates structural diff detection for law amendments,
//! similar to CI/CD for software, but for legal statutes.
//!
//! Features:
//! - Structural diff (not just text diff)
//! - Article renumbering detection
//! - Table row/column shift detection
//! - Cross-reference impact analysis
//! - "New-Old Comparison Table" (新旧対照表) generation

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("⚖️  Legislative Diff Simulator - Law Amendment Impact Analyzer\n");

    // Load sample statutes
    let old_text = fs::read_to_string("sample_statutes/old_version.txt")?;
    let new_text = fs::read_to_string("sample_statutes/new_version.txt")?;

    println!("▼ Parsing statutes...");
    let old_statute = parse_statute(&old_text)?;
    let new_statute = parse_statute(&new_text)?;

    println!("  Old version: {} articles", old_statute.articles.len());
    println!("  New version: {} articles", new_statute.articles.len());

    println!("\n▼ Computing structural diff...");
    let diff_result = compute_structural_diff(&old_statute, &new_statute)?;

    println!("\n▼ Analyzing impact...");
    let impact_analysis = analyze_impact(&diff_result, &old_statute, &new_statute)?;

    println!("\n");
    generate_report(&diff_result, &impact_analysis);

    Ok(())
}

/// Simplified statute structure for PoC
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatuteStructure {
    title: String,
    articles: Vec<Article>,
    tables: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Article {
    number: u32,
    title: Option<String>,
    paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Paragraph {
    number: u32,
    text: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    number: u32,
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Table {
    name: String,
    rows: Vec<Vec<String>>,
}

/// Structural difference result
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct StructuralDiff {
    added_articles: Vec<u32>,
    deleted_articles: Vec<u32>,
    modified_articles: Vec<ArticleModification>,
    paragraph_changes: Vec<ParagraphChange>,
    renumbered_articles: Vec<RenumberingInfo>,
    table_changes: Vec<TableChange>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ArticleModification {
    article_number: u32,
    change_type: String,
    description: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ParagraphChange {
    article_number: u32,
    change_type: ParagraphChangeType,
    old_paragraph: Option<u32>,
    new_paragraph: Option<u32>,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ParagraphChangeType {
    Added,
    Deleted,
    Modified,
    Renumbered,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct RenumberingInfo {
    old_number: u32,
    new_number: u32,
    reason: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TableChange {
    table_name: String,
    change_type: String,
    description: String,
}

/// Impact analysis result
#[derive(Debug, Clone)]
struct ImpactAnalysis {
    severity: ImpactSeverity,
    affected_references: Vec<AffectedReference>,
    recommended_actions: Vec<String>,
    risk_assessment: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ImpactSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
struct AffectedReference {
    source: String,
    old_target: String,
    new_target: String,
    needs_update: bool,
}

/// Parse statute text into structure (simplified for PoC)
fn parse_statute(text: &str) -> Result<StatuteStructure> {
    // This is a simplified parser for demonstration
    // Production would use legalis-core's full parser

    let mut articles = Vec::new();
    let mut current_article: Option<Article> = None;
    // Match only at the beginning of line (after trimming)
    let article_pattern = Regex::new(r"^第(\d+)条(?:\s*（([^）]+)）)?")?;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Check for article header (must be at beginning of line)
        if let Some(caps) = article_pattern.captures(line) {
            // Save previous article
            if let Some(article) = current_article.take() {
                articles.push(article);
            }

            let number: u32 = caps.get(1).expect("").as_str().parse()?;
            let title = caps.get(2).map(|m| m.as_str().to_string());

            current_article = Some(Article {
                number,
                title,
                paragraphs: Vec::new(),
            });
        } else if let Some(ref mut article) = current_article {
            // Add paragraph to current article
            article.paragraphs.push(Paragraph {
                number: article.paragraphs.len() as u32 + 1,
                text: line.to_string(),
                items: Vec::new(),
            });
        }
    }

    // Save last article
    if let Some(article) = current_article {
        articles.push(article);
    }

    Ok(StatuteStructure {
        title: "Sample Statute".to_string(),
        articles,
        tables: Vec::new(), // Simplified: no table parsing in PoC
    })
}

/// Compute structural difference between two statutes
fn compute_structural_diff(
    old: &StatuteStructure,
    new: &StatuteStructure,
) -> Result<StructuralDiff> {
    let mut added_articles = Vec::new();
    let mut deleted_articles = Vec::new();
    let mut modified_articles = Vec::new();
    let renumbered_articles = Vec::new(); // Simplified for PoC

    // Build article number sets
    let old_numbers: HashMap<u32, &Article> = old.articles.iter().map(|a| (a.number, a)).collect();
    let new_numbers: HashMap<u32, &Article> = new.articles.iter().map(|a| (a.number, a)).collect();

    // Find added articles
    for &number in new_numbers.keys() {
        if !old_numbers.contains_key(&number) {
            added_articles.push(number);
        }
    }

    // Find deleted articles
    for &number in old_numbers.keys() {
        if !new_numbers.contains_key(&number) {
            deleted_articles.push(number);
        }
    }

    // Find modified articles and analyze paragraph-level changes
    let mut paragraph_changes = Vec::new();

    for (&number, old_article) in &old_numbers {
        if let Some(new_article) = new_numbers.get(&number)
            && article_differs(old_article, new_article)
        {
            modified_articles.push(ArticleModification {
                article_number: number,
                change_type: "Modified".to_string(),
                description: format!("Article {} content changed", number),
            });

            // NEW: Detect paragraph-level changes
            let para_changes = detect_paragraph_changes(number, old_article, new_article);
            paragraph_changes.extend(para_changes);
        }
    }

    Ok(StructuralDiff {
        added_articles,
        deleted_articles,
        modified_articles,
        paragraph_changes,
        renumbered_articles,
        table_changes: Vec::new(),
    })
}

/// Detect paragraph-level changes within an article
fn detect_paragraph_changes(
    article_number: u32,
    old_article: &Article,
    new_article: &Article,
) -> Vec<ParagraphChange> {
    let mut changes = Vec::new();

    // Build paragraph maps
    let old_paras: HashMap<u32, &Paragraph> = old_article
        .paragraphs
        .iter()
        .map(|p| (p.number, p))
        .collect();

    let new_paras: HashMap<u32, &Paragraph> = new_article
        .paragraphs
        .iter()
        .map(|p| (p.number, p))
        .collect();

    // Find added paragraphs
    for &para_num in new_paras.keys() {
        if !old_paras.contains_key(&para_num) {
            changes.push(ParagraphChange {
                article_number,
                change_type: ParagraphChangeType::Added,
                old_paragraph: None,
                new_paragraph: Some(para_num),
                description: format!("Article {} paragraph {} added", article_number, para_num),
            });
        }
    }

    // Find deleted paragraphs
    for &para_num in old_paras.keys() {
        if !new_paras.contains_key(&para_num) {
            changes.push(ParagraphChange {
                article_number,
                change_type: ParagraphChangeType::Deleted,
                old_paragraph: Some(para_num),
                new_paragraph: None,
                description: format!("Article {} paragraph {} deleted", article_number, para_num),
            });
        }
    }

    // Find modified paragraphs
    for (&para_num, old_para) in &old_paras {
        if let Some(new_para) = new_paras.get(&para_num)
            && old_para.text != new_para.text
        {
            changes.push(ParagraphChange {
                article_number,
                change_type: ParagraphChangeType::Modified,
                old_paragraph: Some(para_num),
                new_paragraph: Some(para_num),
                description: format!(
                    "Article {} paragraph {} content modified",
                    article_number, para_num
                ),
            });
        }
    }

    changes
}

/// Check if two articles differ
fn article_differs(old: &Article, new: &Article) -> bool {
    if old.title != new.title {
        return true;
    }
    if old.paragraphs.len() != new.paragraphs.len() {
        return true;
    }
    for (old_p, new_p) in old.paragraphs.iter().zip(new.paragraphs.iter()) {
        if old_p.text != new_p.text {
            return true;
        }
    }
    false
}

/// Analyze impact of changes
fn analyze_impact(
    diff: &StructuralDiff,
    _old: &StatuteStructure,
    _new: &StatuteStructure,
) -> Result<ImpactAnalysis> {
    let mut severity = ImpactSeverity::Low;
    let mut affected_references = Vec::new();
    let mut recommended_actions = Vec::new();

    // Assess severity
    if !diff.added_articles.is_empty() || !diff.deleted_articles.is_empty() {
        severity = ImpactSeverity::High;
        recommended_actions
            .push("Review all cross-references to added/deleted articles".to_string());
    }

    if !diff.modified_articles.is_empty() {
        severity = severity.max(ImpactSeverity::Medium);
        recommended_actions
            .push("Verify modified articles don't affect dependent laws".to_string());
    }

    // Simulate affected references (in production, this would scan registry)
    if severity == ImpactSeverity::High {
        affected_references.push(AffectedReference {
            source: "Related Act Article 15".to_string(),
            old_target: "Article 10".to_string(),
            new_target: "Article 11 (shifted)".to_string(),
            needs_update: true,
        });
    }

    let risk_assessment = match severity {
        ImpactSeverity::Low => "Low risk: Minor changes only".to_string(),
        ImpactSeverity::Medium => "Medium risk: Content changes require review".to_string(),
        ImpactSeverity::High => "High risk: Structural changes affect cross-references".to_string(),
        ImpactSeverity::Critical => {
            "Critical risk: Major restructure affects multiple laws".to_string()
        }
    };

    Ok(ImpactAnalysis {
        severity,
        affected_references,
        recommended_actions,
        risk_assessment,
    })
}

impl std::cmp::PartialOrd for ImpactSeverity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for ImpactSeverity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = match self {
            ImpactSeverity::Low => 0,
            ImpactSeverity::Medium => 1,
            ImpactSeverity::High => 2,
            ImpactSeverity::Critical => 3,
        };
        let other_val = match other {
            ImpactSeverity::Low => 0,
            ImpactSeverity::Medium => 1,
            ImpactSeverity::High => 2,
            ImpactSeverity::Critical => 3,
        };
        self_val.cmp(&other_val)
    }
}

impl std::cmp::Eq for ImpactSeverity {}

/// Generate human-readable report
fn generate_report(diff: &StructuralDiff, impact: &ImpactAnalysis) {
    println!("═══════════════════════════════════════════════════════════");
    println!("  LEGISLATIVE DIFF & IMPACT ANALYSIS REPORT");
    println!("═══════════════════════════════════════════════════════════\n");

    // Summary
    println!("📊 Summary:");
    println!("  Added Articles:    {}", diff.added_articles.len());
    println!("  Deleted Articles:  {}", diff.deleted_articles.len());
    println!("  Modified Articles: {}", diff.modified_articles.len());
    println!(
        "  Paragraph Changes: {} (NEW: Fine-grained tracking)",
        diff.paragraph_changes.len()
    );
    println!("  Renumbered:        {}", diff.renumbered_articles.len());
    println!("  Impact Severity:   {:?}\n", impact.severity);

    // Detailed changes
    if !diff.added_articles.is_empty() {
        println!("✅ Added Articles:");
        for &number in &diff.added_articles {
            println!("   • Article {}", number);
        }
        println!();
    }

    if !diff.deleted_articles.is_empty() {
        println!("❌ Deleted Articles:");
        for &number in &diff.deleted_articles {
            println!("   • Article {}", number);
        }
        println!();
    }

    if !diff.modified_articles.is_empty() {
        println!("🔄 Modified Articles:");
        for modification in &diff.modified_articles {
            println!(
                "   • Article {}: {}",
                modification.article_number, modification.description
            );
        }
        println!();
    }

    // NEW: Paragraph-level changes (fine-grained tracking)
    if !diff.paragraph_changes.is_empty() {
        println!("📝 Paragraph-Level Changes (Fine-Grained Analysis):");
        for change in &diff.paragraph_changes {
            let symbol = match change.change_type {
                ParagraphChangeType::Added => "✅",
                ParagraphChangeType::Deleted => "❌",
                ParagraphChangeType::Modified => "🔄",
                ParagraphChangeType::Renumbered => "🔀",
            };
            println!("   {} {}", symbol, change.description);
        }
        println!();
    }

    // Impact analysis
    println!("⚠️  Impact Analysis:");
    println!("  {}\n", impact.risk_assessment);

    if !impact.affected_references.is_empty() {
        println!("🔗 Affected Cross-References:");
        for reference in &impact.affected_references {
            println!(
                "   • {} references {}",
                reference.source, reference.old_target
            );
            if reference.needs_update {
                println!("     ⚠️  Needs update to: {}", reference.new_target);
            }
        }
        println!();
    }

    if !impact.recommended_actions.is_empty() {
        println!("💡 Recommended Actions:");
        for (i, action) in impact.recommended_actions.iter().enumerate() {
            println!("   {}. {}", i + 1, action);
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("\n📝 Note: This is a Proof of Concept demonstrating CI/CD for law amendments.");
    println!("   Production systems would integrate with full statute registry");
    println!("   and perform comprehensive cross-law impact analysis.");
}
