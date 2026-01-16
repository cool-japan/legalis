//! Federal-State Boundary Analysis
//!
//! This module analyzes the relationship between federal and state law in the
//! United States, including preemption analysis and Commerce Clause constraints.
//!
//! ## Constitutional Framework
//!
//! The US Constitution establishes a **dual sovereignty system** where both
//! federal and state governments have their own spheres of authority.
//!
//! ### Supremacy Clause (Article VI)
//!
//! > "This Constitution, and the Laws of the United States which shall be made in
//! > Pursuance thereof... shall be the supreme Law of the Land."
//!
//! When federal and state law conflict, federal law prevails through **preemption**.
//!
//! ### Commerce Clause (Article I, Section 8)
//!
//! > "Congress shall have Power... To regulate Commerce with foreign Nations, and
//! > among the several States, and with the Indian Tribes."
//!
//! This grants Congress broad authority to regulate interstate commerce, but also
//! limits state power through the **Dormant Commerce Clause** doctrine.
//!
//! ## Three Types of Preemption
//!
//! ### 1. Express Preemption
//!
//! Congress explicitly states that federal law preempts state law.
//!
//! **Example**: Federal Aviation Administration Authorization Act (FAAAA)
//! ```text
//! "a State... may not enact or enforce a law... related to a price, route, or
//! service of any motor carrier..."
//! ```
//!
//! **Analysis**: Look for statutory text with words like "preempt," "supersede,"
//! "exclusively," "only," etc.
//!
//! ### 2. Implied Field Preemption
//!
//! Federal regulation is so pervasive that it occupies the entire field, leaving
//! no room for state law.
//!
//! **Example**: Immigration law - federal government has exclusive control.
//!
//! **Test**:
//! - Is the federal regulatory scheme comprehensive and detailed?
//! - Did Congress intend to occupy the entire field?
//! - Is the subject matter traditionally federal (foreign affairs, immigration)?
//!
//! ### 3. Conflict Preemption
//!
//! State law conflicts with federal law, either by:
//! - Making compliance with both impossible ("impossibility preemption")
//! - Obstructing federal objectives ("obstacle preemption")
//!
//! **Example**: State tort law requiring warnings that conflict with FDA-approved
//! labeling creates impossibility.
//!
//! ## Presumption Against Preemption
//!
//! Courts presume Congress does NOT intend to preempt state law when:
//! - State law involves traditional state police powers (health, safety, morals)
//! - No clear congressional intent to preempt
//! - State law is a historic field of state regulation
//!
//! **Key Cases**:
//! - *Medtronic, Inc. v. Lohr* (1996) - Medical device preemption
//! - *Rice v. Santa Fe Elevator Corp.* (1331) - Field preemption test
//! - *Geier v. American Honda Motor Co.* (2000) - Obstacle preemption
//!
//! ## Dormant Commerce Clause
//!
//! Even without federal legislation, states cannot:
//! - Discriminate against interstate commerce
//! - Impose undue burdens on interstate commerce
//!
//! **Exceptions**:
//! - Market participant exception (state acting as buyer/seller)
//! - Congressional authorization
//!
//! ## Integration with Legalis-RS
//!
//! This module enables:
//! 1. **Conflict Detection**: Identify when federal law preempts state law
//! 2. **Jurisdiction Hierarchy**: "US" (federal) vs "US-CA" (California state)
//! 3. **Choice of Law**: Federal law always controls when applicable
//! 4. **Compliance Analysis**: Check if state regulation survives preemption

pub mod commerce_clause;
pub mod preemption;

pub use commerce_clause::{
    CommerceClauseAnalysis, CommerceClauseResult, DormantCommerceClauseTest,
};
pub use preemption::{
    ConflictPreemptionType, FieldPreemptionAnalysis, PreemptionAnalysis, PreemptionResult,
    PreemptionType,
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_loads() {
        // Smoke test to ensure module compiles
    }
}
