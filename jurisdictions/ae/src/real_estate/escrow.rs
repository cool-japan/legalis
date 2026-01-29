//! Escrow Law - Federal Decree-Law No. 9/2009

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// Escrow account requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowAccount {
    pub project_name: String,
    pub developer: String,
    pub project_value: Aed,
    pub escrow_opened: bool,
    pub bank: Option<String>,
    pub escrow_percentage: u32,
}

impl EscrowAccount {
    pub fn is_required(&self) -> bool {
        self.project_value.dirhams() > 500_000
    }
}
