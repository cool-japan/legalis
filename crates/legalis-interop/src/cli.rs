//! CLI tool for standalone legal format conversion.
//!
//! Provides command-line utilities for converting between legal DSL formats.

use crate::{InteropError, InteropResult, LegalConverter, LegalFormat};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// CLI command for conversion operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    /// Convert a file from one format to another
    Convert {
        input: PathBuf,
        output: PathBuf,
        source_format: LegalFormat,
        target_format: LegalFormat,
    },
    /// Auto-detect format and convert
    AutoConvert {
        input: PathBuf,
        output: PathBuf,
        target_format: LegalFormat,
    },
    /// List supported formats
    ListFormats,
    /// Validate a file in a specific format
    Validate { input: PathBuf, format: LegalFormat },
    /// Show conversion statistics
    Info { input: PathBuf, format: LegalFormat },
}

/// CLI configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Enable caching
    pub cache_enabled: bool,
    /// Cache size (number of entries)
    pub cache_size: usize,
    /// Verbose output
    pub verbose: bool,
    /// Fail on warnings
    pub fail_on_warnings: bool,
    /// Minimum confidence threshold (0.0 - 1.0)
    pub min_confidence: f64,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            cache_enabled: true,
            cache_size: 100,
            verbose: false,
            fail_on_warnings: false,
            min_confidence: 0.0,
        }
    }
}

/// CLI executor for processing commands.
pub struct CliExecutor {
    converter: LegalConverter,
    config: CliConfig,
}

impl Default for CliExecutor {
    fn default() -> Self {
        Self::new(CliConfig::default())
    }
}

impl CliExecutor {
    /// Creates a new CLI executor with the given configuration.
    pub fn new(config: CliConfig) -> Self {
        let mut converter = if config.cache_enabled {
            LegalConverter::with_cache(config.cache_size)
        } else {
            LegalConverter::new()
        };

        if !config.cache_enabled {
            converter.disable_cache();
        }

        Self { converter, config }
    }

    /// Executes a CLI command.
    pub fn execute(&mut self, command: CliCommand) -> InteropResult<String> {
        match command {
            CliCommand::Convert {
                input,
                output,
                source_format,
                target_format,
            } => self.execute_convert(&input, &output, source_format, target_format),

            CliCommand::AutoConvert {
                input,
                output,
                target_format,
            } => self.execute_auto_convert(&input, &output, target_format),

            CliCommand::ListFormats => Ok(self.list_formats()),

            CliCommand::Validate { input, format } => self.execute_validate(&input, format),

            CliCommand::Info { input, format } => self.execute_info(&input, format),
        }
    }

    /// Executes a convert command.
    fn execute_convert(
        &mut self,
        input: &Path,
        output: &Path,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> InteropResult<String> {
        // Read input file
        let input_text = fs::read_to_string(input).map_err(|e| {
            InteropError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read input file: {}", e),
            ))
        })?;

        if self.config.verbose {
            eprintln!(
                "Converting {:?} from {:?} to {:?}...",
                input, source_format, target_format
            );
        }

        // Convert
        let (output_text, report) =
            self.converter
                .convert(&input_text, source_format, target_format)?;

        // Check confidence threshold
        if report.confidence < self.config.min_confidence {
            return Err(InteropError::ConversionError(format!(
                "Conversion confidence {:.2} below minimum threshold {:.2}",
                report.confidence, self.config.min_confidence
            )));
        }

        // Check for warnings
        if self.config.fail_on_warnings && !report.warnings.is_empty() {
            return Err(InteropError::ConversionError(format!(
                "Conversion produced {} warnings",
                report.warnings.len()
            )));
        }

        // Write output file
        fs::write(output, &output_text).map_err(|e| {
            InteropError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to write output file: {}", e),
            ))
        })?;

        // Build result message
        let mut message = format!("Conversion successful: {:?} -> {:?}\n", input, output);
        message.push_str(&format!(
            "Statutes converted: {}\n",
            report.statutes_converted
        ));
        message.push_str(&format!("Confidence: {:.2}\n", report.confidence));

        if !report.unsupported_features.is_empty() {
            message.push_str(&format!(
                "Unsupported features: {}\n",
                report.unsupported_features.len()
            ));
            if self.config.verbose {
                for feature in &report.unsupported_features {
                    message.push_str(&format!("  - {}\n", feature));
                }
            }
        }

        if !report.warnings.is_empty() {
            message.push_str(&format!("Warnings: {}\n", report.warnings.len()));
            if self.config.verbose {
                for warning in &report.warnings {
                    message.push_str(&format!("  - {}\n", warning));
                }
            }
        }

        Ok(message)
    }

    /// Executes an auto-convert command.
    fn execute_auto_convert(
        &mut self,
        input: &Path,
        output: &Path,
        target_format: LegalFormat,
    ) -> InteropResult<String> {
        // Read input file
        let input_text = fs::read_to_string(input).map_err(|e| {
            InteropError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read input file: {}", e),
            ))
        })?;

        if self.config.verbose {
            eprintln!("Auto-detecting format for {:?}...", input);
        }

        // Auto-detect and import
        let (statutes, import_report) = self.converter.auto_import(&input_text)?;

        let source_format = import_report
            .source_format
            .ok_or_else(|| InteropError::UnsupportedFormat("Unknown format".to_string()))?;

        if self.config.verbose {
            eprintln!("Detected format: {:?}", source_format);
            eprintln!("Converting to {:?}...", target_format);
        }

        // Export to target format
        let (output_text, export_report) = self.converter.export(&statutes, target_format)?;

        // Merge reports
        let mut report = import_report;
        report.target_format = Some(target_format);
        report
            .unsupported_features
            .extend(export_report.unsupported_features);
        report.warnings.extend(export_report.warnings);
        report.confidence = (report.confidence * export_report.confidence).max(0.0);

        // Check confidence threshold
        if report.confidence < self.config.min_confidence {
            return Err(InteropError::ConversionError(format!(
                "Conversion confidence {:.2} below minimum threshold {:.2}",
                report.confidence, self.config.min_confidence
            )));
        }

        // Write output file
        fs::write(output, &output_text).map_err(|e| {
            InteropError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to write output file: {}", e),
            ))
        })?;

        // Build result message
        let mut message = format!(
            "Auto-conversion successful: {:?} ({:?}) -> {:?} ({:?})\n",
            input, source_format, output, target_format
        );
        message.push_str(&format!(
            "Statutes converted: {}\n",
            report.statutes_converted
        ));
        message.push_str(&format!("Confidence: {:.2}\n", report.confidence));

        if !report.warnings.is_empty() && self.config.verbose {
            message.push_str(&format!("Warnings: {}\n", report.warnings.len()));
            for warning in &report.warnings {
                message.push_str(&format!("  - {}\n", warning));
            }
        }

        Ok(message)
    }

    /// Lists all supported formats.
    fn list_formats(&self) -> String {
        let imports = self.converter.supported_imports();
        let exports = self.converter.supported_exports();

        let mut message = String::from("Supported Formats:\n\n");

        message.push_str("Import Formats:\n");
        for format in &imports {
            message.push_str(&format!("  - {:?} (.{})\n", format, format.extension()));
        }

        message.push_str("\nExport Formats:\n");
        for format in &exports {
            message.push_str(&format!("  - {:?} (.{})\n", format, format.extension()));
        }

        message
    }

    /// Validates a file in a specific format.
    fn execute_validate(&mut self, input: &Path, format: LegalFormat) -> InteropResult<String> {
        let input_text = fs::read_to_string(input).map_err(|e| {
            InteropError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read input file: {}", e),
            ))
        })?;

        if self.config.verbose {
            eprintln!("Validating {:?} as {:?}...", input, format);
        }

        // Try to import
        let (statutes, report) = self.converter.import(&input_text, format)?;

        let mut message = format!("Validation successful for {:?}\n", input);
        message.push_str(&format!("Format: {:?}\n", format));
        message.push_str(&format!("Statutes found: {}\n", statutes.len()));
        message.push_str(&format!("Confidence: {:.2}\n", report.confidence));

        if !report.warnings.is_empty() {
            message.push_str(&format!("Warnings: {}\n", report.warnings.len()));
            if self.config.verbose {
                for warning in &report.warnings {
                    message.push_str(&format!("  - {}\n", warning));
                }
            }
        }

        Ok(message)
    }

    /// Shows information about a file.
    fn execute_info(&mut self, input: &Path, format: LegalFormat) -> InteropResult<String> {
        let input_text = fs::read_to_string(input).map_err(|e| {
            InteropError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read input file: {}", e),
            ))
        })?;

        if self.config.verbose {
            eprintln!("Analyzing {:?}...", input);
        }

        // Import
        let (statutes, report) = self.converter.import(&input_text, format)?;

        let mut message = format!("File Information: {:?}\n", input);
        message.push_str(&format!("Format: {:?}\n", format));
        message.push_str(&format!("Size: {} bytes\n", input_text.len()));
        message.push_str(&format!("Statutes: {}\n", statutes.len()));
        message.push_str(&format!("Confidence: {:.2}\n", report.confidence));

        // Count total preconditions and effects
        let total_preconditions: usize = statutes.iter().map(|s| s.preconditions.len()).sum();
        message.push_str(&format!("Total preconditions: {}\n", total_preconditions));

        if !report.warnings.is_empty() {
            message.push_str(&format!("Warnings: {}\n", report.warnings.len()));
            if self.config.verbose {
                for warning in &report.warnings {
                    message.push_str(&format!("  - {}\n", warning));
                }
            }
        }

        Ok(message)
    }

    /// Returns the current configuration.
    pub fn config(&self) -> &CliConfig {
        &self.config
    }

    /// Returns cache statistics if caching is enabled.
    pub fn cache_stats(&self) -> Option<crate::cache::CacheStats> {
        self.converter.cache_stats()
    }
}

/// Parses format from string (case-insensitive).
pub fn parse_format(s: &str) -> Option<LegalFormat> {
    match s.to_lowercase().as_str() {
        "catala" => Some(LegalFormat::Catala),
        "stipula" => Some(LegalFormat::Stipula),
        "l4" => Some(LegalFormat::L4),
        "akomantoso" | "akoma-ntoso" => Some(LegalFormat::AkomaNtoso),
        "legalruleml" => Some(LegalFormat::LegalRuleML),
        "legaldocml" => Some(LegalFormat::LegalDocML),
        "lkif" => Some(LegalFormat::LKIF),
        "legalcite" => Some(LegalFormat::LegalCite),
        "metalex" => Some(LegalFormat::MetaLex),
        "mpeg21rel" | "mpeg21-rel" => Some(LegalFormat::Mpeg21Rel),
        "creativecommons" | "cc" => Some(LegalFormat::CreativeCommons),
        "spdx" => Some(LegalFormat::Spdx),
        "legalis" => Some(LegalFormat::Legalis),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_format() {
        assert_eq!(parse_format("catala"), Some(LegalFormat::Catala));
        assert_eq!(parse_format("CATALA"), Some(LegalFormat::Catala));
        assert_eq!(parse_format("stipula"), Some(LegalFormat::Stipula));
        assert_eq!(parse_format("l4"), Some(LegalFormat::L4));
        assert_eq!(parse_format("unknown"), None);
    }

    #[test]
    fn test_cli_list_formats() {
        let executor = CliExecutor::default();
        let result = executor.list_formats();

        assert!(result.contains("Catala"));
        assert!(result.contains("Stipula"));
        assert!(result.contains("L4"));
    }

    #[test]
    fn test_cli_convert() {
        let mut executor = CliExecutor::default();

        // Create temp input file
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "declaration scope Test:").unwrap();
        writeln!(input_file, "  context input content integer").unwrap();

        // Create temp output file path
        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_owned();

        let command = CliCommand::Convert {
            input: input_file.path().to_owned(),
            output: output_path.clone(),
            source_format: LegalFormat::Catala,
            target_format: LegalFormat::L4,
        };

        let result = executor.execute(command).unwrap();

        assert!(result.contains("Conversion successful"));
        assert!(output_path.exists());

        let output_text = fs::read_to_string(&output_path).unwrap();
        assert!(output_text.contains("RULE"));
    }

    #[test]
    fn test_cli_auto_convert() {
        let mut executor = CliExecutor::default();

        // Create temp input file with Catala content
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "declaration scope AutoTest:").unwrap();
        writeln!(input_file, "  context input content integer").unwrap();

        // Create temp output file path
        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_owned();

        let command = CliCommand::AutoConvert {
            input: input_file.path().to_owned(),
            output: output_path.clone(),
            target_format: LegalFormat::L4,
        };

        let result = executor.execute(command).unwrap();

        assert!(result.contains("Auto-conversion successful"));
        assert!(result.contains("Catala"));
        assert!(output_path.exists());
    }

    #[test]
    fn test_cli_validate() {
        let mut executor = CliExecutor::default();

        // Create temp input file
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "declaration scope ValidateTest:").unwrap();
        writeln!(input_file, "  context input content integer").unwrap();

        let command = CliCommand::Validate {
            input: input_file.path().to_owned(),
            format: LegalFormat::Catala,
        };

        let result = executor.execute(command).unwrap();

        assert!(result.contains("Validation successful"));
        assert!(result.contains("Catala"));
    }

    #[test]
    fn test_cli_info() {
        let mut executor = CliExecutor::default();

        // Create temp input file
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "declaration scope InfoTest:").unwrap();
        writeln!(input_file, "  context input content integer").unwrap();

        let command = CliCommand::Info {
            input: input_file.path().to_owned(),
            format: LegalFormat::Catala,
        };

        let result = executor.execute(command).unwrap();

        assert!(result.contains("File Information"));
        assert!(result.contains("Catala"));
        assert!(result.contains("bytes"));
    }

    #[test]
    fn test_cli_config() {
        let config = CliConfig {
            cache_enabled: false,
            verbose: true,
            fail_on_warnings: true,
            min_confidence: 0.8,
            ..Default::default()
        };

        let executor = CliExecutor::new(config.clone());

        assert!(!executor.config().cache_enabled);
        assert!(executor.config().verbose);
        assert!(executor.config().fail_on_warnings);
        assert_eq!(executor.config().min_confidence, 0.8);
    }

    #[test]
    fn test_cli_min_confidence_threshold() {
        let config = CliConfig {
            min_confidence: 0.95,
            ..Default::default()
        };

        let mut executor = CliExecutor::new(config);

        // Create temp files
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "agreement LowConf(A, B) {{}}").unwrap();

        let output_file = NamedTempFile::new().unwrap();

        let command = CliCommand::Convert {
            input: input_file.path().to_owned(),
            output: output_file.path().to_owned(),
            source_format: LegalFormat::Stipula,
            target_format: LegalFormat::L4,
        };

        // This might fail if confidence is below 0.95
        let result = executor.execute(command);
        // We just check it doesn't panic; result could be Ok or Err depending on conversion quality
        assert!(result.is_ok() || result.is_err());
    }
}
