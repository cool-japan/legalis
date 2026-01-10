//! US Choice of Law Module
//!
//! This module provides specialized choice of law analysis for multi-state disputes
//! within the United States. When a legal dispute involves multiple states, choice of
//! law rules determine which state's substantive law applies.
//!
//! ## Choice of Law Approaches
//!
//! The US has seen significant evolution in choice of law methodology:
//!
//! ### 1. Restatement (First) - Traditional Approach (1934)
//! - **Rule**: Place of wrong (lex loci delicti) for torts
//! - **Status**: Minority (only 6 states still follow)
//! - **Criticism**: Mechanical, ignores policy considerations
//!
//! ### 2. Restatement (Second) - Modern Approach (1971)
//! - **Rule**: Most significant relationship test
//! - **Status**: Majority (44 states follow)
//! - **Factors**: §145 factors for torts, §188 for contracts
//!
//! ### 3. Interest Analysis - California Approach
//! - **Rule**: Identify and weigh state interests
//! - **Pioneer**: Professor Brainerd Currie
//! - **States**: California, New Jersey
//!
//! ### 4. Better Law - Minnesota Approach
//! - **Rule**: Apply the better rule of law
//! - **Status**: Minority (Minnesota, Wisconsin)
//!
//! ### 5. Combined Modern - New York Approach
//! - **Rule**: Hybrid of interest analysis and Restatement (Second)
//! - **Status**: New York, some others
//!
//! ## Module Structure
//!
//! - `factors` - US-specific connecting factors
//! - `restatement_first` - Traditional lex loci approach
//! - `restatement_second` - Most significant relationship test
//! - `analyzer` - Enhanced choice of law analyzer

pub mod analyzer;
pub mod factors;
pub mod restatement_first;
pub mod restatement_second;

// Re-exports
pub use analyzer::{ChoiceOfLawApproach, ChoiceOfLawResult, USChoiceOfLawAnalyzer};
pub use factors::{ContactingFactor, USChoiceOfLawFactors};
pub use restatement_first::RestatementFirst;
pub use restatement_second::RestatementSecond;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_accessible() {
        // Verify all submodules are accessible
        let _ = USChoiceOfLawFactors::default();
        let _ = RestatementFirst::new();
        let _ = RestatementSecond::new();
    }
}
