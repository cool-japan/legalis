//! Information Technology Act 2000 Types
//!
//! Types for cyber law under the Information Technology Act, 2000

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Electronic record (Section 2(1)(t))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElectronicRecord {
    /// Record identifier
    pub id: String,
    /// Record type
    pub record_type: ElectronicRecordType,
    /// Hash/checksum
    pub hash: Option<String>,
    /// Creation date
    pub created: NaiveDate,
    /// Originator
    pub originator: Option<String>,
    /// Addressee
    pub addressee: Option<String>,
}

/// Type of electronic record
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectronicRecordType {
    /// Data/information
    Data,
    /// Electronic document
    Document,
    /// Email
    Email,
    /// Electronic message
    Message,
    /// Digital image
    Image,
    /// Audio recording
    Audio,
    /// Video recording
    Video,
}

/// Digital signature type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalSignatureType {
    /// Class 1 - Individual verification
    Class1,
    /// Class 2 - Organization verification
    Class2,
    /// Class 3 - Personal appearance verification
    Class3,
    /// Aadhaar eSign
    AadhaarEsign,
    /// Electronic signature (Section 3A)
    ElectronicSignature,
}

impl DigitalSignatureType {
    /// Get security level
    pub fn security_level(&self) -> &'static str {
        match self {
            Self::Class1 => "Low",
            Self::Class2 => "Medium",
            Self::Class3 => "High",
            Self::AadhaarEsign => "Medium-High",
            Self::ElectronicSignature => "Variable",
        }
    }

    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::Class1 | Self::Class2 | Self::Class3 => "Section 3",
            Self::AadhaarEsign => "Section 3A",
            Self::ElectronicSignature => "Section 3A",
        }
    }
}

/// Digital signature certificate (Section 35)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalCertificate {
    /// Certificate serial number
    pub serial_number: String,
    /// Subject name
    pub subject_name: String,
    /// Issuing CA
    pub issuer: String,
    /// Certificate type
    pub cert_type: DigitalSignatureType,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Expiry date
    pub expiry_date: NaiveDate,
    /// Revoked status
    pub revoked: bool,
    /// Revocation date
    pub revocation_date: Option<NaiveDate>,
}

impl DigitalCertificate {
    /// Check if certificate is valid
    pub fn is_valid(&self, check_date: NaiveDate) -> bool {
        !self.revoked && check_date >= self.issue_date && check_date <= self.expiry_date
    }
}

/// Cyber crime category (Chapter XI)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CyberCrimeCategory {
    /// Tampering with computer source code (Section 65)
    TamperingSourceCode,
    /// Hacking (Section 66)
    Hacking,
    /// Receiving stolen computer resource (Section 66B)
    ReceivingStolenResource,
    /// Identity theft (Section 66C)
    IdentityTheft,
    /// Cheating by personation (Section 66D)
    CheatingByPersonation,
    /// Violation of privacy (Section 66E)
    ViolationOfPrivacy,
    /// Cyber terrorism (Section 66F)
    CyberTerrorism,
    /// Publishing obscene material (Section 67)
    ObsceneMaterial,
    /// Publishing sexually explicit material (Section 67A)
    SexuallyExplicitMaterial,
    /// Child pornography (Section 67B)
    ChildPornography,
    /// Failure to protect data (Section 43A)
    FailureToProtectData,
    /// Interception breach (Section 69)
    InterceptionBreach,
    /// Blocking access breach (Section 69A)
    BlockingAccessBreach,
}

impl CyberCrimeCategory {
    /// Get section number
    pub fn section(&self) -> &'static str {
        match self {
            Self::TamperingSourceCode => "Section 65",
            Self::Hacking => "Section 66",
            Self::ReceivingStolenResource => "Section 66B",
            Self::IdentityTheft => "Section 66C",
            Self::CheatingByPersonation => "Section 66D",
            Self::ViolationOfPrivacy => "Section 66E",
            Self::CyberTerrorism => "Section 66F",
            Self::ObsceneMaterial => "Section 67",
            Self::SexuallyExplicitMaterial => "Section 67A",
            Self::ChildPornography => "Section 67B",
            Self::FailureToProtectData => "Section 43A",
            Self::InterceptionBreach => "Section 69",
            Self::BlockingAccessBreach => "Section 69A",
        }
    }

    /// Get punishment
    pub fn punishment(&self) -> Punishment {
        match self {
            Self::TamperingSourceCode => Punishment {
                imprisonment_max_years: Some(3),
                fine_max_rupees: Some(200_000),
                bail_status: BailStatus::Bailable,
            },
            Self::Hacking => Punishment {
                imprisonment_max_years: Some(3),
                fine_max_rupees: Some(500_000),
                bail_status: BailStatus::Bailable,
            },
            Self::IdentityTheft | Self::CheatingByPersonation => Punishment {
                imprisonment_max_years: Some(3),
                fine_max_rupees: Some(100_000),
                bail_status: BailStatus::Bailable,
            },
            Self::ViolationOfPrivacy => Punishment {
                imprisonment_max_years: Some(3),
                fine_max_rupees: Some(200_000),
                bail_status: BailStatus::Bailable,
            },
            Self::CyberTerrorism => Punishment {
                imprisonment_max_years: None, // Life imprisonment
                fine_max_rupees: None,
                bail_status: BailStatus::NonBailable,
            },
            Self::ObsceneMaterial => Punishment {
                imprisonment_max_years: Some(5), // First offence: 3, subsequent: 5
                fine_max_rupees: Some(1_000_000),
                bail_status: BailStatus::Bailable,
            },
            Self::SexuallyExplicitMaterial => Punishment {
                imprisonment_max_years: Some(7), // First offence: 5, subsequent: 7
                fine_max_rupees: Some(1_000_000),
                bail_status: BailStatus::NonBailable,
            },
            Self::ChildPornography => Punishment {
                imprisonment_max_years: Some(7), // First offence: 5, subsequent: 7
                fine_max_rupees: Some(1_000_000),
                bail_status: BailStatus::NonBailable,
            },
            Self::FailureToProtectData
            | Self::InterceptionBreach
            | Self::BlockingAccessBreach
            | Self::ReceivingStolenResource => Punishment {
                imprisonment_max_years: Some(3),
                fine_max_rupees: Some(500_000),
                bail_status: BailStatus::Bailable,
            },
        }
    }
}

/// Punishment details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Punishment {
    /// Maximum imprisonment in years (None = life)
    pub imprisonment_max_years: Option<u32>,
    /// Maximum fine in rupees (None = no upper limit)
    pub fine_max_rupees: Option<u64>,
    /// Bail status
    pub bail_status: BailStatus,
}

/// Bail status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BailStatus {
    /// Bailable offence
    Bailable,
    /// Non-bailable offence
    NonBailable,
}

/// Computer related offences (Section 43)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComputerOffence {
    /// Unauthorized access (Section 43(a))
    UnauthorizedAccess,
    /// Unauthorized download (Section 43(b))
    UnauthorizedDownload,
    /// Introducing virus/malware (Section 43(c))
    IntroducingMalware,
    /// Damaging computer system (Section 43(d))
    DamagingSystem,
    /// Disruption of service (Section 43(e))
    DisruptingService,
    /// Denying access (Section 43(f))
    DenyingAccess,
    /// Assisting in contravention (Section 43(g))
    AssistingContravention,
    /// Charging services to another (Section 43(h))
    ChargingServicesToOther,
    /// Destroying evidence (Section 43(i))
    DestroyingEvidence,
    /// Breach of confidentiality (Section 43(j))
    BreachOfConfidentiality,
}

impl ComputerOffence {
    /// Get clause reference
    pub fn clause(&self) -> &'static str {
        match self {
            Self::UnauthorizedAccess => "Section 43(a)",
            Self::UnauthorizedDownload => "Section 43(b)",
            Self::IntroducingMalware => "Section 43(c)",
            Self::DamagingSystem => "Section 43(d)",
            Self::DisruptingService => "Section 43(e)",
            Self::DenyingAccess => "Section 43(f)",
            Self::AssistingContravention => "Section 43(g)",
            Self::ChargingServicesToOther => "Section 43(h)",
            Self::DestroyingEvidence => "Section 43(i)",
            Self::BreachOfConfidentiality => "Section 43(j)",
        }
    }

    /// Civil compensation available
    pub fn max_compensation(&self) -> u64 {
        // Section 43: "he shall be liable to pay damages by way of compensation
        // to the person so affected"
        // No upper limit specified (as amended in 2008, removed 1 crore cap)
        u64::MAX
    }
}

/// Intermediary type (Section 2(1)(w))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntermediaryType {
    /// Internet Service Provider
    Isp,
    /// Social Media Platform
    SocialMedia,
    /// E-commerce Platform
    Ecommerce,
    /// Search Engine
    SearchEngine,
    /// Online Payment Gateway
    PaymentGateway,
    /// Online Gaming Platform
    Gaming,
    /// Cloud Service Provider
    CloudService,
    /// Messaging Service
    Messaging,
    /// Data Centre
    DataCentre,
}

impl IntermediaryType {
    /// Check if Significant Social Media Intermediary (SSMI)
    /// Based on IT Rules 2021
    pub fn is_ssmi_threshold_applicable(&self) -> bool {
        matches!(self, Self::SocialMedia | Self::Messaging | Self::Gaming)
    }

    /// Get compliance requirements
    pub fn compliance_requirements(&self) -> Vec<&'static str> {
        let mut requirements = vec![
            "Due diligence (Rule 3)",
            "Privacy policy (Rule 3(1)(a))",
            "User agreement (Rule 3(1)(b))",
            "Grievance mechanism (Rule 3(2))",
        ];

        if self.is_ssmi_threshold_applicable() {
            requirements.extend([
                "Chief Compliance Officer (Rule 4(1)(a))",
                "Nodal Contact Person (Rule 4(1)(b))",
                "Resident Grievance Officer (Rule 4(1)(c))",
                "Monthly compliance report (Rule 4(1)(d))",
                "First originator identification (Rule 4(2))",
            ]);
        }

        requirements
    }
}

/// Intermediary compliance check parameters (IT Rules 2021)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntermediaryComplianceCheck {
    /// Type of intermediary
    pub intermediary_type: IntermediaryType,
    /// Number of registered users in India
    pub user_count: u64,
    /// Has appointed Grievance Officer
    pub has_grievance_officer: bool,
    /// Has published privacy policy
    pub has_privacy_policy: bool,
    /// Has published user agreement
    pub has_user_agreement: bool,
    /// Has appointed Chief Compliance Officer (SSMI only)
    pub has_compliance_officer: bool,
    /// Has appointed Nodal Contact Person (SSMI only)
    pub has_nodal_person: bool,
    /// Monthly compliance report filed (SSMI only)
    pub monthly_report_filed: bool,
}

impl Default for IntermediaryComplianceCheck {
    fn default() -> Self {
        Self {
            intermediary_type: IntermediaryType::SocialMedia,
            user_count: 0,
            has_grievance_officer: false,
            has_privacy_policy: false,
            has_user_agreement: false,
            has_compliance_officer: false,
            has_nodal_person: false,
            monthly_report_filed: false,
        }
    }
}

/// Intermediary safe harbor conditions (Section 79)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafeHarborConditions {
    /// Function is limited to transmission/storage
    pub limited_function: bool,
    /// Does not initiate transmission
    pub does_not_initiate: bool,
    /// Does not select receiver
    pub does_not_select_receiver: bool,
    /// Does not select/modify information
    pub does_not_modify: bool,
    /// Observes due diligence
    pub due_diligence: bool,
    /// Complies with government directions
    pub complies_with_govt_directions: bool,
    /// Does not conspire/abet/aid
    pub does_not_aid: bool,
}

impl SafeHarborConditions {
    /// Check if eligible for safe harbor
    pub fn is_eligible(&self) -> bool {
        self.limited_function
            && self.does_not_initiate
            && self.does_not_select_receiver
            && self.does_not_modify
            && self.due_diligence
            && self.complies_with_govt_directions
            && self.does_not_aid
    }
}

/// E-commerce entity type (Consumer Protection E-Commerce Rules 2020)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EcommerceModel {
    /// Marketplace model
    Marketplace,
    /// Inventory model
    Inventory,
    /// Hybrid model
    Hybrid,
}

impl EcommerceModel {
    /// Get compliance requirements
    pub fn compliance_requirements(&self) -> Vec<&'static str> {
        match self {
            Self::Marketplace => vec![
                "Ensure seller details are displayed",
                "No false or misleading advertisements",
                "Transparent return/refund/exchange policy",
                "Grievance officer appointment",
                "Consumer complaint mechanism",
            ],
            Self::Inventory => vec![
                "Product origin country display",
                "MRP and delivery charges disclosure",
                "Warranty/guarantee details",
                "Return/refund policy",
                "Personal data protection",
            ],
            Self::Hybrid => vec![
                "All marketplace requirements",
                "All inventory requirements",
                "Clear demarcation of models",
            ],
        }
    }
}

/// Sensitive personal data (IT Rules 2011, Rule 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensitivePersonalData {
    /// Password
    Password,
    /// Financial information
    FinancialInfo,
    /// Physical/physiological/mental health condition
    HealthInfo,
    /// Sexual orientation
    SexualOrientation,
    /// Medical records
    MedicalRecords,
    /// Biometric information
    BiometricInfo,
}

impl SensitivePersonalData {
    /// Get rule reference
    pub fn rule_reference(&self) -> &'static str {
        "IT (Reasonable Security Practices) Rules, 2011, Rule 3"
    }

    /// Consent requirement
    pub fn consent_requirement(&self) -> ConsentRequirement {
        ConsentRequirement {
            written_consent: true,
            purpose_specification: true,
            opt_out_option: true,
            withdrawal_right: true,
        }
    }
}

/// Consent requirement for sensitive data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentRequirement {
    /// Written consent required
    pub written_consent: bool,
    /// Purpose must be specified
    pub purpose_specification: bool,
    /// Opt-out option required
    pub opt_out_option: bool,
    /// Right to withdraw consent
    pub withdrawal_right: bool,
}

/// Cyber Appellate Tribunal jurisdiction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CatJurisdiction {
    /// Civil matters under Section 43
    CivilSection43,
    /// Appeals against Adjudicating Officer orders
    AppealAdjudicatingOfficer,
    /// Appeals against Controller orders
    AppealController,
}

/// Adjudicating officer powers (Section 46)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdjudicatingOfficerPower {
    /// Hold inquiry
    HoldInquiry,
    /// Summon witnesses
    SummonWitnesses,
    /// Require document production
    RequireDocuments,
    /// Receive evidence on affidavit
    ReceiveAffidavit,
    /// Issue commission for witness examination
    IssueCommission,
}

/// Certifying Authority functions (Section 19)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaFunction {
    /// Issue digital signature certificates
    IssueCertificates,
    /// Revoke certificates
    RevokeCertificates,
    /// Suspend certificates
    SuspendCertificates,
    /// Publish certificate revocation list
    PublishCrl,
    /// Maintain records
    MaintainRecords,
}

/// Network service provider liability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkProviderLiability {
    /// Type of intermediary
    pub intermediary_type: IntermediaryType,
    /// Safe harbor conditions met
    pub safe_harbor: SafeHarborConditions,
    /// Has knowledge of illegal content
    pub has_knowledge: bool,
    /// Takedown request received
    pub takedown_received: bool,
    /// Takedown completed
    pub takedown_completed: bool,
    /// Time to act (hours)
    pub response_time_hours: Option<u32>,
}

impl NetworkProviderLiability {
    /// Check if provider is liable
    pub fn is_liable(&self) -> bool {
        if self.safe_harbor.is_eligible() {
            // Safe harbor available only if acted on knowledge/notice
            if self.has_knowledge || self.takedown_received {
                !self.takedown_completed
            } else {
                false
            }
        } else {
            // No safe harbor, may be liable
            true
        }
    }

    /// Check if within required response time (36 hours for unlawful content)
    pub fn is_within_response_time(&self) -> bool {
        match self.response_time_hours {
            Some(hours) => hours <= 36,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_certificate_validity() {
        let cert = DigitalCertificate {
            serial_number: "123456".to_string(),
            subject_name: "Test User".to_string(),
            issuer: "Test CA".to_string(),
            cert_type: DigitalSignatureType::Class2,
            issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            expiry_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            revoked: false,
            revocation_date: None,
        };

        let check_date = NaiveDate::from_ymd_opt(2025, 6, 1).expect("valid date");
        assert!(cert.is_valid(check_date));

        let expired_date = NaiveDate::from_ymd_opt(2027, 1, 1).expect("valid date");
        assert!(!cert.is_valid(expired_date));
    }

    #[test]
    fn test_cyber_crime_sections() {
        assert_eq!(CyberCrimeCategory::Hacking.section(), "Section 66");
        assert_eq!(CyberCrimeCategory::IdentityTheft.section(), "Section 66C");
        assert_eq!(CyberCrimeCategory::CyberTerrorism.section(), "Section 66F");
    }

    #[test]
    fn test_cyber_terrorism_punishment() {
        let punishment = CyberCrimeCategory::CyberTerrorism.punishment();
        assert!(punishment.imprisonment_max_years.is_none()); // Life imprisonment
        assert_eq!(punishment.bail_status, BailStatus::NonBailable);
    }

    #[test]
    fn test_computer_offence_clauses() {
        assert_eq!(
            ComputerOffence::UnauthorizedAccess.clause(),
            "Section 43(a)"
        );
        assert_eq!(
            ComputerOffence::IntroducingMalware.clause(),
            "Section 43(c)"
        );
    }

    #[test]
    fn test_safe_harbor_eligibility() {
        let eligible = SafeHarborConditions {
            limited_function: true,
            does_not_initiate: true,
            does_not_select_receiver: true,
            does_not_modify: true,
            due_diligence: true,
            complies_with_govt_directions: true,
            does_not_aid: true,
        };
        assert!(eligible.is_eligible());

        let not_eligible = SafeHarborConditions {
            due_diligence: false,
            ..eligible
        };
        assert!(!not_eligible.is_eligible());
    }

    #[test]
    fn test_intermediary_ssmi() {
        assert!(IntermediaryType::SocialMedia.is_ssmi_threshold_applicable());
        assert!(!IntermediaryType::Ecommerce.is_ssmi_threshold_applicable());
    }

    #[test]
    fn test_network_provider_liability() {
        let provider = NetworkProviderLiability {
            intermediary_type: IntermediaryType::SocialMedia,
            safe_harbor: SafeHarborConditions {
                limited_function: true,
                does_not_initiate: true,
                does_not_select_receiver: true,
                does_not_modify: true,
                due_diligence: true,
                complies_with_govt_directions: true,
                does_not_aid: true,
            },
            has_knowledge: true,
            takedown_received: true,
            takedown_completed: true,
            response_time_hours: Some(24),
        };
        assert!(!provider.is_liable());
        assert!(provider.is_within_response_time());
    }
}
