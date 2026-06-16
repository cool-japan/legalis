//! API usage policies: definition and enforcement.
//!
//! Defines declarative usage policies that constrain how API consumers may use
//! the service: per-window request quotas, allowed HTTP methods, allowed path
//! prefixes, payload size caps, and required scopes. A [`PolicySet`] evaluates an
//! incoming [`RequestContext`] against the policy bound to its subject (API key /
//! plan / tenant) and returns an allow/deny decision with a reason, plus the
//! remaining quota for the active window.
//!
//! Quota accounting is windowed and rolls over automatically when the window
//! elapses, implemented with monotonic wall-clock arithmetic.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A declarative usage policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePolicy {
    /// Stable policy name (e.g. plan tier).
    pub name: String,
    /// Maximum requests allowed per quota window.
    pub max_requests_per_window: u64,
    /// Quota window length in seconds.
    pub window_secs: i64,
    /// Allowed HTTP methods (empty = all allowed).
    pub allowed_methods: HashSet<String>,
    /// Allowed path prefixes (empty = all allowed).
    pub allowed_path_prefixes: Vec<String>,
    /// Maximum request body size in bytes (None = unlimited).
    pub max_body_bytes: Option<usize>,
    /// Scopes the subject must hold for any request under this policy.
    pub required_scopes: HashSet<String>,
}

impl UsagePolicy {
    /// Creates a policy with a quota and otherwise unrestricted settings.
    pub fn new(name: impl Into<String>, max_requests_per_window: u64, window_secs: i64) -> Self {
        Self {
            name: name.into(),
            max_requests_per_window,
            window_secs: window_secs.max(1),
            allowed_methods: HashSet::new(),
            allowed_path_prefixes: Vec::new(),
            max_body_bytes: None,
            required_scopes: HashSet::new(),
        }
    }

    /// Restricts the allowed HTTP methods.
    pub fn with_methods<I, S>(mut self, methods: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.allowed_methods = methods
            .into_iter()
            .map(|m| m.into().to_uppercase())
            .collect();
        self
    }

    /// Restricts the allowed path prefixes.
    pub fn with_path_prefixes<I, S>(mut self, prefixes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.allowed_path_prefixes = prefixes.into_iter().map(|p| p.into()).collect();
        self
    }

    /// Caps the maximum request body size.
    pub fn with_max_body_bytes(mut self, bytes: usize) -> Self {
        self.max_body_bytes = Some(bytes);
        self
    }

    /// Requires the given scopes.
    pub fn with_required_scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.required_scopes = scopes.into_iter().map(|s| s.into()).collect();
        self
    }
}

/// Context describing an incoming request for policy evaluation.
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Subject identifier (API key id / tenant / plan).
    pub subject: String,
    /// HTTP method (e.g. "GET").
    pub method: String,
    /// Request path.
    pub path: String,
    /// Body size in bytes, if known.
    pub body_bytes: Option<usize>,
    /// Scopes the subject holds.
    pub scopes: HashSet<String>,
}

impl RequestContext {
    /// Creates a minimal request context.
    pub fn new(
        subject: impl Into<String>,
        method: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            method: method.into(),
            path: path.into(),
            body_bytes: None,
            scopes: HashSet::new(),
        }
    }

    /// Sets the body size.
    pub fn with_body_bytes(mut self, bytes: usize) -> Self {
        self.body_bytes = Some(bytes);
        self
    }

    /// Sets the held scopes.
    pub fn with_scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.scopes = scopes.into_iter().map(|s| s.into()).collect();
        self
    }
}

/// The reason a request was denied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "reason")]
pub enum DenyReason {
    /// No policy is bound to the subject.
    NoPolicy,
    /// The quota for the current window is exhausted.
    QuotaExceeded {
        /// The configured limit.
        limit: u64,
    },
    /// The HTTP method is not permitted.
    MethodNotAllowed {
        /// The offending method.
        method: String,
    },
    /// The path is not permitted.
    PathNotAllowed {
        /// The offending path.
        path: String,
    },
    /// The request body exceeds the configured cap.
    BodyTooLarge {
        /// The configured cap.
        limit: usize,
    },
    /// A required scope is missing.
    MissingScope {
        /// The missing scope.
        scope: String,
    },
}

/// The outcome of evaluating a request against the policy set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    /// Whether the request is allowed.
    pub allowed: bool,
    /// Deny reason when `allowed` is false.
    pub deny_reason: Option<DenyReason>,
    /// Requests remaining in the current window after this request (when allowed).
    pub remaining: Option<u64>,
    /// When the current quota window resets.
    pub window_reset: Option<DateTime<Utc>>,
}

impl PolicyDecision {
    fn allow(remaining: u64, reset: DateTime<Utc>) -> Self {
        Self {
            allowed: true,
            deny_reason: None,
            remaining: Some(remaining),
            window_reset: Some(reset),
        }
    }

    fn deny(reason: DenyReason) -> Self {
        Self {
            allowed: false,
            deny_reason: Some(reason),
            remaining: None,
            window_reset: None,
        }
    }
}

/// Per-subject quota accounting window.
#[derive(Debug, Clone)]
struct QuotaWindow {
    count: u64,
    window_start: DateTime<Utc>,
}

/// A set of policies plus live quota accounting.
#[derive(Clone, Default)]
pub struct PolicySet {
    inner: Arc<RwLock<PolicyState>>,
}

#[derive(Default)]
struct PolicyState {
    /// Policy bound to each subject.
    bindings: HashMap<String, UsagePolicy>,
    /// Live quota windows per subject.
    quotas: HashMap<String, QuotaWindow>,
}

impl PolicySet {
    /// Creates an empty policy set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds a policy to a subject (overwriting any existing binding).
    pub async fn bind(&self, subject: impl Into<String>, policy: UsagePolicy) {
        let mut state = self.inner.write().await;
        state.bindings.insert(subject.into(), policy);
    }

    /// Returns the policy bound to a subject, if any.
    pub async fn policy_for(&self, subject: &str) -> Option<UsagePolicy> {
        self.inner.read().await.bindings.get(subject).cloned()
    }

    /// Checks static constraints (method/path/body/scope) without consuming
    /// quota. Returns `None` when the request satisfies all static constraints.
    fn check_static(ctx: &RequestContext, policy: &UsagePolicy) -> Option<DenyReason> {
        if !policy.allowed_methods.is_empty()
            && !policy.allowed_methods.contains(&ctx.method.to_uppercase())
        {
            return Some(DenyReason::MethodNotAllowed {
                method: ctx.method.clone(),
            });
        }
        if !policy.allowed_path_prefixes.is_empty()
            && !policy
                .allowed_path_prefixes
                .iter()
                .any(|p| ctx.path.starts_with(p))
        {
            return Some(DenyReason::PathNotAllowed {
                path: ctx.path.clone(),
            });
        }
        if let (Some(limit), Some(body)) = (policy.max_body_bytes, ctx.body_bytes)
            && body > limit
        {
            return Some(DenyReason::BodyTooLarge { limit });
        }
        for scope in &policy.required_scopes {
            if !ctx.scopes.contains(scope) {
                return Some(DenyReason::MissingScope {
                    scope: scope.clone(),
                });
            }
        }
        None
    }

    /// Evaluates a request at `now`, consuming one unit of quota if allowed.
    pub async fn evaluate(&self, ctx: &RequestContext, now: DateTime<Utc>) -> PolicyDecision {
        let mut state = self.inner.write().await;
        let policy = match state.bindings.get(&ctx.subject).cloned() {
            Some(p) => p,
            None => return PolicyDecision::deny(DenyReason::NoPolicy),
        };

        if let Some(reason) = Self::check_static(ctx, &policy) {
            return PolicyDecision::deny(reason);
        }

        // Windowed quota accounting.
        let window = state
            .quotas
            .entry(ctx.subject.clone())
            .or_insert_with(|| QuotaWindow {
                count: 0,
                window_start: now,
            });

        // Roll the window over if it has elapsed.
        let elapsed = (now - window.window_start).num_seconds();
        if elapsed >= policy.window_secs {
            window.count = 0;
            window.window_start = now;
        }

        if window.count >= policy.max_requests_per_window {
            return PolicyDecision::deny(DenyReason::QuotaExceeded {
                limit: policy.max_requests_per_window,
            });
        }

        window.count += 1;
        let remaining = policy.max_requests_per_window - window.count;
        let reset = window.window_start + chrono::Duration::seconds(policy.window_secs);
        PolicyDecision::allow(remaining, reset)
    }

    /// Returns the quota usage `(used, limit)` for a subject at `now` without
    /// consuming quota.
    pub async fn usage(&self, subject: &str, now: DateTime<Utc>) -> Option<(u64, u64)> {
        let state = self.inner.read().await;
        let policy = state.bindings.get(subject)?;
        let used = state
            .quotas
            .get(subject)
            .map(|w| {
                if (now - w.window_start).num_seconds() >= policy.window_secs {
                    0
                } else {
                    w.count
                }
            })
            .unwrap_or(0);
        Some((used, policy.max_requests_per_window))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_no_policy_denied() {
        let set = PolicySet::new();
        let ctx = RequestContext::new("k1", "GET", "/api/v1/statutes");
        let decision = set.evaluate(&ctx, Utc::now()).await;
        assert!(!decision.allowed);
        assert_eq!(decision.deny_reason, Some(DenyReason::NoPolicy));
    }

    #[tokio::test]
    async fn test_quota_enforced() {
        let set = PolicySet::new();
        set.bind("k1", UsagePolicy::new("basic", 2, 60)).await;
        let ctx = RequestContext::new("k1", "GET", "/x");
        let now = Utc::now();

        let d1 = set.evaluate(&ctx, now).await;
        assert!(d1.allowed);
        assert_eq!(d1.remaining, Some(1));
        let d2 = set.evaluate(&ctx, now).await;
        assert!(d2.allowed);
        assert_eq!(d2.remaining, Some(0));
        let d3 = set.evaluate(&ctx, now).await;
        assert!(!d3.allowed);
        assert_eq!(d3.deny_reason, Some(DenyReason::QuotaExceeded { limit: 2 }));
    }

    #[tokio::test]
    async fn test_window_rollover() {
        let set = PolicySet::new();
        set.bind("k1", UsagePolicy::new("basic", 1, 10)).await;
        let ctx = RequestContext::new("k1", "GET", "/x");
        let now = Utc::now();

        assert!(set.evaluate(&ctx, now).await.allowed);
        assert!(!set.evaluate(&ctx, now).await.allowed);
        // After the window elapses, quota resets.
        let later = now + chrono::Duration::seconds(11);
        assert!(set.evaluate(&ctx, later).await.allowed);
    }

    #[tokio::test]
    async fn test_method_restriction() {
        let set = PolicySet::new();
        set.bind("k1", UsagePolicy::new("ro", 100, 60).with_methods(["GET"]))
            .await;
        let now = Utc::now();
        let get = RequestContext::new("k1", "GET", "/x");
        assert!(set.evaluate(&get, now).await.allowed);
        let post = RequestContext::new("k1", "POST", "/x");
        let d = set.evaluate(&post, now).await;
        assert_eq!(
            d.deny_reason,
            Some(DenyReason::MethodNotAllowed {
                method: "POST".to_string()
            })
        );
    }

    #[tokio::test]
    async fn test_path_restriction() {
        let set = PolicySet::new();
        set.bind(
            "k1",
            UsagePolicy::new("scoped", 100, 60).with_path_prefixes(["/api/v1/statutes"]),
        )
        .await;
        let now = Utc::now();
        let ok = RequestContext::new("k1", "GET", "/api/v1/statutes/123");
        assert!(set.evaluate(&ok, now).await.allowed);
        let bad = RequestContext::new("k1", "GET", "/api/v1/admin");
        let d = set.evaluate(&bad, now).await;
        assert!(matches!(
            d.deny_reason,
            Some(DenyReason::PathNotAllowed { .. })
        ));
    }

    #[tokio::test]
    async fn test_body_size_cap() {
        let set = PolicySet::new();
        set.bind(
            "k1",
            UsagePolicy::new("small", 100, 60).with_max_body_bytes(1024),
        )
        .await;
        let now = Utc::now();
        let ok = RequestContext::new("k1", "POST", "/x").with_body_bytes(500);
        assert!(set.evaluate(&ok, now).await.allowed);
        let big = RequestContext::new("k1", "POST", "/x").with_body_bytes(2048);
        let d = set.evaluate(&big, now).await;
        assert_eq!(
            d.deny_reason,
            Some(DenyReason::BodyTooLarge { limit: 1024 })
        );
    }

    #[tokio::test]
    async fn test_scope_requirement() {
        let set = PolicySet::new();
        set.bind(
            "k1",
            UsagePolicy::new("scoped", 100, 60).with_required_scopes(["write"]),
        )
        .await;
        let now = Utc::now();
        let missing = RequestContext::new("k1", "POST", "/x");
        let d = set.evaluate(&missing, now).await;
        assert_eq!(
            d.deny_reason,
            Some(DenyReason::MissingScope {
                scope: "write".to_string()
            })
        );
        let ok = RequestContext::new("k1", "POST", "/x").with_scopes(["write", "read"]);
        assert!(set.evaluate(&ok, now).await.allowed);
    }

    #[tokio::test]
    async fn test_usage_query() {
        let set = PolicySet::new();
        set.bind("k1", UsagePolicy::new("basic", 5, 60)).await;
        let now = Utc::now();
        let ctx = RequestContext::new("k1", "GET", "/x");
        set.evaluate(&ctx, now).await;
        set.evaluate(&ctx, now).await;
        assert_eq!(set.usage("k1", now).await, Some((2, 5)));
        assert_eq!(set.usage("unknown", now).await, None);
    }

    #[tokio::test]
    async fn test_static_checks_do_not_consume_quota() {
        let set = PolicySet::new();
        set.bind("k1", UsagePolicy::new("ro", 1, 60).with_methods(["GET"]))
            .await;
        let now = Utc::now();
        // A denied-by-method request should not consume quota.
        let post = RequestContext::new("k1", "POST", "/x");
        assert!(!set.evaluate(&post, now).await.allowed);
        // The single GET allowance should still be available.
        let get = RequestContext::new("k1", "GET", "/x");
        assert!(set.evaluate(&get, now).await.allowed);
    }

    #[tokio::test]
    async fn test_policy_for() {
        let set = PolicySet::new();
        set.bind("k1", UsagePolicy::new("tier", 10, 30)).await;
        let p = set.policy_for("k1").await.expect("policy");
        assert_eq!(p.name, "tier");
        assert!(set.policy_for("nope").await.is_none());
    }
}
