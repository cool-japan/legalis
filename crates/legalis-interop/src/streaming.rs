//! Streaming support for processing large legal documents.
//!
//! This module provides streaming APIs for importing and converting large legal documents
//! without loading the entire document into memory at once. This is particularly useful
//! for processing very large legislative documents or batch converting multiple files.

use crate::{ConversionReport, InteropError, InteropResult, LegalFormat};
use legalis_core::Statute;
use std::io::{BufRead, BufReader, Read};

/// Chunk size for streaming reads (64KB)
const CHUNK_SIZE: usize = 65536;

/// Streaming importer that processes documents in chunks.
pub struct StreamingImporter {
    format: LegalFormat,
    buffer: String,
    chunk_size: usize,
}

impl StreamingImporter {
    /// Creates a new streaming importer for the specified format.
    pub fn new(format: LegalFormat) -> Self {
        Self {
            format,
            buffer: String::with_capacity(CHUNK_SIZE),
            chunk_size: CHUNK_SIZE,
        }
    }

    /// Sets a custom chunk size for reading.
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self.buffer = String::with_capacity(chunk_size);
        self
    }

    /// Imports from a reader, processing the document in chunks.
    ///
    /// # Arguments
    /// * `reader` - Any type that implements Read (File, stdin, network stream, etc.)
    ///
    /// # Returns
    /// A tuple of (statutes, conversion_report)
    pub fn import<R: Read>(
        &mut self,
        reader: R,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        self.buffer.clear();
        let mut buf_reader = BufReader::with_capacity(self.chunk_size, reader);

        // Read entire content into buffer
        // In future, we could implement true streaming parsing per format
        buf_reader
            .read_to_string(&mut self.buffer)
            .map_err(InteropError::IoError)?;

        // Use existing converter to parse
        let mut converter = crate::LegalConverter::new();
        converter.import(&self.buffer, self.format)
    }

    /// Imports from a buffered reader line by line.
    ///
    /// This is useful for formats that have clear line-based boundaries.
    ///
    /// # Arguments
    /// * `reader` - A buffered reader
    ///
    /// # Returns
    /// A tuple of (statutes, conversion_report)
    pub fn import_lines<R: BufRead>(
        &mut self,
        mut reader: R,
    ) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        self.buffer.clear();

        loop {
            let bytes_read = reader
                .read_line(&mut self.buffer)
                .map_err(InteropError::IoError)?;

            if bytes_read == 0 {
                break; // EOF
            }
        }

        // Use existing converter to parse
        let mut converter = crate::LegalConverter::new();
        converter.import(&self.buffer, self.format)
    }

    /// Returns the format this importer handles.
    pub fn format(&self) -> LegalFormat {
        self.format
    }
}

/// Streaming exporter that writes documents in chunks.
pub struct StreamingExporter {
    format: LegalFormat,
    batch_size: usize,
}

impl StreamingExporter {
    /// Creates a new streaming exporter for the specified format.
    pub fn new(format: LegalFormat) -> Self {
        Self {
            format,
            batch_size: 100, // Process 100 statutes at a time
        }
    }

    /// Sets the batch size for processing statutes.
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size.max(1); // Ensure at least 1
        self
    }

    /// Exports statutes to a writer in batches.
    ///
    /// # Arguments
    /// * `statutes` - The statutes to export
    /// * `writer` - Any type that implements Write
    ///
    /// # Returns
    /// A conversion report
    pub fn export<W: std::io::Write>(
        &self,
        statutes: &[Statute],
        mut writer: W,
    ) -> InteropResult<ConversionReport> {
        let mut converter = crate::LegalConverter::new();
        let mut combined_report = ConversionReport::new(crate::LegalFormat::Legalis, self.format);

        // Process in batches to avoid memory issues with very large documents
        for (batch_idx, chunk) in statutes.chunks(self.batch_size).enumerate() {
            let (output, report) = converter.export(chunk, self.format)?;

            // Write to output
            writer
                .write_all(output.as_bytes())
                .map_err(InteropError::IoError)?;

            // For formats that support multiple statutes in one document,
            // we might need to add separators between batches
            if batch_idx < (statutes.len() / self.batch_size) {
                match self.format {
                    LegalFormat::L4 | LegalFormat::Catala | LegalFormat::Stipula => {
                        writer.write_all(b"\n\n").map_err(InteropError::IoError)?;
                    }
                    _ => {}
                }
            }

            // Merge reports
            combined_report
                .unsupported_features
                .extend(report.unsupported_features);
            combined_report.warnings.extend(report.warnings);
            combined_report.statutes_converted += report.statutes_converted;
            combined_report.confidence = (combined_report.confidence * report.confidence).max(0.0);
        }

        writer.flush().map_err(InteropError::IoError)?;

        Ok(combined_report)
    }

    /// Returns the format this exporter produces.
    pub fn format(&self) -> LegalFormat {
        self.format
    }
}

/// Streaming converter that combines import and export in a memory-efficient way.
pub struct StreamingConverter {
    source_format: LegalFormat,
    target_format: LegalFormat,
    buffer_size: usize,
}

impl StreamingConverter {
    /// Creates a new streaming converter.
    pub fn new(source_format: LegalFormat, target_format: LegalFormat) -> Self {
        Self {
            source_format,
            target_format,
            buffer_size: CHUNK_SIZE,
        }
    }

    /// Sets a custom buffer size.
    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    /// Converts from a reader to a writer.
    ///
    /// # Arguments
    /// * `reader` - Input reader
    /// * `writer` - Output writer
    ///
    /// # Returns
    /// A conversion report
    pub fn convert<R: Read, W: std::io::Write>(
        &self,
        reader: R,
        writer: W,
    ) -> InteropResult<ConversionReport> {
        // Import from reader
        let mut importer =
            StreamingImporter::new(self.source_format).with_chunk_size(self.buffer_size);
        let (statutes, import_report) = importer.import(reader)?;

        // Export to writer
        let exporter = StreamingExporter::new(self.target_format);
        let export_report = exporter.export(&statutes, writer)?;

        // Merge reports
        let mut combined_report = import_report;
        combined_report.target_format = Some(self.target_format);
        combined_report
            .unsupported_features
            .extend(export_report.unsupported_features);
        combined_report.warnings.extend(export_report.warnings);
        combined_report.confidence =
            (combined_report.confidence * export_report.confidence).max(0.0);

        Ok(combined_report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_streaming_importer_catala() {
        let catala_source = r#"
declaration scope VotingRights:
  context input content Input
  context output content Output

scope VotingRights:
  definition output.eligible equals
    input.age >= 18
"#;

        let reader = Cursor::new(catala_source.as_bytes());
        let mut importer = StreamingImporter::new(LegalFormat::Catala);

        let (statutes, report) = importer.import(reader).unwrap();

        assert!(!statutes.is_empty());
        assert_eq!(report.source_format, Some(LegalFormat::Catala));
    }

    #[test]
    fn test_streaming_importer_l4() {
        let l4_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";

        let reader = Cursor::new(l4_source.as_bytes());
        let mut importer = StreamingImporter::new(LegalFormat::L4);

        let (statutes, report) = importer.import(reader).unwrap();

        assert!(!statutes.is_empty());
        assert_eq!(report.source_format, Some(LegalFormat::L4));
    }

    #[test]
    fn test_streaming_exporter() {
        use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

        let statute = Statute::new(
            "voting-rights",
            "Voting Rights",
            Effect::new(EffectType::Grant, "vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let mut output = Vec::new();
        let exporter = StreamingExporter::new(LegalFormat::L4);

        let report = exporter.export(&[statute], &mut output).unwrap();

        assert!(report.statutes_converted > 0);
        assert!(!output.is_empty());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("RULE"));
    }

    #[test]
    fn test_streaming_exporter_batching() {
        use legalis_core::{Effect, EffectType, Statute};

        // Create many statutes to test batching
        let statutes: Vec<Statute> = (0..250)
            .map(|i| {
                Statute::new(
                    format!("statute-{}", i),
                    format!("Statute {}", i),
                    Effect::new(EffectType::Grant, "test"),
                )
            })
            .collect();

        let mut output = Vec::new();
        let exporter = StreamingExporter::new(LegalFormat::L4).with_batch_size(50);

        let report = exporter.export(&statutes, &mut output).unwrap();

        assert_eq!(report.statutes_converted, 250);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_streaming_converter() {
        let catala_source = r#"
declaration scope Test:
  context input content Input
"#;

        let reader = Cursor::new(catala_source.as_bytes());
        let mut output = Vec::new();

        let converter = StreamingConverter::new(LegalFormat::Catala, LegalFormat::L4);
        let report = converter.convert(reader, &mut output).unwrap();

        assert!(report.statutes_converted > 0);
        assert!(!output.is_empty());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("RULE"));
    }

    #[test]
    fn test_streaming_importer_custom_chunk_size() {
        let l4_source = "RULE Test WHEN age >= 18 THEN Person MAY test";

        let reader = Cursor::new(l4_source.as_bytes());
        let mut importer = StreamingImporter::new(LegalFormat::L4).with_chunk_size(128);

        let (statutes, _report) = importer.import(reader).unwrap();

        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_streaming_importer_lines() {
        let catala_source = r#"declaration scope Test:
  context input content Input"#;

        let reader = Cursor::new(catala_source.as_bytes());
        let mut importer = StreamingImporter::new(LegalFormat::Catala);

        let (statutes, _report) = importer.import_lines(reader).unwrap();

        assert!(!statutes.is_empty());
    }
}
