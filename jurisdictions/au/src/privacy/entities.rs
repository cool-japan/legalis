//! APP Entities (Privacy Act 1988)
//!
//! This module defines APP entities - organisations and agencies that must
//! comply with the Australian Privacy Principles.
//!
//! ## APP Entity Definition (s.6(1))
//!
//! An APP entity is either:
//! - An **agency** (Commonwealth government agency), or
//! - An **organisation** (non-government entity)
//!
//! ## Small Business Exemption (s.6D)
//!
//! Small businesses (annual turnover < $3 million) are generally exempt
//! unless they:
//! - Trade in personal information
//! - Are a contracted service provider
//! - Provide health services
//! - Collect personal information under Commonwealth law
//! - Are a credit reporting body
//! - Are prescribed by regulations
//!
//! ## Agency Exemptions
//!
//! Some agencies are exempt or have modified obligations:
//! - Intelligence agencies (ASIO, ASIS, ASD, AGO, DIO, ONI)
//! - State/territory government agencies
//! - Courts (in relation to judicial functions)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// APP Entity - organisation or agency subject to Privacy Act
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppEntity {
    /// Organisation (private sector)
    Organisation(Organisation),
    /// Agency (government)
    Agency(Agency),
}

impl AppEntity {
    /// Get entity name
    pub fn name(&self) -> &str {
        match self {
            AppEntity::Organisation(org) => &org.name,
            AppEntity::Agency(agency) => &agency.name,
        }
    }

    /// Get entity type
    pub fn entity_type(&self) -> EntityType {
        match self {
            AppEntity::Organisation(org) => org.entity_type,
            AppEntity::Agency(agency) => EntityType::Agency(agency.agency_type),
        }
    }

    /// Check if entity is covered by Privacy Act
    pub fn is_covered(&self) -> bool {
        match self {
            AppEntity::Organisation(org) => org.is_covered(),
            AppEntity::Agency(agency) => agency.is_covered(),
        }
    }

    /// Get exemption reason if not covered
    pub fn exemption_reason(&self) -> Option<String> {
        match self {
            AppEntity::Organisation(org) => org.exemption_reason(),
            AppEntity::Agency(agency) => agency.exemption_reason(),
        }
    }
}

/// Organisation (private sector entity)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organisation {
    /// Organisation name
    pub name: String,
    /// ABN
    pub abn: Option<String>,
    /// ACN
    pub acn: Option<String>,
    /// Entity type
    pub entity_type: EntityType,
    /// Annual turnover (AUD)
    pub annual_turnover_aud: Option<f64>,
    /// Small business exemption status
    pub small_business_exemption: Option<SmallBusinessExemption>,
    /// Industry sector
    pub industry_sector: IndustrySector,
    /// Whether entity is a health service provider
    pub is_health_service_provider: bool,
    /// Whether entity trades in personal information
    pub trades_in_personal_information: bool,
    /// Whether entity is contracted service provider to government
    pub is_contracted_service_provider: bool,
    /// Whether entity is credit reporting body
    pub is_credit_reporting_body: bool,
    /// Date privacy compliance started
    pub compliance_start_date: Option<DateTime<Utc>>,
}

impl Organisation {
    /// Create new organisation
    pub fn new(name: impl Into<String>, entity_type: EntityType) -> Self {
        Self {
            name: name.into(),
            abn: None,
            acn: None,
            entity_type,
            annual_turnover_aud: None,
            small_business_exemption: None,
            industry_sector: IndustrySector::Other,
            is_health_service_provider: false,
            trades_in_personal_information: false,
            is_contracted_service_provider: false,
            is_credit_reporting_body: false,
            compliance_start_date: None,
        }
    }

    /// Set ABN
    pub fn with_abn(mut self, abn: impl Into<String>) -> Self {
        self.abn = Some(abn.into());
        self
    }

    /// Set annual turnover
    pub fn with_turnover(mut self, turnover_aud: f64) -> Self {
        self.annual_turnover_aud = Some(turnover_aud);
        self
    }

    /// Set industry sector
    pub fn with_sector(mut self, sector: IndustrySector) -> Self {
        self.industry_sector = sector;
        self
    }

    /// Check if organisation is covered by Privacy Act
    pub fn is_covered(&self) -> bool {
        // Check small business exemption
        if let Some(ref exemption) = self.small_business_exemption
            && exemption.is_valid()
        {
            return false;
        }

        // Check if annual turnover >= $3M (automatically covered)
        if let Some(turnover) = self.annual_turnover_aud
            && turnover >= 3_000_000.0
        {
            return true;
        }

        // Small businesses are covered if they meet certain criteria
        self.is_health_service_provider
            || self.trades_in_personal_information
            || self.is_contracted_service_provider
            || self.is_credit_reporting_body
    }

    /// Get exemption reason if not covered
    pub fn exemption_reason(&self) -> Option<String> {
        if self.is_covered() {
            return None;
        }

        if let Some(turnover) = self.annual_turnover_aud
            && turnover < 3_000_000.0
        {
            return Some(format!(
                "Small business exemption (s.6D) - annual turnover ${:.0} < $3M",
                turnover
            ));
        }

        Some("Not an APP entity under s.6(1)".to_string())
    }

    /// Assess small business exemption
    pub fn assess_small_business_exemption(&mut self) {
        let turnover = self.annual_turnover_aud.unwrap_or(0.0);

        let qualifies = turnover < 3_000_000.0
            && !self.is_health_service_provider
            && !self.trades_in_personal_information
            && !self.is_contracted_service_provider
            && !self.is_credit_reporting_body;

        if qualifies {
            self.small_business_exemption = Some(SmallBusinessExemption {
                qualifies: true,
                annual_turnover_aud: turnover,
                exceptions_apply: Vec::new(),
                assessment_date: Utc::now(),
            });
        } else {
            let mut exceptions = Vec::new();
            if self.is_health_service_provider {
                exceptions.push(SmallBusinessException::HealthServiceProvider);
            }
            if self.trades_in_personal_information {
                exceptions.push(SmallBusinessException::TradesInPersonalInformation);
            }
            if self.is_contracted_service_provider {
                exceptions.push(SmallBusinessException::ContractedServiceProvider);
            }
            if self.is_credit_reporting_body {
                exceptions.push(SmallBusinessException::CreditReportingBody);
            }

            self.small_business_exemption = Some(SmallBusinessExemption {
                qualifies: false,
                annual_turnover_aud: turnover,
                exceptions_apply: exceptions,
                assessment_date: Utc::now(),
            });
        }
    }
}

/// Agency (government entity)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Agency {
    /// Agency name
    pub name: String,
    /// Agency type
    pub agency_type: AgencyType,
    /// Portfolio
    pub portfolio: Option<String>,
    /// Whether exempt from Privacy Act
    pub is_exempt: bool,
    /// Exemption reason
    pub exemption_type: Option<AgencyExemption>,
}

impl Agency {
    /// Create new agency
    pub fn new(name: impl Into<String>, agency_type: AgencyType) -> Self {
        let exempt_type = if agency_type.is_exempt() {
            Some(AgencyExemption::from_agency_type(agency_type))
        } else {
            None
        };

        Self {
            name: name.into(),
            agency_type,
            portfolio: None,
            is_exempt: agency_type.is_exempt(),
            exemption_type: exempt_type,
        }
    }

    /// Set portfolio
    pub fn with_portfolio(mut self, portfolio: impl Into<String>) -> Self {
        self.portfolio = Some(portfolio.into());
        self
    }

    /// Check if agency is covered
    pub fn is_covered(&self) -> bool {
        !self.is_exempt
    }

    /// Get exemption reason
    pub fn exemption_reason(&self) -> Option<String> {
        if !self.is_exempt {
            return None;
        }

        self.exemption_type
            .as_ref()
            .map(|e| e.description().to_string())
    }
}

/// Entity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    /// Company
    Company,
    /// Partnership
    Partnership,
    /// Trust
    Trust,
    /// Sole trader
    SoleTrader,
    /// Unincorporated association
    UnincorporatedAssociation,
    /// Commonwealth agency
    Agency(AgencyType),
}

/// Agency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyType {
    /// Commonwealth department
    Department,
    /// Statutory authority
    StatutoryAuthority,
    /// Government business enterprise
    GovernmentBusinessEnterprise,
    /// Intelligence agency (exempt)
    IntelligenceAgency,
    /// Norfolk Island authority
    NorfolkIsland,
    /// Court/tribunal (judicial function exempt)
    CourtTribunal,
}

impl AgencyType {
    /// Check if agency type is exempt
    pub fn is_exempt(&self) -> bool {
        matches!(
            self,
            AgencyType::IntelligenceAgency | AgencyType::CourtTribunal
        )
    }
}

/// Agency exemption type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyExemption {
    /// Intelligence agency (s.7(1)(a))
    IntelligenceAgency,
    /// Court/tribunal judicial function
    JudicialFunction,
    /// State/territory agency
    StateTerritory,
}

impl AgencyExemption {
    /// Get exemption description
    pub fn description(&self) -> &'static str {
        match self {
            AgencyExemption::IntelligenceAgency => "Exempt under s.7(1)(a) - intelligence agency",
            AgencyExemption::JudicialFunction => "Exempt in relation to judicial functions",
            AgencyExemption::StateTerritory => {
                "Not a Commonwealth agency - state/territory law applies"
            }
        }
    }

    /// Create from agency type
    fn from_agency_type(agency_type: AgencyType) -> Self {
        match agency_type {
            AgencyType::IntelligenceAgency => AgencyExemption::IntelligenceAgency,
            AgencyType::CourtTribunal => AgencyExemption::JudicialFunction,
            _ => AgencyExemption::StateTerritory, // Default for non-Commonwealth
        }
    }
}

/// Small business exemption assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmallBusinessExemption {
    /// Whether business qualifies for exemption
    pub qualifies: bool,
    /// Annual turnover
    pub annual_turnover_aud: f64,
    /// Exceptions that apply (removing exemption)
    pub exceptions_apply: Vec<SmallBusinessException>,
    /// Assessment date
    pub assessment_date: DateTime<Utc>,
}

impl SmallBusinessExemption {
    /// Check if exemption is valid
    pub fn is_valid(&self) -> bool {
        self.qualifies && self.exceptions_apply.is_empty()
    }
}

/// Small business exceptions (situations where exemption doesn't apply)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmallBusinessException {
    /// Health service provider (s.6D(4)(a))
    HealthServiceProvider,
    /// Trades in personal information (s.6D(4)(b))
    TradesInPersonalInformation,
    /// Contracted service provider (s.6D(4)(c))
    ContractedServiceProvider,
    /// Credit reporting body (s.6D(4)(e))
    CreditReportingBody,
    /// Collects TFNs (s.6D(4)(f))
    CollectsTfn,
    /// Prescribed by regulations (s.6D(4)(h))
    PrescribedByRegulations,
}

impl SmallBusinessException {
    /// Get statutory reference
    pub fn section(&self) -> &'static str {
        match self {
            SmallBusinessException::HealthServiceProvider => "s.6D(4)(a)",
            SmallBusinessException::TradesInPersonalInformation => "s.6D(4)(b)",
            SmallBusinessException::ContractedServiceProvider => "s.6D(4)(c)",
            SmallBusinessException::CreditReportingBody => "s.6D(4)(e)",
            SmallBusinessException::CollectsTfn => "s.6D(4)(f)",
            SmallBusinessException::PrescribedByRegulations => "s.6D(4)(h)",
        }
    }
}

/// Industry sector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndustrySector {
    /// Health
    Health,
    /// Financial services
    FinancialServices,
    /// Telecommunications
    Telecommunications,
    /// Education
    Education,
    /// Retail
    Retail,
    /// Technology
    Technology,
    /// Professional services
    ProfessionalServices,
    /// Media
    Media,
    /// Government contractor
    GovernmentContractor,
    /// Other
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organisation_coverage() {
        let org = Organisation::new("Big Corp", EntityType::Company).with_turnover(5_000_000.0);

        assert!(org.is_covered());
        assert!(org.exemption_reason().is_none());
    }

    #[test]
    fn test_small_business_exemption() {
        let org = Organisation::new("Small Shop", EntityType::SoleTrader).with_turnover(500_000.0);

        assert!(!org.is_covered());
        assert!(org.exemption_reason().is_some());
    }

    #[test]
    fn test_small_business_exception_health() {
        let mut org =
            Organisation::new("Small Clinic", EntityType::Company).with_turnover(500_000.0);
        org.is_health_service_provider = true;

        assert!(org.is_covered()); // Health providers are covered regardless of size
    }

    #[test]
    fn test_agency_exemption() {
        let intel = Agency::new("ASIO", AgencyType::IntelligenceAgency);
        assert!(!intel.is_covered());
        assert!(intel.exemption_reason().is_some());

        let dept = Agency::new("Department of Health", AgencyType::Department);
        assert!(dept.is_covered());
        assert!(dept.exemption_reason().is_none());
    }

    #[test]
    fn test_assess_small_business_exemption() {
        let mut org =
            Organisation::new("Tech Startup", EntityType::Company).with_turnover(1_500_000.0);

        org.assess_small_business_exemption();

        assert!(org.small_business_exemption.is_some());
        let exemption = org.small_business_exemption.as_ref().unwrap();
        assert!(exemption.qualifies);
        assert!(exemption.is_valid());
    }

    #[test]
    fn test_entity_type_agency() {
        let entity = AppEntity::Agency(Agency::new("Treasury", AgencyType::Department));

        match entity.entity_type() {
            EntityType::Agency(agency_type) => {
                assert_eq!(agency_type, AgencyType::Department);
            }
            _ => panic!("Wrong entity type"),
        }
    }

    #[test]
    fn test_small_business_exception_section() {
        assert_eq!(
            SmallBusinessException::HealthServiceProvider.section(),
            "s.6D(4)(a)"
        );
        assert_eq!(
            SmallBusinessException::TradesInPersonalInformation.section(),
            "s.6D(4)(b)"
        );
    }
}
