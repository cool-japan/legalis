//! Legalis-Sim: Simulation engine for Legalis-RS.
//!
//! This crate provides an ECS-like simulation engine for testing
//! legal statutes against populations of agents.
//!
//! # Features
//!
//! - Population-based simulation with parallel execution
//! - Realistic demographic population generation
//! - Time-step based temporal simulation
//! - Retroactive law application
//! - Statute effective date handling
//! - Agent lifecycle management
//! - Behavioral modeling with compliance probability
//! - Inter-agent relationships and organizations
//! - Property and asset management
//! - Statute comparison and A/B testing
//! - Statistical analysis (distribution, correlation, time-series)
//! - Comprehensive metrics collection

mod analysis;
mod behavior;
mod comparison;
mod engine;
mod metrics;
mod population;
mod relationships;
mod temporal;

pub use analysis::*;
pub use behavior::*;
pub use comparison::*;
pub use engine::*;
pub use metrics::*;
pub use population::*;
pub use relationships::*;
pub use temporal::*;
