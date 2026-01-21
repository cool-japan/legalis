//! Common Brazilian legal types and utilities
//!
//! This module provides common types used across Brazilian legal domains:
//! - Currency (BRL - Brazilian Real)
//! - Dates and deadlines
//! - Portuguese name handling
//! - Brazilian document identifiers (CPF, CNPJ)
//! - Geographic divisions (States, Municipalities)

pub mod types;

pub use types::{
    BrazilianCurrency, BrazilianDate, BrazilianDocument, BrazilianState, DocumentType,
    FederalEntity, Municipality, validate_cnpj, validate_cpf,
};
