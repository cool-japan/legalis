//! German jurisdiction support for Legalis-RS.
//!
//! This crate provides structured representations of German law, including:
//! - BGB (Bürgerliches Gesetzbuch) - German Civil Code
//! - StGB (Strafgesetzbuch) - German Criminal Code
//! - GG (Grundgesetz) - German Basic Law (Constitution)

pub mod bgb;

pub use bgb::{bgb_823_1, bgb_823_2, bgb_826};
