//! Async conversion APIs for non-blocking I/O operations.
//!
//! This module provides async versions of the conversion APIs, useful for:
//! - Web servers and async applications
//! - Processing multiple files concurrently
//! - Non-blocking file I/O operations
//!
//! Requires the `async` feature to be enabled.

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;

/// Async wrapper for LegalConverter with async file I/O support.
pub struct AsyncConverter {
    converter: LegalConverter,
}

impl AsyncConverter {
    /// Creates a new async converter.
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
        }
    }

    /// Creates a new async converter with caching enabled.
    pub fn with_cache(cache_size: usize) -> Self {
        Self {
            converter: LegalConverter::with_cache(cache_size),
        }
    }

    /// Imports from a file asynchronously.
    ///
    /// # Arguments
    /// * `path` - Path to the input file
    /// * `format` - Format of the input file
    ///
    /// # Returns
    /// A tuple of (statutes, conversion_report)
    pub async fn import_file(
        &mut self,
        path: &str,
        format: LegalFormat,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let contents = tokio::fs::read_to_string(path)
            .await
            .map_err(InteropError::IoError)?;

        self.converter.import(&contents, format)
    }

    /// Exports to a file asynchronously.
    ///
    /// # Arguments
    /// * `statutes` - Statutes to export
    /// * `path` - Path to the output file
    /// * `format` - Target format
    ///
    /// # Returns
    /// A conversion report
    pub async fn export_file(
        &mut self,
        statutes: &[Statute],
        path: &str,
        format: LegalFormat,
    ) -> InteropResult<ConversionReport> {
        let (output, report) = self.converter.export(statutes, format)?;

        tokio::fs::write(path, output.as_bytes())
            .await
            .map_err(InteropError::IoError)?;

        Ok(report)
    }

    /// Converts a file to another format asynchronously.
    ///
    /// # Arguments
    /// * `input_path` - Path to input file
    /// * `output_path` - Path to output file
    /// * `source_format` - Format of input file
    /// * `target_format` - Desired output format
    ///
    /// # Returns
    /// A conversion report
    pub async fn convert_file(
        &mut self,
        input_path: &str,
        output_path: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> InteropResult<ConversionReport> {
        let (statutes, import_report) = self.import_file(input_path, source_format).await?;
        let mut export_report = self
            .export_file(&statutes, output_path, target_format)
            .await?;

        // Merge reports
        export_report.source_format = import_report.source_format;
        export_report
            .unsupported_features
            .extend(import_report.unsupported_features);
        export_report.warnings.extend(import_report.warnings);
        export_report.confidence = (import_report.confidence * export_report.confidence).max(0.0);

        Ok(export_report)
    }

    /// Batch converts multiple files asynchronously.
    ///
    /// Processes all files concurrently for maximum performance.
    ///
    /// # Arguments
    /// * `conversions` - Vector of (input_path, output_path, source_format, target_format) tuples
    ///
    /// # Returns
    /// Vector of conversion reports
    pub async fn batch_convert_files(
        &mut self,
        conversions: Vec<(String, String, LegalFormat, LegalFormat)>,
    ) -> Vec<InteropResult<ConversionReport>> {
        let mut tasks = Vec::new();

        for (input, output, source_fmt, target_fmt) in conversions {
            // Create a new converter for each task to avoid mutable borrow issues
            let mut converter = AsyncConverter::new();
            let task = tokio::spawn(async move {
                converter
                    .convert_file(&input, &output, source_fmt, target_fmt)
                    .await
            });
            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(InteropError::ConversionError(format!(
                    "Task join error: {}",
                    e
                )))),
            }
        }

        results
    }

    /// Imports from a string (non-async, convenience method).
    pub fn import(
        &mut self,
        source: &str,
        format: LegalFormat,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        self.converter.import(source, format)
    }

    /// Exports to a string (non-async, convenience method).
    pub fn export(
        &mut self,
        statutes: &[Statute],
        format: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        self.converter.export(statutes, format)
    }

    /// Converts between formats (non-async, convenience method).
    pub fn convert(
        &mut self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
    ) -> InteropResult<(String, ConversionReport)> {
        self.converter.convert(source, from, to)
    }

    /// Enables caching with the specified capacity.
    pub fn enable_cache(&mut self, cache_size: usize) {
        self.converter.enable_cache(cache_size);
    }

    /// Disables caching.
    pub fn disable_cache(&mut self) {
        self.converter.disable_cache();
    }

    /// Clears the cache if enabled.
    pub fn clear_cache(&mut self) {
        self.converter.clear_cache();
    }

    /// Returns cache statistics if caching is enabled.
    pub fn cache_stats(&self) -> Option<crate::cache::CacheStats> {
        self.converter.cache_stats()
    }
}

impl Default for AsyncConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_async_import_export_file() {
        let mut converter = AsyncConverter::new();

        // Create a temp file
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("test_input.catala_en");
        let output_path = temp_dir.join("test_output.l4");

        let catala_source = r#"
declaration scope VotingRights:
  context input content Input
  context output content Output

scope VotingRights:
  definition output.eligible equals
    input.age >= 18
"#;

        // Write input file
        fs::write(&input_path, catala_source).await.unwrap();

        // Import
        let (statutes, import_report) = converter
            .import_file(input_path.to_str().unwrap(), LegalFormat::Catala)
            .await
            .unwrap();

        assert!(!statutes.is_empty());
        assert_eq!(import_report.source_format, Some(LegalFormat::Catala));

        // Export
        let export_report = converter
            .export_file(&statutes, output_path.to_str().unwrap(), LegalFormat::L4)
            .await
            .unwrap();

        assert!(export_report.statutes_converted > 0);

        // Verify output file exists
        assert!(output_path.exists());

        // Cleanup
        fs::remove_file(&input_path).await.ok();
        fs::remove_file(&output_path).await.ok();
    }

    #[tokio::test]
    async fn test_async_convert_file() {
        let mut converter = AsyncConverter::new();

        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("test_convert_input.l4");
        let output_path = temp_dir.join("test_convert_output.catala_en");

        let l4_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";

        // Write input file
        fs::write(&input_path, l4_source).await.unwrap();

        // Convert
        let report = converter
            .convert_file(
                input_path.to_str().unwrap(),
                output_path.to_str().unwrap(),
                LegalFormat::L4,
                LegalFormat::Catala,
            )
            .await
            .unwrap();

        assert!(report.statutes_converted > 0);
        assert_eq!(report.source_format, Some(LegalFormat::L4));
        assert_eq!(report.target_format, Some(LegalFormat::Catala));

        // Verify output file exists
        assert!(output_path.exists());

        // Cleanup
        fs::remove_file(&input_path).await.ok();
        fs::remove_file(&output_path).await.ok();
    }

    #[tokio::test]
    async fn test_async_batch_convert_files() {
        let temp_dir = std::env::temp_dir();

        // Create test files
        let conversions = vec![
            (
                temp_dir
                    .join("batch1_input.catala_en")
                    .to_string_lossy()
                    .to_string(),
                temp_dir
                    .join("batch1_output.l4")
                    .to_string_lossy()
                    .to_string(),
                LegalFormat::Catala,
                LegalFormat::L4,
            ),
            (
                temp_dir
                    .join("batch2_input.l4")
                    .to_string_lossy()
                    .to_string(),
                temp_dir
                    .join("batch2_output.stipula")
                    .to_string_lossy()
                    .to_string(),
                LegalFormat::L4,
                LegalFormat::Stipula,
            ),
        ];

        // Write input files
        fs::write(
            &conversions[0].0,
            "declaration scope Test1:\n  context input content integer",
        )
        .await
        .unwrap();
        fs::write(
            &conversions[1].0,
            "RULE Test2 WHEN age >= 21 THEN Person MAY test",
        )
        .await
        .unwrap();

        // Batch convert
        let mut converter = AsyncConverter::new();
        let results = converter.batch_convert_files(conversions.clone()).await;

        assert_eq!(results.len(), 2);
        for result in &results {
            assert!(result.is_ok());
        }

        // Cleanup
        for (input, output, _, _) in conversions {
            fs::remove_file(&input).await.ok();
            fs::remove_file(&output).await.ok();
        }
    }

    #[tokio::test]
    async fn test_async_converter_with_cache() {
        let mut converter = AsyncConverter::with_cache(10);

        let l4_source = "RULE Test WHEN age >= 18 THEN Person MAY test";

        // First conversion - cache miss
        let (output1, _) = converter
            .convert(l4_source, LegalFormat::L4, LegalFormat::Catala)
            .unwrap();

        // Second conversion - cache hit
        let (output2, _) = converter
            .convert(l4_source, LegalFormat::L4, LegalFormat::Catala)
            .unwrap();

        assert_eq!(output1, output2);

        // Verify cache stats
        let stats = converter.cache_stats();
        assert!(stats.is_some());
    }

    #[tokio::test]
    async fn test_async_converter_default() {
        let converter = AsyncConverter::default();
        assert!(converter.cache_stats().is_none());
    }
}
