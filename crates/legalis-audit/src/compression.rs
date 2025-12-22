//! Record compression for storage efficiency.
//!
//! Provides DEFLATE compression for audit records to reduce storage footprint.

use crate::{AuditError, AuditRecord, AuditResult};
use flate2::Compression;
use flate2::read::{DeflateDecoder, DeflateEncoder};
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Compression level for records.
#[derive(Debug, Clone, Copy)]
pub enum CompressionLevel {
    /// No compression (fastest, largest)
    None,
    /// Fast compression (good speed, moderate size)
    Fast,
    /// Default compression (balanced)
    Default,
    /// Best compression (slower, smallest)
    Best,
}

impl CompressionLevel {
    /// Converts to flate2 compression level.
    fn to_flate2(self) -> Compression {
        match self {
            CompressionLevel::None => Compression::none(),
            CompressionLevel::Fast => Compression::fast(),
            CompressionLevel::Default => Compression::default(),
            CompressionLevel::Best => Compression::best(),
        }
    }
}

/// Compressed audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedRecord {
    /// Compressed data (DEFLATE)
    pub data: Vec<u8>,
    /// Original size (uncompressed)
    pub original_size: usize,
    /// Compressed size
    pub compressed_size: usize,
    /// Compression ratio (compressed / original)
    pub ratio: f64,
}

/// Record compressor.
pub struct RecordCompressor {
    level: CompressionLevel,
}

impl RecordCompressor {
    /// Creates a new compressor with the specified compression level.
    ///
    /// # Example
    /// ```
    /// use legalis_audit::compression::{RecordCompressor, CompressionLevel};
    ///
    /// let compressor = RecordCompressor::new(CompressionLevel::Default);
    /// ```
    pub fn new(level: CompressionLevel) -> Self {
        Self { level }
    }

    /// Creates a compressor with default compression.
    pub fn default_compression() -> Self {
        Self::new(CompressionLevel::Default)
    }

    /// Creates a compressor with best compression.
    pub fn best_compression() -> Self {
        Self::new(CompressionLevel::Best)
    }

    /// Compresses an audit record.
    pub fn compress(&self, record: &AuditRecord) -> AuditResult<CompressedRecord> {
        // Serialize to JSON
        let json = serde_json::to_vec(record)?;
        let original_size = json.len();

        // Compress
        let mut encoder = DeflateEncoder::new(json.as_slice(), self.level.to_flate2());
        let mut compressed = Vec::new();
        encoder
            .read_to_end(&mut compressed)
            .map_err(|e| AuditError::StorageError(format!("Compression failed: {}", e)))?;

        let compressed_size = compressed.len();
        let ratio = compressed_size as f64 / original_size as f64;

        Ok(CompressedRecord {
            data: compressed,
            original_size,
            compressed_size,
            ratio,
        })
    }

    /// Decompresses a compressed record.
    pub fn decompress(&self, compressed: &CompressedRecord) -> AuditResult<AuditRecord> {
        // Decompress
        let mut decoder = DeflateDecoder::new(compressed.data.as_slice());
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| AuditError::StorageError(format!("Decompression failed: {}", e)))?;

        // Deserialize
        let record: AuditRecord = serde_json::from_slice(&decompressed)?;
        Ok(record)
    }

    /// Compresses multiple records in batch.
    pub fn compress_batch(&self, records: &[AuditRecord]) -> AuditResult<Vec<CompressedRecord>> {
        records.iter().map(|r| self.compress(r)).collect()
    }

    /// Decompresses multiple records in batch.
    pub fn decompress_batch(
        &self,
        compressed: &[CompressedRecord],
    ) -> AuditResult<Vec<AuditRecord>> {
        compressed.iter().map(|c| self.decompress(c)).collect()
    }

    /// Returns compression statistics for a batch of records.
    pub fn compression_stats(&self, records: &[AuditRecord]) -> AuditResult<CompressionStats> {
        let compressed = self.compress_batch(records)?;

        let total_original: usize = compressed.iter().map(|c| c.original_size).sum();
        let total_compressed: usize = compressed.iter().map(|c| c.compressed_size).sum();
        let average_ratio =
            compressed.iter().map(|c| c.ratio).sum::<f64>() / compressed.len() as f64;
        let space_saved = total_original.saturating_sub(total_compressed);
        let space_saved_percent = (space_saved as f64 / total_original as f64) * 100.0;

        Ok(CompressionStats {
            record_count: records.len(),
            total_original_bytes: total_original,
            total_compressed_bytes: total_compressed,
            average_ratio,
            space_saved_bytes: space_saved,
            space_saved_percent,
        })
    }
}

impl Default for RecordCompressor {
    fn default() -> Self {
        Self::default_compression()
    }
}

/// Compression statistics.
#[derive(Debug, Clone)]
pub struct CompressionStats {
    /// Number of records
    pub record_count: usize,
    /// Total original size in bytes
    pub total_original_bytes: usize,
    /// Total compressed size in bytes
    pub total_compressed_bytes: usize,
    /// Average compression ratio
    pub average_ratio: f64,
    /// Space saved in bytes
    pub space_saved_bytes: usize,
    /// Space saved as percentage
    pub space_saved_percent: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record(statute_id: &str) -> AuditRecord {
        let mut context = DecisionContext::default();
        context
            .attributes
            .insert("key1".to_string(), "value1".to_string());
        context
            .attributes
            .insert("key2".to_string(), "value2".to_string());
        context
            .metadata
            .insert("meta1".to_string(), "metadata1".to_string());

        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            context,
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_compression_decompression() {
        let compressor = RecordCompressor::default_compression();
        let record = create_test_record("statute-1");
        let record_id = record.id;

        let compressed = compressor.compress(&record).unwrap();
        assert!(compressed.compressed_size > 0);
        assert!(compressed.compressed_size < compressed.original_size);
        assert!(compressed.ratio < 1.0);

        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed.id, record_id);
        assert_eq!(decompressed.statute_id, record.statute_id);
    }

    #[test]
    fn test_compression_levels() {
        let record = create_test_record("statute-1");

        let fast = RecordCompressor::new(CompressionLevel::Fast);
        let default = RecordCompressor::default_compression();
        let best = RecordCompressor::best_compression();

        let fast_compressed = fast.compress(&record).unwrap();
        let default_compressed = default.compress(&record).unwrap();
        let best_compressed = best.compress(&record).unwrap();

        // Best compression should produce smaller output
        assert!(best_compressed.compressed_size <= default_compressed.compressed_size);
        assert!(default_compressed.compressed_size <= fast_compressed.compressed_size);

        // All should decompress correctly
        assert_eq!(fast.decompress(&fast_compressed).unwrap().id, record.id);
        assert_eq!(
            default.decompress(&default_compressed).unwrap().id,
            record.id
        );
        assert_eq!(best.decompress(&best_compressed).unwrap().id, record.id);
    }

    #[test]
    fn test_batch_compression() {
        let compressor = RecordCompressor::default_compression();
        let records: Vec<_> = (0..10)
            .map(|i| create_test_record(&format!("statute-{}", i)))
            .collect();

        let compressed = compressor.compress_batch(&records).unwrap();
        assert_eq!(compressed.len(), 10);

        let decompressed = compressor.decompress_batch(&compressed).unwrap();
        assert_eq!(decompressed.len(), 10);

        for (original, decompressed) in records.iter().zip(decompressed.iter()) {
            assert_eq!(original.id, decompressed.id);
            assert_eq!(original.statute_id, decompressed.statute_id);
        }
    }

    #[test]
    fn test_compression_stats() {
        let compressor = RecordCompressor::default_compression();
        let records: Vec<_> = (0..20)
            .map(|i| create_test_record(&format!("statute-{}", i)))
            .collect();

        let stats = compressor.compression_stats(&records).unwrap();
        assert_eq!(stats.record_count, 20);
        assert!(stats.total_compressed_bytes < stats.total_original_bytes);
        assert!(stats.space_saved_bytes > 0);
        assert!(stats.space_saved_percent > 0.0);
        assert!(stats.space_saved_percent < 100.0);
        assert!(stats.average_ratio > 0.0);
        assert!(stats.average_ratio < 1.0);
    }

    #[test]
    fn test_no_compression() {
        let compressor = RecordCompressor::new(CompressionLevel::None);
        let record = create_test_record("statute-1");

        let compressed = compressor.compress(&record).unwrap();
        // With no compression, sizes should be very similar
        // (DEFLATE still adds a small overhead even with no compression)
        assert!(compressed.ratio <= 1.1);

        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed.id, record.id);
    }
}
