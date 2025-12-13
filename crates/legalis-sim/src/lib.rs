//! Legalis-Sim: Simulation engine for Legalis-RS.
//!
//! This crate provides an ECS-like simulation engine for testing
//! legal statutes against populations of agents.
//!
//! # Features
//!
//! - Population-based simulation with parallel execution
//! - Time-step based temporal simulation
//! - Statute effective date handling
//! - Agent lifecycle management
//! - Comprehensive metrics collection

mod engine;
mod metrics;
mod temporal;

pub use engine::*;
pub use metrics::*;
pub use temporal::*;
