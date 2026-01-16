//! Digital Twin Integration Module
//!
//! This module provides capabilities for creating and managing digital twins:
//! - Real-time entity synchronization between physical and digital twins
//! - IoT data ingestion from sensors and devices
//! - Predictive maintenance simulation
//! - Twin-based what-if analysis
//! - Bidirectional updates between physical and digital entities

use crate::error::{SimResult, SimulationError};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Digital twin of a physical entity with real-time synchronization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwin {
    /// Unique identifier for the twin
    pub id: Uuid,
    /// ID of the physical entity this twin represents
    pub physical_entity_id: String,
    /// Current state of the twin
    pub state: HashMap<String, f64>,
    /// Metadata about the twin
    pub metadata: HashMap<String, String>,
    /// Last synchronization timestamp
    pub last_sync: DateTime<Utc>,
    /// Synchronization status
    pub sync_status: SyncStatus,
}

/// Synchronization status between physical entity and digital twin.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    /// In sync with physical entity
    Synced,
    /// Out of sync - update pending
    Pending,
    /// Synchronization in progress
    Syncing,
    /// Synchronization failed
    Failed,
    /// Disconnected from physical entity
    Disconnected,
}

impl DigitalTwin {
    /// Creates a new digital twin for a physical entity.
    pub fn new(physical_entity_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            physical_entity_id,
            state: HashMap::new(),
            metadata: HashMap::new(),
            last_sync: Utc::now(),
            sync_status: SyncStatus::Synced,
        }
    }

    /// Updates the state from a physical entity.
    pub fn update_from_physical(&mut self, state_update: HashMap<String, f64>) {
        self.sync_status = SyncStatus::Syncing;
        self.state.extend(state_update);
        self.last_sync = Utc::now();
        self.sync_status = SyncStatus::Synced;
    }

    /// Gets a state value.
    pub fn get_state(&self, key: &str) -> Option<f64> {
        self.state.get(key).copied()
    }

    /// Sets a state value.
    pub fn set_state(&mut self, key: String, value: f64) {
        self.state.insert(key, value);
        self.sync_status = SyncStatus::Pending;
    }

    /// Checks if the twin needs synchronization.
    pub fn needs_sync(&self) -> bool {
        matches!(self.sync_status, SyncStatus::Pending | SyncStatus::Failed)
    }
}

/// Real-time synchronization manager for digital twins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationManager {
    /// Collection of digital twins
    pub twins: HashMap<Uuid, DigitalTwin>,
    /// Synchronization interval in seconds
    pub sync_interval: u64,
    /// Maximum acceptable delay in seconds
    pub max_delay: u64,
}

impl SynchronizationManager {
    /// Creates a new synchronization manager.
    pub fn new(sync_interval: u64) -> Self {
        Self {
            twins: HashMap::new(),
            sync_interval,
            max_delay: 60,
        }
    }

    /// Registers a new digital twin.
    pub fn register_twin(&mut self, twin: DigitalTwin) -> Uuid {
        let id = twin.id;
        self.twins.insert(id, twin);
        id
    }

    /// Synchronizes all twins that need updates.
    pub fn synchronize_all(&mut self) -> SyncResult {
        let mut synced = 0;
        let mut failed = 0;
        let mut skipped = 0;

        for twin in self.twins.values_mut() {
            if twin.needs_sync() {
                // Simulate synchronization
                let success = rand::rng().random_range(0.0..1.0) > 0.1; // 90% success rate
                if success {
                    twin.sync_status = SyncStatus::Synced;
                    twin.last_sync = Utc::now();
                    synced += 1;
                } else {
                    twin.sync_status = SyncStatus::Failed;
                    failed += 1;
                }
            } else {
                skipped += 1;
            }
        }

        SyncResult {
            synced,
            failed,
            skipped,
            total: self.twins.len(),
        }
    }

    /// Gets twins that are out of sync.
    pub fn get_out_of_sync_twins(&self) -> Vec<Uuid> {
        self.twins
            .iter()
            .filter(|(_, twin)| {
                let elapsed = Utc::now()
                    .signed_duration_since(twin.last_sync)
                    .num_seconds() as u64;
                elapsed > self.max_delay || twin.needs_sync()
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Gets a twin by ID.
    pub fn get_twin(&self, id: &Uuid) -> Option<&DigitalTwin> {
        self.twins.get(id)
    }

    /// Gets a mutable twin by ID.
    pub fn get_twin_mut(&mut self, id: &Uuid) -> Option<&mut DigitalTwin> {
        self.twins.get_mut(id)
    }
}

/// Result of a synchronization operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// Number of twins successfully synced
    pub synced: usize,
    /// Number of twins that failed to sync
    pub failed: usize,
    /// Number of twins that didn't need syncing
    pub skipped: usize,
    /// Total number of twins
    pub total: usize,
}

/// IoT sensor data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    /// Sensor identifier
    pub sensor_id: String,
    /// Timestamp of the measurement
    pub timestamp: DateTime<Utc>,
    /// Sensor reading value
    pub value: f64,
    /// Unit of measurement
    pub unit: String,
    /// Data quality indicator (0.0 to 1.0)
    pub quality: f64,
}

impl SensorData {
    /// Creates a new sensor data point.
    pub fn new(sensor_id: String, value: f64, unit: String) -> Self {
        Self {
            sensor_id,
            timestamp: Utc::now(),
            value,
            unit,
            quality: 1.0,
        }
    }

    /// Checks if the data is high quality.
    pub fn is_high_quality(&self) -> bool {
        self.quality >= 0.8
    }
}

/// IoT data ingestion framework for collecting sensor data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDataIngestion {
    /// Buffer of incoming sensor data
    pub data_buffer: Vec<SensorData>,
    /// Maximum buffer size
    pub max_buffer_size: usize,
    /// Data validation rules
    pub validation_rules: HashMap<String, SensorValidationRule>,
}

/// Validation rule for sensor data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorValidationRule {
    /// Minimum acceptable value
    pub min_value: f64,
    /// Maximum acceptable value
    pub max_value: f64,
    /// Minimum quality threshold
    pub min_quality: f64,
}

impl IoTDataIngestion {
    /// Creates a new IoT data ingestion system.
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            data_buffer: Vec::new(),
            max_buffer_size,
            validation_rules: HashMap::new(),
        }
    }

    /// Adds a validation rule for a sensor.
    pub fn add_validation_rule(&mut self, sensor_id: String, rule: SensorValidationRule) {
        self.validation_rules.insert(sensor_id, rule);
    }

    /// Ingests sensor data with validation.
    pub fn ingest(&mut self, data: SensorData) -> SimResult<()> {
        // Validate data if rule exists
        if let Some(rule) = self.validation_rules.get(&data.sensor_id) {
            if data.value < rule.min_value || data.value > rule.max_value {
                return Err(SimulationError::InvalidParameter(format!(
                    "Sensor value {} out of range [{}, {}]",
                    data.value, rule.min_value, rule.max_value
                )));
            }
            if data.quality < rule.min_quality {
                return Err(SimulationError::InvalidParameter(format!(
                    "Data quality {} below threshold {}",
                    data.quality, rule.min_quality
                )));
            }
        }

        // Add to buffer
        self.data_buffer.push(data);

        // Trim buffer if needed
        if self.data_buffer.len() > self.max_buffer_size {
            self.data_buffer.remove(0);
        }

        Ok(())
    }

    /// Gets recent data for a specific sensor.
    pub fn get_sensor_data(&self, sensor_id: &str, count: usize) -> Vec<&SensorData> {
        self.data_buffer
            .iter()
            .rev()
            .filter(|d| d.sensor_id == sensor_id)
            .take(count)
            .collect()
    }

    /// Clears the data buffer.
    pub fn clear_buffer(&mut self) {
        self.data_buffer.clear();
    }

    /// Gets buffer utilization (0.0 to 1.0).
    pub fn buffer_utilization(&self) -> f64 {
        self.data_buffer.len() as f64 / self.max_buffer_size as f64
    }
}

/// Predictive maintenance model for equipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveMaintenance {
    /// Equipment identifier
    pub equipment_id: String,
    /// Current health score (0.0 to 1.0)
    pub health_score: f64,
    /// Predicted time to failure in hours
    pub predicted_ttf: f64,
    /// Maintenance threshold (trigger maintenance below this)
    pub maintenance_threshold: f64,
    /// Failure probability (0.0 to 1.0)
    pub failure_probability: f64,
}

impl PredictiveMaintenance {
    /// Creates a new predictive maintenance model.
    pub fn new(equipment_id: String) -> Self {
        Self {
            equipment_id,
            health_score: 1.0,
            predicted_ttf: f64::INFINITY,
            maintenance_threshold: 0.3,
            failure_probability: 0.0,
        }
    }

    /// Updates health score based on sensor data.
    pub fn update_health(&mut self, sensor_data: &[SensorData]) {
        if sensor_data.is_empty() {
            return;
        }

        // Simple health model: average of normalized sensor values
        let mut degradation = 0.0;
        for data in sensor_data {
            // Assume higher values indicate more degradation
            degradation += data.value / 100.0;
        }
        degradation /= sensor_data.len() as f64;

        self.health_score = (1.0 - degradation).clamp(0.0, 1.0);
        self.update_predictions();
    }

    /// Updates failure predictions based on current health.
    fn update_predictions(&mut self) {
        // Exponential degradation model
        if self.health_score > 0.0 {
            self.predicted_ttf = -100.0 * self.health_score.ln();
        } else {
            self.predicted_ttf = 0.0;
        }

        // Failure probability increases as health decreases
        self.failure_probability = (1.0 - self.health_score).powi(2);
    }

    /// Checks if maintenance is recommended.
    pub fn needs_maintenance(&self) -> bool {
        self.health_score < self.maintenance_threshold
    }

    /// Gets maintenance urgency level.
    pub fn urgency_level(&self) -> MaintenanceUrgency {
        if self.health_score < 0.1 {
            MaintenanceUrgency::Critical
        } else if self.health_score < 0.3 {
            MaintenanceUrgency::High
        } else if self.health_score < 0.5 {
            MaintenanceUrgency::Medium
        } else if self.health_score < 0.7 {
            MaintenanceUrgency::Low
        } else {
            MaintenanceUrgency::None
        }
    }

    /// Simulates performing maintenance.
    pub fn perform_maintenance(&mut self) {
        self.health_score = 0.95; // Maintenance restores health to 95%
        self.update_predictions();
    }
}

/// Maintenance urgency level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaintenanceUrgency {
    /// No maintenance needed
    None,
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical - immediate action required
    Critical,
}

/// What-if analysis scenario for digital twins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfScenario {
    /// Scenario name
    pub name: String,
    /// Description of the scenario
    pub description: String,
    /// State changes to apply
    pub state_changes: HashMap<String, f64>,
    /// Expected outcomes
    pub expected_outcomes: HashMap<String, f64>,
}

impl WhatIfScenario {
    /// Creates a new what-if scenario.
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            state_changes: HashMap::new(),
            expected_outcomes: HashMap::new(),
        }
    }

    /// Adds a state change to the scenario.
    pub fn add_state_change(&mut self, key: String, value: f64) {
        self.state_changes.insert(key, value);
    }

    /// Adds an expected outcome.
    pub fn add_expected_outcome(&mut self, key: String, value: f64) {
        self.expected_outcomes.insert(key, value);
    }
}

/// What-if analysis engine for digital twins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfAnalysis {
    /// Base digital twin for analysis
    pub base_twin: DigitalTwin,
    /// Scenarios to analyze
    pub scenarios: Vec<WhatIfScenario>,
}

impl WhatIfAnalysis {
    /// Creates a new what-if analysis.
    pub fn new(base_twin: DigitalTwin) -> Self {
        Self {
            base_twin,
            scenarios: Vec::new(),
        }
    }

    /// Adds a scenario to analyze.
    pub fn add_scenario(&mut self, scenario: WhatIfScenario) {
        self.scenarios.push(scenario);
    }

    /// Runs a scenario and returns the resulting twin state.
    pub fn run_scenario(&self, scenario: &WhatIfScenario) -> DigitalTwin {
        let mut twin = self.base_twin.clone();

        // Apply state changes
        for (key, value) in &scenario.state_changes {
            twin.set_state(key.clone(), *value);
        }

        twin
    }

    /// Runs all scenarios and returns results.
    pub fn run_all_scenarios(&self) -> Vec<ScenarioResult> {
        self.scenarios
            .iter()
            .map(|scenario| {
                let result_twin = self.run_scenario(scenario);
                ScenarioResult {
                    scenario_name: scenario.name.clone(),
                    resulting_state: result_twin.state,
                    expected_outcomes: scenario.expected_outcomes.clone(),
                }
            })
            .collect()
    }

    /// Compares scenario outcomes.
    pub fn compare_scenarios(&self) -> TwinScenarioComparison {
        let results = self.run_all_scenarios();

        TwinScenarioComparison {
            base_state: self.base_twin.state.clone(),
            scenario_results: results,
        }
    }
}

/// Result of running a what-if scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    /// Name of the scenario
    pub scenario_name: String,
    /// Resulting state after applying changes
    pub resulting_state: HashMap<String, f64>,
    /// Expected outcomes for comparison
    pub expected_outcomes: HashMap<String, f64>,
}

/// Comparison of multiple scenario results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinScenarioComparison {
    /// Base state before any scenarios
    pub base_state: HashMap<String, f64>,
    /// Results from all scenarios
    pub scenario_results: Vec<ScenarioResult>,
}

/// Bidirectional update manager for twin synchronization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidirectionalSync {
    /// Update queue from physical to digital
    pub physical_to_digital: Vec<StateUpdate>,
    /// Update queue from digital to physical
    pub digital_to_physical: Vec<StateUpdate>,
    /// Maximum queue size
    pub max_queue_size: usize,
}

/// State update between physical and digital entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateUpdate {
    /// Entity identifier
    pub entity_id: Uuid,
    /// State changes
    pub changes: HashMap<String, f64>,
    /// Timestamp of the update
    pub timestamp: DateTime<Utc>,
    /// Priority of the update
    pub priority: UpdatePriority,
}

/// Priority level for state updates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdatePriority {
    /// Low priority - batch updates
    Low,
    /// Normal priority
    Normal,
    /// High priority - process soon
    High,
    /// Critical priority - process immediately
    Critical,
}

impl BidirectionalSync {
    /// Creates a new bidirectional sync manager.
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            physical_to_digital: Vec::new(),
            digital_to_physical: Vec::new(),
            max_queue_size,
        }
    }

    /// Queues an update from physical to digital.
    pub fn queue_physical_update(&mut self, update: StateUpdate) -> SimResult<()> {
        if self.physical_to_digital.len() >= self.max_queue_size {
            return Err(SimulationError::InvalidParameter(
                "Physical to digital queue is full".to_string(),
            ));
        }
        self.physical_to_digital.push(update);
        Self::sort_queue(&mut self.physical_to_digital);
        Ok(())
    }

    /// Queues an update from digital to physical.
    pub fn queue_digital_update(&mut self, update: StateUpdate) -> SimResult<()> {
        if self.digital_to_physical.len() >= self.max_queue_size {
            return Err(SimulationError::InvalidParameter(
                "Digital to physical queue is full".to_string(),
            ));
        }
        self.digital_to_physical.push(update);
        Self::sort_queue(&mut self.digital_to_physical);
        Ok(())
    }

    /// Sorts queue by priority (highest first).
    fn sort_queue(queue: &mut [StateUpdate]) {
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Processes next physical to digital update.
    pub fn process_physical_update(&mut self) -> Option<StateUpdate> {
        if !self.physical_to_digital.is_empty() {
            Some(self.physical_to_digital.remove(0))
        } else {
            None
        }
    }

    /// Processes next digital to physical update.
    pub fn process_digital_update(&mut self) -> Option<StateUpdate> {
        if !self.digital_to_physical.is_empty() {
            Some(self.digital_to_physical.remove(0))
        } else {
            None
        }
    }

    /// Gets queue statistics.
    pub fn queue_stats(&self) -> TwinQueueStats {
        TwinQueueStats {
            physical_to_digital_count: self.physical_to_digital.len(),
            digital_to_physical_count: self.digital_to_physical.len(),
            total_capacity: self.max_queue_size,
        }
    }
}

/// Statistics about update queues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinQueueStats {
    /// Number of pending physical to digital updates
    pub physical_to_digital_count: usize,
    /// Number of pending digital to physical updates
    pub digital_to_physical_count: usize,
    /// Total queue capacity
    pub total_capacity: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_twin_creation() {
        let twin = DigitalTwin::new("entity_123".to_string());
        assert_eq!(twin.physical_entity_id, "entity_123");
        assert_eq!(twin.sync_status, SyncStatus::Synced);
        assert!(twin.state.is_empty());
    }

    #[test]
    fn test_digital_twin_state_update() {
        let mut twin = DigitalTwin::new("entity_123".to_string());
        twin.set_state("temperature".to_string(), 25.0);

        assert_eq!(twin.get_state("temperature"), Some(25.0));
        assert_eq!(twin.sync_status, SyncStatus::Pending);
        assert!(twin.needs_sync());
    }

    #[test]
    fn test_synchronization_manager() {
        let mut manager = SynchronizationManager::new(10);
        let twin = DigitalTwin::new("entity_1".to_string());
        let id = manager.register_twin(twin);

        assert_eq!(manager.twins.len(), 1);
        assert!(manager.get_twin(&id).is_some());
    }

    #[test]
    fn test_sync_result() {
        let mut manager = SynchronizationManager::new(10);

        let mut twin1 = DigitalTwin::new("entity_1".to_string());
        twin1.sync_status = SyncStatus::Pending;
        manager.register_twin(twin1);

        let result = manager.synchronize_all();
        assert_eq!(result.total, 1);
        assert!(result.synced + result.failed == 1);
    }

    #[test]
    fn test_sensor_data_creation() {
        let data = SensorData::new("temp_sensor_1".to_string(), 23.5, "celsius".to_string());

        assert_eq!(data.sensor_id, "temp_sensor_1");
        assert_eq!(data.value, 23.5);
        assert_eq!(data.unit, "celsius");
        assert!(data.is_high_quality());
    }

    #[test]
    fn test_iot_data_ingestion() {
        let mut ingestion = IoTDataIngestion::new(100);
        let data = SensorData::new("sensor_1".to_string(), 50.0, "percent".to_string());

        assert!(ingestion.ingest(data).is_ok());
        assert_eq!(ingestion.data_buffer.len(), 1);
    }

    #[test]
    fn test_iot_validation_rule() {
        let mut ingestion = IoTDataIngestion::new(100);
        ingestion.add_validation_rule(
            "sensor_1".to_string(),
            SensorValidationRule {
                min_value: 0.0,
                max_value: 100.0,
                min_quality: 0.5,
            },
        );

        let valid_data = SensorData::new("sensor_1".to_string(), 50.0, "units".to_string());
        assert!(ingestion.ingest(valid_data).is_ok());

        let invalid_data = SensorData::new("sensor_1".to_string(), 150.0, "units".to_string());
        assert!(ingestion.ingest(invalid_data).is_err());
    }

    #[test]
    fn test_buffer_overflow() {
        let mut ingestion = IoTDataIngestion::new(2);

        for i in 0..5 {
            let data = SensorData::new(format!("sensor_{}", i), i as f64, "units".to_string());
            ingestion.ingest(data).unwrap();
        }

        assert_eq!(ingestion.data_buffer.len(), 2);
    }

    #[test]
    fn test_predictive_maintenance() {
        let pm = PredictiveMaintenance::new("equipment_1".to_string());

        assert_eq!(pm.health_score, 1.0);
        assert!(!pm.needs_maintenance());
        assert_eq!(pm.urgency_level(), MaintenanceUrgency::None);
    }

    #[test]
    fn test_health_degradation() {
        let mut pm = PredictiveMaintenance::new("equipment_1".to_string());

        let data = vec![
            SensorData::new("vibration".to_string(), 80.0, "Hz".to_string()),
            SensorData::new("temperature".to_string(), 90.0, "C".to_string()),
        ];

        pm.update_health(&data);
        assert!(pm.health_score < 1.0);
    }

    #[test]
    fn test_maintenance_urgency() {
        let mut pm = PredictiveMaintenance::new("equipment_1".to_string());

        pm.health_score = 0.05;
        pm.update_predictions();
        assert_eq!(pm.urgency_level(), MaintenanceUrgency::Critical);

        pm.health_score = 0.25;
        pm.update_predictions();
        assert_eq!(pm.urgency_level(), MaintenanceUrgency::High);
    }

    #[test]
    fn test_perform_maintenance() {
        let mut pm = PredictiveMaintenance::new("equipment_1".to_string());
        pm.health_score = 0.2;

        pm.perform_maintenance();
        assert_eq!(pm.health_score, 0.95);
        assert!(!pm.needs_maintenance());
    }

    #[test]
    fn test_whatif_scenario() {
        let mut scenario =
            WhatIfScenario::new("Test Scenario".to_string(), "Testing scenario".to_string());

        scenario.add_state_change("temperature".to_string(), 30.0);
        scenario.add_expected_outcome("efficiency".to_string(), 0.85);

        assert_eq!(scenario.state_changes.len(), 1);
        assert_eq!(scenario.expected_outcomes.len(), 1);
    }

    #[test]
    fn test_whatif_analysis() {
        let mut twin = DigitalTwin::new("entity_1".to_string());
        twin.set_state("temperature".to_string(), 20.0);

        let mut analysis = WhatIfAnalysis::new(twin);
        let mut scenario =
            WhatIfScenario::new("Hot Day".to_string(), "Simulate hot weather".to_string());
        scenario.add_state_change("temperature".to_string(), 35.0);

        analysis.add_scenario(scenario);
        let results = analysis.run_all_scenarios();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].scenario_name, "Hot Day");
    }

    #[test]
    fn test_scenario_comparison() {
        let twin = DigitalTwin::new("entity_1".to_string());
        let mut analysis = WhatIfAnalysis::new(twin);

        let scenario1 = WhatIfScenario::new("Scenario 1".to_string(), "Test 1".to_string());
        let scenario2 = WhatIfScenario::new("Scenario 2".to_string(), "Test 2".to_string());

        analysis.add_scenario(scenario1);
        analysis.add_scenario(scenario2);

        let comparison = analysis.compare_scenarios();
        assert_eq!(comparison.scenario_results.len(), 2);
    }

    #[test]
    fn test_bidirectional_sync() {
        let sync = BidirectionalSync::new(100);

        let stats = sync.queue_stats();
        assert_eq!(stats.physical_to_digital_count, 0);
        assert_eq!(stats.digital_to_physical_count, 0);
        assert_eq!(stats.total_capacity, 100);
    }

    #[test]
    fn test_queue_physical_update() {
        let mut sync = BidirectionalSync::new(10);

        let update = StateUpdate {
            entity_id: Uuid::new_v4(),
            changes: HashMap::new(),
            timestamp: Utc::now(),
            priority: UpdatePriority::Normal,
        };

        assert!(sync.queue_physical_update(update).is_ok());
        assert_eq!(sync.physical_to_digital.len(), 1);
    }

    #[test]
    fn test_queue_digital_update() {
        let mut sync = BidirectionalSync::new(10);

        let update = StateUpdate {
            entity_id: Uuid::new_v4(),
            changes: HashMap::new(),
            timestamp: Utc::now(),
            priority: UpdatePriority::High,
        };

        assert!(sync.queue_digital_update(update).is_ok());
        assert_eq!(sync.digital_to_physical.len(), 1);
    }

    #[test]
    fn test_update_priority_sorting() {
        let mut sync = BidirectionalSync::new(10);

        let low = StateUpdate {
            entity_id: Uuid::new_v4(),
            changes: HashMap::new(),
            timestamp: Utc::now(),
            priority: UpdatePriority::Low,
        };

        let high = StateUpdate {
            entity_id: Uuid::new_v4(),
            changes: HashMap::new(),
            timestamp: Utc::now(),
            priority: UpdatePriority::High,
        };

        sync.queue_physical_update(low).unwrap();
        sync.queue_physical_update(high).unwrap();

        let first = sync.process_physical_update().unwrap();
        assert_eq!(first.priority, UpdatePriority::High);
    }

    #[test]
    fn test_queue_overflow() {
        let mut sync = BidirectionalSync::new(2);

        for i in 0..3 {
            let update = StateUpdate {
                entity_id: Uuid::new_v4(),
                changes: HashMap::new(),
                timestamp: Utc::now(),
                priority: UpdatePriority::Normal,
            };

            let result = sync.queue_physical_update(update);
            if i < 2 {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_process_updates() {
        let mut sync = BidirectionalSync::new(10);

        let update = StateUpdate {
            entity_id: Uuid::new_v4(),
            changes: HashMap::new(),
            timestamp: Utc::now(),
            priority: UpdatePriority::Normal,
        };

        sync.queue_physical_update(update).unwrap();

        let processed = sync.process_physical_update();
        assert!(processed.is_some());

        let none = sync.process_physical_update();
        assert!(none.is_none());
    }

    #[test]
    fn test_buffer_utilization() {
        let mut ingestion = IoTDataIngestion::new(100);

        for i in 0..50 {
            let data = SensorData::new(format!("sensor_{}", i), i as f64, "units".to_string());
            ingestion.ingest(data).unwrap();
        }

        assert_eq!(ingestion.buffer_utilization(), 0.5);
    }

    #[test]
    fn test_get_sensor_data() {
        let mut ingestion = IoTDataIngestion::new(100);

        for i in 0..10 {
            let data = SensorData::new("sensor_1".to_string(), i as f64, "units".to_string());
            ingestion.ingest(data).unwrap();
        }

        let recent = ingestion.get_sensor_data("sensor_1", 5);
        assert_eq!(recent.len(), 5);
    }
}
