//! Legalis-Sim: Simulation engine for Legalis-RS.
//!
//! This crate provides an ECS-like simulation engine for testing
//! legal statutes against populations of agents.

mod engine;
mod metrics;

pub use engine::*;
pub use metrics::*;
