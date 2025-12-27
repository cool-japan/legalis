//! Model quantization support for efficient inference.
//!
//! This module provides support for quantized models including GGUF and AWQ formats,
//! enabling efficient inference with reduced memory footprint.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Quantization format for model weights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantizationFormat {
    /// GGUF format (used by llama.cpp)
    GGUF,
    /// AWQ (Activation-aware Weight Quantization)
    AWQ,
    /// GPTQ (GPT-Quantization)
    GPTQ,
    /// 8-bit quantization
    Int8,
    /// 4-bit quantization
    Int4,
    /// No quantization (full precision)
    None,
}

impl QuantizationFormat {
    /// Returns the typical file extension for this format.
    pub fn file_extension(&self) -> &str {
        match self {
            QuantizationFormat::GGUF => ".gguf",
            QuantizationFormat::AWQ => ".awq",
            QuantizationFormat::GPTQ => ".gptq",
            QuantizationFormat::Int8 => ".int8",
            QuantizationFormat::Int4 => ".int4",
            QuantizationFormat::None => "",
        }
    }

    /// Detects quantization format from file path.
    pub fn from_path(path: &Path) -> Self {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "gguf" => QuantizationFormat::GGUF,
                "awq" => QuantizationFormat::AWQ,
                "gptq" => QuantizationFormat::GPTQ,
                "int8" => QuantizationFormat::Int8,
                "int4" => QuantizationFormat::Int4,
                _ => QuantizationFormat::None,
            }
        } else {
            QuantizationFormat::None
        }
    }
}

/// Quantization precision level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantizationPrecision {
    /// 2-bit quantization (extremely low memory, lower quality)
    Q2,
    /// 3-bit quantization
    Q3,
    /// 4-bit quantization (good balance)
    Q4,
    /// 5-bit quantization
    Q5,
    /// 6-bit quantization
    Q6,
    /// 8-bit quantization (higher quality)
    Q8,
    /// 16-bit floating point (half precision)
    F16,
    /// 32-bit floating point (full precision)
    F32,
}

impl QuantizationPrecision {
    /// Returns the number of bits used for quantization.
    pub fn bits(&self) -> u8 {
        match self {
            QuantizationPrecision::Q2 => 2,
            QuantizationPrecision::Q3 => 3,
            QuantizationPrecision::Q4 => 4,
            QuantizationPrecision::Q5 => 5,
            QuantizationPrecision::Q6 => 6,
            QuantizationPrecision::Q8 => 8,
            QuantizationPrecision::F16 => 16,
            QuantizationPrecision::F32 => 32,
        }
    }

    /// Estimates memory reduction factor compared to F32.
    pub fn memory_reduction_factor(&self) -> f32 {
        32.0 / self.bits() as f32
    }
}

/// Quantization configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfig {
    /// Quantization format
    pub format: QuantizationFormat,
    /// Precision level
    pub precision: QuantizationPrecision,
    /// Group size for quantization (for AWQ/GPTQ)
    pub group_size: Option<usize>,
    /// Whether to use zero-point quantization
    pub use_zero_point: bool,
    /// Custom metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for QuantizationConfig {
    fn default() -> Self {
        Self {
            format: QuantizationFormat::None,
            precision: QuantizationPrecision::F32,
            group_size: None,
            use_zero_point: false,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl QuantizationConfig {
    /// Creates a new quantization configuration.
    pub fn new(format: QuantizationFormat, precision: QuantizationPrecision) -> Self {
        Self {
            format,
            precision,
            ..Default::default()
        }
    }

    /// Creates a GGUF quantization config with specified precision.
    pub fn gguf(precision: QuantizationPrecision) -> Self {
        Self::new(QuantizationFormat::GGUF, precision)
    }

    /// Creates an AWQ quantization config.
    pub fn awq(group_size: usize) -> Self {
        Self {
            format: QuantizationFormat::AWQ,
            precision: QuantizationPrecision::Q4,
            group_size: Some(group_size),
            use_zero_point: true,
            ..Default::default()
        }
    }

    /// Creates a GPTQ quantization config.
    pub fn gptq(group_size: usize) -> Self {
        Self {
            format: QuantizationFormat::GPTQ,
            precision: QuantizationPrecision::Q4,
            group_size: Some(group_size),
            use_zero_point: false,
            ..Default::default()
        }
    }

    /// Sets the group size for quantization.
    pub fn with_group_size(mut self, group_size: usize) -> Self {
        self.group_size = Some(group_size);
        self
    }

    /// Enables zero-point quantization.
    pub fn with_zero_point(mut self) -> Self {
        self.use_zero_point = true;
        self
    }

    /// Adds custom metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Estimates memory usage reduction compared to full precision.
    pub fn estimated_memory_reduction(&self) -> f32 {
        self.precision.memory_reduction_factor()
    }
}

/// Model quantization metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationMetadata {
    /// Original model size in bytes
    pub original_size_bytes: u64,
    /// Quantized model size in bytes
    pub quantized_size_bytes: u64,
    /// Quantization configuration
    pub config: QuantizationConfig,
    /// Model architecture
    pub architecture: Option<String>,
    /// Number of parameters
    pub num_parameters: Option<u64>,
    /// Quantization timestamp
    pub quantized_at: chrono::DateTime<chrono::Utc>,
}

impl QuantizationMetadata {
    /// Creates new quantization metadata.
    pub fn new(original_size: u64, quantized_size: u64, config: QuantizationConfig) -> Self {
        Self {
            original_size_bytes: original_size,
            quantized_size_bytes: quantized_size,
            config,
            architecture: None,
            num_parameters: None,
            quantized_at: chrono::Utc::now(),
        }
    }

    /// Calculates actual compression ratio.
    pub fn compression_ratio(&self) -> f64 {
        if self.quantized_size_bytes == 0 {
            0.0
        } else {
            self.original_size_bytes as f64 / self.quantized_size_bytes as f64
        }
    }

    /// Calculates size reduction percentage.
    pub fn size_reduction_percent(&self) -> f64 {
        if self.original_size_bytes == 0 {
            0.0
        } else {
            ((self.original_size_bytes - self.quantized_size_bytes) as f64
                / self.original_size_bytes as f64)
                * 100.0
        }
    }

    /// Sets the model architecture.
    pub fn with_architecture(mut self, arch: impl Into<String>) -> Self {
        self.architecture = Some(arch.into());
        self
    }

    /// Sets the number of parameters.
    pub fn with_parameters(mut self, params: u64) -> Self {
        self.num_parameters = Some(params);
        self
    }
}

/// GGUF model loader for llama.cpp compatible models.
pub struct GGUFLoader {
    model_path: std::path::PathBuf,
}

impl GGUFLoader {
    /// Creates a new GGUF loader.
    pub fn new(model_path: impl AsRef<Path>) -> Self {
        Self {
            model_path: model_path.as_ref().to_path_buf(),
        }
    }

    /// Validates that the file is a valid GGUF file.
    pub fn validate(&self) -> Result<()> {
        if !self.model_path.exists() {
            return Err(anyhow!("Model file does not exist: {:?}", self.model_path));
        }

        // Check file extension
        if self.model_path.extension().and_then(|e| e.to_str()) != Some("gguf") {
            return Err(anyhow!(
                "File does not have .gguf extension: {:?}",
                self.model_path
            ));
        }

        Ok(())
    }

    /// Gets the model file path.
    pub fn path(&self) -> &Path {
        &self.model_path
    }

    /// Detects the quantization level from filename.
    pub fn detect_quantization(&self) -> Option<QuantizationPrecision> {
        let filename = self.model_path.file_name()?.to_str()?;

        if filename.contains("Q2") {
            Some(QuantizationPrecision::Q2)
        } else if filename.contains("Q3") {
            Some(QuantizationPrecision::Q3)
        } else if filename.contains("Q4") {
            Some(QuantizationPrecision::Q4)
        } else if filename.contains("Q5") {
            Some(QuantizationPrecision::Q5)
        } else if filename.contains("Q6") {
            Some(QuantizationPrecision::Q6)
        } else if filename.contains("Q8") {
            Some(QuantizationPrecision::Q8)
        } else if filename.contains("F16") {
            Some(QuantizationPrecision::F16)
        } else if filename.contains("F32") {
            Some(QuantizationPrecision::F32)
        } else {
            None
        }
    }
}

/// AWQ (Activation-aware Weight Quantization) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AWQConfig {
    /// Bits per weight
    pub bits: u8,
    /// Group size for quantization
    pub group_size: usize,
    /// Whether to use zero-point
    pub zero_point: bool,
    /// Version of AWQ algorithm
    pub version: String,
}

impl Default for AWQConfig {
    fn default() -> Self {
        Self {
            bits: 4,
            group_size: 128,
            zero_point: true,
            version: "gemm".to_string(),
        }
    }
}

impl AWQConfig {
    /// Creates a new AWQ configuration.
    pub fn new(bits: u8, group_size: usize) -> Self {
        Self {
            bits,
            group_size,
            ..Default::default()
        }
    }

    /// Creates a 4-bit AWQ config with group size 128 (recommended).
    pub fn recommended() -> Self {
        Self::default()
    }

    /// Sets the version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }
}

/// Quantization performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationPerformance {
    /// Inference latency in milliseconds
    pub latency_ms: f64,
    /// Throughput in tokens per second
    pub throughput_tps: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Peak memory usage in bytes
    pub peak_memory_bytes: u64,
    /// Model loading time in milliseconds
    pub load_time_ms: f64,
}

impl QuantizationPerformance {
    /// Creates new performance metrics.
    pub fn new(
        latency_ms: f64,
        throughput_tps: f64,
        memory_usage_bytes: u64,
        load_time_ms: f64,
    ) -> Self {
        Self {
            latency_ms,
            throughput_tps,
            memory_usage_bytes,
            peak_memory_bytes: memory_usage_bytes,
            load_time_ms,
        }
    }

    /// Calculates speedup factor compared to baseline.
    pub fn speedup_vs(&self, baseline: &QuantizationPerformance) -> f64 {
        if self.latency_ms == 0.0 {
            0.0
        } else {
            baseline.latency_ms / self.latency_ms
        }
    }

    /// Calculates memory reduction compared to baseline.
    pub fn memory_reduction_vs(&self, baseline: &QuantizationPerformance) -> f64 {
        if self.memory_usage_bytes == 0 {
            0.0
        } else {
            1.0 - (self.memory_usage_bytes as f64 / baseline.memory_usage_bytes as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantization_format_detection() {
        let path = Path::new("model.gguf");
        assert_eq!(
            QuantizationFormat::from_path(path),
            QuantizationFormat::GGUF
        );

        let path = Path::new("model.awq");
        assert_eq!(QuantizationFormat::from_path(path), QuantizationFormat::AWQ);
    }

    #[test]
    fn test_quantization_precision_bits() {
        assert_eq!(QuantizationPrecision::Q4.bits(), 4);
        assert_eq!(QuantizationPrecision::Q8.bits(), 8);
        assert_eq!(QuantizationPrecision::F16.bits(), 16);
        assert_eq!(QuantizationPrecision::F32.bits(), 32);
    }

    #[test]
    fn test_memory_reduction_factor() {
        assert_eq!(QuantizationPrecision::Q4.memory_reduction_factor(), 8.0);
        assert_eq!(QuantizationPrecision::Q8.memory_reduction_factor(), 4.0);
        assert_eq!(QuantizationPrecision::F16.memory_reduction_factor(), 2.0);
        assert_eq!(QuantizationPrecision::F32.memory_reduction_factor(), 1.0);
    }

    #[test]
    fn test_quantization_config_gguf() {
        let config = QuantizationConfig::gguf(QuantizationPrecision::Q4);
        assert_eq!(config.format, QuantizationFormat::GGUF);
        assert_eq!(config.precision, QuantizationPrecision::Q4);
    }

    #[test]
    fn test_quantization_config_awq() {
        let config = QuantizationConfig::awq(128);
        assert_eq!(config.format, QuantizationFormat::AWQ);
        assert_eq!(config.precision, QuantizationPrecision::Q4);
        assert_eq!(config.group_size, Some(128));
        assert!(config.use_zero_point);
    }

    #[test]
    fn test_quantization_metadata_compression_ratio() {
        let config = QuantizationConfig::gguf(QuantizationPrecision::Q4);
        let metadata = QuantizationMetadata::new(1000, 250, config);

        assert_eq!(metadata.compression_ratio(), 4.0);
        assert_eq!(metadata.size_reduction_percent(), 75.0);
    }

    #[test]
    fn test_awq_config_default() {
        let config = AWQConfig::default();
        assert_eq!(config.bits, 4);
        assert_eq!(config.group_size, 128);
        assert!(config.zero_point);
    }

    #[test]
    fn test_quantization_performance_speedup() {
        let baseline = QuantizationPerformance::new(100.0, 10.0, 1000, 50.0);
        let quantized = QuantizationPerformance::new(25.0, 40.0, 250, 10.0);

        assert_eq!(quantized.speedup_vs(&baseline), 4.0);
        assert_eq!(quantized.memory_reduction_vs(&baseline), 0.75);
    }
}
