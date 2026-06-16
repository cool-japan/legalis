//! Task prioritisation.
//!
//! [`TaskPrioritizer`] ranks [`PracticeTask`]s with a transparent weighted model
//! combining four signals: **urgency** (deadline proximity against a horizon),
//! **importance** (an explicit 1-5 band), **dependency leverage** (how many
//! other tasks are blocked by this one) and an **effort** quick-win factor.
//! Dependencies are honoured: a task is *ready* only when all of its
//! dependencies are done, and circular dependencies are detected and reported.

use crate::Jurisdiction;
use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The lifecycle status of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PracticeTaskStatus {
    /// Not started.
    #[default]
    Todo,
    /// In progress.
    InProgress,
    /// Blocked by an external factor.
    Blocked,
    /// Completed.
    Done,
}

impl PracticeTaskStatus {
    /// Returns whether the task is still open (not done).
    pub fn is_open(&self) -> bool {
        !matches!(self, PracticeTaskStatus::Done)
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            PracticeTaskStatus::Todo => "todo",
            PracticeTaskStatus::InProgress => "in progress",
            PracticeTaskStatus::Blocked => "blocked",
            PracticeTaskStatus::Done => "done",
        }
    }
}

/// A unit of legal work to be prioritised.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PracticeTask {
    /// Stable identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Importance band, clamped to `1..=5`.
    pub importance: u8,
    /// Estimated effort in hours.
    pub effort_hours: f64,
    /// Optional deadline.
    pub deadline: Option<NaiveDate>,
    /// Ids of tasks that must be done before this one.
    pub depends_on: Vec<String>,
    /// Lifecycle status.
    pub status: PracticeTaskStatus,
    /// Optional assignee.
    pub assignee: Option<String>,
    /// Optional jurisdiction.
    pub jurisdiction: Option<Jurisdiction>,
}

impl PracticeTask {
    /// Creates a task with default importance (3) and no deadline.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            importance: 3,
            effort_hours: 1.0,
            deadline: None,
            depends_on: Vec::new(),
            status: PracticeTaskStatus::Todo,
            assignee: None,
            jurisdiction: None,
        }
    }

    /// Sets the importance (clamped to `1..=5`).
    pub fn with_importance(mut self, importance: u8) -> Self {
        self.importance = importance.clamp(1, 5);
        self
    }

    /// Sets the estimated effort in hours.
    pub fn with_effort(mut self, effort_hours: f64) -> Self {
        self.effort_hours = effort_hours.max(0.0);
        self
    }

    /// Sets the deadline.
    pub fn with_deadline(mut self, deadline: NaiveDate) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Adds a dependency on another task id.
    pub fn depending_on(mut self, task_id: impl Into<String>) -> Self {
        self.depends_on.push(task_id.into());
        self
    }

    /// Sets the status.
    pub fn with_status(mut self, status: PracticeTaskStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the assignee.
    pub fn with_assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }
}

/// The relative weights applied to each priority signal.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PriorityWeights {
    /// Weight of deadline urgency.
    pub urgency: f64,
    /// Weight of explicit importance.
    pub importance: f64,
    /// Weight of dependency leverage.
    pub dependency: f64,
    /// Weight of the quick-win (low-effort) factor.
    pub effort: f64,
}

impl Default for PriorityWeights {
    fn default() -> Self {
        Self {
            urgency: 0.40,
            importance: 0.30,
            dependency: 0.20,
            effort: 0.10,
        }
    }
}

impl PriorityWeights {
    /// Returns the sum of all weights (used to normalise the score).
    pub fn total(&self) -> f64 {
        self.urgency + self.importance + self.dependency + self.effort
    }
}

/// The individual normalised (`0.0`..=`1.0`) signal values for a task.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PriorityComponents {
    /// Deadline urgency.
    pub urgency: f64,
    /// Explicit importance.
    pub importance: f64,
    /// Dependency leverage.
    pub dependency: f64,
    /// Quick-win factor.
    pub effort: f64,
}

/// The computed priority of a single task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorityScore {
    /// Task id.
    pub task_id: String,
    /// Task title.
    pub title: String,
    /// Overall score in `0.0`..=`100.0`.
    pub score: f64,
    /// The component breakdown.
    pub components: PriorityComponents,
    /// Whether all dependencies are done (and the task itself is open).
    pub is_ready: bool,
    /// Whether the task is blocked (explicitly or by dependencies).
    pub is_blocked: bool,
    /// 1-based rank after sorting.
    pub rank: usize,
}

/// Ranks tasks by a weighted priority model.
#[derive(Debug, Clone)]
pub struct TaskPrioritizer {
    weights: PriorityWeights,
    horizon_days: i64,
}

impl Default for TaskPrioritizer {
    fn default() -> Self {
        Self {
            weights: PriorityWeights::default(),
            horizon_days: 30,
        }
    }
}

impl TaskPrioritizer {
    /// Creates a prioritiser with default weights and a 30-day urgency horizon.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the scoring weights.
    pub fn with_weights(mut self, weights: PriorityWeights) -> Self {
        self.weights = weights;
        self
    }

    /// Sets the urgency horizon (in days). Deadlines beyond it contribute no
    /// urgency.
    pub fn with_horizon(mut self, horizon_days: i64) -> Self {
        self.horizon_days = horizon_days.max(1);
        self
    }

    /// Ranks the tasks as of `today`, returning scores sorted by descending
    /// priority. Returns an error if the dependency graph contains a cycle.
    pub fn prioritize(
        &self,
        tasks: &[PracticeTask],
        today: NaiveDate,
    ) -> Result<Vec<PriorityScore>> {
        detect_cycle(tasks)?;

        let done_ids: HashSet<&str> = tasks
            .iter()
            .filter(|task| task.status == PracticeTaskStatus::Done)
            .map(|task| task.id.as_str())
            .collect();

        // Count how many tasks depend on each task (dependency leverage).
        let mut dependents: HashMap<&str, usize> = HashMap::new();
        for task in tasks {
            for dependency in &task.depends_on {
                *dependents.entry(dependency.as_str()).or_insert(0) += 1;
            }
        }
        let max_dependents = dependents.values().copied().max().unwrap_or(0);
        let max_effort = tasks
            .iter()
            .map(|task| task.effort_hours)
            .fold(0.0_f64, f64::max);

        let mut scores: Vec<PriorityScore> = tasks
            .iter()
            .map(|task| {
                let is_done = task.status == PracticeTaskStatus::Done;
                let deps_done = task
                    .depends_on
                    .iter()
                    .all(|dep| done_ids.contains(dep.as_str()));
                let is_ready = !is_done && deps_done;
                let is_blocked =
                    !is_done && (task.status == PracticeTaskStatus::Blocked || !deps_done);

                let components = PriorityComponents {
                    urgency: urgency_factor(task.deadline, today, self.horizon_days),
                    importance: f64::from(task.importance.clamp(1, 5)) / 5.0,
                    dependency: if max_dependents > 0 {
                        dependents.get(task.id.as_str()).copied().unwrap_or(0) as f64
                            / max_dependents as f64
                    } else {
                        0.0
                    },
                    effort: if max_effort > 0.0 {
                        1.0 - (task.effort_hours / max_effort)
                    } else {
                        0.0
                    },
                };

                let score = if is_done {
                    0.0
                } else {
                    let weighted = self.weights.urgency * components.urgency
                        + self.weights.importance * components.importance
                        + self.weights.dependency * components.dependency
                        + self.weights.effort * components.effort;
                    let total = self.weights.total();
                    if total > 0.0 {
                        100.0 * weighted / total
                    } else {
                        0.0
                    }
                };

                PriorityScore {
                    task_id: task.id.clone(),
                    title: task.title.clone(),
                    score,
                    components,
                    is_ready,
                    is_blocked,
                    rank: 0,
                }
            })
            .collect();

        scores.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.task_id.cmp(&b.task_id))
        });
        for (index, score) in scores.iter_mut().enumerate() {
            score.rank = index + 1;
        }
        Ok(scores)
    }

    /// Returns the prioritised, ready (unblocked, open) tasks as of `today`.
    pub fn ready_tasks(
        &self,
        tasks: &[PracticeTask],
        today: NaiveDate,
    ) -> Result<Vec<PriorityScore>> {
        Ok(self
            .prioritize(tasks, today)?
            .into_iter()
            .filter(|score| score.is_ready)
            .collect())
    }

    /// Returns the single highest-priority ready task as of `today`.
    pub fn next_action(
        &self,
        tasks: &[PracticeTask],
        today: NaiveDate,
    ) -> Result<Option<PriorityScore>> {
        Ok(self.ready_tasks(tasks, today)?.into_iter().next())
    }
}

/// Computes the urgency factor (`0.0`..=`1.0`) for a deadline.
fn urgency_factor(deadline: Option<NaiveDate>, today: NaiveDate, horizon_days: i64) -> f64 {
    match deadline {
        None => 0.0,
        Some(due) => {
            let days = (due - today).num_days();
            if days <= 0 {
                1.0
            } else if days >= horizon_days {
                0.0
            } else {
                1.0 - (days as f64 / horizon_days as f64)
            }
        }
    }
}

/// Detects a cycle in the task dependency graph.
fn detect_cycle(tasks: &[PracticeTask]) -> Result<()> {
    #[derive(Clone, Copy, PartialEq)]
    enum Mark {
        Unvisited,
        InStack,
        Done,
    }

    let index: HashMap<&str, &PracticeTask> =
        tasks.iter().map(|task| (task.id.as_str(), task)).collect();
    let mut marks: HashMap<&str, Mark> = tasks
        .iter()
        .map(|task| (task.id.as_str(), Mark::Unvisited))
        .collect();

    // Iterative DFS to avoid recursion depth issues on large graphs.
    for task in tasks {
        if marks.get(task.id.as_str()).copied() != Some(Mark::Unvisited) {
            continue;
        }
        let mut stack: Vec<(&str, usize)> = vec![(task.id.as_str(), 0)];
        marks.insert(task.id.as_str(), Mark::InStack);
        while let Some((node, child)) = stack.last().copied() {
            let deps: &[String] = index
                .get(node)
                .map(|task| task.depends_on.as_slice())
                .unwrap_or(&[]);
            if child < deps.len() {
                if let Some(last) = stack.last_mut() {
                    last.1 += 1;
                }
                let next = deps[child].as_str();
                // Dependencies on unknown ids are treated as leaves.
                if !index.contains_key(next) {
                    continue;
                }
                match marks.get(next).copied() {
                    Some(Mark::InStack) => bail!("dependency cycle detected involving '{}'", next),
                    Some(Mark::Done) => {}
                    _ => {
                        marks.insert(next, Mark::InStack);
                        stack.push((next, 0));
                    }
                }
            } else {
                marks.insert(node, Mark::Done);
                stack.pop();
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    #[test]
    fn test_urgency_drives_score() {
        let today = date(2026, 6, 14);
        let tasks = vec![
            PracticeTask::new("soon", "Due soon")
                .with_importance(3)
                .with_deadline(date(2026, 6, 16)),
            PracticeTask::new("later", "Due later")
                .with_importance(3)
                .with_deadline(date(2026, 9, 1)),
        ];
        let prioritizer = TaskPrioritizer::new();
        let scores = prioritizer.prioritize(&tasks, today).expect("ok");
        assert_eq!(scores[0].task_id, "soon");
        assert_eq!(scores[0].rank, 1);
        assert!(scores[0].score > scores[1].score);
        assert!(scores[0].components.urgency > scores[1].components.urgency);
    }

    #[test]
    fn test_dependency_readiness() {
        let today = date(2026, 6, 14);
        let tasks = vec![
            PracticeTask::new("a", "Prereq"),
            PracticeTask::new("b", "Dependent").depending_on("a"),
        ];
        let prioritizer = TaskPrioritizer::new();
        let scores = prioritizer.prioritize(&tasks, today).expect("ok");
        let b = scores.iter().find(|s| s.task_id == "b").expect("b");
        assert!(!b.is_ready);
        assert!(b.is_blocked);

        let tasks_done = vec![
            PracticeTask::new("a", "Prereq").with_status(PracticeTaskStatus::Done),
            PracticeTask::new("b", "Dependent").depending_on("a"),
        ];
        let scores2 = prioritizer.prioritize(&tasks_done, today).expect("ok");
        let b2 = scores2.iter().find(|s| s.task_id == "b").expect("b");
        assert!(b2.is_ready);
        assert!(!b2.is_blocked);
    }

    #[test]
    fn test_dependency_leverage() {
        let today = date(2026, 6, 14);
        // `blocker` is depended on by three tasks -> high dependency leverage.
        let tasks = vec![
            PracticeTask::new("blocker", "Blocks many").with_importance(3),
            PracticeTask::new("x", "X").depending_on("blocker"),
            PracticeTask::new("y", "Y").depending_on("blocker"),
            PracticeTask::new("z", "Z").depending_on("blocker"),
            PracticeTask::new("lonely", "No leverage").with_importance(3),
        ];
        let prioritizer = TaskPrioritizer::new();
        let scores = prioritizer.prioritize(&tasks, today).expect("ok");
        let blocker = scores.iter().find(|s| s.task_id == "blocker").expect("b");
        let lonely = scores.iter().find(|s| s.task_id == "lonely").expect("l");
        assert!((blocker.components.dependency - 1.0).abs() < f64::EPSILON);
        assert!(lonely.components.dependency.abs() < f64::EPSILON);
        assert!(blocker.score > lonely.score);
    }

    #[test]
    fn test_cycle_detection() {
        let tasks = vec![
            PracticeTask::new("a", "A").depending_on("b"),
            PracticeTask::new("b", "B").depending_on("a"),
        ];
        let prioritizer = TaskPrioritizer::new();
        assert!(prioritizer.prioritize(&tasks, date(2026, 6, 14)).is_err());
    }

    #[test]
    fn test_next_action_and_done_excluded() {
        let today = date(2026, 6, 14);
        let tasks = vec![
            PracticeTask::new("done", "Done already")
                .with_importance(5)
                .with_deadline(date(2026, 6, 15))
                .with_status(PracticeTaskStatus::Done),
            PracticeTask::new("urgent", "Urgent ready")
                .with_importance(5)
                .with_deadline(date(2026, 6, 15)),
            PracticeTask::new("blocked", "Blocked task")
                .with_importance(5)
                .with_deadline(date(2026, 6, 15))
                .depending_on("urgent"),
        ];
        let prioritizer = TaskPrioritizer::new();
        let next = prioritizer
            .next_action(&tasks, today)
            .expect("ok")
            .expect("some");
        assert_eq!(next.task_id, "urgent");
        // Done task scores zero.
        let scores = prioritizer.prioritize(&tasks, today).expect("ok");
        let done = scores.iter().find(|s| s.task_id == "done").expect("d");
        assert!(done.score.abs() < f64::EPSILON);
    }
}
