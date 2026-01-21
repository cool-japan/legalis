//! Bankruptcy Law Types (Bankruptcy Code Title 11)
//!
//! This module provides types for US bankruptcy law under Title 11 of the United States Code.

#![allow(missing_docs)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Bankruptcy case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankruptcyCase {
    /// Case number
    pub case_number: String,

    /// Chapter under which case is filed
    pub chapter: BankruptcyChapter,

    /// Debtor information
    pub debtor: Debtor,

    /// Filing date (petition date)
    pub filing_date: NaiveDate,

    /// Case status
    pub status: CaseStatus,

    /// Bankruptcy court district
    pub court_district: String,

    /// Assigned judge
    pub judge: Option<String>,

    /// Trustee (if appointed)
    pub trustee: Option<Trustee>,

    /// Automatic stay in effect
    pub automatic_stay_active: bool,

    /// Estate assets
    pub estate: Option<BankruptcyEstate>,

    /// Schedule of creditors and claims
    pub creditors: Vec<Creditor>,

    /// Discharge information (if applicable)
    pub discharge: Option<Discharge>,

    /// Conversion information (if case converted to different chapter)
    pub conversion: Option<ChapterConversion>,
}

impl BankruptcyCase {
    /// Check if automatic stay is in effect
    pub fn has_automatic_stay(&self) -> bool {
        self.automatic_stay_active
            && matches!(
                self.status,
                CaseStatus::Filed | CaseStatus::Pending | CaseStatus::ActiveReorganization
            )
    }

    /// Check if debtor is eligible for discharge
    pub fn is_eligible_for_discharge(&self) -> bool {
        match self.chapter {
            BankruptcyChapter::Chapter7 => {
                // Chapter 7 discharge eligibility
                !self.debtor.has_prior_discharge_within_8_years()
            }
            BankruptcyChapter::Chapter13 => {
                // Chapter 13 discharge after plan completion
                !self.debtor.has_prior_discharge_within_period(2, 4)
            }
            BankruptcyChapter::Chapter11 => {
                // Chapter 11 discharge upon plan confirmation
                true
            }
            _ => false,
        }
    }

    /// Total scheduled debt
    pub fn total_debt(&self) -> f64 {
        self.creditors.iter().map(|c| c.claim_amount).sum()
    }

    /// Total secured debt
    pub fn secured_debt(&self) -> f64 {
        self.creditors
            .iter()
            .filter(|c| matches!(c.claim_type, ClaimType::Secured { .. }))
            .map(|c| c.claim_amount)
            .sum()
    }

    /// Total unsecured debt
    pub fn unsecured_debt(&self) -> f64 {
        self.creditors
            .iter()
            .filter(|c| {
                matches!(
                    c.claim_type,
                    ClaimType::UnsecuredPriority { .. } | ClaimType::UnsecuredNonPriority
                )
            })
            .map(|c| c.claim_amount)
            .sum()
    }
}

/// Bankruptcy chapters under Title 11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankruptcyChapter {
    /// Chapter 7: Liquidation
    Chapter7,

    /// Chapter 9: Municipality debt adjustment
    Chapter9,

    /// Chapter 11: Reorganization (business or individual)
    Chapter11,

    /// Chapter 12: Family farmer or fisherman debt adjustment
    Chapter12,

    /// Chapter 13: Individual debt adjustment (wage earner plan)
    Chapter13,

    /// Chapter 15: Ancillary and cross-border cases
    Chapter15,
}

impl BankruptcyChapter {
    /// Get the typical duration of the chapter
    pub fn typical_duration_months(&self) -> Option<u32> {
        match self {
            BankruptcyChapter::Chapter7 => Some(4),   // 4-6 months
            BankruptcyChapter::Chapter13 => Some(36), // 3-5 years
            BankruptcyChapter::Chapter11 => Some(18), // 1.5-2 years (varies widely)
            BankruptcyChapter::Chapter12 => Some(36), // 3-5 years
            _ => None,
        }
    }

    /// Check if chapter allows individual debtors
    pub fn allows_individuals(&self) -> bool {
        matches!(
            self,
            BankruptcyChapter::Chapter7
                | BankruptcyChapter::Chapter11
                | BankruptcyChapter::Chapter12
                | BankruptcyChapter::Chapter13
        )
    }

    /// Check if chapter allows business debtors
    pub fn allows_businesses(&self) -> bool {
        matches!(
            self,
            BankruptcyChapter::Chapter7 | BankruptcyChapter::Chapter11
        )
    }
}

/// Debtor in bankruptcy case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debtor {
    /// Debtor name (individual or business)
    pub name: String,

    /// Debtor type
    pub debtor_type: DebtorType,

    /// Social Security Number or EIN (last 4 digits only for privacy)
    pub tax_id_last_four: String,

    /// Address
    pub address: String,

    /// Date of birth (for individuals)
    pub date_of_birth: Option<NaiveDate>,

    /// Employment status (for individuals)
    pub employment_status: Option<EmploymentStatus>,

    /// Monthly income
    pub monthly_income: f64,

    /// Monthly expenses
    pub monthly_expenses: f64,

    /// Total assets (from Schedule A/B)
    pub total_assets: f64,

    /// Total liabilities (from Schedule D/E/F)
    pub total_liabilities: f64,

    /// Prior bankruptcy filings
    pub prior_bankruptcies: Vec<PriorBankruptcy>,

    /// Whether debtor is an individual or joint filing
    pub is_joint_filing: bool,
}

impl Debtor {
    /// Calculate disposable income
    pub fn disposable_income(&self) -> f64 {
        (self.monthly_income - self.monthly_expenses).max(0.0)
    }

    /// Check if debtor passes means test (Chapter 7)
    pub fn passes_means_test(&self, median_income: f64) -> bool {
        self.monthly_income * 12.0 <= median_income
    }

    /// Check if debtor has prior discharge within specified years
    pub fn has_prior_discharge_within_8_years(&self) -> bool {
        self.prior_bankruptcies.iter().any(|pb| {
            pb.discharge_date
                .map(|d| (chrono::Utc::now().naive_utc().date() - d).num_days() < 365 * 8)
                .unwrap_or(false)
        })
    }

    /// Check prior discharge within period (for Chapter 13)
    pub fn has_prior_discharge_within_period(&self, ch7_years: i64, ch13_years: i64) -> bool {
        self.prior_bankruptcies.iter().any(|pb| {
            let years = match pb.chapter {
                BankruptcyChapter::Chapter7 => ch7_years,
                BankruptcyChapter::Chapter13 => ch13_years,
                _ => return false,
            };

            pb.discharge_date
                .map(|d| (chrono::Utc::now().naive_utc().date() - d).num_days() < 365 * years)
                .unwrap_or(false)
        })
    }
}

/// Type of debtor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebtorType {
    /// Individual consumer debtor
    Individual,

    /// Joint filing (married couple)
    JointIndividual,

    /// Corporation
    Corporation,

    /// Partnership
    Partnership,

    /// Limited liability company
    Llc,

    /// Sole proprietorship
    SoleProprietorship,

    /// Municipality (Chapter 9 only)
    Municipality,

    /// Family farmer (Chapter 12)
    FamilyFarmer,

    /// Family fisherman (Chapter 12)
    FamilyFisherman,
}

/// Employment status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentStatus {
    pub employed: bool,
    pub employer: Option<String>,
    pub occupation: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
}

/// Prior bankruptcy filing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorBankruptcy {
    pub case_number: String,
    pub chapter: BankruptcyChapter,
    pub filing_date: NaiveDate,
    pub discharge_date: Option<NaiveDate>,
    pub dismissed: bool,
    pub court_district: String,
}

/// Case status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseStatus {
    /// Petition filed
    Filed,

    /// Case pending (ongoing)
    Pending,

    /// Active reorganization plan (Chapter 11/13)
    ActiveReorganization,

    /// Discharged
    Discharged,

    /// Dismissed
    Dismissed,

    /// Converted to different chapter
    Converted,

    /// Closed
    Closed,

    /// Reopened
    Reopened,
}

/// Trustee appointed in bankruptcy case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trustee {
    /// Trustee name
    pub name: String,

    /// Trustee type
    pub trustee_type: TrusteeType,

    /// Appointment date
    pub appointment_date: NaiveDate,

    /// Contact information
    pub contact: String,
}

/// Type of trustee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrusteeType {
    /// Chapter 7 trustee (liquidation)
    Chapter7Trustee,

    /// Chapter 11 trustee (rare - usually debtor in possession)
    Chapter11Trustee,

    /// Chapter 12 trustee (family farmer/fisherman)
    Chapter12Trustee,

    /// Chapter 13 trustee (individual reorganization)
    Chapter13Trustee,

    /// US Trustee (Department of Justice)
    UsTrustee,
}

/// Bankruptcy estate (property of the estate under Section 541)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankruptcyEstate {
    /// All property interests as of petition date
    pub assets: Vec<Asset>,

    /// Exempt property (not part of estate under Section 522)
    pub exemptions: Vec<Exemption>,

    /// Estate's equity in property
    pub total_equity: f64,

    /// Administrative expenses
    pub administrative_expenses: f64,
}

impl BankruptcyEstate {
    /// Calculate total asset value
    pub fn total_asset_value(&self) -> f64 {
        self.assets.iter().map(|a| a.current_value).sum()
    }

    /// Calculate total exempt property
    pub fn total_exemptions(&self) -> f64 {
        self.exemptions.iter().map(|e| e.exemption_amount).sum()
    }

    /// Calculate non-exempt property available to creditors
    pub fn available_to_creditors(&self) -> f64 {
        (self.total_asset_value() - self.total_exemptions() - self.administrative_expenses).max(0.0)
    }
}

/// Asset in bankruptcy estate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    /// Description of asset
    pub description: String,

    /// Asset type
    pub asset_type: AssetType,

    /// Current market value
    pub current_value: f64,

    /// Amount of liens/encumbrances
    pub secured_claims: f64,

    /// Equity (value minus secured claims)
    pub equity: f64,

    /// Whether asset is exempt
    pub is_exempt: bool,

    /// Exemption claimed (if any)
    pub exemption_claimed: Option<ExemptionType>,
}

/// Type of asset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetType {
    /// Real property (real estate)
    RealProperty { address: String },

    /// Vehicle
    Vehicle { make_model: String, year: u32 },

    /// Bank account
    BankAccount { institution: String },

    /// Investment account
    InvestmentAccount { account_type: String },

    /// Retirement account (401k, IRA, etc.)
    RetirementAccount { account_type: String },

    /// Personal property
    PersonalProperty { description: String },

    /// Business interest
    BusinessInterest { business_name: String },

    /// Intellectual property
    IntellectualProperty { ip_type: String },

    /// Other asset
    Other { description: String },
}

/// Exemption claimed under Section 522
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exemption {
    /// Asset to which exemption applies
    pub asset_description: String,

    /// Exemption type
    pub exemption_type: ExemptionType,

    /// Amount of exemption
    pub exemption_amount: f64,

    /// Legal basis for exemption
    pub legal_basis: String,

    /// Whether exemption was objected to
    pub objection_filed: bool,

    /// Outcome of objection (if any)
    pub objection_outcome: Option<ObjectionOutcome>,
}

/// Type of bankruptcy exemption
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExemptionType {
    /// Federal exemptions (11 USC § 522(d))
    Federal { subsection: String },

    /// State exemptions (opt-out states)
    State { state: String, statute: String },

    /// Homestead exemption
    Homestead { state: String },

    /// Motor vehicle exemption
    MotorVehicle,

    /// Wildcard exemption
    Wildcard,

    /// Retirement account exemption (ERISA, IRA)
    RetirementAccount,

    /// Tools of trade exemption
    ToolsOfTrade,

    /// Personal property exemption
    PersonalProperty,

    /// Other exemption
    Other { description: String },
}

/// Outcome of exemption objection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectionOutcome {
    /// Exemption allowed (sustained)
    Allowed,

    /// Exemption disallowed (overruled)
    Disallowed,

    /// Partially allowed
    PartiallyAllowed,
}

/// Creditor in bankruptcy case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creditor {
    /// Creditor name
    pub name: String,

    /// Claim type
    pub claim_type: ClaimType,

    /// Scheduled claim amount
    pub claim_amount: f64,

    /// Proof of claim filed
    pub proof_of_claim_filed: bool,

    /// Allowed claim amount (if claim allowed)
    pub allowed_claim_amount: Option<f64>,

    /// Priority level (for priority claims)
    pub priority: Option<ClaimPriority>,

    /// Security interest (if secured claim)
    pub security_interest: Option<SecurityInterest>,

    /// Creditor's address
    pub address: String,

    /// Whether creditor objected to discharge
    pub objected_to_discharge: bool,
}

impl Creditor {
    /// Check if claim is secured
    pub fn is_secured(&self) -> bool {
        matches!(self.claim_type, ClaimType::Secured { .. })
    }

    /// Check if claim is priority
    pub fn is_priority(&self) -> bool {
        matches!(self.claim_type, ClaimType::UnsecuredPriority { .. })
    }

    /// Get recovery percentage estimate
    pub fn estimated_recovery_percentage(&self) -> f64 {
        match self.claim_type {
            ClaimType::Secured { .. } => 100.0, // Secured creditors typically recover in full (up to collateral value)
            ClaimType::UnsecuredPriority { .. } => 80.0, // Priority claims paid before general unsecured
            ClaimType::UnsecuredNonPriority => 10.0, // General unsecured creditors often recover little
            ClaimType::SubordinatedDebt => 0.0,      // Subordinated debt rarely recovers anything
        }
    }
}

/// Type of creditor claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClaimType {
    /// Secured claim (backed by collateral)
    Secured { collateral_description: String },

    /// Unsecured priority claim (11 USC § 507)
    UnsecuredPriority { priority_category: ClaimPriority },

    /// General unsecured non-priority claim
    UnsecuredNonPriority,

    /// Subordinated debt
    SubordinatedDebt,
}

/// Priority of claims under Section 507
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimPriority {
    /// Domestic support obligations (child support, alimony) - First priority
    DomesticSupportObligation,

    /// Administrative expenses (trustee fees, attorney fees) - Second priority
    AdministrativeExpense,

    /// Gap creditors (involuntary case) - Third priority
    GapCreditor,

    /// Wages, salaries, commissions (up to $13,650 per employee) - Fourth priority
    WagesAndSalaries,

    /// Employee benefit plans (up to $13,650 per employee) - Fifth priority
    EmployeeBenefitPlan,

    /// Farmers and fishermen (up to $6,725) - Sixth priority
    FarmerFisherman,

    /// Consumer deposits (up to $3,025) - Seventh priority
    ConsumerDeposit,

    /// Tax claims (federal, state, local) - Eighth priority
    TaxClaim,

    /// Other priority claims
    Other,
}

/// Security interest held by secured creditor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityInterest {
    /// Type of security interest
    pub interest_type: SecurityInterestType,

    /// Collateral description
    pub collateral: String,

    /// Value of collateral
    pub collateral_value: f64,

    /// Amount of secured claim (lesser of debt or collateral value)
    pub secured_amount: f64,

    /// Undersecured portion (if collateral value < debt)
    pub unsecured_deficiency: f64,

    /// Whether lien is perfected
    pub lien_perfected: bool,

    /// Whether lien can be avoided (preferential transfer, fraudulent transfer)
    pub lien_avoidable: bool,
}

/// Type of security interest
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityInterestType {
    /// Mortgage on real property
    Mortgage,

    /// Deed of trust
    DeedOfTrust,

    /// UCC Article 9 security interest (personal property)
    UccSecurityInterest,

    /// Vehicle lien
    VehicleLien,

    /// Judgment lien
    JudgmentLien,

    /// Tax lien
    TaxLien,

    /// Mechanic's lien
    MechanicsLien,

    /// Other lien type
    Other,
}

/// Automatic stay under Section 362
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomaticStay {
    /// Whether stay is in effect
    pub in_effect: bool,

    /// Effective date (petition date)
    pub effective_date: NaiveDate,

    /// Termination date (if lifted)
    pub termination_date: Option<NaiveDate>,

    /// Relief from stay granted
    pub relief_motions: Vec<ReliefFromStay>,

    /// Exceptions to stay
    pub exceptions: Vec<StayException>,
}

/// Motion for relief from automatic stay
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReliefFromStay {
    /// Moving party (creditor seeking relief)
    pub moving_party: String,

    /// Property subject to motion
    pub property: String,

    /// Grounds for relief (cause, lack of adequate protection, no equity)
    pub grounds: Vec<String>,

    /// Filing date
    pub filing_date: NaiveDate,

    /// Outcome
    pub outcome: Option<ReliefOutcome>,
}

/// Outcome of relief from stay motion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReliefOutcome {
    /// Relief granted
    Granted,

    /// Relief denied
    Denied,

    /// Relief granted with conditions (adequate protection)
    GrantedWithConditions,
}

/// Exception to automatic stay (Section 362(b))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StayException {
    /// Criminal proceedings
    CriminalProceedings,

    /// Domestic support obligations
    DomesticSupportObligation,

    /// Tax audits and assessments
    TaxAudit,

    /// Setoff of mutual debts
    Setoff,

    /// Eviction for endangering property or illegal drug use
    EvictionForCause,

    /// Other statutory exception
    Other { description: String },
}

/// Discharge under Section 727 (Chapter 7) or 1328 (Chapter 13)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discharge {
    /// Discharge date
    pub discharge_date: NaiveDate,

    /// Chapter under which discharge granted
    pub chapter: BankruptcyChapter,

    /// Whether discharge was contested
    pub contested: bool,

    /// Objections to discharge filed
    pub objections: Vec<DischargeObjection>,

    /// Nondischargeable debts
    pub nondischargeable_debts: Vec<NondischargeableDebt>,

    /// Discharge revoked (rare)
    pub revoked: bool,
}

/// Objection to discharge
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DischargeObjection {
    /// Objecting party
    pub objecting_party: String,

    /// Grounds for objection (11 USC § 727(a))
    pub grounds: DischargeObjectionGrounds,

    /// Filing date
    pub filing_date: NaiveDate,

    /// Outcome
    pub outcome: Option<DischargeObjectionOutcome>,
}

/// Grounds for objecting to discharge (Section 727(a))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DischargeObjectionGrounds {
    /// Debtor not an individual (Section 727(a)(1))
    NotIndividual,

    /// Transfer/concealment of property with intent to defraud (Section 727(a)(2))
    FraudulentTransfer { details: String },

    /// Failure to keep records (Section 727(a)(3))
    FailureToKeepRecords,

    /// False oath or account (Section 727(a)(4))
    FalseOath { details: String },

    /// Failure to explain loss of assets (Section 727(a)(5))
    FailureToExplainLoss,

    /// Refusal to testify (Section 727(a)(6))
    RefusalToTestify,

    /// Prior discharge within time period (Section 727(a)(8))
    PriorDischarge { prior_date: NaiveDate },

    /// Other grounds
    Other { description: String },
}

/// Outcome of discharge objection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DischargeObjectionOutcome {
    /// Objection sustained - discharge denied
    DischargeDenied,

    /// Objection overruled - discharge granted
    DischargeGranted,

    /// Objection withdrawn
    Withdrawn,
}

/// Nondischargeable debt under Section 523
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NondischargeableDebt {
    /// Creditor name
    pub creditor: String,

    /// Debt amount (in cents to avoid f64)
    pub amount_cents: i64,

    /// Type of nondischargeable debt
    pub debt_type: NondischargeableDebtType,

    /// Whether dischargeability was contested
    pub contested: bool,
}

/// Type of nondischargeable debt (Section 523)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NondischargeableDebtType {
    /// Priority tax debts (Section 523(a)(1))
    TaxDebt,

    /// Debts obtained by fraud (Section 523(a)(2))
    FraudulentDebt { details: String },

    /// Debts not listed in bankruptcy (Section 523(a)(3))
    UnscheduledDebt,

    /// Embezzlement, larceny, breach of fiduciary duty (Section 523(a)(4))
    EmbezzlementOrFraud,

    /// Domestic support obligations (Section 523(a)(5))
    DomesticSupportObligation,

    /// Willful and malicious injury (Section 523(a)(6))
    WillfulInjury,

    /// Fines and penalties owed to government (Section 523(a)(7))
    FinesAndPenalties,

    /// Student loans (Section 523(a)(8)) - unless undue hardship
    StudentLoan { undue_hardship_shown: bool },

    /// Death or injury caused by DUI (Section 523(a)(9))
    DuiInjury,

    /// Debts from prior bankruptcy where discharge denied (Section 523(a)(10))
    PriorBankruptcy,

    /// Other nondischargeable debt
    Other { description: String },
}

/// Chapter conversion (e.g., Chapter 13 to Chapter 7)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterConversion {
    /// Original chapter
    pub from_chapter: BankruptcyChapter,

    /// New chapter
    pub to_chapter: BankruptcyChapter,

    /// Conversion date
    pub conversion_date: NaiveDate,

    /// Reason for conversion
    pub reason: String,

    /// Whether conversion was voluntary or involuntary
    pub voluntary: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debtor_disposable_income() {
        let debtor = Debtor {
            name: "John Doe".to_string(),
            debtor_type: DebtorType::Individual,
            tax_id_last_four: "1234".to_string(),
            address: "123 Main St".to_string(),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1980, 1, 1).expect("valid date")),
            employment_status: Some(EmploymentStatus {
                employed: true,
                employer: Some("ACME Corp".to_string()),
                occupation: Some("Engineer".to_string()),
                employment_start_date: Some(
                    NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
                ),
            }),
            monthly_income: 5000.0,
            monthly_expenses: 3500.0,
            total_assets: 50000.0,
            total_liabilities: 100000.0,
            prior_bankruptcies: vec![],
            is_joint_filing: false,
        };

        assert_eq!(debtor.disposable_income(), 1500.0);
    }

    #[test]
    fn test_bankruptcy_case_total_debt() {
        let case = BankruptcyCase {
            case_number: "24-12345".to_string(),
            chapter: BankruptcyChapter::Chapter7,
            debtor: Debtor {
                name: "Jane Smith".to_string(),
                debtor_type: DebtorType::Individual,
                tax_id_last_four: "5678".to_string(),
                address: "456 Oak Ave".to_string(),
                date_of_birth: Some(NaiveDate::from_ymd_opt(1975, 5, 15).expect("valid date")),
                employment_status: None,
                monthly_income: 3000.0,
                monthly_expenses: 2500.0,
                total_assets: 20000.0,
                total_liabilities: 80000.0,
                prior_bankruptcies: vec![],
                is_joint_filing: false,
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            status: CaseStatus::Filed,
            court_district: "N.D. California".to_string(),
            judge: None,
            trustee: None,
            automatic_stay_active: true,
            estate: None,
            creditors: vec![
                Creditor {
                    name: "Bank of America".to_string(),
                    claim_type: ClaimType::Secured {
                        collateral_description: "2020 Honda Civic".to_string(),
                    },
                    claim_amount: 15000.0,
                    proof_of_claim_filed: true,
                    allowed_claim_amount: Some(15000.0),
                    priority: None,
                    security_interest: None,
                    address: "P.O. Box 123".to_string(),
                    objected_to_discharge: false,
                },
                Creditor {
                    name: "Credit Card Co".to_string(),
                    claim_type: ClaimType::UnsecuredNonPriority,
                    claim_amount: 25000.0,
                    proof_of_claim_filed: true,
                    allowed_claim_amount: Some(25000.0),
                    priority: None,
                    security_interest: None,
                    address: "P.O. Box 456".to_string(),
                    objected_to_discharge: false,
                },
            ],
            discharge: None,
            conversion: None,
        };

        assert_eq!(case.total_debt(), 40000.0);
        assert_eq!(case.secured_debt(), 15000.0);
        assert_eq!(case.unsecured_debt(), 25000.0);
    }

    #[test]
    fn test_chapter_allows_individuals() {
        assert!(BankruptcyChapter::Chapter7.allows_individuals());
        assert!(BankruptcyChapter::Chapter13.allows_individuals());
        assert!(!BankruptcyChapter::Chapter9.allows_individuals());
    }

    #[test]
    fn test_creditor_priority() {
        let priority_creditor = Creditor {
            name: "IRS".to_string(),
            claim_type: ClaimType::UnsecuredPriority {
                priority_category: ClaimPriority::TaxClaim,
            },
            claim_amount: 10000.0,
            proof_of_claim_filed: true,
            allowed_claim_amount: Some(10000.0),
            priority: Some(ClaimPriority::TaxClaim),
            security_interest: None,
            address: "Washington, DC".to_string(),
            objected_to_discharge: false,
        };

        assert!(priority_creditor.is_priority());
        assert!(!priority_creditor.is_secured());
    }
}
