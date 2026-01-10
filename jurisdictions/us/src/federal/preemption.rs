//! Federal Preemption Analysis
//!
//! Analyzes whether and how federal law preempts state law under the Supremacy Clause
//! (U.S. Const. Art. VI, cl. 2), which establishes that federal law is "the supreme Law
//! of the Land."
//!
//! # Overview
//!
//! The **Supremacy Clause** provides that federal law takes precedence over state law
//! when the two conflict. However, determining whether preemption occurs requires careful
//! analysis because:
//! - The Constitution preserves state sovereignty (Tenth Amendment)
//! - Federal and state governments have concurrent powers in many areas
//! - Courts presume against preemption, especially in areas of traditional state concern
//! - Congressional intent is paramount
//!
//! This module provides structured analysis of the three main categories of preemption
//! recognized by the Supreme Court.
//!
//! # Three Categories of Preemption
//!
//! ## 1. Express Preemption
//!
//! Congress explicitly states in statutory text that federal law preempts state law.
//! This is the clearest form of preemption.
//!
//! **Characteristics**:
//! - Explicit preemption clause in federal statute
//! - May preempt all state law in an area or only conflicting provisions
//! - Courts construe ambiguous preemption clauses narrowly (*Cipollone v. Liggett Group*)
//!
//! **Examples**:
//! - **ERISA**: "shall supersede any and all State laws insofar as they may now or hereafter
//!   relate to any employee benefit plan" (29 U.S.C. § 1144(a))
//! - **FAAAA** (Federal Aviation Administration Authorization Act): Preempts state regulation
//!   of airline prices, routes, and services
//! - **FIFRA** (Federal Insecticide, Fungicide, and Rodenticide Act): Preempts state pesticide
//!   labeling requirements differing from federal standards
//!
//! ## 2. Implied Field Preemption
//!
//! Federal regulation is so comprehensive that Congress intended to "occupy the field,"
//! leaving no room for state regulation—even if no express preemption clause exists.
//!
//! **Indicators of Field Preemption**:
//! - Comprehensive and detailed federal regulatory scheme
//! - Congressional intent to occupy entire field (from legislative history)
//! - Subject matter traditionally within exclusive federal domain
//! - Federal agency with exclusive regulatory authority
//! - Need for uniformity (e.g., foreign affairs, immigration)
//!
//! **Examples**:
//! - **Immigration**: Federal government exclusively controls immigration and naturalization
//!   (*Arizona v. United States*, 2012)
//! - **Nuclear Safety**: Atomic Energy Act occupies field of nuclear safety regulation
//!   (*Pacific Gas & Electric v. State Energy Resources Conservation*, 1983)
//! - **Foreign Affairs**: Federal government has exclusive control over international relations
//!
//! ## 3. Implied Conflict Preemption
//!
//! State law conflicts with federal law, making preemption necessary even without express
//! language or field occupation. Two subcategories:
//!
//! ### Impossibility Preemption
//! It is physically impossible to comply with both federal and state law simultaneously.
//!
//! **Examples**:
//! - Federal law requires X; state law forbids X
//! - Federal law sets maximum standard; state law sets higher minimum
//! - Federal law permits activity; state law prohibits it
//!
//! ### Obstacle Preemption
//! State law "stands as an obstacle to the accomplishment and execution of the full
//! purposes and objectives of Congress" (*Hines v. Davidowitz*, 1941).
//!
//! **Characteristics**:
//! - More flexible and fact-intensive than impossibility preemption
//! - Requires identifying federal objectives and assessing state interference
//! - Courts use this sparingly due to federalism concerns
//!
//! **Examples**:
//! - State law frustrates federal deregulatory objectives
//! - State law imposes additional burdens undermining federal scheme
//! - State enforcement actions interfere with federal regulatory balance
//!
//! # Presumption Against Preemption
//!
//! Courts apply a **presumption against preemption** in areas of traditional state
//! concern, requiring clear evidence of Congressional intent (*Wyeth v. Levine*, 2009).
//!
//! **Areas of Traditional State Concern**:
//! - Health and safety regulation
//! - Family law (marriage, divorce, child custody)
//! - Property and real estate
//! - Professional licensing
//! - Tort law and insurance
//! - Criminal law (except federal crimes)
//! - Education
//!
//! In these areas, courts interpret ambiguous federal statutes to preserve state authority.
//!
//! # Analytical Framework
//!
//! When analyzing preemption:
//!
//! 1. **Check for Express Preemption**: Does the statute contain an explicit preemption clause?
//!    If yes, interpret clause's scope (may be narrow or broad).
//!
//! 2. **Assess Field Preemption**: Even without express clause, has Congress occupied the field?
//!    Consider comprehensiveness, intent, tradition, agency authority, and uniformity needs.
//!
//! 3. **Evaluate Conflict Preemption**: Does state law conflict with federal law?
//!    - **Impossibility**: Can one comply with both?
//!    - **Obstacle**: Does state law frustrate federal objectives?
//!
//! 4. **Apply Presumptions**: In areas of traditional state concern, presume against preemption
//!    unless Congressional intent is clear.
//!
//! # Key Supreme Court Cases
//!
//! - ***Gibbons v. Ogden*, 22 U.S. 1 (1824)**: Early statement of federal supremacy in
//!   interstate commerce
//! - ***Hines v. Davidowitz*, 312 U.S. 52 (1941)**: Established obstacle preemption test
//! - ***Rice v. Santa Fe Elevator Corp.*, 331 U.S. 218 (1947)**: Presumption against
//!   preemption in traditional state areas
//! - ***Cipollone v. Liggett Group*, 505 U.S. 504 (1992)**: Express preemption clause
//!   interpretation (cigarette labeling)
//! - ***Geier v. American Honda Motor Co.*, 529 U.S. 861 (2000)**: Obstacle preemption
//!   upheld (federal vehicle safety standards vs. state tort law)
//! - ***Wyeth v. Levine*, 555 U.S. 555 (2009)**: Presumption against preemption applied
//!   (FDA drug labeling vs. state tort claims)
//! - ***Arizona v. United States*, 567 U.S. 387 (2012)**: Field preemption in immigration law
//!
//! # Practical Considerations
//!
//! Preemption analysis is highly fact-specific and context-dependent. This module provides
//! structured analysis tools, but actual preemption determinations require:
//! - Careful reading of federal statutory text and legislative history
//! - Understanding federal regulatory scheme and agency interpretations
//! - Knowledge of relevant Supreme Court and circuit precedents
//! - Assessment of federalism concerns and state interests
//!
//! When in doubt, consult preemption case law in the specific subject area.

use serde::{Deserialize, Serialize};

/// Type of preemption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreemptionType {
    /// Congress explicitly preempts state law in statutory text
    Express,

    /// Federal regulation occupies entire field
    ImpliedField,

    /// State law conflicts with federal law
    ImpliedConflict,

    /// No preemption found
    None,
}

impl PreemptionType {
    /// Get human-readable description.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Express => "Express Preemption (explicit statutory language)",
            Self::ImpliedField => "Implied Field Preemption (federal scheme occupies field)",
            Self::ImpliedConflict => "Implied Conflict Preemption (impossibility or obstacle)",
            Self::None => "No Preemption",
        }
    }

    /// Check if preemption applies.
    #[must_use]
    pub fn is_preempted(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Type of conflict preemption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictPreemptionType {
    /// Impossible to comply with both federal and state law
    Impossibility,

    /// State law stands as obstacle to federal objectives
    Obstacle,

    /// No conflict
    None,
}

impl ConflictPreemptionType {
    /// Get human-readable description.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Impossibility => "Impossibility - Cannot comply with both federal and state law",
            Self::Obstacle => "Obstacle - State law frustrates federal objectives",
            Self::None => "No Conflict",
        }
    }
}

/// Field preemption analysis factors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldPreemptionAnalysis {
    /// Is federal regulation comprehensive and detailed?
    pub comprehensive_federal_scheme: bool,

    /// Did Congress intend to occupy the entire field?
    pub congressional_intent_to_occupy: bool,

    /// Is subject matter traditionally federal?
    pub traditionally_federal_domain: bool,

    /// Is there a federal agency with exclusive authority?
    pub exclusive_federal_agency: bool,

    /// Additional explanatory notes
    pub notes: Vec<String>,
}

impl FieldPreemptionAnalysis {
    /// Create new field preemption analysis.
    #[must_use]
    pub fn new() -> Self {
        Self {
            comprehensive_federal_scheme: false,
            congressional_intent_to_occupy: false,
            traditionally_federal_domain: false,
            exclusive_federal_agency: false,
            notes: vec![],
        }
    }

    /// Add a note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Determine if field preemption likely applies.
    #[must_use]
    pub fn is_field_preempted(&self) -> bool {
        // Strong indicators: comprehensive scheme + intent to occupy
        let strong_indicators =
            self.comprehensive_federal_scheme && self.congressional_intent_to_occupy;

        // Traditional federal domain (immigration, foreign affairs) is strong signal
        let traditional_domain = self.traditionally_federal_domain;

        // Exclusive agency authority suggests field preemption
        let exclusive_agency = self.exclusive_federal_agency;

        strong_indicators || traditional_domain || exclusive_agency
    }
}

impl Default for FieldPreemptionAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

/// Preemption analysis input and result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreemptionAnalysis {
    /// Federal law description
    pub federal_law: String,

    /// State law description
    pub state_law: String,

    /// Federal statute citation
    pub federal_citation: Option<String>,

    /// State statute citation
    pub state_citation: Option<String>,

    /// Express preemption language found
    pub express_language: Option<String>,

    /// Field preemption analysis
    pub field_analysis: Option<FieldPreemptionAnalysis>,

    /// Conflict type (if applicable)
    pub conflict_type: Option<ConflictPreemptionType>,

    /// Subject matter area
    pub subject_matter: Option<String>,

    /// Presumption against preemption applies?
    pub presumption_against: bool,
}

impl PreemptionAnalysis {
    /// Create new preemption analysis.
    #[must_use]
    pub fn new(federal_law: impl Into<String>, state_law: impl Into<String>) -> Self {
        Self {
            federal_law: federal_law.into(),
            state_law: state_law.into(),
            federal_citation: None,
            state_citation: None,
            express_language: None,
            field_analysis: None,
            conflict_type: None,
            subject_matter: None,
            presumption_against: false,
        }
    }

    /// Set federal citation.
    #[must_use]
    pub fn with_federal_citation(mut self, citation: impl Into<String>) -> Self {
        self.federal_citation = Some(citation.into());
        self
    }

    /// Set state citation.
    #[must_use]
    pub fn with_state_citation(mut self, citation: impl Into<String>) -> Self {
        self.state_citation = Some(citation.into());
        self
    }

    /// Set express preemption language.
    #[must_use]
    pub fn with_express_language(mut self, language: impl Into<String>) -> Self {
        self.express_language = Some(language.into());
        self
    }

    /// Set field analysis.
    #[must_use]
    pub fn with_field_analysis(mut self, analysis: FieldPreemptionAnalysis) -> Self {
        self.field_analysis = Some(analysis);
        self
    }

    /// Set conflict type.
    #[must_use]
    pub fn with_conflict_type(mut self, conflict_type: ConflictPreemptionType) -> Self {
        self.conflict_type = Some(conflict_type);
        self
    }

    /// Set subject matter.
    #[must_use]
    pub fn with_subject_matter(mut self, subject: impl Into<String>) -> Self {
        self.subject_matter = Some(subject.into());
        self
    }

    /// Enable presumption against preemption.
    #[must_use]
    pub fn with_presumption_against(mut self) -> Self {
        self.presumption_against = true;
        self
    }

    /// Analyze preemption and return result.
    #[must_use]
    pub fn analyze(&self) -> PreemptionResult {
        let mut reasons = Vec::new();

        // 1. Check for express preemption (strongest)
        if let Some(express_lang) = &self.express_language {
            reasons.push(format!(
                "Express preemption language found: \"{}\"",
                express_lang
            ));

            return PreemptionResult {
                preemption_type: PreemptionType::Express,
                preempted: true,
                confidence: 0.95, // Very high confidence for express preemption
                reasoning: reasons,
                presumption_against_applied: self.presumption_against,
            };
        }

        // 2. Check for field preemption
        if let Some(field) = &self.field_analysis {
            if field.is_field_preempted() {
                reasons.push("Federal law occupies entire field:".to_string());
                if field.comprehensive_federal_scheme {
                    reasons.push("  - Comprehensive federal regulatory scheme".to_string());
                }
                if field.congressional_intent_to_occupy {
                    reasons.push("  - Congressional intent to occupy field".to_string());
                }
                if field.traditionally_federal_domain {
                    reasons.push("  - Traditionally exclusive federal domain".to_string());
                }
                if field.exclusive_federal_agency {
                    reasons.push("  - Exclusive federal agency authority".to_string());
                }

                for note in &field.notes {
                    reasons.push(format!("  - {note}"));
                }

                let confidence = if self.presumption_against {
                    0.70 // Lower confidence with presumption against
                } else {
                    0.85
                };

                return PreemptionResult {
                    preemption_type: PreemptionType::ImpliedField,
                    preempted: true,
                    confidence,
                    reasoning: reasons,
                    presumption_against_applied: self.presumption_against,
                };
            }
        }

        // 3. Check for conflict preemption
        if let Some(conflict) = &self.conflict_type {
            match conflict {
                ConflictPreemptionType::Impossibility => {
                    reasons.push(
                        "Impossibility: Cannot comply with both federal and state law".to_string(),
                    );

                    return PreemptionResult {
                        preemption_type: PreemptionType::ImpliedConflict,
                        preempted: true,
                        confidence: 0.90,
                        reasoning: reasons,
                        presumption_against_applied: self.presumption_against,
                    };
                }
                ConflictPreemptionType::Obstacle => {
                    reasons.push("Obstacle: State law frustrates federal objectives".to_string());

                    let confidence = if self.presumption_against {
                        0.65 // Obstacle preemption is fact-intensive
                    } else {
                        0.75
                    };

                    return PreemptionResult {
                        preemption_type: PreemptionType::ImpliedConflict,
                        preempted: true,
                        confidence,
                        reasoning: reasons,
                        presumption_against_applied: self.presumption_against,
                    };
                }
                ConflictPreemptionType::None => {
                    // Continue to no preemption
                }
            }
        }

        // 4. No preemption found
        reasons.push("No preemption found:".to_string());
        reasons.push("  - No express preemption language".to_string());
        reasons.push("  - Federal scheme does not occupy entire field".to_string());
        reasons.push("  - No conflict between federal and state law".to_string());

        if self.presumption_against {
            reasons.push(
                "  - Presumption against preemption applies (traditional state police power)"
                    .to_string(),
            );
        }

        PreemptionResult {
            preemption_type: PreemptionType::None,
            preempted: false,
            confidence: 0.80,
            reasoning: reasons,
            presumption_against_applied: self.presumption_against,
        }
    }
}

/// Result of preemption analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreemptionResult {
    /// Type of preemption found (or None)
    pub preemption_type: PreemptionType,

    /// Whether state law is preempted
    pub preempted: bool,

    /// Confidence in result (0.0-1.0)
    pub confidence: f64,

    /// Detailed reasoning
    pub reasoning: Vec<String>,

    /// Whether presumption against preemption was applied
    pub presumption_against_applied: bool,
}

impl PreemptionResult {
    /// Generate summary explanation.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut report = String::new();

        report.push_str("# Federal Preemption Analysis\n\n");
        report.push_str(&format!(
            "**Result**: {}\n\n",
            if self.preempted {
                "STATE LAW IS PREEMPTED"
            } else {
                "STATE LAW IS NOT PREEMPTED"
            }
        ));

        report.push_str(&format!(
            "**Preemption Type**: {}\n\n",
            self.preemption_type.description()
        ));

        report.push_str(&format!(
            "**Confidence**: {:.1}%\n\n",
            self.confidence * 100.0
        ));

        if self.presumption_against_applied {
            report.push_str(
                "**Presumption Against Preemption**: Applied (traditional state police power)\n\n",
            );
        }

        report.push_str("## Reasoning\n\n");
        for reason in &self.reasoning {
            report.push_str(&format!("{reason}\n"));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preemption_type_description() {
        assert_eq!(
            PreemptionType::Express.description(),
            "Express Preemption (explicit statutory language)"
        );
        assert_eq!(
            PreemptionType::ImpliedField.description(),
            "Implied Field Preemption (federal scheme occupies field)"
        );
        assert_eq!(PreemptionType::None.description(), "No Preemption");
    }

    #[test]
    fn test_preemption_type_is_preempted() {
        assert!(PreemptionType::Express.is_preempted());
        assert!(PreemptionType::ImpliedField.is_preempted());
        assert!(PreemptionType::ImpliedConflict.is_preempted());
        assert!(!PreemptionType::None.is_preempted());
    }

    #[test]
    fn test_conflict_preemption_type_description() {
        assert_eq!(
            ConflictPreemptionType::Impossibility.description(),
            "Impossibility - Cannot comply with both federal and state law"
        );
        assert_eq!(
            ConflictPreemptionType::Obstacle.description(),
            "Obstacle - State law frustrates federal objectives"
        );
    }

    #[test]
    fn test_express_preemption() {
        let analysis = PreemptionAnalysis::new(
            "Federal Aviation Authorization Act",
            "California AB5 (gig worker classification)",
        )
        .with_express_language(
            "a State may not enact or enforce a law related to a price, route, or service",
        );

        let result = analysis.analyze();

        assert_eq!(result.preemption_type, PreemptionType::Express);
        assert!(result.preempted);
        assert!(result.confidence > 0.9);
        assert!(!result.reasoning.is_empty());
    }

    #[test]
    fn test_field_preemption_immigration() {
        let field = FieldPreemptionAnalysis::new()
            .with_note("Immigration is exclusively federal under Constitution");

        let field = FieldPreemptionAnalysis {
            comprehensive_federal_scheme: true,
            congressional_intent_to_occupy: true,
            traditionally_federal_domain: true,
            exclusive_federal_agency: true,
            notes: field.notes,
        };

        assert!(field.is_field_preempted());

        let analysis = PreemptionAnalysis::new(
            "Immigration and Nationality Act",
            "Arizona SB 1070 (state immigration enforcement)",
        )
        .with_field_analysis(field)
        .with_subject_matter("Immigration");

        let result = analysis.analyze();

        assert_eq!(result.preemption_type, PreemptionType::ImpliedField);
        assert!(result.preempted);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_impossibility_preemption() {
        let analysis = PreemptionAnalysis::new(
            "FDA drug labeling requirements",
            "State tort law requiring additional warnings",
        )
        .with_conflict_type(ConflictPreemptionType::Impossibility)
        .with_subject_matter("Drug labeling");

        let result = analysis.analyze();

        assert_eq!(result.preemption_type, PreemptionType::ImpliedConflict);
        assert!(result.preempted);
        assert!(result.confidence > 0.85);
    }

    #[test]
    fn test_obstacle_preemption() {
        let analysis = PreemptionAnalysis::new(
            "Federal fuel efficiency standards (CAFE)",
            "State emissions standards",
        )
        .with_conflict_type(ConflictPreemptionType::Obstacle);

        let result = analysis.analyze();

        assert_eq!(result.preemption_type, PreemptionType::ImpliedConflict);
        assert!(result.preempted);
    }

    #[test]
    fn test_no_preemption() {
        let analysis = PreemptionAnalysis::new(
            "Federal workplace safety standards (OSHA)",
            "State workers' compensation law",
        )
        .with_presumption_against();

        let result = analysis.analyze();

        assert_eq!(result.preemption_type, PreemptionType::None);
        assert!(!result.preempted);
        assert!(result.presumption_against_applied);
    }

    #[test]
    fn test_presumption_against_reduces_confidence() {
        let field = FieldPreemptionAnalysis {
            comprehensive_federal_scheme: true,
            congressional_intent_to_occupy: true,
            traditionally_federal_domain: false,
            exclusive_federal_agency: false,
            notes: vec![],
        };

        let without_presumption = PreemptionAnalysis::new("Federal law", "State law")
            .with_field_analysis(field.clone())
            .analyze();

        let with_presumption = PreemptionAnalysis::new("Federal law", "State law")
            .with_field_analysis(field)
            .with_presumption_against()
            .analyze();

        // Presumption against should reduce confidence
        assert!(with_presumption.confidence < without_presumption.confidence);
    }

    #[test]
    fn test_field_preemption_weak_indicators() {
        let field = FieldPreemptionAnalysis {
            comprehensive_federal_scheme: true,
            congressional_intent_to_occupy: false, // Missing intent
            traditionally_federal_domain: false,
            exclusive_federal_agency: false,
            notes: vec![],
        };

        // Weak indicators - should not trigger field preemption
        assert!(!field.is_field_preempted());
    }

    #[test]
    fn test_preemption_result_summary() {
        let result = PreemptionResult {
            preemption_type: PreemptionType::Express,
            preempted: true,
            confidence: 0.95,
            reasoning: vec!["Express preemption found".to_string()],
            presumption_against_applied: false,
        };

        let summary = result.summary();

        assert!(summary.contains("STATE LAW IS PREEMPTED"));
        assert!(summary.contains("Express Preemption"));
        assert!(summary.contains("95.0%"));
    }
}
