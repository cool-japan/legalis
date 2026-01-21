//! Core IP types shared across all IP domains

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of intellectual property right
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpRightType {
    /// Patent (Patents Act 1990)
    Patent,
    /// Innovation patent (now phased out)
    InnovationPatent,
    /// Trade mark (Trade Marks Act 1995)
    TradeMark,
    /// Copyright (Copyright Act 1968)
    Copyright,
    /// Registered design (Designs Act 2003)
    RegisteredDesign,
    /// Plant breeder's right (Plant Breeder's Rights Act 1994)
    PlantBreedersRight,
    /// Circuit layout right (Circuit Layouts Act 1989)
    CircuitLayoutRight,
}

/// Generic IP right
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpRight {
    /// Type of right
    pub right_type: IpRightType,
    /// Unique identifier (application/registration number)
    pub id: String,
    /// Title/name
    pub title: String,
    /// Owner
    pub owner: IpOwner,
    /// Filing/creation date
    pub filing_date: Option<NaiveDate>,
    /// Grant/registration date
    pub grant_date: Option<NaiveDate>,
    /// Expiry date
    pub expiry_date: Option<NaiveDate>,
    /// Registration status
    pub status: RegistrationStatus,
    /// Jurisdiction (always "AU" for this module)
    pub jurisdiction: String,
}

impl IpRight {
    /// Check if the IP right is currently in force
    pub fn is_in_force(&self) -> bool {
        matches!(self.status, RegistrationStatus::Granted)
            && self
                .expiry_date
                .map(|d| d > chrono::Utc::now().date_naive())
                .unwrap_or(true)
    }
}

/// IP right owner
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpOwner {
    /// Name
    pub name: String,
    /// Type (individual, company, partnership)
    pub owner_type: OwnerType,
    /// Address
    pub address: Option<String>,
    /// Country (ISO 3166-1 alpha-2)
    pub country: String,
    /// ABN (for Australian entities)
    pub abn: Option<String>,
    /// ACN (for Australian companies)
    pub acn: Option<String>,
}

impl IpOwner {
    /// Create an Australian individual owner
    pub fn australian_individual(name: impl Into<String>, address: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            owner_type: OwnerType::Individual,
            address: Some(address.into()),
            country: "AU".to_string(),
            abn: None,
            acn: None,
        }
    }

    /// Create an Australian company owner
    pub fn australian_company(
        name: impl Into<String>,
        acn: impl Into<String>,
        abn: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            owner_type: OwnerType::Company,
            address: None,
            country: "AU".to_string(),
            abn,
            acn: Some(acn.into()),
        }
    }
}

/// Type of IP owner
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OwnerType {
    /// Individual person
    Individual,
    /// Company/corporation
    Company,
    /// Partnership
    Partnership,
    /// Trust
    Trust,
    /// University/research institution
    Institution,
    /// Government entity
    Government,
    /// Joint owners
    JointOwnership,
}

/// Registration/grant status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Application filed but not yet examined
    Pending,
    /// Under examination
    UnderExamination,
    /// Accepted (awaiting registration)
    Accepted,
    /// Granted/registered
    Granted,
    /// Refused
    Refused,
    /// Lapsed (non-payment of renewal fees)
    Lapsed,
    /// Revoked/cancelled
    Revoked,
    /// Expired naturally
    Expired,
    /// Abandoned by applicant
    Abandoned,
    /// Opposed (in opposition proceedings)
    Opposed,
    /// Certified (for designs)
    Certified,
}

impl RegistrationStatus {
    /// Whether the status represents a valid, enforceable right
    pub fn is_enforceable(&self) -> bool {
        matches!(
            self,
            RegistrationStatus::Granted | RegistrationStatus::Certified
        )
    }

    /// Whether the status allows opposition
    pub fn allows_opposition(&self) -> bool {
        matches!(self, RegistrationStatus::Accepted)
    }
}

/// Prior art reference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PriorArt {
    /// Type of prior art
    pub art_type: PriorArtType,
    /// Reference identifier (patent number, publication, etc.)
    pub reference: String,
    /// Publication/disclosure date
    pub publication_date: Option<NaiveDate>,
    /// Relevance to invention/mark
    pub relevance: String,
    /// Country of origin
    pub country: Option<String>,
}

/// Type of prior art
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriorArtType {
    /// Earlier Australian patent
    AustralianPatent,
    /// Earlier foreign patent
    ForeignPatent,
    /// PCT application
    PctApplication,
    /// Scientific publication
    Publication,
    /// Public use/disclosure in Australia
    PublicUseAustralia,
    /// Public use/disclosure overseas
    PublicUseOverseas,
    /// Earlier trademark registration
    TradeMark,
    /// Common law unregistered mark
    CommonLawMark,
    /// Website/online content
    OnlineContent,
    /// Prior design
    PriorDesign,
}

/// License type for IP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LicenseType {
    /// Exclusive license (licensee only can use)
    Exclusive,
    /// Sole license (licensor and licensee only)
    Sole,
    /// Non-exclusive license
    NonExclusive,
    /// Compulsory license (imposed by Federal Court - s.133 Patents Act)
    Compulsory,
    /// Cross-license
    CrossLicense,
}

/// Geographic scope of IP right
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicScope {
    /// Australia-wide
    Australia,
    /// State/territory specific
    StateTerritory(String),
    /// International via Paris Convention
    International,
    /// PCT countries
    Pct,
    /// Madrid Protocol (trade marks)
    MadridProtocol,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_right_in_force() {
        let right = IpRight {
            right_type: IpRightType::Patent,
            id: "2024100001".to_string(),
            title: "Test invention".to_string(),
            owner: IpOwner::australian_individual("Test Person", "Sydney, Australia"),
            filing_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            grant_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
            expiry_date: Some(NaiveDate::from_ymd_opt(2044, 1, 1).unwrap()),
            status: RegistrationStatus::Granted,
            jurisdiction: "AU".to_string(),
        };

        assert!(right.is_in_force());
    }

    #[test]
    fn test_ip_right_expired() {
        let right = IpRight {
            right_type: IpRightType::Patent,
            id: "2000100001".to_string(),
            title: "Old invention".to_string(),
            owner: IpOwner::australian_individual("Old Person", "Melbourne, Australia"),
            filing_date: Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            grant_date: Some(NaiveDate::from_ymd_opt(2001, 1, 1).unwrap()),
            expiry_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            status: RegistrationStatus::Expired,
            jurisdiction: "AU".to_string(),
        };

        assert!(!right.is_in_force());
    }

    #[test]
    fn test_owner_creation() {
        let individual = IpOwner::australian_individual("John Smith", "123 Main St, Sydney");
        assert_eq!(individual.country, "AU");
        assert_eq!(individual.owner_type, OwnerType::Individual);

        let company = IpOwner::australian_company(
            "Tech Pty Ltd",
            "123456789",
            Some("12345678901".to_string()),
        );
        assert_eq!(company.country, "AU");
        assert!(company.acn.is_some());
    }

    #[test]
    fn test_registration_status() {
        assert!(RegistrationStatus::Granted.is_enforceable());
        assert!(RegistrationStatus::Certified.is_enforceable());
        assert!(!RegistrationStatus::Pending.is_enforceable());
        assert!(RegistrationStatus::Accepted.allows_opposition());
    }
}
