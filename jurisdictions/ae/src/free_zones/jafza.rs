//! Jebel Ali Free Zone Authority (JAFZA)
//!
//! JAFZA is one of the largest and oldest free zones in the UAE, located in Dubai.
//! Unlike DIFC and ADGM, JAFZA operates under **UAE Federal Law**, not Common Law.
//!
//! ## Key Features
//!
//! - 100% foreign ownership
//! - 100% repatriation of capital and profits
//! - No personal income tax
//! - 50-year corporate tax exemption (renewable)
//! - No import/export duties
//! - Streamlined business setup
//! - Multi-sector operations allowed
//!
//! ## Legal Framework
//!
//! - UAE Federal Commercial Companies Law (with free zone exemptions)
//! - UAE Federal Labour Law applies
//! - JAFZA specific regulations
//! - Dubai Courts jurisdiction (not independent courts)

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for JAFZA operations
pub type JafzaResult<T> = Result<T, JafzaError>;

/// JAFZA company types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JafzaCompanyType {
    /// Free Zone Establishment (FZE) - Single shareholder
    Fze,
    /// Free Zone Company (FZC) - Multiple shareholders (2-5)
    Fzc,
    /// Branch of foreign company
    ForeignBranch,
    /// Service company (for UAE nationals only)
    ServiceCompany,
}

impl JafzaCompanyType {
    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Fze => "Free Zone Establishment",
            Self::Fzc => "Free Zone Company",
            Self::ForeignBranch => "Branch of Foreign Company",
            Self::ServiceCompany => "Service Company",
        }
    }

    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Fze => "مؤسسة منطقة حرة",
            Self::Fzc => "شركة منطقة حرة",
            Self::ForeignBranch => "فرع شركة أجنبية",
            Self::ServiceCompany => "شركة خدمات",
        }
    }

    /// Get abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::Fze => "FZE",
            Self::Fzc => "FZC",
            Self::ForeignBranch => "Branch",
            Self::ServiceCompany => "Service Co.",
        }
    }

    /// Minimum share capital (AED)
    pub fn minimum_capital(&self) -> Option<Aed> {
        match self {
            Self::Fze | Self::Fzc => Some(Aed::from_dirhams(1_000)), // Nominal
            Self::ForeignBranch => None,
            Self::ServiceCompany => Some(Aed::from_dirhams(10_000)),
        }
    }

    /// Minimum number of shareholders
    pub fn minimum_shareholders(&self) -> u32 {
        match self {
            Self::Fze => 1,
            Self::Fzc => 2,
            Self::ForeignBranch => 0,
            Self::ServiceCompany => 1,
        }
    }

    /// Maximum number of shareholders
    pub fn maximum_shareholders(&self) -> Option<u32> {
        match self {
            Self::Fze => Some(1),
            Self::Fzc => Some(5),
            Self::ForeignBranch => None,
            Self::ServiceCompany => Some(1),
        }
    }

    /// Check if 100% foreign ownership is allowed
    pub fn allows_foreign_ownership(&self) -> bool {
        !matches!(self, Self::ServiceCompany)
    }
}

/// JAFZA business activities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JafzaActivity {
    /// Trading (import/export)
    Trading,
    /// Manufacturing
    Manufacturing,
    /// Logistics and Warehousing
    Logistics,
    /// Services (consulting, IT, etc.)
    Services,
    /// Industrial
    Industrial,
    /// E-commerce
    Ecommerce,
    /// Distribution
    Distribution,
}

impl JafzaActivity {
    /// Get activity name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Trading => "Trading",
            Self::Manufacturing => "Manufacturing",
            Self::Logistics => "Logistics and Warehousing",
            Self::Services => "Services",
            Self::Industrial => "Industrial",
            Self::Ecommerce => "E-commerce",
            Self::Distribution => "Distribution",
        }
    }

    /// Get activity name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Trading => "التجارة",
            Self::Manufacturing => "التصنيع",
            Self::Logistics => "اللوجستيات والتخزين",
            Self::Services => "الخدمات",
            Self::Industrial => "الصناعية",
            Self::Ecommerce => "التجارة الإلكترونية",
            Self::Distribution => "التوزيع",
        }
    }

    /// Check if activity requires physical premises
    pub fn requires_physical_premises(&self) -> bool {
        matches!(
            self,
            Self::Manufacturing | Self::Logistics | Self::Industrial
        )
    }
}

/// JAFZA license types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JafzaLicenseType {
    /// Commercial Trading License
    Commercial,
    /// Industrial License
    Industrial,
    /// Service License
    Service,
    /// General Trading License
    GeneralTrading,
    /// E-commerce License
    Ecommerce,
}

impl JafzaLicenseType {
    /// Get license name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Commercial => "Commercial Trading License",
            Self::Industrial => "Industrial License",
            Self::Service => "Service License",
            Self::GeneralTrading => "General Trading License",
            Self::Ecommerce => "E-commerce License",
        }
    }

    /// Get license name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Commercial => "رخصة تجارية",
            Self::Industrial => "رخصة صناعية",
            Self::Service => "رخصة خدمات",
            Self::GeneralTrading => "رخصة تجارة عامة",
            Self::Ecommerce => "رخصة تجارة إلكترونية",
        }
    }

    /// Get typical annual license cost (approximate)
    pub fn typical_annual_cost(&self) -> Aed {
        match self {
            Self::Commercial => Aed::from_dirhams(15_000),
            Self::Industrial => Aed::from_dirhams(25_000),
            Self::Service => Aed::from_dirhams(12_000),
            Self::GeneralTrading => Aed::from_dirhams(20_000),
            Self::Ecommerce => Aed::from_dirhams(10_000),
        }
    }
}

/// JAFZA company registration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JafzaRegistration {
    /// Company name (English)
    pub company_name_en: String,
    /// Company name (Arabic)
    pub company_name_ar: Option<String>,
    /// Company type
    pub company_type: JafzaCompanyType,
    /// Share capital
    pub share_capital: Aed,
    /// Number of shareholders
    pub shareholders: u32,
    /// License type
    pub license_type: JafzaLicenseType,
    /// Business activities
    pub activities: Vec<JafzaActivity>,
}

impl JafzaRegistration {
    /// Validate registration
    pub fn is_valid(&self) -> JafzaResult<()> {
        // Check shareholder count
        let min = self.company_type.minimum_shareholders();
        let max = self.company_type.maximum_shareholders();

        if self.shareholders < min {
            return Err(JafzaError::InvalidRegistration {
                reason: format!("Minimum {} shareholders required", min),
            });
        }

        if let Some(max_shareholders) = max
            && self.shareholders > max_shareholders
        {
            return Err(JafzaError::InvalidRegistration {
                reason: format!("Maximum {} shareholders allowed", max_shareholders),
            });
        }

        // Check capital
        if let Some(min_capital) = self.company_type.minimum_capital()
            && self.share_capital.fils() < min_capital.fils()
        {
            return Err(JafzaError::InvalidRegistration {
                reason: format!("Minimum capital {} required", min_capital.format_en()),
            });
        }

        // Check company name
        if self.company_name_en.is_empty() {
            return Err(JafzaError::InvalidRegistration {
                reason: "Company name is required".to_string(),
            });
        }

        // Check activities
        if self.activities.is_empty() {
            return Err(JafzaError::InvalidRegistration {
                reason: "At least one business activity required".to_string(),
            });
        }

        Ok(())
    }
}

/// JAFZA office space types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JafzaOfficeType {
    /// Flexi Desk (shared workspace)
    FlexiDesk,
    /// Business Center Office
    BusinessCenter,
    /// Warehouse
    Warehouse,
    /// Office Space (dedicated)
    Office,
    /// Industrial Plot
    IndustrialPlot,
    /// Virtual Office (no physical space)
    Virtual,
}

impl JafzaOfficeType {
    /// Get typical monthly cost (approximate, varies by size)
    pub fn typical_monthly_cost(&self) -> Aed {
        match self {
            Self::FlexiDesk => Aed::from_dirhams(1_500),
            Self::BusinessCenter => Aed::from_dirhams(3_500),
            Self::Warehouse => Aed::from_dirhams(8_000),
            Self::Office => Aed::from_dirhams(5_000),
            Self::IndustrialPlot => Aed::from_dirhams(15_000),
            Self::Virtual => Aed::from_dirhams(500),
        }
    }

    /// Get office type name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::FlexiDesk => "Flexi Desk",
            Self::BusinessCenter => "Business Center Office",
            Self::Warehouse => "Warehouse",
            Self::Office => "Dedicated Office Space",
            Self::IndustrialPlot => "Industrial Plot",
            Self::Virtual => "Virtual Office",
        }
    }
}

/// JAFZA errors
#[derive(Debug, Error)]
pub enum JafzaError {
    /// Invalid company registration
    #[error("Invalid JAFZA registration: {reason}")]
    InvalidRegistration { reason: String },

    /// License requirement not met
    #[error("License requirement not met: {requirement}")]
    LicenseRequirement { requirement: String },

    /// Activity restriction
    #[error("Activity not permitted: {activity}")]
    ActivityRestriction { activity: String },
}

/// Get JAFZA setup cost estimate
pub fn estimate_setup_cost(
    _company_type: &JafzaCompanyType,
    license_type: &JafzaLicenseType,
    office_type: &JafzaOfficeType,
    visa_count: u32,
) -> Aed {
    let mut total = Aed::from_fils(0);

    // License cost
    total = total + license_type.typical_annual_cost();

    // Office cost (annual)
    let office_annual = Aed::from_fils(office_type.typical_monthly_cost().fils() * 12);
    total = total + office_annual;

    // Registration fees (approximate)
    let registration = Aed::from_dirhams(10_000);
    total = total + registration;

    // Visa costs (approximate AED 3,000 per visa)
    let visa_costs = Aed::from_fils(3_000 * 100 * visa_count as i64);
    total = total + visa_costs;

    total
}

/// Get JAFZA benefits checklist
pub fn get_jafza_benefits() -> Vec<(&'static str, &'static str)> {
    vec![
        ("100% Foreign Ownership", "Full foreign ownership allowed"),
        (
            "Tax Exemption",
            "50-year corporate tax exemption (renewable)",
        ),
        ("No Customs Duties", "Zero import/export duties"),
        (
            "Capital Repatriation",
            "100% profit and capital repatriation",
        ),
        ("Strategic Location", "Close to Jebel Ali Port"),
        ("Multi-Sector", "Diverse business activities permitted"),
        ("Quick Setup", "Fast company formation process"),
        ("Visa Support", "Employment visa processing"),
        ("Infrastructure", "World-class facilities"),
        ("Business Support", "Value-added services available"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jafza_company_types() {
        let fze = JafzaCompanyType::Fze;
        assert_eq!(fze.abbreviation(), "FZE");
        assert_eq!(fze.minimum_shareholders(), 1);
        assert_eq!(fze.maximum_shareholders(), Some(1));
        assert!(fze.allows_foreign_ownership());

        let fzc = JafzaCompanyType::Fzc;
        assert_eq!(fzc.minimum_shareholders(), 2);
        assert_eq!(fzc.maximum_shareholders(), Some(5));
    }

    #[test]
    fn test_jafza_activities() {
        let trading = JafzaActivity::Trading;
        assert_eq!(trading.name_en(), "Trading");
        assert_eq!(trading.name_ar(), "التجارة");
        assert!(!trading.requires_physical_premises());

        let manufacturing = JafzaActivity::Manufacturing;
        assert!(manufacturing.requires_physical_premises());
    }

    #[test]
    fn test_jafza_license_types() {
        let commercial = JafzaLicenseType::Commercial;
        assert!(commercial.typical_annual_cost().dirhams() > 0);
        assert_eq!(commercial.name_ar(), "رخصة تجارية");
    }

    #[test]
    fn test_jafza_registration_valid() {
        let reg = JafzaRegistration {
            company_name_en: "Test Trading FZE".to_string(),
            company_name_ar: Some("شركة الاختبار للتجارة".to_string()),
            company_type: JafzaCompanyType::Fze,
            share_capital: Aed::from_dirhams(50_000),
            shareholders: 1,
            license_type: JafzaLicenseType::Commercial,
            activities: vec![JafzaActivity::Trading],
        };

        assert!(reg.is_valid().is_ok());
    }

    #[test]
    fn test_jafza_registration_invalid_shareholders() {
        let reg = JafzaRegistration {
            company_name_en: "Test FZE".to_string(),
            company_name_ar: None,
            company_type: JafzaCompanyType::Fze,
            share_capital: Aed::from_dirhams(10_000),
            shareholders: 3, // FZE can only have 1 shareholder
            license_type: JafzaLicenseType::Service,
            activities: vec![JafzaActivity::Services],
        };

        assert!(reg.is_valid().is_err());
    }

    #[test]
    fn test_jafza_registration_no_activities() {
        let reg = JafzaRegistration {
            company_name_en: "Test Company".to_string(),
            company_name_ar: None,
            company_type: JafzaCompanyType::Fzc,
            share_capital: Aed::from_dirhams(20_000),
            shareholders: 2,
            license_type: JafzaLicenseType::GeneralTrading,
            activities: vec![],
        };

        assert!(reg.is_valid().is_err());
    }

    #[test]
    fn test_jafza_office_types() {
        let virtual_office = JafzaOfficeType::Virtual;
        assert!(virtual_office.typical_monthly_cost().dirhams() < 1_000);

        let warehouse = JafzaOfficeType::Warehouse;
        assert!(warehouse.typical_monthly_cost().dirhams() > 5_000);
    }

    #[test]
    fn test_estimate_setup_cost() {
        let total = estimate_setup_cost(
            &JafzaCompanyType::Fze,
            &JafzaLicenseType::Commercial,
            &JafzaOfficeType::FlexiDesk,
            2, // 2 visas
        );

        assert!(total.dirhams() > 20_000);
    }

    #[test]
    fn test_jafza_benefits() {
        let benefits = get_jafza_benefits();
        assert!(!benefits.is_empty());
        assert!(benefits.len() >= 10);
    }

    #[test]
    fn test_service_company_restrictions() {
        let service_co = JafzaCompanyType::ServiceCompany;
        assert!(!service_co.allows_foreign_ownership());
    }
}
