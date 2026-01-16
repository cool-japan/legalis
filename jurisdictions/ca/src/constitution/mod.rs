//! Canada Constitutional Law Module
//!
//! This module provides comprehensive analysis of Canadian constitutional law,
//! including the Canadian Charter of Rights and Freedoms and the division of
//! powers between federal and provincial governments.
//!
//! # Overview
//!
//! Canadian constitutional law is based on:
//! - **Constitution Act, 1867**: Division of powers (ss.91-92)
//! - **Constitution Act, 1982**: Charter of Rights and Freedoms
//! - **Unwritten constitutional principles**: Federalism, democracy, constitutionalism,
//!   rule of law, protection of minorities
//!
//! # Key Concepts
//!
//! ## Charter of Rights and Freedoms
//!
//! The Charter protects fundamental rights:
//! - Fundamental freedoms (s.2)
//! - Democratic rights (ss.3-5)
//! - Mobility rights (s.6)
//! - Legal rights (ss.7-14)
//! - Equality rights (s.15)
//! - Language rights (ss.16-23)
//!
//! Rights may be limited under s.1 if the limitation is "demonstrably justified
//! in a free and democratic society" (Oakes test).
//!
//! ## Division of Powers
//!
//! - **Section 91**: Federal powers (criminal law, banking, trade, etc.)
//! - **Section 92**: Provincial powers (property, civil rights, local matters, etc.)
//! - **POGG**: Federal residual power for matters of national concern
//!
//! # Key Cases
//!
//! - **R v Oakes** [1986] 1 SCR 103 - s.1 justification test
//! - **Haida Nation v BC** [2004] 3 SCR 511 - Duty to consult
//! - **Tsilhqot'in Nation v BC** [2014] 2 SCR 256 - Aboriginal title
//! - **Reference re Secession of Quebec** [1998] 2 SCR 217 - Constitutional principles
//! - **Carter v Canada** [2015] 1 SCR 331 - Medical assistance in dying
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_ca::constitution::{
//!     CharterAnalyzer, CharterClaimFacts, CharterRight, GovernmentAction,
//! };
//!
//! let facts = CharterClaimFacts {
//!     claimant: "Individual".to_string(),
//!     right_claimed: CharterRight::FreedomOfExpression,
//!     government_action: GovernmentAction::Legislation {
//!         name: "Advertising Act".to_string(),
//!         section: "s.5".to_string(),
//!     },
//!     alleged_infringement: "Bans commercial speech".to_string(),
//!     evidence: vec!["Advertising prohibited".to_string()],
//!     section_1_asserted: true,
//! };
//!
//! let result = CharterAnalyzer::analyze(&facts);
//! println!("Infringement: {}", result.infringement_found);
//! println!("Justified: {:?}", result.section_1_justified);
//! ```

#![allow(missing_docs)]

pub mod charter;
pub mod division;
pub mod error;
pub mod types;

// Re-export types
pub use types::{
    // Aboriginal Rights
    AboriginalRight,
    // Charter
    CharterRight,
    // Cases
    ConstitutionalCase,
    ConstitutionalDoctrine,
    // Division of Powers
    FederalPower,
    HeadOfPower,
    MinimalImpairment,
    OakesTest,
    PithAndSubstance,
    PressAndSubstantial,
    ProportionalityAnalysis,
    ProportionalityStrictoSensu,
    ProvincialPower,
    RationalConnection,
};

// Re-export charter analysis
pub use charter::{
    CharterAnalyzer, CharterClaimFacts, CharterClaimResult, CharterRemedy, GovernmentAction,
    OakesAnalyzer,
};

// Re-export division analysis
pub use division::{
    ConflictType, ConflictingLaw, DivisionAnalyzer, DivisionFacts, DivisionResult, EnactingBody,
    IjiAnalysis, ParamountcyAnalysis, PoggAnalysis, PoggAnalyzer, PoggBranch,
};

// Re-export errors
pub use error::{ConstitutionalError, ConstitutionalResult};

// ============================================================================
// Integration with legalis-core
// ============================================================================

use legalis_core::{Effect, Statute, StatuteBuilder};

/// Create Charter statute for legalis-core integration
pub fn create_charter_statute() -> Statute {
    StatuteBuilder::new()
        .id("ca-charter-1982")
        .title("Canadian Charter of Rights and Freedoms")
        .effect(Effect::grant("Guarantees fundamental rights and freedoms"))
        .jurisdiction("CA")
        .build()
        .expect("Charter statute should build successfully")
}

/// Create Constitution Act, 1867 statute
pub fn create_constitution_1867_statute() -> Statute {
    StatuteBuilder::new()
        .id("ca-constitution-1867")
        .title("Constitution Act, 1867")
        .effect(Effect::grant(
            "Establishes federal structure and division of powers",
        ))
        .jurisdiction("CA")
        .build()
        .expect("Constitution 1867 statute should build successfully")
}

/// Create Constitution Act, 1982 statute
pub fn create_constitution_1982_statute() -> Statute {
    StatuteBuilder::new()
        .id("ca-constitution-1982")
        .title("Constitution Act, 1982")
        .effect(Effect::grant("Patriates constitution and enacts Charter"))
        .jurisdiction("CA")
        .build()
        .expect("Constitution 1982 statute should build successfully")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_charter_statute() {
        let statute = create_charter_statute();
        assert_eq!(statute.id, "ca-charter-1982");
    }

    #[test]
    fn test_create_constitution_1867() {
        let statute = create_constitution_1867_statute();
        assert_eq!(statute.id, "ca-constitution-1867");
    }

    #[test]
    fn test_charter_right() {
        let right = CharterRight::LifeLibertySecurityOfPerson;
        assert_eq!(right.section(), "7");
    }

    #[test]
    fn test_federal_power() {
        let power = FederalPower::CriminalLaw;
        assert_eq!(power.section(), "91(27)");
    }
}
