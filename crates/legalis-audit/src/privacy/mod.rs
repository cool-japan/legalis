//! Privacy-preserving audit features.
//!
//! This module provides cryptographic techniques for maintaining audit trails
//! while preserving privacy through:
//! - Zero-knowledge proofs for audit verification without revealing data
//! - Differential privacy for statistical queries with privacy guarantees
//! - Homomorphic encryption for computation on encrypted data
//! - Selective disclosure for controlled information release

pub mod differential_privacy;
pub mod homomorphic;
pub mod selective_disclosure;
pub mod zkp;

pub use differential_privacy::DifferentialPrivacy;
pub use homomorphic::HomomorphicAggregator;
pub use selective_disclosure::SelectiveDisclosure;
pub use zkp::{ZkProof, ZkProofGenerator, ZkProofVerifier};
