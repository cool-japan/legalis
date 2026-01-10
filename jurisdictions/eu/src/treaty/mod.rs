//! Treaty Framework - EU Primary Law (TFEU, TEU, Charter)
//!
//! This module provides a **skeleton implementation** of EU primary law, covering:
//! - **Four Freedoms** (Articles 28-66 TFEU)
//! - **Charter of Fundamental Rights**
//! - **CJEU Landmark Cases** (Van Gend en Loos, Costa v ENEL, etc.)
//!
//! ## Status: SKELETON IMPLEMENTATION
//!
//! This module provides type definitions and basic structure for EU treaty law.
//! Full validation logic and case law integration are planned for future releases.
//!
//! ## Four Freedoms (Articles 28-66 TFEU)
//!
//! The four fundamental freedoms that underpin the EU internal market:
//!
//! 1. **Free Movement of Goods** (Articles 28-37)
//!    - Article 34: Prohibition of quantitative restrictions
//!    - Article 36: Public policy exceptions
//!    - Cassis de Dijon principle (mutual recognition)
//!
//! 2. **Free Movement of Persons** (Articles 45-48)
//!    - Article 45: Free movement of workers
//!    - Citizens' Rights Directive
//!
//! 3. **Freedom to Provide Services** (Articles 56-62)
//!    - Article 56: Prohibition of restrictions on services
//!
//! 4. **Free Movement of Capital** (Articles 63-66)
//!    - Article 63: Prohibition of restrictions on capital movements
//!
//! ## Charter of Fundamental Rights
//!
//! Key articles relevant to digital rights and business:
//! - Article 7: Respect for private and family life
//! - Article 8: Protection of personal data
//! - Article 11: Freedom of expression and information
//! - Article 16: Freedom to conduct a business
//! - Article 47: Right to an effective remedy
//!
//! ## CJEU Landmark Cases
//!
//! Foundation cases establishing EU legal principles:
//! - **Van Gend en Loos** (C-26/62): Direct effect doctrine
//! - **Costa v ENEL** (C-6/64): Supremacy of EU law
//! - **Cassis de Dijon** (C-120/78): Mutual recognition
//! - **Francovich** (C-6/90, C-9/90): State liability

pub mod case_law;
pub mod charter;
pub mod four_freedoms;
pub mod types;

// Re-exports
pub use case_law::{CjeuCase, CjeuPrinciple, LandmarkCase};
pub use charter::{CharterArticle, FundamentalRight};
pub use four_freedoms::{FourFreedom, FreedomType, JustificationGround, Restriction};
pub use types::{TreatyArticle, TreatyProvision, TreatyType};
