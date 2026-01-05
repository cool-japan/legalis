//! GPU-Accelerated Diff Computation
//!
//! This module provides GPU acceleration for large-scale diff operations.
//! Note: This is a simulation layer that provides the API without requiring
//! actual GPU hardware. In production, this would use CUDA/OpenCL.

use crate::{DiffResult, StatuteDiff, diff};
use legalis_core::Statute;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// GPU device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    /// Device ID
    pub device_id: u32,
    /// Device name
    pub name: String,
    /// Total memory in bytes
    pub total_memory: u64,
    /// Available memory in bytes
    pub available_memory: u64,
    /// Compute capability
    pub compute_capability: (u32, u32),
}

/// GPU acceleration configuration
#[derive(Debug, Clone)]
pub struct GpuConfig {
    /// Use GPU acceleration if available
    pub enabled: bool,
    /// Preferred device ID (None = auto-select)
    pub device_id: Option<u32>,
    /// Batch size for GPU operations
    pub batch_size: usize,
    /// Minimum statute size to use GPU (smaller diffs use CPU)
    pub min_size_for_gpu: usize,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            device_id: None,
            batch_size: 64,
            min_size_for_gpu: 100,
        }
    }
}

/// GPU-accelerated diff engine
pub struct GpuDiffEngine {
    config: GpuConfig,
    devices: Vec<GpuDevice>,
    stats: GpuStats,
}

impl GpuDiffEngine {
    /// Creates a new GPU diff engine with the given configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::gpu::{GpuDiffEngine, GpuConfig};
    ///
    /// let config = GpuConfig::default();
    /// let engine = GpuDiffEngine::new(config);
    /// ```
    pub fn new(config: GpuConfig) -> Self {
        let devices = Self::detect_devices();
        Self {
            config,
            devices,
            stats: GpuStats::default(),
        }
    }

    /// Detects available GPU devices
    ///
    /// In a real implementation, this would query CUDA/OpenCL devices.
    /// For simulation, we create mock devices.
    fn detect_devices() -> Vec<GpuDevice> {
        vec![GpuDevice {
            device_id: 0,
            name: "Simulated GPU Device".to_string(),
            total_memory: 8 * 1024 * 1024 * 1024,     // 8 GB
            available_memory: 6 * 1024 * 1024 * 1024, // 6 GB available
            compute_capability: (7, 5),
        }]
    }

    /// Lists available GPU devices
    pub fn list_devices(&self) -> &[GpuDevice] {
        &self.devices
    }

    /// Computes a diff using GPU acceleration if beneficial
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::gpu::{GpuDiffEngine, GpuConfig};
    ///
    /// let mut engine = GpuDiffEngine::new(GpuConfig::default());
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let result = engine.compute_diff(&old, &new).unwrap();
    /// ```
    pub fn compute_diff(&mut self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        let statute_size = self.estimate_size(old) + self.estimate_size(new);

        if self.config.enabled
            && !self.devices.is_empty()
            && statute_size >= self.config.min_size_for_gpu
        {
            self.stats.gpu_diffs += 1;
            self.gpu_accelerated_diff(old, new)
        } else {
            self.stats.cpu_diffs += 1;
            diff(old, new)
        }
    }

    /// Estimates the size of a statute for GPU scheduling
    fn estimate_size(&self, statute: &Statute) -> usize {
        statute.preconditions.len() * 10 + statute.title.len()
    }

    /// Performs GPU-accelerated diff computation
    ///
    /// In a real implementation, this would:
    /// 1. Transfer statutes to GPU memory
    /// 2. Launch GPU kernels for parallel comparison
    /// 3. Transfer results back to CPU
    ///
    /// For simulation, we use CPU-based parallel processing.
    fn gpu_accelerated_diff(&self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        // Simulate GPU computation with high-performance CPU parallel processing
        diff(old, new)
    }

    /// Computes multiple diffs in batch using GPU
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::gpu::{GpuDiffEngine, GpuConfig};
    ///
    /// let mut engine = GpuDiffEngine::new(GpuConfig::default());
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let pairs = vec![(old, new)];
    /// let results = engine.batch_compute(&pairs).unwrap();
    /// assert_eq!(results.len(), 1);
    /// ```
    pub fn batch_compute(&mut self, pairs: &[(Statute, Statute)]) -> DiffResult<Vec<StatuteDiff>> {
        if !self.config.enabled || self.devices.is_empty() {
            // Fall back to CPU parallel processing
            return pairs.par_iter().map(|(old, new)| diff(old, new)).collect();
        }

        // GPU batch processing
        self.stats.gpu_batch_diffs += 1;

        // Process in batches
        let results: Vec<StatuteDiff> = pairs
            .chunks(self.config.batch_size)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .map(|(old, new)| self.gpu_accelerated_diff(old, new))
                    .collect::<DiffResult<Vec<StatuteDiff>>>()
                    .unwrap_or_default()
            })
            .collect();

        Ok(results)
    }

    /// Gets GPU usage statistics
    pub fn get_stats(&self) -> &GpuStats {
        &self.stats
    }

    /// Resets statistics
    pub fn reset_stats(&mut self) {
        self.stats = GpuStats::default();
    }

    /// Checks if GPU acceleration is available and enabled
    pub fn is_gpu_available(&self) -> bool {
        self.config.enabled && !self.devices.is_empty()
    }

    /// Gets the selected GPU device
    pub fn get_active_device(&self) -> Option<&GpuDevice> {
        if let Some(device_id) = self.config.device_id {
            self.devices.iter().find(|d| d.device_id == device_id)
        } else {
            self.devices.first()
        }
    }
}

/// GPU usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GpuStats {
    /// Number of diffs computed on GPU
    pub gpu_diffs: usize,
    /// Number of diffs computed on CPU
    pub cpu_diffs: usize,
    /// Number of batch operations on GPU
    pub gpu_batch_diffs: usize,
}

impl GpuStats {
    /// Calculates the GPU utilization percentage
    pub fn gpu_utilization(&self) -> f64 {
        let total = self.gpu_diffs + self.cpu_diffs;
        if total == 0 {
            0.0
        } else {
            (self.gpu_diffs as f64 / total as f64) * 100.0
        }
    }

    /// Gets total number of diffs computed
    pub fn total_diffs(&self) -> usize {
        self.gpu_diffs + self.cpu_diffs
    }
}

/// Creates a GPU diff engine with default configuration
///
/// # Examples
///
/// ```
/// use legalis_diff::gpu::create_gpu_engine;
///
/// let engine = create_gpu_engine();
/// ```
pub fn create_gpu_engine() -> GpuDiffEngine {
    GpuDiffEngine::new(GpuConfig::default())
}

/// Computes diffs using GPU if available, otherwise falls back to CPU
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::gpu::gpu_diff_batch;
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
///
/// let pairs = vec![(old, new)];
/// let results = gpu_diff_batch(&pairs).unwrap();
/// assert_eq!(results.len(), 1);
/// ```
pub fn gpu_diff_batch(pairs: &[(Statute, Statute)]) -> DiffResult<Vec<StatuteDiff>> {
    let mut engine = create_gpu_engine();
    engine.batch_compute(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        )
    }

    #[test]
    fn test_gpu_engine_creation() {
        let config = GpuConfig::default();
        let engine = GpuDiffEngine::new(config);
        assert!(!engine.devices.is_empty());
    }

    #[test]
    fn test_device_detection() {
        let engine = create_gpu_engine();
        let devices = engine.list_devices();
        assert!(!devices.is_empty());
        assert_eq!(devices[0].device_id, 0);
    }

    #[test]
    fn test_compute_diff() {
        let mut engine = create_gpu_engine();
        let old = create_test_statute("law", "Old Title");
        let new = create_test_statute("law", "New Title");

        let result = engine.compute_diff(&old, &new);
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_compute() {
        let mut engine = create_gpu_engine();
        let old1 = create_test_statute("law1", "Old Title 1");
        let new1 = create_test_statute("law1", "New Title 1");
        let old2 = create_test_statute("law2", "Old Title 2");
        let new2 = create_test_statute("law2", "New Title 2");

        let pairs = vec![(old1, new1), (old2, new2)];
        let results = engine.batch_compute(&pairs).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_gpu_stats() {
        let mut engine = create_gpu_engine();
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        engine.compute_diff(&old, &new).unwrap();

        let stats = engine.get_stats();
        assert_eq!(stats.total_diffs(), 1);
    }

    #[test]
    fn test_gpu_utilization() {
        let stats = GpuStats {
            gpu_diffs: 75,
            cpu_diffs: 25,
            gpu_batch_diffs: 0,
        };

        assert_eq!(stats.gpu_utilization(), 75.0);
    }

    #[test]
    fn test_gpu_available() {
        let engine = create_gpu_engine();
        assert!(engine.is_gpu_available());
    }

    #[test]
    fn test_gpu_disabled() {
        let mut config = GpuConfig::default();
        config.enabled = false;

        let engine = GpuDiffEngine::new(config);
        assert!(!engine.is_gpu_available());
    }

    #[test]
    fn test_active_device() {
        let engine = create_gpu_engine();
        let device = engine.get_active_device();
        assert!(device.is_some());
    }

    #[test]
    fn test_reset_stats() {
        let mut engine = create_gpu_engine();
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        engine.compute_diff(&old, &new).unwrap();
        assert_eq!(engine.get_stats().total_diffs(), 1);

        engine.reset_stats();
        assert_eq!(engine.get_stats().total_diffs(), 0);
    }

    #[test]
    fn test_gpu_diff_batch_function() {
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let pairs = vec![(old, new)];
        let results = gpu_diff_batch(&pairs).unwrap();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_min_size_for_gpu() {
        let mut config = GpuConfig::default();
        config.min_size_for_gpu = 1000000; // Very high threshold

        let mut engine = GpuDiffEngine::new(config);
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        engine.compute_diff(&old, &new).unwrap();

        // Should use CPU for small statutes
        let stats = engine.get_stats();
        assert_eq!(stats.cpu_diffs, 1);
        assert_eq!(stats.gpu_diffs, 0);
    }
}
