//! Format compatibility analysis.
//!
//! This module provides tools to analyze compatibility between legal DSL formats,
//! helping users understand which conversions will be lossless and which may lose information.

use crate::LegalFormat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Compatibility level between two formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    /// Full compatibility - lossless conversion
    Full,
    /// High compatibility - minimal information loss
    High,
    /// Medium compatibility - some features may not convert
    Medium,
    /// Low compatibility - significant feature gaps
    Low,
    /// Incompatible - very limited conversion possible
    Incompatible,
}

impl CompatibilityLevel {
    /// Returns a score from 0.0 to 1.0.
    pub fn score(&self) -> f64 {
        match self {
            Self::Full => 1.0,
            Self::High => 0.8,
            Self::Medium => 0.6,
            Self::Low => 0.4,
            Self::Incompatible => 0.2,
        }
    }

    /// Returns a human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Full => "Fully compatible - lossless conversion",
            Self::High => "Highly compatible - minimal loss",
            Self::Medium => "Moderately compatible - some features may not convert",
            Self::Low => "Low compatibility - significant limitations",
            Self::Incompatible => "Limited compatibility - basic structure only",
        }
    }
}

/// Compatibility information between two formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// Source format
    pub from: LegalFormat,
    /// Target format
    pub to: LegalFormat,
    /// Compatibility level
    pub level: CompatibilityLevel,
    /// Features that convert well
    pub compatible_features: Vec<String>,
    /// Features that may be lost or degraded
    pub incompatible_features: Vec<String>,
    /// Recommended workflow notes
    pub notes: Vec<String>,
}

/// Compatibility matrix for all format pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityMatrix {
    /// Map from (source, target) to compatibility info
    compatibility: HashMap<(LegalFormat, LegalFormat), CompatibilityInfo>,
}

impl CompatibilityMatrix {
    /// Creates a new compatibility matrix with default compatibility data.
    pub fn new() -> Self {
        let mut matrix = Self {
            compatibility: HashMap::new(),
        };
        matrix.populate_default_compatibility();
        matrix
    }

    /// Gets compatibility information for converting from one format to another.
    pub fn get_compatibility(
        &self,
        from: LegalFormat,
        to: LegalFormat,
    ) -> Option<&CompatibilityInfo> {
        self.compatibility.get(&(from, to))
    }

    /// Returns all format pairs with a given minimum compatibility level.
    pub fn get_by_level(&self, min_level: CompatibilityLevel) -> Vec<&CompatibilityInfo> {
        self.compatibility
            .values()
            .filter(|info| info.level >= min_level)
            .collect()
    }

    /// Populates the matrix with default compatibility data.
    fn populate_default_compatibility(&mut self) {
        // Define all format pairs
        let formats = vec![
            LegalFormat::Catala,
            LegalFormat::Stipula,
            LegalFormat::L4,
            LegalFormat::AkomaNtoso,
            LegalFormat::LegalRuleML,
            LegalFormat::LegalDocML,
            LegalFormat::LKIF,
            LegalFormat::Legalis,
        ];

        for &from in &formats {
            for &to in &formats {
                if from != to {
                    let info = Self::analyze_pair(from, to);
                    self.compatibility.insert((from, to), info);
                }
            }
        }
    }

    /// Analyzes compatibility between a specific pair of formats.
    fn analyze_pair(from: LegalFormat, to: LegalFormat) -> CompatibilityInfo {
        use LegalFormat::*;

        let (level, compatible, incompatible, notes) = match (from, to) {
            // Conversions to Legalis (native format) - generally high compatibility
            (Catala, Legalis) | (Stipula, Legalis) | (L4, Legalis) => (
                CompatibilityLevel::High,
                vec![
                    "Basic conditions".to_string(),
                    "Rules and effects".to_string(),
                    "Metadata".to_string(),
                ],
                vec!["Format-specific features".to_string()],
                vec!["Good conversion path for further processing".to_string()],
            ),
            (AkomaNtoso | LegalRuleML | LegalDocML | LKIF, Legalis) => (
                CompatibilityLevel::Medium,
                vec!["Document structure".to_string(), "Basic rules".to_string()],
                vec![
                    "Rich metadata".to_string(),
                    "Complex relationships".to_string(),
                ],
                vec!["Structural information preserved".to_string()],
            ),

            // Conversions from Legalis to other formats
            (Legalis, Catala | Stipula | L4) => (
                CompatibilityLevel::High,
                vec![
                    "Conditions and rules".to_string(),
                    "Effect types".to_string(),
                ],
                vec!["Some Legalis-specific features".to_string()],
                vec!["Recommended for code generation".to_string()],
            ),
            (Legalis, AkomaNtoso | LegalRuleML | LegalDocML | LKIF) => (
                CompatibilityLevel::Medium,
                vec!["Rule structure".to_string()],
                vec!["Computational semantics".to_string()],
                vec!["Better for documentation than execution".to_string()],
            ),

            // DSL to DSL conversions (through Legalis as intermediate)
            (Catala, L4) | (L4, Catala) | (Stipula, L4) | (L4, Stipula) => (
                CompatibilityLevel::Medium,
                vec![
                    "Basic conditions".to_string(),
                    "Age and simple comparisons".to_string(),
                ],
                vec![
                    "Format-specific constructs".to_string(),
                    "Idiomatic expressions".to_string(),
                ],
                vec!["Consider manual review after conversion".to_string()],
            ),
            (Catala, Stipula) | (Stipula, Catala) => (
                CompatibilityLevel::Low,
                vec!["Very basic structure".to_string()],
                vec![
                    "Catala scopes vs Stipula agreements".to_string(),
                    "Different computational models".to_string(),
                ],
                vec!["Significant semantic gaps - use with caution".to_string()],
            ),

            // XML format conversions among themselves
            (AkomaNtoso, LegalRuleML)
            | (LegalRuleML, AkomaNtoso)
            | (LegalDocML, LegalRuleML)
            | (LegalRuleML, LegalDocML) => (
                CompatibilityLevel::Medium,
                vec!["XML structure".to_string(), "Metadata".to_string()],
                vec!["Format-specific semantics".to_string()],
                vec!["Structural conversion possible".to_string()],
            ),
            (AkomaNtoso | LegalDocML, LKIF) | (LKIF, AkomaNtoso | LegalDocML) => (
                CompatibilityLevel::Low,
                vec!["Basic rules".to_string()],
                vec![
                    "LKIF's logical focus".to_string(),
                    "Document vs knowledge representation".to_string(),
                ],
                vec!["Different purposes - limited conversion".to_string()],
            ),

            // Other DSL to XML conversions
            (Catala | Stipula | L4, AkomaNtoso | LegalDocML) => (
                CompatibilityLevel::Medium,
                vec!["Rule structure".to_string(), "Basic conditions".to_string()],
                vec!["Executable semantics".to_string()],
                vec!["Good for documentation generation".to_string()],
            ),
            (AkomaNtoso | LegalDocML, Catala | Stipula | L4) => (
                CompatibilityLevel::Low,
                vec!["Text structure".to_string()],
                vec![
                    "No computational semantics in source".to_string(),
                    "Requires manual annotation".to_string(),
                ],
                vec!["Consider manual enhancement after conversion".to_string()],
            ),

            // DSL to LKIF
            (Catala | Stipula | L4, LegalRuleML | LKIF) => (
                CompatibilityLevel::Medium,
                vec![
                    "Rules and conditions".to_string(),
                    "Logic structure".to_string(),
                ],
                vec!["Execution semantics".to_string()],
                vec!["Preserves logical structure".to_string()],
            ),
            (LegalRuleML | LKIF, Catala | Stipula | L4) => (
                CompatibilityLevel::Low,
                vec!["Rule definitions".to_string()],
                vec!["No execution model".to_string()],
                vec!["Requires interpretation".to_string()],
            ),

            // Same format (shouldn't happen, but include for completeness)
            _ => (
                CompatibilityLevel::Full,
                vec!["All features".to_string()],
                vec![],
                vec!["Identity conversion".to_string()],
            ),
        };

        CompatibilityInfo {
            from,
            to,
            level,
            compatible_features: compatible,
            incompatible_features: incompatible,
            notes,
        }
    }

    /// Generates a markdown table showing the compatibility matrix.
    pub fn to_markdown_table(&self) -> String {
        let formats = vec![
            LegalFormat::Catala,
            LegalFormat::Stipula,
            LegalFormat::L4,
            LegalFormat::AkomaNtoso,
            LegalFormat::LegalRuleML,
            LegalFormat::LegalDocML,
            LegalFormat::LKIF,
            LegalFormat::Legalis,
        ];

        let mut md = String::from("# Format Compatibility Matrix\n\n");
        md.push_str(
            "Compatibility levels: 🟢 Full | 🔵 High | 🟡 Medium | 🟠 Low | 🔴 Incompatible\n\n",
        );

        // Header
        md.push_str("| From \\ To |");
        for format in &formats {
            md.push_str(&format!(" {:?} |", format));
        }
        md.push('\n');

        // Separator
        md.push_str("|-----------|");
        for _ in &formats {
            md.push_str("------|");
        }
        md.push('\n');

        // Rows
        for &from in &formats {
            md.push_str(&format!("| **{:?}** |", from));
            for &to in &formats {
                let icon = if from == to {
                    "—"
                } else if let Some(info) = self.get_compatibility(from, to) {
                    match info.level {
                        CompatibilityLevel::Full => "🟢",
                        CompatibilityLevel::High => "🔵",
                        CompatibilityLevel::Medium => "🟡",
                        CompatibilityLevel::Low => "🟠",
                        CompatibilityLevel::Incompatible => "🔴",
                    }
                } else {
                    "?"
                };
                md.push_str(&format!(" {} |", icon));
            }
            md.push('\n');
        }

        md.push_str("\n## Legend\n\n");
        md.push_str("- 🟢 **Full**: Lossless conversion, all features preserved\n");
        md.push_str("- 🔵 **High**: Minimal information loss, highly recommended\n");
        md.push_str("- 🟡 **Medium**: Some features may not convert, review recommended\n");
        md.push_str("- 🟠 **Low**: Significant feature gaps, use with caution\n");
        md.push_str("- 🔴 **Incompatible**: Very limited conversion, not recommended\n");

        md
    }

    /// Gets the best conversion path from source to target format.
    /// Returns a list of formats representing the conversion path.
    pub fn find_best_path(&self, from: LegalFormat, to: LegalFormat) -> Vec<LegalFormat> {
        // For now, we use a simple heuristic: go through Legalis if compatibility is low
        if from == to {
            return vec![from];
        }

        if let Some(direct) = self.get_compatibility(from, to) {
            if direct.level >= CompatibilityLevel::Medium {
                // Direct conversion is good enough
                return vec![from, to];
            }
        }

        // Try via Legalis
        if from != LegalFormat::Legalis && to != LegalFormat::Legalis {
            let via_legalis_score = self
                .get_compatibility(from, LegalFormat::Legalis)
                .map(|i| i.level.score())
                .unwrap_or(0.0)
                * self
                    .get_compatibility(LegalFormat::Legalis, to)
                    .map(|i| i.level.score())
                    .unwrap_or(0.0);

            let direct_score = self
                .get_compatibility(from, to)
                .map(|i| i.level.score())
                .unwrap_or(0.0);

            if via_legalis_score > direct_score {
                return vec![from, LegalFormat::Legalis, to];
            }
        }

        // Default to direct conversion
        vec![from, to]
    }
}

impl Default for CompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatibility_level_score() {
        assert_eq!(CompatibilityLevel::Full.score(), 1.0);
        assert_eq!(CompatibilityLevel::High.score(), 0.8);
        assert!(CompatibilityLevel::Medium.score() > CompatibilityLevel::Low.score());
    }

    #[test]
    fn test_matrix_creation() {
        let matrix = CompatibilityMatrix::new();
        assert!(!matrix.compatibility.is_empty());
    }

    #[test]
    fn test_get_compatibility() {
        let matrix = CompatibilityMatrix::new();
        let info = matrix
            .get_compatibility(LegalFormat::Catala, LegalFormat::Legalis)
            .unwrap();
        assert_eq!(info.from, LegalFormat::Catala);
        assert_eq!(info.to, LegalFormat::Legalis);
        // Catala to Legalis should be High compatibility
        assert_eq!(
            info.level,
            CompatibilityLevel::High,
            "Expected High compatibility for Catala -> Legalis, got {:?}",
            info.level
        );
    }

    #[test]
    fn test_get_by_level() {
        let matrix = CompatibilityMatrix::new();
        let high_compat = matrix.get_by_level(CompatibilityLevel::High);
        assert!(!high_compat.is_empty());

        // All returned items should be at least High
        for info in high_compat {
            assert!(info.level >= CompatibilityLevel::High);
        }
    }

    #[test]
    fn test_markdown_table() {
        let matrix = CompatibilityMatrix::new();
        let md = matrix.to_markdown_table();
        assert!(md.contains("# Format Compatibility Matrix"));
        assert!(md.contains("Catala"));
        assert!(md.contains("L4"));
        assert!(md.contains("Legend"));
    }

    #[test]
    fn test_find_best_path() {
        let matrix = CompatibilityMatrix::new();

        // Same format
        let path = matrix.find_best_path(LegalFormat::L4, LegalFormat::L4);
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], LegalFormat::L4);

        // Direct high compatibility
        let path = matrix.find_best_path(LegalFormat::Catala, LegalFormat::Legalis);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], LegalFormat::Catala);
        assert_eq!(path[1], LegalFormat::Legalis);

        // Any path should start with source and end with target
        let path = matrix.find_best_path(LegalFormat::Catala, LegalFormat::L4);
        assert_eq!(path[0], LegalFormat::Catala);
        assert_eq!(*path.last().unwrap(), LegalFormat::L4);
    }

    #[test]
    fn test_compatibility_info_fields() {
        let matrix = CompatibilityMatrix::new();
        let info = matrix
            .get_compatibility(LegalFormat::L4, LegalFormat::Legalis)
            .unwrap();

        assert!(!info.compatible_features.is_empty());
        assert!(!info.notes.is_empty());
    }
}
