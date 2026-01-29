//! Civil Code Module (民法典)
//!
//! # 中华人民共和国民法典 / Civil Code of the PRC
//!
//! Implements the Civil Code effective January 1, 2021, which unified and superseded
//! previous separate civil laws.
//!
//! ## Structure (七编结构)
//!
//! The Civil Code consists of 7 books with 1,260 articles:
//!
//! ### Book I: General Provisions (总则编, Articles 1-204)
//!
//! - Basic principles of civil law
//! - Natural persons, legal persons, unincorporated organizations
//! - Civil rights and civil juristic acts
//! - Agency (代理)
//! - Civil liability
//! - Limitation periods
//!
//! ### Book II: Property Rights (物权编, Articles 205-462)
//!
//! - Ownership (所有权)
//! - Usufruct (用益物权)
//! - Security interests (担保物权)
//! - Possession (占有)
//!
//! ### Book III: Contracts (合同编, Articles 463-988)
//!
//! - General provisions on contracts
//! - Formation, performance, modification, termination
//! - 29 specific contract types (sale, lease, loan, etc.)
//! - Quasi-contracts
//!
//! ### Book IV: Personality Rights (人格权编, Articles 989-1039)
//!
//! - Right to life, body, health
//! - Right to name, image, reputation, honor
//! - Right to privacy (隐私权)
//! - Personal information protection (个人信息保护)
//!
//! ### Book V: Marriage and Family (婚姻家庭编, Articles 1040-1118)
//!
//! - Marriage (婚姻)
//! - Parent-child relationships (亲子关系)
//! - Adoption (收养)
//! - Support and maintenance obligations
//!
//! ### Book VI: Succession (继承编, Articles 1119-1163)
//!
//! - Testamentary succession (遗嘱继承)
//! - Intestate succession (法定继承)
//! - Bequests and agreements
//! - Partition of estate
//!
//! ### Book VII: Tort Liability (侵权责任编, Articles 1164-1258)
//!
//! - General provisions on tort liability
//! - Specific tort types (medical malpractice, traffic accidents, products liability, etc.)
//! - Environmental pollution liability
//! - Liability for harm caused by highly dangerous activities
//!
//! ## Key Principles (基本原则)
//!
//! 1. **Equality (平等原则)** - Equal status of civil subjects
//! 2. **Voluntariness (自愿原则)** - Freedom of will in civil activities
//! 3. **Fairness (公平原则)** - Fair determination of rights and obligations
//! 4. **Good Faith (诚信原则)** - Honesty and keeping promises
//! 5. **Lawfulness (守法原则)** - Compliance with laws and public morals
//! 6. **Green Principle (绿色原则)** - Resource conservation and environmental protection
//!
//! ## Legal Capacity (民事行为能力)
//!
//! | Age | Capacity | Description |
//! |-----|----------|-------------|
//! | 0-7 | 无民事行为能力 | No capacity |
//! | 8-17 | 限制民事行为能力 | Limited capacity |
//! | 18+ | 完全民事行为能力 | Full capacity |
//!
//! Exception: 16-17 year olds with independent income have full capacity.
//!
//! ## Statute of Limitations (诉讼时效)
//!
//! - **General**: 3 years from knowledge of harm and tortfeasor (Article 188)
//! - **Maximum**: 20 years from occurrence of harm (Article 188)
//! - **Specific periods**: Certain claims have specific limitation periods
//!
//! ## Civil Liability Methods (民事责任承担方式, Article 179)
//!
//! 1. Cessation of infringement (停止侵害)
//! 2. Removal of obstruction (排除妨碍)
//! 3. Elimination of danger (消除危险)
//! 4. Return of property (返还财产)
//! 5. Restoration to original condition (恢复原状)
//! 6. Repair, rework, or replacement (修理、重作、更换)
//! 7. Continued performance (继续履行)
//! 8. Compensation for losses (赔偿损失)
//! 9. Payment of liquidated damages (支付违约金)
//! 10. Elimination of effects, restoration of reputation (消除影响、恢复名誉)
//! 11. Apology (赔礼道歉)
//!
//! ## Historical Context
//!
//! The Civil Code unified and replaced:
//! - General Principles of Civil Law (1986)
//! - Marriage Law (1980, 2001 amendment)
//! - Succession Law (1985)
//! - Adoption Law (1991, 1998 amendment)
//! - Guarantee Law (1995)
//! - Contract Law (1999)
//! - Property Law (2007)
//! - Tort Liability Law (2009)
//!
//! ## References
//!
//! - 《中华人民共和国民法典》(2020年5月28日通过，2021年1月1日施行)
//! - Supreme People's Court interpretations on Civil Code application

#![allow(missing_docs)]

pub mod contracts;
pub mod general_provisions;
pub mod marriage_family;
pub mod personality_rights;
pub mod property_rights;
pub mod succession;
pub mod tort_liability;

pub use contracts::*;
pub use general_provisions::*;
pub use marriage_family::*;
pub use personality_rights::*;
pub use property_rights::*;
pub use succession::*;
pub use tort_liability::*;
