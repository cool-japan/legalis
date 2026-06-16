#![allow(clippy::too_many_arguments)]

//! Legalis-Porting: Legal system porting for Legalis-RS.
//!
//! This crate enables "Soft ODA" (Official Development Assistance through legal framework
//! sharing) - porting legal frameworks between jurisdictions while adapting to local
//! cultural parameters and legal systems.
//!
//! # Features
//!
//! ## Core Porting
//!
//! - **Cross-jurisdiction statute translation**: Port statutes between different legal systems
//! - **Cultural parameter injection**: Automatically adapt age limits, prohibitions, etc.
//! - **Compatibility reports**: Assess feasibility and generate detailed analysis
//! - **Change tracking**: Document all adaptations made during porting
//! - **Partial porting**: Port specific sections of statutes
//! - **Reverse porting**: Analyze what would be needed to port back to source
//!
//! ## Intelligence & Validation
//!
//! - **AI-assisted adaptation**: Generate cultural adaptation suggestions using LLM
//! - **Conflict detection**: Identify conflicts with target jurisdiction laws
//! - **Semantic preservation**: Validate that legal meaning is preserved
//! - **Risk assessment**: Evaluate risks in ported statutes
//! - **Similar statute finding**: Find equivalent statutes across jurisdictions
//! - **Automatic term replacement**: Replace legal terms with local equivalents
//! - **Context-aware parameter adjustment**: Adjust values based on context
//!
//! ## Workflow & Compliance
//!
//! - **Legal expert review workflow**: Submit ported statutes for expert review
//! - **Automated compliance checking**: Check compliance with target regulations
//! - **Porting workflow management**: Track multi-step porting processes
//! - **Version control**: Manage versioned ported statutes
//!
//! ## Bilateral Cooperation
//!
//! - **Bilateral legal agreement templates**: Create agreements between jurisdictions
//! - **Regulatory equivalence mapping**: Map equivalent regulations
//! - **Batch porting**: Port multiple statutes efficiently
//!
//! # Examples
//!
//! ## Basic Porting
//!
//! ```rust
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
//! use legalis_porting::{PortingEngine, PortingOptions};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create jurisdictions
//! let japan = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
//!     .with_legal_system(LegalSystem::CivilLaw)
//!     .with_cultural_params(CulturalParams::japan());
//!
//! let usa = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
//!     .with_legal_system(LegalSystem::CommonLaw)
//!     .with_cultural_params(CulturalParams::for_country("US"));
//!
//! // Create porting engine
//! let engine = PortingEngine::new(japan, usa);
//!
//! // Create a statute
//! let statute = Statute::new(
//!     "adult-rights",
//!     "成人権利法",
//!     Effect::new(EffectType::Grant, "Full legal capacity"),
//! );
//!
//! // Port with options
//! let options = PortingOptions {
//!     apply_cultural_params: true,
//!     translate_terms: true,
//!     generate_report: true,
//!     ..Default::default()
//! };
//!
//! let ported = engine.port_statute(&statute, &options)?;
//!
//! // Review changes
//! for change in &ported.changes {
//!     println!("{:?}: {}", change.change_type, change.description);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Batch Porting with Validation
//!
//! ```rust
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
//! use legalis_porting::{PortingEngine, PortingOptions};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let japan = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
//! #     .with_legal_system(LegalSystem::CivilLaw)
//! #     .with_cultural_params(CulturalParams::japan());
//! # let usa = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
//! #     .with_legal_system(LegalSystem::CommonLaw)
//! #     .with_cultural_params(CulturalParams::for_country("US"));
//! let engine = PortingEngine::new(japan, usa);
//!
//! let statutes = [
//!     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Right 1")),
//!     Statute::new("s2", "Statute 2", Effect::new(EffectType::Grant, "Right 2")),
//! ];
//!
//! let options = PortingOptions {
//!     apply_cultural_params: true,
//!     detect_conflicts: true,
//!     validate_semantics: true,
//!     ..Default::default()
//! };
//!
//! let result = engine.batch_port(&statutes, &options).await?;
//!
//! println!("Ported {} statutes", result.statutes.len());
//! println!("Detected {} conflicts", result.conflicts.len());
//! # Ok(())
//! # }
//! ```
//!
//! # Architecture
//!
//! The porting process follows these stages:
//!
//! 1. **Analysis**: Examine source statute structure and cultural parameters
//! 2. **Compatibility Check**: Assess legal system compatibility
//! 3. **Cultural Injection**: Apply target jurisdiction parameters
//! 4. **Conflict Detection**: Identify conflicts with target laws
//! 5. **Semantic Validation**: Verify legal meaning preservation
//! 6. **Risk Assessment**: Evaluate implementation risks
//! 7. **Report Generation**: Document all changes and recommendations

pub mod blockchain;
mod functions;
mod functions_2;
#[cfg(test)]
mod functions_2_a;
#[cfg(test)]
mod functions_2_b;
#[cfg(test)]
mod functions_2_c;
#[cfg(test)]
mod functions_2_d;
#[cfg(test)]
mod functions_2_e;
pub mod metaverse;
mod trait_impls;
mod types;
mod types_10;
mod types_11;
mod types_12;
mod types_3;
mod types_4;
mod types_5;
mod types_6;
mod types_7;
mod types_8;
mod types_9;

// trait_impls and functions_2 contain impl blocks and tests only, not types to re-export
pub use functions::*;
pub use types::*;
pub use types_3::*;
pub use types_4::*;
pub use types_5::*;
pub use types_6::*;
pub use types_7::*;
pub use types_8::*;
pub use types_9::*;
pub use types_10::*;
pub use types_11::*;
pub use types_12::*;
