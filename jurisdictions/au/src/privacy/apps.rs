//! Australian Privacy Principles (APPs) Analysis
//!
//! This module implements analysis of compliance with the 13 Australian
//! Privacy Principles under Schedule 1 of the Privacy Act 1988.

use serde::{Deserialize, Serialize};

use super::types::{Consent, PersonalInformation, PrivacyPolicy};

/// Australian Privacy Principle identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum App {
    /// APP 1 - Open and transparent management
    App1,
    /// APP 2 - Anonymity and pseudonymity
    App2,
    /// APP 3 - Collection of solicited personal information
    App3,
    /// APP 4 - Dealing with unsolicited personal information
    App4,
    /// APP 5 - Notification of collection
    App5,
    /// APP 6 - Use or disclosure
    App6,
    /// APP 7 - Direct marketing
    App7,
    /// APP 8 - Cross-border disclosure
    App8,
    /// APP 9 - Government identifiers
    App9,
    /// APP 10 - Quality of personal information
    App10,
    /// APP 11 - Security of personal information
    App11,
    /// APP 12 - Access to personal information
    App12,
    /// APP 13 - Correction of personal information
    App13,
}

impl App {
    /// Get APP number
    pub fn number(&self) -> u8 {
        match self {
            App::App1 => 1,
            App::App2 => 2,
            App::App3 => 3,
            App::App4 => 4,
            App::App5 => 5,
            App::App6 => 6,
            App::App7 => 7,
            App::App8 => 8,
            App::App9 => 9,
            App::App10 => 10,
            App::App11 => 11,
            App::App12 => 12,
            App::App13 => 13,
        }
    }

    /// Get APP title
    pub fn title(&self) -> &'static str {
        match self {
            App::App1 => "Open and transparent management of personal information",
            App::App2 => "Anonymity and pseudonymity",
            App::App3 => "Collection of solicited personal information",
            App::App4 => "Dealing with unsolicited personal information",
            App::App5 => "Notification of the collection of personal information",
            App::App6 => "Use or disclosure of personal information",
            App::App7 => "Direct marketing",
            App::App8 => "Cross-border disclosure of personal information",
            App::App9 => "Adoption, use or disclosure of government related identifiers",
            App::App10 => "Quality of personal information",
            App::App11 => "Security of personal information",
            App::App12 => "Access to personal information",
            App::App13 => "Correction of personal information",
        }
    }
}

/// APP compliance result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppCompliance {
    /// APP being assessed
    pub app: App,
    /// Compliance status
    pub compliant: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

impl AppCompliance {
    /// Create compliant result
    pub fn compliant(app: App) -> Self {
        Self {
            app,
            compliant: true,
            issues: Vec::new(),
            recommendations: Vec::new(),
            reasoning: format!("APP {} requirements met", app.number()),
        }
    }

    /// Create non-compliant result
    pub fn non_compliant(app: App, issues: Vec<String>) -> Self {
        Self {
            app,
            compliant: false,
            issues,
            recommendations: Vec::new(),
            reasoning: format!("APP {} requirements not met", app.number()),
        }
    }

    /// Add recommendation
    pub fn with_recommendation(mut self, rec: impl Into<String>) -> Self {
        self.recommendations.push(rec.into());
        self
    }
}

/// Collection analysis for APP 3
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionAnalysis {
    /// Information being collected
    pub information: PersonalInformation,
    /// Purpose of collection
    pub purpose: String,
    /// Whether collection is reasonably necessary
    pub reasonably_necessary: bool,
    /// Whether sensitive and consent obtained
    pub sensitive_consent_valid: bool,
    /// Collection method
    pub collection_method: CollectionMethod,
    /// Compliance result
    pub compliance: AppCompliance,
}

/// Method of collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectionMethod {
    /// Direct from individual
    Direct,
    /// From third party
    ThirdParty,
    /// Automated/technical means
    Automated,
    /// Public records
    PublicRecords,
}

/// Disclosure analysis for APP 6
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisclosureAnalysis {
    /// Information being disclosed
    pub information: PersonalInformation,
    /// Primary purpose
    pub primary_purpose: String,
    /// Actual purpose of disclosure
    pub disclosure_purpose: String,
    /// Whether for primary purpose
    pub is_primary_purpose: bool,
    /// Whether secondary purpose is related
    pub secondary_purpose_related: bool,
    /// Whether individual would reasonably expect
    pub reasonable_expectation: bool,
    /// Whether consent obtained
    pub consent_obtained: bool,
    /// Exception applies
    pub exception: Option<DisclosureException>,
    /// Compliance result
    pub compliance: AppCompliance,
}

/// Exceptions to use/disclosure restrictions (APP 6.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureException {
    /// Individual has consented (APP 6.2(a))
    Consent,
    /// Individual would reasonably expect (APP 6.2(a))
    ReasonableExpectation,
    /// Required by law (APP 6.2(b))
    RequiredByLaw,
    /// Enforcement body request (APP 6.2(c))
    EnforcementBody,
    /// Serious threat to health/safety (APP 6.2(c))
    SeriousThreat,
    /// Unlawful activity (APP 6.2(c))
    UnlawfulActivity,
    /// Legal claims (APP 6.2(c))
    LegalClaims,
}

/// Direct marketing analysis for APP 7
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectMarketingAnalysis {
    /// Whether direct marketing occurring
    pub is_direct_marketing: bool,
    /// Whether consent obtained
    pub consent_obtained: bool,
    /// Whether existing customer relationship
    pub existing_relationship: bool,
    /// Whether opt-out provided
    pub opt_out_provided: bool,
    /// Whether opt-out honoured
    pub opt_out_honoured: bool,
    /// Compliance result
    pub compliance: AppCompliance,
}

/// Cross-border disclosure analysis for APP 8
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossBorderAnalysis {
    /// Destination country
    pub destination_country: String,
    /// Recipient entity
    pub recipient: String,
    /// Whether recipient subject to similar law
    pub similar_law: bool,
    /// Whether binding scheme applies
    pub binding_scheme: bool,
    /// Whether consent obtained
    pub consent_obtained: bool,
    /// Whether required by law
    pub required_by_law: bool,
    /// Compliance result
    pub compliance: AppCompliance,
}

/// Security analysis for APP 11
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    /// Security measures in place
    pub security_measures: Vec<SecurityMeasure>,
    /// Gaps identified
    pub gaps: Vec<String>,
    /// Risk level
    pub risk_level: SecurityRiskLevel,
    /// Compliance result
    pub compliance: AppCompliance,
}

/// Security measure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityMeasure {
    /// Measure type
    pub measure_type: SecurityMeasureType,
    /// Description
    pub description: String,
    /// Implemented
    pub implemented: bool,
    /// Effective
    pub effective: bool,
}

/// Type of security measure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityMeasureType {
    /// Access controls
    AccessControls,
    /// Encryption
    Encryption,
    /// Network security
    NetworkSecurity,
    /// Physical security
    PhysicalSecurity,
    /// Staff training
    StaffTraining,
    /// Incident response
    IncidentResponse,
    /// Data retention/destruction
    DataRetention,
    /// Third party management
    ThirdPartyManagement,
}

/// Security risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityRiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// APP analyzer
pub struct AppAnalyzer;

impl AppAnalyzer {
    /// Analyze APP 1 compliance (privacy policy)
    pub fn analyze_app1(policy: &PrivacyPolicy) -> AppCompliance {
        let mut issues = Vec::new();

        if policy.information_types_collected.is_empty() {
            issues.push("Policy does not describe types of information collected".into());
        }
        if policy.collection_purposes.is_empty() {
            issues.push("Policy does not describe purposes of collection".into());
        }
        if policy.access_correction_process.is_empty() {
            issues.push("Policy does not describe access/correction process".into());
        }
        if policy.complaints_process.is_empty() {
            issues.push("Policy does not describe complaints process".into());
        }
        if policy.overseas_disclosure && policy.overseas_countries.is_empty() {
            issues.push("Overseas disclosure indicated but countries not specified".into());
        }

        if issues.is_empty() {
            AppCompliance::compliant(App::App1)
        } else {
            AppCompliance::non_compliant(App::App1, issues)
                .with_recommendation("Update privacy policy to address missing elements")
        }
    }

    /// Analyze APP 3 compliance (collection)
    pub fn analyze_app3(
        information: &PersonalInformation,
        purpose: impl Into<String>,
        reasonably_necessary: bool,
        consent: Option<&Consent>,
    ) -> CollectionAnalysis {
        let purpose = purpose.into();
        let is_sensitive = information.is_sensitive;

        let sensitive_consent_valid = if is_sensitive {
            consent
                .map(|c| c.is_currently_valid() && c.method.valid_for_sensitive())
                .unwrap_or(false)
        } else {
            true // Non-sensitive doesn't require express consent
        };

        let compliant = reasonably_necessary && (!is_sensitive || sensitive_consent_valid);

        let mut issues = Vec::new();
        if !reasonably_necessary {
            issues.push("Collection not reasonably necessary for purpose".into());
        }
        if is_sensitive && !sensitive_consent_valid {
            issues.push("Sensitive information collected without valid express consent".into());
        }

        let compliance = if compliant {
            AppCompliance::compliant(App::App3)
        } else {
            AppCompliance::non_compliant(App::App3, issues)
        };

        CollectionAnalysis {
            information: information.clone(),
            purpose,
            reasonably_necessary,
            sensitive_consent_valid,
            collection_method: CollectionMethod::Direct,
            compliance,
        }
    }

    /// Analyze APP 6 compliance (use/disclosure)
    pub fn analyze_app6(
        information: &PersonalInformation,
        primary_purpose: impl Into<String>,
        disclosure_purpose: impl Into<String>,
        consent: Option<&Consent>,
        exception: Option<DisclosureException>,
    ) -> DisclosureAnalysis {
        let primary = primary_purpose.into();
        let disclosure = disclosure_purpose.into();
        let is_primary = primary == disclosure;
        let consent_obtained = consent.map(|c| c.is_currently_valid()).unwrap_or(false);

        let compliant = is_primary || consent_obtained || exception.is_some();

        let mut issues = Vec::new();
        if !compliant {
            issues.push("Disclosure for secondary purpose without consent or exception".into());
        }

        let compliance = if compliant {
            AppCompliance::compliant(App::App6)
        } else {
            AppCompliance::non_compliant(App::App6, issues)
        };

        DisclosureAnalysis {
            information: information.clone(),
            primary_purpose: primary,
            disclosure_purpose: disclosure,
            is_primary_purpose: is_primary,
            secondary_purpose_related: false,
            reasonable_expectation: false,
            consent_obtained,
            exception,
            compliance,
        }
    }

    /// Analyze APP 7 compliance (direct marketing)
    pub fn analyze_app7(
        consent_obtained: bool,
        existing_relationship: bool,
        opt_out_provided: bool,
        sensitive_information: bool,
    ) -> DirectMarketingAnalysis {
        let mut issues = Vec::new();

        // Sensitive information requires express consent
        let sensitive_ok = !sensitive_information || consent_obtained;

        // Must either have consent, existing relationship with expectation, or opt-out
        let marketing_ok = consent_obtained || (existing_relationship && opt_out_provided);

        if !opt_out_provided {
            issues.push("Opt-out mechanism not provided".into());
        }
        if sensitive_information && !consent_obtained {
            issues.push("Direct marketing with sensitive information without consent".into());
        }
        if !marketing_ok {
            issues.push("Direct marketing without valid consent or relationship".into());
        }

        let compliant = sensitive_ok && marketing_ok && opt_out_provided;

        let compliance = if compliant {
            AppCompliance::compliant(App::App7)
        } else {
            AppCompliance::non_compliant(App::App7, issues)
        };

        DirectMarketingAnalysis {
            is_direct_marketing: true,
            consent_obtained,
            existing_relationship,
            opt_out_provided,
            opt_out_honoured: true, // Assume honoured unless indicated
            compliance,
        }
    }

    /// Analyze APP 8 compliance (cross-border)
    pub fn analyze_app8(
        country: impl Into<String>,
        recipient: impl Into<String>,
        similar_law: bool,
        consent: Option<&Consent>,
        required_by_law: bool,
    ) -> CrossBorderAnalysis {
        let country = country.into();
        let recipient = recipient.into();
        let consent_obtained = consent.map(|c| c.is_currently_valid()).unwrap_or(false);

        // Compliant if: similar law, consent, or required by law
        let compliant = similar_law || consent_obtained || required_by_law;

        let mut issues = Vec::new();
        if !compliant {
            issues.push(format!(
                "Cross-border disclosure to {} without adequate protection or consent",
                country
            ));
        }

        let compliance = if compliant {
            AppCompliance::compliant(App::App8)
        } else {
            AppCompliance::non_compliant(App::App8, issues)
                .with_recommendation("Obtain consent or ensure recipient is bound by similar law")
        };

        CrossBorderAnalysis {
            destination_country: country,
            recipient,
            similar_law,
            binding_scheme: false,
            consent_obtained,
            required_by_law,
            compliance,
        }
    }

    /// Analyze APP 11 compliance (security)
    pub fn analyze_app11(measures: Vec<SecurityMeasure>) -> SecurityAnalysis {
        let mut gaps = Vec::new();

        // Check essential measures
        let has_access_controls = measures
            .iter()
            .any(|m| m.measure_type == SecurityMeasureType::AccessControls && m.implemented);
        let has_encryption = measures
            .iter()
            .any(|m| m.measure_type == SecurityMeasureType::Encryption && m.implemented);
        let has_incident_response = measures
            .iter()
            .any(|m| m.measure_type == SecurityMeasureType::IncidentResponse && m.implemented);

        if !has_access_controls {
            gaps.push("No access controls implemented".into());
        }
        if !has_encryption {
            gaps.push("No encryption implemented".into());
        }
        if !has_incident_response {
            gaps.push("No incident response plan".into());
        }

        let risk_level = match gaps.len() {
            0 => SecurityRiskLevel::Low,
            1 => SecurityRiskLevel::Medium,
            2 => SecurityRiskLevel::High,
            _ => SecurityRiskLevel::Critical,
        };

        let compliant = gaps.is_empty();

        let compliance = if compliant {
            AppCompliance::compliant(App::App11)
        } else {
            AppCompliance::non_compliant(App::App11, gaps.clone())
                .with_recommendation("Implement missing security measures")
        };

        SecurityAnalysis {
            security_measures: measures,
            gaps,
            risk_level,
            compliance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{ConsentMethod, PersonalInformationType};
    use super::*;

    #[test]
    fn test_app_numbers() {
        assert_eq!(App::App1.number(), 1);
        assert_eq!(App::App13.number(), 13);
    }

    #[test]
    fn test_app1_compliance() {
        let mut policy = PrivacyPolicy::new("Test Corp", "1.0");
        policy
            .information_types_collected
            .push(PersonalInformationType::Name);
        policy.collection_purposes.push("Service delivery".into());
        policy.access_correction_process = "Contact us".into();
        policy.complaints_process = "Lodge complaint".into();

        let result = AppAnalyzer::analyze_app1(&policy);
        assert!(result.compliant);
    }

    #[test]
    fn test_app1_non_compliance() {
        let policy = PrivacyPolicy::new("Test Corp", "1.0");

        let result = AppAnalyzer::analyze_app1(&policy);
        assert!(!result.compliant);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_app3_sensitive_without_consent() {
        let info = PersonalInformation::new("rec-001", PersonalInformationType::Health, "user-123");

        let result = AppAnalyzer::analyze_app3(&info, "Medical treatment", true, None);

        assert!(!result.compliance.compliant);
        assert!(!result.sensitive_consent_valid);
    }

    #[test]
    fn test_app3_sensitive_with_consent() {
        let info = PersonalInformation::new("rec-001", PersonalInformationType::Health, "user-123");
        let consent = Consent::new(
            "consent-001",
            "user-123",
            super::super::types::ConsentPurpose::SensitiveCollection,
            ConsentMethod::ExpressWritten,
            "I consent",
        );

        let result = AppAnalyzer::analyze_app3(&info, "Medical treatment", true, Some(&consent));

        assert!(result.compliance.compliant);
    }

    #[test]
    fn test_app6_primary_purpose() {
        let info = PersonalInformation::new("rec-001", PersonalInformationType::Name, "user-123");

        let result =
            AppAnalyzer::analyze_app6(&info, "Customer service", "Customer service", None, None);

        assert!(result.compliance.compliant);
        assert!(result.is_primary_purpose);
    }

    #[test]
    fn test_app6_secondary_purpose_without_consent() {
        let info = PersonalInformation::new("rec-001", PersonalInformationType::Name, "user-123");

        let result = AppAnalyzer::analyze_app6(&info, "Customer service", "Marketing", None, None);

        assert!(!result.compliance.compliant);
    }

    #[test]
    fn test_app7_direct_marketing() {
        let result = AppAnalyzer::analyze_app7(true, false, true, false);
        assert!(result.compliance.compliant);

        let result = AppAnalyzer::analyze_app7(false, false, false, false);
        assert!(!result.compliance.compliant);
    }

    #[test]
    fn test_app11_security() {
        let measures = vec![
            SecurityMeasure {
                measure_type: SecurityMeasureType::AccessControls,
                description: "RBAC".into(),
                implemented: true,
                effective: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::Encryption,
                description: "AES-256".into(),
                implemented: true,
                effective: true,
            },
            SecurityMeasure {
                measure_type: SecurityMeasureType::IncidentResponse,
                description: "IR plan".into(),
                implemented: true,
                effective: true,
            },
        ];

        let result = AppAnalyzer::analyze_app11(measures);

        assert!(result.compliance.compliant);
        assert!(result.gaps.is_empty());
        assert_eq!(result.risk_level, SecurityRiskLevel::Low);
    }
}
