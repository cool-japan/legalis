//! Legal Reasoning Engine for Chinese Law (中国法律推理引擎)
//!
//! Provides tools for legal analysis and reasoning specific to Chinese law.
//!
//! ## Chinese Legal System Characteristics
//!
//! - **Civil Law System (大陆法系)**: Based on codified statutes
//! - **Socialist Legal System with Chinese Characteristics (中国特色社会主义法律体系)**
//! - **Guiding Cases (指导性案例)**: Similar to precedents but not binding
//! - **Statutory Interpretation**: Hierarchical statute system
//!
//! ## Legal Hierarchy (法律位阶)
//!
//! 1. **Constitution (宪法)** - Supreme law
//! 2. **Laws (法律)** - National People's Congress
//! 3. **Administrative Regulations (行政法规)** - State Council
//! 4. **Local Regulations (地方性法规)** - Provincial/municipal legislature
//! 5. **Departmental Rules (部门规章)** - Ministries/commissions
//! 6. **Local Government Rules (地方政府规章)** - Local governments
//!
//! ## Reasoning Methods
//!
//! - Statutory interpretation (法律解释)
//! - Analogical reasoning (类比推理)
//! - A fortiori reasoning (当然解释)
//! - Purpose-based interpretation (目的解释)
//! - Systematic interpretation (体系解释)

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Legal hierarchy level (法律位阶)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LegalHierarchy {
    /// Constitution (宪法)
    Constitution,
    /// Law (法律)
    Law,
    /// Administrative regulation (行政法规)
    AdministrativeRegulation,
    /// Local regulation (地方性法规)
    LocalRegulation,
    /// Departmental rule (部门规章)
    DepartmentalRule,
    /// Local government rule (地方政府规章)
    LocalGovernmentRule,
}

impl LegalHierarchy {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Constitution => BilingualText::new("宪法", "Constitution"),
            Self::Law => BilingualText::new("法律", "Law"),
            Self::AdministrativeRegulation => {
                BilingualText::new("行政法规", "Administrative regulation")
            }
            Self::LocalRegulation => BilingualText::new("地方性法规", "Local regulation"),
            Self::DepartmentalRule => BilingualText::new("部门规章", "Departmental rule"),
            Self::LocalGovernmentRule => {
                BilingualText::new("地方政府规章", "Local government rule")
            }
        }
    }
}

/// Legal provision (法律规定)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalProvision {
    /// Statute name
    pub statute_name: BilingualText,
    /// Hierarchy level
    pub hierarchy: LegalHierarchy,
    /// Article/Section number
    pub article: String,
    /// Provision text
    pub text: BilingualText,
    /// Effective date
    pub effective_date: DateTime<Utc>,
    /// Superseded date (if any)
    pub superseded_date: Option<DateTime<Utc>>,
}

impl LegalProvision {
    /// Check if provision is currently in effect
    pub fn is_in_effect(&self, current_date: DateTime<Utc>) -> bool {
        current_date >= self.effective_date && self.superseded_date.is_none_or(|d| current_date < d)
    }
}

/// Statutory interpretation method (法律解释方法)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpretationMethod {
    /// Literal interpretation (文义解释)
    Literal,
    /// Systematic interpretation (体系解释)
    Systematic,
    /// Purpose-based interpretation (目的解释)
    PurposeBased,
    /// Historical interpretation (历史解释)
    Historical,
    /// Analogical reasoning (类比推理)
    Analogical,
    /// A fortiori reasoning (当然解释)
    AFortiori,
}

impl InterpretationMethod {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Literal => BilingualText::new("文义解释", "Literal interpretation"),
            Self::Systematic => BilingualText::new("体系解释", "Systematic interpretation"),
            Self::PurposeBased => BilingualText::new("目的解释", "Purpose-based interpretation"),
            Self::Historical => BilingualText::new("历史解释", "Historical interpretation"),
            Self::Analogical => BilingualText::new("类比推理", "Analogical reasoning"),
            Self::AFortiori => BilingualText::new("当然解释", "A fortiori reasoning"),
        }
    }
}

/// Guiding case (指导性案例)
///
/// Supreme People's Court guiding cases provide interpretative guidance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidingCase {
    /// Case number
    pub case_number: String,
    /// Case name
    pub case_name: BilingualText,
    /// Court
    pub court: BilingualText,
    /// Decision date
    pub decision_date: DateTime<Utc>,
    /// Legal principle (裁判要点)
    pub legal_principle: BilingualText,
    /// Applicable law
    pub applicable_law: Vec<String>,
    /// Is still valid
    pub is_valid: bool,
}

/// Legal fact (法律事实)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalFact {
    /// Fact description
    pub description: BilingualText,
    /// Date of occurrence
    pub occurrence_date: DateTime<Utc>,
    /// Evidence supporting the fact
    pub evidence: Vec<BilingualText>,
}

/// Legal issue (法律问题)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalIssue {
    /// Issue description
    pub description: BilingualText,
    /// Relevant facts
    pub relevant_facts: Vec<LegalFact>,
    /// Applicable provisions
    pub applicable_provisions: Vec<LegalProvision>,
    /// Relevant guiding cases
    pub relevant_cases: Vec<GuidingCase>,
}

/// Legal conclusion (法律结论)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalConclusion {
    /// Issue being resolved
    pub issue: BilingualText,
    /// Conclusion
    pub conclusion: BilingualText,
    /// Legal basis
    pub legal_basis: Vec<LegalProvision>,
    /// Reasoning method used
    pub reasoning_method: InterpretationMethod,
    /// Supporting cases
    pub supporting_cases: Vec<GuidingCase>,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
}

/// Legal analysis (法律分析)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Case identifier
    pub case_id: String,
    /// Facts
    pub facts: Vec<LegalFact>,
    /// Legal issues
    pub issues: Vec<LegalIssue>,
    /// Conclusions
    pub conclusions: Vec<LegalConclusion>,
    /// Analysis date
    pub analysis_date: DateTime<Utc>,
}

/// Conflict resolution rule (冲突规则)
///
/// When multiple provisions conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolutionRule {
    /// Higher hierarchy prevails (上位法优于下位法)
    HigherHierarchyPrevails,
    /// Special law prevails over general law (特别法优于一般法)
    SpecialLawPrevails,
    /// Later law prevails over earlier law (新法优于旧法)
    LaterLawPrevails,
}

impl ConflictResolutionRule {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::HigherHierarchyPrevails => {
                BilingualText::new("上位法优于下位法", "Higher hierarchy prevails")
            }
            Self::SpecialLawPrevails => {
                BilingualText::new("特别法优于一般法", "Special law prevails")
            }
            Self::LaterLawPrevails => BilingualText::new("新法优于旧法", "Later law prevails"),
        }
    }
}

// ============================================================================
// Reasoning Functions
// ============================================================================

/// Resolve conflict between provisions
///
/// Article 92-97 of Legislation Law (立法法)
pub fn resolve_conflict<'a>(
    provision1: &'a LegalProvision,
    provision2: &'a LegalProvision,
    _current_date: DateTime<Utc>,
) -> Result<(&'a LegalProvision, ConflictResolutionRule), ReasoningError> {
    // Rule 1: Higher hierarchy prevails (Article 94)
    if provision1.hierarchy != provision2.hierarchy {
        if provision1.hierarchy < provision2.hierarchy {
            return Ok((provision1, ConflictResolutionRule::HigherHierarchyPrevails));
        } else {
            return Ok((provision2, ConflictResolutionRule::HigherHierarchyPrevails));
        }
    }

    // Rule 3: Later law prevails (Article 94)
    if provision1.effective_date != provision2.effective_date {
        if provision1.effective_date > provision2.effective_date {
            return Ok((provision1, ConflictResolutionRule::LaterLawPrevails));
        } else {
            return Ok((provision2, ConflictResolutionRule::LaterLawPrevails));
        }
    }

    // Cannot resolve
    Err(ReasoningError::CannotResolveConflict {
        provision1: provision1.statute_name.clone(),
        provision2: provision2.statute_name.clone(),
    })
}

/// Find applicable provisions for a legal issue
pub fn find_applicable_provisions(
    issue: &LegalIssue,
    all_provisions: &[LegalProvision],
    current_date: DateTime<Utc>,
) -> Vec<LegalProvision> {
    all_provisions
        .iter()
        .filter(|p| p.is_in_effect(current_date))
        .filter(|p| {
            // Simple text matching - in practice would use more sophisticated matching
            issue.description.zh.contains(&p.statute_name.zh)
                || issue.description.en.contains(&p.statute_name.en)
        })
        .cloned()
        .collect()
}

/// Apply analogical reasoning
///
/// If case A is similar to case B, and case B has established rule R,
/// then rule R should apply to case A.
pub fn apply_analogical_reasoning(current_facts: &[LegalFact], _guiding_case: &GuidingCase) -> f64 {
    // Simple similarity score based on fact overlap
    // In practice, would use more sophisticated similarity analysis

    if current_facts.is_empty() {
        0.0
    } else {
        // Placeholder: actual implementation would compare facts in detail
        0.7
    }
}

/// Build legal argument chain
pub fn build_argument_chain(
    _facts: &[LegalFact],
    provisions: &[LegalProvision],
    method: InterpretationMethod,
) -> LegalConclusion {
    // Simplified argument building
    let conclusion_text = match method {
        InterpretationMethod::Literal => {
            BilingualText::new("根据法律文义解释", "Based on literal interpretation")
        }
        InterpretationMethod::Systematic => {
            BilingualText::new("根据体系解释", "Based on systematic interpretation")
        }
        InterpretationMethod::PurposeBased => {
            BilingualText::new("根据目的解释", "Based on purpose interpretation")
        }
        _ => BilingualText::new("根据法律推理", "Based on legal reasoning"),
    };

    LegalConclusion {
        issue: BilingualText::new("法律问题", "Legal issue"),
        conclusion: conclusion_text,
        legal_basis: provisions.to_vec(),
        reasoning_method: method,
        supporting_cases: Vec::new(),
        confidence: 0.8,
    }
}

/// Validate legal reasoning
pub fn validate_reasoning(analysis: &LegalAnalysis) -> Result<(), ReasoningError> {
    // Check that facts exist
    if analysis.facts.is_empty() {
        return Err(ReasoningError::NoFactsProvided);
    }

    // Check that issues are identified
    if analysis.issues.is_empty() {
        return Err(ReasoningError::NoIssuesIdentified);
    }

    // Check that conclusions are drawn
    if analysis.conclusions.is_empty() {
        return Err(ReasoningError::NoConclusionsDrawn);
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Legal Reasoning
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ReasoningError {
    /// Cannot resolve conflict between provisions
    #[error("Cannot resolve conflict between {provision1} and {provision2}")]
    CannotResolveConflict {
        /// Provision 1
        provision1: BilingualText,
        /// Provision 2
        provision2: BilingualText,
    },

    /// No applicable provision found
    #[error("No applicable provision found for issue: {issue}")]
    NoApplicableProvision {
        /// Issue description
        issue: BilingualText,
    },

    /// No facts provided
    #[error("No facts provided for legal analysis")]
    NoFactsProvided,

    /// No issues identified
    #[error("No legal issues identified")]
    NoIssuesIdentified,

    /// No conclusions drawn
    #[error("No conclusions drawn from analysis")]
    NoConclusionsDrawn,

    /// Invalid reasoning method
    #[error("Invalid reasoning method for this type of issue")]
    InvalidReasoningMethod,
}

/// Result type for Reasoning operations
pub type ReasoningResult<T> = Result<T, ReasoningError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_hierarchy_ordering() {
        assert!(LegalHierarchy::Constitution < LegalHierarchy::Law);
        assert!(LegalHierarchy::Law < LegalHierarchy::AdministrativeRegulation);
        assert!(LegalHierarchy::AdministrativeRegulation < LegalHierarchy::LocalRegulation);
    }

    #[test]
    fn test_provision_in_effect() {
        let now = Utc::now();
        let provision = LegalProvision {
            statute_name: BilingualText::new("民法典", "Civil Code"),
            hierarchy: LegalHierarchy::Law,
            article: "1".to_string(),
            text: BilingualText::new("本法规定...", "This law provides..."),
            effective_date: now - chrono::Duration::days(365),
            superseded_date: None,
        };

        assert!(provision.is_in_effect(now));
    }

    #[test]
    fn test_conflict_resolution_hierarchy() {
        let now = Utc::now();
        let law = LegalProvision {
            statute_name: BilingualText::new("法律", "Law"),
            hierarchy: LegalHierarchy::Law,
            article: "1".to_string(),
            text: BilingualText::new("规定A", "Provision A"),
            effective_date: now - chrono::Duration::days(365),
            superseded_date: None,
        };

        let regulation = LegalProvision {
            statute_name: BilingualText::new("行政法规", "Regulation"),
            hierarchy: LegalHierarchy::AdministrativeRegulation,
            article: "1".to_string(),
            text: BilingualText::new("规定B", "Provision B"),
            effective_date: now - chrono::Duration::days(365),
            superseded_date: None,
        };

        let result = resolve_conflict(&law, &regulation, now);
        assert!(result.is_ok());
        let (prevailing, rule) = result.unwrap();
        assert_eq!(prevailing.hierarchy, LegalHierarchy::Law);
        assert_eq!(rule, ConflictResolutionRule::HigherHierarchyPrevails);
    }

    #[test]
    fn test_conflict_resolution_later_law() {
        let now = Utc::now();
        let old_law = LegalProvision {
            statute_name: BilingualText::new("旧法", "Old law"),
            hierarchy: LegalHierarchy::Law,
            article: "1".to_string(),
            text: BilingualText::new("旧规定", "Old provision"),
            effective_date: now - chrono::Duration::days(730),
            superseded_date: None,
        };

        let new_law = LegalProvision {
            statute_name: BilingualText::new("新法", "New law"),
            hierarchy: LegalHierarchy::Law,
            article: "1".to_string(),
            text: BilingualText::new("新规定", "New provision"),
            effective_date: now - chrono::Duration::days(365),
            superseded_date: None,
        };

        let result = resolve_conflict(&old_law, &new_law, now);
        assert!(result.is_ok());
        let (prevailing, rule) = result.unwrap();
        assert_eq!(prevailing.effective_date, new_law.effective_date);
        assert_eq!(rule, ConflictResolutionRule::LaterLawPrevails);
    }

    #[test]
    fn test_legal_analysis_validation() {
        let analysis = LegalAnalysis {
            case_id: "CASE-001".to_string(),
            facts: vec![LegalFact {
                description: BilingualText::new("事实", "Fact"),
                occurrence_date: Utc::now(),
                evidence: Vec::new(),
            }],
            issues: vec![LegalIssue {
                description: BilingualText::new("法律问题", "Legal issue"),
                relevant_facts: Vec::new(),
                applicable_provisions: Vec::new(),
                relevant_cases: Vec::new(),
            }],
            conclusions: vec![LegalConclusion {
                issue: BilingualText::new("问题", "Issue"),
                conclusion: BilingualText::new("结论", "Conclusion"),
                legal_basis: Vec::new(),
                reasoning_method: InterpretationMethod::Literal,
                supporting_cases: Vec::new(),
                confidence: 0.8,
            }],
            analysis_date: Utc::now(),
        };

        assert!(validate_reasoning(&analysis).is_ok());
    }

    #[test]
    fn test_legal_analysis_no_facts() {
        let analysis = LegalAnalysis {
            case_id: "CASE-002".to_string(),
            facts: Vec::new(), // No facts
            issues: vec![LegalIssue {
                description: BilingualText::new("法律问题", "Legal issue"),
                relevant_facts: Vec::new(),
                applicable_provisions: Vec::new(),
                relevant_cases: Vec::new(),
            }],
            conclusions: Vec::new(),
            analysis_date: Utc::now(),
        };

        assert!(validate_reasoning(&analysis).is_err());
    }
}
