//! # Corporate Law - Direito Societário
//!
//! Brazilian corporate law including corporations, limited liability companies, and securities.
//!
//! ## Overview
//!
//! | Law | Description | Year |
//! |-----|-------------|------|
//! | Lei 6.404/1976 | Corporations Law (Lei das S.A.) | 1976 |
//! | Código Civil | Limited Liability Companies (Arts. 1052-1087) | 2002 |
//! | Lei 6.385/1976 | Securities Market (CVM) | 1976 |
//!
//! ## Company Types
//!
//! | Type | Characteristics | Liability |
//! |------|-----------------|-----------|
//! | S.A. | Corporation (shares) | Limited |
//! | Ltda. | Limited Liability | Limited to capital |
//! | Simples | Simple Partnership | Unlimited |
//! | Eireli | Single-member LLC (extinct 2021) | Limited |

pub mod corporations_law;
pub mod cvm;
pub mod limited_liability;

pub use corporations_law::*;
pub use cvm::*;
pub use limited_liability::*;
