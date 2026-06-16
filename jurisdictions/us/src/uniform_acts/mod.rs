//! US Uniform Acts Tracker
//!
//! This module tracks the adoption status of uniform laws across US states,
//! focusing primarily on commercial law standardization efforts by the
//! Uniform Law Commission (ULC).
//!
//! ## What are Uniform Acts?
//!
//! Uniform Acts are model statutes drafted by the Uniform Law Commission
//! (formerly National Conference of Commissioners on Uniform State Laws)
//! to promote consistency across state laws in areas where uniformity is
//! beneficial but federal preemption is undesirable.
//!
//! ## Key Uniform Acts
//!
//! ### 1. Uniform Commercial Code (UCC)
//!
//! The most successful uniform law in US history. Governs commercial
//! transactions including:
//! - **Article 1**: General Provisions
//! - **Article 2**: Sales of Goods
//! - **Article 2A**: Leases
//! - **Article 3**: Negotiable Instruments
//! - **Article 4**: Bank Deposits
//! - **Article 4A**: Funds Transfers
//! - **Article 5**: Letters of Credit
//! - **Article 6**: Bulk Transfers (repealed in most states)
//! - **Article 7**: Documents of Title
//! - **Article 8**: Investment Securities
//! - **Article 9**: Secured Transactions
//!
//! **Adoption Status**: All 50 states + DC have adopted the UCC, but with
//! varying amendments and versions.
//!
//! **Louisiana Exception**: Louisiana (Civil Law state) adopted Articles 1, 3, 4,
//! 5, 7, 8, 9 but NOT Article 2 (conflicts with Louisiana's sale of goods law
//! in the Civil Code).
//!
//! ### 2. Uniform Partnership Act (UPA) / Revised Uniform Partnership Act (RUPA)
//!
//! Governs partnership formation and operation.
//! - **UPA (1914)**: Original version
//! - **RUPA (1997)**: Modern revision adopted by majority of states
//!
//! ### 3. Other Tracked Uniform Acts
//!
//! Each of the following is modeled with model-act metadata, key provisions,
//! state adoption tracking, and substantive validators:
//!
//! - **Uniform Trust Code (UTC, 2000)** [`utc`]: Trust law standardization
//! - **Uniform Probate Code (UPC, 1969/1990)** [`upc`]: Wills, intestacy, estate administration
//! - **Revised Uniform Limited Liability Company Act (RULLCA, 2006)** [`ullca`]: LLC governance
//! - **Revised Uniform Arbitration Act (RUAA, 2000)** [`uaa`]: Arbitration procedures
//! - **Uniform Electronic Transactions Act (UETA, 1999)** [`ueta`]: Legal recognition of
//!   electronic records, signatures, and contracts (49 jurisdictions; New York non-uniform)
//!
//! ## Why Uniform Acts Matter for Legalis-RS
//!
//! 1. **Interstate Commerce**: Companies need predictable rules across states
//! 2. **Version Tracking**: States adopt different versions at different times
//! 3. **State Variations**: Even when adopted, states make local amendments
//! 4. **Choice of Law**: UCC has special choice of law rules (e.g., § 1-301)
//!
//! ## Integration with Choice of Law
//!
//! Uniform Acts often include their own choice of law provisions:
//! - UCC § 1-301: Parties may choose applicable law for contracts
//! - UCC § 9-301: Special rules for secured transactions
//!
//! This module tracks which version/variation each state has adopted,
//! enabling accurate analysis when different states' UCC provisions conflict.

pub mod adoption_status;
pub mod error;
pub mod model_act;
pub mod uaa;
pub mod ucc;
pub mod ueta;
pub mod ullca;
pub mod upa;
pub mod upc;
pub mod utc;

pub use adoption_status::{AdoptionComparison, AdoptionStatus, UniformActComparator};
pub use error::{Result, UniformActError};
pub use model_act::{DraftingBody, ModelActMetadata, US_JURISDICTIONS};
pub use uaa::{
    ArbitrationActVersion, ArbitrationAgreement, RuaaSection, UaaAdoption, UaaTracker,
    VacaturGround, arbitration_agreement_issues, validate_arbitration_agreement,
};
pub use ucc::{UCCAdoption, UCCArticle, UCCTracker, UCCVersion};
pub use ueta::{
    ElectronicRecord, SignatureMethod, UetaAdoption, UetaSection, UetaTracker,
    electronic_record_issues, signature_attributable, validate_electronic_record,
};
pub use ullca::{
    LlcFormation, LlcManagementStructure, RullcaSection, UllcaAdoption, UllcaTracker, UllcaVersion,
    default_management_structure, llc_formation_issues, validate_llc_formation,
};
pub use upa::{PartnershipActVersion, UPAAdoption, UPATracker};
pub use upc::{
    UpcAdoption, UpcArticle, UpcSection, UpcTracker, WillExecution, validate_will_execution,
    will_execution_issues,
};
pub use utc::{
    TrustCreation, UtcAdoption, UtcArticle, UtcSection, UtcTracker, trust_creation_issues,
    validate_trust_creation,
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_loads() {
        // Smoke test to ensure module compiles
    }
}
