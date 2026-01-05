//! Observability and metrics for LLM operations.
//!
//! This module provides metrics collection, performance tracking,
//! and monitoring capabilities for LLM providers.

use crate::{LLMProvider, TextStream, TokenUsage};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{Instrument, info_span};

/// Metrics for a single LLM request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Provider name
    pub provider: String,
    /// Model name
    pub model: String,
    /// Request duration
    pub duration_ms: u128,
    /// Token usage
    pub tokens: Option<TokenUsage>,
    /// Estimated cost in USD
    pub cost_usd: Option<f64>,
    /// Whether the request succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl RequestMetrics {
    /// Creates a new request metrics record.
    pub fn new(provider: String, model: String, duration_ms: u128, success: bool) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            provider,
            model,
            duration_ms,
            tokens: None,
            cost_usd: None,
            success,
            error: None,
        }
    }

    /// Adds token usage information.
    pub fn with_tokens(mut self, tokens: TokenUsage) -> Self {
        self.tokens = Some(tokens);
        self
    }

    /// Adds cost information.
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost_usd = Some(cost);
        self
    }

    /// Adds error information.
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

/// Aggregated metrics across multiple requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Total number of requests
    pub total_requests: u64,
    /// Number of successful requests
    pub successful_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Total tokens used (input + output)
    pub total_tokens: u64,
    /// Total cost in USD
    pub total_cost_usd: f64,
    /// Average request duration in milliseconds
    pub avg_duration_ms: f64,
    /// p50 (median) latency in milliseconds
    pub p50_latency_ms: u128,
    /// p95 latency in milliseconds
    pub p95_latency_ms: u128,
    /// p99 latency in milliseconds
    pub p99_latency_ms: u128,
}

impl Default for AggregatedMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            success_rate: 0.0,
            total_tokens: 0,
            total_cost_usd: 0.0,
            avg_duration_ms: 0.0,
            p50_latency_ms: 0,
            p95_latency_ms: 0,
            p99_latency_ms: 0,
        }
    }
}

/// Metrics collector for LLM operations.
pub struct MetricsCollector {
    metrics: Arc<RwLock<Vec<RequestMetrics>>>,
    max_history: usize,
}

impl MetricsCollector {
    /// Creates a new metrics collector.
    pub fn new(max_history: usize) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(Vec::new())),
            max_history,
        }
    }

    /// Records a request metric.
    pub async fn record(&self, metric: RequestMetrics) {
        let mut metrics = self.metrics.write().await;
        metrics.push(metric);

        // Trim to max history
        if metrics.len() > self.max_history {
            let excess = metrics.len() - self.max_history;
            metrics.drain(0..excess);
        }
    }

    /// Returns all recorded metrics.
    pub async fn get_all(&self) -> Vec<RequestMetrics> {
        self.metrics.read().await.clone()
    }

    /// Computes aggregated metrics.
    pub async fn aggregate(&self) -> AggregatedMetrics {
        let metrics = self.metrics.read().await;

        if metrics.is_empty() {
            return AggregatedMetrics::default();
        }

        let total_requests = metrics.len() as u64;
        let successful_requests = metrics.iter().filter(|m| m.success).count() as u64;
        let failed_requests = total_requests - successful_requests;
        let success_rate = successful_requests as f64 / total_requests as f64;

        let total_tokens: u64 = metrics
            .iter()
            .filter_map(|m| m.tokens.as_ref().map(|t| t.total_tokens as u64))
            .sum();

        let total_cost_usd: f64 = metrics.iter().filter_map(|m| m.cost_usd).sum();

        let avg_duration_ms =
            metrics.iter().map(|m| m.duration_ms as f64).sum::<f64>() / total_requests as f64;

        // Calculate percentiles
        let mut durations: Vec<u128> = metrics.iter().map(|m| m.duration_ms).collect();
        durations.sort_unstable();

        let p50_idx = (durations.len() as f64 * 0.50) as usize;
        let p95_idx = (durations.len() as f64 * 0.95) as usize;
        let p99_idx = (durations.len() as f64 * 0.99) as usize;

        let p50_latency_ms = durations.get(p50_idx).copied().unwrap_or(0);
        let p95_latency_ms = durations.get(p95_idx).copied().unwrap_or(0);
        let p99_latency_ms = durations.get(p99_idx).copied().unwrap_or(0);

        AggregatedMetrics {
            total_requests,
            successful_requests,
            failed_requests,
            success_rate,
            total_tokens,
            total_cost_usd,
            avg_duration_ms,
            p50_latency_ms,
            p95_latency_ms,
            p99_latency_ms,
        }
    }

    /// Clears all recorded metrics.
    pub async fn clear(&self) {
        self.metrics.write().await.clear();
    }

    /// Returns metrics for a specific time window.
    pub async fn get_since(&self, since: chrono::DateTime<chrono::Utc>) -> Vec<RequestMetrics> {
        let metrics = self.metrics.read().await;
        metrics
            .iter()
            .filter(|m| m.timestamp >= since)
            .cloned()
            .collect()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// LLM provider with metrics collection.
pub struct ObservableProvider<P> {
    provider: P,
    collector: Arc<MetricsCollector>,
}

impl<P> ObservableProvider<P> {
    /// Creates a new observable provider.
    pub fn new(provider: P, collector: Arc<MetricsCollector>) -> Self {
        Self {
            provider,
            collector,
        }
    }

    /// Gets the metrics collector.
    pub fn collector(&self) -> Arc<MetricsCollector> {
        self.collector.clone()
    }

    /// Gets the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for ObservableProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let start = Instant::now();
        let result = self.provider.generate_text(prompt).await;
        let duration = start.elapsed();

        let metric = match &result {
            Ok(_) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                true,
            ),
            Err(e) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                false,
            )
            .with_error(e.to_string()),
        };

        self.collector.record(metric).await;
        result
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let start = Instant::now();
        let result = self.provider.generate_structured::<T>(prompt).await;
        let duration = start.elapsed();

        let metric = match &result {
            Ok(_) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                true,
            ),
            Err(e) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                false,
            )
            .with_error(e.to_string()),
        };

        self.collector.record(metric).await;
        result
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let start = Instant::now();
        let result = self.provider.generate_text_stream(prompt).await;

        // Record initial metric (stream started)
        let metric = match &result {
            Ok(_) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                start.elapsed().as_millis(),
                true,
            ),
            Err(e) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                start.elapsed().as_millis(),
                false,
            )
            .with_error(e.to_string()),
        };

        self.collector.record(metric).await;
        result
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

/// Performance timer for tracking operation durations.
pub struct PerformanceTimer {
    start: Instant,
    label: String,
}

impl PerformanceTimer {
    /// Starts a new timer with a label.
    pub fn start(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            label: label.into(),
        }
    }

    /// Returns the elapsed duration.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Stops the timer and returns the elapsed duration.
    pub fn stop(self) -> Duration {
        self.elapsed()
    }

    /// Returns the label.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Prometheus metrics exporter.
///
/// Exports LLM metrics in Prometheus text format.
pub struct PrometheusExporter {
    collector: Arc<MetricsCollector>,
    namespace: String,
}

impl PrometheusExporter {
    /// Creates a new Prometheus exporter.
    pub fn new(collector: Arc<MetricsCollector>) -> Self {
        Self {
            collector,
            namespace: "llm".to_string(),
        }
    }

    /// Sets a custom namespace for metrics (default: "llm").
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    /// Exports metrics in Prometheus text format.
    pub async fn export(&self) -> String {
        let aggregated = self.collector.aggregate().await;
        let mut output = String::new();

        // Total requests counter
        output.push_str(&format!(
            "# HELP {}_requests_total Total number of LLM requests\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_requests_total counter\n",
            self.namespace
        ));
        output.push_str(&format!(
            "{}_requests_total {}\n\n",
            self.namespace, aggregated.total_requests
        ));

        // Successful requests counter
        output.push_str(&format!(
            "# HELP {}_requests_successful_total Total number of successful LLM requests\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_requests_successful_total counter\n",
            self.namespace
        ));
        output.push_str(&format!(
            "{}_requests_successful_total {}\n\n",
            self.namespace, aggregated.successful_requests
        ));

        // Failed requests counter
        output.push_str(&format!(
            "# HELP {}_requests_failed_total Total number of failed LLM requests\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_requests_failed_total counter\n",
            self.namespace
        ));
        output.push_str(&format!(
            "{}_requests_failed_total {}\n\n",
            self.namespace, aggregated.failed_requests
        ));

        // Success rate gauge
        output.push_str(&format!(
            "# HELP {}_success_rate Success rate of LLM requests (0.0 to 1.0)\n",
            self.namespace
        ));
        output.push_str(&format!("# TYPE {}_success_rate gauge\n", self.namespace));
        output.push_str(&format!(
            "{}_success_rate {}\n\n",
            self.namespace, aggregated.success_rate
        ));

        // Total tokens counter
        output.push_str(&format!(
            "# HELP {}_tokens_total Total number of tokens used\n",
            self.namespace
        ));
        output.push_str(&format!("# TYPE {}_tokens_total counter\n", self.namespace));
        output.push_str(&format!(
            "{}_tokens_total {}\n\n",
            self.namespace, aggregated.total_tokens
        ));

        // Total cost counter
        output.push_str(&format!(
            "# HELP {}_cost_usd_total Total cost in USD\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_cost_usd_total counter\n",
            self.namespace
        ));
        output.push_str(&format!(
            "{}_cost_usd_total {}\n\n",
            self.namespace, aggregated.total_cost_usd
        ));

        // Average duration gauge
        output.push_str(&format!(
            "# HELP {}_duration_avg_ms Average request duration in milliseconds\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_duration_avg_ms gauge\n",
            self.namespace
        ));
        output.push_str(&format!(
            "{}_duration_avg_ms {}\n\n",
            self.namespace, aggregated.avg_duration_ms
        ));

        // Latency percentiles summary
        output.push_str(&format!(
            "# HELP {}_duration_ms Request duration in milliseconds\n",
            self.namespace
        ));
        output.push_str(&format!("# TYPE {}_duration_ms summary\n", self.namespace));
        output.push_str(&format!(
            "{}duration_ms{{quantile=\"0.5\"}} {}\n",
            self.namespace, aggregated.p50_latency_ms
        ));
        output.push_str(&format!(
            "{}duration_ms{{quantile=\"0.95\"}} {}\n",
            self.namespace, aggregated.p95_latency_ms
        ));
        output.push_str(&format!(
            "{}duration_ms{{quantile=\"0.99\"}} {}\n\n",
            self.namespace, aggregated.p99_latency_ms
        ));

        output
    }

    /// Exports per-provider metrics in Prometheus text format.
    pub async fn export_by_provider(&self) -> String {
        let all_metrics = self.collector.get_all().await;
        let mut output = String::new();

        // Group metrics by provider
        let mut by_provider: std::collections::HashMap<String, Vec<&RequestMetrics>> =
            std::collections::HashMap::new();

        for metric in &all_metrics {
            by_provider
                .entry(metric.provider.clone())
                .or_default()
                .push(metric);
        }

        // Export per-provider counters
        output.push_str(&format!(
            "# HELP {}_provider_requests_total Total requests per provider\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_provider_requests_total counter\n",
            self.namespace
        ));

        for (provider, metrics) in &by_provider {
            output.push_str(&format!(
                "{}_provider_requests_total{{provider=\"{}\"}} {}\n",
                self.namespace,
                provider,
                metrics.len()
            ));
        }
        output.push('\n');

        // Export per-provider success rates
        output.push_str(&format!(
            "# HELP {}_provider_success_rate Success rate per provider\n",
            self.namespace
        ));
        output.push_str(&format!(
            "# TYPE {}_provider_success_rate gauge\n",
            self.namespace
        ));

        for (provider, metrics) in &by_provider {
            let successful = metrics.iter().filter(|m| m.success).count();
            let success_rate = if metrics.is_empty() {
                0.0
            } else {
                successful as f64 / metrics.len() as f64
            };
            output.push_str(&format!(
                "{}_provider_success_rate{{provider=\"{}\"}} {}\n",
                self.namespace, provider, success_rate
            ));
        }
        output.push('\n');

        output
    }
}

/// OpenTelemetry integration for distributed tracing.
pub mod otel {
    use super::*;
    use anyhow::anyhow;
    use opentelemetry::{
        Context as OtelContext, KeyValue, global,
        trace::{Span, Status, TraceContextExt, Tracer},
    };
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::{
        Resource,
        trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    };
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    /// Configuration for OpenTelemetry integration.
    #[derive(Debug, Clone)]
    pub struct OtelConfig {
        /// Service name for telemetry
        pub service_name: String,
        /// OTLP endpoint (e.g., "http://localhost:4317")
        pub otlp_endpoint: Option<String>,
        /// Sampling rate (0.0 to 1.0)
        pub sampling_rate: f64,
        /// Additional resource attributes
        pub resource_attributes: Vec<(String, String)>,
    }

    impl Default for OtelConfig {
        fn default() -> Self {
            Self {
                service_name: "legalis-llm".to_string(),
                otlp_endpoint: None,
                sampling_rate: 1.0,
                resource_attributes: Vec::new(),
            }
        }
    }

    impl OtelConfig {
        /// Creates a new OpenTelemetry configuration.
        pub fn new(service_name: impl Into<String>) -> Self {
            Self {
                service_name: service_name.into(),
                ..Default::default()
            }
        }

        /// Sets the OTLP endpoint.
        pub fn with_otlp_endpoint(mut self, endpoint: impl Into<String>) -> Self {
            self.otlp_endpoint = Some(endpoint.into());
            self
        }

        /// Sets the sampling rate.
        pub fn with_sampling_rate(mut self, rate: f64) -> Self {
            self.sampling_rate = rate.clamp(0.0, 1.0);
            self
        }

        /// Adds a resource attribute.
        pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.resource_attributes.push((key.into(), value.into()));
            self
        }
    }

    /// OpenTelemetry tracer provider manager.
    pub struct TracerProvider {
        provider: SdkTracerProvider,
    }

    impl TracerProvider {
        /// Initializes the global OpenTelemetry tracer provider.
        pub fn init(config: OtelConfig) -> Result<Self> {
            // Build resource with service name and custom attributes
            let mut resource_kvs = vec![KeyValue::new("service.name", config.service_name.clone())];
            for (key, value) in config.resource_attributes {
                resource_kvs.push(KeyValue::new(key, value));
            }
            let resource = Resource::builder_empty()
                .with_attributes(resource_kvs)
                .build();

            // Configure sampler based on sampling rate
            let sampler = if (config.sampling_rate - 1.0).abs() < f64::EPSILON {
                Sampler::AlwaysOn
            } else if config.sampling_rate.abs() < f64::EPSILON {
                Sampler::AlwaysOff
            } else {
                Sampler::TraceIdRatioBased(config.sampling_rate)
            };

            // Build tracer provider
            let provider_builder = SdkTracerProvider::builder()
                .with_sampler(sampler)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource);

            let provider = if let Some(endpoint) = config.otlp_endpoint {
                // Configure OTLP exporter
                let exporter = opentelemetry_otlp::SpanExporter::builder()
                    .with_http()
                    .with_endpoint(endpoint)
                    .build()
                    .map_err(|e| anyhow!("Failed to build OTLP exporter: {}", e))?;

                provider_builder.with_batch_exporter(exporter).build()
            } else {
                // No exporter, just build provider
                provider_builder.build()
            };

            // Set as global provider
            global::set_tracer_provider(provider.clone());

            Ok(Self { provider })
        }

        /// Shuts down the tracer provider.
        pub fn shutdown(self) -> Result<()> {
            self.provider
                .shutdown()
                .map_err(|e| anyhow!("Failed to shutdown tracer provider: {}", e))
        }

        /// Gets a reference to the underlying provider.
        pub fn provider(&self) -> &SdkTracerProvider {
            &self.provider
        }
    }

    /// Span context for distributed tracing.
    #[derive(Debug, Clone)]
    pub struct SpanContext {
        /// Trace ID
        pub trace_id: String,
        /// Span ID
        pub span_id: String,
        /// Parent span ID
        pub parent_span_id: Option<String>,
    }

    impl SpanContext {
        /// Extracts span context from the current tracing span.
        pub fn current() -> Option<Self> {
            let span = tracing::Span::current();
            let context = span.context();
            let span_ref = context.span();
            let otel_context = span_ref.span_context();

            if otel_context.is_valid() {
                Some(Self {
                    trace_id: otel_context.trace_id().to_string(),
                    span_id: otel_context.span_id().to_string(),
                    parent_span_id: None,
                })
            } else {
                None
            }
        }

        /// Creates a new span context.
        pub fn new(trace_id: impl Into<String>, span_id: impl Into<String>) -> Self {
            Self {
                trace_id: trace_id.into(),
                span_id: span_id.into(),
                parent_span_id: None,
            }
        }

        /// Sets the parent span ID.
        pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
            self.parent_span_id = Some(parent_id.into());
            self
        }
    }

    /// LLM provider with distributed tracing support.
    pub struct TracedProvider<P> {
        provider: P,
        tracer: opentelemetry::global::BoxedTracer,
    }

    impl<P> TracedProvider<P> {
        /// Creates a new traced provider.
        pub fn new(provider: P) -> Self {
            let tracer = global::tracer("legalis-llm");
            Self { provider, tracer }
        }

        /// Creates a new traced provider with a custom tracer name.
        pub fn with_tracer_name(provider: P, tracer_name: impl Into<String>) -> Self {
            let tracer = global::tracer(tracer_name.into());
            Self { provider, tracer }
        }

        /// Gets a reference to the underlying provider.
        pub fn provider(&self) -> &P {
            &self.provider
        }
    }

    #[async_trait]
    impl<P: LLMProvider> LLMProvider for TracedProvider<P> {
        async fn generate_text(&self, prompt: &str) -> Result<String> {
            let span = info_span!(
                "llm.generate_text",
                provider = self.provider.provider_name(),
                model = self.provider.model_name(),
                prompt_length = prompt.len(),
            );

            async move {
                let mut otel_span = self
                    .tracer
                    .start_with_context("llm.generate_text", &OtelContext::current());
                otel_span.set_attribute(KeyValue::new(
                    "llm.provider",
                    self.provider.provider_name().to_string(),
                ));
                otel_span.set_attribute(KeyValue::new(
                    "llm.model",
                    self.provider.model_name().to_string(),
                ));
                otel_span.set_attribute(KeyValue::new("llm.prompt_length", prompt.len() as i64));

                let result = self.provider.generate_text(prompt).await;

                match &result {
                    Ok(response) => {
                        otel_span.set_attribute(KeyValue::new(
                            "llm.response_length",
                            response.len() as i64,
                        ));
                        otel_span.set_status(Status::Ok);
                    }
                    Err(e) => {
                        otel_span.set_attribute(KeyValue::new("error", true));
                        otel_span.set_attribute(KeyValue::new("error.message", e.to_string()));
                        otel_span.set_status(Status::error(e.to_string()));
                    }
                }

                otel_span.end();
                result
            }
            .instrument(span)
            .await
        }

        async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
            &self,
            prompt: &str,
        ) -> Result<T> {
            let span = info_span!(
                "llm.generate_structured",
                provider = self.provider.provider_name(),
                model = self.provider.model_name(),
                prompt_length = prompt.len(),
            );

            async move {
                let mut otel_span = self
                    .tracer
                    .start_with_context("llm.generate_structured", &OtelContext::current());
                otel_span.set_attribute(KeyValue::new(
                    "llm.provider",
                    self.provider.provider_name().to_string(),
                ));
                otel_span.set_attribute(KeyValue::new(
                    "llm.model",
                    self.provider.model_name().to_string(),
                ));
                otel_span.set_attribute(KeyValue::new("llm.prompt_length", prompt.len() as i64));
                otel_span.set_attribute(KeyValue::new("llm.structured", true));

                let result = self.provider.generate_structured::<T>(prompt).await;

                match &result {
                    Ok(_) => {
                        otel_span.set_status(Status::Ok);
                    }
                    Err(e) => {
                        otel_span.set_attribute(KeyValue::new("error", true));
                        otel_span.set_attribute(KeyValue::new("error.message", e.to_string()));
                        otel_span.set_status(Status::error(e.to_string()));
                    }
                }

                otel_span.end();
                result
            }
            .instrument(span)
            .await
        }

        async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
            let span = info_span!(
                "llm.generate_text_stream",
                provider = self.provider.provider_name(),
                model = self.provider.model_name(),
                prompt_length = prompt.len(),
            );

            async move {
                let mut otel_span = self
                    .tracer
                    .start_with_context("llm.generate_text_stream", &OtelContext::current());
                otel_span.set_attribute(KeyValue::new(
                    "llm.provider",
                    self.provider.provider_name().to_string(),
                ));
                otel_span.set_attribute(KeyValue::new(
                    "llm.model",
                    self.provider.model_name().to_string(),
                ));
                otel_span.set_attribute(KeyValue::new("llm.prompt_length", prompt.len() as i64));
                otel_span.set_attribute(KeyValue::new("llm.streaming", true));

                let result = self.provider.generate_text_stream(prompt).await;

                match &result {
                    Ok(_) => {
                        otel_span.set_status(Status::Ok);
                    }
                    Err(e) => {
                        otel_span.set_attribute(KeyValue::new("error", true));
                        otel_span.set_attribute(KeyValue::new("error.message", e.to_string()));
                        otel_span.set_status(Status::error(e.to_string()));
                    }
                }

                otel_span.end();
                result
            }
            .instrument(span)
            .await
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
}

/// Custom metrics dashboard system.
pub mod dashboard {
    use super::*;
    use std::collections::HashMap;

    /// Dashboard widget type.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum WidgetType {
        /// Counter widget showing a single numeric value
        Counter {
            /// Widget title
            title: String,
            /// Metric value
            value: f64,
            /// Optional unit (e.g., "requests", "ms", "USD")
            unit: Option<String>,
        },
        /// Gauge widget showing a value with min/max bounds
        Gauge {
            /// Widget title
            title: String,
            /// Current value
            value: f64,
            /// Minimum value
            min: f64,
            /// Maximum value
            max: f64,
            /// Optional unit
            unit: Option<String>,
        },
        /// Time series chart widget
        TimeSeries {
            /// Widget title
            title: String,
            /// Data points (timestamp, value)
            data: Vec<(chrono::DateTime<chrono::Utc>, f64)>,
            /// Series label
            label: String,
        },
        /// Bar chart widget
        BarChart {
            /// Widget title
            title: String,
            /// Data points (category, value)
            data: Vec<(String, f64)>,
        },
        /// Table widget
        Table {
            /// Widget title
            title: String,
            /// Column headers
            headers: Vec<String>,
            /// Table rows
            rows: Vec<Vec<String>>,
        },
    }

    /// Dashboard layout configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Dashboard {
        /// Dashboard title
        pub title: String,
        /// Dashboard description
        pub description: Option<String>,
        /// Dashboard widgets
        pub widgets: Vec<WidgetType>,
        /// Refresh interval in seconds (for auto-refresh)
        pub refresh_interval: Option<u32>,
        /// Custom metadata
        pub metadata: HashMap<String, String>,
    }

    impl Dashboard {
        /// Creates a new dashboard.
        pub fn new(title: impl Into<String>) -> Self {
            Self {
                title: title.into(),
                description: None,
                widgets: Vec::new(),
                refresh_interval: None,
                metadata: HashMap::new(),
            }
        }

        /// Sets the dashboard description.
        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description = Some(description.into());
            self
        }

        /// Sets the refresh interval.
        pub fn with_refresh_interval(mut self, seconds: u32) -> Self {
            self.refresh_interval = Some(seconds);
            self
        }

        /// Adds a widget to the dashboard.
        pub fn add_widget(mut self, widget: WidgetType) -> Self {
            self.widgets.push(widget);
            self
        }

        /// Adds custom metadata.
        pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.metadata.insert(key.into(), value.into());
            self
        }

        /// Exports the dashboard as JSON.
        pub fn to_json(&self) -> Result<String> {
            serde_json::to_string_pretty(self)
                .map_err(|e| anyhow::anyhow!("JSON serialization error: {}", e))
        }

        /// Exports the dashboard as HTML.
        pub fn to_html(&self) -> String {
            let mut html = String::new();

            html.push_str("<!DOCTYPE html>\n");
            html.push_str("<html lang=\"en\">\n<head>\n");
            html.push_str("<meta charset=\"UTF-8\">\n");
            html.push_str(
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
            );
            html.push_str(&format!("<title>{}</title>\n", self.title));

            // Add basic CSS
            html.push_str("<style>\n");
            html.push_str("body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5; }\n");
            html.push_str(".container { max-width: 1200px; margin: 0 auto; }\n");
            html.push_str("h1 { color: #333; margin-bottom: 10px; }\n");
            html.push_str(".description { color: #666; margin-bottom: 30px; }\n");
            html.push_str(".widgets { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }\n");
            html.push_str(".widget { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
            html.push_str(".widget-title { font-size: 18px; font-weight: bold; margin-bottom: 15px; color: #333; }\n");
            html.push_str(
                ".counter-value { font-size: 36px; font-weight: bold; color: #007bff; }\n",
            );
            html.push_str(".counter-unit { font-size: 14px; color: #666; margin-left: 5px; }\n");
            html.push_str(".gauge { position: relative; height: 150px; }\n");
            html.push_str(".gauge-bar { width: 100%; height: 30px; background: #e0e0e0; border-radius: 15px; overflow: hidden; }\n");
            html.push_str(".gauge-fill { height: 100%; background: linear-gradient(90deg, #28a745, #ffc107, #dc3545); }\n");
            html.push_str(".gauge-value { text-align: center; margin-top: 10px; font-size: 24px; font-weight: bold; }\n");
            html.push_str("table { width: 100%; border-collapse: collapse; }\n");
            html.push_str(
                "th, td { padding: 10px; text-align: left; border-bottom: 1px solid #ddd; }\n",
            );
            html.push_str("th { background-color: #f8f9fa; font-weight: bold; }\n");
            html.push_str(".chart { width: 100%; height: 200px; }\n");
            html.push_str("</style>\n");

            // Add auto-refresh script if configured
            if let Some(interval) = self.refresh_interval {
                html.push_str(&format!(
                    "<script>\nsetTimeout(() => {{ location.reload(); }}, {});\n</script>\n",
                    interval * 1000
                ));
            }

            html.push_str("</head>\n<body>\n");
            html.push_str("<div class=\"container\">\n");
            html.push_str(&format!("<h1>{}</h1>\n", self.title));

            if let Some(desc) = &self.description {
                html.push_str(&format!("<div class=\"description\">{}</div>\n", desc));
            }

            html.push_str("<div class=\"widgets\">\n");

            // Render each widget
            for widget in &self.widgets {
                html.push_str("<div class=\"widget\">\n");

                match widget {
                    WidgetType::Counter { title, value, unit } => {
                        html.push_str(&format!("<div class=\"widget-title\">{}</div>\n", title));
                        html.push_str("<div class=\"counter-value\">");
                        html.push_str(&format!("{:.2}", value));
                        if let Some(u) = unit {
                            html.push_str(&format!("<span class=\"counter-unit\">{}</span>", u));
                        }
                        html.push_str("</div>\n");
                    }
                    WidgetType::Gauge {
                        title,
                        value,
                        min,
                        max,
                        unit,
                    } => {
                        html.push_str(&format!("<div class=\"widget-title\">{}</div>\n", title));
                        html.push_str("<div class=\"gauge\">\n");
                        let percentage = ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                        html.push_str(&format!("<div class=\"gauge-bar\"><div class=\"gauge-fill\" style=\"width: {}%\"></div></div>\n", percentage));
                        html.push_str(&format!("<div class=\"gauge-value\">{:.2}", value));
                        if let Some(u) = unit {
                            html.push_str(&format!(" {}", u));
                        }
                        html.push_str("</div>\n");
                        html.push_str(&format!("<div style=\"text-align: center; color: #666; font-size: 12px;\">Range: {:.2} - {:.2}</div>\n", min, max));
                        html.push_str("</div>\n");
                    }
                    WidgetType::TimeSeries { title, data, label } => {
                        html.push_str(&format!("<div class=\"widget-title\">{}</div>\n", title));
                        html.push_str(&format!(
                            "<div class=\"chart\">Time series chart: {} ({} data points)</div>\n",
                            label,
                            data.len()
                        ));
                    }
                    WidgetType::BarChart { title, data } => {
                        html.push_str(&format!("<div class=\"widget-title\">{}</div>\n", title));
                        html.push_str("<div class=\"chart\">");
                        for (category, value) in data {
                            html.push_str(&format!("<div>{}: {:.2}</div>", category, value));
                        }
                        html.push_str("</div>\n");
                    }
                    WidgetType::Table {
                        title,
                        headers,
                        rows,
                    } => {
                        html.push_str(&format!("<div class=\"widget-title\">{}</div>\n", title));
                        html.push_str("<table>\n<thead><tr>\n");
                        for header in headers {
                            html.push_str(&format!("<th>{}</th>\n", header));
                        }
                        html.push_str("</tr></thead>\n<tbody>\n");
                        for row in rows {
                            html.push_str("<tr>\n");
                            for cell in row {
                                html.push_str(&format!("<td>{}</td>\n", cell));
                            }
                            html.push_str("</tr>\n");
                        }
                        html.push_str("</tbody>\n</table>\n");
                    }
                }

                html.push_str("</div>\n");
            }

            html.push_str("</div>\n");
            html.push_str("</div>\n");
            html.push_str("</body>\n</html>\n");

            html
        }
    }

    /// Dashboard builder for creating dashboards from metrics.
    pub struct DashboardBuilder {
        collector: Arc<MetricsCollector>,
        title: String,
        description: Option<String>,
        refresh_interval: Option<u32>,
    }

    impl DashboardBuilder {
        /// Creates a new dashboard builder.
        pub fn new(collector: Arc<MetricsCollector>, title: impl Into<String>) -> Self {
            Self {
                collector,
                title: title.into(),
                description: None,
                refresh_interval: None,
            }
        }

        /// Sets the dashboard description.
        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description = Some(description.into());
            self
        }

        /// Sets the refresh interval.
        pub fn with_refresh_interval(mut self, seconds: u32) -> Self {
            self.refresh_interval = Some(seconds);
            self
        }

        /// Builds a default dashboard with common metrics.
        pub async fn build_default(&self) -> Dashboard {
            let aggregated = self.collector.aggregate().await;
            let all_metrics = self.collector.get_all().await;

            let mut dashboard = Dashboard::new(&self.title);

            if let Some(desc) = &self.description {
                dashboard = dashboard.with_description(desc);
            }

            if let Some(interval) = self.refresh_interval {
                dashboard = dashboard.with_refresh_interval(interval);
            }

            // Add counter widgets
            dashboard = dashboard
                .add_widget(WidgetType::Counter {
                    title: "Total Requests".to_string(),
                    value: aggregated.total_requests as f64,
                    unit: Some("requests".to_string()),
                })
                .add_widget(WidgetType::Counter {
                    title: "Total Tokens".to_string(),
                    value: aggregated.total_tokens as f64,
                    unit: Some("tokens".to_string()),
                })
                .add_widget(WidgetType::Counter {
                    title: "Total Cost".to_string(),
                    value: aggregated.total_cost_usd,
                    unit: Some("USD".to_string()),
                });

            // Add gauge widgets
            dashboard = dashboard
                .add_widget(WidgetType::Gauge {
                    title: "Success Rate".to_string(),
                    value: aggregated.success_rate,
                    min: 0.0,
                    max: 1.0,
                    unit: None,
                })
                .add_widget(WidgetType::Gauge {
                    title: "Average Latency".to_string(),
                    value: aggregated.avg_duration_ms,
                    min: 0.0,
                    max: 5000.0,
                    unit: Some("ms".to_string()),
                });

            // Add latency table
            let latency_headers = vec!["Metric".to_string(), "Value (ms)".to_string()];
            let latency_rows = vec![
                vec![
                    "p50 Latency".to_string(),
                    format!("{}", aggregated.p50_latency_ms),
                ],
                vec![
                    "p95 Latency".to_string(),
                    format!("{}", aggregated.p95_latency_ms),
                ],
                vec![
                    "p99 Latency".to_string(),
                    format!("{}", aggregated.p99_latency_ms),
                ],
                vec![
                    "Average Latency".to_string(),
                    format!("{:.2}", aggregated.avg_duration_ms),
                ],
            ];
            dashboard = dashboard.add_widget(WidgetType::Table {
                title: "Latency Metrics".to_string(),
                headers: latency_headers,
                rows: latency_rows,
            });

            // Add provider breakdown
            let mut provider_requests: HashMap<String, u64> = HashMap::new();
            for metric in &all_metrics {
                *provider_requests
                    .entry(metric.provider.clone())
                    .or_insert(0) += 1;
            }

            let provider_data: Vec<(String, f64)> = provider_requests
                .into_iter()
                .map(|(k, v)| (k, v as f64))
                .collect();

            if !provider_data.is_empty() {
                dashboard = dashboard.add_widget(WidgetType::BarChart {
                    title: "Requests by Provider".to_string(),
                    data: provider_data,
                });
            }

            // Add time series (requests over time)
            let time_series_data: Vec<(chrono::DateTime<chrono::Utc>, f64)> = all_metrics
                .iter()
                .enumerate()
                .map(|(i, m)| (m.timestamp, i as f64 + 1.0))
                .collect();

            if !time_series_data.is_empty() {
                dashboard = dashboard.add_widget(WidgetType::TimeSeries {
                    title: "Cumulative Requests".to_string(),
                    data: time_series_data,
                    label: "Requests".to_string(),
                });
            }

            dashboard
        }

        /// Builds a custom dashboard.
        pub fn build_custom(self) -> Dashboard {
            let mut dashboard = Dashboard::new(self.title);

            if let Some(desc) = self.description {
                dashboard = dashboard.with_description(desc);
            }

            if let Some(interval) = self.refresh_interval {
                dashboard = dashboard.with_refresh_interval(interval);
            }

            dashboard
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_request_metrics() {
        let metric = RequestMetrics::new("test".to_string(), "model".to_string(), 100, true)
            .with_tokens(TokenUsage::new(10, 20))
            .with_cost(0.05);

        assert_eq!(metric.provider, "test");
        assert_eq!(metric.model, "model");
        assert_eq!(metric.duration_ms, 100);
        assert!(metric.success);
        assert_eq!(metric.tokens.unwrap().total_tokens, 30);
        assert_eq!(metric.cost_usd.unwrap(), 0.05);
    }

    #[tokio::test]
    async fn test_metrics_collector() {
        let collector = MetricsCollector::new(100);

        let metric1 = RequestMetrics::new("provider1".to_string(), "model1".to_string(), 50, true);
        let metric2 = RequestMetrics::new("provider2".to_string(), "model2".to_string(), 100, true);

        collector.record(metric1).await;
        collector.record(metric2).await;

        let all = collector.get_all().await;
        assert_eq!(all.len(), 2);

        collector.clear().await;
        let all = collector.get_all().await;
        assert_eq!(all.len(), 0);
    }

    #[tokio::test]
    async fn test_aggregated_metrics() {
        let collector = MetricsCollector::new(100);

        for i in 0..10 {
            let success = i < 8;
            let metric = RequestMetrics::new(
                "test".to_string(),
                "model".to_string(),
                (i + 1) * 10,
                success,
            );
            collector.record(metric).await;
        }

        let agg = collector.aggregate().await;
        assert_eq!(agg.total_requests, 10);
        assert_eq!(agg.successful_requests, 8);
        assert_eq!(agg.failed_requests, 2);
        assert!((agg.success_rate - 0.8).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_observable_provider() {
        let provider = MockProvider::default();
        let collector = Arc::new(MetricsCollector::new(100));
        let observable = ObservableProvider::new(provider, collector.clone());

        let result = observable.generate_text("test").await;
        assert!(result.is_ok());

        let metrics = collector.get_all().await;
        assert_eq!(metrics.len(), 1);
        assert!(metrics[0].success);
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::start("test operation");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.stop();

        assert!(elapsed.as_millis() >= 10);
    }

    #[tokio::test]
    async fn test_metrics_time_window() {
        let collector = MetricsCollector::new(100);

        let now = chrono::Utc::now();
        let past = now - chrono::Duration::hours(1);

        let metric1 = RequestMetrics::new("test".to_string(), "model".to_string(), 10, true);
        collector.record(metric1).await;

        // Get metrics from the past hour
        let recent = collector.get_since(past).await;
        assert_eq!(recent.len(), 1);

        // Get metrics from the future (should be empty)
        let future = collector.get_since(now + chrono::Duration::hours(1)).await;
        assert_eq!(future.len(), 0);
    }

    #[tokio::test]
    async fn test_prometheus_export() {
        let collector = Arc::new(MetricsCollector::new(100));

        // Add some test metrics
        for i in 0..5 {
            let metric = RequestMetrics::new(
                "openai".to_string(),
                "gpt-4".to_string(),
                (i + 1) * 100,
                true,
            );
            collector.record(metric).await;
        }

        let exporter = PrometheusExporter::new(collector);
        let output = exporter.export().await;

        // Verify output contains expected Prometheus format
        assert!(output.contains("# HELP llm_requests_total"));
        assert!(output.contains("# TYPE llm_requests_total counter"));
        assert!(output.contains("llm_requests_total 5"));
        assert!(output.contains("llm_requests_successful_total 5"));
        assert!(output.contains("llm_success_rate 1"));
    }

    #[tokio::test]
    async fn test_prometheus_export_custom_namespace() {
        let collector = Arc::new(MetricsCollector::new(100));

        let metric = RequestMetrics::new("test".to_string(), "model".to_string(), 100, true);
        collector.record(metric).await;

        let exporter = PrometheusExporter::new(collector).with_namespace("custom");
        let output = exporter.export().await;

        assert!(output.contains("# HELP custom_requests_total"));
        assert!(output.contains("custom_requests_total 1"));
    }

    #[tokio::test]
    async fn test_prometheus_export_by_provider() {
        let collector = Arc::new(MetricsCollector::new(100));

        // Add metrics for different providers
        collector
            .record(RequestMetrics::new(
                "openai".to_string(),
                "gpt-4".to_string(),
                100,
                true,
            ))
            .await;
        collector
            .record(RequestMetrics::new(
                "anthropic".to_string(),
                "claude".to_string(),
                200,
                true,
            ))
            .await;
        collector
            .record(RequestMetrics::new(
                "openai".to_string(),
                "gpt-3.5".to_string(),
                150,
                false,
            ))
            .await;

        let exporter = PrometheusExporter::new(collector);
        let output = exporter.export_by_provider().await;

        // Verify per-provider metrics
        assert!(output.contains("llm_provider_requests_total{provider=\"openai\"} 2"));
        assert!(output.contains("llm_provider_requests_total{provider=\"anthropic\"} 1"));
        assert!(output.contains("llm_provider_success_rate{provider=\"openai\"} 0.5"));
        assert!(output.contains("llm_provider_success_rate{provider=\"anthropic\"} 1"));
    }

    #[test]
    fn test_dashboard_creation() {
        use super::dashboard::*;

        let dashboard = Dashboard::new("Test Dashboard")
            .with_description("A test dashboard")
            .with_refresh_interval(30)
            .add_widget(WidgetType::Counter {
                title: "Test Counter".to_string(),
                value: 42.0,
                unit: Some("requests".to_string()),
            });

        assert_eq!(dashboard.title, "Test Dashboard");
        assert_eq!(dashboard.description, Some("A test dashboard".to_string()));
        assert_eq!(dashboard.refresh_interval, Some(30));
        assert_eq!(dashboard.widgets.len(), 1);
    }

    #[test]
    fn test_dashboard_widgets() {
        use super::dashboard::*;

        let dashboard = Dashboard::new("Widgets Test")
            .add_widget(WidgetType::Counter {
                title: "Counter".to_string(),
                value: 100.0,
                unit: Some("items".to_string()),
            })
            .add_widget(WidgetType::Gauge {
                title: "Gauge".to_string(),
                value: 0.75,
                min: 0.0,
                max: 1.0,
                unit: None,
            })
            .add_widget(WidgetType::BarChart {
                title: "Bar Chart".to_string(),
                data: vec![("A".to_string(), 10.0), ("B".to_string(), 20.0)],
            })
            .add_widget(WidgetType::Table {
                title: "Table".to_string(),
                headers: vec!["Name".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Item 1".to_string(), "100".to_string()],
                    vec!["Item 2".to_string(), "200".to_string()],
                ],
            });

        assert_eq!(dashboard.widgets.len(), 4);
    }

    #[test]
    fn test_dashboard_to_json() {
        use super::dashboard::*;

        let dashboard = Dashboard::new("JSON Test").add_widget(WidgetType::Counter {
            title: "Test".to_string(),
            value: 123.0,
            unit: None,
        });

        let json = dashboard.to_json().unwrap();
        assert!(json.contains("JSON Test"));
        assert!(json.contains("Test"));
        assert!(json.contains("123"));
    }

    #[test]
    fn test_dashboard_to_html() {
        use super::dashboard::*;

        let dashboard = Dashboard::new("HTML Test")
            .with_description("Test HTML generation")
            .add_widget(WidgetType::Counter {
                title: "Test Counter".to_string(),
                value: 42.0,
                unit: Some("units".to_string()),
            });

        let html = dashboard.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("HTML Test"));
        assert!(html.contains("Test HTML generation"));
        assert!(html.contains("Test Counter"));
        assert!(html.contains("42.00"));
    }

    #[tokio::test]
    async fn test_dashboard_builder() {
        use super::dashboard::*;

        let collector = Arc::new(MetricsCollector::new(100));

        // Add some test metrics
        for i in 0..10 {
            let metric = RequestMetrics::new(
                "test_provider".to_string(),
                "test_model".to_string(),
                (i + 1) * 100,
                i < 8,
            );
            collector.record(metric).await;
        }

        let builder = DashboardBuilder::new(collector, "Test Builder Dashboard")
            .with_description("Built from metrics")
            .with_refresh_interval(60);

        let dashboard = builder.build_default().await;

        assert_eq!(dashboard.title, "Test Builder Dashboard");
        assert_eq!(
            dashboard.description,
            Some("Built from metrics".to_string())
        );
        assert_eq!(dashboard.refresh_interval, Some(60));
        assert!(!dashboard.widgets.is_empty());
    }

    #[tokio::test]
    async fn test_dashboard_builder_custom() {
        use super::dashboard::*;

        let collector = Arc::new(MetricsCollector::new(100));

        let builder = DashboardBuilder::new(collector, "Custom Dashboard")
            .with_description("Custom builder test");

        let dashboard = builder.build_custom();

        assert_eq!(dashboard.title, "Custom Dashboard");
        assert_eq!(
            dashboard.description,
            Some("Custom builder test".to_string())
        );
        assert_eq!(dashboard.widgets.len(), 0);
    }
}
