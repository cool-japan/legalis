//! Immigration Law Types (Immigration and Nationality Act)
//!
//! This module provides types for US immigration law under the INA.

#![allow(missing_docs)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Visa category under INA
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisaCategory {
    // Nonimmigrant visas
    B1B2Tourist,
    F1Student,
    H1BSpecialtyWorker,
    L1IntracompanyTransferee,
    O1ExtraordinaryAbility,
    E2TreatyInvestor,
    J1ExchangeVisitor,

    // Immigrant visas (Employment-based)
    EB1ExtraordinaryAbility,
    EB2AdvancedDegree,
    EB3SkilledWorker,
    EB4SpecialImmigrant,
    EB5Investor,

    // Family-based
    IR1ImmediateRelative,
    F1UnmarriedChild,
    F2ASpouse,
    F3MarriedChild,
    F4Sibling,

    Other { description: String },
}

/// Immigration status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmigrationStatus {
    NonimmigrantVisa { category: VisaCategory },
    LawfulPermanentResident { receipt_number: String },
    Naturalized { certificate_number: String },
    Pending { petition_type: String },
    Unauthorized,
}

/// Priority date for visa processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriorityDate {
    pub date: NaiveDate,
    pub category: VisaCategory,
    pub country_of_chargeability: String,
}

/// Green card application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GreenCardApplication {
    pub applicant_name: String,
    pub priority_date: Option<PriorityDate>,
    pub category: VisaCategory,
    pub sponsoring_employer: Option<String>,
    pub labor_certification: Option<LaborCertification>,
    pub adjustment_of_status: bool,
}

/// Labor certification (PERM)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaborCertification {
    pub employer: String,
    pub job_title: String,
    pub certification_date: NaiveDate,
    pub prevailing_wage: f64,
}

/// Naturalization application
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalizationApplication {
    pub applicant_name: String,
    pub green_card_date: NaiveDate,
    pub continuous_residence_start: NaiveDate,
    pub physical_presence_days: u32,
    pub good_moral_character: bool,
    pub english_proficiency: bool,
    pub civics_knowledge: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visa_category() {
        let visa = VisaCategory::H1BSpecialtyWorker;
        assert!(matches!(visa, VisaCategory::H1BSpecialtyWorker));
    }
}
