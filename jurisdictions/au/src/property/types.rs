//! Property Law Types
//!
//! Types for Australian property law analysis.

use serde::{Deserialize, Serialize};

// ============================================================================
// Land Tenure
// ============================================================================

/// Type of land tenure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandTenure {
    /// Freehold (fee simple)
    Freehold,
    /// Leasehold
    Leasehold,
    /// Crown land
    CrownLand,
    /// Native title
    NativeTitle,
    /// Strata title
    StrataTitle,
    /// Community title
    CommunityTitle,
}

/// Type of interest in land
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandInterest {
    /// Legal interest
    Legal,
    /// Equitable interest
    Equitable,
    /// Mere equity
    MereEquity,
}

// ============================================================================
// Torrens System
// ============================================================================

/// Torrens registration principles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TorrensPrinciple {
    /// Indefeasibility of title
    Indefeasibility,
    /// Curtain principle (register is conclusive)
    CurtainPrinciple,
    /// Mirror principle (register reflects all interests)
    MirrorPrinciple,
    /// Insurance principle (compensation for loss)
    InsurancePrinciple,
}

/// Exception to indefeasibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndefeasibilityException {
    /// Fraud (including agent fraud)
    Fraud,
    /// Prior certificate of title
    PriorCertificate,
    /// Registered proprietor previously in possession
    PreviousPossession,
    /// Short-term lease
    ShortTermLease,
    /// Easement by prescription
    EasementPrescription,
    /// Adverse possession (limited)
    AdversePossession,
    /// Error of boundaries
    BoundaryError,
}

/// Interest that overrides registration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverridingInterest {
    /// Easement acquired by prescription
    PrescriptiveEasement,
    /// Short-term lease (typically < 3 years)
    ShortLease,
    /// Interests of persons in actual occupation
    ActualOccupation,
}

// ============================================================================
// Estates and Interests
// ============================================================================

/// Type of estate
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Estate {
    /// Fee simple absolute
    FeeSimple,
    /// Fee tail (historical)
    FeeTail,
    /// Life estate
    LifeEstate,
    /// Leasehold estate
    Leasehold,
}

/// Type of easement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementType {
    /// Right of way
    RightOfWay,
    /// Right to light
    RightToLight,
    /// Right to support
    RightToSupport,
    /// Right to water
    RightToWater,
    /// Right to drainage
    Drainage,
    /// Profit a prendre
    ProfitAPrendre,
}

/// Type of covenant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CovenantType {
    /// Restrictive covenant
    Restrictive,
    /// Positive covenant
    Positive,
    /// Building covenant
    Building,
}

// ============================================================================
// Native Title
// ============================================================================

/// Native title right type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeTitleRight {
    /// Exclusive possession
    ExclusivePossession,
    /// Non-exclusive access
    NonExclusiveAccess,
    /// Hunting rights
    Hunting,
    /// Fishing rights
    Fishing,
    /// Gathering rights
    Gathering,
    /// Ceremonial rights
    Ceremonial,
    /// Right to negotiate
    RightToNegotiate,
}

/// Native title determination type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeterminationType {
    /// Consent determination
    Consent,
    /// Litigated determination
    Litigated,
    /// Combined
    Combined,
}

/// Future act type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutureActType {
    /// Primary production
    PrimaryProduction,
    /// Mining
    Mining,
    /// Infrastructure
    Infrastructure,
    /// Compulsory acquisition
    CompulsoryAcquisition,
}

// ============================================================================
// Strata/Community Title
// ============================================================================

/// Strata lot type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrataLotType {
    /// Residential lot
    Residential,
    /// Commercial lot
    Commercial,
    /// Utility lot
    Utility,
    /// Common property
    CommonProperty,
}

/// Owners corporation function
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnersCorpFunction {
    /// Management of common property
    ManageCommonProperty,
    /// Insurance
    Insurance,
    /// By-law enforcement
    ByLawEnforcement,
    /// Levies/contributions
    Levies,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Australian property law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyCase {
    /// Case name
    pub name: String,
    /// Citation
    pub citation: String,
    /// Key principle
    pub principle: String,
}

impl PropertyCase {
    /// Mabo v Queensland (No 2) (1992) - Native title
    pub fn mabo() -> Self {
        Self {
            name: "Mabo v Queensland (No 2)".to_string(),
            citation: "(1992) 175 CLR 1".to_string(),
            principle: "Recognition of native title - rejection of terra nullius".to_string(),
        }
    }

    /// Breskvar v Wall (1971) - Indefeasibility
    pub fn breskvar() -> Self {
        Self {
            name: "Breskvar v Wall".to_string(),
            citation: "(1971) 126 CLR 376".to_string(),
            principle: "Immediate indefeasibility upon registration".to_string(),
        }
    }

    /// Frazer v Walker (1967) - Paramountcy of register
    pub fn frazer() -> Self {
        Self {
            name: "Frazer v Walker".to_string(),
            citation: "[1967] 1 AC 569".to_string(),
            principle: "Registered title paramount even against prior equities".to_string(),
        }
    }

    /// Wik v Queensland (1996) - Pastoral leases and native title
    pub fn wik() -> Self {
        Self {
            name: "Wik Peoples v Queensland".to_string(),
            citation: "(1996) 187 CLR 1".to_string(),
            principle: "Pastoral leases do not necessarily extinguish native title".to_string(),
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
    fn test_mabo_case() {
        let case = PropertyCase::mabo();
        assert!(case.citation.contains("175 CLR"));
    }

    #[test]
    fn test_torrens_principles() {
        let principles = [
            TorrensPrinciple::Indefeasibility,
            TorrensPrinciple::CurtainPrinciple,
            TorrensPrinciple::MirrorPrinciple,
            TorrensPrinciple::InsurancePrinciple,
        ];
        assert_eq!(principles.len(), 4);
    }

    #[test]
    fn test_native_title_rights() {
        let rights = [
            NativeTitleRight::ExclusivePossession,
            NativeTitleRight::Hunting,
            NativeTitleRight::Fishing,
        ];
        assert!(!rights.is_empty());
    }
}
