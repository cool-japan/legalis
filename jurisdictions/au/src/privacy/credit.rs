//! Credit Reporting (Part IIIA Privacy Act 1988)
//!
//! This module implements credit reporting provisions under Part IIIA of the
//! Privacy Act 1988, covering credit reporting bodies (CRBs) and credit
//! providers.
//!
//! ## Key Entities
//!
//! - **Credit Reporting Body (CRB)**: Entity that collects and discloses
//!   credit information (e.g., Equifax, Experian, illion)
//! - **Credit Provider (CP)**: Entity that provides credit and accesses
//!   credit reports

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Credit Reporting Body
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditReportingBody {
    /// Name
    pub name: String,
    /// ABN
    pub abn: Option<String>,
    /// CR Code signatory
    pub cr_code_signatory: bool,
    /// Website
    pub website: Option<String>,
    /// Contact details
    pub contact_details: ContactDetails,
}

impl CreditReportingBody {
    /// Create new CRB
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            abn: None,
            cr_code_signatory: false,
            website: None,
            contact_details: ContactDetails::default(),
        }
    }
}

/// Credit Provider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditProvider {
    /// Name
    pub name: String,
    /// ABN
    pub abn: Option<String>,
    /// Provider type
    pub provider_type: CreditProviderType,
    /// Australian Credit Licence number
    pub acl_number: Option<String>,
    /// Contact details
    pub contact_details: ContactDetails,
}

impl CreditProvider {
    /// Create new credit provider
    pub fn new(name: impl Into<String>, provider_type: CreditProviderType) -> Self {
        Self {
            name: name.into(),
            abn: None,
            provider_type,
            acl_number: None,
            contact_details: ContactDetails::default(),
        }
    }
}

/// Type of credit provider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditProviderType {
    /// Authorised deposit-taking institution (bank)
    Adi,
    /// Non-ADI lender
    NonAdiLender,
    /// Telecommunications provider
    Telco,
    /// Energy provider
    EnergyProvider,
    /// Insurance provider
    InsuranceProvider,
    /// Other credit provider
    Other,
}

/// Contact details
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContactDetails {
    /// Phone number
    pub phone: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Address
    pub address: Option<String>,
}

/// Credit information (information that can be held by CRB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditInformation {
    /// Type of credit information
    pub information_type: CreditInformationType,
    /// Description
    pub description: String,
    /// Collection date
    pub collection_date: DateTime<Utc>,
    /// Retention period (months)
    pub retention_period_months: u32,
    /// Expiry date
    pub expiry_date: DateTime<Utc>,
}

impl CreditInformation {
    /// Create new credit information
    pub fn new(information_type: CreditInformationType, description: impl Into<String>) -> Self {
        let now = Utc::now();
        let retention = information_type.retention_period_months();
        let expiry = now + chrono::Duration::days((retention * 30) as i64);

        Self {
            information_type,
            description: description.into(),
            collection_date: now,
            retention_period_months: retention,
            expiry_date: expiry,
        }
    }

    /// Check if information has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry_date
    }
}

/// Type of credit information (s.6N)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditInformationType {
    /// Identification information
    Identification,
    /// Consumer credit liability information
    ConsumerCreditLiability,
    /// Repayment history information
    RepaymentHistory,
    /// Default information
    Default,
    /// Payment information (clearing a default)
    Payment,
    /// New arrangement information
    NewArrangement,
    /// Court proceedings information
    CourtProceedings,
    /// Personal insolvency information
    PersonalInsolvency,
    /// Publicly available information
    PubliclyAvailable,
    /// Credit enquiry
    CreditEnquiry,
}

impl CreditInformationType {
    /// Get retention period in months (s.20W)
    pub fn retention_period_months(&self) -> u32 {
        match self {
            CreditInformationType::Identification => 0, // No limit while credit info held
            CreditInformationType::ConsumerCreditLiability => 24, // 2 years after close
            CreditInformationType::RepaymentHistory => 24, // 2 years
            CreditInformationType::Default => 60,       // 5 years
            CreditInformationType::Payment => 60,       // Same as default
            CreditInformationType::NewArrangement => 60, // Same as default
            CreditInformationType::CourtProceedings => 60, // 5 years
            CreditInformationType::PersonalInsolvency => 84, // 7 years (or longer for some)
            CreditInformationType::PubliclyAvailable => 84, // 7 years
            CreditInformationType::CreditEnquiry => 60, // 5 years
        }
    }
}

/// Credit report access request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditReportAccess {
    /// Request ID
    pub request_id: String,
    /// Individual requesting
    pub individual_id: String,
    /// CRB requested from
    pub crb: String,
    /// Request date
    pub request_date: DateTime<Utc>,
    /// Access type
    pub access_type: CreditAccessType,
    /// Status
    pub status: CreditAccessStatus,
}

impl CreditReportAccess {
    /// Create new access request
    pub fn new(
        request_id: impl Into<String>,
        individual_id: impl Into<String>,
        crb: impl Into<String>,
        access_type: CreditAccessType,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            individual_id: individual_id.into(),
            crb: crb.into(),
            request_date: Utc::now(),
            access_type,
            status: CreditAccessStatus::Pending,
        }
    }
}

/// Type of credit report access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditAccessType {
    /// Free annual report
    FreeAnnualReport,
    /// Paid credit report
    PaidCreditReport,
    /// Correction request
    CorrectionRequest,
    /// Dispute resolution
    DisputeResolution,
}

/// Credit access request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditAccessStatus {
    /// Pending
    Pending,
    /// Provided
    Provided,
    /// Denied
    Denied,
}

/// Credit reporting analyzer
pub struct CreditReportingAnalyzer;

impl CreditReportingAnalyzer {
    /// Check if credit information can be disclosed
    pub fn can_disclose(
        _crb: &CreditReportingBody,
        _recipient: &CreditProvider,
        purpose: CreditDisclosurePurpose,
    ) -> bool {
        // Disclosure permitted for specified purposes (s.20E)
        matches!(
            purpose,
            CreditDisclosurePurpose::AssessApplication
                | CreditDisclosurePurpose::CollectOverduePayment
                | CreditDisclosurePurpose::AssessGuarantor
        )
    }

    /// Calculate retention end date
    pub fn retention_end_date(info: &CreditInformation) -> DateTime<Utc> {
        info.collection_date + chrono::Duration::days((info.retention_period_months * 30) as i64)
    }

    /// Check if information should be destroyed
    pub fn should_destroy(info: &CreditInformation) -> bool {
        info.is_expired()
    }
}

/// Purpose of credit information disclosure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditDisclosurePurpose {
    /// Assess credit application
    AssessApplication,
    /// Collect overdue payment
    CollectOverduePayment,
    /// Assess application by guarantor
    AssessGuarantor,
    /// Internal management
    InternalManagement,
    /// Dealing with hardship
    HardshipArrangement,
    /// Other permitted purpose
    OtherPermitted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_information_retention() {
        let default_info =
            CreditInformation::new(CreditInformationType::Default, "Missed payment > 60 days");

        assert_eq!(default_info.retention_period_months, 60);
        assert!(!default_info.is_expired());
    }

    #[test]
    fn test_retention_periods() {
        assert_eq!(
            CreditInformationType::RepaymentHistory.retention_period_months(),
            24
        );
        assert_eq!(CreditInformationType::Default.retention_period_months(), 60);
        assert_eq!(
            CreditInformationType::PersonalInsolvency.retention_period_months(),
            84
        );
    }

    #[test]
    fn test_crb_creation() {
        let crb = CreditReportingBody::new("Equifax Australia");
        assert_eq!(crb.name, "Equifax Australia");
        assert!(!crb.cr_code_signatory);
    }

    #[test]
    fn test_credit_provider() {
        let provider = CreditProvider::new("Big Bank", CreditProviderType::Adi);
        assert_eq!(provider.provider_type, CreditProviderType::Adi);
    }

    #[test]
    fn test_disclosure_check() {
        let crb = CreditReportingBody::new("Test CRB");
        let provider = CreditProvider::new("Test Provider", CreditProviderType::Adi);

        assert!(CreditReportingAnalyzer::can_disclose(
            &crb,
            &provider,
            CreditDisclosurePurpose::AssessApplication
        ));
    }
}
