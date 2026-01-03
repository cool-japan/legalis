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
//! ## Persistence & Recovery
//! - File-based checkpoint persistence (save/load to disk)
//! - Resume from failure detection and recovery
//! - Automatic periodic checkpointing
//! - Checkpoint validation and integrity checking
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
//! - Agent Intelligence (reinforcement learning, game theory, BDI, bounded rationality)
//! - Demographic Modeling (census data, mortality/fertility rates, migration, households, income mobility)
//! - Policy Analysis (multi-objective optimization, sensitivity analysis, stakeholder impacts, distributional analysis)
//! - Validation Framework (empirical validation, cross-validation, confidence intervals, uncertainty quantification)
//! - Domain-Specific Models (tax systems, benefit eligibility, regulatory compliance, court case prediction, legislative forecasting)
//!
//! ## GPU Acceleration (v0.2.0)
//! - CUDA backend for NVIDIA GPU acceleration
//! - OpenCL backend for cross-platform GPU support
//! - WebGPU backend for browser-based simulations
//! - GPU-optimized condition evaluation kernels
//! - Tensor-based population representations
//! - Memory pooling for efficient GPU memory management
//!
//! ## Distributed Simulation (v0.2.1)
//! - Multi-node simulation framework with message passing
//! - Partition-based entity distribution strategies
//! - Cross-node communication abstractions
//! - Dynamic load balancing with multiple strategies
//! - Fault-tolerant coordination and synchronization
//!
//! ## Agent-Based Modeling 2.0 (v0.2.2)
//! - Deep reinforcement learning (DQN, Actor-Critic)
//! - Multi-agent coordination protocols (Contract Net, AMAS)
//! - Emergent behavior detection
//! - Social network dynamics
//! - Cultural evolution modeling
//!
//! ## Real-Time Simulation (v0.2.3)
//! - Streaming simulation updates
//! - Live parameter adjustment
//! - Real-time visualization integration
//! - Simulation pause/resume/rewind
//! - Breakpoint debugging
//!
//! ## Synthetic Data Generation (v0.2.4)
//! - GAN-based entity generation
//! - Privacy-preserving synthetic populations (differential privacy)
//! - Demographic-consistent data synthesis
//! - Realistic income/wealth distributions (log-normal, Pareto, exponential)
//! - Geographic-aware population generation with clustering
//!
//! ## Economic Simulation Extensions (v0.2.5)
//! - DSGE (Dynamic Stochastic General Equilibrium) models
//! - Input-output economic modeling (Leontief matrices)
//! - Financial contagion simulation with network effects
//! - Market microstructure modeling (order books, market depth)
//! - Behavioral economics (prospect theory, hyperbolic discounting, anchoring)
//!
//! ## Integration & API (2025-Q4)
//! - Simulation-as-a-Service API with job queuing
//! - Persistent result storage with file-based backend
//! - Comparison API for analyzing multiple simulations
//! - Webhook notifications for job completion
//! - Priority-based job scheduling
//!
//! ## Orchestration & Advanced Job Management (2025-Q4)
//! - Job retry logic with exponential/linear backoff
//! - Job timeout handling with configurable actions
//! - Batch job execution with dependency graphs
//! - Parameter sweep orchestration for sensitivity analysis
//! - Execution history tracking and statistics

mod agent_based_2;
mod agent_intelligence;
mod analysis;
mod api;
mod behavior;
mod builder;
mod calibration;
mod comparison;
mod demographic_modeling;
mod distributed;
mod domain_models;
mod economic;
mod economic_extensions;
mod engine;
mod error;
mod event_driven;
mod forecasting;
mod gpu;
mod impact;
mod incremental;
mod metrics;
mod monte_carlo;
mod network_effects;
mod optimization;
mod orchestration;
mod performance;
mod persistence;
mod policy_analysis;
mod population;
mod portfolio;
mod realtime;
mod relationships;
mod risk;
mod scenarios;
mod stress_tests;
mod synthetic_data;
mod temporal;
mod utils;
mod validation;
mod visualization;

pub use agent_based_2::*;
pub use agent_intelligence::*;
pub use analysis::*;
pub use api::*;
pub use behavior::*;
pub use builder::*;
pub use calibration::*;
pub use comparison::*;
pub use demographic_modeling::*;
pub use distributed::*;
pub use domain_models::*;
pub use economic::*;
pub use economic_extensions::*;
pub use engine::*;
pub use error::*;
pub use event_driven::*;
pub use forecasting::*;
pub use gpu::*;
pub use impact::*;
pub use incremental::*;
pub use metrics::*;
pub use monte_carlo::*;
pub use network_effects::*;
pub use optimization::*;
pub use orchestration::*;
pub use performance::*;
pub use persistence::*;
pub use policy_analysis::*;
pub use population::*;
pub use portfolio::*;
pub use realtime::*;
pub use relationships::*;
pub use risk::*;
pub use scenarios::*;
pub use synthetic_data::*;
pub use temporal::*;
pub use utils::*;
pub use validation::*;
pub use visualization::*;
