//! StGB Besonderer Teil (Special Part) - selected offences
//!
//! This module implements selected offences of the Special Part of the German
//! Criminal Code (StGB, §§ 80-358), each modelled with its objective and
//! subjective elements (Tatbestandsmerkmale), a builder/validator, and the
//! statutory sentencing range (Strafrahmen) drawn from [`crate::stgb::strafe`].
//!
//! - [`toetungsdelikte`] - **§§ 211-222**: homicide (Mord, Totschlag, fahrlässige
//!   Tötung).
//! - [`koerperverletzung`] - **§§ 223-231**: bodily harm (Körperverletzung,
//!   gefährliche, schwere).
//! - [`diebstahl`] - **§§ 242-248c**: theft (Diebstahl, besonders schwerer Fall,
//!   Qualifikationen).
//! - [`raub`] - **§§ 249-255**: robbery and extortion (Raub, schwerer Raub,
//!   räuberischer Diebstahl, räuberische Erpressung).
//! - [`betrug`] - **§§ 263-266**: fraud and breach of trust (Betrug,
//!   Computerbetrug, Untreue).
//! - [`urkundenfaelschung`] - **§§ 267-282**: forgery of documents
//!   (Urkundenfälschung).
//! - [`sexualdelikte`] - **§§ 177-184**: sexual offences (sexueller Übergriff /
//!   Vergewaltigung).

pub mod betrug;
pub mod diebstahl;
pub mod koerperverletzung;
pub mod raub;
pub mod sexualdelikte;
pub mod toetungsdelikte;
pub mod urkundenfaelschung;
