//! GPU scheduling and batching for LLM inference.
//!
//! This module provides GPU resource management and request batching
//! to optimize throughput and latency for LLM inference operations.

use crate::{LLMProvider, TextStream};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};
use tokio::time::sleep;

/// GPU device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    /// Device ID
    pub id: usize,
    /// Device name
    pub name: String,
    /// Total memory in bytes
    pub total_memory: u64,
    /// Available memory in bytes
    pub available_memory: u64,
    /// GPU utilization (0.0 to 1.0)
    pub utilization: f64,
    /// Whether the device is currently available
    pub is_available: bool,
}

impl GpuDevice {
    /// Creates a new GPU device.
    pub fn new(id: usize, name: impl Into<String>, total_memory: u64) -> Self {
        Self {
            id,
            name: name.into(),
            total_memory,
            available_memory: total_memory,
            utilization: 0.0,
            is_available: true,
        }
    }

    /// Returns the memory usage percentage (0.0 to 1.0).
    pub fn memory_usage(&self) -> f64 {
        if self.total_memory == 0 {
            0.0
        } else {
            1.0 - (self.available_memory as f64 / self.total_memory as f64)
        }
    }

    /// Checks if the device has enough available memory.
    pub fn has_memory(&self, required_bytes: u64) -> bool {
        self.available_memory >= required_bytes
    }
}

/// GPU scheduling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulingStrategy {
    /// First-Come-First-Served
    FCFS,
    /// Round-robin across available GPUs
    RoundRobin,
    /// Least loaded GPU (by memory)
    LeastLoaded,
    /// Least loaded GPU (by utilization)
    LeastUtilized,
    /// Bin packing - fit requests into GPUs to minimize fragmentation
    BinPacking,
}

/// GPU batching configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuBatchConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Maximum wait time before processing a partial batch
    pub max_wait_time: Duration,
    /// Whether to dynamically adjust batch size based on GPU memory
    pub dynamic_batching: bool,
    /// Target GPU memory utilization (0.0 to 1.0)
    pub target_memory_utilization: f64,
}

impl Default for GpuBatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 32,
            max_wait_time: Duration::from_millis(100),
            dynamic_batching: true,
            target_memory_utilization: 0.8,
        }
    }
}

/// GPU scheduler for managing inference requests.
pub struct GpuScheduler {
    devices: Arc<Mutex<Vec<GpuDevice>>>,
    strategy: SchedulingStrategy,
    batch_config: GpuBatchConfig,
    current_device: Arc<Mutex<usize>>,
    semaphore: Arc<Semaphore>,
}

impl GpuScheduler {
    /// Creates a new GPU scheduler.
    pub fn new(devices: Vec<GpuDevice>, strategy: SchedulingStrategy) -> Self {
        let max_concurrent = devices.len().max(1);
        Self {
            devices: Arc::new(Mutex::new(devices)),
            strategy,
            batch_config: GpuBatchConfig::default(),
            current_device: Arc::new(Mutex::new(0)),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Sets the batch configuration.
    pub fn with_batch_config(mut self, config: GpuBatchConfig) -> Self {
        self.batch_config = config;
        self
    }

    /// Returns the list of GPU devices.
    pub async fn devices(&self) -> Vec<GpuDevice> {
        self.devices.lock().await.clone()
    }

    /// Selects the best GPU device for a request based on the scheduling strategy.
    pub async fn select_device(&self, estimated_memory: u64) -> Option<usize> {
        let devices = self.devices.lock().await;

        match self.strategy {
            SchedulingStrategy::FCFS => {
                // Find first available device with enough memory
                devices
                    .iter()
                    .position(|d| d.is_available && d.has_memory(estimated_memory))
            }
            SchedulingStrategy::RoundRobin => {
                // Round-robin across available devices
                let mut current = self.current_device.lock().await;
                let start = *current;

                loop {
                    if devices[*current].is_available
                        && devices[*current].has_memory(estimated_memory)
                    {
                        let selected = *current;
                        *current = (*current + 1) % devices.len();
                        return Some(selected);
                    }

                    *current = (*current + 1) % devices.len();
                    if *current == start {
                        // Wrapped around without finding a device
                        return None;
                    }
                }
            }
            SchedulingStrategy::LeastLoaded => {
                // Select device with most available memory
                devices
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d.is_available && d.has_memory(estimated_memory))
                    .max_by_key(|(_, d)| d.available_memory)
                    .map(|(idx, _)| idx)
            }
            SchedulingStrategy::LeastUtilized => {
                // Select device with lowest utilization
                devices
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d.is_available && d.has_memory(estimated_memory))
                    .min_by(|(_, a), (_, b)| {
                        a.utilization
                            .partial_cmp(&b.utilization)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(idx, _)| idx)
            }
            SchedulingStrategy::BinPacking => {
                // Select device with least available memory that can still fit the request
                devices
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d.is_available && d.has_memory(estimated_memory))
                    .min_by_key(|(_, d)| d.available_memory)
                    .map(|(idx, _)| idx)
            }
        }
    }

    /// Acquires a GPU slot for inference.
    pub async fn acquire(&self, estimated_memory: u64) -> Result<GpuSlot> {
        let permit = self.semaphore.clone().acquire_owned().await?;

        let device_id = self.select_device(estimated_memory).await;

        Ok(GpuSlot {
            device_id,
            _permit: permit,
        })
    }

    /// Updates GPU device statistics.
    pub async fn update_device(&self, device_id: usize, available_memory: u64, utilization: f64) {
        let mut devices = self.devices.lock().await;
        if let Some(device) = devices.get_mut(device_id) {
            device.available_memory = available_memory;
            device.utilization = utilization;
        }
    }

    /// Marks a device as available or unavailable.
    pub async fn set_device_availability(&self, device_id: usize, available: bool) {
        let mut devices = self.devices.lock().await;
        if let Some(device) = devices.get_mut(device_id) {
            device.is_available = available;
        }
    }

    /// Returns scheduler statistics.
    pub async fn stats(&self) -> SchedulerStats {
        let devices = self.devices.lock().await;
        let total_devices = devices.len();
        let available_devices = devices.iter().filter(|d| d.is_available).count();
        let total_memory = devices.iter().map(|d| d.total_memory).sum();
        let available_memory = devices.iter().map(|d| d.available_memory).sum();
        let avg_utilization = if devices.is_empty() {
            0.0
        } else {
            devices.iter().map(|d| d.utilization).sum::<f64>() / devices.len() as f64
        };

        SchedulerStats {
            total_devices,
            available_devices,
            total_memory,
            available_memory,
            avg_utilization,
        }
    }
}

/// GPU slot representing an acquired GPU resource.
pub struct GpuSlot {
    /// Device ID (None if no specific device)
    pub device_id: Option<usize>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

/// Scheduler statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerStats {
    /// Total number of devices
    pub total_devices: usize,
    /// Number of available devices
    pub available_devices: usize,
    /// Total memory across all devices
    pub total_memory: u64,
    /// Available memory across all devices
    pub available_memory: u64,
    /// Average GPU utilization
    pub avg_utilization: f64,
}

/// Type alias for queue items to reduce complexity.
type QueueItem<T> = (T, tokio::sync::oneshot::Sender<String>);

/// Batching queue for LLM requests.
pub struct BatchQueue<T> {
    queue: Arc<Mutex<VecDeque<QueueItem<T>>>>,
    config: GpuBatchConfig,
    processing: Arc<Mutex<bool>>,
}

impl<T> BatchQueue<T>
where
    T: Clone + Send + 'static,
{
    /// Creates a new batch queue.
    pub fn new(config: GpuBatchConfig) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            config,
            processing: Arc::new(Mutex::new(false)),
        }
    }

    /// Enqueues a request and returns a receiver for the result.
    pub async fn enqueue(&self, request: T) -> tokio::sync::oneshot::Receiver<String> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let mut queue = self.queue.lock().await;
        queue.push_back((request, tx));
        rx
    }

    /// Processes a batch of requests.
    pub async fn process_batch<F, Fut>(&self, processor: F)
    where
        F: Fn(Vec<T>) -> Fut,
        Fut: std::future::Future<Output = Result<Vec<String>>>,
    {
        let mut is_processing = self.processing.lock().await;
        if *is_processing {
            return;
        }
        *is_processing = true;
        drop(is_processing);

        let start = Instant::now();

        loop {
            // Wait for max_wait_time or until we have a full batch
            sleep(self.config.max_wait_time).await;

            let mut queue = self.queue.lock().await;
            if queue.is_empty() {
                break;
            }

            let batch_size = queue.len().min(self.config.max_batch_size);
            let mut batch_items = Vec::new();
            let mut senders = Vec::new();

            for _ in 0..batch_size {
                if let Some((item, sender)) = queue.pop_front() {
                    batch_items.push(item);
                    senders.push(sender);
                }
            }

            drop(queue);

            // Process the batch
            match processor(batch_items).await {
                Ok(results) => {
                    for (sender, result) in senders.into_iter().zip(results.into_iter()) {
                        let _ = sender.send(result);
                    }
                }
                Err(_) => {
                    // Send error to all senders
                    for sender in senders {
                        let _ = sender.send("Error processing batch".to_string());
                    }
                }
            }

            // Check if we should continue processing
            let queue = self.queue.lock().await;
            if queue.is_empty() || start.elapsed() > Duration::from_secs(60) {
                break;
            }
        }

        let mut is_processing = self.processing.lock().await;
        *is_processing = false;
    }

    /// Returns the current queue size.
    pub async fn size(&self) -> usize {
        self.queue.lock().await.len()
    }
}

/// LLM provider with GPU scheduling.
pub struct GpuScheduledProvider<P> {
    provider: P,
    scheduler: Arc<GpuScheduler>,
    estimated_memory_per_request: u64,
}

impl<P> GpuScheduledProvider<P> {
    /// Creates a new GPU-scheduled provider.
    pub fn new(provider: P, scheduler: Arc<GpuScheduler>) -> Self {
        Self {
            provider,
            scheduler,
            estimated_memory_per_request: 1024 * 1024 * 100, // 100 MB default
        }
    }

    /// Sets the estimated memory per request.
    pub fn with_estimated_memory(mut self, bytes: u64) -> Self {
        self.estimated_memory_per_request = bytes;
        self
    }

    /// Gets a reference to the scheduler.
    pub fn scheduler(&self) -> Arc<GpuScheduler> {
        self.scheduler.clone()
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for GpuScheduledProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let _slot = self
            .scheduler
            .acquire(self.estimated_memory_per_request)
            .await?;
        self.provider.generate_text(prompt).await
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let _slot = self
            .scheduler
            .acquire(self.estimated_memory_per_request)
            .await?;
        self.provider.generate_structured::<T>(prompt).await
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let _slot = self
            .scheduler
            .acquire(self.estimated_memory_per_request)
            .await?;
        self.provider.generate_text_stream(prompt).await
    }

    fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }

    fn model_name(&self) -> &str {
        self.provider.model_name()
    }

    fn supports_streaming(&self) -> bool {
        self.provider.supports_streaming()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_device() {
        let device = GpuDevice::new(0, "NVIDIA A100", 40 * 1024 * 1024 * 1024); // 40 GB

        assert_eq!(device.id, 0);
        assert_eq!(device.name, "NVIDIA A100");
        assert_eq!(device.total_memory, 40 * 1024 * 1024 * 1024);
        assert_eq!(device.memory_usage(), 0.0);
        assert!(device.is_available);
    }

    #[test]
    fn test_gpu_device_memory_usage() {
        let mut device = GpuDevice::new(0, "GPU", 1000);
        device.available_memory = 500;

        assert_eq!(device.memory_usage(), 0.5);
        assert!(device.has_memory(400));
        assert!(!device.has_memory(600));
    }

    #[tokio::test]
    async fn test_scheduler_fcfs() {
        let devices = vec![
            GpuDevice::new(0, "GPU0", 1000),
            GpuDevice::new(1, "GPU1", 2000),
        ];

        let scheduler = GpuScheduler::new(devices, SchedulingStrategy::FCFS);

        let device_id = scheduler.select_device(500).await;
        assert_eq!(device_id, Some(0)); // First device
    }

    #[tokio::test]
    async fn test_scheduler_least_loaded() {
        let mut devices = vec![
            GpuDevice::new(0, "GPU0", 1000),
            GpuDevice::new(1, "GPU1", 2000),
        ];
        devices[0].available_memory = 500;
        devices[1].available_memory = 1500;

        let scheduler = GpuScheduler::new(devices, SchedulingStrategy::LeastLoaded);

        let device_id = scheduler.select_device(500).await;
        assert_eq!(device_id, Some(1)); // Device with more available memory
    }

    #[tokio::test]
    async fn test_scheduler_round_robin() {
        let devices = vec![
            GpuDevice::new(0, "GPU0", 1000),
            GpuDevice::new(1, "GPU1", 1000),
        ];

        let scheduler = GpuScheduler::new(devices, SchedulingStrategy::RoundRobin);

        let device1 = scheduler.select_device(100).await;
        let device2 = scheduler.select_device(100).await;

        assert_ne!(device1, device2);
    }

    #[tokio::test]
    async fn test_scheduler_stats() {
        let devices = vec![
            GpuDevice::new(0, "GPU0", 1000),
            GpuDevice::new(1, "GPU1", 2000),
        ];

        let scheduler = GpuScheduler::new(devices, SchedulingStrategy::FCFS);
        let stats = scheduler.stats().await;

        assert_eq!(stats.total_devices, 2);
        assert_eq!(stats.available_devices, 2);
        assert_eq!(stats.total_memory, 3000);
        assert_eq!(stats.available_memory, 3000);
    }

    #[tokio::test]
    async fn test_batch_queue() {
        let config = GpuBatchConfig {
            max_batch_size: 2,
            max_wait_time: Duration::from_millis(50),
            dynamic_batching: false,
            target_memory_utilization: 0.8,
        };

        let queue = BatchQueue::new(config);

        let _rx1 = queue.enqueue("request1".to_string()).await;
        let _rx2 = queue.enqueue("request2".to_string()).await;

        assert_eq!(queue.size().await, 2);
    }

    #[tokio::test]
    async fn test_scheduler_acquire() {
        let devices = vec![GpuDevice::new(0, "GPU0", 1000)];
        let scheduler = GpuScheduler::new(devices, SchedulingStrategy::FCFS);

        let slot = scheduler.acquire(100).await;
        assert!(slot.is_ok());
    }

    #[tokio::test]
    async fn test_scheduler_update_device() {
        let devices = vec![GpuDevice::new(0, "GPU0", 1000)];
        let scheduler = GpuScheduler::new(devices, SchedulingStrategy::FCFS);

        scheduler.update_device(0, 500, 0.5).await;

        let devices = scheduler.devices().await;
        assert_eq!(devices[0].available_memory, 500);
        assert_eq!(devices[0].utilization, 0.5);
    }
}
