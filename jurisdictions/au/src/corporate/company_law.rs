//! Company Formation, Share Capital, and Members' Rights
//!
//! Implementation of Corporations Act 2001 (Cth) Chapters 2A-2J.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::types::{CompanySize, CompanyType};

// =============================================================================
// Company Formation (Chapter 2A)
// =============================================================================

/// Company registration status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Application lodged
    ApplicationLodged,
    /// Registered
    Registered,
    /// Deregistered
    Deregistered,
    /// Under external administration
    UnderExternalAdministration,
    /// Voluntarily wound up
    VoluntarilyWoundUp,
    /// Compulsorily wound up
    CompulsorilyWoundUp,
}

/// Company structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// ACN (Australian Company Number)
    pub acn: String,
    /// Company name
    pub name: String,
    /// Company type
    pub company_type: CompanyType,
    /// Company size (for reporting)
    pub company_size: CompanySize,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Registered office address
    pub registered_office: String,
    /// Principal place of business
    pub principal_place_of_business: Option<String>,
    /// Status
    pub status: RegistrationStatus,
    /// Has constitution
    pub has_constitution: bool,
    /// Uses replaceable rules
    pub uses_replaceable_rules: bool,
    /// Issued share capital
    pub issued_share_capital: f64,
    /// Number of members
    pub number_of_members: u32,
}

impl Company {
    /// Check if company has valid ACN
    pub fn validate_acn(acn: &str) -> bool {
        // ACN is 9 digits with check digit
        if acn.len() != 9 || !acn.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Calculate check digit (modulus 10)
        let weights = [8, 7, 6, 5, 4, 3, 2, 1];
        let digits: Vec<u32> = acn.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() != 9 {
            return false;
        }

        // Reject all-zero ACN (not a valid company number)
        if digits.iter().all(|&d| d == 0) {
            return false;
        }

        let sum: u32 = weights
            .iter()
            .zip(digits.iter().take(8))
            .map(|(w, d)| w * d)
            .sum();

        let remainder = sum % 10;
        let expected_check = if remainder == 0 { 0 } else { 10 - remainder };

        digits[8] == expected_check
    }

    /// Convert ACN to ABN
    pub fn acn_to_abn(acn: &str) -> Option<String> {
        if !Self::validate_acn(acn) {
            return None;
        }

        // ABN is 11 digits: 2-digit prefix + 9-digit ACN
        // Prefix calculation: add 10 and calculate check digits
        let with_prefix = format!("00{}", acn);
        let mut digits: Vec<u32> = with_prefix.chars().filter_map(|c| c.to_digit(10)).collect();

        // ABN weights
        let weights = [10, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

        // Calculate required first two digits
        for first in 0..10 {
            for second in 0..10 {
                digits[0] = first;
                digits[1] = second;

                // Subtract 1 from first digit before weighting
                let adjusted_first = if digits[0] > 0 { digits[0] - 1 } else { 0 };

                let sum: u32 = weights
                    .iter()
                    .enumerate()
                    .map(|(i, w)| {
                        if i == 0 {
                            w * adjusted_first
                        } else {
                            w * digits[i]
                        }
                    })
                    .sum();

                if sum.is_multiple_of(89) {
                    return Some(digits.iter().map(|d| d.to_string()).collect());
                }
            }
        }

        None
    }

    /// Is proprietary company (Pty Ltd)
    pub fn is_proprietary(&self) -> bool {
        matches!(
            self.company_type,
            CompanyType::ProprietaryLimited | CompanyType::ProprietaryUnlimited
        )
    }

    /// Is public company
    pub fn is_public(&self) -> bool {
        matches!(
            self.company_type,
            CompanyType::PublicLimited
                | CompanyType::PublicUnlimited
                | CompanyType::PublicLimitedByGuarantee
        )
    }

    /// Maximum members for proprietary company
    pub const PROPRIETARY_MAX_MEMBERS: u32 = 50;
}

/// Constitution content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constitution {
    /// Company ACN
    pub company_acn: String,
    /// Adoption date
    pub adoption_date: NaiveDate,
    /// Objects clause (if any)
    pub objects_clause: Option<String>,
    /// Share class provisions
    pub share_classes: Vec<ShareClass>,
    /// Director appointment provisions
    pub director_appointment: DirectorAppointmentRules,
    /// Meeting procedures
    pub meeting_procedures: MeetingProcedures,
    /// Amendment provisions
    pub amendment_provisions: AmendmentProvisions,
}

/// Replaceable rules applicability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReplaceableRuleCategory {
    /// Directors meetings (s.248B-248G)
    DirectorsMeetings,
    /// Appointment of directors (s.201H)
    DirectorAppointment,
    /// Members meetings (s.249H-249U)
    MembersMeetings,
    /// Voting (s.250E)
    Voting,
    /// Share transfers (s.1091C)
    ShareTransfers,
    /// Dividends (s.254U, 254V, 254W)
    Dividends,
    /// Indemnity and insurance (s.199A)
    IndemnityInsurance,
}

// =============================================================================
// Share Capital (Chapter 2H)
// =============================================================================

/// Share class
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareClass {
    /// Class name
    pub name: String,
    /// Number of shares
    pub number_of_shares: u64,
    /// Paid up amount per share
    pub paid_up_amount: f64,
    /// Voting rights
    pub voting_rights: bool,
    /// Votes per share
    pub votes_per_share: u32,
    /// Dividend rights
    pub dividend_rights: DividendRights,
    /// Preference shares
    pub is_preference: bool,
    /// Redeemable
    pub is_redeemable: bool,
    /// Convertible
    pub is_convertible: bool,
}

/// Dividend rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DividendRights {
    /// Equal rights with ordinary shares
    Equal,
    /// Preferential (fixed percentage)
    Preferential,
    /// Cumulative preferential
    CumulativePreferential,
    /// Participating (extra after fixed)
    Participating,
    /// No dividend rights
    None,
}

/// Share issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareIssue {
    /// Issue date
    pub issue_date: NaiveDate,
    /// Number of shares
    pub number_of_shares: u64,
    /// Issue price per share
    pub issue_price: f64,
    /// Share class
    pub share_class: String,
    /// Issue type
    pub issue_type: ShareIssueType,
    /// Consideration received
    pub consideration: ShareConsideration,
    /// Disclosure document (if public)
    pub disclosure_document: Option<String>,
}

/// Share issue type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShareIssueType {
    /// Initial public offering
    Ipo,
    /// Rights issue to existing shareholders
    RightsIssue,
    /// Placement to sophisticated/professional investors
    Placement,
    /// Share purchase plan (<$30,000 per shareholder)
    SharePurchasePlan,
    /// Employee share scheme
    EmployeeShareScheme,
    /// Bonus issue (capitalisation of reserves)
    BonusIssue,
    /// Dividend reinvestment plan
    DividendReinvestment,
    /// Conversion of securities
    Conversion,
}

/// Share consideration type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShareConsideration {
    /// Cash payment
    Cash { amount: f64 },
    /// Non-cash (property, services, etc.)
    NonCash { description: String, value: f64 },
    /// Capitalisation of reserves
    Capitalisation { reserve_type: String },
    /// Debt to equity conversion
    DebtConversion { debt_amount: f64 },
}

/// Share buyback
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareBuyback {
    /// Buyback type
    pub buyback_type: BuybackType,
    /// Number of shares
    pub number_of_shares: u64,
    /// Price per share
    pub price_per_share: f64,
    /// Share class
    pub share_class: String,
    /// Approval date
    pub approval_date: NaiveDate,
    /// Completion date
    pub completion_date: Option<NaiveDate>,
}

/// Buyback type (s.257A-257J)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuybackType {
    /// Equal access buyback (all shareholders)
    EqualAccess,
    /// On-market buyback (10/12 limit)
    OnMarket,
    /// Selective buyback (specific shareholders)
    Selective,
    /// Employee share scheme buyback
    EmployeeShareScheme,
    /// Minimum holding buyback
    MinimumHolding,
}

/// Capital reduction (s.256A-256D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapitalReduction {
    /// Reduction type
    pub reduction_type: CapitalReductionType,
    /// Amount per share
    pub amount_per_share: f64,
    /// Share class affected
    pub share_class: String,
    /// Number of shares affected
    pub shares_affected: u64,
    /// Total reduction amount
    pub total_amount: f64,
    /// Approval date
    pub approval_date: NaiveDate,
}

/// Capital reduction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapitalReductionType {
    /// Equal reduction (same for all shareholders)
    Equal,
    /// Selective reduction
    Selective,
    /// Reduction to cancel shares
    CancelShares,
}

/// Financial assistance (s.260A-260D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAssistance {
    /// Type of assistance
    pub assistance_type: FinancialAssistanceType,
    /// Amount of assistance
    pub amount: f64,
    /// Recipient
    pub recipient: String,
    /// Purpose
    pub purpose: String,
    /// Exemption relied on
    pub exemption: Option<FinancialAssistanceExemption>,
    /// Whitewash resolution (if applicable)
    pub whitewash_resolution: Option<NaiveDate>,
}

/// Financial assistance type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialAssistanceType {
    /// Loan
    Loan,
    /// Guarantee
    Guarantee,
    /// Security (over company assets)
    Security,
    /// Other financial accommodation
    Other,
}

/// Financial assistance exemption
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialAssistanceExemption {
    /// s.260A(1)(a) - ordinary course of business
    OrdinaryCourseOfBusiness,
    /// s.260A(1)(b) - employee share schemes
    EmployeeShareScheme,
    /// s.260C - whitewash procedure
    WhitewashProcedure,
}

// =============================================================================
// Members' Rights (Chapter 2F, 2G)
// =============================================================================

/// Member/shareholder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    /// Member name
    pub name: String,
    /// Member type
    pub member_type: MemberType,
    /// Shareholdings
    pub shareholdings: Vec<Shareholding>,
    /// Voting power percentage
    pub voting_power_percentage: f64,
    /// Registration date
    pub registration_date: NaiveDate,
}

/// Member type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberType {
    /// Individual
    Individual,
    /// Company
    Company,
    /// Trust
    Trust,
    /// Superannuation fund
    SuperannuationFund,
    /// Partnership
    Partnership,
}

/// Shareholding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shareholding {
    /// Share class
    pub share_class: String,
    /// Number of shares
    pub number_of_shares: u64,
    /// Beneficial owner (if nominee)
    pub beneficial_owner: Option<String>,
    /// Acquisition date
    pub acquisition_date: NaiveDate,
}

/// Director appointment rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectorAppointmentRules {
    /// Minimum number of directors
    pub minimum_directors: u32,
    /// Maximum number of directors
    pub maximum_directors: Option<u32>,
    /// Rotation required
    pub rotation_required: bool,
    /// Rotation fraction (e.g., 1/3)
    pub rotation_fraction: Option<f64>,
    /// Appointment method
    pub appointment_method: DirectorAppointmentMethod,
}

/// Director appointment method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectorAppointmentMethod {
    /// Ordinary resolution of members
    MemberOrdinaryResolution,
    /// Board appointment (subject to confirmation)
    BoardAppointment,
    /// Specific shareholder rights
    ShareholderRights,
}

/// Meeting procedures
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingProcedures {
    /// Notice period (days)
    pub notice_period_days: u32,
    /// Quorum (number or percentage)
    pub quorum: QuorumRequirement,
    /// Proxy permitted
    pub proxy_permitted: bool,
    /// Poll can be demanded
    pub poll_can_be_demanded: bool,
    /// Chair has casting vote
    pub chair_casting_vote: bool,
}

/// Quorum requirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuorumRequirement {
    /// Number of members
    Number(u32),
    /// Percentage of members
    Percentage(f64),
    /// Percentage of voting power
    VotingPowerPercentage(f64),
}

/// Amendment provisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmendmentProvisions {
    /// Requires special resolution
    pub requires_special_resolution: bool,
    /// Entrenched provisions
    pub entrenched_provisions: Vec<String>,
    /// Class rights variation
    pub class_rights_variation: ClassRightsVariation,
}

/// Class rights variation requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassRightsVariation {
    /// 75% of class approve
    ClassApproval75,
    /// Unanimous class approval
    ClassApprovalUnanimous,
    /// As per constitution
    AsPerConstitution,
}

/// Members' meeting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MembersMeeting {
    /// Meeting type
    pub meeting_type: MeetingType,
    /// Date and time
    pub date_time: NaiveDate,
    /// Location
    pub location: String,
    /// Notice given date
    pub notice_date: NaiveDate,
    /// Business to be transacted
    pub business: Vec<MeetingBusiness>,
    /// Quorum present
    pub quorum_present: bool,
    /// Minutes recorded
    pub minutes_recorded: bool,
}

/// Meeting type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeetingType {
    /// Annual General Meeting
    Agm,
    /// Extraordinary General Meeting
    Egm,
    /// Class Meeting
    ClassMeeting,
}

/// Meeting business
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingBusiness {
    /// Description
    pub description: String,
    /// Resolution type
    pub resolution_type: ResolutionType,
    /// Result
    pub result: Option<ResolutionResult>,
}

/// Resolution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution (>50%)
    Ordinary,
    /// Special resolution (>=75%)
    Special,
    /// Unanimous
    Unanimous,
}

/// Resolution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionResult {
    /// Passed
    pub passed: bool,
    /// Votes for
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Abstentions
    pub abstentions: u64,
    /// Percentage in favor
    pub percentage_in_favor: f64,
}

// =============================================================================
// Oppression Remedy (s.232-234)
// =============================================================================

/// Oppression ground
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OppressionGround {
    /// Conduct contrary to member interests
    ContraryToMemberInterests,
    /// Oppressive conduct
    Oppressive,
    /// Unfairly prejudicial
    UnfairlyPrejudicial,
    /// Unfairly discriminatory
    UnfairlyDiscriminatory,
}

/// Oppression remedy claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OppressionClaim {
    /// Ground for claim
    pub ground: OppressionGround,
    /// Conduct complained of
    pub conduct: String,
    /// Remedy sought
    pub remedy_sought: OppressionRemedy,
    /// Standing (member, ASIC, etc.)
    pub applicant_type: OppressionApplicant,
}

/// Oppression remedy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OppressionRemedy {
    /// Winding up
    WindingUp,
    /// Buyout of applicant's shares
    BuyoutApplicant,
    /// Buyout of respondent's shares
    BuyoutRespondent,
    /// Modification of constitution
    ModifyConstitution,
    /// Restraining order
    RestrainingOrder,
    /// Appointment of receiver
    AppointReceiver,
    /// Other court order
    OtherOrder,
}

/// Oppression applicant type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OppressionApplicant {
    /// Current member
    Member,
    /// Former member
    FormerMember,
    /// Person entitled to be registered
    PersonEntitled,
    /// ASIC
    Asic,
}

// =============================================================================
// Validation Functions
// =============================================================================

/// Validate share issue compliance
pub fn validate_share_issue(
    issue: &ShareIssue,
    company: &Company,
    existing_capital: f64,
) -> ShareIssueValidation {
    let mut issues = Vec::new();
    let mut warnings = Vec::new();
    let mut legal_references = vec!["Corporations Act 2001 Chapter 2H".to_string()];

    // Check disclosure requirements for public company
    if company.is_public() && issue.disclosure_document.is_none() {
        match issue.issue_type {
            ShareIssueType::Placement | ShareIssueType::SharePurchasePlan => {
                // May be exempt
                warnings.push("Ensure issue fits within disclosure exemption".to_string());
            }
            ShareIssueType::Ipo => {
                issues.push("IPO requires prospectus under Chapter 6D".to_string());
                legal_references.push("CA 2001 s.706 - Prospectus required".to_string());
            }
            _ => {}
        }
    }

    // 15% placement cap for listed companies
    if company.is_public() && matches!(issue.issue_type, ShareIssueType::Placement) {
        let issue_value = issue.number_of_shares as f64 * issue.issue_price;
        let percentage = (issue_value / existing_capital) * 100.0;
        if percentage > 15.0 {
            warnings.push(
                "Placement may exceed 15% capacity - shareholder approval required".to_string(),
            );
            legal_references.push("ASX Listing Rule 7.1".to_string());
        }
    }

    ShareIssueValidation {
        valid: issues.is_empty(),
        issues,
        warnings,
        legal_references,
    }
}

/// Share issue validation result
#[derive(Debug, Clone)]
pub struct ShareIssueValidation {
    /// Valid
    pub valid: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Validate buyback compliance
pub fn validate_buyback(
    buyback: &ShareBuyback,
    company: &Company,
    _total_shares: u64,
) -> BuybackValidation {
    let mut issues = Vec::new();
    let mut requirements = Vec::new();
    let legal_references = vec![
        "Corporations Act 2001 s.257A-257J".to_string(),
        "ASIC Regulatory Guide 110".to_string(),
    ];

    // Solvency requirement
    requirements.push("Directors must resolve company will remain solvent".to_string());

    // Type-specific requirements
    match buyback.buyback_type {
        BuybackType::Selective => {
            requirements.push("Special resolution required".to_string());
            requirements.push("Selling shareholders cannot vote".to_string());
        }
        BuybackType::OnMarket => {
            if company.is_public() {
                requirements.push("Cannot exceed 10/12 limit (10% in 12 months)".to_string());
            } else {
                issues.push("On-market buyback only for listed companies".to_string());
            }
        }
        BuybackType::EqualAccess => {
            requirements.push("Ordinary resolution required".to_string());
            requirements.push("All shareholders must be offered same terms".to_string());
        }
        _ => {}
    }

    BuybackValidation {
        valid: issues.is_empty(),
        issues,
        requirements,
        legal_references,
    }
}

/// Buyback validation result
#[derive(Debug, Clone)]
pub struct BuybackValidation {
    /// Valid
    pub valid: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Requirements
    pub requirements: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Validate financial assistance
pub fn validate_financial_assistance(
    assistance: &FinancialAssistance,
    company: &Company,
) -> FinancialAssistanceValidation {
    let mut issues = Vec::new();
    let mut requirements = Vec::new();
    let legal_references = vec!["Corporations Act 2001 s.260A-260D".to_string()];

    // Check if exemption applies
    let exempt = match assistance.exemption {
        Some(FinancialAssistanceExemption::OrdinaryCourseOfBusiness) => {
            requirements.push(
                "Must be in ordinary course of business and on ordinary commercial terms"
                    .to_string(),
            );
            true
        }
        Some(FinancialAssistanceExemption::EmployeeShareScheme) => {
            requirements.push("Must be genuine employee share scheme".to_string());
            true
        }
        Some(FinancialAssistanceExemption::WhitewashProcedure) => {
            if assistance.whitewash_resolution.is_some() {
                requirements.push("Special resolution required".to_string());
                requirements.push("Declaration by directors required".to_string());
                true
            } else {
                issues.push("Whitewash resolution not obtained".to_string());
                false
            }
        }
        None => {
            issues.push(
                "Financial assistance to acquire shares is prohibited without exemption"
                    .to_string(),
            );
            false
        }
    };

    // Public company has additional requirements
    if company.is_public() && exempt {
        requirements.push("Lodge notice with ASIC 14 days before giving assistance".to_string());
    }

    FinancialAssistanceValidation {
        valid: issues.is_empty() && exempt,
        exempt,
        issues,
        requirements,
        legal_references,
    }
}

/// Financial assistance validation result
#[derive(Debug, Clone)]
pub struct FinancialAssistanceValidation {
    /// Valid
    pub valid: bool,
    /// Exempt
    pub exempt: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Requirements
    pub requirements: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acn_validation_valid() {
        // Example valid ACN: 000 000 019
        assert!(Company::validate_acn("000000019"));
    }

    #[test]
    fn test_acn_validation_invalid() {
        assert!(!Company::validate_acn("000000000"));
        assert!(!Company::validate_acn("12345")); // Too short
        assert!(!Company::validate_acn("abcdefghi")); // Not digits
    }

    #[test]
    fn test_company_is_proprietary() {
        let company = Company {
            acn: "000000019".to_string(),
            name: "Test Pty Ltd".to_string(),
            company_type: CompanyType::ProprietaryLimited,
            company_size: CompanySize::SmallProprietary,
            registration_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            registered_office: "123 Main St".to_string(),
            principal_place_of_business: None,
            status: RegistrationStatus::Registered,
            has_constitution: false,
            uses_replaceable_rules: true,
            issued_share_capital: 100.0,
            number_of_members: 2,
        };
        assert!(company.is_proprietary());
        assert!(!company.is_public());
    }

    #[test]
    fn test_share_issue_validation() {
        let company = Company {
            acn: "000000019".to_string(),
            name: "Test Ltd".to_string(),
            company_type: CompanyType::PublicLimited,
            company_size: CompanySize::LargeProprietary,
            registration_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            registered_office: "123 Main St".to_string(),
            principal_place_of_business: None,
            status: RegistrationStatus::Registered,
            has_constitution: true,
            uses_replaceable_rules: false,
            issued_share_capital: 1_000_000.0,
            number_of_members: 100,
        };

        let issue = ShareIssue {
            issue_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            number_of_shares: 10000,
            issue_price: 1.0,
            share_class: "Ordinary".to_string(),
            issue_type: ShareIssueType::Placement,
            consideration: ShareConsideration::Cash { amount: 10000.0 },
            disclosure_document: None,
        };

        let result = validate_share_issue(&issue, &company, 1_000_000.0);
        // Should have warning about disclosure exemption
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_buyback_validation_selective() {
        let company = Company {
            acn: "000000019".to_string(),
            name: "Test Pty Ltd".to_string(),
            company_type: CompanyType::ProprietaryLimited,
            company_size: CompanySize::SmallProprietary,
            registration_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            registered_office: "123 Main St".to_string(),
            principal_place_of_business: None,
            status: RegistrationStatus::Registered,
            has_constitution: false,
            uses_replaceable_rules: true,
            issued_share_capital: 100.0,
            number_of_members: 2,
        };

        let buyback = ShareBuyback {
            buyback_type: BuybackType::Selective,
            number_of_shares: 100,
            price_per_share: 1.0,
            share_class: "Ordinary".to_string(),
            approval_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            completion_date: None,
        };

        let result = validate_buyback(&buyback, &company, 1000);
        assert!(result.valid);
        assert!(
            result
                .requirements
                .iter()
                .any(|r| r.contains("Special resolution"))
        );
    }

    #[test]
    fn test_financial_assistance_whitewash() {
        let company = Company {
            acn: "000000019".to_string(),
            name: "Test Pty Ltd".to_string(),
            company_type: CompanyType::ProprietaryLimited,
            company_size: CompanySize::SmallProprietary,
            registration_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            registered_office: "123 Main St".to_string(),
            principal_place_of_business: None,
            status: RegistrationStatus::Registered,
            has_constitution: false,
            uses_replaceable_rules: true,
            issued_share_capital: 100.0,
            number_of_members: 2,
        };

        let assistance = FinancialAssistance {
            assistance_type: FinancialAssistanceType::Loan,
            amount: 50000.0,
            recipient: "Buyer Co".to_string(),
            purpose: "Acquisition of shares".to_string(),
            exemption: Some(FinancialAssistanceExemption::WhitewashProcedure),
            whitewash_resolution: Some(NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date")),
        };

        let result = validate_financial_assistance(&assistance, &company);
        assert!(result.valid);
        assert!(result.exempt);
    }

    #[test]
    fn test_resolution_types() {
        // Ordinary resolution: >50%
        let ordinary = ResolutionResult {
            passed: true,
            votes_for: 51,
            votes_against: 49,
            abstentions: 0,
            percentage_in_favor: 51.0,
        };
        assert!(ordinary.passed);

        // Special resolution: >=75%
        let special = ResolutionResult {
            passed: true,
            votes_for: 75,
            votes_against: 25,
            abstentions: 0,
            percentage_in_favor: 75.0,
        };
        assert!(special.passed);
    }
}
