//! Core superannuation types

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Superannuation fund type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FundType {
    /// Industry fund
    Industry,
    /// Retail fund
    Retail,
    /// Corporate fund
    Corporate,
    /// Public sector fund
    PublicSector,
    /// Self-managed super fund (SMSF)
    Smsf,
    /// Small APRA fund
    SmallApra,
}

impl FundType {
    /// Whether fund is APRA-regulated
    pub fn is_apra_regulated(&self) -> bool {
        !matches!(self, FundType::Smsf)
    }

    /// Whether fund is ATO-regulated
    pub fn is_ato_regulated(&self) -> bool {
        matches!(self, FundType::Smsf)
    }
}

/// Superannuation fund
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuperannuationFund {
    /// Fund name
    pub name: String,
    /// Australian Business Number
    pub abn: String,
    /// Unique Superannuation Identifier
    pub usi: Option<String>,
    /// Fund type
    pub fund_type: FundType,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Is complying fund
    pub is_complying: bool,
    /// MySuper product (if any)
    pub my_super_product: Option<String>,
    /// Total members
    pub total_members: u32,
    /// Total assets (AUD)
    pub total_assets: f64,
}

/// Member category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberCategory {
    /// Accumulation phase member
    Accumulation,
    /// Transition to retirement (TTR) pension
    TransitionToRetirement,
    /// Account-based pension
    AccountBasedPension,
    /// Defined benefit member
    DefinedBenefit,
}

/// Member preservation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreservationStatus {
    /// Preserved (cannot access until condition of release met)
    Preserved,
    /// Restricted non-preserved
    RestrictedNonPreserved,
    /// Unrestricted non-preserved (can access anytime)
    UnrestrictedNonPreserved,
}

/// Fund member
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundMember {
    /// Member identifier
    pub member_id: String,
    /// Tax File Number (encrypted/hashed)
    pub tfn_provided: bool,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// Join date
    pub join_date: NaiveDate,
    /// Member category
    pub category: MemberCategory,
    /// Account balance
    pub account_balance: f64,
    /// Preserved amount
    pub preserved_amount: f64,
    /// Restricted non-preserved amount
    pub restricted_non_preserved: f64,
    /// Unrestricted non-preserved amount
    pub unrestricted_non_preserved: f64,
    /// Insurance cover
    pub insurance: Option<MemberInsurance>,
    /// Beneficiary nominations
    pub beneficiaries: Vec<BeneficiaryNomination>,
}

impl FundMember {
    /// Calculate member's age at a given date
    pub fn age_at(&self, date: NaiveDate) -> u32 {
        let years = date.year() - self.date_of_birth.year();
        let had_birthday =
            (date.month(), date.day()) >= (self.date_of_birth.month(), self.date_of_birth.day());
        if had_birthday {
            years as u32
        } else {
            (years - 1).max(0) as u32
        }
    }

    /// Check if member has reached preservation age (varies by birth year)
    pub fn preservation_age(&self) -> u32 {
        let birth_year = self.date_of_birth.year();
        match birth_year {
            y if y <= 1960 => 55,
            1961 => 56,
            1962 => 57,
            1963 => 58,
            1964 => 59,
            _ => 60,
        }
    }
}

/// Member insurance coverage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberInsurance {
    /// Death cover amount
    pub death_cover: Option<f64>,
    /// Total and permanent disability (TPD) cover
    pub tpd_cover: Option<f64>,
    /// Income protection cover
    pub income_protection: Option<IncomeProtection>,
    /// Premium per month
    pub monthly_premium: f64,
}

/// Income protection insurance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeProtection {
    /// Monthly benefit amount
    pub monthly_benefit: f64,
    /// Waiting period (days)
    pub waiting_period_days: u32,
    /// Benefit period (years, 0 = to age 65)
    pub benefit_period_years: u32,
}

/// Beneficiary nomination type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NominationType {
    /// Non-binding (trustee has discretion)
    NonBinding,
    /// Binding (trustee must follow)
    Binding,
    /// Reversionary (pension continues to nominated person)
    Reversionary,
}

/// Beneficiary nomination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeneficiaryNomination {
    /// Beneficiary name
    pub name: String,
    /// Relationship to member
    pub relationship: BeneficiaryRelationship,
    /// Nomination type
    pub nomination_type: NominationType,
    /// Percentage of benefit
    pub percentage: f64,
    /// Date of nomination
    pub nomination_date: NaiveDate,
    /// Expiry date (for binding, 3 years max)
    pub expiry_date: Option<NaiveDate>,
}

/// Beneficiary relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeneficiaryRelationship {
    /// Spouse (includes de facto)
    Spouse,
    /// Child (includes step/adopted)
    Child,
    /// Financial dependant
    FinancialDependant,
    /// Interdependency relationship
    Interdependent,
    /// Legal personal representative (estate)
    LegalPersonalRepresentative,
}

impl BeneficiaryRelationship {
    /// Whether this is a "dependant" for death benefit tax purposes
    pub fn is_tax_dependant(&self) -> bool {
        matches!(
            self,
            BeneficiaryRelationship::Spouse
                | BeneficiaryRelationship::Child
                | BeneficiaryRelationship::FinancialDependant
                | BeneficiaryRelationship::Interdependent
        )
    }
}

/// Contribution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContributionType {
    /// Employer SG contribution
    SuperannuationGuarantee,
    /// Salary sacrifice
    SalarySacrifice,
    /// Personal deductible (concessional)
    PersonalDeductible,
    /// Personal non-deductible (non-concessional)
    PersonalNonDeductible,
    /// Spouse contribution
    SpouseContribution,
    /// Government co-contribution
    GovernmentCoContribution,
    /// Downsizer contribution
    Downsizer,
    /// CGT small business contribution
    CgtSmallBusiness,
    /// Transfer from foreign fund
    ForeignTransfer,
}

impl ContributionType {
    /// Whether this is a concessional contribution
    pub fn is_concessional(&self) -> bool {
        matches!(
            self,
            ContributionType::SuperannuationGuarantee
                | ContributionType::SalarySacrifice
                | ContributionType::PersonalDeductible
        )
    }

    /// Whether this counts towards non-concessional cap
    pub fn is_non_concessional(&self) -> bool {
        matches!(
            self,
            ContributionType::PersonalNonDeductible | ContributionType::SpouseContribution
        )
    }

    /// Whether this contribution has a separate cap
    pub fn has_separate_cap(&self) -> bool {
        matches!(
            self,
            ContributionType::Downsizer | ContributionType::CgtSmallBusiness
        )
    }
}

/// Superannuation contribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contribution {
    /// Contribution type
    pub contribution_type: ContributionType,
    /// Amount
    pub amount: f64,
    /// Date received by fund
    pub date_received: NaiveDate,
    /// Financial year
    pub financial_year: String,
    /// Employer (if employer contribution)
    pub employer: Option<String>,
}

/// Condition of release
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionOfRelease {
    /// Retirement (ceased gainful employment after reaching preservation age)
    Retirement,
    /// Reached age 65
    Age65,
    /// Permanent incapacity
    PermanentIncapacity,
    /// Terminal medical condition
    TerminalMedicalCondition,
    /// Severe financial hardship
    SevereFinancialHardship,
    /// Compassionate grounds
    CompassionateGrounds,
    /// Temporary resident permanently departing
    DepartingTemporaryResident,
    /// Transition to retirement (preservation age, still employed)
    TransitionToRetirement,
    /// Death
    Death,
    /// First Home Super Saver Scheme (FHSSS)
    FirstHomeSuperSaver,
    /// Lost member account (<$200 and inactive)
    LostMemberAccount,
    /// Release authority (excess contributions)
    ReleaseAuthority,
}

impl ConditionOfRelease {
    /// Whether full balance can be accessed
    pub fn allows_full_access(&self) -> bool {
        matches!(
            self,
            ConditionOfRelease::Retirement
                | ConditionOfRelease::Age65
                | ConditionOfRelease::PermanentIncapacity
                | ConditionOfRelease::TerminalMedicalCondition
                | ConditionOfRelease::Death
        )
    }

    /// Whether lump sum can be taken
    pub fn allows_lump_sum(&self) -> bool {
        !matches!(self, ConditionOfRelease::TransitionToRetirement)
    }
}

/// Benefit payment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BenefitPaymentType {
    /// Lump sum
    LumpSum,
    /// Income stream (pension)
    IncomeStream,
    /// Combination
    Combination,
}

/// SMSF trustee type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmsfTrusteeType {
    /// Individual trustees (2-6 members who are all trustees)
    Individual,
    /// Corporate trustee (single corporate trustee)
    Corporate,
}

/// SMSF (Self-Managed Super Fund) specific details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsfDetails {
    /// Trustee type
    pub trustee_type: SmsfTrusteeType,
    /// Corporate trustee ABN (if corporate trustee)
    pub corporate_trustee_abn: Option<String>,
    /// Registered SMSF auditor
    pub auditor_number: Option<String>,
    /// Electronic service address
    pub electronic_service_address: Option<String>,
    /// Investment strategy documented
    pub has_investment_strategy: bool,
    /// Last annual return lodged
    pub last_annual_return: Option<NaiveDate>,
    /// Last audit completed
    pub last_audit_date: Option<NaiveDate>,
}

/// Contribution cap type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContributionCapType {
    /// Concessional contributions cap
    Concessional,
    /// Non-concessional contributions cap
    NonConcessional,
    /// Bring-forward (non-concessional)
    BringForward,
}

/// Contribution caps for a financial year
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributionCaps {
    /// Financial year
    pub financial_year: String,
    /// Concessional cap
    pub concessional_cap: f64,
    /// Non-concessional cap
    pub non_concessional_cap: f64,
    /// Total superannuation balance threshold for NCC
    pub total_super_balance_threshold: f64,
    /// Transfer balance cap
    pub transfer_balance_cap: f64,
}

impl ContributionCaps {
    /// Get 2024-25 caps
    pub fn fy_2024_25() -> Self {
        Self {
            financial_year: "2024-25".to_string(),
            concessional_cap: 30_000.0,
            non_concessional_cap: 120_000.0,
            total_super_balance_threshold: 1_900_000.0,
            transfer_balance_cap: 1_900_000.0,
        }
    }

    /// Get 2025-26 caps
    pub fn fy_2025_26() -> Self {
        Self {
            financial_year: "2025-26".to_string(),
            concessional_cap: 30_000.0,
            non_concessional_cap: 120_000.0,
            total_super_balance_threshold: 1_900_000.0,
            transfer_balance_cap: 1_900_000.0,
        }
    }
}

/// SG (Superannuation Guarantee) rates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SgRate {
    /// Financial year
    pub financial_year: &'static str,
    /// SG rate as percentage
    pub rate_percent: f64,
}

/// Current and future SG rates
pub const SG_RATES: &[SgRate] = &[
    SgRate {
        financial_year: "2024-25",
        rate_percent: 11.5,
    },
    SgRate {
        financial_year: "2025-26",
        rate_percent: 12.0,
    },
];

/// Get SG rate for a financial year
pub fn sg_rate_for_year(financial_year: &str) -> f64 {
    SG_RATES
        .iter()
        .find(|r| r.financial_year == financial_year)
        .map(|r| r.rate_percent)
        .unwrap_or(12.0) // Default to 12% for future years
}

/// Maximum super contribution base (quarterly)
pub const MAX_SUPER_CONTRIBUTION_BASE_QUARTERLY_2024_25: f64 = 62_500.0;

/// SG payment due dates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SgQuarter {
    /// July-September (due 28 October)
    Q1,
    /// October-December (due 28 January)
    Q2,
    /// January-March (due 28 April)
    Q3,
    /// April-June (due 28 July)
    Q4,
}

impl SgQuarter {
    /// Get the day of month due date
    pub fn due_day(&self) -> u32 {
        28
    }

    /// Get the due month (1-12)
    pub fn due_month(&self) -> u32 {
        match self {
            SgQuarter::Q1 => 10, // October
            SgQuarter::Q2 => 1,  // January
            SgQuarter::Q3 => 4,  // April
            SgQuarter::Q4 => 7,  // July
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund_type_regulation() {
        assert!(FundType::Industry.is_apra_regulated());
        assert!(!FundType::Smsf.is_apra_regulated());
        assert!(FundType::Smsf.is_ato_regulated());
    }

    #[test]
    fn test_preservation_age() {
        let member = FundMember {
            member_id: "M001".to_string(),
            tfn_provided: true,
            date_of_birth: NaiveDate::from_ymd_opt(1965, 5, 15).unwrap(),
            join_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            category: MemberCategory::Accumulation,
            account_balance: 100_000.0,
            preserved_amount: 100_000.0,
            restricted_non_preserved: 0.0,
            unrestricted_non_preserved: 0.0,
            insurance: None,
            beneficiaries: vec![],
        };

        assert_eq!(member.preservation_age(), 60);
    }

    #[test]
    fn test_contribution_type_classification() {
        assert!(ContributionType::SuperannuationGuarantee.is_concessional());
        assert!(ContributionType::SalarySacrifice.is_concessional());
        assert!(!ContributionType::PersonalNonDeductible.is_concessional());
        assert!(ContributionType::PersonalNonDeductible.is_non_concessional());
        assert!(ContributionType::Downsizer.has_separate_cap());
    }

    #[test]
    fn test_beneficiary_tax_dependant() {
        assert!(BeneficiaryRelationship::Spouse.is_tax_dependant());
        assert!(BeneficiaryRelationship::Child.is_tax_dependant());
        assert!(!BeneficiaryRelationship::LegalPersonalRepresentative.is_tax_dependant());
    }

    #[test]
    fn test_condition_of_release_access() {
        assert!(ConditionOfRelease::Retirement.allows_full_access());
        assert!(!ConditionOfRelease::SevereFinancialHardship.allows_full_access());
        assert!(ConditionOfRelease::Retirement.allows_lump_sum());
        assert!(!ConditionOfRelease::TransitionToRetirement.allows_lump_sum());
    }

    #[test]
    fn test_contribution_caps() {
        let caps = ContributionCaps::fy_2024_25();
        assert_eq!(caps.concessional_cap, 30_000.0);
        assert_eq!(caps.non_concessional_cap, 120_000.0);
        assert_eq!(caps.transfer_balance_cap, 1_900_000.0);
    }

    #[test]
    fn test_sg_rate() {
        assert_eq!(sg_rate_for_year("2024-25"), 11.5);
        assert_eq!(sg_rate_for_year("2025-26"), 12.0);
        assert_eq!(sg_rate_for_year("2030-31"), 12.0); // Default
    }

    #[test]
    fn test_member_age_calculation() {
        let member = FundMember {
            member_id: "M001".to_string(),
            tfn_provided: true,
            date_of_birth: NaiveDate::from_ymd_opt(1980, 6, 15).unwrap(),
            join_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            category: MemberCategory::Accumulation,
            account_balance: 100_000.0,
            preserved_amount: 100_000.0,
            restricted_non_preserved: 0.0,
            unrestricted_non_preserved: 0.0,
            insurance: None,
            beneficiaries: vec![],
        };

        // Before birthday
        let before = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
        assert_eq!(member.age_at(before), 44);

        // After birthday
        let after = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
        assert_eq!(member.age_at(after), 45);
    }
}
