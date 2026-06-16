//! German Administrative Law (Verwaltungsrecht) - VwVfG
//!
//! Type-safe representations and validation for German administrative procedure
//! under the Verwaltungsverfahrensgesetz (VwVfG), focused on the administrative
//! act (Verwaltungsakt) and its life-cycle.
//!
//! See [`verwaltungsakt`] for the framework (§ 35 definition, § 36
//! Nebenbestimmungen, §§ 43-44 Wirksamkeit/Nichtigkeit, §§ 48-49 Rücknahme und
//! Widerruf) and [`rechtsbehelfe`] for legal remedies (Widerspruch / Anfechtung).

pub mod error;
pub mod rechtsbehelfe;
pub mod verwaltungsakt;

pub use error::{Result, VwVfGError};
pub use rechtsbehelfe::*;
pub use verwaltungsakt::*;
