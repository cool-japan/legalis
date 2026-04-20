use super::*;
use std::collections::BTreeMap;

// ========================================================================
// 1. Event Replay with Time-Travel Queries
// ========================================================================

/// Time-travel query builder for replaying events.
#[derive(Debug, Clone)]
pub struct TimeTravelQuery {
    /// Target point in time
    pub target_time: DateTime<Utc>,
    /// Optional statute filter
    pub statute_filter: Option<String>,
    /// Include only specific event types
    pub event_types: Vec<String>,
}

impl TimeTravelQuery {
    /// Creates a new time-travel query for a specific point in time.
    pub fn new(target_time: DateTime<Utc>) -> Self {
        Self {
            target_time,
            statute_filter: None,
            event_types: Vec::new(),
        }
    }

    /// Filters for a specific statute.
    pub fn for_statute(mut self, statute_id: String) -> Self {
        self.statute_filter = Some(statute_id);
        self
    }

    /// Filters for specific event types.
    pub fn with_event_types(mut self, event_types: Vec<String>) -> Self {
        self.event_types = event_types;
        self
    }
}

/// Result of replaying events to reconstruct state at a point in time.
#[derive(Debug, Clone)]
pub struct ReplayResult {
    /// Reconstructed statute state
    pub statutes: HashMap<String, StatuteEntry>,
    /// Number of events replayed
    pub events_replayed: usize,
    /// Target timestamp
    pub target_time: DateTime<Utc>,
    /// Replay duration
    pub replay_duration: std::time::Duration,
}

/// Event replay engine for time-travel queries.
#[derive(Debug)]
pub struct EventReplayEngine {
    event_store: Arc<Mutex<EventStore>>,
}

impl EventReplayEngine {
    /// Creates a new event replay engine.
    pub fn new(event_store: Arc<Mutex<EventStore>>) -> Self {
        Self { event_store }
    }

    /// Replays events up to a specific point in time.
    pub fn replay(&self, query: TimeTravelQuery) -> Result<ReplayResult, String> {
        let start = std::time::Instant::now();
        let store = self.event_store.lock().expect("event_store mutex poisoned");

        let mut statutes = HashMap::new();
        let mut events_replayed = 0;

        for event in store.all_events() {
            let event_time = self.get_event_timestamp(event);
            if event_time > query.target_time {
                break;
            }

            // Apply statute filter
            if let Some(ref statute_id) = query.statute_filter
                && !self.event_matches_statute(event, statute_id)
            {
                continue;
            }

            // Apply event
            self.apply_event(&mut statutes, event);
            events_replayed += 1;
        }

        Ok(ReplayResult {
            statutes,
            events_replayed,
            target_time: query.target_time,
            replay_duration: start.elapsed(),
        })
    }

    fn get_event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
        match event {
            RegistryEvent::StatuteRegistered { timestamp, .. }
            | RegistryEvent::StatuteUpdated { timestamp, .. }
            | RegistryEvent::StatusChanged { timestamp, .. }
            | RegistryEvent::TagAdded { timestamp, .. }
            | RegistryEvent::TagRemoved { timestamp, .. }
            | RegistryEvent::ReferenceAdded { timestamp, .. }
            | RegistryEvent::ReferenceRemoved { timestamp, .. }
            | RegistryEvent::MetadataUpdated { timestamp, .. }
            | RegistryEvent::StatuteDeleted { timestamp, .. }
            | RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
        }
    }

    fn event_matches_statute(&self, event: &RegistryEvent, statute_id: &str) -> bool {
        match event {
            RegistryEvent::StatuteRegistered { statute_id: id, .. }
            | RegistryEvent::StatuteUpdated { statute_id: id, .. }
            | RegistryEvent::StatusChanged { statute_id: id, .. }
            | RegistryEvent::TagAdded { statute_id: id, .. }
            | RegistryEvent::TagRemoved { statute_id: id, .. }
            | RegistryEvent::ReferenceAdded { statute_id: id, .. }
            | RegistryEvent::ReferenceRemoved { statute_id: id, .. }
            | RegistryEvent::MetadataUpdated { statute_id: id, .. }
            | RegistryEvent::StatuteDeleted { statute_id: id, .. }
            | RegistryEvent::StatuteArchived { statute_id: id, .. } => id == statute_id,
        }
    }

    fn apply_event(&self, statutes: &mut HashMap<String, StatuteEntry>, event: &RegistryEvent) {
        match event {
            RegistryEvent::StatuteRegistered {
                statute_id,
                jurisdiction,
                ..
            } => {
                let statute = legalis_core::Statute {
                    id: statute_id.clone(),
                    title: format!("Statute {}", statute_id),
                    preconditions: Vec::new(),
                    effect: legalis_core::Effect {
                        effect_type: legalis_core::EffectType::Obligation,
                        description: "Default effect".to_string(),
                        parameters: HashMap::new(),
                    },
                    discretion_logic: None,
                    temporal_validity: legalis_core::TemporalValidity {
                        effective_date: None,
                        expiry_date: None,
                        enacted_at: None,
                        amended_at: None,
                    },
                    version: 1,
                    jurisdiction: Some(jurisdiction.clone()),
                    derives_from: Vec::new(),
                    applies_to: Vec::new(),
                    exceptions: Vec::new(),
                };
                statutes.insert(statute_id.clone(), StatuteEntry::new(statute, jurisdiction));
            }
            RegistryEvent::StatusChanged {
                statute_id,
                new_status,
                ..
            } => {
                if let Some(entry) = statutes.get_mut(statute_id) {
                    entry.status = *new_status;
                }
            }
            RegistryEvent::StatuteDeleted { statute_id, .. } => {
                statutes.remove(statute_id);
            }
            _ => {}
        }
    }
}

// ========================================================================
// 2. Event Projections for Analytics
// ========================================================================

/// Projection types for analytics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionType {
    /// Count events by type
    EventTypeCount,
    /// Count events by statute
    StatuteActivityCount,
    /// Track status changes over time
    StatusChangeTimeline,
    /// Tag usage statistics
    TagUsageStats,
    /// Daily activity summary
    DailyActivitySummary,
}

/// Event projection result.
#[derive(Debug, Clone)]
pub struct ProjectionResult {
    /// Projection type
    pub projection_type: ProjectionType,
    /// Aggregated data
    pub data: BTreeMap<String, usize>,
    /// Number of events processed
    pub events_processed: usize,
    /// Time range covered
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
}

/// Event projection engine for analytics.
#[derive(Debug)]
pub struct ProjectionEngine {
    event_store: Arc<Mutex<EventStore>>,
}

impl ProjectionEngine {
    /// Creates a new projection engine.
    pub fn new(event_store: Arc<Mutex<EventStore>>) -> Self {
        Self { event_store }
    }

    /// Computes a projection from the event stream.
    pub fn project(&self, projection_type: ProjectionType) -> ProjectionResult {
        let store = self.event_store.lock().expect("event_store mutex poisoned");
        let events = store.all_events();

        let mut data = BTreeMap::new();
        let mut min_time = Utc::now();
        let mut max_time = DateTime::<Utc>::MIN_UTC;

        for event in &events {
            let timestamp = self.get_event_timestamp(event);
            if timestamp < min_time {
                min_time = timestamp;
            }
            if timestamp > max_time {
                max_time = timestamp;
            }

            match projection_type {
                ProjectionType::EventTypeCount => {
                    let event_type = self.get_event_type_name(event);
                    *data.entry(event_type).or_insert(0) += 1;
                }
                ProjectionType::StatuteActivityCount => {
                    if let Some(statute_id) = self.get_statute_id(event) {
                        *data.entry(statute_id).or_insert(0) += 1;
                    }
                }
                ProjectionType::StatusChangeTimeline => {
                    if let RegistryEvent::StatusChanged { new_status, .. } = event {
                        let status_str = format!("{:?}", new_status);
                        *data.entry(status_str).or_insert(0) += 1;
                    }
                }
                ProjectionType::TagUsageStats => {
                    if let RegistryEvent::TagAdded { tag, .. } = event {
                        *data.entry(tag.clone()).or_insert(0) += 1;
                    }
                }
                ProjectionType::DailyActivitySummary => {
                    let date_key = timestamp.format("%Y-%m-%d").to_string();
                    *data.entry(date_key).or_insert(0) += 1;
                }
            }
        }

        ProjectionResult {
            projection_type,
            data,
            events_processed: events.len(),
            time_range: (min_time, max_time),
        }
    }

    fn get_event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
        match event {
            RegistryEvent::StatuteRegistered { timestamp, .. }
            | RegistryEvent::StatuteUpdated { timestamp, .. }
            | RegistryEvent::StatusChanged { timestamp, .. }
            | RegistryEvent::TagAdded { timestamp, .. }
            | RegistryEvent::TagRemoved { timestamp, .. }
            | RegistryEvent::ReferenceAdded { timestamp, .. }
            | RegistryEvent::ReferenceRemoved { timestamp, .. }
            | RegistryEvent::MetadataUpdated { timestamp, .. }
            | RegistryEvent::StatuteDeleted { timestamp, .. }
            | RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
        }
    }

    fn get_event_type_name(&self, event: &RegistryEvent) -> String {
        match event {
            RegistryEvent::StatuteRegistered { .. } => "StatuteRegistered".to_string(),
            RegistryEvent::StatuteUpdated { .. } => "StatuteUpdated".to_string(),
            RegistryEvent::StatusChanged { .. } => "StatusChanged".to_string(),
            RegistryEvent::TagAdded { .. } => "TagAdded".to_string(),
            RegistryEvent::TagRemoved { .. } => "TagRemoved".to_string(),
            RegistryEvent::ReferenceAdded { .. } => "ReferenceAdded".to_string(),
            RegistryEvent::ReferenceRemoved { .. } => "ReferenceRemoved".to_string(),
            RegistryEvent::MetadataUpdated { .. } => "MetadataUpdated".to_string(),
            RegistryEvent::StatuteDeleted { .. } => "StatuteDeleted".to_string(),
            RegistryEvent::StatuteArchived { .. } => "StatuteArchived".to_string(),
        }
    }

    fn get_statute_id(&self, event: &RegistryEvent) -> Option<String> {
        match event {
            RegistryEvent::StatuteRegistered { statute_id, .. }
            | RegistryEvent::StatuteUpdated { statute_id, .. }
            | RegistryEvent::StatusChanged { statute_id, .. }
            | RegistryEvent::TagAdded { statute_id, .. }
            | RegistryEvent::TagRemoved { statute_id, .. }
            | RegistryEvent::ReferenceAdded { statute_id, .. }
            | RegistryEvent::ReferenceRemoved { statute_id, .. }
            | RegistryEvent::MetadataUpdated { statute_id, .. }
            | RegistryEvent::StatuteDeleted { statute_id, .. }
            | RegistryEvent::StatuteArchived { statute_id, .. } => Some(statute_id.clone()),
        }
    }
}

// ========================================================================
// 3. Event-Driven Notifications
// ========================================================================

/// Notification channel for event-driven updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationChannel {
    /// Email notification
    Email(String),
    /// Webhook URL
    Webhook(String),
    /// SMS notification
    Sms(String),
    /// In-app notification
    InApp(String),
}

/// Notification rule configuration.
#[derive(Debug, Clone)]
pub struct NotificationRule {
    /// Rule ID
    pub id: Uuid,
    /// Rule name
    pub name: String,
    /// Event filter pattern
    pub event_pattern: String,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
    /// Enabled flag
    pub enabled: bool,
}

impl NotificationRule {
    /// Creates a new notification rule.
    pub fn new(name: String, event_pattern: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            event_pattern,
            channels: Vec::new(),
            enabled: true,
        }
    }

    /// Adds a notification channel.
    pub fn add_channel(mut self, channel: NotificationChannel) -> Self {
        self.channels.push(channel);
        self
    }
}

/// Event notification manager.
#[derive(Debug)]
pub struct NotificationManager {
    rules: Arc<Mutex<Vec<NotificationRule>>>,
}

impl NotificationManager {
    /// Creates a new notification manager.
    pub fn new() -> Self {
        Self {
            rules: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a notification rule.
    pub fn add_rule(&self, rule: NotificationRule) {
        let mut rules = self.rules.lock().expect("rules mutex poisoned");
        rules.push(rule);
    }

    /// Removes a notification rule.
    pub fn remove_rule(&self, rule_id: Uuid) -> bool {
        let mut rules = self.rules.lock().expect("rules mutex poisoned");
        if let Some(pos) = rules.iter().position(|r| r.id == rule_id) {
            rules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Processes an event and sends notifications.
    pub fn process_event(&self, event: &RegistryEvent) -> usize {
        let rules = self.rules.lock().expect("rules mutex poisoned");
        let mut notifications_sent = 0;

        for rule in rules.iter() {
            if !rule.enabled {
                continue;
            }

            if self.event_matches_pattern(event, &rule.event_pattern) {
                for channel in &rule.channels {
                    self.send_notification(channel, event);
                    notifications_sent += 1;
                }
            }
        }

        notifications_sent
    }

    fn event_matches_pattern(&self, event: &RegistryEvent, pattern: &str) -> bool {
        let event_type = match event {
            RegistryEvent::StatuteRegistered { .. } => "StatuteRegistered",
            RegistryEvent::StatuteUpdated { .. } => "StatuteUpdated",
            RegistryEvent::StatusChanged { .. } => "StatusChanged",
            RegistryEvent::TagAdded { .. } => "TagAdded",
            RegistryEvent::TagRemoved { .. } => "TagRemoved",
            RegistryEvent::ReferenceAdded { .. } => "ReferenceAdded",
            RegistryEvent::ReferenceRemoved { .. } => "ReferenceRemoved",
            RegistryEvent::MetadataUpdated { .. } => "MetadataUpdated",
            RegistryEvent::StatuteDeleted { .. } => "StatuteDeleted",
            RegistryEvent::StatuteArchived { .. } => "StatuteArchived",
        };

        pattern == "*" || event_type.contains(pattern)
    }

    fn send_notification(&self, _channel: &NotificationChannel, _event: &RegistryEvent) {
        // Actual notification sending would be implemented here
        // For now, this is a placeholder
    }

    /// Lists all notification rules.
    pub fn list_rules(&self) -> Vec<NotificationRule> {
        self.rules.lock().expect("rules mutex poisoned").clone()
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// 4. Event Archiving with Cold Storage
// ========================================================================

/// Cold storage configuration.
#[derive(Debug, Clone)]
pub struct ColdStorageConfig {
    /// Archive events older than this duration
    pub archive_after: chrono::Duration,
    /// Compression enabled
    pub compression: bool,
    /// Archive path
    pub archive_path: String,
}

impl Default for ColdStorageConfig {
    fn default() -> Self {
        Self {
            archive_after: chrono::Duration::days(90),
            compression: true,
            archive_path: std::env::temp_dir()
                .join("legalis-coldstore")
                .to_string_lossy()
                .into_owned(),
        }
    }
}

/// Archived event batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedEventBatch {
    /// Archive ID
    pub id: Uuid,
    /// Events in this batch
    pub events: Vec<RegistryEvent>,
    /// Archive timestamp
    pub archived_at: DateTime<Utc>,
    /// Compressed flag
    pub compressed: bool,
}

/// Event archiver for cold storage management.
#[derive(Debug)]
pub struct EventArchiver {
    config: ColdStorageConfig,
    archived_batches: Arc<Mutex<Vec<ArchivedEventBatch>>>,
}

impl EventArchiver {
    /// Creates a new event archiver.
    pub fn new(config: ColdStorageConfig) -> Self {
        Self {
            config,
            archived_batches: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Archives old events from the event store.
    pub fn archive_old_events(&self, event_store: &mut EventStore) -> Result<usize, String> {
        let cutoff_time = Utc::now() - self.config.archive_after;
        let all_events = event_store.all_events();

        let (to_archive, to_keep): (Vec<_>, Vec<_>) =
            all_events.into_iter().cloned().partition(|event| {
                let timestamp = self.get_event_timestamp(event);
                timestamp < cutoff_time
            });

        if to_archive.is_empty() {
            return Ok(0);
        }

        let batch = ArchivedEventBatch {
            id: Uuid::new_v4(),
            events: to_archive.clone(),
            archived_at: Utc::now(),
            compressed: self.config.compression,
        };

        let archived_count = batch.events.len();

        // Store the archived batch
        let mut batches = self
            .archived_batches
            .lock()
            .expect("archived_batches mutex poisoned");
        batches.push(batch);

        // Clear and repopulate event store with non-archived events
        event_store.clear();
        for event in to_keep {
            event_store.record(event);
        }

        Ok(archived_count)
    }

    /// Retrieves archived events.
    pub fn get_archived_events(&self) -> Vec<ArchivedEventBatch> {
        self.archived_batches
            .lock()
            .expect("archived_batches mutex poisoned")
            .clone()
    }

    /// Restores events from an archived batch.
    pub fn restore_batch(
        &self,
        batch_id: Uuid,
        event_store: &mut EventStore,
    ) -> Result<usize, String> {
        let batches = self
            .archived_batches
            .lock()
            .expect("archived_batches mutex poisoned");

        if let Some(batch) = batches.iter().find(|b| b.id == batch_id) {
            let count = batch.events.len();
            for event in &batch.events {
                event_store.record(event.clone());
            }
            Ok(count)
        } else {
            Err("Batch not found".to_string())
        }
    }

    fn get_event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
        match event {
            RegistryEvent::StatuteRegistered { timestamp, .. }
            | RegistryEvent::StatuteUpdated { timestamp, .. }
            | RegistryEvent::StatusChanged { timestamp, .. }
            | RegistryEvent::TagAdded { timestamp, .. }
            | RegistryEvent::TagRemoved { timestamp, .. }
            | RegistryEvent::ReferenceAdded { timestamp, .. }
            | RegistryEvent::ReferenceRemoved { timestamp, .. }
            | RegistryEvent::MetadataUpdated { timestamp, .. }
            | RegistryEvent::StatuteDeleted { timestamp, .. }
            | RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
        }
    }
}

// ========================================================================
// 5. Event Schema Evolution Support
// ========================================================================

/// Event schema version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SchemaVersion {
    /// Creates a new schema version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the current schema version.
    pub fn current() -> Self {
        Self::new(1, 0, 0)
    }
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Event envelope with schema versioning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedEvent {
    /// Schema version
    pub schema_version: SchemaVersion,
    /// Event ID
    pub event_id: Uuid,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Event data
    pub event: RegistryEvent,
    /// Migration history
    pub migration_history: Vec<SchemaVersion>,
}

impl VersionedEvent {
    /// Creates a new versioned event.
    pub fn new(event: RegistryEvent) -> Self {
        Self {
            schema_version: SchemaVersion::current(),
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event,
            migration_history: Vec::new(),
        }
    }
}

/// Schema migration handler.
#[allow(clippy::wrong_self_convention)]
pub trait SchemaMigration: Send + Sync {
    /// Source schema version.
    fn from_version(&self) -> SchemaVersion;

    /// Target schema version.
    fn to_version(&self) -> SchemaVersion;

    /// Migrates an event to the new schema.
    fn migrate(&self, event: RegistryEvent) -> Result<RegistryEvent, String>;
}

/// Schema evolution manager.
pub struct SchemaEvolutionManager {
    current_version: SchemaVersion,
    migrations: Vec<Box<dyn SchemaMigration>>,
}

impl SchemaEvolutionManager {
    /// Creates a new schema evolution manager.
    pub fn new() -> Self {
        Self {
            current_version: SchemaVersion::current(),
            migrations: Vec::new(),
        }
    }

    /// Registers a schema migration.
    pub fn register_migration(&mut self, migration: Box<dyn SchemaMigration>) {
        self.migrations.push(migration);
    }

    /// Migrates an event to the current schema version.
    pub fn migrate_event(&self, mut versioned: VersionedEvent) -> Result<VersionedEvent, String> {
        while versioned.schema_version < self.current_version {
            let migration = self.find_migration(versioned.schema_version)?;
            versioned.event = migration.migrate(versioned.event)?;
            versioned.migration_history.push(versioned.schema_version);
            versioned.schema_version = migration.to_version();
        }
        Ok(versioned)
    }

    fn find_migration(&self, from_version: SchemaVersion) -> Result<&dyn SchemaMigration, String> {
        self.migrations
            .iter()
            .find(|m| m.from_version() == from_version)
            .map(|b| b.as_ref())
            .ok_or_else(|| format!("No migration found from version {}", from_version))
    }

    /// Gets the current schema version.
    pub fn current_version(&self) -> SchemaVersion {
        self.current_version
    }
}

impl Default for SchemaEvolutionManager {
    fn default() -> Self {
        Self::new()
    }
}
