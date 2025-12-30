//! Kubernetes operator for auto-scaling LLM infrastructure.
//!
//! This module provides a Kubernetes operator that automatically scales
//! LLM deployments based on load metrics, queue depth, and other factors.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Custom Resource Definition for LLM deployments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMDeployment {
    /// Metadata about the deployment
    pub metadata: DeploymentMetadata,
    /// Specification for the deployment
    pub spec: DeploymentSpec,
    /// Current status of the deployment
    pub status: Option<DeploymentStatus>,
}

/// Metadata for LLM deployments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMetadata {
    /// Deployment name
    pub name: String,
    /// Kubernetes namespace
    pub namespace: String,
    /// Labels for the deployment
    pub labels: HashMap<String, String>,
    /// Annotations
    pub annotations: HashMap<String, String>,
}

/// Specification for LLM deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSpec {
    /// LLM provider to use
    pub provider: String,
    /// Model name
    pub model: String,
    /// Minimum number of replicas
    pub min_replicas: u32,
    /// Maximum number of replicas
    pub max_replicas: u32,
    /// Target requests per second per replica
    pub target_rps: f64,
    /// Target average latency in milliseconds
    pub target_latency_ms: f64,
    /// Target queue depth
    pub target_queue_depth: u32,
    /// Resource requests
    pub resources: ResourceRequirements,
    /// Auto-scaling configuration
    pub autoscaling: AutoScalingConfig,
}

/// Resource requirements for LLM pods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU request (in cores)
    pub cpu: f64,
    /// Memory request (in GB)
    pub memory: f64,
    /// GPU request (optional)
    pub gpu: Option<u32>,
    /// GPU type (e.g., "nvidia.com/gpu")
    pub gpu_type: Option<String>,
}

/// Auto-scaling configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    /// Enable auto-scaling
    pub enabled: bool,
    /// Scale-up cooldown period in seconds
    pub scale_up_cooldown: u64,
    /// Scale-down cooldown period in seconds
    pub scale_down_cooldown: u64,
    /// Metrics to use for scaling decisions
    pub metrics: Vec<ScalingMetric>,
    /// Scaling behavior
    pub behavior: ScalingBehavior,
}

/// Metrics used for auto-scaling decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScalingMetric {
    /// Requests per second
    RequestsPerSecond { threshold: f64 },
    /// Average latency
    AverageLatency { threshold_ms: f64 },
    /// Queue depth
    QueueDepth { threshold: u32 },
    /// CPU utilization percentage
    CpuUtilization { threshold: f64 },
    /// Memory utilization percentage
    MemoryUtilization { threshold: f64 },
    /// GPU utilization percentage
    GpuUtilization { threshold: f64 },
    /// Custom metric
    Custom { name: String, threshold: f64 },
}

/// Scaling behavior configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingBehavior {
    /// Maximum number of pods to scale up at once
    pub scale_up_max_pods: u32,
    /// Maximum number of pods to scale down at once
    pub scale_down_max_pods: u32,
    /// Scale up percentage (0-100)
    pub scale_up_percentage: u32,
    /// Scale down percentage (0-100)
    pub scale_down_percentage: u32,
}

/// Status of an LLM deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStatus {
    /// Current number of replicas
    pub current_replicas: u32,
    /// Desired number of replicas
    pub desired_replicas: u32,
    /// Ready replicas
    pub ready_replicas: u32,
    /// Current metrics
    pub metrics: DeploymentMetrics,
    /// Last scaling event
    pub last_scale_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Conditions
    pub conditions: Vec<DeploymentCondition>,
}

/// Current metrics for a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMetrics {
    /// Current requests per second
    pub current_rps: f64,
    /// Current average latency
    pub current_latency_ms: f64,
    /// Current queue depth
    pub current_queue_depth: u32,
    /// CPU utilization
    pub cpu_utilization: f64,
    /// Memory utilization
    pub memory_utilization: f64,
    /// GPU utilization (if applicable)
    pub gpu_utilization: Option<f64>,
}

/// Condition of a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCondition {
    /// Type of condition
    pub condition_type: ConditionType,
    /// Status (True, False, Unknown)
    pub status: String,
    /// Last transition time
    pub last_transition_time: chrono::DateTime<chrono::Utc>,
    /// Reason for the condition
    pub reason: String,
    /// Human-readable message
    pub message: String,
}

/// Types of deployment conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    /// Deployment is progressing
    Progressing,
    /// Deployment is available
    Available,
    /// Deployment has failed
    Failed,
    /// Scaling is in progress
    Scaling,
}

/// Kubernetes operator for LLM deployments.
pub struct LLMOperator {
    /// Deployments being managed
    deployments: Arc<RwLock<HashMap<String, LLMDeployment>>>,
    /// Metrics collector
    metrics_collector: Arc<K8sMetricsCollector>,
    /// Scaler
    scaler: Arc<AutoScaler>,
}

impl LLMOperator {
    /// Creates a new LLM operator.
    pub fn new() -> Self {
        Self {
            deployments: Arc::new(RwLock::new(HashMap::new())),
            metrics_collector: Arc::new(K8sMetricsCollector::new()),
            scaler: Arc::new(AutoScaler::new()),
        }
    }

    /// Registers a new deployment.
    pub async fn register_deployment(&self, deployment: LLMDeployment) -> Result<()> {
        let key = format!(
            "{}/{}",
            deployment.metadata.namespace, deployment.metadata.name
        );
        let mut deployments = self.deployments.write().await;
        deployments.insert(key, deployment);
        Ok(())
    }

    /// Unregisters a deployment.
    pub async fn unregister_deployment(&self, namespace: &str, name: &str) -> Result<()> {
        let key = format!("{}/{}", namespace, name);
        let mut deployments = self.deployments.write().await;
        deployments.remove(&key);
        Ok(())
    }

    /// Reconciles a deployment (controller loop).
    pub async fn reconcile(&self, namespace: &str, name: &str) -> Result<()> {
        let key = format!("{}/{}", namespace, name);
        let deployments = self.deployments.read().await;

        let deployment = deployments.get(&key).context("Deployment not found")?;

        if !deployment.spec.autoscaling.enabled {
            return Ok(());
        }

        // Collect current metrics
        let metrics = self
            .metrics_collector
            .collect_metrics(&deployment.metadata)
            .await?;

        // Determine if scaling is needed
        let scaling_decision = self
            .scaler
            .make_scaling_decision(deployment, &metrics)
            .await?;

        if let Some(new_replicas) = scaling_decision {
            self.scale_deployment(deployment, new_replicas).await?;
        }

        Ok(())
    }

    /// Scales a deployment to the specified number of replicas.
    async fn scale_deployment(&self, deployment: &LLMDeployment, new_replicas: u32) -> Result<()> {
        // In a real implementation, this would call the Kubernetes API
        // to update the deployment's replica count
        println!(
            "Scaling deployment {}/{} to {} replicas",
            deployment.metadata.namespace, deployment.metadata.name, new_replicas
        );
        Ok(())
    }

    /// Runs the operator's main control loop.
    pub async fn run(&self) -> Result<()> {
        loop {
            let deployments = self.deployments.read().await;
            let deployment_keys: Vec<String> = deployments.keys().cloned().collect();
            drop(deployments);

            for key in deployment_keys {
                let parts: Vec<&str> = key.split('/').collect();
                if parts.len() == 2 {
                    if let Err(e) = self.reconcile(parts[0], parts[1]).await {
                        eprintln!("Error reconciling {}: {}", key, e);
                    }
                }
            }

            // Sleep before next reconciliation cycle
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}

impl Default for LLMOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Collects metrics from LLM deployments in Kubernetes.
pub struct K8sMetricsCollector {
    /// Cached metrics
    cache: Arc<RwLock<HashMap<String, DeploymentMetrics>>>,
}

impl K8sMetricsCollector {
    /// Creates a new metrics collector.
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Collects metrics for a deployment.
    pub async fn collect_metrics(
        &self,
        metadata: &DeploymentMetadata,
    ) -> Result<DeploymentMetrics> {
        // In a real implementation, this would query Prometheus or the Kubernetes metrics API
        let metrics = DeploymentMetrics {
            current_rps: 100.0,
            current_latency_ms: 150.0,
            current_queue_depth: 10,
            cpu_utilization: 70.0,
            memory_utilization: 60.0,
            gpu_utilization: Some(80.0),
        };

        let key = format!("{}/{}", metadata.namespace, metadata.name);
        let mut cache = self.cache.write().await;
        cache.insert(key, metrics.clone());

        Ok(metrics)
    }
}

impl Default for K8sMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Auto-scaler for LLM deployments.
pub struct AutoScaler {
    /// Last scaling decisions
    last_scale_times: Arc<RwLock<HashMap<String, chrono::DateTime<chrono::Utc>>>>,
}

impl AutoScaler {
    /// Creates a new auto-scaler.
    pub fn new() -> Self {
        Self {
            last_scale_times: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Makes a scaling decision based on current metrics.
    pub async fn make_scaling_decision(
        &self,
        deployment: &LLMDeployment,
        metrics: &DeploymentMetrics,
    ) -> Result<Option<u32>> {
        let current_replicas = deployment
            .status
            .as_ref()
            .map(|s| s.current_replicas)
            .unwrap_or(deployment.spec.min_replicas);

        let key = format!(
            "{}/{}",
            deployment.metadata.namespace, deployment.metadata.name
        );

        // Check cooldown period
        let last_scale_times = self.last_scale_times.read().await;
        if let Some(last_time) = last_scale_times.get(&key) {
            let now = chrono::Utc::now();
            let elapsed = (now - *last_time).num_seconds() as u64;

            if elapsed < deployment.spec.autoscaling.scale_up_cooldown {
                return Ok(None);
            }
        }
        drop(last_scale_times);

        // Evaluate metrics against thresholds
        let mut scale_up_votes = 0;
        let mut scale_down_votes = 0;

        for metric in &deployment.spec.autoscaling.metrics {
            match metric {
                ScalingMetric::RequestsPerSecond { threshold } => {
                    let target_per_replica = threshold / current_replicas as f64;
                    if metrics.current_rps > *threshold {
                        scale_up_votes += 1;
                    } else if metrics.current_rps < target_per_replica * 0.5 {
                        scale_down_votes += 1;
                    }
                }
                ScalingMetric::AverageLatency { threshold_ms } => {
                    if metrics.current_latency_ms > *threshold_ms {
                        scale_up_votes += 1;
                    } else if metrics.current_latency_ms < threshold_ms * 0.5 {
                        scale_down_votes += 1;
                    }
                }
                ScalingMetric::QueueDepth { threshold } => {
                    if metrics.current_queue_depth > *threshold {
                        scale_up_votes += 1;
                    } else if metrics.current_queue_depth < threshold / 2 {
                        scale_down_votes += 1;
                    }
                }
                ScalingMetric::CpuUtilization { threshold } => {
                    if metrics.cpu_utilization > *threshold {
                        scale_up_votes += 1;
                    } else if metrics.cpu_utilization < threshold * 0.5 {
                        scale_down_votes += 1;
                    }
                }
                ScalingMetric::MemoryUtilization { threshold } => {
                    if metrics.memory_utilization > *threshold {
                        scale_up_votes += 1;
                    } else if metrics.memory_utilization < threshold * 0.5 {
                        scale_down_votes += 1;
                    }
                }
                ScalingMetric::GpuUtilization { threshold } => {
                    if let Some(gpu_util) = metrics.gpu_utilization {
                        if gpu_util > *threshold {
                            scale_up_votes += 1;
                        } else if gpu_util < threshold * 0.5 {
                            scale_down_votes += 1;
                        }
                    }
                }
                ScalingMetric::Custom { .. } => {
                    // Custom metrics would be evaluated here
                }
            }
        }

        // Make scaling decision
        let new_replicas = if scale_up_votes > scale_down_votes {
            // Scale up
            let scale_amount = std::cmp::max(
                1,
                (current_replicas * deployment.spec.autoscaling.behavior.scale_up_percentage / 100)
                    .min(deployment.spec.autoscaling.behavior.scale_up_max_pods),
            );
            std::cmp::min(
                current_replicas + scale_amount,
                deployment.spec.max_replicas,
            )
        } else if scale_down_votes > scale_up_votes {
            // Scale down
            let scale_amount = std::cmp::max(
                1,
                (current_replicas * deployment.spec.autoscaling.behavior.scale_down_percentage
                    / 100)
                    .min(deployment.spec.autoscaling.behavior.scale_down_max_pods),
            );
            std::cmp::max(
                current_replicas.saturating_sub(scale_amount),
                deployment.spec.min_replicas,
            )
        } else {
            current_replicas
        };

        if new_replicas != current_replicas {
            let mut last_scale_times = self.last_scale_times.write().await;
            last_scale_times.insert(key, chrono::Utc::now());
            Ok(Some(new_replicas))
        } else {
            Ok(None)
        }
    }
}

impl Default for AutoScaler {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for LLM deployment specifications.
pub struct DeploymentBuilder {
    name: String,
    namespace: String,
    provider: String,
    model: String,
    min_replicas: u32,
    max_replicas: u32,
    target_rps: f64,
    target_latency_ms: f64,
    target_queue_depth: u32,
    resources: ResourceRequirements,
    autoscaling: AutoScalingConfig,
}

impl DeploymentBuilder {
    /// Creates a new deployment builder.
    pub fn new(name: impl Into<String>, namespace: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: namespace.into(),
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            min_replicas: 1,
            max_replicas: 10,
            target_rps: 100.0,
            target_latency_ms: 200.0,
            target_queue_depth: 50,
            resources: ResourceRequirements {
                cpu: 2.0,
                memory: 4.0,
                gpu: None,
                gpu_type: None,
            },
            autoscaling: AutoScalingConfig {
                enabled: true,
                scale_up_cooldown: 60,
                scale_down_cooldown: 300,
                metrics: vec![
                    ScalingMetric::RequestsPerSecond { threshold: 100.0 },
                    ScalingMetric::AverageLatency {
                        threshold_ms: 200.0,
                    },
                ],
                behavior: ScalingBehavior {
                    scale_up_max_pods: 5,
                    scale_down_max_pods: 2,
                    scale_up_percentage: 50,
                    scale_down_percentage: 25,
                },
            },
        }
    }

    /// Sets the provider.
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = provider.into();
        self
    }

    /// Sets the model.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Sets the replica range.
    pub fn replicas(mut self, min: u32, max: u32) -> Self {
        self.min_replicas = min;
        self.max_replicas = max;
        self
    }

    /// Sets the target RPS.
    pub fn target_rps(mut self, rps: f64) -> Self {
        self.target_rps = rps;
        self
    }

    /// Sets resource requirements.
    pub fn resources(mut self, cpu: f64, memory: f64) -> Self {
        self.resources.cpu = cpu;
        self.resources.memory = memory;
        self
    }

    /// Adds GPU requirements.
    pub fn with_gpu(mut self, count: u32, gpu_type: impl Into<String>) -> Self {
        self.resources.gpu = Some(count);
        self.resources.gpu_type = Some(gpu_type.into());
        self
    }

    /// Builds the deployment.
    pub fn build(self) -> LLMDeployment {
        LLMDeployment {
            metadata: DeploymentMetadata {
                name: self.name,
                namespace: self.namespace,
                labels: HashMap::new(),
                annotations: HashMap::new(),
            },
            spec: DeploymentSpec {
                provider: self.provider,
                model: self.model,
                min_replicas: self.min_replicas,
                max_replicas: self.max_replicas,
                target_rps: self.target_rps,
                target_latency_ms: self.target_latency_ms,
                target_queue_depth: self.target_queue_depth,
                resources: self.resources,
                autoscaling: self.autoscaling,
            },
            status: Some(DeploymentStatus {
                current_replicas: self.min_replicas,
                desired_replicas: self.min_replicas,
                ready_replicas: self.min_replicas,
                metrics: DeploymentMetrics {
                    current_rps: 0.0,
                    current_latency_ms: 0.0,
                    current_queue_depth: 0,
                    cpu_utilization: 0.0,
                    memory_utilization: 0.0,
                    gpu_utilization: None,
                },
                last_scale_time: None,
                conditions: vec![],
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_builder() {
        let deployment = DeploymentBuilder::new("llm-api", "default")
            .provider("openai")
            .model("gpt-4")
            .replicas(2, 20)
            .target_rps(500.0)
            .resources(4.0, 8.0)
            .with_gpu(1, "nvidia.com/gpu")
            .build();

        assert_eq!(deployment.metadata.name, "llm-api");
        assert_eq!(deployment.metadata.namespace, "default");
        assert_eq!(deployment.spec.provider, "openai");
        assert_eq!(deployment.spec.model, "gpt-4");
        assert_eq!(deployment.spec.min_replicas, 2);
        assert_eq!(deployment.spec.max_replicas, 20);
        assert_eq!(deployment.spec.resources.gpu, Some(1));
    }

    #[tokio::test]
    async fn test_operator_registration() {
        let operator = LLMOperator::new();
        let deployment = DeploymentBuilder::new("test", "default").build();

        operator.register_deployment(deployment).await.unwrap();

        let deployments = operator.deployments.read().await;
        assert!(deployments.contains_key("default/test"));
    }

    #[tokio::test]
    async fn test_metrics_collector() {
        let collector = K8sMetricsCollector::new();
        let metadata = DeploymentMetadata {
            name: "test".to_string(),
            namespace: "default".to_string(),
            labels: HashMap::new(),
            annotations: HashMap::new(),
        };

        let metrics = collector.collect_metrics(&metadata).await.unwrap();
        assert!(metrics.current_rps >= 0.0);
    }

    #[tokio::test]
    async fn test_scaling_decision_scale_up() {
        let scaler = AutoScaler::new();
        let deployment = DeploymentBuilder::new("test", "default")
            .replicas(2, 10)
            .build();

        // High load metrics should trigger scale up
        let metrics = DeploymentMetrics {
            current_rps: 200.0,        // Above threshold
            current_latency_ms: 300.0, // Above threshold
            current_queue_depth: 100,  // Above threshold
            cpu_utilization: 90.0,
            memory_utilization: 85.0,
            gpu_utilization: Some(95.0),
        };

        let decision = scaler
            .make_scaling_decision(&deployment, &metrics)
            .await
            .unwrap();
        assert!(decision.is_some());
        assert!(decision.unwrap() > 2); // Should scale up from 2 replicas
    }

    #[tokio::test]
    async fn test_scaling_decision_scale_down() {
        let mut deployment = DeploymentBuilder::new("test", "default")
            .replicas(2, 10)
            .build();

        // Set current replicas to 5
        if let Some(status) = deployment.status.as_mut() {
            status.current_replicas = 5;
        }

        let scaler = AutoScaler::new();

        // Low load metrics should trigger scale down
        let metrics = DeploymentMetrics {
            current_rps: 10.0,        // Below threshold
            current_latency_ms: 50.0, // Below threshold
            current_queue_depth: 2,   // Below threshold
            cpu_utilization: 20.0,
            memory_utilization: 30.0,
            gpu_utilization: Some(25.0),
        };

        let decision = scaler
            .make_scaling_decision(&deployment, &metrics)
            .await
            .unwrap();
        assert!(decision.is_some());
        assert!(decision.unwrap() < 5); // Should scale down from 5 replicas
    }
}
