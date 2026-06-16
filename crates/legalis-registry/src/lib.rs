//! Legalis-Registry: Statute registry and version management for Legalis-RS.
//!
//! This crate provides a central registry for managing statute collections:
//! - Version control for statutes
//! - Hierarchical statute organization
//! - Cross-reference management
//! - Amendment tracking
//! - LRU caching for performance
//! - Full-text search capabilities
//! - Fuzzy matching for statute IDs
//! - Pagination support

use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

mod functions;
mod functions_10;
mod functions_11;
mod functions_2;
mod functions_3;
mod functions_4;
mod functions_5;
mod functions_6;
mod functions_7;
mod functions_8;
mod trait_impls;
mod types;
mod types_3;
mod types_4;
mod types_4_impl;
mod types_5;
mod types_6;
mod types_7;
mod types_8;

pub use functions::*;
pub use functions_2::*;
pub use types::*;
pub use types_3::*;
pub use types_4::*;
pub use types_5::*;
pub use types_6::*;
pub use types_7::*;
pub use types_8::*;

/// Errors during registry operations.
/// A backup of the registry state.
/// Metadata for a registry backup.
/// A point-in-time snapshot of the registry.
/// Incremental backup containing only changes since last backup.
/// Lazy loading configuration.
/// Statistics about the registry.
/// Lightweight statute summary for lazy loading.
/// Pagination parameters.
/// Paginated result.
/// Search query for statutes.
/// A search result with relevance scoring.
/// Ranking configuration for search results.
/// A versioned statute entry.
/// Status of a statute.
/// Events that can occur in the registry.
/// Event store for tracking all changes.
/// Webhook subscription.
/// Filter for webhook events.
/// Webhook manager for event notifications.
/// Calculates relevance score for a statute entry.
/// Helper to extract timestamp from an event.
/// Exports aggregation result to CSV format (feature-gated).
/// Cached analytics with timestamp for TTL.
/// Allows managing multiple isolated registries for different tenants.
/// Statistics for a tenant.
/// Archive entry for a statute.
/// Archive for storing removed or superseded statutes.
/// Retention policy rule for auto-archiving statutes.
/// Configuration for retention policies.
/// Result of applying retention policies.
/// Dependency graph for a statute.
/// Temporal analytics for tracking registry growth and changes over time.
/// Relationship analytics for analyzing statute dependencies and supersession chains.
/// Tag analytics for analyzing tag usage patterns.
/// Activity analytics for tracking modification patterns.
/// Field projection options for efficient queries.
/// Aggregation functions for grouping and counting.
// =============================================================================
#[cfg(feature = "async")]
pub mod async_api;

// =============================================================================
#[cfg(all(feature = "async", feature = "async-stream"))]
pub mod streaming;

// =============================================================================
pub mod transaction;

// =============================================================================
#[cfg(feature = "akoma-ntoso")]
pub mod akoma_ntoso;

// =============================================================================
#[cfg(any(feature = "sqlite", feature = "postgres"))]
pub mod storage;

// =============================================================================
#[cfg(feature = "graphql")]
pub mod graphql;

/// Represents a change in a field between two statute versions.
/// Represents differences between two statute entries.
/// A validation error.
/// Validates that statute ID is not empty.
/// Validates that title is not empty.
/// Validates that jurisdiction is valid.
/// Validates that effective and expiry dates are logical.
/// Validates that tags are not empty and unique.
/// A collection of validation rules.
/// Operation metrics for the registry.
/// Strategy for resolving conflicts during merge.
/// A conflict that occurred during merge.
/// Result of a merge operation.
// ============================================================================
// ============================================================================
// ============================================================================
/// Result of batch validation.
/// Cache entry with TTL (Time To Live).
/// Search cache configuration.
/// Audit log entry capturing detailed operation information.
/// Types of auditable operations.
/// Result of an audited operation.
/// Audit trail manager for tracking all operations.
/// Health status of the registry.
/// Comprehensive health check result.
/// Health status of individual components.
/// Difference between two registries.
/// Details of differences in a specific statute.
/// Configuration for bulk operations.
/// Result of a bulk operation.
/// Performance benchmark result.
/// Benchmark suite for registry operations.
/// Rate limit configuration.
/// Rate limiter for protecting against abuse.
/// Circuit breaker state.
/// Circuit breaker configuration.
/// Circuit breaker for fault tolerance.
/// Log level for observability.
/// Structured log entry.
/// Metric type for observability.
/// Metric entry for observability.
/// Observability collector for logs and metrics.
/// Quality score for a statute entry (0.0 - 100.0).
/// Quality assessment for a statute entry.
/// Similarity measure between two statutes.
/// A potential duplicate statute pair.
/// Result of duplicate detection.
/// Data profile for a field in the registry.
/// Comprehensive data profile for the registry.
/// Enrichment suggestion for a statute entry.
/// Type of data enrichment.
/// Result of automatic enrichment analysis.
/// Enrichment configuration.
/// Type of lineage operation.
/// Lineage entry tracking data provenance.
/// Data lineage tracker.
/// Records a lineage entry for a statute.
/// PII (Personally Identifiable Information) field types.
/// A detected PII instance in statute content.
/// Result of PII detection scan.
/// PII masking strategy.
/// PII detector and handler.
/// Data retention rule for automatic cleanup.
/// Data retention configuration.
/// Retention rules
/// Auto-apply retention rules
/// Dry-run mode (don't actually delete)
/// Result of applying retention rules.
/// Audit report format.
/// Audit report configuration.
/// Generated audit report.
/// Creates a new audit report.
/// Geographic region for data sovereignty.
/// Data sovereignty configuration.
/// Compliance dashboard metrics.
/// Creates a new compliance dashboard.
/// Permission types for statute operations.
/// User role in the system.
/// Attribute-based access control condition.
/// Access control policy.
/// Temporary access grant.
/// User with access control attributes.
/// Access control manager.
/// Registered users
/// Access policies
/// Temporary access grants
/// Enable/disable access control
/// Government database import configuration and execution.
pub mod government_import;

/// Scheduled synchronization for periodic imports.
pub mod sync;

/// Format migration utilities.
pub mod migration;

/// Export templates for reporting.
pub mod templates;

/// Approval workflows for statute changes.
pub mod workflow;

/// Notification system for stakeholders.
pub mod notifications;

/// Task assignment for reviews.
pub mod tasks;

/// SLA tracking for approvals.
pub mod sla;

/// Escalation rules.
pub mod escalation;

/// Advanced search features.
pub mod advanced_search;

/// Version control features for Git-like statute management.
pub mod version_control;

/// API Extensions for high-performance and real-time features.
pub mod api_extensions;

// ============================================================================
pub mod distributed;
// ============================================================================
pub mod vector_search;
// ============================================================================
pub mod blockchain;
// ============================================================================
pub mod graph_db;
// ============================================================================
pub mod multi_tenant;
// ============================================================================
pub mod ai_features;
/// Autonomous Registry Management module (v0.3.1)
pub mod autonomous;
/// Event Sourcing 2.0: Advanced event replay, projections, and archiving.
pub mod event_sourcing_v2;
/// Global Registry Network module (v0.3.0)
pub mod global_network;
/// case law cross-references, knowledge graph visualization, and AI-powered legal research.
pub mod knowledge_base;

/// Federation Protocol: Multi-registry federation and cross-registry queries.
pub mod federation;

#[cfg(test)]
mod tests;

/// - Change stream subscriptions
pub mod realtime;

/// - Field-level encryption
pub mod enterprise_security;

/// Regulatory Sandbox module (v0.3.3): isolated statute simulation
/// environments, impact prediction, A/B testing of statute variants,
/// regulatory experiment tracking, and rollback-safe statute testing.
pub mod sandbox;

/// Quantum-Safe Registry module (v0.3.4): post-quantum hash-based signatures for
/// statute entries and versions, quantum-resistant content hashing for the
/// registry store, a BB84-style QKD key-agreement model, hybrid classical+PQ
/// signature envelopes, and quantum-safe audit-trail verification over the
/// registry's event log.
pub mod quantum_safe;
