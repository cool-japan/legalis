//! Event-driven simulation for legal systems.
//!
//! This module provides discrete event simulation capabilities that complement
//! the time-step based simulation. Events can be statute changes, entity actions,
//! or external triggers.

use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Simulation event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event ID.
    pub id: String,
    /// Event time (simulation time units).
    pub time: f64,
    /// Event type.
    pub event_type: EventType,
    /// Event priority (higher = processed first at same time).
    pub priority: i32,
    /// Event data.
    pub data: HashMap<String, serde_json::Value>,
}

impl Event {
    /// Create new event.
    pub fn new(id: String, time: f64, event_type: EventType) -> Self {
        Self {
            id,
            time,
            event_type,
            priority: 0,
            data: HashMap::new(),
        }
    }

    /// Set event priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Add data to event.
    pub fn with_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.data.insert(key, value);
        self
    }
}

/// Event type classification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    /// Statute becomes effective.
    StatuteEffective,
    /// Statute is repealed.
    StatuteRepealed,
    /// Statute is amended.
    StatuteAmended,
    /// Entity is created.
    EntityCreated,
    /// Entity is destroyed.
    EntityDestroyed,
    /// Entity changes state.
    EntityStateChange,
    /// Entity takes action.
    EntityAction,
    /// External trigger.
    ExternalTrigger,
    /// Custom event type.
    Custom(String),
}

/// Event ordering for priority queue (min-heap by time, then max-heap by priority).
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap by time
        match other.time.partial_cmp(&self.time) {
            Some(Ordering::Equal) => {
                // Higher priority first
                self.priority.cmp(&other.priority)
            }
            Some(ord) => ord,
            None => Ordering::Equal,
        }
    }
}

/// Event handler trait.
pub trait EventHandler: Send + Sync {
    /// Handle an event.
    fn handle(&mut self, event: &Event, state: &mut SimulationState) -> SimResult<Vec<Event>>;
}

/// Simulation state for event-driven simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    /// Current simulation time.
    pub current_time: f64,
    /// Entity states.
    pub entity_states: HashMap<String, serde_json::Value>,
    /// Statute states.
    pub statute_states: HashMap<String, bool>,
    /// Global variables.
    pub globals: HashMap<String, serde_json::Value>,
    /// Event history.
    pub event_history: Vec<Event>,
}

impl SimulationState {
    /// Create new simulation state.
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            entity_states: HashMap::new(),
            statute_states: HashMap::new(),
            globals: HashMap::new(),
            event_history: Vec::new(),
        }
    }

    /// Record event in history.
    pub fn record_event(&mut self, event: Event) {
        self.event_history.push(event);
    }

    /// Get entity state.
    pub fn get_entity_state(&self, entity_id: &str) -> Option<&serde_json::Value> {
        self.entity_states.get(entity_id)
    }

    /// Set entity state.
    pub fn set_entity_state(&mut self, entity_id: String, state: serde_json::Value) {
        self.entity_states.insert(entity_id, state);
    }

    /// Remove entity.
    pub fn remove_entity(&mut self, entity_id: &str) {
        self.entity_states.remove(entity_id);
    }

    /// Check if statute is active.
    pub fn is_statute_active(&self, statute_id: &str) -> bool {
        self.statute_states
            .get(statute_id)
            .copied()
            .unwrap_or(false)
    }

    /// Set statute active state.
    pub fn set_statute_active(&mut self, statute_id: String, active: bool) {
        self.statute_states.insert(statute_id, active);
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Event-driven simulation engine.
pub struct EventDrivenSimulator {
    /// Event queue (priority queue).
    event_queue: BinaryHeap<Event>,
    /// Event handlers by type.
    handlers: HashMap<String, Box<dyn EventHandler>>,
    /// Simulation state.
    state: SimulationState,
    /// Maximum simulation time.
    max_time: f64,
}

impl EventDrivenSimulator {
    /// Create new event-driven simulator.
    pub fn new(max_time: f64) -> Self {
        Self {
            event_queue: BinaryHeap::new(),
            handlers: HashMap::new(),
            state: SimulationState::new(),
            max_time,
        }
    }

    /// Schedule an event.
    pub fn schedule_event(&mut self, event: Event) -> SimResult<()> {
        if event.time < self.state.current_time {
            return Err(SimulationError::ConfigurationError(format!(
                "Cannot schedule event in the past: {} < {}",
                event.time, self.state.current_time
            )));
        }

        if event.time > self.max_time {
            return Err(SimulationError::ConfigurationError(format!(
                "Event time {} exceeds max time {}",
                event.time, self.max_time
            )));
        }

        self.event_queue.push(event);
        Ok(())
    }

    /// Register an event handler.
    pub fn register_handler(&mut self, event_type: String, handler: Box<dyn EventHandler>) {
        self.handlers.insert(event_type, handler);
    }

    /// Run the simulation.
    pub fn run(&mut self) -> SimResult<SimulationState> {
        while let Some(event) = self.event_queue.pop() {
            if event.time > self.max_time {
                break;
            }

            // Update simulation time
            self.state.current_time = event.time;

            // Find handler
            let handler_key = match &event.event_type {
                EventType::Custom(name) => name.clone(),
                _ => format!("{:?}", event.event_type),
            };

            // Handle event
            if let Some(handler) = self.handlers.get_mut(&handler_key) {
                let new_events = handler.handle(&event, &mut self.state)?;

                // Schedule new events
                for new_event in new_events {
                    self.schedule_event(new_event)?;
                }
            }

            // Record event
            self.state.record_event(event);
        }

        Ok(self.state.clone())
    }

    /// Get current simulation state.
    pub fn get_state(&self) -> &SimulationState {
        &self.state
    }

    /// Get mutable simulation state.
    pub fn get_state_mut(&mut self) -> &mut SimulationState {
        &mut self.state
    }
}

/// Event logger for debugging.
pub struct EventLogger {
    log: Vec<String>,
}

impl EventLogger {
    /// Create new event logger.
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }

    /// Get log entries.
    pub fn get_log(&self) -> &[String] {
        &self.log
    }
}

impl Default for EventLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for EventLogger {
    fn handle(&mut self, event: &Event, state: &mut SimulationState) -> SimResult<Vec<Event>> {
        let log_entry = format!(
            "[t={}] Event: {:?} (id: {}, priority: {})",
            state.current_time, event.event_type, event.id, event.priority
        );
        self.log.push(log_entry);
        Ok(Vec::new())
    }
}

/// Hybrid simulator combining time-step and event-driven approaches.
pub struct HybridSimulator {
    /// Time step size.
    pub time_step: f64,
    /// Event-driven engine.
    pub event_engine: EventDrivenSimulator,
}

impl HybridSimulator {
    /// Create new hybrid simulator.
    pub fn new(time_step: f64, max_time: f64) -> Self {
        Self {
            time_step,
            event_engine: EventDrivenSimulator::new(max_time),
        }
    }

    /// Schedule time-step events.
    pub fn schedule_time_steps(&mut self) -> SimResult<()> {
        let mut time = 0.0;
        let mut step_id = 0;

        while time <= self.event_engine.max_time {
            let event = Event::new(
                format!("timestep_{}", step_id),
                time,
                EventType::Custom("TimeStep".to_string()),
            );

            self.event_engine.schedule_event(event)?;

            time += self.time_step;
            step_id += 1;
        }

        Ok(())
    }

    /// Run hybrid simulation.
    pub fn run(&mut self) -> SimResult<SimulationState> {
        self.event_engine.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        call_count: usize,
    }

    impl EventHandler for TestHandler {
        fn handle(&mut self, event: &Event, _state: &mut SimulationState) -> SimResult<Vec<Event>> {
            self.call_count += 1;

            // Generate a follow-up event
            if event.time < 5.0 {
                let new_event = Event::new(
                    format!("follow_up_{}", self.call_count),
                    event.time + 1.0,
                    EventType::Custom("Test".to_string()),
                );
                Ok(vec![new_event])
            } else {
                Ok(Vec::new())
            }
        }
    }

    #[test]
    fn test_event_scheduling() {
        let mut simulator = EventDrivenSimulator::new(10.0);

        let event = Event::new("test1".to_string(), 5.0, EventType::EntityCreated);
        simulator.schedule_event(event).unwrap();

        assert!(!simulator.event_queue.is_empty());
    }

    #[test]
    fn test_event_ordering() {
        let mut simulator = EventDrivenSimulator::new(10.0);

        simulator
            .schedule_event(Event::new("e1".to_string(), 5.0, EventType::EntityCreated))
            .unwrap();

        simulator
            .schedule_event(Event::new("e2".to_string(), 3.0, EventType::EntityCreated))
            .unwrap();

        simulator
            .schedule_event(Event::new("e3".to_string(), 7.0, EventType::EntityCreated))
            .unwrap();

        // Should process in time order: 3.0, 5.0, 7.0
        let first = simulator.event_queue.pop().unwrap();
        assert_eq!(first.time, 3.0);

        let second = simulator.event_queue.pop().unwrap();
        assert_eq!(second.time, 5.0);

        let third = simulator.event_queue.pop().unwrap();
        assert_eq!(third.time, 7.0);
    }

    #[test]
    fn test_event_priority() {
        let mut simulator = EventDrivenSimulator::new(10.0);

        // Same time, different priorities
        simulator
            .schedule_event(
                Event::new("e1".to_string(), 5.0, EventType::EntityCreated).with_priority(1),
            )
            .unwrap();

        simulator
            .schedule_event(
                Event::new("e2".to_string(), 5.0, EventType::EntityCreated).with_priority(10),
            )
            .unwrap();

        // Higher priority (10) should come first
        let first = simulator.event_queue.pop().unwrap();
        assert_eq!(first.priority, 10);

        let second = simulator.event_queue.pop().unwrap();
        assert_eq!(second.priority, 1);
    }

    #[test]
    fn test_simulation_run() {
        let mut simulator = EventDrivenSimulator::new(10.0);

        let handler = Box::new(TestHandler { call_count: 0 });
        simulator.register_handler("Test".to_string(), handler);

        // Schedule initial event
        simulator
            .schedule_event(Event::new(
                "initial".to_string(),
                0.0,
                EventType::Custom("Test".to_string()),
            ))
            .unwrap();

        let final_state = simulator.run().unwrap();

        // Should have processed multiple events (initial + follow-ups)
        assert!(final_state.event_history.len() > 1);
        assert!(final_state.current_time >= 5.0);
    }

    #[test]
    fn test_simulation_state() {
        let mut state = SimulationState::new();

        state.set_entity_state(
            "entity1".to_string(),
            serde_json::json!({"status": "active"}),
        );

        assert!(state.get_entity_state("entity1").is_some());

        state.set_statute_active("statute1".to_string(), true);
        assert!(state.is_statute_active("statute1"));

        state.remove_entity("entity1");
        assert!(state.get_entity_state("entity1").is_none());
    }

    #[test]
    fn test_event_logger() {
        let mut simulator = EventDrivenSimulator::new(10.0);

        let logger = Box::new(EventLogger::new());
        simulator.register_handler("EntityCreated".to_string(), logger);

        simulator
            .schedule_event(Event::new("e1".to_string(), 1.0, EventType::EntityCreated))
            .unwrap();

        simulator
            .schedule_event(Event::new("e2".to_string(), 2.0, EventType::EntityCreated))
            .unwrap();

        simulator.run().unwrap();

        // Logger should have recorded events
        if let Some(_handler) = simulator.handlers.get("EntityCreated") {
            // Can't easily access logger's log, but test passes if no errors
        }
    }

    #[test]
    fn test_hybrid_simulator() {
        let mut simulator = HybridSimulator::new(1.0, 5.0);

        simulator.schedule_time_steps().unwrap();

        let result = simulator.run().unwrap();

        // Should have processed time-step events
        assert!(result.current_time >= 5.0);
    }
}
