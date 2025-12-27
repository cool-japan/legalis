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
//!
//! ## Advanced Features (2025-Q4)
//! - Monte Carlo simulation for probabilistic analysis
//! - Economic modeling (tax revenue, compliance costs, cost-benefit analysis)
//! - Network effects and social influence modeling
//! - Policy optimization (gradient-free, multi-objective)
//! - Calibration and validation tools
//! - Impact assessment and equity analysis
//! - Event-driven simulation and hybrid approaches
//! - Risk analysis (VaR, CVaR, risk metrics, tail risk)
//! - Portfolio analysis (efficient frontier, diversification, correlation)
//! - Scenario planning (scenario trees, probability weighting, sensitivity)
//! - Forecasting (linear trends, moving average, exponential smoothing)

mod analysis;
mod behavior;
mod builder;
mod calibration;
mod comparison;
mod economic;
mod engine;
mod error;
mod event_driven;
mod forecasting;
mod impact;
mod incremental;
mod metrics;
mod monte_carlo;
mod network_effects;
mod optimization;
mod performance;
mod population;
mod portfolio;
mod relationships;
mod risk;
mod scenarios;
mod stress_tests;
mod temporal;
mod utils;
mod visualization;

pub use analysis::*;
pub use behavior::*;
pub use builder::*;
pub use calibration::*;
pub use comparison::*;
pub use economic::*;
pub use engine::*;
pub use error::*;
pub use event_driven::*;
pub use forecasting::*;
pub use impact::*;
pub use incremental::*;
pub use metrics::*;
pub use monte_carlo::*;
pub use network_effects::*;
pub use optimization::*;
pub use performance::*;
pub use population::*;
pub use portfolio::*;
pub use relationships::*;
pub use risk::*;
pub use scenarios::*;
pub use temporal::*;
pub use utils::*;
pub use visualization::*;
