//! Securities Law Module

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityType {
    pub name: String,
}
