//! Progress tracking and estimation for long-running operations.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Progress tracker with ETA estimation.
#[derive(Clone)]
pub struct ProgressTracker {
    bar: ProgressBar,
    start_time: Arc<Mutex<Instant>>,
    items_processed: Arc<Mutex<usize>>,
}

impl ProgressTracker {
    /// Create a new progress tracker with a known total.
    pub fn new(total: u64, message: &str) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) ETA: {eta}")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        bar.set_message(message.to_string());

        Self {
            bar,
            start_time: Arc::new(Mutex::new(Instant::now())),
            items_processed: Arc::new(Mutex::new(0)),
        }
    }

    /// Create a spinner for indeterminate progress.
    pub fn new_spinner(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Invalid spinner template")
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );
        bar.set_message(message.to_string());

        Self {
            bar,
            start_time: Arc::new(Mutex::new(Instant::now())),
            items_processed: Arc::new(Mutex::new(0)),
        }
    }

    /// Create a hidden progress tracker (for quiet mode).
    pub fn hidden() -> Self {
        let bar = ProgressBar::hidden();
        Self {
            bar,
            start_time: Arc::new(Mutex::new(Instant::now())),
            items_processed: Arc::new(Mutex::new(0)),
        }
    }

    /// Increment progress by one.
    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
        if let Ok(mut count) = self.items_processed.lock() {
            *count += delta as usize;
        }
    }

    /// Set the current position.
    pub fn set_position(&self, pos: u64) {
        self.bar.set_position(pos);
        if let Ok(mut count) = self.items_processed.lock() {
            *count = pos as usize;
        }
    }

    /// Update the message.
    pub fn set_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }

    /// Mark as finished.
    pub fn finish(&self) {
        self.bar.finish();
    }

    /// Mark as finished with a message.
    pub fn finish_with_message(&self, message: &str) {
        self.bar.finish_with_message(message.to_string());
    }

    /// Get the estimated time remaining.
    pub fn eta(&self) -> Option<Duration> {
        let start = self.start_time.lock().ok()?;
        let processed = *self.items_processed.lock().ok()?;
        let total = self.bar.length()?;

        if processed == 0 {
            return None;
        }

        let elapsed = start.elapsed();
        let rate = processed as f64 / elapsed.as_secs_f64();
        let remaining = total.saturating_sub(self.bar.position());

        if rate > 0.0 {
            Some(Duration::from_secs_f64(remaining as f64 / rate))
        } else {
            None
        }
    }

    /// Get the current processing rate (items per second).
    pub fn rate(&self) -> f64 {
        let start = self.start_time.lock().ok();
        let processed = self.items_processed.lock().ok();

        match (start, processed) {
            (Some(start), Some(processed)) => {
                let elapsed = start.elapsed();
                if elapsed.as_secs_f64() > 0.0 {
                    *processed as f64 / elapsed.as_secs_f64()
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    /// Enable steady tick for spinners.
    pub fn enable_steady_tick(&self, interval: Duration) {
        self.bar.enable_steady_tick(interval);
    }
}

/// Multi-progress manager for multiple concurrent operations.
pub struct MultiProgressManager {
    multi: MultiProgress,
}

impl MultiProgressManager {
    /// Create a new multi-progress manager.
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    /// Add a progress tracker to the multi-progress.
    pub fn add_tracker(&self, total: u64, message: &str) -> ProgressTracker {
        let tracker = ProgressTracker::new(total, message);
        let bar = self.multi.add(tracker.bar.clone());
        ProgressTracker {
            bar,
            start_time: tracker.start_time,
            items_processed: tracker.items_processed,
        }
    }

    /// Add a spinner to the multi-progress.
    pub fn add_spinner(&self, message: &str) -> ProgressTracker {
        let tracker = ProgressTracker::new_spinner(message);
        let bar = self.multi.add(tracker.bar.clone());
        ProgressTracker {
            bar,
            start_time: tracker.start_time,
            items_processed: tracker.items_processed,
        }
    }
}

impl Default for MultiProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress estimator for operations without a known total.
pub struct ProgressEstimator {
    start_time: Instant,
    items_processed: usize,
    estimated_total: Option<usize>,
}

impl ProgressEstimator {
    /// Create a new progress estimator.
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            items_processed: 0,
            estimated_total: None,
        }
    }

    /// Record progress on an item.
    pub fn record_progress(&mut self, count: usize) {
        self.items_processed += count;
    }

    /// Update the estimated total.
    pub fn update_estimate(&mut self, estimated_total: usize) {
        self.estimated_total = Some(estimated_total);
    }

    /// Get the current processing rate.
    pub fn rate(&self) -> f64 {
        let elapsed = self.start_time.elapsed();
        if elapsed.as_secs_f64() > 0.0 {
            self.items_processed as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get estimated time remaining.
    pub fn eta(&self) -> Option<Duration> {
        let total = self.estimated_total?;
        let remaining = total.saturating_sub(self.items_processed);

        let rate = self.rate();
        if rate > 0.0 {
            Some(Duration::from_secs_f64(remaining as f64 / rate))
        } else {
            None
        }
    }

    /// Get the completion percentage.
    pub fn percent_complete(&self) -> Option<f64> {
        let total = self.estimated_total?;
        if total > 0 {
            Some((self.items_processed as f64 / total as f64) * 100.0)
        } else {
            None
        }
    }

    /// Get elapsed time.
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Default for ProgressEstimator {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a duration as a human-readable string.
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

/// Format a rate as items per second.
pub fn format_rate(rate: f64) -> String {
    if rate < 1.0 {
        format!("{:.2} items/s", rate)
    } else if rate < 1000.0 {
        format!("{:.1} items/s", rate)
    } else {
        format!("{:.1}k items/s", rate / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new(100, "Testing");
        assert_eq!(tracker.bar.length(), Some(100));
        tracker.finish();
    }

    #[test]
    fn test_progress_tracker_increment() {
        let tracker = ProgressTracker::new(100, "Testing");
        tracker.inc(10);
        assert_eq!(tracker.bar.position(), 10);
        tracker.finish();
    }

    #[test]
    fn test_progress_tracker_set_position() {
        let tracker = ProgressTracker::new(100, "Testing");
        tracker.set_position(50);
        assert_eq!(tracker.bar.position(), 50);
        tracker.finish();
    }

    #[test]
    fn test_progress_estimator() {
        let mut estimator = ProgressEstimator::new();
        estimator.record_progress(10);
        assert_eq!(estimator.items_processed, 10);

        estimator.update_estimate(100);
        assert_eq!(estimator.estimated_total, Some(100));

        let percent = estimator.percent_complete();
        assert!(percent.is_some());
        assert!((percent.unwrap() - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
    }

    #[test]
    fn test_format_rate() {
        assert_eq!(format_rate(0.5), "0.50 items/s");
        assert_eq!(format_rate(10.5), "10.5 items/s");
        assert_eq!(format_rate(1500.0), "1.5k items/s");
    }

    #[test]
    fn test_multi_progress_manager() {
        let manager = MultiProgressManager::new();
        let tracker1 = manager.add_tracker(100, "Task 1");
        let tracker2 = manager.add_tracker(50, "Task 2");

        tracker1.inc(10);
        tracker2.inc(5);

        assert_eq!(tracker1.bar.position(), 10);
        assert_eq!(tracker2.bar.position(), 5);

        tracker1.finish();
        tracker2.finish();
    }
}
