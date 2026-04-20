//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::types_6::StatuteEntry;
use super::types_7::SearchQuery;

/// Task assignment for reviews.
pub mod tasks {
    use super::*;
    /// Task status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TaskStatus {
        /// Not yet started
        NotStarted,
        /// In progress
        InProgress,
        /// Blocked
        Blocked,
        /// Completed
        Completed,
        /// Cancelled
        Cancelled,
    }
    /// Review task.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReviewTask {
        /// Task ID
        pub task_id: Uuid,
        /// Task title
        pub title: String,
        /// Task description
        pub description: Option<String>,
        /// Assigned to user ID
        pub assigned_to: String,
        /// Assigned by user ID
        pub assigned_by: String,
        /// Related statute ID
        pub statute_id: String,
        /// Task status
        pub status: TaskStatus,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Started timestamp
        pub started_at: Option<DateTime<Utc>>,
        /// Completed timestamp
        pub completed_at: Option<DateTime<Utc>>,
        /// Due date
        pub due_date: Option<DateTime<Utc>>,
        /// Review notes
        pub notes: Vec<String>,
    }
    impl ReviewTask {
        /// Creates a new review task.
        pub fn new(
            title: impl Into<String>,
            assigned_to: impl Into<String>,
            assigned_by: impl Into<String>,
            statute_id: impl Into<String>,
        ) -> Self {
            Self {
                task_id: Uuid::new_v4(),
                title: title.into(),
                description: None,
                assigned_to: assigned_to.into(),
                assigned_by: assigned_by.into(),
                statute_id: statute_id.into(),
                status: TaskStatus::NotStarted,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                due_date: None,
                notes: Vec::new(),
            }
        }
        /// Sets description.
        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description = Some(description.into());
            self
        }
        /// Sets due date.
        pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
            self.due_date = Some(due_date);
            self
        }
        /// Starts the task.
        pub fn start(&mut self) {
            self.status = TaskStatus::InProgress;
            self.started_at = Some(Utc::now());
        }
        /// Completes the task.
        pub fn complete(&mut self) {
            self.status = TaskStatus::Completed;
            self.completed_at = Some(Utc::now());
        }
        /// Adds a note.
        pub fn add_note(&mut self, note: impl Into<String>) {
            self.notes.push(note.into());
        }
        /// Checks if overdue.
        pub fn is_overdue(&self) -> bool {
            if let Some(due) = self.due_date {
                Utc::now() > due && self.status != TaskStatus::Completed
            } else {
                false
            }
        }
    }
    /// Task manager.
    #[derive(Debug)]
    pub struct TaskManager {
        tasks: HashMap<Uuid, ReviewTask>,
    }
    impl TaskManager {
        /// Creates a new task manager.
        pub fn new() -> Self {
            Self {
                tasks: HashMap::new(),
            }
        }
        /// Creates a task.
        pub fn create_task(&mut self, task: ReviewTask) -> Uuid {
            let id = task.task_id;
            self.tasks.insert(id, task);
            id
        }
        /// Gets a task by ID.
        pub fn get_task(&self, task_id: Uuid) -> Option<&ReviewTask> {
            self.tasks.get(&task_id)
        }
        /// Gets a mutable task by ID.
        pub fn get_task_mut(&mut self, task_id: Uuid) -> Option<&mut ReviewTask> {
            self.tasks.get_mut(&task_id)
        }
        /// Gets tasks assigned to a user.
        pub fn tasks_for_user(&self, user_id: &str) -> Vec<&ReviewTask> {
            self.tasks
                .values()
                .filter(|t| t.assigned_to == user_id)
                .collect()
        }
        /// Gets overdue tasks.
        pub fn overdue_tasks(&self) -> Vec<&ReviewTask> {
            self.tasks.values().filter(|t| t.is_overdue()).collect()
        }
        /// Gets tasks by status.
        pub fn tasks_by_status(&self, status: TaskStatus) -> Vec<&ReviewTask> {
            self.tasks.values().filter(|t| t.status == status).collect()
        }
    }
    impl Default for TaskManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// SLA tracking for approvals.
pub mod sla {
    use super::*;
    /// SLA metric type.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SlaMetric {
        /// Time to first response
        TimeToFirstResponse,
        /// Time to approval
        TimeToApproval,
        /// Time to completion
        TimeToCompletion,
        /// Custom metric
        Custom(String),
    }
    /// SLA definition.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SlaDefinition {
        /// SLA ID
        pub sla_id: Uuid,
        /// SLA name
        pub name: String,
        /// Metric being tracked
        pub metric: SlaMetric,
        /// Target duration in seconds
        pub target_seconds: i64,
        /// Warning threshold (percentage of target)
        pub warning_threshold: f64,
    }
    impl SlaDefinition {
        /// Creates a new SLA definition.
        pub fn new(name: impl Into<String>, metric: SlaMetric, target_seconds: i64) -> Self {
            Self {
                sla_id: Uuid::new_v4(),
                name: name.into(),
                metric,
                target_seconds,
                warning_threshold: 0.8,
            }
        }
        /// Sets warning threshold.
        pub fn with_warning_threshold(mut self, threshold: f64) -> Self {
            self.warning_threshold = threshold.clamp(0.0, 1.0);
            self
        }
        /// Gets target duration.
        pub fn target_duration(&self) -> chrono::Duration {
            chrono::Duration::seconds(self.target_seconds)
        }
        /// Gets warning duration.
        pub fn warning_duration(&self) -> chrono::Duration {
            chrono::Duration::seconds((self.target_seconds as f64 * self.warning_threshold) as i64)
        }
    }
    /// SLA status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SlaStatus {
        /// Met the SLA
        Met,
        /// Warning - approaching SLA breach
        Warning,
        /// Breached the SLA
        Breached,
    }
    /// SLA measurement.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SlaMeasurement {
        /// Measurement ID
        pub measurement_id: Uuid,
        /// SLA definition ID
        pub sla_id: Uuid,
        /// Related entity ID
        pub entity_id: String,
        /// Start time
        pub start_time: DateTime<Utc>,
        /// End time
        pub end_time: Option<DateTime<Utc>>,
        /// Actual duration in seconds
        pub duration_seconds: Option<i64>,
        /// SLA status
        pub status: SlaStatus,
    }
    impl SlaMeasurement {
        /// Creates a new SLA measurement.
        pub fn new(sla_id: Uuid, entity_id: impl Into<String>) -> Self {
            Self {
                measurement_id: Uuid::new_v4(),
                sla_id,
                entity_id: entity_id.into(),
                start_time: Utc::now(),
                end_time: None,
                duration_seconds: None,
                status: SlaStatus::Met,
            }
        }
        /// Completes the measurement.
        pub fn complete(&mut self, sla: &SlaDefinition) {
            self.end_time = Some(Utc::now());
            let duration = self
                .end_time
                .expect("invariant: end_time was just set to Some")
                - self.start_time;
            self.duration_seconds = Some(duration.num_seconds());
            if duration > sla.target_duration() {
                self.status = SlaStatus::Breached;
            } else if duration > sla.warning_duration() {
                self.status = SlaStatus::Warning;
            } else {
                self.status = SlaStatus::Met;
            }
        }
        /// Checks current status against SLA.
        pub fn check_status(&mut self, sla: &SlaDefinition) -> SlaStatus {
            if self.end_time.is_some() {
                return self.status;
            }
            let elapsed = Utc::now() - self.start_time;
            if elapsed > sla.target_duration() {
                self.status = SlaStatus::Breached;
            } else if elapsed > sla.warning_duration() {
                self.status = SlaStatus::Warning;
            } else {
                self.status = SlaStatus::Met;
            }
            self.status
        }
    }
    /// SLA tracker.
    #[derive(Debug)]
    pub struct SlaTracker {
        definitions: HashMap<Uuid, SlaDefinition>,
        measurements: Vec<SlaMeasurement>,
    }
    impl SlaTracker {
        /// Creates a new SLA tracker.
        pub fn new() -> Self {
            Self {
                definitions: HashMap::new(),
                measurements: Vec::new(),
            }
        }
        /// Adds an SLA definition.
        pub fn add_definition(&mut self, definition: SlaDefinition) -> Uuid {
            let id = definition.sla_id;
            self.definitions.insert(id, definition);
            id
        }
        /// Starts tracking an SLA.
        pub fn start_tracking(&mut self, sla_id: Uuid, entity_id: impl Into<String>) -> Uuid {
            let measurement = SlaMeasurement::new(sla_id, entity_id);
            let id = measurement.measurement_id;
            self.measurements.push(measurement);
            id
        }
        /// Completes an SLA measurement.
        pub fn complete_measurement(&mut self, measurement_id: Uuid) -> Result<SlaStatus, String> {
            let measurement = self
                .measurements
                .iter_mut()
                .find(|m| m.measurement_id == measurement_id)
                .ok_or_else(|| "Measurement not found".to_string())?;
            let sla = self
                .definitions
                .get(&measurement.sla_id)
                .ok_or_else(|| "SLA definition not found".to_string())?;
            measurement.complete(sla);
            Ok(measurement.status)
        }
        /// Gets measurements in warning or breach status.
        pub fn at_risk_measurements(&mut self) -> Vec<&mut SlaMeasurement> {
            for m in &mut self.measurements {
                if let Some(sla) = self.definitions.get(&m.sla_id) {
                    m.check_status(sla);
                }
            }
            self.measurements
                .iter_mut()
                .filter(|m| m.status == SlaStatus::Warning || m.status == SlaStatus::Breached)
                .collect()
        }
        /// Gets completion rate for an SLA.
        pub fn completion_rate(&self, sla_id: Uuid) -> f64 {
            let total: Vec<_> = self
                .measurements
                .iter()
                .filter(|m| m.sla_id == sla_id && m.end_time.is_some())
                .collect();
            if total.is_empty() {
                return 1.0;
            }
            let met_count = total.iter().filter(|m| m.status == SlaStatus::Met).count();
            met_count as f64 / total.len() as f64
        }
    }
    impl Default for SlaTracker {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// Escalation rules.
pub mod escalation {
    use super::*;
    /// Escalation condition.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum EscalationCondition {
        /// Time-based: escalate after duration
        AfterDuration { seconds: i64 },
        /// Overdue task or approval
        Overdue,
        /// SLA breach
        SlaBreach,
        /// No response after duration
        NoResponseAfter { seconds: i64 },
        /// Multiple rejections
        MultipleRejections { count: usize },
    }
    impl EscalationCondition {
        /// Checks if condition is met for a timestamp.
        pub fn is_met(&self, created_at: DateTime<Utc>, _has_response: bool) -> bool {
            match self {
                Self::AfterDuration { seconds } => {
                    let elapsed = Utc::now() - created_at;
                    elapsed.num_seconds() >= *seconds
                }
                Self::Overdue => false,
                Self::SlaBreach => false,
                Self::NoResponseAfter { seconds } => {
                    if _has_response {
                        false
                    } else {
                        let elapsed = Utc::now() - created_at;
                        elapsed.num_seconds() >= *seconds
                    }
                }
                Self::MultipleRejections { count: _ } => false,
            }
        }
    }
    /// Escalation action.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum EscalationAction {
        /// Notify additional users
        Notify { users: Vec<String> },
        /// Reassign to different user
        Reassign { to_user: String },
        /// Escalate to manager
        EscalateToManager,
        /// Auto-approve
        AutoApprove,
        /// Custom action
        Custom(String),
    }
    /// Escalation rule.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EscalationRule {
        /// Rule ID
        pub rule_id: Uuid,
        /// Rule name
        pub name: String,
        /// Condition to trigger escalation
        pub condition: EscalationCondition,
        /// Action to take
        pub action: EscalationAction,
        /// Priority (higher = evaluated first)
        pub priority: i32,
        /// Whether the rule is enabled
        pub enabled: bool,
    }
    impl EscalationRule {
        /// Creates a new escalation rule.
        pub fn new(
            name: impl Into<String>,
            condition: EscalationCondition,
            action: EscalationAction,
        ) -> Self {
            Self {
                rule_id: Uuid::new_v4(),
                name: name.into(),
                condition,
                action,
                priority: 0,
                enabled: true,
            }
        }
        /// Sets priority.
        pub fn with_priority(mut self, priority: i32) -> Self {
            self.priority = priority;
            self
        }
        /// Checks if the rule should be triggered.
        pub fn should_trigger(&self, created_at: DateTime<Utc>, has_response: bool) -> bool {
            self.enabled && self.condition.is_met(created_at, has_response)
        }
    }
    /// Escalation event.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EscalationEvent {
        /// Event ID
        pub event_id: Uuid,
        /// Rule that triggered
        pub rule_id: Uuid,
        /// Entity that was escalated
        pub entity_id: String,
        /// Action taken
        pub action: EscalationAction,
        /// Timestamp
        pub escalated_at: DateTime<Utc>,
    }
    /// Escalation manager.
    #[derive(Debug)]
    pub struct EscalationManager {
        rules: Vec<EscalationRule>,
        events: Vec<EscalationEvent>,
    }
    impl EscalationManager {
        /// Creates a new escalation manager.
        pub fn new() -> Self {
            Self {
                rules: Vec::new(),
                events: Vec::new(),
            }
        }
        /// Adds an escalation rule.
        pub fn add_rule(&mut self, rule: EscalationRule) {
            self.rules.push(rule);
            self.rules.sort_by_key(|b| std::cmp::Reverse(b.priority));
        }
        /// Checks for escalations and applies rules.
        pub fn check_escalations(
            &mut self,
            entity_id: impl Into<String>,
            created_at: DateTime<Utc>,
            has_response: bool,
        ) -> Vec<EscalationAction> {
            let entity_id = entity_id.into();
            let mut actions = Vec::new();
            for rule in &self.rules {
                if rule.should_trigger(created_at, has_response) {
                    let event = EscalationEvent {
                        event_id: Uuid::new_v4(),
                        rule_id: rule.rule_id,
                        entity_id: entity_id.clone(),
                        action: rule.action.clone(),
                        escalated_at: Utc::now(),
                    };
                    actions.push(rule.action.clone());
                    self.events.push(event);
                }
            }
            actions
        }
        /// Gets escalation events for an entity.
        pub fn events_for_entity(&self, entity_id: &str) -> Vec<&EscalationEvent> {
            self.events
                .iter()
                .filter(|e| e.entity_id == entity_id)
                .collect()
        }
        /// Gets all rules.
        pub fn rules(&self) -> &[EscalationRule] {
            &self.rules
        }
    }
    impl Default for EscalationManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// Advanced search features.
pub mod advanced_search {
    use super::*;
    /// Facet type for search aggregations.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum FacetType {
        /// Status facet
        Status,
        /// Jurisdiction facet
        Jurisdiction,
        /// Tags facet
        Tags,
        /// Year (from effective date)
        Year,
        /// Month (from effective date)
        Month,
        /// Custom facet
        Custom(String),
    }
    /// Facet value with count.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FacetValue {
        /// Value of the facet
        pub value: String,
        /// Count of items with this value
        pub count: usize,
    }
    /// Facet result for a specific facet type.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FacetResult {
        /// Facet type
        pub facet_type: FacetType,
        /// Values with their counts
        pub values: Vec<FacetValue>,
        /// Total number of unique values
        pub total_values: usize,
    }
    impl FacetResult {
        /// Gets top N values by count.
        pub fn top_values(&self, n: usize) -> Vec<&FacetValue> {
            let mut sorted: Vec<_> = self.values.iter().collect();
            sorted.sort_by_key(|b| std::cmp::Reverse(b.count));
            sorted.into_iter().take(n).collect()
        }
        /// Finds a specific value.
        pub fn find_value(&self, value: &str) -> Option<&FacetValue> {
            self.values.iter().find(|v| v.value == value)
        }
    }
    /// Faceted search results.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FacetedSearchResult {
        /// Matching statute IDs
        pub statute_ids: Vec<String>,
        /// Facet results
        pub facets: Vec<FacetResult>,
        /// Total matches
        pub total_matches: usize,
    }
    /// Search suggestion.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchSuggestion {
        /// Suggested text
        pub text: String,
        /// Suggestion type
        pub suggestion_type: SuggestionType,
        /// Relevance score
        pub score: f64,
    }
    /// Type of search suggestion.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SuggestionType {
        /// Statute ID
        StatuteId,
        /// Statute title
        Title,
        /// Tag
        Tag,
        /// Jurisdiction
        Jurisdiction,
        /// General term
        Term,
    }
    /// Autocomplete provider.
    #[derive(Debug)]
    pub struct AutocompleteProvider {
        /// Index of statute IDs
        statute_ids: Vec<String>,
        /// Index of titles
        titles: Vec<String>,
        /// Index of tags
        tags: Vec<String>,
        /// Index of jurisdictions
        jurisdictions: Vec<String>,
    }
    impl AutocompleteProvider {
        /// Creates a new autocomplete provider.
        pub fn new() -> Self {
            Self {
                statute_ids: Vec::new(),
                titles: Vec::new(),
                tags: Vec::new(),
                jurisdictions: Vec::new(),
            }
        }
        /// Indexes a statute for autocomplete.
        pub fn index_statute(&mut self, entry: &StatuteEntry) {
            if !self.statute_ids.contains(&entry.statute.id) {
                self.statute_ids.push(entry.statute.id.clone());
            }
            let title = entry.statute.title.clone();
            if !self.titles.contains(&title) {
                self.titles.push(title);
            }
            for tag in &entry.tags {
                if !self.tags.contains(tag) {
                    self.tags.push(tag.clone());
                }
            }
            if !self.jurisdictions.contains(&entry.jurisdiction) {
                self.jurisdictions.push(entry.jurisdiction.clone());
            }
        }
        /// Gets suggestions for a query.
        pub fn suggest(&self, query: &str, max_results: usize) -> Vec<SearchSuggestion> {
            let query_lower = query.to_lowercase();
            let mut suggestions = Vec::new();
            for id in &self.statute_ids {
                if id.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: id.clone(),
                        suggestion_type: SuggestionType::StatuteId,
                        score: Self::calculate_score(&query_lower, &id.to_lowercase()),
                    });
                }
            }
            for title in &self.titles {
                if title.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: title.clone(),
                        suggestion_type: SuggestionType::Title,
                        score: Self::calculate_score(&query_lower, &title.to_lowercase()),
                    });
                }
            }
            for tag in &self.tags {
                if tag.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: tag.clone(),
                        suggestion_type: SuggestionType::Tag,
                        score: Self::calculate_score(&query_lower, &tag.to_lowercase()),
                    });
                }
            }
            for jurisdiction in &self.jurisdictions {
                if jurisdiction.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: jurisdiction.clone(),
                        suggestion_type: SuggestionType::Jurisdiction,
                        score: Self::calculate_score(&query_lower, &jurisdiction.to_lowercase()),
                    });
                }
            }
            suggestions.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            suggestions.truncate(max_results);
            suggestions
        }
        /// Calculates relevance score.
        fn calculate_score(query: &str, text: &str) -> f64 {
            if query == text {
                return 1.0;
            }
            if text.starts_with(query) {
                return 0.9;
            }
            if text.contains(query) {
                return 0.7;
            }
            0.5
        }
    }
    impl Default for AutocompleteProvider {
        fn default() -> Self {
            Self::new()
        }
    }
    /// Saved search.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SavedSearch {
        /// Search ID
        pub search_id: Uuid,
        /// Search name
        pub name: String,
        /// Search query
        pub query: SearchQuery,
        /// Owner user ID
        pub owner: String,
        /// Alert enabled
        pub alert_enabled: bool,
        /// Alert frequency in seconds
        pub alert_frequency_seconds: Option<i64>,
        /// Last executed
        pub last_executed: Option<DateTime<Utc>>,
        /// Last result count
        pub last_result_count: Option<usize>,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
    }
    impl SavedSearch {
        /// Creates a new saved search.
        pub fn new(name: impl Into<String>, query: SearchQuery, owner: impl Into<String>) -> Self {
            Self {
                search_id: Uuid::new_v4(),
                name: name.into(),
                query,
                owner: owner.into(),
                alert_enabled: false,
                alert_frequency_seconds: None,
                last_executed: None,
                last_result_count: None,
                created_at: Utc::now(),
            }
        }
        /// Enables alerts with frequency.
        pub fn with_alert(mut self, frequency_seconds: i64) -> Self {
            self.alert_enabled = true;
            self.alert_frequency_seconds = Some(frequency_seconds);
            self
        }
        /// Checks if alert should be triggered.
        pub fn should_trigger_alert(&self) -> bool {
            if !self.alert_enabled {
                return false;
            }
            if let Some(freq) = self.alert_frequency_seconds {
                if let Some(last_exec) = self.last_executed {
                    let elapsed = Utc::now() - last_exec;
                    return elapsed.num_seconds() >= freq;
                }
                return true;
            }
            false
        }
        /// Updates execution info.
        pub fn update_execution(&mut self, result_count: usize) {
            self.last_executed = Some(Utc::now());
            self.last_result_count = Some(result_count);
        }
    }
    /// Search analytics tracker.
    #[derive(Debug)]
    pub struct SearchAnalytics {
        /// Query frequency tracking
        query_counts: HashMap<String, usize>,
        /// Recent searches
        recent_searches: Vec<(String, DateTime<Utc>)>,
        /// Search result counts
        result_counts: Vec<usize>,
        /// Max recent searches to track
        max_recent: usize,
    }
    impl SearchAnalytics {
        /// Creates a new search analytics tracker.
        pub fn new() -> Self {
            Self {
                query_counts: HashMap::new(),
                recent_searches: Vec::new(),
                result_counts: Vec::new(),
                max_recent: 1000,
            }
        }
        /// Records a search.
        pub fn record_search(&mut self, query: &str, result_count: usize) {
            *self.query_counts.entry(query.to_string()).or_insert(0) += 1;
            self.recent_searches.push((query.to_string(), Utc::now()));
            if self.recent_searches.len() > self.max_recent {
                self.recent_searches
                    .drain(0..self.recent_searches.len() - self.max_recent);
            }
            self.result_counts.push(result_count);
        }
        /// Gets most popular queries.
        pub fn top_queries(&self, n: usize) -> Vec<(String, usize)> {
            let mut queries: Vec<_> = self
                .query_counts
                .iter()
                .map(|(q, c)| (q.clone(), *c))
                .collect();
            queries.sort_by_key(|b| std::cmp::Reverse(b.1));
            queries.into_iter().take(n).collect()
        }
        /// Gets average result count.
        pub fn average_result_count(&self) -> f64 {
            if self.result_counts.is_empty() {
                return 0.0;
            }
            let sum: usize = self.result_counts.iter().sum();
            sum as f64 / self.result_counts.len() as f64
        }
        /// Gets zero-result queries.
        pub fn zero_result_queries(&self) -> Vec<String> {
            self.recent_searches
                .iter()
                .enumerate()
                .filter(|(i, _)| self.result_counts.get(*i).map(|&c| c == 0).unwrap_or(false))
                .map(|(_, (q, _))| q.clone())
                .collect()
        }
        /// Gets total searches.
        pub fn total_searches(&self) -> usize {
            self.recent_searches.len()
        }
        /// Gets searches in time range.
        pub fn searches_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> usize {
            self.recent_searches
                .iter()
                .filter(|(_, ts)| ts >= &start && ts <= &end)
                .count()
        }
    }
    impl Default for SearchAnalytics {
        fn default() -> Self {
            Self::new()
        }
    }
    /// Semantic search using embeddings (placeholder for future ML integration).
    #[derive(Debug)]
    pub struct SemanticSearch {
        /// Enabled flag
        enabled: bool,
        /// Embedding dimension
        dimension: usize,
    }
    impl SemanticSearch {
        /// Creates a new semantic search engine.
        pub fn new(dimension: usize) -> Self {
            Self {
                enabled: false,
                dimension,
            }
        }
        /// Enables semantic search.
        pub fn enable(&mut self) {
            self.enabled = true;
        }
        /// Checks if enabled.
        pub fn is_enabled(&self) -> bool {
            self.enabled
        }
        /// Gets embedding dimension.
        pub fn dimension(&self) -> usize {
            self.dimension
        }
        /// Placeholder for semantic search (would integrate with ML models).
        pub fn search(&self, _query: &str, _top_k: usize) -> Vec<(String, f64)> {
            Vec::new()
        }
    }
    impl Default for SemanticSearch {
        fn default() -> Self {
            Self::new(384)
        }
    }
}
