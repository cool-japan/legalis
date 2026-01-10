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
//! ### 3. Other Notable Uniform Acts
//!
//! - **Uniform Trust Code (UTC)**: Trust law standardization
//! - **Uniform Probate Code (UPC)**: Estate administration
//! - **Uniform Arbitration Act (UAA)**: Arbitration procedures
//! - **Uniform Electronic Transactions Act (UETA)**: E-commerce
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
pub mod ucc;
pub mod upa;

pub use adoption_status::{AdoptionComparison, AdoptionStatus, UniformActComparator};
pub use ucc::{UCCAdoption, UCCArticle, UCCTracker, UCCVersion};
pub use upa::{PartnershipActVersion, UPAAdoption, UPATracker};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_loads() {
        // Smoke test to ensure module compiles
        assert!(true);
    }
}
