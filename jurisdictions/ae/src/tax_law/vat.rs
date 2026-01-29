//! VAT (Value Added Tax) specific implementation
//!
//! Federal Decree-Law No. 8/2017

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// VAT registration thresholds
pub const VAT_MANDATORY_THRESHOLD: i64 = 375_000;
pub const VAT_VOLUNTARY_THRESHOLD: i64 = 187_500;

/// VAT rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRate {
    Standard,
    ZeroRated,
    Exempt,
    OutOfScope,
}

impl VatRate {
    pub fn percentage(&self) -> f64 {
        match self {
            Self::Standard => 5.0,
            _ => 0.0,
        }
    }

    pub fn calculate_vat(&self, amount: Aed) -> Aed {
        match self {
            Self::Standard => Aed::from_fils(amount.fils() * 5 / 100),
            _ => Aed::from_fils(0),
        }
    }
}
