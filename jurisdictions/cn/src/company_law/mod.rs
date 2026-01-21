//! Company Law Module (公司法)
//!
//! # 中华人民共和国公司法 / Company Law of the PRC
//!
//! Implements the Company Law (2023 Revision) effective July 1, 2024.
//!
//! ## Company Types
//!
//! ### Limited Liability Company (有限责任公司)
//!
//! - 2-50 shareholders
//! - Shareholder liability limited to subscribed capital
//! - Equity transfer restricted (preemptive rights)
//!
//! ### Joint Stock Company (股份有限公司)
//!
//! - 2+ promoters (2023 revision)
//! - Shares divided into equal portions
//! - Can be listed on stock exchange
//!
//! ### Single-Shareholder LLC (一人有限责任公司)
//!
//! - One natural person or legal entity shareholder
//! - Must clearly state "one-person LLC" in registration
//! - Shareholder must prove separation of personal and company assets
//!
//! ## Capital Contribution
//!
//! ### Methods (Article 27)
//!
//! - Monetary: Cash contributions
//! - Physical assets: Must be valued
//! - Intellectual property: Must be valued
//! - Land use rights: Must be valued
//! - Equity/Debt claims: Subject to certain conditions
//!
//! ### Timeline
//!
//! - Subscribed capital: No minimum timeline but must be paid within business term
//! - 2023 revision: 5-year contribution period for new companies
//!
//! ## Corporate Governance
//!
//! ### Board of Directors (董事会)
//!
//! | Company Type | Directors |
//! |--------------|-----------|
//! | LLC | 0-13 (can use executive director) |
//! | JSC | 5-19 |
//!
//! Key duties:
//! - Execute shareholder resolutions
//! - Decide on business plans
//! - Prepare profit distribution proposals
//!
//! ### Supervisory Board (监事会)
//!
//! | Company Type | Supervisors |
//! |--------------|-------------|
//! | LLC | 1+ (can use single supervisor) |
//! | JSC | 3+ |
//!
//! - At least 1/3 employee representatives
//! - Cannot include directors or senior managers
//!
//! ### Shareholder Meeting (股东会/股东大会)
//!
//! #### Ordinary Resolution (>50%)
//! - Business policies
//! - Election of directors/supervisors
//! - Approval of annual reports
//!
//! #### Special Resolution (≥2/3)
//! - Amendment of articles
//! - Capital increase/decrease
//! - Merger, division, dissolution
//! - Change of company form
//!
//! ## Equity Transfer (LLC)
//!
//! ### Internal Transfer
//! - Free transfer between existing shareholders
//!
//! ### External Transfer (Article 71)
//! 1. Written notice to other shareholders
//! 2. 30-day response period
//! 3. Majority consent required
//! 4. Other shareholders have preemptive rights
//! 5. Same terms as proposed external transfer
//!
//! ## Director Duties
//!
//! ### Duty of Loyalty (忠实义务)
//! - No self-dealing without approval
//! - No competing business
//! - No misappropriation of company assets
//! - Keep confidential information
//!
//! ### Duty of Diligence (勤勉义务)
//! - Exercise care of prudent manager
//! - Act in company's best interest
//!
//! ## Director Disqualification (Article 146)
//!
//! Cannot serve as director:
//! - Criminal conviction for certain offenses
//! - Director of bankrupt company with personal liability (<3 years)
//! - Large personal debts unpaid
//! - Director of revoked company with personal liability (<3 years)
//!
//! ## Shareholder Rights
//!
//! - Dividend rights
//! - Preemptive rights (new share issuance)
//! - Right to information
//! - Voting rights
//! - Right to propose meeting
//! - Derivative action rights
//! - Exit rights (dissenting shareholders)
//!
//! ## Piercing Corporate Veil (Article 20)
//!
//! Shareholders may be jointly liable if:
//! - Assets commingled with company
//! - Company used to evade debts
//! - Abuse of limited liability
//! - No separate existence

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
