//! Rental regulations under RERA

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// RERA rental contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalContract {
    pub annual_rent: Aed,
    pub duration_years: u32,
    pub security_deposit: Aed,
    pub payment_frequency: u32,
    pub ejari_registered: bool,
}

impl RentalContract {
    pub fn standard(annual_rent: Aed) -> Self {
        let security_deposit = Aed::from_fils(annual_rent.fils() * 5 / 100);
        Self {
            annual_rent,
            duration_years: 1,
            security_deposit,
            payment_frequency: 1,
            ejari_registered: false,
        }
    }
}
