//! Canada Corporate Law - Types
//!
//! Core types for Canadian corporate law (CBCA, provincial corporations acts).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::{CaseCitation, Court, Province};

// ============================================================================
// Incorporation Types
// ============================================================================

/// Incorporation jurisdiction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncorporationJurisdiction {
    /// Federal (CBCA)
    Federal,
    /// Provincial
    Provincial(Province),
}

/// Corporate type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporateType {
    /// Business corporation
    BusinessCorporation,
    /// Not-for-profit corporation
    NotForProfit,
    /// Professional corporation
    Professional { profession: String },
    /// Cooperative
    Cooperative,
    /// Crown corporation
    CrownCorporation,
    /// Unlimited liability company (Nova Scotia, Alberta)
    UnlimitedLiability,
}

/// Corporate status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporateStatus {
    /// Active and in good standing
    Active,
    /// Not in good standing (annual returns overdue)
    NotInGoodStanding,
    /// Dissolved
    Dissolved { date: String },
    /// Amalgamated
    Amalgamated { successor: String },
    /// Continued out
    ContinuedOut { jurisdiction: String },
}

/// Share structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareStructure {
    /// Classes of shares
    pub classes: Vec<ShareClass>,
    /// Total authorized shares (None = unlimited)
    pub total_authorized: Option<u64>,
}

/// Share class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareClass {
    /// Class name
    pub name: String,
    /// Number authorized (None = unlimited)
    pub authorized: Option<u64>,
    /// Voting rights
    pub voting: bool,
    /// Par value (None = no par)
    pub par_value: Option<f64>,
    /// Special rights/restrictions
    pub special_rights: Vec<ShareRight>,
}

/// Share rights
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareRight {
    /// Right to dividends
    Dividend,
    /// Preference on liquidation
    LiquidationPreference,
    /// Cumulative dividends
    CumulativeDividend,
    /// Participating
    Participating,
    /// Redeemable at holder's option
    HolderRedeemable,
    /// Redeemable at corporation's option
    CorporationRedeemable,
    /// Convertible to another class
    Convertible { into_class: String },
    /// Multiple votes per share
    MultipleVotes { votes_per_share: u32 },
}

// ============================================================================
// Director and Officer Types
// ============================================================================

/// Director qualification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorQualification {
    /// Must be individual (not corporation)
    Individual,
    /// Age requirement (18+)
    MinimumAge,
    /// Mental capacity
    MentalCapacity,
    /// Not an undischarged bankrupt
    NotBankrupt,
    /// Canadian residency requirement
    CanadianResidency { percentage: u32 },
    /// Shareholding requirement (if articles require)
    Shareholding { minimum_shares: u32 },
}

/// Director disqualification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorDisqualification {
    /// Under 18 years old
    UnderAge,
    /// Mental incompetence
    MentalIncapacity,
    /// Not an individual
    NotIndividual,
    /// Undischarged bankrupt
    Bankrupt,
    /// Court order prohibiting
    CourtProhibition,
    /// Securities law prohibition
    SecuritiesProhibition,
    /// Failed residency requirement
    ResidencyNonCompliance,
}

/// Director duty type (CBCA s.122)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorDuty {
    /// Duty of care (s.122(1)(b))
    DutyOfCare,
    /// Fiduciary duty (s.122(1)(a))
    FiduciaryDuty,
    /// Duty to act honestly and in good faith
    HonestyGoodFaith,
    /// Duty to act in best interests of corporation
    BestInterests,
    /// Duty of skill
    Skill,
    /// Duty of diligence
    Diligence,
    /// Duty of prudence
    Prudence,
}

/// Breach of duty type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DutyBreach {
    /// Breach of fiduciary duty
    FiduciaryBreach(FiduciaryBreachType),
    /// Breach of duty of care
    DutyOfCareBreach,
    /// Conflict of interest
    ConflictOfInterest,
    /// Self-dealing
    SelfDealing,
    /// Corporate opportunity usurpation
    CorporateOpportunity,
    /// Disclosure failure
    DisclosureFailure,
}

/// Fiduciary breach type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiduciaryBreachType {
    /// Acting for improper purpose
    ImproperPurpose,
    /// Failing to act in best interests
    NotBestInterests,
    /// Bad faith
    BadFaith,
    /// Conflict of interest not disclosed
    UndisclosedConflict,
    /// Competing with corporation
    CompetingBusiness,
}

/// Business judgment rule elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessJudgmentElement {
    /// Decision made in good faith
    GoodFaith,
    /// Informed decision (reasonable inquiry)
    InformedDecision,
    /// No conflict of interest
    NoConflict,
    /// Rational business purpose
    RationalPurpose,
    /// Within range of reasonable alternatives
    ReasonableAlternatives,
}

// ============================================================================
// Stakeholder Types
// ============================================================================

/// Stakeholder type (BCE Inc v 1976 Debentureholders)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StakeholderType {
    /// Shareholders
    Shareholders,
    /// Creditors (secured)
    SecuredCreditors,
    /// Creditors (unsecured)
    UnsecuredCreditors,
    /// Bondholders/debentureholders
    Bondholders,
    /// Employees
    Employees,
    /// Suppliers
    Suppliers,
    /// Customers
    Customers,
    /// Community
    Community,
    /// Environment
    Environment,
}

/// Stakeholder interest balancing (BCE framework)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StakeholderInterest {
    /// Fair treatment (not necessarily equal)
    FairTreatment,
    /// Reasonable expectations
    ReasonableExpectations,
    /// Contractual rights
    ContractualRights,
    /// Statutory protections
    StatutoryProtections,
}

// ============================================================================
// Shareholder Remedies
// ============================================================================

/// Shareholder remedy type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderRemedy {
    /// Oppression remedy (s.241 CBCA)
    Oppression,
    /// Derivative action (s.239 CBCA)
    DerivativeAction,
    /// Dissent and appraisal (s.190 CBCA)
    DissentAppraisal,
    /// Compliance order (s.247 CBCA)
    ComplianceOrder,
    /// Investigation order (s.229 CBCA)
    Investigation,
    /// Winding up and liquidation
    WindingUp,
}

/// Oppression remedy elements (BCE framework)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OppressionElement {
    /// Complainant status
    ComplainantStatus,
    /// Reasonable expectations
    ReasonableExpectations,
    /// Oppressive/unfairly prejudicial/unfairly disregard
    Conduct(OppressionConduct),
}

/// Oppression conduct type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OppressionConduct {
    /// Oppressive conduct
    Oppressive,
    /// Unfairly prejudicial conduct
    UnfairlyPrejudicial,
    /// Unfairly disregards interests
    UnfairlyDisregards,
}

/// Complainant type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplainantType {
    /// Registered shareholder
    RegisteredShareholder,
    /// Beneficial shareholder
    BeneficialShareholder,
    /// Former shareholder
    FormerShareholder,
    /// Director or former director
    Director,
    /// Officer or former officer
    Officer,
    /// Creditor (discretionary)
    Creditor,
    /// Any proper person (court discretion)
    ProperPerson,
}

/// Derivative action requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivativeRequirement {
    /// Leave of court required
    LeaveOfCourt,
    /// Complainant status
    ComplainantStatus,
    /// Notice to directors
    NoticeToDirectors,
    /// Directors' failure to act
    DirectorsFailure,
    /// Good faith
    GoodFaith,
    /// Appears to be in corporation's interest
    CorporationInterest,
}

// ============================================================================
// Corporate Transactions
// ============================================================================

/// Fundamental change type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FundamentalChange {
    /// Amalgamation
    Amalgamation(AmalgamationType),
    /// Arrangement
    Arrangement,
    /// Continuance (import/export)
    Continuance { direction: ContinuanceDirection },
    /// Sale of substantially all assets
    AssetSale,
    /// Dissolution
    Dissolution,
    /// Amendment of articles
    ArticleAmendment,
}

/// Amalgamation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmalgamationType {
    /// Long-form (two or more corporations)
    LongForm,
    /// Short-form vertical (parent absorbs subsidiary)
    ShortFormVertical,
    /// Short-form horizontal (sister corporations)
    ShortFormHorizontal,
}

/// Continuance direction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinuanceDirection {
    /// Importing into jurisdiction
    Import,
    /// Exporting out of jurisdiction
    Export,
}

/// Approval requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalRequirement {
    /// Board approval only
    BoardOnly,
    /// Shareholder ordinary resolution (majority)
    OrdinaryResolution,
    /// Shareholder special resolution (2/3)
    SpecialResolution,
    /// Class vote required
    ClassVote,
    /// Court approval required
    CourtApproval,
    /// Regulatory approval required
    RegulatoryApproval,
}

// ============================================================================
// Securities and Capital Markets
// ============================================================================

/// Security type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    /// Common shares
    CommonShares,
    /// Preferred shares
    PreferredShares,
    /// Debt securities
    DebtSecurities,
    /// Convertible securities
    ConvertibleSecurities,
    /// Options/warrants
    OptionsWarrants,
    /// Units
    Units,
}

/// Reporting issuer status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportingIssuerStatus {
    /// Reporting issuer
    ReportingIssuer,
    /// Non-reporting (private)
    Private,
    /// Venture issuer
    VentureIssuer,
    /// Emerging market issuer
    EmergingMarket,
}

/// Prospectus exemption
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProspectusExemption {
    /// Accredited investor
    AccreditedInvestor,
    /// Private placement ($150K minimum)
    MinimumAmount,
    /// Family, friends, business associates
    FamilyFriends,
    /// Existing security holder
    ExistingHolder,
    /// Offering memorandum
    OfferingMemorandum,
    /// Crowdfunding
    Crowdfunding,
}

// ============================================================================
// Corporate Cases
// ============================================================================

/// Corporate law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of corporate law
    pub area: CorporateArea,
}

/// Area of corporate law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporateArea {
    /// Director duties
    DirectorDuties,
    /// Oppression remedy
    Oppression,
    /// Derivative action
    DerivativeAction,
    /// Stakeholder interests
    StakeholderInterests,
    /// Corporate veil
    CorporateVeil,
    /// Insider trading
    InsiderTrading,
    /// Takeover bids
    TakeoverBids,
}

impl CorporateCase {
    /// BCE Inc v 1976 Debentureholders \[2008\] - Stakeholder interests and oppression
    pub fn bce() -> Self {
        Self {
            citation: CaseCitation::scc(
                "BCE Inc v 1976 Debentureholders",
                2008,
                69,
                "Director duties and stakeholder interests",
            ),
            principle: "Directors owe fiduciary duty to corporation, not to any particular \
                stakeholder. In considering best interests of corporation, directors may \
                consider stakeholder interests. Oppression analysis: (1) Identify \
                reasonable expectations, (2) Determine if conduct oppressive/unfairly \
                prejudicial. Fair treatment does not require equal treatment."
                .to_string(),
            area: CorporateArea::DirectorDuties,
        }
    }

    /// Peoples Department Stores v Wise \[2004\] - Duty of care and business judgment
    pub fn peoples() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Peoples Department Stores Inc (Trustee of) v Wise",
                2004,
                68,
                "Duty of care and business judgment rule",
            ),
            principle: "Directors owe: (1) Fiduciary duty to corporation (s.122(1)(a)), \
                (2) Duty of care to corporation (s.122(1)(b)). Business judgment rule \
                protects good faith decisions within range of reasonable alternatives. \
                Creditor interests may be considered but directors don't owe them \
                fiduciary duty."
                .to_string(),
            area: CorporateArea::DirectorDuties,
        }
    }

    /// Transamerica v Torstar \[2005\] - Oppression remedy scope
    pub fn ebrahimi() -> Self {
        Self {
            citation: CaseCitation {
                name: "Ebrahimi v Westbourne Galleries Ltd".to_string(),
                year: 1973,
                neutral_citation: Some("[1973] AC 360".to_string()),
                report_citation: Some("[1973] AC 360 (HL)".to_string()),
                court: Court::Tribunal {
                    name: "House of Lords (UK - persuasive)".to_string(),
                },
                principle: "Quasi-partnership and legitimate expectations".to_string(),
            },
            principle: "In quasi-partnership, shareholders have legitimate expectations \
                beyond strict legal rights. Exclusion from management may be oppressive \
                even if technically lawful. Winding up on just and equitable grounds \
                available when relationship of trust breaks down."
                .to_string(),
            area: CorporateArea::Oppression,
        }
    }

    /// Kosmopoulos v Constitution Insurance \[1987\] - Corporate veil
    pub fn kosmopoulos() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Kosmopoulos v Constitution Insurance Co",
                1987,
                2,
                "Insurable interest and corporate veil",
            ),
            principle: "Corporation has separate legal personality from shareholders \
                (Salomon principle). Shareholder's interest in corporate property is \
                not insurable interest. Corporate veil pierced only in exceptional \
                circumstances (fraud, agency, instrument)."
                .to_string(),
            area: CorporateArea::CorporateVeil,
        }
    }

    /// UPM-Kymmene v UPM-Kymmene Miramichi \[2004\] - Oppression remedy
    pub fn upm_kymmene() -> Self {
        Self {
            citation: CaseCitation {
                name: "UPM-Kymmene Corp v UPM-Kymmene Miramichi Inc".to_string(),
                year: 2004,
                neutral_citation: Some("2004 ONCA".to_string()),
                report_citation: Some("183 OAC 310".to_string()),
                court: Court::ProvincialCourtOfAppeal {
                    province: Province::Ontario,
                },
                principle: "Oppression remedy - share valuation".to_string(),
            },
            principle: "In oppression cases, court may order shares to be purchased at \
                fair value. Valuation date typically when oppression began. Minority \
                discount not applied where oppression caused need for buyout. \
                Remedy should be tailored to facts."
                .to_string(),
            area: CorporateArea::Oppression,
        }
    }

    /// Foss v Harbottle \[1843\] - Proper plaintiff rule
    pub fn foss_v_harbottle() -> Self {
        Self {
            citation: CaseCitation {
                name: "Foss v Harbottle".to_string(),
                year: 1843,
                neutral_citation: None,
                report_citation: Some("(1843) 67 ER 189".to_string()),
                court: Court::Tribunal {
                    name: "Court of Chancery (UK - foundational)".to_string(),
                },
                principle: "Proper plaintiff rule in derivative actions".to_string(),
            },
            principle: "The proper plaintiff for wrongs to corporation is corporation \
                itself. Majority rule - court won't interfere with internal management \
                if ratifiable. Exceptions: ultra vires, fraud on minority, personal \
                rights. Derivative action codified in CBCA s.239."
                .to_string(),
            area: CorporateArea::DerivativeAction,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corporate_type() {
        let business = CorporateType::BusinessCorporation;
        let nfp = CorporateType::NotForProfit;
        assert_ne!(business, nfp);
    }

    #[test]
    fn test_director_duty() {
        let care = DirectorDuty::DutyOfCare;
        let fiduciary = DirectorDuty::FiduciaryDuty;
        assert_ne!(care, fiduciary);
    }

    #[test]
    fn test_shareholder_remedy() {
        let oppression = ShareholderRemedy::Oppression;
        let derivative = ShareholderRemedy::DerivativeAction;
        assert_ne!(oppression, derivative);
    }

    #[test]
    fn test_bce_case() {
        let case = CorporateCase::bce();
        assert_eq!(case.citation.year, 2008);
        assert!(case.principle.contains("stakeholder"));
    }

    #[test]
    fn test_peoples_case() {
        let case = CorporateCase::peoples();
        assert_eq!(case.area, CorporateArea::DirectorDuties);
        assert!(case.principle.contains("Business judgment"));
    }

    #[test]
    fn test_kosmopoulos_case() {
        let case = CorporateCase::kosmopoulos();
        assert!(case.principle.contains("Salomon"));
    }

    #[test]
    fn test_foss_v_harbottle_case() {
        let case = CorporateCase::foss_v_harbottle();
        assert!(case.principle.contains("proper plaintiff"));
    }

    #[test]
    fn test_fundamental_change() {
        let amalg = FundamentalChange::Amalgamation(AmalgamationType::LongForm);
        let arrangement = FundamentalChange::Arrangement;
        assert_ne!(format!("{:?}", amalg), format!("{:?}", arrangement));
    }

    #[test]
    fn test_stakeholder_type() {
        let shareholders = StakeholderType::Shareholders;
        let creditors = StakeholderType::SecuredCreditors;
        assert_ne!(shareholders, creditors);
    }
}
