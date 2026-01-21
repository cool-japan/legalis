//! Canada Employment Law Module
//!
//! This module provides comprehensive modeling of Canadian employment law,
//! covering both federal (Canada Labour Code) and provincial employment standards.
//!
//! ## Key Areas
//!
//! - **Employment Status**: Employee vs. independent contractor (Sagaz test)
//! - **Employment Standards**: Provincial ESA minimums
//! - **Termination**: Reasonable notice (Bardal factors), just cause (McKinley)
//! - **Human Rights**: Protected grounds, duty to accommodate (Meiorin)
//! - **Wrongful Dismissal**: Damages, mitigation, bad faith (Keays v Honda)
//!
//! ## Dual Jurisdiction System
//!
//! Canada has a dual employment jurisdiction system:
//! - **Federal**: Canada Labour Code (banking, telecoms, transportation, etc.)
//! - **Provincial**: Employment Standards Acts (most employees)
//!
//! ## Key Cases
//!
//! - **Bardal v Globe & Mail** \[1960\]: Reasonable notice factors
//! - **671122 Ontario v Sagaz** \[2001\] SCC 59: Employee vs. contractor test
//! - **McKinley v BC Tel** \[2001\] SCC 38: Contextual approach to just cause
//! - **Wallace v United Grain Growers** \[1997\]: Bad faith in dismissal
//! - **Keays v Honda** \[2008\] SCC 39: Aggravated damages framework
//! - **Potter v NB Legal Aid** \[2015\] SCC 10: Constructive dismissal
//! - **BC v BCGSEU (Meiorin)** \[1999\] SCC 48: BFOR test

mod error;
mod termination;
mod types;

pub use error::{EmploymentError, EmploymentResult};
pub use termination::{
    JustCauseAnalyzer, JustCauseFacts, JustCauseResult, ReasonableNoticeAnalyzer,
    ReasonableNoticeFacts, ReasonableNoticeResult, WrongfulDismissalAnalyzer,
    WrongfulDismissalFacts, WrongfulDismissalResult,
};
pub use types::{
    AccommodationType, BardalFactor, DiscriminationType, DutyToAccommodate, EmploymentArea,
    EmploymentCase, EmploymentJurisdiction, EmploymentStandards, EmploymentStatus, FederalIndustry,
    HardshipFactor, JustCauseGround, MitigationRequirement, ProtectedGround, SagazFactor,
    StandardType, TerminationType, WrongfulDismissalDamages,
};

// Re-export legalis-core types
pub use legalis_core::{Effect, EffectType, Statute};

use crate::common::Province;

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Employment Standards Act statute for a province
pub fn create_employment_standards_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => (
            "ON-ESA-2000",
            "Employment Standards Act, 2000, SO 2000, c 41",
        ),
        Province::BritishColumbia => ("BC-ESA-1996", "Employment Standards Act, RSBC 1996, c 113"),
        Province::Alberta => ("AB-ESC-2000", "Employment Standards Code, RSA 2000, c E-9"),
        Province::Quebec => ("QC-LSA", "Act respecting labour standards, CQLR c N-1.1"),
        Province::Manitoba => ("MB-ESC", "Employment Standards Code, CCSM c E110"),
        Province::Saskatchewan => ("SK-SEA", "Saskatchewan Employment Act, SS 2013, c S-15.1"),
        Province::NovaScotia => ("NS-LSC", "Labour Standards Code, RSNS 1989, c 246"),
        Province::NewBrunswick => ("NB-ESA", "Employment Standards Act, SNB 1982, c E-7.2"),
        Province::NewfoundlandLabrador => ("NL-LSA", "Labour Standards Act, RSNL 1990, c L-2"),
        Province::PrinceEdwardIsland => ("PE-ESA", "Employment Standards Act, RSPEI 1988, c E-6.2"),
        _ => ("ESA", "Employment Standards Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Obligation,
            "Employers must comply with minimum employment standards including wages, \
             hours of work, overtime, vacation, leaves of absence, and termination notice",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Canada Labour Code statute (federal)
pub fn create_canada_labour_code() -> Statute {
    Statute::new(
        "CLC",
        "Canada Labour Code, RSC 1985, c L-2",
        Effect::new(
            EffectType::Obligation,
            "Regulates employment in federally regulated industries including \
             banking, telecommunications, transportation, and Crown corporations",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Human Rights Code statute for a province
pub fn create_human_rights_code(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-HRC", "Human Rights Code, RSO 1990, c H.19"),
        Province::BritishColumbia => ("BC-HRC", "Human Rights Code, RSBC 1996, c 210"),
        Province::Alberta => ("AB-AHRA", "Alberta Human Rights Act, RSA 2000, c A-25.5"),
        Province::Quebec => (
            "QC-Charter",
            "Charter of Human Rights and Freedoms, CQLR c C-12",
        ),
        Province::Manitoba => ("MB-HRC", "Human Rights Code, CCSM c H175"),
        Province::Saskatchewan => (
            "SK-SHRA",
            "Saskatchewan Human Rights Code, 2018, SS 2018, c S-24.2",
        ),
        Province::NovaScotia => ("NS-HRA", "Human Rights Act, RSNS 1989, c 214"),
        Province::NewBrunswick => ("NB-HRA", "Human Rights Act, RSNB 2011, c 171"),
        Province::NewfoundlandLabrador => ("NL-HRA", "Human Rights Act, 2010, SNL 2010, c H-13.1"),
        Province::PrinceEdwardIsland => ("PE-HRA", "Human Rights Act, RSPEI 1988, c H-12"),
        _ => ("HRC", "Human Rights Code"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Prohibition,
            "Prohibits discrimination in employment on protected grounds including \
             race, sex, disability, religion, age, and family status",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Canadian Human Rights Act (federal)
pub fn create_canadian_human_rights_act() -> Statute {
    Statute::new(
        "CHRA",
        "Canadian Human Rights Act, RSC 1985, c H-6",
        Effect::new(
            EffectType::Prohibition,
            "Prohibits discrimination in federally regulated employment on \
             protected grounds including race, national or ethnic origin, colour, \
             religion, age, sex, sexual orientation, gender identity, marital status, \
             family status, genetic characteristics, disability, and pardoned conviction",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create statutes for employment law in a province
pub fn create_employment_statutes(province: &Province) -> Vec<Statute> {
    vec![
        create_employment_standards_act(province),
        create_human_rights_code(province),
    ]
}

/// Create federal employment statutes
pub fn create_federal_employment_statutes() -> Vec<Statute> {
    vec![
        create_canada_labour_code(),
        create_canadian_human_rights_act(),
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_employment_standards_act() {
        let statute = create_employment_standards_act(&Province::Ontario);
        assert!(statute.title.contains("Employment Standards Act, 2000"));
    }

    #[test]
    fn test_create_canada_labour_code() {
        let statute = create_canada_labour_code();
        assert!(statute.title.contains("Canada Labour Code"));
    }

    #[test]
    fn test_create_human_rights_code() {
        let statute = create_human_rights_code(&Province::Ontario);
        assert!(statute.title.contains("Human Rights Code"));
    }

    #[test]
    fn test_create_quebec_charter() {
        let statute = create_human_rights_code(&Province::Quebec);
        assert!(
            statute
                .title
                .contains("Charter of Human Rights and Freedoms")
        );
    }

    #[test]
    fn test_create_employment_statutes() {
        let statutes = create_employment_statutes(&Province::BritishColumbia);
        assert_eq!(statutes.len(), 2);
    }

    #[test]
    fn test_create_federal_employment_statutes() {
        let statutes = create_federal_employment_statutes();
        assert_eq!(statutes.len(), 2);
    }
}
