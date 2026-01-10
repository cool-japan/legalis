// ! State Legislative Tracking
//!
//! This module tracks policy adoption and constitutional provisions across US states.
//!
//! ## Policy Topics Tracked
//!
//! - **Cannabis Legalization**: Recreational, medical, decriminalized, illegal status
//! - **Data Privacy Laws**: CCPA, VCDPA, CPA, CTDPA, UCPA, and other comprehensive privacy laws
//! - **Right to Repair**: Electronics and automotive repair laws
//!
//! ## Constitutional Provisions
//!
//! - **State Privacy Rights**: Explicit vs implicit constitutional protections
//! - **Initiative and Referendum**: Direct democracy powers
//! - **Constitutional Protections Beyond Federal Floor**: State-specific rights
//!
//! ## Example: Cannabis Legalization Status
//!
//! ```rust
//! use legalis_us::legislative::policy_tracker::{cannabis_status, CannabisStatus};
//!
//! let ca_status = cannabis_status("CA");
//! assert_eq!(ca_status, CannabisStatus::RecreationalLegal { year_enacted: 2016 });
//!
//! let tx_status = cannabis_status("TX");
//! assert_eq!(tx_status, CannabisStatus::MedicalOnly { year_enacted: 2015 });
//! ```
//!
//! ## Example: Data Privacy Laws
//!
//! ```rust
//! use legalis_us::legislative::policy_tracker::has_comprehensive_privacy_law;
//!
//! assert!(has_comprehensive_privacy_law("CA")); // CCPA
//! assert!(has_comprehensive_privacy_law("VA")); // VCDPA
//! assert!(!has_comprehensive_privacy_law("AL")); // No comprehensive law
//! ```

pub mod constitutional;
pub mod policy_tracker;

// Re-exports
pub use constitutional::{
    ConstitutionalPrivacyRight, DirectDemocracyPowers, InitiativeReferendumStatus,
    StateConstitutionalProvisions, constitutional_privacy_right, has_initiative_referendum,
    state_constitutional_provisions,
};

pub use policy_tracker::{
    CannabisStatus, DataPrivacyLaw, PolicyAdoptionTracker, RightToRepairStatus, cannabis_status,
    comprehensive_privacy_laws, has_comprehensive_privacy_law, right_to_repair_status,
    states_with_recreational_cannabis,
};
