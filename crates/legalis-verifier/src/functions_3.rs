//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::functions::{analyze_complexity, detect_statute_conflicts};
use super::functions_2::verify_ctl;
use super::functions_2::{check_ctl_star_state, extract_statute_references_from_conditions};
use super::types::{NotificationMessage, QualityMetrics};
#[cfg(feature = "smt-solver")]
use super::types_3::OptimizationSuggestion;
use super::types_3::{AmbiguityType, CtlFormula, TimedAutomaton};
use super::types_4::{
    Ambiguity, ChangeImpact, CoverageGap, CtlStarPathFormula, LtlFormula, Severity, TemporalState,
    TransitionSystem,
};
use super::types_5::{NotificationConfig, StatuteChange, TimedConfiguration};

#[allow(dead_code)]
pub(super) fn check_ctl_star_path(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    match path {
        CtlStarPathFormula::State(formula) => check_ctl_star_state(system, state, formula, visited),
        CtlStarPathFormula::Not(p) => !check_ctl_star_path(system, state, p, visited, path_visited),
        CtlStarPathFormula::And(left, right) => {
            check_ctl_star_path(system, state, left, visited, path_visited)
                && check_ctl_star_path(system, state, right, visited, path_visited)
        }
        CtlStarPathFormula::Or(left, right) => {
            check_ctl_star_path(system, state, left, visited, path_visited)
                || check_ctl_star_path(system, state, right, visited, path_visited)
        }
        CtlStarPathFormula::Next(p) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return false;
            }
            successors
                .iter()
                .any(|s| check_ctl_star_path(system, s, p, visited, path_visited))
        }
        CtlStarPathFormula::Eventually(p) => {
            if path_visited.contains(&state.id) {
                return false;
            }
            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());
            if check_ctl_star_path(system, state, p, visited, &mut new_path_visited) {
                return true;
            }
            let successors = system.successors(&state.id);
            successors.iter().any(|s| {
                check_ctl_star_path(
                    system,
                    s,
                    &CtlStarPathFormula::Eventually(p.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        CtlStarPathFormula::Always(p) => {
            if !check_ctl_star_path(system, state, p, visited, path_visited) {
                return false;
            }
            if path_visited.contains(&state.id) {
                return true;
            }
            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors.iter().any(|s| {
                check_ctl_star_path(
                    system,
                    s,
                    &CtlStarPathFormula::Always(p.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        CtlStarPathFormula::Until(left, right) => {
            if path_visited.contains(&state.id) {
                return false;
            }
            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());
            if check_ctl_star_path(system, state, right, visited, &mut new_path_visited.clone()) {
                return true;
            }
            if !check_ctl_star_path(system, state, left, visited, &mut new_path_visited.clone()) {
                return false;
            }
            let successors = system.successors(&state.id);
            successors.iter().any(|s| {
                check_ctl_star_path(
                    system,
                    s,
                    &CtlStarPathFormula::Until(left.clone(), right.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        CtlStarPathFormula::Release(left, right) => {
            let not_left = CtlStarPathFormula::Not(left.clone());
            let not_right = CtlStarPathFormula::Not(right.clone());
            !check_ctl_star_path(
                system,
                state,
                &CtlStarPathFormula::Until(Box::new(not_left), Box::new(not_right)),
                visited,
                path_visited,
            )
        }
    }
}
#[allow(dead_code)]
pub(super) fn check_ctl_star_path_universal(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    match path {
        CtlStarPathFormula::State(formula) => check_ctl_star_state(system, state, formula, visited),
        CtlStarPathFormula::Next(p) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors
                .iter()
                .all(|s| check_ctl_star_path_universal(system, s, p, visited, path_visited))
        }
        CtlStarPathFormula::Always(p) => {
            if !check_ctl_star_path_universal(system, state, p, visited, path_visited) {
                return false;
            }
            if path_visited.contains(&state.id) {
                return true;
            }
            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors.iter().all(|s| {
                check_ctl_star_path_universal(
                    system,
                    s,
                    &CtlStarPathFormula::Always(p.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        _ => check_ctl_star_path(system, state, path, visited, path_visited),
    }
}
/// Verifies reachability in a timed automaton.
///
/// Returns true if an accepting location is reachable from the initial location
/// within the given time bound.
pub fn verify_timed_reachability(automaton: &TimedAutomaton, time_bound: u64) -> bool {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();
    let mut initial_config = TimedConfiguration::new(automaton.initial.clone());
    for clock in &automaton.clocks {
        initial_config.valuations.insert(clock.name.clone(), 0);
    }
    queue.push_back((initial_config, 0u64));
    while let Some((config, time)) = queue.pop_front() {
        if time > time_bound {
            continue;
        }
        let state_key = format!("{:?}", (&config.location, &config.valuations));
        if visited.contains(&state_key) {
            continue;
        }
        visited.insert(state_key);
        if let Some(location) = automaton.locations.get(&config.location) {
            if location.accepting {
                return true;
            }
            if let Some(ref invariant) = location.invariant
                && !invariant.satisfied(&config.valuations)
            {
                continue;
            }
        }
        for transition in &automaton.transitions {
            if transition.from != config.location {
                continue;
            }
            if let Some(ref guard) = transition.guard
                && !guard.satisfied(&config.valuations)
            {
                continue;
            }
            let mut new_valuations = config.valuations.clone();
            for clock in &transition.resets {
                new_valuations.insert(clock.name.clone(), 0);
            }
            for (_, val) in new_valuations.iter_mut() {
                *val += 1;
            }
            let new_config = TimedConfiguration {
                location: transition.to.clone(),
                valuations: new_valuations,
            };
            queue.push_back((new_config, time + 1));
        }
    }
    false
}
/// Synthesizes a temporal property from positive and negative examples.
///
/// Given traces that should satisfy a property (positive examples) and
/// traces that should not (negative examples), this function attempts to
/// synthesize an LTL formula that separates them.
///
/// Returns the synthesized LTL formula if successful.
pub fn synthesize_ltl_property(
    positive_traces: &[Vec<HashSet<String>>],
    negative_traces: &[Vec<HashSet<String>>],
) -> Option<LtlFormula> {
    let mut all_props = HashSet::new();
    for trace in positive_traces.iter().chain(negative_traces.iter()) {
        for state_props in trace {
            all_props.extend(state_props.clone());
        }
    }
    if all_props.is_empty() {
        return None;
    }
    for prop in &all_props {
        let formula = LtlFormula::always(LtlFormula::atom(prop));
        if check_formula_on_traces(&formula, positive_traces, true)
            && check_formula_on_traces(&formula, negative_traces, false)
        {
            return Some(formula);
        }
    }
    for prop in &all_props {
        let formula = LtlFormula::eventually(LtlFormula::atom(prop));
        if check_formula_on_traces(&formula, positive_traces, true)
            && check_formula_on_traces(&formula, negative_traces, false)
        {
            return Some(formula);
        }
    }
    for p in &all_props {
        for q in &all_props {
            if p == q {
                continue;
            }
            let formula = LtlFormula::always(LtlFormula::implies(
                LtlFormula::atom(p),
                LtlFormula::eventually(LtlFormula::atom(q)),
            ));
            if check_formula_on_traces(&formula, positive_traces, true)
                && check_formula_on_traces(&formula, negative_traces, false)
            {
                return Some(formula);
            }
        }
    }
    for p in &all_props {
        for q in &all_props {
            let formula = LtlFormula::and(
                LtlFormula::always(LtlFormula::atom(p)),
                LtlFormula::eventually(LtlFormula::atom(q)),
            );
            if check_formula_on_traces(&formula, positive_traces, true)
                && check_formula_on_traces(&formula, negative_traces, false)
            {
                return Some(formula);
            }
        }
    }
    None
}
/// Checks if a formula holds on all traces with expected result.
fn check_formula_on_traces(
    formula: &LtlFormula,
    traces: &[Vec<HashSet<String>>],
    expected: bool,
) -> bool {
    for trace in traces {
        let holds = check_formula_on_trace(formula, trace);
        if holds != expected {
            return false;
        }
    }
    true
}
/// Checks if an LTL formula holds on a single trace.
pub(crate) fn check_formula_on_trace(formula: &LtlFormula, trace: &[HashSet<String>]) -> bool {
    if trace.is_empty() {
        return false;
    }
    check_ltl_at_position(formula, trace, 0)
}
/// Checks if an LTL formula holds starting at a specific position in a trace.
fn check_ltl_at_position(formula: &LtlFormula, trace: &[HashSet<String>], pos: usize) -> bool {
    if pos >= trace.len() {
        return false;
    }
    match formula {
        LtlFormula::Atom(prop) => trace[pos].contains(prop),
        LtlFormula::Not(f) => !check_ltl_at_position(f, trace, pos),
        LtlFormula::And(left, right) => {
            check_ltl_at_position(left, trace, pos) && check_ltl_at_position(right, trace, pos)
        }
        LtlFormula::Or(left, right) => {
            check_ltl_at_position(left, trace, pos) || check_ltl_at_position(right, trace, pos)
        }
        LtlFormula::Implies(left, right) => {
            !check_ltl_at_position(left, trace, pos) || check_ltl_at_position(right, trace, pos)
        }
        LtlFormula::Next(f) => {
            if pos + 1 < trace.len() {
                check_ltl_at_position(f, trace, pos + 1)
            } else {
                false
            }
        }
        LtlFormula::Eventually(f) => (pos..trace.len()).any(|i| check_ltl_at_position(f, trace, i)),
        LtlFormula::Always(f) => (pos..trace.len()).all(|i| check_ltl_at_position(f, trace, i)),
        LtlFormula::Until(left, right) => {
            for i in pos..trace.len() {
                if check_ltl_at_position(right, trace, i) {
                    return (pos..i).all(|j| check_ltl_at_position(left, trace, j));
                }
            }
            false
        }
        LtlFormula::Release(left, right) => {
            let not_left = LtlFormula::not(*left.clone());
            let not_right = LtlFormula::not(*right.clone());
            !check_ltl_at_position(&LtlFormula::until(not_left, not_right), trace, pos)
        }
    }
}
/// Synthesizes a CTL property from a transition system and examples.
///
/// This is a simplified synthesis that generates basic CTL patterns
/// based on the structure of the transition system and desired properties.
pub fn synthesize_ctl_property(
    system: &TransitionSystem,
    desired_properties: &[String],
) -> Option<CtlFormula> {
    if desired_properties.is_empty() {
        return None;
    }
    for prop in desired_properties {
        let formula = CtlFormula::exists_eventually(CtlFormula::atom(prop));
        if verify_ctl(system, &formula) {
            return Some(formula);
        }
    }
    for prop in desired_properties {
        let formula = CtlFormula::all_eventually(CtlFormula::atom(prop));
        if verify_ctl(system, &formula) {
            return Some(formula);
        }
    }
    for prop in desired_properties {
        let formula = CtlFormula::all_always(CtlFormula::atom(prop));
        if verify_ctl(system, &formula) {
            return Some(formula);
        }
    }
    None
}
/// Sends a notification based on configuration.
///
/// This is a mock implementation. In production, this would actually send
/// webhooks, emails, or invoke callbacks.
pub fn send_notification(config: &NotificationConfig, message: &NotificationMessage) -> bool {
    if !config.trigger_on.contains(&message.notification_type) {
        return false;
    }
    !config.channels.is_empty()
}
#[cfg(feature = "watch")]
pub mod watch {
    //! Watch mode for continuous verification of statute files.
    //!
    //! This module provides functionality to monitor directories for changes
    //! and automatically trigger verification when statute files are modified.
    use super::*;
    use crate::{StatuteVerifier, VerificationResult};
    use crossbeam_channel::{bounded, select};
    use notify::{
        Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult,
        Watcher,
    };
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    /// Configuration for watch mode.
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct WatchConfig {
        /// Paths to watch
        pub paths: Vec<PathBuf>,
        /// File extensions to watch (e.g., ["json", "toml"])
        pub extensions: Vec<String>,
        /// Debounce delay in milliseconds
        pub debounce_ms: u64,
        /// Whether to watch recursively
        pub recursive: bool,
    }
    impl Default for WatchConfig {
        fn default() -> Self {
            Self {
                paths: vec![PathBuf::from(".")],
                extensions: vec!["json".to_string(), "toml".to_string()],
                debounce_ms: 500,
                recursive: true,
            }
        }
    }
    impl WatchConfig {
        /// Creates a new watch configuration.
        pub fn new() -> Self {
            Self::default()
        }
        /// Adds a path to watch.
        pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
            self.paths.push(path.into());
            self
        }
        /// Sets the file extensions to watch.
        pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
            self.extensions = extensions;
            self
        }
        /// Sets the debounce delay.
        pub fn with_debounce(mut self, ms: u64) -> Self {
            self.debounce_ms = ms;
            self
        }
        /// Sets whether to watch recursively.
        pub fn recursive(mut self, recursive: bool) -> Self {
            self.recursive = recursive;
            self
        }
    }
    /// Statistics about watch mode operations.
    #[derive(Debug, Clone, Default)]
    pub struct WatchStats {
        /// Number of file changes detected
        pub changes_detected: usize,
        /// Number of verifications triggered
        pub verifications_triggered: usize,
        /// Number of verification errors
        pub verification_errors: usize,
    }
    /// A watcher that monitors files and triggers verification on changes.
    pub struct StatuteWatcher {
        config: WatchConfig,
        verifier: Arc<Mutex<StatuteVerifier>>,
        stats: Arc<Mutex<WatchStats>>,
    }
    impl StatuteWatcher {
        /// Creates a new statute watcher.
        pub fn new(config: WatchConfig, verifier: StatuteVerifier) -> Self {
            Self {
                config,
                verifier: Arc::new(Mutex::new(verifier)),
                stats: Arc::new(Mutex::new(WatchStats::default())),
            }
        }
        /// Checks if a path should be watched based on the configuration.
        fn should_watch(&self, path: &Path) -> bool {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                self.config.extensions.iter().any(|e| e == &*ext_str)
            } else {
                false
            }
        }
        /// Starts watching and returns when stopped.
        pub fn watch<F>(&self, mut on_change: F) -> NotifyResult<()>
        where
            F: FnMut(&Path, &VerificationResult) + Send + 'static,
        {
            let (tx, rx) = bounded(1);
            let mut watcher = RecommendedWatcher::new(
                move |res: NotifyResult<Event>| {
                    if let Ok(event) = res {
                        let _ = tx.send(event);
                    }
                },
                Config::default(),
            )?;
            for path in &self.config.paths {
                let mode = if self.config.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                };
                watcher.watch(path, mode)?;
            }
            println!("Watching for changes in {:?}...", self.config.paths);
            println!("Press Ctrl+C to stop");
            loop {
                select! {
                    recv(rx) -> event => { if let Ok(event) = event { self
                    .handle_event(event, & mut on_change); } }
                }
                std::thread::sleep(Duration::from_millis(self.config.debounce_ms));
            }
        }
        /// Handles a file system event.
        fn handle_event<F>(&self, event: Event, on_change: &mut F)
        where
            F: FnMut(&Path, &VerificationResult),
        {
            if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                return;
            }
            for path in event.paths {
                if !self.should_watch(&path) {
                    continue;
                }
                {
                    let mut stats = self.stats.lock().expect("stats mutex poisoned");
                    stats.changes_detected += 1;
                }
                println!("Change detected: {:?}", path);
                match self.load_and_verify(&path) {
                    Ok(result) => {
                        let mut stats = self.stats.lock().expect("stats mutex poisoned");
                        stats.verifications_triggered += 1;
                        if !result.passed {
                            stats.verification_errors += result.errors.len();
                        }
                        drop(stats);
                        on_change(&path, &result);
                    }
                    Err(e) => {
                        eprintln!("Error verifying {}: {}", path.display(), e);
                    }
                }
            }
        }
        /// Loads a statute file and verifies it.
        fn load_and_verify(&self, path: &Path) -> anyhow::Result<VerificationResult> {
            let content = std::fs::read_to_string(path)?;
            let statutes: Vec<Statute> = serde_json::from_str(&content)?;
            let verifier = self.verifier.lock().expect("verifier mutex poisoned");
            Ok(verifier.verify(&statutes))
        }
        /// Returns the current watch statistics.
        pub fn stats(&self) -> WatchStats {
            self.stats.lock().expect("stats mutex poisoned").clone()
        }
        /// Resets the watch statistics.
        pub fn reset_stats(&self) {
            let mut stats = self.stats.lock().expect("stats mutex poisoned");
            *stats = WatchStats::default();
        }
    }
}
/// Analyzes statutes and suggests optimizations for complex conditions.
///
/// This function uses SMT-based analysis to identify simplification opportunities.
#[cfg(feature = "smt-solver")]
pub fn suggest_optimizations(statutes: &[Statute]) -> Vec<OptimizationSuggestion> {
    use crate::smt::SmtVerifier;
    let mut verifier = SmtVerifier::new();
    let mut suggestions = Vec::new();
    for statute in statutes {
        for condition in &statute.preconditions {
            let (complexity, smt_suggestions) = verifier.analyze_complexity(condition);
            if (!smt_suggestions.is_empty() || complexity > 10)
                && let Ok((simplified, changed)) = verifier.simplify(condition)
            {
                let optimized_complexity = if changed {
                    let (opt_comp, _) = verifier.analyze_complexity(&simplified);
                    opt_comp
                } else {
                    complexity
                };
                suggestions.push(OptimizationSuggestion {
                    statute_id: statute.id.clone(),
                    current_complexity: complexity,
                    suggested_condition: if changed {
                        Some(format!("{}", simplified))
                    } else {
                        None
                    },
                    suggestions: smt_suggestions,
                    optimized_complexity,
                });
            }
        }
    }
    suggestions
}
/// Analyzes statute coverage and identifies potential gaps.
///
/// This performs a heuristic analysis to find common scenarios that
/// might not be covered by the provided statutes.
pub fn analyze_coverage_gaps(statutes: &[Statute]) -> Vec<CoverageGap> {
    let mut gaps = Vec::new();
    let age_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            s.preconditions
                .iter()
                .any(|c| matches!(c, legalis_core::Condition::Age { .. }))
        })
        .collect();
    if !age_statutes.is_empty() {
        let mut age_thresholds: Vec<u32> = age_statutes
            .iter()
            .flat_map(|s| {
                s.preconditions.iter().filter_map(|c| {
                    if let legalis_core::Condition::Age { value, .. } = c {
                        Some(*value)
                    } else {
                        None
                    }
                })
            })
            .collect();
        age_thresholds.sort_unstable();
        age_thresholds.dedup();
        if age_thresholds.len() >= 2 {
            for window in age_thresholds.windows(2) {
                if window[1] - window[0] > 5 {
                    gaps.push(CoverageGap {
                        description: format!(
                            "Potential gap in age coverage between {} and {}",
                            window[0], window[1]
                        ),
                        example_scenario: format!(
                            "Person aged {} may not be covered by any statute",
                            (window[0] + window[1]) / 2
                        ),
                        severity: Severity::Warning,
                        related_statutes: age_statutes.iter().map(|s| s.id.clone()).collect(),
                    });
                }
            }
        }
    }
    let income_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            s.preconditions
                .iter()
                .any(|c| matches!(c, legalis_core::Condition::Income { .. }))
        })
        .collect();
    if !income_statutes.is_empty() {
        gaps.push(CoverageGap {
            description: "Income-based statutes detected - verify edge cases".to_string(),
            example_scenario: "Persons at exact income thresholds may need special handling"
                .to_string(),
            severity: Severity::Info,
            related_statutes: income_statutes.iter().map(|s| s.id.clone()).collect(),
        });
    }
    let jurisdictions: std::collections::HashSet<_> = statutes
        .iter()
        .filter_map(|s| s.jurisdiction.as_ref())
        .collect();
    if jurisdictions.len() > 1 {
        for statute in statutes {
            if statute.jurisdiction.is_none() {
                gaps.push(CoverageGap {
                    description: format!(
                        "Statute '{}' has no jurisdiction specified", statute.id
                    ),
                    example_scenario: "May apply too broadly or conflict with jurisdictional statutes"
                        .to_string(),
                    severity: Severity::Warning,
                    related_statutes: vec![statute.id.clone()],
                });
            }
        }
    }
    gaps
}
/// Generates a report of coverage gaps and optimization suggestions.
pub fn optimization_and_gaps_report(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Optimization and Gap Analysis Report\n\n");
    let gaps = analyze_coverage_gaps(statutes);
    report.push_str("## Coverage Gaps\n\n");
    if gaps.is_empty() {
        report.push_str("No significant coverage gaps detected.\n\n");
    } else {
        for (i, gap) in gaps.iter().enumerate() {
            report.push_str(&format!("### Gap #{}: {}\n", i + 1, gap.description));
            report.push_str(&format!("- **Severity**: {:?}\n", gap.severity));
            report.push_str(&format!("- **Example**: {}\n", gap.example_scenario));
            report.push_str(&format!(
                "- **Related statutes**: {}\n\n",
                gap.related_statutes.join(", ")
            ));
        }
    }
    #[cfg(feature = "smt-solver")]
    {
        let optimizations = suggest_optimizations(statutes);
        report.push_str("## Optimization Suggestions\n\n");
        if optimizations.is_empty() {
            report.push_str("No optimization opportunities detected.\n\n");
        } else {
            for opt in &optimizations {
                report.push_str(&format!("### Statute: {}\n", opt.statute_id));
                report.push_str(&format!(
                    "- **Current complexity**: {}\n",
                    opt.current_complexity
                ));
                report.push_str(&format!(
                    "- **Optimized complexity**: {}\n",
                    opt.optimized_complexity
                ));
                if let Some(ref suggested) = opt.suggested_condition {
                    report.push_str(&format!(
                        "- **Suggested simplification**: `{}`\n",
                        suggested
                    ));
                }
                if !opt.suggestions.is_empty() {
                    report.push_str("- **Recommendations**:\n");
                    for suggestion in &opt.suggestions {
                        report.push_str(&format!("  - {}\n", suggestion));
                    }
                }
                report.push('\n');
            }
        }
    }
    #[cfg(not(feature = "smt-solver"))]
    {
        report.push_str("## Optimization Suggestions\n\n");
        report.push_str(
            "*Optimization suggestions require the `smt-solver` feature to be enabled.*\n\n",
        );
    }
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total statutes analyzed: {}\n", statutes.len()));
    report.push_str(&format!("- Coverage gaps found: {}\n", gaps.len()));
    #[cfg(feature = "smt-solver")]
    {
        let optimizations = suggest_optimizations(statutes);
        report.push_str(&format!(
            "- Optimization opportunities: {}\n",
            optimizations.len()
        ));
    }
    report
}
/// Exports statute dependencies as a GraphViz DOT format graph.
///
/// This can be visualized using tools like Graphviz, which supports
/// rendering DOT files to SVG, PNG, PDF, and other formats.
///
/// # Example
/// ```ignore
/// let statutes = vec![...];
/// let dot = export_dependency_graph(&statutes);
/// std::fs::write("dependencies.dot", dot)?;
/// // Then run: dot -Tpng dependencies.dot -o dependencies.png
/// ```
pub fn export_dependency_graph(statutes: &[Statute]) -> String {
    let mut dot = String::from("digraph StatuteDependencies {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
    for statute in statutes {
        let label = format!("{}\\n{}", statute.id, statute.title);
        dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", statute.id, label));
    }
    dot.push('\n');
    let statute_ids: HashSet<String> = statutes.iter().map(|s| s.id.clone()).collect();
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        for ref_id in refs {
            if statute_ids.contains(&ref_id) {
                dot.push_str(&format!(
                    "  \"{}\" -> \"{}\" [label=\"references\"];\n",
                    statute.id, ref_id
                ));
            }
        }
    }
    dot.push_str("}\n");
    dot
}
/// Exports statute dependencies with conflict highlighting.
///
/// Conflicting statutes are colored in red, and conflict edges are dashed.
pub fn export_dependency_graph_with_conflicts(statutes: &[Statute]) -> String {
    let conflicts = detect_statute_conflicts(statutes);
    let mut conflict_pairs: HashSet<(String, String)> = HashSet::new();
    let mut conflicting_statute_ids: HashSet<String> = HashSet::new();
    for conflict in &conflicts {
        for statute_id in &conflict.statute_ids {
            conflicting_statute_ids.insert(statute_id.clone());
        }
        if conflict.statute_ids.len() >= 2 {
            let id1 = &conflict.statute_ids[0];
            let id2 = &conflict.statute_ids[1];
            conflict_pairs.insert((id1.clone(), id2.clone()));
            conflict_pairs.insert((id2.clone(), id1.clone()));
        }
    }
    let mut dot = String::from("digraph StatuteDependenciesWithConflicts {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box, style=filled];\n\n");
    for statute in statutes {
        let color = if conflicting_statute_ids.contains(&statute.id) {
            "lightcoral"
        } else {
            "lightblue"
        };
        let label = format!("{}\\n{}", statute.id, statute.title);
        dot.push_str(&format!(
            "  \"{}\" [label=\"{}\", fillcolor={}];\n",
            statute.id, label, color
        ));
    }
    dot.push('\n');
    let statute_ids: HashSet<String> = statutes.iter().map(|s| s.id.clone()).collect();
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        for ref_id in refs {
            if statute_ids.contains(&ref_id) {
                dot.push_str(&format!(
                    "  \"{}\" -> \"{}\" [label=\"references\"];\n",
                    statute.id, ref_id
                ));
            }
        }
    }
    for (id1, id2) in &conflict_pairs {
        if statute_ids.contains(id1) && statute_ids.contains(id2) {
            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [style=dashed, color=red, label=\"conflicts\"];\n",
                id1, id2
            ));
        }
    }
    dot.push_str("}\n");
    dot
}
/// Calculates the legislative drafting quality score (0-100).
///
/// This evaluates the statute against legislative drafting best practices:
/// - Clear structure and organization
/// - Consistent terminology
/// - Appropriate level of detail
/// - Proper use of conditions and effects
/// - Temporal validity properly defined
fn calculate_drafting_quality(statute: &Statute) -> f64 {
    let mut score: f64 = 0.0;
    if !statute.title.is_empty() {
        let title_words = statute.title.split_whitespace().count();
        if (3..=20).contains(&title_words) {
            score += 10.0;
        } else if title_words > 0 {
            score += 5.0;
        }
    }
    if !statute.effect.description.is_empty() {
        let desc_words = statute.effect.description.split_whitespace().count();
        if (5..=100).contains(&desc_words) {
            score += 15.0;
        } else if desc_words > 0 {
            score += 8.0;
        }
    }
    if statute.temporal_validity.enacted_at.is_some() {
        score += 10.0;
    }
    if statute.temporal_validity.effective_date.is_some() {
        score += 5.0;
    }
    if statute.jurisdiction.is_some() {
        score += 10.0;
    }
    let precondition_count = statute.preconditions.len();
    if (1..=7).contains(&precondition_count) {
        score += 15.0;
    } else if precondition_count > 0 {
        score += 8.0;
    }
    if statute.discretion_logic.is_some() {
        score += 10.0;
    }
    let effect_keywords_match = match statute.effect.effect_type {
        legalis_core::EffectType::Grant => {
            statute.effect.description.to_lowercase().contains("grant")
                || statute.effect.description.to_lowercase().contains("allow")
        }
        legalis_core::EffectType::Prohibition => {
            statute
                .effect
                .description
                .to_lowercase()
                .contains("prohibit")
                || statute.effect.description.to_lowercase().contains("forbid")
                || statute
                    .effect
                    .description
                    .to_lowercase()
                    .contains("not allow")
        }
        legalis_core::EffectType::Obligation => {
            statute.effect.description.to_lowercase().contains("must")
                || statute
                    .effect
                    .description
                    .to_lowercase()
                    .contains("require")
                || statute.effect.description.to_lowercase().contains("shall")
        }
        _ => true,
    };
    if effect_keywords_match {
        score += 10.0;
    }
    let mut metadata_score = 0.0;
    if !statute.id.is_empty() {
        metadata_score += 5.0;
    }
    if !statute.title.is_empty() {
        metadata_score += 5.0;
    }
    if statute.jurisdiction.is_some() {
        metadata_score += 5.0;
    }
    score += metadata_score;
    score.min(100.0)
}
/// Calculates the clarity index (0-100).
///
/// Measures how clear and understandable the statute is based on:
/// - Simple language in titles and descriptions
/// - Logical condition structure
/// - Unambiguous terminology
/// - Appropriate complexity level
fn calculate_clarity_index(statute: &Statute) -> f64 {
    let mut score: f64 = 50.0;
    let title_words = statute.title.split_whitespace().count();
    if (3..=12).contains(&title_words) {
        score += 15.0;
    } else if title_words > 0 && title_words <= 20 {
        score += 8.0;
    }
    let desc_words = statute.effect.description.split_whitespace().count();
    if (5..=50).contains(&desc_words) {
        score += 20.0;
    } else if desc_words > 0 && desc_words <= 100 {
        score += 10.0;
    } else if desc_words > 100 {
        score -= 5.0;
    }
    let complexity = analyze_complexity(statute);
    if complexity.complexity_score <= 25 {
        score += 15.0;
    } else if complexity.complexity_score <= 50 {
        score += 10.0;
    } else if complexity.complexity_score <= 75 {
        score += 5.0;
    } else {
        score -= 5.0;
    }
    if statute.discretion_logic.is_some() {
        score += 10.0;
    }
    score.clamp(0.0, 100.0)
}
/// Calculates the testability assessment score (0-100).
///
/// Evaluates how testable and verifiable the statute conditions are:
/// - Concrete, measurable conditions
/// - Clear pass/fail criteria
/// - Deterministic evaluation
/// - Observable outcomes
fn calculate_testability(statute: &Statute) -> f64 {
    let mut score = 0.0;
    if !statute.preconditions.is_empty() {
        score += 20.0;
        let mut testable_count = 0;
        let total_conditions = count_all_conditions(&statute.preconditions);
        for condition in &statute.preconditions {
            if is_testable_condition(condition) {
                testable_count += 1;
            }
        }
        if total_conditions > 0 {
            let testable_ratio = testable_count as f64 / total_conditions as f64;
            score += testable_ratio * 30.0;
        }
    } else {
        score += 10.0;
    }
    if !statute.effect.description.is_empty() {
        score += 20.0;
    }
    if statute.temporal_validity.effective_date.is_some() {
        score += 10.0;
    }
    if statute.temporal_validity.expiry_date.is_some() {
        score += 5.0;
    }
    if statute.jurisdiction.is_some() {
        score += 15.0;
    }
    score.min(100.0)
}
/// Calculates the maintainability score (0-100).
///
/// Assesses how easy it would be to modify or extend the statute:
/// - Modular structure
/// - Clear dependencies
/// - Appropriate abstraction level
/// - Documentation quality
fn calculate_maintainability(statute: &Statute) -> f64 {
    let mut score: f64 = 30.0;
    let complexity = analyze_complexity(statute);
    if complexity.complexity_score <= 30 {
        score += 25.0;
    } else if complexity.complexity_score <= 60 {
        score += 15.0;
    } else if complexity.complexity_score <= 80 {
        score += 8.0;
    }
    if let Some(logic) = &statute.discretion_logic
        && !logic.is_empty()
    {
        score += 20.0;
    }
    let precondition_count = statute.preconditions.len();
    if precondition_count <= 5 {
        score += 15.0;
    } else if precondition_count <= 10 {
        score += 10.0;
    } else if precondition_count <= 15 {
        score += 5.0;
    }
    let mut metadata_score = 0.0;
    if !statute.id.is_empty() && !statute.id.contains("unknown") {
        metadata_score += 5.0;
    }
    if !statute.title.is_empty() {
        metadata_score += 5.0;
    }
    if statute.jurisdiction.is_some() {
        metadata_score += 5.0;
    }
    if statute.temporal_validity.enacted_at.is_some() {
        metadata_score += 5.0;
    }
    score += metadata_score;
    score.min(100.0)
}
/// Counts all conditions recursively (including nested conditions).
fn count_all_conditions(conditions: &[legalis_core::Condition]) -> usize {
    let mut count = 0;
    for condition in conditions {
        count += count_condition_recursive(condition);
    }
    count
}
/// Recursively counts a single condition and its children.
fn count_condition_recursive(condition: &legalis_core::Condition) -> usize {
    use legalis_core::Condition;
    match condition {
        Condition::And(left, right) | Condition::Or(left, right) => {
            1 + count_condition_recursive(left) + count_condition_recursive(right)
        }
        Condition::Not(inner) => 1 + count_condition_recursive(inner),
        Condition::Composite { conditions, .. } => {
            1 + conditions
                .iter()
                .map(|(_, c)| count_condition_recursive(c))
                .sum::<usize>()
        }
        Condition::Probabilistic { condition, .. } => 1 + count_condition_recursive(condition),
        _ => 1,
    }
}
/// Checks if a condition is testable (has concrete, measurable criteria).
fn is_testable_condition(condition: &legalis_core::Condition) -> bool {
    use legalis_core::Condition;
    match condition {
        Condition::Age { .. }
        | Condition::Income { .. }
        | Condition::DateRange { .. }
        | Condition::ResidencyDuration { .. }
        | Condition::Duration { .. }
        | Condition::Percentage { .. }
        | Condition::SetMembership { .. }
        | Condition::Pattern { .. }
        | Condition::Calculation { .. }
        | Condition::Threshold { .. }
        | Condition::Temporal { .. } => true,
        Condition::HasAttribute { .. } | Condition::AttributeEquals { .. } => true,
        Condition::Geographic { .. } | Condition::EntityRelationship { .. } => true,
        Condition::And(left, right) | Condition::Or(left, right) => {
            is_testable_condition(left) && is_testable_condition(right)
        }
        Condition::Not(inner) => is_testable_condition(inner),
        Condition::Composite { conditions, .. } => {
            conditions.iter().all(|(_, c)| is_testable_condition(c))
        }
        Condition::Probabilistic { condition, .. } => is_testable_condition(condition),
        Condition::Fuzzy { .. } | Condition::Custom { .. } => false,
    }
}
/// Analyzes statute quality and returns comprehensive metrics.
pub fn analyze_quality(statute: &Statute) -> QualityMetrics {
    let complexity_metrics = analyze_complexity(statute);
    let max_complexity = 100.0;
    let complexity_score = ((max_complexity
        - complexity_metrics
            .complexity_score
            .min(max_complexity as u32) as f64)
        / max_complexity
        * 100.0)
        .max(0.0);
    let mut readability_score = 50.0;
    if !statute.title.is_empty() && statute.title.len() > 10 {
        readability_score += 20.0;
    }
    if statute.discretion_logic.is_some() {
        readability_score += 30.0;
    }
    let mut consistency_score = 50.0;
    if statute.jurisdiction.is_some() {
        consistency_score += 25.0;
    }
    if statute.temporal_validity.enacted_at.is_some() {
        consistency_score += 25.0;
    }
    let mut completeness_score = 0.0;
    if !statute.id.is_empty() {
        completeness_score += 20.0;
    }
    if !statute.title.is_empty() {
        completeness_score += 20.0;
    }
    if statute.jurisdiction.is_some() {
        completeness_score += 20.0;
    }
    if statute.temporal_validity.enacted_at.is_some() {
        completeness_score += 20.0;
    }
    if !statute.preconditions.is_empty() || !statute.effect.description.is_empty() {
        completeness_score += 20.0;
    }
    let drafting_quality_score = calculate_drafting_quality(statute);
    let clarity_index = calculate_clarity_index(statute);
    let testability_score = calculate_testability(statute);
    let maintainability_score = calculate_maintainability(statute);
    let mut metrics = QualityMetrics::new(
        statute.id.clone(),
        complexity_score,
        readability_score,
        consistency_score,
        completeness_score,
        drafting_quality_score,
        clarity_index,
        testability_score,
        maintainability_score,
    );
    if complexity_metrics.complexity_score > 70 {
        metrics = metrics.with_issue(format!(
            "High complexity ({}), consider simplification",
            complexity_metrics.complexity_score
        ));
    }
    if statute.discretion_logic.is_none() {
        metrics = metrics.with_issue("Missing discretion logic");
    }
    if statute.jurisdiction.is_none() {
        metrics = metrics.with_issue("Missing jurisdiction");
    }
    if statute.temporal_validity.enacted_at.is_none() {
        metrics = metrics.with_issue("Missing enactment date");
    }
    if complexity_metrics.complexity_score <= 30 {
        metrics = metrics.with_strength("Low complexity");
    }
    if statute.discretion_logic.is_some() {
        metrics = metrics.with_strength("Has discretion logic");
    }
    if statute.jurisdiction.is_some() && statute.temporal_validity.enacted_at.is_some() {
        metrics = metrics.with_strength("Complete metadata");
    }
    metrics
}
/// Detects ambiguities in a statute.
///
/// This function analyzes a statute for various types of ambiguities including:
/// - Vague or undefined terms
/// - Overlapping conditions
/// - Unclear effects
/// - Missing discretion logic
/// - Temporal ambiguities
pub fn detect_ambiguities(statute: &Statute) -> Vec<Ambiguity> {
    let mut ambiguities = Vec::new();
    if contains_vague_terms(&statute.title) {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::VagueTerm,
            "title",
            format!("Title contains vague terms: '{}'", statute.title),
            "Use more specific and precise terminology",
            6,
        ));
    }
    if contains_vague_terms(&statute.effect.description) {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::VagueTerm,
            "effect.description",
            format!(
                "Effect description contains vague terms: '{}'",
                statute.effect.description
            ),
            "Specify exact requirements, amounts, or procedures",
            8,
        ));
    }
    if statute.effect.description.is_empty() {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::UnclearEffect,
            "effect.description",
            "Effect description is empty",
            "Provide a clear description of what this statute does",
            9,
        ));
    } else if statute.effect.description.split_whitespace().count() < 3 {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::UnclearEffect,
            "effect.description",
            "Effect description is too brief to be clear",
            "Expand the description to clearly explain the effect",
            7,
        ));
    }
    if statute.discretion_logic.is_none() && statute.preconditions.len() > 3 {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::MissingDiscretion,
            "discretion_logic",
            format!(
                "Complex statute with {} conditions lacks discretion logic",
                statute.preconditions.len()
            ),
            "Add discretion logic to clarify how conditions should be evaluated",
            7,
        ));
    }
    if statute.temporal_validity.effective_date.is_none()
        && statute.temporal_validity.enacted_at.is_some()
    {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::TemporalAmbiguity,
            "temporal_validity.effective_date",
            "Statute has enactment date but no effective date",
            "Specify when this statute becomes effective",
            6,
        ));
    }
    if statute.temporal_validity.enacted_at.is_none()
        && statute.temporal_validity.effective_date.is_none()
    {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::TemporalAmbiguity,
            "temporal_validity",
            "No temporal information specified",
            "Add enacted_at and effective_date to clarify when this statute applies",
            8,
        ));
    }
    if contains_ambiguous_quantifiers(&statute.effect.description) {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::QuantifierAmbiguity,
            "effect.description",
            "Effect description contains ambiguous quantifiers (e.g., 'some', 'several', 'many')",
            "Use specific numbers or percentages instead of vague quantifiers",
            7,
        ));
    }
    for (idx, condition) in statute.preconditions.iter().enumerate() {
        if let legalis_core::Condition::Custom { description } = condition
            && (description.len() < 10 || contains_vague_terms(description))
        {
            ambiguities.push(Ambiguity::new(
                AmbiguityType::ImplicitAssumption,
                format!("preconditions[{}]", idx),
                format!(
                    "Custom condition may have implicit assumptions: '{}'",
                    description
                ),
                "Replace custom condition with explicit, testable conditions",
                8,
            ));
        }
    }
    #[cfg(feature = "smt-solver")]
    {
        if let Some(overlaps) = detect_overlapping_conditions(&statute.preconditions) {
            ambiguities.push(Ambiguity::new(
                AmbiguityType::OverlappingConditions,
                "preconditions",
                overlaps,
                "Simplify conditions to remove overlap or clarify the relationship",
                6,
            ));
        }
    }
    ambiguities.sort_by_key(|b| std::cmp::Reverse(b.severity));
    ambiguities
}
/// Checks if a text contains vague or ambiguous terms.
fn contains_vague_terms(text: &str) -> bool {
    let vague_terms = [
        "reasonable",
        "appropriate",
        "sufficient",
        "adequate",
        "proper",
        "necessary",
        "significant",
        "substantial",
        "may",
        "might",
        "should",
        "could",
        "approximately",
        "around",
        "about",
        "roughly",
        "generally",
        "typically",
        "normally",
        "usually",
        "often",
        "sometimes",
        "occasionally",
    ];
    let text_lower = text.to_lowercase();
    vague_terms
        .iter()
        .any(|term| text_lower.contains(&format!(" {} ", term)) || text_lower.starts_with(term))
}
/// Checks if text contains ambiguous quantifiers.
fn contains_ambiguous_quantifiers(text: &str) -> bool {
    let ambiguous_quantifiers = [
        "some", "several", "many", "few", "multiple", "various", "numerous", "certain",
    ];
    let text_lower = text.to_lowercase();
    ambiguous_quantifiers
        .iter()
        .any(|quant| text_lower.contains(&format!(" {} ", quant)) || text_lower.starts_with(quant))
}
/// Detects overlapping conditions using SMT solver.
#[cfg(feature = "smt-solver")]
fn detect_overlapping_conditions(conditions: &[legalis_core::Condition]) -> Option<String> {
    use crate::smt::SmtVerifier;
    if conditions.len() < 2 {
        return None;
    }
    let mut verifier = SmtVerifier::new();
    for i in 0..conditions.len() {
        for j in (i + 1)..conditions.len() {
            if let Ok(true) = verifier.implies(&conditions[i], &conditions[j]) {
                return Some(format!(
                    "Condition {} implies condition {} (redundant)",
                    i, j
                ));
            }
            if let Ok(true) = verifier.implies(&conditions[j], &conditions[i]) {
                return Some(format!(
                    "Condition {} implies condition {} (redundant)",
                    j, i
                ));
            }
        }
    }
    None
}
/// Generates an ambiguity detection report for a statute.
pub fn ambiguity_report(statute: &Statute) -> String {
    let ambiguities = detect_ambiguities(statute);
    if ambiguities.is_empty() {
        return format!(
            "# Ambiguity Report for '{}'\n\nNo ambiguities detected.\n",
            statute.id
        );
    }
    let mut report = String::new();
    report.push_str(&format!("# Ambiguity Report for '{}'\n\n", statute.id));
    report.push_str(&format!("**Total Ambiguities**: {}\n\n", ambiguities.len()));
    let critical = ambiguities.iter().filter(|a| a.severity >= 8).count();
    let high = ambiguities
        .iter()
        .filter(|a| (6..8).contains(&a.severity))
        .count();
    let medium = ambiguities.iter().filter(|a| a.severity < 6).count();
    report.push_str("## Summary by Severity\n\n");
    if critical > 0 {
        report.push_str(&format!("- **Critical** (8-10): {}\n", critical));
    }
    if high > 0 {
        report.push_str(&format!("- **High** (6-7): {}\n", high));
    }
    if medium > 0 {
        report.push_str(&format!("- **Medium** (1-5): {}\n", medium));
    }
    report.push_str("\n## Detected Ambiguities\n\n");
    for (idx, ambiguity) in ambiguities.iter().enumerate() {
        report.push_str(&format!(
            "### {}. {} (Severity: {})\n\n",
            idx + 1,
            ambiguity.ambiguity_type,
            ambiguity.severity
        ));
        report.push_str(&format!("- **Location**: `{}`\n", ambiguity.location));
        report.push_str(&format!("- **Issue**: {}\n", ambiguity.description));
        report.push_str(&format!("- **Suggestion**: {}\n\n", ambiguity.suggestion));
    }
    report
}
/// Generates an ambiguity detection report for multiple statutes.
pub fn batch_ambiguity_report(statutes: &[Statute]) -> String {
    let mut report = String::from("# Batch Ambiguity Detection Report\n\n");
    let mut total_ambiguities = 0;
    let mut statutes_with_ambiguities = 0;
    for statute in statutes {
        let ambiguities = detect_ambiguities(statute);
        if !ambiguities.is_empty() {
            statutes_with_ambiguities += 1;
            total_ambiguities += ambiguities.len();
        }
    }
    report.push_str(&format!(
        "**Total Statutes Analyzed**: {}\n",
        statutes.len()
    ));
    report.push_str(&format!(
        "**Statutes with Ambiguities**: {}\n",
        statutes_with_ambiguities
    ));
    report.push_str(&format!(
        "**Total Ambiguities Found**: {}\n\n",
        total_ambiguities
    ));
    if total_ambiguities == 0 {
        report.push_str("No ambiguities detected in any statute.\n");
        return report;
    }
    report.push_str("## Individual Statute Reports\n\n");
    for statute in statutes {
        let ambiguities = detect_ambiguities(statute);
        if !ambiguities.is_empty() {
            report.push_str(&format!(
                "### {} - {} ({} ambiguities)\n\n",
                statute.id,
                statute.title,
                ambiguities.len()
            ));
            for ambiguity in &ambiguities {
                report.push_str(&format!(
                    "- **{}** (Severity {}): {} [{}]\n",
                    ambiguity.ambiguity_type,
                    ambiguity.severity,
                    ambiguity.description,
                    ambiguity.location
                ));
            }
            report.push('\n');
        }
    }
    report
}
/// Generates a quality report for multiple statutes.
pub fn quality_report(statutes: &[Statute]) -> String {
    let mut report = String::from("# Statute Quality Report\n\n");
    let mut total_score = 0.0;
    let mut grade_counts: HashMap<char, usize> = HashMap::new();
    for statute in statutes {
        let metrics = analyze_quality(statute);
        total_score += metrics.overall_score;
        *grade_counts.entry(metrics.grade()).or_insert(0) += 1;
        report.push_str(&format!(
            "## Statute: {} - {}\n\n",
            statute.id, statute.title
        ));
        report.push_str(&format!(
            "**Overall Score**: {:.1}/100 (Grade: {})\n\n",
            metrics.overall_score,
            metrics.grade()
        ));
        report.push_str("### Detailed Scores\n\n");
        report.push_str(&format!(
            "- Complexity: {:.1}/100\n",
            metrics.complexity_score
        ));
        report.push_str(&format!(
            "- Readability: {:.1}/100\n",
            metrics.readability_score
        ));
        report.push_str(&format!(
            "- Consistency: {:.1}/100\n",
            metrics.consistency_score
        ));
        report.push_str(&format!(
            "- Completeness: {:.1}/100\n",
            metrics.completeness_score
        ));
        report.push_str(&format!(
            "- Drafting Quality: {:.1}/100\n",
            metrics.drafting_quality_score
        ));
        report.push_str(&format!(
            "- Clarity Index: {:.1}/100\n",
            metrics.clarity_index
        ));
        report.push_str(&format!(
            "- Testability: {:.1}/100\n",
            metrics.testability_score
        ));
        report.push_str(&format!(
            "- Maintainability: {:.1}/100\n\n",
            metrics.maintainability_score
        ));
        if !metrics.strengths.is_empty() {
            report.push_str("### Strengths\n\n");
            for strength in &metrics.strengths {
                report.push_str(&format!("- {}\n", strength));
            }
            report.push('\n');
        }
        if !metrics.issues.is_empty() {
            report.push_str("### Issues\n\n");
            for issue in &metrics.issues {
                report.push_str(&format!("- {}\n", issue));
            }
            report.push('\n');
        }
    }
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total statutes analyzed: {}\n", statutes.len()));
    if !statutes.is_empty() {
        let average_score = total_score / statutes.len() as f64;
        report.push_str(&format!(
            "- Average quality score: {:.1}/100\n",
            average_score
        ));
    }
    report.push_str("\n### Grade Distribution\n\n");
    for grade in ['A', 'B', 'C', 'D', 'F'] {
        let count = grade_counts.get(&grade).unwrap_or(&0);
        report.push_str(&format!("- Grade {}: {}\n", grade, count));
    }
    report
}
/// Compares two versions of a statute and identifies changes.
pub fn compare_statutes(old: &Statute, new: &Statute) -> Vec<StatuteChange> {
    let mut changes = Vec::new();
    if old.title != new.title {
        changes.push(StatuteChange::TitleChanged {
            old: old.title.clone(),
            new: new.title.clone(),
        });
    }
    if old.discretion_logic != new.discretion_logic {
        changes.push(StatuteChange::DescriptionChanged {
            old: old.discretion_logic.clone(),
            new: new.discretion_logic.clone(),
        });
    }
    if old.jurisdiction != new.jurisdiction {
        changes.push(StatuteChange::JurisdictionChanged {
            old: old.jurisdiction.clone(),
            new: new.jurisdiction.clone(),
        });
    }
    let old_effect_str = format!("{:?}", old.effect);
    let new_effect_str = format!("{:?}", new.effect);
    if old_effect_str != new_effect_str {
        changes.push(StatuteChange::EffectChanged {
            old: old_effect_str,
            new: new_effect_str,
        });
    }
    if old.preconditions.len() != new.preconditions.len() || old.preconditions != new.preconditions
    {
        changes.push(StatuteChange::PreconditionsChanged {
            old_count: old.preconditions.len(),
            new_count: new.preconditions.len(),
        });
    }
    let old_enacted = old
        .temporal_validity
        .enacted_at
        .as_ref()
        .map(|d| d.to_string());
    let new_enacted = new
        .temporal_validity
        .enacted_at
        .as_ref()
        .map(|d| d.to_string());
    if old_enacted != new_enacted {
        changes.push(StatuteChange::EnactmentDateChanged {
            old: old_enacted,
            new: new_enacted,
        });
    }
    let old_effective = old
        .temporal_validity
        .effective_date
        .as_ref()
        .map(|d| d.to_string());
    let new_effective = new
        .temporal_validity
        .effective_date
        .as_ref()
        .map(|d| d.to_string());
    if old_effective != new_effective {
        changes.push(StatuteChange::EffectiveDateChanged {
            old: old_effective,
            new: new_effective,
        });
    }
    changes
}
/// Analyzes the impact of changing a statute in a collection.
pub fn analyze_change_impact(
    changed_statute: &Statute,
    old_version: &Statute,
    all_statutes: &[Statute],
) -> ChangeImpact {
    let changes = compare_statutes(old_version, changed_statute);
    let mut affected_statutes = Vec::new();
    for statute in all_statutes {
        if statute.id != changed_statute.id {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            if refs.contains(&changed_statute.id) {
                affected_statutes.push(statute.id.clone());
            }
        }
    }
    let impact_severity = if changes.iter().any(|c| {
        matches!(
            c,
            StatuteChange::EffectChanged { .. } | StatuteChange::PreconditionsChanged { .. }
        )
    }) && !affected_statutes.is_empty()
    {
        Severity::Critical
    } else if !affected_statutes.is_empty() || changes.len() > 3 {
        Severity::Warning
    } else {
        Severity::Info
    };
    let mut recommendations = Vec::new();
    if !affected_statutes.is_empty() {
        recommendations.push(format!(
            "Review and re-verify {} affected statute(s)",
            affected_statutes.len()
        ));
    }
    if changes
        .iter()
        .any(|c| matches!(c, StatuteChange::EffectChanged { .. }))
    {
        recommendations
            .push("Effect changed - verify compatibility with dependent statutes".to_string());
    }
    if changes
        .iter()
        .any(|c| matches!(c, StatuteChange::PreconditionsChanged { .. }))
    {
        recommendations.push("Preconditions changed - update test cases".to_string());
    }
    if changes
        .iter()
        .any(|c| matches!(c, StatuteChange::JurisdictionChanged { .. }))
    {
        recommendations.push("Jurisdiction changed - verify compliance requirements".to_string());
    }
    ChangeImpact {
        statute_id: changed_statute.id.clone(),
        changes,
        affected_statutes,
        impact_severity,
        recommendations,
    }
}
