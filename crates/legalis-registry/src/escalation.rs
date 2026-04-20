use super::*;

/// Escalation condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    /// Time-based: escalate after duration
    AfterDuration { seconds: i64 },
    /// Overdue task or approval
    Overdue,
    /// SLA breach
    SlaBreach,
    /// No response after duration
    NoResponseAfter { seconds: i64 },
    /// Multiple rejections
    MultipleRejections { count: usize },
}

impl EscalationCondition {
    /// Checks if condition is met for a timestamp.
    pub fn is_met(&self, created_at: DateTime<Utc>, _has_response: bool) -> bool {
        match self {
            Self::AfterDuration { seconds } => {
                let elapsed = Utc::now() - created_at;
                elapsed.num_seconds() >= *seconds
            }
            Self::Overdue => {
                // Would need due date to check properly
                false
            }
            Self::SlaBreach => {
                // Would need SLA tracking
                false
            }
            Self::NoResponseAfter { seconds } => {
                if _has_response {
                    false
                } else {
                    let elapsed = Utc::now() - created_at;
                    elapsed.num_seconds() >= *seconds
                }
            }
            Self::MultipleRejections { count: _ } => {
                // Would need rejection tracking
                false
            }
        }
    }
}

/// Escalation action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationAction {
    /// Notify additional users
    Notify { users: Vec<String> },
    /// Reassign to different user
    Reassign { to_user: String },
    /// Escalate to manager
    EscalateToManager,
    /// Auto-approve
    AutoApprove,
    /// Custom action
    Custom(String),
}

/// Escalation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    /// Rule ID
    pub rule_id: Uuid,
    /// Rule name
    pub name: String,
    /// Condition to trigger escalation
    pub condition: EscalationCondition,
    /// Action to take
    pub action: EscalationAction,
    /// Priority (higher = evaluated first)
    pub priority: i32,
    /// Whether the rule is enabled
    pub enabled: bool,
}

impl EscalationRule {
    /// Creates a new escalation rule.
    pub fn new(
        name: impl Into<String>,
        condition: EscalationCondition,
        action: EscalationAction,
    ) -> Self {
        Self {
            rule_id: Uuid::new_v4(),
            name: name.into(),
            condition,
            action,
            priority: 0,
            enabled: true,
        }
    }

    /// Sets priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Checks if the rule should be triggered.
    pub fn should_trigger(&self, created_at: DateTime<Utc>, has_response: bool) -> bool {
        self.enabled && self.condition.is_met(created_at, has_response)
    }
}

/// Escalation event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationEvent {
    /// Event ID
    pub event_id: Uuid,
    /// Rule that triggered
    pub rule_id: Uuid,
    /// Entity that was escalated
    pub entity_id: String,
    /// Action taken
    pub action: EscalationAction,
    /// Timestamp
    pub escalated_at: DateTime<Utc>,
}

/// Escalation manager.
#[derive(Debug)]
pub struct EscalationManager {
    rules: Vec<EscalationRule>,
    events: Vec<EscalationEvent>,
}

impl EscalationManager {
    /// Creates a new escalation manager.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Adds an escalation rule.
    pub fn add_rule(&mut self, rule: EscalationRule) {
        self.rules.push(rule);
        // Sort by priority (descending)
        self.rules.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }

    /// Checks for escalations and applies rules.
    pub fn check_escalations(
        &mut self,
        entity_id: impl Into<String>,
        created_at: DateTime<Utc>,
        has_response: bool,
    ) -> Vec<EscalationAction> {
        let entity_id = entity_id.into();
        let mut actions = Vec::new();

        for rule in &self.rules {
            if rule.should_trigger(created_at, has_response) {
                let event = EscalationEvent {
                    event_id: Uuid::new_v4(),
                    rule_id: rule.rule_id,
                    entity_id: entity_id.clone(),
                    action: rule.action.clone(),
                    escalated_at: Utc::now(),
                };
                actions.push(rule.action.clone());
                self.events.push(event);
            }
        }

        actions
    }

    /// Gets escalation events for an entity.
    pub fn events_for_entity(&self, entity_id: &str) -> Vec<&EscalationEvent> {
        self.events
            .iter()
            .filter(|e| e.entity_id == entity_id)
            .collect()
    }

    /// Gets all rules.
    pub fn rules(&self) -> &[EscalationRule] {
        &self.rules
    }
}

impl Default for EscalationManager {
    fn default() -> Self {
        Self::new()
    }
}
