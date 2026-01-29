//! Insolvency and Bankruptcy Code (IBC) 2016 Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InsolvencyProcessType {
    CorporateInsolvency,
    Liquidation,
    VoluntaryLiquidation,
    IndividualInsolvency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateInsolvencyProcess {
    pub corporate_debtor: String,
    pub default_amount: f64,
    pub commencement_date: NaiveDate,
    pub resolution_professional: String,
    pub moratorium_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditorType {
    FinancialCreditor,
    OperationalCreditor,
    SecuredCreditor,
    UnsecuredCreditor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creditor {
    pub name: String,
    pub creditor_type: CreditorType,
    pub claim_amount: f64,
    pub admitted_amount: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IbcStage {
    Application,
    Admission,
    Cirp,
    ResolutionPlan,
    Liquidation,
    Completed,
}
