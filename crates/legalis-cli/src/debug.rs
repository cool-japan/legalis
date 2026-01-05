//! Debugging utilities for step-through evaluation.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Debug trace containing evaluation steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugTrace {
    /// Evaluation steps
    pub steps: Vec<EvaluationStep>,
    /// Total execution time
    pub total_time: Duration,
    /// Peak memory usage (if available)
    pub peak_memory: Option<usize>,
}

/// A single evaluation step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationStep {
    /// Step number
    pub step_num: usize,
    /// Description of what's being evaluated
    pub description: String,
    /// Input state/variables
    pub inputs: serde_json::Value,
    /// Output/result
    pub output: serde_json::Value,
    /// Time taken for this step
    pub duration: Duration,
    /// Memory delta (if available)
    pub memory_delta: Option<isize>,
    /// Stack depth (for nested evaluations)
    pub depth: usize,
}

impl DebugTrace {
    /// Create a new empty debug trace.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            total_time: Duration::ZERO,
            peak_memory: None,
        }
    }

    /// Add an evaluation step.
    pub fn add_step(&mut self, step: EvaluationStep) {
        self.total_time += step.duration;
        self.steps.push(step);
    }

    /// Format the trace as a human-readable report.
    pub fn format_report(&self, show_timing: bool, show_memory: bool) -> String {
        use colored::Colorize;

        let mut report = String::new();

        report.push_str(&format!("\n{}\n", "=== Debug Trace ===".bold().cyan()));
        report.push_str(&format!(
            "Total Time: {}\n",
            format_duration(self.total_time)
        ));

        if show_memory {
            if let Some(peak) = self.peak_memory {
                report.push_str(&format!("Peak Memory: {}\n\n", format_bytes(peak)));
            } else {
                report.push_str("Peak Memory: N/A\n\n");
            }
        } else {
            report.push_str("\n");
        }

        for step in &self.steps {
            let indent = "  ".repeat(step.depth);

            report.push_str(&format!(
                "{}[Step {}] {}\n",
                indent,
                step.step_num,
                step.description.bold()
            ));

            report.push_str(&format!(
                "{}  Inputs:  {}\n",
                indent,
                serde_json::to_string(&step.inputs).unwrap_or_else(|_| "N/A".to_string())
            ));

            report.push_str(&format!(
                "{}  Output:  {}\n",
                indent,
                serde_json::to_string(&step.output).unwrap_or_else(|_| "N/A".to_string())
            ));

            if show_timing {
                report.push_str(&format!(
                    "{}  Time:    {}\n",
                    indent,
                    format_duration(step.duration)
                ));
            }

            if show_memory {
                if let Some(delta) = step.memory_delta {
                    let sign = if delta >= 0 { "+" } else { "" };
                    report.push_str(&format!(
                        "{}  Memory:  {}{}\n",
                        indent,
                        sign,
                        format_bytes(delta.unsigned_abs())
                    ));
                }
            }

            report.push_str("\n");
        }

        report
    }

    /// Format as JSON.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize debug trace to JSON")
    }
}

impl Default for DebugTrace {
    fn default() -> Self {
        Self::new()
    }
}

/// Debugger for step-through evaluation.
pub struct Debugger {
    trace: DebugTrace,
    current_depth: usize,
    #[allow(dead_code)]
    show_timing: bool,
    show_memory: bool,
    interactive: bool,
}

impl Debugger {
    /// Create a new debugger.
    pub fn new(interactive: bool, show_timing: bool, show_memory: bool) -> Self {
        Self {
            trace: DebugTrace::new(),
            current_depth: 0,
            show_timing,
            show_memory,
            interactive,
        }
    }

    /// Begin a new evaluation step.
    pub fn begin_step(
        &mut self,
        description: impl Into<String>,
        inputs: serde_json::Value,
    ) -> StepGuard<'_> {
        let step_num = self.trace.steps.len() + 1;
        let depth = self.current_depth;
        let _show_memory = self.show_memory;

        self.current_depth += 1;

        #[cfg(target_os = "linux")]
        let start_memory = if _show_memory {
            get_current_memory_usage().ok()
        } else {
            None
        };

        #[cfg(not(target_os = "linux"))]
        let start_memory = None;

        StepGuard {
            debugger: self,
            step_num,
            description: description.into(),
            inputs,
            depth,
            start_time: Instant::now(),
            start_memory,
        }
    }

    /// Get the debug trace.
    pub fn trace(&self) -> &DebugTrace {
        &self.trace
    }

    /// Enter interactive mode (wait for user input).
    pub fn interactive_pause(&self, step: &EvaluationStep) {
        if !self.interactive {
            return;
        }

        use colored::Colorize;

        println!("\n{}", "=== Debugger Paused ===".bold().yellow());
        println!("Step {}: {}", step.step_num, step.description);
        println!(
            "\nInputs: {}",
            serde_json::to_string_pretty(&step.inputs).unwrap_or_default()
        );
        println!("\nPress Enter to continue, 'q' to quit...");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            if input.trim() == "q" {
                println!("Exiting debugger...");
                std::process::exit(0);
            }
        }
    }
}

/// RAII guard for evaluation steps.
pub struct StepGuard<'a> {
    debugger: &'a mut Debugger,
    step_num: usize,
    description: String,
    inputs: serde_json::Value,
    depth: usize,
    start_time: Instant,
    start_memory: Option<usize>,
}

impl<'a> StepGuard<'a> {
    /// Complete the step with an output value.
    pub fn complete(self, output: serde_json::Value) {
        let duration = self.start_time.elapsed();

        #[cfg(target_os = "linux")]
        let memory_delta = if let Some(start_mem) = self.start_memory {
            get_current_memory_usage()
                .ok()
                .map(|end_mem| end_mem as isize - start_mem as isize)
        } else {
            None
        };

        #[cfg(not(target_os = "linux"))]
        let memory_delta = None;

        let step = EvaluationStep {
            step_num: self.step_num,
            description: self.description,
            inputs: self.inputs,
            output,
            duration,
            memory_delta,
            depth: self.depth,
        };

        // Update peak memory
        if let Some(delta) = memory_delta {
            if delta > 0 {
                let current_mem = self.start_memory.unwrap_or(0) + delta as usize;
                if let Some(ref mut peak) = self.debugger.trace.peak_memory {
                    if current_mem > *peak {
                        *peak = current_mem;
                    }
                } else {
                    self.debugger.trace.peak_memory = Some(current_mem);
                }
            }
        }

        // Interactive pause
        self.debugger.interactive_pause(&step);

        self.debugger.trace.add_step(step);
        self.debugger.current_depth -= 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_trace_creation() {
        let trace = DebugTrace::new();
        assert_eq!(trace.steps.len(), 0);
        assert_eq!(trace.total_time, Duration::ZERO);
    }

    #[test]
    fn test_add_step() {
        let mut trace = DebugTrace::new();

        let step = EvaluationStep {
            step_num: 1,
            description: "Test step".to_string(),
            inputs: serde_json::json!({"x": 10}),
            output: serde_json::json!({"y": 20}),
            duration: Duration::from_millis(10),
            memory_delta: Some(1024),
            depth: 0,
        };

        trace.add_step(step);

        assert_eq!(trace.steps.len(), 1);
        assert_eq!(trace.total_time, Duration::from_millis(10));
    }

    #[test]
    fn test_debugger_creation() {
        let debugger = Debugger::new(false, true, true);
        assert_eq!(debugger.trace.steps.len(), 0);
        assert_eq!(debugger.current_depth, 0);
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
}
