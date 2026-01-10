//! French family law (Code civil Book I).
//!
//! This module provides structured representations of French family law, including:
//! - **Marriage** (Mariage): Requirements, procedures, nullity (Articles 143-180)
//! - **Divorce** (Divorce): 4 types - mutual consent, acceptance, alteration, fault (Articles 229-247)
//! - **Property Regimes** (Régimes matrimoniaux): Communauté, Séparation de biens (Articles 1387-1536)
//! - **Parental Relations** (Relations parentales): Parental authority, maintenance (Articles 371-373)
//! - **PACS**: Civil solidarity pacts (Articles 515-1 to 515-7)
//!
//! # Comparison with Japanese Law
//!
//! ## Marriage Age
//! - France: 18 for all (Article 144, since 2006)
//! - Japan: 18 for all (民法731条, reformed 2022)
//!
//! ## Same-Sex Marriage
//! - France: Legal since 2013 (Article 143)
//! - Japan: Not recognized (婚姻は異性間のみ)
//!
//! ## PACS vs Japanese Partnership
//! - France: PACS provides legal framework (Article 515-1, since 1999)
//! - Japan: Partnership certificates (パートナーシップ制度) vary by municipality
//!
//! ## Divorce Types
//! - France: 4 types (mutual/acceptance/alteration/fault)
//! - Japan: 2 types (協議離婚 + 裁判離婚)
//!   - Mutual consent divorce in Japan doesn't require court (90% of divorces)
//!
//! ## Property Regime
//! - France: Communauté réduite aux acquêts (default since 1966)
//! - Japan: Separate property (別産制) with division on divorce (財産分与)
//!
//! ## Parental Authority
//! - France: Joint parental authority default (Article 373-2, since 1993)
//! - Japan: Single custody (単独親権) - reform debate ongoing
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_fr::family::{Marriage, Person, MaritalStatus, validate_marriage_conditions};
//!
//! let mut marriage = Marriage::new(
//!     Person::new("Alice Dupont".to_string(), 25, Nationality::French, MaritalStatus::Single),
//!     Person::new("Bob Martin".to_string(), 27, Nationality::French, MaritalStatus::Single),
//! )
//! .with_consent([true, true])
//! .with_banns_published(true);
//!
//! assert!(validate_marriage_conditions(&marriage).is_ok());
//! ```

pub mod divorce;
pub mod error;
pub mod marriage;
pub mod property_regime;
pub mod types;
pub mod validator;

pub use divorce::{
    article229, article230, article233, article237, article242, article247,
    validate_acceptance_principle_divorce, validate_definitive_alteration_divorce,
    validate_divorce_proceedings, validate_fault_divorce, validate_mutual_consent_divorce,
};
pub use error::{BilingualString, FamilyLawError, FamilyLawResult};
pub use marriage::{
    article143, article144, article146, article146_1, article147, article161, article165,
    article180, check_oppositions, validate_banns_publication, validate_consent,
    validate_marriage_conditions, validate_minimum_age, validate_no_bigamy,
    validate_no_consanguinity, validate_personal_presence,
};
pub use property_regime::{
    article1387, article1400, article1401, article1404, article1536, is_default_regime,
    regime_name_en, regime_name_fr, validate_pacs_property_regime,
    validate_property_regime_contract,
};
pub use types::*;
pub use validator::{
    validate_divorce, validate_marriage, validate_pacs, validate_pacs_dissolution,
    validate_property_regime,
};
