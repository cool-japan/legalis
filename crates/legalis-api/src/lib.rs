//! Legalis-API: REST/GraphQL API server for Legalis-RS.
//!
//! This crate provides a web API for interacting with the Legalis-RS framework:
//! - CRUD operations for statutes
//! - Verification endpoints
//! - Simulation endpoints
//! - Registry queries
//! - OpenAPI 3.0 documentation
//! - Authentication and authorization (RBAC + ReBAC)

pub mod ai_suggestions;
pub mod anomaly;
pub mod async_jobs;
pub mod audit;
pub mod auth;
pub mod cache;
pub mod collaborative;
pub mod config;
pub mod contract_test;
// pub mod dataloader; // TODO: Re-enable when Loader trait signature issues are resolved
pub mod edge_cache;
pub mod field_selection;
pub mod gateway;
pub mod graphql;
#[cfg(feature = "grpc")]
pub mod grpc;
pub mod live_queries;
pub mod load_test;
pub mod logging;
mod metrics;
pub mod multitenancy;
pub mod oauth2_provider;
pub mod observability;
pub mod openapi;
pub mod persisted_queries;
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
