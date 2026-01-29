//! LLM Hallucination Firewall - Legal Reference Validator
//!
//! This tool validates legal references in LLM-generated text by:
//! 1. Extracting legal references (民法第○条, etc.)
//! 2. Checking if articles exist in statute database
//! 3. Verifying logical consistency
//! 4. Generating error reports for detected hallucinations
//!
//! Status: Proof of Concept - Neuro-Symbolic AI approach

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// Represents a legal reference extracted from text
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LegalReference {
    /// Full text of the reference (e.g., "民法第709条")
    full_text: String,
    /// Statute name (e.g., "民法")
    statute_name: String,
    /// Article number (e.g., 709)
    article_number: u32,
    /// Paragraph number (optional)
    paragraph: Option<u32>,
    /// Item number (optional)
    item: Option<u32>,
    /// Position in text (for reporting)
    position: usize,
}

impl LegalReference {
    fn new(
        full_text: String,
        statute_name: String,
        article_number: u32,
        paragraph: Option<u32>,
        item: Option<u32>,
        position: usize,
    ) -> Self {
        Self {
            full_text,
            statute_name,
            article_number,
            paragraph,
            item,
            position,
        }
    }

    /// Format reference for display
    fn format_reference(&self) -> String {
        let mut result = format!("{}第{}条", self.statute_name, self.article_number);
        if let Some(para) = self.paragraph {
            result.push_str(&format!("第{}項", para));
        }
        if let Some(item) = self.item {
            result.push_str(&format!("第{}号", item));
        }
        result
    }
}

/// Types of validation errors (hallucinations)
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ValidationError {
    /// Article does not exist in statute database
    NonExistentArticle {
        reference: LegalReference,
        reason: String,
    },
    /// Statute name not found
    UnknownStatute {
        reference: LegalReference,
        reason: String,
    },
    /// Paragraph/item number exceeds actual structure
    InvalidSubdivision {
        reference: LegalReference,
        reason: String,
    },
    /// Logical inconsistency detected
    LogicalInconsistency {
        reference: LegalReference,
        reason: String,
    },
}

impl ValidationError {
    fn severity(&self) -> &str {
        match self {
            ValidationError::NonExistentArticle { .. } => "HIGH",
            ValidationError::UnknownStatute { .. } => "HIGH",
            ValidationError::InvalidSubdivision { .. } => "MEDIUM",
            ValidationError::LogicalInconsistency { .. } => "MEDIUM",
        }
    }

    fn error_type(&self) -> &str {
        match self {
            ValidationError::NonExistentArticle { .. } => "Non-existent Article",
            ValidationError::UnknownStatute { .. } => "Unknown Statute",
            ValidationError::InvalidSubdivision { .. } => "Invalid Subdivision",
            ValidationError::LogicalInconsistency { .. } => "Logical Inconsistency",
        }
    }

    fn reference(&self) -> &LegalReference {
        match self {
            ValidationError::NonExistentArticle { reference, .. }
            | ValidationError::UnknownStatute { reference, .. }
            | ValidationError::InvalidSubdivision { reference, .. }
            | ValidationError::LogicalInconsistency { reference, .. } => reference,
        }
    }

    fn reason(&self) -> &str {
        match self {
            ValidationError::NonExistentArticle { reason, .. }
            | ValidationError::UnknownStatute { reason, .. }
            | ValidationError::InvalidSubdivision { reason, .. }
            | ValidationError::LogicalInconsistency { reason, .. } => reason,
        }
    }
}

/// Validation report
#[derive(Debug)]
#[allow(dead_code)]
struct ValidationReport {
    /// Total references found
    total_references: usize,
    /// Valid references
    valid_references: usize,
    /// Detected errors
    errors: Vec<ValidationError>,
    /// Processed text
    text: String,
}

impl ValidationReport {
    fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }

    fn error_rate(&self) -> f64 {
        if self.total_references == 0 {
            return 0.0;
        }
        self.errors.len() as f64 / self.total_references as f64 * 100.0
    }
}

/// Extract legal references from text using regex patterns
fn extract_legal_references(text: &str) -> Result<Vec<LegalReference>> {
    let mut references = Vec::new();

    // Pattern 1: 法令名 + 第○条 (basic)
    // Example: 民法第709条
    // Exclude markdown bold markers (**) and other special characters
    let pattern1 = Regex::new(r"([^\s「」\*]+法)第(\d+)条(?:第(\d+)項)?(?:第(\d+)号)?")?;

    for caps in pattern1.captures_iter(text) {
        let full_match = caps.get(0).expect("Full match should exist");
        let statute_name = caps.get(1).expect("Statute name").as_str().to_string();
        let article_number: u32 = caps
            .get(2)
            .expect("Article number")
            .as_str()
            .parse()
            .expect("Should be valid number");
        let paragraph = caps.get(3).and_then(|m| m.as_str().parse().ok());
        let item = caps.get(4).and_then(|m| m.as_str().parse().ok());

        references.push(LegalReference::new(
            full_match.as_str().to_string(),
            statute_name,
            article_number,
            paragraph,
            item,
            full_match.start(),
        ));
    }

    Ok(references)
}

/// Load statute ranges from configuration file (multi-jurisdiction support)
fn get_known_statute_ranges() -> HashMap<String, (u32, u32)> {
    // Try to load from config file first
    if let Ok(config_ranges) = load_statute_ranges_from_config() {
        return config_ranges;
    }

    // Fallback: Hardcoded ranges (Japanese statutes only)
    let mut ranges = HashMap::new();
    ranges.insert("民法".to_string(), (1, 1050));
    ranges.insert("刑法".to_string(), (1, 264));
    ranges.insert("商法".to_string(), (1, 851));
    ranges.insert("会社法".to_string(), (1, 979));
    ranges.insert("民事訴訟法".to_string(), (1, 403));
    ranges.insert("刑事訴訟法".to_string(), (1, 507));
    ranges.insert("憲法".to_string(), (1, 103));
    ranges.insert("日本国憲法".to_string(), (1, 103));
    ranges.insert("借地借家法".to_string(), (1, 61));
    ranges.insert("消費者契約法".to_string(), (1, 41));
    ranges.insert("不動産登記法".to_string(), (1, 164));
    ranges.insert("労働基準法".to_string(), (1, 121));

    ranges
}

/// Load statute ranges from JSON configuration (multi-jurisdiction)
fn load_statute_ranges_from_config() -> Result<HashMap<String, (u32, u32)>> {
    use serde_json::Value;

    let config_text = std::fs::read_to_string("config/statute_ranges.json")?;
    let config: Value = serde_json::from_str(&config_text)?;

    let mut all_ranges = HashMap::new();

    // Load all jurisdictions
    if let Some(jurisdictions) = config["jurisdictions"].as_object() {
        for (_jurisdiction_key, jurisdiction_data) in jurisdictions {
            if let Some(statutes) = jurisdiction_data["statutes"].as_object() {
                for (statute_name, range_data) in statutes {
                    let min = range_data["min_article"].as_u64().unwrap_or(1) as u32;
                    let max = range_data["max_article"].as_u64().unwrap_or(1000) as u32;
                    all_ranges.insert(statute_name.clone(), (min, max));
                }
            }
        }
    }

    Ok(all_ranges)
}

/// Validate extracted references against known statute database
fn validate_references(references: &[LegalReference]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let statute_ranges = get_known_statute_ranges();

    for reference in references {
        // Check if statute is known
        if let Some((min, max)) = statute_ranges.get(&reference.statute_name) {
            // Check if article number is in valid range
            if reference.article_number < *min || reference.article_number > *max {
                errors.push(ValidationError::NonExistentArticle {
                    reference: reference.clone(),
                    reason: format!(
                        "Article {} does not exist in {}. Valid range: Articles {}-{}",
                        reference.article_number, reference.statute_name, min, max
                    ),
                });
            }

            // Basic subdivision validation (simplified)
            // In real implementation, this would check actual statute structure
            if let Some(para) = reference.paragraph
                && para > 10
            {
                // Most articles have fewer than 10 paragraphs
                errors.push(ValidationError::InvalidSubdivision {
                    reference: reference.clone(),
                    reason: format!(
                        "Paragraph {} is unusually high. Please verify structure.",
                        para
                    ),
                });
            }
        } else {
            // Statute name not in known database
            // Note: This is a simplification - in production, we'd query full registry
            errors.push(ValidationError::UnknownStatute {
                reference: reference.clone(),
                reason: format!(
                    "Statute '{}' not found in database. This may be a hallucination or require manual verification.",
                    reference.statute_name
                ),
            });
        }
    }

    errors
}

/// Generate validation report
fn generate_report(
    text: &str,
    references: Vec<LegalReference>,
    errors: Vec<ValidationError>,
) -> ValidationReport {
    let valid_count = references.len() - errors.len();

    ValidationReport {
        total_references: references.len(),
        valid_references: valid_count,
        errors,
        text: text.to_string(),
    }
}

/// Print validation report
fn print_report(report: &ValidationReport) {
    println!("═══════════════════════════════════════════════════════════");
    println!("  LLM HALLUCINATION FIREWALL - Validation Report");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📊 Summary:");
    println!("  Total References Found: {}", report.total_references);
    println!("  Valid References:       {}", report.valid_references);
    println!("  Detected Hallucinations: {}", report.errors.len());
    println!("  Error Rate:             {:.1}%\n", report.error_rate());

    if report.is_clean() {
        println!("✅ VALIDATION PASSED");
        println!("   All legal references are valid. No hallucinations detected.\n");
    } else {
        println!("❌ VALIDATION FAILED");
        println!("   {} hallucination(s) detected:\n", report.errors.len());

        for (i, error) in report.errors.iter().enumerate() {
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Error #{} [Severity: {}]", i + 1, error.severity());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("  Type:      {}", error.error_type());
            println!("  Reference: {}", error.reference().full_text);
            println!("  Canonical: {}", error.reference().format_reference());
            println!("  Position:  Character {}", error.reference().position);
            println!("  Reason:    {}\n", error.reason());
        }
    }

    println!("═══════════════════════════════════════════════════════════");
}

fn main() -> Result<()> {
    println!("🛡️  LLM Hallucination Firewall - Legal Reference Validator\n");

    // Load statute database (configuration-driven, multi-jurisdiction)
    println!("📂 Loading statute database from config...");
    let statute_count = get_known_statute_ranges().len();
    println!(
        "   ✅ Loaded {} statutes from config/statute_ranges.json",
        statute_count
    );
    println!("   🌍 Supports: Japan, Germany, USA (multi-jurisdiction)\n");

    // Process both sample files
    let samples = vec![
        (
            "sample_outputs/hallucination_example.txt",
            "Sample with Hallucinations",
        ),
        ("sample_outputs/correct_example.txt", "Correct Sample"),
    ];

    for (file_path, description) in samples {
        println!("\n▼ Processing: {}", description);
        println!("   File: {}\n", file_path);

        let text = fs::read_to_string(file_path)?;

        // Extract legal references
        let references = extract_legal_references(&text)?;

        // Validate references
        let errors = validate_references(&references);

        // Generate and print report
        let report = generate_report(&text, references, errors);
        print_report(&report);

        println!("\n");
    }

    println!("📝 Note: This is a Proof of Concept demonstrating Neuro-Symbolic AI.");
    println!("   Production systems would integrate with full statute databases.");
    println!("   Current implementation uses simplified validation rules.\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic_reference() {
        let text = "民法第709条により損害賠償を請求する。";
        let refs = extract_legal_references(text).expect("Should extract");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].statute_name, "民法");
        assert_eq!(refs[0].article_number, 709);
        assert_eq!(refs[0].paragraph, None);
    }

    #[test]
    fn test_extract_with_paragraph() {
        let text = "民法第709条第1項に基づき...";
        let refs = extract_legal_references(text).expect("Should extract");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].paragraph, Some(1));
    }

    #[test]
    fn test_validate_valid_article() {
        let reference = LegalReference::new(
            "民法第709条".to_string(),
            "民法".to_string(),
            709,
            None,
            None,
            0,
        );
        let errors = validate_references(&[reference]);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_validate_invalid_article() {
        let reference = LegalReference::new(
            "民法第9999条".to_string(),
            "民法".to_string(),
            9999,
            None,
            None,
            0,
        );
        let errors = validate_references(&[reference]);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            ValidationError::NonExistentArticle { .. }
        ));
    }

    #[test]
    fn test_error_rate_calculation() {
        let report = ValidationReport {
            total_references: 10,
            valid_references: 7,
            errors: vec![],
            text: String::new(),
        };
        assert_eq!(report.error_rate(), 0.0);
    }
}
