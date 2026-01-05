//! Hybrid symbolic-neural reasoning combining traditional logic with ML.
//!
//! This module combines symbolic legal reasoning (rule-based, deterministic) with
//! neural predictions to leverage the strengths of both approaches.

use crate::LegalResult;

#[allow(unused_imports)]
use crate::{Condition, Statute};
#[allow(unused_imports)]
use std::collections::HashMap;

/// Hybrid reasoning strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningStrategy {
    /// Use symbolic reasoning when confident, neural as fallback
    SymbolicFirst,
    /// Use neural predictions, validate with symbolic rules
    NeuralFirst,
    /// Ensemble both approaches and vote
    Ensemble,
    /// Use symbolic for deterministic parts, neural for discretionary
    Adaptive,
}

/// Result from hybrid reasoning.
#[derive(Debug, Clone)]
pub struct HybridResult<T> {
    /// The final result
    pub value: T,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Which strategy was used
    pub strategy_used: ReasoningStrategy,
    /// Symbolic component result (if available)
    pub symbolic_result: Option<LegalResult<T>>,
    /// Neural component confidence (if used)
    pub neural_confidence: Option<f64>,
}

/// Hybrid reasoner combining symbolic and neural approaches.
pub struct HybridReasoner {
    strategy: ReasoningStrategy,
    symbolic_weight: f64,
    neural_weight: f64,
    stats: HybridStats,
}

impl HybridReasoner {
    /// Creates a new hybrid reasoner with ensemble strategy.
    pub fn new() -> Self {
        Self {
            strategy: ReasoningStrategy::Ensemble,
            symbolic_weight: 0.5,
            neural_weight: 0.5,
            stats: HybridStats::new(),
        }
    }

    /// Sets the reasoning strategy.
    pub fn with_strategy(mut self, strategy: ReasoningStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets weights for ensemble (must sum to 1.0).
    pub fn with_weights(mut self, symbolic: f64, neural: f64) -> Self {
        assert!(
            (symbolic + neural - 1.0).abs() < 0.001,
            "Weights must sum to 1.0"
        );
        self.symbolic_weight = symbolic;
        self.neural_weight = neural;
        self
    }

    /// Returns hybrid statistics.
    pub fn stats(&self) -> &HybridStats {
        &self.stats
    }
}

impl Default for HybridReasoner {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for hybrid reasoning.
#[derive(Debug, Clone, Default)]
pub struct HybridStats {
    /// Times symbolic reasoning was used
    pub symbolic_count: u64,
    /// Times neural reasoning was used
    pub neural_count: u64,
    /// Times ensemble was used
    pub ensemble_count: u64,
    /// Agreements between symbolic and neural
    pub agreements: u64,
    /// Disagreements
    pub disagreements: u64,
}

impl HybridStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns agreement rate between symbolic and neural.
    pub fn agreement_rate(&self) -> f64 {
        let total = self.agreements + self.disagreements;
        if total == 0 {
            0.0
        } else {
            self.agreements as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_reasoner() {
        let reasoner = HybridReasoner::new();
        assert_eq!(reasoner.strategy, ReasoningStrategy::Ensemble);
        assert_eq!(reasoner.symbolic_weight, 0.5);
    }

    #[test]
    fn test_reasoning_strategies() {
        let reasoner = HybridReasoner::new().with_strategy(ReasoningStrategy::SymbolicFirst);
        assert_eq!(reasoner.strategy, ReasoningStrategy::SymbolicFirst);
    }

    #[test]
    fn test_hybrid_stats() {
        let mut stats = HybridStats::new();
        stats.agreements = 8;
        stats.disagreements = 2;
        assert_eq!(stats.agreement_rate(), 0.8);
    }
}
