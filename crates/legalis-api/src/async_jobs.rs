//! Async job management for long-running operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Status of an async job.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Async job information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job<T> {
    pub id: String,
    pub status: JobStatus,
    pub progress: f32,
    pub result: Option<T>,
    pub error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl<T> Job<T> {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            status: JobStatus::Pending,
            progress: 0.0,
            result: None,
            error: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_running(&mut self) {
        self.status = JobStatus::Running;
        self.updated_at = chrono::Utc::now();
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 100.0);
        self.updated_at = chrono::Utc::now();
    }

    pub fn complete(&mut self, result: T) {
        self.status = JobStatus::Completed;
        self.progress = 100.0;
        self.result = Some(result);
        self.updated_at = chrono::Utc::now();
    }

    pub fn fail(&mut self, error: String) {
        self.status = JobStatus::Failed;
        self.error = Some(error);
        self.updated_at = chrono::Utc::now();
    }
}

impl<T> Default for Job<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Job manager for tracking async jobs.
pub struct JobManager<T> {
    jobs: Arc<RwLock<HashMap<String, Job<T>>>>,
}

impl<T: Clone> JobManager<T> {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new job and return its ID.
    pub async fn create_job(&self) -> String {
        let job = Job::new();
        let id = job.id.clone();
        let mut jobs = self.jobs.write().await;
        jobs.insert(id.clone(), job);
        id
    }

    /// Get job by ID.
    pub async fn get_job(&self, id: &str) -> Option<Job<T>> {
        let jobs = self.jobs.read().await;
        jobs.get(id).cloned()
    }

    /// Update job status.
    pub async fn update_job<F>(&self, id: &str, updater: F) -> bool
    where
        F: FnOnce(&mut Job<T>),
    {
        let mut jobs = self.jobs.write().await;
        if let Some(job) = jobs.get_mut(id) {
            updater(job);
            true
        } else {
            false
        }
    }

    /// Remove old jobs (older than specified duration).
    pub async fn cleanup_old_jobs(&self, max_age: chrono::Duration) {
        let mut jobs = self.jobs.write().await;
        let now = chrono::Utc::now();
        jobs.retain(|_, job| now.signed_duration_since(job.created_at) < max_age);
    }
}

impl<T: Clone> Default for JobManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for JobManager<T> {
    fn clone(&self) -> Self {
        Self {
            jobs: Arc::clone(&self.jobs),
        }
    }
}
