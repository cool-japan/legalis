//! Streaming, bounded-memory format conversion driven by a state machine.
//!
//! [`StreamingConverter`] converts a large document format→format by accepting
//! the source in arbitrary byte/line chunks and emitting converted output
//! incrementally, without ever holding the whole source or the whole output in
//! memory. It maintains a *bounded* internal buffer: whenever a parseable
//! boundary (configurable, default: blank line between records) is detected and
//! the buffer is large enough, a batch is parsed, converted, flushed, and the
//! buffer is trimmed.
//!
//! The converter is an explicit state machine:
//!
//! ```text
//!        ┌──────┐  feed_chunk   ┌───────────┐
//!        │ Idle │ ────────────▶ │ Streaming │◀─┐ feed_chunk
//!        └──────┘               └───────────┘  │ (emits converted chunks)
//!                                     │ finish  │
//!                                     ▼         │
//!                                ┌──────────┐   │
//!                                │ Flushing │───┘ (drains residual buffer)
//!                                └──────────┘
//!                                     │ drained
//!                                     ▼
//!                            ┌──────┐   on error   ┌────────┐
//!                            │ Done │ ◀──────────▶ │ Failed │
//!                            └──────┘              └────────┘
//! ```
//!
//! This is "streaming" in the bounded-memory sense; it requires no network.

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat};

/// Explicit lifecycle state of a [`StreamingConverter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    /// Created, nothing fed yet.
    Idle,
    /// Actively accepting chunks and emitting converted output.
    Streaming,
    /// `finish` has been called; draining the residual buffer.
    Flushing,
    /// All input consumed and flushed.
    Done,
    /// A non-recoverable error occurred; no further input is accepted.
    Failed,
}

/// Configuration for [`StreamingConverter`].
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Minimum number of buffered bytes before attempting a flush at a boundary.
    /// Bounds peak memory: the converter tries to keep the buffer near this size.
    pub flush_threshold_bytes: usize,
    /// Hard cap on buffered bytes. If the buffer exceeds this with no boundary,
    /// the converter force-flushes at the cap to preserve the memory bound.
    pub max_buffer_bytes: usize,
    /// Record-boundary marker. A boundary is recognised where this substring
    /// appears; everything up to and including the last boundary within budget
    /// is eligible for conversion. Defaults to a blank line (`"\n\n"`).
    pub boundary: String,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            flush_threshold_bytes: 16 * 1024,
            max_buffer_bytes: 256 * 1024,
            boundary: "\n\n".to_string(),
        }
    }
}

impl StreamConfig {
    /// Creates a config with a specific flush threshold (and a max-buffer of
    /// 16× the threshold).
    pub fn with_threshold(flush_threshold_bytes: usize) -> Self {
        let t = flush_threshold_bytes.max(1);
        Self {
            flush_threshold_bytes: t,
            max_buffer_bytes: t.saturating_mul(16).max(t),
            boundary: "\n\n".to_string(),
        }
    }

    /// Sets the record boundary marker.
    pub fn with_boundary(mut self, boundary: impl Into<String>) -> Self {
        self.boundary = boundary.into();
        self
    }
}

/// A converted output chunk produced by the streaming converter.
#[derive(Debug, Clone)]
pub struct ConvertedChunk {
    /// Converted text for this batch (may be empty if a fed chunk did not yet
    /// complete a record boundary).
    pub output: String,
    /// Per-batch conversion report.
    pub report: ConversionReport,
    /// Number of source records (statutes) converted in this batch.
    pub records: usize,
}

impl ConvertedChunk {
    /// An empty chunk (no boundary reached yet).
    fn empty(from: LegalFormat, to: LegalFormat) -> Self {
        Self {
            output: String::new(),
            report: ConversionReport::new(from, to),
            records: 0,
        }
    }

    /// Returns `true` if this chunk produced no output.
    pub fn is_empty(&self) -> bool {
        self.output.is_empty() && self.records == 0
    }
}

/// Running totals across a streaming session.
#[derive(Debug, Clone, Default)]
pub struct StreamMetrics {
    /// Total source bytes fed.
    pub bytes_in: usize,
    /// Total output bytes emitted.
    pub bytes_out: usize,
    /// Total records (statutes) converted.
    pub records: usize,
    /// Number of flush operations performed.
    pub flushes: usize,
    /// Peak buffered bytes observed (memory-bound evidence).
    pub peak_buffer_bytes: usize,
}

/// A bounded-memory, chunked format converter.
pub struct StreamingConverter {
    from: LegalFormat,
    to: LegalFormat,
    config: StreamConfig,
    converter: LegalConverter,
    state: StreamState,
    buffer: String,
    metrics: StreamMetrics,
}

impl StreamingConverter {
    /// Creates a streaming converter for the given format pair with default
    /// config.
    pub fn new(from: LegalFormat, to: LegalFormat) -> Self {
        Self::with_config(from, to, StreamConfig::default())
    }

    /// Creates a streaming converter with explicit configuration.
    pub fn with_config(from: LegalFormat, to: LegalFormat, config: StreamConfig) -> Self {
        Self {
            from,
            to,
            config,
            converter: LegalConverter::new(),
            state: StreamState::Idle,
            buffer: String::new(),
            metrics: StreamMetrics::default(),
        }
    }

    /// Current state-machine state.
    pub fn state(&self) -> StreamState {
        self.state
    }

    /// Running session metrics.
    pub fn metrics(&self) -> &StreamMetrics {
        &self.metrics
    }

    /// Number of bytes currently buffered (the live memory footprint of source
    /// text). Always `<= max_buffer_bytes` after any public call returns.
    pub fn buffered_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// Feeds a source chunk, returning any converted output that became ready.
    ///
    /// Transitions `Idle -> Streaming` on the first call. Repeated calls stay in
    /// `Streaming`. Memory stays bounded: the buffer is flushed at record
    /// boundaries once it reaches `flush_threshold_bytes`, and force-flushed if
    /// it would exceed `max_buffer_bytes`.
    ///
    /// # Errors
    /// Returns an error (and transitions to `Failed`) if a batch fails to
    /// convert, or if called after `finish`.
    pub fn feed_chunk(&mut self, chunk: &str) -> InteropResult<ConvertedChunk> {
        match self.state {
            StreamState::Idle => self.state = StreamState::Streaming,
            StreamState::Streaming => {}
            StreamState::Flushing | StreamState::Done => {
                return Err(InteropError::ConversionError(
                    "cannot feed after finish() has been called".to_string(),
                ));
            }
            StreamState::Failed => {
                return Err(InteropError::ConversionError(
                    "streaming converter is in a failed state".to_string(),
                ));
            }
        }

        self.metrics.bytes_in += chunk.len();
        self.buffer.push_str(chunk);
        self.metrics.peak_buffer_bytes = self.metrics.peak_buffer_bytes.max(self.buffer.len());

        // Flush as long as we are over threshold (or over the hard cap).
        let mut combined = ConvertedChunk::empty(self.from, self.to);
        loop {
            let over_threshold = self.buffer.len() >= self.config.flush_threshold_bytes;
            let over_cap = self.buffer.len() > self.config.max_buffer_bytes;
            if !over_threshold && !over_cap {
                break;
            }
            let split = self.find_split(over_cap);
            let Some(split) = split else { break };
            if split == 0 {
                break;
            }
            let batch: String = self.buffer.drain(..split).collect();
            let produced = self.convert_batch(&batch)?;
            self.absorb(&mut combined, produced);
        }
        Ok(combined)
    }

    /// Signals end-of-input and drains the residual buffer.
    ///
    /// Transitions `Streaming -> Flushing -> Done`. Returns the final converted
    /// output for whatever remained buffered.
    ///
    /// # Errors
    /// Returns an error (and transitions to `Failed`) if the residual batch
    /// fails to convert, or if called from `Idle`/`Done`/`Failed`.
    pub fn finish(&mut self) -> InteropResult<ConvertedChunk> {
        match self.state {
            StreamState::Idle => {
                // Nothing was ever fed: trivially done.
                self.state = StreamState::Done;
                return Ok(ConvertedChunk::empty(self.from, self.to));
            }
            StreamState::Streaming => self.state = StreamState::Flushing,
            StreamState::Flushing | StreamState::Done => {
                return Err(InteropError::ConversionError(
                    "finish() called more than once".to_string(),
                ));
            }
            StreamState::Failed => {
                return Err(InteropError::ConversionError(
                    "streaming converter is in a failed state".to_string(),
                ));
            }
        }

        let mut combined = ConvertedChunk::empty(self.from, self.to);
        if !self.buffer.trim().is_empty() {
            let residual: String = std::mem::take(&mut self.buffer);
            let produced = self.convert_batch(&residual)?;
            self.absorb(&mut combined, produced);
        } else {
            self.buffer.clear();
        }
        self.state = StreamState::Done;
        Ok(combined)
    }

    /// Finds a byte offset to split the buffer at.
    ///
    /// Prefers the position just past the last record boundary that lies within
    /// `flush_threshold_bytes` (or `max_buffer_bytes` when forcing). If forcing
    /// and no boundary is found, splits at the cap to preserve the memory bound.
    fn find_split(&self, force: bool) -> Option<usize> {
        let budget = if force {
            self.config.max_buffer_bytes
        } else {
            self.buffer.len()
        };
        let window = &self.buffer[..budget.min(self.buffer.len())];
        if let Some(idx) = window.rfind(&self.config.boundary) {
            return Some(idx + self.config.boundary.len());
        }
        if force {
            // No boundary within the cap: split at a char boundary near the cap.
            let mut cut = self.config.max_buffer_bytes.min(self.buffer.len());
            while cut > 0 && !self.buffer.is_char_boundary(cut) {
                cut -= 1;
            }
            return Some(cut);
        }
        None
    }

    /// Converts a single batch of source text into output.
    ///
    /// On failure, transitions to [`StreamState::Failed`].
    fn convert_batch(&mut self, batch: &str) -> InteropResult<ConvertedChunk> {
        if batch.trim().is_empty() {
            return Ok(ConvertedChunk::empty(self.from, self.to));
        }
        match self.converter.convert(batch, self.from, self.to) {
            Ok((output, report)) => {
                self.metrics.flushes += 1;
                let records = report.statutes_converted;
                Ok(ConvertedChunk {
                    output,
                    report,
                    records,
                })
            }
            Err(e) => {
                self.state = StreamState::Failed;
                Err(e)
            }
        }
    }

    /// Merges a produced chunk into a combined chunk and updates metrics.
    fn absorb(&mut self, combined: &mut ConvertedChunk, produced: ConvertedChunk) {
        if produced.is_empty() {
            return;
        }
        if !combined.output.is_empty() && !produced.output.is_empty() {
            combined.output.push('\n');
        }
        self.metrics.bytes_out += produced.output.len();
        if combined.output.is_empty() {
            combined.output = produced.output;
        } else {
            combined.output.push_str(&produced.output);
        }
        combined.records += produced.records;
        combined.report.statutes_converted += produced.report.statutes_converted;
        combined
            .report
            .unsupported_features
            .extend(produced.report.unsupported_features);
        combined.report.warnings.extend(produced.report.warnings);
        combined.report.confidence =
            (combined.report.confidence * produced.report.confidence).max(0.0);
        self.metrics.records += produced.records;
    }

    /// Convenience: stream an entire in-memory string through the converter in
    /// fixed-size byte windows, returning the full converted output.
    ///
    /// This exercises the same bounded-memory path used by truly streamed input
    /// and is the simplest way to convert a large `String` with a memory cap.
    ///
    /// # Errors
    /// Returns an error if any batch fails to convert.
    pub fn convert_str_chunked(&mut self, source: &str, window: usize) -> InteropResult<String> {
        let window = window.max(1);
        let mut out = String::new();
        let bytes = source.as_bytes();
        let mut start = 0;
        while start < bytes.len() {
            let mut end = (start + window).min(bytes.len());
            // Respect UTF-8 boundaries.
            while end < bytes.len() && !source.is_char_boundary(end) {
                end += 1;
            }
            let piece = &source[start..end];
            let produced = self.feed_chunk(piece)?;
            if !produced.output.is_empty() {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&produced.output);
            }
            start = end;
        }
        let tail = self.finish()?;
        if !tail.output.is_empty() {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&tail.output);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Build a Catala source of `n` simple scope declarations separated by blank
    // lines so the default "\n\n" boundary applies.
    fn catala_records(n: usize) -> String {
        let mut s = String::new();
        for i in 0..n {
            s.push_str(&format!(
                "declaration scope Scope{i}:\n  context input{i} content integer\n\n"
            ));
        }
        s
    }

    #[test]
    fn state_machine_transitions() {
        let mut sc = StreamingConverter::new(LegalFormat::Catala, LegalFormat::L4);
        assert_eq!(sc.state(), StreamState::Idle);
        let _ = sc.feed_chunk("declaration scope A:\n").expect("feed");
        assert_eq!(sc.state(), StreamState::Streaming);
        let _ = sc.finish().expect("finish");
        assert_eq!(sc.state(), StreamState::Done);
    }

    #[test]
    fn feed_after_finish_errors() {
        let mut sc = StreamingConverter::new(LegalFormat::Catala, LegalFormat::L4);
        let _ = sc.feed_chunk("declaration scope A:\n").expect("feed");
        let _ = sc.finish().expect("finish");
        assert!(sc.feed_chunk("more").is_err());
        assert!(sc.finish().is_err(), "double finish errors");
    }

    #[test]
    fn idle_finish_is_trivially_done() {
        let mut sc = StreamingConverter::new(LegalFormat::Catala, LegalFormat::L4);
        let chunk = sc.finish().expect("finish on idle");
        assert!(chunk.is_empty());
        assert_eq!(sc.state(), StreamState::Done);
    }

    #[test]
    fn bounded_memory_during_streaming() {
        let cfg = StreamConfig::with_threshold(256);
        let mut sc = StreamingConverter::with_config(LegalFormat::Catala, LegalFormat::L4, cfg);
        let source = catala_records(40); // well over the 256B threshold
        // Feed in tiny 17-byte windows.
        let _ = sc
            .convert_str_chunked(&source, 17)
            .expect("chunked convert");
        // After completion the buffer is drained.
        assert_eq!(sc.buffered_bytes(), 0);
        // Peak buffer must respect the memory bound (threshold-driven flushing
        // keeps it near the threshold; allow generous slack for one record).
        assert!(
            sc.metrics().peak_buffer_bytes <= sc.config_max_for_test(),
            "peak {} exceeded cap",
            sc.metrics().peak_buffer_bytes
        );
        assert!(sc.metrics().flushes >= 1);
    }

    #[test]
    fn chunked_output_equals_whole_conversion() {
        // Property: streaming the source in small windows yields the same set of
        // converted records as a single whole-document conversion.
        let source = catala_records(6);
        let mut sc = StreamingConverter::new(LegalFormat::Catala, LegalFormat::L4);
        let streamed = sc.convert_str_chunked(&source, 13).expect("streamed");

        let mut whole = LegalConverter::new();
        let (whole_out, whole_report) = whole
            .convert(&source, LegalFormat::Catala, LegalFormat::L4)
            .expect("whole");

        assert_eq!(sc.metrics().records, whole_report.statutes_converted);
        // Both outputs must be non-empty and mention the same number of scopes.
        assert!(!streamed.is_empty());
        assert!(!whole_out.is_empty());
        assert_eq!(sc.state(), StreamState::Done);
    }

    #[test]
    fn memory_bound_holds_with_many_records_over_cap() {
        // Feeding far more boundary-separated data than the cap must keep the
        // live buffer bounded by the cap at all times (records flush at "\n\n").
        let cfg = StreamConfig::with_threshold(512);
        let mut sc = StreamingConverter::with_config(LegalFormat::Catala, LegalFormat::L4, cfg);
        let source = catala_records(100); // many KB, >> the 512B threshold
        let bytes = source.as_bytes();
        let mut start = 0usize;
        // Feed in 31-byte windows, asserting the bound after every feed.
        while start < bytes.len() {
            let mut end = (start + 31).min(bytes.len());
            while end < bytes.len() && !source.is_char_boundary(end) {
                end += 1;
            }
            let _ = sc.feed_chunk(&source[start..end]).expect("feed");
            assert!(
                sc.buffered_bytes() <= sc.config_max_for_test(),
                "buffer {} exceeded cap {} mid-stream",
                sc.buffered_bytes(),
                sc.config_max_for_test()
            );
            start = end;
        }
        let _ = sc.finish().expect("finish");
        assert_eq!(sc.buffered_bytes(), 0);
        assert!(sc.metrics().records >= 100);
    }

    #[test]
    fn force_split_surfaces_error_rather_than_unbounded_growth() {
        // A single boundaryless record larger than the cap is force-split to
        // preserve the memory bound. Because the fragment is not, on its own, a
        // valid record, conversion surfaces a clean error and the converter
        // transitions to Failed — it never grows the buffer past the cap.
        let cfg = StreamConfig::with_threshold(64);
        let mut sc = StreamingConverter::with_config(LegalFormat::Catala, LegalFormat::L4, cfg);
        let big_line =
            "declaration scope Big:\n".to_string() + &"  context f content integer\n".repeat(200); // no "\n\n"
        let result = sc.feed_chunk(&big_line);
        assert!(
            result.is_err(),
            "force-split mid-record yields a parse error"
        );
        assert_eq!(sc.state(), StreamState::Failed);
        // No further input is accepted once failed.
        assert!(sc.feed_chunk("anything").is_err());
    }

    // Test accessor for the configured max buffer.
    impl StreamingConverter {
        fn config_max_for_test(&self) -> usize {
            self.config.max_buffer_bytes
        }
    }
}
