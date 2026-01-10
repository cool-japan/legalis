//! German Commercial Code (Handelsgesetzbuch - HGB) types
//!
//! Type-safe representations of partnerships, merchant status, and commercial
//! entities under the HGB.
//!
//! # Legal Context
//!
//! The HGB regulates commercial activities and business entities in Germany:
//! - Merchant status (Kaufmannseigenschaft) - §1-7 HGB
//! - General Partnership (OHG - Offene Handelsgesellschaft) - §105-160 HGB
//! - Limited Partnership (KG - Kommanditgesellschaft) - §161-177a HGB

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

// =============================================================================
// Merchant Status (Kaufmannseigenschaft)
// =============================================================================

/// Merchant status under German commercial law
///
/// Determines whether an entity is subject to HGB regulations.
///
/// # Legal Background
///
/// The HGB distinguishes between different types of merchants based on
/// how they acquire merchant status:
///
/// - **Istkaufmann** (§1 HGB): Operates a commercial business
/// - **Kannkaufmann** (§2, §3 HGB): Voluntary registration
/// - **Formkaufmann** (§6 HGB): By legal form (AG, GmbH, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MerchantType {
    /// Commercial merchant (Istkaufmann - §1 HGB)
    ///
    /// Acquires merchant status by operating a commercial business
    /// requiring commercial organization due to nature or scope.
    IstkaufmannCommercial,

    /// Optional merchant (Kannkaufmann - §2 HGB)
    ///
    /// Small businesses that voluntarily register in the commercial register.
    /// Typical for freelancers upgrading to merchant status.
    Kannkaufmann,

    /// Agricultural/forestry merchant (Kannkaufmann - §3 HGB)
    ///
    /// Agricultural or forestry businesses that register voluntarily.
    KannkaufmannAgricultural,

    /// Deemed merchant (Formkaufmann - §6 HGB)
    ///
    /// Acquires merchant status by legal form (AG, GmbH, KG, OHG).
    /// Commercial activity not required.
    Formkaufmann,

    /// Not a merchant (Kleingewerbe)
    ///
    /// Small business not requiring commercial organization.
    /// Subject to civil law (BGB) rather than commercial law (HGB).
    NonMerchant,
}

// =============================================================================
// General Partnership (OHG - Offene Handelsgesellschaft)
// =============================================================================

/// General Partnership per §105-160 HGB
///
/// OHG is a partnership where all partners have unlimited personal liability
/// for partnership debts.
///
/// # Legal Requirements
///
/// - **Minimum partners**: 2 (§105 Abs. 1 HGB)
/// - **Liability**: Unlimited and joint for all partners (§128 HGB)
/// - **Management**: Each partner has management authority (§114 HGB)
/// - **Profit sharing**: Equal unless otherwise agreed (§121 HGB)
/// - **Commercial register**: Registration required (§106 HGB)
/// - **Partnership name**: Must include "OHG" or "offene Handelsgesellschaft" (§19 HGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OHG {
    /// Partnership name (Firma) - must include "OHG" suffix (§19 HGB)
    pub partnership_name: String,

    /// Registered office (Sitz) - German city
    pub registered_office: String,

    /// Business purpose (Unternehmensgegenstand)
    pub business_purpose: String,

    /// Partners (Gesellschafter) - minimum 2 required (§105 Abs. 1 HGB)
    ///
    /// All partners have unlimited liability (§128 HGB)
    pub partners: Vec<Partner>,

    /// Partnership agreement date (Gesellschaftsvertrag)
    pub formation_date: Option<DateTime<Utc>>,

    /// Fiscal year end
    pub fiscal_year_end: Option<FiscalYearEnd>,

    /// Unlimited liability flag (always true for OHG)
    pub unlimited_liability: bool,
}

/// Partner in OHG
///
/// Each partner has:
/// - Unlimited personal liability (§128 HGB)
/// - Management authority (§114 HGB)
/// - Right to represent the partnership (§125 HGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Partner {
    /// Partner name (natural person or legal entity)
    pub name: String,

    /// Address (Anschrift)
    pub address: String,

    /// Capital contribution (Einlage) - optional for OHG
    pub contribution: Option<Capital>,

    /// Contribution paid
    pub contribution_paid: Option<Capital>,

    /// Partner type (natural person or legal entity)
    pub partner_type: PartnerType,

    /// Management authority (Geschäftsführungsbefugnis)
    ///
    /// Default: true (§114 HGB - each partner has management rights)
    pub has_management_authority: bool,

    /// Representation authority (Vertretungsbefugnis)
    ///
    /// Default: true (§125 HGB - each partner can represent)
    pub has_representation_authority: bool,
}

/// Partner type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnerType {
    /// Natural person (natürliche Person)
    NaturalPerson,

    /// Legal entity (juristische Person) - e.g., GmbH as partner
    LegalEntity,
}

// =============================================================================
// Limited Partnership (KG - Kommanditgesellschaft)
// =============================================================================

/// Limited Partnership per §161-177a HGB
///
/// KG has two types of partners with different liability:
/// - **General partners (Komplementäre)**: Unlimited liability
/// - **Limited partners (Kommanditisten)**: Limited to contribution amount
///
/// # Legal Requirements
///
/// - **Minimum partners**: 1 general partner + 1 limited partner (§161 Abs. 1 HGB)
/// - **General partners**: Unlimited liability like OHG (§161 Abs. 2 HGB)
/// - **Limited partners**: Liability limited to agreed amount (§171 HGB)
/// - **Management**: Only general partners (§164 HGB)
/// - **Commercial register**: Registration required with liability limits (§162 HGB)
/// - **Partnership name**: Must include "KG" or "Kommanditgesellschaft" (§19 HGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KG {
    /// Partnership name (Firma) - must include "KG" suffix (§19 HGB)
    pub partnership_name: String,

    /// Registered office (Sitz) - German city
    pub registered_office: String,

    /// Business purpose (Unternehmensgegenstand)
    pub business_purpose: String,

    /// General partners (Komplementäre) - minimum 1 required
    ///
    /// Have unlimited liability and management authority (§161 Abs. 2, §164 HGB)
    pub general_partners: Vec<Partner>,

    /// Limited partners (Kommanditisten) - minimum 1 required
    ///
    /// Liability limited to contribution amount (§171 HGB)
    pub limited_partners: Vec<LimitedPartner>,

    /// Partnership agreement date (Gesellschaftsvertrag)
    pub formation_date: Option<DateTime<Utc>>,

    /// Fiscal year end
    pub fiscal_year_end: Option<FiscalYearEnd>,
}

/// Limited partner in KG (Kommanditist)
///
/// Key characteristics:
/// - Liability limited to agreed amount (Haftsumme) per §171 HGB
/// - No management authority (§164 HGB)
/// - Limited representation rights (§170 HGB)
/// - Must be registered in commercial register with liability limit (§162 HGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitedPartner {
    /// Limited partner name
    pub name: String,

    /// Address (Anschrift)
    pub address: String,

    /// Liability limit (Haftsumme) per §171 HGB
    ///
    /// Amount up to which partner is personally liable.
    /// Once contributed, liability is reduced/eliminated (§171 Abs. 1 HGB).
    pub liability_limit: Capital,

    /// Contribution paid (geleistete Einlage)
    ///
    /// Amount actually contributed by limited partner.
    pub contribution_paid: Capital,

    /// Partner type (typically natural person, but can be legal entity)
    pub partner_type: PartnerType,

    /// Special representation rights (außerordentliche Vertretungsbefugnis)
    ///
    /// Default: false (§170 HGB - limited partners generally cannot represent)
    pub has_special_representation: bool,
}

// =============================================================================
// GmbH & Co. KG (Hybrid Structure)
// =============================================================================

/// GmbH & Co. KG - Hybrid partnership structure
///
/// Special form of KG where the general partner (Komplementär) is a GmbH,
/// combining limited liability of GmbH with tax advantages of KG.
///
/// # Structure
///
/// - **General partner**: GmbH (manages the KG, unlimited liability)
/// - **Limited partners**: Natural persons or legal entities (limited liability)
/// - **Tax treatment**: Transparent taxation like partnership
/// - **Liability**: GmbH has unlimited liability, but GmbH itself has limited liability
///
/// # Legal Basis
///
/// Not explicitly regulated in HGB but recognized by courts and practice.
/// Treated as KG under §161 HGB with GmbH as Komplementär.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GmbHCoKG {
    /// Partnership name - must include "GmbH & Co. KG"
    pub partnership_name: String,

    /// Registered office (Sitz)
    pub registered_office: String,

    /// Business purpose (Unternehmensgegenstand)
    pub business_purpose: String,

    /// GmbH serving as general partner (Komplementär-GmbH)
    ///
    /// This GmbH has unlimited liability for KG debts, but the GmbH
    /// shareholders have limited liability.
    pub gmbh_general_partner: GmbHPartner,

    /// Limited partners (Kommanditisten)
    pub limited_partners: Vec<LimitedPartner>,

    /// Formation date
    pub formation_date: Option<DateTime<Utc>>,

    /// Fiscal year end
    pub fiscal_year_end: Option<FiscalYearEnd>,
}

/// GmbH serving as general partner in GmbH & Co. KG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GmbHPartner {
    /// GmbH company name (must include "GmbH")
    pub company_name: String,

    /// GmbH registered office
    pub registered_office: String,

    /// GmbH managing directors (Geschäftsführer)
    ///
    /// These individuals manage both the GmbH and the KG.
    pub managing_directors: Vec<String>,

    /// GmbH share capital
    pub share_capital: Capital,
}

// =============================================================================
// Supporting Types
// =============================================================================

/// Fiscal year end (Geschäftsjahresende)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalYearEnd {
    /// Month (1-12)
    pub month: u8,

    /// Day (1-31, depending on month)
    pub day: u8,
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merchant_type_enum() {
        let merchant = MerchantType::IstkaufmannCommercial;
        let non_merchant = MerchantType::NonMerchant;

        assert_ne!(merchant, non_merchant);
    }

    #[test]
    fn test_partner_creation() {
        let partner = Partner {
            name: "Max Mustermann".to_string(),
            address: "Berlin".to_string(),
            contribution: Some(Capital::from_euros(10_000)),
            contribution_paid: Some(Capital::from_euros(10_000)),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        };

        assert_eq!(partner.name, "Max Mustermann");
        assert!(partner.has_management_authority);
    }

    #[test]
    fn test_ohg_creation() {
        let ohg = OHG {
            partnership_name: "Mustermann & Schmidt OHG".to_string(),
            registered_office: "Berlin".to_string(),
            business_purpose: "Softwareentwicklung".to_string(),
            partners: vec![],
            formation_date: None,
            fiscal_year_end: None,
            unlimited_liability: true,
        };

        assert_eq!(ohg.partnership_name, "Mustermann & Schmidt OHG");
        assert!(ohg.unlimited_liability);
    }

    #[test]
    fn test_limited_partner_creation() {
        let limited_partner = LimitedPartner {
            name: "Anna Schmidt".to_string(),
            address: "Hamburg".to_string(),
            liability_limit: Capital::from_euros(50_000),
            contribution_paid: Capital::from_euros(50_000),
            partner_type: PartnerType::NaturalPerson,
            has_special_representation: false,
        };

        assert_eq!(limited_partner.liability_limit.to_euros(), 50_000.0);
        assert_eq!(limited_partner.contribution_paid.to_euros(), 50_000.0);
        assert!(!limited_partner.has_special_representation);
    }

    #[test]
    fn test_kg_creation() {
        let kg = KG {
            partnership_name: "Tech Ventures KG".to_string(),
            registered_office: "München".to_string(),
            business_purpose: "IT-Beratung".to_string(),
            general_partners: vec![],
            limited_partners: vec![],
            formation_date: None,
            fiscal_year_end: None,
        };

        assert_eq!(kg.partnership_name, "Tech Ventures KG");
    }

    #[test]
    fn test_fiscal_year_end() {
        let fye = FiscalYearEnd { month: 12, day: 31 };

        assert_eq!(fye.month, 12);
        assert_eq!(fye.day, 31);
    }

    #[test]
    fn test_partner_type_serialization() {
        let natural = PartnerType::NaturalPerson;
        let legal = PartnerType::LegalEntity;

        assert_ne!(natural, legal);
    }

    #[test]
    fn test_gmbh_partner_creation() {
        let gmbh_partner = GmbHPartner {
            company_name: "Verwaltungs GmbH".to_string(),
            registered_office: "Berlin".to_string(),
            managing_directors: vec!["Max Mustermann".to_string()],
            share_capital: Capital::from_euros(25_000),
        };

        assert_eq!(gmbh_partner.company_name, "Verwaltungs GmbH");
        assert_eq!(gmbh_partner.managing_directors.len(), 1);
    }
}
