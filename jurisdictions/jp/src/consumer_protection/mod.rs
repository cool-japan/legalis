//! Consumer Protection Law Module (消費者保護法モジュール)
//!
//! This module provides comprehensive support for Japanese consumer protection law,
//! including the Consumer Contract Act (消費者契約法) and Specified Commercial
//! Transactions Act (特定商取引法).
//!
//! # Features
//!
//! - Consumer contract validation (消費者契約検証)
//! - Unfair terms detection with risk scoring (不当条項検出・リスク評価)
//! - Cooling-off period calculation and validation (クーリング・オフ)
//! - Rescission rights validation (取消権検証)
//! - Automatic clause risk analysis (条項リスク自動分析)
//! - Type-safe validation and error handling
//!
//! # Legal Framework
//!
//! ## Consumer Contract Act (消費者契約法 - Act No. 61 of 2000)
//!
//! The Consumer Contract Act protects consumers from unfair contract terms:
//!
//! **Article 8: Exemption Clauses (免責条項)**
//! - Article 8-1-1: Full exemption from liability is invalid
//! - Article 8-1-2/3: Partial exemptions for gross negligence are invalid
//!
//! **Article 9: Penalty/Cancellation Fee Limits (損害賠償額の予定等)**
//! - Penalties cannot exceed average damages
//! - Cancellation fees must be reasonable
//!
//! **Article 10: General Unfair Terms (消費者の利益を一方的に害する条項)**
//! - Clauses that unfairly disadvantage consumers are invalid
//! - Unreasonable burdens on consumers are prohibited
//!
//! **Article 4: Rescission Rights (取消権)**
//! - Article 4-1-1: Misrepresentation (不実告知)
//! - Article 4-1-2: Definite judgment on uncertain matters (断定的判断)
//! - Article 4-2: Non-disclosure of disadvantages (不利益事実の不告知)
//! - Article 4-3: Undue influence, threats, obstruction (困惑行為)
//!
//! **Article 7: Rescission Period (取消権の行使期間)**
//! - 6 months from knowledge of grounds
//! - 5 years from contract date (absolute limit)
//!
//! ## Specified Commercial Transactions Act (特定商取引法 - Act No. 57 of 1976)
//!
//! The SCTA regulates specific transaction types and provides cooling-off rights:
//!
//! **Transaction Types:**
//! - Door-to-door sales (訪問販売) - 8 days cooling-off
//! - Telemarketing (電話勧誘販売) - 8 days cooling-off
//! - Mail-order (通信販売) - No cooling-off (return policy varies)
//! - Multi-level marketing (連鎖販売取引) - 20 days cooling-off
//! - Business opportunity sales (業務提供誘引販売) - 20 days cooling-off
//!
//! **Cooling-Off (クーリング・オフ - Articles 9, 24, 40, 48, 58):**
//! - Unconditional cancellation right within specified period
//! - Period starts from receipt of contract documents
//! - No penalty or reason required
//!
//! # Examples
//!
//! ## Detecting Unfair Contract Terms
//!
//! ```rust
//! use legalis_jp::consumer_protection::*;
//!
//! let term_text = "当社は一切責任を負いません。"; // "We bear no responsibility"
//! let (risk_score, potentially_unfair, unfair_type) = detect_unfair_terms(term_text);
//!
//! assert!(risk_score >= 40); // High risk score
//! assert!(potentially_unfair);
//! assert_eq!(unfair_type, Some(UnfairTermType::FullExemption));
//! ```
//!
//! ## Checking Cooling-Off Period
//!
//! ```rust
//! use legalis_jp::consumer_protection::*;
//! use chrono::Utc;
//!
//! let transaction = SpecifiedCommercialTransaction {
//!     transaction_type: TransactionType::DoorToDoor,
//!     seller_name: "販売業者".to_string(),
//!     purchaser_name: "購入者".to_string(),
//!     contract_date: Utc::now(),
//!     document_receipt_date: Some(Utc::now()),
//!     contract_amount_jpy: 100_000,
//!     product_description: "商品説明".to_string(),
//!     payment_method: "現金".to_string(),
//!     cooling_off_notice_provided: true,
//! };
//!
//! assert_eq!(transaction.transaction_type.cooling_off_period_days(), 8);
//! assert!(transaction.is_within_cooling_off_period());
//! ```
//!
//! ## Validating Consumer Contract
//!
//! ```rust
//! use legalis_jp::consumer_protection::*;
//! use chrono::Utc;
//!
//! let contract = ConsumerContract {
//!     title: "サービス契約".to_string(),
//!     business_name: "事業者株式会社".to_string(),
//!     consumer_name: "消費者名".to_string(),
//!     contract_date: Utc::now(),
//!     contract_amount_jpy: 100_000,
//!     terms: vec![
//!         ContractTerm {
//!             term_number: 1,
//!             text: "通常の契約条項".to_string(),
//!             potentially_unfair: false,
//!             unfair_type: None,
//!             risk_score: 10,
//!         },
//!     ],
//!     cancellation_policy: None,
//!     penalty_clause: None,
//! };
//!
//! assert!(validate_consumer_contract(&contract).is_ok());
//! ```

pub mod ecommerce;
pub mod ecommerce_validator;
pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use ecommerce::*;
pub use ecommerce_validator::*;
pub use error::{ConsumerProtectionError, Result};
pub use types::*;
pub use validator::*;
