//! Temporal simulation module for time-step based legal simulations.
//!
//! This module provides:
//! - Time-step based simulation with configurable intervals
//! - Statute effective date handling
//! - Agent lifecycle management (birth, death, status changes)
//! - Temporal metrics collection
//! - Event-driven state transitions

use crate::engine::{LawApplicationResult, SimEngine};
use crate::metrics::SimulationMetrics;
use chrono::{Duration, NaiveDate};
use legalis_core::{BasicEntity, LegalEntity, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Time step granularity for simulations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeStep {
    /// Daily steps
    Day,
    /// Weekly steps
    Week,
    /// Monthly steps
    Month,
    /// Quarterly steps
    Quarter,
    /// Yearly steps
    Year,
}

impl TimeStep {
    /// Returns the duration of this time step.
    pub fn to_duration(&self) -> Duration {
        match self {
            TimeStep::Day => Duration::days(1),
            TimeStep::Week => Duration::weeks(1),
            TimeStep::Month => Duration::days(30), // Approximation
            TimeStep::Quarter => Duration::days(91),
            TimeStep::Year => Duration::days(365),
        }
    }

    /// Returns the number of steps between two dates.
    pub fn steps_between(&self, start: NaiveDate, end: NaiveDate) -> i64 {
        let days = (end - start).num_days();
        match self {
            TimeStep::Day => days,
            TimeStep::Week => days / 7,
            TimeStep::Month => days / 30,
            TimeStep::Quarter => days / 91,
            TimeStep::Year => days / 365,
        }
    }
}

/// Configuration for temporal simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConfig {
    /// Start date of simulation
    pub start_date: NaiveDate,
    /// End date of simulation
    pub end_date: NaiveDate,
    /// Time step granularity
    pub time_step: TimeStep,
    /// Whether to track agent state history
    pub track_history: bool,
    /// Maximum parallel tasks
    pub max_parallelism: usize,
}

impl Default for TemporalConfig {
    fn default() -> Self {
        Self {
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            time_step: TimeStep::Month,
            track_history: true,
            max_parallelism: num_cpus::get(),
        }
    }
}

impl TemporalConfig {
    /// Creates a new temporal configuration.
    pub fn new(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            start_date,
            end_date,
            ..Default::default()
        }
    }

    /// Sets the time step.
    pub fn with_time_step(mut self, step: TimeStep) -> Self {
        self.time_step = step;
        self
    }

    /// Sets whether to track history.
    pub fn with_history(mut self, track: bool) -> Self {
        self.track_history = track;
        self
    }

    /// Returns the total number of time steps.
    pub fn total_steps(&self) -> i64 {
        self.time_step.steps_between(self.start_date, self.end_date)
    }
}

/// An event that occurs during temporal simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalEvent {
    /// Agent enters the simulation
    AgentBirth { agent_id: Uuid, date: NaiveDate },
    /// Agent exits the simulation
    AgentDeath { agent_id: Uuid, date: NaiveDate },
    /// Agent attribute changes
    AttributeChange {
        agent_id: Uuid,
        date: NaiveDate,
        attribute: String,
        old_value: Option<String>,
        new_value: String,
    },
    /// Statute becomes effective
    StatuteEffective { statute_id: String, date: NaiveDate },
    /// Statute expires
    StatuteExpired { statute_id: String, date: NaiveDate },
    /// Statute amended
    StatuteAmended {
        statute_id: String,
        date: NaiveDate,
        changes: Vec<String>,
    },
}

/// Temporal state of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Agent ID
    pub id: Uuid,
    /// Whether the agent is active
    pub active: bool,
    /// Birth date (when agent entered simulation)
    pub birth_date: Option<NaiveDate>,
    /// Death date (when agent exited simulation)
    pub death_date: Option<NaiveDate>,
    /// Current attributes
    pub attributes: HashMap<String, String>,
    /// History of attribute changes (date -> attribute -> value)
    pub history: Vec<(NaiveDate, String, String)>,
}

impl AgentState {
    /// Creates a new agent state.
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            active: true,
            birth_date: None,
            death_date: None,
            attributes: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Creates from an existing entity.
    pub fn from_entity(entity: &dyn LegalEntity, birth_date: NaiveDate) -> Self {
        let mut state = Self::new(entity.id());
        state.birth_date = Some(birth_date);
        state
    }

    /// Sets an attribute with history tracking.
    pub fn set_attribute(&mut self, key: &str, value: String, date: NaiveDate) {
        self.history.push((date, key.to_string(), value.clone()));
        self.attributes.insert(key.to_string(), value);
    }

    /// Gets the attribute value at a specific date.
    pub fn get_attribute_at(&self, key: &str, date: NaiveDate) -> Option<&str> {
        // Find the most recent value before or at the given date
        let mut result: Option<&str> = None;
        for (event_date, attr_key, value) in &self.history {
            if attr_key == key && *event_date <= date {
                result = Some(value.as_str());
            }
        }
        result
    }

    /// Checks if agent is active at a given date.
    pub fn is_active_at(&self, date: NaiveDate) -> bool {
        let after_birth = self.birth_date.is_none_or(|bd| date >= bd);
        let before_death = self.death_date.is_none_or(|dd| date < dd);
        self.active && after_birth && before_death
    }
}

/// Temporal statute wrapper with effective dates.
#[derive(Debug, Clone)]
pub struct TemporalStatute {
    /// The underlying statute
    pub statute: Statute,
    /// Effective date (when statute becomes active)
    pub effective_date: Option<NaiveDate>,
    /// Expiry date (when statute becomes inactive)
    pub expiry_date: Option<NaiveDate>,
    /// Amendment history
    pub amendments: Vec<(NaiveDate, Statute)>,
}

impl TemporalStatute {
    /// Creates a new temporal statute.
    pub fn new(statute: Statute) -> Self {
        Self {
            statute,
            effective_date: None,
            expiry_date: None,
            amendments: Vec::new(),
        }
    }

    /// Sets the effective date.
    pub fn with_effective_date(mut self, date: NaiveDate) -> Self {
        self.effective_date = Some(date);
        self
    }

    /// Sets the expiry date.
    pub fn with_expiry_date(mut self, date: NaiveDate) -> Self {
        self.expiry_date = Some(date);
        self
    }

    /// Adds an amendment.
    pub fn with_amendment(mut self, date: NaiveDate, amended: Statute) -> Self {
        self.amendments.push((date, amended));
        // Sort amendments by date
        self.amendments.sort_by_key(|(d, _)| *d);
        self
    }

    /// Checks if the statute is effective at a given date.
    pub fn is_effective_at(&self, date: NaiveDate) -> bool {
        let after_effective = self.effective_date.is_none_or(|ed| date >= ed);
        let before_expiry = self.expiry_date.is_none_or(|xd| date < xd);
        after_effective && before_expiry
    }

    /// Gets the version of the statute at a given date.
    pub fn version_at(&self, date: NaiveDate) -> &Statute {
        // Find the most recent amendment before or at the given date
        let mut current = &self.statute;
        for (amendment_date, amended) in &self.amendments {
            if *amendment_date <= date {
                current = amended;
            } else {
                break;
            }
        }
        current
    }
}

/// Snapshot of simulation state at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSnapshot {
    /// The date of this snapshot
    pub date: NaiveDate,
    /// Metrics at this time point
    pub metrics: SimulationMetrics,
    /// Active agent count
    pub active_agents: usize,
    /// Active statute count
    pub active_statutes: usize,
    /// Events that occurred at this time
    pub events: Vec<TemporalEvent>,
}

/// Temporal simulation metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemporalMetrics {
    /// Per-timestep snapshots
    pub snapshots: Vec<TimeSnapshot>,
    /// Cumulative metrics over entire simulation
    pub cumulative: SimulationMetrics,
    /// Event log
    pub events: Vec<TemporalEvent>,
    /// Per-statute effectiveness over time
    pub statute_trends: HashMap<String, Vec<(NaiveDate, f64)>>,
}

impl TemporalMetrics {
    /// Creates new temporal metrics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a snapshot.
    pub fn add_snapshot(&mut self, snapshot: TimeSnapshot) {
        // Record statute trends
        for (statute_id, statute_metrics) in &snapshot.metrics.statute_metrics {
            self.statute_trends
                .entry(statute_id.clone())
                .or_default()
                .push((snapshot.date, statute_metrics.effectiveness()));
        }

        self.snapshots.push(snapshot);
    }

    /// Records an event.
    pub fn record_event(&mut self, event: TemporalEvent) {
        self.events.push(event);
    }

    /// Generates a time-series report.
    pub fn time_series_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Temporal Simulation Report ===\n\n");

        report.push_str("Timeline:\n");
        for snapshot in &self.snapshots {
            report.push_str(&format!(
                "  {}: {} agents, {} statutes, D={:.1}%\n",
                snapshot.date,
                snapshot.active_agents,
                snapshot.active_statutes,
                snapshot.metrics.deterministic_ratio() * 100.0
            ));
        }

        report.push_str("\nEvents:\n");
        for event in &self.events {
            report.push_str(&format!("  {:?}\n", event));
        }

        report.push_str("\nStatute Trends:\n");
        for (statute_id, trends) in &self.statute_trends {
            if let (Some(first), Some(last)) = (trends.first(), trends.last()) {
                let delta = last.1 - first.1;
                let trend_indicator = if delta > 0.05 {
                    "↑"
                } else if delta < -0.05 {
                    "↓"
                } else {
                    "→"
                };
                report.push_str(&format!(
                    "  {}: {:.1}% -> {:.1}% {}\n",
                    statute_id,
                    first.1 * 100.0,
                    last.1 * 100.0,
                    trend_indicator
                ));
            }
        }

        report
    }
}

/// Temporal simulation engine.
pub struct TemporalSimEngine {
    /// Configuration
    config: TemporalConfig,
    /// Temporal statutes
    statutes: Vec<TemporalStatute>,
    /// Agent states
    agents: Arc<RwLock<HashMap<Uuid, AgentState>>>,
    /// Scheduled events
    scheduled_events: Vec<(NaiveDate, TemporalEvent)>,
}

impl TemporalSimEngine {
    /// Creates a new temporal simulation engine.
    pub fn new(config: TemporalConfig) -> Self {
        Self {
            config,
            statutes: Vec::new(),
            agents: Arc::new(RwLock::new(HashMap::new())),
            scheduled_events: Vec::new(),
        }
    }

    /// Adds a temporal statute.
    pub fn add_statute(&mut self, statute: TemporalStatute) {
        self.statutes.push(statute);
    }

    /// Adds multiple temporal statutes.
    pub fn add_statutes(&mut self, statutes: impl IntoIterator<Item = TemporalStatute>) {
        self.statutes.extend(statutes);
    }

    /// Adds an agent from an entity.
    pub async fn add_agent(&mut self, entity: &dyn LegalEntity) {
        let state = AgentState::from_entity(entity, self.config.start_date);
        self.agents.write().await.insert(entity.id(), state);
    }

    /// Adds a population of entities.
    pub async fn add_population(&mut self, population: Vec<Box<dyn LegalEntity>>) {
        let mut agents = self.agents.write().await;
        for entity in population {
            let mut state = AgentState::from_entity(entity.as_ref(), self.config.start_date);
            // Copy initial attributes
            state.attributes = entity
                .id()
                .to_string()
                .chars()
                .take(0)
                .map(|_| (String::new(), String::new()))
                .collect();
            agents.insert(entity.id(), state);
        }
    }

    /// Adds a population with initial attributes.
    pub async fn add_population_with_attributes(&mut self, population: &[BasicEntity]) {
        let mut agents = self.agents.write().await;
        for entity in population {
            let mut state = AgentState::new(entity.id());
            state.birth_date = Some(self.config.start_date);
            // Copy attributes from entity
            if let Some(age) = entity.get_attribute("age") {
                state.set_attribute("age", age.clone(), self.config.start_date);
            }
            if let Some(income) = entity.get_attribute("income") {
                state.set_attribute("income", income.clone(), self.config.start_date);
            }
            agents.insert(entity.id(), state);
        }
    }

    /// Schedules an event for a future date.
    pub fn schedule_event(&mut self, date: NaiveDate, event: TemporalEvent) {
        self.scheduled_events.push((date, event));
        self.scheduled_events.sort_by_key(|(d, _)| *d);
    }

    /// Runs the temporal simulation.
    pub async fn run(&mut self) -> TemporalMetrics {
        let mut metrics = TemporalMetrics::new();
        let mut current_date = self.config.start_date;

        while current_date <= self.config.end_date {
            // Process scheduled events for this date
            let events_today = self.process_events(current_date).await;
            for event in &events_today {
                metrics.record_event(event.clone());
            }

            // Get active statutes for this date
            let active_statutes: Vec<&Statute> = self
                .statutes
                .iter()
                .filter(|s| s.is_effective_at(current_date))
                .map(|s| s.version_at(current_date))
                .collect();

            // Get active agents for this date
            let agents_read = self.agents.read().await;
            let active_agents: Vec<&AgentState> = agents_read
                .values()
                .filter(|a| a.is_active_at(current_date))
                .collect();

            // Run simulation for this time step
            let step_metrics = self.simulate_step(current_date, &active_statutes, &active_agents);

            // Create snapshot
            let snapshot = TimeSnapshot {
                date: current_date,
                metrics: step_metrics.clone(),
                active_agents: active_agents.len(),
                active_statutes: active_statutes.len(),
                events: events_today,
            };

            // Merge into cumulative metrics
            metrics.cumulative.total_applications += step_metrics.total_applications;
            metrics.cumulative.deterministic_count += step_metrics.deterministic_count;
            metrics.cumulative.discretion_count += step_metrics.discretion_count;
            metrics.cumulative.void_count += step_metrics.void_count;

            for (statute_id, statute_metrics) in step_metrics.statute_metrics {
                let cumulative = metrics
                    .cumulative
                    .statute_metrics
                    .entry(statute_id)
                    .or_default();
                cumulative.total += statute_metrics.total;
                cumulative.deterministic += statute_metrics.deterministic;
                cumulative.discretion += statute_metrics.discretion;
                cumulative.void += statute_metrics.void;
            }

            metrics.add_snapshot(snapshot);

            // Advance time
            current_date += self.config.time_step.to_duration();
        }

        metrics
    }

    /// Processes events for a specific date.
    async fn process_events(&mut self, date: NaiveDate) -> Vec<TemporalEvent> {
        let mut processed = Vec::new();
        let mut i = 0;

        while i < self.scheduled_events.len() {
            if self.scheduled_events[i].0 <= date {
                let (_, event) = self.scheduled_events.remove(i);
                self.apply_event(&event).await;
                processed.push(event);
            } else {
                i += 1;
            }
        }

        processed
    }

    /// Applies an event to the simulation state.
    async fn apply_event(&self, event: &TemporalEvent) {
        let mut agents = self.agents.write().await;

        match event {
            TemporalEvent::AgentBirth { agent_id, date } => {
                if let Some(agent) = agents.get_mut(agent_id) {
                    agent.birth_date = Some(*date);
                    agent.active = true;
                }
            }
            TemporalEvent::AgentDeath { agent_id, date } => {
                if let Some(agent) = agents.get_mut(agent_id) {
                    agent.death_date = Some(*date);
                    agent.active = false;
                }
            }
            TemporalEvent::AttributeChange {
                agent_id,
                date,
                attribute,
                new_value,
                ..
            } => {
                if let Some(agent) = agents.get_mut(agent_id) {
                    agent.set_attribute(attribute, new_value.clone(), *date);
                }
            }
            // Statute events are handled by the temporal statute wrapper
            TemporalEvent::StatuteEffective { .. }
            | TemporalEvent::StatuteExpired { .. }
            | TemporalEvent::StatuteAmended { .. } => {}
        }
    }

    /// Simulates a single time step.
    fn simulate_step(
        &self,
        date: NaiveDate,
        statutes: &[&Statute],
        agents: &[&AgentState],
    ) -> SimulationMetrics {
        let mut metrics = SimulationMetrics::new();
        let date_str = date.format("%Y-%m-%d").to_string();

        for agent in agents {
            for statute in statutes {
                // Create a temporary entity with the agent's current attributes
                let mut temp_entity = BasicEntity::with_id(agent.id);
                for (key, value) in &agent.attributes {
                    temp_entity.set_attribute(key, value.clone());
                }
                // Add current date as an attribute for date-based conditions
                temp_entity.set_attribute("current_date", date_str.clone());

                let result = SimEngine::apply_law(&temp_entity, statute);

                metrics.record_result(&LawApplicationResult {
                    agent_id: agent.id,
                    statute_id: statute.id.clone(),
                    result,
                });
            }
        }

        metrics
    }

    /// Runs simulation with aging (incrementing age attribute).
    pub async fn run_with_aging(&mut self) -> TemporalMetrics {
        // Schedule age increments for yearly simulations
        if self.config.time_step == TimeStep::Year {
            let agents = self.agents.read().await;
            for (agent_id, state) in agents.iter() {
                if let Some(age_str) = state.attributes.get("age") {
                    if let Ok(age) = age_str.parse::<u32>() {
                        let mut current = self.config.start_date;
                        let mut current_age = age;
                        while current <= self.config.end_date {
                            current_age += 1;
                            current += Duration::days(365);
                            if current <= self.config.end_date {
                                // Store events to schedule
                                let event = TemporalEvent::AttributeChange {
                                    agent_id: *agent_id,
                                    date: current,
                                    attribute: "age".to_string(),
                                    old_value: Some((current_age - 1).to_string()),
                                    new_value: current_age.to_string(),
                                };
                                self.scheduled_events.push((current, event));
                            }
                        }
                    }
                }
            }
            self.scheduled_events.sort_by_key(|(d, _)| *d);
            drop(agents);
        }

        self.run().await
    }

    /// Applies a statute retroactively from a given date.
    /// This re-simulates all time steps from the retroactive date onwards.
    pub async fn apply_retroactive_statute(
        &mut self,
        statute: TemporalStatute,
        retroactive_from: NaiveDate,
    ) -> TemporalMetrics {
        // Add the statute to the collection
        self.statutes.push(statute);

        // Re-run simulation from the retroactive date
        let original_start = self.config.start_date;
        self.config.start_date = retroactive_from;

        let metrics = self.run().await;

        // Restore original start date
        self.config.start_date = original_start;

        metrics
    }

    /// Simulates what-if scenarios by comparing current state with alternative statute.
    pub async fn what_if_scenario(
        &self,
        alternative_statute: TemporalStatute,
        from_date: NaiveDate,
    ) -> (TemporalMetrics, TemporalMetrics) {
        // Clone current engine for baseline
        let mut baseline_engine = TemporalSimEngine::new(self.config.clone());
        baseline_engine.statutes = self.statutes.clone();

        // Clone agents
        let agents_read = self.agents.read().await;
        let mut baseline_agents = baseline_engine.agents.write().await;
        for (id, state) in agents_read.iter() {
            baseline_agents.insert(*id, state.clone());
        }
        drop(baseline_agents);
        drop(agents_read);

        // Run baseline from the specified date
        baseline_engine.config.start_date = from_date;
        let baseline_metrics = baseline_engine.run().await;

        // Create alternative engine
        let mut alternative_engine = TemporalSimEngine::new(self.config.clone());
        alternative_engine.statutes = self.statutes.clone();
        alternative_engine.statutes.push(alternative_statute);

        // Clone agents for alternative
        let agents_read = self.agents.read().await;
        let mut alt_agents = alternative_engine.agents.write().await;
        for (id, state) in agents_read.iter() {
            alt_agents.insert(*id, state.clone());
        }
        drop(alt_agents);
        drop(agents_read);

        // Run alternative from the specified date
        alternative_engine.config.start_date = from_date;
        let alternative_metrics = alternative_engine.run().await;

        (baseline_metrics, alternative_metrics)
    }

    /// Projects a numeric attribute value into the future based on a projection model.
    #[allow(dead_code)]
    fn project_value(&self, current_value: f64, model: &ProjectionModel, steps: i64) -> f64 {
        match model {
            ProjectionModel::Constant => current_value,
            ProjectionModel::Linear => current_value + steps as f64,
            ProjectionModel::Exponential { rate } => {
                current_value * (1.0 + rate).powi(steps as i32)
            }
            ProjectionModel::CustomRate { rate_per_step } => {
                current_value * (1.0 + rate_per_step).powi(steps as i32)
            }
        }
    }

    /// Runs a future projection simulation with demographic changes.
    ///
    /// This extends the simulation into the future with projected population growth,
    /// income changes, aging, births, and deaths based on the projection config.
    pub async fn run_with_projection(
        &mut self,
        projection_config: ProjectionConfig,
    ) -> TemporalMetrics {
        let mut metrics = TemporalMetrics::new();
        let mut current_date = self.config.start_date;
        let time_step_years = match self.config.time_step {
            TimeStep::Day => 1.0 / 365.0,
            TimeStep::Week => 1.0 / 52.0,
            TimeStep::Month => 1.0 / 12.0,
            TimeStep::Quarter => 0.25,
            TimeStep::Year => 1.0,
        };

        let mut step_count = 0i64;

        while current_date <= self.config.end_date {
            // Process scheduled events for this date
            let events_today = self.process_events(current_date).await;
            for event in &events_today {
                metrics.record_event(event.clone());
            }

            // Apply demographic projections
            if step_count > 0 {
                self.apply_demographic_projection(
                    current_date,
                    &projection_config,
                    time_step_years,
                    &mut metrics,
                )
                .await;
            }

            // Get active statutes for this date
            let active_statutes: Vec<&Statute> = self
                .statutes
                .iter()
                .filter(|s| s.is_effective_at(current_date))
                .map(|s| s.version_at(current_date))
                .collect();

            // Get active agents for this date
            let agents_read = self.agents.read().await;
            let active_agents: Vec<&AgentState> = agents_read
                .values()
                .filter(|a| a.is_active_at(current_date))
                .collect();

            // Run simulation for this time step
            let step_metrics = self.simulate_step(current_date, &active_statutes, &active_agents);

            // Create snapshot
            let snapshot = TimeSnapshot {
                date: current_date,
                metrics: step_metrics.clone(),
                active_agents: active_agents.len(),
                active_statutes: active_statutes.len(),
                events: events_today,
            };

            // Merge into cumulative metrics
            metrics.cumulative.total_applications += step_metrics.total_applications;
            metrics.cumulative.deterministic_count += step_metrics.deterministic_count;
            metrics.cumulative.discretion_count += step_metrics.discretion_count;
            metrics.cumulative.void_count += step_metrics.void_count;

            for (statute_id, statute_metrics) in step_metrics.statute_metrics {
                let cumulative = metrics
                    .cumulative
                    .statute_metrics
                    .entry(statute_id)
                    .or_default();
                cumulative.total += statute_metrics.total;
                cumulative.deterministic += statute_metrics.deterministic;
                cumulative.discretion += statute_metrics.discretion;
                cumulative.void += statute_metrics.void;
            }

            metrics.add_snapshot(snapshot);

            // Advance time
            current_date += self.config.time_step.to_duration();
            step_count += 1;
        }

        metrics
    }

    /// Applies demographic projection changes to the population.
    #[allow(clippy::too_many_arguments)]
    async fn apply_demographic_projection(
        &mut self,
        current_date: NaiveDate,
        config: &ProjectionConfig,
        time_step_years: f64,
        metrics: &mut TemporalMetrics,
    ) {
        let mut agents = self.agents.write().await;
        let agent_ids: Vec<Uuid> = agents.keys().copied().collect();

        // Update existing agents
        for agent_id in &agent_ids {
            if let Some(agent) = agents.get_mut(agent_id) {
                if !agent.is_active_at(current_date) {
                    continue;
                }

                // Age progression
                if let Some(age_str) = agent.attributes.get("age") {
                    if let Ok(age) = age_str.parse::<u32>() {
                        let years_passed = if self.config.time_step == TimeStep::Year {
                            1
                        } else {
                            0
                        };
                        if years_passed > 0 {
                            let new_age = age + years_passed;
                            agent.set_attribute("age", new_age.to_string(), current_date);
                        }
                    }
                }

                // Income projection
                if let Some(income_str) = agent.attributes.get("income") {
                    if let Ok(income) = income_str.parse::<f64>() {
                        let projected_income = match &config.income_model {
                            ProjectionModel::Exponential { rate } => {
                                income * (1.0 + rate * time_step_years)
                            }
                            ProjectionModel::CustomRate { rate_per_step } => {
                                income * (1.0 + rate_per_step)
                            }
                            _ => income,
                        };
                        if (projected_income - income).abs() > 0.01 {
                            agent.set_attribute(
                                "income",
                                projected_income.round().to_string(),
                                current_date,
                            );
                        }
                    }
                }
            }
        }

        // Handle births (new agents)
        let active_count = agents
            .values()
            .filter(|a| a.is_active_at(current_date))
            .count();
        let birth_count =
            (active_count as f64 * config.birth_rate * time_step_years).round() as usize;

        for _ in 0..birth_count {
            let new_id = Uuid::new_v4();
            let mut new_agent = AgentState::new(new_id);
            new_agent.birth_date = Some(current_date);
            new_agent.set_attribute("age", "0".to_string(), current_date);
            new_agent.set_attribute("income", "0".to_string(), current_date);
            agents.insert(new_id, new_agent);

            metrics.record_event(TemporalEvent::AgentBirth {
                agent_id: new_id,
                date: current_date,
            });
        }

        // Handle deaths
        let death_count =
            (active_count as f64 * config.death_rate * time_step_years).round() as usize;
        let active_ids: Vec<Uuid> = agents
            .iter()
            .filter(|(_, a)| a.is_active_at(current_date))
            .map(|(id, _)| *id)
            .collect();

        for &agent_id in active_ids.iter().take(death_count) {
            if let Some(agent) = agents.get_mut(&agent_id) {
                agent.death_date = Some(current_date);
                agent.active = false;

                metrics.record_event(TemporalEvent::AgentDeath {
                    agent_id,
                    date: current_date,
                });
            }
        }
    }
}

/// Projection model for future simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectionModel {
    /// Linear trend projection
    Linear,
    /// Exponential growth/decay
    Exponential { rate: f64 },
    /// Custom growth rate per time step
    CustomRate { rate_per_step: f64 },
    /// Constant (no change)
    Constant,
}

/// Configuration for future projection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionConfig {
    /// Model for population growth
    pub population_model: ProjectionModel,
    /// Model for income changes
    pub income_model: ProjectionModel,
    /// Model for age progression (typically constant +1 per year)
    pub age_model: ProjectionModel,
    /// Birth rate (new agents per year, as fraction of population)
    pub birth_rate: f64,
    /// Death rate (agents leaving per year, as fraction of population)
    pub death_rate: f64,
}

impl Default for ProjectionConfig {
    fn default() -> Self {
        Self {
            population_model: ProjectionModel::Linear,
            income_model: ProjectionModel::Exponential { rate: 0.03 }, // 3% annual growth
            age_model: ProjectionModel::Constant,
            birth_rate: 0.012, // 1.2% per year
            death_rate: 0.008, // 0.8% per year
        }
    }
}

/// Builder for temporal simulations.
pub struct TemporalSimBuilder {
    config: TemporalConfig,
    statutes: Vec<TemporalStatute>,
    entities: Vec<BasicEntity>,
    events: Vec<(NaiveDate, TemporalEvent)>,
}

impl TemporalSimBuilder {
    /// Creates a new builder.
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            config: TemporalConfig::new(start, end),
            statutes: Vec::new(),
            entities: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Sets the time step.
    pub fn with_time_step(mut self, step: TimeStep) -> Self {
        self.config.time_step = step;
        self
    }

    /// Adds a statute.
    pub fn add_statute(mut self, statute: TemporalStatute) -> Self {
        self.statutes.push(statute);
        self
    }

    /// Adds a population.
    pub fn add_population(mut self, entities: Vec<BasicEntity>) -> Self {
        self.entities.extend(entities);
        self
    }

    /// Schedules an event.
    pub fn schedule_event(mut self, date: NaiveDate, event: TemporalEvent) -> Self {
        self.events.push((date, event));
        self
    }

    /// Builds and returns the simulation engine.
    pub async fn build(self) -> TemporalSimEngine {
        let mut engine = TemporalSimEngine::new(self.config);
        engine.add_statutes(self.statutes);
        engine.add_population_with_attributes(&self.entities).await;
        for (date, event) in self.events {
            engine.schedule_event(date, event);
        }
        engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn create_test_config() -> TemporalConfig {
        TemporalConfig::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .with_time_step(TimeStep::Month)
    }

    #[test]
    fn test_time_step_duration() {
        assert_eq!(TimeStep::Day.to_duration(), Duration::days(1));
        assert_eq!(TimeStep::Week.to_duration(), Duration::weeks(1));
        assert_eq!(TimeStep::Year.to_duration(), Duration::days(365));
    }

    #[test]
    fn test_temporal_statute_effectiveness() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let temporal = TemporalStatute::new(statute)
            .with_effective_date(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());

        assert!(!temporal.is_effective_at(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
        assert!(temporal.is_effective_at(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()));
        assert!(!temporal.is_effective_at(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));
    }

    #[test]
    fn test_agent_state_history() {
        let mut state = AgentState::new(Uuid::new_v4());
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();

        state.set_attribute("income", "50000".to_string(), date1);
        state.set_attribute("income", "60000".to_string(), date2);

        assert_eq!(state.get_attribute_at("income", date1), Some("50000"));
        assert_eq!(state.get_attribute_at("income", date2), Some("60000"));
        assert_eq!(
            state.get_attribute_at("income", NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()),
            Some("50000")
        );
    }

    #[test]
    fn test_temporal_statute_amendments() {
        let original = Statute::new(
            "tax-rate",
            "Tax Rate",
            Effect::new(EffectType::Obligation, "10% tax"),
        );

        let amended = Statute::new(
            "tax-rate",
            "Tax Rate",
            Effect::new(EffectType::Obligation, "15% tax"),
        );

        let temporal = TemporalStatute::new(original.clone()).with_amendment(
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            amended.clone(),
        );

        let before = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let after = NaiveDate::from_ymd_opt(2024, 8, 1).unwrap();

        assert_eq!(temporal.version_at(before).effect.description, "10% tax");
        assert_eq!(temporal.version_at(after).effect.description, "15% tax");
    }

    #[tokio::test]
    async fn test_temporal_simulation_basic() {
        let config = create_test_config();
        let mut engine = TemporalSimEngine::new(config);

        // Add a statute that becomes effective mid-year
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights",
            Effect::new(EffectType::Grant, "Full capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        engine.add_statute(
            TemporalStatute::new(statute)
                .with_effective_date(NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()),
        );

        // Add some agents
        let mut adult = BasicEntity::new();
        adult.set_attribute("age", "25".to_string());
        engine.add_population_with_attributes(&[adult]).await;

        let metrics = engine.run().await;

        // Should have approximately 12-13 snapshots (monthly, depends on day count)
        assert!(metrics.snapshots.len() >= 12 && metrics.snapshots.len() <= 13);

        // Early snapshots (before July) should have 0 active statutes
        let early_snapshots: Vec<_> = metrics
            .snapshots
            .iter()
            .filter(|s| s.date < NaiveDate::from_ymd_opt(2024, 7, 1).unwrap())
            .collect();
        for snapshot in early_snapshots {
            assert_eq!(snapshot.active_statutes, 0);
        }

        // Later snapshots (July onwards) should have 1 active statute
        let later_snapshots: Vec<_> = metrics
            .snapshots
            .iter()
            .filter(|s| s.date >= NaiveDate::from_ymd_opt(2024, 7, 1).unwrap())
            .collect();
        for snapshot in later_snapshots {
            assert_eq!(snapshot.active_statutes, 1);
        }
    }

    #[tokio::test]
    async fn test_temporal_builder() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();

        let statute = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let mut entity = BasicEntity::new();
        entity.set_attribute("age", "30".to_string());

        let mut engine = TemporalSimBuilder::new(start, end)
            .with_time_step(TimeStep::Month)
            .add_statute(TemporalStatute::new(statute))
            .add_population(vec![entity])
            .build()
            .await;

        let metrics = engine.run().await;

        // Should have approximately 3-4 snapshots (90 days / 30 days per month)
        assert!(metrics.snapshots.len() >= 3 && metrics.snapshots.len() <= 4);
        assert!(metrics.cumulative.total_applications > 0);
    }

    #[test]
    fn test_config_total_steps() {
        let config = TemporalConfig::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .with_time_step(TimeStep::Month);

        assert_eq!(config.total_steps(), 12);
    }
}
