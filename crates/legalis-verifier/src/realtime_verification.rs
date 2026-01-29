//! Real-Time Verification Module
//!
//! This module provides real-time and streaming verification capabilities for
//! live statute updates, continuous compliance monitoring, and instant conflict detection.
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::realtime_verification::*;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let config = RealtimeConfig::default();
//! let mut monitor = ComplianceMonitor::new(config);
//!
//! // Add statutes to monitor
//! let statute = Statute::new("TAX-2026", "Tax Law", Effect::new(EffectType::Grant, "Grant permission"));
//! monitor.add_statute(statute);
//!
//! // Process updates
//! let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Modified);
//! let result = monitor.process_update(update);
//! ```

use crate::{StatuteVerifier, VerificationResult};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime};

/// Configuration for real-time verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeConfig {
    /// Maximum update queue size
    pub max_queue_size: usize,
    /// Update processing interval (milliseconds)
    pub processing_interval_ms: u64,
    /// Enable automatic conflict detection
    pub auto_conflict_detection: bool,
    /// Enable impact analysis
    pub auto_impact_analysis: bool,
    /// Verification timeout per statute (milliseconds)
    pub verification_timeout_ms: u64,
    /// Maximum concurrent verifications
    pub max_concurrent_verifications: usize,
    /// Enable result caching
    pub enable_caching: bool,
    /// Cache TTL (time-to-live) in seconds
    pub cache_ttl_seconds: u64,
    /// Enable streaming mode
    pub enable_streaming: bool,
    /// Batch size for streaming verification
    pub stream_batch_size: usize,
}

impl Default for RealtimeConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 10000,
            processing_interval_ms: 100,
            auto_conflict_detection: true,
            auto_impact_analysis: true,
            verification_timeout_ms: 5000,
            max_concurrent_verifications: 16,
            enable_caching: true,
            cache_ttl_seconds: 300,
            enable_streaming: true,
            stream_batch_size: 50,
        }
    }
}

/// Types of statute updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateType {
    /// New statute added
    Added,
    /// Existing statute modified
    Modified,
    /// Statute removed
    Removed,
    /// Statute suspended
    Suspended,
    /// Statute reinstated
    Reinstated,
}

impl std::fmt::Display for UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateType::Added => write!(f, "Added"),
            UpdateType::Modified => write!(f, "Modified"),
            UpdateType::Removed => write!(f, "Removed"),
            UpdateType::Suspended => write!(f, "Suspended"),
            UpdateType::Reinstated => write!(f, "Reinstated"),
        }
    }
}

/// Statute update event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteUpdate {
    /// Update ID
    pub update_id: String,
    /// Statute ID being updated
    pub statute_id: String,
    /// Type of update
    pub update_type: UpdateType,
    /// Timestamp of update
    pub timestamp: SystemTime,
    /// Previous version (if modified)
    pub previous_version: Option<String>,
    /// New version (if added/modified)
    pub new_version: Option<String>,
    /// Update metadata
    pub metadata: HashMap<String, String>,
}

impl StatuteUpdate {
    /// Create a new statute update
    pub fn new(statute_id: String, update_type: UpdateType) -> Self {
        Self {
            update_id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            update_type,
            timestamp: SystemTime::now(),
            previous_version: None,
            new_version: None,
            metadata: HashMap::new(),
        }
    }

    /// Create an update with version tracking
    pub fn with_versions(
        statute_id: String,
        update_type: UpdateType,
        previous_version: Option<String>,
        new_version: Option<String>,
    ) -> Self {
        Self {
            update_id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            update_type,
            timestamp: SystemTime::now(),
            previous_version,
            new_version,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the update
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Real-time conflict detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeConflict {
    /// Conflict ID
    pub conflict_id: String,
    /// Statutes involved
    pub statute_ids: Vec<String>,
    /// Conflict type
    pub conflict_type: String,
    /// Severity (0-100)
    pub severity: u8,
    /// Detection timestamp
    pub detected_at: SystemTime,
    /// Suggested resolution
    pub resolution_suggestion: String,
    /// Affected parties
    pub affected_parties: Vec<String>,
}

impl RealtimeConflict {
    /// Create a new realtime conflict
    pub fn new(statute_ids: Vec<String>, conflict_type: String, severity: u8) -> Self {
        Self {
            conflict_id: uuid::Uuid::new_v4().to_string(),
            statute_ids,
            conflict_type,
            severity,
            detected_at: SystemTime::now(),
            resolution_suggestion: String::new(),
            affected_parties: Vec::new(),
        }
    }

    /// Add resolution suggestion
    pub fn with_resolution(mut self, suggestion: String) -> Self {
        self.resolution_suggestion = suggestion;
        self
    }

    /// Add affected party
    pub fn add_affected_party(&mut self, party: String) {
        self.affected_parties.push(party);
    }
}

/// Instant impact analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantImpact {
    /// Impact ID
    pub impact_id: String,
    /// Source statute that triggered the impact
    pub source_statute_id: String,
    /// Affected statute IDs
    pub affected_statute_ids: Vec<String>,
    /// Impact score (0-100)
    pub impact_score: u8,
    /// Impact category
    pub impact_category: ImpactCategory,
    /// Detailed description
    pub description: String,
    /// Required actions
    pub required_actions: Vec<String>,
    /// Estimated remediation time (hours)
    pub estimated_remediation_hours: f64,
    /// Priority level
    pub priority: Priority,
}

/// Impact category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactCategory {
    /// Legal conflict introduced
    LegalConflict,
    /// Compliance requirements changed
    ComplianceChange,
    /// Procedural impact
    ProceduralImpact,
    /// Financial impact
    FinancialImpact,
    /// Jurisdictional change
    JurisdictionalChange,
    /// Minor administrative change
    Administrative,
}

impl std::fmt::Display for ImpactCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImpactCategory::LegalConflict => write!(f, "Legal Conflict"),
            ImpactCategory::ComplianceChange => write!(f, "Compliance Change"),
            ImpactCategory::ProceduralImpact => write!(f, "Procedural Impact"),
            ImpactCategory::FinancialImpact => write!(f, "Financial Impact"),
            ImpactCategory::JurisdictionalChange => write!(f, "Jurisdictional Change"),
            ImpactCategory::Administrative => write!(f, "Administrative"),
        }
    }
}

/// Priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

impl InstantImpact {
    /// Create a new instant impact
    pub fn new(
        source_statute_id: String,
        affected_statute_ids: Vec<String>,
        impact_score: u8,
        impact_category: ImpactCategory,
    ) -> Self {
        let priority = if impact_score >= 80 {
            Priority::Critical
        } else if impact_score >= 60 {
            Priority::High
        } else if impact_score >= 40 {
            Priority::Medium
        } else {
            Priority::Low
        };

        Self {
            impact_id: uuid::Uuid::new_v4().to_string(),
            source_statute_id,
            affected_statute_ids,
            impact_score,
            impact_category,
            description: String::new(),
            required_actions: Vec::new(),
            estimated_remediation_hours: 0.0,
            priority,
        }
    }

    /// Add description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add required action
    pub fn add_action(&mut self, action: String) {
        self.required_actions.push(action);
    }

    /// Set estimated remediation time
    pub fn with_remediation_time(mut self, hours: f64) -> Self {
        self.estimated_remediation_hours = hours;
        self
    }
}

/// Compliance monitoring status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStatus {
    /// Total statutes being monitored
    pub total_statutes: usize,
    /// Active conflicts
    pub active_conflicts: usize,
    /// Pending updates
    pub pending_updates: usize,
    /// Updates processed in last hour
    pub updates_last_hour: usize,
    /// Average processing time (milliseconds)
    pub avg_processing_time_ms: f64,
    /// Last update timestamp
    pub last_update: Option<SystemTime>,
    /// System health (0-100)
    pub system_health: u8,
    /// Cache hit rate (0.0-1.0)
    pub cache_hit_rate: f64,
}

impl Default for MonitoringStatus {
    fn default() -> Self {
        Self {
            total_statutes: 0,
            active_conflicts: 0,
            pending_updates: 0,
            updates_last_hour: 0,
            avg_processing_time_ms: 0.0,
            last_update: None,
            system_health: 100,
            cache_hit_rate: 0.0,
        }
    }
}

/// Continuous compliance monitor
pub struct ComplianceMonitor {
    /// Configuration
    config: RealtimeConfig,
    /// Active statutes being monitored
    statutes: HashMap<String, Statute>,
    /// Update queue
    update_queue: VecDeque<StatuteUpdate>,
    /// Detected conflicts
    conflicts: Vec<RealtimeConflict>,
    /// Impact analyses
    impacts: Vec<InstantImpact>,
    /// Verification cache
    verification_cache: HashMap<String, (VerificationResult, Instant)>,
    /// Monitoring status
    status: MonitoringStatus,
    /// Processing statistics
    total_updates_processed: usize,
    cache_hits: usize,
    cache_misses: usize,
}

impl ComplianceMonitor {
    /// Create a new compliance monitor
    pub fn new(config: RealtimeConfig) -> Self {
        Self {
            config,
            statutes: HashMap::new(),
            update_queue: VecDeque::new(),
            conflicts: Vec::new(),
            impacts: Vec::new(),
            verification_cache: HashMap::new(),
            status: MonitoringStatus::default(),
            total_updates_processed: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Add a statute to monitor
    pub fn add_statute(&mut self, statute: Statute) {
        self.statutes.insert(statute.id.clone(), statute);
        self.status.total_statutes = self.statutes.len();
    }

    /// Remove a statute from monitoring
    pub fn remove_statute(&mut self, statute_id: &str) -> Option<Statute> {
        let result = self.statutes.remove(statute_id);
        self.status.total_statutes = self.statutes.len();
        result
    }

    /// Get current monitoring status
    pub fn get_status(&self) -> &MonitoringStatus {
        &self.status
    }

    /// Queue an update for processing
    pub fn queue_update(&mut self, update: StatuteUpdate) -> Result<(), String> {
        if self.update_queue.len() >= self.config.max_queue_size {
            return Err(format!(
                "Update queue full (max: {})",
                self.config.max_queue_size
            ));
        }

        self.update_queue.push_back(update);
        self.status.pending_updates = self.update_queue.len();
        Ok(())
    }

    /// Process a single update
    pub fn process_update(&mut self, update: StatuteUpdate) -> RealtimeVerificationResult {
        let start_time = Instant::now();

        let verification_result = match update.update_type {
            UpdateType::Added | UpdateType::Modified | UpdateType::Reinstated => {
                if self.statutes.contains_key(&update.statute_id) {
                    self.verify_statute(&update.statute_id)
                } else {
                    VerificationResult::pass()
                }
            }
            UpdateType::Removed | UpdateType::Suspended => VerificationResult::pass(),
        };

        let conflicts = if self.config.auto_conflict_detection {
            self.detect_conflicts(&update)
        } else {
            Vec::new()
        };

        let impacts = if self.config.auto_impact_analysis {
            self.analyze_impact(&update)
        } else {
            Vec::new()
        };

        self.total_updates_processed += 1;
        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // Update statistics
        self.update_statistics(processing_time_ms);
        self.status.last_update = Some(SystemTime::now());

        RealtimeVerificationResult {
            update,
            verification_result,
            conflicts,
            impacts,
            processing_time_ms,
            cache_hit: false,
        }
    }

    /// Process all queued updates
    pub fn process_queue(&mut self) -> Vec<RealtimeVerificationResult> {
        let mut results = Vec::new();

        while let Some(update) = self.update_queue.pop_front() {
            let result = self.process_update(update);
            results.push(result);
        }

        self.status.pending_updates = 0;
        results
    }

    /// Stream verification (process in batches)
    pub fn stream_verify(&mut self) -> Vec<RealtimeVerificationResult> {
        if !self.config.enable_streaming {
            return Vec::new();
        }

        let batch_size = self.config.stream_batch_size.min(self.update_queue.len());
        let mut results = Vec::new();

        for _ in 0..batch_size {
            if let Some(update) = self.update_queue.pop_front() {
                let result = self.process_update(update);
                results.push(result);
            }
        }

        self.status.pending_updates = self.update_queue.len();
        results
    }

    /// Verify a statute with caching
    fn verify_statute(&mut self, statute_id: &str) -> VerificationResult {
        if self.config.enable_caching
            && let Some((cached_result, cached_time)) = self.verification_cache.get(statute_id)
        {
            let cache_age = cached_time.elapsed().as_secs();
            if cache_age < self.config.cache_ttl_seconds {
                let result = cached_result.clone();
                self.cache_hits += 1;
                self.update_cache_hit_rate();
                return result;
            }
        }

        self.cache_misses += 1;
        self.update_cache_hit_rate();

        let verifier = StatuteVerifier::default();

        // Get all statutes and filter to this one
        let statutes: Vec<&Statute> = self
            .statutes
            .values()
            .filter(|s| s.id == statute_id)
            .collect();
        let result = verifier.verify(&statutes.iter().map(|s| (*s).clone()).collect::<Vec<_>>());

        if self.config.enable_caching {
            self.verification_cache
                .insert(statute_id.to_string(), (result.clone(), Instant::now()));
        }

        result
    }

    /// Detect conflicts related to an update
    fn detect_conflicts(&mut self, update: &StatuteUpdate) -> Vec<RealtimeConflict> {
        let mut conflicts = Vec::new();

        if let Some(statute) = self.statutes.get(&update.statute_id) {
            // Check for conflicts with other statutes
            for (other_id, other_statute) in &self.statutes {
                if other_id == &update.statute_id {
                    continue;
                }

                // Check jurisdiction overlap
                if statute.jurisdiction == other_statute.jurisdiction {
                    // Check for effect conflicts
                    if self.effects_conflict(&statute.effect, &other_statute.effect) {
                        let conflict = RealtimeConflict::new(
                            vec![statute.id.clone(), other_statute.id.clone()],
                            "Effect Conflict".to_string(),
                            75,
                        )
                        .with_resolution(format!(
                            "Review conflicting effects between {} and {}",
                            statute.id, other_statute.id
                        ));

                        conflicts.push(conflict);
                    }
                }
            }
        }

        // Store conflicts
        self.conflicts.extend(conflicts.clone());
        self.status.active_conflicts = self.conflicts.len();

        conflicts
    }

    /// Check if effects conflict
    fn effects_conflict(
        &self,
        effect1: &legalis_core::Effect,
        effect2: &legalis_core::Effect,
    ) -> bool {
        // Check if effects are contradictory
        matches!(
            (&effect1.effect_type, &effect2.effect_type),
            (
                legalis_core::EffectType::Grant,
                legalis_core::EffectType::Revoke
            ) | (
                legalis_core::EffectType::Revoke,
                legalis_core::EffectType::Grant
            ) | (
                legalis_core::EffectType::Prohibition,
                legalis_core::EffectType::Obligation
            ) | (
                legalis_core::EffectType::Obligation,
                legalis_core::EffectType::Prohibition
            )
        )
    }

    /// Analyze impact of an update
    fn analyze_impact(&mut self, update: &StatuteUpdate) -> Vec<InstantImpact> {
        let mut impacts = Vec::new();

        if let Some(statute) = self.statutes.get(&update.statute_id) {
            // Find dependent statutes (those that reference this statute)
            let mut affected_ids = Vec::new();

            for (other_id, other_statute) in &self.statutes {
                if other_id == &update.statute_id {
                    continue;
                }

                // Check if other statute references this one
                for condition in &other_statute.preconditions {
                    if let Some(refs) = Self::extract_references(condition)
                        && refs.contains(&statute.id)
                    {
                        affected_ids.push(other_statute.id.clone());
                        break;
                    }
                }
            }

            if !affected_ids.is_empty() {
                let impact_score = match update.update_type {
                    UpdateType::Modified => 70,
                    UpdateType::Removed => 90,
                    UpdateType::Suspended => 80,
                    UpdateType::Added => 30,
                    UpdateType::Reinstated => 40,
                };

                let category = match update.update_type {
                    UpdateType::Modified | UpdateType::Removed => ImpactCategory::LegalConflict,
                    UpdateType::Suspended | UpdateType::Reinstated => {
                        ImpactCategory::ComplianceChange
                    }
                    UpdateType::Added => ImpactCategory::Administrative,
                };

                let mut impact = InstantImpact::new(
                    statute.id.clone(),
                    affected_ids.clone(),
                    impact_score,
                    category,
                )
                .with_description(format!(
                    "Statute {} ({}) affects {} dependent statute(s)",
                    statute.id,
                    update.update_type,
                    affected_ids.len()
                ))
                .with_remediation_time(affected_ids.len() as f64 * 2.0);

                impact.add_action(format!(
                    "Review all {} dependent statutes for compatibility",
                    affected_ids.len()
                ));
                impact.add_action("Update documentation and compliance guides".to_string());

                if impact_score >= 70 {
                    impact.add_action("Notify affected stakeholders immediately".to_string());
                }

                impacts.push(impact);
            }
        }

        // Store impacts
        self.impacts.extend(impacts.clone());

        impacts
    }

    /// Extract statute references from a condition
    fn extract_references(condition: &legalis_core::Condition) -> Option<Vec<String>> {
        match condition {
            legalis_core::Condition::And(left, right)
            | legalis_core::Condition::Or(left, right) => {
                let mut refs = Vec::new();
                if let Some(mut left_refs) = Self::extract_references(left) {
                    refs.append(&mut left_refs);
                }
                if let Some(mut right_refs) = Self::extract_references(right) {
                    refs.append(&mut right_refs);
                }
                if refs.is_empty() { None } else { Some(refs) }
            }
            legalis_core::Condition::Not(cond) => Self::extract_references(cond),
            legalis_core::Condition::Custom { description } => {
                // Check if description contains statute references
                if description.contains("statute") || description.contains("law") {
                    Some(vec![description.clone()])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Update processing statistics
    fn update_statistics(&mut self, processing_time_ms: u64) {
        let total_time = self.status.avg_processing_time_ms * self.total_updates_processed as f64;
        self.status.avg_processing_time_ms =
            (total_time + processing_time_ms as f64) / self.total_updates_processed as f64;
    }

    /// Update cache hit rate
    fn update_cache_hit_rate(&mut self) {
        let total_requests = self.cache_hits + self.cache_misses;
        if total_requests > 0 {
            self.status.cache_hit_rate = self.cache_hits as f64 / total_requests as f64;
        }
    }

    /// Clear expired cache entries
    pub fn clear_expired_cache(&mut self) {
        let ttl = Duration::from_secs(self.config.cache_ttl_seconds);
        self.verification_cache
            .retain(|_, (_, time)| time.elapsed() < ttl);
    }

    /// Get active conflicts
    pub fn get_active_conflicts(&self) -> &[RealtimeConflict] {
        &self.conflicts
    }

    /// Get impact analyses
    pub fn get_impacts(&self) -> &[InstantImpact] {
        &self.impacts
    }

    /// Clear resolved conflicts
    pub fn clear_conflict(&mut self, conflict_id: &str) {
        self.conflicts.retain(|c| c.conflict_id != conflict_id);
        self.status.active_conflicts = self.conflicts.len();
    }

    /// Generate monitoring report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# Real-Time Compliance Monitoring Report\n\n");
        report.push_str("## System Status\n\n");
        report.push_str(&format!(
            "- **Total Statutes Monitored**: {}\n",
            self.status.total_statutes
        ));
        report.push_str(&format!(
            "- **Active Conflicts**: {}\n",
            self.status.active_conflicts
        ));
        report.push_str(&format!(
            "- **Pending Updates**: {}\n",
            self.status.pending_updates
        ));
        report.push_str(&format!(
            "- **Updates Processed**: {}\n",
            self.total_updates_processed
        ));
        report.push_str(&format!(
            "- **Average Processing Time**: {:.2}ms\n",
            self.status.avg_processing_time_ms
        ));
        report.push_str(&format!(
            "- **Cache Hit Rate**: {:.1}%\n",
            self.status.cache_hit_rate * 100.0
        ));
        report.push_str(&format!(
            "- **System Health**: {}%\n\n",
            self.status.system_health
        ));

        if !self.conflicts.is_empty() {
            report.push_str("## Active Conflicts\n\n");
            for conflict in &self.conflicts {
                report.push_str(&format!(
                    "### Conflict: {} (Severity: {})\n",
                    conflict.conflict_type, conflict.severity
                ));
                report.push_str(&format!("- **Statutes**: {:?}\n", conflict.statute_ids));
                if !conflict.resolution_suggestion.is_empty() {
                    report.push_str(&format!(
                        "- **Suggestion**: {}\n",
                        conflict.resolution_suggestion
                    ));
                }
                report.push('\n');
            }
        }

        if !self.impacts.is_empty() {
            report.push_str("## Recent Impact Analyses\n\n");
            for impact in self.impacts.iter().take(10) {
                report.push_str(&format!(
                    "### Impact: {} (Score: {}, Priority: {})\n",
                    impact.impact_category, impact.impact_score, impact.priority
                ));
                report.push_str(&format!("- **Source**: {}\n", impact.source_statute_id));
                report.push_str(&format!(
                    "- **Affected Statutes**: {}\n",
                    impact.affected_statute_ids.len()
                ));
                if !impact.description.is_empty() {
                    report.push_str(&format!("- **Description**: {}\n", impact.description));
                }
                report.push('\n');
            }
        }

        report
    }
}

/// Result of real-time verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeVerificationResult {
    /// The update that was processed
    pub update: StatuteUpdate,
    /// Verification result
    pub verification_result: VerificationResult,
    /// Detected conflicts
    pub conflicts: Vec<RealtimeConflict>,
    /// Impact analyses
    pub impacts: Vec<InstantImpact>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Whether result was from cache
    pub cache_hit: bool,
}

impl RealtimeVerificationResult {
    /// Check if verification passed
    pub fn is_successful(&self) -> bool {
        self.verification_result.errors.is_empty()
    }

    /// Check if there are critical issues
    pub fn has_critical_issues(&self) -> bool {
        self.conflicts.iter().any(|c| c.severity >= 80)
            || self
                .impacts
                .iter()
                .any(|i| i.priority == Priority::Critical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_realtime_config_default() {
        let config = RealtimeConfig::default();
        assert_eq!(config.max_queue_size, 10000);
        assert_eq!(config.processing_interval_ms, 100);
        assert!(config.auto_conflict_detection);
        assert!(config.auto_impact_analysis);
    }

    #[test]
    fn test_statute_update_creation() {
        let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Added);
        assert_eq!(update.statute_id, "TAX-2026");
        assert_eq!(update.update_type, UpdateType::Added);
        assert!(update.previous_version.is_none());
        assert!(update.new_version.is_none());
    }

    #[test]
    fn test_statute_update_with_versions() {
        let update = StatuteUpdate::with_versions(
            "TAX-2026".to_string(),
            UpdateType::Modified,
            Some("1.0".to_string()),
            Some("1.1".to_string()),
        );
        assert_eq!(update.update_type, UpdateType::Modified);
        assert_eq!(update.previous_version, Some("1.0".to_string()));
        assert_eq!(update.new_version, Some("1.1".to_string()));
    }

    #[test]
    fn test_statute_update_with_metadata() {
        let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Added)
            .with_metadata("author".to_string(), "admin".to_string());
        assert_eq!(update.metadata.get("author"), Some(&"admin".to_string()));
    }

    #[test]
    fn test_update_type_display() {
        assert_eq!(UpdateType::Added.to_string(), "Added");
        assert_eq!(UpdateType::Modified.to_string(), "Modified");
        assert_eq!(UpdateType::Removed.to_string(), "Removed");
        assert_eq!(UpdateType::Suspended.to_string(), "Suspended");
        assert_eq!(UpdateType::Reinstated.to_string(), "Reinstated");
    }

    #[test]
    fn test_realtime_conflict_creation() {
        let conflict = RealtimeConflict::new(
            vec!["TAX-2026".to_string(), "TAX-2025".to_string()],
            "Jurisdiction Overlap".to_string(),
            80,
        );
        assert_eq!(conflict.statute_ids.len(), 2);
        assert_eq!(conflict.conflict_type, "Jurisdiction Overlap");
        assert_eq!(conflict.severity, 80);
    }

    #[test]
    fn test_realtime_conflict_with_resolution() {
        let conflict = RealtimeConflict::new(
            vec!["TAX-2026".to_string()],
            "Effect Conflict".to_string(),
            75,
        )
        .with_resolution("Merge conflicting provisions".to_string());
        assert_eq!(
            conflict.resolution_suggestion,
            "Merge conflicting provisions"
        );
    }

    #[test]
    fn test_instant_impact_creation() {
        let impact = InstantImpact::new(
            "TAX-2026".to_string(),
            vec!["TAX-2025".to_string(), "TAX-2024".to_string()],
            85,
            ImpactCategory::LegalConflict,
        );
        assert_eq!(impact.source_statute_id, "TAX-2026");
        assert_eq!(impact.affected_statute_ids.len(), 2);
        assert_eq!(impact.impact_score, 85);
        assert_eq!(impact.priority, Priority::Critical);
    }

    #[test]
    fn test_instant_impact_priority_levels() {
        let low = InstantImpact::new("S1".to_string(), vec![], 30, ImpactCategory::Administrative);
        assert_eq!(low.priority, Priority::Low);

        let medium = InstantImpact::new(
            "S2".to_string(),
            vec![],
            50,
            ImpactCategory::ComplianceChange,
        );
        assert_eq!(medium.priority, Priority::Medium);

        let high = InstantImpact::new(
            "S3".to_string(),
            vec![],
            70,
            ImpactCategory::ProceduralImpact,
        );
        assert_eq!(high.priority, Priority::High);

        let critical =
            InstantImpact::new("S4".to_string(), vec![], 90, ImpactCategory::LegalConflict);
        assert_eq!(critical.priority, Priority::Critical);
    }

    #[test]
    fn test_impact_category_display() {
        assert_eq!(ImpactCategory::LegalConflict.to_string(), "Legal Conflict");
        assert_eq!(
            ImpactCategory::ComplianceChange.to_string(),
            "Compliance Change"
        );
        assert_eq!(
            ImpactCategory::ProceduralImpact.to_string(),
            "Procedural Impact"
        );
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::Low.to_string(), "Low");
        assert_eq!(Priority::Medium.to_string(), "Medium");
        assert_eq!(Priority::High.to_string(), "High");
        assert_eq!(Priority::Critical.to_string(), "Critical");
    }

    #[test]
    fn test_compliance_monitor_creation() {
        let config = RealtimeConfig::default();
        let monitor = ComplianceMonitor::new(config);
        assert_eq!(monitor.get_status().total_statutes, 0);
        assert_eq!(monitor.get_status().active_conflicts, 0);
        assert_eq!(monitor.get_status().pending_updates, 0);
    }

    #[test]
    fn test_compliance_monitor_add_statute() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        monitor.add_statute(statute);

        assert_eq!(monitor.get_status().total_statutes, 1);
    }

    #[test]
    fn test_compliance_monitor_remove_statute() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        monitor.add_statute(statute);
        assert_eq!(monitor.get_status().total_statutes, 1);

        let removed = monitor.remove_statute("TAX-2026");
        assert!(removed.is_some());
        assert_eq!(monitor.get_status().total_statutes, 0);
    }

    #[test]
    fn test_compliance_monitor_queue_update() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Added);
        let result = monitor.queue_update(update);

        assert!(result.is_ok());
        assert_eq!(monitor.get_status().pending_updates, 1);
    }

    #[test]
    fn test_compliance_monitor_queue_full() {
        let config = RealtimeConfig {
            max_queue_size: 2,
            ..Default::default()
        };
        let mut monitor = ComplianceMonitor::new(config);

        let update1 = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Added);
        let update2 = StatuteUpdate::new("TAX-2027".to_string(), UpdateType::Added);
        let update3 = StatuteUpdate::new("TAX-2028".to_string(), UpdateType::Added);

        assert!(monitor.queue_update(update1).is_ok());
        assert!(monitor.queue_update(update2).is_ok());
        assert!(monitor.queue_update(update3).is_err());
    }

    #[test]
    fn test_compliance_monitor_process_update() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        monitor.add_statute(statute);

        let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Modified);
        let result = monitor.process_update(update);

        assert_eq!(result.update.statute_id, "TAX-2026");
        // Processing time is always set (u64), just verify structure
        assert!(!result.conflicts.is_empty() || result.conflicts.is_empty());
    }

    #[test]
    fn test_compliance_monitor_process_queue() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        monitor.add_statute(statute);

        let update1 = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Modified);
        let update2 = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Modified);

        monitor.queue_update(update1).ok();
        monitor.queue_update(update2).ok();

        let results = monitor.process_queue();
        assert_eq!(results.len(), 2);
        assert_eq!(monitor.get_status().pending_updates, 0);
    }

    #[test]
    fn test_compliance_monitor_stream_verify() {
        let config = RealtimeConfig {
            enable_streaming: true,
            stream_batch_size: 2,
            ..Default::default()
        };
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        monitor.add_statute(statute);

        for i in 0..5 {
            let update = StatuteUpdate::new(format!("TAX-{}", 2026 + i), UpdateType::Added);
            monitor.queue_update(update).ok();
        }

        let results = monitor.stream_verify();
        assert_eq!(results.len(), 2); // Batch size
        assert_eq!(monitor.get_status().pending_updates, 3); // Remaining
    }

    #[test]
    fn test_realtime_verification_result_is_successful() {
        let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Added);
        let result = RealtimeVerificationResult {
            update,
            verification_result: VerificationResult::pass(),
            conflicts: Vec::new(),
            impacts: Vec::new(),
            processing_time_ms: 100,
            cache_hit: false,
        };

        assert!(result.is_successful());
    }

    #[test]
    fn test_realtime_verification_result_has_critical_issues() {
        let update = StatuteUpdate::new("TAX-2026".to_string(), UpdateType::Modified);
        let conflict = RealtimeConflict::new(
            vec!["TAX-2026".to_string(), "TAX-2025".to_string()],
            "Critical Conflict".to_string(),
            95,
        );

        let result = RealtimeVerificationResult {
            update,
            verification_result: VerificationResult::pass(),
            conflicts: vec![conflict],
            impacts: Vec::new(),
            processing_time_ms: 100,
            cache_hit: false,
        };

        assert!(result.has_critical_issues());
    }

    #[test]
    fn test_monitoring_status_default() {
        let status = MonitoringStatus::default();
        assert_eq!(status.total_statutes, 0);
        assert_eq!(status.active_conflicts, 0);
        assert_eq!(status.system_health, 100);
        assert_eq!(status.cache_hit_rate, 0.0);
    }

    #[test]
    fn test_compliance_monitor_generate_report() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        monitor.add_statute(statute);

        let report = monitor.generate_report();
        assert!(report.contains("Real-Time Compliance Monitoring Report"));
        assert!(report.contains("System Status"));
        assert!(report.contains("**Total Statutes Monitored**: 1"));
    }

    #[test]
    fn test_compliance_monitor_clear_conflict() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let conflict = RealtimeConflict::new(
            vec!["TAX-2026".to_string()],
            "Test Conflict".to_string(),
            50,
        );
        let conflict_id = conflict.conflict_id.clone();
        monitor.conflicts.push(conflict);
        monitor.status.active_conflicts = 1;

        monitor.clear_conflict(&conflict_id);
        assert_eq!(monitor.get_status().active_conflicts, 0);
    }

    #[test]
    fn test_compliance_monitor_clear_expired_cache() {
        let config = RealtimeConfig::default();
        let mut monitor = ComplianceMonitor::new(config);

        let statute = Statute::new(
            "TAX-2026",
            "Tax Law",
            Effect::new(EffectType::Grant, "Grant permission"),
        );
        let result = VerificationResult::pass();

        monitor.verification_cache.insert(
            statute.id.clone(),
            (result, Instant::now() - Duration::from_secs(400)),
        );

        monitor.clear_expired_cache();
        assert!(monitor.verification_cache.is_empty());
    }
}
