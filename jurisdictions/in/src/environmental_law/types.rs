//! Environmental Protection Act 1986 Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentalClearanceType {
    StateLevelClearance,
    CentralClearance,
    ForestClearance,
    CoastalZoneClearance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalClearance {
    pub clearance_type: EnvironmentalClearanceType,
    pub project_name: String,
    pub proponent: String,
    pub clearance_date: Option<NaiveDate>,
    pub validity_years: u32,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PollutionType {
    Air,
    Water,
    Noise,
    SolidWaste,
    HazardousWaste,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollutionControlCompliance {
    pub pollution_type: PollutionType,
    pub consent_obtained: bool,
    pub monitoring_compliant: bool,
    pub emission_standards_met: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectCategory {
    CategoryA,
    CategoryB1,
    CategoryB2,
}
