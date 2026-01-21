//! Australian Intellectual Property Law
//!
//! Comprehensive implementation of Australian IP law administered by IP Australia:
//! - Patents (Patents Act 1990)
//! - Trade Marks (Trade Marks Act 1995)
//! - Copyright (Copyright Act 1968)
//! - Designs (Designs Act 2003)
//!
//! ## Key Legislation
//!
//! ### Patents Act 1990 (Cth)
//!
//! Australian patent law setting out requirements for patentable inventions.
//!
//! - **Section 7**: Novelty and inventive step
//! - **Section 18**: Patentable inventions (manner of manufacture, novel, inventive step, useful)
//! - **Section 40**: Specification requirements
//! - **Section 117**: Infringement
//! - **Section 138**: Revocation
//!
//! Key cases:
//! - **NRDC v Commissioner of Patents (1959)**: Manner of manufacture test
//! - **Lockwood v Doric (2004)**: Inventive step analysis
//! - **D'Arcy v Myriad Genetics (2015)**: Isolated gene sequences not patentable
//!
//! ### Trade Marks Act 1995 (Cth)
//!
//! Australian trade mark registration and protection.
//!
//! - **Section 17**: What is a trade mark
//! - **Section 41**: Trade mark not distinguishing applicant's goods/services
//! - **Section 42**: Trade marks contrary to law or scandalous
//! - **Section 43**: Trade marks likely to deceive or cause confusion
//! - **Section 44**: Trade marks identical or substantially identical to registered marks
//! - **Section 120**: Infringement
//!
//! ### Copyright Act 1968 (Cth)
//!
//! Protection for original literary, dramatic, musical, and artistic works.
//!
//! - **Part III**: Original works (literary, dramatic, musical, artistic)
//! - **Part IV**: Subject matter other than works (films, sound recordings, broadcasts)
//! - **Section 31**: Nature of copyright
//! - **Section 36**: Infringement by doing acts in relation to work
//! - **Sections 40-43**: Fair dealing (research/study, criticism/review, parody/satire, news)
//!
//! Key cases:
//! - **IceTV v Nine Network (2009)**: Originality requires independent intellectual effort
//! - **Telstra v Phone Directories (2010)**: Compilations and originality
//!
//! ### Designs Act 2003 (Cth)
//!
//! Protection for visual appearance of products.
//!
//! - **Section 15**: Registrable designs
//! - **Section 16**: Designs that are new and distinctive
//! - **Section 71**: Infringement
//!
//! ## IP Australia
//!
//! IP Australia is the Australian Government agency responsible for:
//! - Patent examination and grant
//! - Trade mark registration
//! - Design registration
//! - Plant Breeder's Rights
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::intellectual_property::*;
//!
//! // Check patent validity under Australian law
//! let invention = PatentApplication {
//!     title: "Novel mining extraction method".to_string(),
//!     claims: vec![PatentClaim {
//!         number: 1,
//!         text: "A method of extracting minerals comprising...".to_string(),
//!         independent: true,
//!         depends_on: None,
//!     }],
//!     prior_art: vec![],
//!     filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     // ...
//! };
//!
//! let validity = check_patentability(&invention)?;
//! assert!(validity.is_novel);
//! assert!(validity.has_inventive_step);
//! assert!(validity.is_manner_of_manufacture); // Australian-specific test
//!
//! // Check copyright subsistence
//! let work = CopyrightWork {
//!     work_type: WorkType::LiteraryWork,
//!     title: "Technical Manual".to_string(),
//!     author: "Engineer Pty Ltd".to_string(),
//!     creation_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     // ...
//! };
//!
//! let subsistence = check_copyright_subsistence(&work)?;
//! ```

pub mod copyright;
pub mod designs;
pub mod error;
pub mod patents;
pub mod trademarks;
pub mod types;

// Re-exports
pub use copyright::{
    CopyrightDuration, CopyrightWork, FairDealingPurpose, MoralRights, PartIVSubject, WorkType,
    calculate_copyright_duration, check_copyright_subsistence, check_fair_dealing,
    check_infringement as check_copyright_infringement,
};
pub use designs::{
    Design, DesignApplication, DesignExamination, DesignInfringement, DesignType,
    check_design_infringement, check_design_validity, validate_design_application,
};
pub use error::{IpError, Result};
pub use patents::{
    IndustrialApplicability, InventiveStep, MannerOfManufacture, Novelty, Patent,
    PatentApplication, PatentClaim, PatentInfringement, Patentability,
    check_industrial_application, check_manner_of_manufacture, check_novelty, check_patentability,
    validate_patent_claim,
};
pub use trademarks::{
    AbsoluteGrounds, LikelihoodOfConfusion, RelativeGrounds, TradeMark, TradeMarkApplication,
    TradeMarkClass, TradeMarkInfringement, assess_likelihood_of_confusion, check_absolute_grounds,
    check_relative_grounds, validate_trademark_application,
};
pub use types::{
    GeographicScope, IpOwner, IpRight, IpRightType, LicenseType, OwnerType, PriorArt, PriorArtType,
    RegistrationStatus,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Patents Act 1990 statute
pub fn create_patents_act() -> Statute {
    Statute::new(
        "AU-PA-1990",
        "Patents Act 1990 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Grants exclusive rights to exploit inventions that are manner of manufacture, \
             novel, involve an inventive step, and are useful",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Trade Marks Act 1995 statute
pub fn create_trade_marks_act() -> Statute {
    Statute::new(
        "AU-TMA-1995",
        "Trade Marks Act 1995 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Grants exclusive rights to use registered trade marks to distinguish goods \
             and services in trade",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Copyright Act 1968 statute
pub fn create_copyright_act() -> Statute {
    Statute::new(
        "AU-CA-1968",
        "Copyright Act 1968 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Grants exclusive rights over original literary, dramatic, musical and artistic \
             works, and subject matter other than works",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Designs Act 2003 statute
pub fn create_designs_act() -> Statute {
    Statute::new(
        "AU-DA-2003",
        "Designs Act 2003 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Grants exclusive rights over the visual appearance of products that are \
             new and distinctive",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_patents_act() {
        let statute = create_patents_act();
        assert_eq!(statute.id, "AU-PA-1990");
        assert!(statute.title.contains("Patents Act"));
    }

    #[test]
    fn test_create_trade_marks_act() {
        let statute = create_trade_marks_act();
        assert_eq!(statute.id, "AU-TMA-1995");
        assert!(statute.title.contains("Trade Marks"));
    }

    #[test]
    fn test_create_copyright_act() {
        let statute = create_copyright_act();
        assert_eq!(statute.id, "AU-CA-1968");
        assert!(statute.title.contains("Copyright"));
    }

    #[test]
    fn test_create_designs_act() {
        let statute = create_designs_act();
        assert_eq!(statute.id, "AU-DA-2003");
        assert!(statute.title.contains("Designs"));
    }
}
