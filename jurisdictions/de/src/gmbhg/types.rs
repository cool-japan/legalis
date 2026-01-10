//! German company law types (GmbH-Gesetz)
//!
//! Type-safe representations of German company structures, capital requirements,
//! articles of association, and managing directors under the GmbHG.
//!
//! # Legal Context
//!
//! The GmbH (Gesellschaft mit beschränkter Haftung) is Germany's most popular
//! limited liability company form, requiring €25,000 minimum capital (§5 GmbHG).
//!
//! The UG (Unternehmergesellschaft - haftungsbeschränkt) is a "mini-GmbH"
//! allowing formation with as little as €1, but requiring 25% of annual profits
//! to be allocated to reserves until reaching €25,000 (§5a GmbHG).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// =============================================================================
// Company Types
// =============================================================================

/// Company type under German law
///
/// # German Legal Forms
///
/// - **GmbH** (Gesellschaft mit beschränkter Haftung): Standard limited liability company
/// - **UG** (Unternehmergesellschaft - haftungsbeschränkt): "Mini-GmbH" with €1 minimum capital
/// - **AG** (Aktiengesellschaft): Stock corporation
/// - **OHG** (Offene Handelsgesellschaft): General partnership
/// - **KG** (Kommanditgesellschaft): Limited partnership
/// - **GmbH & Co. KG**: Hybrid structure (GmbH as general partner of KG)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// GmbH - Standard limited liability company
    ///
    /// Minimum capital: €25,000 (§5 GmbHG)
    /// Initial contribution: At least 50% or €12,500 (§7 Abs. 2 GmbHG)
    GmbH,

    /// UG (haftungsbeschränkt) - "Mini-GmbH"
    ///
    /// Minimum capital: €1 (§5a GmbHG)
    /// Maximum capital: €24,999 (at €25,000 becomes GmbH)
    /// Reserve requirement: 25% of annual profits until reaching €25,000
    UG,

    /// AG - Stock corporation (Aktiengesellschaft)
    ///
    /// Minimum capital: €50,000 (§7 AktG)
    /// Requires supervisory board (Aufsichtsrat)
    AG,

    /// OHG - General partnership (Offene Handelsgesellschaft)
    ///
    /// No minimum capital, unlimited liability for all partners
    OHG,

    /// KG - Limited partnership (Kommanditgesellschaft)
    ///
    /// Mixed liability: General partners (unlimited), Limited partners (limited)
    KG,

    /// GmbH & Co. KG - Hybrid structure
    ///
    /// GmbH serves as general partner (Komplementär) of the KG
    GmbHCoKG,
}

// =============================================================================
// Capital Structure
// =============================================================================

/// Capital amount in Euro (€)
///
/// Stored as cents (u64) to avoid floating-point precision issues.
///
/// # Examples
///
/// ```
/// use legalis_de::gmbhg::Capital;
///
/// let capital = Capital::from_euros(25_000);
/// assert_eq!(capital.to_euros(), 25_000.0);
/// assert!(capital.is_valid_for_gmbh());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Capital {
    /// Amount in Euro cents (to avoid floating point)
    pub amount_cents: u64,
}

impl Capital {
    /// GmbH minimum capital: €25,000 (§5 GmbHG)
    pub const GMBH_MINIMUM_CENTS: u64 = 2_500_000; // €25,000

    /// UG minimum capital: €1 (§5a GmbHG)
    pub const UG_MINIMUM_CENTS: u64 = 100; // €1

    /// UG maximum capital: €24,999.99 (at €25,000 becomes GmbH)
    pub const UG_MAXIMUM_CENTS: u64 = 2_499_999; // €24,999.99

    /// AG minimum capital: €50,000 (§7 AktG)
    pub const AG_MINIMUM_CENTS: u64 = 5_000_000; // €50,000

    /// GmbH initial contribution requirement: €12,500 minimum (§7 Abs. 2 GmbHG)
    pub const GMBH_INITIAL_CONTRIBUTION_MIN_CENTS: u64 = 1_250_000; // €12,500

    /// Create capital from euros and cents
    ///
    /// # Arguments
    ///
    /// * `euros` - Amount in euros
    /// * `cents` - Additional cents (0-99)
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_de::gmbhg::Capital;
    ///
    /// let capital = Capital::new(25_000, 0);
    /// assert_eq!(capital.to_euros(), 25_000.0);
    /// ```
    pub fn new(euros: u64, cents: u64) -> Self {
        Self {
            amount_cents: euros * 100 + cents.min(99),
        }
    }

    /// Create capital from euros only
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_de::gmbhg::Capital;
    ///
    /// let capital = Capital::from_euros(50_000);
    /// assert_eq!(capital.to_euros(), 50_000.0);
    /// ```
    pub fn from_euros(euros: u64) -> Self {
        Self {
            amount_cents: euros * 100,
        }
    }

    /// Create capital from cents
    pub fn from_cents(cents: u64) -> Self {
        Self {
            amount_cents: cents,
        }
    }

    /// Convert to euros as f64
    pub fn to_euros(&self) -> f64 {
        (self.amount_cents as f64) / 100.0
    }

    /// Get euros component
    pub fn euros(&self) -> u64 {
        self.amount_cents / 100
    }

    /// Get cents component (0-99)
    pub fn cents(&self) -> u64 {
        self.amount_cents % 100
    }

    /// Check if capital meets GmbH minimum requirement (§5 GmbHG)
    pub fn is_valid_for_gmbh(&self) -> bool {
        self.amount_cents >= Self::GMBH_MINIMUM_CENTS
    }

    /// Check if capital meets UG requirements (§5a GmbHG)
    ///
    /// UG capital must be:
    /// - At least €1
    /// - Less than €25,000 (otherwise it's a GmbH)
    pub fn is_valid_for_ug(&self) -> bool {
        self.amount_cents >= Self::UG_MINIMUM_CENTS && self.amount_cents <= Self::UG_MAXIMUM_CENTS
    }

    /// Check if capital meets AG minimum requirement (§7 AktG)
    pub fn is_valid_for_ag(&self) -> bool {
        self.amount_cents >= Self::AG_MINIMUM_CENTS
    }
}

// =============================================================================
// Articles of Association (Gesellschaftsvertrag)
// =============================================================================

/// Articles of Association per §3 GmbHG
///
/// The articles of association (Gesellschaftsvertrag) must be notarially
/// certified (notariell beurkundet) per §2 GmbHG.
///
/// # Mandatory Elements (§3 Abs. 1 GmbHG)
///
/// 1. Company name (Firma) - §4
/// 2. Registered office (Sitz) - §4a
/// 3. Business purpose (Unternehmensgegenstand) - §3(1) Nr. 2
/// 4. Share capital amount (Stammkapital) - §5
/// 5. Nominal amount of each share (Nennbetrag der Geschäftsanteile) - §3(1) Nr. 4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticlesOfAssociation {
    // ========================================================================
    // MANDATORY ELEMENTS (§3 Abs. 1 GmbHG)
    // ========================================================================
    /// Company name (Firma) - must include "GmbH" or "UG (haftungsbeschränkt)" (§4 GmbHG)
    pub company_name: String,

    /// Registered office (Sitz) - must be a German city (§4a GmbHG)
    pub registered_office: RegisteredOffice,

    /// Business purpose (Unternehmensgegenstand) - specific, lawful (§3 Abs. 1 Nr. 2 GmbHG)
    pub business_purpose: String,

    /// Share capital (Stammkapital) - €25,000 for GmbH, €1-€24,999 for UG (§5 GmbHG)
    pub share_capital: Capital,

    /// Nominal amount of each share (Nennbetrag jedes Geschäftsanteils) (§3 Abs. 1 Nr. 4 GmbHG)
    ///
    /// Sum of all nominal amounts must equal share_capital
    pub share_structure: Vec<ShareAllocation>,

    // ========================================================================
    // OPTIONAL BUT COMMON ELEMENTS
    // ========================================================================
    /// Duration (Dauer) - usually unlimited
    pub duration: Option<Duration>,

    /// Fiscal year end (Geschäftsjahr)
    pub fiscal_year_end: Option<FiscalYearEnd>,

    /// Formation date (Gründungsdatum)
    pub formation_date: Option<DateTime<Utc>>,

    /// Shareholder resolution requirements (custom supermajority requirements, etc.)
    pub resolution_requirements: Option<ResolutionRequirements>,
}

/// Registered office (Sitz der Gesellschaft)
///
/// Must be located in Germany (§4a GmbHG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredOffice {
    /// City name (Stadtname) - e.g., "Berlin", "München", "Hamburg"
    pub city: String,

    /// Full address (optional in articles, required for commercial register)
    pub full_address: Option<String>,
}

/// Share allocation to shareholder (Geschäftsanteil)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareAllocation {
    /// Shareholder (Gesellschafter)
    pub shareholder: Shareholder,

    /// Nominal amount (Nennbetrag) in cents - must be at least €1
    pub nominal_amount_cents: u64,

    /// Contribution paid (Einlage geleistet) - at least 50% for GmbH or €12,500 (§7 Abs. 2 GmbHG)
    pub contribution_paid_cents: u64,
}

/// Shareholder (Gesellschafter)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shareholder {
    /// Name (natural person or legal entity)
    pub name: String,

    /// Address (Anschrift)
    pub address: String,

    /// Type (natural person or legal entity)
    pub shareholder_type: ShareholderType,
}

/// Shareholder type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderType {
    /// Natural person (natürliche Person)
    NaturalPerson,

    /// Legal entity (juristische Person) - e.g., another GmbH, AG
    LegalEntity,
}

/// Company duration (Dauer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Duration {
    /// Unlimited (unbegrenzt) - most common
    Unlimited,

    /// Until specific date
    UntilDate(DateTime<Utc>),

    /// For specific number of years from formation
    ForYears(u32),
}

/// Fiscal year end (Geschäftsjahresende)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalYearEnd {
    /// Month (1-12)
    pub month: u8,

    /// Day (1-31, depending on month)
    pub day: u8,
}

/// Shareholder resolution requirements
///
/// Allows customization of voting thresholds beyond statutory defaults
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionRequirements {
    /// Quorum requirement (percentage of capital represented)
    pub quorum_percentage: Option<u8>,

    /// Simple majority threshold (default: 50% + 1)
    pub simple_majority_percentage: Option<u8>,

    /// Supermajority threshold for important decisions
    pub supermajority_percentage: Option<u8>,
}

// =============================================================================
// Managing Directors (Geschäftsführer)
// =============================================================================

/// Managing director(s) per §35 GmbHG
///
/// At least one managing director is required (§6 Abs. 3 GmbHG).
/// Must be a natural person with full legal capacity (§6 Abs. 2 S. 2 GmbHG).
///
/// # Legal Requirements
///
/// - Natural person only (keine juristische Person)
/// - Full legal capacity (voll geschäftsfähig)
/// - No general disqualification (keine generelle Ungeeignetheit)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagingDirector {
    /// Name (Vor- und Nachname)
    pub name: String,

    /// Date of birth (Geburtsdatum) - required for commercial register
    pub date_of_birth: Option<NaiveDate>,

    /// Address (Wohnort)
    pub address: String,

    /// Appointment date (Bestellung)
    pub appointment_date: DateTime<Utc>,

    /// Representation authority (Vertretungsbefugnis)
    ///
    /// Defines how the managing director can represent the company
    pub representation_authority: RepresentationAuthority,

    /// Capacity to serve (Geschäftsfähigkeit)
    ///
    /// Must be true - natural person with full capacity (§6 Abs. 2 S. 2 GmbHG)
    pub has_capacity: bool,
}

/// Collection of managing directors
///
/// At least one managing director is required (§6 Abs. 3 GmbHG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagingDirectors {
    /// List of managing directors (Geschäftsführer)
    pub directors: Vec<ManagingDirector>,
}

/// Representation authority (Vertretungsbefugnis)
///
/// Defines how managing directors can represent the company externally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepresentationAuthority {
    /// Sole representation (Einzelvertretung)
    ///
    /// Director can represent the company alone
    Sole,

    /// Joint representation with all directors (Gesamtvertretung)
    ///
    /// Director must act together with all other directors
    JointWithAll,

    /// Joint representation with another director (Gesamtvertretung mit einem anderen)
    ///
    /// Director must act together with at least one other director
    JointWithOne,
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capital_from_euros() {
        let capital = Capital::from_euros(25_000);
        assert_eq!(capital.amount_cents, 2_500_000);
        assert_eq!(capital.to_euros(), 25_000.0);
        assert_eq!(capital.euros(), 25_000);
        assert_eq!(capital.cents(), 0);
    }

    #[test]
    fn test_capital_new_with_cents() {
        let capital = Capital::new(100, 50);
        assert_eq!(capital.amount_cents, 10_050);
        assert_eq!(capital.to_euros(), 100.50);
        assert_eq!(capital.euros(), 100);
        assert_eq!(capital.cents(), 50);
    }

    #[test]
    fn test_capital_gmbh_validation() {
        let valid = Capital::from_euros(25_000);
        assert!(valid.is_valid_for_gmbh());

        let invalid = Capital::from_euros(24_999);
        assert!(!invalid.is_valid_for_gmbh());

        let high = Capital::from_euros(1_000_000);
        assert!(high.is_valid_for_gmbh());
    }

    #[test]
    fn test_capital_ug_validation() {
        let min = Capital::from_euros(1);
        assert!(min.is_valid_for_ug());

        let max = Capital::from_cents(2_499_999);
        assert!(max.is_valid_for_ug());

        let too_low = Capital::from_cents(99);
        assert!(!too_low.is_valid_for_ug());

        let too_high = Capital::from_euros(25_000);
        assert!(!too_high.is_valid_for_ug());
    }

    #[test]
    fn test_capital_ag_validation() {
        let valid = Capital::from_euros(50_000);
        assert!(valid.is_valid_for_ag());

        let invalid = Capital::from_euros(49_999);
        assert!(!invalid.is_valid_for_ag());
    }

    #[test]
    fn test_capital_ordering() {
        let low = Capital::from_euros(1);
        let mid = Capital::from_euros(25_000);
        let high = Capital::from_euros(50_000);

        assert!(low < mid);
        assert!(mid < high);
        assert!(low < high);
    }

    #[test]
    fn test_company_type_enum() {
        let gmbh = CompanyType::GmbH;
        let ug = CompanyType::UG;

        assert_ne!(gmbh, ug);

        // Test serialization round-trip
        let json = serde_json::to_string(&gmbh).unwrap();
        let deserialized: CompanyType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, gmbh);
    }

    #[test]
    fn test_registered_office() {
        let office = RegisteredOffice {
            city: "Berlin".to_string(),
            full_address: Some("Alexanderplatz 1, 10178 Berlin".to_string()),
        };

        assert_eq!(office.city, "Berlin");
        assert!(office.full_address.is_some());
    }

    #[test]
    fn test_shareholder_type() {
        let natural = ShareholderType::NaturalPerson;
        let legal = ShareholderType::LegalEntity;

        assert_ne!(natural, legal);
    }

    #[test]
    fn test_representation_authority() {
        let sole = RepresentationAuthority::Sole;
        let joint_all = RepresentationAuthority::JointWithAll;
        let joint_one = RepresentationAuthority::JointWithOne;

        assert_ne!(sole, joint_all);
        assert_ne!(sole, joint_one);
        assert_ne!(joint_all, joint_one);
    }

    #[test]
    fn test_fiscal_year_end() {
        let fye = FiscalYearEnd { month: 12, day: 31 };

        assert_eq!(fye.month, 12);
        assert_eq!(fye.day, 31);
    }

    #[test]
    fn test_duration_enum() {
        let unlimited = Duration::Unlimited;
        let for_years = Duration::ForYears(10);

        assert_ne!(unlimited, for_years);
    }
}
