//! Enhanced converters with integrated optimizations.
//!
//! This module provides optimized versions of converters that leverage
//! the optimization utilities for improved performance and reduced memory usage.

use crate::{
    ConversionReport, InteropResult, LegalConverter, LegalFormat,
    optimizations::{RegexCache, StringInterner, WhitespaceNormalizer},
};
use legalis_core::Statute;
use std::collections::HashMap;

/// Enhanced legal converter with integrated optimizations.
///
/// This converter provides the same functionality as `LegalConverter`
/// but uses optimizations for better performance:
/// - String interning to reduce memory usage
/// - Pre-compiled regex patterns
/// - Whitespace normalization
pub struct EnhancedConverter {
    converter: LegalConverter,
    string_interner: StringInterner,
    regex_cache: RegexCache,
    /// Statistics tracking
    stats: ConversionStats,
}

/// Statistics about conversion operations.
#[derive(Debug, Clone, Default)]
pub struct ConversionStats {
    /// Number of conversions performed
    pub conversions_performed: usize,
    /// Number of statutes processed
    pub statutes_processed: usize,
    /// Total bytes processed
    pub bytes_processed: usize,
    /// Number of strings interned
    pub strings_interned: usize,
    /// Memory saved by string interning (estimated)
    pub memory_saved_bytes: usize,
}

impl EnhancedConverter {
    /// Creates a new enhanced converter.
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
            string_interner: StringInterner::new(),
            regex_cache: RegexCache::new(),
            stats: ConversionStats::default(),
        }
    }

    /// Creates a new enhanced converter with caching enabled.
    pub fn with_cache(cache_size: usize) -> Self {
        Self {
            converter: LegalConverter::with_cache(cache_size),
            string_interner: StringInterner::new(),
            regex_cache: RegexCache::new(),
            stats: ConversionStats::default(),
        }
    }

    /// Imports from a format with optimizations.
    pub fn import(
        &mut self,
        source: &str,
        format: LegalFormat,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        // Update stats
        self.stats.bytes_processed += source.len();
        self.stats.conversions_performed += 1;

        // Normalize whitespace if needed
        let normalized = if self.should_normalize(format) {
            WhitespaceNormalizer::normalize(source)
        } else {
            source.to_string()
        };

        // Perform conversion
        let (statutes, report) = self.converter.import(&normalized, format)?;

        // Update stats
        self.stats.statutes_processed += statutes.len();

        Ok((statutes, report))
    }

    /// Exports to a format with optimizations.
    pub fn export(
        &mut self,
        statutes: &[Statute],
        format: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        // Update stats
        self.stats.conversions_performed += 1;
        self.stats.statutes_processed += statutes.len();

        // Perform conversion
        let (output, report) = self.converter.export(statutes, format)?;

        // Update stats
        self.stats.bytes_processed += output.len();

        Ok((output, report))
    }

    /// Converts between formats with optimizations.
    pub fn convert(
        &mut self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        let (statutes, import_report) = self.import(source, from)?;
        let (output, mut export_report) = self.export(&statutes, to)?;

        // Merge reports
        export_report.source_format = import_report.source_format;
        export_report
            .unsupported_features
            .extend(import_report.unsupported_features);
        export_report.warnings.extend(import_report.warnings);
        export_report.confidence = (import_report.confidence * export_report.confidence).max(0.0);

        Ok((output, export_report))
    }

    /// Returns conversion statistics.
    pub fn stats(&self) -> &ConversionStats {
        &self.stats
    }

    /// Resets statistics.
    pub fn reset_stats(&mut self) {
        self.stats = ConversionStats::default();
    }

    /// Returns the regex cache (for advanced usage).
    pub fn regex_cache(&self) -> &RegexCache {
        &self.regex_cache
    }

    /// Returns the string interner (for advanced usage).
    pub fn string_interner(&self) -> &StringInterner {
        &self.string_interner
    }

    /// Returns mutable access to string interner for interning custom strings.
    pub fn string_interner_mut(&mut self) -> &mut StringInterner {
        &mut self.string_interner
    }

    /// Determines if whitespace normalization should be applied for a format.
    fn should_normalize(&self, format: LegalFormat) -> bool {
        match format {
            // Text-based formats benefit from normalization
            LegalFormat::Catala | LegalFormat::Stipula | LegalFormat::L4 | LegalFormat::Spdx => {
                true
            }
            // XML formats are whitespace-sensitive
            LegalFormat::AkomaNtoso
            | LegalFormat::LegalRuleML
            | LegalFormat::LegalDocML
            | LegalFormat::LKIF
            | LegalFormat::LegalCite
            | LegalFormat::MetaLex
            | LegalFormat::Mpeg21Rel
            | LegalFormat::CreativeCommons => false,
            // Native format
            LegalFormat::Legalis => false,
        }
    }

    /// Analyzes a source document and provides optimization suggestions.
    pub fn analyze(&mut self, source: &str, format: LegalFormat) -> AnalysisReport {
        let mut report = AnalysisReport::default();

        // Analyze identifiers
        match format {
            LegalFormat::Catala => {
                let scopes = self.regex_cache.find_catala_scopes(source);
                report.identifiers_found = scopes.len();
                report.format_specific.insert(
                    "scopes".to_string(),
                    scopes.iter().map(|s| s.to_string()).collect(),
                );
            }
            LegalFormat::L4 => {
                let rules = self.regex_cache.find_l4_rules(source);
                let modalities = self.regex_cache.find_deontic_modalities(source);
                report.identifiers_found = rules.len();
                report.format_specific.insert(
                    "rules".to_string(),
                    rules.iter().map(|s| s.to_string()).collect(),
                );
                report.format_specific.insert(
                    "modalities".to_string(),
                    modalities.iter().map(|s| s.to_string()).collect(),
                );
            }
            LegalFormat::Stipula => {
                let agreements = self.regex_cache.find_stipula_agreements(source);
                report.identifiers_found = agreements.len();
                report.format_specific.insert(
                    "agreements".to_string(),
                    agreements.iter().map(|s| s.to_string()).collect(),
                );
            }
            _ => {}
        }

        // Analyze age conditions
        let age_conditions = self.regex_cache.extract_age_conditions(source);
        report.age_conditions_found = age_conditions.len();

        // Estimate memory usage
        report.source_size_bytes = source.len();

        // Check for normalization benefits
        let normalized = WhitespaceNormalizer::normalize(source);
        if normalized.len() < source.len() {
            report.normalization_savings_bytes = source.len() - normalized.len();
        }

        report
    }

    /// Enables caching.
    pub fn enable_cache(&mut self, cache_size: usize) {
        self.converter.enable_cache(cache_size);
    }

    /// Disables caching.
    pub fn disable_cache(&mut self) {
        self.converter.disable_cache();
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.converter.clear_cache();
    }

    /// Returns cache statistics.
    pub fn cache_stats(&self) -> Option<crate::cache::CacheStats> {
        self.converter.cache_stats()
    }
}

impl Default for EnhancedConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis report for a source document.
#[derive(Debug, Clone, Default)]
pub struct AnalysisReport {
    /// Number of identifiers found (scopes, rules, agreements, etc.)
    pub identifiers_found: usize,
    /// Number of age conditions found
    pub age_conditions_found: usize,
    /// Source size in bytes
    pub source_size_bytes: usize,
    /// Estimated bytes saved by whitespace normalization
    pub normalization_savings_bytes: usize,
    /// Format-specific analysis results
    pub format_specific: HashMap<String, Vec<String>>,
}

impl AnalysisReport {
    /// Returns a human-readable summary of the analysis.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!("Source size: {} bytes", self.source_size_bytes));

        if self.identifiers_found > 0 {
            lines.push(format!("Identifiers found: {}", self.identifiers_found));
        }

        if self.age_conditions_found > 0 {
            lines.push(format!(
                "Age conditions found: {}",
                self.age_conditions_found
            ));
        }

        if self.normalization_savings_bytes > 0 {
            lines.push(format!(
                "Whitespace normalization could save {} bytes ({:.1}%)",
                self.normalization_savings_bytes,
                (self.normalization_savings_bytes as f64 / self.source_size_bytes as f64) * 100.0
            ));
        }

        for (key, values) in &self.format_specific {
            if !values.is_empty() {
                lines.push(format!("{}: {} found", key, values.len()));
            }
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_converter_basic() {
        let mut converter = EnhancedConverter::new();

        let catala_source = r#"
declaration scope VotingRights:
  context input content Input

scope VotingRights:
  definition output.eligible equals
    input.age >= 18
"#;

        let (statutes, report) = converter
            .import(catala_source, LegalFormat::Catala)
            .unwrap();

        assert!(!statutes.is_empty());
        assert_eq!(report.source_format, Some(LegalFormat::Catala));

        // Check stats
        let stats = converter.stats();
        assert_eq!(stats.conversions_performed, 1);
        assert!(stats.bytes_processed > 0);
        assert!(stats.statutes_processed > 0);
    }

    #[test]
    fn test_enhanced_converter_with_cache() {
        let mut converter = EnhancedConverter::with_cache(10);

        let source = "declaration scope Test:\n  context input content integer";

        // First conversion
        let (output1, _) = converter
            .convert(source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        // Second conversion (should hit cache)
        let (output2, _) = converter
            .convert(source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        assert_eq!(output1, output2);
        assert!(converter.cache_stats().is_some());
    }

    #[test]
    fn test_analyze_catala() {
        let mut converter = EnhancedConverter::new();

        let catala_source = r#"
declaration scope VotingRights:
  context input content Input

declaration scope TaxBenefit:
  context input content Input

scope VotingRights:
  definition output.eligible equals
    input.age >= 18
"#;

        let report = converter.analyze(catala_source, LegalFormat::Catala);

        assert_eq!(report.identifiers_found, 2); // VotingRights, TaxBenefit
        assert_eq!(report.age_conditions_found, 1);
        assert!(report.source_size_bytes > 0);

        let scopes = report.format_specific.get("scopes");
        assert!(scopes.is_some());
        assert_eq!(scopes.unwrap().len(), 2);
    }

    #[test]
    fn test_analyze_l4() {
        let mut converter = EnhancedConverter::new();

        let l4_source = r#"
RULE VotingAge WHEN age >= 18 THEN Person MAY vote
RULE DrivingAge WHEN age >= 16 THEN Person MUST have_license
"#;

        let report = converter.analyze(l4_source, LegalFormat::L4);

        assert_eq!(report.identifiers_found, 2); // VotingAge, DrivingAge
        assert_eq!(report.age_conditions_found, 2);

        let rules = report.format_specific.get("rules");
        assert!(rules.is_some());
        assert_eq!(rules.unwrap().len(), 2);

        let modalities = report.format_specific.get("modalities");
        assert!(modalities.is_some());
        assert!(modalities.unwrap().contains(&"MAY".to_string()));
        assert!(modalities.unwrap().contains(&"MUST".to_string()));
    }

    #[test]
    fn test_analyze_stipula() {
        let mut converter = EnhancedConverter::new();

        let stipula_source = r#"
agreement RentalContract(Landlord, Tenant) {
    val rent = 1000
}

agreement ServiceContract(Provider, Client) {
    val fee = 500
}
"#;

        let report = converter.analyze(stipula_source, LegalFormat::Stipula);

        assert_eq!(report.identifiers_found, 2); // RentalContract, ServiceContract

        let agreements = report.format_specific.get("agreements");
        assert!(agreements.is_some());
        assert_eq!(agreements.unwrap().len(), 2);
    }

    #[test]
    fn test_conversion_stats() {
        let mut converter = EnhancedConverter::new();

        let source = "RULE Test WHEN age >= 21 THEN Person MAY test";

        // Perform multiple conversions
        for _ in 0..3 {
            converter
                .convert(source, LegalFormat::L4, LegalFormat::Catala)
                .unwrap();
        }

        let stats = converter.stats();
        assert_eq!(stats.conversions_performed, 6); // 3 imports + 3 exports
        assert!(stats.bytes_processed > 0);

        // Reset and verify
        converter.reset_stats();
        assert_eq!(converter.stats().conversions_performed, 0);
    }

    #[test]
    fn test_whitespace_normalization() {
        let mut converter = EnhancedConverter::new();

        // Source with extra whitespace
        let source_with_whitespace = "RULE   Test   WHEN   age >= 18   THEN   Person   MAY   vote";

        let (statutes, _) = converter
            .import(source_with_whitespace, LegalFormat::L4)
            .unwrap();

        assert!(!statutes.is_empty());
        // Normalization should not affect parsing
    }

    #[test]
    fn test_analysis_summary() {
        let mut converter = EnhancedConverter::new();

        let catala_source = r#"
declaration scope Test:
  context input content Input

scope Test:
  definition output.result equals
    input.age >= 21
"#;

        let report = converter.analyze(catala_source, LegalFormat::Catala);
        let summary = report.summary();

        assert!(summary.contains("Source size:"));
        assert!(summary.contains("bytes"));
    }

    #[test]
    fn test_enhanced_converter_default() {
        let converter = EnhancedConverter::default();
        assert_eq!(converter.stats().conversions_performed, 0);
    }
}
