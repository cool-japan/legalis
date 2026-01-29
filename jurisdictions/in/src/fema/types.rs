//! Foreign Exchange Management Act (FEMA) 1999 Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FdiRoute {
    Automatic,
    GovernmentApproval,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    CurrentAccount,
    CapitalAccount,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignInvestment {
    pub investor_name: String,
    pub investor_country: String,
    pub investment_amount_usd: f64,
    pub investment_date: NaiveDate,
    pub sector: String,
    pub fdi_route: FdiRoute,
    pub equity_percentage: f64,
}

impl ForeignInvestment {
    pub fn check_automatic_route_eligibility(&self) -> bool {
        matches!(self.fdi_route, FdiRoute::Automatic) && self.equity_percentage <= 100.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FemaViolationType {
    UnauthorizedForexTransaction,
    ExcessiveRemittance,
    NonCompliance,
    FalseCertification,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemittanceTransaction {
    pub remitter: String,
    pub beneficiary: String,
    pub amount_usd: f64,
    pub purpose: String,
    pub account_type: AccountType,
    pub lrs_compliant: bool,
}
