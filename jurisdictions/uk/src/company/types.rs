//! Company Law Types (Companies Act 2006)
//!
//! This module provides types for UK company law under the Companies Act 2006.

#![allow(missing_docs)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of company under Companies Act 2006
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Private company limited by shares (most common)
    /// Must have "Limited" or "Ltd" suffix
    PrivateLimitedByShares,

    /// Public limited company
    /// Must have minimum £50,000 share capital (25% paid up)
    /// Must have "Public Limited Company" or "PLC" suffix
    PublicLimitedCompany,

    /// Private company limited by guarantee
    /// No share capital, members guarantee amount
    PrivateLimitedByGuarantee,

    /// Limited liability partnership
    /// Governed by Limited Liability Partnerships Act 2000
    LimitedLiabilityPartnership,

    /// Unlimited company
    /// No limit on members' liability
    UnlimitedCompany,
}

impl CompanyType {
    /// Get required suffix for company name
    pub fn required_suffix(&self) -> &'static str {
        match self {
            Self::PrivateLimitedByShares | Self::PrivateLimitedByGuarantee => "Limited",
            Self::PublicLimitedCompany => "Public Limited Company",
            Self::LimitedLiabilityPartnership => "Limited Liability Partnership",
            Self::UnlimitedCompany => "",
        }
    }

    /// Get abbreviated suffix options
    pub fn abbreviated_suffix(&self) -> Option<&'static str> {
        match self {
            Self::PrivateLimitedByShares | Self::PrivateLimitedByGuarantee => Some("Ltd"),
            Self::PublicLimitedCompany => Some("PLC"),
            Self::LimitedLiabilityPartnership => Some("LLP"),
            Self::UnlimitedCompany => None,
        }
    }

    /// Check if company type requires share capital
    pub fn requires_share_capital(&self) -> bool {
        matches!(
            self,
            Self::PrivateLimitedByShares | Self::PublicLimitedCompany
        )
    }

    /// Get minimum share capital requirement (in GBP)
    pub fn minimum_share_capital(&self) -> Option<f64> {
        match self {
            Self::PublicLimitedCompany => Some(50_000.0), // £50,000
            Self::PrivateLimitedByShares => Some(0.01),   // Nominal (1p minimum)
            _ => None,
        }
    }
}

/// Company formation details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyFormation {
    /// Company name (must comply with CA 2006 ss.53-81)
    pub company_name: String,

    /// Company type
    pub company_type: CompanyType,

    /// Registered office address (must be in UK)
    pub registered_office: RegisteredOffice,

    /// Share capital (if applicable)
    pub share_capital: Option<ShareCapital>,

    /// Initial directors (minimum 1 for private, 2 for public)
    pub directors: Vec<Director>,

    /// Initial shareholders/members
    pub shareholders: Vec<Shareholder>,

    /// Company secretary (required for PLC, optional for private)
    pub secretary: Option<CompanySecretary>,

    /// Statement of compliance (CA 2006 s.13)
    pub statement_of_compliance: bool,

    /// Formation date
    pub formation_date: Option<NaiveDate>,
}

/// Registered office address
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredOffice {
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub county: Option<String>,
    pub postcode: String,
    pub country: RegisteredOfficeCountry,
}

/// Country for registered office (must be within UK)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisteredOfficeCountry {
    England,
    Wales,
    Scotland,
    NorthernIreland,
}

/// Share capital structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareCapital {
    /// Total nominal (authorized) capital
    pub nominal_capital_gbp: f64,

    /// Amount paid up (must be at least 25% for PLC)
    pub paid_up_capital_gbp: f64,

    /// Number of shares
    pub number_of_shares: u64,

    /// Nominal value per share
    pub nominal_value_per_share_gbp: f64,

    /// Share classes
    pub share_classes: Vec<ShareClass>,
}

impl ShareCapital {
    /// Calculate percentage paid up
    pub fn percentage_paid_up(&self) -> f64 {
        if self.nominal_capital_gbp == 0.0 {
            0.0
        } else {
            (self.paid_up_capital_gbp / self.nominal_capital_gbp) * 100.0
        }
    }

    /// Check if meets minimum paid up requirement for PLC (25%)
    pub fn meets_plc_paid_up_requirement(&self) -> bool {
        self.percentage_paid_up() >= 25.0
    }
}

/// Share class
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareClass {
    pub class_name: String,
    pub number_of_shares: u64,
    pub rights: ShareRights,
}

/// Rights attached to shares
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareRights {
    /// Voting rights (votes per share)
    pub voting_rights: u32,

    /// Dividend rights
    pub dividend_rights: DividendRights,

    /// Capital distribution rights
    pub capital_rights: CapitalRights,
}

/// Dividend rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DividendRights {
    /// Ordinary shares (variable dividend)
    Ordinary,

    /// Preference shares (fixed dividend rate)
    Preference { fixed_rate_percent: u32 },

    /// No dividend rights
    NoDividend,
}

/// Capital distribution rights on winding up
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapitalRights {
    /// Equal rights to capital distribution
    Equal,

    /// Priority in capital distribution
    Priority,

    /// No capital rights
    NoRights,
}

/// Director of a company
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Director {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub nationality: String,
    pub service_address: ServiceAddress,
    pub appointment_date: NaiveDate,
    pub resignation_date: Option<NaiveDate>,
    pub director_type: DirectorType,
}

/// Type of director
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorType {
    /// Individual person
    Individual,

    /// Corporate director (company acting as director)
    Corporate,
}

/// Service address for director
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceAddress {
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub postcode: String,
    pub country: String,
}

/// Shareholder/member of a company
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shareholder {
    pub name: String,
    pub address: String,
    pub number_of_shares: u64,
    pub share_class: String,
    pub amount_paid_gbp: f64,
    pub amount_unpaid_gbp: f64,
}

/// Company secretary
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanySecretary {
    pub name: String,
    pub address: ServiceAddress,
    pub appointment_date: NaiveDate,
}

/// Seven statutory director duties (CA 2006 ss.171-177)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectorDutiesCompliance {
    /// s.171: Duty to act within powers
    pub act_within_powers: DutyCompliance,

    /// s.172: Duty to promote the success of the company
    pub promote_success: PromoteSuccessCompliance,

    /// s.173: Duty to exercise independent judgment
    pub independent_judgment: DutyCompliance,

    /// s.174: Duty to exercise reasonable care, skill and diligence
    pub reasonable_care: ReasonableCareCompliance,

    /// s.175: Duty to avoid conflicts of interest
    pub avoid_conflicts: ConflictsCompliance,

    /// s.176: Duty not to accept benefits from third parties
    pub no_third_party_benefits: DutyCompliance,

    /// s.177: Duty to declare interest in proposed transaction
    pub declare_interest: DeclareInterestCompliance,
}

/// Generic duty compliance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DutyCompliance {
    pub compliant: bool,
    pub evidence: String,
    pub breach_details: Option<String>,
}

/// s.172 Promote success compliance (6 considerations)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromoteSuccessCompliance {
    pub compliant: bool,

    /// s.172(1)(a): Likely consequences of any decision in the long term
    pub long_term_consequences_considered: bool,

    /// s.172(1)(b): Interests of the company's employees
    pub employee_interests_considered: bool,

    /// s.172(1)(c): Need to foster business relationships with suppliers, customers and others
    pub business_relationships_considered: bool,

    /// s.172(1)(d): Impact of company's operations on community and environment
    pub community_environment_considered: bool,

    /// s.172(1)(e): Desirability of maintaining reputation for high standards
    pub reputation_considered: bool,

    /// s.172(1)(f): Need to act fairly between members
    pub fairness_between_members_considered: bool,

    pub evidence: String,
}

/// s.174 Reasonable care, skill and diligence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasonableCareCompliance {
    pub compliant: bool,

    /// Objective test: care reasonably expected from person in that position
    pub objective_standard_met: bool,

    /// Subjective test: care expected given director's actual knowledge and experience
    pub subjective_standard_met: bool,

    pub evidence: String,
}

/// s.175 Conflicts of interest
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictsCompliance {
    pub compliant: bool,

    /// Any conflicts declared and authorized
    pub conflicts_declared: Vec<ConflictOfInterest>,

    /// Board authorization obtained where required
    pub board_authorization_obtained: bool,
}

/// Conflict of interest declaration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictOfInterest {
    pub nature_of_conflict: String,
    pub date_declared: NaiveDate,
    pub authorization_obtained: bool,
    pub authorization_reference: Option<String>,
}

/// s.177 Declaration of interest in proposed transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeclareInterestCompliance {
    pub compliant: bool,

    /// All interests in proposed transactions declared
    pub interests_declared: Vec<InterestDeclaration>,
}

/// Interest declaration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterestDeclaration {
    pub transaction_description: String,
    pub nature_of_interest: String,
    pub date_declared: NaiveDate,
    pub declared_to_board: bool,
}

/// Company name validation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanyNameValidation {
    pub name: String,
    pub valid: bool,
    pub has_correct_suffix: bool,
    pub contains_sensitive_words: bool,
    pub too_similar_to_existing: bool,
    pub contains_prohibited_words: bool,
    pub validation_errors: Vec<String>,
}

/// Annual accounts requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnualAccountsRequirement {
    pub company_type: CompanyType,
    pub financial_year_end: NaiveDate,
    pub filing_deadline: NaiveDate,
    pub accounts_prepared: bool,
    pub accounts_audited: bool,
    pub audit_required: bool,
}

impl AnnualAccountsRequirement {
    /// Calculate filing deadline (9 months for private, 6 months for public)
    pub fn calculate_filing_deadline(
        company_type: CompanyType,
        financial_year_end: NaiveDate,
    ) -> NaiveDate {
        let months = match company_type {
            CompanyType::PublicLimitedCompany => 6,
            _ => 9,
        };

        // Add months to financial year end
        financial_year_end
            .checked_add_months(chrono::Months::new(months))
            .unwrap_or(financial_year_end)
    }

    /// Check if audit is required (CA 2006 s.475)
    /// Small companies exempt if turnover ≤ £10.2m and balance sheet ≤ £5.1m
    pub fn is_audit_required(
        company_type: CompanyType,
        turnover_gbp: f64,
        balance_sheet_total_gbp: f64,
    ) -> bool {
        match company_type {
            CompanyType::PublicLimitedCompany => true,
            CompanyType::PrivateLimitedByShares | CompanyType::PrivateLimitedByGuarantee => {
                // Small company exemption thresholds (CA 2006 s.382)
                turnover_gbp > 10_200_000.0 || balance_sheet_total_gbp > 5_100_000.0
            }
            _ => false,
        }
    }
}

/// General meeting type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeetingType {
    /// Annual General Meeting (AGM) - required for PLCs
    AnnualGeneralMeeting,

    /// General Meeting
    GeneralMeeting,

    /// Board meeting (directors)
    BoardMeeting,
}

/// Resolution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution (simple majority > 50%)
    Ordinary,

    /// Special resolution (75% majority)
    Special,

    /// Written resolution (private companies only)
    Written,
}

impl ResolutionType {
    /// Get required majority percentage
    pub fn required_majority(&self) -> f64 {
        match self {
            Self::Ordinary => 50.0,
            Self::Special => 75.0,
            Self::Written => 50.0, // Majority of eligible members
        }
    }

    /// Check if resolution passes
    pub fn passes(&self, votes_for: u64, votes_against: u64) -> bool {
        let total_votes = votes_for + votes_against;
        if total_votes == 0 {
            return false;
        }

        let percentage_for = (votes_for as f64 / total_votes as f64) * 100.0;

        match self {
            // Ordinary resolution requires > 50%
            Self::Ordinary | Self::Written => percentage_for > self.required_majority(),
            // Special resolution requires >= 75% (not just >75%)
            Self::Special => percentage_for >= self.required_majority(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type_suffixes() {
        assert_eq!(
            CompanyType::PrivateLimitedByShares.required_suffix(),
            "Limited"
        );
        assert_eq!(
            CompanyType::PublicLimitedCompany.required_suffix(),
            "Public Limited Company"
        );
        assert_eq!(
            CompanyType::PrivateLimitedByShares.abbreviated_suffix(),
            Some("Ltd")
        );
        assert_eq!(
            CompanyType::PublicLimitedCompany.abbreviated_suffix(),
            Some("PLC")
        );
    }

    #[test]
    fn test_minimum_share_capital() {
        assert_eq!(
            CompanyType::PublicLimitedCompany.minimum_share_capital(),
            Some(50_000.0)
        );
        assert_eq!(
            CompanyType::PrivateLimitedByShares.minimum_share_capital(),
            Some(0.01)
        );
    }

    #[test]
    fn test_share_capital_calculations() {
        let share_capital = ShareCapital {
            nominal_capital_gbp: 100_000.0,
            paid_up_capital_gbp: 25_000.0,
            number_of_shares: 100_000,
            nominal_value_per_share_gbp: 1.0,
            share_classes: vec![],
        };

        assert_eq!(share_capital.percentage_paid_up(), 25.0);
        assert!(share_capital.meets_plc_paid_up_requirement());
    }

    #[test]
    fn test_resolution_voting() {
        assert!(ResolutionType::Ordinary.passes(51, 49));
        assert!(!ResolutionType::Ordinary.passes(50, 50));
        assert!(ResolutionType::Special.passes(76, 24));
        assert!(!ResolutionType::Special.passes(74, 26));
    }

    #[test]
    fn test_filing_deadlines() {
        let year_end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let private_deadline = AnnualAccountsRequirement::calculate_filing_deadline(
            CompanyType::PrivateLimitedByShares,
            year_end,
        );
        assert_eq!(
            private_deadline,
            NaiveDate::from_ymd_opt(2025, 9, 30).unwrap()
        );

        let public_deadline = AnnualAccountsRequirement::calculate_filing_deadline(
            CompanyType::PublicLimitedCompany,
            year_end,
        );
        assert_eq!(
            public_deadline,
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
        );
    }

    #[test]
    fn test_audit_requirements() {
        // PLC always requires audit
        assert!(AnnualAccountsRequirement::is_audit_required(
            CompanyType::PublicLimitedCompany,
            1_000_000.0,
            500_000.0
        ));

        // Small private company exempt
        assert!(!AnnualAccountsRequirement::is_audit_required(
            CompanyType::PrivateLimitedByShares,
            5_000_000.0,
            2_000_000.0
        ));

        // Large private company requires audit
        assert!(AnnualAccountsRequirement::is_audit_required(
            CompanyType::PrivateLimitedByShares,
            15_000_000.0,
            8_000_000.0
        ));
    }
}
