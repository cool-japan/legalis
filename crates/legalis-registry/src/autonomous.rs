//! Autonomous Registry Management - Self-healing, auto-scaling, and intelligent monitoring.
//!
//! This module provides autonomous management capabilities:
//! - Self-healing registry nodes with automatic failure recovery
//! - Auto-scaling based on load metrics and predictions
//! - Predictive capacity planning using historical data
//! - Automated backup verification and integrity checking
//! - Anomaly-based intrusion detection with threat scoring

use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ============================================================================
// 1. Self-Healing Registry Nodes
// ============================================================================

/// Health status of a registry node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeHealth {
    /// Node is healthy and operating normally
    Healthy,
    /// Node is degraded but operational
    Degraded,
    /// Node has failed and requires intervention
    Failed,
    /// Node is recovering from failure
    Recovering,
}

/// A healable registry node with automatic recovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealableNode {
    /// Node identifier
    pub node_id: Uuid,
    /// Current health status
    pub health: NodeHealth,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Number of consecutive failures
    pub failure_count: u32,
    /// Recovery attempts made
    pub recovery_attempts: u32,
    /// Maximum recovery attempts before manual intervention
    pub max_recovery_attempts: u32,
    /// Last known error
    pub last_error: Option<String>,
}

impl HealableNode {
    /// Creates a new healable node.
    pub fn new(node_id: Uuid) -> Self {
        Self {
            node_id,
            health: NodeHealth::Healthy,
            last_check: Utc::now(),
            failure_count: 0,
            recovery_attempts: 0,
            max_recovery_attempts: 3,
            last_error: None,
        }
    }

    /// Report a health check result.
    pub fn report_health(&mut self, is_healthy: bool, error: Option<String>) {
        self.last_check = Utc::now();

        if is_healthy {
            self.failure_count = 0;
            if self.health == NodeHealth::Recovering {
                self.health = NodeHealth::Healthy;
                self.recovery_attempts = 0;
            }
        } else {
            self.failure_count += 1;
            self.last_error = error;

            if self.failure_count >= 3 {
                self.health = NodeHealth::Failed;
            } else if self.failure_count >= 1 {
                self.health = NodeHealth::Degraded;
            }
        }
    }

    /// Attempt to heal the node.
    pub fn attempt_recovery(&mut self) -> RecoveryAction {
        if self.recovery_attempts >= self.max_recovery_attempts {
            return RecoveryAction::RequiresManualIntervention;
        }

        self.recovery_attempts += 1;
        self.health = NodeHealth::Recovering;

        match self.failure_count {
            1..=2 => RecoveryAction::SoftRestart,
            3..=5 => RecoveryAction::HardRestart,
            _ => RecoveryAction::Failover,
        }
    }

    /// Check if node needs healing.
    pub fn needs_healing(&self) -> bool {
        matches!(self.health, NodeHealth::Degraded | NodeHealth::Failed)
    }

    /// Reset recovery state after successful manual intervention.
    pub fn reset_recovery(&mut self) {
        self.recovery_attempts = 0;
        self.failure_count = 0;
        self.health = NodeHealth::Healthy;
        self.last_error = None;
    }
}

/// Actions that can be taken to recover a failed node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryAction {
    /// Soft restart (restart service)
    SoftRestart,
    /// Hard restart (restart container/VM)
    HardRestart,
    /// Failover to backup node
    Failover,
    /// Requires manual intervention
    RequiresManualIntervention,
}

/// Self-healing manager for registry nodes.
#[derive(Debug, Clone)]
pub struct SelfHealingManager {
    nodes: Arc<Mutex<HashMap<Uuid, HealableNode>>>,
    healing_log: Arc<Mutex<Vec<HealingEvent>>>,
}

impl SelfHealingManager {
    /// Creates a new self-healing manager.
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
            healing_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a node for monitoring.
    pub fn register_node(&self, node: HealableNode) {
        self.nodes.lock().unwrap().insert(node.node_id, node);
    }

    /// Report node health status.
    pub fn report_health(&self, node_id: Uuid, is_healthy: bool, error: Option<String>) {
        if let Some(node) = self.nodes.lock().unwrap().get_mut(&node_id) {
            let old_health = node.health.clone();
            node.report_health(is_healthy, error);
            let new_health = node.health.clone();

            if old_health != new_health {
                self.log_event(HealingEvent {
                    timestamp: Utc::now(),
                    node_id,
                    old_health,
                    new_health,
                    action_taken: None,
                });
            }
        }
    }

    /// Attempt to heal all unhealthy nodes.
    pub fn heal_nodes(&self) -> Vec<(Uuid, RecoveryAction)> {
        let mut actions = Vec::new();
        let mut nodes = self.nodes.lock().unwrap();

        for (node_id, node) in nodes.iter_mut() {
            if node.needs_healing() {
                let action = node.attempt_recovery();
                actions.push((*node_id, action.clone()));

                self.log_event(HealingEvent {
                    timestamp: Utc::now(),
                    node_id: *node_id,
                    old_health: node.health.clone(),
                    new_health: NodeHealth::Recovering,
                    action_taken: Some(action),
                });
            }
        }

        actions
    }

    /// Get node health status.
    pub fn get_node_health(&self, node_id: Uuid) -> Option<NodeHealth> {
        self.nodes
            .lock()
            .unwrap()
            .get(&node_id)
            .map(|n| n.health.clone())
    }

    /// Get all unhealthy nodes.
    pub fn unhealthy_nodes(&self) -> Vec<HealableNode> {
        self.nodes
            .lock()
            .unwrap()
            .values()
            .filter(|n| n.needs_healing())
            .cloned()
            .collect()
    }

    /// Get healing event log.
    pub fn get_healing_log(&self) -> Vec<HealingEvent> {
        self.healing_log.lock().unwrap().clone()
    }

    fn log_event(&self, event: HealingEvent) {
        self.healing_log.lock().unwrap().push(event);
    }

    /// Clear healing log.
    pub fn clear_log(&self) {
        self.healing_log.lock().unwrap().clear();
    }
}

impl Default for SelfHealingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Event logged during healing operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingEvent {
    pub timestamp: DateTime<Utc>,
    pub node_id: Uuid,
    pub old_health: NodeHealth,
    pub new_health: NodeHealth,
    pub action_taken: Option<RecoveryAction>,
}

// ============================================================================
// 2. Auto-Scaling Based on Load
// ============================================================================

/// Load metrics for a node or cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    /// CPU utilization (0.0 - 1.0)
    pub cpu: f64,
    /// Memory utilization (0.0 - 1.0)
    pub memory: f64,
    /// Request rate (requests per second)
    pub request_rate: f64,
    /// Average response time in milliseconds
    pub response_time_ms: f64,
    /// Error rate (0.0 - 1.0)
    pub error_rate: f64,
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
}

impl LoadMetrics {
    /// Creates a new load metrics snapshot.
    pub fn new(
        cpu: f64,
        memory: f64,
        request_rate: f64,
        response_time_ms: f64,
        error_rate: f64,
    ) -> Self {
        Self {
            cpu,
            memory,
            request_rate,
            response_time_ms,
            error_rate,
            timestamp: Utc::now(),
        }
    }

    /// Calculate overall load score (0.0 - 1.0).
    pub fn load_score(&self) -> f64 {
        // Take max of CPU and memory, adjusted by error rate
        let base_load = self.cpu.max(self.memory);
        (base_load + self.error_rate * 0.2).clamp(0.0, 1.0)
    }

    /// Check if metrics indicate high load.
    pub fn is_high_load(&self) -> bool {
        self.load_score() > 0.8
    }

    /// Check if metrics indicate low load.
    pub fn is_low_load(&self) -> bool {
        self.load_score() < 0.3
    }
}

/// Scaling policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    /// Minimum number of nodes
    pub min_nodes: usize,
    /// Maximum number of nodes
    pub max_nodes: usize,
    /// Target CPU utilization
    pub target_cpu: f64,
    /// Target memory utilization
    pub target_memory: f64,
    /// Scale up threshold
    pub scale_up_threshold: f64,
    /// Scale down threshold
    pub scale_down_threshold: f64,
    /// Cooldown period between scaling actions (seconds)
    pub cooldown_seconds: i64,
}

impl ScalingPolicy {
    /// Creates a default scaling policy.
    pub fn default_policy() -> Self {
        Self {
            min_nodes: 2,
            max_nodes: 10,
            target_cpu: 0.7,
            target_memory: 0.7,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            cooldown_seconds: 300, // 5 minutes
        }
    }

    /// Creates a conservative policy (slower to scale).
    pub fn conservative() -> Self {
        Self {
            min_nodes: 3,
            max_nodes: 8,
            target_cpu: 0.6,
            target_memory: 0.6,
            scale_up_threshold: 0.85,
            scale_down_threshold: 0.2,
            cooldown_seconds: 600, // 10 minutes
        }
    }

    /// Creates an aggressive policy (faster to scale).
    pub fn aggressive() -> Self {
        Self {
            min_nodes: 1,
            max_nodes: 20,
            target_cpu: 0.75,
            target_memory: 0.75,
            scale_up_threshold: 0.7,
            scale_down_threshold: 0.4,
            cooldown_seconds: 120, // 2 minutes
        }
    }
}

/// Auto-scaling decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalingDecision {
    /// Scale up by specified number of nodes
    ScaleUp(usize),
    /// Scale down by specified number of nodes
    ScaleDown(usize),
    /// No scaling needed
    NoChange,
    /// Scaling prevented by cooldown period
    InCooldown,
    /// At minimum or maximum capacity
    AtLimit,
}

/// Auto-scaling manager.
#[derive(Debug, Clone)]
pub struct AutoScaler {
    policy: Arc<Mutex<ScalingPolicy>>,
    current_nodes: Arc<Mutex<usize>>,
    last_scaling_action: Arc<Mutex<Option<DateTime<Utc>>>>,
    scaling_history: Arc<Mutex<Vec<ScalingEvent>>>,
}

impl AutoScaler {
    /// Creates a new auto-scaler with a policy.
    pub fn new(policy: ScalingPolicy) -> Self {
        let min_nodes = policy.min_nodes;
        Self {
            policy: Arc::new(Mutex::new(policy)),
            current_nodes: Arc::new(Mutex::new(min_nodes)),
            last_scaling_action: Arc::new(Mutex::new(None)),
            scaling_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Evaluate metrics and make scaling decision.
    pub fn evaluate(&self, metrics: &LoadMetrics) -> ScalingDecision {
        let policy = self.policy.lock().unwrap();
        let current = *self.current_nodes.lock().unwrap();
        let last_action = *self.last_scaling_action.lock().unwrap();

        // Check cooldown
        if let Some(last) = last_action {
            let elapsed = (Utc::now() - last).num_seconds();
            if elapsed < policy.cooldown_seconds {
                return ScalingDecision::InCooldown;
            }
        }

        let load_score = metrics.load_score();

        // Decide scaling action
        if load_score > policy.scale_up_threshold {
            if current >= policy.max_nodes {
                return ScalingDecision::AtLimit;
            }
            let scale_count = ((load_score - policy.target_cpu) * current as f64).ceil() as usize;
            ScalingDecision::ScaleUp(scale_count.max(1))
        } else if load_score < policy.scale_down_threshold {
            if current <= policy.min_nodes {
                return ScalingDecision::AtLimit;
            }
            let scale_count = ((policy.target_cpu - load_score) * current as f64).ceil() as usize;
            ScalingDecision::ScaleDown(scale_count.max(1))
        } else {
            ScalingDecision::NoChange
        }
    }

    /// Apply a scaling decision.
    pub fn apply_scaling(&self, decision: ScalingDecision) -> usize {
        let mut current = self.current_nodes.lock().unwrap();
        let policy = self.policy.lock().unwrap();
        let old_count = *current;

        let new_count = match decision {
            ScalingDecision::ScaleUp(n) => (*current + n).min(policy.max_nodes),
            ScalingDecision::ScaleDown(n) => (*current - n).max(policy.min_nodes),
            _ => *current,
        };

        if new_count != old_count {
            *current = new_count;
            *self.last_scaling_action.lock().unwrap() = Some(Utc::now());

            self.scaling_history.lock().unwrap().push(ScalingEvent {
                timestamp: Utc::now(),
                old_node_count: old_count,
                new_node_count: new_count,
                decision: decision.clone(),
            });
        }

        new_count
    }

    /// Get current node count.
    pub fn current_node_count(&self) -> usize {
        *self.current_nodes.lock().unwrap()
    }

    /// Get scaling history.
    pub fn scaling_history(&self) -> Vec<ScalingEvent> {
        self.scaling_history.lock().unwrap().clone()
    }

    /// Update scaling policy.
    pub fn update_policy(&self, policy: ScalingPolicy) {
        *self.policy.lock().unwrap() = policy;
    }
}

/// Event logged during scaling operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingEvent {
    pub timestamp: DateTime<Utc>,
    pub old_node_count: usize,
    pub new_node_count: usize,
    pub decision: ScalingDecision,
}

// ============================================================================
// 3. Predictive Capacity Planning
// ============================================================================

/// Historical capacity data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityDataPoint {
    pub timestamp: DateTime<Utc>,
    pub node_count: usize,
    pub total_requests: u64,
    pub average_load: f64,
    pub peak_load: f64,
}

/// Capacity prediction result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPrediction {
    /// Predicted timestamp
    pub for_timestamp: DateTime<Utc>,
    /// Predicted required nodes
    pub predicted_nodes: usize,
    /// Predicted load
    pub predicted_load: f64,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Trend direction
    pub trend: Trend,
}

/// Capacity trend direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    Increasing,
    Decreasing,
    Stable,
}

/// Predictive capacity planner.
#[derive(Debug, Clone)]
pub struct CapacityPlanner {
    history: Arc<Mutex<VecDeque<CapacityDataPoint>>>,
    max_history: usize,
}

impl CapacityPlanner {
    /// Creates a new capacity planner.
    pub fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(VecDeque::new())),
            max_history: 1000,
        }
    }

    /// Record a capacity data point.
    pub fn record(&self, data_point: CapacityDataPoint) {
        let mut history = self.history.lock().unwrap();
        history.push_back(data_point);

        if history.len() > self.max_history {
            history.pop_front();
        }
    }

    /// Predict capacity needs for a future time.
    pub fn predict(&self, hours_ahead: i64) -> Option<CapacityPrediction> {
        let history = self.history.lock().unwrap();

        if history.len() < 10 {
            return None; // Not enough data
        }

        // Simple linear regression on load data
        let recent: Vec<_> = history.iter().rev().take(100).collect();
        let n = recent.len() as f64;

        let avg_load: f64 = recent.iter().map(|d| d.average_load).sum::<f64>() / n;
        let avg_nodes: f64 = recent.iter().map(|d| d.node_count as f64).sum::<f64>() / n;

        // Calculate trend (first half is most recent, second half is older)
        let first_half_avg = recent
            .iter()
            .take(recent.len() / 2)
            .map(|d| d.average_load)
            .sum::<f64>()
            / (n / 2.0);
        let second_half_avg = recent
            .iter()
            .skip(recent.len() / 2)
            .map(|d| d.average_load)
            .sum::<f64>()
            / (n / 2.0);

        let trend = if first_half_avg > second_half_avg * 1.1 {
            Trend::Increasing
        } else if first_half_avg < second_half_avg * 0.9 {
            Trend::Decreasing
        } else {
            Trend::Stable
        };

        // Predict based on trend (first half is recent, second half is old)
        let growth_rate = (first_half_avg - second_half_avg) / second_half_avg;
        let predicted_load = avg_load * (1.0 + growth_rate * (hours_ahead as f64 / 24.0));
        let predicted_nodes = (avg_nodes * (predicted_load / avg_load)).ceil() as usize;

        Some(CapacityPrediction {
            for_timestamp: Utc::now() + Duration::hours(hours_ahead),
            predicted_nodes: predicted_nodes.max(1),
            predicted_load: predicted_load.clamp(0.0, 1.0),
            confidence: (n / self.max_history as f64).min(1.0),
            trend,
        })
    }

    /// Get historical data points.
    pub fn get_history(&self) -> Vec<CapacityDataPoint> {
        self.history.lock().unwrap().iter().cloned().collect()
    }

    /// Calculate capacity utilization over time.
    pub fn utilization_stats(&self) -> CapacityStats {
        let history = self.history.lock().unwrap();

        if history.is_empty() {
            return CapacityStats::default();
        }

        let loads: Vec<f64> = history.iter().map(|d| d.average_load).collect();
        let avg = loads.iter().sum::<f64>() / loads.len() as f64;
        let max = loads.iter().cloned().fold(0.0f64, f64::max);
        let min = loads.iter().cloned().fold(1.0f64, f64::min);

        CapacityStats {
            average_utilization: avg,
            peak_utilization: max,
            minimum_utilization: min,
            data_points: history.len(),
        }
    }
}

impl Default for CapacityPlanner {
    fn default() -> Self {
        Self::new()
    }
}

/// Capacity utilization statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapacityStats {
    pub average_utilization: f64,
    pub peak_utilization: f64,
    pub minimum_utilization: f64,
    pub data_points: usize,
}

// ============================================================================
// 4. Automated Backup Verification
// ============================================================================

/// Backup verification result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVerification {
    pub backup_id: Uuid,
    pub verified_at: DateTime<Utc>,
    pub integrity_check: bool,
    pub completeness_check: bool,
    pub restoration_test: bool,
    pub errors: Vec<String>,
}

impl BackupVerification {
    /// Check if backup passed all verifications.
    pub fn is_valid(&self) -> bool {
        self.integrity_check
            && self.completeness_check
            && self.restoration_test
            && self.errors.is_empty()
    }

    /// Get verification score (0.0 - 1.0).
    pub fn score(&self) -> f64 {
        let mut score = 0.0;
        if self.integrity_check {
            score += 0.4;
        }
        if self.completeness_check {
            score += 0.3;
        }
        if self.restoration_test {
            score += 0.3;
        }
        score
    }
}

/// Backup metadata for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub backup_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
    pub checksum: String,
    pub record_count: usize,
}

impl BackupMetadata {
    /// Creates new backup metadata.
    pub fn new(size_bytes: u64, checksum: String, record_count: usize) -> Self {
        Self {
            backup_id: Uuid::new_v4(),
            created_at: Utc::now(),
            size_bytes,
            checksum,
            record_count,
        }
    }
}

/// Automated backup verifier.
#[derive(Debug, Clone)]
pub struct BackupVerifier {
    verifications: Arc<Mutex<HashMap<Uuid, BackupVerification>>>,
}

impl BackupVerifier {
    /// Creates a new backup verifier.
    pub fn new() -> Self {
        Self {
            verifications: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Verify a backup.
    pub fn verify_backup(
        &self,
        metadata: &BackupMetadata,
        actual_checksum: String,
        actual_records: usize,
    ) -> BackupVerification {
        let mut errors = Vec::new();

        // Check integrity (checksum)
        let integrity_check = metadata.checksum == actual_checksum;
        if !integrity_check {
            errors.push(format!(
                "Checksum mismatch: expected {}, got {}",
                metadata.checksum, actual_checksum
            ));
        }

        // Check completeness (record count)
        let completeness_check = metadata.record_count == actual_records;
        if !completeness_check {
            errors.push(format!(
                "Record count mismatch: expected {}, got {}",
                metadata.record_count, actual_records
            ));
        }

        // Simulate restoration test (in real implementation, would restore to temp location)
        let restoration_test = integrity_check && completeness_check;

        let verification = BackupVerification {
            backup_id: metadata.backup_id,
            verified_at: Utc::now(),
            integrity_check,
            completeness_check,
            restoration_test,
            errors,
        };

        self.verifications
            .lock()
            .unwrap()
            .insert(metadata.backup_id, verification.clone());
        verification
    }

    /// Get verification result for a backup.
    pub fn get_verification(&self, backup_id: Uuid) -> Option<BackupVerification> {
        self.verifications.lock().unwrap().get(&backup_id).cloned()
    }

    /// Get all verifications.
    pub fn get_all_verifications(&self) -> Vec<BackupVerification> {
        self.verifications
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    /// Get failed verifications.
    pub fn failed_verifications(&self) -> Vec<BackupVerification> {
        self.verifications
            .lock()
            .unwrap()
            .values()
            .filter(|v| !v.is_valid())
            .cloned()
            .collect()
    }

    /// Calculate overall backup health.
    pub fn backup_health_score(&self) -> f64 {
        let verifications = self.verifications.lock().unwrap();
        if verifications.is_empty() {
            return 0.0;
        }

        let total_score: f64 = verifications.values().map(|v| v.score()).sum();
        total_score / verifications.len() as f64
    }
}

impl Default for BackupVerifier {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Anomaly-Based Intrusion Detection
// ============================================================================

/// Security event for anomaly detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub source_ip: String,
    pub user_id: Option<String>,
    pub resource: String,
    pub details: String,
}

/// Type of security event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityEventType {
    LoginAttempt,
    FailedLogin,
    UnauthorizedAccess,
    DataExfiltration,
    AnomalousQuery,
    RateLimitExceeded,
    SuspiciousPattern,
}

impl SecurityEventType {
    /// Get base threat score for this event type.
    pub fn base_threat_score(&self) -> f64 {
        match self {
            SecurityEventType::LoginAttempt => 0.1,
            SecurityEventType::FailedLogin => 0.3,
            SecurityEventType::UnauthorizedAccess => 0.7,
            SecurityEventType::DataExfiltration => 0.9,
            SecurityEventType::AnomalousQuery => 0.5,
            SecurityEventType::RateLimitExceeded => 0.4,
            SecurityEventType::SuspiciousPattern => 0.6,
        }
    }
}

/// Anomaly detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    pub event: SecurityEvent,
    pub is_anomaly: bool,
    pub threat_score: f64,
    pub confidence: f64,
    pub reason: String,
}

impl AnomalyDetection {
    /// Check if this is a high-threat anomaly.
    pub fn is_high_threat(&self) -> bool {
        self.is_anomaly && self.threat_score > 0.7
    }

    /// Check if this requires immediate action.
    pub fn requires_action(&self) -> bool {
        self.is_high_threat() && self.confidence > 0.8
    }
}

/// Anomaly-based intrusion detector.
#[derive(Debug, Clone)]
pub struct IntrusionDetector {
    events: Arc<Mutex<VecDeque<SecurityEvent>>>,
    anomalies: Arc<Mutex<Vec<AnomalyDetection>>>,
    max_events: usize,
}

impl IntrusionDetector {
    /// Creates a new intrusion detector.
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::new())),
            anomalies: Arc::new(Mutex::new(Vec::new())),
            max_events: 10000,
        }
    }

    /// Record a security event and detect anomalies.
    pub fn record_event(&self, event: SecurityEvent) -> Option<AnomalyDetection> {
        let mut events = self.events.lock().unwrap();

        // Check for anomalies before adding
        let detection = self.detect_anomaly(&event, &events);

        events.push_back(event.clone());
        if events.len() > self.max_events {
            events.pop_front();
        }

        if let Some(anomaly) = &detection {
            if anomaly.is_anomaly {
                self.anomalies.lock().unwrap().push(anomaly.clone());
            }
        }

        detection
    }

    fn detect_anomaly(
        &self,
        event: &SecurityEvent,
        history: &VecDeque<SecurityEvent>,
    ) -> Option<AnomalyDetection> {
        let mut threat_score = event.event_type.base_threat_score();
        let mut reasons = Vec::new();
        let mut is_anomaly = false;

        // Check for repeated failed logins from same IP
        if event.event_type == SecurityEventType::FailedLogin {
            let recent_failures = history
                .iter()
                .rev()
                .take(100)
                .filter(|e| {
                    e.event_type == SecurityEventType::FailedLogin
                        && e.source_ip == event.source_ip
                        && (event.timestamp - e.timestamp).num_minutes() < 10
                })
                .count();

            if recent_failures > 5 {
                threat_score += 0.3;
                is_anomaly = true;
                reasons.push(format!("{} failed logins in 10 minutes", recent_failures));
            }
        }

        // Check for unusual access patterns
        if let Some(user_id) = &event.user_id {
            let user_events: Vec<_> = history
                .iter()
                .filter(|e| e.user_id.as_ref() == Some(user_id))
                .collect();

            // Check for access outside normal hours (simplified)
            let hour = event.timestamp.hour();
            if !(6..=22).contains(&hour) {
                threat_score += 0.2;
                reasons.push("Access outside normal hours".to_string());
            }

            // Check for rapid sequential access
            if let Some(last_event) = user_events.last() {
                let time_diff = (event.timestamp - last_event.timestamp).num_seconds();
                if time_diff < 1 {
                    threat_score += 0.3;
                    is_anomaly = true;
                    reasons.push("Suspiciously rapid access pattern".to_string());
                }
            }
        }

        // Check for rate limit violations
        if event.event_type == SecurityEventType::RateLimitExceeded {
            is_anomaly = true;
            reasons.push("Rate limit exceeded".to_string());
        }

        let confidence = if reasons.is_empty() { 0.5 } else { 0.8 };

        Some(AnomalyDetection {
            event: event.clone(),
            is_anomaly: is_anomaly || threat_score > 0.7,
            threat_score: threat_score.min(1.0),
            confidence,
            reason: reasons.join("; "),
        })
    }

    /// Get all detected anomalies.
    pub fn get_anomalies(&self) -> Vec<AnomalyDetection> {
        self.anomalies.lock().unwrap().clone()
    }

    /// Get high-threat anomalies.
    pub fn high_threat_anomalies(&self) -> Vec<AnomalyDetection> {
        self.anomalies
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.is_high_threat())
            .cloned()
            .collect()
    }

    /// Get anomalies requiring immediate action.
    pub fn actionable_anomalies(&self) -> Vec<AnomalyDetection> {
        self.anomalies
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.requires_action())
            .cloned()
            .collect()
    }

    /// Calculate overall security score (0.0 - 1.0, higher is safer).
    pub fn security_score(&self) -> f64 {
        let anomalies = self.anomalies.lock().unwrap();
        if anomalies.is_empty() {
            return 1.0;
        }

        let recent: Vec<_> = anomalies.iter().rev().take(100).collect();
        let avg_threat = recent.iter().map(|a| a.threat_score).sum::<f64>() / recent.len() as f64;

        1.0 - avg_threat
    }

    /// Clear old anomalies.
    pub fn clear_old_anomalies(&self, days: i64) {
        let cutoff = Utc::now() - Duration::days(days);
        self.anomalies
            .lock()
            .unwrap()
            .retain(|a| a.event.timestamp > cutoff);
    }
}

impl Default for IntrusionDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healable_node_creation() {
        let node = HealableNode::new(Uuid::new_v4());
        assert_eq!(node.health, NodeHealth::Healthy);
        assert_eq!(node.failure_count, 0);
        assert!(!node.needs_healing());
    }

    #[test]
    fn test_node_health_degradation() {
        let mut node = HealableNode::new(Uuid::new_v4());

        node.report_health(false, Some("Connection timeout".to_string()));
        assert_eq!(node.health, NodeHealth::Degraded);
        assert_eq!(node.failure_count, 1);

        node.report_health(false, None);
        node.report_health(false, None);
        assert_eq!(node.health, NodeHealth::Failed);
        assert!(node.needs_healing());
    }

    #[test]
    fn test_node_recovery() {
        let mut node = HealableNode::new(Uuid::new_v4());
        node.failure_count = 2;
        node.health = NodeHealth::Degraded;

        let action = node.attempt_recovery();
        assert_eq!(action, RecoveryAction::SoftRestart);
        assert_eq!(node.health, NodeHealth::Recovering);

        node.report_health(true, None);
        assert_eq!(node.health, NodeHealth::Healthy);
        assert_eq!(node.failure_count, 0);
    }

    #[test]
    fn test_self_healing_manager() {
        let manager = SelfHealingManager::new();
        let node_id = Uuid::new_v4();
        let node = HealableNode::new(node_id);

        manager.register_node(node);
        manager.report_health(node_id, false, Some("Error".to_string()));
        manager.report_health(node_id, false, None);
        manager.report_health(node_id, false, None);

        assert_eq!(manager.unhealthy_nodes().len(), 1);

        let actions = manager.heal_nodes();
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_load_metrics() {
        let metrics = LoadMetrics::new(0.8, 0.7, 1000.0, 50.0, 0.01);
        assert!(metrics.is_high_load());
        assert!(!metrics.is_low_load());

        let low_metrics = LoadMetrics::new(0.2, 0.1, 100.0, 10.0, 0.0);
        assert!(low_metrics.is_low_load());
    }

    #[test]
    fn test_auto_scaler_scale_up() {
        let policy = ScalingPolicy::default_policy();
        let scaler = AutoScaler::new(policy);

        let high_load = LoadMetrics::new(0.9, 0.85, 5000.0, 100.0, 0.02);
        let decision = scaler.evaluate(&high_load);

        assert!(matches!(decision, ScalingDecision::ScaleUp(_)));

        let new_count = scaler.apply_scaling(decision);
        assert!(new_count > 2);
    }

    #[test]
    fn test_auto_scaler_scale_down() {
        let policy = ScalingPolicy::default_policy();
        let scaler = AutoScaler::new(policy);
        *scaler.current_nodes.lock().unwrap() = 5;

        let low_load = LoadMetrics::new(0.2, 0.1, 100.0, 10.0, 0.0);
        let decision = scaler.evaluate(&low_load);

        assert!(matches!(decision, ScalingDecision::ScaleDown(_)));
    }

    #[test]
    fn test_capacity_planner() {
        let planner = CapacityPlanner::new();

        for i in 0..20 {
            planner.record(CapacityDataPoint {
                timestamp: Utc::now() - Duration::hours(20 - i),
                node_count: 3 + i as usize / 5,
                total_requests: (1000 * (i + 1)) as u64,
                average_load: 0.5 + (i as f64 * 0.01),
                peak_load: 0.7 + (i as f64 * 0.01),
            });
        }

        let prediction = planner.predict(24);
        assert!(prediction.is_some());

        let stats = planner.utilization_stats();
        assert!(stats.average_utilization > 0.0);
    }

    #[test]
    fn test_backup_verification() {
        let verifier = BackupVerifier::new();
        let metadata = BackupMetadata::new(1024, "abc123".to_string(), 100);

        let verification = verifier.verify_backup(&metadata, "abc123".to_string(), 100);
        assert!(verification.is_valid());
        assert_eq!(verification.score(), 1.0);

        let failed = verifier.verify_backup(&metadata, "wrong".to_string(), 100);
        assert!(!failed.is_valid());
    }

    #[test]
    fn test_intrusion_detector() {
        let detector = IntrusionDetector::new();

        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::FailedLogin,
            source_ip: "192.168.1.1".to_string(),
            user_id: Some("user1".to_string()),
            resource: "/login".to_string(),
            details: "Invalid password".to_string(),
        };

        // Record multiple failed logins
        for _ in 0..10 {
            detector.record_event(event.clone());
        }

        let anomalies = detector.get_anomalies();
        assert!(!anomalies.is_empty());
    }

    #[test]
    fn test_security_event_threat_scores() {
        assert_eq!(SecurityEventType::LoginAttempt.base_threat_score(), 0.1);
        assert_eq!(SecurityEventType::DataExfiltration.base_threat_score(), 0.9);
    }

    #[test]
    fn test_scaling_cooldown() {
        let policy = ScalingPolicy::default_policy();
        let scaler = AutoScaler::new(policy);

        let high_load = LoadMetrics::new(0.9, 0.85, 5000.0, 100.0, 0.02);

        let decision1 = scaler.evaluate(&high_load);
        scaler.apply_scaling(decision1);

        // Immediate second evaluation should be in cooldown
        let decision2 = scaler.evaluate(&high_load);
        assert_eq!(decision2, ScalingDecision::InCooldown);
    }

    #[test]
    fn test_healing_event_log() {
        let manager = SelfHealingManager::new();
        let node_id = Uuid::new_v4();
        let node = HealableNode::new(node_id);

        manager.register_node(node);
        manager.report_health(node_id, false, Some("Error".to_string()));

        let log = manager.get_healing_log();
        assert!(!log.is_empty());

        manager.clear_log();
        assert!(manager.get_healing_log().is_empty());
    }

    #[test]
    fn test_capacity_prediction_trend() {
        let planner = CapacityPlanner::new();

        // Record increasing load
        for i in 0..50 {
            planner.record(CapacityDataPoint {
                timestamp: Utc::now() - Duration::hours(50 - i),
                node_count: 3,
                total_requests: (1000 * (i + 1)) as u64,
                average_load: 0.3 + (i as f64 * 0.01),
                peak_load: 0.5 + (i as f64 * 0.01),
            });
        }

        let prediction = planner.predict(24).unwrap();
        assert_eq!(prediction.trend, Trend::Increasing);
    }

    #[test]
    fn test_backup_health_score() {
        let verifier = BackupVerifier::new();

        let good = BackupMetadata::new(1024, "abc".to_string(), 100);
        verifier.verify_backup(&good, "abc".to_string(), 100);

        let bad = BackupMetadata::new(1024, "xyz".to_string(), 100);
        verifier.verify_backup(&bad, "wrong".to_string(), 50);

        let score = verifier.backup_health_score();
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_anomaly_detection_rapid_access() {
        let detector = IntrusionDetector::new();

        let event1 = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::LoginAttempt,
            source_ip: "192.168.1.1".to_string(),
            user_id: Some("user1".to_string()),
            resource: "/api/data".to_string(),
            details: "Access".to_string(),
        };

        detector.record_event(event1.clone());

        let event2 = SecurityEvent {
            timestamp: Utc::now(),
            ..event1
        };

        let detection = detector.record_event(event2);
        assert!(detection.is_some());
    }

    #[test]
    fn test_scaling_policy_variants() {
        let conservative = ScalingPolicy::conservative();
        assert_eq!(conservative.min_nodes, 3);
        assert!(conservative.cooldown_seconds > 300);

        let aggressive = ScalingPolicy::aggressive();
        assert_eq!(aggressive.min_nodes, 1);
        assert!(aggressive.cooldown_seconds < 300);
    }

    #[test]
    fn test_max_recovery_attempts() {
        let mut node = HealableNode::new(Uuid::new_v4());
        node.max_recovery_attempts = 2;
        node.failure_count = 5;
        node.health = NodeHealth::Failed;

        node.attempt_recovery();
        node.attempt_recovery();
        let action = node.attempt_recovery();

        assert_eq!(action, RecoveryAction::RequiresManualIntervention);
    }

    #[test]
    fn test_security_score_calculation() {
        let detector = IntrusionDetector::new();

        // Initially should be perfect
        assert_eq!(detector.security_score(), 1.0);

        // Add some failed login events to trigger anomaly detection
        for _ in 0..10 {
            detector.record_event(SecurityEvent {
                timestamp: Utc::now(),
                event_type: SecurityEventType::FailedLogin,
                source_ip: "192.168.1.1".to_string(),
                user_id: Some("user1".to_string()),
                resource: "/login".to_string(),
                details: "Failed login".to_string(),
            });
        }

        let score = detector.security_score();
        assert!(score < 1.0 && score > 0.0);
    }
}
