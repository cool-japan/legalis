//! AML/CTF Types (Money Laundering Regulations 2017, POCA 2002, Terrorism Act 2000)

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Customer Due Diligence level (MLR 2017 Reg 27-28, 33)
///
/// Three levels of CDD based on risk assessment:
/// - **Simplified**: Lower-risk customers (MLR 2017 Reg 37)
/// - **Standard**: Default level for most customers
/// - **Enhanced**: High-risk customers including PEPs (MLR 2017 Reg 33)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CddLevel {
    /// Simplified Due Diligence (lower-risk customers)
    ///
    /// MLR 2017 Reg 37: Credit institutions, listed companies
    Simplified,

    /// Standard Due Diligence (default level)
    ///
    /// MLR 2017 Reg 27-28: Apply to most customers
    Standard,

    /// Enhanced Due Diligence (high-risk customers)
    ///
    /// MLR 2017 Reg 33: PEPs, high-risk countries, complex ownership structures
    Enhanced,
}

/// Customer type for CDD purposes
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CustomerType {
    /// Individual customer
    Individual,

    /// Entity (company, partnership, trust)
    Entity {
        /// Entity type
        entity_type: EntityType,
    },
}

/// Entity type
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EntityType {
    /// Limited company
    Company,

    /// Partnership
    Partnership,

    /// Trust
    Trust,

    /// Other entity type
    Other {
        /// Description of entity type
        description: String,
    },
}

/// Customer Due Diligence assessment (MLR 2017 Reg 27)
///
/// Firms must apply CDD measures when:
/// - Establishing a business relationship
/// - Carrying out an occasional transaction ≥€15,000
/// - Suspecting money laundering or terrorist financing
/// - Doubting the veracity of previously obtained customer identification
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CustomerDueDiligence {
    /// Customer name
    pub customer_name: String,

    /// Customer type (Individual or Entity)
    pub customer_type: CustomerType,

    /// CDD level applied
    pub cdd_level: CddLevel,

    /// Date of CDD assessment
    pub assessment_date: NaiveDate,

    // Identity verification (MLR 2017 Reg 28(2))
    /// Whether customer identity has been verified
    pub identity_verified: bool,

    /// Identity documents used for verification
    pub identity_documents: Vec<IdentityDocument>,

    // Beneficial ownership (MLR 2017 Reg 5, 28(3))
    /// Beneficial owners (>25% ownership threshold)
    pub beneficial_owners: Vec<BeneficialOwner>,

    /// Whether ownership structure has been verified
    pub ownership_structure_verified: bool,

    // Business relationship (MLR 2017 Reg 28(3)(c))
    /// Purpose and intended nature of business relationship
    pub purpose_of_relationship: String,

    /// Nature of customer's business
    pub nature_of_business: String,

    // Source of funds/wealth (MLR 2017 Reg 33(4)(b) for EDD)
    /// Source of funds (for Enhanced DD)
    pub source_of_funds: Option<String>,

    /// Source of wealth (for Enhanced DD)
    pub source_of_wealth: Option<String>,

    // Risk assessment
    /// Risk rating assigned to customer
    pub risk_rating: RiskRating,

    /// PEP status (triggers Enhanced DD if not NonPep)
    pub pep_status: PepStatus,

    /// Whether sanctions screening has been passed
    pub sanctions_screening_passed: bool,

    // Ongoing monitoring (MLR 2017 Reg 28(4))
    /// Frequency of ongoing monitoring
    pub ongoing_monitoring_frequency: MonitoringFrequency,

    /// Date of last CDD review
    pub last_review_date: Option<NaiveDate>,
}

/// Identity document used for verification
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdentityDocument {
    /// Document type (e.g., "Passport", "Driving Licence", "National ID Card")
    pub document_type: String,

    /// Document number
    pub document_number: String,

    /// Issuing country
    pub issuing_country: String,

    /// Document expiry date
    pub expiry_date: Option<NaiveDate>,

    /// Whether document has been verified
    pub verified: bool,
}

/// Beneficial owner (MLR 2017 Reg 5)
///
/// Individual who ultimately owns or controls >25% of entity
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeneficialOwner {
    /// Name of beneficial owner
    pub name: String,

    /// Ownership percentage
    pub ownership_percentage: f64,

    /// Control type (e.g., "Shareholding", "Voting rights", "Other control")
    pub control_type: String,

    /// Whether identity has been verified
    pub identity_verified: bool,
}

/// Ongoing monitoring frequency
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MonitoringFrequency {
    /// Annual review
    Annual,

    /// Semi-annual review
    SemiAnnual,

    /// Quarterly review
    Quarterly,

    /// Monthly review (for high-risk customers)
    Monthly,

    /// Continuous monitoring
    Continuous,
}

/// Politically Exposed Person status (MLR 2017 Reg 35)
///
/// PEPs are individuals who are or have been entrusted with prominent public functions.
/// Enhanced Due Diligence required for all PEP categories.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PepStatus {
    /// Not a PEP
    NonPep,

    /// Domestic PEP (UK public figure)
    ///
    /// Individuals entrusted with prominent public functions in the UK
    DomesticPep {
        /// Position held (e.g., "Minister", "MP", "Senior Judge")
        position: String,
    },

    /// Foreign PEP (entrusted with prominent function by foreign state)
    ///
    /// Individuals entrusted with prominent public functions by foreign country
    ForeignPep {
        /// Country where position is held
        country: String,

        /// Position held
        position: String,
    },

    /// International organization PEP
    ///
    /// Members of governing bodies of international organizations (UN, EU, NATO, etc.)
    InternationalOrgPep {
        /// Organization name
        organization: String,

        /// Position held
        position: String,
    },

    /// Family member of PEP (MLR 2017 Reg 35(14)(a))
    ///
    /// Spouse, civil partner, children and their spouses/partners, parents
    PepFamilyMember {
        /// Relationship to PEP
        relationship_to_pep: String,
    },

    /// Known close associate of PEP (MLR 2017 Reg 35(14)(b))
    ///
    /// Individuals known to have joint beneficial ownership or close business relationship
    PepCloseAssociate {
        /// Relationship to PEP
        relationship_to_pep: String,
    },
}

impl PepStatus {
    /// Check if Enhanced Due Diligence required (MLR 2017 Reg 35(4))
    ///
    /// Returns true for all PEP categories except NonPep.
    /// Enhanced DD requires:
    /// - Senior management approval
    /// - Adequate measures to establish source of wealth and funds
    /// - Enhanced ongoing monitoring
    pub fn requires_edd(&self) -> bool {
        !matches!(self, PepStatus::NonPep)
    }

    /// Check if this is a domestic PEP (different treatment from foreign PEPs)
    pub fn is_domestic_pep(&self) -> bool {
        matches!(self, PepStatus::DomesticPep { .. })
    }
}

/// Risk rating for AML/CTF purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskRating {
    /// Low risk customer
    Low,

    /// Medium risk customer
    Medium,

    /// High risk customer (requires Enhanced DD)
    High,

    /// Prohibited - too high risk, cannot onboard
    Prohibited,
}

/// Suspicious Activity Report (MLR 2017 Reg 40, POCA 2002 s.330-332)
///
/// Firms in the regulated sector must report knowledge or suspicion of money laundering
/// or terrorist financing to the National Crime Agency (NCA).
///
/// Failure to report is a criminal offence punishable by up to 5 years imprisonment.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SuspiciousActivityReport {
    /// Unique report identifier
    pub report_id: String,

    /// Date report created
    pub report_date: NaiveDate,

    /// Name of subject of suspicion
    pub subject_name: String,

    /// Subject identifier (e.g., customer ID)
    pub subject_id: Option<String>,

    /// Type of suspicious activity
    pub suspicion_type: SuspicionType,

    /// Grounds for suspicion (detailed description)
    pub grounds_for_suspicion: String,

    // Transaction details (if applicable)
    /// Transaction amount in GBP
    pub transaction_amount_gbp: Option<f64>,

    /// Transaction date
    pub transaction_date: Option<NaiveDate>,

    /// Transaction description
    pub transaction_description: Option<String>,

    // Reporting to NCA
    /// Whether report has been submitted to NCA
    pub reported_to_nca: bool,

    /// NCA reference number (assigned after submission)
    pub nca_reference: Option<String>,

    /// Whether NCA consent obtained (POCA s.335 consent regime)
    ///
    /// For some transactions, firms must obtain NCA consent before proceeding
    pub nca_consent_obtained: Option<bool>,
}

/// Type of suspicious activity
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SuspicionType {
    /// Money laundering (POCA 2002 s.330)
    MoneyLaundering,

    /// Terrorist financing (Terrorism Act 2000 s.21A)
    TerroristFinancing,

    /// Sanctions violation
    SanctionsViolation,

    /// Fraud
    Fraud,

    /// Other suspicious activity
    Other {
        /// Description of activity type
        description: String,
    },
}

/// Sanctions screening (OFSI, UN, EU sanctions)
///
/// Firms must screen customers against sanctions lists to ensure they are not
/// dealing with sanctioned persons or entities.
///
/// UK sanctions administered by Office of Financial Sanctions Implementation (OFSI).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SanctionsScreening {
    /// Date screening performed
    pub screening_date: NaiveDate,

    /// Name of person/entity being screened
    pub subject_name: String,

    // Sanctions lists checked
    /// UK OFSI sanctions list checked
    pub ofsi_checked: bool,

    /// UN sanctions list checked
    pub un_checked: bool,

    /// EU sanctions list checked (post-Brexit)
    pub eu_checked: bool,

    // Screening result
    /// Whether a match was found on any sanctions list
    pub match_found: bool,

    /// Details of any match found
    pub match_details: Option<String>,

    // False positive review
    /// Whether match has been determined to be false positive
    pub false_positive: bool,

    /// Person who reviewed false positive
    pub reviewed_by: Option<String>,
}

/// Cryptoasset Travel Rule compliance (MLR 2017 reg 14A, FATF Recommendation 16)
///
/// Cryptoasset exchange providers must obtain and transmit information on originator
/// and beneficiary for transfers ≥£1,000.
///
/// This implements FATF Recommendation 16 (Travel Rule) for virtual assets.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TravelRuleTransfer {
    /// Unique transaction identifier
    pub transaction_id: String,

    /// Transaction date
    pub transaction_date: NaiveDate,

    /// Transfer amount in GBP (threshold: £1000)
    pub amount_gbp: f64,

    // Originator information (MLR 2017 reg 14A(3))
    /// Originator name
    pub originator_name: String,

    /// Originator wallet address
    pub originator_wallet_address: String,

    /// Originator account number (if applicable)
    pub originator_account_number: Option<String>,

    // Beneficiary information (MLR 2017 reg 14A(4))
    /// Beneficiary name
    pub beneficiary_name: String,

    /// Beneficiary wallet address
    pub beneficiary_wallet_address: String,

    /// Beneficiary account number (if applicable)
    pub beneficiary_account_number: Option<String>,

    // Travel Rule compliance
    /// Whether required information has been transmitted to receiving institution
    pub information_transmitted: bool,

    /// Method used to transmit information
    pub transmission_method: Option<String>,
}

impl TravelRuleTransfer {
    /// Check if Travel Rule applies (≥£1000 threshold)
    ///
    /// MLR 2017 reg 14A applies to cryptoasset transfers ≥£1,000
    pub fn travel_rule_applies(&self) -> bool {
        self.amount_gbp >= 1000.0
    }

    /// Check if transfer is compliant with Travel Rule
    pub fn is_compliant(&self) -> bool {
        if !self.travel_rule_applies() {
            // Travel Rule doesn't apply for transfers < £1000
            return true;
        }

        // Check all required information is present and transmitted
        self.information_transmitted
            && !self.originator_name.is_empty()
            && !self.originator_wallet_address.is_empty()
            && !self.beneficiary_name.is_empty()
            && !self.beneficiary_wallet_address.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pep_status_requires_edd() {
        assert!(!PepStatus::NonPep.requires_edd());

        assert!(
            PepStatus::DomesticPep {
                position: "Minister".to_string()
            }
            .requires_edd()
        );

        assert!(
            PepStatus::ForeignPep {
                country: "France".to_string(),
                position: "Minister".to_string()
            }
            .requires_edd()
        );

        assert!(
            PepStatus::PepFamilyMember {
                relationship_to_pep: "Spouse".to_string()
            }
            .requires_edd()
        );
    }

    #[test]
    fn test_pep_status_is_domestic() {
        assert!(!PepStatus::NonPep.is_domestic_pep());

        assert!(
            PepStatus::DomesticPep {
                position: "MP".to_string()
            }
            .is_domestic_pep()
        );

        assert!(
            !PepStatus::ForeignPep {
                country: "USA".to_string(),
                position: "Senator".to_string()
            }
            .is_domestic_pep()
        );
    }

    #[test]
    fn test_travel_rule_threshold() {
        let transfer_below_threshold = TravelRuleTransfer {
            transaction_id: "TX001".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 999.99,
            originator_name: String::new(),
            originator_wallet_address: String::new(),
            originator_account_number: None,
            beneficiary_name: String::new(),
            beneficiary_wallet_address: String::new(),
            beneficiary_account_number: None,
            information_transmitted: false,
            transmission_method: None,
        };

        assert!(!transfer_below_threshold.travel_rule_applies());
        assert!(transfer_below_threshold.is_compliant()); // Compliant because < £1000

        let transfer_above_threshold = TravelRuleTransfer {
            amount_gbp: 1000.0,
            ..transfer_below_threshold.clone()
        };

        assert!(transfer_above_threshold.travel_rule_applies());
        assert!(!transfer_above_threshold.is_compliant()); // Non-compliant: information not transmitted
    }

    #[test]
    fn test_travel_rule_compliance() {
        let compliant_transfer = TravelRuleTransfer {
            transaction_id: "TX002".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 5000.0,
            originator_name: "Alice Smith".to_string(),
            originator_wallet_address: "0x1234...".to_string(),
            originator_account_number: Some("ACC001".to_string()),
            beneficiary_name: "Bob Jones".to_string(),
            beneficiary_wallet_address: "0x5678...".to_string(),
            beneficiary_account_number: Some("ACC002".to_string()),
            information_transmitted: true,
            transmission_method: Some("SWIFT message".to_string()),
        };

        assert!(compliant_transfer.is_compliant());
    }

    #[test]
    fn test_risk_rating_ordering() {
        assert!(RiskRating::Low < RiskRating::Medium);
        assert!(RiskRating::Medium < RiskRating::High);
        assert!(RiskRating::High < RiskRating::Prohibited);
    }
}
