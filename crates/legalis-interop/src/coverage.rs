//! Format coverage analysis and reporting.
//!
//! This module provides tools to analyze what features of each legal DSL format
//! are supported by the interop layer, enabling users to understand conversion
//! capabilities and limitations.

use crate::{LegalConverter, LegalFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Feature coverage information for a format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCoverage {
    /// Format name
    pub format: LegalFormat,
    /// Supported features
    pub supported_features: Vec<String>,
    /// Unsupported features
    pub unsupported_features: Vec<String>,
    /// Partially supported features with notes
    pub partial_features: HashMap<String, String>,
    /// Overall coverage percentage (0.0 - 100.0)
    pub coverage_percentage: f64,
}

impl FormatCoverage {
    /// Creates coverage for a format.
    pub fn new(format: LegalFormat) -> Self {
        let (supported, unsupported, partial) = Self::analyze_format(format);
        let total = supported.len() + unsupported.len() + partial.len();
        let coverage = if total > 0 {
            (supported.len() as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Self {
            format,
            supported_features: supported,
            unsupported_features: unsupported,
            partial_features: partial,
            coverage_percentage: coverage,
        }
    }

    fn analyze_format(format: LegalFormat) -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        match format {
            LegalFormat::Catala => Self::analyze_catala(),
            LegalFormat::Stipula => Self::analyze_stipula(),
            LegalFormat::L4 => Self::analyze_l4(),
            LegalFormat::AkomaNtoso => Self::analyze_akoma_ntoso(),
            LegalFormat::LegalRuleML => Self::analyze_legalruleml(),
            LegalFormat::LegalDocML => Self::analyze_legaldocml(),
            LegalFormat::LKIF => Self::analyze_lkif(),
            LegalFormat::Legalis => Self::analyze_legalis(),
        }
    }

    fn analyze_legaldocml() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Section/Article structure".to_string(),
            "Document metadata".to_string(),
            "Cross-references".to_string(),
            "Basic conditions".to_string(),
        ];

        let unsupported = vec![
            "Amendments".to_string(),
            "Quoted structures".to_string(),
            "Complex versioning".to_string(),
        ];

        let mut partial = HashMap::new();
        partial.insert(
            "Natural language parsing".to_string(),
            "Simple heuristics only".to_string(),
        );

        (supported, unsupported, partial)
    }

    fn analyze_catala() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Scope declarations".to_string(),
            "Context variables".to_string(),
            "Definitions".to_string(),
            "Conditional logic".to_string(),
            "Age conditions".to_string(),
            "Literate programming style".to_string(),
            "Basic expressions".to_string(),
        ];

        let unsupported = vec![
            "Exception handling".to_string(),
            "Scope inheritance".to_string(),
            "Legal article references".to_string(),
            "Complex data structures".to_string(),
        ];

        let mut partial = HashMap::new();
        partial.insert(
            "Temporal logic".to_string(),
            "Basic date ranges supported, advanced temporal operators not yet implemented"
                .to_string(),
        );

        (supported, unsupported, partial)
    }

    fn analyze_stipula() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Agreement declarations".to_string(),
            "Party definitions".to_string(),
            "Basic conditions".to_string(),
            "Age requirements".to_string(),
            "Contract structure".to_string(),
        ];

        let unsupported = vec![
            "State machines".to_string(),
            "Temporal obligations".to_string(),
            "Asset transfer semantics".to_string(),
            "Event handling".to_string(),
        ];

        let partial = HashMap::new();

        (supported, unsupported, partial)
    }

    fn analyze_l4() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Rule declarations".to_string(),
            "Deontic logic (MAY, MUST, SHANT)".to_string(),
            "When-Then structure".to_string(),
            "Age conditions".to_string(),
            "Boolean conditions".to_string(),
            "Entity-action pairs".to_string(),
        ];

        let unsupported = vec![
            "Decision tables".to_string(),
            "Temporal operators".to_string(),
            "Default logic".to_string(),
            "Complex event patterns".to_string(),
        ];

        let mut partial = HashMap::new();
        partial.insert(
            "Condition combinations".to_string(),
            "AND conditions supported, OR and NOT partially supported".to_string(),
        );

        (supported, unsupported, partial)
    }

    fn analyze_akoma_ntoso() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Article structure".to_string(),
            "Document metadata".to_string(),
            "Hierarchical sections".to_string(),
            "Title and headings".to_string(),
            "Basic content".to_string(),
        ];

        let unsupported = vec![
            "Amendments".to_string(),
            "References".to_string(),
            "Complex document relationships".to_string(),
            "Lifecycle events".to_string(),
        ];

        let partial = HashMap::new();

        (supported, unsupported, partial)
    }

    fn analyze_legalruleml() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Legal rules".to_string(),
            "Premises (conditions)".to_string(),
            "Conclusions (effects)".to_string(),
            "Rule metadata".to_string(),
            "Prescriptive/Constitutive rules".to_string(),
        ];

        let unsupported = vec![
            "Defeasibility".to_string(),
            "Argumentation structures".to_string(),
            "Rule overrides".to_string(),
        ];

        let mut partial = HashMap::new();
        partial.insert(
            "Deontic qualifiers".to_string(),
            "Basic qualifiers supported, complex deontic logic partial".to_string(),
        );

        (supported, unsupported, partial)
    }

    fn analyze_lkif() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "Rule definitions".to_string(),
            "Atoms (predicates)".to_string(),
            "Rule body (premises)".to_string(),
            "Rule head (conclusions)".to_string(),
            "Logical operators (AND, OR, NOT)".to_string(),
        ];

        let unsupported = vec![
            "Case representations".to_string(),
            "Arguments".to_string(),
            "Ontologies".to_string(),
            "Complex domain models".to_string(),
        ];

        let partial = HashMap::new();

        (supported, unsupported, partial)
    }

    fn analyze_legalis() -> (Vec<String>, Vec<String>, HashMap<String, String>) {
        let supported = vec![
            "All core features".to_string(),
            "Statutes and rules".to_string(),
            "Conditions (all types)".to_string(),
            "Effects (all types)".to_string(),
            "Metadata".to_string(),
            "Temporal logic".to_string(),
            "Entity relationships".to_string(),
        ];

        let unsupported = vec![];
        let partial = HashMap::new();

        (supported, unsupported, partial)
    }
}

/// Coverage report for all formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    /// Coverage for each format
    pub format_coverage: HashMap<LegalFormat, FormatCoverage>,
    /// Overall average coverage percentage
    pub average_coverage: f64,
}

impl CoverageReport {
    /// Generates a full coverage report.
    pub fn generate() -> Self {
        let formats = vec![
            LegalFormat::Catala,
            LegalFormat::Stipula,
            LegalFormat::L4,
            LegalFormat::AkomaNtoso,
            LegalFormat::LegalRuleML,
            LegalFormat::LKIF,
            LegalFormat::Legalis,
        ];

        let mut format_coverage = HashMap::new();
        let mut total_coverage = 0.0;

        for format in formats {
            let coverage = FormatCoverage::new(format);
            total_coverage += coverage.coverage_percentage;
            format_coverage.insert(format, coverage);
        }

        let average_coverage = total_coverage / format_coverage.len() as f64;

        Self {
            format_coverage,
            average_coverage,
        }
    }

    /// Generates coverage report from a converter instance.
    pub fn from_converter(converter: &LegalConverter) -> Self {
        let imports = converter.supported_imports();
        let exports = converter.supported_exports();

        let mut all_formats = imports;
        all_formats.extend(exports);
        all_formats.sort();
        all_formats.dedup();

        let mut format_coverage = HashMap::new();
        let mut total_coverage = 0.0;

        for format in all_formats {
            let coverage = FormatCoverage::new(format);
            total_coverage += coverage.coverage_percentage;
            format_coverage.insert(format, coverage);
        }

        let average_coverage = if !format_coverage.is_empty() {
            total_coverage / format_coverage.len() as f64
        } else {
            0.0
        };

        Self {
            format_coverage,
            average_coverage,
        }
    }

    /// Returns a markdown-formatted report.
    pub fn to_markdown(&self) -> String {
        let mut md = String::from("# Format Coverage Report\n\n");
        md.push_str(&format!(
            "**Average Coverage:** {:.1}%\n\n",
            self.average_coverage
        ));

        let mut formats: Vec<_> = self.format_coverage.keys().collect();
        formats.sort_by_key(|f| format!("{:?}", f));

        for format in formats {
            if let Some(coverage) = self.format_coverage.get(format) {
                md.push_str(&format!("## {:?}\n\n", coverage.format));
                md.push_str(&format!(
                    "**Coverage:** {:.1}%\n\n",
                    coverage.coverage_percentage
                ));

                if !coverage.supported_features.is_empty() {
                    md.push_str("### Supported Features\n");
                    for feature in &coverage.supported_features {
                        md.push_str(&format!("- ✅ {}\n", feature));
                    }
                    md.push('\n');
                }

                if !coverage.partial_features.is_empty() {
                    md.push_str("### Partially Supported Features\n");
                    for (feature, note) in &coverage.partial_features {
                        md.push_str(&format!("- ⚠️  {} ({})\n", feature, note));
                    }
                    md.push('\n');
                }

                if !coverage.unsupported_features.is_empty() {
                    md.push_str("### Unsupported Features\n");
                    for feature in &coverage.unsupported_features {
                        md.push_str(&format!("- ❌ {}\n", feature));
                    }
                    md.push('\n');
                }
            }
        }

        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_coverage() {
        let coverage = FormatCoverage::new(LegalFormat::Catala);
        assert!(!coverage.supported_features.is_empty());
        assert!(coverage.coverage_percentage > 0.0);
        assert!(coverage.coverage_percentage <= 100.0);
    }

    #[test]
    fn test_coverage_report() {
        let report = CoverageReport::generate();
        assert!(!report.format_coverage.is_empty());
        assert!(report.average_coverage > 0.0);
        assert!(report.average_coverage <= 100.0);
    }

    #[test]
    fn test_coverage_from_converter() {
        let converter = LegalConverter::new();
        let report = CoverageReport::from_converter(&converter);
        assert!(!report.format_coverage.is_empty());
    }

    #[test]
    fn test_markdown_output() {
        let report = CoverageReport::generate();
        let md = report.to_markdown();
        assert!(md.contains("# Format Coverage Report"));
        assert!(md.contains("Average Coverage"));
    }

    #[test]
    fn test_all_formats_covered() {
        let report = CoverageReport::generate();
        assert!(report.format_coverage.contains_key(&LegalFormat::Catala));
        assert!(report.format_coverage.contains_key(&LegalFormat::Stipula));
        assert!(report.format_coverage.contains_key(&LegalFormat::L4));
        assert!(
            report
                .format_coverage
                .contains_key(&LegalFormat::AkomaNtoso)
        );
        assert!(
            report
                .format_coverage
                .contains_key(&LegalFormat::LegalRuleML)
        );
        assert!(report.format_coverage.contains_key(&LegalFormat::LKIF));
        assert!(report.format_coverage.contains_key(&LegalFormat::Legalis));
    }
}
