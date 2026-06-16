//! GPU Acceleration for Simulations
//!
//! This module provides GPU acceleration capabilities for large-scale simulations,
//! including CUDA, OpenCL, and WebGPU backends.

use crate::{SimResult, SimulationError};
use legalis_core::LegalEntity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Real NVIDIA CUDA backend (driver API + NVRTC runtime kernel compilation).
//
// Design note: legalis is intentionally free of the SciRS2 stack (zero `scirs2`
// dependencies across the workspace), so — per the COOLJAPAN SciRS2 policy escape
// hatch for projects without a SciRS2 policy — the GPU path binds `cudarc`
// directly rather than routing through `scirs2-core::gpu`. `cudarc` is the same
// pure-Rust CUDA binding the rest of the ecosystem (scirs2, optirs, torsh) builds
// on. The whole backend is gated behind the optional `cuda` feature; with the
// feature off the module is byte-for-byte the previous CPU model.
#[cfg(feature = "cuda")]
use cudarc::driver::{
    CudaContext, CudaFunction, CudaModule, CudaStream, LaunchConfig, PushKernelArg,
};
#[cfg(feature = "cuda")]
use cudarc::nvrtc::compile_ptx;
#[cfg(feature = "cuda")]
use std::sync::Arc;

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
        if let Some(blocks) = self.free_blocks.get_mut(&size)
            && let Some(block_id) = blocks.pop()
        {
            self.allocated_blocks.insert(block_id, size);
            return Ok(block_id);
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
    /// Active real CUDA backend, when the `cuda` feature is enabled and a device
    /// was initialised. `None` means the CPU model is used for all operations.
    #[cfg(feature = "cuda")]
    cuda: Option<CudaState>,
}

impl GpuExecutor {
    /// Create a new GPU executor.
    ///
    /// With the `cuda` feature enabled and `config.backend == GpuBackend::Cuda`,
    /// this initialises a real CUDA context on device 0 and NVRTC-compiles the
    /// condition-evaluation kernels. If no CUDA device is available at runtime (or
    /// the feature is disabled) it transparently falls back to the CPU model, so
    /// the call never fails for lack of a GPU.
    pub fn new(config: GpuConfig) -> SimResult<Self> {
        #[cfg(feature = "cuda")]
        {
            if config.backend == GpuBackend::Cuda
                && let Some(state) = CudaState::try_new()
            {
                let device = state.device.clone();
                let memory_pool = GpuMemoryPool::new(config.backend);
                return Ok(GpuExecutor {
                    config,
                    device,
                    memory_pool,
                    kernels: HashMap::new(),
                    cuda: Some(state),
                });
            }
        }

        let device = GpuDevice::cpu_fallback();
        let memory_pool = GpuMemoryPool::new(config.backend);
        Ok(GpuExecutor {
            config,
            device,
            memory_pool,
            kernels: HashMap::new(),
            #[cfg(feature = "cuda")]
            cuda: None,
        })
    }

    /// Get available devices.
    ///
    /// With the `cuda` feature and a working driver this enumerates the real CUDA
    /// devices; otherwise it returns the single CPU fallback device.
    pub fn list_devices() -> Vec<GpuDevice> {
        #[cfg(feature = "cuda")]
        {
            let devices = CudaState::list_cuda_devices();
            if !devices.is_empty() {
                return devices;
            }
        }
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

        let threshold = params.get("threshold").copied().unwrap_or(0.0);
        let mut output = EntityTensor::new(input.num_entities(), 1);
        output.feature_names = vec!["result".to_string()];
        output.entity_ids = input.entity_ids.clone();

        // Real GPU path: run the NVRTC-compiled condition kernel on-device. Any
        // runtime failure falls through to the identical CPU computation below.
        #[cfg(feature = "cuda")]
        if let Some(state) = &self.cuda
            && let Ok(results) = state.run_condition_eval(input, threshold)
        {
            for (entity_idx, value) in results.into_iter().enumerate() {
                output.set(entity_idx, 0, value)?;
            }
            return Ok(output);
        }

        // CPU computation: sum of features >= threshold.
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

    /// Returns `true` when a real GPU backend is active (the `cuda` feature is
    /// enabled and a device was initialised). When `false`, every GPU operation
    /// is served by the CPU model.
    pub fn is_gpu_active(&self) -> bool {
        #[cfg(feature = "cuda")]
        let active = self.cuda.is_some();
        #[cfg(not(feature = "cuda"))]
        let active = false;
        active
    }

    /// Evaluate a weighted-sum threshold condition
    /// (`Σⱼ attrⱼ · multⱼ  <op>  value`) across an entire population.
    ///
    /// This is the GPU-accelerated analogue of the engine's `Condition::Threshold`
    /// evaluation: it runs as an NVRTC-compiled kernel when a CUDA device is
    /// active, and on the CPU otherwise. Both paths return identical results. The
    /// number of `multipliers` must equal the tensor's feature count.
    pub fn evaluate_population_threshold(
        &self,
        input: &EntityTensor,
        multipliers: &[f32],
        value: f32,
        op: ThresholdOp,
    ) -> SimResult<Vec<bool>> {
        if multipliers.len() != input.num_features() {
            return Err(SimulationError::InvalidParameter(format!(
                "expected {} multipliers (one per feature), got {}",
                input.num_features(),
                multipliers.len()
            )));
        }

        #[cfg(feature = "cuda")]
        if let Some(state) = &self.cuda
            && let Ok(results) = state.run_threshold(input, multipliers, value, op)
        {
            return Ok(results);
        }

        Ok(cpu_evaluate_threshold(input, multipliers, value, op))
    }
}

// ===================================================================
// Threshold condition evaluation (backend-agnostic + CUDA acceleration)
// ===================================================================

/// Comparison operator for a weighted-sum threshold condition.
///
/// Mirrors `legalis_core::ComparisonOp` and the simulation engine's
/// `Condition::Threshold` semantics, so a population can be evaluated on the GPU
/// and on the CPU with identical results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThresholdOp {
    /// `total == value` (within 1e-6)
    Equal,
    /// `total != value` (outside 1e-6)
    NotEqual,
    /// `total > value`
    GreaterThan,
    /// `total >= value`
    GreaterOrEqual,
    /// `total < value`
    LessThan,
    /// `total <= value`
    LessOrEqual,
}

impl ThresholdOp {
    /// Integer op-code passed to the CUDA kernel (kept in sync with the kernel's
    /// `switch` statement).
    pub fn op_code(self) -> i32 {
        match self {
            ThresholdOp::Equal => 0,
            ThresholdOp::NotEqual => 1,
            ThresholdOp::GreaterThan => 2,
            ThresholdOp::GreaterOrEqual => 3,
            ThresholdOp::LessThan => 4,
            ThresholdOp::LessOrEqual => 5,
        }
    }

    /// Apply the operator on the CPU (reference semantics).
    pub fn apply(self, total: f32, value: f32) -> bool {
        match self {
            ThresholdOp::Equal => (total - value).abs() < 1e-6,
            ThresholdOp::NotEqual => (total - value).abs() >= 1e-6,
            ThresholdOp::GreaterThan => total > value,
            ThresholdOp::GreaterOrEqual => total >= value,
            ThresholdOp::LessThan => total < value,
            ThresholdOp::LessOrEqual => total <= value,
        }
    }
}

/// CPU reference implementation of the weighted-sum threshold evaluation.
///
/// Used whenever the GPU backend is unavailable, and as the oracle the GPU path
/// is validated against in tests.
pub fn cpu_evaluate_threshold(
    input: &EntityTensor,
    multipliers: &[f32],
    value: f32,
    op: ThresholdOp,
) -> Vec<bool> {
    (0..input.num_entities())
        .map(|entity_idx| {
            let mut total = 0.0f32;
            for feature_idx in 0..input.num_features() {
                let attr = input.get(entity_idx, feature_idx).unwrap_or(0.0);
                let mult = multipliers.get(feature_idx).copied().unwrap_or(1.0);
                total += attr * mult;
            }
            op.apply(total, value)
        })
        .collect()
}

/// Returns `true` if a real CUDA device can be initialised right now.
///
/// Always `false` unless the `cuda` feature is enabled and a working driver and
/// device are present at runtime.
pub fn gpu_available() -> bool {
    #[cfg(feature = "cuda")]
    let available = CudaState::try_new().is_some();
    #[cfg(not(feature = "cuda"))]
    let available = false;
    available
}

/// CUDA C source for the uniform sum-threshold condition kernel (one flag per
/// entity: `Σ featuresᵢ >= threshold`).
#[cfg(feature = "cuda")]
const CUDA_CONDITION_SRC: &str = r#"
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
"#;

/// CUDA C source for the weighted-sum threshold kernel (per-feature multipliers
/// plus a comparison op-code matching [`ThresholdOp::op_code`]).
#[cfg(feature = "cuda")]
const CUDA_THRESHOLD_SRC: &str = r#"
extern "C" __global__ void eval_threshold(
    const float* input,
    const float* mult,
    float* output,
    int num_entities,
    int num_features,
    float value,
    int op
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < num_entities) {
        float total = 0.0f;
        for (int i = 0; i < num_features; i++) {
            total += input[idx * num_features + i] * mult[i];
        }
        float r = 0.0f;
        switch (op) {
            case 0: r = (fabsf(total - value) < 1e-6f) ? 1.0f : 0.0f; break;
            case 1: r = (fabsf(total - value) >= 1e-6f) ? 1.0f : 0.0f; break;
            case 2: r = (total > value) ? 1.0f : 0.0f; break;
            case 3: r = (total >= value) ? 1.0f : 0.0f; break;
            case 4: r = (total < value) ? 1.0f : 0.0f; break;
            case 5: r = (total <= value) ? 1.0f : 0.0f; break;
            default: r = 0.0f; break;
        }
        output[idx] = r;
    }
}
"#;

/// Live CUDA backend: a retained context/stream plus NVRTC-compiled kernels.
#[cfg(feature = "cuda")]
struct CudaState {
    /// Retained primary context (keeps the device alive for `stream`).
    _ctx: Arc<CudaContext>,
    /// Default stream used for all transfers and launches.
    stream: Arc<CudaStream>,
    /// Loaded kernel functions keyed by entry-point name.
    functions: HashMap<String, CudaFunction>,
    /// Retained modules (own the loaded functions for their lifetime).
    _modules: Vec<Arc<CudaModule>>,
    /// Real device descriptor.
    device: GpuDevice,
}

#[cfg(feature = "cuda")]
impl std::fmt::Debug for CudaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CudaState")
            .field("device", &self.device.name)
            .field("kernels", &self.functions.len())
            .finish()
    }
}

#[cfg(feature = "cuda")]
fn cuda_err<E: std::fmt::Display>(e: E) -> SimulationError {
    SimulationError::ExecutionError(format!("CUDA error: {e}"))
}

#[cfg(feature = "cuda")]
impl CudaState {
    /// Attempt to initialise CUDA device 0 and compile the kernels. Returns
    /// `None` if no driver/device is available or compilation fails — callers
    /// then fall back to the CPU model.
    fn try_new() -> Option<Self> {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let ctx = CudaContext::new(0).ok()?;
            let stream = ctx.default_stream();

            let mut functions = HashMap::new();
            let mut modules = Vec::new();
            for (entry, src) in [
                ("eval_condition", CUDA_CONDITION_SRC),
                ("eval_threshold", CUDA_THRESHOLD_SRC),
            ] {
                let ptx = compile_ptx(src).ok()?;
                let module = ctx.load_module(ptx).ok()?;
                let func = module.load_function(entry).ok()?;
                functions.insert(entry.to_string(), func);
                modules.push(module);
            }

            let device = Self::device_descriptor(0, &ctx);
            Some(CudaState {
                _ctx: ctx,
                stream,
                functions,
                _modules: modules,
                device,
            })
        }));
        result.ok().flatten()
    }

    /// Build a `GpuDevice` descriptor for an initialised context.
    fn device_descriptor(ordinal: usize, ctx: &Arc<CudaContext>) -> GpuDevice {
        let name = ctx
            .name()
            .unwrap_or_else(|_| format!("CUDA device {ordinal}"));
        GpuDevice {
            name,
            id: ordinal,
            backend: GpuBackend::Cuda,
            total_memory: 0,
            available_memory: 0,
            compute_capability: None,
            max_work_group_size: 1024,
            max_threads_per_block: 1024,
        }
    }

    /// Enumerate all CUDA devices as descriptors (does not retain contexts).
    fn list_cuda_devices() -> Vec<GpuDevice> {
        let count = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            CudaContext::device_count().unwrap_or(0)
        }))
        .unwrap_or(0);
        let mut devices = Vec::new();
        for ordinal in 0..count as usize {
            let ctx_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                CudaContext::new(ordinal)
            }));
            if let Ok(Ok(ctx)) = ctx_result {
                devices.push(Self::device_descriptor(ordinal, &ctx));
            }
        }
        devices
    }

    /// Run the uniform sum-threshold kernel; returns one 1.0/0.0 flag per entity.
    fn run_condition_eval(&self, input: &EntityTensor, threshold: f32) -> SimResult<Vec<f32>> {
        let func = self
            .functions
            .get("eval_condition")
            .ok_or_else(|| cuda_err("eval_condition kernel not loaded"))?;

        let num_entities = input.num_entities();
        if num_entities == 0 {
            return Ok(Vec::new());
        }
        let entities_i = num_entities as i32;
        let features_i = input.num_features() as i32;

        let d_input = self.stream.clone_htod(&input.data).map_err(cuda_err)?;
        let mut d_output = self
            .stream
            .alloc_zeros::<f32>(num_entities)
            .map_err(cuda_err)?;

        unsafe {
            self.stream
                .launch_builder(func)
                .arg(&d_input)
                .arg(&mut d_output)
                .arg(&entities_i)
                .arg(&features_i)
                .arg(&threshold)
                .launch(LaunchConfig::for_num_elems(num_entities as u32))
        }
        .map_err(cuda_err)?;

        self.stream.clone_dtoh(&d_output).map_err(cuda_err)
    }

    /// Run the weighted-sum threshold kernel; returns one bool per entity.
    fn run_threshold(
        &self,
        input: &EntityTensor,
        multipliers: &[f32],
        value: f32,
        op: ThresholdOp,
    ) -> SimResult<Vec<bool>> {
        let func = self
            .functions
            .get("eval_threshold")
            .ok_or_else(|| cuda_err("eval_threshold kernel not loaded"))?;

        let num_entities = input.num_entities();
        if num_entities == 0 {
            return Ok(Vec::new());
        }
        let entities_i = num_entities as i32;
        let features_i = input.num_features() as i32;
        let op_code = op.op_code();

        let d_input = self.stream.clone_htod(&input.data).map_err(cuda_err)?;
        let d_mult = self.stream.clone_htod(multipliers).map_err(cuda_err)?;
        let mut d_output = self
            .stream
            .alloc_zeros::<f32>(num_entities)
            .map_err(cuda_err)?;

        unsafe {
            self.stream
                .launch_builder(func)
                .arg(&d_input)
                .arg(&d_mult)
                .arg(&mut d_output)
                .arg(&entities_i)
                .arg(&features_i)
                .arg(&value)
                .arg(&op_code)
                .launch(LaunchConfig::for_num_elems(num_entities as u32))
        }
        .map_err(cuda_err)?;

        let raw = self.stream.clone_dtoh(&d_output).map_err(cuda_err)?;
        Ok(raw.into_iter().map(|v| v != 0.0).collect())
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
        // Without the `cuda` feature (or with no GPU at runtime) the executor
        // falls back to the CPU device; with the feature and a real CUDA device
        // present it initialises the GPU backend instead.
        if executor.is_gpu_active() {
            assert_eq!(executor.device().backend, GpuBackend::Cuda);
        } else {
            assert_eq!(executor.device().backend, GpuBackend::CpuFallback);
        }
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
        // With the `cuda` feature and a real device the first entry is a GPU;
        // otherwise it is the CPU fallback device.
        if gpu_available() {
            assert!(devices[0].is_gpu());
        } else {
            assert_eq!(devices[0].backend, GpuBackend::CpuFallback);
        }
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

    // ---- Threshold evaluation (CPU; always compiled) ----

    fn small_tensor() -> EntityTensor {
        // 3 entities x 2 features; weighted totals (mult [1, 2]) are 5, 3, 1.5.
        let mut input = EntityTensor::new(3, 2);
        for (i, row) in [[1.0f32, 2.0], [3.0, 0.0], [0.5, 0.5]].iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                input.set(i, j, *v).unwrap();
            }
        }
        input
    }

    #[test]
    fn test_threshold_op_codes_and_apply() {
        assert_eq!(ThresholdOp::Equal.op_code(), 0);
        assert_eq!(ThresholdOp::LessOrEqual.op_code(), 5);
        assert!(ThresholdOp::GreaterOrEqual.apply(5.0, 5.0));
        assert!(!ThresholdOp::GreaterThan.apply(5.0, 5.0));
        assert!(ThresholdOp::LessThan.apply(1.0, 2.0));
        assert!(ThresholdOp::Equal.apply(3.0, 3.0));
        assert!(ThresholdOp::NotEqual.apply(3.0, 4.0));
    }

    #[test]
    fn test_cpu_evaluate_threshold() {
        let input = small_tensor();
        let res = cpu_evaluate_threshold(&input, &[1.0, 2.0], 3.0, ThresholdOp::GreaterOrEqual);
        assert_eq!(res, vec![true, true, false]);
    }

    #[test]
    fn test_evaluate_population_threshold_cpu_path() {
        // Default config uses the CPU fallback backend (even when the `cuda`
        // feature is compiled in), so this exercises the CPU path.
        let executor = GpuExecutor::new(GpuConfig::default()).unwrap();
        let input = small_tensor();
        let res = executor
            .evaluate_population_threshold(&input, &[1.0, 2.0], 3.0, ThresholdOp::GreaterOrEqual)
            .unwrap();
        assert_eq!(res, vec![true, true, false]);
    }

    #[test]
    fn test_evaluate_population_threshold_multiplier_mismatch() {
        let executor = GpuExecutor::new(GpuConfig::default()).unwrap();
        let input = EntityTensor::new(2, 3);
        let result =
            executor.evaluate_population_threshold(&input, &[1.0], 0.0, ThresholdOp::Equal);
        assert!(result.is_err());
    }

    #[test]
    fn test_gpu_available_does_not_panic() {
        // True only with the `cuda` feature and a real device; must never panic.
        let _ = gpu_available();
    }

    // ---- Real CUDA backend (feature = "cuda", requires a GPU at runtime) ----

    #[cfg(feature = "cuda")]
    #[test]
    fn test_cuda_condition_eval_matches_cpu() {
        if !gpu_available() {
            return; // no CUDA device present at runtime
        }
        let mut executor = GpuExecutor::new(GpuConfig::cuda()).unwrap();
        assert!(executor.is_gpu_active());
        assert_eq!(executor.device().backend, GpuBackend::Cuda);
        executor
            .add_kernel(GpuKernel::condition_eval_cuda())
            .unwrap();

        let mut input = EntityTensor::new(4, 3);
        input.entity_ids = vec!["e0".into(), "e1".into(), "e2".into(), "e3".into()];
        for (i, row) in [
            [1.0f32, 2.0, 3.0], // sum 6
            [0.0, 0.0, 0.0],    // sum 0
            [2.0, 2.0, 2.0],    // sum 6
            [1.0, 0.0, 0.0],    // sum 1
        ]
        .iter()
        .enumerate()
        {
            for (j, v) in row.iter().enumerate() {
                input.set(i, j, *v).unwrap();
            }
        }
        let mut params = HashMap::new();
        params.insert("threshold".to_string(), 5.0);

        let out = executor.execute("condition_eval", &input, &params).unwrap();
        assert_eq!(out.get(0, 0), Some(1.0));
        assert_eq!(out.get(1, 0), Some(0.0));
        assert_eq!(out.get(2, 0), Some(1.0));
        assert_eq!(out.get(3, 0), Some(0.0));
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn test_cuda_threshold_matches_cpu_all_ops() {
        if !gpu_available() {
            return;
        }
        let executor = GpuExecutor::new(GpuConfig::cuda()).unwrap();
        assert!(executor.is_gpu_active());

        let mut input = EntityTensor::new(5, 2);
        for (i, row) in [
            [1.0f32, 2.0],
            [3.0, 0.0],
            [0.5, 0.5],
            [10.0, 10.0],
            [0.0, 0.0],
        ]
        .iter()
        .enumerate()
        {
            for (j, v) in row.iter().enumerate() {
                input.set(i, j, *v).unwrap();
            }
        }
        let multipliers = [1.0f32, 2.0];
        let value = 5.0f32;
        for op in [
            ThresholdOp::Equal,
            ThresholdOp::NotEqual,
            ThresholdOp::GreaterThan,
            ThresholdOp::GreaterOrEqual,
            ThresholdOp::LessThan,
            ThresholdOp::LessOrEqual,
        ] {
            let gpu = executor
                .evaluate_population_threshold(&input, &multipliers, value, op)
                .unwrap();
            let cpu = cpu_evaluate_threshold(&input, &multipliers, value, op);
            assert_eq!(gpu, cpu, "GPU/CPU mismatch for {op:?}");
        }
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn test_cuda_list_devices_reports_gpu() {
        if !gpu_available() {
            return;
        }
        let devices = GpuExecutor::list_devices();
        assert!(devices.iter().any(|d| d.is_gpu()));
        assert!(devices.iter().any(|d| d.backend == GpuBackend::Cuda));
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn test_cuda_empty_population() {
        if !gpu_available() {
            return;
        }
        let executor = GpuExecutor::new(GpuConfig::cuda()).unwrap();
        let input = EntityTensor::new(0, 2);
        let res = executor
            .evaluate_population_threshold(&input, &[1.0, 1.0], 0.0, ThresholdOp::GreaterOrEqual)
            .unwrap();
        assert!(res.is_empty());
    }
}
