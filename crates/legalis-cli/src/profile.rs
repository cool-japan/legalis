//! Profiling utilities for performance analysis.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Profile data structure for CPU and memory metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    /// CPU profiling data
    pub cpu: CpuProfile,
    /// Memory profiling data
    pub memory: MemoryProfile,
    /// Overall statistics
    pub stats: ProfileStats,
}

/// CPU profiling data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfile {
    /// Total execution time across all iterations
    pub total_time: Duration,
    /// Average time per iteration
    pub avg_time: Duration,
    /// Minimum time
    pub min_time: Duration,
    /// Maximum time
    pub max_time: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// Samples (time for each iteration)
    pub samples: Vec<Duration>,
}

/// Memory profiling data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Peak memory usage (in bytes)
    pub peak_bytes: usize,
    /// Average memory usage
    pub avg_bytes: usize,
    /// Memory usage samples
    pub samples: Vec<usize>,
}

/// Overall profiling statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStats {
    /// Number of iterations
    pub iterations: usize,
    /// Total operations performed
    pub operations: usize,
    /// Operations per second
    pub ops_per_sec: f64,
}

impl ProfileData {
    /// Create a new empty profile data.
    pub fn new() -> Self {
        Self {
            cpu: CpuProfile {
                total_time: Duration::ZERO,
                avg_time: Duration::ZERO,
                min_time: Duration::MAX,
                max_time: Duration::ZERO,
                std_dev: Duration::ZERO,
                samples: Vec::new(),
            },
            memory: MemoryProfile {
                peak_bytes: 0,
                avg_bytes: 0,
                samples: Vec::new(),
            },
            stats: ProfileStats {
                iterations: 0,
                operations: 0,
                ops_per_sec: 0.0,
            },
        }
    }

    /// Record a CPU sample.
    pub fn record_cpu_sample(&mut self, duration: Duration) {
        self.cpu.samples.push(duration);
        self.cpu.total_time += duration;
        if duration < self.cpu.min_time {
            self.cpu.min_time = duration;
        }
        if duration > self.cpu.max_time {
            self.cpu.max_time = duration;
        }
    }

    /// Record a memory sample.
    pub fn record_memory_sample(&mut self, bytes: usize) {
        self.memory.samples.push(bytes);
        if bytes > self.memory.peak_bytes {
            self.memory.peak_bytes = bytes;
        }
    }

    /// Finalize the profile data by computing averages and statistics.
    pub fn finalize(&mut self) {
        // Compute CPU statistics
        let sample_count = self.cpu.samples.len();
        if sample_count > 0 {
            self.cpu.avg_time = self.cpu.total_time / sample_count as u32;

            // Compute standard deviation
            let avg_nanos = self.cpu.avg_time.as_nanos() as f64;
            let variance: f64 = self
                .cpu
                .samples
                .iter()
                .map(|s| {
                    let diff = s.as_nanos() as f64 - avg_nanos;
                    diff * diff
                })
                .sum::<f64>()
                / sample_count as f64;
            let std_dev_nanos = variance.sqrt();
            self.cpu.std_dev = Duration::from_nanos(std_dev_nanos as u64);
        }

        // Compute memory statistics
        if !self.memory.samples.is_empty() {
            let total_bytes: usize = self.memory.samples.iter().sum();
            self.memory.avg_bytes = total_bytes / self.memory.samples.len();
        }

        // Compute overall statistics
        self.stats.iterations = sample_count;
        if !self.cpu.total_time.is_zero() {
            self.stats.ops_per_sec =
                self.stats.operations as f64 / self.cpu.total_time.as_secs_f64();
        }
    }

    /// Format the profile data as a human-readable report.
    pub fn format_report(&self) -> String {
        use colored::Colorize;

        let mut report = String::new();

        report.push_str(&format!("\n{}\n", "=== Profile Report ===".bold().cyan()));

        // CPU Profile
        report.push_str(&format!("\n{}\n", "CPU Profile:".bold()));
        report.push_str(&format!(
            "  Total Time:   {:>12}\n",
            format_duration(self.cpu.total_time)
        ));
        report.push_str(&format!(
            "  Avg Time:     {:>12}\n",
            format_duration(self.cpu.avg_time)
        ));
        report.push_str(&format!(
            "  Min Time:     {:>12}\n",
            format_duration(self.cpu.min_time)
        ));
        report.push_str(&format!(
            "  Max Time:     {:>12}\n",
            format_duration(self.cpu.max_time)
        ));
        report.push_str(&format!(
            "  Std Dev:      {:>12}\n",
            format_duration(self.cpu.std_dev)
        ));

        // Memory Profile
        report.push_str(&format!("\n{}\n", "Memory Profile:".bold()));
        report.push_str(&format!(
            "  Peak Memory:  {:>12}\n",
            format_bytes(self.memory.peak_bytes)
        ));
        report.push_str(&format!(
            "  Avg Memory:   {:>12}\n",
            format_bytes(self.memory.avg_bytes)
        ));

        // Statistics
        report.push_str(&format!("\n{}\n", "Statistics:".bold()));
        report.push_str(&format!("  Iterations:   {:>12}\n", self.stats.iterations));
        report.push_str(&format!("  Operations:   {:>12}\n", self.stats.operations));
        report.push_str(&format!(
            "  Ops/Sec:      {:>12.2}\n",
            self.stats.ops_per_sec
        ));

        report
    }
}

impl Default for ProfileData {
    fn default() -> Self {
        Self::new()
    }
}

/// Profiler for measuring CPU and memory usage.
pub struct Profiler {
    profile_cpu: bool,
    profile_memory: bool,
    data: ProfileData,
}

impl Profiler {
    /// Create a new profiler.
    pub fn new(profile_cpu: bool, profile_memory: bool) -> Self {
        Self {
            profile_cpu,
            profile_memory,
            data: ProfileData::new(),
        }
    }

    /// Run a profiling session.
    pub fn profile<F>(&mut self, iterations: usize, mut operation: F) -> Result<ProfileData>
    where
        F: FnMut() -> Result<()>,
    {
        for i in 0..iterations {
            // CPU profiling
            let start = if self.profile_cpu {
                Some(Instant::now())
            } else {
                None
            };

            // Memory profiling (before operation)
            #[cfg(target_os = "linux")]
            let mem_before = if self.profile_memory {
                get_current_memory_usage().ok()
            } else {
                None
            };

            // Run the operation
            operation().with_context(|| format!("Operation failed at iteration {}", i + 1))?;

            // Record CPU time
            if let Some(start_time) = start {
                let elapsed = start_time.elapsed();
                self.data.record_cpu_sample(elapsed);
            }

            // Record memory usage
            #[cfg(target_os = "linux")]
            if let Some(mem_before_bytes) = mem_before {
                if let Ok(mem_after) = get_current_memory_usage() {
                    let mem_used = mem_after.saturating_sub(mem_before_bytes);
                    self.data.record_memory_sample(mem_used);
                }
            }

            self.data.stats.operations += 1;
        }

        self.data.finalize();
        Ok(self.data.clone())
    }

    /// Get the profile data.
    pub fn data(&self) -> &ProfileData {
        &self.data
    }
}

/// Format a duration in a human-readable way.
fn format_duration(duration: Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2}µs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", nanos as f64 / 1_000_000_000.0)
    }
}

/// Format bytes in a human-readable way.
fn format_bytes(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f < KB {
        format!("{}B", bytes)
    } else if bytes_f < MB {
        format!("{:.2}KB", bytes_f / KB)
    } else if bytes_f < GB {
        format!("{:.2}MB", bytes_f / MB)
    } else {
        format!("{:.2}GB", bytes_f / GB)
    }
}

/// Get current memory usage (Linux only).
#[cfg(target_os = "linux")]
fn get_current_memory_usage() -> Result<usize> {
    let status = std::fs::read_to_string("/proc/self/status")?;

    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let kb: usize = parts[1].parse().context("Failed to parse memory value")?;
                return Ok(kb * 1024); // Convert to bytes
            }
        }
    }

    anyhow::bail!("Could not find VmRSS in /proc/self/status")
}

/// Get current memory usage (non-Linux platforms - stub).
#[cfg(not(target_os = "linux"))]
#[allow(dead_code)]
fn get_current_memory_usage() -> Result<usize> {
    // Memory profiling is only supported on Linux for now
    Ok(0)
}

/// Timing breakdown tracker for complex operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimingBreakdown {
    /// Timing entries
    pub entries: Vec<TimingEntry>,
}

/// A single timing entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingEntry {
    /// Name of the operation
    pub name: String,
    /// Duration of the operation
    pub duration: Duration,
    /// Percentage of total time
    pub percentage: f64,
}

impl TimingBreakdown {
    /// Create a new timing breakdown.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a timing entry.
    pub fn add_entry(&mut self, name: impl Into<String>, duration: Duration) {
        self.entries.push(TimingEntry {
            name: name.into(),
            duration,
            percentage: 0.0,
        });
    }

    /// Finalize the breakdown by computing percentages.
    pub fn finalize(&mut self) {
        let total: Duration = self.entries.iter().map(|e| e.duration).sum();
        let total_nanos = total.as_nanos() as f64;

        if total_nanos > 0.0 {
            for entry in &mut self.entries {
                entry.percentage = (entry.duration.as_nanos() as f64 / total_nanos) * 100.0;
            }
        }
    }

    /// Format as a report.
    pub fn format_report(&self) -> String {
        use colored::Colorize;

        let mut report = String::new();
        report.push_str(&format!("\n{}\n", "=== Timing Breakdown ===".bold().cyan()));

        for entry in &self.entries {
            report.push_str(&format!(
                "  {:<30} {:>12} ({:>6.2}%)\n",
                entry.name,
                format_duration(entry.duration),
                entry.percentage
            ));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_data_creation() {
        let data = ProfileData::new();
        assert_eq!(data.cpu.samples.len(), 0);
        assert_eq!(data.memory.samples.len(), 0);
    }

    #[test]
    fn test_cpu_sample_recording() {
        let mut data = ProfileData::new();
        data.record_cpu_sample(Duration::from_millis(10));
        data.record_cpu_sample(Duration::from_millis(20));
        data.record_cpu_sample(Duration::from_millis(15));

        assert_eq!(data.cpu.samples.len(), 3);
        assert_eq!(data.cpu.min_time, Duration::from_millis(10));
        assert_eq!(data.cpu.max_time, Duration::from_millis(20));
    }

    #[test]
    fn test_memory_sample_recording() {
        let mut data = ProfileData::new();
        data.record_memory_sample(1024);
        data.record_memory_sample(2048);
        data.record_memory_sample(1536);

        assert_eq!(data.memory.samples.len(), 3);
        assert_eq!(data.memory.peak_bytes, 2048);
    }

    #[test]
    fn test_finalize() {
        let mut data = ProfileData::new();
        data.record_cpu_sample(Duration::from_millis(10));
        data.record_cpu_sample(Duration::from_millis(20));
        data.record_cpu_sample(Duration::from_millis(30));

        data.record_memory_sample(1000);
        data.record_memory_sample(2000);
        data.record_memory_sample(3000);

        data.finalize();

        assert_eq!(data.cpu.avg_time, Duration::from_millis(20));
        assert_eq!(data.memory.avg_bytes, 2000);
        assert_eq!(data.stats.iterations, 3);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_nanos(500)), "500ns");
        assert_eq!(format_duration(Duration::from_micros(500)), "500.00µs");
        assert_eq!(format_duration(Duration::from_millis(500)), "500.00ms");
        assert_eq!(format_duration(Duration::from_secs(5)), "5.00s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512B");
        assert_eq!(format_bytes(1536), "1.50KB");
        assert_eq!(format_bytes(1_572_864), "1.50MB");
        assert_eq!(format_bytes(1_610_612_736), "1.50GB");
    }

    #[test]
    fn test_timing_breakdown() {
        let mut breakdown = TimingBreakdown::new();
        breakdown.add_entry("parse", Duration::from_millis(100));
        breakdown.add_entry("verify", Duration::from_millis(200));
        breakdown.add_entry("export", Duration::from_millis(100));

        breakdown.finalize();

        assert_eq!(breakdown.entries.len(), 3);
        assert!((breakdown.entries[0].percentage - 25.0).abs() < 0.1);
        assert!((breakdown.entries[1].percentage - 50.0).abs() < 0.1);
        assert!((breakdown.entries[2].percentage - 25.0).abs() < 0.1);
    }
}
