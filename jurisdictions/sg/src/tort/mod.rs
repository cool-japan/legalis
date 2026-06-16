//! Tort Law (Singapore common law)
//!
//! Type-safe modelling of the Singapore law of tort, covering the four areas in
//! the development plan: **negligence**, **defamation**, **nuisance** (private
//! and public) and **occupiers' liability**, together with the general defences.
//! The law is predominantly common law as developed by the Singapore courts,
//! supplemented by statute (the Defamation Act 1957; the Contributory Negligence
//! and Personal Injuries Act 1953).
//!
//! ## 1. Negligence
//!
//! The cause of action requires duty, breach, causation and actionable damage.
//! Duty is determined by the single, universal **two-stage test** in
//! *Spandeck Engineering (Private) Ltd v Defence Science & Technology Agency*
//! \[2007\] SGCA 37, applied against a threshold of factual foreseeability:
//!
//! 1. Factual foreseeability (threshold).
//! 2. Stage 1 — legal proximity (*Hedley Byrne v Heller* \[1964\] AC 465).
//! 3. Stage 2 — policy considerations negating the duty.
//!
//! Breach is judged against the standard of the reasonable person (*Blyth v
//! Birmingham Waterworks*), or the *Bolam*/*Bolitho* standard for professionals,
//! weighing the risk-calculus factors (*Bolton v Stone* \[1951\] AC 850).
//! Causation requires factual ("but for") causation (*Barnett v Chelsea &
//! Kensington Hospital* \[1969\] 1 QB 428), an unbroken chain, and damage of a
//! reasonably foreseeable kind (*The Wagon Mound (No 1)* \[1961\] AC 388).
//!
//! ## 2. Defamation
//!
//! Governed by the common law and the Defamation Act 1957. The claimant must
//! show a defamatory statement that refers to it and was published to a third
//! party. **Libel** is actionable per se; **slander** requires special damage
//! save for the exceptions in ss. 5–6. Defences include justification (s. 8),
//! fair comment (*Review Publishing v Lee Hsien Loong* \[2009\] SGCA 46),
//! absolute and qualified privilege (*Reynolds*), and offer of amends (s. 7).
//!
//! ## 3. Nuisance
//!
//! **Private nuisance** protects a person with a proprietary interest against a
//! substantial and unreasonable interference with the use or enjoyment of land
//! (*Hunter v Canary Wharf* \[1997\] AC 655; *St Helen's Smelting v Tipping*).
//! **Public nuisance** is actionable in tort by a private claimant only on proof
//! of special damage (*Tate & Lyle v GLC* \[1983\] 2 AC 509).
//!
//! ## 4. Occupiers' liability
//!
//! An occupier owes a duty to take reasonable care for the safety of entrants,
//! the content of which varies with the entrant's status — lawful visitor versus
//! trespasser (*British Railways Board v Herrington* \[1972\] AC 877).
//!
//! ## General defences
//!
//! Contributory negligence apportions damages (Contributory Negligence and
//! Personal Injuries Act 1953, s. 3); volenti non fit injuria and illegality
//! (*Ochroid Trading v Chua Siok Lui* \[2018\] SGCA 5) are complete defences.
//!
//! ## Example
//!
//! ```rust
//! use legalis_sg::tort::*;
//!
//! // Build and assess a negligence claim.
//! let claim = NegligenceClaim::new(
//!     "n1",
//!     "Injured Pedestrian",
//!     "Careless Driver",
//!     DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
//!     BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
//!     CausationAnalysis::established(),
//!     2_000_000, // SGD 20,000 in cents
//! );
//! assert!(negligence_succeeds(&claim));
//!
//! // Apportion for the claimant's 25% contributory negligence.
//! let reduced = apportion_for_contributory_negligence(2_000_000, 25).unwrap();
//! assert_eq!(reduced, 1_500_000); // SGD 15,000
//!
//! // Assess a libel that is met by the defence of justification.
//! let mut libel = DefamationClaim::new("d1", "P", "D", "P is a fraud", DefamationForm::Libel);
//! libel.add_defence(DefamationDefence::Justification);
//! assert!(!defamation_succeeds(&libel));
//! ```
//!
//! ## Submodules
//!
//! - [`types`] — negligence and defamation models.
//! - [`nuisance`] — private/public nuisance, occupiers' liability, defences.
//! - [`validator`] — assessment functions and the assessment report.
//! - [`error`] — [`error::TortError`] with attributed authorities.

pub mod error;
pub mod nuisance;
pub mod types;
pub mod validator;

pub use error::{Result, TortError};
pub use nuisance::{
    EntrantStatus, InterferenceKind, OccupiersLiabilityClaim, PrivateNuisanceClaim,
    PublicNuisanceClaim, TortDefence,
};
pub use types::{
    BreachAnalysis, CausationAnalysis, DefamationClaim, DefamationDefence, DefamationForm,
    DutyOfCareAnalysis, HarmCategory, NegligenceClaim, SlanderPerSeException, StandardOfCare,
};
pub use validator::{
    TortAssessmentReport, TortCategory, apportion_for_contributory_negligence, assess_defamation,
    assess_negligence, assess_occupiers_liability, assess_private_nuisance, assess_public_nuisance,
    defamation_succeeds, negligence_succeeds,
};
