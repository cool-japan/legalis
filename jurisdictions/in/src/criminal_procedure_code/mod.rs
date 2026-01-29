//! Criminal Procedure Code (CrPC) 1973 / BNSS 2023
//!
//! Criminal procedure law for India (now replaced by Bharatiya Nagarik Suraksha Sanhita 2023).
//!
//! ## Key Provisions
//!
//! - **Section 154**: FIR registration
//! - **Section 167**: Police custody (max 15 days)
//! - **Section 173**: Chargesheet (60/90 days)
//! - **Section 436-450**: Bail provisions
//! - **Section 320**: Compoundable offences
//!
//! ## Investigation Timeline
//!
//! | Stage | Time Limit | Section |
//! |-------|------------|---------|
//! | Production before magistrate | 24 hours | 167(1) |
//! | Chargesheet (up to 10 years) | 60 days | 167(2) |
//! | Chargesheet (10+ years/death/life) | 90 days | 167(2) |
//!
//! ## Appeal Limitation
//!
//! | Appeal Type | Limitation |
//! |-------------|------------|
//! | Sessions Court | 30 days |
//! | High Court | 60 days |
//! | Revision | 90 days |
//! | SLP to SC | 90 days |

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
