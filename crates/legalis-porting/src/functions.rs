//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use async_trait::async_trait;
use legalis_i18n::Jurisdiction;

use super::types_4::{PortingOutput, PortingRequest};
use super::types_6::PortingError;
use super::types_8::CompatibilityReport;

/// Result type for porting operations.
pub type PortingResult<T> = Result<T, PortingError>;
/// Simple dyn-compatible trait for text generation.
#[async_trait]
pub trait TextGenerator: Send + Sync {
    /// Generates text from a prompt.
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
}
/// Trait for porting adapters.
#[async_trait]
pub trait PortingAdapter: Send + Sync {
    /// Ports statutes from source to target jurisdiction.
    async fn port(&self, request: &PortingRequest) -> PortingResult<PortingOutput>;
    /// Analyzes compatibility between jurisdictions.
    async fn analyze_compatibility(
        &self,
        source: &Jurisdiction,
        target: &Jurisdiction,
    ) -> PortingResult<CompatibilityReport>;
}
