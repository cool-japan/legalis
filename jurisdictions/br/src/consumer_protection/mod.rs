//! # Consumer Protection - CDC (Código de Defesa do Consumidor)
//!
//! Lei nº 8.078/1990 - Brazil's comprehensive consumer protection law.
//!
//! ## Overview
//!
//! The CDC is one of the most advanced consumer protection laws globally,
//! establishing the consumer as a **vulnerable party** deserving special protection.
//!
//! ## Key Principles
//!
//! | Principle | Article | Description |
//! |-----------|---------|-------------|
//! | Vulnerability | Art. 4 | Consumer presumed vulnerable in market relations |
//! | Strict Liability | Arts. 12-14 | Provider liable regardless of fault |
//! | Contract Interpretation | Art. 47 | Ambiguities favor consumer |
//! | Good Faith | Art. 4, III | Objective good faith required |
//! | Information | Art. 6, III | Right to clear, adequate information |
//!
//! ## Consumer Rights (Art. 6)
//!
//! 1. Protection of life, health, and safety
//! 2. Education about consumption
//! 3. Adequate information about products/services
//! 4. Protection against misleading advertising
//! 5. Contract modification for disproportionate clauses
//! 6. Effective prevention and reparation of damages
//! 7. Access to justice
//! 8. Facilitation of rights defense (burden reversal)
//! 9. Quality of public services
//!
//! ## Abusive Clauses (Art. 51)
//!
//! The CDC lists 16 types of **null and void** abusive clauses:
//!
//! | Type | Example |
//! |------|---------|
//! | Liability Exclusion | "Not responsible for defects" |
//! | Mandatory Arbitration | Forced arbitration clause |
//! | Unilateral Modification | Provider can change terms unilaterally |
//! | Burden Reversal | Consumer proves non-defect |
//!
//! ## Product/Service Liability
//!
//! ### Product Defects (Art. 12)
//!
//! - **Design Defect**: Flaw in product conception
//! - **Manufacturing Defect**: Flaw in production
//! - **Information Defect**: Inadequate warnings/instructions
//!
//! ### Service Defects (Art. 14)
//!
//! - **Quality Defect**: Service doesn't meet expectations
//! - **Safety Defect**: Service creates unreasonable risk
//!
//! ## Withdrawal Right (Art. 49)
//!
//! ```text
//! Distance/doorstep purchases → 7 calendar days to withdraw → Full refund
//! ```
//!
//! ## PROCON Enforcement
//!
//! Consumer protection agencies (PROCONs) in each state enforce the CDC:
//! - Administrative complaints
//! - Mediation
//! - Fines up to R$ 10 million
//!
//! ## Usage Example
//!
//! ```rust
//! use legalis_br::consumer_protection::*;
//!
//! // Check if clause is abusive
//! let clause = AbusiveClause::new(
//!     AbusiveClauseType::LiabilityExclusion,
//!     "Provider not responsible for any defects".to_string(),
//! );
//! assert!(clause.is_null_and_void());
//!
//! // Validate withdrawal right
//! let withdrawal = WithdrawalRight::new(5); // 5 days since purchase
//! assert!(withdrawal.is_valid());
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
