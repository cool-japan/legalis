//! Performance profiling and monitoring for Legalis CLI.
//!
//! This module provides command execution profiling, memory usage tracking,
//! bottleneck detection, performance reports, and optimization suggestions.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Performance profiler for tracking command execution.
pub struct PerformanceProfiler {
    /// Base directory for performance data
    base_dir: PathBuf,
    /// Current profiling session
    session: Option<ProfilingSession>,
    /// Whether profiling is enabled
    enabled: bool,
}

/// A profiling session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingSession {
    /// Session ID
    pub id: String,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// Command metrics
    pub metrics: Vec<CommandMetric>,
    /// Memory snapshots
    pub memory_snapshots: Vec<MemorySnapshot>,
}

/// Metrics for a single command execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetric {
    /// Command name
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Execution start time
    pub started_at: DateTime<Utc>,
    /// Execution duration
    pub duration: Duration,
    /// Memory used (bytes)
    pub memory_used: u64,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Performance category
    pub category: PerformanceCategory,
}

/// Performance category for a command.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceCategory {
    /// Very fast (< 100ms)
    VeryFast,
    /// Fast (100ms - 500ms)
    Fast,
    /// Normal (500ms - 2s)
    Normal,
    /// Slow (2s - 5s)
    Slow,
    /// Very slow (> 5s)
    VerySlow,
}

impl PerformanceCategory {
    /// Categorize based on duration.
    pub fn from_duration(duration: Duration) -> Self {
        let millis = duration.as_millis();
        if millis < 100 {
            Self::VeryFast
        } else if millis < 500 {
            Self::Fast
        } else if millis < 2000 {
            Self::Normal
        } else if millis < 5000 {
            Self::Slow
        } else {
            Self::VerySlow
        }
    }
}

/// Memory usage snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Total memory used (bytes)
    pub total_memory: u64,
    /// Resident set size (bytes)
    pub rss: u64,
    /// Virtual memory size (bytes)
    pub virtual_memory: u64,
}

/// Bottleneck detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// Command that caused the bottleneck
    pub command: String,
    /// Bottleneck type
    pub bottleneck_type: BottleneckType,
    /// Severity
    pub severity: Severity,
    /// Description
    pub description: String,
    /// Suggested fix
    pub suggestion: String,
}

/// Type of bottleneck.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BottleneckType {
    /// CPU-bound operation
    Cpu,
    /// Memory-bound operation
    Memory,
    /// I/O-bound operation
    Io,
    /// Network-bound operation
    Network,
}

/// Severity level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Performance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Report ID
    pub id: String,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Total commands profiled
    pub total_commands: usize,
    /// Average execution time
    pub avg_execution_time: Duration,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Memory statistics
    pub memory_stats: MemoryStatistics,
    /// Command statistics
    pub command_stats: HashMap<String, CommandStatistics>,
    /// Detected bottlenecks
    pub bottlenecks: Vec<Bottleneck>,
    /// Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
}

/// Memory statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    /// Average memory usage (bytes)
    pub avg_memory: u64,
    /// Peak memory usage (bytes)
    pub peak_memory: u64,
    /// Minimum memory usage (bytes)
    pub min_memory: u64,
}

/// Statistics for a specific command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandStatistics {
    /// Number of executions
    pub count: usize,
    /// Average duration
    pub avg_duration: Duration,
    /// Minimum duration
    pub min_duration: Duration,
    /// Maximum duration
    pub max_duration: Duration,
    /// Average memory usage
    pub avg_memory: u64,
}

/// Optimization suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// Suggestion title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Impact level
    pub impact: Impact,
    /// Implementation difficulty
    pub difficulty: Difficulty,
}

/// Impact level of an optimization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Impact {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
}

/// Difficulty of implementing an optimization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    /// Easy to implement
    Easy,
    /// Medium difficulty
    Medium,
    /// Hard to implement
    Hard,
}

impl PerformanceProfiler {
    /// Create a new performance profiler.
    pub fn new() -> Result<Self> {
        let base_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?
            .join("legalis")
            .join("performance");

        fs::create_dir_all(&base_dir)?;

        Ok(Self {
            base_dir,
            session: None,
            enabled: false,
        })
    }

    /// Enable profiling.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable profiling.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if profiling is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Start a new profiling session.
    pub fn start_session(&mut self) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let session = ProfilingSession {
            id: id.clone(),
            started_at: Utc::now(),
            metrics: Vec::new(),
            memory_snapshots: Vec::new(),
        };
        self.session = Some(session);
        Ok(id)
    }

    /// End the current profiling session.
    pub fn end_session(&mut self) -> Result<Option<ProfilingSession>> {
        if let Some(session) = self.session.take() {
            // Save session to disk
            let session_file = self.base_dir.join(format!("{}.json", session.id));
            let json_str = serde_json::to_string_pretty(&session)?;
            fs::write(session_file, json_str)?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// Record a command execution.
    pub fn record_command(
        &mut self,
        command: &str,
        args: Vec<String>,
        duration: Duration,
        memory_used: u64,
        exit_code: Option<i32>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let metric = CommandMetric {
            command: command.to_string(),
            args,
            started_at: Utc::now(),
            duration,
            memory_used,
            exit_code,
            category: PerformanceCategory::from_duration(duration),
        };

        if let Some(ref mut session) = self.session {
            session.metrics.push(metric);
        }

        Ok(())
    }

    /// Take a memory snapshot.
    pub fn snapshot_memory(&mut self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        #[cfg(target_os = "linux")]
        let snapshot = self.get_memory_snapshot_linux()?;

        #[cfg(not(target_os = "linux"))]
        let snapshot = self.get_memory_snapshot_fallback();

        if let Some(ref mut session) = self.session {
            session.memory_snapshots.push(snapshot);
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn get_memory_snapshot_linux(&self) -> Result<MemorySnapshot> {
        let status = fs::read_to_string("/proc/self/status")?;
        let mut rss = 0u64;
        let mut vm_size = 0u64;

        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    rss = parts[1].parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            } else if line.starts_with("VmSize:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    vm_size = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                }
            }
        }

        Ok(MemorySnapshot {
            timestamp: Utc::now(),
            total_memory: rss,
            rss,
            virtual_memory: vm_size,
        })
    }

    #[cfg(not(target_os = "linux"))]
    fn get_memory_snapshot_fallback(&self) -> MemorySnapshot {
        // Fallback for non-Linux systems
        MemorySnapshot {
            timestamp: Utc::now(),
            total_memory: 0,
            rss: 0,
            virtual_memory: 0,
        }
    }

    /// Analyze session and detect bottlenecks.
    pub fn detect_bottlenecks(&self, session: &ProfilingSession) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();

        // Detect slow commands
        for metric in &session.metrics {
            if matches!(metric.category, PerformanceCategory::VerySlow) {
                bottlenecks.push(Bottleneck {
                    command: metric.command.clone(),
                    bottleneck_type: BottleneckType::Cpu,
                    severity: Severity::High,
                    description: format!(
                        "Command '{}' took {:.2}s to execute",
                        metric.command,
                        metric.duration.as_secs_f64()
                    ),
                    suggestion: "Consider optimizing this command or running it in the background"
                        .to_string(),
                });
            } else if matches!(metric.category, PerformanceCategory::Slow) {
                bottlenecks.push(Bottleneck {
                    command: metric.command.clone(),
                    bottleneck_type: BottleneckType::Cpu,
                    severity: Severity::Medium,
                    description: format!(
                        "Command '{}' took {:.2}s to execute",
                        metric.command,
                        metric.duration.as_secs_f64()
                    ),
                    suggestion: "Consider caching results or optimizing the operation".to_string(),
                });
            }

            // Detect high memory usage
            if metric.memory_used > 100 * 1024 * 1024 {
                // > 100MB
                bottlenecks.push(Bottleneck {
                    command: metric.command.clone(),
                    bottleneck_type: BottleneckType::Memory,
                    severity: Severity::High,
                    description: format!(
                        "Command '{}' used {:.2}MB of memory",
                        metric.command,
                        metric.memory_used as f64 / (1024.0 * 1024.0)
                    ),
                    suggestion: "Consider processing data in chunks or streaming".to_string(),
                });
            }
        }

        bottlenecks
    }

    /// Generate optimization suggestions.
    pub fn generate_suggestions(&self, session: &ProfilingSession) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Count command frequencies
        let mut command_counts: HashMap<String, usize> = HashMap::new();
        for metric in &session.metrics {
            *command_counts.entry(metric.command.clone()).or_insert(0) += 1;
        }

        // Suggest caching for frequently used commands
        for (command, count) in &command_counts {
            if *count > 5 {
                suggestions.push(OptimizationSuggestion {
                    title: format!("Enable caching for '{}'", command),
                    description: format!(
                        "Command '{}' was executed {} times. Consider enabling caching to improve performance.",
                        command, count
                    ),
                    impact: Impact::High,
                    difficulty: Difficulty::Easy,
                });
            }
        }

        // Suggest batch operations for multiple similar commands
        let parse_count = command_counts.get("parse").unwrap_or(&0);
        let verify_count = command_counts.get("verify").unwrap_or(&0);
        if *parse_count > 3 || *verify_count > 3 {
            suggestions.push(OptimizationSuggestion {
                title: "Use batch operations".to_string(),
                description: "Multiple similar commands were executed. Consider using batch operations for better performance.".to_string(),
                impact: Impact::High,
                difficulty: Difficulty::Easy,
            });
        }

        // Suggest parallel execution
        if session.metrics.len() > 10 {
            suggestions.push(OptimizationSuggestion {
                title: "Enable parallel execution".to_string(),
                description: "Many commands were executed sequentially. Consider using parallel execution for independent operations.".to_string(),
                impact: Impact::Medium,
                difficulty: Difficulty::Medium,
            });
        }

        suggestions
    }

    /// Generate a performance report.
    pub fn generate_report(&self, session: &ProfilingSession) -> PerformanceReport {
        let total_commands = session.metrics.len();
        let total_execution_time: Duration = session.metrics.iter().map(|m| m.duration).sum();
        let avg_execution_time = if total_commands > 0 {
            total_execution_time / total_commands as u32
        } else {
            Duration::from_secs(0)
        };

        // Calculate memory statistics
        let memory_stats = if !session.memory_snapshots.is_empty() {
            let memories: Vec<u64> = session
                .memory_snapshots
                .iter()
                .map(|s| s.total_memory)
                .collect();
            let avg_memory = memories.iter().sum::<u64>() / memories.len() as u64;
            let peak_memory = *memories.iter().max().unwrap_or(&0);
            let min_memory = *memories.iter().min().unwrap_or(&0);

            MemoryStatistics {
                avg_memory,
                peak_memory,
                min_memory,
            }
        } else {
            MemoryStatistics {
                avg_memory: 0,
                peak_memory: 0,
                min_memory: 0,
            }
        };

        // Calculate command statistics
        let mut command_stats: HashMap<String, CommandStatistics> = HashMap::new();
        for metric in &session.metrics {
            let entry = command_stats
                .entry(metric.command.clone())
                .or_insert_with(|| CommandStatistics {
                    count: 0,
                    avg_duration: Duration::from_secs(0),
                    min_duration: Duration::from_secs(u64::MAX),
                    max_duration: Duration::from_secs(0),
                    avg_memory: 0,
                });

            entry.count += 1;
            entry.min_duration = entry.min_duration.min(metric.duration);
            entry.max_duration = entry.max_duration.max(metric.duration);
        }

        // Calculate averages
        for metric in &session.metrics {
            if let Some(stats) = command_stats.get_mut(&metric.command) {
                let total_duration: Duration = session
                    .metrics
                    .iter()
                    .filter(|m| m.command == metric.command)
                    .map(|m| m.duration)
                    .sum();
                stats.avg_duration = total_duration / stats.count as u32;

                let total_memory: u64 = session
                    .metrics
                    .iter()
                    .filter(|m| m.command == metric.command)
                    .map(|m| m.memory_used)
                    .sum();
                stats.avg_memory = total_memory / stats.count as u64;
            }
        }

        let bottlenecks = self.detect_bottlenecks(session);
        let suggestions = self.generate_suggestions(session);

        PerformanceReport {
            id: uuid::Uuid::new_v4().to_string(),
            generated_at: Utc::now(),
            total_commands,
            avg_execution_time,
            total_execution_time,
            memory_stats,
            command_stats,
            bottlenecks,
            suggestions,
        }
    }

    /// Load a profiling session from disk.
    pub fn load_session(&self, session_id: &str) -> Result<ProfilingSession> {
        let session_file = self.base_dir.join(format!("{}.json", session_id));
        let json_str = fs::read_to_string(session_file)?;
        let session: ProfilingSession = serde_json::from_str(&json_str)?;
        Ok(session)
    }

    /// List all profiling sessions.
    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let mut sessions = Vec::new();
        if self.base_dir.exists() {
            for entry in fs::read_dir(&self.base_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    if let Some(filename) = entry.file_name().to_str() {
                        if filename.ends_with(".json") {
                            sessions.push(filename.trim_end_matches(".json").to_string());
                        }
                    }
                }
            }
        }
        Ok(sessions)
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new().expect("Failed to create performance profiler")
    }
}

/// Command execution timer helper.
pub struct CommandTimer {
    start: Instant,
}

impl CommandTimer {
    /// Start a new timer.
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Get elapsed time.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}
