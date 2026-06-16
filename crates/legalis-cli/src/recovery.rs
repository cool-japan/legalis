//! Automatic error recovery for transient command failures.
//!
//! Some CLI operations touch resources that can fail *transiently* — a network
//! probe, a registry sync, a file briefly locked by another process. This module
//! provides a small, dependency-free retry engine that:
//!
//! - distinguishes **transient** from **permanent** failures via a classifier,
//! - retries transient failures with **exponential backoff + jitter**,
//! - is fully deterministic in tests (the backoff *sleeper* is injectable, so no
//!   wall-clock time is consumed).
//!
//! The default classifier recognizes common transient I/O conditions
//! (`Interrupted`, `WouldBlock`, `TimedOut`, `ConnectionReset`,
//! `ConnectionRefused`, …) and transient phrases in `anyhow` error chains.

use std::io;
use std::time::Duration;

/// Whether a failure should be retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transience {
    /// Retry-able; the operation may succeed if attempted again.
    Transient,
    /// Not retry-able; retrying will not help.
    Permanent,
}

impl Transience {
    /// Whether this classification permits a retry.
    pub fn is_transient(self) -> bool {
        matches!(self, Transience::Transient)
    }
}

/// Configuration for the retry engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Maximum number of *attempts* (including the first). Must be >= 1.
    pub max_attempts: u32,
    /// Base backoff used for the first retry.
    pub base_delay: Duration,
    /// Cap on any single backoff delay.
    pub max_delay: Duration,
    /// Multiplier applied to the delay after each retry.
    pub multiplier: u32,
    /// Whether to add deterministic jitter (based on attempt number).
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            multiplier: 2,
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// A policy that performs no retries (single attempt).
    pub fn no_retry() -> Self {
        Self {
            max_attempts: 1,
            ..Self::default()
        }
    }

    /// Builder: set the maximum number of attempts (clamped to >= 1).
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts.max(1);
        self
    }

    /// Builder: set the base delay.
    pub fn with_base_delay(mut self, delay: Duration) -> Self {
        self.base_delay = delay;
        self
    }

    /// Computes the backoff delay before the retry following `attempt`
    /// (1-based: `delay_for(1)` is the wait before attempt #2).
    pub fn delay_for(&self, attempt: u32) -> Duration {
        let exponent = attempt.saturating_sub(1);
        let factor = self.multiplier.saturating_pow(exponent).max(1);
        let raw = self.base_delay.saturating_mul(factor).min(self.max_delay);
        if self.jitter {
            // Deterministic jitter: add up to 25% based on the attempt number,
            // keeping tests reproducible while still de-synchronizing retries.
            let millis = raw.as_millis() as u64;
            let extra = (millis / 4).saturating_mul((attempt as u64 % 4) + 1) / 4;
            Duration::from_millis(millis.saturating_add(extra)).min(self.max_delay)
        } else {
            raw
        }
    }
}

/// The outcome of a retried operation, including diagnostics.
#[derive(Debug)]
pub struct RetryOutcome<T> {
    /// The successful value, or the final error.
    pub result: anyhow::Result<T>,
    /// How many attempts were made.
    pub attempts: u32,
    /// The total simulated backoff that was waited.
    pub total_backoff: Duration,
}

/// Classifies an [`io::Error`] as transient or permanent.
pub fn classify_io(error: &io::Error) -> Transience {
    use io::ErrorKind::*;
    match error.kind() {
        Interrupted | WouldBlock | TimedOut | ConnectionReset | ConnectionAborted
        | ConnectionRefused | NotConnected | BrokenPipe | AddrInUse | AddrNotAvailable
        | ResourceBusy => Transience::Transient,
        _ => Transience::Permanent,
    }
}

/// Classifies an [`anyhow::Error`] by walking its source chain for an
/// [`io::Error`] and otherwise scanning the message for transient phrases.
pub fn classify_anyhow(error: &anyhow::Error) -> Transience {
    for cause in error.chain() {
        if let Some(io_err) = cause.downcast_ref::<io::Error>() {
            return classify_io(io_err);
        }
    }
    let message = error.to_string().to_ascii_lowercase();
    const TRANSIENT_PHRASES: &[&str] = &[
        "timed out",
        "timeout",
        "temporarily unavailable",
        "connection reset",
        "connection refused",
        "would block",
        "try again",
        "resource busy",
        "interrupted",
    ];
    if TRANSIENT_PHRASES.iter().any(|p| message.contains(p)) {
        Transience::Transient
    } else {
        Transience::Permanent
    }
}

/// Trait abstracting "waiting" so tests can avoid real sleeps.
pub trait Backoff {
    /// Wait for `delay` (or simulate doing so).
    fn wait(&mut self, delay: Duration);
}

/// A real sleeper that blocks the thread.
#[derive(Debug, Default)]
pub struct ThreadSleeper;

impl Backoff for ThreadSleeper {
    fn wait(&mut self, delay: Duration) {
        if !delay.is_zero() {
            std::thread::sleep(delay);
        }
    }
}

/// A test backoff that records waits without sleeping.
#[derive(Debug, Default)]
pub struct RecordingBackoff {
    /// The sequence of waits requested.
    pub waits: Vec<Duration>,
}

impl Backoff for RecordingBackoff {
    fn wait(&mut self, delay: Duration) {
        self.waits.push(delay);
    }
}

/// Runs `operation` under `policy`, retrying transient failures (classified by
/// `classify`) and waiting via `backoff`.
///
/// Returns a [`RetryOutcome`] describing the final result and the work done.
pub fn retry_with<T, F, C, B>(
    policy: &RetryPolicy,
    classify: C,
    backoff: &mut B,
    mut operation: F,
) -> RetryOutcome<T>
where
    F: FnMut(u32) -> anyhow::Result<T>,
    C: Fn(&anyhow::Error) -> Transience,
    B: Backoff,
{
    let max = policy.max_attempts.max(1);
    let mut total_backoff = Duration::ZERO;
    let mut attempt = 0;
    loop {
        attempt += 1;
        match operation(attempt) {
            Ok(value) => {
                return RetryOutcome {
                    result: Ok(value),
                    attempts: attempt,
                    total_backoff,
                };
            }
            Err(error) => {
                let transient = classify(&error).is_transient();
                if !transient || attempt >= max {
                    return RetryOutcome {
                        result: Err(error),
                        attempts: attempt,
                        total_backoff,
                    };
                }
                let delay = policy.delay_for(attempt);
                total_backoff = total_backoff.saturating_add(delay);
                backoff.wait(delay);
            }
        }
    }
}

/// Convenience wrapper: retry using the default `anyhow` classifier and a real
/// thread-blocking sleeper. Returns just the final `Result`.
pub fn retry<T, F>(policy: &RetryPolicy, operation: F) -> anyhow::Result<T>
where
    F: FnMut(u32) -> anyhow::Result<T>,
{
    let mut sleeper = ThreadSleeper;
    retry_with(policy, classify_anyhow, &mut sleeper, operation).result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn transient_err(msg: &str) -> anyhow::Error {
        anyhow::anyhow!("{}", msg)
    }

    #[test]
    fn test_success_first_try() {
        let policy = RetryPolicy::default();
        let mut backoff = RecordingBackoff::default();
        let outcome = retry_with(&policy, classify_anyhow, &mut backoff, |_| {
            Ok::<_, anyhow::Error>(42)
        });
        assert_eq!(outcome.attempts, 1);
        assert!(backoff.waits.is_empty());
        assert_eq!(outcome.result.expect("ok"), 42);
    }

    #[test]
    fn test_retries_transient_then_succeeds() {
        let policy = RetryPolicy::default().with_max_attempts(5);
        let mut backoff = RecordingBackoff::default();
        let calls = AtomicU32::new(0);
        let outcome = retry_with(&policy, classify_anyhow, &mut backoff, |_| {
            let n = calls.fetch_add(1, Ordering::SeqCst) + 1;
            if n < 3 {
                Err(transient_err("connection reset by peer"))
            } else {
                Ok(n)
            }
        });
        assert_eq!(outcome.attempts, 3);
        assert_eq!(backoff.waits.len(), 2);
        assert_eq!(outcome.result.expect("ok"), 3);
    }

    #[test]
    fn test_permanent_not_retried() {
        let policy = RetryPolicy::default().with_max_attempts(5);
        let mut backoff = RecordingBackoff::default();
        let calls = AtomicU32::new(0);
        let outcome: RetryOutcome<()> = retry_with(&policy, classify_anyhow, &mut backoff, |_| {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(anyhow::anyhow!("file not found: nonexistent.ldsl"))
        });
        assert_eq!(outcome.attempts, 1);
        assert!(backoff.waits.is_empty());
        assert!(outcome.result.is_err());
    }

    #[test]
    fn test_exhausts_attempts() {
        let policy = RetryPolicy::default().with_max_attempts(3);
        let mut backoff = RecordingBackoff::default();
        let outcome: RetryOutcome<()> = retry_with(&policy, classify_anyhow, &mut backoff, |_| {
            Err(transient_err("operation timed out"))
        });
        assert_eq!(outcome.attempts, 3);
        // Two waits between three attempts.
        assert_eq!(backoff.waits.len(), 2);
        assert!(outcome.result.is_err());
    }

    #[test]
    fn test_no_retry_policy() {
        let policy = RetryPolicy::no_retry();
        let mut backoff = RecordingBackoff::default();
        let outcome: RetryOutcome<()> = retry_with(&policy, classify_anyhow, &mut backoff, |_| {
            Err(transient_err("timeout"))
        });
        assert_eq!(outcome.attempts, 1);
        assert!(backoff.waits.is_empty());
    }

    #[test]
    fn test_backoff_increases_and_caps() {
        let policy = RetryPolicy {
            max_attempts: 10,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(800),
            multiplier: 2,
            jitter: false,
        };
        assert_eq!(policy.delay_for(1), Duration::from_millis(100));
        assert_eq!(policy.delay_for(2), Duration::from_millis(200));
        assert_eq!(policy.delay_for(3), Duration::from_millis(400));
        assert_eq!(policy.delay_for(4), Duration::from_millis(800));
        // Capped at max_delay.
        assert_eq!(policy.delay_for(5), Duration::from_millis(800));
    }

    #[test]
    fn test_jitter_stays_within_cap() {
        let policy = RetryPolicy {
            max_attempts: 10,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(800),
            multiplier: 2,
            jitter: true,
        };
        for attempt in 1..=8 {
            assert!(policy.delay_for(attempt) <= Duration::from_millis(800));
        }
    }

    #[test]
    fn test_classify_io() {
        let interrupted = io::Error::from(io::ErrorKind::Interrupted);
        assert!(classify_io(&interrupted).is_transient());
        let not_found = io::Error::from(io::ErrorKind::NotFound);
        assert!(!classify_io(&not_found).is_transient());
    }

    #[test]
    fn test_classify_anyhow_via_io_source() {
        let io_err = io::Error::from(io::ErrorKind::TimedOut);
        let wrapped = anyhow::Error::new(io_err).context("while syncing");
        assert!(classify_anyhow(&wrapped).is_transient());
    }

    #[test]
    fn test_classify_anyhow_permanent() {
        let error = anyhow::anyhow!("invalid statute syntax");
        assert!(!classify_anyhow(&error).is_transient());
    }

    #[test]
    fn test_total_backoff_accumulates() {
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(1),
            multiplier: 2,
            jitter: false,
        };
        let mut backoff = RecordingBackoff::default();
        let outcome: RetryOutcome<()> = retry_with(&policy, classify_anyhow, &mut backoff, |_| {
            Err(transient_err("would block"))
        });
        // 10ms + 20ms = 30ms total.
        assert_eq!(outcome.total_backoff, Duration::from_millis(30));
    }
}
