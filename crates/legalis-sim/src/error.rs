//! Error types for simulation operations.

use thiserror::Error;

/// Errors that can occur during simulation.
#[derive(Debug, Error)]
pub enum SimulationError {
    /// Invalid configuration provided
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Population is empty
    #[error("Population cannot be empty")]
    EmptyPopulation,

    /// No statutes provided
    #[error("At least one statute is required")]
    NoStatutes,

    /// Simulation was cancelled
    #[error("Simulation was cancelled")]
    Cancelled,

    /// I/O error during export/import
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid date range
    #[error("Invalid date range: start must be before end")]
    InvalidDateRange,

    /// Temporal configuration error
    #[error("Temporal configuration error: {0}")]
    TemporalConfig(String),

    /// Population generation error
    #[error("Population generation error: {0}")]
    PopulationGeneration(String),

    /// Checkpoint error
    #[error("Checkpoint error: {0}")]
    Checkpoint(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Execution error
    #[error("Execution error: {0}")]
    ExecutionError(String),
}

/// Result type for simulation operations.
pub type SimResult<T> = Result<T, SimulationError>;
