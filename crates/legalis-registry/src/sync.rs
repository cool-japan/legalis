use super::*;
use chrono::{Datelike, Timelike};

/// Synchronization schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncSchedule {
    /// Manual synchronization only
    Manual,
    /// Hourly synchronization
    Hourly,
    /// Daily synchronization at specified hour
    Daily { hour: u8 },
    /// Weekly synchronization on specified day and hour
    Weekly { day: u8, hour: u8 },
    /// Monthly synchronization on specified day and hour
    Monthly { day: u8, hour: u8 },
    /// Custom interval in seconds
    Interval { seconds: u64 },
}

impl SyncSchedule {
    /// Returns the next sync time from a given timestamp.
    pub fn next_sync(&self, from: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Self::Manual => None,
            Self::Hourly => Some(from + chrono::Duration::hours(1)),
            Self::Daily { hour } => {
                let next = from + chrono::Duration::days(1);
                Some(next.with_hour(*hour as u32).unwrap_or(next))
            }
            Self::Weekly { day: _, hour } => {
                let next = from + chrono::Duration::weeks(1);
                Some(next.with_hour(*hour as u32).unwrap_or(next))
            }
            Self::Monthly { day, hour } => {
                let next = from.with_day(*day as u32).unwrap_or(from) + chrono::Duration::days(30);
                Some(next.with_hour(*hour as u32).unwrap_or(next))
            }
            Self::Interval { seconds } => Some(from + chrono::Duration::seconds(*seconds as i64)),
        }
    }

    /// Checks if a sync is due from a given last sync time.
    pub fn is_due(&self, last_sync: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        match self.next_sync(last_sync) {
            Some(next) => now >= next,
            None => false,
        }
    }
}

/// Synchronization job configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJob {
    /// Job ID
    pub id: Uuid,
    /// Job name
    pub name: String,
    /// Import source
    pub source: government_import::ImportSource,
    /// Schedule
    pub schedule: SyncSchedule,
    /// Last sync timestamp
    pub last_sync: Option<DateTime<Utc>>,
    /// Last sync result
    pub last_result: Option<government_import::BulkImportResult>,
    /// Whether the job is enabled
    pub enabled: bool,
}

impl SyncJob {
    /// Creates a new sync job.
    pub fn new(
        name: impl Into<String>,
        source: government_import::ImportSource,
        schedule: SyncSchedule,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            source,
            schedule,
            last_sync: None,
            last_result: None,
            enabled: true,
        }
    }

    /// Checks if the job is due for execution.
    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        if !self.enabled {
            return false;
        }
        match self.last_sync {
            Some(last) => self.schedule.is_due(last, now),
            None => true, // Never synced, so it's due
        }
    }

    /// Marks the job as completed with a result.
    pub fn mark_completed(&mut self, result: government_import::BulkImportResult) {
        self.last_sync = Some(Utc::now());
        self.last_result = Some(result);
    }
}

/// Synchronization manager.
#[derive(Debug)]
pub struct SyncManager {
    jobs: Vec<SyncJob>,
}

impl SyncManager {
    /// Creates a new sync manager.
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    /// Adds a sync job.
    pub fn add_job(&mut self, job: SyncJob) {
        self.jobs.push(job);
    }

    /// Removes a sync job by ID.
    pub fn remove_job(&mut self, job_id: Uuid) -> bool {
        if let Some(pos) = self.jobs.iter().position(|j| j.id == job_id) {
            self.jobs.remove(pos);
            true
        } else {
            false
        }
    }

    /// Gets all jobs.
    pub fn jobs(&self) -> &[SyncJob] {
        &self.jobs
    }

    /// Gets all jobs that are due for execution.
    pub fn due_jobs(&self, now: DateTime<Utc>) -> Vec<&SyncJob> {
        self.jobs.iter().filter(|j| j.is_due(now)).collect()
    }

    /// Updates a job's result.
    pub fn update_job_result(&mut self, job_id: Uuid, result: government_import::BulkImportResult) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            job.mark_completed(result);
        }
    }

    /// Enables or disables a job.
    pub fn set_job_enabled(&mut self, job_id: Uuid, enabled: bool) -> bool {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            job.enabled = enabled;
            true
        } else {
            false
        }
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}
