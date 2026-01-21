//! UK Public Law Module
//!
//! This module provides comprehensive analysis of UK public law, covering:
//!
//! - **Judicial Review**: Grounds (illegality, irrationality, procedural impropriety),
//!   standing, time limits, and remedies
//! - **Human Rights**: HRA 1998 framework, ECHR articles, proportionality analysis,
//!   and public authority duties
//! - **Constitutional Law**: Parliamentary sovereignty, rule of law, separation of
//!   powers, and royal prerogative
//!
//! # Key Cases
//!
//! ## Judicial Review
//! - Associated Provincial Picture Houses v Wednesbury \[1948\] - irrationality test
//! - Council of Civil Service Unions v Minister \[1985\] (GCHQ) - three grounds
//! - Anisminic v Foreign Compensation Commission \[1969\] - jurisdictional error
//! - R v Secretary of State, ex parte Daly \[2001\] - proportionality
//!
//! ## Human Rights
//! - Ghaidan v Godin-Mendoza \[2004\] - HRA s.3 interpretation
//! - YL v Birmingham City Council \[2007\] - public authority status
//! - A v Secretary of State \[2004\] - Belmarsh detainees
//!
//! ## Constitutional
//! - R (Miller) v Secretary of State \[2017\] - Article 50
//! - R (Miller) v The Prime Minister \[2019\] - prorogation
//! - Entick v Carrington (1765) - rule of law foundation
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::public_law::{
//!     JudicialReviewAnalyzer, HraAnalyzer, ConstitutionalAnalyzer,
//!     GroundOfReview, EchrArticle, PublicBodyType,
//! };
//!
//! // Analyze a judicial review claim
//! let jr_result = JudicialReviewAnalyzer::analyze(
//!     "Home Office refusal of visa",
//!     &PublicBodyType::CentralGovernment { department: "Home Office".into() },
//!     vec![GroundOfReview::Illegality(IllegalityType::FetteringDiscretion)],
//!     true, // has standing
//!     30,   // days since decision
//! );
//!
//! // Analyze human rights claim
//! let hra_result = HraAnalyzer::analyze(
//!     EchrArticle::Article8,
//!     "Deportation of long-term resident",
//!     true,  // interference
//!     Some("Immigration control".into()),
//!     Some("Proportionate to aim".into()),
//! );
//! ```

#![allow(missing_docs)]

pub mod constitutional;
pub mod error;
pub mod human_rights;
pub mod judicial_review;
pub mod types;

// Re-export main types
pub use types::{
    BiasType,
    // Constitutional principles
    ConstitutionalPrinciple,
    DamagesBasis,
    DecisionNature,
    DeclarationOfIncompatibility,
    // ECHR articles
    EchrArticle,
    ExpectationType,
    // Judicial review grounds
    GroundOfReview,
    HraAnalysisResult,
    // HRA duties
    HraDuty,
    IllegalityType,
    InjunctionType,
    IrrationalityType,
    // Analysis results
    JrAnalysisResult,
    // Remedies
    JrRemedy,
    // Time limits
    JrTimeLimit,
    LegitimateAim,
    PrerogativePower,
    ProceduralType,
    // Proportionality
    ProportionalityAnalysis,
    // Public bodies and decisions
    PublicBodyType,
    // Case citations
    PublicLawCitation,
    RuleOfLawPrinciple,
    Section3Outcome,
    Section6Authority,
    SpecificLimit,
    // Standing
    StandingType,
    SuccessLikelihood,
};

// Re-export error types
pub use error::{
    ConstitutionalError, HumanRightsError, JudicialReviewError, PublicLawError, PublicLawResult,
};

// Re-export judicial review module
pub use judicial_review::{
    ClaimantType,
    DamagesClaimFacts,
    DecisionType,
    ExpectationFacts,
    // Supporting types
    GroundStrength,
    // Analysis result types
    GroundsAnalysisResult,
    // Analyzers
    GroundsAnalyzer,
    GroundsFacts,
    HumanRightsFacts,
    IllegalityFacts,
    IrrationalityFacts,
    JrFacts,
    JudicialReviewAnalyzer,
    ProceduralFacts,
    PromiseOrPractice,
    RemediesAnalysisResult,
    RemediesAnalyzer,
    RemediesFacts,
    StandingAnalysisResult,
    StandingAnalyzer,
    StandingFacts,
    StatutoryPower,
    TimeLimitAnalyzer,
    TimeLimitFacts,
    TimeLimitResult,
};

// Re-export human rights module
pub use human_rights::{
    // Analysis result types
    ArticleAnalysisResult,
    // Analyzers
    ArticleAnalyzer,
    ArticleEngagement,
    BodyType,
    ClaimantFacts,
    HraAnalyzer,
    // Supporting types
    HraFacts,
    InterferenceFacts,
    InterferenceSeverity,
    JustificationFacts,
    RespondentFacts,
    Section3Analyzer,
    Section3Facts,
    Section3Result,
    Section4Analyzer,
    Section4Facts,
    Section4Result,
    Section6Analyzer,
    Section6Result,
    VictimType,
};

// Re-export constitutional module
pub use constitutional::{
    ConstitutionalAnalysis,
    ConstitutionalAnalyzer,
    ConstitutionalAssessment,
    ConstitutionalBranch,
    // Case references
    ConstitutionalCase,
    PrerogativeAnalysis,
    PrerogativeAnalyzer,
    RuleOfLawAnalysis,
    RuleOfLawAnalyzer,
    RuleOfLawAssessment,
    // Supporting types
    RuleOfLawFactors,
    RuleOfLawViolation,
    SeparationAnalysis,
    SeparationAnalyzer,
    SeparationConflict,
    // Analysis types
    SovereigntyAnalysis,
    // Analyzers
    SovereigntyAnalyzer,
    SovereigntyLimitation,
    ViolationSeverity,
};

// ============================================================================
// Integration with legalis-core
// ============================================================================

use legalis_core::{Effect, Statute, StatuteBuilder};

/// Create HRA 1998 statute for reasoning
pub fn create_hra_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-hra-1998")
        .title("Human Rights Act 1998")
        .effect(Effect::grant("Convention rights enforceable in UK courts"))
        .jurisdiction("UK")
        .build()
        .expect("HRA statute should build successfully")
}

/// Create CPR Part 54 (Judicial Review) statute for reasoning
pub fn create_cpr_54_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-cpr-part-54")
        .title("Civil Procedure Rules Part 54")
        .effect(Effect::grant("Judicial review procedure"))
        .jurisdiction("UK")
        .build()
        .expect("CPR 54 statute should build successfully")
}

/// Create Senior Courts Act 1981 s.31 statute for reasoning
pub fn create_sca_1981_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-sca-1981-s31")
        .title("Senior Courts Act 1981 s.31")
        .effect(Effect::grant("Judicial review remedies"))
        .jurisdiction("UK")
        .build()
        .expect("SCA 1981 statute should build successfully")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hra_statute_creation() {
        let statute = create_hra_statute();
        assert!(statute.title.contains("Human Rights Act"));
    }

    #[test]
    fn test_cpr_54_statute_creation() {
        let statute = create_cpr_54_statute();
        assert!(statute.title.contains("Civil Procedure Rules"));
    }

    #[test]
    fn test_sca_statute_creation() {
        let statute = create_sca_1981_statute();
        assert!(statute.title.contains("Senior Courts Act"));
    }

    #[test]
    fn test_public_law_module_exports() {
        // Test that key types are accessible
        let _body = PublicBodyType::GovernmentDepartment {
            name: "Home Office".into(),
        };
        let _ground = GroundOfReview::Illegality(IllegalityType::UltraVires);
        let _article = EchrArticle::Article8;
        let _principle = ConstitutionalPrinciple::RuleOfLaw;
    }

    #[test]
    fn test_error_types_accessible() {
        let _err = PublicLawError::InvalidInput("test".into());
        let _jr_err = JudicialReviewError::NoStanding {
            reason: "test".into(),
        };
        let _hr_err = HumanRightsError::NotPublicAuthority {
            reason: "test".into(),
        };
        let _const_err = ConstitutionalError::PoliticalQuestion {
            matter: "test".into(),
        };
    }

    #[test]
    fn test_analyzer_accessibility() {
        // Verify analyzers are accessible
        let _jr = JudicialReviewAnalyzer;
        let _hra = HraAnalyzer;
        let _const = ConstitutionalAnalyzer;
    }
}
