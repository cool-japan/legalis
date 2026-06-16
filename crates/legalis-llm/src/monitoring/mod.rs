//! Production Monitoring & Analytics.
//!
//! A self-contained, pure-Rust production-monitoring toolkit that operates over
//! a stream of request/response *observations*. Every capability in this module
//! works fully offline with no LLM call: anomalies are detected with robust
//! statistics, quality is gated with configurable QA checks, cost-per-query is
//! aggregated from token usage, provider uptime is computed from health probes,
//! errors are classified and tracked by category, user feedback is collected and
//! turned into satisfaction metrics, and A/B experiments are analysed with proper
//! statistical hypothesis testing.
//!
//! ## Live-transport boundary
//!
//! This crate is offline and pure-Rust. The *computations* and an exportable
//! point-in-time [`dashboard::MonitoringSnapshot`] (renderable to the existing
//! [`crate::dashboard::Dashboard`] and to JSON) are fully implemented here.
//! Pushing those snapshots to a browser over a websocket / SSE / HTTP server, or
//! scheduling continuous probes against remote endpoints, is the responsibility
//! of an external transport layer and is intentionally out of scope - this
//! module models and produces the data, it does not host a server.
//!
//! ## Sub-modules
//!
//! * [`stats`] - inferential statistics (normal & Student-t CDFs, hypothesis tests).
//! * [`anomaly`] - statistical anomaly detection over response observations.
//! * [`quality`] - configurable quality-assurance checks and pass-rate metrics.
//! * [`cost`] - cost-per-query tracking and percentile breakdowns.
//! * [`uptime`] - provider uptime / availability monitoring from health probes.
//! * [`errors`] - error-rate tracking by category with burst detection.
//! * [`feedback`] - user feedback collection and satisfaction (CSAT) metrics.
//! * [`experiment`] - A/B test result analysis with significance testing.
//! * [`dashboard`] - the [`dashboard::ProductionMonitor`] orchestrator + snapshot.

mod anomaly;
mod cost;
mod dashboard;
mod errors;
mod experiment;
mod feedback;
mod quality;
mod stats;
mod uptime;

pub use anomaly::*;
pub use cost::*;
pub use dashboard::*;
pub use errors::*;
pub use experiment::*;
pub use feedback::*;
pub use quality::*;
pub use stats::*;
pub use uptime::*;

use crate::TokenUsage;
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Shared data model
// ============================================================================

/// The outcome of a single LLM request.
///
/// A request either succeeded or failed with a classified [`ErrorCategory`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RequestOutcome {
    /// The request completed successfully.
    #[default]
    Success,
    /// The request failed; the payload is the classified error category.
    Error(ErrorCategory),
}

impl RequestOutcome {
    /// Returns whether the outcome is a success.
    pub fn is_success(&self) -> bool {
        matches!(self, RequestOutcome::Success)
    }

    /// Returns whether the outcome is an error.
    pub fn is_error(&self) -> bool {
        matches!(self, RequestOutcome::Error(_))
    }

    /// Returns the error category, if this is an error outcome.
    pub fn error_category(&self) -> Option<ErrorCategory> {
        match self {
            RequestOutcome::Success => None,
            RequestOutcome::Error(category) => Some(*category),
        }
    }
}

/// A classified category for a request failure.
///
/// Categories are derived from a provider's raw error message with
/// [`ErrorCategory::classify`], giving a stable, low-cardinality dimension for
/// error-rate tracking that is independent of free-form error text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// The request timed out.
    Timeout,
    /// The provider rate-limited the request (HTTP 429).
    RateLimit,
    /// Authentication or authorization failed (HTTP 401/403).
    Authentication,
    /// The provider service was unavailable (HTTP 5xx).
    ServiceUnavailable,
    /// The provider returned a malformed / unparseable response.
    InvalidResponse,
    /// A transport / network-level failure (DNS, connection reset, ...).
    Network,
    /// A billing quota / budget limit was exceeded.
    Quota,
    /// The request or response was blocked by a content filter.
    ContentFilter,
    /// The prompt exceeded the model's context window.
    ContextLengthExceeded,
    /// The request was cancelled by the caller.
    Cancelled,
    /// An uncategorised failure.
    Unknown,
}

impl ErrorCategory {
    /// Classifies a raw provider error message into a category.
    ///
    /// The matcher is case-insensitive and checks for the most specific
    /// categories first so that, for example, a "rate limit" message is never
    /// mis-filed as a generic "service unavailable".
    pub fn classify(message: &str) -> ErrorCategory {
        let lower = message.to_lowercase();
        let has = |needles: &[&str]| needles.iter().any(|needle| lower.contains(needle));

        if has(&[
            "rate limit",
            "rate-limit",
            "ratelimit",
            "429",
            "too many requests",
        ]) {
            ErrorCategory::RateLimit
        } else if has(&["quota", "budget", "billing", "insufficient_quota", "credit"]) {
            ErrorCategory::Quota
        } else if has(&[
            "context length",
            "context window",
            "maximum context",
            "too many tokens",
            "max_tokens",
        ]) {
            ErrorCategory::ContextLengthExceeded
        } else if has(&[
            "content filter",
            "content_filter",
            "content policy",
            "safety",
            "flagged",
            "moderation",
        ]) {
            ErrorCategory::ContentFilter
        } else if has(&["cancel", "aborted"]) {
            ErrorCategory::Cancelled
        } else if has(&["timed out", "timeout", "deadline exceeded"]) {
            ErrorCategory::Timeout
        } else if has(&[
            "unauthorized",
            "forbidden",
            "authentication",
            "api key",
            "api_key",
            "401",
            "403",
            "invalid token",
        ]) {
            ErrorCategory::Authentication
        } else if has(&[
            "service unavailable",
            "unavailable",
            "internal server error",
            "bad gateway",
            "500",
            "502",
            "503",
            "504",
            "overloaded",
        ]) {
            ErrorCategory::ServiceUnavailable
        } else if has(&[
            "connection",
            "network",
            "dns",
            "reset by peer",
            "broken pipe",
            "refused",
            "unreachable",
        ]) {
            ErrorCategory::Network
        } else if has(&[
            "parse",
            "malformed",
            "invalid json",
            "deserialize",
            "unexpected response",
            "invalid response",
        ]) {
            ErrorCategory::InvalidResponse
        } else {
            ErrorCategory::Unknown
        }
    }

    /// Returns a stable, lowercase label for the category.
    pub fn label(&self) -> &'static str {
        match self {
            ErrorCategory::Timeout => "timeout",
            ErrorCategory::RateLimit => "rate_limit",
            ErrorCategory::Authentication => "authentication",
            ErrorCategory::ServiceUnavailable => "service_unavailable",
            ErrorCategory::InvalidResponse => "invalid_response",
            ErrorCategory::Network => "network",
            ErrorCategory::Quota => "quota",
            ErrorCategory::ContentFilter => "content_filter",
            ErrorCategory::ContextLengthExceeded => "context_length_exceeded",
            ErrorCategory::Cancelled => "cancelled",
            ErrorCategory::Unknown => "unknown",
        }
    }

    /// Returns whether a failure of this category is typically transient and
    /// worth retrying.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ErrorCategory::Timeout
                | ErrorCategory::RateLimit
                | ErrorCategory::ServiceUnavailable
                | ErrorCategory::Network
        )
    }

    /// All categories, in declaration order. Useful for exhaustive reporting.
    pub fn all() -> [ErrorCategory; 11] {
        [
            ErrorCategory::Timeout,
            ErrorCategory::RateLimit,
            ErrorCategory::Authentication,
            ErrorCategory::ServiceUnavailable,
            ErrorCategory::InvalidResponse,
            ErrorCategory::Network,
            ErrorCategory::Quota,
            ErrorCategory::ContentFilter,
            ErrorCategory::ContextLengthExceeded,
            ErrorCategory::Cancelled,
            ErrorCategory::Unknown,
        ]
    }
}

/// A single request/response observation - the atomic unit of monitoring.
///
/// One observation captures everything the monitoring layer needs about a single
/// LLM call: when it happened, which provider/model/variant served it, how long
/// it took, how many tokens / how much money it consumed, whether it succeeded,
/// and (optionally) the response body for quality analysis. Every analytic in
/// this module is computed from a slice of these records, so they can be fed
/// from a live request path or replayed from a log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseObservation {
    /// Stable unique identifier (used to attach feedback later).
    pub id: String,
    /// When the request completed.
    pub timestamp: DateTime<Utc>,
    /// Provider name (e.g. `"openai"`).
    pub provider: String,
    /// Model name (e.g. `"gpt-4"`).
    pub model: String,
    /// Optional request category (e.g. `"contract_analysis"`).
    pub category: Option<String>,
    /// Optional A/B experiment variant label this request was routed to.
    pub variant: Option<String>,
    /// Optional tenant / user identifier for multi-tenant attribution.
    pub tenant_id: Option<String>,
    /// End-to-end latency in milliseconds.
    pub latency_ms: u64,
    /// Token usage, when known.
    pub usage: Option<TokenUsage>,
    /// Estimated cost in USD, when known.
    pub cost_usd: Option<f64>,
    /// Whether the request succeeded or failed (with category).
    pub outcome: RequestOutcome,
    /// The response body, when retained for quality analysis.
    pub response_text: Option<String>,
    /// Length of the response in characters (`0` for failures).
    pub response_chars: usize,
}

impl ResponseObservation {
    /// Creates a new successful observation with the given provider and model.
    pub fn new(provider: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            provider: provider.into(),
            model: model.into(),
            category: None,
            variant: None,
            tenant_id: None,
            latency_ms: 0,
            usage: None,
            cost_usd: None,
            outcome: RequestOutcome::Success,
            response_text: None,
            response_chars: 0,
        }
    }

    /// Overrides the identifier (otherwise a random UUID is used).
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Overrides the timestamp (otherwise "now").
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the request category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the A/B variant label.
    pub fn with_variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    /// Sets the tenant identifier.
    pub fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Sets the latency.
    pub fn with_latency(mut self, latency_ms: u64) -> Self {
        self.latency_ms = latency_ms;
        self
    }

    /// Sets the token usage.
    pub fn with_usage(mut self, usage: TokenUsage) -> Self {
        self.usage = Some(usage);
        self
    }

    /// Sets the estimated cost.
    pub fn with_cost(mut self, cost_usd: f64) -> Self {
        self.cost_usd = Some(cost_usd);
        self
    }

    /// Attaches the response body and records its character length.
    pub fn with_response(mut self, response: impl Into<String>) -> Self {
        let text = response.into();
        self.response_chars = text.chars().count();
        self.response_text = Some(text);
        self
    }

    /// Marks the observation as a failure with an explicit category.
    pub fn with_error(mut self, category: ErrorCategory) -> Self {
        self.outcome = RequestOutcome::Error(category);
        self.response_chars = 0;
        self.response_text = None;
        self
    }

    /// Marks the observation as a failure, classifying the raw error message.
    pub fn failed(mut self, message: &str) -> Self {
        self.outcome = RequestOutcome::Error(ErrorCategory::classify(message));
        self.response_chars = 0;
        self.response_text = None;
        self
    }

    /// Returns whether the observation is a success.
    pub fn is_success(&self) -> bool {
        self.outcome.is_success()
    }

    /// Returns the total tokens consumed, or `0` when usage is unknown.
    pub fn total_tokens(&self) -> usize {
        self.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
    }

    /// Returns the cost in USD, or `0.0` when unknown.
    pub fn cost_or_zero(&self) -> f64 {
        self.cost_usd.unwrap_or(0.0)
    }
}

// ============================================================================
// Shared descriptive-statistics helpers
// ============================================================================

/// Returns the arithmetic mean of a slice, or `0.0` when empty.
pub(crate) fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Returns the *population* variance of a slice, or `0.0` when empty.
pub(crate) fn population_variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let avg = mean(values);
    values
        .iter()
        .map(|value| (value - avg).powi(2))
        .sum::<f64>()
        / values.len() as f64
}

/// Returns the population standard deviation of a slice.
pub(crate) fn population_std_dev(values: &[f64]) -> f64 {
    population_variance(values).sqrt()
}

/// Returns the *sample* variance (Bessel-corrected, divides by `n - 1`).
///
/// Returns `0.0` for fewer than two values where the correction is undefined.
pub(crate) fn sample_variance(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let avg = mean(values);
    values
        .iter()
        .map(|value| (value - avg).powi(2))
        .sum::<f64>()
        / (values.len() as f64 - 1.0)
}

/// Returns the median of an already-sorted slice, or `0.0` when empty.
pub(crate) fn median_sorted(sorted: &[f64]) -> f64 {
    let len = sorted.len();
    if len == 0 {
        return 0.0;
    }
    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    }
}

/// Returns the median of a slice (sorts a copy internally).
pub(crate) fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    median_sorted(&sorted)
}

/// Returns the linearly-interpolated `p`-th percentile of a sorted slice.
///
/// `p` is a percentage in `[0, 100]`. Uses the "linear interpolation between
/// closest ranks" method (the same definition used by NumPy's default).
pub(crate) fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let clamped = p.clamp(0.0, 100.0);
    let rank = (clamped / 100.0) * (sorted.len() as f64 - 1.0);
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        return sorted[lower];
    }
    let frac = rank - lower as f64;
    sorted[lower] + (sorted[upper] - sorted[lower]) * frac
}

/// Returns the `p`-th percentile of a slice (sorts a copy internally).
pub(crate) fn percentile(values: &[f64], p: f64) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    percentile_sorted(&sorted, p)
}

/// Truncates a timestamp down to the start of its [`TrendGranularity`] bucket.
///
/// Shared by the cost and error-rate trend builders so that both bucket time
/// the same way (hour- or day-aligned in UTC).
pub(crate) fn truncate_to_bucket(
    timestamp: DateTime<Utc>,
    granularity: TrendGranularity,
) -> DateTime<Utc> {
    let naive = timestamp.naive_utc();
    let truncated = match granularity {
        TrendGranularity::Hourly => naive
            .date()
            .and_hms_opt(naive.hour(), 0, 0)
            .unwrap_or(naive),
        TrendGranularity::Daily => naive.date().and_hms_opt(0, 0, 0).unwrap_or(naive),
    };
    DateTime::from_naive_utc_and_offset(truncated, Utc)
}

/// Returns the median absolute deviation (MAD) of a slice, scaled to be a
/// consistent estimator of the standard deviation for normal data.
///
/// `MAD = 1.4826 * median(|x_i - median(x)|)`. The `1.4826` factor makes the
/// MAD comparable to a standard deviation under normality, so MAD-based z-scores
/// can use the same thresholds as ordinary z-scores while being robust to
/// outliers.
pub(crate) fn scaled_mad(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let center = median(values);
    let deviations: Vec<f64> = values.iter().map(|value| (value - center).abs()).collect();
    1.4826 * median(&deviations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_category_classification() {
        assert_eq!(
            ErrorCategory::classify("HTTP 429 Too Many Requests"),
            ErrorCategory::RateLimit
        );
        assert_eq!(
            ErrorCategory::classify("Request timed out after 30s"),
            ErrorCategory::Timeout
        );
        assert_eq!(
            ErrorCategory::classify("401 Unauthorized: invalid api key"),
            ErrorCategory::Authentication
        );
        assert_eq!(
            ErrorCategory::classify("503 Service Unavailable"),
            ErrorCategory::ServiceUnavailable
        );
        assert_eq!(
            ErrorCategory::classify("connection reset by peer"),
            ErrorCategory::Network
        );
        assert_eq!(
            ErrorCategory::classify("insufficient_quota: billing limit"),
            ErrorCategory::Quota
        );
        assert_eq!(
            ErrorCategory::classify("response flagged by content filter"),
            ErrorCategory::ContentFilter
        );
        assert_eq!(
            ErrorCategory::classify("maximum context length exceeded"),
            ErrorCategory::ContextLengthExceeded
        );
        assert_eq!(
            ErrorCategory::classify("failed to parse invalid json"),
            ErrorCategory::InvalidResponse
        );
        assert_eq!(
            ErrorCategory::classify("something weird happened"),
            ErrorCategory::Unknown
        );
    }

    #[test]
    fn test_error_category_metadata() {
        assert!(ErrorCategory::Timeout.is_retryable());
        assert!(ErrorCategory::RateLimit.is_retryable());
        assert!(!ErrorCategory::Authentication.is_retryable());
        assert!(!ErrorCategory::ContentFilter.is_retryable());
        assert_eq!(ErrorCategory::all().len(), 11);
        assert_eq!(
            ErrorCategory::ContextLengthExceeded.label(),
            "context_length_exceeded"
        );
    }

    #[test]
    fn test_request_outcome() {
        let success = RequestOutcome::Success;
        assert!(success.is_success());
        assert!(!success.is_error());
        assert_eq!(success.error_category(), None);

        let failure = RequestOutcome::Error(ErrorCategory::Timeout);
        assert!(failure.is_error());
        assert_eq!(failure.error_category(), Some(ErrorCategory::Timeout));
        assert_eq!(RequestOutcome::default(), RequestOutcome::Success);
    }

    #[test]
    fn test_observation_builder() {
        let obs = ResponseObservation::new("openai", "gpt-4")
            .with_category("analysis")
            .with_variant("a")
            .with_tenant("acme")
            .with_latency(1200)
            .with_usage(TokenUsage::new(1000, 500))
            .with_cost(0.045)
            .with_response("This is a legal analysis of the contract.");

        assert_eq!(obs.provider, "openai");
        assert_eq!(obs.category.as_deref(), Some("analysis"));
        assert_eq!(obs.variant.as_deref(), Some("a"));
        assert_eq!(obs.latency_ms, 1200);
        assert_eq!(obs.total_tokens(), 1500);
        assert!((obs.cost_or_zero() - 0.045).abs() < f64::EPSILON);
        assert!(obs.is_success());
        assert!(obs.response_chars > 0);
    }

    #[test]
    fn test_observation_failure_clears_response() {
        let obs = ResponseObservation::new("openai", "gpt-4")
            .with_response("partial")
            .failed("Request timed out");
        assert!(!obs.is_success());
        assert_eq!(obs.outcome.error_category(), Some(ErrorCategory::Timeout));
        assert_eq!(obs.response_chars, 0);
        assert!(obs.response_text.is_none());
    }

    #[test]
    fn test_descriptive_stats() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((mean(&values) - 5.0).abs() < 1e-9);
        // population std dev of this classic dataset is 2.0
        assert!((population_std_dev(&values) - 2.0).abs() < 1e-9);
        // sample variance (n-1) exceeds population variance (n) for the same data
        assert!(sample_variance(&values) > population_variance(&values));
        assert!((median(&values) - 4.5).abs() < 1e-9);
    }

    #[test]
    fn test_percentile() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert!((percentile(&values, 0.0) - 1.0).abs() < 1e-9);
        assert!((percentile(&values, 100.0) - 10.0).abs() < 1e-9);
        // 50th percentile via linear interpolation = 5.5
        assert!((percentile(&values, 50.0) - 5.5).abs() < 1e-9);
        assert_eq!(percentile(&[], 50.0), 0.0);
    }

    #[test]
    fn test_scaled_mad_robust_to_outliers() {
        let clean = [10.0, 10.0, 10.0, 11.0, 9.0, 10.0, 10.0];
        let with_outlier = [10.0, 10.0, 10.0, 11.0, 9.0, 10.0, 1000.0];
        let mad_clean = scaled_mad(&clean);
        let mad_outlier = scaled_mad(&with_outlier);
        // MAD barely moves despite the huge outlier, unlike std dev.
        assert!((mad_clean - mad_outlier).abs() < mad_clean.max(1.0));
        assert!(population_std_dev(&with_outlier) > 100.0);
    }
}
