//! Provider uptime / availability monitoring.
//!
//! Computes availability statistics from a series of [`HealthProbe`] records:
//! uptime percentage, mean time between failures (MTBF), mean time to recovery
//! (MTTR), the longest outage, the list of downtime incidents and SLA
//! compliance. The current rollup status reuses the crate's existing
//! [`crate::HealthStatus`].
//!
//! ## Live-transport boundary
//!
//! [`UptimeMonitor::probe`] performs a *real* health probe by issuing a tiny
//! request through any [`crate::LLMProvider`] and timing it (so it works offline
//! against a mock provider). Running such probes continuously on a schedule and
//! against remote endpoints is an operational concern handled by an external
//! scheduler - this module models, records and analyses probe results.

use super::mean;
use crate::{HealthStatus, LLMProvider};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Serde adapter for the crate's [`HealthStatus`], which is not itself
/// `Serialize`/`Deserialize`. It is represented as a lowercase string so that
/// [`UptimeStats`] can round-trip without modifying the reused enum.
mod health_status_serde {
    use crate::HealthStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        status: &HealthStatus,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let label = match status {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
        };
        serializer.serialize_str(label)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<HealthStatus, D::Error> {
        let label = String::deserialize(deserializer)?;
        Ok(match label.as_str() {
            "degraded" => HealthStatus::Degraded,
            "unhealthy" => HealthStatus::Unhealthy,
            _ => HealthStatus::Healthy,
        })
    }
}

/// A single point-in-time health probe of a provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthProbe {
    /// When the probe was taken.
    pub timestamp: DateTime<Utc>,
    /// Provider that was probed.
    pub provider: String,
    /// Whether the provider responded successfully.
    pub up: bool,
    /// Probe round-trip latency in milliseconds.
    pub latency_ms: u64,
    /// Error message when the probe failed.
    pub error: Option<String>,
}

impl HealthProbe {
    /// Creates a successful "up" probe.
    pub fn up(provider: impl Into<String>, latency_ms: u64) -> Self {
        Self {
            timestamp: Utc::now(),
            provider: provider.into(),
            up: true,
            latency_ms,
            error: None,
        }
    }

    /// Creates a failed "down" probe.
    pub fn down(provider: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            provider: provider.into(),
            up: false,
            latency_ms: 0,
            error: Some(error.into()),
        }
    }

    /// Overrides the timestamp (otherwise "now").
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// A contiguous run of "down" probes - a single outage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DowntimeIncident {
    /// When the outage began (first down probe).
    pub started_at: DateTime<Utc>,
    /// When service was first observed restored, if it has been.
    pub ended_at: Option<DateTime<Utc>>,
    /// Outage duration in seconds, if resolved.
    pub duration_secs: Option<f64>,
    /// Number of failed probes during the outage.
    pub failed_probes: usize,
}

impl DowntimeIncident {
    /// Returns whether the outage is still ongoing.
    pub fn is_ongoing(&self) -> bool {
        self.ended_at.is_none()
    }
}

/// Aggregated uptime statistics for one provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UptimeStats {
    /// Provider name.
    pub provider: String,
    /// Total probes taken.
    pub total_probes: usize,
    /// Number of "up" probes.
    pub up_probes: usize,
    /// Number of "down" probes.
    pub down_probes: usize,
    /// Probe-based availability in `[0, 100]`.
    pub uptime_pct: f64,
    /// Mean latency over successful probes (ms).
    pub avg_latency_ms: f64,
    /// Current rollup status.
    #[serde(with = "health_status_serde")]
    pub current_status: HealthStatus,
    /// Mean time between failures, in seconds (`None` if undefined).
    pub mtbf_secs: Option<f64>,
    /// Mean time to recovery, in seconds (`None` if no resolved outage).
    pub mttr_secs: Option<f64>,
    /// Longest resolved outage, in seconds (`None` if no resolved outage).
    pub longest_downtime_secs: Option<f64>,
    /// All detected downtime incidents.
    pub incidents: Vec<DowntimeIncident>,
}

impl UptimeStats {
    /// Returns whether availability meets a target SLA percentage.
    pub fn meets_sla(&self, target_pct: f64) -> bool {
        self.uptime_pct >= target_pct
    }
}

/// Computes uptime statistics for one provider from its probes.
///
/// The probes need not be pre-sorted; they are sorted by timestamp internally.
pub fn uptime_stats(provider: &str, probes: &[HealthProbe]) -> UptimeStats {
    let mut sorted: Vec<&HealthProbe> = probes.iter().filter(|p| p.provider == provider).collect();
    sorted.sort_by_key(|p| p.timestamp);

    let total = sorted.len();
    let up_probes = sorted.iter().filter(|p| p.up).count();
    let down_probes = total - up_probes;
    let uptime_pct = if total == 0 {
        100.0
    } else {
        up_probes as f64 / total as f64 * 100.0
    };

    let up_latencies: Vec<f64> = sorted
        .iter()
        .filter(|p| p.up)
        .map(|p| p.latency_ms as f64)
        .collect();
    let avg_latency_ms = mean(&up_latencies);

    let incidents = detect_incidents(&sorted);
    let resolved: Vec<&DowntimeIncident> = incidents.iter().filter(|i| !i.is_ongoing()).collect();

    let total_downtime: f64 = resolved.iter().filter_map(|i| i.duration_secs).sum();
    let mttr_secs = if resolved.is_empty() {
        None
    } else {
        Some(total_downtime / resolved.len() as f64)
    };
    let longest_downtime_secs = resolved
        .iter()
        .filter_map(|i| i.duration_secs)
        .fold(None, |acc: Option<f64>, value| {
            Some(acc.map_or(value, |current| current.max(value)))
        });

    let mtbf_secs = compute_mtbf(&sorted, &incidents, total_downtime);
    let current_status = rollup_status(&sorted);

    UptimeStats {
        provider: provider.to_string(),
        total_probes: total,
        up_probes,
        down_probes,
        uptime_pct,
        avg_latency_ms,
        current_status,
        mtbf_secs,
        mttr_secs,
        longest_downtime_secs,
        incidents,
    }
}

/// Walks sorted probes and groups contiguous "down" runs into incidents.
fn detect_incidents(sorted: &[&HealthProbe]) -> Vec<DowntimeIncident> {
    let mut incidents = Vec::new();
    let mut current: Option<DowntimeIncident> = None;

    for probe in sorted {
        if probe.up {
            if let Some(mut incident) = current.take() {
                incident.ended_at = Some(probe.timestamp);
                incident.duration_secs = Some(
                    (probe.timestamp - incident.started_at)
                        .num_milliseconds()
                        .max(0) as f64
                        / 1000.0,
                );
                incidents.push(incident);
            }
        } else {
            match current.as_mut() {
                Some(incident) => incident.failed_probes += 1,
                None => {
                    current = Some(DowntimeIncident {
                        started_at: probe.timestamp,
                        ended_at: None,
                        duration_secs: None,
                        failed_probes: 1,
                    });
                }
            }
        }
    }

    if let Some(incident) = current.take() {
        incidents.push(incident);
    }
    incidents
}

/// MTBF = total observed uptime / number of failure incidents.
fn compute_mtbf(
    sorted: &[&HealthProbe],
    incidents: &[DowntimeIncident],
    total_downtime: f64,
) -> Option<f64> {
    if incidents.is_empty() {
        return None;
    }
    let first = sorted.first()?;
    let last = sorted.last()?;
    let span = (last.timestamp - first.timestamp).num_milliseconds().max(0) as f64 / 1000.0;
    let uptime = (span - total_downtime).max(0.0);
    Some(uptime / incidents.len() as f64)
}

/// Derives a rollup [`HealthStatus`] from the most recent probes.
fn rollup_status(sorted: &[&HealthProbe]) -> HealthStatus {
    let last = match sorted.last() {
        Some(probe) => probe,
        None => return HealthStatus::Healthy,
    };
    if !last.up {
        return HealthStatus::Unhealthy;
    }
    let recent_window = 10.min(sorted.len());
    let recent_failures = sorted[sorted.len() - recent_window..]
        .iter()
        .filter(|p| !p.up)
        .count();
    if recent_failures == 0 {
        HealthStatus::Healthy
    } else {
        HealthStatus::Degraded
    }
}

/// A stateful, multi-provider uptime monitor.
pub struct UptimeMonitor {
    probes: Arc<RwLock<Vec<HealthProbe>>>,
}

impl UptimeMonitor {
    /// Creates a new uptime monitor.
    pub fn new() -> Self {
        Self {
            probes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Records a probe result.
    pub async fn record(&self, probe: HealthProbe) {
        self.probes.write().await.push(probe);
    }

    /// Performs a real health probe through a provider and records the result.
    ///
    /// Issues `prompt` via [`LLMProvider::generate_text`], timing the round-trip;
    /// a successful response records an "up" probe with the measured latency, and
    /// any error records a "down" probe carrying the error text. Returns the
    /// recorded probe.
    pub async fn probe<P: LLMProvider>(&self, provider: &P, prompt: &str) -> HealthProbe {
        let provider_name = provider.provider_name().to_string();
        let start = Instant::now();
        let probe = match provider.generate_text(prompt).await {
            Ok(_) => HealthProbe::up(provider_name, start.elapsed().as_millis() as u64),
            Err(error) => HealthProbe::down(provider_name, error.to_string()),
        };
        self.probes.write().await.push(probe.clone());
        probe
    }

    /// Computes uptime statistics for a single provider.
    pub async fn stats(&self, provider: &str) -> UptimeStats {
        let probes = self.probes.read().await;
        uptime_stats(provider, &probes)
    }

    /// Computes uptime statistics for every observed provider.
    pub async fn stats_all(&self) -> BTreeMap<String, UptimeStats> {
        let probes = self.probes.read().await;
        let mut providers: Vec<String> = probes.iter().map(|p| p.provider.clone()).collect();
        providers.sort();
        providers.dedup();
        providers
            .into_iter()
            .map(|provider| {
                let stats = uptime_stats(&provider, &probes);
                (provider, stats)
            })
            .collect()
    }

    /// Returns whether a provider currently meets a target SLA percentage.
    pub async fn meets_sla(&self, provider: &str, target_pct: f64) -> bool {
        self.stats(provider).await.meets_sla(target_pct)
    }

    /// Returns the total number of recorded probes.
    pub async fn probe_count(&self) -> usize {
        self.probes.read().await.len()
    }
}

impl Default for UptimeMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(base: DateTime<Utc>, secs: i64) -> DateTime<Utc> {
        base + chrono::Duration::seconds(secs)
    }

    #[test]
    fn test_all_up() {
        let base = Utc::now();
        let probes: Vec<HealthProbe> = (0..10)
            .map(|i| HealthProbe::up("openai", 100).with_timestamp(at(base, i * 60)))
            .collect();
        let stats = uptime_stats("openai", &probes);
        assert_eq!(stats.total_probes, 10);
        assert!((stats.uptime_pct - 100.0).abs() < 1e-9);
        assert_eq!(stats.current_status, HealthStatus::Healthy);
        assert!(stats.incidents.is_empty());
        assert!(stats.mttr_secs.is_none());
        assert!((stats.avg_latency_ms - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_outage_and_recovery() {
        let base = Utc::now();
        let probes = vec![
            HealthProbe::up("openai", 100).with_timestamp(at(base, 0)),
            HealthProbe::down("openai", "503").with_timestamp(at(base, 60)),
            HealthProbe::down("openai", "503").with_timestamp(at(base, 120)),
            HealthProbe::up("openai", 110).with_timestamp(at(base, 180)),
            HealthProbe::up("openai", 90).with_timestamp(at(base, 240)),
        ];
        let stats = uptime_stats("openai", &probes);
        assert_eq!(stats.total_probes, 5);
        assert_eq!(stats.down_probes, 2);
        assert!((stats.uptime_pct - 60.0).abs() < 1e-9);
        assert_eq!(stats.incidents.len(), 1);
        let incident = &stats.incidents[0];
        assert!(!incident.is_ongoing());
        // outage from t=60 to t=180 => 120s.
        assert!((incident.duration_secs.unwrap_or(0.0) - 120.0).abs() < 1e-6);
        assert!((stats.mttr_secs.unwrap_or(0.0) - 120.0).abs() < 1e-6);
        assert!(stats.longest_downtime_secs.is_some());
        assert!(stats.mtbf_secs.is_some());
        assert_eq!(stats.current_status, HealthStatus::Degraded);
    }

    #[test]
    fn test_ongoing_outage_status() {
        let base = Utc::now();
        let probes = vec![
            HealthProbe::up("anthropic", 100).with_timestamp(at(base, 0)),
            HealthProbe::down("anthropic", "timeout").with_timestamp(at(base, 60)),
        ];
        let stats = uptime_stats("anthropic", &probes);
        assert_eq!(stats.current_status, HealthStatus::Unhealthy);
        assert_eq!(stats.incidents.len(), 1);
        assert!(stats.incidents[0].is_ongoing());
        // ongoing outage has no resolved MTTR.
        assert!(stats.mttr_secs.is_none());
    }

    #[test]
    fn test_sla() {
        let base = Utc::now();
        let mut probes: Vec<HealthProbe> = (0..99)
            .map(|i| HealthProbe::up("p", 50).with_timestamp(at(base, i * 10)))
            .collect();
        probes.push(HealthProbe::down("p", "x").with_timestamp(at(base, 1000)));
        let stats = uptime_stats("p", &probes);
        assert!(stats.meets_sla(99.0));
        assert!(!stats.meets_sla(99.95));
    }

    #[test]
    fn test_empty_provider_defaults_healthy() {
        let stats = uptime_stats("nobody", &[]);
        assert_eq!(stats.total_probes, 0);
        assert!((stats.uptime_pct - 100.0).abs() < 1e-9);
        assert_eq!(stats.current_status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_monitor_probe_with_mock_provider() {
        let monitor = UptimeMonitor::new();
        let provider = crate::MockProvider::new();
        let probe = monitor.probe(&provider, "ping").await;
        assert!(probe.up);
        assert_eq!(monitor.probe_count().await, 1);

        let stats = monitor.stats(probe.provider.as_str()).await;
        assert_eq!(stats.up_probes, 1);
        assert!(stats.meets_sla(100.0));
    }

    #[tokio::test]
    async fn test_monitor_stats_all() {
        let monitor = UptimeMonitor::new();
        monitor.record(HealthProbe::up("openai", 100)).await;
        monitor.record(HealthProbe::up("anthropic", 120)).await;
        monitor.record(HealthProbe::down("anthropic", "503")).await;
        let all = monitor.stats_all().await;
        assert_eq!(all.len(), 2);
        assert!(all.contains_key("openai"));
        assert!(all.contains_key("anthropic"));
        assert!(monitor.meets_sla("openai", 100.0).await);
    }
}
