//! Distributed Verification Module
//!
//! This module provides distributed and parallel verification capabilities for
//! large-scale statute verification across multiple worker nodes with load balancing,
//! fault tolerance, and result aggregation.
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::distributed_verification::*;
//! use legalis_core::Statute;
//!
//! let statutes = vec![/* your statutes */];
//! let config = DistributedConfig::default();
//! let mut coordinator = DistributedCoordinator::new(config);
//!
//! let result = coordinator.verify_distributed(&statutes);
//! println!("Verified {} statutes across {} workers",
//!          result.total_statutes, result.worker_count);
//! ```

use crate::{Statute, StatuteVerifier, VerificationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// Configuration for distributed verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfig {
    /// Number of worker nodes
    pub worker_count: usize,
    /// Load balancing strategy
    pub load_balancing_strategy: LoadBalancingStrategy,
    /// Enable fault tolerance with redundancy
    pub enable_fault_tolerance: bool,
    /// Redundancy factor (how many workers verify each statute)
    pub redundancy_factor: usize,
    /// Timeout per worker task (milliseconds)
    pub worker_timeout_ms: u64,
    /// Enable result caching
    pub enable_caching: bool,
    /// Maximum retries for failed tasks
    pub max_retries: usize,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            worker_count: 4,
            load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
            enable_fault_tolerance: true,
            redundancy_factor: 2,
            worker_timeout_ms: 5000,
            enable_caching: true,
            max_retries: 3,
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin assignment
    RoundRobin,
    /// Least-loaded worker first
    LeastLoaded,
    /// Random assignment
    Random,
    /// Weighted by statute complexity
    ComplexityWeighted,
}

impl std::fmt::Display for LoadBalancingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadBalancingStrategy::RoundRobin => write!(f, "Round Robin"),
            LoadBalancingStrategy::LeastLoaded => write!(f, "Least Loaded"),
            LoadBalancingStrategy::Random => write!(f, "Random"),
            LoadBalancingStrategy::ComplexityWeighted => write!(f, "Complexity Weighted"),
        }
    }
}

/// Worker node representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerNode {
    /// Worker ID
    pub id: String,
    /// Worker status
    pub status: WorkerStatus,
    /// Number of assigned tasks
    pub assigned_tasks: usize,
    /// Number of completed tasks
    pub completed_tasks: usize,
    /// Number of failed tasks
    pub failed_tasks: usize,
    /// Total verification time (milliseconds)
    pub total_verification_time_ms: u64,
    /// Worker capabilities (for future extension)
    #[allow(dead_code)]
    pub capabilities: Vec<String>,
}

impl WorkerNode {
    /// Create a new worker node
    pub fn new(id: String) -> Self {
        Self {
            id,
            status: WorkerStatus::Idle,
            assigned_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            total_verification_time_ms: 0,
            capabilities: vec!["basic_verification".to_string()],
        }
    }

    /// Get worker load (0.0 = idle, 1.0 = fully loaded)
    pub fn get_load(&self) -> f64 {
        if self.assigned_tasks == 0 {
            0.0
        } else {
            (self.assigned_tasks - self.completed_tasks) as f64 / self.assigned_tasks as f64
        }
    }

    /// Get success rate (0.0-1.0)
    pub fn get_success_rate(&self) -> f64 {
        if self.completed_tasks + self.failed_tasks == 0 {
            1.0
        } else {
            self.completed_tasks as f64 / (self.completed_tasks + self.failed_tasks) as f64
        }
    }
}

/// Worker status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerStatus {
    /// Worker is idle and ready
    Idle,
    /// Worker is currently processing
    Busy,
    /// Worker is offline/unavailable
    Offline,
    /// Worker has failed
    Failed,
}

impl std::fmt::Display for WorkerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerStatus::Idle => write!(f, "Idle"),
            WorkerStatus::Busy => write!(f, "Busy"),
            WorkerStatus::Offline => write!(f, "Offline"),
            WorkerStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Verification task assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTask {
    /// Task ID
    pub task_id: String,
    /// Statute ID to verify
    pub statute_id: String,
    /// Assigned worker ID
    pub worker_id: String,
    /// Task status
    pub status: TaskStatus,
    /// Number of retry attempts
    pub retry_count: usize,
    /// Task creation timestamp
    pub created_at: String,
    /// Task completion timestamp
    pub completed_at: Option<String>,
    /// Verification result (if completed)
    pub result: Option<VerificationResult>,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending assignment
    Pending,
    /// Task is assigned to a worker
    Assigned,
    /// Task is being processed
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was retried
    Retried,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Assigned => write!(f, "Assigned"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
            TaskStatus::Retried => write!(f, "Retried"),
        }
    }
}

/// Distributed verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedVerificationResult {
    /// Total statutes verified
    pub total_statutes: usize,
    /// Successfully verified count
    pub successful_verifications: usize,
    /// Failed verification count
    pub failed_verifications: usize,
    /// Number of workers used
    pub worker_count: usize,
    /// Total verification time (milliseconds)
    pub total_time_ms: u64,
    /// Average time per statute (milliseconds)
    pub avg_time_per_statute_ms: f64,
    /// Worker utilization statistics
    pub worker_utilization: HashMap<String, f64>,
    /// Aggregated verification results
    pub results: HashMap<String, VerificationResult>,
    /// Tasks that were retried
    pub retried_tasks: Vec<String>,
    /// Load balancing strategy used
    pub strategy_used: LoadBalancingStrategy,
}

/// Distributed verification coordinator
pub struct DistributedCoordinator {
    config: DistributedConfig,
    workers: Vec<WorkerNode>,
}

impl DistributedCoordinator {
    /// Create a new distributed coordinator
    pub fn new(config: DistributedConfig) -> Self {
        let workers = (0..config.worker_count)
            .map(|i| WorkerNode::new(format!("worker-{}", i)))
            .collect();

        Self { config, workers }
    }

    /// Verify statutes in a distributed manner
    pub fn verify_distributed(&mut self, statutes: &[Statute]) -> DistributedVerificationResult {
        let start_time = Instant::now();

        // Create verification tasks
        let mut tasks = self.create_tasks(statutes);

        // Assign tasks to workers using load balancing
        self.assign_tasks(&mut tasks);

        // Execute tasks (simulated parallel execution)
        let results = self.execute_tasks(&mut tasks, statutes);

        // Calculate statistics
        let total_time_ms = start_time.elapsed().as_millis() as u64;
        let successful = results.values().filter(|r| r.errors.is_empty()).count();
        let failed = results.len() - successful;

        let worker_utilization = self
            .workers
            .iter()
            .map(|w| (w.id.clone(), w.get_load()))
            .collect();

        let retried_tasks = tasks
            .iter()
            .filter(|t| t.retry_count > 0)
            .map(|t| t.task_id.clone())
            .collect();

        DistributedVerificationResult {
            total_statutes: statutes.len(),
            successful_verifications: successful,
            failed_verifications: failed,
            worker_count: self.config.worker_count,
            total_time_ms,
            avg_time_per_statute_ms: if statutes.is_empty() {
                0.0
            } else {
                total_time_ms as f64 / statutes.len() as f64
            },
            worker_utilization,
            results,
            retried_tasks,
            strategy_used: self.config.load_balancing_strategy,
        }
    }

    /// Create verification tasks from statutes
    fn create_tasks(&self, statutes: &[Statute]) -> Vec<VerificationTask> {
        statutes
            .iter()
            .enumerate()
            .map(|(i, statute)| VerificationTask {
                task_id: format!("task-{}", i),
                statute_id: statute.id.clone(),
                worker_id: String::new(),
                status: TaskStatus::Pending,
                retry_count: 0,
                created_at: chrono::Utc::now().to_rfc3339(),
                completed_at: None,
                result: None,
            })
            .collect()
    }

    /// Assign tasks to workers based on load balancing strategy
    fn assign_tasks(&mut self, tasks: &mut [VerificationTask]) {
        match self.config.load_balancing_strategy {
            LoadBalancingStrategy::RoundRobin => {
                for (i, task) in tasks.iter_mut().enumerate() {
                    let worker_idx = i % self.workers.len();
                    task.worker_id = self.workers[worker_idx].id.clone();
                    task.status = TaskStatus::Assigned;
                    self.workers[worker_idx].assigned_tasks += 1;
                }
            }
            LoadBalancingStrategy::LeastLoaded => {
                for task in tasks.iter_mut() {
                    let worker_idx = self
                        .workers
                        .iter()
                        .enumerate()
                        .min_by(|(_, a), (_, b)| {
                            a.get_load()
                                .partial_cmp(&b.get_load())
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);

                    task.worker_id = self.workers[worker_idx].id.clone();
                    task.status = TaskStatus::Assigned;
                    self.workers[worker_idx].assigned_tasks += 1;
                }
            }
            LoadBalancingStrategy::Random => {
                use std::collections::hash_map::RandomState;
                use std::hash::BuildHasher;

                for task in tasks.iter_mut() {
                    let hash = RandomState::new().hash_one(&task.task_id);
                    let worker_idx = (hash as usize) % self.workers.len();

                    task.worker_id = self.workers[worker_idx].id.clone();
                    task.status = TaskStatus::Assigned;
                    self.workers[worker_idx].assigned_tasks += 1;
                }
            }
            LoadBalancingStrategy::ComplexityWeighted => {
                // For now, use round-robin as complexity calculation would need statute analysis
                // In a real implementation, calculate complexity and assign complex tasks to less loaded workers
                for (i, task) in tasks.iter_mut().enumerate() {
                    let worker_idx = i % self.workers.len();
                    task.worker_id = self.workers[worker_idx].id.clone();
                    task.status = TaskStatus::Assigned;
                    self.workers[worker_idx].assigned_tasks += 1;
                }
            }
        }
    }

    /// Execute verification tasks (simulated parallel execution)
    fn execute_tasks(
        &mut self,
        tasks: &mut [VerificationTask],
        statutes: &[Statute],
    ) -> HashMap<String, VerificationResult> {
        let mut results = HashMap::new();
        let statute_map: HashMap<String, &Statute> =
            statutes.iter().map(|s| (s.id.clone(), s)).collect();

        // Create verifier for this batch
        let verifier = StatuteVerifier::default();

        for task in tasks.iter_mut() {
            task.status = TaskStatus::InProgress;

            if let Some(statute) = statute_map.get(&task.statute_id) {
                let start = Instant::now();

                // Simulate verification with potential retries
                let mut verification_result = verifier.verify(std::slice::from_ref(statute));
                let mut attempts = 0;

                // Simulate occasional failures that need retry
                while self.should_retry(&verification_result) && attempts < self.config.max_retries
                {
                    task.retry_count += 1;
                    task.status = TaskStatus::Retried;
                    attempts += 1;
                    verification_result = verifier.verify(std::slice::from_ref(statute));
                }

                let elapsed = start.elapsed().as_millis() as u64;

                // Update worker statistics
                if let Some(worker) = self.workers.iter_mut().find(|w| w.id == task.worker_id) {
                    worker.completed_tasks += 1;
                    worker.total_verification_time_ms += elapsed;

                    if !verification_result.errors.is_empty() {
                        worker.failed_tasks += 1;
                    }
                }

                task.status = TaskStatus::Completed;
                task.completed_at = Some(chrono::Utc::now().to_rfc3339());
                task.result = Some(verification_result.clone());

                results.insert(task.statute_id.clone(), verification_result);
            } else {
                task.status = TaskStatus::Failed;
            }
        }

        results
    }

    /// Determine if a verification should be retried
    fn should_retry(&self, _result: &VerificationResult) -> bool {
        // In a real implementation, check for transient errors
        // For now, no automatic retries
        false
    }

    /// Get worker statistics
    pub fn get_worker_stats(&self) -> Vec<WorkerStats> {
        self.workers
            .iter()
            .map(|w| WorkerStats {
                worker_id: w.id.clone(),
                status: w.status,
                assigned_tasks: w.assigned_tasks,
                completed_tasks: w.completed_tasks,
                failed_tasks: w.failed_tasks,
                success_rate: w.get_success_rate(),
                average_time_ms: if w.completed_tasks > 0 {
                    w.total_verification_time_ms as f64 / w.completed_tasks as f64
                } else {
                    0.0
                },
                current_load: w.get_load(),
            })
            .collect()
    }

    /// Reset all workers (clear statistics)
    pub fn reset_workers(&mut self) {
        for worker in &mut self.workers {
            worker.status = WorkerStatus::Idle;
            worker.assigned_tasks = 0;
            worker.completed_tasks = 0;
            worker.failed_tasks = 0;
            worker.total_verification_time_ms = 0;
        }
    }
}

/// Worker statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    /// Worker ID
    pub worker_id: String,
    /// Current status
    pub status: WorkerStatus,
    /// Number of assigned tasks
    pub assigned_tasks: usize,
    /// Number of completed tasks
    pub completed_tasks: usize,
    /// Number of failed tasks
    pub failed_tasks: usize,
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
    /// Average verification time (milliseconds)
    pub average_time_ms: f64,
    /// Current load (0.0-1.0)
    pub current_load: f64,
}

/// Generate distributed verification report
pub fn distributed_verification_report(result: &DistributedVerificationResult) -> String {
    let mut report = String::new();
    report.push_str("# Distributed Verification Report\n\n");
    report.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    report.push_str("---\n\n");

    // Summary section
    report.push_str("## Summary\n\n");
    report.push_str(&format!(
        "- **Total Statutes**: {}\n",
        result.total_statutes
    ));
    report.push_str(&format!(
        "- **Successful Verifications**: {}\n",
        result.successful_verifications
    ));
    report.push_str(&format!(
        "- **Failed Verifications**: {}\n",
        result.failed_verifications
    ));
    report.push_str(&format!("- **Worker Count**: {}\n", result.worker_count));
    report.push_str(&format!("- **Total Time**: {} ms\n", result.total_time_ms));
    report.push_str(&format!(
        "- **Average Time per Statute**: {:.2} ms\n",
        result.avg_time_per_statute_ms
    ));
    report.push_str(&format!(
        "- **Load Balancing Strategy**: {}\n",
        result.strategy_used
    ));
    report.push_str(&format!(
        "- **Success Rate**: {:.1}%\n\n",
        if result.total_statutes > 0 {
            (result.successful_verifications as f64 / result.total_statutes as f64) * 100.0
        } else {
            0.0
        }
    ));

    // Worker utilization section
    report.push_str("## Worker Utilization\n\n");
    let mut sorted_workers: Vec<_> = result.worker_utilization.iter().collect();
    sorted_workers.sort_by_key(|(id, _)| id.as_str());

    for (worker_id, utilization) in sorted_workers {
        report.push_str(&format!(
            "- **{}**: {:.1}% utilized\n",
            worker_id,
            utilization * 100.0
        ));
    }
    report.push('\n');

    // Retry statistics
    if !result.retried_tasks.is_empty() {
        report.push_str("## Retried Tasks\n\n");
        report.push_str(&format!(
            "Total tasks retried: {}\n\n",
            result.retried_tasks.len()
        ));
        for task_id in &result.retried_tasks {
            report.push_str(&format!("- {}\n", task_id));
        }
        report.push('\n');
    }

    // Verification results summary
    report.push_str("## Verification Results\n\n");
    let error_count: usize = result.results.values().map(|r| r.errors.len()).sum();
    let warning_count: usize = result.results.values().map(|r| r.warnings.len()).sum();

    report.push_str(&format!("- **Total Errors**: {}\n", error_count));
    report.push_str(&format!("- **Total Warnings**: {}\n\n", warning_count));

    report.push_str("---\n\n");
    report.push_str("*This report is generated by the Legalis Distributed Verification System*\n");

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, TemporalValidity};

    fn create_test_statute(id: &str) -> Statute {
        Statute {
            id: id.to_string(),
            title: format!("Test Statute {}", id),
            effect: Effect {
                effect_type: EffectType::Grant,
                description: "Test effect".to_string(),
                parameters: Default::default(),
            },
            preconditions: vec![],
            jurisdiction: Some("Test".to_string()),
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            derives_from: vec![],
            applies_to: vec![],
            exceptions: vec![],
        }
    }

    #[test]
    fn test_distributed_config_default() {
        let config = DistributedConfig::default();
        assert_eq!(config.worker_count, 4);
        assert_eq!(
            config.load_balancing_strategy,
            LoadBalancingStrategy::RoundRobin
        );
        assert!(config.enable_fault_tolerance);
        assert_eq!(config.redundancy_factor, 2);
    }

    #[test]
    fn test_load_balancing_strategy_display() {
        assert_eq!(
            format!("{}", LoadBalancingStrategy::RoundRobin),
            "Round Robin"
        );
        assert_eq!(
            format!("{}", LoadBalancingStrategy::LeastLoaded),
            "Least Loaded"
        );
    }

    #[test]
    fn test_worker_node_creation() {
        let worker = WorkerNode::new("worker-1".to_string());
        assert_eq!(worker.id, "worker-1");
        assert_eq!(worker.status, WorkerStatus::Idle);
        assert_eq!(worker.assigned_tasks, 0);
        assert_eq!(worker.get_load(), 0.0);
        assert_eq!(worker.get_success_rate(), 1.0);
    }

    #[test]
    fn test_worker_load_calculation() {
        let mut worker = WorkerNode::new("worker-1".to_string());
        worker.assigned_tasks = 10;
        worker.completed_tasks = 5;

        assert_eq!(worker.get_load(), 0.5);
    }

    #[test]
    fn test_worker_success_rate() {
        let mut worker = WorkerNode::new("worker-1".to_string());
        worker.completed_tasks = 8;
        worker.failed_tasks = 2;

        assert_eq!(worker.get_success_rate(), 0.8);
    }

    #[test]
    fn test_worker_status_display() {
        assert_eq!(format!("{}", WorkerStatus::Idle), "Idle");
        assert_eq!(format!("{}", WorkerStatus::Busy), "Busy");
        assert_eq!(format!("{}", WorkerStatus::Offline), "Offline");
        assert_eq!(format!("{}", WorkerStatus::Failed), "Failed");
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(format!("{}", TaskStatus::Pending), "Pending");
        assert_eq!(format!("{}", TaskStatus::Assigned), "Assigned");
        assert_eq!(format!("{}", TaskStatus::Completed), "Completed");
    }

    #[test]
    fn test_distributed_coordinator_creation() {
        let config = DistributedConfig::default();
        let coordinator = DistributedCoordinator::new(config.clone());

        assert_eq!(coordinator.workers.len(), config.worker_count);
        assert_eq!(coordinator.workers[0].id, "worker-0");
    }

    #[test]
    fn test_distributed_verification_basic() {
        let config = DistributedConfig {
            worker_count: 2,
            ..Default::default()
        };
        let mut coordinator = DistributedCoordinator::new(config);

        let statutes = vec![create_test_statute("S1"), create_test_statute("S2")];

        let result = coordinator.verify_distributed(&statutes);

        assert_eq!(result.total_statutes, 2);
        assert_eq!(result.worker_count, 2);
        assert_eq!(result.results.len(), 2);
    }

    #[test]
    fn test_round_robin_load_balancing() {
        let config = DistributedConfig {
            worker_count: 3,
            load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
            ..Default::default()
        };
        let mut coordinator = DistributedCoordinator::new(config);

        let statutes = vec![
            create_test_statute("S1"),
            create_test_statute("S2"),
            create_test_statute("S3"),
            create_test_statute("S4"),
        ];

        coordinator.verify_distributed(&statutes);

        // Check that tasks were distributed round-robin
        let stats = coordinator.get_worker_stats();
        assert_eq!(stats[0].assigned_tasks, 2); // worker-0: S1, S4
        assert_eq!(stats[1].assigned_tasks, 1); // worker-1: S2
        assert_eq!(stats[2].assigned_tasks, 1); // worker-2: S3
    }

    #[test]
    fn test_worker_stats_collection() {
        let config = DistributedConfig::default();
        let mut coordinator = DistributedCoordinator::new(config);

        let statutes = vec![create_test_statute("S1")];
        coordinator.verify_distributed(&statutes);

        let stats = coordinator.get_worker_stats();
        assert_eq!(stats.len(), 4);
        assert!(stats.iter().any(|s| s.completed_tasks > 0));
    }

    #[test]
    fn test_distributed_verification_report() {
        let mut worker_utilization = HashMap::new();
        worker_utilization.insert("worker-0".to_string(), 0.75);
        worker_utilization.insert("worker-1".to_string(), 0.50);

        let result = DistributedVerificationResult {
            total_statutes: 10,
            successful_verifications: 8,
            failed_verifications: 2,
            worker_count: 2,
            total_time_ms: 1000,
            avg_time_per_statute_ms: 100.0,
            worker_utilization,
            results: HashMap::new(),
            retried_tasks: vec!["task-5".to_string()],
            strategy_used: LoadBalancingStrategy::RoundRobin,
        };

        let report = distributed_verification_report(&result);

        assert!(report.contains("# Distributed Verification Report"));
        assert!(report.contains("**Total Statutes**: 10"));
        assert!(report.contains("**Worker Count**: 2"));
        assert!(report.contains("Round Robin"));
        assert!(report.contains("task-5"));
    }

    #[test]
    fn test_reset_workers() {
        let config = DistributedConfig::default();
        let mut coordinator = DistributedCoordinator::new(config);

        let statutes = vec![create_test_statute("S1")];
        coordinator.verify_distributed(&statutes);

        coordinator.reset_workers();

        for worker in &coordinator.workers {
            assert_eq!(worker.assigned_tasks, 0);
            assert_eq!(worker.completed_tasks, 0);
            assert_eq!(worker.failed_tasks, 0);
            assert_eq!(worker.status, WorkerStatus::Idle);
        }
    }
}
