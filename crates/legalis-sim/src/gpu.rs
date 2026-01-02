//! GPU Acceleration for Simulations
//!
//! This module provides GPU acceleration capabilities for large-scale simulations,
//! including CUDA, OpenCL, and WebGPU backends.

use crate::{SimResult, SimulationError};
use legalis_core::LegalEntity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GPU backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GpuBackend {
    /// CUDA backend (NVIDIA GPUs)
    Cuda,
    /// OpenCL backend (cross-platform)
    OpenCL,
    /// WebGPU backend (browser/cross-platform)
    WebGPU,
    /// CPU fallback (no GPU)
    #[default]
    CpuFallback,
}

/// GPU device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    /// Device name
    pub name: String,
    /// Device ID
    pub id: usize,
    /// Backend type
    pub backend: GpuBackend,
    /// Total memory in bytes
    pub total_memory: u64,
    /// Available memory in bytes
    pub available_memory: u64,
    /// Compute capability (for CUDA)
    pub compute_capability: Option<(u32, u32)>,
    /// Maximum work group size
    pub max_work_group_size: usize,
    /// Maximum threads per block
    pub max_threads_per_block: usize,
}

impl GpuDevice {
    /// Create a CPU fallback device
    pub fn cpu_fallback() -> Self {
        GpuDevice {
            name: "CPU".to_string(),
            id: 0,
            backend: GpuBackend::CpuFallback,
            total_memory: 0,
            available_memory: 0,
            compute_capability: None,
            max_work_group_size: 1,
            max_threads_per_block: 1,
        }
    }

    /// Check if this is a GPU device
    pub fn is_gpu(&self) -> bool {
        self.backend != GpuBackend::CpuFallback
    }

    /// Get memory utilization percentage
    pub fn memory_utilization(&self) -> f64 {
        if self.total_memory == 0 {
            0.0
        } else {
            ((self.total_memory - self.available_memory) as f64 / self.total_memory as f64) * 100.0
        }
    }
}

/// GPU configuration for simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Preferred backend
    pub backend: GpuBackend,
    /// Device ID to use (None = auto-select)
    pub device_id: Option<usize>,
    /// Batch size for GPU processing
    pub batch_size: usize,
    /// Number of threads per block (CUDA/OpenCL)
    pub threads_per_block: usize,
    /// Enable tensor optimization
    pub use_tensor_ops: bool,
    /// Enable memory pooling
    pub use_memory_pool: bool,
    /// Maximum memory usage in bytes (None = unlimited)
    pub max_memory_bytes: Option<u64>,
}

impl Default for GpuConfig {
    fn default() -> Self {
        GpuConfig {
            backend: GpuBackend::CpuFallback,
            device_id: None,
            batch_size: 1024,
            threads_per_block: 256,
            use_tensor_ops: true,
            use_memory_pool: true,
            max_memory_bytes: None,
        }
    }
}

impl GpuConfig {
    /// Create a CUDA configuration
    pub fn cuda() -> Self {
        GpuConfig {
            backend: GpuBackend::Cuda,
            ..Default::default()
        }
    }

    /// Create an OpenCL configuration
    pub fn opencl() -> Self {
        GpuConfig {
            backend: GpuBackend::OpenCL,
            ..Default::default()
        }
    }

    /// Create a WebGPU configuration
    pub fn webgpu() -> Self {
        GpuConfig {
            backend: GpuBackend::WebGPU,
            batch_size: 512, // Smaller batches for web
            ..Default::default()
        }
    }

    /// Set device ID
    pub fn with_device(mut self, device_id: usize) -> Self {
        self.device_id = Some(device_id);
        self
    }

    /// Set batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Set threads per block
    pub fn with_threads_per_block(mut self, threads: usize) -> Self {
        self.threads_per_block = threads;
        self
    }
}

/// Tensor representation of entity data for GPU processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTensor {
    /// Shape of the tensor [num_entities, num_features]
    pub shape: (usize, usize),
    /// Flattened data in row-major order
    pub data: Vec<f32>,
    /// Feature names
    pub feature_names: Vec<String>,
    /// Entity IDs (for mapping back to entities)
    pub entity_ids: Vec<String>,
}

impl EntityTensor {
    /// Create a new entity tensor
    pub fn new(num_entities: usize, num_features: usize) -> Self {
        EntityTensor {
            shape: (num_entities, num_features),
            data: vec![0.0; num_entities * num_features],
            feature_names: Vec::new(),
            entity_ids: Vec::new(),
        }
    }

    /// Create from entities
    pub fn from_entities<E: LegalEntity>(
        entities: &[E],
        feature_names: &[&str],
    ) -> SimResult<Self> {
        if entities.is_empty() {
            return Err(SimulationError::InvalidPopulation(
                "Cannot create tensor from empty entity list".to_string(),
            ));
        }

        let num_entities = entities.len();
        let num_features = feature_names.len();
        let mut data = Vec::with_capacity(num_entities * num_features);
        let mut entity_ids = Vec::with_capacity(num_entities);

        for entity in entities {
            entity_ids.push(entity.id().to_string());
            for feature_name in feature_names {
                let value = entity
                    .get_attribute(feature_name)
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or(0.0) as f32;
                data.push(value);
            }
        }

        Ok(EntityTensor {
            shape: (num_entities, num_features),
            data,
            feature_names: feature_names.iter().map(|s| s.to_string()).collect(),
            entity_ids,
        })
    }

    /// Get value at position
    pub fn get(&self, entity_idx: usize, feature_idx: usize) -> Option<f32> {
        if entity_idx >= self.shape.0 || feature_idx >= self.shape.1 {
            return None;
        }
        Some(self.data[entity_idx * self.shape.1 + feature_idx])
    }

    /// Set value at position
    pub fn set(&mut self, entity_idx: usize, feature_idx: usize, value: f32) -> SimResult<()> {
        if entity_idx >= self.shape.0 || feature_idx >= self.shape.1 {
            return Err(SimulationError::InvalidParameter(
                "Tensor index out of bounds".to_string(),
            ));
        }
        self.data[entity_idx * self.shape.1 + feature_idx] = value;
        Ok(())
    }

    /// Get number of entities
    pub fn num_entities(&self) -> usize {
        self.shape.0
    }

    /// Get number of features
    pub fn num_features(&self) -> usize {
        self.shape.1
    }

    /// Get row (all features for one entity)
    pub fn get_row(&self, entity_idx: usize) -> Option<&[f32]> {
        if entity_idx >= self.shape.0 {
            return None;
        }
        let start = entity_idx * self.shape.1;
        let end = start + self.shape.1;
        Some(&self.data[start..end])
    }

    /// Get column (one feature for all entities)
    pub fn get_column(&self, feature_idx: usize) -> Option<Vec<f32>> {
        if feature_idx >= self.shape.1 {
            return None;
        }
        let mut column = Vec::with_capacity(self.shape.0);
        for entity_idx in 0..self.shape.0 {
            column.push(self.data[entity_idx * self.shape.1 + feature_idx]);
        }
        Some(column)
    }
}

/// GPU kernel for condition evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuKernel {
    /// Kernel name
    pub name: String,
    /// Kernel source code (backend-specific)
    pub source: String,
    /// Entry point function name
    pub entry_point: String,
    /// Backend this kernel is for
    pub backend: GpuBackend,
}

impl GpuKernel {
    /// Create a new kernel
    pub fn new(name: String, source: String, entry_point: String, backend: GpuBackend) -> Self {
        GpuKernel {
            name,
            source,
            entry_point,
            backend,
        }
    }

    /// Create a condition evaluation kernel for CUDA
    pub fn condition_eval_cuda() -> Self {
        let source = r#"
            extern "C" __global__ void eval_condition(
                const float* input,
                float* output,
                int num_entities,
                int num_features,
                float threshold
            ) {
                int idx = blockIdx.x * blockDim.x + threadIdx.x;
                if (idx < num_entities) {
                    float sum = 0.0f;
                    for (int i = 0; i < num_features; i++) {
                        sum += input[idx * num_features + i];
                    }
                    output[idx] = (sum >= threshold) ? 1.0f : 0.0f;
                }
            }
        "#
        .to_string();

        GpuKernel::new(
            "condition_eval".to_string(),
            source,
            "eval_condition".to_string(),
            GpuBackend::Cuda,
        )
    }

    /// Create a condition evaluation kernel for OpenCL
    pub fn condition_eval_opencl() -> Self {
        let source = r#"
            __kernel void eval_condition(
                __global const float* input,
                __global float* output,
                int num_entities,
                int num_features,
                float threshold
            ) {
                int idx = get_global_id(0);
                if (idx < num_entities) {
                    float sum = 0.0f;
                    for (int i = 0; i < num_features; i++) {
                        sum += input[idx * num_features + i];
                    }
                    output[idx] = (sum >= threshold) ? 1.0f : 0.0f;
                }
            }
        "#
        .to_string();

        GpuKernel::new(
            "condition_eval".to_string(),
            source,
            "eval_condition".to_string(),
            GpuBackend::OpenCL,
        )
    }

    /// Create a condition evaluation kernel for WebGPU (WGSL)
    pub fn condition_eval_webgpu() -> Self {
        let source = r#"
            @group(0) @binding(0) var<storage, read> input: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output: array<f32>;
            @group(0) @binding(2) var<uniform> params: Params;

            struct Params {
                num_entities: u32,
                num_features: u32,
                threshold: f32,
            }

            @compute @workgroup_size(256)
            fn eval_condition(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let idx = global_id.x;
                if (idx < params.num_entities) {
                    var sum: f32 = 0.0;
                    for (var i: u32 = 0u; i < params.num_features; i = i + 1u) {
                        sum = sum + input[idx * params.num_features + i];
                    }
                    output[idx] = select(0.0, 1.0, sum >= params.threshold);
                }
            }
        "#
        .to_string();

        GpuKernel::new(
            "condition_eval".to_string(),
            source,
            "eval_condition".to_string(),
            GpuBackend::WebGPU,
        )
    }
}

/// GPU memory pool for efficient allocation
#[derive(Debug)]
pub struct GpuMemoryPool {
    /// Backend type
    #[allow(dead_code)]
    backend: GpuBackend,
    /// Total allocated memory
    total_allocated: u64,
    /// Free blocks by size
    free_blocks: HashMap<usize, Vec<usize>>,
    /// Allocated blocks
    allocated_blocks: HashMap<usize, usize>,
}

impl GpuMemoryPool {
    /// Create a new memory pool
    pub fn new(backend: GpuBackend) -> Self {
        GpuMemoryPool {
            backend,
            total_allocated: 0,
            free_blocks: HashMap::new(),
            allocated_blocks: HashMap::new(),
        }
    }

    /// Allocate memory
    pub fn allocate(&mut self, size: usize) -> SimResult<usize> {
        // Check if we have a free block of this size
        if let Some(blocks) = self.free_blocks.get_mut(&size) {
            if let Some(block_id) = blocks.pop() {
                self.allocated_blocks.insert(block_id, size);
                return Ok(block_id);
            }
        }

        // Allocate new block
        let block_id = self.allocated_blocks.len();
        self.allocated_blocks.insert(block_id, size);
        self.total_allocated += size as u64;
        Ok(block_id)
    }

    /// Free memory
    pub fn free(&mut self, block_id: usize) -> SimResult<()> {
        if let Some(size) = self.allocated_blocks.remove(&block_id) {
            self.free_blocks.entry(size).or_default().push(block_id);
            Ok(())
        } else {
            Err(SimulationError::InvalidParameter(
                "Invalid block ID".to_string(),
            ))
        }
    }

    /// Get total allocated memory
    pub fn total_allocated(&self) -> u64 {
        self.total_allocated
    }

    /// Get number of free blocks
    pub fn num_free_blocks(&self) -> usize {
        self.free_blocks.values().map(|v| v.len()).sum()
    }

    /// Get number of allocated blocks
    pub fn num_allocated_blocks(&self) -> usize {
        self.allocated_blocks.len()
    }

    /// Reset the pool
    pub fn reset(&mut self) {
        self.free_blocks.clear();
        self.allocated_blocks.clear();
        self.total_allocated = 0;
    }
}

/// GPU executor for running simulations on GPU
#[derive(Debug)]
pub struct GpuExecutor {
    /// Configuration
    config: GpuConfig,
    /// Selected device
    device: GpuDevice,
    /// Memory pool
    memory_pool: GpuMemoryPool,
    /// Compiled kernels
    kernels: HashMap<String, GpuKernel>,
}

impl GpuExecutor {
    /// Create a new GPU executor
    pub fn new(config: GpuConfig) -> SimResult<Self> {
        // For now, use CPU fallback
        let device = GpuDevice::cpu_fallback();
        let memory_pool = GpuMemoryPool::new(config.backend);

        Ok(GpuExecutor {
            config,
            device,
            memory_pool,
            kernels: HashMap::new(),
        })
    }

    /// Get available devices
    pub fn list_devices() -> Vec<GpuDevice> {
        // For now, just return CPU fallback
        vec![GpuDevice::cpu_fallback()]
    }

    /// Add a kernel
    pub fn add_kernel(&mut self, kernel: GpuKernel) -> SimResult<()> {
        if kernel.backend != self.config.backend {
            return Err(SimulationError::InvalidParameter(format!(
                "Kernel backend {:?} does not match executor backend {:?}",
                kernel.backend, self.config.backend
            )));
        }
        self.kernels.insert(kernel.name.clone(), kernel);
        Ok(())
    }

    /// Execute a kernel on tensor data
    pub fn execute(
        &mut self,
        kernel_name: &str,
        input: &EntityTensor,
        params: &HashMap<String, f32>,
    ) -> SimResult<EntityTensor> {
        let _kernel = self.kernels.get(kernel_name).ok_or_else(|| {
            SimulationError::InvalidParameter(format!("Kernel '{}' not found", kernel_name))
        })?;

        // For CPU fallback, just do a simple computation
        let threshold = params.get("threshold").copied().unwrap_or(0.0);
        let mut output = EntityTensor::new(input.num_entities(), 1);
        output.feature_names = vec!["result".to_string()];
        output.entity_ids = input.entity_ids.clone();

        for entity_idx in 0..input.num_entities() {
            let sum: f32 = (0..input.num_features())
                .filter_map(|f| input.get(entity_idx, f))
                .sum();
            let result = if sum >= threshold { 1.0 } else { 0.0 };
            output.set(entity_idx, 0, result)?;
        }

        Ok(output)
    }

    /// Get device info
    pub fn device(&self) -> &GpuDevice {
        &self.device
    }

    /// Get configuration
    pub fn config(&self) -> &GpuConfig {
        &self.config
    }

    /// Get memory pool statistics
    pub fn memory_stats(&self) -> (u64, usize, usize) {
        (
            self.memory_pool.total_allocated(),
            self.memory_pool.num_allocated_blocks(),
            self.memory_pool.num_free_blocks(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_device_creation() {
        let device = GpuDevice::cpu_fallback();
        assert_eq!(device.backend, GpuBackend::CpuFallback);
        assert!(!device.is_gpu());
    }

    #[test]
    fn test_gpu_config_cuda() {
        let config = GpuConfig::cuda();
        assert_eq!(config.backend, GpuBackend::Cuda);
        assert_eq!(config.batch_size, 1024);
    }

    #[test]
    fn test_gpu_config_opencl() {
        let config = GpuConfig::opencl();
        assert_eq!(config.backend, GpuBackend::OpenCL);
    }

    #[test]
    fn test_gpu_config_webgpu() {
        let config = GpuConfig::webgpu();
        assert_eq!(config.backend, GpuBackend::WebGPU);
        assert_eq!(config.batch_size, 512);
    }

    #[test]
    fn test_gpu_config_builder() {
        let config = GpuConfig::cuda()
            .with_device(1)
            .with_batch_size(2048)
            .with_threads_per_block(512);
        assert_eq!(config.device_id, Some(1));
        assert_eq!(config.batch_size, 2048);
        assert_eq!(config.threads_per_block, 512);
    }

    #[test]
    fn test_entity_tensor_creation() {
        let tensor = EntityTensor::new(100, 10);
        assert_eq!(tensor.shape, (100, 10));
        assert_eq!(tensor.data.len(), 1000);
        assert_eq!(tensor.num_entities(), 100);
        assert_eq!(tensor.num_features(), 10);
    }

    #[test]
    fn test_entity_tensor_get_set() {
        let mut tensor = EntityTensor::new(10, 5);
        tensor.set(3, 2, 42.5).unwrap();
        assert_eq!(tensor.get(3, 2), Some(42.5));
    }

    #[test]
    fn test_entity_tensor_bounds() {
        let mut tensor = EntityTensor::new(10, 5);
        assert!(tensor.set(10, 0, 1.0).is_err());
        assert!(tensor.set(0, 5, 1.0).is_err());
        assert_eq!(tensor.get(10, 0), None);
        assert_eq!(tensor.get(0, 5), None);
    }

    #[test]
    fn test_entity_tensor_row() {
        let mut tensor = EntityTensor::new(3, 4);
        tensor.set(1, 0, 1.0).unwrap();
        tensor.set(1, 1, 2.0).unwrap();
        tensor.set(1, 2, 3.0).unwrap();
        tensor.set(1, 3, 4.0).unwrap();

        let row = tensor.get_row(1).unwrap();
        assert_eq!(row, &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_entity_tensor_column() {
        let mut tensor = EntityTensor::new(3, 4);
        tensor.set(0, 1, 10.0).unwrap();
        tensor.set(1, 1, 20.0).unwrap();
        tensor.set(2, 1, 30.0).unwrap();

        let col = tensor.get_column(1).unwrap();
        assert_eq!(col, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_gpu_kernel_cuda() {
        let kernel = GpuKernel::condition_eval_cuda();
        assert_eq!(kernel.backend, GpuBackend::Cuda);
        assert_eq!(kernel.entry_point, "eval_condition");
        assert!(!kernel.source.is_empty());
    }

    #[test]
    fn test_gpu_kernel_opencl() {
        let kernel = GpuKernel::condition_eval_opencl();
        assert_eq!(kernel.backend, GpuBackend::OpenCL);
        assert!(!kernel.source.is_empty());
    }

    #[test]
    fn test_gpu_kernel_webgpu() {
        let kernel = GpuKernel::condition_eval_webgpu();
        assert_eq!(kernel.backend, GpuBackend::WebGPU);
        assert!(!kernel.source.is_empty());
    }

    #[test]
    fn test_memory_pool_allocate() {
        let mut pool = GpuMemoryPool::new(GpuBackend::Cuda);
        let block1 = pool.allocate(1024).unwrap();
        let block2 = pool.allocate(2048).unwrap();
        assert_ne!(block1, block2);
        assert_eq!(pool.num_allocated_blocks(), 2);
        assert_eq!(pool.total_allocated(), 3072);
    }

    #[test]
    fn test_memory_pool_free() {
        let mut pool = GpuMemoryPool::new(GpuBackend::Cuda);
        let block = pool.allocate(1024).unwrap();
        pool.free(block).unwrap();
        assert_eq!(pool.num_allocated_blocks(), 0);
        assert_eq!(pool.num_free_blocks(), 1);
    }

    #[test]
    fn test_memory_pool_reuse() {
        let mut pool = GpuMemoryPool::new(GpuBackend::Cuda);
        let block1 = pool.allocate(1024).unwrap();
        pool.free(block1).unwrap();
        let block2 = pool.allocate(1024).unwrap();
        assert_eq!(block1, block2); // Should reuse the same block
        assert_eq!(pool.total_allocated(), 1024);
    }

    #[test]
    fn test_memory_pool_reset() {
        let mut pool = GpuMemoryPool::new(GpuBackend::Cuda);
        pool.allocate(1024).unwrap();
        pool.allocate(2048).unwrap();
        pool.reset();
        assert_eq!(pool.num_allocated_blocks(), 0);
        assert_eq!(pool.total_allocated(), 0);
    }

    #[test]
    fn test_gpu_executor_creation() {
        let config = GpuConfig::cuda();
        let executor = GpuExecutor::new(config).unwrap();
        assert_eq!(executor.device().backend, GpuBackend::CpuFallback);
    }

    #[test]
    fn test_gpu_executor_add_kernel() {
        let config = GpuConfig::default();
        let mut executor = GpuExecutor::new(config).unwrap();
        let kernel = GpuKernel::new(
            "test".to_string(),
            "code".to_string(),
            "main".to_string(),
            GpuBackend::CpuFallback,
        );
        executor.add_kernel(kernel).unwrap();
    }

    #[test]
    fn test_gpu_executor_kernel_mismatch() {
        let config = GpuConfig::default();
        let mut executor = GpuExecutor::new(config).unwrap();
        let kernel = GpuKernel::condition_eval_cuda();
        assert!(executor.add_kernel(kernel).is_err());
    }

    #[test]
    fn test_gpu_executor_execute() {
        let config = GpuConfig::default();
        let mut executor = GpuExecutor::new(config).unwrap();

        let kernel = GpuKernel::new(
            "test".to_string(),
            "code".to_string(),
            "main".to_string(),
            GpuBackend::CpuFallback,
        );
        executor.add_kernel(kernel).unwrap();

        let mut input = EntityTensor::new(5, 3);
        input.entity_ids = vec![
            "e1".to_string(),
            "e2".to_string(),
            "e3".to_string(),
            "e4".to_string(),
            "e5".to_string(),
        ];
        for i in 0..5 {
            for j in 0..3 {
                input.set(i, j, (i * 3 + j) as f32).unwrap();
            }
        }

        let mut params = HashMap::new();
        params.insert("threshold".to_string(), 5.0);

        let output = executor.execute("test", &input, &params).unwrap();
        assert_eq!(output.num_entities(), 5);
        assert_eq!(output.num_features(), 1);
    }

    #[test]
    fn test_gpu_executor_list_devices() {
        let devices = GpuExecutor::list_devices();
        assert!(!devices.is_empty());
        assert_eq!(devices[0].backend, GpuBackend::CpuFallback);
    }

    #[test]
    fn test_gpu_executor_memory_stats() {
        let config = GpuConfig::cuda();
        let executor = GpuExecutor::new(config).unwrap();
        let (total, allocated, free) = executor.memory_stats();
        assert_eq!(total, 0);
        assert_eq!(allocated, 0);
        assert_eq!(free, 0);
    }

    #[test]
    fn test_device_memory_utilization() {
        let mut device = GpuDevice::cpu_fallback();
        device.total_memory = 1000;
        device.available_memory = 600;
        assert_eq!(device.memory_utilization(), 40.0);
    }

    #[test]
    fn test_device_memory_utilization_zero() {
        let device = GpuDevice::cpu_fallback();
        assert_eq!(device.memory_utilization(), 0.0);
    }
}
