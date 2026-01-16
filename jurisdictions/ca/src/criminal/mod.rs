//! Canada Criminal Law Module
//!
//! This module provides comprehensive modeling of Canadian criminal law,
//! primarily the Criminal Code (RSC 1985, c C-46).
//!
//! ## Key Areas
//!
//! - **Homicide**: Murder (first/second degree), manslaughter, criminal negligence
//! - **Assault**: Common assault to aggravated assault (ss.265-269)
//! - **Sexual Offences**: Sexual assault, aggravated sexual assault
//! - **Property Offences**: Theft, fraud, break and enter
//! - **Defences**: Self-defence (s.34), necessity (Perka), duress, NCR (s.16)
//! - **Sentencing**: Principles (s.718), Gladue factors (s.718.2(e))
//!
//! ## Mens Rea Framework
//!
//! Canadian criminal law recognizes several levels of mens rea:
//! - **Intention**: Subjective purpose or desire
//! - **Knowledge**: Awareness of circumstances
//! - **Recklessness**: Subjective awareness of risk
//! - **Criminal Negligence**: Marked departure from reasonable standard
//!
//! ## Key Cases
//!
//! - **R v Woollin** [1999]: Oblique intention - virtual certainty test
//! - **R v Martineau** [1990] SCC: Subjective foresight for murder
//! - **R v Creighton** [1993] SCC: Objective foreseeability for manslaughter
//! - **R v Grant** [2009] SCC: Section 24(2) exclusion framework
//! - **R v Jordan** [2016] SCC: Trial delay presumptive ceilings
//! - **R v Gladue** [1999] SCC: Indigenous sentencing under s.718.2(e)
//! - **Perka v The Queen** [1984] SCC: Necessity defence elements

mod error;
mod offences;
mod sentencing;
mod types;

pub use error::{CriminalError, CriminalResult};
pub use offences::{
    AssaultAnalyzer, AssaultFacts, AssaultResult, CausationFacts, DefenceAnalyzer, DefenceFacts,
    DefenceOutcome, DefenceResult, HomicideAnalyzer, HomicideFacts, HomicideResult, InterveningAct,
    MentalStateFacts, PlannedDeliberateFacts, ProvocationFacts,
};
pub use sentencing::{
    GladueAnalysis, HarmLevel, OffenderInfo, PriorConviction, SentenceRange, SentencingAnalyzer,
    SentencingFacts, SentencingResult, VictimImpact,
};
pub use types::{
    AccusedElection, ActusReus, AggravatingFactor, AssaultType, BailType, BodilyHarmLevel,
    BreachSeriousness, BreakEnterType, CharterRemedy, CriminalArea, CriminalCase,
    CriminalCharterRight, CriminalDefence, CrownElection, DetentionGround, DuressElements,
    DutySource, FirstDegreeFactor, FraudType, GladueFactor, GrantAnalysis, HomicideType,
    ImpactLevel, IntentionType, IntoxicationDefence, ManslaughterType, MensRea,
    MentalDisorderElements, MitigatingFactor, ModeOfTrial, NecessityElements, OffenceCategory,
    OffenceType, RecklessnessType, SelfDefenceElements, SentenceType, SentencingPrinciple,
    SocietalInterest, TheftType,
};

// Re-export legalis-core types
pub use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Criminal Code statute
pub fn create_criminal_code() -> Statute {
    Statute::new(
        "CCC",
        "Criminal Code, RSC 1985, c C-46",
        Effect::new(
            EffectType::Prohibition,
            "Codifies criminal offences, defences, and procedure in Canada. \
             Covers offences against the person, property, public order, and administration of justice.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Controlled Drugs and Substances Act statute
pub fn create_cdsa() -> Statute {
    Statute::new(
        "CDSA",
        "Controlled Drugs and Substances Act, SC 1996, c 19",
        Effect::new(
            EffectType::Prohibition,
            "Regulates controlled substances. Creates offences for possession, trafficking, \
             production, and import/export of scheduled substances.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Youth Criminal Justice Act statute
pub fn create_ycja() -> Statute {
    Statute::new(
        "YCJA",
        "Youth Criminal Justice Act, SC 2002, c 1",
        Effect::new(
            EffectType::Obligation,
            "Governs criminal justice system for young persons (12-17). \
             Emphasizes rehabilitation, reintegration, and extrajudicial measures.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Charter statute for criminal rights
pub fn create_charter_criminal_rights() -> Statute {
    Statute::new(
        "CHARTER-CRIM",
        "Canadian Charter of Rights and Freedoms (Criminal Rights)",
        Effect::new(
            EffectType::Grant,
            "Protects fundamental rights in criminal process: s.7 life, liberty, security; \
             s.8 search and seizure; s.9 arbitrary detention; s.10 rights on arrest; \
             s.11 trial rights; s.12 cruel punishment; s.24 remedies.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create all criminal law statutes
pub fn create_criminal_statutes() -> Vec<Statute> {
    vec![
        create_criminal_code(),
        create_cdsa(),
        create_ycja(),
        create_charter_criminal_rights(),
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_criminal_code() {
        let statute = create_criminal_code();
        assert!(statute.title.contains("Criminal Code"));
    }

    #[test]
    fn test_create_cdsa() {
        let statute = create_cdsa();
        assert!(statute.title.contains("Controlled Drugs"));
    }

    #[test]
    fn test_create_ycja() {
        let statute = create_ycja();
        assert!(statute.title.contains("Youth"));
    }

    #[test]
    fn test_create_criminal_statutes() {
        let statutes = create_criminal_statutes();
        assert_eq!(statutes.len(), 4);
    }
}
