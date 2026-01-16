//! Core IP types shared across all IP domains

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of intellectual property right
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpRightType {
    /// Patent (Patents Act 1977)
    Patent,
    /// Trade mark (Trade Marks Act 1994)
    TradeMark,
    /// Copyright (CDPA 1988)
    Copyright,
    /// Registered design
    RegisteredDesign,
    /// Unregistered design right
    UnregisteredDesignRight,
    /// Database right
    DatabaseRight,
    /// Performer's right
    PerformersRight,
}

/// Generic IP right
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpRight {
    /// Type of right
    pub right_type: IpRightType,
    /// Unique identifier
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
    /// Jurisdiction
    pub jurisdiction: String,
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
    /// Country
    pub country: String,
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
    /// University/research institution
    Institution,
    /// Government entity
    Government,
}

/// Registration/grant status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Application filed but not yet examined
    Pending,
    /// Under examination
    UnderExamination,
    /// Granted/registered
    Granted,
    /// Refused
    Refused,
    /// Lapsed (non-payment of renewal fees)
    Lapsed,
    /// Revoked/invalidated
    Revoked,
    /// Expired naturally
    Expired,
    /// Abandoned by applicant
    Abandoned,
}

/// Prior art reference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PriorArt {
    /// Type of prior art
    pub art_type: PriorArtType,
    /// Reference identifier (patent number, publication, etc.)
    pub reference: String,
    /// Publication date
    pub publication_date: Option<NaiveDate>,
    /// Relevance to invention/mark
    pub relevance: String,
}

/// Type of prior art
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriorArtType {
    /// Earlier patent
    Patent,
    /// Scientific publication
    Publication,
    /// Public use/disclosure
    PublicUse,
    /// Earlier trademark registration
    TradeMark,
    /// Common law unregistered mark
    CommonLawMark,
    /// Website/online content
    OnlineContent,
}

/// License type for IP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LicenseType {
    /// Exclusive license
    Exclusive,
    /// Sole license (licensor and licensee only)
    Sole,
    /// Non-exclusive license
    NonExclusive,
    /// Compulsory license (imposed by authority)
    Compulsory,
}

/// Geographic scope of IP right
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicScope {
    /// UK-wide
    UnitedKingdom,
    /// England and Wales only
    EnglandWales,
    /// Scotland only
    Scotland,
    /// Northern Ireland only
    NorthernIreland,
    /// European (retained EU rights)
    European,
    /// International (PCT, Madrid Protocol)
    International,
}
