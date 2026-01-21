//! File-based persistence for simulation checkpoints and resume from failure.
//!
//! This module provides:
//! - File-based checkpoint persistence (save/load to disk)
//! - Resume from failure detection and recovery
//! - Automatic periodic checkpointing
//! - Checkpoint validation and integrity
//! - Compressed checkpoint storage

use crate::{Checkpoint, SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Configuration for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Directory for storing checkpoints
    pub checkpoint_dir: PathBuf,
    /// Maximum number of checkpoints to keep
    pub max_checkpoints: usize,
    /// Enable automatic periodic checkpointing
    pub auto_checkpoint: bool,
    /// Checkpoint interval (in simulation steps)
    pub checkpoint_interval: usize,
    /// Enable compression
    pub compression: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            checkpoint_dir: PathBuf::from("./checkpoints"),
            max_checkpoints: 10,
            auto_checkpoint: true,
            checkpoint_interval: 100,
            compression: false,
        }
    }
}

impl PersistenceConfig {
    /// Creates a new persistence configuration
    pub fn new(checkpoint_dir: PathBuf) -> Self {
        Self {
            checkpoint_dir,
            ..Default::default()
        }
    }

    /// Sets the maximum number of checkpoints to keep
    pub fn with_max_checkpoints(mut self, max: usize) -> Self {
        self.max_checkpoints = max;
        self
    }

    /// Enables or disables automatic checkpointing
    pub fn with_auto_checkpoint(mut self, enabled: bool) -> Self {
        self.auto_checkpoint = enabled;
        self
    }

    /// Sets the checkpoint interval
    pub fn with_checkpoint_interval(mut self, interval: usize) -> Self {
        self.checkpoint_interval = interval;
        self
    }

    /// Enables or disables compression
    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.compression = enabled;
        self
    }
}

/// File-based checkpoint store
#[derive(Debug)]
pub struct CheckpointStore {
    config: PersistenceConfig,
}

impl CheckpointStore {
    /// Creates a new checkpoint store
    pub fn new(config: PersistenceConfig) -> SimResult<Self> {
        // Create checkpoint directory if it doesn't exist
        if !config.checkpoint_dir.exists() {
            fs::create_dir_all(&config.checkpoint_dir).map_err(|e| {
                SimulationError::Checkpoint(format!("Failed to create checkpoint directory: {}", e))
            })?;
        }

        Ok(Self { config })
    }

    /// Saves a checkpoint to disk
    pub fn save(&self, checkpoint: &Checkpoint) -> SimResult<PathBuf> {
        let file_path = self.checkpoint_path(&checkpoint.id);

        // Serialize to JSON
        let json = checkpoint
            .to_json()
            .map_err(|e| SimulationError::Checkpoint(format!("Serialization failed: {}", e)))?;

        // Write to file
        fs::write(&file_path, json).map_err(|e| {
            SimulationError::Checkpoint(format!("Failed to write checkpoint: {}", e))
        })?;

        // Clean up old checkpoints if needed
        self.cleanup_old_checkpoints()?;

        Ok(file_path)
    }

    /// Loads a checkpoint from disk
    pub fn load(&self, id: &str) -> SimResult<Checkpoint> {
        let file_path = self.checkpoint_path(id);

        if !file_path.exists() {
            return Err(SimulationError::Checkpoint(format!(
                "Checkpoint not found: {}",
                id
            )));
        }

        // Read from file
        let json = fs::read_to_string(&file_path).map_err(|e| {
            SimulationError::Checkpoint(format!("Failed to read checkpoint: {}", e))
        })?;

        // Deserialize from JSON
        Checkpoint::from_json(&json)
            .map_err(|e| SimulationError::Checkpoint(format!("Deserialization failed: {}", e)))
    }

    /// Lists all available checkpoint IDs
    pub fn list_checkpoints(&self) -> SimResult<Vec<String>> {
        if !self.config.checkpoint_dir.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.config.checkpoint_dir).map_err(|e| {
            SimulationError::Checkpoint(format!("Failed to read checkpoint directory: {}", e))
        })?;

        let mut checkpoint_ids = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                SimulationError::Checkpoint(format!("Failed to read directory entry: {}", e))
            })?;

            if let Some(file_name) = entry.file_name().to_str()
                && file_name.ends_with(".json")
            {
                let id = file_name.trim_end_matches(".json");
                checkpoint_ids.push(id.to_string());
            }
        }

        Ok(checkpoint_ids)
    }

    /// Deletes a checkpoint
    pub fn delete(&self, id: &str) -> SimResult<()> {
        let file_path = self.checkpoint_path(id);

        if file_path.exists() {
            fs::remove_file(&file_path).map_err(|e| {
                SimulationError::Checkpoint(format!("Failed to delete checkpoint: {}", e))
            })?;
        }

        Ok(())
    }

    /// Loads the most recent checkpoint
    pub fn load_latest(&self) -> SimResult<Option<Checkpoint>> {
        let checkpoint_ids = self.list_checkpoints()?;

        if checkpoint_ids.is_empty() {
            return Ok(None);
        }

        // Load all checkpoints to find the latest
        let mut latest: Option<Checkpoint> = None;
        for id in checkpoint_ids {
            if let Ok(checkpoint) = self.load(&id) {
                if let Some(ref current) = latest {
                    if checkpoint.timestamp > current.timestamp {
                        latest = Some(checkpoint);
                    }
                } else {
                    latest = Some(checkpoint);
                }
            }
        }

        Ok(latest)
    }

    /// Cleans up old checkpoints to stay within max_checkpoints limit
    fn cleanup_old_checkpoints(&self) -> SimResult<()> {
        let checkpoint_ids = self.list_checkpoints()?;

        if checkpoint_ids.len() <= self.config.max_checkpoints {
            return Ok(());
        }

        // Load timestamps
        let mut checkpoints_with_time: Vec<(String, chrono::DateTime<chrono::Utc>)> = Vec::new();
        for id in &checkpoint_ids {
            if let Ok(checkpoint) = self.load(id) {
                checkpoints_with_time.push((id.clone(), checkpoint.timestamp));
            }
        }

        // Sort by timestamp
        checkpoints_with_time.sort_by(|a, b| a.1.cmp(&b.1));

        // Delete oldest checkpoints
        let to_delete = checkpoints_with_time.len() - self.config.max_checkpoints;
        for checkpoint_info in checkpoints_with_time.iter().take(to_delete) {
            self.delete(&checkpoint_info.0)?;
        }

        Ok(())
    }

    /// Gets the file path for a checkpoint
    fn checkpoint_path(&self, id: &str) -> PathBuf {
        self.config.checkpoint_dir.join(format!("{}.json", id))
    }

    /// Validates a checkpoint file
    pub fn validate(&self, id: &str) -> SimResult<bool> {
        let checkpoint = self.load(id)?;

        // Basic validation
        if checkpoint.id.is_empty() {
            return Ok(false);
        }

        // Check that entity states are valid
        for attrs in checkpoint.entity_states.values() {
            if attrs.is_empty() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Resume manager for handling simulation recovery
#[derive(Debug)]
pub struct ResumeManager {
    store: CheckpointStore,
    /// Metadata about interrupted simulations
    interrupted_simulations: HashMap<String, InterruptedSimulation>,
}

/// Information about an interrupted simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptedSimulation {
    /// Simulation ID
    pub simulation_id: String,
    /// Last checkpoint ID
    pub last_checkpoint_id: String,
    /// Timestamp of interruption
    pub interrupted_at: chrono::DateTime<chrono::Utc>,
    /// Current step when interrupted
    pub current_step: usize,
    /// Total steps planned
    pub total_steps: usize,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ResumeManager {
    /// Creates a new resume manager
    pub fn new(config: PersistenceConfig) -> SimResult<Self> {
        let store = CheckpointStore::new(config)?;
        let interrupted_simulations = HashMap::new();

        Ok(Self {
            store,
            interrupted_simulations,
        })
    }

    /// Marks a simulation as interrupted
    pub fn mark_interrupted(
        &mut self,
        simulation_id: String,
        last_checkpoint_id: String,
        current_step: usize,
        total_steps: usize,
    ) {
        let interrupted = InterruptedSimulation {
            simulation_id: simulation_id.clone(),
            last_checkpoint_id,
            interrupted_at: chrono::Utc::now(),
            current_step,
            total_steps,
            metadata: HashMap::new(),
        };

        self.interrupted_simulations
            .insert(simulation_id, interrupted);
    }

    /// Checks if a simulation can be resumed
    pub fn can_resume(&self, simulation_id: &str) -> bool {
        self.interrupted_simulations.contains_key(simulation_id)
    }

    /// Gets information about an interrupted simulation
    pub fn get_interrupted(&self, simulation_id: &str) -> Option<&InterruptedSimulation> {
        self.interrupted_simulations.get(simulation_id)
    }

    /// Resumes a simulation by loading its last checkpoint
    pub fn resume(&mut self, simulation_id: &str) -> SimResult<(Checkpoint, usize, usize)> {
        let interrupted = self
            .interrupted_simulations
            .get(simulation_id)
            .ok_or_else(|| {
                SimulationError::Checkpoint(format!(
                    "No interrupted simulation found: {}",
                    simulation_id
                ))
            })?
            .clone();

        // Load the last checkpoint
        let checkpoint = self.store.load(&interrupted.last_checkpoint_id)?;

        // Remove from interrupted list
        self.interrupted_simulations.remove(simulation_id);

        Ok((
            checkpoint,
            interrupted.current_step,
            interrupted.total_steps,
        ))
    }

    /// Clears all interrupted simulation records
    pub fn clear_interrupted(&mut self) {
        self.interrupted_simulations.clear();
    }

    /// Lists all interrupted simulations
    pub fn list_interrupted(&self) -> Vec<String> {
        self.interrupted_simulations.keys().cloned().collect()
    }

    /// Access to the checkpoint store
    pub fn store(&self) -> &CheckpointStore {
        &self.store
    }
}

/// Auto-checkpoint tracker for periodic checkpointing
#[derive(Debug)]
pub struct AutoCheckpoint {
    config: PersistenceConfig,
    store: CheckpointStore,
    current_step: usize,
    last_checkpoint_step: usize,
}

impl AutoCheckpoint {
    /// Creates a new auto-checkpoint tracker
    pub fn new(config: PersistenceConfig) -> SimResult<Self> {
        let store = CheckpointStore::new(config.clone())?;

        Ok(Self {
            config,
            store,
            current_step: 0,
            last_checkpoint_step: 0,
        })
    }

    /// Increments the step counter and returns true if checkpoint is needed
    pub fn step(&mut self) -> bool {
        self.current_step += 1;

        if !self.config.auto_checkpoint {
            return false;
        }

        let steps_since_checkpoint = self.current_step - self.last_checkpoint_step;
        steps_since_checkpoint >= self.config.checkpoint_interval
    }

    /// Saves a checkpoint
    pub fn save_checkpoint(&mut self, checkpoint: &Checkpoint) -> SimResult<PathBuf> {
        let path = self.store.save(checkpoint)?;
        self.last_checkpoint_step = self.current_step;
        Ok(path)
    }

    /// Gets the current step
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    /// Resets the step counter
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.last_checkpoint_step = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimulationMetrics;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn temp_dir() -> PathBuf {
        let id = Uuid::new_v4();
        PathBuf::from(format!("/tmp/legalis-sim-test-{}", id))
    }

    #[test]
    fn test_persistence_config() {
        let config = PersistenceConfig::new(PathBuf::from("./test"))
            .with_max_checkpoints(5)
            .with_auto_checkpoint(true)
            .with_checkpoint_interval(50)
            .with_compression(true);

        assert_eq!(config.max_checkpoints, 5);
        assert!(config.auto_checkpoint);
        assert_eq!(config.checkpoint_interval, 50);
        assert!(config.compression);
    }

    #[test]
    fn test_checkpoint_store_save_load() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let store = CheckpointStore::new(config).unwrap();

        let mut checkpoint = Checkpoint::new("test-1".to_string(), SimulationMetrics::new());
        let entity_id = Uuid::new_v4();
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), "Alice".to_string());
        checkpoint.add_entity_state(entity_id, attrs);

        // Save checkpoint
        let path = store.save(&checkpoint).unwrap();
        assert!(path.exists());

        // Load checkpoint
        let loaded = store.load("test-1").unwrap();
        assert_eq!(loaded.id, "test-1");
        assert!(loaded.entity_states.contains_key(&entity_id));

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_checkpoint_store_list() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let store = CheckpointStore::new(config).unwrap();

        // Save multiple checkpoints
        for i in 1..=3 {
            let checkpoint = Checkpoint::new(format!("test-{}", i), SimulationMetrics::new());
            store.save(&checkpoint).unwrap();
        }

        // List checkpoints
        let ids = store.list_checkpoints().unwrap();
        assert_eq!(ids.len(), 3);

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_checkpoint_store_delete() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let store = CheckpointStore::new(config).unwrap();

        let checkpoint = Checkpoint::new("test-1".to_string(), SimulationMetrics::new());
        store.save(&checkpoint).unwrap();

        // Delete checkpoint
        store.delete("test-1").unwrap();

        // Verify it's gone
        assert!(store.load("test-1").is_err());

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_checkpoint_store_latest() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let store = CheckpointStore::new(config).unwrap();

        // Save checkpoints with slight delays
        for i in 1..=3 {
            let checkpoint = Checkpoint::new(format!("test-{}", i), SimulationMetrics::new());
            store.save(&checkpoint).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Get latest
        let latest = store.load_latest().unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().id, "test-3");

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_checkpoint_store_cleanup() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone()).with_max_checkpoints(2);
        let store = CheckpointStore::new(config).unwrap();

        // Save 3 checkpoints
        for i in 1..=3 {
            let checkpoint = Checkpoint::new(format!("test-{}", i), SimulationMetrics::new());
            store.save(&checkpoint).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Should only have 2 checkpoints
        let ids = store.list_checkpoints().unwrap();
        assert_eq!(ids.len(), 2);

        // Oldest should be deleted
        assert!(store.load("test-1").is_err());

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_resume_manager() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let mut manager = ResumeManager::new(config).unwrap();

        // Mark simulation as interrupted
        manager.mark_interrupted("sim-1".to_string(), "cp-1".to_string(), 50, 100);

        assert!(manager.can_resume("sim-1"));
        assert!(!manager.can_resume("sim-2"));

        let interrupted = manager.get_interrupted("sim-1").unwrap();
        assert_eq!(interrupted.current_step, 50);
        assert_eq!(interrupted.total_steps, 100);

        // List interrupted
        let list = manager.list_interrupted();
        assert_eq!(list.len(), 1);

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_resume_manager_resume() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let mut manager = ResumeManager::new(config).unwrap();

        // Create and save a checkpoint
        let checkpoint = Checkpoint::new("cp-1".to_string(), SimulationMetrics::new());
        manager.store.save(&checkpoint).unwrap();

        // Mark as interrupted
        manager.mark_interrupted("sim-1".to_string(), "cp-1".to_string(), 50, 100);

        // Resume
        let (loaded_checkpoint, current_step, total_steps) = manager.resume("sim-1").unwrap();
        assert_eq!(loaded_checkpoint.id, "cp-1");
        assert_eq!(current_step, 50);
        assert_eq!(total_steps, 100);

        // Should no longer be in interrupted list
        assert!(!manager.can_resume("sim-1"));

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_auto_checkpoint() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone())
            .with_auto_checkpoint(true)
            .with_checkpoint_interval(10);

        let mut auto_cp = AutoCheckpoint::new(config).unwrap();

        // Step 9 times - should not need checkpoint
        for _ in 0..9 {
            assert!(!auto_cp.step());
        }

        // Step 10th time - should need checkpoint
        assert!(auto_cp.step());

        // Save checkpoint
        let checkpoint = Checkpoint::new("test-1".to_string(), SimulationMetrics::new());
        auto_cp.save_checkpoint(&checkpoint).unwrap();

        // Next 9 steps should not need checkpoint
        for _ in 0..9 {
            assert!(!auto_cp.step());
        }

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_auto_checkpoint_disabled() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone()).with_auto_checkpoint(false);

        let mut auto_cp = AutoCheckpoint::new(config).unwrap();

        // Even after many steps, should never need checkpoint when disabled
        for _ in 0..100 {
            assert!(!auto_cp.step());
        }

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_checkpoint_validation() {
        let dir = temp_dir();
        let config = PersistenceConfig::new(dir.clone());
        let store = CheckpointStore::new(config).unwrap();

        let mut checkpoint = Checkpoint::new("test-1".to_string(), SimulationMetrics::new());
        let entity_id = Uuid::new_v4();
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), "Alice".to_string());
        checkpoint.add_entity_state(entity_id, attrs);

        store.save(&checkpoint).unwrap();

        // Validate checkpoint
        assert!(store.validate("test-1").unwrap());

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }
}
