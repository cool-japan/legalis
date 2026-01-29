//! Arbitration and Conciliation Act 1996 Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArbitrationType {
    Domestic,
    International,
    AdHoc,
    Institutional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArbitrationAgreement {
    pub parties: Vec<String>,
    pub agreement_type: ArbitrationType,
    pub seat_of_arbitration: String,
    pub governing_law: String,
    pub arbitrator_count: u32,
    pub execution_date: NaiveDate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AwardType {
    Final,
    Interim,
    Additional,
    Corrected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArbitralAward {
    pub award_type: AwardType,
    pub amount: Option<f64>,
    pub award_date: NaiveDate,
    pub enforceability: bool,
}
