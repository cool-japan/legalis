//! Real-Time Simulation - Streaming updates, live adjustment, and debugging.
//!
//! This module provides real-time simulation capabilities including:
//! - Streaming simulation updates
//! - Live parameter adjustment
//! - Real-time visualization integration
//! - Simulation pause/resume/rewind
//! - Breakpoint debugging

use crate::metrics::SimulationMetrics;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ============================================================================
// Streaming Simulation Updates
// ============================================================================

/// Update event from a running simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationUpdate {
    /// Simulation started.
    Started {
        /// Simulation identifier.
        sim_id: Uuid,
        /// Total number of time steps.
        total_steps: usize,
    },
    /// Progress update at a time step.
    Progress {
        /// Current time step.
        step: usize,
        /// Total steps.
        total: usize,
        /// Metrics at this step.
        metrics: SimulationMetrics,
        /// Timestamp of update.
        timestamp: u64,
    },
    /// Simulation paused.
    Paused {
        /// Time step when paused.
        at_step: usize,
    },
    /// Simulation resumed.
    Resumed {
        /// Time step when resumed.
        from_step: usize,
    },
    /// Simulation completed.
    Completed {
        /// Final metrics.
        final_metrics: SimulationMetrics,
        /// Total elapsed time (ms).
        elapsed_ms: u64,
    },
    /// Error occurred.
    Error {
        /// Error message.
        message: String,
        /// Time step when error occurred.
        at_step: usize,
    },
}

/// Stream of simulation updates.
#[derive(Debug, Clone)]
pub struct UpdateStream {
    /// Stream identifier.
    pub id: Uuid,
    /// Buffered updates.
    updates: Arc<Mutex<VecDeque<SimulationUpdate>>>,
    /// Maximum buffer size.
    pub max_buffer_size: usize,
}

impl UpdateStream {
    /// Creates a new update stream.
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            updates: Arc::new(Mutex::new(VecDeque::with_capacity(max_buffer_size))),
            max_buffer_size,
        }
    }

    /// Publishes an update to the stream.
    pub fn publish(&self, update: SimulationUpdate) {
        let mut updates = self.updates.lock().unwrap();
        if updates.len() >= self.max_buffer_size {
            updates.pop_front();
        }
        updates.push_back(update);
    }

    /// Retrieves all pending updates.
    pub fn consume_updates(&self) -> Vec<SimulationUpdate> {
        let mut updates = self.updates.lock().unwrap();
        updates.drain(..).collect()
    }

    /// Peeks at updates without consuming them.
    pub fn peek_updates(&self) -> Vec<SimulationUpdate> {
        let updates = self.updates.lock().unwrap();
        updates.iter().cloned().collect()
    }

    /// Gets the number of pending updates.
    pub fn pending_count(&self) -> usize {
        self.updates.lock().unwrap().len()
    }

    /// Clears all pending updates.
    pub fn clear(&self) {
        self.updates.lock().unwrap().clear();
    }
}

// ============================================================================
// Live Parameter Adjustment
// ============================================================================

/// Parameter that can be adjusted during simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveParameter {
    /// Parameter name.
    pub name: String,
    /// Current value.
    pub value: f64,
    /// Minimum allowed value.
    pub min: f64,
    /// Maximum allowed value.
    pub max: f64,
    /// Description.
    pub description: String,
}

impl LiveParameter {
    /// Creates a new live parameter.
    pub fn new(name: String, value: f64, min: f64, max: f64, description: String) -> Self {
        Self {
            name,
            value: value.clamp(min, max),
            min,
            max,
            description,
        }
    }

    /// Updates the parameter value with validation.
    pub fn set_value(&mut self, value: f64) -> Result<(), String> {
        if value < self.min || value > self.max {
            return Err(format!(
                "Value {} is out of range [{}, {}]",
                value, self.min, self.max
            ));
        }
        self.value = value;
        Ok(())
    }
}

/// Manager for live parameter adjustments.
#[derive(Debug, Clone)]
pub struct ParameterAdjuster {
    /// Adjuster identifier.
    pub id: Uuid,
    /// Parameters that can be adjusted.
    parameters: Arc<Mutex<HashMap<String, LiveParameter>>>,
    /// History of adjustments.
    history: Arc<Mutex<Vec<ParameterAdjustment>>>,
}

/// Record of a parameter adjustment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterAdjustment {
    /// Parameter name.
    pub parameter_name: String,
    /// Old value.
    pub old_value: f64,
    /// New value.
    pub new_value: f64,
    /// Time step when adjusted.
    pub at_step: usize,
    /// Timestamp.
    pub timestamp: u64,
}

impl ParameterAdjuster {
    /// Creates a new parameter adjuster.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            parameters: Arc::new(Mutex::new(HashMap::new())),
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers a parameter for live adjustment.
    pub fn register_parameter(&self, param: LiveParameter) {
        let mut params = self.parameters.lock().unwrap();
        params.insert(param.name.clone(), param);
    }

    /// Adjusts a parameter value.
    pub fn adjust(&self, name: &str, value: f64, at_step: usize) -> Result<(), String> {
        let mut params = self.parameters.lock().unwrap();

        let param = params
            .get_mut(name)
            .ok_or_else(|| format!("Parameter '{}' not found", name))?;

        let old_value = param.value;
        param.set_value(value)?;

        // Record adjustment
        let adjustment = ParameterAdjustment {
            parameter_name: name.to_string(),
            old_value,
            new_value: value,
            at_step,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut history = self.history.lock().unwrap();
        history.push(adjustment);

        Ok(())
    }

    /// Gets the current value of a parameter.
    pub fn get_value(&self, name: &str) -> Option<f64> {
        let params = self.parameters.lock().unwrap();
        params.get(name).map(|p| p.value)
    }

    /// Gets all parameters.
    pub fn get_all_parameters(&self) -> Vec<LiveParameter> {
        let params = self.parameters.lock().unwrap();
        params.values().cloned().collect()
    }

    /// Gets adjustment history.
    pub fn get_history(&self) -> Vec<ParameterAdjustment> {
        let history = self.history.lock().unwrap();
        history.clone()
    }

    /// Clears adjustment history.
    pub fn clear_history(&self) {
        let mut history = self.history.lock().unwrap();
        history.clear();
    }
}

impl Default for ParameterAdjuster {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Real-Time Visualization Integration
// ============================================================================

/// Visualization update hook.
pub type VisualizationHook = Arc<dyn Fn(&SimulationUpdate) + Send + Sync>;

/// Manager for real-time visualization integration.
#[derive(Clone)]
pub struct VisualizationIntegration {
    /// Integration identifier.
    pub id: Uuid,
    /// Registered hooks.
    hooks: Arc<Mutex<Vec<VisualizationHook>>>,
    /// Update throttle (minimum ms between updates).
    pub throttle_ms: u64,
    /// Last update timestamp.
    last_update: Arc<Mutex<u64>>,
}

impl VisualizationIntegration {
    /// Creates a new visualization integration.
    pub fn new(throttle_ms: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            hooks: Arc::new(Mutex::new(Vec::new())),
            throttle_ms,
            last_update: Arc::new(Mutex::new(0)),
        }
    }

    /// Registers a visualization hook.
    pub fn register_hook<F>(&self, hook: F)
    where
        F: Fn(&SimulationUpdate) + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.lock().unwrap();
        hooks.push(Arc::new(hook));
    }

    /// Notifies all hooks of an update.
    pub fn notify(&self, update: &SimulationUpdate) {
        // Check throttle
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut last = self.last_update.lock().unwrap();
        if now - *last < self.throttle_ms {
            return;
        }
        *last = now;

        // Notify hooks
        let hooks = self.hooks.lock().unwrap();
        for hook in hooks.iter() {
            hook(update);
        }
    }

    /// Gets the number of registered hooks.
    pub fn hook_count(&self) -> usize {
        self.hooks.lock().unwrap().len()
    }

    /// Clears all hooks.
    pub fn clear_hooks(&self) {
        self.hooks.lock().unwrap().clear();
    }
}

// ============================================================================
// Simulation Control (Pause/Resume/Rewind)
// ============================================================================

/// Simulation control state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealtimeSimulationState {
    /// Simulation is running.
    Running,
    /// Simulation is paused.
    Paused,
    /// Simulation is completed.
    Completed,
    /// Simulation encountered an error.
    Error,
}

/// Controller for simulation execution.
#[derive(Debug, Clone)]
pub struct SimulationController {
    /// Controller identifier.
    pub id: Uuid,
    /// Current state.
    state: Arc<Mutex<RealtimeSimulationState>>,
    /// Current time step.
    current_step: Arc<Mutex<usize>>,
    /// History of states (for rewind).
    state_history: Arc<Mutex<Vec<SimulationSnapshot>>>,
    /// Maximum history size.
    pub max_history: usize,
}

/// Snapshot of simulation state at a time step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSnapshot {
    /// Time step.
    pub step: usize,
    /// Metrics at this step.
    pub metrics: SimulationMetrics,
    /// Timestamp.
    pub timestamp: u64,
}

impl SimulationController {
    /// Creates a new simulation controller.
    pub fn new(max_history: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: Arc::new(Mutex::new(RealtimeSimulationState::Running)),
            current_step: Arc::new(Mutex::new(0)),
            state_history: Arc::new(Mutex::new(Vec::new())),
            max_history,
        }
    }

    /// Gets the current state.
    pub fn get_state(&self) -> RealtimeSimulationState {
        *self.state.lock().unwrap()
    }

    /// Pauses the simulation.
    pub fn pause(&self) {
        let mut state = self.state.lock().unwrap();
        if *state == RealtimeSimulationState::Running {
            *state = RealtimeSimulationState::Paused;
        }
    }

    /// Resumes the simulation.
    pub fn resume(&self) {
        let mut state = self.state.lock().unwrap();
        if *state == RealtimeSimulationState::Paused {
            *state = RealtimeSimulationState::Running;
        }
    }

    /// Checks if simulation should continue.
    pub fn should_continue(&self) -> bool {
        let state = self.state.lock().unwrap();
        *state == RealtimeSimulationState::Running
    }

    /// Marks simulation as completed.
    pub fn complete(&self) {
        let mut state = self.state.lock().unwrap();
        *state = RealtimeSimulationState::Completed;
    }

    /// Marks simulation as errored.
    pub fn error(&self) {
        let mut state = self.state.lock().unwrap();
        *state = RealtimeSimulationState::Error;
    }

    /// Records a snapshot at the current step.
    pub fn snapshot(&self, metrics: SimulationMetrics) {
        let step = *self.current_step.lock().unwrap();
        let snapshot = SimulationSnapshot {
            step,
            metrics,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut history = self.state_history.lock().unwrap();
        if history.len() >= self.max_history {
            history.remove(0);
        }
        history.push(snapshot);
    }

    /// Advances to the next step.
    pub fn advance_step(&self) {
        let mut step = self.current_step.lock().unwrap();
        *step += 1;
    }

    /// Gets the current step.
    pub fn get_current_step(&self) -> usize {
        *self.current_step.lock().unwrap()
    }

    /// Rewinds to a previous step.
    pub fn rewind_to(&self, target_step: usize) -> Option<SimulationSnapshot> {
        let history = self.state_history.lock().unwrap();

        // Find closest snapshot <= target_step
        let snapshot = history
            .iter()
            .filter(|s| s.step <= target_step)
            .max_by_key(|s| s.step)?;

        let mut current = self.current_step.lock().unwrap();
        *current = snapshot.step;

        Some(snapshot.clone())
    }

    /// Gets all snapshots in history.
    pub fn get_history(&self) -> Vec<SimulationSnapshot> {
        let history = self.state_history.lock().unwrap();
        history.clone()
    }

    /// Clears history.
    pub fn clear_history(&self) {
        let mut history = self.state_history.lock().unwrap();
        history.clear();
    }
}

// ============================================================================
// Breakpoint Debugging
// ============================================================================

/// Breakpoint condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakpointCondition {
    /// Break at a specific time step.
    AtStep(usize),
    /// Break when a metric exceeds a threshold.
    MetricThreshold {
        /// Metric name.
        metric: String,
        /// Comparison operator.
        operator: ComparisonOperator,
        /// Threshold value.
        threshold: f64,
    },
    /// Break when a condition is met.
    Custom {
        /// Condition description.
        description: String,
    },
}

/// Comparison operator for breakpoint conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Greater than.
    GreaterThan,
    /// Less than.
    LessThan,
    /// Equal to.
    EqualTo,
    /// Greater than or equal to.
    GreaterThanOrEqual,
    /// Less than or equal to.
    LessThanOrEqual,
}

impl ComparisonOperator {
    /// Evaluates the operator.
    pub fn evaluate(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            ComparisonOperator::GreaterThan => lhs > rhs,
            ComparisonOperator::LessThan => lhs < rhs,
            ComparisonOperator::EqualTo => (lhs - rhs).abs() < f64::EPSILON,
            ComparisonOperator::GreaterThanOrEqual => lhs >= rhs,
            ComparisonOperator::LessThanOrEqual => lhs <= rhs,
        }
    }
}

/// Breakpoint for debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    /// Breakpoint identifier.
    pub id: Uuid,
    /// Condition to trigger breakpoint.
    pub condition: BreakpointCondition,
    /// Whether breakpoint is enabled.
    pub enabled: bool,
    /// Number of times hit.
    pub hit_count: usize,
}

impl Breakpoint {
    /// Creates a new breakpoint.
    pub fn new(condition: BreakpointCondition) -> Self {
        Self {
            id: Uuid::new_v4(),
            condition,
            enabled: true,
            hit_count: 0,
        }
    }

    /// Checks if the breakpoint should trigger.
    pub fn should_trigger(&self, step: usize, metrics: &SimulationMetrics) -> bool {
        if !self.enabled {
            return false;
        }

        match &self.condition {
            BreakpointCondition::AtStep(target_step) => step == *target_step,
            BreakpointCondition::MetricThreshold {
                metric,
                operator,
                threshold,
            } => {
                // Simple metric check (would need actual metric extraction in real use)
                if metric == "total_applications" {
                    let value = metrics.total_applications as f64;
                    operator.evaluate(value, *threshold)
                } else {
                    false
                }
            }
            BreakpointCondition::Custom { .. } => {
                // Custom conditions would need to be evaluated externally
                false
            }
        }
    }
}

/// Debugger for simulations.
#[derive(Debug, Clone)]
pub struct SimulationDebugger {
    /// Debugger identifier.
    pub id: Uuid,
    /// Active breakpoints.
    breakpoints: Arc<Mutex<HashMap<Uuid, Breakpoint>>>,
    /// Breakpoint hits.
    hits: Arc<Mutex<Vec<BreakpointHit>>>,
}

/// Record of a breakpoint being hit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointHit {
    /// Breakpoint ID.
    pub breakpoint_id: Uuid,
    /// Time step when hit.
    pub at_step: usize,
    /// Metrics at hit.
    pub metrics: SimulationMetrics,
    /// Timestamp.
    pub timestamp: u64,
}

impl SimulationDebugger {
    /// Creates a new simulation debugger.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            breakpoints: Arc::new(Mutex::new(HashMap::new())),
            hits: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a breakpoint.
    pub fn add_breakpoint(&self, breakpoint: Breakpoint) -> Uuid {
        let id = breakpoint.id;
        let mut breakpoints = self.breakpoints.lock().unwrap();
        breakpoints.insert(id, breakpoint);
        id
    }

    /// Removes a breakpoint.
    pub fn remove_breakpoint(&self, id: &Uuid) -> bool {
        let mut breakpoints = self.breakpoints.lock().unwrap();
        breakpoints.remove(id).is_some()
    }

    /// Enables a breakpoint.
    pub fn enable_breakpoint(&self, id: &Uuid) {
        let mut breakpoints = self.breakpoints.lock().unwrap();
        if let Some(bp) = breakpoints.get_mut(id) {
            bp.enabled = true;
        }
    }

    /// Disables a breakpoint.
    pub fn disable_breakpoint(&self, id: &Uuid) {
        let mut breakpoints = self.breakpoints.lock().unwrap();
        if let Some(bp) = breakpoints.get_mut(id) {
            bp.enabled = false;
        }
    }

    /// Checks if any breakpoint should trigger.
    pub fn check_breakpoints(&self, step: usize, metrics: &SimulationMetrics) -> Option<Uuid> {
        let mut breakpoints = self.breakpoints.lock().unwrap();

        for (id, bp) in breakpoints.iter_mut() {
            if bp.should_trigger(step, metrics) {
                bp.hit_count += 1;

                // Record hit
                let hit = BreakpointHit {
                    breakpoint_id: *id,
                    at_step: step,
                    metrics: metrics.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                let mut hits = self.hits.lock().unwrap();
                hits.push(hit);

                return Some(*id);
            }
        }

        None
    }

    /// Gets all breakpoints.
    pub fn get_breakpoints(&self) -> Vec<Breakpoint> {
        let breakpoints = self.breakpoints.lock().unwrap();
        breakpoints.values().cloned().collect()
    }

    /// Gets breakpoint hit history.
    pub fn get_hits(&self) -> Vec<BreakpointHit> {
        let hits = self.hits.lock().unwrap();
        hits.clone()
    }

    /// Clears hit history.
    pub fn clear_hits(&self) {
        let mut hits = self.hits.lock().unwrap();
        hits.clear();
    }
}

impl Default for SimulationDebugger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_stream() {
        let stream = UpdateStream::new(10);

        stream.publish(SimulationUpdate::Started {
            sim_id: Uuid::new_v4(),
            total_steps: 100,
        });

        assert_eq!(stream.pending_count(), 1);

        let updates = stream.consume_updates();
        assert_eq!(updates.len(), 1);
        assert_eq!(stream.pending_count(), 0);
    }

    #[test]
    fn test_update_stream_overflow() {
        let stream = UpdateStream::new(3);

        for i in 0..5 {
            stream.publish(SimulationUpdate::Progress {
                step: i,
                total: 10,
                metrics: SimulationMetrics::default(),
                timestamp: i as u64,
            });
        }

        // Should only keep last 3
        assert_eq!(stream.pending_count(), 3);
    }

    #[test]
    fn test_live_parameter() {
        let mut param = LiveParameter::new(
            "learning_rate".to_string(),
            0.01,
            0.0,
            1.0,
            "Learning rate for agent".to_string(),
        );

        assert!(param.set_value(0.5).is_ok());
        assert_eq!(param.value, 0.5);

        assert!(param.set_value(1.5).is_err());
        assert_eq!(param.value, 0.5); // Unchanged
    }

    #[test]
    fn test_parameter_adjuster() {
        let adjuster = ParameterAdjuster::new();

        let param = LiveParameter::new(
            "discount_factor".to_string(),
            0.99,
            0.0,
            1.0,
            "Discount factor".to_string(),
        );

        adjuster.register_parameter(param);

        assert!(adjuster.adjust("discount_factor", 0.95, 0).is_ok());
        assert_eq!(adjuster.get_value("discount_factor"), Some(0.95));

        let history = adjuster.get_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].old_value, 0.99);
        assert_eq!(history[0].new_value, 0.95);
    }

    #[test]
    fn test_visualization_integration() {
        let viz = VisualizationIntegration::new(0);

        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        viz.register_hook(move |_update| {
            *called_clone.lock().unwrap() = true;
        });

        assert_eq!(viz.hook_count(), 1);

        viz.notify(&SimulationUpdate::Started {
            sim_id: Uuid::new_v4(),
            total_steps: 10,
        });

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_simulation_controller() {
        let controller = SimulationController::new(100);

        assert_eq!(controller.get_state(), RealtimeSimulationState::Running);
        assert!(controller.should_continue());

        controller.pause();
        assert_eq!(controller.get_state(), RealtimeSimulationState::Paused);
        assert!(!controller.should_continue());

        controller.resume();
        assert_eq!(controller.get_state(), RealtimeSimulationState::Running);

        controller.complete();
        assert_eq!(controller.get_state(), RealtimeSimulationState::Completed);
    }

    #[test]
    fn test_simulation_snapshot() {
        let controller = SimulationController::new(10);

        controller.snapshot(SimulationMetrics::default());
        controller.advance_step();
        controller.snapshot(SimulationMetrics::default());

        let history = controller.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].step, 0);
        assert_eq!(history[1].step, 1);
    }

    #[test]
    fn test_simulation_rewind() {
        let controller = SimulationController::new(10);

        for _ in 0..5 {
            controller.snapshot(SimulationMetrics::default());
            controller.advance_step();
        }

        let snapshot = controller.rewind_to(2);
        assert!(snapshot.is_some());
        assert_eq!(snapshot.unwrap().step, 2);
        assert_eq!(controller.get_current_step(), 2);
    }

    #[test]
    fn test_breakpoint_at_step() {
        let bp = Breakpoint::new(BreakpointCondition::AtStep(10));

        assert!(!bp.should_trigger(5, &SimulationMetrics::default()));
        assert!(bp.should_trigger(10, &SimulationMetrics::default()));
    }

    #[test]
    fn test_breakpoint_disabled() {
        let mut bp = Breakpoint::new(BreakpointCondition::AtStep(10));
        bp.enabled = false;

        assert!(!bp.should_trigger(10, &SimulationMetrics::default()));
    }

    #[test]
    fn test_comparison_operators() {
        assert!(ComparisonOperator::GreaterThan.evaluate(5.0, 3.0));
        assert!(ComparisonOperator::LessThan.evaluate(3.0, 5.0));
        assert!(ComparisonOperator::EqualTo.evaluate(5.0, 5.0));
        assert!(ComparisonOperator::GreaterThanOrEqual.evaluate(5.0, 5.0));
        assert!(ComparisonOperator::LessThanOrEqual.evaluate(3.0, 5.0));
    }

    #[test]
    fn test_simulation_debugger() {
        let debugger = SimulationDebugger::new();

        let bp = Breakpoint::new(BreakpointCondition::AtStep(5));
        let bp_id = debugger.add_breakpoint(bp);

        let metrics = SimulationMetrics::default();

        assert!(debugger.check_breakpoints(3, &metrics).is_none());
        assert_eq!(debugger.check_breakpoints(5, &metrics), Some(bp_id));

        let hits = debugger.get_hits();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].at_step, 5);
    }

    #[test]
    fn test_debugger_enable_disable() {
        let debugger = SimulationDebugger::new();

        let bp = Breakpoint::new(BreakpointCondition::AtStep(5));
        let bp_id = debugger.add_breakpoint(bp);

        debugger.disable_breakpoint(&bp_id);

        let metrics = SimulationMetrics::default();
        assert!(debugger.check_breakpoints(5, &metrics).is_none());

        debugger.enable_breakpoint(&bp_id);
        assert_eq!(debugger.check_breakpoints(5, &metrics), Some(bp_id));
    }

    #[test]
    fn test_debugger_remove_breakpoint() {
        let debugger = SimulationDebugger::new();

        let bp = Breakpoint::new(BreakpointCondition::AtStep(5));
        let bp_id = debugger.add_breakpoint(bp);

        assert!(debugger.remove_breakpoint(&bp_id));
        assert!(!debugger.remove_breakpoint(&bp_id)); // Already removed
    }
}
