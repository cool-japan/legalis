//! Corporate Law Types
//!
//! Types for Australian corporate law under Corporations Act 2001.

use serde::{Deserialize, Serialize};

// ============================================================================
// Company Types
// ============================================================================

/// Company type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Proprietary company limited by shares
    ProprietaryLimited,
    /// Public company limited by shares
    PublicLimited,
    /// Company limited by guarantee
    LimitedByGuarantee,
    /// Unlimited company
    Unlimited,
    /// No liability company (mining)
    NoLiability,
}

/// Company size classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanySize {
    /// Small proprietary company
    SmallProprietary,
    /// Large proprietary company
    LargeProprietary,
    /// Listed public company
    ListedPublic,
    /// Unlisted public company
    UnlistedPublic,
}

// ============================================================================
// Directors Duties
// ============================================================================

/// Directors duty type (Part 2D.1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorsDuty {
    /// Care and diligence (s.180)
    CareAndDiligence,
    /// Good faith and proper purpose (s.181)
    GoodFaithProperPurpose,
    /// Not to improperly use position (s.182)
    NotImproperlyUsePosition,
    /// Not to improperly use information (s.183)
    NotImproperlyUseInformation,
    /// To prevent insolvent trading (s.588G)
    PreventInsolventTrading,
    /// Related party transactions (Ch 2E)
    RelatedPartyTransactions,
}

impl DirectorsDuty {
    /// Get Corporations Act section
    pub fn section(&self) -> &'static str {
        match self {
            Self::CareAndDiligence => "s.180",
            Self::GoodFaithProperPurpose => "s.181",
            Self::NotImproperlyUsePosition => "s.182",
            Self::NotImproperlyUseInformation => "s.183",
            Self::PreventInsolventTrading => "s.588G",
            Self::RelatedPartyTransactions => "Ch 2E",
        }
    }
}

/// Business judgment rule element (s.180(2))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessJudgmentElement {
    /// Made in good faith for proper purpose
    GoodFaithProperPurpose,
    /// No material personal interest
    NoPersonalInterest,
    /// Informed themselves appropriately
    InformedAppropriately,
    /// Rationally believed in company's best interests
    RationalBelief,
}

// ============================================================================
// Insolvency
// ============================================================================

/// Type of external administration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalAdministration {
    /// Voluntary administration (Part 5.3A)
    VoluntaryAdministration,
    /// Liquidation - members voluntary
    MembersVoluntaryLiquidation,
    /// Liquidation - creditors voluntary
    CreditorsVoluntaryLiquidation,
    /// Liquidation - court ordered
    CourtOrderedLiquidation,
    /// Receivership
    Receivership,
    /// Deed of company arrangement
    DeedOfCompanyArrangement,
    /// Small business restructuring
    SmallBusinessRestructuring,
}

/// Insolvent trading defence (s.588H)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsolventTradingDefence {
    /// Reasonable grounds to expect solvency (s.588H(2))
    ReasonableExpectation,
    /// No reasonable grounds to suspect insolvency (s.588H(3))
    NoSuspicion,
    /// Reasonable steps to prevent debt (s.588H(4))
    ReasonableSteps,
    /// Illness defence (s.588H(5))
    Illness,
    /// Safe harbour (s.588GA)
    SafeHarbour,
}

/// Priority of claims in liquidation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidationPriority {
    /// Costs of winding up
    CostsOfWindingUp,
    /// Employee entitlements
    EmployeeEntitlements,
    /// Circulating security interests
    CirculatingSecurityInterests,
    /// Secured creditors (non-circulating)
    SecuredCreditors,
    /// Unsecured creditors
    UnsecuredCreditors,
    /// Subordinated debt
    SubordinatedDebt,
    /// Shareholders
    Shareholders,
}

// ============================================================================
// ASIC Regulation
// ============================================================================

/// ASIC enforcement power
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsicPower {
    /// Investigation
    Investigation,
    /// Disqualification order
    DisqualificationOrder,
    /// Civil penalty proceeding
    CivilPenalty,
    /// Banning order
    BanningOrder,
    /// Enforceable undertaking
    EnforceableUndertaking,
    /// Infringement notice
    InfringementNotice,
}

// ============================================================================
// Takeovers
// ============================================================================

/// Takeover threshold
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TakeoverThreshold {
    /// 20% voting power threshold
    TwentyPercent,
    /// 30% compulsory acquisition threshold
    ThirtyPercent,
    /// 90% compulsory acquisition threshold
    NinetyPercent,
}

/// Takeover method
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TakeoverMethod {
    /// Off-market bid
    OffMarketBid,
    /// On-market bid
    OnMarketBid,
    /// Scheme of arrangement
    SchemeOfArrangement,
    /// Creeping acquisition (3% in 6 months)
    CreepingAcquisition,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Australian corporate law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateCase {
    /// Case name
    pub name: String,
    /// Citation
    pub citation: String,
    /// Key principle
    pub principle: String,
}

impl CorporateCase {
    /// ASIC v Healey (2011) - Directors duty of care
    pub fn asic_v_healey() -> Self {
        Self {
            name: "ASIC v Healey".to_string(),
            citation: "[2011] FCA 717".to_string(),
            principle: "Directors must read and understand financial statements".to_string(),
        }
    }

    /// Shafron v ASIC (2012) - Officers duty
    pub fn shafron() -> Self {
        Self {
            name: "Shafron v ASIC".to_string(),
            citation: "(2012) 247 CLR 465".to_string(),
            principle: "General counsel can be officer; duty of care applies".to_string(),
        }
    }

    /// ASIC v Adler (2002) - Duty to act in good faith
    pub fn asic_v_adler() -> Self {
        Self {
            name: "ASIC v Adler".to_string(),
            citation: "(2002) 168 FLR 253".to_string(),
            principle: "Directors must act in good faith and for proper purpose".to_string(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directors_duty_sections() {
        assert_eq!(DirectorsDuty::CareAndDiligence.section(), "s.180");
        assert_eq!(DirectorsDuty::PreventInsolventTrading.section(), "s.588G");
    }

    #[test]
    fn test_asic_v_healey() {
        let case = CorporateCase::asic_v_healey();
        assert!(case.citation.contains("FCA"));
    }
}
