//! Thai Foreign Business Act (FBA) - พ.ร.บ. ประกอบธุรกิจของคนต่างด้าว พ.ศ. 2542
//!
//! Thailand's FBA (B.E. 2542 / 1999 CE) restricts foreign ownership in specified businesses.
//!
//! ## Three-Tier Restriction System
//!
//! - **List 1**: Prohibited (media, land, forestry)
//! - **List 2**: Cabinet approval required (weapons, transport)
//! - **List 3**: License required (retail, construction, legal)
//!
//! ## Exemptions
//!
//! - BOI-promoted investments
//! - ASEAN treaty benefits
//! - US Treaty of Amity (for US nationals)

mod error;
mod types;
mod validator;

pub use error::{FbaError, FbaResult};
pub use types::{
    BusinessActivity, BusinessRestrictionList, ForeignBusinessLicense, ForeignInvestor,
    ForeignOwnership, OwnershipStructure, TreatyExemption,
};
pub use validator::{
    FbaCompliance, get_fba_checklist, validate_fba_compliance, validate_foreign_ownership,
    validate_ownership_structure, validate_treaty_exemption,
};
