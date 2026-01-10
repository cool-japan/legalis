//! Companies Act (Cap. 50) - Type Definitions
//!
//! This module provides type-safe representations of Singapore company structures,
//! directors, shareholders, and share capital as defined in the Companies Act.
//!
//! ## Key Types
//!
//! - [`Company`]: Complete company structure with directors, shareholders, share capital
//! - [`Director`]: Company director with eligibility requirements (s. 145)
//! - [`Shareholder`]: Shareholder with share allocation
//! - [`ShareCapital`]: Company share capital structure (can be no-par value)
//! - [`CompanyType`]: Type of Singapore business entity
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::companies::types::*;
//! use chrono::Utc;
//!
//! // Create a private limited company
//! let company = Company {
//!     uen: "202401234A".to_string(),
//!     name: "Tech Innovations Pte Ltd".to_string(),
//!     company_type: CompanyType::PrivateLimited,
//!     registration_date: Utc::now(),
//!     registered_address: Address::singapore("1 Raffles Place", "048616"),
//!     share_capital: ShareCapital::new(100_000_00), // SGD 100,000
//!     directors: vec![],
//!     shareholders: vec![],
//!     company_secretary: None,
//!     financial_year_end: MonthDay { month: 12, day: 31 },
//! };
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of Singapore business entity
///
/// ## Companies Act (Cap. 50)
///
/// Singapore recognizes several types of business entities under the Companies Act:
///
/// - **Private Limited Company (Pte Ltd)**: Most common, max 50 shareholders, no public offering
/// - **Public Limited Company (Ltd)**: Can have unlimited shareholders, no public offering
/// - **Public Listed Company (PLC)**: Listed on Singapore Exchange (SGX)
/// - **Limited Liability Partnership (LLP)**: Hybrid between company and partnership
/// - **Sole Proprietorship**: Unincorporated, unlimited personal liability
/// - **Foreign Company**: Branch or representative office of foreign entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Private Limited Company (Pte Ltd) - s. 18
    ///
    /// Requirements:
    /// - Max 50 shareholders (s. 18(1))
    /// - Restrictions on share transfer (s. 18(1)(a))
    /// - No public offering of shares/debentures (s. 18(1)(b))
    PrivateLimited,

    /// Public Limited Company (Ltd)
    ///
    /// Requirements:
    /// - Unlimited shareholders allowed
    /// - No public offering (unlisted)
    /// - More stringent reporting requirements
    PublicLimited,

    /// Public Listed Company (PLC)
    ///
    /// Requirements:
    /// - Listed on Singapore Exchange (SGX)
    /// - Subject to SGX listing rules
    /// - Continuous disclosure obligations
    PublicListedCompany,

    /// Limited Liability Partnership (LLP) - Limited Liability Partnerships Act
    ///
    /// Requirements:
    /// - At least 2 partners
    /// - Partners not personally liable for debts
    /// - More flexible than company structure
    LimitedLiabilityPartnership,

    /// Sole Proprietorship
    ///
    /// Requirements:
    /// - Single owner
    /// - Unlimited personal liability
    /// - Simplest business structure
    SoleProprietorship,

    /// Foreign Company (Branch/Representative Office)
    ///
    /// Requirements:
    /// - Registration within 2 months of establishing place of business (s. 368)
    /// - Agent for service of documents in Singapore
    /// - Annual filing requirements
    ForeignCompany,
}

impl CompanyType {
    /// Returns whether this company type requires a resident director (s. 145)
    pub fn requires_resident_director(&self) -> bool {
        matches!(
            self,
            CompanyType::PrivateLimited
                | CompanyType::PublicLimited
                | CompanyType::PublicListedCompany
                | CompanyType::LimitedLiabilityPartnership
        )
    }

    /// Returns whether this company type requires a company secretary (s. 171)
    pub fn requires_company_secretary(&self) -> bool {
        matches!(
            self,
            CompanyType::PrivateLimited
                | CompanyType::PublicLimited
                | CompanyType::PublicListedCompany
        )
    }

    /// Returns maximum number of shareholders (None = unlimited)
    pub fn max_shareholders(&self) -> Option<usize> {
        match self {
            CompanyType::PrivateLimited => Some(50), // s. 18(1)
            _ => None,
        }
    }

    /// Returns legal suffix required in company name
    pub fn legal_suffix(&self) -> &'static str {
        match self {
            CompanyType::PrivateLimited => "Pte Ltd",
            CompanyType::PublicLimited => "Ltd",
            CompanyType::PublicListedCompany => "Ltd",
            CompanyType::LimitedLiabilityPartnership => "LLP",
            CompanyType::SoleProprietorship => "", // No suffix
            CompanyType::ForeignCompany => "",     // Uses foreign name
        }
    }
}

impl std::fmt::Display for CompanyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanyType::PrivateLimited => write!(f, "Private Limited (Pte Ltd)"),
            CompanyType::PublicLimited => write!(f, "Public Limited (Ltd)"),
            CompanyType::PublicListedCompany => write!(f, "Public Listed Company (PLC)"),
            CompanyType::LimitedLiabilityPartnership => {
                write!(f, "Limited Liability Partnership (LLP)")
            }
            CompanyType::SoleProprietorship => write!(f, "Sole Proprietorship"),
            CompanyType::ForeignCompany => write!(f, "Foreign Company"),
        }
    }
}

/// Complete company structure
///
/// Represents a Singapore company registered with ACRA.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// Unique Entity Number (UEN) assigned by ACRA
    ///
    /// Format: 9-10 digits, e.g., "202401234A"
    pub uen: String,

    /// Registered company name (must end with legal suffix)
    ///
    /// Requirements:
    /// - Must be unique (not identical to existing company)
    /// - Must include legal form suffix (Pte Ltd, Ltd, LLP)
    /// - Cannot be offensive, misleading, or suggest government connection
    pub name: String,

    /// Type of business entity
    pub company_type: CompanyType,

    /// Date of incorporation/registration with ACRA
    pub registration_date: DateTime<Utc>,

    /// Registered office address in Singapore
    ///
    /// Requirement (s. 142): Every company must have registered office in Singapore
    pub registered_address: Address,

    /// Share capital structure
    pub share_capital: ShareCapital,

    /// List of directors (minimum 1, at least 1 must be resident - s. 145)
    pub directors: Vec<Director>,

    /// List of shareholders/members
    pub shareholders: Vec<Shareholder>,

    /// Company secretary (mandatory within 6 months of incorporation - s. 171)
    pub company_secretary: Option<CompanySecretary>,

    /// Financial year end (for annual return filing)
    pub financial_year_end: MonthDay,
}

impl Company {
    /// Creates a new company with minimum required fields
    pub fn new(
        uen: impl Into<String>,
        name: impl Into<String>,
        company_type: CompanyType,
        registered_address: Address,
    ) -> Self {
        Self {
            uen: uen.into(),
            name: name.into(),
            company_type,
            registration_date: Utc::now(),
            registered_address,
            share_capital: ShareCapital::default(),
            directors: Vec::new(),
            shareholders: Vec::new(),
            company_secretary: None,
            financial_year_end: MonthDay { month: 12, day: 31 }, // Default Dec 31
        }
    }

    /// Returns whether company has at least one resident director (s. 145 requirement)
    pub fn has_resident_director(&self) -> bool {
        self.directors.iter().any(|d| d.is_resident_director)
    }

    /// Returns whether company has appointed company secretary (s. 171 requirement)
    pub fn has_company_secretary(&self) -> bool {
        self.company_secretary.is_some()
    }

    /// Returns number of shareholders
    pub fn shareholder_count(&self) -> usize {
        self.shareholders.len()
    }

    /// Returns whether company name includes required legal suffix
    pub fn has_valid_legal_suffix(&self) -> bool {
        let suffix = self.company_type.legal_suffix();
        if suffix.is_empty() {
            return true; // No suffix required
        }
        self.name.ends_with(suffix) || self.name.contains(suffix)
    }
}

/// Company director
///
/// ## Section 145: Resident Director Requirement
///
/// Every company must have at least one director who is ordinarily resident in Singapore.
/// "Ordinarily resident" means physically present in Singapore for extended periods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Director {
    /// Full legal name
    pub name: String,

    /// Singapore NRIC/FIN (for residents/PRs)
    pub nric_fin: Option<String>,

    /// Passport number (for foreigners)
    pub passport: Option<String>,

    /// Nationality
    pub nationality: String,

    /// Residential address
    pub residential_address: Address,

    /// Date of appointment as director
    pub appointment_date: DateTime<Utc>,

    /// Whether ordinarily resident in Singapore (s. 145 requirement)
    ///
    /// Definition: Physically present in Singapore or intending to be physically
    /// present in Singapore for extended periods (not just short visits)
    pub is_resident_director: bool,

    /// Director qualifications (if any)
    pub qualifications: Vec<DirectorQualification>,

    /// Disqualification status
    pub disqualification_status: DisqualificationStatus,

    /// Whether director has sole representation authority
    ///
    /// If false, requires joint signature with another director
    pub sole_representation: bool,
}

impl Director {
    /// Creates a new director with minimum required information
    pub fn new(
        name: impl Into<String>,
        nric_or_passport: impl Into<String>,
        is_resident: bool,
    ) -> Self {
        let id = nric_or_passport.into();
        let is_nric = id.starts_with('S')
            || id.starts_with('T')
            || id.starts_with('F')
            || id.starts_with('G');
        let (nric_fin, passport) = if is_nric {
            (Some(id), None)
        } else {
            (None, Some(id))
        };

        let is_singapore = nric_fin.is_some();
        Self {
            name: name.into(),
            nric_fin,
            passport,
            nationality: if is_singapore {
                "Singapore".to_string()
            } else {
                "Foreign".to_string()
            },
            residential_address: Address::default(),
            appointment_date: Utc::now(),
            is_resident_director: is_resident,
            qualifications: Vec::new(),
            disqualification_status: DisqualificationStatus::Eligible,
            sole_representation: true,
        }
    }

    /// Returns whether director is eligible (not disqualified)
    pub fn is_eligible(&self) -> bool {
        matches!(
            self.disqualification_status,
            DisqualificationStatus::Eligible
        )
    }
}

/// Director qualifications
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectorQualification {
    /// Professional qualification (e.g., CPA, lawyer)
    Professional(String),

    /// Industry experience (years)
    IndustryExperience(u32),

    /// Prior director experience at other companies
    PriorDirectorship(u32),

    /// Completed directors training/certification
    DirectorsTraining(String),
}

/// Director disqualification status
///
/// ## Sections 148, 149, 155: Disqualification
///
/// A person is disqualified from being director if:
/// - **s. 148**: Convicted of offense involving fraud/dishonesty (5 years)
/// - **s. 149**: Undischarged bankrupt
/// - **s. 155**: Court order disqualification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisqualificationStatus {
    /// Eligible to be appointed as director
    Eligible,

    /// Disqualified under s. 148 (conviction for fraud/dishonesty)
    ConvictionDisqualification {
        /// Conviction date
        conviction_date: DateTime<Utc>,
        /// Nature of offense
        offense: String,
        /// Disqualification end date (usually 5 years from conviction)
        disqualification_until: DateTime<Utc>,
    },

    /// Disqualified under s. 149 (undischarged bankrupt)
    BankruptcyDisqualification {
        /// Date of bankruptcy order
        bankruptcy_date: DateTime<Utc>,
    },

    /// Disqualified under s. 155 (court order)
    CourtOrderDisqualification {
        /// Date of court order
        order_date: DateTime<Utc>,
        /// Reasons for disqualification
        reason: String,
        /// Disqualification period (if specified)
        disqualification_until: Option<DateTime<Utc>>,
    },
}

/// Company shareholder/member
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shareholder {
    /// Name (individual or corporate)
    pub name: String,

    /// NRIC/FIN/UEN (identification)
    pub identification: String,

    /// Nationality (for individuals) or country of incorporation (for corporates)
    pub nationality_or_jurisdiction: String,

    /// Address
    pub address: Address,

    /// Share allocation
    pub share_allocation: ShareAllocation,

    /// Date became shareholder
    pub acquisition_date: DateTime<Utc>,
}

/// Share allocation for a shareholder
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareAllocation {
    /// Share class (e.g., "Ordinary", "Preference")
    pub share_class: String,

    /// Number of shares held
    pub number_of_shares: u64,

    /// Amount paid per share (in cents)
    pub amount_paid_per_share_cents: u64,

    /// Total amount paid (in cents)
    ///
    /// Should equal: number_of_shares * amount_paid_per_share_cents
    pub total_paid_cents: u64,
}

impl ShareAllocation {
    /// Creates a new share allocation
    pub fn new(share_class: impl Into<String>, shares: u64, price_per_share_cents: u64) -> Self {
        Self {
            share_class: share_class.into(),
            number_of_shares: shares,
            amount_paid_per_share_cents: price_per_share_cents,
            total_paid_cents: shares * price_per_share_cents,
        }
    }

    /// Returns total paid in SGD (from cents)
    pub fn total_paid_sgd(&self) -> f64 {
        self.total_paid_cents as f64 / 100.0
    }

    /// Returns ownership percentage of total shares
    pub fn ownership_percentage(&self, total_shares: u64) -> f64 {
        if total_shares == 0 {
            0.0
        } else {
            (self.number_of_shares as f64 / total_shares as f64) * 100.0
        }
    }
}

/// Company share capital
///
/// Singapore allows companies to have no-par value shares (s. 67A), meaning
/// shares can be issued without a nominal/par value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareCapital {
    /// Authorized capital (in cents), if applicable
    ///
    /// For no-par value companies, this may be None
    pub authorized_capital_cents: Option<u64>,

    /// Total number of issued shares
    pub issued_shares: u64,

    /// Total paid-up capital (in cents)
    ///
    /// This is the actual amount received from shareholders for shares
    pub paid_up_capital_cents: u64,

    /// Share classes (e.g., Ordinary, Preference, etc.)
    pub share_classes: Vec<ShareClass>,

    /// Whether shares have par value (false = no-par value shares)
    pub has_par_value: bool,
}

impl ShareCapital {
    /// Creates new share capital with paid-up amount (in cents)
    pub fn new(paid_up_cents: u64) -> Self {
        Self {
            authorized_capital_cents: Some(paid_up_cents),
            issued_shares: 0,
            paid_up_capital_cents: paid_up_cents,
            share_classes: Vec::new(),
            has_par_value: false, // Default to no-par value (modern approach)
        }
    }

    /// Creates no-par value share capital
    pub fn no_par_value(paid_up_cents: u64, issued_shares: u64) -> Self {
        Self {
            authorized_capital_cents: None,
            issued_shares,
            paid_up_capital_cents: paid_up_cents,
            share_classes: Vec::new(),
            has_par_value: false,
        }
    }

    /// Returns paid-up capital in SGD
    pub fn paid_up_sgd(&self) -> f64 {
        self.paid_up_capital_cents as f64 / 100.0
    }

    /// Returns authorized capital in SGD (if applicable)
    pub fn authorized_sgd(&self) -> Option<f64> {
        self.authorized_capital_cents
            .map(|cents| cents as f64 / 100.0)
    }

    /// Adds a share class
    pub fn add_share_class(&mut self, share_class: ShareClass) {
        self.share_classes.push(share_class);
    }
}

impl Default for ShareCapital {
    fn default() -> Self {
        Self::new(10_000) // Default SGD 100 paid-up capital
    }
}

/// Share class (e.g., Ordinary, Preference)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareClass {
    /// Class name (e.g., "Ordinary", "Preference A")
    pub class_name: String,

    /// Number of shares in this class
    pub shares: u64,

    /// Par value per share (in cents), if applicable
    ///
    /// None for no-par value shares
    pub par_value_cents: Option<u64>,

    /// Whether shares have voting rights
    pub voting_rights: bool,

    /// Dividend rights
    pub dividend_rights: DividendRights,

    /// Whether shares are redeemable
    pub redeemable: bool,
}

impl ShareClass {
    /// Creates an ordinary share class (voting, no preference)
    pub fn ordinary(shares: u64, par_value_cents: Option<u64>) -> Self {
        Self {
            class_name: "Ordinary".to_string(),
            shares,
            par_value_cents,
            voting_rights: true,
            dividend_rights: DividendRights::NonPreference,
            redeemable: false,
        }
    }

    /// Creates a preference share class (typically non-voting, dividend priority)
    pub fn preference(shares: u64, par_value_cents: Option<u64>, dividend_rate_bps: u32) -> Self {
        Self {
            class_name: "Preference".to_string(),
            shares,
            par_value_cents,
            voting_rights: false,
            dividend_rights: DividendRights::Preference {
                rate_bps: dividend_rate_bps,
            },
            redeemable: false,
        }
    }
}

/// Dividend rights for share class
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DividendRights {
    /// No preference (ordinary shares)
    NonPreference,

    /// Preference dividend at fixed rate (percentage stored as integer basis points: 500 = 5.00%)
    Preference {
        /// Annual dividend rate in basis points (e.g., 500 for 5%)
        rate_bps: u32,
    },

    /// Cumulative preference (unpaid dividends accumulate)
    CumulativePreference {
        /// Annual dividend rate in basis points
        rate_bps: u32,
    },

    /// Participating preference (shares in excess profits)
    ParticipatingPreference {
        /// Preference dividend rate in basis points
        preference_rate_bps: u32,
        /// Additional participation rate in basis points
        participation_rate_bps: u32,
    },
}

/// Company secretary
///
/// ## Section 171: Company Secretary Requirement
///
/// Every company must appoint a company secretary within 6 months of incorporation.
/// The secretary must be ordinarily resident in Singapore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanySecretary {
    /// Full name
    pub name: String,

    /// NRIC/FIN
    pub nric_fin: String,

    /// Date of appointment
    pub appointment_date: DateTime<Utc>,

    /// Whether qualified secretary (e.g., ACCA, CPA, lawyer)
    pub qualified: bool,

    /// Contact information
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
}

impl CompanySecretary {
    /// Creates a new company secretary
    pub fn new(name: impl Into<String>, nric_fin: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nric_fin: nric_fin.into(),
            appointment_date: Utc::now(),
            qualified: false,
            contact_email: None,
            contact_phone: None,
        }
    }
}

/// Singapore address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Address {
    /// Street address (e.g., "1 Raffles Place #12-34")
    pub street: String,

    /// Postal code (6 digits, e.g., "048616")
    pub postal_code: String,

    /// Country (default: "Singapore")
    pub country: String,
}

impl Address {
    /// Creates a Singapore address
    pub fn singapore(street: impl Into<String>, postal_code: impl Into<String>) -> Self {
        Self {
            street: street.into(),
            postal_code: postal_code.into(),
            country: "Singapore".to_string(),
        }
    }

    /// Creates a foreign address
    pub fn foreign(
        street: impl Into<String>,
        postal_code: impl Into<String>,
        country: impl Into<String>,
    ) -> Self {
        Self {
            street: street.into(),
            postal_code: postal_code.into(),
            country: country.into(),
        }
    }

    /// Returns whether this is a Singapore address
    pub fn is_singapore(&self) -> bool {
        self.country == "Singapore"
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, Singapore {}", self.street, self.postal_code)
    }
}

/// Month and day (for financial year end)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MonthDay {
    /// Month (1-12)
    pub month: u8,
    /// Day (1-31)
    pub day: u8,
}

impl MonthDay {
    /// Creates a new MonthDay
    pub fn new(month: u8, day: u8) -> Option<Self> {
        if (1..=12).contains(&month) && (1..=31).contains(&day) {
            Some(Self { month, day })
        } else {
            None
        }
    }

    /// Returns MonthDay as formatted string (e.g., "31 Dec")
    pub fn format(&self) -> String {
        let month_name = match self.month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "???",
        };
        format!("{} {}", self.day, month_name)
    }
}

impl std::fmt::Display for MonthDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type_requires_resident_director() {
        assert!(CompanyType::PrivateLimited.requires_resident_director());
        assert!(CompanyType::PublicLimited.requires_resident_director());
        assert!(!CompanyType::SoleProprietorship.requires_resident_director());
    }

    #[test]
    fn test_company_type_max_shareholders() {
        assert_eq!(CompanyType::PrivateLimited.max_shareholders(), Some(50));
        assert_eq!(CompanyType::PublicLimited.max_shareholders(), None);
    }

    #[test]
    fn test_share_capital_sgd_conversion() {
        let capital = ShareCapital::new(100_000_00); // SGD 100,000
        assert_eq!(capital.paid_up_sgd(), 100_000.0);
    }

    #[test]
    fn test_share_allocation_ownership_percentage() {
        let allocation = ShareAllocation::new("Ordinary", 25, 100_00);
        assert_eq!(allocation.ownership_percentage(100), 25.0);
    }

    #[test]
    fn test_director_eligibility() {
        let director = Director::new("John Tan", "S1234567A", true);
        assert!(director.is_eligible());
        assert_eq!(
            director.disqualification_status,
            DisqualificationStatus::Eligible
        );
    }

    #[test]
    fn test_address_is_singapore() {
        let sg_addr = Address::singapore("1 Raffles Place", "048616");
        assert!(sg_addr.is_singapore());

        let foreign_addr = Address::foreign("123 Main St", "10001", "USA");
        assert!(!foreign_addr.is_singapore());
    }

    #[test]
    fn test_month_day_format() {
        let fye = MonthDay { month: 12, day: 31 };
        assert_eq!(fye.format(), "31 Dec");
    }

    #[test]
    fn test_company_has_resident_director() {
        let mut company = Company::new(
            "202401234A",
            "Test Pte Ltd",
            CompanyType::PrivateLimited,
            Address::singapore("1 Raffles Place", "048616"),
        );

        assert!(!company.has_resident_director());

        company
            .directors
            .push(Director::new("John Tan", "S1234567A", true));
        assert!(company.has_resident_director());
    }

    #[test]
    fn test_share_class_ordinary() {
        let ordinary = ShareClass::ordinary(1000, Some(100_00));
        assert_eq!(ordinary.class_name, "Ordinary");
        assert!(ordinary.voting_rights);
        assert!(!ordinary.redeemable);
    }

    #[test]
    fn test_share_class_preference() {
        let preference = ShareClass::preference(500, Some(100_00), 500); // 5% = 500 bps
        assert_eq!(preference.class_name, "Preference");
        assert!(!preference.voting_rights);
        match preference.dividend_rights {
            DividendRights::Preference { rate_bps } => assert_eq!(rate_bps, 500),
            _ => panic!("Expected Preference dividend rights"),
        }
    }
}
