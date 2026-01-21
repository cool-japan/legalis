//! Companies Act 2013 Types
//!
//! Types for Indian corporate law under the Companies Act, 2013

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Company type under Companies Act 2013
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Private limited company (Section 2(68))
    PrivateLimited,
    /// Public limited company (Section 2(71))
    PublicLimited,
    /// One Person Company (Section 2(62))
    OnePerson,
    /// Section 8 Company (non-profit)
    Section8,
    /// Producer Company (Part IXA)
    Producer,
    /// Nidhi Company (Section 406)
    Nidhi,
    /// Government Company (Section 2(45))
    Government,
    /// Holding Company (Section 2(46))
    Holding,
    /// Subsidiary Company (Section 2(87))
    Subsidiary,
    /// Associate Company (Section 2(6))
    Associate,
    /// Small Company (Section 2(85))
    Small,
    /// Listed Company
    Listed,
}

impl CompanyType {
    /// Get the section number in Companies Act 2013
    pub fn section(&self) -> &'static str {
        match self {
            Self::PrivateLimited => "Section 2(68)",
            Self::PublicLimited => "Section 2(71)",
            Self::OnePerson => "Section 2(62)",
            Self::Section8 => "Section 8",
            Self::Producer => "Part IXA",
            Self::Nidhi => "Section 406",
            Self::Government => "Section 2(45)",
            Self::Holding => "Section 2(46)",
            Self::Subsidiary => "Section 2(87)",
            Self::Associate => "Section 2(6)",
            Self::Small => "Section 2(85)",
            Self::Listed => "Listing Regulations",
        }
    }

    /// Get minimum number of directors required
    pub fn min_directors(&self) -> u32 {
        match self {
            Self::PrivateLimited | Self::OnePerson | Self::Small => 2,
            Self::PublicLimited | Self::Listed => 3,
            Self::Section8 => 2,
            Self::Producer => 5,
            Self::Nidhi => 3,
            Self::Government | Self::Holding | Self::Subsidiary | Self::Associate => 3,
        }
    }

    /// Get minimum paid-up capital requirement (if any)
    pub fn min_paid_up_capital(&self) -> Option<u64> {
        match self {
            Self::OnePerson => Some(100_000), // Rs. 1 lakh (abolished in 2015, kept for reference)
            Self::Nidhi => Some(500_000),     // Rs. 5 lakhs
            _ => None,                        // No minimum capital requirement
        }
    }
}

/// Company status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyStatus {
    /// Active company
    Active,
    /// Dormant company (Section 455)
    Dormant,
    /// Under liquidation
    UnderLiquidation,
    /// Under insolvency resolution (IBC)
    UnderInsolvency,
    /// Struck off
    StruckOff,
    /// Dissolved
    Dissolved,
    /// Converted
    Converted,
    /// Amalgamated
    Amalgamated,
}

/// Director Identification Number (DIN) status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DinStatus {
    /// Approved and active
    Approved,
    /// Pending approval
    Pending,
    /// Deactivated (non-filing)
    Deactivated,
    /// Surrendered
    Surrendered,
    /// Cancelled
    Cancelled,
}

/// Director category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectorCategory {
    /// Executive director
    Executive,
    /// Non-executive director
    NonExecutive,
    /// Independent director (Section 149(6))
    Independent,
    /// Managing Director (Section 2(54))
    Managing,
    /// Whole-time Director (Section 2(94))
    WholeTime,
    /// Nominee Director
    Nominee,
    /// Alternate Director (Section 161(2))
    Alternate,
    /// Additional Director (Section 161(1))
    Additional,
    /// Small Shareholder Director
    SmallShareholderDirector,
    /// Woman Director (Section 149(1))
    Woman,
}

impl DirectorCategory {
    /// Whether this director counts towards independent director requirements
    pub fn is_independent(&self) -> bool {
        matches!(self, Self::Independent)
    }

    /// Whether this director is a whole-time key managerial personnel
    pub fn is_whole_time(&self) -> bool {
        matches!(self, Self::Managing | Self::WholeTime | Self::Executive)
    }
}

/// Director information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Director {
    /// Director Identification Number
    pub din: String,
    /// Name
    pub name: String,
    /// Category
    pub category: DirectorCategory,
    /// Date of appointment
    pub appointment_date: NaiveDate,
    /// Term end date (for independent directors)
    pub term_end: Option<NaiveDate>,
    /// Whether the director is resident in India
    pub resident_in_india: bool,
    /// Whether disqualified under Section 164
    pub disqualified: bool,
    /// DIN status
    pub din_status: DinStatus,
    /// Number of other directorships
    pub other_directorships: u32,
}

impl Director {
    /// Check if director is eligible under Section 164
    pub fn is_eligible(&self) -> bool {
        !self.disqualified && matches!(self.din_status, DinStatus::Approved)
    }

    /// Check if within directorship limit (Section 165)
    pub fn within_directorship_limit(&self, is_public_company: bool) -> bool {
        if is_public_company {
            self.other_directorships < 10 // Max 10 public companies
        } else {
            self.other_directorships < 20 // Max 20 companies total
        }
    }

    /// Calculate tenure in years
    pub fn tenure_years(&self, current_date: NaiveDate) -> f64 {
        let days = (current_date - self.appointment_date).num_days();
        days as f64 / 365.25
    }
}

/// Key Managerial Personnel (KMP) type (Section 2(51))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KmpType {
    /// Managing Director
    ManagingDirector,
    /// Chief Executive Officer
    Ceo,
    /// Manager
    Manager,
    /// Company Secretary
    CompanySecretary,
    /// Chief Financial Officer
    Cfo,
    /// Whole-time Director
    WholeTimeDirector,
}

impl KmpType {
    /// Get the section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::ManagingDirector => "Section 2(54)",
            Self::Ceo => "Section 2(18)",
            Self::Manager => "Section 2(53)",
            Self::CompanySecretary => "Section 2(24)",
            Self::Cfo => "Section 2(19)",
            Self::WholeTimeDirector => "Section 2(94)",
        }
    }
}

/// Key Managerial Personnel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kmp {
    /// Name
    pub name: String,
    /// Type
    pub kmp_type: KmpType,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// PAN (Permanent Account Number)
    pub pan: Option<String>,
    /// Membership number (for CS)
    pub membership_number: Option<String>,
}

/// Share class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShareClass {
    /// Equity shares (Section 43(a))
    Equity,
    /// Preference shares (Section 43(b))
    Preference,
    /// Sweat equity shares (Section 54)
    SweatEquity,
    /// Employees stock options
    Esop,
}

/// Share capital type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareCapitalType {
    /// Authorized capital
    Authorized,
    /// Issued capital
    Issued,
    /// Subscribed capital
    Subscribed,
    /// Paid-up capital
    PaidUp,
}

/// Shareholder category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShareholderCategory {
    /// Promoter
    Promoter,
    /// Promoter group
    PromoterGroup,
    /// Public
    Public,
    /// Foreign Institutional Investor
    Fii,
    /// Domestic Institutional Investor
    Dii,
    /// Mutual Fund
    MutualFund,
    /// Employee
    Employee,
    /// Director
    Director,
    /// Government
    Government,
}

/// Shareholder information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shareholder {
    /// Name/Entity name
    pub name: String,
    /// Category
    pub category: ShareholderCategory,
    /// Number of shares held
    pub shares_held: u64,
    /// Percentage holding
    pub percentage: f64,
    /// PAN
    pub pan: Option<String>,
    /// Beneficial owner name (if different)
    pub beneficial_owner: Option<String>,
}

impl Shareholder {
    /// Check if significant shareholder (>= 10%)
    pub fn is_significant(&self) -> bool {
        self.percentage >= 10.0
    }

    /// Check if majority shareholder (> 50%)
    pub fn is_majority(&self) -> bool {
        self.percentage > 50.0
    }
}

/// Board committee type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommitteeType {
    /// Audit Committee (Section 177)
    Audit,
    /// Nomination and Remuneration Committee (Section 178)
    NominationRemuneration,
    /// Stakeholders Relationship Committee (Section 178)
    StakeholdersRelationship,
    /// Corporate Social Responsibility Committee (Section 135)
    Csr,
    /// Risk Management Committee (LODR)
    RiskManagement,
}

impl CommitteeType {
    /// Get the section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::Audit => "Section 177",
            Self::NominationRemuneration => "Section 178",
            Self::StakeholdersRelationship => "Section 178",
            Self::Csr => "Section 135",
            Self::RiskManagement => "LODR Regulation 21",
        }
    }

    /// Get minimum members required
    pub fn min_members(&self) -> u32 {
        match self {
            Self::Audit | Self::NominationRemuneration => 3,
            Self::StakeholdersRelationship | Self::Csr | Self::RiskManagement => 3,
        }
    }

    /// Get minimum independent directors required
    pub fn min_independent_directors(&self) -> u32 {
        match self {
            Self::Audit => 2, // Majority should be independent
            Self::NominationRemuneration => 2,
            Self::StakeholdersRelationship => 0,
            Self::Csr => 0,
            Self::RiskManagement => 0,
        }
    }
}

/// Board committee
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Committee {
    /// Committee type
    pub committee_type: CommitteeType,
    /// Chairperson DIN
    pub chairperson_din: String,
    /// Member DIns
    pub member_dins: Vec<String>,
    /// Formation date
    pub formation_date: NaiveDate,
}

impl Committee {
    /// Check if committee composition is valid
    pub fn is_valid_composition(&self, directors: &[Director]) -> bool {
        let member_count = self.member_dins.len() as u32;
        if member_count < self.committee_type.min_members() {
            return false;
        }

        let independent_count = self
            .member_dins
            .iter()
            .filter(|din| {
                directors
                    .iter()
                    .any(|d| &d.din == *din && d.category.is_independent())
            })
            .count() as u32;

        independent_count >= self.committee_type.min_independent_directors()
    }
}

/// Resolution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution (Section 114)
    Ordinary,
    /// Special resolution (Section 114(2))
    Special,
    /// Board resolution
    Board,
    /// Circular resolution
    Circular,
}

impl ResolutionType {
    /// Get required majority percentage
    pub fn required_majority(&self) -> f64 {
        match self {
            Self::Ordinary | Self::Board | Self::Circular => 50.0,
            Self::Special => 75.0,
        }
    }

    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::Ordinary => "Section 114(1)",
            Self::Special => "Section 114(2)",
            Self::Board => "Section 179",
            Self::Circular => "Section 175",
        }
    }
}

/// Matters requiring special resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialResolutionMatter {
    /// Alteration of articles (Section 14)
    AlterationOfArticles,
    /// Change of name (Section 13)
    ChangeName,
    /// Change of registered office between states (Section 13)
    ChangeRegisteredOfficeOutState,
    /// Variation of shareholder rights (Section 48)
    VariationOfRights,
    /// Issue of sweat equity shares (Section 54)
    IssueSweatEquity,
    /// Buy-back of shares (Section 68)
    BuybackShares,
    /// Related party transactions (Section 188)
    RelatedPartyTransaction,
    /// Removal of auditor before term (Section 140)
    RemovalOfAuditor,
    /// Reduction of share capital (Section 66)
    ReductionOfCapital,
    /// Conversion of company type (Section 18)
    ConversionOfCompany,
    /// Winding up (Section 271)
    VoluntaryWindingUp,
    /// Approval of scheme of arrangement (Section 230)
    SchemeOfArrangement,
}

impl SpecialResolutionMatter {
    /// Get section reference
    pub fn section(&self) -> u32 {
        match self {
            Self::AlterationOfArticles => 14,
            Self::ChangeName | Self::ChangeRegisteredOfficeOutState => 13,
            Self::VariationOfRights => 48,
            Self::IssueSweatEquity => 54,
            Self::BuybackShares => 68,
            Self::RelatedPartyTransaction => 188,
            Self::RemovalOfAuditor => 140,
            Self::ReductionOfCapital => 66,
            Self::ConversionOfCompany => 18,
            Self::VoluntaryWindingUp => 271,
            Self::SchemeOfArrangement => 230,
        }
    }
}

/// Annual filing type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnnualFilingType {
    /// Annual Return (MGT-7/MGT-7A)
    AnnualReturn,
    /// Financial Statements (AOC-4)
    FinancialStatements,
    /// Director KYC (DIR-3 KYC)
    DirectorKyc,
    /// Auditor Appointment (ADT-1)
    AuditorAppointment,
    /// Charge Registration (CHG-1)
    ChargeRegistration,
    /// Event-based forms (various)
    EventBased,
}

impl AnnualFilingType {
    /// Get form number
    pub fn form_number(&self) -> &'static str {
        match self {
            Self::AnnualReturn => "MGT-7/MGT-7A",
            Self::FinancialStatements => "AOC-4/AOC-4 CFS",
            Self::DirectorKyc => "DIR-3 KYC",
            Self::AuditorAppointment => "ADT-1",
            Self::ChargeRegistration => "CHG-1",
            Self::EventBased => "Various",
        }
    }

    /// Get due date from financial year end
    pub fn due_days_from_fy_end(&self) -> Option<u32> {
        match self {
            Self::AnnualReturn => Some(60),        // 60 days from AGM
            Self::FinancialStatements => Some(30), // 30 days from AGM
            Self::DirectorKyc => Some(270),        // September 30 each year
            Self::AuditorAppointment => Some(15),  // 15 days from AGM
            Self::ChargeRegistration => None,      // 30 days from charge creation
            Self::EventBased => None,
        }
    }
}

/// Corporate Social Responsibility (CSR) activity category (Schedule VII)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CsrCategory {
    /// Eradicating hunger, poverty, malnutrition
    PovertyEradication,
    /// Promoting education
    Education,
    /// Promoting gender equality
    GenderEquality,
    /// Environmental sustainability
    Environment,
    /// Protection of national heritage
    NationalHeritage,
    /// Armed forces welfare
    ArmedForcesWelfare,
    /// Sports training
    Sports,
    /// Rural development
    RuralDevelopment,
    /// Slum area development
    SlumDevelopment,
    /// Disaster relief
    DisasterRelief,
    /// Technology incubators
    TechnologyIncubators,
}

impl CsrCategory {
    /// Get Schedule VII clause
    pub fn schedule_vii_clause(&self) -> &'static str {
        match self {
            Self::PovertyEradication => "(i)",
            Self::Education => "(ii)",
            Self::GenderEquality => "(iii)",
            Self::Environment => "(iv)",
            Self::NationalHeritage => "(v)",
            Self::ArmedForcesWelfare => "(vi)",
            Self::Sports => "(vii)",
            Self::RuralDevelopment => "(viii)",
            Self::SlumDevelopment => "(ix)",
            Self::DisasterRelief => "(xii)",
            Self::TechnologyIncubators => "(ix)",
        }
    }
}

/// CSR obligation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsrObligation {
    /// Financial year
    pub financial_year: String,
    /// Average net profit of last 3 years
    pub average_net_profit: i64,
    /// 2% CSR obligation amount
    pub obligation_amount: i64,
    /// Amount spent
    pub amount_spent: i64,
    /// Unspent amount
    pub unspent_amount: i64,
    /// Activities undertaken
    pub activities: Vec<CsrCategory>,
}

impl CsrObligation {
    /// Calculate CSR obligation (2% of average net profit)
    pub fn calculate_obligation(average_net_profit: i64) -> i64 {
        if average_net_profit > 0 {
            (average_net_profit as f64 * 0.02).round() as i64
        } else {
            0
        }
    }

    /// Check if CSR is mandatory (Section 135 threshold)
    pub fn is_csr_mandatory(net_worth: i64, turnover: i64, net_profit: i64) -> bool {
        net_worth >= 500_000_000 ||      // Rs. 500 crore
        turnover >= 1_000_000_000 ||     // Rs. 1000 crore
        net_profit >= 50_000_000 // Rs. 5 crore
    }
}

/// Related party transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelatedPartyTransactionType {
    /// Sale/purchase of goods
    SalePurchaseGoods,
    /// Sale/purchase of assets
    SalePurchaseAssets,
    /// Leasing of property
    LeasingProperty,
    /// Availing/rendering of services
    Services,
    /// Appointment to office/place of profit
    Appointment,
    /// Underwriting subscription of securities
    Underwriting,
}

/// Related party (Section 2(76))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelatedPartyType {
    /// Director or KMP
    DirectorKmp,
    /// Relative of director or KMP
    RelativeOfDirectorKmp,
    /// Firm in which director/KMP is partner
    PartnerFirm,
    /// Private company in which director is member
    PrivateCompanyMember,
    /// Public company in which director is director and holds >2%
    PublicCompanyDirector,
    /// Body corporate whose Board/MD/Manager accustomed to act on director's advice
    BodyCorporateUnderInfluence,
    /// Holding, subsidiary, associate company
    GroupCompany,
}

/// Company registration details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// Corporate Identity Number (CIN)
    pub cin: String,
    /// Company name
    pub name: String,
    /// Company type
    pub company_type: CompanyType,
    /// Date of incorporation
    pub incorporation_date: NaiveDate,
    /// Registered office address
    pub registered_office: String,
    /// State of registration
    pub state: String,
    /// Registrar of Companies (ROC)
    pub roc: String,
    /// Company status
    pub status: CompanyStatus,
    /// Authorized capital
    pub authorized_capital: i64,
    /// Paid-up capital
    pub paid_up_capital: i64,
    /// Financial year end month
    pub fy_end_month: u32,
    /// Listed status
    pub is_listed: bool,
    /// Directors
    pub directors: Vec<Director>,
    /// Key Managerial Personnel
    pub kmps: Vec<Kmp>,
    /// Shareholders
    pub shareholders: Vec<Shareholder>,
    /// Committees
    pub committees: Vec<Committee>,
}

impl Company {
    /// Check if company qualifies as small company (Section 2(85))
    pub fn is_small_company(&self) -> bool {
        !self.is_listed
            && matches!(self.company_type, CompanyType::PrivateLimited)
            && self.paid_up_capital <= 40_000_000 // Rs. 4 crore (amended limit)
    }

    /// Check if CSR is applicable
    pub fn is_csr_applicable(&self, net_worth: i64, turnover: i64, net_profit: i64) -> bool {
        CsrObligation::is_csr_mandatory(net_worth, turnover, net_profit)
    }

    /// Get required number of independent directors
    pub fn required_independent_directors(&self) -> u32 {
        if self.is_listed {
            // Listed companies: 1/3 of total board strength
            let board_size = self.directors.len() as u32;
            (board_size as f64 / 3.0).ceil() as u32
        } else if matches!(self.company_type, CompanyType::PublicLimited) {
            2 // Public unlisted: at least 2
        } else {
            0
        }
    }

    /// Count current independent directors
    pub fn independent_director_count(&self) -> u32 {
        self.directors
            .iter()
            .filter(|d| d.category.is_independent() && d.is_eligible())
            .count() as u32
    }

    /// Check if woman director is required
    pub fn requires_woman_director(&self) -> bool {
        self.is_listed
            || (matches!(self.company_type, CompanyType::PublicLimited)
                && self.paid_up_capital >= 1_000_000_000) // Rs. 100 crore
    }

    /// Check if company has woman director
    pub fn has_woman_director(&self) -> bool {
        self.directors
            .iter()
            .any(|d| matches!(d.category, DirectorCategory::Woman) && d.is_eligible())
    }

    /// Check if resident director requirement is met
    pub fn has_resident_director(&self) -> bool {
        self.directors
            .iter()
            .any(|d| d.resident_in_india && d.is_eligible())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type_sections() {
        assert_eq!(CompanyType::PrivateLimited.section(), "Section 2(68)");
        assert_eq!(CompanyType::PublicLimited.section(), "Section 2(71)");
        assert_eq!(CompanyType::OnePerson.section(), "Section 2(62)");
    }

    #[test]
    fn test_company_type_min_directors() {
        assert_eq!(CompanyType::PrivateLimited.min_directors(), 2);
        assert_eq!(CompanyType::PublicLimited.min_directors(), 3);
        assert_eq!(CompanyType::OnePerson.min_directors(), 2);
    }

    #[test]
    fn test_director_eligibility() {
        let director = Director {
            din: "12345678".to_string(),
            name: "Test Director".to_string(),
            category: DirectorCategory::Independent,
            appointment_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            term_end: None,
            resident_in_india: true,
            disqualified: false,
            din_status: DinStatus::Approved,
            other_directorships: 5,
        };
        assert!(director.is_eligible());
        assert!(director.within_directorship_limit(true));
    }

    #[test]
    fn test_resolution_majority() {
        assert_eq!(ResolutionType::Ordinary.required_majority(), 50.0);
        assert_eq!(ResolutionType::Special.required_majority(), 75.0);
    }

    #[test]
    fn test_csr_obligation() {
        let obligation = CsrObligation::calculate_obligation(100_000_000);
        assert_eq!(obligation, 2_000_000); // 2% of 10 crore = 20 lakhs
    }

    #[test]
    fn test_csr_mandatory_threshold() {
        // Net profit threshold
        assert!(CsrObligation::is_csr_mandatory(0, 0, 50_000_000));
        assert!(!CsrObligation::is_csr_mandatory(0, 0, 40_000_000));

        // Turnover threshold
        assert!(CsrObligation::is_csr_mandatory(0, 1_000_000_000, 0));

        // Net worth threshold
        assert!(CsrObligation::is_csr_mandatory(500_000_000, 0, 0));
    }

    #[test]
    fn test_shareholder_significance() {
        let shareholder = Shareholder {
            name: "Test Promoter".to_string(),
            category: ShareholderCategory::Promoter,
            shares_held: 1_000_000,
            percentage: 25.0,
            pan: Some("ABCDE1234F".to_string()),
            beneficial_owner: None,
        };
        assert!(shareholder.is_significant());
        assert!(!shareholder.is_majority());
    }

    #[test]
    fn test_committee_types() {
        assert_eq!(CommitteeType::Audit.min_members(), 3);
        assert_eq!(CommitteeType::Audit.min_independent_directors(), 2);
        assert_eq!(CommitteeType::Csr.min_independent_directors(), 0);
    }
}
