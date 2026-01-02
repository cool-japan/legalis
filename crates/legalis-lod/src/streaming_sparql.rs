//! Streaming SPARQL query processing for continuous data flows.
//!
//! This module provides streaming evaluation of SPARQL queries over
//! continuous RDF data streams, enabling real-time legal intelligence.

#[cfg(test)]
use crate::RdfValue;
use crate::{LodError, LodResult, Triple};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Time window for stream processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeWindow {
    /// Sliding window in seconds
    Sliding { seconds: u64 },
    /// Tumbling window in seconds
    Tumbling { seconds: u64 },
    /// Session window with gap in seconds
    Session { gap_seconds: u64 },
}

impl TimeWindow {
    /// Creates a sliding window.
    pub fn sliding(seconds: u64) -> Self {
        Self::Sliding { seconds }
    }

    /// Creates a tumbling window.
    pub fn tumbling(seconds: u64) -> Self {
        Self::Tumbling { seconds }
    }

    /// Creates a session window.
    pub fn session(gap_seconds: u64) -> Self {
        Self::Session { gap_seconds }
    }

    /// Returns the window size in seconds.
    pub fn size(&self) -> u64 {
        match self {
            Self::Sliding { seconds } | Self::Tumbling { seconds } => *seconds,
            Self::Session { gap_seconds } => *gap_seconds,
        }
    }
}

/// Stream element with timestamp.
#[derive(Debug, Clone)]
pub struct StreamElement {
    /// RDF triple
    pub triple: Triple,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Optional window ID
    pub window_id: Option<String>,
}

impl StreamElement {
    /// Creates a new stream element.
    pub fn new(triple: Triple) -> Self {
        Self {
            triple,
            timestamp: chrono::Utc::now(),
            window_id: None,
        }
    }

    /// Creates a stream element with a specific timestamp.
    pub fn with_timestamp(triple: Triple, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            triple,
            timestamp,
            window_id: None,
        }
    }

    /// Sets the window ID.
    pub fn with_window_id(mut self, window_id: impl Into<String>) -> Self {
        self.window_id = Some(window_id.into());
        self
    }
}

/// Streaming SPARQL processor.
pub struct StreamingSparqlProcessor {
    /// Time window
    window: TimeWindow,
    /// Query patterns to match
    query_patterns: Vec<String>,
    /// Buffer of recent triples
    buffer: Arc<Mutex<VecDeque<StreamElement>>>,
    /// Maximum buffer size
    max_buffer_size: usize,
}

impl StreamingSparqlProcessor {
    /// Creates a new streaming processor.
    pub fn new(window: TimeWindow) -> Self {
        Self {
            window,
            query_patterns: Vec::new(),
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            max_buffer_size: 10000,
        }
    }

    /// Adds a query pattern.
    pub fn add_pattern(&mut self, pattern: impl Into<String>) {
        self.query_patterns.push(pattern.into());
    }

    /// Sets the maximum buffer size.
    pub fn with_max_buffer_size(mut self, size: usize) -> Self {
        self.max_buffer_size = size;
        self
    }

    /// Processes a new triple from the stream.
    pub fn process(&mut self, element: StreamElement) -> LodResult<Vec<StreamResult>> {
        let mut buffer = self
            .buffer
            .lock()
            .map_err(|e| LodError::SerializationError(format!("Failed to lock buffer: {}", e)))?;

        // Add new element
        buffer.push_back(element.clone());

        // Evict old elements based on window
        self.evict_old_elements(&mut buffer, &element.timestamp);

        // Limit buffer size
        while buffer.len() > self.max_buffer_size {
            buffer.pop_front();
        }

        // Execute query on current window
        let current_triples: Vec<Triple> = buffer.iter().map(|e| e.triple.clone()).collect();
        self.execute_query(&current_triples, &element.timestamp)
    }

    /// Evicts elements outside the time window.
    fn evict_old_elements(
        &self,
        buffer: &mut VecDeque<StreamElement>,
        current_time: &chrono::DateTime<chrono::Utc>,
    ) {
        let window_size = chrono::Duration::seconds(self.window.size() as i64);

        match self.window {
            TimeWindow::Sliding { .. } | TimeWindow::Tumbling { .. } => {
                // Remove elements older than window
                while let Some(front) = buffer.front() {
                    if *current_time - front.timestamp > window_size {
                        buffer.pop_front();
                    } else {
                        break;
                    }
                }
            }
            TimeWindow::Session { .. } => {
                // For session windows, keep until gap is exceeded
                if let Some(front) = buffer.front() {
                    if *current_time - front.timestamp > window_size {
                        buffer.clear();
                    }
                }
            }
        }
    }

    /// Executes the query on current window.
    fn execute_query(
        &self,
        triples: &[Triple],
        timestamp: &chrono::DateTime<chrono::Utc>,
    ) -> LodResult<Vec<StreamResult>> {
        // Simplified query execution - in production, use full SPARQL engine
        let mut results = Vec::new();

        // For now, just return matching triples
        for triple in triples {
            if self.matches_query(triple) {
                results.push(StreamResult {
                    bindings: self.extract_bindings(triple),
                    timestamp: *timestamp,
                    window_id: None,
                });
            }
        }

        Ok(results)
    }

    /// Checks if a triple matches the query pattern.
    fn matches_query(&self, triple: &Triple) -> bool {
        // Simplified pattern matching
        if self.query_patterns.is_empty() {
            return true; // No patterns means match all
        }

        for pattern in &self.query_patterns {
            // Basic pattern matching logic
            if pattern.contains(&triple.predicate) {
                return true;
            }
        }
        false
    }

    /// Extracts variable bindings from a triple.
    fn extract_bindings(&self, triple: &Triple) -> HashMap<String, String> {
        let mut bindings = HashMap::new();
        bindings.insert("subject".to_string(), triple.subject.clone());
        bindings.insert("predicate".to_string(), triple.predicate.clone());
        bindings.insert("object".to_string(), format!("{:?}", triple.object));
        bindings
    }

    /// Returns the current buffer size.
    pub fn buffer_size(&self) -> usize {
        self.buffer.lock().map(|b| b.len()).unwrap_or(0)
    }
}

/// Result from streaming query evaluation.
#[derive(Debug, Clone)]
pub struct StreamResult {
    /// Variable bindings
    pub bindings: HashMap<String, String>,
    /// Result timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Window ID
    pub window_id: Option<String>,
}

/// Stream aggregator for computing aggregates over streams.
pub struct StreamAggregator {
    /// Aggregation function
    function: AggregateFunction,
    /// Current aggregated value
    value: f64,
    /// Count of processed elements
    count: usize,
}

impl StreamAggregator {
    /// Creates a new stream aggregator.
    pub fn new(function: AggregateFunction) -> Self {
        Self {
            function,
            value: 0.0,
            count: 0,
        }
    }

    /// Processes a new value.
    pub fn process(&mut self, value: f64) {
        match self.function {
            AggregateFunction::Sum => self.value += value,
            AggregateFunction::Count => self.count += 1,
            AggregateFunction::Average => {
                self.value = (self.value * self.count as f64 + value) / (self.count + 1) as f64;
                self.count += 1;
            }
            AggregateFunction::Min => {
                if self.count == 0 || value < self.value {
                    self.value = value;
                }
                self.count += 1;
            }
            AggregateFunction::Max => {
                if self.count == 0 || value > self.value {
                    self.value = value;
                }
                self.count += 1;
            }
        }
    }

    /// Returns the current aggregate value.
    pub fn get_value(&self) -> f64 {
        match self.function {
            AggregateFunction::Count => self.count as f64,
            _ => self.value,
        }
    }

    /// Resets the aggregator.
    pub fn reset(&mut self) {
        self.value = 0.0;
        self.count = 0;
    }
}

/// Aggregate function for stream processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateFunction {
    Sum,
    Count,
    Average,
    Min,
    Max,
}

/// Stream join operator for combining multiple streams.
pub struct StreamJoin {
    /// Left stream buffer
    left_buffer: VecDeque<StreamElement>,
    /// Right stream buffer
    right_buffer: VecDeque<StreamElement>,
    /// Join predicate
    #[allow(dead_code)]
    join_key: String,
    /// Time window
    window: TimeWindow,
}

impl StreamJoin {
    /// Creates a new stream join.
    pub fn new(join_key: impl Into<String>, window: TimeWindow) -> Self {
        Self {
            left_buffer: VecDeque::new(),
            right_buffer: VecDeque::new(),
            join_key: join_key.into(),
            window,
        }
    }

    /// Processes an element from the left stream.
    pub fn process_left(&mut self, element: StreamElement) -> Vec<(StreamElement, StreamElement)> {
        let timestamp = element.timestamp;
        let window_size = chrono::Duration::seconds(self.window.size() as i64);

        self.left_buffer.push_back(element.clone());
        Self::evict_old_elements(&mut self.left_buffer, &timestamp, window_size);
        self.find_matches(&element, &self.right_buffer)
    }

    /// Processes an element from the right stream.
    pub fn process_right(&mut self, element: StreamElement) -> Vec<(StreamElement, StreamElement)> {
        let timestamp = element.timestamp;
        let window_size = chrono::Duration::seconds(self.window.size() as i64);

        self.right_buffer.push_back(element.clone());
        Self::evict_old_elements(&mut self.right_buffer, &timestamp, window_size);
        self.find_matches(&element, &self.left_buffer)
    }

    /// Finds matching elements.
    fn find_matches(
        &self,
        element: &StreamElement,
        buffer: &VecDeque<StreamElement>,
    ) -> Vec<(StreamElement, StreamElement)> {
        let mut matches = Vec::new();
        for other in buffer {
            if self.matches(element, other) {
                matches.push((element.clone(), other.clone()));
            }
        }
        matches
    }

    /// Checks if two elements match on join key.
    fn matches(&self, left: &StreamElement, right: &StreamElement) -> bool {
        // Simplified join logic - compare subjects
        left.triple.subject == right.triple.subject
    }

    /// Evicts old elements from buffer.
    fn evict_old_elements(
        buffer: &mut VecDeque<StreamElement>,
        current_time: &chrono::DateTime<chrono::Utc>,
        window_size: chrono::Duration,
    ) {
        while let Some(front) = buffer.front() {
            if *current_time - front.timestamp > window_size {
                buffer.pop_front();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triple() -> Triple {
        Triple {
            subject: "ex:s1".to_string(),
            predicate: "ex:p1".to_string(),
            object: RdfValue::string("o1"),
        }
    }

    #[test]
    fn test_time_window() {
        let sliding = TimeWindow::sliding(60);
        assert_eq!(sliding.size(), 60);

        let tumbling = TimeWindow::tumbling(30);
        assert_eq!(tumbling.size(), 30);

        let session = TimeWindow::session(10);
        assert_eq!(session.size(), 10);
    }

    #[test]
    fn test_stream_element() {
        let triple = sample_triple();
        let element = StreamElement::new(triple.clone());

        assert_eq!(element.triple.subject, "ex:s1");
        assert!(element.window_id.is_none());
    }

    #[test]
    fn test_stream_element_with_window() {
        let triple = sample_triple();
        let element = StreamElement::new(triple).with_window_id("window-1");

        assert_eq!(element.window_id, Some("window-1".to_string()));
    }

    #[test]
    fn test_streaming_processor() {
        let window = TimeWindow::sliding(60);
        let mut processor = StreamingSparqlProcessor::new(window);

        let triple = sample_triple();
        let element = StreamElement::new(triple);

        let _results = processor.process(element).unwrap();
        assert_eq!(processor.buffer_size(), 1);
    }

    #[test]
    fn test_processor_max_buffer() {
        let window = TimeWindow::sliding(60);
        let mut processor = StreamingSparqlProcessor::new(window).with_max_buffer_size(5);

        for i in 0..10 {
            let mut triple = sample_triple();
            triple.subject = format!("ex:s{}", i);
            let element = StreamElement::new(triple);
            processor.process(element).unwrap();
        }

        assert_eq!(processor.buffer_size(), 5);
    }

    #[test]
    fn test_stream_aggregator_sum() {
        let mut agg = StreamAggregator::new(AggregateFunction::Sum);
        agg.process(10.0);
        agg.process(20.0);
        agg.process(30.0);

        assert_eq!(agg.get_value(), 60.0);
    }

    #[test]
    fn test_stream_aggregator_count() {
        let mut agg = StreamAggregator::new(AggregateFunction::Count);
        agg.process(1.0);
        agg.process(2.0);
        agg.process(3.0);

        assert_eq!(agg.get_value(), 3.0);
    }

    #[test]
    fn test_stream_aggregator_average() {
        let mut agg = StreamAggregator::new(AggregateFunction::Average);
        agg.process(10.0);
        agg.process(20.0);
        agg.process(30.0);

        assert_eq!(agg.get_value(), 20.0);
    }

    #[test]
    fn test_stream_aggregator_min() {
        let mut agg = StreamAggregator::new(AggregateFunction::Min);
        agg.process(30.0);
        agg.process(10.0);
        agg.process(20.0);

        assert_eq!(agg.get_value(), 10.0);
    }

    #[test]
    fn test_stream_aggregator_max() {
        let mut agg = StreamAggregator::new(AggregateFunction::Max);
        agg.process(10.0);
        agg.process(30.0);
        agg.process(20.0);

        assert_eq!(agg.get_value(), 30.0);
    }

    #[test]
    fn test_aggregator_reset() {
        let mut agg = StreamAggregator::new(AggregateFunction::Sum);
        agg.process(10.0);
        agg.process(20.0);
        assert_eq!(agg.get_value(), 30.0);

        agg.reset();
        assert_eq!(agg.get_value(), 0.0);
    }

    #[test]
    fn test_stream_join() {
        let window = TimeWindow::sliding(60);
        let mut join = StreamJoin::new("subject", window);

        let triple1 = sample_triple();
        let element1 = StreamElement::new(triple1);

        let triple2 = sample_triple();
        let element2 = StreamElement::new(triple2);

        let matches1 = join.process_left(element1);
        assert_eq!(matches1.len(), 0);

        let matches2 = join.process_right(element2);
        assert_eq!(matches2.len(), 1);
    }

    #[test]
    fn test_join_eviction() {
        let window = TimeWindow::sliding(1); // 1 second window
        let mut join = StreamJoin::new("subject", window);

        let triple1 = sample_triple();
        let old_time = chrono::Utc::now() - chrono::Duration::seconds(5);
        let element1 = StreamElement::with_timestamp(triple1, old_time);

        join.process_left(element1);

        // Process another left element with current time - should evict old one
        let triple2 = sample_triple();
        let element2 = StreamElement::new(triple2);
        join.process_left(element2);

        // Old element should be evicted from left buffer
        assert_eq!(join.left_buffer.len(), 1);
    }
}
