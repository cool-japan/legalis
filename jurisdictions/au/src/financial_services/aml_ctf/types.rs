//! AML/CTF Types (AUSTRAC)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Australian Customer Due Diligence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuCustomerDueDiligence {
    /// Customer name
    pub customer_name: String,
    /// Customer type
    pub customer_type: CustomerType,
    /// CDD level applied
    pub cdd_level: CddLevel,
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// Identity verified
    pub identity_verified: bool,
    /// Verification documents
    pub documents: Vec<IdentityDocument>,
    /// Beneficial owners (for entities)
    pub beneficial_owners: Vec<BeneficialOwner>,
    /// PEP status
    pub pep_status: PepStatus,
    /// Risk rating
    pub risk_rating: RiskRating,
    /// Ongoing monitoring scheduled
    pub ongoing_monitoring: bool,
    /// Monitoring frequency
    pub monitoring_frequency: Option<MonitoringFrequency>,
    /// Source of funds (for EDD)
    pub source_of_funds: Option<String>,
    /// Source of wealth (for EDD)
    pub source_of_wealth: Option<String>,
    /// Purpose of business relationship
    pub purpose_of_relationship: Option<String>,
    /// Sanctions screening passed
    pub sanctions_screening_passed: bool,
}

/// Customer types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomerType {
    /// Individual customer
    Individual,
    /// Sole trader
    SoleTrader,
    /// Company
    Company,
    /// Trust
    Trust,
    /// Partnership
    Partnership,
    /// Association
    Association,
    /// Government body
    GovernmentBody,
    /// Superannuation fund
    SuperannuationFund,
}

/// CDD levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CddLevel {
    /// Simplified due diligence (low risk)
    Simplified,
    /// Standard due diligence
    Standard,
    /// Enhanced due diligence (high risk)
    Enhanced,
}

impl CddLevel {
    /// Check if source of funds required
    pub fn requires_source_of_funds(&self) -> bool {
        matches!(self, CddLevel::Enhanced)
    }

    /// Check if source of wealth required
    pub fn requires_source_of_wealth(&self) -> bool {
        matches!(self, CddLevel::Enhanced)
    }

    /// Check if senior management approval required
    pub fn requires_senior_approval(&self) -> bool {
        matches!(self, CddLevel::Enhanced)
    }
}

/// Identity document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityDocument {
    /// Document type
    pub document_type: DocumentType,
    /// Document number
    pub document_number: String,
    /// Issuing country
    pub issuing_country: String,
    /// Expiry date
    pub expiry_date: Option<NaiveDate>,
    /// Document verified
    pub verified: bool,
}

/// Document types for identity verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// Australian passport
    Passport,
    /// Australian driver licence
    DriverLicence,
    /// Medicare card
    MedicareCard,
    /// Birth certificate
    BirthCertificate,
    /// Citizenship certificate
    CitizenshipCertificate,
    /// ImmiCard
    ImmiCard,
    /// Foreign passport
    ForeignPassport,
    /// National identity card
    NationalIdCard,
    /// Photo ID card
    PhotoIdCard,
}

impl DocumentType {
    /// Check if primary document (Category A)
    pub fn is_primary(&self) -> bool {
        matches!(
            self,
            DocumentType::Passport
                | DocumentType::DriverLicence
                | DocumentType::ForeignPassport
                | DocumentType::NationalIdCard
        )
    }

    /// Check if secondary document (Category B)
    pub fn is_secondary(&self) -> bool {
        matches!(
            self,
            DocumentType::MedicareCard
                | DocumentType::BirthCertificate
                | DocumentType::CitizenshipCertificate
                | DocumentType::ImmiCard
        )
    }
}

/// Beneficial owner
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeneficialOwner {
    /// Owner name
    pub name: String,
    /// Ownership percentage
    pub ownership_percent: Option<f64>,
    /// Nature of control
    pub control_type: ControlType,
    /// Identity verified
    pub identity_verified: bool,
    /// PEP status
    pub pep_status: PepStatus,
    /// Country of residence
    pub country_of_residence: String,
}

/// Types of control for beneficial ownership
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlType {
    /// Direct ownership (>25%)
    DirectOwnership,
    /// Indirect ownership through chain
    IndirectOwnership,
    /// Voting rights
    VotingRights,
    /// Director or senior management
    DirectorOrManagement,
    /// Trustee or beneficiary
    TrusteeOrBeneficiary,
    /// Other control
    OtherControl,
}

/// Entity types for beneficial ownership
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    /// Company
    Company,
    /// Trust
    Trust,
    /// Partnership
    Partnership,
    /// Association
    Association,
    /// Managed investment scheme
    ManagedInvestmentScheme,
    /// Superannuation fund
    SuperannuationFund,
}

/// PEP (Politically Exposed Person) status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PepStatus {
    /// Not a PEP
    NonPep,
    /// Domestic PEP (Australian)
    DomesticPep { position: String },
    /// Foreign PEP
    ForeignPep { country: String, position: String },
    /// International organisation PEP
    InternationalOrgPep {
        organisation: String,
        position: String,
    },
    /// PEP family member
    PepFamilyMember {
        relationship: String,
        pep_name: String,
    },
    /// PEP close associate
    PepCloseAssociate { nature: String, pep_name: String },
}

impl PepStatus {
    /// Check if PEP
    pub fn is_pep(&self) -> bool {
        !matches!(self, PepStatus::NonPep)
    }

    /// Check if enhanced DD required
    pub fn requires_enhanced_dd(&self) -> bool {
        self.is_pep()
    }
}

/// Risk rating
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskRating {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Prohibited (cannot onboard)
    Prohibited,
}

impl RiskRating {
    /// Get required CDD level
    pub fn required_cdd_level(&self) -> CddLevel {
        match self {
            RiskRating::Low => CddLevel::Simplified,
            RiskRating::Medium => CddLevel::Standard,
            RiskRating::High | RiskRating::Prohibited => CddLevel::Enhanced,
        }
    }
}

/// Monitoring frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringFrequency {
    /// Monthly review
    Monthly,
    /// Quarterly review
    Quarterly,
    /// Semi-annual review
    SemiAnnual,
    /// Annual review
    Annual,
    /// Continuous monitoring
    Continuous,
}

/// AUSTRAC compliance status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AustracCompliance {
    /// Has AML/CTF program
    pub has_program: bool,
    /// Program last reviewed
    pub program_review_date: Option<NaiveDate>,
    /// Has MLRO appointed
    pub has_mlro: bool,
    /// MLRO details
    pub mlro: Option<MlroDetails>,
    /// Employee training completed
    pub employee_training: bool,
    /// Training completion rate (%)
    pub training_completion_rate: Option<f64>,
    /// Registered with AUSTRAC
    pub austrac_registered: bool,
    /// Registration number
    pub registration_number: Option<String>,
    /// Independent review conducted
    pub independent_review: bool,
    /// Last independent review date
    pub last_review_date: Option<NaiveDate>,
}

/// MLRO (Money Laundering Reporting Officer) details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MlroDetails {
    /// MLRO name
    pub name: String,
    /// Position
    pub position: String,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Training completed
    pub training_completed: bool,
    /// Contact details on file
    pub contact_on_file: bool,
}

/// AML/CTF Program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmlCtfProgram {
    /// Part A: General requirements
    pub part_a: PartAProgram,
    /// Part B: Customer identification
    pub part_b: PartBProgram,
    /// Last review date
    pub last_review_date: NaiveDate,
    /// Board approved
    pub board_approved: bool,
}

/// Part A - General requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartAProgram {
    /// MLRO appointed
    pub mlro_appointed: bool,
    /// Employee due diligence
    pub employee_due_diligence: bool,
    /// Training program
    pub training_program: bool,
    /// Record keeping procedures
    pub record_keeping: bool,
    /// Risk assessment
    pub risk_assessment: bool,
    /// Reporting procedures
    pub reporting_procedures: bool,
}

/// Part B - Customer identification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartBProgram {
    /// CDD procedures documented
    pub cdd_procedures: bool,
    /// Verification methods
    pub verification_methods: bool,
    /// Beneficial ownership procedures
    pub beneficial_ownership: bool,
    /// PEP screening
    pub pep_screening: bool,
    /// Sanctions screening
    pub sanctions_screening: bool,
    /// EDD procedures
    pub edd_procedures: bool,
}

/// Suspicious Matter Report (SMR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuspiciousMatterReport {
    /// Report ID
    pub report_id: String,
    /// Report date
    pub report_date: DateTime<Utc>,
    /// Subject name
    pub subject_name: String,
    /// Subject identifier
    pub subject_identifier: Option<String>,
    /// Suspicion type
    pub suspicion_type: SuspicionType,
    /// Grounds for suspicion
    pub grounds: String,
    /// Transaction amount (AUD)
    pub transaction_amount_aud: Option<f64>,
    /// Transaction date
    pub transaction_date: Option<NaiveDate>,
    /// Transaction description
    pub transaction_description: Option<String>,
    /// Submitted to AUSTRAC
    pub submitted_to_austrac: bool,
    /// AUSTRAC reference
    pub austrac_reference: Option<String>,
    /// Submission deadline
    pub submission_deadline: DateTime<Utc>,
}

/// Suspicion types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuspicionType {
    /// Money laundering
    MoneyLaundering,
    /// Terrorism financing
    TerrorismFinancing,
    /// Tax evasion
    TaxEvasion,
    /// Fraud
    Fraud,
    /// Structuring
    Structuring,
    /// Other criminal activity
    OtherCriminal,
}

/// Threshold Transaction Report (TTR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThresholdTransaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Transaction date
    pub transaction_date: NaiveDate,
    /// Transaction type
    pub transaction_type: ThresholdTransactionType,
    /// Amount (AUD)
    pub amount_aud: f64,
    /// Customer name
    pub customer_name: String,
    /// Customer identifier
    pub customer_identifier: String,
    /// Submitted to AUSTRAC
    pub submitted_to_austrac: bool,
    /// Submission deadline
    pub submission_deadline: NaiveDate,
}

/// Threshold transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThresholdTransactionType {
    /// Cash deposit
    CashDeposit,
    /// Cash withdrawal
    CashWithdrawal,
    /// Cash exchange
    CashExchange,
    /// Traveller's cheques
    TravellersCheques,
}

/// International Funds Transfer Instruction (IFTI)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternationalFundsTransfer {
    /// Transfer ID
    pub transfer_id: String,
    /// Transfer date
    pub transfer_date: NaiveDate,
    /// Direction
    pub direction: TransferDirection,
    /// Amount (AUD equivalent)
    pub amount_aud: f64,
    /// Currency
    pub currency: String,
    /// Originator name
    pub originator_name: String,
    /// Originator country
    pub originator_country: String,
    /// Beneficiary name
    pub beneficiary_name: String,
    /// Beneficiary country
    pub beneficiary_country: String,
    /// Submitted to AUSTRAC
    pub submitted_to_austrac: bool,
    /// Submission deadline
    pub submission_deadline: NaiveDate,
}

/// Transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferDirection {
    /// Incoming to Australia
    Incoming,
    /// Outgoing from Australia
    Outgoing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdd_level_requirements() {
        assert!(!CddLevel::Standard.requires_source_of_funds());
        assert!(CddLevel::Enhanced.requires_source_of_funds());
        assert!(CddLevel::Enhanced.requires_senior_approval());
    }

    #[test]
    fn test_pep_status() {
        assert!(!PepStatus::NonPep.is_pep());
        assert!(!PepStatus::NonPep.requires_enhanced_dd());

        let pep = PepStatus::ForeignPep {
            country: "US".to_string(),
            position: "Senator".to_string(),
        };
        assert!(pep.is_pep());
        assert!(pep.requires_enhanced_dd());
    }

    #[test]
    fn test_risk_rating_cdd() {
        assert_eq!(RiskRating::Low.required_cdd_level(), CddLevel::Simplified);
        assert_eq!(RiskRating::Medium.required_cdd_level(), CddLevel::Standard);
        assert_eq!(RiskRating::High.required_cdd_level(), CddLevel::Enhanced);
    }

    #[test]
    fn test_document_type_categories() {
        assert!(DocumentType::Passport.is_primary());
        assert!(DocumentType::DriverLicence.is_primary());
        assert!(!DocumentType::MedicareCard.is_primary());
        assert!(DocumentType::MedicareCard.is_secondary());
    }
}
