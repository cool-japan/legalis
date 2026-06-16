//! StGB (Strafgesetzbuch) - German Criminal Code
//!
//! Type-safe representations and validation for the German Criminal Code (StGB),
//! covering both the General Part (Allgemeiner Teil, §§ 1-79b) and selected
//! offences of the Special Part (Besonderer Teil, §§ 80-358).
//!
//! # Structure (Aufbau)
//!
//! The StGB is divided into two parts:
//!
//! - **Allgemeiner Teil** (General Part) - §§ 1-79b: rules applicable to all
//!   offences (liability, intent/negligence, capacity, attempt, participation,
//!   justification/excuse, penalties).
//! - **Besonderer Teil** (Special Part) - §§ 80-358: the individual offences
//!   (homicide, bodily harm, theft, robbery, fraud, forgery, sexual offences …).
//!
//! # General Part (Allgemeiner Teil)
//!
//! See [`allgemeiner_teil`] for:
//! - **§§ 13-14** - Liability (Begehen durch Unterlassen / Garantenstellung;
//!   Handeln für einen anderen)
//! - **§§ 15-18** - Intent and negligence (Vorsatz/Fahrlässigkeit, Irrtum,
//!   erfolgsqualifizierte Delikte)
//! - **§§ 19-21** - Capacity (Schuldunfähigkeit / verminderte Schuldfähigkeit)
//! - **§§ 22-24** - Attempt (Versuch, Rücktritt)
//! - **§§ 25-30** - Perpetration and participation (Täterschaft und Teilnahme)
//! - **§§ 32-35** - Justification and excuse (Notwehr, Notstand)
//!
//! # Special Part (Besonderer Teil)
//!
//! See [`besonderer_teil`] for:
//! - **§§ 211-222** - Homicide (Tötungsdelikte)
//! - **§§ 223-231** - Bodily harm (Körperverletzung)
//! - **§§ 242-248c** - Theft (Diebstahl)
//! - **§§ 249-255** - Robbery (Raub und Erpressung)
//! - **§§ 263-266** - Fraud and breach of trust (Betrug und Untreue)
//! - **§§ 267-282** - Forgery (Urkundenfälschung)
//! - **§§ 177-184** - Sexual offences (Sexualdelikte)
//!
//! # Penalties (Strafen)
//!
//! See [`strafe`] for the sentencing framework (§§ 38-43 StGB): custodial
//! sentences (Freiheitsstrafe), day-fines (Geldstrafe in Tagessätzen), and the
//! abstract statutory ranges (Strafrahmen) attached to each offence.

pub mod allgemeiner_teil;
pub mod besonderer_teil;
pub mod error;
pub mod strafe;

pub use error::{Result, StgbError};
pub use strafe::{
    FREIHEITSSTRAFE_MAX_MONTHS, FREIHEITSSTRAFE_MIN_MONTHS, Freiheitsstrafe, Geldstrafe, Strafe,
    Strafrahmen, TAGESSAETZE_MAX, TAGESSAETZE_MIN, TAGESSATZ_MAX_CENTS, TAGESSATZ_MIN_CENTS,
};

pub use allgemeiner_teil as at;
pub use besonderer_teil as bt;
