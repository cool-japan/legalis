//! Workflow state management.
//!
//! A [`WorkflowDefinition`] is a declarative state machine: named
//! [`WorkflowState`]s, [`WorkflowTransition`]s carrying an action name and a
//! declarative [`TransitionGuard`], and a designated initial state. A
//! [`WorkflowInstance`] is a running matter that holds the current state, a
//! context of [`FieldValue`] flags evaluated by guards, and a full
//! [`TransitionRecord`] history. A [`WorkflowEngine`] registers and starts
//! validated definitions.

use super::FieldValue;
use anyhow::{Result, anyhow, bail};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single state in a workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowState {
    /// Stable identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Whether the state is terminal (the workflow is complete here).
    pub terminal: bool,
}

impl WorkflowState {
    /// Creates a non-terminal state.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            terminal: false,
        }
    }

    /// Creates a terminal state.
    pub fn terminal(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            terminal: true,
        }
    }
}

/// A declarative guard evaluated against a workflow context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionGuard {
    /// Always permits the transition.
    Always,
    /// Permits when the named flag is present and truthy.
    FlagSet(String),
    /// Permits when the named flag is absent or falsy.
    FlagClear(String),
    /// Permits when the named flag equals the given value.
    Equals(String, FieldValue),
    /// Permits when the named flag does not equal the given value.
    NotEquals(String, FieldValue),
    /// Permits when all inner guards permit.
    AllOf(Vec<TransitionGuard>),
    /// Permits when any inner guard permits.
    AnyOf(Vec<TransitionGuard>),
    /// Permits when the inner guard does not.
    Not(Box<TransitionGuard>),
}

impl TransitionGuard {
    /// Evaluates the guard against a context.
    pub fn evaluate(&self, context: &HashMap<String, FieldValue>) -> bool {
        match self {
            TransitionGuard::Always => true,
            TransitionGuard::FlagSet(name) => context
                .get(name)
                .map(FieldValue::is_truthy)
                .unwrap_or(false),
            TransitionGuard::FlagClear(name) => !context
                .get(name)
                .map(FieldValue::is_truthy)
                .unwrap_or(false),
            TransitionGuard::Equals(name, value) => context.get(name) == Some(value),
            TransitionGuard::NotEquals(name, value) => context.get(name) != Some(value),
            TransitionGuard::AllOf(guards) => guards.iter().all(|guard| guard.evaluate(context)),
            TransitionGuard::AnyOf(guards) => guards.iter().any(|guard| guard.evaluate(context)),
            TransitionGuard::Not(guard) => !guard.evaluate(context),
        }
    }
}

/// A transition between two states triggered by a named action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransition {
    /// Source state id.
    pub from: String,
    /// Target state id.
    pub to: String,
    /// Action name that triggers the transition.
    pub action: String,
    /// Guard that must be satisfied for the transition to fire.
    pub guard: TransitionGuard,
    /// Optional description.
    pub description: Option<String>,
}

impl WorkflowTransition {
    /// Creates an unguarded transition.
    pub fn new(from: impl Into<String>, to: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            action: action.into(),
            guard: TransitionGuard::Always,
            description: None,
        }
    }

    /// Sets the guard.
    pub fn with_guard(mut self, guard: TransitionGuard) -> Self {
        self.guard = guard;
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// A declarative workflow state machine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Stable identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// States.
    pub states: Vec<WorkflowState>,
    /// Transitions.
    pub transitions: Vec<WorkflowTransition>,
    /// Initial state id.
    pub initial: String,
}

impl WorkflowDefinition {
    /// Creates a workflow with the given initial state id.
    pub fn new(id: impl Into<String>, name: impl Into<String>, initial: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            states: Vec::new(),
            transitions: Vec::new(),
            initial: initial.into(),
        }
    }

    /// Adds a state (builder style).
    pub fn with_state(mut self, state: WorkflowState) -> Self {
        self.states.push(state);
        self
    }

    /// Adds a transition (builder style).
    pub fn with_transition(mut self, transition: WorkflowTransition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Returns a state by id.
    pub fn state(&self, id: &str) -> Option<&WorkflowState> {
        self.states.iter().find(|state| state.id == id)
    }

    /// Returns the terminal states.
    pub fn terminal_states(&self) -> Vec<&WorkflowState> {
        self.states.iter().filter(|state| state.terminal).collect()
    }

    /// Validates the definition's internal consistency.
    pub fn validate(&self) -> Result<()> {
        if self.states.is_empty() {
            bail!("workflow '{}' has no states", self.id);
        }
        if self.state(&self.initial).is_none() {
            bail!(
                "workflow '{}' initial state '{}' does not exist",
                self.id,
                self.initial
            );
        }
        for transition in &self.transitions {
            if transition.action.trim().is_empty() {
                bail!(
                    "workflow '{}' has a transition with an empty action",
                    self.id
                );
            }
            if self.state(&transition.from).is_none() {
                bail!(
                    "transition references unknown source state '{}'",
                    transition.from
                );
            }
            if self.state(&transition.to).is_none() {
                bail!(
                    "transition references unknown target state '{}'",
                    transition.to
                );
            }
        }
        Ok(())
    }

    /// Validates and starts a new instance in the initial state.
    pub fn start(&self) -> Result<WorkflowInstance> {
        self.validate()?;
        Ok(WorkflowInstance {
            definition: self.clone(),
            current: self.initial.clone(),
            context: HashMap::new(),
            history: Vec::new(),
        })
    }

    /// Builds a standard contract-review workflow with a guarded approval.
    pub fn contract_review() -> Self {
        Self::new("contract_review", "Contract Review", "draft")
            .with_state(WorkflowState::new("draft", "Draft"))
            .with_state(WorkflowState::new("in_review", "In Review"))
            .with_state(WorkflowState::new("revisions", "Revisions Requested"))
            .with_state(WorkflowState::terminal("approved", "Approved"))
            .with_state(WorkflowState::terminal("rejected", "Rejected"))
            .with_transition(WorkflowTransition::new("draft", "in_review", "submit"))
            .with_transition(WorkflowTransition::new(
                "in_review",
                "revisions",
                "request_changes",
            ))
            .with_transition(WorkflowTransition::new(
                "revisions",
                "in_review",
                "resubmit",
            ))
            .with_transition(
                WorkflowTransition::new("in_review", "approved", "approve")
                    .with_guard(TransitionGuard::FlagSet("legal_ok".to_string()))
                    .with_description("Requires the legal_ok flag to be set"),
            )
            .with_transition(WorkflowTransition::new("in_review", "rejected", "reject"))
    }
}

/// A record of a fired transition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionRecord {
    /// State transitioned from.
    pub from: String,
    /// State transitioned to.
    pub to: String,
    /// Action that triggered the transition.
    pub action: String,
    /// When the transition occurred.
    pub at: DateTime<Utc>,
    /// Optional note attached to the transition.
    pub note: Option<String>,
}

/// A running instance of a workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowInstance {
    definition: WorkflowDefinition,
    current: String,
    context: HashMap<String, FieldValue>,
    history: Vec<TransitionRecord>,
}

impl WorkflowInstance {
    /// Returns the definition driving this instance.
    pub fn definition(&self) -> &WorkflowDefinition {
        &self.definition
    }

    /// Returns the current state id.
    pub fn current_state(&self) -> &str {
        &self.current
    }

    /// Returns the current state definition.
    pub fn current_state_def(&self) -> Option<&WorkflowState> {
        self.definition.state(&self.current)
    }

    /// Returns whether the instance is in a terminal state.
    pub fn is_complete(&self) -> bool {
        self.current_state_def()
            .map(|state| state.terminal)
            .unwrap_or(false)
    }

    /// Returns the context flags.
    pub fn context(&self) -> &HashMap<String, FieldValue> {
        &self.context
    }

    /// Sets a context flag.
    pub fn set_flag(&mut self, name: impl Into<String>, value: FieldValue) {
        self.context.insert(name.into(), value);
    }

    /// Seeds the context with the supplied flags (builder style).
    pub fn with_context(mut self, context: HashMap<String, FieldValue>) -> Self {
        self.context = context;
        self
    }

    /// Returns the transitions currently available (guards satisfied).
    pub fn available_actions(&self) -> Vec<&WorkflowTransition> {
        self.definition
            .transitions
            .iter()
            .filter(|transition| {
                transition.from == self.current && transition.guard.evaluate(&self.context)
            })
            .collect()
    }

    /// Returns whether an action can currently fire.
    pub fn can_fire(&self, action: &str) -> bool {
        self.available_actions()
            .iter()
            .any(|transition| transition.action == action)
    }

    /// Fires an action, transitioning the instance and recording history.
    pub fn fire(&mut self, action: &str, note: Option<String>) -> Result<&WorkflowState> {
        let from = self.current.clone();
        let matches: Vec<&WorkflowTransition> = self
            .definition
            .transitions
            .iter()
            .filter(|transition| transition.from == from && transition.action == action)
            .collect();
        if matches.is_empty() {
            bail!("no transition '{}' from state '{}'", action, from);
        }
        let target = match matches
            .into_iter()
            .find(|transition| transition.guard.evaluate(&self.context))
        {
            Some(transition) => transition.to.clone(),
            None => bail!("transition '{}' from '{}' is guarded off", action, from),
        };

        self.history.push(TransitionRecord {
            from,
            to: target.clone(),
            action: action.to_string(),
            at: Utc::now(),
            note,
        });
        self.current = target;
        self.current_state_def()
            .ok_or_else(|| anyhow!("target state '{}' not found", self.current))
    }

    /// Returns the transition history.
    pub fn history(&self) -> &[TransitionRecord] {
        &self.history
    }
}

/// A registry of workflow definitions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowEngine {
    definitions: HashMap<String, WorkflowDefinition>,
}

impl WorkflowEngine {
    /// Creates an empty engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a definition after validating it.
    pub fn register(&mut self, definition: WorkflowDefinition) -> Result<()> {
        definition.validate()?;
        self.definitions.insert(definition.id.clone(), definition);
        Ok(())
    }

    /// Returns a registered definition.
    pub fn get(&self, id: &str) -> Option<&WorkflowDefinition> {
        self.definitions.get(id)
    }

    /// Returns the registered definition ids (sorted).
    pub fn ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.definitions.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Returns the number of registered definitions.
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Returns whether the engine has no definitions.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Starts a new instance of a registered definition.
    pub fn start(&self, id: &str) -> Result<WorkflowInstance> {
        let definition = self
            .definitions
            .get(id)
            .ok_or_else(|| anyhow!("unknown workflow: {}", id))?;
        definition.start()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_rejects_bad_definitions() {
        // Unknown initial state.
        let bad_initial =
            WorkflowDefinition::new("w", "W", "missing").with_state(WorkflowState::new("a", "A"));
        assert!(bad_initial.validate().is_err());

        // Dangling transition target.
        let dangling = WorkflowDefinition::new("w", "W", "a")
            .with_state(WorkflowState::new("a", "A"))
            .with_transition(WorkflowTransition::new("a", "b", "go"));
        assert!(dangling.validate().is_err());

        // Valid definition.
        assert!(WorkflowDefinition::contract_review().validate().is_ok());
    }

    #[test]
    fn test_happy_path_to_terminal() {
        let mut instance = WorkflowDefinition::contract_review()
            .start()
            .expect("starts");
        assert_eq!(instance.current_state(), "draft");
        assert!(!instance.is_complete());

        instance.fire("submit", None).expect("submit");
        assert_eq!(instance.current_state(), "in_review");

        // approve is guarded off until legal_ok is set.
        assert!(!instance.can_fire("approve"));
        instance.set_flag("legal_ok", FieldValue::Boolean(true));
        assert!(instance.can_fire("approve"));

        let state = instance
            .fire("approve", Some("LGTM".to_string()))
            .expect("approve");
        assert_eq!(state.id, "approved");
        assert!(instance.is_complete());
        assert_eq!(instance.history().len(), 2);
        assert_eq!(instance.history()[1].action, "approve");
        assert_eq!(instance.history()[1].note.as_deref(), Some("LGTM"));
    }

    #[test]
    fn test_guarded_transition_blocks() {
        let mut instance = WorkflowDefinition::contract_review()
            .start()
            .expect("starts");
        instance.fire("submit", None).expect("submit");
        // Without legal_ok, firing approve fails (guarded off).
        assert!(instance.fire("approve", None).is_err());
        // The reject path is always available.
        assert!(instance.can_fire("reject"));
        let state = instance.fire("reject", None).expect("reject");
        assert_eq!(state.id, "rejected");
        assert!(instance.is_complete());
    }

    #[test]
    fn test_invalid_action_errors() {
        let mut instance = WorkflowDefinition::contract_review()
            .start()
            .expect("starts");
        assert!(instance.fire("approve", None).is_err()); // wrong state for action
        assert!(instance.fire("nonexistent", None).is_err());
        assert_eq!(instance.current_state(), "draft");
        assert!(instance.history().is_empty());
    }

    #[test]
    fn test_guard_combinators() {
        let mut context = HashMap::new();
        context.insert("a".to_string(), FieldValue::Boolean(true));
        context.insert("b".to_string(), FieldValue::text("x"));

        let guard = TransitionGuard::AllOf(vec![
            TransitionGuard::FlagSet("a".to_string()),
            TransitionGuard::Equals("b".to_string(), FieldValue::text("x")),
        ]);
        assert!(guard.evaluate(&context));

        let negated = TransitionGuard::Not(Box::new(TransitionGuard::FlagSet("a".to_string())));
        assert!(!negated.evaluate(&context));

        let any = TransitionGuard::AnyOf(vec![
            TransitionGuard::FlagSet("missing".to_string()),
            TransitionGuard::FlagClear("missing".to_string()),
        ]);
        assert!(any.evaluate(&context));
    }

    #[test]
    fn test_engine_register_and_start() {
        let mut engine = WorkflowEngine::new();
        engine
            .register(WorkflowDefinition::contract_review())
            .expect("registers");
        assert_eq!(engine.len(), 1);
        assert!(engine.ids().contains(&"contract_review".to_string()));

        let instance = engine.start("contract_review").expect("starts");
        assert_eq!(instance.current_state(), "draft");
        assert!(engine.start("missing").is_err());
    }
}
