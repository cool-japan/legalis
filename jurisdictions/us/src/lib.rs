//! United States Jurisdiction Support for Legalis-RS
//!
//! This module provides Common Law tort support for the United States legal system,
//! including:
//!
//! - **Restatement of Torts** (ALI) - Synthesized principles from case law
//! - **Famous tort cases** - Landmark precedents (Palsgraf, Donoghue, etc.)
//! - **Stare decisis** - Case law precedent system
//!
//! ## Common Law vs Civil Law
//!
//! The US legal system (inherited from English Common Law) differs fundamentally
//! from Civil Law systems (Japan, Germany, France) in how legal rules develop:
//!
//! ### Civil Law Approach (大陸法)
//!
//! ```text
//! Legislature
//!     ↓
//! Code/Statute (e.g., 民法709条, BGB §823, Code civil 1240)
//!     ↓
//! Courts apply statute to cases
//! ```
//!
//! ### Common Law Approach (英米法)
//!
//! ```text
//! Case 1 → Precedent A
//!     ↓
//! Case 2 cites Case 1 → Refines Precedent A
//!     ↓
//! Case 3 distinguishes → Exception to Precedent A
//!     ↓
//! Restatement synthesizes → § X: Rule A (non-binding)
//!     ↓
//! Case 4 adopts Restatement § X
//! ```
//!
//! ## Key Differences
//!
//! | Feature | Civil Law | Common Law |
//! |---------|-----------|------------|
//! | Primary Source | Statutes/Codes | Cases/Precedents |
//! | Court Role | Apply code | Make law |
//! | Reasoning | Deductive (code → case) | Analogical (case → case) |
//! | Binding Force | Statute text | Prior holdings (stare decisis) |
//! | Flexibility | Low (legislature must amend) | High (courts distinguish) |
//!
//! ## Why This Matters for Legalis-RS
//!
//! Civil Law modeling uses `Statute` objects (e.g., 民法709条).
//! Common Law modeling uses `Case` objects with `precedent_weight()`.
//!
//! The same tort concept appears differently:
//! - **Civil Law**: Article 709 (statute) → "intent OR negligence"
//! - **Common Law**: Palsgraf (case) → "duty to foreseeable plaintiff"
//!
//! We need both modeling approaches in Legalis-RS.

pub mod cases;
pub mod restatement;

// Re-export key functions for convenience
pub use cases::{
    donoghue_v_stevenson, garratt_v_dailey, palsgraf_v_long_island, vosburg_v_putney,
};
pub use restatement::{
    battery_as_statute, iied_as_statute, products_liability_as_statute, section_158_battery,
    section_402a_products_liability, section_46_iied,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restatement_functions_available() {
        // Verify Restatement sections are accessible
        let battery = section_158_battery();
        assert!(battery.name.contains("158"));

        let iied = section_46_iied();
        assert!(iied.name.contains("46"));

        let products = section_402a_products_liability();
        assert!(products.name.contains("402A"));
    }

    #[test]
    fn test_cases_available() {
        // Verify famous cases are accessible
        let palsgraf = palsgraf_v_long_island();
        assert_eq!(palsgraf.year, 1928);

        let donoghue = donoghue_v_stevenson();
        assert_eq!(donoghue.year, 1932);

        let garratt = garratt_v_dailey();
        assert_eq!(garratt.year, 1955);

        let vosburg = vosburg_v_putney();
        assert_eq!(vosburg.year, 1891);
    }

    #[test]
    fn test_statute_versions_available() {
        // Verify statute representations of Restatement sections
        let battery_statute = battery_as_statute();
        assert_eq!(battery_statute.jurisdiction, Some("US-RESTATEMENT".to_string()));

        let iied_statute = iied_as_statute();
        assert!(iied_statute.is_valid());

        let products_statute = products_liability_as_statute();
        assert_eq!(products_statute.version, 2); // Restatement (Second)
    }
}
