//! Pluggable block-compression codecs for record batches.
//!
//! The existing [`crate::compression`] module DEFLATE-compresses records *one at
//! a time*, which wastes the cross-record redundancy that dominates real audit
//! corpora (the same statute id, event type, and structural JSON keys repeat
//! across millions of rows). This module adds a [`Codec`] abstraction over
//! whole **blocks** of records and two implementations:
//!
//! - [`DeflateCodec`] — serialises the batch once and DEFLATEs it with the
//!   crate's existing `oxiarc-deflate` dependency. Sharing one DEFLATE window
//!   across the batch already beats per-record compression.
//! - [`ColumnarCodec`] — a documented, fully-reversible columnar transform
//!   applied *before* DEFLATE. `statute_id` and `event_type` are
//!   **dictionary**-encoded (distinct values stored once) with **run-length**
//!   encoded per-row indices (audit streams have long runs of the same statute
//!   / event type); `timestamp` is split into seconds (**delta + zig-zag
//!   varint**, since timestamps are near-monotonic) and sub-second nanoseconds
//!   (varint); every other field stays in a per-row JSON "residual". The
//!   transformed buffer is then DEFLATEd, so repeated strings and large
//!   timestamps are removed before the entropy coder ever runs, making the
//!   result materially smaller for large, repetitive batches.
//!
//! Both codecs round-trip exactly: decoding reproduces byte-for-byte the same
//! [`AuditRecord`] values (and therefore the same hash chain).

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use oxiarc_deflate::{deflate, inflate};
use serde_json::Value;
use std::collections::HashMap;

/// A compressed block of records produced by a [`Codec`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedBlock {
    /// Name of the codec that produced this block.
    pub codec: String,
    /// Number of records encoded.
    pub record_count: usize,
    /// Size of the canonical JSON serialisation of the batch (the ratio base).
    pub original_bytes: usize,
    /// Size of the encoded payload.
    pub encoded_bytes: usize,
    /// The encoded bytes.
    pub payload: Vec<u8>,
}

impl EncodedBlock {
    /// Compression ratio (`encoded / original`); 1.0 when there is no baseline.
    pub fn ratio(&self) -> f64 {
        if self.original_bytes == 0 {
            1.0
        } else {
            self.encoded_bytes as f64 / self.original_bytes as f64
        }
    }

    /// Bytes saved relative to the JSON baseline.
    pub fn space_saved_bytes(&self) -> usize {
        self.original_bytes.saturating_sub(self.encoded_bytes)
    }

    /// Percentage of the JSON baseline saved.
    pub fn space_saved_percent(&self) -> f64 {
        if self.original_bytes == 0 {
            0.0
        } else {
            self.space_saved_bytes() as f64 / self.original_bytes as f64 * 100.0
        }
    }
}

/// A block codec for batches of [`AuditRecord`]s.
pub trait Codec: Send + Sync {
    /// Stable codec name (also stored on the [`EncodedBlock`]).
    fn name(&self) -> &'static str;

    /// Encodes a batch of records.
    fn encode(&self, records: &[AuditRecord]) -> AuditResult<EncodedBlock>;

    /// Decodes a previously-encoded block.
    fn decode(&self, block: &EncodedBlock) -> AuditResult<Vec<AuditRecord>>;
}

/// Whole-batch DEFLATE codec.
#[derive(Debug, Clone, Copy)]
pub struct DeflateCodec {
    level: u8,
}

impl DeflateCodec {
    /// Creates a codec with a DEFLATE level (clamped to `0..=9`).
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(9),
        }
    }

    /// Balanced level (6).
    pub fn balanced() -> Self {
        Self::new(6)
    }

    /// Maximum compression (9).
    pub fn best() -> Self {
        Self::new(9)
    }
}

impl Default for DeflateCodec {
    fn default() -> Self {
        Self::balanced()
    }
}

impl Codec for DeflateCodec {
    fn name(&self) -> &'static str {
        "deflate-block"
    }

    fn encode(&self, records: &[AuditRecord]) -> AuditResult<EncodedBlock> {
        let json = serde_json::to_vec(records)?;
        let original_bytes = json.len();
        let payload = deflate(&json, self.level)
            .map_err(|e| AuditError::StorageError(format!("deflate failed: {e}")))?;
        Ok(EncodedBlock {
            codec: self.name().to_string(),
            record_count: records.len(),
            original_bytes,
            encoded_bytes: payload.len(),
            payload,
        })
    }

    fn decode(&self, block: &EncodedBlock) -> AuditResult<Vec<AuditRecord>> {
        let bytes = inflate(&block.payload)
            .map_err(|e| AuditError::StorageError(format!("inflate failed: {e}")))?;
        let records = serde_json::from_slice(&bytes)?;
        Ok(records)
    }
}

/// Columnar (dictionary + RLE + delta) codec, DEFLATEd after the transform.
#[derive(Debug, Clone, Copy)]
pub struct ColumnarCodec {
    level: u8,
}

const COLUMNAR_VERSION: u8 = 1;
const FLAG_TS_EXTRACTED: u8 = 0b0000_0001;

impl ColumnarCodec {
    /// Creates a codec with a DEFLATE level (clamped to `0..=9`).
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(9),
        }
    }

    /// Balanced level (6).
    pub fn balanced() -> Self {
        Self::new(6)
    }

    /// Maximum compression (9).
    pub fn best() -> Self {
        Self::new(9)
    }
}

impl Default for ColumnarCodec {
    fn default() -> Self {
        Self::balanced()
    }
}

impl Codec for ColumnarCodec {
    fn name(&self) -> &'static str {
        "columnar-deflate"
    }

    fn encode(&self, records: &[AuditRecord]) -> AuditResult<EncodedBlock> {
        let original_bytes = serde_json::to_vec(records)?.len();
        let count = records.len();

        // Build the dictionary-encoded string columns.
        let statutes: Vec<String> = records.iter().map(|r| r.statute_id.clone()).collect();
        let mut events: Vec<String> = Vec::with_capacity(count);
        for r in records {
            events.push(serde_json::to_string(&r.event_type)?);
        }
        let (statute_dict, statute_idx) = build_dict(&statutes);
        let (event_dict, event_idx) = build_dict(&events);

        // Per-row residual JSON with the columnarised fields stripped out.
        let mut residuals: Vec<Vec<u8>> = Vec::with_capacity(count);
        let mut secs: Vec<i64> = Vec::with_capacity(count);
        let mut nanos: Vec<u32> = Vec::with_capacity(count);
        for r in records {
            secs.push(r.timestamp.timestamp());
            nanos.push(r.timestamp.timestamp_subsec_nanos());

            let mut value = serde_json::to_value(r)?;
            let object = value.as_object_mut().ok_or_else(|| {
                AuditError::StorageError("record did not serialise to a JSON object".to_string())
            })?;
            object.remove("statute_id");
            object.remove("event_type");
            object.remove("timestamp");
            residuals.push(serde_json::to_vec(&value)?);
        }

        let mut buf = Vec::new();
        buf.push(COLUMNAR_VERSION);
        buf.push(FLAG_TS_EXTRACTED);
        write_uvarint(&mut buf, count as u64);

        write_dict(&mut buf, &statute_dict);
        write_rle_u32(&mut buf, &statute_idx);
        write_dict(&mut buf, &event_dict);
        write_rle_u32(&mut buf, &event_idx);

        // Timestamp seconds: delta + zig-zag varint. Nanoseconds: varint.
        let mut prev: i64 = 0;
        for s in &secs {
            write_uvarint(&mut buf, zigzag(s.wrapping_sub(prev)));
            prev = *s;
        }
        for n in &nanos {
            write_uvarint(&mut buf, *n as u64);
        }

        for residual in &residuals {
            write_uvarint(&mut buf, residual.len() as u64);
            buf.extend_from_slice(residual);
        }

        let payload = deflate(&buf, self.level)
            .map_err(|e| AuditError::StorageError(format!("deflate failed: {e}")))?;
        Ok(EncodedBlock {
            codec: self.name().to_string(),
            record_count: count,
            original_bytes,
            encoded_bytes: payload.len(),
            payload,
        })
    }

    fn decode(&self, block: &EncodedBlock) -> AuditResult<Vec<AuditRecord>> {
        let buf = inflate(&block.payload)
            .map_err(|e| AuditError::StorageError(format!("inflate failed: {e}")))?;
        let mut reader = Reader::new(&buf);

        let version = reader.read_u8()?;
        if version != COLUMNAR_VERSION {
            return Err(AuditError::StorageError(format!(
                "unsupported columnar version {version}"
            )));
        }
        let flags = reader.read_u8()?;
        let count = reader.read_uvarint()? as usize;

        let statute_dict = read_dict(&mut reader)?;
        let statute_idx = read_rle_u32(&mut reader, count)?;
        let event_dict = read_dict(&mut reader)?;
        let event_idx = read_rle_u32(&mut reader, count)?;

        let ts_extracted = flags & FLAG_TS_EXTRACTED != 0;
        let mut secs = Vec::with_capacity(count);
        let mut nanos = Vec::with_capacity(count);
        if ts_extracted {
            let mut prev: i64 = 0;
            for _ in 0..count {
                let delta = unzigzag(reader.read_uvarint()?);
                prev = prev.wrapping_add(delta);
                secs.push(prev);
            }
            for _ in 0..count {
                nanos.push(reader.read_uvarint()? as u32);
            }
        }

        let mut records = Vec::with_capacity(count);
        for i in 0..count {
            let len = reader.read_uvarint()? as usize;
            let bytes = reader.read_bytes(len)?;
            let mut value: Value = serde_json::from_slice(bytes)?;
            let object = value.as_object_mut().ok_or_else(|| {
                AuditError::StorageError("residual is not a JSON object".to_string())
            })?;

            let statute = statute_dict
                .get(statute_idx[i] as usize)
                .ok_or_else(|| AuditError::StorageError("statute dict index OOB".to_string()))?;
            object.insert("statute_id".to_string(), Value::String(statute.clone()));

            let event_text = event_dict
                .get(event_idx[i] as usize)
                .ok_or_else(|| AuditError::StorageError("event dict index OOB".to_string()))?;
            let event_value: Value = serde_json::from_str(event_text)?;
            object.insert("event_type".to_string(), event_value);

            if ts_extracted {
                let dt = DateTime::<Utc>::from_timestamp(secs[i], nanos[i]).ok_or_else(|| {
                    AuditError::StorageError("timestamp out of range".to_string())
                })?;
                object.insert("timestamp".to_string(), serde_json::to_value(dt)?);
            }

            records.push(serde_json::from_value(value)?);
        }
        Ok(records)
    }
}

/// A single row in a codec comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct CodecComparison {
    /// Codec name.
    pub name: String,
    /// JSON baseline size.
    pub original_bytes: usize,
    /// Encoded size.
    pub encoded_bytes: usize,
    /// Compression ratio.
    pub ratio: f64,
}

/// Encodes `records` with each codec and reports the resulting ratios, sorted
/// from smallest (best) ratio to largest — useful for picking the cheapest
/// codec for a given workload.
pub fn compare_codecs(
    records: &[AuditRecord],
    codecs: &[&dyn Codec],
) -> AuditResult<Vec<CodecComparison>> {
    let mut out = Vec::with_capacity(codecs.len());
    for codec in codecs {
        let block = codec.encode(records)?;
        out.push(CodecComparison {
            name: codec.name().to_string(),
            original_bytes: block.original_bytes,
            encoded_bytes: block.encoded_bytes,
            ratio: block.ratio(),
        });
    }
    out.sort_by(|a, b| a.ratio.total_cmp(&b.ratio));
    Ok(out)
}

// --- low-level helpers -----------------------------------------------------

fn build_dict(values: &[String]) -> (Vec<String>, Vec<u32>) {
    let mut dict: Vec<String> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut idx: Vec<u32> = Vec::with_capacity(values.len());
    for value in values {
        let id = match map.get(value) {
            Some(existing) => *existing,
            None => {
                let new_id = dict.len() as u32;
                dict.push(value.clone());
                map.insert(value.clone(), new_id);
                new_id
            }
        };
        idx.push(id);
    }
    (dict, idx)
}

fn write_uvarint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn zigzag(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

fn unzigzag(value: u64) -> i64 {
    ((value >> 1) as i64) ^ -((value & 1) as i64)
}

fn write_dict(out: &mut Vec<u8>, dict: &[String]) {
    write_uvarint(out, dict.len() as u64);
    for entry in dict {
        let bytes = entry.as_bytes();
        write_uvarint(out, bytes.len() as u64);
        out.extend_from_slice(bytes);
    }
}

fn read_dict(reader: &mut Reader<'_>) -> AuditResult<Vec<String>> {
    let len = reader.read_uvarint()? as usize;
    let mut dict = Vec::with_capacity(len);
    for _ in 0..len {
        let str_len = reader.read_uvarint()? as usize;
        let bytes = reader.read_bytes(str_len)?;
        let text = String::from_utf8(bytes.to_vec())
            .map_err(|e| AuditError::StorageError(format!("invalid utf8 in dict: {e}")))?;
        dict.push(text);
    }
    Ok(dict)
}

fn write_rle_u32(out: &mut Vec<u8>, values: &[u32]) {
    let mut runs: Vec<(u64, u64)> = Vec::new();
    let mut i = 0;
    while i < values.len() {
        let value = values[i];
        let mut j = i + 1;
        while j < values.len() && values[j] == value {
            j += 1;
        }
        runs.push(((j - i) as u64, value as u64));
        i = j;
    }
    write_uvarint(out, runs.len() as u64);
    for (run_len, value) in runs {
        write_uvarint(out, run_len);
        write_uvarint(out, value);
    }
}

fn read_rle_u32(reader: &mut Reader<'_>, expected: usize) -> AuditResult<Vec<u32>> {
    let run_count = reader.read_uvarint()? as usize;
    let mut out = Vec::with_capacity(expected);
    for _ in 0..run_count {
        let run_len = reader.read_uvarint()?;
        let value = reader.read_uvarint()? as u32;
        for _ in 0..run_len {
            out.push(value);
        }
    }
    if out.len() != expected {
        return Err(AuditError::StorageError(format!(
            "RLE length mismatch: expected {expected}, got {}",
            out.len()
        )));
    }
    Ok(out)
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_u8(&mut self) -> AuditResult<u8> {
        let byte = *self
            .data
            .get(self.pos)
            .ok_or_else(|| AuditError::StorageError("unexpected end of block".to_string()))?;
        self.pos += 1;
        Ok(byte)
    }

    fn read_uvarint(&mut self) -> AuditResult<u64> {
        let mut result: u64 = 0;
        let mut shift = 0u32;
        loop {
            let byte = self.read_u8()?;
            if shift >= 64 {
                return Err(AuditError::StorageError("varint overflow".to_string()));
            }
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        Ok(result)
    }

    fn read_bytes(&mut self, len: usize) -> AuditResult<&'a [u8]> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or_else(|| AuditError::StorageError("length overflow".to_string()))?;
        if end > self.data.len() {
            return Err(AuditError::StorageError(
                "unexpected end of block".to_string(),
            ));
        }
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::Duration;
    use std::collections::HashMap as Map;
    use uuid::Uuid;

    fn batch(n: usize) -> Vec<AuditRecord> {
        let base = Utc::now();
        (0..n)
            .map(|i| {
                let mut r = AuditRecord::new(
                    if i % 4 == 0 {
                        EventType::HumanOverride
                    } else {
                        EventType::AutomaticDecision
                    },
                    Actor::System {
                        component: "engine".to_string(),
                    },
                    format!("statute-{}", i % 3),
                    Uuid::new_v4(),
                    DecisionContext::default(),
                    DecisionResult::Deterministic {
                        effect_applied: "approved".to_string(),
                        parameters: Map::new(),
                    },
                    None,
                );
                r.timestamp = base + Duration::seconds(i as i64);
                // Re-derive the hash so the record is self-consistent after the
                // timestamp override (the hash binds the timestamp).
                r.relink(None);
                r
            })
            .collect()
    }

    fn assert_roundtrip(codec: &dyn Codec, records: &[AuditRecord]) {
        let block = codec.encode(records).expect("encode");
        assert_eq!(block.record_count, records.len());
        let decoded = codec.decode(&block).expect("decode");
        assert_eq!(decoded.len(), records.len());
        for (orig, got) in records.iter().zip(decoded.iter()) {
            assert_eq!(orig.id, got.id);
            assert_eq!(orig.statute_id, got.statute_id);
            assert_eq!(orig.subject_id, got.subject_id);
            assert_eq!(orig.timestamp, got.timestamp);
            assert_eq!(orig.record_hash, got.record_hash);
            assert_eq!(orig.previous_hash, got.previous_hash);
            assert!(got.verify());
        }
    }

    #[test]
    fn test_deflate_roundtrip() {
        assert_roundtrip(&DeflateCodec::balanced(), &batch(50));
    }

    #[test]
    fn test_columnar_roundtrip() {
        assert_roundtrip(&ColumnarCodec::balanced(), &batch(50));
    }

    #[test]
    fn test_columnar_roundtrip_best() {
        assert_roundtrip(&ColumnarCodec::best(), &batch(120));
    }

    #[test]
    fn test_empty_batch_roundtrips() {
        assert_roundtrip(&DeflateCodec::balanced(), &[]);
        assert_roundtrip(&ColumnarCodec::balanced(), &[]);
    }

    #[test]
    fn test_single_record_roundtrips() {
        assert_roundtrip(&ColumnarCodec::balanced(), &batch(1));
    }

    #[test]
    fn test_codecs_compress() {
        let records = batch(200);
        let deflate = DeflateCodec::balanced().encode(&records).expect("encode");
        let columnar = ColumnarCodec::balanced().encode(&records).expect("encode");
        // Both shrink the JSON baseline substantially.
        assert!(deflate.ratio() < 1.0);
        assert!(columnar.ratio() < 1.0);
        assert!(deflate.space_saved_percent() > 0.0);
    }

    #[test]
    fn test_columnar_beats_deflate_on_repetitive_data() {
        // Many records, few distinct statutes/event types: the columnar
        // dictionary+RLE transform should pay off.
        let records = batch(500);
        let deflate = DeflateCodec::best().encode(&records).expect("encode");
        let columnar = ColumnarCodec::best().encode(&records).expect("encode");
        assert!(
            columnar.encoded_bytes <= deflate.encoded_bytes,
            "columnar {} should not exceed deflate {}",
            columnar.encoded_bytes,
            deflate.encoded_bytes
        );
    }

    #[test]
    fn test_compare_codecs_sorted() {
        let records = batch(80);
        let deflate = DeflateCodec::balanced();
        let columnar = ColumnarCodec::balanced();
        let cmp = compare_codecs(&records, &[&deflate, &columnar]).expect("compare");
        assert_eq!(cmp.len(), 2);
        assert!(cmp[0].ratio <= cmp[1].ratio);
    }

    #[test]
    fn test_varint_zigzag_roundtrip() {
        for v in [0i64, 1, -1, 42, -42, i64::MAX, i64::MIN, 1_000_000_000] {
            assert_eq!(unzigzag(zigzag(v)), v);
        }
        let mut buf = Vec::new();
        for v in [0u64, 1, 127, 128, 300, u64::MAX] {
            write_uvarint(&mut buf, v);
        }
        let mut reader = Reader::new(&buf);
        for v in [0u64, 1, 127, 128, 300, u64::MAX] {
            assert_eq!(reader.read_uvarint().expect("read"), v);
        }
    }

    #[test]
    fn test_decode_rejects_truncated_block() {
        let records = batch(10);
        let mut block = ColumnarCodec::balanced().encode(&records).expect("encode");
        block.payload.truncate(block.payload.len() / 2);
        assert!(ColumnarCodec::balanced().decode(&block).is_err());
    }
}
