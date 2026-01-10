//! Construction and Real Estate Types
//!
//! Core data structures for Construction Business Act (建設業法) and
//! Real Estate Transactions Act (宅地建物取引業法).

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Construction license type (建設業法第3条)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstructionLicenseType {
    /// General construction business license (一般建設業許可)
    General,
    /// Special construction business license (特定建設業許可)
    Special,
}

impl ConstructionLicenseType {
    /// Get minimum capital requirement in JPY
    pub fn minimum_capital(&self) -> u64 {
        match self {
            Self::General => 5_000_000,  // ¥5M for general
            Self::Special => 20_000_000, // ¥20M for special (Article 7-1)
        }
    }

    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::General => "一般建設業許可",
            Self::Special => "特定建設業許可",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::General => "General Construction License",
            Self::Special => "Special Construction License",
        }
    }
}

/// Construction type (29 types under Article 2-1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstructionType {
    /// Civil engineering (土木工事業)
    Civil,
    /// Architecture (建築工事業)
    Architecture,
    /// Carpentry (大工工事業)
    Carpentry,
    /// Plumbing and heating (管工事業)
    PlumbingHeating,
    /// Electrical (電気工事業)
    Electrical,
    /// Other construction type
    Other(u8), // Simplified - represents other 24 types
}

/// Manager qualification (建設業法第8条)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ManagerQualification {
    /// First-class architect (一級建築士)
    FirstClassArchitect,
    /// Second-class architect (二級建築士)
    SecondClassArchitect,
    /// Civil engineer (土木施工管理技士)
    CivilEngineer,
    /// Construction manager (建設業経営管理責任者)
    ConstructionManager,
    /// Other qualification
    Other(String),
}

/// Construction business manager
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Manager {
    /// Manager name
    pub name: String,
    /// Qualification
    pub qualification: ManagerQualification,
    /// Certification number
    pub certification_number: String,
    /// Certification date
    pub certification_date: NaiveDate,
}

/// Construction business license
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstructionBusinessLicense {
    /// License number
    pub license_number: String,
    /// Business name
    pub business_name: String,
    /// License type
    pub license_type: ConstructionLicenseType,
    /// Construction types permitted
    pub construction_types: Vec<ConstructionType>,
    /// Registered capital in JPY
    pub registered_capital_jpy: u64,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Expiration date (5 years - Article 3-3)
    pub expiration_date: NaiveDate,
    /// Managers
    pub managers: Vec<Manager>,
}

impl ConstructionBusinessLicense {
    /// Check if license is currently valid
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().date_naive();
        now >= self.issue_date && now <= self.expiration_date
    }

    /// Check if capital requirement is met
    pub fn meets_capital_requirement(&self) -> bool {
        self.registered_capital_jpy >= self.license_type.minimum_capital()
    }

    /// Get days until expiration
    pub fn days_until_expiration(&self) -> i64 {
        let now = chrono::Utc::now().date_naive();
        (self.expiration_date - now).num_days()
    }
}

/// Real estate transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TransactionType {
    /// Sale (売買)
    Sale,
    /// Lease (賃貸借)
    Lease,
    /// Exchange (交換)
    Exchange,
    /// Brokerage (仲介)
    Brokerage,
}

/// Property type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PropertyType {
    /// Land (土地)
    Land,
    /// Building (建物)
    Building,
    /// Land and building (土地建物)
    LandAndBuilding,
}

/// Property information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Property {
    /// Property type
    pub property_type: PropertyType,
    /// Address
    pub address: String,
    /// Area in square meters
    pub area_sqm: f64,
    /// Price in JPY
    pub price_jpy: u64,
    /// Description
    pub description: Option<String>,
}

/// Party in transaction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Party {
    /// Name
    pub name: String,
    /// Address
    pub address: String,
    /// Contact information
    pub contact: Option<String>,
}

/// Licensed real estate agent (宅地建物取引士)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LicensedAgent {
    /// Agent name
    pub name: String,
    /// Registration number
    pub registration_number: String,
    /// Registration date
    pub registration_date: NaiveDate,
}

impl LicensedAgent {
    /// Check if registration is valid (5-year renewal period)
    pub fn is_registration_valid(&self) -> bool {
        let now = chrono::Utc::now().date_naive();
        let years_since_registration = (now - self.registration_date).num_days() / 365;
        years_since_registration < 5
    }
}

/// Licensed broker information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LicensedBroker {
    /// Company name
    pub company_name: String,
    /// License number
    pub license_number: String,
    /// Licensed agent
    pub agent: LicensedAgent,
    /// Commission in JPY
    pub commission_jpy: u64,
}

/// Real estate transaction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RealEstateTransaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Property
    pub property: Property,
    /// Buyer/tenant
    pub buyer: Party,
    /// Seller/landlord
    pub seller: Party,
    /// Broker (optional for direct transactions)
    pub broker: Option<LicensedBroker>,
    /// Whether important matters were explained (Article 35)
    pub important_matters_explained: bool,
    /// Contract date
    pub contract_date: NaiveDate,
}

impl RealEstateTransaction {
    /// Calculate maximum allowed commission (Article 46)
    pub fn calculate_max_commission(&self) -> u64 {
        let price = self.property.price_jpy;

        // Commission rates (simplified):
        // ¥0-¥2M: 5% + tax
        // ¥2M-¥4M: 4% + tax
        // Over ¥4M: 3% + tax
        const TAX_RATE: f64 = 0.10; // 10% consumption tax

        let base_commission = if price <= 2_000_000 {
            (price as f64 * 0.05) as u64
        } else if price <= 4_000_000 {
            let part1 = 2_000_000.0 * 0.05;
            let part2 = ((price - 2_000_000) as f64) * 0.04;
            (part1 + part2) as u64
        } else {
            let part1 = 2_000_000.0 * 0.05;
            let part2 = 2_000_000.0 * 0.04;
            let part3 = ((price - 4_000_000) as f64) * 0.03;
            (part1 + part2 + part3) as u64
        };

        // Add consumption tax
        (base_commission as f64 * (1.0 + TAX_RATE)) as u64
    }

    /// Check if commission is within legal limits
    pub fn is_commission_valid(&self) -> bool {
        if let Some(broker) = &self.broker {
            broker.commission_jpy <= self.calculate_max_commission()
        } else {
            true // No broker, no commission issue
        }
    }
}

/// Real estate license
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RealEstateLicense {
    /// License number
    pub license_number: String,
    /// Business name
    pub business_name: String,
    /// Prefecture
    pub prefecture: String,
    /// Licensed agents
    pub licensed_agents: Vec<LicensedAgent>,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Expiration date (5 years - Article 3-3)
    pub expiration_date: NaiveDate,
}

impl RealEstateLicense {
    /// Check if license is currently valid
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().date_naive();
        now >= self.issue_date && now <= self.expiration_date
    }

    /// Check if at least one licensed agent is present
    pub fn has_valid_agent(&self) -> bool {
        !self.licensed_agents.is_empty()
            && self
                .licensed_agents
                .iter()
                .any(|a| a.is_registration_valid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_construction_license_type() {
        assert_eq!(
            ConstructionLicenseType::General.minimum_capital(),
            5_000_000
        );
        assert_eq!(
            ConstructionLicenseType::Special.minimum_capital(),
            20_000_000
        );
    }

    #[test]
    fn test_construction_license_validity() {
        let license = ConstructionBusinessLicense {
            license_number: "TEST-001".to_string(),
            business_name: "Test Construction".to_string(),
            license_type: ConstructionLicenseType::General,
            construction_types: vec![ConstructionType::Architecture],
            registered_capital_jpy: 10_000_000,
            issue_date: Utc::now().date_naive() - chrono::Duration::days(30),
            expiration_date: Utc::now().date_naive() + chrono::Duration::days(365),
            managers: vec![],
        };

        assert!(license.is_valid());
        assert!(license.meets_capital_requirement());
    }

    #[test]
    fn test_insufficient_capital() {
        let license = ConstructionBusinessLicense {
            license_number: "TEST-002".to_string(),
            business_name: "Test Construction".to_string(),
            license_type: ConstructionLicenseType::Special,
            construction_types: vec![ConstructionType::Civil],
            registered_capital_jpy: 10_000_000, // Less than required ¥20M
            issue_date: Utc::now().date_naive(),
            expiration_date: Utc::now().date_naive() + chrono::Duration::days(365 * 5),
            managers: vec![],
        };

        assert!(!license.meets_capital_requirement());
    }

    #[test]
    fn test_commission_calculation() {
        let transaction = RealEstateTransaction {
            transaction_id: "TX-001".to_string(),
            transaction_type: TransactionType::Sale,
            property: Property {
                property_type: PropertyType::Building,
                address: "Tokyo".to_string(),
                area_sqm: 100.0,
                price_jpy: 10_000_000,
                description: None,
            },
            buyer: Party {
                name: "Buyer".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            seller: Party {
                name: "Seller".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            broker: None,
            important_matters_explained: true,
            contract_date: Utc::now().date_naive(),
        };

        let max_commission = transaction.calculate_max_commission();
        assert!(max_commission > 0);
        assert!(max_commission < 1_000_000); // Reasonable upper bound
    }

    #[test]
    fn test_commission_validation() {
        let agent = LicensedAgent {
            name: "Agent".to_string(),
            registration_number: "AG-001".to_string(),
            registration_date: Utc::now().date_naive(),
        };

        let broker = LicensedBroker {
            company_name: "Test Realty".to_string(),
            license_number: "LIC-001".to_string(),
            agent,
            commission_jpy: 100_000, // Within limits for ¥5M property
        };

        let transaction = RealEstateTransaction {
            transaction_id: "TX-002".to_string(),
            transaction_type: TransactionType::Sale,
            property: Property {
                property_type: PropertyType::LandAndBuilding,
                address: "Osaka".to_string(),
                area_sqm: 150.0,
                price_jpy: 5_000_000,
                description: None,
            },
            buyer: Party {
                name: "Buyer".to_string(),
                address: "Osaka".to_string(),
                contact: None,
            },
            seller: Party {
                name: "Seller".to_string(),
                address: "Osaka".to_string(),
                contact: None,
            },
            broker: Some(broker),
            important_matters_explained: true,
            contract_date: Utc::now().date_naive(),
        };

        assert!(transaction.is_commission_valid());
    }

    #[test]
    fn test_licensed_agent_validity() {
        let recent_agent = LicensedAgent {
            name: "Recent Agent".to_string(),
            registration_number: "AG-RECENT".to_string(),
            registration_date: Utc::now().date_naive() - chrono::Duration::days(365),
        };
        assert!(recent_agent.is_registration_valid());

        let old_agent = LicensedAgent {
            name: "Old Agent".to_string(),
            registration_number: "AG-OLD".to_string(),
            registration_date: Utc::now().date_naive() - chrono::Duration::days(365 * 6),
        };
        assert!(!old_agent.is_registration_valid());
    }
}
