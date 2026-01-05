//! Integration features for external systems and workflows.
//!
//! This module provides:
//! - Git integration for version control
//! - GitHub/GitLab PR diff integration
//! - Notification webhooks for changes
//! - Diff-based CI/CD triggers
//! - Diff API for external tools

use crate::{DiffError, DiffResult, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Webhook notification payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    /// Event type
    pub event: WebhookEvent,
    /// Diff data
    pub diff: StatuteDiff,
    /// Timestamp of the event
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of webhook events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebhookEvent {
    /// Diff was created
    DiffCreated,
    /// Diff was updated
    DiffUpdated,
    /// Breaking change detected
    BreakingChange,
    /// Major impact detected
    MajorImpact,
    /// Change approved
    ChangeApproved,
    /// Change rejected
    ChangeRejected,
    /// Change enacted
    ChangeEnacted,
}

impl WebhookPayload {
    /// Creates a new webhook payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, integration::{WebhookPayload, WebhookEvent}};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let payload = WebhookPayload::new(WebhookEvent::DiffCreated, diff_result);
    /// assert_eq!(payload.event, WebhookEvent::DiffCreated);
    /// ```
    pub fn new(event: WebhookEvent, diff: StatuteDiff) -> Self {
        Self {
            event,
            diff,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the payload.
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Converts the payload to JSON.
    pub fn to_json(&self) -> DiffResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| DiffError::SerializationError(e.to_string()))
    }
}

/// CI/CD trigger configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiCdTrigger {
    /// Trigger name
    pub name: String,
    /// Condition for triggering
    pub condition: TriggerCondition,
    /// Actions to execute
    pub actions: Vec<TriggerAction>,
    /// Whether the trigger is enabled
    pub enabled: bool,
}

/// Conditions that can trigger a CI/CD action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Any change detected
    AnyChange,
    /// Breaking change detected
    BreakingChange,
    /// Severity threshold met
    SeverityThreshold(crate::Severity),
    /// Specific change type
    ChangeType(crate::ChangeType),
    /// Affects eligibility
    AffectsEligibility,
    /// Affects outcome
    AffectsOutcome,
    /// Custom condition (script or expression)
    Custom(String),
}

/// Actions to execute when a trigger is activated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerAction {
    /// Run a shell command
    RunCommand(String),
    /// Send a webhook notification
    SendWebhook { url: String, event: WebhookEvent },
    /// Create a Git commit
    CreateCommit { message: String },
    /// Create a pull request
    CreatePullRequest {
        title: String,
        description: String,
        target_branch: String,
    },
    /// Send an email notification
    SendEmail { to: Vec<String>, subject: String },
    /// Fail the build
    FailBuild { reason: String },
}

impl CiCdTrigger {
    /// Creates a new CI/CD trigger.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::integration::{CiCdTrigger, TriggerCondition};
    ///
    /// let trigger = CiCdTrigger::new("breaking-change-check", TriggerCondition::BreakingChange);
    /// assert_eq!(trigger.name, "breaking-change-check");
    /// assert!(trigger.enabled);
    /// ```
    pub fn new(name: impl Into<String>, condition: TriggerCondition) -> Self {
        Self {
            name: name.into(),
            condition,
            actions: Vec::new(),
            enabled: true,
        }
    }

    /// Adds an action to the trigger.
    pub fn add_action(mut self, action: TriggerAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Sets whether the trigger is enabled.
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Evaluates whether this trigger should fire for a given diff.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, integration::{CiCdTrigger, TriggerCondition}};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let mut new = old.clone();
    /// new.effect = Effect::new(EffectType::Revoke, "Revoke");
    ///
    /// let diff_result = diff(&old, &new).unwrap();
    /// let trigger = CiCdTrigger::new("breaking-check", TriggerCondition::BreakingChange);
    ///
    /// assert!(trigger.should_trigger(&diff_result));
    /// ```
    pub fn should_trigger(&self, diff: &StatuteDiff) -> bool {
        if !self.enabled {
            return false;
        }

        match &self.condition {
            TriggerCondition::AnyChange => !diff.changes.is_empty(),
            TriggerCondition::BreakingChange => {
                diff.impact.severity >= crate::Severity::Major
                    || diff.impact.affects_outcome
                    || diff.impact.discretion_changed
            }
            TriggerCondition::SeverityThreshold(threshold) => diff.impact.severity >= *threshold,
            TriggerCondition::ChangeType(change_type) => {
                diff.changes.iter().any(|c| c.change_type == *change_type)
            }
            TriggerCondition::AffectsEligibility => diff.impact.affects_eligibility,
            TriggerCondition::AffectsOutcome => diff.impact.affects_outcome,
            TriggerCondition::Custom(_) => {
                // Custom conditions would need to be evaluated by an external script
                false
            }
        }
    }
}

/// GitHub/GitLab pull request integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestIntegration {
    /// PR number
    pub pr_number: u64,
    /// Repository name (e.g., "owner/repo")
    pub repository: String,
    /// Platform (GitHub or GitLab)
    pub platform: Platform,
    /// Base branch
    pub base_branch: String,
    /// Head branch
    pub head_branch: String,
}

/// Git hosting platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    GitHub,
    GitLab,
}

impl PullRequestIntegration {
    /// Creates a new PR integration.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::integration::{PullRequestIntegration, Platform};
    ///
    /// let pr = PullRequestIntegration::new(
    ///     42,
    ///     "owner/repo",
    ///     Platform::GitHub,
    ///     "main",
    ///     "feature-branch"
    /// );
    /// assert_eq!(pr.pr_number, 42);
    /// ```
    pub fn new(
        pr_number: u64,
        repository: impl Into<String>,
        platform: Platform,
        base_branch: impl Into<String>,
        head_branch: impl Into<String>,
    ) -> Self {
        Self {
            pr_number,
            repository: repository.into(),
            platform,
            base_branch: base_branch.into(),
            head_branch: head_branch.into(),
        }
    }

    /// Generates a PR comment with diff summary.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, integration::{PullRequestIntegration, Platform}};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let pr = PullRequestIntegration::new(42, "owner/repo", Platform::GitHub, "main", "feature");
    /// let comment = pr.generate_comment(&diff_result);
    ///
    /// assert!(comment.contains("Statute Diff Summary"));
    /// ```
    pub fn generate_comment(&self, diff: &StatuteDiff) -> String {
        let mut comment = String::new();

        comment.push_str("## Statute Diff Summary\n\n");
        comment.push_str(&format!("**Statute:** `{}`\n", diff.statute_id));
        comment.push_str(&format!("**Changes:** {}\n", diff.changes.len()));
        comment.push_str(&format!("**Severity:** {:?}\n\n", diff.impact.severity));

        if diff.impact.severity >= crate::Severity::Major {
            comment.push_str("⚠️ **Warning:** This change has major impact!\n\n");
        }

        comment.push_str("### Impact\n\n");
        comment.push_str(&format!(
            "- Affects Eligibility: {}\n",
            if diff.impact.affects_eligibility {
                "✅ Yes"
            } else {
                "❌ No"
            }
        ));
        comment.push_str(&format!(
            "- Affects Outcome: {}\n",
            if diff.impact.affects_outcome {
                "✅ Yes"
            } else {
                "❌ No"
            }
        ));
        comment.push_str(&format!(
            "- Discretion Changed: {}\n",
            if diff.impact.discretion_changed {
                "✅ Yes"
            } else {
                "❌ No"
            }
        ));

        if !diff.changes.is_empty() {
            comment.push_str("\n### Changes\n\n");
            for change in &diff.changes {
                comment.push_str(&format!(
                    "- **{:?}** {}: {}\n",
                    change.change_type, change.target, change.description
                ));
            }
        }

        if !diff.impact.notes.is_empty() {
            comment.push_str("\n### Notes\n\n");
            for note in &diff.impact.notes {
                comment.push_str(&format!("- {}\n", note));
            }
        }

        comment
    }

    /// Gets the PR URL.
    pub fn get_url(&self) -> String {
        match self.platform {
            Platform::GitHub => {
                format!(
                    "https://github.com/{}/pull/{}",
                    self.repository, self.pr_number
                )
            }
            Platform::GitLab => {
                format!(
                    "https://gitlab.com/{}/-/merge_requests/{}",
                    self.repository, self.pr_number
                )
            }
        }
    }
}

/// API endpoint for external tools.
#[derive(Debug, Clone)]
pub struct DiffApi {
    /// Base URL for the API
    pub base_url: String,
    /// API key for authentication
    pub api_key: Option<String>,
}

impl DiffApi {
    /// Creates a new diff API client.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::integration::DiffApi;
    ///
    /// let api = DiffApi::new("https://api.example.com");
    /// assert_eq!(api.base_url, "https://api.example.com");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
        }
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Generates a REST API endpoint for diff operations.
    pub fn get_endpoint(&self, operation: &str) -> String {
        format!("{}/diff/{}", self.base_url, operation)
    }

    /// Generates headers for API requests.
    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        if let Some(key) = &self.api_key {
            headers.insert("Authorization".to_string(), format!("Bearer {}", key));
        }

        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_diff() -> StatuteDiff {
        let old = Statute::new(
            "test-law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let new = Statute::new(
            "test-law",
            "New Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        diff(&old, &new).unwrap()
    }

    fn breaking_diff() -> StatuteDiff {
        let old = Statute::new("test-law", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");
        diff(&old, &new).unwrap()
    }

    #[test]
    fn test_webhook_payload() {
        let diff = test_diff();
        let payload =
            WebhookPayload::new(WebhookEvent::DiffCreated, diff).add_metadata("source", "test");

        assert_eq!(payload.event, WebhookEvent::DiffCreated);
        assert_eq!(payload.metadata.get("source"), Some(&"test".to_string()));
    }

    #[test]
    fn test_webhook_payload_to_json() {
        let diff = test_diff();
        let payload = WebhookPayload::new(WebhookEvent::DiffCreated, diff);
        let json = payload.to_json().unwrap();

        assert!(json.contains("DiffCreated"));
        assert!(json.contains("test-law"));
    }

    #[test]
    fn test_cicd_trigger_basic() {
        let trigger = CiCdTrigger::new("test", TriggerCondition::AnyChange);
        assert_eq!(trigger.name, "test");
        assert!(trigger.enabled);
    }

    #[test]
    fn test_cicd_trigger_any_change() {
        let diff = test_diff();
        let trigger = CiCdTrigger::new("test", TriggerCondition::AnyChange);

        assert!(trigger.should_trigger(&diff));
    }

    #[test]
    fn test_cicd_trigger_breaking_change() {
        let diff = breaking_diff();
        let trigger = CiCdTrigger::new("test", TriggerCondition::BreakingChange);

        assert!(trigger.should_trigger(&diff));
    }

    #[test]
    fn test_cicd_trigger_severity_threshold() {
        let diff = breaking_diff();
        let trigger = CiCdTrigger::new(
            "test",
            TriggerCondition::SeverityThreshold(crate::Severity::Major),
        );

        assert!(trigger.should_trigger(&diff));
    }

    #[test]
    fn test_cicd_trigger_disabled() {
        let diff = test_diff();
        let trigger = CiCdTrigger::new("test", TriggerCondition::AnyChange).set_enabled(false);

        assert!(!trigger.should_trigger(&diff));
    }

    #[test]
    fn test_pr_integration_github() {
        let pr = PullRequestIntegration::new(42, "owner/repo", Platform::GitHub, "main", "feature");

        assert_eq!(pr.pr_number, 42);
        assert_eq!(pr.platform, Platform::GitHub);
        assert!(pr.get_url().contains("github.com"));
    }

    #[test]
    fn test_pr_integration_gitlab() {
        let pr = PullRequestIntegration::new(42, "owner/repo", Platform::GitLab, "main", "feature");

        assert_eq!(pr.platform, Platform::GitLab);
        assert!(pr.get_url().contains("gitlab.com"));
    }

    #[test]
    fn test_pr_comment_generation() {
        let diff = test_diff();
        let pr = PullRequestIntegration::new(42, "owner/repo", Platform::GitHub, "main", "feature");

        let comment = pr.generate_comment(&diff);

        assert!(comment.contains("Statute Diff Summary"));
        assert!(comment.contains("test-law"));
    }

    #[test]
    fn test_diff_api() {
        let api = DiffApi::new("https://api.example.com").with_api_key("test-key");

        assert_eq!(api.base_url, "https://api.example.com");
        assert_eq!(api.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_diff_api_endpoints() {
        let api = DiffApi::new("https://api.example.com");
        let endpoint = api.get_endpoint("compare");

        assert_eq!(endpoint, "https://api.example.com/diff/compare");
    }

    #[test]
    fn test_diff_api_headers() {
        let api = DiffApi::new("https://api.example.com").with_api_key("test-key");

        let headers = api.get_headers();

        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer test-key".to_string())
        );
    }
}
