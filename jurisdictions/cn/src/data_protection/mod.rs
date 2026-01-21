//! Personal Information Protection Law (PIPL) Module
//!
//! # 个人信息保护法 / Personal Information Protection Law
//!
//! Implements the Personal Information Protection Law of the PRC (中华人民共和国个人信息保护法),
//! effective November 1, 2021.
//!
//! ## Key Concepts
//!
//! - **个人信息 (Personal Information)**: Information relating to identified or identifiable natural persons
//! - **敏感个人信息 (Sensitive PI)**: Special categories requiring explicit consent
//! - **个人信息处理者 (PI Handler)**: Equivalent to GDPR's data controller
//! - **数据出境 (Cross-border Transfer)**: Subject to security assessment/certification
//!
//! ## Legal Basis for Processing (Article 13)
//!
//! 1. Individual consent
//! 2. Contract necessity
//! 3. Legal obligation
//! 4. Public health emergency
//! 5. Public interest (news reporting, supervision)
//! 6. Publicly available information
//! 7. Other circumstances prescribed by law
//!
//! ## References
//!
//! - 《中华人民共和国个人信息保护法》(2021)
//! - 《个人信息出境标准合同办法》
//! - CAC 《数据出境安全评估办法》

pub mod cross_border;
pub mod error;
pub mod types;
pub mod validator;

pub use cross_border::*;
pub use error::*;
pub use types::*;
pub use validator::*;
