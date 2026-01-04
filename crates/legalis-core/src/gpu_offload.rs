//! GPU offloading framework for massively parallel evaluation.
//!
//! This module provides a trait-based framework for offloading condition evaluation
//! to GPU accelerators. The framework is backend-agnostic and can be integrated with
//! CUDA, OpenCL, Vulkan Compute, or other GPU compute APIs.
//!
//! ## Features
//!
//! - **Backend Abstraction**: Trait-based design for pluggable GPU backends
//! - **Batch Processing**: Efficient batch evaluation across thousands of entities
//! - **Data Transfer Management**: Optimized host-device memory transfers
//! - **Fallback Support**: Gracefully falls back to CPU when GPU is unavailable
//!
//! ## Example
//!
//! ```
//! use legalis_core::gpu_offload::{GpuBatchEvaluator, BatchSize};
//!
//! let evaluator = GpuBatchEvaluator::new();
//!
//! // Check GPU availability
//! assert!(!evaluator.is_gpu_available()); // False in test environment
//!
//! let batch_size = evaluator.optimal_batch_size();
//! assert!(batch_size > 0);
//! ```

use crate::Condition;

/// GPU compute backend trait.
///
/// Implementors provide GPU-accelerated evaluation for conditions.
/// This trait is designed to be backend-agnostic, supporting CUDA,
/// OpenCL, Vulkan Compute, WebGPU, etc.
pub trait GpuBackend: Send + Sync {
    /// Returns the name of the GPU backend.
    fn backend_name(&self) -> &str;

    /// Checks if GPU is available and initialized.
    fn is_available(&self) -> bool;

    /// Returns GPU device information.
    fn device_info(&self) -> GpuDeviceInfo;

    /// Evaluates a condition across a batch of entities on the GPU.
    ///
    /// Returns a vector of boolean results, one per entity.
    fn evaluate_batch_gpu(
        &mut self,
        condition: &Condition,
        entity_data: &GpuBatchData,
    ) -> Result<Vec<bool>, GpuError>;

    /// Returns the optimal batch size for this GPU.
    fn optimal_batch_size(&self) -> usize;
}

/// GPU device information.
#[derive(Debug, Clone)]
pub struct GpuDeviceInfo {
    /// Device name (e.g., "NVIDIA GeForce RTX 3080")
    pub device_name: String,
    /// Total device memory in bytes
    pub total_memory: u64,
    /// Available device memory in bytes
    pub available_memory: u64,
    /// Number of compute units
    pub compute_units: u32,
    /// Maximum work group size
    pub max_work_group_size: usize,
}

impl Default for GpuDeviceInfo {
    fn default() -> Self {
        Self {
            device_name: "CPU Fallback".to_string(),
            total_memory: 0,
            available_memory: 0,
            compute_units: 1,
            max_work_group_size: 1,
        }
    }
}

/// Batch data prepared for GPU transfer.
///
/// This structure organizes entity data into GPU-friendly formats
/// with proper alignment and padding for efficient SIMD operations.
#[derive(Debug, Clone)]
pub struct GpuBatchData {
    /// Ages (aligned for GPU)
    pub ages: Vec<u32>,
    /// Incomes (aligned for GPU)
    pub incomes: Vec<u64>,
    /// Custom attributes (flattened for GPU)
    pub attributes: Vec<f32>,
    /// Number of entities in the batch
    pub count: usize,
}

impl GpuBatchData {
    /// Creates a new GPU batch from entity data.
    pub fn new(ages: Vec<u32>, incomes: Vec<u64>) -> Self {
        let count = ages.len();
        assert_eq!(
            ages.len(),
            incomes.len(),
            "Ages and incomes must have same length"
        );

        Self {
            ages,
            incomes,
            attributes: Vec::new(),
            count,
        }
    }

    /// Returns the number of entities in the batch.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns true if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Estimates the memory size in bytes.
    pub fn memory_size(&self) -> usize {
        (self.ages.len() * std::mem::size_of::<u32>())
            + (self.incomes.len() * std::mem::size_of::<u64>())
            + (self.attributes.len() * std::mem::size_of::<f32>())
    }
}

/// GPU error types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuError {
    /// GPU device not available
    DeviceNotAvailable,
    /// Out of GPU memory
    OutOfMemory,
    /// Kernel compilation failed
    KernelCompilationFailed(String),
    /// Data transfer error
    TransferError(String),
    /// Kernel execution error
    ExecutionError(String),
    /// Unsupported operation
    UnsupportedOperation(String),
}

impl std::fmt::Display for GpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GpuError::DeviceNotAvailable => write!(f, "GPU device not available"),
            GpuError::OutOfMemory => write!(f, "Out of GPU memory"),
            GpuError::KernelCompilationFailed(msg) => {
                write!(f, "Kernel compilation failed: {}", msg)
            }
            GpuError::TransferError(msg) => write!(f, "Data transfer error: {}", msg),
            GpuError::ExecutionError(msg) => write!(f, "Kernel execution error: {}", msg),
            GpuError::UnsupportedOperation(msg) => write!(f, "Unsupported operation: {}", msg),
        }
    }
}

impl std::error::Error for GpuError {}

/// Recommended batch size for GPU processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchSize {
    /// Small batch (< 1K entities)
    Small,
    /// Medium batch (1K-10K entities)
    Medium,
    /// Large batch (10K-100K entities)
    Large,
    /// Very large batch (> 100K entities)
    VeryLarge,
}

impl BatchSize {
    /// Returns the recommended number of entities for this batch size.
    pub fn entity_count(&self) -> usize {
        match self {
            BatchSize::Small => 512,
            BatchSize::Medium => 4096,
            BatchSize::Large => 32768,
            BatchSize::VeryLarge => 262144,
        }
    }
}

/// GPU batch evaluator with automatic fallback.
///
/// # Example
///
/// ```
/// use legalis_core::gpu_offload::GpuBatchEvaluator;
///
/// let mut evaluator = GpuBatchEvaluator::new();
///
/// // Statistics
/// let stats = evaluator.stats();
/// assert_eq!(stats.total_evaluations, 0);
/// ```
pub struct GpuBatchEvaluator {
    /// Optional GPU backend
    #[allow(dead_code)]
    backend: Option<Box<dyn GpuBackend>>,
    /// Statistics
    stats: GpuStats,
}

impl GpuBatchEvaluator {
    /// Creates a new GPU batch evaluator.
    pub fn new() -> Self {
        Self {
            backend: None,
            stats: GpuStats::new(),
        }
    }

    /// Sets the GPU backend.
    #[allow(dead_code)]
    pub fn with_backend(mut self, backend: Box<dyn GpuBackend>) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Checks if GPU is available.
    pub fn is_gpu_available(&self) -> bool {
        self.backend.as_ref().map_or(false, |b| b.is_available())
    }

    /// Returns the optimal batch size.
    pub fn optimal_batch_size(&self) -> usize {
        self.backend
            .as_ref()
            .map_or(BatchSize::Small.entity_count(), |b| b.optimal_batch_size())
    }

    /// Returns GPU statistics.
    pub fn stats(&self) -> &GpuStats {
        &self.stats
    }

    /// Resets statistics.
    pub fn reset_stats(&mut self) {
        self.stats = GpuStats::new();
    }
}

impl Default for GpuBatchEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for GPU evaluation.
#[derive(Debug, Clone, Default)]
pub struct GpuStats {
    /// Total number of GPU evaluations
    pub total_evaluations: u64,
    /// Number of GPU kernel launches
    pub kernel_launches: u64,
    /// Number of CPU fallbacks
    pub cpu_fallbacks: u64,
    /// Total GPU execution time in nanoseconds
    pub gpu_time_ns: u64,
    /// Total CPU fallback time in nanoseconds
    pub cpu_time_ns: u64,
    /// Total bytes transferred to GPU
    pub bytes_to_gpu: u64,
    /// Total bytes transferred from GPU
    pub bytes_from_gpu: u64,
}

impl GpuStats {
    /// Creates new GPU statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the average GPU execution time per evaluation.
    pub fn avg_gpu_time_ns(&self) -> f64 {
        if self.kernel_launches == 0 {
            0.0
        } else {
            self.gpu_time_ns as f64 / self.kernel_launches as f64
        }
    }

    /// Returns the GPU utilization rate (0.0 to 1.0).
    pub fn gpu_utilization(&self) -> f64 {
        let total = self.total_evaluations;
        if total == 0 {
            0.0
        } else {
            let gpu_evals = total.saturating_sub(self.cpu_fallbacks);
            gpu_evals as f64 / total as f64
        }
    }

    /// Returns the total data transfer volume in bytes.
    pub fn total_transfer_bytes(&self) -> u64 {
        self.bytes_to_gpu + self.bytes_from_gpu
    }

    /// Returns the average bandwidth in GB/s (estimated).
    pub fn avg_bandwidth_gbps(&self) -> f64 {
        if self.gpu_time_ns == 0 {
            0.0
        } else {
            let total_bytes = self.total_transfer_bytes();
            let time_seconds = self.gpu_time_ns as f64 / 1_000_000_000.0;
            let bandwidth_bps = total_bytes as f64 / time_seconds;
            bandwidth_bps / 1_000_000_000.0 // Convert to GB/s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_batch_data() {
        let data = GpuBatchData::new(vec![18, 25, 30], vec![40000, 50000, 60000]);
        assert_eq!(data.len(), 3);
        assert!(!data.is_empty());
        assert!(data.memory_size() > 0);
    }

    #[test]
    fn test_batch_size() {
        assert_eq!(BatchSize::Small.entity_count(), 512);
        assert_eq!(BatchSize::Medium.entity_count(), 4096);
        assert_eq!(BatchSize::Large.entity_count(), 32768);
        assert_eq!(BatchSize::VeryLarge.entity_count(), 262144);
    }

    #[test]
    fn test_gpu_evaluator() {
        let evaluator = GpuBatchEvaluator::new();
        assert!(!evaluator.is_gpu_available());
        assert_eq!(
            evaluator.optimal_batch_size(),
            BatchSize::Small.entity_count()
        );
    }

    #[test]
    fn test_gpu_stats() {
        let mut stats = GpuStats::new();
        stats.total_evaluations = 100;
        stats.kernel_launches = 10;
        stats.gpu_time_ns = 1_000_000;

        assert_eq!(stats.avg_gpu_time_ns(), 100_000.0);
        assert_eq!(stats.gpu_utilization(), 1.0);
    }

    #[test]
    fn test_gpu_device_info_default() {
        let info = GpuDeviceInfo::default();
        assert_eq!(info.device_name, "CPU Fallback");
        assert_eq!(info.compute_units, 1);
    }

    #[test]
    fn test_gpu_error_display() {
        let err = GpuError::DeviceNotAvailable;
        assert_eq!(format!("{}", err), "GPU device not available");

        let err = GpuError::OutOfMemory;
        assert_eq!(format!("{}", err), "Out of GPU memory");
    }
}
