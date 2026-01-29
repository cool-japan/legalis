//! Corporate Tax implementation
//!
//! Federal Decree-Law No. 47/2022

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// Corporate tax threshold
pub const CORPORATE_TAX_THRESHOLD: i64 = 375_000;

/// Corporate tax rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorporateTaxRate {
    SmallBusinessRelief,
    Standard,
    FreeZoneQualifying,
}

impl CorporateTaxRate {
    pub fn percentage(&self) -> f64 {
        match self {
            Self::SmallBusinessRelief | Self::FreeZoneQualifying => 0.0,
            Self::Standard => 9.0,
        }
    }
}
