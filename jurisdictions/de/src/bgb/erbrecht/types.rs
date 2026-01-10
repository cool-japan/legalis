//! Types for German Succession Law (Erbrecht - BGB Book 5)
//!
//! This module provides type-safe representations of German inheritance law concepts
//! including legal succession, testamentary succession, wills, and compulsory portions.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

/// Type of succession (legal vs testamentary)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccessionType {
    /// Legal succession (Gesetzliche Erbfolge) - §§1924-1936 BGB
    Legal,
    /// Testamentary succession (Gewillkürte Erbfolge) - Based on will or inheritance contract
    Testamentary,
}

/// Deceased person (Erblasser)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deceased {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub date_of_death: NaiveDate,
    pub place_of_death: String,
    pub last_residence: String,
    pub nationality: String,
}

impl Deceased {
    /// Calculate age at death
    pub fn age_at_death(&self) -> u32 {
        let years = self.date_of_death.year() - self.date_of_birth.year();
        if self.date_of_death.month() < self.date_of_birth.month()
            || (self.date_of_death.month() == self.date_of_birth.month()
                && self.date_of_death.day() < self.date_of_birth.day())
        {
            (years - 1) as u32
        } else {
            years as u32
        }
    }
}

/// Heir (Erbe)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heir {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub relationship: RelationshipToDeceased,
    pub inheritance_share: InheritanceShare,
    pub is_statutory_heir: bool, // Gesetzlicher Erbe
}

/// Relationship to deceased person
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipToDeceased {
    /// Spouse (Ehegatte) - §1931 BGB
    Spouse,
    /// Child (Kind) - §1924 BGB (First order - Erste Ordnung)
    Child,
    /// Grandchild (Enkelkind) - §1924 BGB (First order)
    Grandchild,
    /// Parent (Elternteil) - §1925 BGB (Second order - Zweite Ordnung)
    Parent,
    /// Sibling (Geschwister) - §1925 BGB (Second order)
    Sibling,
    /// Grandparent (Großelternteil) - §1926 BGB (Third order - Dritte Ordnung)
    Grandparent,
    /// Uncle/Aunt (Onkel/Tante) - §1926 BGB (Third order)
    UncleAunt,
    /// Other relative (Sonstiger Verwandter)
    OtherRelative,
    /// Not related (Nicht verwandt) - testamentary heir only
    NotRelated,
}

/// Inheritance share (Erbteil)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InheritanceShare {
    /// Full estate (entire inheritance)
    Full,
    /// Fraction of estate (e.g., 1/2, 1/4)
    Fraction { numerator: u32, denominator: u32 },
    /// Specific amount in Euros
    SpecificAmount(Capital),
}

impl InheritanceShare {
    /// Convert fraction to decimal
    pub fn as_decimal(&self) -> Option<f64> {
        match self {
            InheritanceShare::Full => Some(1.0),
            InheritanceShare::Fraction {
                numerator,
                denominator,
            } => {
                if *denominator == 0 {
                    None
                } else {
                    Some(*numerator as f64 / *denominator as f64)
                }
            }
            InheritanceShare::SpecificAmount(_) => None, // Cannot express as fraction
        }
    }
}

/// Order of succession (Ordnung) under German law - §§1924-1929 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccessionOrder {
    /// First order (Erste Ordnung) - Descendants (§1924 BGB)
    First,
    /// Second order (Zweite Ordnung) - Parents and their descendants (§1925 BGB)
    Second,
    /// Third order (Dritte Ordnung) - Grandparents and their descendants (§1926 BGB)
    Third,
    /// Fourth order (Vierte Ordnung) - Great-grandparents and their descendants (§1928 BGB)
    Fourth,
}

/// Legal succession (Gesetzliche Erbfolge) - §§1924-1936 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalSuccession {
    pub deceased: Deceased,
    pub heirs: Vec<Heir>,
    pub succession_order: SuccessionOrder,
    pub spouse_inheritance: Option<SpouseInheritance>,
}

/// Spouse inheritance (Ehegattenerbrecht) - §1931 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpouseInheritance {
    pub spouse_name: String,
    pub marriage_date: NaiveDate,
    pub matrimonial_property_regime: MatrimonialPropertyRegime,
    pub share: InheritanceShare, // Depends on order and property regime
}

/// Matrimonial property regime (for inheritance purposes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatrimonialPropertyRegime {
    /// Community of accrued gains (Zugewinngemeinschaft) - DEFAULT
    /// Spouse gets 1/4 + 1/4 "accrued gains bonus" = 1/2 alongside first order
    CommunityOfAccruedGains,
    /// Separation of property (Gütertrennung)
    /// Spouse gets 1/4 alongside first order (no bonus)
    SeparationOfProperty,
    /// Community of property (Gütergemeinschaft)
    CommunityOfProperty,
}

/// Will type (Testamentsformen) - §§2231-2247 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WillType {
    /// Holographic will (Eigenhändiges Testament) - §2247 BGB
    /// Must be entirely handwritten and signed by testator
    Holographic,
    /// Public will (Öffentliches Testament) - §2232 BGB
    /// Declared before notary or deposited with court
    Public,
    /// Emergency will (Nottestament) - §§2249-2251 BGB
    /// In exceptional circumstances (e.g., imminent death, isolation)
    Emergency,
}

/// Will (Testament) - §§2064-2086 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Will {
    pub testator: Deceased, // Person who made the will
    pub will_type: WillType,
    pub created_at: NaiveDate,
    pub place_of_creation: String,
    pub is_handwritten: bool, // Required for holographic will
    pub has_signature: bool,  // Required for all wills
    pub has_date: bool,       // Recommended but not always required
    pub beneficiaries: Vec<WillBeneficiary>,
    pub revoked: bool,
    pub revoked_at: Option<NaiveDate>,
}

impl Will {
    /// Check if will is still valid (not revoked and testator deceased)
    pub fn is_valid(&self) -> bool {
        !self.revoked
    }

    /// Check if holographic will meets formality requirements (§2247 BGB)
    pub fn meets_holographic_requirements(&self) -> bool {
        matches!(self.will_type, WillType::Holographic) && self.is_handwritten && self.has_signature
    }
}

/// Beneficiary named in will
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WillBeneficiary {
    pub name: String,
    pub relationship: RelationshipToDeceased,
    pub inheritance_share: InheritanceShare,
    pub conditions: Vec<String>, // Conditions attached to inheritance
}

/// Compulsory portion (Pflichtteil) - §§2303-2338 BGB
///
/// Close relatives entitled to minimum share even if disinherited
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompulsoryPortion {
    pub claimant: CompulsoryPortionClaimant,
    pub deceased: Deceased,
    pub estate_value: Capital,
    pub portion: InheritanceShare, // Always 1/2 of legal share
    pub amount: Capital,           // Calculated from estate value
}

impl CompulsoryPortion {
    /// Calculate compulsory portion amount
    /// Pflichtteil = 1/2 of legal inheritance share
    pub fn calculate_amount(&self) -> Capital {
        if let Some(decimal) = self.portion.as_decimal() {
            Capital::from_cents((self.estate_value.amount_cents as f64 * decimal) as u64)
        } else {
            Capital::from_cents(0)
        }
    }
}

/// Person entitled to compulsory portion (Pflichtteilsberechtigter)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompulsoryPortionClaimant {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub relationship: RelationshipToDeceased,
}

impl CompulsoryPortionClaimant {
    /// Check if relationship entitles to compulsory portion (§2303 BGB)
    /// Entitled: Descendants, parents (if no descendants), spouse
    pub fn is_entitled(&self) -> bool {
        matches!(
            self.relationship,
            RelationshipToDeceased::Child
                | RelationshipToDeceased::Grandchild
                | RelationshipToDeceased::Parent
                | RelationshipToDeceased::Spouse
        )
    }
}

/// Inheritance contract (Erbvertrag) - §§2274-2302 BGB
///
/// Binding agreement about succession (more formal than will)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InheritanceContract {
    pub testator: String, // Person making the contract
    pub beneficiary: String,
    pub contract_date: NaiveDate,
    pub notarized: bool, // REQUIRED per §2276 BGB
    pub inheritance_share: InheritanceShare,
    pub is_mutual: bool, // Mutual contracts between spouses
    pub revoked: bool,
}

/// Estate (Nachlass)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Estate {
    pub deceased: Deceased,
    pub total_value: Capital,
    pub assets: Vec<Asset>,
    pub liabilities: Vec<Liability>,
    pub net_value: Capital,
}

impl Estate {
    /// Calculate net estate value (assets - liabilities)
    pub fn calculate_net_value(&self) -> Capital {
        let total_assets: u64 = self.assets.iter().map(|a| a.value.amount_cents).sum();
        let total_liabilities: u64 = self.liabilities.iter().map(|l| l.amount.amount_cents).sum();

        if total_assets > total_liabilities {
            Capital::from_cents(total_assets - total_liabilities)
        } else {
            Capital::from_cents(0)
        }
    }
}

/// Asset in estate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub description: String,
    pub asset_type: AssetType,
    pub value: Capital,
}

/// Type of asset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    RealEstate,
    BankAccount,
    Securities,
    BusinessInterest,
    MovableProperty,
    IntellectualProperty,
    Other,
}

/// Liability in estate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Liability {
    pub description: String,
    pub creditor: String,
    pub amount: Capital,
}

/// Acceptance or renunciation of inheritance (Annahme/Ausschlagung) - §§1942-2063 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InheritanceDecision {
    pub heir: Heir,
    pub decision: InheritanceDecisionType,
    pub decision_date: NaiveDate,
    pub deadline: NaiveDate, // 6 weeks from knowledge of inheritance (§1944 BGB)
}

/// Type of inheritance decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritanceDecisionType {
    /// Acceptance (Annahme) - §1942 BGB
    /// Heir accepts inheritance with all assets and liabilities
    Acceptance,
    /// Renunciation (Ausschlagung) - §1943 BGB
    /// Heir rejects inheritance (must be done within 6 weeks)
    Renunciation,
}

/// Testamentary capacity (Testierfähigkeit) - §§2229-2230 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestamentaryCapacity {
    /// Full capacity (age 18+) - §2229 BGB
    Full,
    /// Limited capacity (age 16-17, with special formalities) - §2229 BGB
    Limited,
    /// No capacity (under 16 or incapacitated)
    None,
}

/// Certificate of inheritance (Erbschein) - §§2353-2370 BGB
///
/// Official document proving heir status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateOfInheritance {
    pub heir_name: String,
    pub deceased_name: String,
    pub inheritance_share: String, // "Sole heir", "1/2", etc.
    pub issued_by: String,         // Court (Nachlassgericht)
    pub issued_at: NaiveDate,
    pub is_joint_certificate: bool, // For multiple heirs
}

/// Right of representation (Eintrittsrecht) - §1924 Abs. 2-3 BGB
///
/// Descendants of a deceased heir inherit in their place
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RightOfRepresentation {
    pub original_heir: String,           // Deceased heir
    pub original_heir_deceased: bool,    // Must be deceased for representation
    pub representing_heirs: Vec<String>, // Descendants who inherit instead
}
