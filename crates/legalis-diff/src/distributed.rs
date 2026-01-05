//! Distributed Diff Computation
//!
//! This module provides distributed computation capabilities for large-scale
//! diff operations across multiple nodes.

use crate::{DiffResult, StatuteDiff, diff};
use legalis_core::Statute;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Distributed computation node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node identifier
    pub node_id: String,
    /// Node address (host:port)
    pub address: String,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: usize,
    /// Node capacity (arbitrary units)
    pub capacity: usize,
}

/// Distributed diff task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTask {
    /// Task identifier
    pub task_id: String,
    /// Old statute
    pub old_statute: Statute,
    /// New statute
    pub new_statute: Statute,
    /// Priority level (higher = more important)
    pub priority: u32,
}

/// Task result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task identifier
    pub task_id: String,
    /// Node that computed the result
    pub node_id: String,
    /// Computed diff
    pub diff: StatuteDiff,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Distributed computation coordinator
pub struct DistributedCoordinator {
    nodes: Vec<NodeConfig>,
    task_queue: Arc<Mutex<Vec<DiffTask>>>,
    results: Arc<Mutex<HashMap<String, TaskResult>>>,
}

impl DistributedCoordinator {
    /// Creates a new distributed coordinator with the given nodes
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::distributed::{DistributedCoordinator, NodeConfig};
    ///
    /// let nodes = vec![
    ///     NodeConfig {
    ///         node_id: "node-1".to_string(),
    ///         address: "localhost:8001".to_string(),
    ///         max_concurrent_tasks: 4,
    ///         capacity: 100,
    ///     },
    ///     NodeConfig {
    ///         node_id: "node-2".to_string(),
    ///         address: "localhost:8002".to_string(),
    ///         max_concurrent_tasks: 4,
    ///         capacity: 100,
    ///     },
    /// ];
    ///
    /// let coordinator = DistributedCoordinator::new(nodes);
    /// ```
    pub fn new(nodes: Vec<NodeConfig>) -> Self {
        Self {
            nodes,
            task_queue: Arc::new(Mutex::new(Vec::new())),
            results: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Submits a diff task to the distributed queue
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::distributed::{DistributedCoordinator, DiffTask};
    ///
    /// let coordinator = DistributedCoordinator::new(vec![]);
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let task = DiffTask {
    ///     task_id: "task-001".to_string(),
    ///     old_statute: old,
    ///     new_statute: new,
    ///     priority: 5,
    /// };
    ///
    /// coordinator.submit_task(task);
    /// ```
    pub fn submit_task(&self, task: DiffTask) {
        let mut queue = self.task_queue.lock().unwrap();
        queue.push(task);
        // Sort by priority (higher priority first)
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Submits multiple diff tasks
    pub fn submit_batch(&self, tasks: Vec<DiffTask>) {
        let mut queue = self.task_queue.lock().unwrap();
        queue.extend(tasks);
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Processes all queued tasks using local parallel computation
    ///
    /// In a real distributed system, this would distribute tasks across nodes.
    /// For this implementation, we simulate distributed processing using rayon.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::distributed::{DistributedCoordinator, DiffTask, NodeConfig};
    ///
    /// let nodes = vec![
    ///     NodeConfig {
    ///         node_id: "node-1".to_string(),
    ///         address: "localhost:8001".to_string(),
    ///         max_concurrent_tasks: 4,
    ///         capacity: 100,
    ///     },
    /// ];
    ///
    /// let coordinator = DistributedCoordinator::new(nodes);
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let task = DiffTask {
    ///     task_id: "task-001".to_string(),
    ///     old_statute: old,
    ///     new_statute: new,
    ///     priority: 5,
    /// };
    ///
    /// coordinator.submit_task(task);
    /// coordinator.process_all().unwrap();
    /// ```
    pub fn process_all(&self) -> DiffResult<()> {
        let tasks: Vec<DiffTask> = {
            let mut queue = self.task_queue.lock().unwrap();
            queue.drain(..).collect()
        };

        if tasks.is_empty() {
            return Ok(());
        }

        // Process tasks in parallel using rayon
        let results: Vec<TaskResult> = tasks
            .par_iter()
            .enumerate()
            .map(|(idx, task)| {
                let start = std::time::Instant::now();
                let diff_result = diff(&task.old_statute, &task.new_statute)?;
                let elapsed = start.elapsed().as_millis() as u64;

                // Simulate node assignment (round-robin)
                let node_id = if !self.nodes.is_empty() {
                    self.nodes[idx % self.nodes.len()].node_id.clone()
                } else {
                    "local".to_string()
                };

                Ok(TaskResult {
                    task_id: task.task_id.clone(),
                    node_id,
                    diff: diff_result,
                    computation_time_ms: elapsed,
                })
            })
            .collect::<DiffResult<Vec<TaskResult>>>()?;

        // Store results
        let mut result_map = self.results.lock().unwrap();
        for result in results {
            result_map.insert(result.task_id.clone(), result);
        }

        Ok(())
    }

    /// Gets the result of a specific task
    pub fn get_result(&self, task_id: &str) -> Option<TaskResult> {
        let results = self.results.lock().unwrap();
        results.get(task_id).cloned()
    }

    /// Gets all completed results
    pub fn get_all_results(&self) -> Vec<TaskResult> {
        let results = self.results.lock().unwrap();
        results.values().cloned().collect()
    }

    /// Gets the number of pending tasks
    pub fn pending_tasks(&self) -> usize {
        let queue = self.task_queue.lock().unwrap();
        queue.len()
    }

    /// Gets the number of completed tasks
    pub fn completed_tasks(&self) -> usize {
        let results = self.results.lock().unwrap();
        results.len()
    }

    /// Clears all results
    pub fn clear_results(&self) {
        let mut results = self.results.lock().unwrap();
        results.clear();
    }

    /// Gets statistics about the distributed computation
    pub fn get_statistics(&self) -> ComputationStats {
        let results = self.results.lock().unwrap();
        let queue = self.task_queue.lock().unwrap();

        let total_time: u64 = results.values().map(|r| r.computation_time_ms).sum();
        let avg_time = if results.is_empty() {
            0.0
        } else {
            total_time as f64 / results.len() as f64
        };

        let mut node_counts: HashMap<String, usize> = HashMap::new();
        for result in results.values() {
            *node_counts.entry(result.node_id.clone()).or_insert(0) += 1;
        }

        ComputationStats {
            total_tasks: results.len() + queue.len(),
            completed_tasks: results.len(),
            pending_tasks: queue.len(),
            total_computation_time_ms: total_time,
            average_computation_time_ms: avg_time,
            tasks_per_node: node_counts,
        }
    }
}

/// Statistics about distributed computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationStats {
    /// Total number of tasks (completed + pending)
    pub total_tasks: usize,
    /// Number of completed tasks
    pub completed_tasks: usize,
    /// Number of pending tasks
    pub pending_tasks: usize,
    /// Total computation time in milliseconds
    pub total_computation_time_ms: u64,
    /// Average computation time per task
    pub average_computation_time_ms: f64,
    /// Tasks processed per node
    pub tasks_per_node: HashMap<String, usize>,
}

/// Creates a distributed diff batch computation
///
/// This is a convenience function that creates a coordinator, submits tasks,
/// processes them, and returns the results.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::distributed::{distributed_diff_batch, NodeConfig};
///
/// let nodes = vec![
///     NodeConfig {
///         node_id: "node-1".to_string(),
///         address: "localhost:8001".to_string(),
///         max_concurrent_tasks: 4,
///         capacity: 100,
///     },
/// ];
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
///
/// let pairs = vec![(old, new)];
/// let results = distributed_diff_batch(&pairs, &nodes).unwrap();
/// assert_eq!(results.len(), 1);
/// ```
pub fn distributed_diff_batch(
    statute_pairs: &[(Statute, Statute)],
    nodes: &[NodeConfig],
) -> DiffResult<Vec<StatuteDiff>> {
    let coordinator = DistributedCoordinator::new(nodes.to_vec());

    let tasks: Vec<DiffTask> = statute_pairs
        .iter()
        .enumerate()
        .map(|(idx, (old, new))| DiffTask {
            task_id: format!("task-{}", idx),
            old_statute: old.clone(),
            new_statute: new.clone(),
            priority: 0,
        })
        .collect();

    coordinator.submit_batch(tasks);
    coordinator.process_all()?;

    let results = coordinator.get_all_results();
    Ok(results.into_iter().map(|r| r.diff).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn create_test_statutes() -> (Statute, Statute) {
        let old = Statute::new(
            "test-law",
            "Old Title",
            Effect::new(EffectType::Grant, "Old benefit"),
        );
        let new = Statute::new(
            "test-law",
            "New Title",
            Effect::new(EffectType::Grant, "New benefit"),
        );
        (old, new)
    }

    fn create_test_nodes() -> Vec<NodeConfig> {
        vec![
            NodeConfig {
                node_id: "node-1".to_string(),
                address: "localhost:8001".to_string(),
                max_concurrent_tasks: 4,
                capacity: 100,
            },
            NodeConfig {
                node_id: "node-2".to_string(),
                address: "localhost:8002".to_string(),
                max_concurrent_tasks: 4,
                capacity: 100,
            },
        ]
    }

    #[test]
    fn test_coordinator_creation() {
        let nodes = create_test_nodes();
        let coordinator = DistributedCoordinator::new(nodes);
        assert_eq!(coordinator.pending_tasks(), 0);
        assert_eq!(coordinator.completed_tasks(), 0);
    }

    #[test]
    fn test_submit_task() {
        let coordinator = DistributedCoordinator::new(vec![]);
        let (old, new) = create_test_statutes();

        let task = DiffTask {
            task_id: "task-001".to_string(),
            old_statute: old,
            new_statute: new,
            priority: 5,
        };

        coordinator.submit_task(task);
        assert_eq!(coordinator.pending_tasks(), 1);
    }

    #[test]
    fn test_submit_batch() {
        let coordinator = DistributedCoordinator::new(vec![]);
        let (old, new) = create_test_statutes();

        let tasks = vec![
            DiffTask {
                task_id: "task-001".to_string(),
                old_statute: old.clone(),
                new_statute: new.clone(),
                priority: 5,
            },
            DiffTask {
                task_id: "task-002".to_string(),
                old_statute: old,
                new_statute: new,
                priority: 3,
            },
        ];

        coordinator.submit_batch(tasks);
        assert_eq!(coordinator.pending_tasks(), 2);
    }

    #[test]
    fn test_process_all() {
        let nodes = create_test_nodes();
        let coordinator = DistributedCoordinator::new(nodes);
        let (old, new) = create_test_statutes();

        let task = DiffTask {
            task_id: "task-001".to_string(),
            old_statute: old,
            new_statute: new,
            priority: 5,
        };

        coordinator.submit_task(task);
        coordinator.process_all().unwrap();

        assert_eq!(coordinator.pending_tasks(), 0);
        assert_eq!(coordinator.completed_tasks(), 1);
    }

    #[test]
    fn test_get_result() {
        let coordinator = DistributedCoordinator::new(vec![]);
        let (old, new) = create_test_statutes();

        let task = DiffTask {
            task_id: "task-001".to_string(),
            old_statute: old,
            new_statute: new,
            priority: 5,
        };

        coordinator.submit_task(task);
        coordinator.process_all().unwrap();

        let result = coordinator.get_result("task-001");
        assert!(result.is_some());

        let task_result = result.unwrap();
        assert_eq!(task_result.task_id, "task-001");
    }

    #[test]
    fn test_get_all_results() {
        let coordinator = DistributedCoordinator::new(vec![]);
        let (old, new) = create_test_statutes();

        let tasks = vec![
            DiffTask {
                task_id: "task-001".to_string(),
                old_statute: old.clone(),
                new_statute: new.clone(),
                priority: 5,
            },
            DiffTask {
                task_id: "task-002".to_string(),
                old_statute: old,
                new_statute: new,
                priority: 3,
            },
        ];

        coordinator.submit_batch(tasks);
        coordinator.process_all().unwrap();

        let results = coordinator.get_all_results();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_statistics() {
        let nodes = create_test_nodes();
        let coordinator = DistributedCoordinator::new(nodes);
        let (old, new) = create_test_statutes();

        let task = DiffTask {
            task_id: "task-001".to_string(),
            old_statute: old,
            new_statute: new,
            priority: 5,
        };

        coordinator.submit_task(task);
        coordinator.process_all().unwrap();

        let stats = coordinator.get_statistics();
        assert_eq!(stats.completed_tasks, 1);
        assert_eq!(stats.pending_tasks, 0);
        assert_eq!(stats.total_tasks, 1);
    }

    #[test]
    fn test_distributed_batch() {
        let nodes = create_test_nodes();
        let (old1, new1) = create_test_statutes();
        let (old2, new2) = create_test_statutes();

        let pairs = vec![(old1, new1), (old2, new2)];
        let results = distributed_diff_batch(&pairs, &nodes).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_priority_ordering() {
        let coordinator = DistributedCoordinator::new(vec![]);
        let (old, new) = create_test_statutes();

        let low_priority = DiffTask {
            task_id: "low".to_string(),
            old_statute: old.clone(),
            new_statute: new.clone(),
            priority: 1,
        };

        let high_priority = DiffTask {
            task_id: "high".to_string(),
            old_statute: old,
            new_statute: new,
            priority: 10,
        };

        coordinator.submit_task(low_priority);
        coordinator.submit_task(high_priority);

        // Queue should be sorted by priority
        let queue = coordinator.task_queue.lock().unwrap();
        assert_eq!(queue[0].task_id, "high");
        assert_eq!(queue[1].task_id, "low");
    }

    #[test]
    fn test_clear_results() {
        let coordinator = DistributedCoordinator::new(vec![]);
        let (old, new) = create_test_statutes();

        let task = DiffTask {
            task_id: "task-001".to_string(),
            old_statute: old,
            new_statute: new,
            priority: 5,
        };

        coordinator.submit_task(task);
        coordinator.process_all().unwrap();
        assert_eq!(coordinator.completed_tasks(), 1);

        coordinator.clear_results();
        assert_eq!(coordinator.completed_tasks(), 0);
    }
}
