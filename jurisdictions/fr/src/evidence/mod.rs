//! French Evidence Law (Droit de la preuve)
//!
//! This module implements French evidence law provisions from:
//! - Code civil Book III, Title XX (Articles 1353-1378)
//! - Code de procédure civile (CPC) - Civil Procedure Code
//!
//! ## Key Concepts
//!
//! - **Burden of Proof** (Charge de la preuve): Article 1353
//! - **Presumptions** (Présomptions): Articles 1354-1355
//! - **Electronic Evidence** (Preuve électronique): Articles 1366-1378
//! - **Expert Evidence** (Expertise): CPC Articles 227-229
//! - **Free Evaluation** (Libre appréciation): CPC Article 9
//!
//! ## Historical Context
//!
//! French evidence law underwent major reform in 2016 (Ordonnance n°2016-131)
//! modernizing rules for electronic evidence and digital signatures.

pub mod burden;
pub mod error;
pub mod types;
pub mod validator;

// Re-export core types
pub use error::{EvidenceLawError, EvidenceLawResult};
pub use types::{
    BurdenOfProof, Evidence, EvidenceType, ExpertReport, PresumptionType, WitnessTestimony,
};

// Re-export validation functions
pub use validator::{validate_burden_of_proof, validate_evidence, validate_presumption};

// Re-export burden of proof articles
pub use burden::{article1353, article1354, article1355};

// Re-export admissibility articles (will be implemented)
// pub use admissibility::{article1366, article1367_1378, article_cpc_9, article_cpc_145, article_cpc_227_229};
