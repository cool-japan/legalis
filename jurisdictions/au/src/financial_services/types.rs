//! Financial Services Types (Corporations Act 2001 Chapter 7)
//!
//! This module provides types for Australian financial services regulation under
//! Chapter 7 of the Corporations Act 2001 and ASIC regulations.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Financial services provider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialServicesProvider {
    /// Entity name
    pub name: String,
    /// Australian Business Number (ABN)
    pub abn: String,
    /// Australian Company Number (ACN) if applicable
    pub acn: Option<String>,
    /// AFSL number if licensed
    pub afsl_number: Option<String>,
    /// Authorised representative number if AR
    pub ar_number: Option<String>,
    /// Provider type
    pub provider_type: ProviderType,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
}

/// Provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    /// AFSL holder
    AfslHolder,
    /// Authorised representative
    AuthorisedRepresentative,
    /// Corporate authorised representative
    CorporateAuthorisedRepresentative,
    /// Exempt provider
    ExemptProvider,
}

/// Client classification (Wholesale vs Retail)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientClassification {
    /// Client name
    pub client_name: String,
    /// Classification result
    pub classification: ClientClass,
    /// Classification basis
    pub basis: ClassificationBasis,
    /// Classification date
    pub classification_date: NaiveDate,
    /// Expiry date (if applicable)
    pub expiry_date: Option<NaiveDate>,
    /// Supporting documentation
    pub documentation: Vec<String>,
}

/// Client classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientClass {
    /// Retail client (highest protection)
    Retail,
    /// Wholesale client (reduced protection)
    Wholesale,
}

impl ClientClass {
    /// Check if PDS required for this client class
    pub fn requires_pds(&self) -> bool {
        matches!(self, ClientClass::Retail)
    }

    /// Check if FSG required
    pub fn requires_fsg(&self) -> bool {
        matches!(self, ClientClass::Retail)
    }

    /// Check if SOA required for personal advice
    pub fn requires_soa(&self) -> bool {
        matches!(self, ClientClass::Retail)
    }

    /// Check if best interests duty applies
    pub fn best_interests_duty_applies(&self) -> bool {
        matches!(self, ClientClass::Retail)
    }

    /// Check if access to external dispute resolution
    pub fn has_edr_access(&self) -> bool {
        matches!(self, ClientClass::Retail)
    }
}

/// Basis for wholesale client classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClassificationBasis {
    /// Product value test - $500,000+ consideration (s.761G(7)(a))
    ProductValueTest { consideration_aud: f64 },
    /// Assets test - net assets $2.5M+ (s.761G(7)(c))
    AssetsTest {
        net_assets_aud: f64,
        accountant_certificate: bool,
    },
    /// Income test - gross income $250,000+ for last 2 years (s.761G(7)(c))
    IncomeTest {
        income_year_1_aud: f64,
        income_year_2_aud: f64,
        accountant_certificate: bool,
    },
    /// Professional investor (s.708(11))
    ProfessionalInvestor { certificate_date: NaiveDate },
    /// Controlling entity of body corporate (s.761G(7)(b))
    ControllingEntity { controlled_entity: String },
    /// AFSL holder
    AfslHolder { afsl_number: String },
    /// Listed entity or subsidiary (s.761G(7)(d))
    ListedEntity {
        exchange: String,
        asx_code: Option<String>,
    },
    /// Regulated superannuation fund >$10M (s.761G(7)(e))
    RegulatedSuperFund {
        fund_name: String,
        net_assets_aud: f64,
    },
    /// Government body
    GovernmentBody { body_type: String },
    /// Body regulated by APRA (s.761G(7)(f))
    ApraRegulatedBody,
    /// Retail client (default)
    RetailDefault,
}

/// General obligations compliance (s.912A)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralObligationsCompliance {
    /// Efficient, honest and fair (s.912A(1)(a))
    pub efficient_honest_fair: ObligationCompliance,
    /// Conflicts management (s.912A(1)(aa))
    pub conflicts_management: ObligationCompliance,
    /// Compliance with financial services laws (s.912A(1)(c))
    pub legal_compliance: ObligationCompliance,
    /// Representative compliance (s.912A(1)(ca))
    pub representative_compliance: ObligationCompliance,
    /// Risk management systems (s.912A(1)(h))
    pub risk_management: ObligationCompliance,
    /// Competence (s.912A(1)(e))
    pub competence: ObligationCompliance,
    /// Representative training (s.912A(1)(f))
    pub representative_training: ObligationCompliance,
    /// Adequate resources (s.912A(1)(d))
    pub adequate_resources: ObligationCompliance,
    /// Dispute resolution (s.912A(1)(g))
    pub dispute_resolution: ObligationCompliance,
    /// Compensation arrangements (s.912B)
    pub compensation_arrangements: ObligationCompliance,
}

impl GeneralObligationsCompliance {
    /// Check if all obligations are compliant
    pub fn is_fully_compliant(&self) -> bool {
        self.efficient_honest_fair.compliant
            && self.conflicts_management.compliant
            && self.legal_compliance.compliant
            && self.representative_compliance.compliant
            && self.risk_management.compliant
            && self.competence.compliant
            && self.representative_training.compliant
            && self.adequate_resources.compliant
            && self.dispute_resolution.compliant
            && self.compensation_arrangements.compliant
    }

    /// Get list of non-compliant obligations
    pub fn non_compliant_obligations(&self) -> Vec<&str> {
        let mut issues = Vec::new();
        if !self.efficient_honest_fair.compliant {
            issues.push("Efficient, honest and fair (s.912A(1)(a))");
        }
        if !self.conflicts_management.compliant {
            issues.push("Conflicts management (s.912A(1)(aa))");
        }
        if !self.legal_compliance.compliant {
            issues.push("Legal compliance (s.912A(1)(c))");
        }
        if !self.representative_compliance.compliant {
            issues.push("Representative compliance (s.912A(1)(ca))");
        }
        if !self.risk_management.compliant {
            issues.push("Risk management (s.912A(1)(h))");
        }
        if !self.competence.compliant {
            issues.push("Competence (s.912A(1)(e))");
        }
        if !self.representative_training.compliant {
            issues.push("Representative training (s.912A(1)(f))");
        }
        if !self.adequate_resources.compliant {
            issues.push("Adequate resources (s.912A(1)(d))");
        }
        if !self.dispute_resolution.compliant {
            issues.push("Dispute resolution (s.912A(1)(g))");
        }
        if !self.compensation_arrangements.compliant {
            issues.push("Compensation arrangements (s.912B)");
        }
        issues
    }
}

/// Individual obligation compliance
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObligationCompliance {
    /// Whether compliant
    pub compliant: bool,
    /// Evidence of compliance
    pub evidence: String,
    /// Details of any breach
    pub breach_details: Option<String>,
    /// Last review date
    pub last_review_date: Option<NaiveDate>,
}

/// ASIC obligation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsicObligation {
    /// Efficient, honest and fair (s.912A(1)(a))
    EfficientHonestFair,
    /// Conflicts management (s.912A(1)(aa))
    ConflictsManagement,
    /// Legal compliance (s.912A(1)(c))
    LegalCompliance,
    /// Representative compliance (s.912A(1)(ca))
    RepresentativeCompliance,
    /// Risk management (s.912A(1)(h))
    RiskManagement,
    /// Competence (s.912A(1)(e))
    Competence,
    /// Representative training (s.912A(1)(f))
    RepresentativeTraining,
    /// Adequate resources (s.912A(1)(d))
    AdequateResources,
    /// Dispute resolution (s.912A(1)(g))
    DisputeResolution,
    /// Compensation arrangements (s.912B)
    CompensationArrangements,
}

impl AsicObligation {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            AsicObligation::EfficientHonestFair => "s.912A(1)(a)",
            AsicObligation::ConflictsManagement => "s.912A(1)(aa)",
            AsicObligation::LegalCompliance => "s.912A(1)(c)",
            AsicObligation::RepresentativeCompliance => "s.912A(1)(ca)",
            AsicObligation::RiskManagement => "s.912A(1)(h)",
            AsicObligation::Competence => "s.912A(1)(e)",
            AsicObligation::RepresentativeTraining => "s.912A(1)(f)",
            AsicObligation::AdequateResources => "s.912A(1)(d)",
            AsicObligation::DisputeResolution => "s.912A(1)(g)",
            AsicObligation::CompensationArrangements => "s.912B",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            AsicObligation::EfficientHonestFair => {
                "Do all things necessary to ensure financial services are provided efficiently, \
                 honestly and fairly"
            }
            AsicObligation::ConflictsManagement => {
                "Have adequate arrangements for managing conflicts of interest"
            }
            AsicObligation::LegalCompliance => "Comply with financial services laws",
            AsicObligation::RepresentativeCompliance => {
                "Take reasonable steps to ensure representatives comply with financial services laws"
            }
            AsicObligation::RiskManagement => "Have adequate risk management systems",
            AsicObligation::Competence => "Maintain competence to provide the financial services",
            AsicObligation::RepresentativeTraining => {
                "Ensure representatives are adequately trained and competent"
            }
            AsicObligation::AdequateResources => {
                "Have adequate resources (financial, technological, human)"
            }
            AsicObligation::DisputeResolution => "Have a dispute resolution system",
            AsicObligation::CompensationArrangements => "Have adequate compensation arrangements",
        }
    }
}

/// Resource requirements (s.912A(1)(d))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirement {
    /// Financial resources
    pub financial: FinancialResources,
    /// Human resources
    pub human: HumanResources,
    /// Technological resources
    pub technological: TechnologicalResources,
}

/// Financial resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialResources {
    /// Net tangible assets (AUD)
    pub net_tangible_assets_aud: f64,
    /// Required NTA (varies by activity)
    pub required_nta_aud: f64,
    /// Cash reserves (AUD)
    pub cash_reserves_aud: f64,
    /// Professional indemnity insurance
    pub pi_insurance_aud: Option<f64>,
    /// Surplus assets
    pub surplus_assets_aud: f64,
    /// Meets requirements
    pub meets_requirements: bool,
}

/// Human resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanResources {
    /// Number of responsible managers
    pub responsible_managers: u32,
    /// Number of authorised representatives
    pub authorised_representatives: u32,
    /// Number of compliance staff
    pub compliance_staff: u32,
    /// Training program in place
    pub training_program: bool,
    /// Competency standards met
    pub competency_standards_met: bool,
}

/// Technological resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechnologicalResources {
    /// Record keeping systems
    pub record_keeping_systems: bool,
    /// Transaction monitoring
    pub transaction_monitoring: bool,
    /// Security controls
    pub security_controls: bool,
    /// Disaster recovery
    pub disaster_recovery: bool,
    /// Privacy compliance
    pub privacy_compliance: bool,
}

/// Dispute resolution (s.912A(1)(g))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisputeResolution {
    /// Internal dispute resolution (IDR) system
    pub idr_system: InternalDisputeResolution,
    /// External dispute resolution (EDR) membership
    pub edr_membership: Option<ExternalDisputeResolution>,
}

/// Internal dispute resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalDisputeResolution {
    /// Complies with RG 271
    pub rg271_compliant: bool,
    /// Maximum response time (days)
    pub response_time_days: u32,
    /// Maximum resolution time (days)
    pub resolution_time_days: u32,
    /// Dedicated complaints officer
    pub complaints_officer: bool,
    /// Written procedures
    pub written_procedures: bool,
}

/// External dispute resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalDisputeResolution {
    /// AFCA membership number
    pub afca_member_number: String,
    /// AFCA membership date
    pub membership_date: NaiveDate,
    /// Current status
    pub status: EdrStatus,
}

/// EDR status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdrStatus {
    /// Active membership
    Active,
    /// Suspended
    Suspended,
    /// Terminated
    Terminated,
}

/// Compensation arrangements (s.912B)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompensationArrangement {
    /// Has adequate arrangements
    pub adequate: bool,
    /// Professional indemnity insurance
    pub pi_insurance: Option<ProfessionalIndemnityInsurance>,
    /// Alternative arrangements
    pub alternative_arrangements: Option<String>,
    /// Last review date
    pub last_review_date: NaiveDate,
}

/// Professional indemnity insurance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfessionalIndemnityInsurance {
    /// Insurer name
    pub insurer: String,
    /// Policy number
    pub policy_number: String,
    /// Coverage amount (AUD)
    pub coverage_aud: f64,
    /// Excess (AUD)
    pub excess_aud: f64,
    /// Policy start date
    pub start_date: NaiveDate,
    /// Policy end date
    pub end_date: NaiveDate,
    /// Covers all authorized services
    pub covers_all_services: bool,
    /// Run-off cover
    pub run_off_cover: bool,
}

/// Breach notification (s.912D-912E)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachNotification {
    /// Breach ID
    pub breach_id: String,
    /// Date breach identified
    pub identification_date: DateTime<Utc>,
    /// Date breach occurred
    pub breach_date: Option<NaiveDate>,
    /// Breach type
    pub breach_type: BreachType,
    /// Breach significance
    pub significance: BreachSignificance,
    /// Description
    pub description: String,
    /// Affected clients
    pub affected_clients: Option<u32>,
    /// Financial loss (AUD)
    pub financial_loss_aud: Option<f64>,
    /// Remediation actions
    pub remediation: Vec<String>,
    /// Reported to ASIC
    pub reported_to_asic: bool,
    /// ASIC notification date
    pub asic_notification_date: Option<DateTime<Utc>>,
    /// Remediation complete
    pub remediation_complete: bool,
}

/// Breach types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Compliance breach
    ComplianceBreach,
    /// Conduct breach
    ConductBreach,
    /// Disclosure breach
    DisclosureBreach,
    /// Licensing breach
    LicensingBreach,
    /// System breach
    SystemBreach,
    /// Supervision failure
    SupervisionFailure,
}

/// Breach significance (s.912D(1))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachSignificance {
    /// Significant breach - must be reported
    Significant,
    /// Not significant - internal record only
    NotSignificant,
    /// Investigation ongoing
    UnderInvestigation,
}

impl BreachSignificance {
    /// Check if ASIC notification required
    pub fn requires_asic_notification(&self) -> bool {
        matches!(self, BreachSignificance::Significant)
    }

    /// Get notification timeframe (days)
    pub fn notification_timeframe_days(&self) -> Option<u32> {
        match self {
            BreachSignificance::Significant => Some(30),
            _ => None,
        }
    }
}

/// Market integrity rules compliance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketIntegrityCompliance {
    /// Compliance with ASIC market integrity rules
    pub mir_compliant: bool,
    /// Trading systems
    pub trading_systems_adequate: bool,
    /// Best execution policy
    pub best_execution_policy: bool,
    /// Order handling rules
    pub order_handling_compliant: bool,
    /// Market manipulation prevention
    pub manipulation_prevention: bool,
    /// Insider trading prevention
    pub insider_trading_prevention: bool,
}

/// ASIC enforcement action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAction {
    /// License condition
    LicenseCondition,
    /// License suspension
    LicenseSuspension,
    /// License cancellation
    LicenseCancellation,
    /// Banning order
    BanningOrder,
    /// Infringement notice
    InfringementNotice,
    /// Civil penalty proceedings
    CivilPenalty,
    /// Criminal prosecution
    CriminalProsecution,
    /// Enforceable undertaking
    EnforceableUndertaking,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_class_requirements() {
        let retail = ClientClass::Retail;
        assert!(retail.requires_pds());
        assert!(retail.requires_fsg());
        assert!(retail.requires_soa());
        assert!(retail.best_interests_duty_applies());
        assert!(retail.has_edr_access());

        let wholesale = ClientClass::Wholesale;
        assert!(!wholesale.requires_pds());
        assert!(!wholesale.requires_fsg());
        assert!(!wholesale.requires_soa());
        assert!(!wholesale.best_interests_duty_applies());
        assert!(!wholesale.has_edr_access());
    }

    #[test]
    fn test_asic_obligation_sections() {
        assert_eq!(
            AsicObligation::EfficientHonestFair.section(),
            "s.912A(1)(a)"
        );
        assert_eq!(
            AsicObligation::ConflictsManagement.section(),
            "s.912A(1)(aa)"
        );
        assert_eq!(AsicObligation::CompensationArrangements.section(), "s.912B");
    }

    #[test]
    fn test_breach_significance_notification() {
        let significant = BreachSignificance::Significant;
        assert!(significant.requires_asic_notification());
        assert_eq!(significant.notification_timeframe_days(), Some(30));

        let not_significant = BreachSignificance::NotSignificant;
        assert!(!not_significant.requires_asic_notification());
        assert_eq!(not_significant.notification_timeframe_days(), None);
    }

    #[test]
    fn test_general_obligations_compliance() {
        let compliant_ob = ObligationCompliance {
            compliant: true,
            evidence: "Documented procedures".to_string(),
            breach_details: None,
            last_review_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        };

        let non_compliant_ob = ObligationCompliance {
            compliant: false,
            evidence: String::new(),
            breach_details: Some("Training not completed".to_string()),
            last_review_date: None,
        };

        let compliance = GeneralObligationsCompliance {
            efficient_honest_fair: compliant_ob.clone(),
            conflicts_management: compliant_ob.clone(),
            legal_compliance: compliant_ob.clone(),
            representative_compliance: compliant_ob.clone(),
            risk_management: compliant_ob.clone(),
            competence: compliant_ob.clone(),
            representative_training: non_compliant_ob, // Not compliant
            adequate_resources: compliant_ob.clone(),
            dispute_resolution: compliant_ob.clone(),
            compensation_arrangements: compliant_ob,
        };

        assert!(!compliance.is_fully_compliant());
        let issues = compliance.non_compliant_obligations();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].contains("training"));
    }
}
