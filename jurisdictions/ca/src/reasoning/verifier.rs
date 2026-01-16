//! Canada Constitutional Verifier
//!
//! Verification of laws against Canadian constitutional requirements.

#![allow(missing_docs)]

use legalis_core::{EffectType, Statute};
use serde::{Deserialize, Serialize};

use crate::constitution::{CharterRight, EnactingBody};

// ============================================================================
// Constitutional Verifier
// ============================================================================

/// Constitutional verifier for Canadian law
pub struct ConstitutionalVerifier;

impl ConstitutionalVerifier {
    /// Verify statute against constitutional requirements
    pub fn verify(statute: &Statute, context: &VerificationContext) -> VerificationResult {
        let mut issues = Vec::new();

        // Check Charter compliance
        if let Some(charter_issues) = Self::check_charter(statute, context) {
            issues.extend(charter_issues);
        }

        // Check division of powers
        if let Some(division_issues) = Self::check_division_of_powers(statute, context) {
            issues.extend(division_issues);
        }

        // Check Aboriginal rights
        if let Some(aboriginal_issues) = Self::check_aboriginal_rights(statute, context) {
            issues.extend(aboriginal_issues);
        }

        let valid = issues.is_empty() || issues.iter().all(|i| !i.fatal);
        let reasoning = Self::build_reasoning(statute, &issues);

        VerificationResult {
            statute_id: statute.id.clone(),
            valid,
            issues,
            reasoning,
        }
    }

    /// Check Charter compliance
    fn check_charter(
        statute: &Statute,
        context: &VerificationContext,
    ) -> Option<Vec<ConstitutionalIssue>> {
        let mut issues = Vec::new();

        // Check if statute potentially limits Charter rights
        for right in &context.charter_rights_engaged {
            if Self::potentially_limits_right(statute, right) {
                // Check if limitation is saved by s.1
                if context.section_1_justified.contains(right) {
                    issues.push(ConstitutionalIssue {
                        issue_type: IssueType::CharterLimitation,
                        description: format!(
                            "Limits {:?} but justified under s.1 Oakes test",
                            right
                        ),
                        fatal: false,
                        remedial_action: Some("No action required - s.1 saves".to_string()),
                    });
                } else {
                    issues.push(ConstitutionalIssue {
                        issue_type: IssueType::CharterViolation,
                        description: format!(
                            "Potentially violates {:?} - s.1 justification needed",
                            right
                        ),
                        fatal: true,
                        remedial_action: Some(
                            "Demonstrate pressing objective and proportionality (Oakes test)"
                                .to_string(),
                        ),
                    });
                }
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check if statute potentially limits a Charter right
    fn potentially_limits_right(statute: &Statute, right: &CharterRight) -> bool {
        // Prohibitions are more likely to limit rights
        if statute.effect.effect_type == EffectType::Prohibition {
            return true;
        }

        // Check description for limitation indicators
        let desc = statute.effect.description.to_lowercase();

        match right {
            CharterRight::FreedomOfExpression => {
                desc.contains("speech") || desc.contains("expression") || desc.contains("publish")
            }
            CharterRight::FreedomOfReligion => {
                desc.contains("religion") || desc.contains("religious") || desc.contains("worship")
            }
            CharterRight::LifeLibertySecurityOfPerson => {
                desc.contains("imprison") || desc.contains("detain") || desc.contains("custody")
            }
            CharterRight::EqualityRights => {
                desc.contains("discriminat") || desc.contains("distinction")
            }
            _ => false,
        }
    }

    /// Check division of powers
    fn check_division_of_powers(
        statute: &Statute,
        context: &VerificationContext,
    ) -> Option<Vec<ConstitutionalIssue>> {
        let mut issues = Vec::new();

        // Determine if statute jurisdiction matches claimed power
        if let Some(jurisdiction) = &statute.jurisdiction {
            let is_federal = jurisdiction.contains("FED");

            // Check if enacted by correct level of government
            match &context.enacting_body {
                EnactingBody::Parliament if !is_federal => {
                    issues.push(ConstitutionalIssue {
                        issue_type: IssueType::DivisionOfPowers,
                        description: "Parliament enacting provincial matter".to_string(),
                        fatal: true,
                        remedial_action: Some("Identify federal head of power".to_string()),
                    });
                }
                EnactingBody::ProvincialLegislature(_) if is_federal => {
                    issues.push(ConstitutionalIssue {
                        issue_type: IssueType::DivisionOfPowers,
                        description: "Province enacting federal matter".to_string(),
                        fatal: true,
                        remedial_action: Some("Provincial jurisdiction not available".to_string()),
                    });
                }
                _ => {}
            }

            // Check for interjurisdictional immunity
            if context.affects_federal_undertaking && !is_federal {
                issues.push(ConstitutionalIssue {
                    issue_type: IssueType::InterjurisdictionalImmunity,
                    description: "Provincial law may impair core federal undertaking".to_string(),
                    fatal: false,
                    remedial_action: Some("Check if IJI doctrine applies".to_string()),
                });
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Check Aboriginal rights (s.35)
    fn check_aboriginal_rights(
        statute: &Statute,
        context: &VerificationContext,
    ) -> Option<Vec<ConstitutionalIssue>> {
        let mut issues = Vec::new();

        if context.affects_aboriginal_rights {
            // Check for duty to consult
            if !context.consultation_completed {
                issues.push(ConstitutionalIssue {
                    issue_type: IssueType::DutyToConsult,
                    description: "May trigger duty to consult (Haida Nation)".to_string(),
                    fatal: false,
                    remedial_action: Some(
                        "Conduct consultation proportionate to claim strength and impact"
                            .to_string(),
                    ),
                });
            }

            // Check for infringement
            if statute.effect.effect_type == EffectType::Prohibition
                || statute.effect.effect_type == EffectType::Revoke
            {
                issues.push(ConstitutionalIssue {
                    issue_type: IssueType::AboriginalRightsInfringement,
                    description: "May infringe s.35 Aboriginal rights (Sparrow test)".to_string(),
                    fatal: true,
                    remedial_action: Some(
                        "Demonstrate valid objective and honour of Crown".to_string(),
                    ),
                });
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    /// Build reasoning
    fn build_reasoning(statute: &Statute, issues: &[ConstitutionalIssue]) -> String {
        let mut parts = Vec::new();

        parts.push(format!("Constitutional review of: {}", statute.title));

        if issues.is_empty() {
            parts.push("No constitutional issues identified".to_string());
        } else {
            let fatal_count = issues.iter().filter(|i| i.fatal).count();
            let warning_count = issues.len() - fatal_count;

            parts.push(format!(
                "Issues identified: {} fatal, {} warnings",
                fatal_count, warning_count
            ));

            for issue in issues {
                parts.push(format!("{:?}: {}", issue.issue_type, issue.description));
            }
        }

        parts.join(". ")
    }
}

/// Context for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationContext {
    /// Charter rights potentially engaged
    pub charter_rights_engaged: Vec<CharterRight>,
    /// Rights for which s.1 justification exists
    pub section_1_justified: Vec<CharterRight>,
    /// Enacting body
    pub enacting_body: EnactingBody,
    /// Affects federal undertaking
    pub affects_federal_undertaking: bool,
    /// Affects Aboriginal rights
    pub affects_aboriginal_rights: bool,
    /// Consultation completed
    pub consultation_completed: bool,
}

impl Default for VerificationContext {
    fn default() -> Self {
        Self {
            charter_rights_engaged: Vec::new(),
            section_1_justified: Vec::new(),
            enacting_body: EnactingBody::Parliament,
            affects_federal_undertaking: false,
            affects_aboriginal_rights: false,
            consultation_completed: false,
        }
    }
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Statute being verified
    pub statute_id: String,
    /// Overall validity
    pub valid: bool,
    /// Issues found
    pub issues: Vec<ConstitutionalIssue>,
    /// Reasoning
    pub reasoning: String,
}

/// Constitutional issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalIssue {
    /// Type of issue
    pub issue_type: IssueType,
    /// Description
    pub description: String,
    /// Whether issue is fatal to validity
    pub fatal: bool,
    /// Suggested remedial action
    pub remedial_action: Option<String>,
}

/// Type of constitutional issue
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    /// Charter violation (not saved by s.1)
    CharterViolation,
    /// Charter limitation (potentially saved by s.1)
    CharterLimitation,
    /// Division of powers issue
    DivisionOfPowers,
    /// Interjurisdictional immunity
    InterjurisdictionalImmunity,
    /// Federal paramountcy
    Paramountcy,
    /// Duty to consult triggered
    DutyToConsult,
    /// Aboriginal rights infringement
    AboriginalRightsInfringement,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Province;
    use legalis_core::Effect;

    #[test]
    fn test_verify_no_issues() {
        let statute = Statute::new(
            "TEST",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grants rights"),
        )
        .with_jurisdiction("CA-FED");

        let context = VerificationContext::default();
        let result = ConstitutionalVerifier::verify(&statute, &context);

        assert!(result.valid);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_verify_charter_issue() {
        let statute = Statute::new(
            "TEST",
            "Speech Limitation Act",
            Effect::new(EffectType::Prohibition, "Prohibits certain speech"),
        )
        .with_jurisdiction("CA-FED");

        let context = VerificationContext {
            charter_rights_engaged: vec![CharterRight::FreedomOfExpression],
            section_1_justified: vec![],
            ..Default::default()
        };

        let result = ConstitutionalVerifier::verify(&statute, &context);

        assert!(!result.valid);
        assert!(result.issues.iter().any(|i| i.fatal));
    }

    #[test]
    fn test_verify_charter_saved_by_s1() {
        let statute = Statute::new(
            "TEST",
            "Reasonable Limit Act",
            Effect::new(EffectType::Prohibition, "Prohibition with justification"),
        )
        .with_jurisdiction("CA-FED");

        let context = VerificationContext {
            charter_rights_engaged: vec![CharterRight::FreedomOfExpression],
            section_1_justified: vec![CharterRight::FreedomOfExpression],
            ..Default::default()
        };

        let result = ConstitutionalVerifier::verify(&statute, &context);

        assert!(result.valid);
    }

    #[test]
    fn test_verify_aboriginal_rights() {
        let statute = Statute::new(
            "TEST",
            "Resource Development Act",
            Effect::new(EffectType::Prohibition, "Prohibits traditional practices"),
        )
        .with_jurisdiction("CA-FED");

        let context = VerificationContext {
            affects_aboriginal_rights: true,
            consultation_completed: false,
            ..Default::default()
        };

        let result = ConstitutionalVerifier::verify(&statute, &context);

        assert!(!result.valid);
        assert!(
            result
                .issues
                .iter()
                .any(|i| matches!(i.issue_type, IssueType::AboriginalRightsInfringement))
        );
    }

    #[test]
    fn test_verify_division_of_powers() {
        let statute = Statute::new(
            "TEST",
            "Provincial Criminal Law",
            Effect::new(EffectType::Prohibition, "Criminal prohibition"),
        )
        .with_jurisdiction("CA-FED");

        let context = VerificationContext {
            enacting_body: EnactingBody::ProvincialLegislature(Province::Ontario),
            ..Default::default()
        };

        let result = ConstitutionalVerifier::verify(&statute, &context);

        assert!(!result.valid);
    }
}
