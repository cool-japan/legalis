//! Antitrust Law Module (Sherman Act, Clayton Act, FTC Act)
//!
//! # Key Antitrust Laws
//!
//! ## Sherman Antitrust Act of 1890
//!
//! ### Section 1: Anticompetitive Agreements
//!
//! "Every contract, combination in the form of trust or otherwise, or conspiracy,
//! in restraint of trade or commerce among the several States, or with foreign nations,
//! is declared to be illegal."
//!
//! **Per Se Illegal** (no inquiry into reasonableness):
//! - **Price fixing**: Horizontal competitors agree on prices
//! - **Bid rigging**: Competitors coordinate bids in procurement
//! - **Market allocation**: Competitors divide markets or customers
//! - **Group boycott**: Competitors agree to exclude competitor
//!
//! **Rule of Reason** (weighs pro-competitive vs anti-competitive effects):
//! - Most vertical restraints (e.g., resale price maintenance post-Leegin)
//! - Ancillary restraints (reasonably necessary to achieve legitimate purpose)
//! - Joint ventures, standard-setting organizations
//!
//! ### Section 2: Monopolization
//!
//! "Every person who shall monopolize, or attempt to monopolize, or combine or conspire
//! with any other person or persons, to monopolize any part of the trade or commerce
//! among the several States, or with foreign nations, shall be deemed guilty of a felony."
//!
//! **Elements**:
//! 1. **Possession of monopoly power** in relevant market (usually >65% market share)
//! 2. **Willful acquisition or maintenance** of that power through exclusionary conduct
//! 3. **Not through superior skill, foresight, or industry**
//!
//! **Exclusionary Conduct Examples**:
//! - Predatory pricing (pricing below cost to drive out competitors)
//! - Refusal to deal with essential input
//! - Exclusive dealing arrangements
//! - Tying (conditioning sale of one product on purchase of another)
//!
//! ## Clayton Antitrust Act of 1914
//!
//! ### Section 7: Anticompetitive Mergers
//!
//! "No person shall acquire... the whole or any part of the stock or assets of another person
//! where the effect... may be substantially to lessen competition, or to tend to create a monopoly."
//!
//! **Merger Review Process**:
//! 1. **Market definition** (product and geographic market)
//! 2. **Market concentration** (HHI - Herfindahl-Hirschman Index)
//! 3. **Competitive effects analysis** (unilateral, coordinated, vertical)
//! 4. **Entry analysis** (barriers to entry)
//! 5. **Efficiencies defense** (merger-specific efficiencies)
//!
//! **HHI Thresholds** (2023 Merger Guidelines):
//! - **HHI < 1,800**: Unconcentrated (unlikely to challenge)
//! - **1,800 ≤ HHI < 2,500**: Moderately concentrated (potential concern if Δ HHI > 100)
//! - **HHI ≥ 2,500**: Highly concentrated (likely challenge if Δ HHI > 200)
//!
//! ### Section 2: Robinson-Patman Act (Price Discrimination)
//!
//! Prohibits price discrimination that harms competition (limited enforcement).
//!
//! ### Section 3: Exclusive Dealing and Tying
//!
//! Prohibits tying and exclusive dealing "where the effect may be to substantially
//! lessen competition or tend to create a monopoly."
//!
//! ## Federal Trade Commission Act of 1914
//!
//! ### Section 5: Unfair Methods of Competition
//!
//! "Unfair methods of competition in or affecting commerce, and unfair or deceptive
//! acts or practices in or affecting commerce, are hereby declared unlawful."
//!
//! Broader than Sherman Act - FTC can challenge conduct that violates "spirit" of
//! antitrust laws even if not violation of Sherman/Clayton Acts.
//!
//! ## Hart-Scott-Rodino Antitrust Improvements Act of 1976 (HSR Act)
//!
//! Requires pre-merger notification to FTC and DOJ for large transactions.
//!
//! **Filing Thresholds** (2024, adjusted annually):
//! - **Size of Transaction**: > $111.4 million (automatic filing)
//! - **Size of Transaction + Size of Persons**: > $445.5M transaction AND one party > $222.7M,
//!   other > $22.3M
//!
//! **Waiting Period**:
//! - **15 days** for cash tender offers
//! - **30 days** for all other transactions
//! - **Early termination** possible if agencies clear quickly
//! - **Second Request** extends investigation (no time limit)
//!
//! **Filing Fees** (tiered based on transaction size):
//! - < $161.5M: $45,000
//! - $161.5M - $500M: $125,000
//! - $500M - $1B: $280,000
//! - $1B - $2B: $400,000
//! - $2B - $5B: $800,000
//! - ≥ $5B: $2,250,000
//!
//! # Landmark Cases
//!
//! - *Standard Oil v. United States* (1911): Breakup of Standard Oil, rule of reason
//! - *United States v. Aluminum Co. of America (Alcoa)* (1945): 90% market share = monopoly
//! - *Brown Shoe v. United States* (1962): Vertical merger blocked
//! - *Continental T.V. v. GTE Sylvania* (1977): Rule of reason for vertical restraints
//! - *Leegin Creative Leather v. PSKS* (2007): Resale price maintenance rule of reason
//! - *Ohio v. American Express* (2018): Two-sided markets, burden of proof
//! - *United States v. Microsoft* (2001): Tying, exclusive dealing, monopoly maintenance
//! - *United States v. AT&T/Time Warner* (2018): Vertical merger approved
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_us::antitrust::*;
//!
//! // Analyze merger
//! let merger = MergerAnalysis {
//!     acquirer: "MegaCorp".to_string(),
//!     target: "SmallCo".to_string(),
//!     transaction_value: 2_000_000_000.0,
//!     relevant_market: "Industrial Widgets".to_string(),
//!     pre_merger_hhi: 2200.0,
//!     post_merger_hhi: 2800.0,
//!     hhi_delta: 600.0,
//!     concentration_level: ConcentrationLevel::HighlyConcentrated,
//!     hsr_filing_required: true,
//!     competitive_effects: vec![CompetitiveEffect::UnilateralEffects],
//!     efficiencies: vec![],
//!     likely_anticompetitive: true,
//! };
//!
//! assert_eq!(merger.competitive_concern_level(), CompetitiveConcern::High);
//!
//! // Check HSR filing requirement
//! let filing_required = HsrFiling::is_required(
//!     2_000_000_000.0,  // transaction value
//!     5_000_000_000.0,  // acquirer size
//!     500_000_000.0,    // target size
//! );
//!
//! assert!(filing_required);
//! ```

pub mod types;

pub use types::*;
