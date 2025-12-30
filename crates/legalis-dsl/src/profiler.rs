//! Performance profiling utilities for parsing and analysis.
//!
//! This module provides detailed profiling capabilities for:
//! - Parse time breakdown by component
//! - Memory usage tracking
//! - Performance comparison between runs

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

/// A profiling session that tracks performance metrics.
#[derive(Debug, Clone)]
pub struct Profiler {
    /// Named timing measurements
    timings: HashMap<String, Duration>,
    /// Start times for active measurements
    active: HashMap<String, Instant>,
    /// Memory usage snapshots (label -> bytes)
    memory_snapshots: HashMap<String, usize>,
    /// Counter for events
    counters: HashMap<String, u64>,
}

impl Profiler {
    /// Creates a new profiler instance.
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
            active: HashMap::new(),
            memory_snapshots: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    /// Starts timing a named section.
    pub fn start(&mut self, label: &str) {
        self.active.insert(label.to_string(), Instant::now());
    }

    /// Stops timing a named section and records the duration.
    pub fn stop(&mut self, label: &str) {
        if let Some(start) = self.active.remove(label) {
            let duration = start.elapsed();
            *self
                .timings
                .entry(label.to_string())
                .or_insert(Duration::ZERO) += duration;
        }
    }

    /// Records a scoped timing - automatically stops when the guard is dropped.
    pub fn scope<'a>(&'a mut self, label: &str) -> ScopeGuard<'a> {
        self.start(label);
        ScopeGuard {
            profiler: self,
            label: label.to_string(),
        }
    }

    /// Records memory usage at a specific point.
    pub fn snapshot_memory(&mut self, label: &str, bytes: usize) {
        self.memory_snapshots.insert(label.to_string(), bytes);
    }

    /// Increments a counter.
    pub fn count(&mut self, label: &str, value: u64) {
        *self.counters.entry(label.to_string()).or_insert(0) += value;
    }

    /// Gets the total duration for a named section.
    pub fn get_duration(&self, label: &str) -> Option<Duration> {
        self.timings.get(label).copied()
    }

    /// Gets all recorded timings.
    pub fn timings(&self) -> &HashMap<String, Duration> {
        &self.timings
    }

    /// Gets all memory snapshots.
    pub fn memory_snapshots(&self) -> &HashMap<String, usize> {
        &self.memory_snapshots
    }

    /// Gets all counters.
    pub fn counters(&self) -> &HashMap<String, u64> {
        &self.counters
    }

    /// Resets all profiling data.
    pub fn reset(&mut self) {
        self.timings.clear();
        self.active.clear();
        self.memory_snapshots.clear();
        self.counters.clear();
    }

    /// Generates a summary report.
    pub fn report(&self) -> ProfileReport {
        ProfileReport {
            timings: self.timings.clone(),
            memory_snapshots: self.memory_snapshots.clone(),
            counters: self.counters.clone(),
        }
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// A RAII guard that stops timing when dropped.
pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
    label: String,
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.profiler.stop(&self.label);
    }
}

/// A report containing profiling results.
#[derive(Debug, Clone)]
pub struct ProfileReport {
    timings: HashMap<String, Duration>,
    memory_snapshots: HashMap<String, usize>,
    counters: HashMap<String, u64>,
}

impl ProfileReport {
    /// Gets the total time across all measured sections.
    pub fn total_time(&self) -> Duration {
        self.timings.values().sum()
    }

    /// Gets the timing for a specific section.
    pub fn get_timing(&self, label: &str) -> Option<Duration> {
        self.timings.get(label).copied()
    }

    /// Gets memory usage for a specific snapshot.
    pub fn get_memory(&self, label: &str) -> Option<usize> {
        self.memory_snapshots.get(label).copied()
    }

    /// Gets counter value.
    pub fn get_counter(&self, label: &str) -> Option<u64> {
        self.counters.get(label).copied()
    }

    /// Compares this report with another and returns the differences.
    pub fn compare(&self, other: &ProfileReport) -> ProfileComparison {
        let mut timing_diffs = HashMap::new();
        for (label, duration) in &self.timings {
            if let Some(other_duration) = other.timings.get(label) {
                let diff = duration.as_nanos() as i128 - other_duration.as_nanos() as i128;
                timing_diffs.insert(label.clone(), diff);
            }
        }

        let mut memory_diffs = HashMap::new();
        for (label, bytes) in &self.memory_snapshots {
            if let Some(other_bytes) = other.memory_snapshots.get(label) {
                let diff = *bytes as i128 - *other_bytes as i128;
                memory_diffs.insert(label.clone(), diff);
            }
        }

        ProfileComparison {
            timing_diffs,
            memory_diffs,
        }
    }
}

impl fmt::Display for ProfileReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Performance Profile Report")?;
        writeln!(f, "==========================")?;
        writeln!(f)?;

        if !self.timings.is_empty() {
            writeln!(f, "Timings:")?;
            let mut timings: Vec<_> = self.timings.iter().collect();
            timings.sort_by(|a, b| b.1.cmp(a.1));
            for (label, duration) in timings {
                writeln!(
                    f,
                    "  {:<30} {:>12.3} ms",
                    label,
                    duration.as_secs_f64() * 1000.0
                )?;
            }
            writeln!(
                f,
                "  {:<30} {:>12.3} ms",
                "TOTAL",
                self.total_time().as_secs_f64() * 1000.0
            )?;
            writeln!(f)?;
        }

        if !self.memory_snapshots.is_empty() {
            writeln!(f, "Memory Snapshots:")?;
            let mut snapshots: Vec<_> = self.memory_snapshots.iter().collect();
            snapshots.sort_by(|a, b| b.1.cmp(a.1));
            for (label, bytes) in snapshots {
                writeln!(f, "  {:<30} {:>12} bytes", label, bytes)?;
            }
            writeln!(f)?;
        }

        if !self.counters.is_empty() {
            writeln!(f, "Counters:")?;
            let mut counters: Vec<_> = self.counters.iter().collect();
            counters.sort_by_key(|a| a.0);
            for (label, count) in counters {
                writeln!(f, "  {:<30} {:>12}", label, count)?;
            }
        }

        Ok(())
    }
}

/// A comparison between two profile reports.
#[derive(Debug, Clone)]
pub struct ProfileComparison {
    /// Timing differences in nanoseconds (positive = slower, negative = faster)
    timing_diffs: HashMap<String, i128>,
    /// Memory differences in bytes (positive = more memory, negative = less)
    memory_diffs: HashMap<String, i128>,
}

impl fmt::Display for ProfileComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Performance Comparison")?;
        writeln!(f, "=====================")?;
        writeln!(f)?;

        if !self.timing_diffs.is_empty() {
            writeln!(f, "Timing Differences:")?;
            let mut diffs: Vec<_> = self.timing_diffs.iter().collect();
            diffs.sort_by_key(|a| a.1.abs());
            diffs.reverse();
            for (label, diff_ns) in diffs {
                let diff_ms = *diff_ns as f64 / 1_000_000.0;
                let sign = if *diff_ns >= 0 { "+" } else { "" };
                writeln!(f, "  {:<30} {:>12} {} ms", label, sign, diff_ms)?;
            }
            writeln!(f)?;
        }

        if !self.memory_diffs.is_empty() {
            writeln!(f, "Memory Differences:")?;
            let mut diffs: Vec<_> = self.memory_diffs.iter().collect();
            diffs.sort_by_key(|a| a.1.abs());
            diffs.reverse();
            for (label, diff_bytes) in diffs {
                let sign = if *diff_bytes >= 0 { "+" } else { "" };
                writeln!(f, "  {:<30} {:>12} {} bytes", label, sign, diff_bytes)?;
            }
        }

        Ok(())
    }
}

/// Helper for profiling parse operations.
pub struct ParseProfiler {
    profiler: Profiler,
}

impl ParseProfiler {
    /// Creates a new parse profiler.
    pub fn new() -> Self {
        Self {
            profiler: Profiler::new(),
        }
    }

    /// Profiles a full document parse.
    pub fn profile_parse<F, T>(&mut self, input: &str, parse_fn: F) -> T
    where
        F: FnOnce(&str, &mut Profiler) -> T,
    {
        self.profiler.start("total_parse");
        self.profiler.count("input_size_bytes", input.len() as u64);
        self.profiler
            .count("input_size_lines", input.lines().count() as u64);

        let result = parse_fn(input, &mut self.profiler);

        self.profiler.stop("total_parse");
        result
    }

    /// Gets the underlying profiler.
    pub fn profiler(&self) -> &Profiler {
        &self.profiler
    }

    /// Gets a mutable reference to the profiler.
    pub fn profiler_mut(&mut self) -> &mut Profiler {
        &mut self.profiler
    }

    /// Generates the profile report.
    pub fn report(&self) -> ProfileReport {
        self.profiler.report()
    }
}

impl Default for ParseProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_basic_timing() {
        let mut profiler = Profiler::new();
        profiler.start("test");
        std::thread::sleep(Duration::from_millis(10));
        profiler.stop("test");

        let duration = profiler.get_duration("test").unwrap();
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_profiler_scope_guard() {
        let mut profiler = Profiler::new();
        {
            let _guard = profiler.scope("scoped");
            std::thread::sleep(Duration::from_millis(10));
        }

        let duration = profiler.get_duration("scoped").unwrap();
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_profiler_memory_snapshot() {
        let mut profiler = Profiler::new();
        profiler.snapshot_memory("start", 1024);
        profiler.snapshot_memory("end", 2048);

        assert_eq!(profiler.memory_snapshots().get("start"), Some(&1024));
        assert_eq!(profiler.memory_snapshots().get("end"), Some(&2048));
    }

    #[test]
    fn test_profiler_counters() {
        let mut profiler = Profiler::new();
        profiler.count("lines", 10);
        profiler.count("lines", 5);

        assert_eq!(profiler.counters().get("lines"), Some(&15));
    }

    #[test]
    fn test_profile_report_display() {
        let mut profiler = Profiler::new();
        profiler.start("test");
        std::thread::sleep(Duration::from_millis(1));
        profiler.stop("test");
        profiler.snapshot_memory("peak", 4096);
        profiler.count("tokens", 100);

        let report = profiler.report();
        let display = format!("{}", report);

        assert!(display.contains("Performance Profile Report"));
        assert!(display.contains("Timings:"));
        assert!(display.contains("Memory Snapshots:"));
        assert!(display.contains("Counters:"));
    }

    #[test]
    fn test_profile_comparison() {
        let mut profiler1 = Profiler::new();
        profiler1.start("parse");
        std::thread::sleep(Duration::from_millis(10));
        profiler1.stop("parse");

        let mut profiler2 = Profiler::new();
        profiler2.start("parse");
        std::thread::sleep(Duration::from_millis(5));
        profiler2.stop("parse");

        let report1 = profiler1.report();
        let report2 = profiler2.report();
        let comparison = report1.compare(&report2);

        let display = format!("{}", comparison);
        assert!(display.contains("Performance Comparison"));
    }

    #[test]
    fn test_parse_profiler() {
        let mut profiler = ParseProfiler::new();

        let result = profiler.profile_parse("test input", |input, prof| {
            prof.start("tokenize");
            std::thread::sleep(Duration::from_millis(1));
            prof.stop("tokenize");
            input.len()
        });

        assert_eq!(result, 10);
        let report = profiler.report();
        assert!(report.get_timing("total_parse").is_some());
        assert!(report.get_timing("tokenize").is_some());
        assert_eq!(report.get_counter("input_size_bytes"), Some(10));
    }
}
