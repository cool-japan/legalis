//! Intelligent assistant: usage learning, contextual suggestions, and
//! proactive recommendations.
//!
//! This module persists a small, local-only model of how the user drives the
//! CLI and uses it for three things:
//!
//! - **Learning from user patterns** ([`UsageStats`]): per-command counts, a
//!   first-order Markov transition table (`previous -> next`), and recency.
//! - **Contextual command suggestions** ([`UsageStats::suggest_next`]): rank the
//!   most likely follow-up commands given the previous command, blending the
//!   learned transition frequencies with a curated static workflow graph.
//! - **Proactive recommendations** ([`recommendations`]): surface actionable
//!   advice derived from usage and project state (e.g. "you parse a lot but
//!   never verify — try `legalis verify`").
//!
//! The store is JSON-backed under the data directory; tests redirect it via
//! `LEGALIS_DATA_DIR`.

use crate::paths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A curated static "what usually comes next" graph, used to seed suggestions
/// before enough history accumulates and to blend with learned data.
const STATIC_FOLLOWUPS: &[(&str, &[&str])] = &[
    ("new", &["verify", "lint", "format"]),
    ("init", &["new", "verify"]),
    ("parse", &["verify", "viz", "explain"]),
    ("verify", &["simulate", "audit", "publish"]),
    ("lint", &["format", "verify"]),
    ("format", &["verify", "lint"]),
    ("simulate", &["audit", "complexity"]),
    ("import", &["validate", "verify", "convert"]),
    ("convert", &["validate", "verify"]),
    ("audit", &["complexity", "explain"]),
    ("install", &["list", "verify"]),
    ("search", &["install"]),
    ("publish", &["list", "registry"]),
];

/// A scored suggestion with a short rationale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// The suggested command name.
    pub command: String,
    /// A relative score (higher is more likely); not normalized.
    pub score: u64,
    /// Why it was suggested.
    pub reason: String,
}

/// A proactive recommendation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommendation {
    /// A short title.
    pub title: String,
    /// The actionable detail.
    pub detail: String,
}

/// Persistent, local usage statistics powering the assistant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageStats {
    /// Total invocation count per command.
    #[serde(default)]
    pub counts: HashMap<String, u64>,
    /// First-order transitions: `previous -> (next -> count)`.
    #[serde(default)]
    pub transitions: HashMap<String, HashMap<String, u64>>,
    /// The command recorded in the previous invocation (for chaining).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_command: Option<String>,
    /// RFC 3339 timestamp of the last recorded command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// Total number of recorded invocations.
    #[serde(default)]
    pub total: u64,
}

impl UsageStats {
    /// Loads the default usage-stats store, tolerating a missing/corrupt file.
    pub fn load() -> Result<Self> {
        let path = paths::usage_stats_path()?;
        Self::load_from(&path)
    }

    /// Loads usage stats from a path; a missing file yields defaults and a
    /// corrupt file is treated as empty (so the assistant never blocks the CLI).
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read usage stats: {}", path.display()))?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    }

    /// Persists usage stats to the default store.
    pub fn save(&self) -> Result<()> {
        let path = paths::usage_stats_path()?;
        self.save_to(&path)
    }

    /// Persists usage stats to a path.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create usage stats dir: {}", parent.display())
            })?;
        }
        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize usage stats")?;
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write usage stats: {}", path.display()))?;
        Ok(())
    }

    /// Records an invocation of `command`, updating counts, the transition table
    /// (chaining from [`Self::last_command`]), recency, and the running total.
    pub fn record(&mut self, command: &str) {
        *self.counts.entry(command.to_string()).or_insert(0) += 1;
        self.total += 1;
        if let Some(prev) = self.last_command.clone() {
            *self
                .transitions
                .entry(prev)
                .or_default()
                .entry(command.to_string())
                .or_insert(0) += 1;
        }
        self.last_command = Some(command.to_string());
        self.last_seen = Some(chrono::Utc::now().to_rfc3339());
    }

    /// The number of distinct commands ever used.
    pub fn distinct_commands(&self) -> usize {
        self.counts.len()
    }

    /// The most-used commands, descending by count, capped at `limit`.
    pub fn top_commands(&self, limit: usize) -> Vec<(String, u64)> {
        let mut entries: Vec<(String, u64)> =
            self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        entries.truncate(limit);
        entries
    }

    /// Suggests likely next commands given an optional previous command.
    ///
    /// Scores blend three signals (weighted): learned transitions from
    /// `previous`, the curated static follow-up graph, and overall popularity.
    /// The previous command itself is never suggested. Results are sorted by
    /// score descending and capped at `limit`.
    pub fn suggest_next(&self, previous: Option<&str>, limit: usize) -> Vec<Suggestion> {
        let mut scores: HashMap<String, u64> = HashMap::new();
        let mut reasons: HashMap<String, String> = HashMap::new();

        let prev = previous.or(self.last_command.as_deref());

        // Signal 1: learned transitions (weight 5).
        if let Some(p) = prev
            && let Some(next_map) = self.transitions.get(p)
        {
            for (next, count) in next_map {
                *scores.entry(next.clone()).or_insert(0) += count * 5;
                reasons
                    .entry(next.clone())
                    .or_insert_with(|| format!("often follows '{p}'"));
            }
        }

        // Signal 2: static workflow graph (weight 3, decaying by position).
        if let Some(p) = prev
            && let Some((_, followups)) = STATIC_FOLLOWUPS.iter().find(|(cmd, _)| *cmd == p)
        {
            for (index, next) in followups.iter().enumerate() {
                let weight = 3u64.saturating_sub(index as u64).max(1);
                *scores.entry((*next).to_string()).or_insert(0) += weight;
                reasons
                    .entry((*next).to_string())
                    .or_insert_with(|| format!("common after '{p}'"));
            }
        }

        // Signal 3: overall popularity (weight 1) as a tie-breaker / cold-start.
        for (command, count) in &self.counts {
            *scores.entry(command.clone()).or_insert(0) += (*count).min(3);
            reasons
                .entry(command.clone())
                .or_insert_with(|| "frequently used".to_string());
        }

        // Never suggest the previous command back to the user.
        if let Some(p) = prev {
            scores.remove(p);
        }

        let mut suggestions: Vec<Suggestion> = scores
            .into_iter()
            .map(|(command, score)| {
                let reason = reasons
                    .remove(&command)
                    .unwrap_or_else(|| "suggested".to_string());
                Suggestion {
                    command,
                    score,
                    reason,
                }
            })
            .collect();
        suggestions.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| a.command.cmp(&b.command))
        });
        suggestions.truncate(limit);
        suggestions
    }
}

/// Produces proactive recommendations from usage stats and the current project
/// state. Pure given its inputs, so it is straightforward to test.
pub fn recommendations(stats: &UsageStats, in_project: bool) -> Vec<Recommendation> {
    let mut recs = Vec::new();

    let count = |c: &str| stats.counts.get(c).copied().unwrap_or(0);

    // Parses a lot but rarely verifies.
    let parses = count("parse") + count("import");
    if parses >= 3 && count("verify") == 0 {
        recs.push(Recommendation {
            title: "Verify your statutes".to_string(),
            detail: "You parse statutes often but have not run `legalis verify`. Verifying catches logical inconsistencies early.".to_string(),
        });
    }

    // Edits/formats but never lints.
    if count("format") >= 3 && count("lint") == 0 {
        recs.push(Recommendation {
            title: "Lint for best practices".to_string(),
            detail: "You format frequently; `legalis lint` additionally flags style and best-practice issues.".to_string(),
        });
    }

    // Verifies a lot but never simulates.
    if count("verify") >= 5 && count("simulate") == 0 {
        recs.push(Recommendation {
            title: "Try a simulation".to_string(),
            detail: "With many verifications under your belt, `legalis simulate` can show how a statute behaves across a population.".to_string(),
        });
    }

    // Heavy user without aliases is a candidate for shell completions/aliases.
    if stats.total >= 25 {
        recs.push(Recommendation {
            title: "Speed up with completions".to_string(),
            detail: "You are a frequent user — install shell completions with `legalis completions <shell>` to type less.".to_string(),
        });
    }

    // No project detected.
    if !in_project && stats.total >= 3 {
        recs.push(Recommendation {
            title: "Initialize a project".to_string(),
            detail: "No legalis.toml was found here. `legalis init` scaffolds a project for repeatable configuration.".to_string(),
        });
    }

    recs
}

/// Returns the default usage-stats path (for display/diagnostics).
pub fn stats_path() -> Result<PathBuf> {
    paths::usage_stats_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path() -> PathBuf {
        std::env::temp_dir().join(format!("legalis-usage-{}.json", uuid::Uuid::new_v4()))
    }

    #[test]
    fn test_record_counts_and_total() {
        let mut stats = UsageStats::default();
        stats.record("verify");
        stats.record("verify");
        stats.record("lint");
        assert_eq!(stats.counts.get("verify"), Some(&2));
        assert_eq!(stats.counts.get("lint"), Some(&1));
        assert_eq!(stats.total, 3);
        assert_eq!(stats.distinct_commands(), 2);
        assert_eq!(stats.last_command.as_deref(), Some("lint"));
    }

    #[test]
    fn test_transitions_recorded() {
        let mut stats = UsageStats::default();
        stats.record("new");
        stats.record("verify");
        stats.record("new");
        stats.record("verify");
        let from_new = stats.transitions.get("new").expect("new transitions");
        assert_eq!(from_new.get("verify"), Some(&2));
    }

    #[test]
    fn test_top_commands() {
        let mut stats = UsageStats::default();
        for _ in 0..5 {
            stats.record("verify");
        }
        for _ in 0..2 {
            stats.record("lint");
        }
        stats.record("format");
        let top = stats.top_commands(2);
        assert_eq!(top[0].0, "verify");
        assert_eq!(top[0].1, 5);
        assert_eq!(top[1].0, "lint");
    }

    #[test]
    fn test_suggest_uses_learned_transitions() {
        let mut stats = UsageStats::default();
        // Teach: after `verify`, the user almost always runs `publish`.
        for _ in 0..10 {
            stats.record("verify");
            stats.record("publish");
        }
        let suggestions = stats.suggest_next(Some("verify"), 3);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].command, "publish");
        // The previous command must not be suggested back.
        assert!(suggestions.iter().all(|s| s.command != "verify"));
    }

    #[test]
    fn test_suggest_cold_start_uses_static_graph() {
        let stats = UsageStats::default();
        let suggestions = stats.suggest_next(Some("parse"), 5);
        // From the static graph, `verify` is a follow-up of `parse`.
        assert!(suggestions.iter().any(|s| s.command == "verify"));
    }

    #[test]
    fn test_suggest_limit_respected() {
        let mut stats = UsageStats::default();
        for cmd in ["a", "b", "c", "d", "e", "f"] {
            stats.record(cmd);
        }
        let suggestions = stats.suggest_next(None, 3);
        assert!(suggestions.len() <= 3);
    }

    #[test]
    fn test_persistence_roundtrip() {
        let path = temp_path();
        let mut stats = UsageStats::default();
        stats.record("verify");
        stats.record("simulate");
        stats.save_to(&path).expect("save");
        let loaded = UsageStats::load_from(&path).expect("load");
        assert_eq!(loaded.total, 2);
        assert_eq!(loaded.counts.get("verify"), Some(&1));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_missing_is_default() {
        let path = temp_path();
        let stats = UsageStats::load_from(&path).expect("load missing");
        assert_eq!(stats.total, 0);
    }

    #[test]
    fn test_load_corrupt_is_default() {
        let path = temp_path();
        std::fs::write(&path, "{ not valid json").expect("write");
        let stats = UsageStats::load_from(&path).expect("load corrupt");
        assert_eq!(stats.total, 0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_recommendations_verify() {
        let mut stats = UsageStats::default();
        for _ in 0..4 {
            stats.record("parse");
        }
        let recs = recommendations(&stats, true);
        assert!(recs.iter().any(|r| r.title.contains("Verify")));
    }

    #[test]
    fn test_recommendations_init_when_no_project() {
        let mut stats = UsageStats::default();
        for _ in 0..3 {
            stats.record("verify");
        }
        let recs = recommendations(&stats, false);
        assert!(recs.iter().any(|r| r.title.contains("Initialize")));
        // In a project, that recommendation should not appear.
        let recs_in = recommendations(&stats, true);
        assert!(!recs_in.iter().any(|r| r.title.contains("Initialize")));
    }

    #[test]
    fn test_recommendations_empty_when_quiet_usage() {
        let stats = UsageStats::default();
        let recs = recommendations(&stats, true);
        assert!(recs.is_empty());
    }
}
