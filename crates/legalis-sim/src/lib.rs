//! Legalis-Sim: Simulation engine for Legalis-RS.
//!
//! This crate provides an ECS-like simulation engine for testing
//! legal statutes against populations of agents.
//!
//! # Features
//!
//! ## Core Simulation
//! - Population-based simulation with parallel execution
//! - Realistic demographic population generation
//! - Time-step based temporal simulation
//! - Retroactive law application
//! - Statute effective date handling
//! - Agent lifecycle management
//! - Behavioral modeling with compliance probability
//! - Inter-agent relationships and organizations
//! - Property and asset management
//!
//! ## Analysis & Comparison
//! - Statute comparison and A/B testing
//! - Statistical analysis (distribution, correlation, time-series)
//! - Comprehensive metrics collection
//! - Sensitivity analysis and cohort tracking
//!
//! ## Visualization
//! - GraphViz DOT format export for relationship graphs
//! - D3.js compatible JSON output for interactive visualizations
//! - Geographic data export for map visualizations
//! - Interactive dashboard data generation
//!
//! ## Performance & Scalability
//! - Batch processing for large populations
//! - Memory-efficient streaming mode
//! - Entity pooling and recycling
//! - Lazy attribute evaluation
//! - Optimized work distribution across threads
//! - Parallel execution with work-stealing scheduler
//! - Memory-mapped population storage
//!
//! ## Incremental Simulation
//! - Dirty tracking for efficient re-simulation
//! - Delta-based updates for change tracking
//! - Checkpoint and restore functionality
//! - Simulation replay for debugging
//!
//! ## Testing & Validation
//! - Stress testing for memory limits
//! - Simulation verification tests
//! - Reproducible random testing

mod analysis;
mod behavior;
mod builder;
mod comparison;
mod engine;
mod error;
mod incremental;
mod metrics;
mod performance;
mod population;
mod relationships;
mod stress_tests;
mod temporal;
mod utils;
mod visualization;

pub use analysis::*;
pub use behavior::*;
pub use builder::*;
pub use comparison::*;
pub use engine::*;
pub use error::*;
pub use incremental::*;
pub use metrics::*;
pub use performance::*;
pub use population::*;
pub use relationships::*;
pub use temporal::*;
pub use utils::*;
pub use visualization::*;
