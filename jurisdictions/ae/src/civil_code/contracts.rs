//! Contract Law under UAE Civil Code

use serde::{Deserialize, Serialize};

/// Contract formation requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFormation {
    pub offer: bool,
    pub acceptance: bool,
    pub capacity: bool,
    pub lawful_object: bool,
    pub lawful_cause: bool,
}

impl ContractFormation {
    pub fn is_valid(&self) -> bool {
        self.offer && self.acceptance && self.capacity && self.lawful_object && self.lawful_cause
    }
}
