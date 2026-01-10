//! OpenTelemetry distributed tracing support.
//!
//! This module provides OpenTelemetry tracing integration for distributed tracing
//! and observability across services.

#[cfg(feature = "otel-tracing")]
use opentelemetry::{KeyValue, global};
#[cfg(feature = "otel-tracing")]
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
#[cfg(feature = "otel-tracing")]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// OpenTelemetry configuration.
#[derive(Debug, Clone)]
pub struct OtelConfig {
    /// Service name for tracing
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// OTLP endpoint (e.g., "http://localhost:4317")
    pub otlp_endpoint: Option<String>,
    /// Sample rate (0.0 to 1.0)
    pub sample_rate: f64,
    /// Enable console exporter for development
    pub console_exporter: bool,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            service_name: "legalis-api".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            otlp_endpoint: None,
            sample_rate: 1.0,
            console_exporter: false,
        }
    }
}

impl OtelConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(name) = std::env::var("OTEL_SERVICE_NAME") {
            config.service_name = name;
        }

        if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
            config.otlp_endpoint = Some(endpoint);
        }

        if let Ok(rate) = std::env::var("OTEL_TRACE_SAMPLE_RATE") {
            if let Ok(r) = rate.parse::<f64>() {
                config.sample_rate = r.clamp(0.0, 1.0);
            }
        }

        if let Ok(console) = std::env::var("OTEL_CONSOLE_EXPORTER") {
            config.console_exporter = console.to_lowercase() == "true" || console == "1";
        }

        config
    }
}

/// Initialize OpenTelemetry tracing (requires "otel-tracing" feature).
#[cfg(feature = "otel-tracing")]
pub fn init_telemetry(config: OtelConfig) -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::trace::TracerProvider;
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::trace::Sampler;

    // Create resource with service information
    let resource = Resource::builder()
        .with_service_name(config.service_name.clone())
        .with_attribute(KeyValue::new(
            "service.version",
            config.service_version.clone(),
        ))
        .build();

    // Build the tracer provider
    let mut provider_builder = SdkTracerProvider::builder().with_resource(resource);

    // Configure sampler based on sample rate
    if config.sample_rate < 1.0 {
        provider_builder =
            provider_builder.with_sampler(Sampler::TraceIdRatioBased(config.sample_rate));
    }

    // Configure OTLP exporter if endpoint is provided
    if let Some(endpoint) = config.otlp_endpoint {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()?;
        provider_builder = provider_builder.with_batch_exporter(exporter);
    }

    let provider = provider_builder.build();

    // Get a tracer from the provider
    let tracer = provider.tracer("legalis-api");

    // Set as global provider
    global::set_tracer_provider(provider);

    // Create tracing layer
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Initialize subscriber with telemetry layer
    let subscriber = tracing_subscriber::registry()
        .with(telemetry_layer)
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    subscriber.try_init()?;

    tracing::info!(
        service_name = %config.service_name,
        service_version = %config.service_version,
        sample_rate = %config.sample_rate,
        "OpenTelemetry tracing initialized"
    );

    Ok(())
}

/// Shutdown OpenTelemetry (requires "otel-tracing" feature).
#[cfg(feature = "otel-tracing")]
pub fn shutdown_telemetry() {
    // In OpenTelemetry 0.31+, providers are dropped when going out of scope
    // For explicit shutdown, the provider needs to be stored and dropped
    tracing::info!("OpenTelemetry tracing shut down");
}

/// Stub init function when feature is disabled.
#[cfg(not(feature = "otel-tracing"))]
pub fn init_telemetry(_config: OtelConfig) -> Result<(), Box<dyn std::error::Error>> {
    tracing::warn!("OpenTelemetry tracing is not enabled. Enable the 'otel-tracing' feature.");
    Ok(())
}

/// Stub shutdown function when feature is disabled.
#[cfg(not(feature = "otel-tracing"))]
pub fn shutdown_telemetry() {
    // No-op when feature is disabled
}

/// Trace attributes for common operations.
pub mod trace_attrs {
    /// Statute ID attribute.
    pub const STATUTE_ID: &str = "statute.id";
    /// Statute version attribute.
    pub const STATUTE_VERSION: &str = "statute.version";
    /// User ID attribute.
    pub const USER_ID: &str = "user.id";
    /// User role attribute.
    pub const USER_ROLE: &str = "user.role";
    /// Operation type attribute.
    pub const OPERATION: &str = "operation.type";
    /// Verification status attribute.
    pub const VERIFICATION_STATUS: &str = "verification.status";
    /// Simulation size attribute.
    pub const SIMULATION_SIZE: &str = "simulation.population_size";
    /// Error count attribute.
    pub const ERROR_COUNT: &str = "error.count";
    /// Warning count attribute.
    pub const WARNING_COUNT: &str = "warning.count";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otel_config_default() {
        let config = OtelConfig::default();
        assert_eq!(config.service_name, "legalis-api");
        assert_eq!(config.sample_rate, 1.0);
        assert!(!config.console_exporter);
    }

    #[test]
    fn test_otel_config_from_env() {
        unsafe {
            std::env::set_var("OTEL_SERVICE_NAME", "test-service");
            std::env::set_var("OTEL_TRACE_SAMPLE_RATE", "0.5");
        }

        let config = OtelConfig::from_env();
        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.sample_rate, 0.5);

        unsafe {
            std::env::remove_var("OTEL_SERVICE_NAME");
            std::env::remove_var("OTEL_TRACE_SAMPLE_RATE");
        }
    }

    #[test]
    fn test_trace_attributes() {
        assert_eq!(trace_attrs::STATUTE_ID, "statute.id");
        assert_eq!(trace_attrs::USER_ID, "user.id");
        assert_eq!(trace_attrs::OPERATION, "operation.type");
    }
}
