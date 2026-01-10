//! German Labor Law (Arbeitsrecht)
//!
//! This module provides type-safe representations and validation for German labor law,
//! covering employment contracts, dismissal protection, working hours, and employee rights.
//!
//! # Legal Context
//!
//! German labor law protects employees through:
//! - **Employment Contracts** (Arbeitsvertrag) - Written documentation required (§2 NachwG)
//! - **Dismissal Protection** (Kündigungsschutz) - KSchG applies for 10+ employees
//! - **Working Hours** (Arbeitszeitgesetz - ArbZG) - Max 8-10 hours/day
//! - **Annual Leave** (Bundesurlaubsgesetz - BUrlG) - Min 24 days (6-day week)
//! - **Sick Pay** (Entgeltfortzahlungsgesetz - EFZG) - 6 weeks at 100%
//! - **Maternity Protection** (Mutterschutzgesetz - MuSchG)
//! - **Parental Leave** (Elternzeit - BEEG) - Up to 3 years
//! - **Works Councils** (Betriebsverfassungsgesetz - BetrVG) - Required for 5+ employees
//!
//! # Key Statutes
//!
//! - **§622 BGB**: Notice periods
//! - **§623 BGB**: Written form requirement for dismissals
//! - **§626 BGB**: Extraordinary dismissal with good cause
//! - **KSchG**: Dismissal Protection Act (applies 10+ employees, 6 months tenure)
//! - **ArbZG**: Working Hours Act (8h/day regular, 10h with compensation)
//! - **BUrlG**: Federal Leave Act (minimum 4 weeks annual leave)
//! - **EFZG**: Continued Remuneration Act (6 weeks sick pay)
//! - **BetrVG**: Works Constitution Act (co-determination rights)

pub mod error;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use error::{LaborLawError, Result};
pub use types::*;
pub use validator::*;
