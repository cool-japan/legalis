//! # Legalis-API: REST, GraphQL and gRPC Server for Legalis-RS
//!
//! `legalis-api` is the HTTP gateway layer for the
//! [Legalis-RS](https://github.com/cool-japan/legalis) legal framework.
//! It exposes every core capability — statute CRUD, formal verification,
//! simulation, registry queries and governance reporting — over three transport
//! protocols: REST (Axum), GraphQL, and gRPC (optional, `grpc` feature).
//!
//! ## Features
//!
//! | Category | What is included |
//! |----------|-----------------|
//! | Statute management | Full CRUD, versioning, batch operations, field selection |
//! | Verification | Single and batch verification, async jobs, detailed diagnostics |
//! | Simulation | Run legal simulations, save/restore runs, what-if analysis |
//! | GraphQL | Schema stitching, persisted queries, live queries, batching |
//! | gRPC | Reflection, health checks, server-streaming (`grpc` feature) |
//! | OpenAPI | Auto-generated OpenAPI 3.0 specification at `/api/openapi.json` |
//! | Auth | JWT Bearer tokens, API-key auth, RBAC + ReBAC |
//! | Rate limiting | Fixed-window, adaptive intelligent limiting, abuse detection |
//! | Observability | Structured logging, OpenTelemetry tracing, Prometheus metrics, SLOs |
//! | Compliance | Consent ledger, data-classification registry, audit export, GDPR/SOX |
//! | Caching | In-memory (default) or Redis (`redis-cache` feature), predictive prefetch |
//! | Real-time | WebSocket push, Server-Sent Events, collaborative editing, presence |
//!
//! ## Architecture
//!
//! ```text
//! HTTP / gRPC client
//!        │
//!        ▼
//! ┌──────────────────────────────────────────────┐
//! │          Axum Router  (TLS optional)          │
//! │  ┌──────────┐  ┌───────────┐  ┌───────────┐ │
//! │  │  REST    │  │  GraphQL  │  │   gRPC    │ │
//! │  └────┬─────┘  └─────┬─────┘  └─────┬─────┘ │
//! │       └──────────────┼───────────────┘       │
//! │              ┌───────▼────────┐               │
//! │              │   AppState     │               │
//! │              │  (Arc-wrapped) │               │
//! └──────────────┴───────┬────────┴───────────────┘
//!                        │
//!           ┌────────────┴────────────┐
//!           │                         │
//!    legalis-core             legalis-verifier
//!    legalis-registry         legalis-sim
//!    legalis-audit            legalis-llm
//! ```
//!
//! [`AppState`] holds the shared in-memory statute store, ReBAC engine,
//! async job manager, cache, WebSocket broadcaster, audit log, and all
//! governance sub-systems.  It is created once and injected into every
//! handler via Axum's `State` extractor.
//!
//! ## Quick Start
//!
//! ```no_run
//! use legalis_api::{create_router, AppState};
//! use legalis_api::config::Config;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load configuration from environment variables (falls back to defaults)
//!     let config = Config::from_env();
//!
//!     // Initialise shared application state
//!     let state = Arc::new(AppState::new());
//!
//!     // Build the Axum router with all registered routes
//!     let app = create_router(state);
//!
//!     // Bind and serve
//!     let addr = format!("{}:{}", config.host, config.port);
//!     let listener = tokio::net::TcpListener::bind(&addr).await?;
//!     axum::serve(listener, app).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## API Endpoints
//!
//! All REST endpoints are prefixed with `/api/v1`.
//!
//! ### Statutes
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | `GET` | `/api/v1/statutes` | List / search statutes |
//! | `POST` | `/api/v1/statutes` | Create a statute |
//! | `GET` | `/api/v1/statutes/:id` | Fetch a statute by ID |
//! | `PUT` | `/api/v1/statutes/:id` | Update a statute |
//! | `DELETE` | `/api/v1/statutes/:id` | Delete a statute |
//! | `POST` | `/api/v1/statutes/batch` | Batch-create statutes |
//!
//! ### Verification
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | `POST` | `/api/v1/verify` | Verify statutes synchronously |
//! | `POST` | `/api/v1/verify/batch` | Launch an async batch-verify job |
//! | `GET` | `/api/v1/verify/jobs/:id` | Poll async verification job status |
//!
//! ### Discovery
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | `GET` | `/api/openapi.json` | OpenAPI 3.0 specification |
//! | `GET` | `/health` | Health check |
//! | `GET` | `/metrics` | Prometheus metrics |
//!
//! ## Configuration
//!
//! [`config::Config`] is populated via [`config::Config::from_env`] or
//! `Config::default()`.  The relevant environment variables use the
//! `LEGALIS_API_` prefix:
//!
//! | Variable | Default | Description |
//! |----------|---------|-------------|
//! | `LEGALIS_API_HOST` | `127.0.0.1` | Bind address |
//! | `LEGALIS_API_PORT` | `3000` | TCP port |
//! | `LEGALIS_API_LOG_LEVEL` | `info` | `trace` / `debug` / `info` / `warn` / `error` |
//! | `LEGALIS_API_CORS_ORIGINS` | *(none)* | Comma-separated allowed origins |
//! | `LEGALIS_API_STRUCTURED_LOGGING` | `false` | Emit JSON log lines when `true` |
//! | `LEGALIS_API_MAX_BODY_SIZE` | `10485760` | Max request body in bytes |
//!
//! ## Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `grpc` *(default)* | gRPC transport via Tonic with reflection and health checks |
//! | `redis-cache` | Swap in-memory [`cache`] for a Redis-backed store |
//! | `oauth2-auth` | OAuth2 provider endpoints (requires `oauth2` + `reqwest`) |
//! | `otel-tracing` | Export spans to an OpenTelemetry collector via OTLP |
//!
//! ## Security
//!
//! Authentication is handled by the [`auth`] module.  Every protected route
//! requires a valid **JWT Bearer token** (from `POST /api/v1/auth/login`) or
//! an **API key** (from `POST /api/v1/auth/api-keys`).
//!
//! Authorization follows two complementary models:
//!
//! - **RBAC** — coarse-grained roles ([`auth::Role`]): `Admin`, `Editor`, `Viewer`.
//! - **ReBAC** ([`rebac`]) — fine-grained, relationship-based permissions per statute.
//!
//! Additional hardening layers:
//!
//! - [`security_headers`] — HSTS, `X-Frame-Options`, Content Security Policy
//! - [`ip_whitelist`] — allowlist-only IP filtering
//! - [`rate_limit`] / [`intelligent_rate_limit`] — sliding-window and adaptive limits
//! - [`abuse_detection`] — anomalous request-pattern detection

pub mod ai_suggestions;
pub mod anomaly;
pub mod async_jobs;
pub mod audit;
pub mod auth;
pub mod cache;
pub mod collaborative;
pub mod config;
pub mod contract_test;
pub mod dataloader;
pub mod edge_cache;
pub mod field_selection;
pub mod gateway;
// Advanced Security (v0.2.7)
pub mod ip_whitelist;
pub mod key_rotation;
pub mod security_headers;
// Performance Optimization (v0.2.8)
pub mod pagination;
pub mod partial_response;
pub mod prefetch;
pub mod streaming;
// Compliance & Governance (v0.2.9)
pub mod audit_export;
pub mod consent;
pub mod data_classification;
pub mod governance_routes;
pub mod regulatory_reporting;
pub mod usage_policy;
// AI-Powered API (v0.3.0) — pure-Rust algorithmic
pub mod abuse_detection;
pub mod graphql;
#[cfg(feature = "grpc")]
pub mod grpc;
pub mod intelligent_rate_limit;
pub mod live_queries;
pub mod load_test;
pub mod logging;
mod metrics;
pub mod multitenancy;
pub mod oauth2_provider;
pub mod observability;
pub mod openapi;
pub mod persisted_queries;
pub mod predictive_cache;
pub mod presence;
pub mod query_batch;
pub mod query_cost;
pub mod rate_limit;
pub mod rebac;
pub mod sampling;
pub mod schema_stitching;
pub mod security;
pub mod slo;
pub mod telemetry;
pub mod versioning;
pub mod websocket;

// Event-Driven Architecture (v0.2.4)
pub mod cqrs;
pub mod event_replay;
pub mod event_schema;
pub mod event_sourcing;
pub mod event_streaming;

// Developer Experience (v0.2.5)
pub mod changelog;
pub mod mocking;
pub mod playground;
pub mod sdk_generator;
pub mod sdk_notifications;
pub mod test_utils;

mod functions;
mod functions_2;
mod trait_impls;
mod types;

pub use functions::*;
pub use types::*;
