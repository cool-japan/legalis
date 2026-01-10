//! Consumer Protection (Sale of Goods Act + Consumer Protection Fair Trading Act)
//!
//! This module provides type-safe implementations of Singapore's consumer protection framework,
//! covering both the Sale of Goods Act (Cap. 393) and the Consumer Protection (Fair Trading) Act (Cap. 52A).
//!
//! ## Key Legislation
//!
//! ### Sale of Goods Act (Cap. 393)
//! - **Section 13**: Implied condition - goods must correspond to description
//! - **Section 14(2)**: Implied condition - merchantable quality (seller in business)
//! - **Section 14(3)**: Implied condition - fitness for particular purpose
//! - **Section 15**: Sale by sample - bulk corresponds to sample
//!
//! ### Consumer Protection (Fair Trading) Act (Cap. 52A)
//! - **Section 4**: Prohibition of false or misleading representation
//! - **Section 5**: Prohibition of unconscionable conduct
//! - **Section 6**: Prohibition of bait advertising
//! - **Section 7**: Prohibition of harassment or coercion
//! - **Section 7A**: Pyramid selling schemes
//!
//! ### Lemon Law (Consumer Protection (Fair Trading) Act Amendment 2012)
//! - Applies to defective goods within **6 months** of delivery
//! - Remedies: Repair, replacement, price reduction, or refund
//! - Consumer must give supplier reasonable opportunity to remedy
//!
//! ### Small Claims Tribunals Act
//! - Handles disputes up to **SGD 20,000** (or SGD 30,000 with consent)
//! - Covers goods/services, motor vehicle accidents, residential property damage
//! - Fast, low-cost alternative to civil courts
//!
//! ## Common Scenarios
//!
//! ### 1. Sale of Defective Goods
//! ```rust
//! use legalis_sg::consumer::*;
//!
//! // Create a sale of goods contract
//! let mut sale = SaleOfGoods::new(
//!     "s001",
//!     true,  // Seller is in business
//!     "Washing machine"
//! );
//!
//! // Report a defect discovered within 6 months
//! sale.report_defect("Motor fails to start");
//!
//! // Validate - will detect Lemon Law violation
//! match validate_sale_of_goods(&sale) {
//!     Err(ConsumerError::DefectDiscovered { description }) => {
//!         println!("Lemon Law applies: {}", description);
//!         // Consumer entitled to: repair, replacement, refund, price reduction
//!     }
//!     _ => {}
//! }
//!
//! // Check which implied terms apply
//! let terms = validate_implied_terms(&sale).unwrap();
//! assert!(terms.contains(&ImpliedTerm::MerchantableQuality));
//! ```
//!
//! ### 2. Detecting Unfair Trading Practices
//! ```rust
//! use legalis_sg::consumer::*;
//!
//! // Create a consumer contract
//! let contract = ConsumerContract::new(
//!     "c001",
//!     "Dodgy Deals Pte Ltd",
//!     "John Tan",
//!     TransactionType::Services,
//!     250_000,  // SGD 2,500
//!     "Guaranteed miracle weight loss - 100% effective!"
//! );
//!
//! // Detect unfair practices
//! let practices = detect_unfair_practices(&contract);
//! for practice in &practices {
//!     println!("{}: {} ({})",
//!         practice.practice_type.statute_reference(),
//!         practice.description,
//!         practice.severity
//!     );
//! }
//! ```
//!
//! ### 3. Warranty Validation
//! ```rust
//! use legalis_sg::consumer::*;
//!
//! // Create warranty terms
//! let mut warranty = WarrantyTerms::new(
//!     365,  // 1 year
//!     WarrantyType::Manufacturer,
//!     "Defects in materials and workmanship"
//! );
//!
//! warranty.add_exclusion("Damage from misuse");
//! warranty.add_exclusion("Normal wear and tear");
//!
//! // Validate warranty (100 days since purchase)
//! match validate_warranty(&warranty, 100) {
//!     Ok(()) => println!("Warranty still valid"),
//!     Err(ConsumerError::WarrantyExpired { days_ago }) => {
//!         println!("Warranty expired {} days ago", days_ago);
//!     }
//!     _ => {}
//! }
//! ```
//!
//! ### 4. Contract Risk Assessment
//! ```rust
//! use legalis_sg::consumer::*;
//!
//! // Create contract with terms
//! let mut contract = ConsumerContract::new(
//!     "c002",
//!     "Renovator Pte Ltd",
//!     "Mary Lim",
//!     TransactionType::Services,
//!     1_800_000,  // SGD 18,000
//!     "Home renovation"
//! );
//!
//! // Add a potentially unfair term
//! let mut term = ContractTerm::new(
//!     "t1",
//!     "Company shall not be liable for any damages whatsoever",
//!     TermCategory::LiabilityLimitation
//! );
//! term.mark_unfair("Overly broad liability exclusion");
//! contract.add_term(term);
//!
//! // Detect unfair practices
//! let practices = detect_unfair_practices(&contract);
//! for practice in practices {
//!     contract.add_unfair_practice(practice);
//! }
//!
//! // Calculate risk score
//! contract.calculate_risk_score();
//! println!("Contract risk score: {}/100", contract.risk_score);
//! ```
//!
//! ## Implementation Notes
//!
//! ### Implied Terms (SOGA)
//! - **s. 13** applies to ALL sales
//! - **s. 14(2)** applies only when seller acts "in the course of business"
//! - **s. 14(3)** applies when buyer makes purpose known and relies on seller's skill/judgment
//! - **s. 15** applies only when sale is explicitly "by sample"
//!
//! ### Unfair Practices Detection (CPFTA)
//! - Keyword-based detection for common violations
//! - Risk scoring: Severity 1-10, scaled to 50 max per practice
//! - Multiple practices can accumulate (capped at 100)
//!
//! ### Small Claims Tribunal
//! - SGD 20,000 limit without consent
//! - SGD 30,000 limit with both parties' consent
//! - Use `contract.is_sct_eligible()` to check eligibility
//!
//! ### Lemon Law
//! - 6-month window from delivery date
//! - Consumer must give supplier **opportunity to remedy**
//! - Use `sale.is_lemon_law_applicable()` to check eligibility
//!
//! ## Statute References
//!
//! All errors include bilingual messages (English + Chinese) and statute references:
//! - `SOGA s. 13` - Sale of Goods Act, section 13
//! - `SOGA s. 14(2)` - Sale of Goods Act, section 14(2)
//! - `CPFTA s. 4` - Consumer Protection (Fair Trading) Act, section 4
//! - `Lemon Law` - Consumer Protection (Fair Trading) Act Amendment 2012
//!
//! ## Related Modules
//! - [`types`] - Core consumer protection types
//! - [`error`] - Consumer protection error types
//! - [`validator`] - Validation functions for contracts and sales

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
