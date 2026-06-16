//! StGB Allgemeiner Teil (General Part) - §§ 1-79b StGB
//!
//! The General Part of the German Criminal Code contains the rules common to all
//! offences. This module implements the actionable, pure-Rust subset:
//!
//! - [`unterlassen`] - **§§ 13-14**: liability by omission (Begehen durch
//!   Unterlassen / Garantenstellung) and acting for another (Handeln für einen
//!   anderen).
//! - [`schuld`] - **§§ 15-18**: intent and negligence (Vorsatz/Fahrlässigkeit),
//!   mistake (Irrtum, §§ 16-17), result-qualified offences (§ 18).
//! - [`schuldfaehigkeit`] - **§§ 19-21**: criminal capacity (Schuldunfähigkeit /
//!   verminderte Schuldfähigkeit).
//! - [`versuch_teilnahme`] - **§§ 22-30**: attempt (Versuch, Rücktritt) and
//!   perpetration/participation (Täterschaft und Teilnahme).
//! - [`rechtfertigung`] - **§§ 32-35**: justification and excuse (Notwehr,
//!   rechtfertigender und entschuldigender Notstand).
//! - [`strafen`] - **§§ 38-43**: penalties (Freiheitsstrafe, Geldstrafe,
//!   Ersatzfreiheitsstrafe).
//!
//! # Three-step structure of criminal liability (Verbrechensaufbau)
//!
//! German doctrine analyses every offence in three steps:
//! 1. **Tatbestand** (offence definition) - objective and subjective elements,
//!    see [`schuld`] (§ 15) and [`unterlassen`] (§ 13).
//! 2. **Rechtswidrigkeit** (unlawfulness) - presumed unless a justification
//!    ground applies, see [`rechtfertigung`] (§§ 32, 34).
//! 3. **Schuld** (culpability) - capacity (§§ 19-21, [`schuldfaehigkeit`]),
//!    awareness of wrongdoing (§ 17, [`schuld`]) and the absence of an excuse
//!    (§ 35, [`rechtfertigung`]).

pub mod rechtfertigung;
pub mod schuld;
pub mod schuldfaehigkeit;
pub mod strafen;
pub mod unterlassen;
pub mod versuch_teilnahme;

// Re-exports for convenience.
pub use rechtfertigung::{
    DefenceKind, EntschuldigenderNotstand, NotstandsRechtsgut, Notwehr, RechtfertigenderNotstand,
    evaluate_entschuldigender_notstand, evaluate_notwehr, evaluate_rechtfertigender_notstand,
};
pub use schuld::{
    Erfolgsqualifikation, Irrtum, OffenceMensRea, Schuldform, check_erfolgsqualifikation,
    check_mens_rea, evaluate_mistake,
};
pub use schuldfaehigkeit::{
    CapacityAssessment, CapacityImpact, CapacityResult, STRAFMUENDIGKEIT_ALTER, SeelischeStoerung,
    assess_capacity,
};
pub use strafen::{CombinedPenalty, SentenceMeasureUnit, ersatzfreiheitsstrafe_days, measure_unit};
pub use unterlassen::{
    Garantenstellung, UnterlassungsCase, VertretungsBasis, VertretungsCase, validate_unterlassung,
    validate_vertretung,
};
pub use versuch_teilnahme::{
    Deliktstyp, Haupttat, Ruecktritt, Taeterschaft, Teilnahme, TeilnahmeCase, Versuch,
    Versuchsstadium, validate_teilnahme, validate_versuch,
};
