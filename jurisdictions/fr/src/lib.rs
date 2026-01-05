//! French jurisdiction support for Legalis-RS.
//!
//! This crate provides structured representations of French law, including:
//! - Code civil (Civil Code) - Napoleonic Code of 1804
//! - Code pénal (Criminal Code)
//! - Constitution de 1958 (Fifth Republic Constitution)

pub mod code_civil;

pub use code_civil::{article_1240, article_1241, article_1242};
