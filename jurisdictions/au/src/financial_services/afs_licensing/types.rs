//! AFS Licensing Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Australian Financial Services License
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AfslLicense {
    /// AFSL number (6-digit)
    pub license_number: String,
    /// Licensee name
    pub licensee_name: String,
    /// Australian Business Number
    pub abn: String,
    /// Australian Company Number (if applicable)
    pub acn: Option<String>,
    /// License status
    pub status: LicenseStatus,
    /// Date license was issued
    pub issue_date: NaiveDate,
    /// Date license was varied (if applicable)
    pub variation_date: Option<NaiveDate>,
    /// Authorized financial services
    pub authorized_services: Vec<AuthorizedService>,
    /// License conditions
    pub conditions: Vec<AfslCondition>,
    /// Responsible managers
    pub responsible_managers: Vec<ResponsibleManager>,
    /// Authorised representatives count
    pub authorised_rep_count: Option<u32>,
}

impl AfslLicense {
    /// Check if license is current
    pub fn is_current(&self) -> bool {
        matches!(self.status, LicenseStatus::Current)
    }

    /// Check if service is authorized
    pub fn is_service_authorized(&self, service: &AuthorizedService) -> bool {
        self.authorized_services.iter().any(|s| {
            // Check if service type matches (ignoring specific product/client variations)
            std::mem::discriminant(s) == std::mem::discriminant(service)
        })
    }

    /// Get conditions of specific type
    pub fn conditions_of_type(&self, condition_type: ConditionType) -> Vec<&AfslCondition> {
        self.conditions
            .iter()
            .filter(|c| c.condition_type == condition_type)
            .collect()
    }
}

/// AFSL status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    /// License is current and holder may conduct authorized activities
    Current,
    /// License is suspended - cannot conduct activities
    Suspended,
    /// License has been cancelled
    Cancelled,
    /// Application in progress
    ApplicationPending,
    /// License surrendered by holder
    Surrendered,
}

impl LicenseStatus {
    /// Check if licensee can conduct activities
    pub fn can_conduct_activities(&self) -> bool {
        matches!(self, LicenseStatus::Current)
    }
}

/// Authorized financial services under AFSL
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorizedService {
    /// Provide financial product advice (s.766B)
    ProvideFinancialProductAdvice {
        product_type: ProductType,
        client_type: ClientType,
    },
    /// Deal in a financial product (s.766C)
    DealInFinancialProduct {
        product_type: ProductType,
        deal_type: DealType,
        client_type: ClientType,
    },
    /// Make a market for a financial product (s.766D)
    MakeMarket { product_type: ProductType },
    /// Operate a registered managed investment scheme (s.766E)
    OperateRegisteredScheme,
    /// Provide custodial or depository service (s.766E)
    ProvideCustodialService { client_type: ClientType },
    /// Provide traditional trustee company services
    ProvideTrusteeServices,
    /// Arrange for a person to deal in a financial product
    ArrangeDeals {
        product_type: ProductType,
        client_type: ClientType,
    },
    /// Issue financial products
    IssueFinancialProducts { product_type: ProductType },
    /// Underwrite issues of financial products
    UnderwriteIssues { product_type: ProductType },
}

impl AuthorizedService {
    /// Get product type if applicable
    pub fn product_type(&self) -> Option<&ProductType> {
        match self {
            AuthorizedService::ProvideFinancialProductAdvice { product_type, .. }
            | AuthorizedService::DealInFinancialProduct { product_type, .. }
            | AuthorizedService::MakeMarket { product_type }
            | AuthorizedService::ArrangeDeals { product_type, .. }
            | AuthorizedService::IssueFinancialProducts { product_type }
            | AuthorizedService::UnderwriteIssues { product_type } => Some(product_type),
            _ => None,
        }
    }

    /// Get client type if applicable
    pub fn client_type(&self) -> Option<&ClientType> {
        match self {
            AuthorizedService::ProvideFinancialProductAdvice { client_type, .. }
            | AuthorizedService::DealInFinancialProduct { client_type, .. }
            | AuthorizedService::ProvideCustodialService { client_type }
            | AuthorizedService::ArrangeDeals { client_type, .. } => Some(client_type),
            _ => None,
        }
    }
}

/// Financial product types (s.764A)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductType {
    /// Securities (shares, debentures, etc.)
    Securities,
    /// Managed investment scheme interests
    ManagedInvestmentSchemeInterests,
    /// Derivatives
    Derivatives,
    /// Superannuation products
    SuperannuationProducts,
    /// Life insurance products
    LifeInsurance,
    /// General insurance products
    GeneralInsurance,
    /// Basic deposit products
    BasicDepositProducts,
    /// Non-basic deposit products
    NonBasicDepositProducts,
    /// Foreign exchange contracts
    ForeignExchange,
    /// Margin lending facilities
    MarginLending,
    /// Standard margin lending facilities
    StandardMarginLending,
    /// Retirement savings accounts
    RetirementSavingsAccounts,
    /// All financial products
    AllProducts,
}

/// Client types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientType {
    /// Retail clients only
    Retail,
    /// Wholesale clients only
    Wholesale,
    /// Both retail and wholesale
    Both,
}

impl ClientType {
    /// Check if includes retail clients
    pub fn includes_retail(&self) -> bool {
        matches!(self, ClientType::Retail | ClientType::Both)
    }

    /// Check if includes wholesale clients
    pub fn includes_wholesale(&self) -> bool {
        matches!(self, ClientType::Wholesale | ClientType::Both)
    }
}

/// Deal types for dealing services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DealType {
    /// Applying for, acquiring, or disposing (issuer-side)
    ApplyAcquireDispose,
    /// Issuing a financial product
    Issue,
    /// Deal on behalf of another person
    OnBehalfOfAnother,
}

/// AFSL condition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AfslCondition {
    /// Condition identifier
    pub condition_id: String,
    /// Condition type
    pub condition_type: ConditionType,
    /// Condition description
    pub description: String,
    /// Date condition imposed
    pub imposed_date: NaiveDate,
    /// Compliance status
    pub compliant: bool,
    /// Last compliance check date
    pub last_check_date: Option<NaiveDate>,
}

/// Types of license conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    /// Standard condition (applies to all licensees)
    Standard,
    /// Specific condition (tailored to licensee's authorization)
    Specific,
    /// Imposed condition (added by ASIC after licence grant)
    Imposed,
    /// Bespoke condition (individually negotiated)
    Bespoke,
}

/// Responsible manager
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsibleManager {
    /// Manager name
    pub name: String,
    /// Role/position
    pub position: String,
    /// Start date
    pub start_date: NaiveDate,
    /// Qualifications
    pub qualifications: Vec<String>,
    /// Years of experience
    pub experience_years: u32,
    /// Areas of responsibility
    pub responsibility_areas: Vec<String>,
    /// RG 105 compliant (fit and proper)
    pub fit_and_proper: bool,
}

/// Authorised representative
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedRepresentative {
    /// AR number
    pub ar_number: String,
    /// Representative name
    pub name: String,
    /// Is corporate AR
    pub is_corporate: bool,
    /// ABN (for corporate AR)
    pub abn: Option<String>,
    /// Principal AFSL number
    pub principal_afsl: String,
    /// Authorized services under this authorization
    pub authorized_services: Vec<AuthorizedService>,
    /// Authorization date
    pub authorization_date: NaiveDate,
    /// Status
    pub status: ArStatus,
    /// Training completed
    pub training_completed: bool,
    /// Meets RG 146 requirements
    pub rg146_compliant: bool,
}

impl AuthorizedRepresentative {
    /// Check if AR can provide service
    pub fn can_provide_service(&self, service: &AuthorizedService) -> bool {
        self.status == ArStatus::Active
            && self
                .authorized_services
                .iter()
                .any(|s| std::mem::discriminant(s) == std::mem::discriminant(service))
    }
}

/// Authorised representative status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArStatus {
    /// Active authorization
    Active,
    /// Authorization suspended
    Suspended,
    /// Authorization ceased
    Ceased,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_status() {
        assert!(LicenseStatus::Current.can_conduct_activities());
        assert!(!LicenseStatus::Suspended.can_conduct_activities());
        assert!(!LicenseStatus::Cancelled.can_conduct_activities());
    }

    #[test]
    fn test_client_type() {
        assert!(ClientType::Retail.includes_retail());
        assert!(!ClientType::Retail.includes_wholesale());

        assert!(!ClientType::Wholesale.includes_retail());
        assert!(ClientType::Wholesale.includes_wholesale());

        assert!(ClientType::Both.includes_retail());
        assert!(ClientType::Both.includes_wholesale());
    }

    #[test]
    fn test_authorized_service_product_type() {
        let service = AuthorizedService::ProvideFinancialProductAdvice {
            product_type: ProductType::Securities,
            client_type: ClientType::Retail,
        };

        assert_eq!(service.product_type(), Some(&ProductType::Securities));
        assert_eq!(service.client_type(), Some(&ClientType::Retail));
    }

    #[test]
    fn test_afsl_license_is_service_authorized() {
        let license = AfslLicense {
            license_number: "123456".to_string(),
            licensee_name: "Test Pty Ltd".to_string(),
            abn: "12345678901".to_string(),
            acn: None,
            status: LicenseStatus::Current,
            issue_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            variation_date: None,
            authorized_services: vec![AuthorizedService::ProvideFinancialProductAdvice {
                product_type: ProductType::Securities,
                client_type: ClientType::Retail,
            }],
            conditions: vec![],
            responsible_managers: vec![],
            authorised_rep_count: None,
        };

        let service = AuthorizedService::ProvideFinancialProductAdvice {
            product_type: ProductType::Securities,
            client_type: ClientType::Retail,
        };

        assert!(license.is_service_authorized(&service));
        assert!(license.is_current());
    }
}
