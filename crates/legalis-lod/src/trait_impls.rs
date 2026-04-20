//! # Namespaces - Trait Implementations
//!
//! This module contains trait implementations for `Namespaces`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{Namespaces, ProvenanceInfo};

impl Default for Namespaces {
    fn default() -> Self {
        Self {
            base: "https://example.org/legalis/".to_string(),
            custom: HashMap::new(),
        }
    }
}

impl Default for ProvenanceInfo {
    fn default() -> Self {
        Self::new()
    }
}
