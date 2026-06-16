//! Smart-contract integration for automated diff workflows.
//!
//! A [`SmartContract`] is a list of [`Clause`]s, each pairing a [`Trigger`]
//! predicate over a recorded diff with a list of [`Action`]s to execute when
//! the trigger fires. Execution is performed by a deterministic, gas-metered
//! [`ContractEngine`]: every trigger node and action consumes gas, and a
//! contract that exceeds its budget fails cleanly rather than looping. Actions
//! can drive a [`LegalWorkflow`] state machine, providing the "automated
//! workflow" behaviour — e.g. a breaking change automatically routes a statute
//! into review and freezes it until approved.

use super::ledger::DiffRecord;
use crate::{DiffError, DiffResult, Severity};
use serde::{Deserialize, Serialize};

/// Default per-contract gas budget.
pub const DEFAULT_GAS_LIMIT: u64 = 100_000;

/// Gas charged for evaluating a single trigger node.
const TRIGGER_GAS: u64 = 1;
/// Base gas charged per executed action.
const ACTION_BASE_GAS: u64 = 10;

/// A predicate over a recorded diff. Triggers compose with [`Trigger::and`],
/// [`Trigger::or`] and [`Trigger::not`] into an evaluable AST.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Trigger {
    /// Always fires.
    Always,
    /// Fires if the diff's severity is at least the given level.
    SeverityAtLeast(Severity),
    /// Fires if the diff affects eligibility.
    AffectsEligibility,
    /// Fires if the diff affects the outcome/effect.
    AffectsOutcome,
    /// Fires if discretion requirements changed.
    DiscretionChanged,
    /// Fires if the diff has at least this many changes.
    ChangeCountAtLeast(usize),
    /// Fires if the record is for this statute.
    StatuteIs(String),
    /// Fires if the record was created by this actor.
    RecorderIs(String),
    /// Logical conjunction.
    And(Box<Trigger>, Box<Trigger>),
    /// Logical disjunction.
    Or(Box<Trigger>, Box<Trigger>),
    /// Logical negation.
    Not(Box<Trigger>),
}

impl Trigger {
    /// Conjunction combinator.
    pub fn and(self, other: Trigger) -> Trigger {
        Trigger::And(Box::new(self), Box::new(other))
    }

    /// Disjunction combinator.
    pub fn or(self, other: Trigger) -> Trigger {
        Trigger::Or(Box::new(self), Box::new(other))
    }

    /// Negation combinator.
    pub fn negate(self) -> Trigger {
        Trigger::Not(Box::new(self))
    }
}

/// A step in the legal lifecycle that an action can request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStep {
    /// Move a proposed change into review.
    SubmitForReview,
    /// Register an approval.
    Approve,
    /// Enact an approved change.
    Enact,
    /// Reject a change under review.
    Reject,
}

/// An action executed when a clause's trigger fires.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Emit a named event.
    Emit(String),
    /// Notify a stakeholder role.
    NotifyRole(String),
    /// Require a number of approvals before enactment (also routes to review).
    RequireApprovals(u32),
    /// Freeze the statute pending resolution.
    FreezeStatute,
    /// Transfer tokens to a recipient (settled by the token ledger elsewhere).
    TransferTokens { to: String, amount: u64 },
    /// Mint an NFT for the diff.
    MintNft,
    /// Advance the attached workflow.
    AdvanceWorkflow(WorkflowStep),
}

impl Action {
    /// Gas surcharge on top of [`ACTION_BASE_GAS`] for this action.
    fn surcharge(&self) -> u64 {
        match self {
            Action::Emit(_) | Action::NotifyRole(_) => 0,
            Action::RequireApprovals(_) | Action::AdvanceWorkflow(_) => 5,
            Action::FreezeStatute => 15,
            Action::TransferTokens { .. } => 20,
            Action::MintNft => 40,
        }
    }
}

/// A trigger paired with the actions to run when it fires.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clause {
    /// The condition under which the actions run.
    pub trigger: Trigger,
    /// Actions executed (in order) when `trigger` fires.
    pub actions: Vec<Action>,
}

impl Clause {
    /// Creates a clause.
    pub fn new(trigger: Trigger, actions: Vec<Action>) -> Self {
        Self { trigger, actions }
    }
}

/// A named collection of clauses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmartContract {
    /// Stable contract identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Ordered clauses.
    pub clauses: Vec<Clause>,
}

impl SmartContract {
    /// Creates an empty contract.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            clauses: Vec::new(),
        }
    }

    /// Adds a clause (builder style).
    pub fn with_clause(mut self, clause: Clause) -> Self {
        self.clauses.push(clause);
        self
    }
}

/// Category of an emitted contract event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    /// A user-named event was emitted.
    Emitted,
    /// A stakeholder notification.
    Notification,
    /// An approval requirement was registered.
    ApprovalRequest,
    /// The statute was frozen.
    StatuteFrozen,
    /// A token transfer was requested.
    TokenTransfer,
    /// An NFT mint was requested.
    NftMint,
    /// A workflow transition occurred.
    WorkflowTransition,
}

/// A single effect produced during contract execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractEvent {
    /// The category of effect.
    pub kind: EventKind,
    /// Human-readable detail.
    pub detail: String,
    /// The statute the effect pertains to.
    pub statute_id: String,
}

/// The outcome of executing a contract against a diff record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractReceipt {
    /// The contract that was executed.
    pub contract_id: String,
    /// The statute the record pertained to.
    pub statute_id: String,
    /// Number of clauses whose trigger fired.
    pub triggered_clauses: usize,
    /// Total gas consumed.
    pub gas_used: u64,
    /// Effects produced, in execution order.
    pub events: Vec<ContractEvent>,
}

/// Lifecycle state of a statute change under a contract-driven workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WorkflowState {
    /// Newly proposed, not yet under review.
    #[default]
    Proposed,
    /// Actively under review.
    UnderReview,
    /// Approved, awaiting enactment.
    Approved,
    /// Fully enacted.
    Enacted,
    /// Frozen pending resolution.
    Frozen,
    /// Rejected; terminal.
    Rejected,
}

/// A small state machine modelling a statute change's review lifecycle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalWorkflow {
    /// Current state.
    pub state: WorkflowState,
    /// Approvals required before the change can be approved.
    pub approvals_required: u32,
    /// Approvals registered so far.
    pub approvals: u32,
}

impl LegalWorkflow {
    /// Creates a workflow in the [`WorkflowState::Proposed`] state.
    pub fn new() -> Self {
        Self {
            state: WorkflowState::Proposed,
            approvals_required: 1,
            approvals: 0,
        }
    }

    /// Sets how many approvals are required.
    pub fn set_required(&mut self, n: u32) {
        self.approvals_required = n.max(1);
    }

    /// Moves a proposed change into review.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] from a non-[`WorkflowState::Proposed`]
    /// state.
    pub fn submit_for_review(&mut self) -> DiffResult<()> {
        match self.state {
            WorkflowState::Proposed | WorkflowState::Frozen => {
                self.state = WorkflowState::UnderReview;
                Ok(())
            }
            other => Err(DiffError::ContractError(format!(
                "cannot submit for review from {:?}",
                other
            ))),
        }
    }

    /// Registers an approval, transitioning to [`WorkflowState::Approved`] once
    /// the required count is met.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] if not under review.
    pub fn approve(&mut self) -> DiffResult<()> {
        if self.state != WorkflowState::UnderReview {
            return Err(DiffError::ContractError(format!(
                "cannot approve from {:?}",
                self.state
            )));
        }
        self.approvals += 1;
        if self.approvals >= self.approvals_required {
            self.state = WorkflowState::Approved;
        }
        Ok(())
    }

    /// Enacts an approved change.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] if not approved.
    pub fn enact(&mut self) -> DiffResult<()> {
        if self.state != WorkflowState::Approved {
            return Err(DiffError::ContractError(format!(
                "cannot enact from {:?}",
                self.state
            )));
        }
        self.state = WorkflowState::Enacted;
        Ok(())
    }

    /// Rejects a change that is proposed or under review.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] from a terminal/enacted state.
    pub fn reject(&mut self) -> DiffResult<()> {
        match self.state {
            WorkflowState::Proposed | WorkflowState::UnderReview | WorkflowState::Frozen => {
                self.state = WorkflowState::Rejected;
                Ok(())
            }
            other => Err(DiffError::ContractError(format!(
                "cannot reject from {:?}",
                other
            ))),
        }
    }

    /// Freezes a non-terminal change.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] if already enacted or rejected.
    pub fn freeze(&mut self) -> DiffResult<()> {
        match self.state {
            WorkflowState::Enacted | WorkflowState::Rejected => Err(DiffError::ContractError(
                format!("cannot freeze from {:?}", self.state),
            )),
            _ => {
                self.state = WorkflowState::Frozen;
                Ok(())
            }
        }
    }

    fn apply_step(&mut self, step: WorkflowStep) -> DiffResult<()> {
        match step {
            WorkflowStep::SubmitForReview => self.submit_for_review(),
            WorkflowStep::Approve => self.approve(),
            WorkflowStep::Enact => self.enact(),
            WorkflowStep::Reject => self.reject(),
        }
    }
}

impl Default for LegalWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

/// A deterministic, gas-metered executor for smart contracts.
#[derive(Debug, Clone)]
pub struct ContractEngine {
    gas_limit: u64,
}

impl ContractEngine {
    /// Creates an engine with the given per-contract gas limit.
    pub fn new(gas_limit: u64) -> Self {
        Self { gas_limit }
    }

    /// The configured gas limit.
    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    /// Executes a contract against a record, ignoring workflow side effects.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] if the gas budget is exceeded.
    pub fn execute(
        &self,
        contract: &SmartContract,
        record: &DiffRecord,
    ) -> DiffResult<ContractReceipt> {
        self.run(contract, record, None)
    }

    /// Executes a contract against a record, applying workflow transitions to
    /// `workflow` as actions request them.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ContractError`] if the gas budget is exceeded or an
    /// action requests an illegal workflow transition.
    pub fn execute_with_workflow(
        &self,
        contract: &SmartContract,
        record: &DiffRecord,
        workflow: &mut LegalWorkflow,
    ) -> DiffResult<ContractReceipt> {
        self.run(contract, record, Some(workflow))
    }

    fn run(
        &self,
        contract: &SmartContract,
        record: &DiffRecord,
        mut workflow: Option<&mut LegalWorkflow>,
    ) -> DiffResult<ContractReceipt> {
        let mut gas_used: u64 = 0;
        let mut events = Vec::new();
        let mut triggered = 0usize;

        for clause in &contract.clauses {
            if eval_trigger(&clause.trigger, record, &mut gas_used, self.gas_limit)? {
                triggered += 1;
                for action in &clause.actions {
                    gas_used += ACTION_BASE_GAS + action.surcharge();
                    if gas_used > self.gas_limit {
                        return Err(DiffError::ContractError(format!(
                            "contract '{}' exceeded gas limit of {}",
                            contract.id, self.gas_limit
                        )));
                    }
                    let event = execute_action(action, record, workflow.as_deref_mut())?;
                    events.push(event);
                }
            }
        }

        Ok(ContractReceipt {
            contract_id: contract.id.clone(),
            statute_id: record.statute_id.clone(),
            triggered_clauses: triggered,
            gas_used,
            events,
        })
    }
}

impl Default for ContractEngine {
    fn default() -> Self {
        Self::new(DEFAULT_GAS_LIMIT)
    }
}

/// Recursively evaluates a trigger, charging gas per node.
fn eval_trigger(
    trigger: &Trigger,
    record: &DiffRecord,
    gas_used: &mut u64,
    gas_limit: u64,
) -> DiffResult<bool> {
    *gas_used += TRIGGER_GAS;
    if *gas_used > gas_limit {
        return Err(DiffError::ContractError(format!(
            "trigger evaluation exceeded gas limit of {}",
            gas_limit
        )));
    }
    let result = match trigger {
        Trigger::Always => true,
        Trigger::SeverityAtLeast(level) => record.severity >= *level,
        Trigger::AffectsEligibility => record.diff.impact.affects_eligibility,
        Trigger::AffectsOutcome => record.diff.impact.affects_outcome,
        Trigger::DiscretionChanged => record.diff.impact.discretion_changed,
        Trigger::ChangeCountAtLeast(n) => record.change_count >= *n,
        Trigger::StatuteIs(id) => &record.statute_id == id,
        Trigger::RecorderIs(actor) => &record.recorder == actor,
        Trigger::And(a, b) => {
            eval_trigger(a, record, gas_used, gas_limit)?
                && eval_trigger(b, record, gas_used, gas_limit)?
        }
        Trigger::Or(a, b) => {
            eval_trigger(a, record, gas_used, gas_limit)?
                || eval_trigger(b, record, gas_used, gas_limit)?
        }
        Trigger::Not(inner) => !eval_trigger(inner, record, gas_used, gas_limit)?,
    };
    Ok(result)
}

/// Executes a single action, optionally driving the workflow, and returns the
/// resulting event.
fn execute_action(
    action: &Action,
    record: &DiffRecord,
    workflow: Option<&mut LegalWorkflow>,
) -> DiffResult<ContractEvent> {
    let statute_id = record.statute_id.clone();
    let event = match action {
        Action::Emit(name) => ContractEvent {
            kind: EventKind::Emitted,
            detail: name.clone(),
            statute_id,
        },
        Action::NotifyRole(role) => ContractEvent {
            kind: EventKind::Notification,
            detail: format!("notify role '{}'", role),
            statute_id,
        },
        Action::RequireApprovals(n) => {
            if let Some(wf) = workflow {
                wf.set_required(*n);
                if wf.state == WorkflowState::Proposed {
                    wf.submit_for_review()?;
                }
            }
            ContractEvent {
                kind: EventKind::ApprovalRequest,
                detail: format!("require {} approval(s)", n),
                statute_id,
            }
        }
        Action::FreezeStatute => {
            if let Some(wf) = workflow {
                wf.freeze()?;
            }
            ContractEvent {
                kind: EventKind::StatuteFrozen,
                detail: "statute frozen".to_string(),
                statute_id,
            }
        }
        Action::TransferTokens { to, amount } => ContractEvent {
            kind: EventKind::TokenTransfer,
            detail: format!("transfer {} to {}", amount, to),
            statute_id,
        },
        Action::MintNft => ContractEvent {
            kind: EventKind::NftMint,
            detail: "mint diff NFT".to_string(),
            statute_id,
        },
        Action::AdvanceWorkflow(step) => {
            if let Some(wf) = workflow {
                wf.apply_step(*step)?;
            }
            ContractEvent {
                kind: EventKind::WorkflowTransition,
                detail: format!("{:?}", step),
                statute_id,
            }
        }
    };
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::ledger::DiffRecord;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn record(id: &str, breaking: bool) -> DiffRecord {
        let old = Statute::new(id, "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        if breaking {
            new.effect = Effect::new(EffectType::Revoke, "Revoked");
        } else {
            new.title = "New".to_string();
        }
        let d = diff(&old, &new).expect("diff");
        DiffRecord::from_diff(&d, "alice").expect("record")
    }

    #[test]
    fn test_always_trigger_fires() {
        let contract = SmartContract::new("c1", "always").with_clause(Clause::new(
            Trigger::Always,
            vec![Action::Emit("hello".to_string())],
        ));
        let engine = ContractEngine::default();
        let receipt = engine
            .execute(&contract, &record("s", false))
            .expect("exec");
        assert_eq!(receipt.triggered_clauses, 1);
        assert_eq!(receipt.events.len(), 1);
        assert_eq!(receipt.events[0].kind, EventKind::Emitted);
        assert!(receipt.gas_used > 0);
    }

    #[test]
    fn test_severity_trigger_gating() {
        let contract = SmartContract::new("c2", "sev").with_clause(Clause::new(
            Trigger::SeverityAtLeast(Severity::Major),
            vec![Action::FreezeStatute],
        ));
        let engine = ContractEngine::default();

        let minor = engine
            .execute(&contract, &record("s", false))
            .expect("exec");
        assert_eq!(minor.triggered_clauses, 0);

        let major = engine.execute(&contract, &record("s", true)).expect("exec");
        assert_eq!(major.triggered_clauses, 1);
        assert_eq!(major.events[0].kind, EventKind::StatuteFrozen);
    }

    #[test]
    fn test_boolean_combinators() {
        let t = Trigger::AffectsOutcome
            .and(Trigger::SeverityAtLeast(Severity::Major))
            .or(Trigger::StatuteIs("never".to_string()));
        let contract =
            SmartContract::new("c3", "combo").with_clause(Clause::new(t, vec![Action::MintNft]));
        let engine = ContractEngine::default();
        let receipt = engine.execute(&contract, &record("s", true)).expect("exec");
        assert_eq!(receipt.triggered_clauses, 1);
        assert_eq!(receipt.events[0].kind, EventKind::NftMint);
    }

    #[test]
    fn test_not_combinator() {
        let contract = SmartContract::new("c4", "not").with_clause(Clause::new(
            Trigger::AffectsOutcome.negate(),
            vec![Action::Emit("no-outcome".to_string())],
        ));
        let engine = ContractEngine::default();
        // Non-breaking diff does not affect outcome -> Not(...) is true.
        let receipt = engine
            .execute(&contract, &record("s", false))
            .expect("exec");
        assert_eq!(receipt.triggered_clauses, 1);
    }

    #[test]
    fn test_gas_exhaustion() {
        // Tiny budget: even a single action overruns.
        let engine = ContractEngine::new(3);
        let contract = SmartContract::new("c5", "gas")
            .with_clause(Clause::new(Trigger::Always, vec![Action::MintNft]));
        assert!(engine.execute(&contract, &record("s", false)).is_err());
    }

    #[test]
    fn test_gas_accounting_increases_with_actions() {
        let engine = ContractEngine::default();
        let one = SmartContract::new("c", "one")
            .with_clause(Clause::new(Trigger::Always, vec![Action::Emit("a".into())]));
        let two = SmartContract::new("c", "two").with_clause(Clause::new(
            Trigger::Always,
            vec![Action::Emit("a".into()), Action::MintNft],
        ));
        let r1 = engine.execute(&one, &record("s", false)).expect("exec");
        let r2 = engine.execute(&two, &record("s", false)).expect("exec");
        assert!(r2.gas_used > r1.gas_used);
    }

    #[test]
    fn test_workflow_happy_path() {
        let mut wf = LegalWorkflow::new();
        assert_eq!(wf.state, WorkflowState::Proposed);
        wf.set_required(2);
        wf.submit_for_review().expect("review");
        assert_eq!(wf.state, WorkflowState::UnderReview);
        wf.approve().expect("approve 1");
        assert_eq!(wf.state, WorkflowState::UnderReview);
        wf.approve().expect("approve 2");
        assert_eq!(wf.state, WorkflowState::Approved);
        wf.enact().expect("enact");
        assert_eq!(wf.state, WorkflowState::Enacted);
    }

    #[test]
    fn test_workflow_illegal_transition() {
        let mut wf = LegalWorkflow::new();
        // Cannot enact straight from Proposed.
        assert!(wf.enact().is_err());
    }

    #[test]
    fn test_contract_drives_workflow() {
        // A breaking change requires approvals (routes to review) and freezes.
        let contract = SmartContract::new("gov", "governance")
            .with_clause(Clause::new(
                Trigger::SeverityAtLeast(Severity::Major),
                vec![Action::RequireApprovals(1), Action::FreezeStatute],
            ))
            .with_clause(Clause::new(
                Trigger::AffectsOutcome,
                vec![Action::NotifyRole("legal-counsel".to_string())],
            ));
        let engine = ContractEngine::default();
        let mut wf = LegalWorkflow::new();
        let receipt = engine
            .execute_with_workflow(&contract, &record("s", true), &mut wf)
            .expect("exec");
        assert_eq!(receipt.triggered_clauses, 2);
        assert_eq!(wf.state, WorkflowState::Frozen);
        assert!(
            receipt
                .events
                .iter()
                .any(|e| e.kind == EventKind::Notification)
        );
    }

    #[test]
    fn test_workflow_advance_steps() {
        let contract = SmartContract::new("flow", "flow").with_clause(Clause::new(
            Trigger::Always,
            vec![
                Action::AdvanceWorkflow(WorkflowStep::SubmitForReview),
                Action::AdvanceWorkflow(WorkflowStep::Approve),
                Action::AdvanceWorkflow(WorkflowStep::Enact),
            ],
        ));
        let engine = ContractEngine::default();
        let mut wf = LegalWorkflow::new();
        engine
            .execute_with_workflow(&contract, &record("s", false), &mut wf)
            .expect("exec");
        assert_eq!(wf.state, WorkflowState::Enacted);
    }

    #[test]
    fn test_workflow_illegal_step_fails_execution() {
        let contract = SmartContract::new("bad", "bad").with_clause(Clause::new(
            Trigger::Always,
            vec![Action::AdvanceWorkflow(WorkflowStep::Enact)],
        ));
        let engine = ContractEngine::default();
        let mut wf = LegalWorkflow::new();
        assert!(
            engine
                .execute_with_workflow(&contract, &record("s", false), &mut wf)
                .is_err()
        );
    }

    #[test]
    fn test_reject_and_freeze_paths() {
        let mut wf = LegalWorkflow::new();
        wf.submit_for_review().expect("review");
        wf.reject().expect("reject");
        assert_eq!(wf.state, WorkflowState::Rejected);
        // Cannot freeze a rejected change.
        assert!(wf.freeze().is_err());
    }

    #[test]
    fn test_recorder_and_statute_triggers() {
        let contract = SmartContract::new("audit", "audit")
            .with_clause(Clause::new(
                Trigger::RecorderIs("alice".to_string()),
                vec![Action::Emit("alice-change".to_string())],
            ))
            .with_clause(Clause::new(
                Trigger::StatuteIs("other".to_string()),
                vec![Action::Emit("other".to_string())],
            ));
        let engine = ContractEngine::default();
        let receipt = engine
            .execute(&contract, &record("s", false))
            .expect("exec");
        assert_eq!(receipt.triggered_clauses, 1);
        assert_eq!(receipt.events[0].detail, "alice-change");
    }

    #[test]
    fn test_change_count_trigger() {
        let contract = SmartContract::new("cc", "cc").with_clause(Clause::new(
            Trigger::ChangeCountAtLeast(1),
            vec![Action::Emit("has-changes".to_string())],
        ));
        let engine = ContractEngine::default();
        let receipt = engine.execute(&contract, &record("s", true)).expect("exec");
        assert_eq!(receipt.triggered_clauses, 1);
    }

    #[test]
    fn test_receipt_serde_roundtrip() {
        let contract = SmartContract::new("c", "c")
            .with_clause(Clause::new(Trigger::Always, vec![Action::Emit("x".into())]));
        let engine = ContractEngine::default();
        let receipt = engine
            .execute(&contract, &record("s", false))
            .expect("exec");
        let json = serde_json::to_string(&receipt).expect("ser");
        let back: ContractReceipt = serde_json::from_str(&json).expect("de");
        assert_eq!(receipt, back);
    }
}
