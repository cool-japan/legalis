//! Commerce Clause Analysis
//!
//! Analyzes state laws under the Dormant Commerce Clause doctrine.

use serde::{Deserialize, Serialize};

/// Dormant Commerce Clause test type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DormantCommerceClauseTest {
    /// State law discriminates against interstate commerce (strict scrutiny)
    Discrimination,

    /// State law burdens interstate commerce (Pike balancing)
    PikeBalancing,

    /// No Commerce Clause issue
    None,
}

impl DormantCommerceClauseTest {
    /// Get human-readable description.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Discrimination => "Discrimination Test (strict scrutiny - nearly per se invalid)",
            Self::PikeBalancing => "Pike Balancing Test (burden vs. benefit)",
            Self::None => "No Commerce Clause Issue",
        }
    }
}

/// Commerce Clause analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceClauseAnalysis {
    /// State law being analyzed
    pub state_law: String,

    /// State enacting the law
    pub state: String,

    /// Does law discriminate against interstate commerce?
    pub discriminates: bool,

    /// Discrimination explanation
    pub discrimination_basis: Option<String>,

    /// Does market participant exception apply?
    pub market_participant_exception: bool,

    /// Does Congress authorize state law?
    pub congressional_authorization: bool,

    /// Burden on interstate commerce (if non-discriminatory)
    pub burden_description: Option<String>,

    /// Local benefit of state law
    pub local_benefit_description: Option<String>,

    /// Subject matter
    pub subject_matter: Option<String>,
}

impl CommerceClauseAnalysis {
    /// Create new Commerce Clause analysis.
    #[must_use]
    pub fn new(state: impl Into<String>, state_law: impl Into<String>) -> Self {
        Self {
            state_law: state_law.into(),
            state: state.into(),
            discriminates: false,
            discrimination_basis: None,
            market_participant_exception: false,
            congressional_authorization: false,
            burden_description: None,
            local_benefit_description: None,
            subject_matter: None,
        }
    }

    /// Set discrimination.
    #[must_use]
    pub fn with_discrimination(mut self, basis: impl Into<String>) -> Self {
        self.discriminates = true;
        self.discrimination_basis = Some(basis.into());
        self
    }

    /// Enable market participant exception.
    #[must_use]
    pub fn with_market_participant_exception(mut self) -> Self {
        self.market_participant_exception = true;
        self
    }

    /// Enable congressional authorization.
    #[must_use]
    pub fn with_congressional_authorization(mut self) -> Self {
        self.congressional_authorization = true;
        self
    }

    /// Set burden on interstate commerce.
    #[must_use]
    pub fn with_burden(mut self, burden: impl Into<String>) -> Self {
        self.burden_description = Some(burden.into());
        self
    }

    /// Set local benefit.
    #[must_use]
    pub fn with_local_benefit(mut self, benefit: impl Into<String>) -> Self {
        self.local_benefit_description = Some(benefit.into());
        self
    }

    /// Set subject matter.
    #[must_use]
    pub fn with_subject_matter(mut self, subject: impl Into<String>) -> Self {
        self.subject_matter = Some(subject.into());
        self
    }

    /// Analyze under Dormant Commerce Clause.
    #[must_use]
    pub fn analyze(&self) -> CommerceClauseResult {
        let mut reasons = Vec::new();

        // Check for exceptions first
        if self.congressional_authorization {
            reasons.push(
                "Congressional authorization: State law is authorized by Congress".to_string(),
            );
            reasons.push("Result: State law is VALID (Commerce Clause does not apply)".to_string());

            return CommerceClauseResult {
                test: DormantCommerceClauseTest::None,
                valid: true,
                confidence: 0.95,
                reasoning: reasons,
            };
        }

        if self.market_participant_exception {
            reasons.push(
                "Market participant exception: State acting as buyer/seller, not regulator"
                    .to_string(),
            );
            reasons.push(
                "Result: State law is VALID (exception to Dormant Commerce Clause)".to_string(),
            );

            return CommerceClauseResult {
                test: DormantCommerceClauseTest::None,
                valid: true,
                confidence: 0.90,
                reasoning: reasons,
            };
        }

        // Check for discrimination
        if self.discriminates {
            reasons.push("State law DISCRIMINATES against interstate commerce:".to_string());
            if let Some(basis) = &self.discrimination_basis {
                reasons.push(format!("  - {basis}"));
            }
            reasons.push("Strict scrutiny applies: Law is invalid unless:".to_string());
            reasons.push("  - Serves legitimate local purpose".to_string());
            reasons.push("  - No non-discriminatory alternatives available".to_string());
            reasons.push(
                "Result: State law is likely INVALID (discrimination is nearly per se invalid)"
                    .to_string(),
            );

            return CommerceClauseResult {
                test: DormantCommerceClauseTest::Discrimination,
                valid: false,
                confidence: 0.85, // High confidence discrimination is invalid
                reasoning: reasons,
            };
        }

        // Pike balancing for non-discriminatory burdens
        if self.burden_description.is_some() || self.local_benefit_description.is_some() {
            reasons.push(
                "State law is non-discriminatory but may burden interstate commerce".to_string(),
            );
            reasons.push("Pike v. Bruce Church balancing test applies:".to_string());

            if let Some(burden) = &self.burden_description {
                reasons.push(format!("Burden on interstate commerce: {burden}"));
            }

            if let Some(benefit) = &self.local_benefit_description {
                reasons.push(format!("Local benefit: {benefit}"));
            }

            // Simple heuristic: if burden described but no benefit, likely invalid
            let likely_invalid =
                self.burden_description.is_some() && self.local_benefit_description.is_none();

            if likely_invalid {
                reasons.push("Analysis: Burden appears to outweigh local benefits".to_string());
                reasons.push("Result: State law is likely INVALID".to_string());

                return CommerceClauseResult {
                    test: DormantCommerceClauseTest::PikeBalancing,
                    valid: false,
                    confidence: 0.65, // Lower confidence - Pike balancing is fact-intensive
                    reasoning: reasons,
                };
            } else {
                reasons.push("Analysis: Burden may be justified by local benefits".to_string());
                reasons
                    .push("Result: State law is likely VALID (fact-intensive inquiry)".to_string());

                return CommerceClauseResult {
                    test: DormantCommerceClauseTest::PikeBalancing,
                    valid: true,
                    confidence: 0.60, // Low confidence - depends on facts
                    reasoning: reasons,
                };
            }
        }

        // No Commerce Clause issue identified
        reasons.push("No Dormant Commerce Clause issue identified:".to_string());
        reasons.push("  - No discrimination against interstate commerce".to_string());
        reasons.push("  - No burden on interstate commerce described".to_string());
        reasons.push("Result: State law appears VALID under Commerce Clause".to_string());

        CommerceClauseResult {
            test: DormantCommerceClauseTest::None,
            valid: true,
            confidence: 0.75,
            reasoning: reasons,
        }
    }
}

/// Result of Commerce Clause analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceClauseResult {
    /// Test applied
    pub test: DormantCommerceClauseTest,

    /// Whether state law is valid
    pub valid: bool,

    /// Confidence in result (0.0-1.0)
    pub confidence: f64,

    /// Detailed reasoning
    pub reasoning: Vec<String>,
}

impl CommerceClauseResult {
    /// Generate summary explanation.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut report = String::new();

        report.push_str("# Dormant Commerce Clause Analysis\n\n");
        report.push_str(&format!(
            "**Result**: {}\n\n",
            if self.valid {
                "STATE LAW IS VALID"
            } else {
                "STATE LAW IS INVALID"
            }
        ));

        report.push_str(&format!("**Test**: {}\n\n", self.test.description()));
        report.push_str(&format!(
            "**Confidence**: {:.1}%\n\n",
            self.confidence * 100.0
        ));

        report.push_str("## Analysis\n\n");
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
    fn test_dormant_commerce_clause_test_description() {
        assert_eq!(
            DormantCommerceClauseTest::Discrimination.description(),
            "Discrimination Test (strict scrutiny - nearly per se invalid)"
        );
        assert_eq!(
            DormantCommerceClauseTest::PikeBalancing.description(),
            "Pike Balancing Test (burden vs. benefit)"
        );
    }

    #[test]
    fn test_discrimination_against_out_of_state() {
        let analysis =
            CommerceClauseAnalysis::new("NJ", "Prohibition on out-of-state waste imports")
                .with_discrimination(
                    "Explicitly prohibits out-of-state waste while allowing in-state waste",
                )
                .with_subject_matter("Waste disposal");

        let result = analysis.analyze();

        assert_eq!(result.test, DormantCommerceClauseTest::Discrimination);
        assert!(!result.valid);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_market_participant_exception() {
        let analysis = CommerceClauseAnalysis::new(
            "MD",
            "Preference for Maryland residents in state park jobs",
        )
        .with_market_participant_exception()
        .with_subject_matter("State employment");

        let result = analysis.analyze();

        assert_eq!(result.test, DormantCommerceClauseTest::None);
        assert!(result.valid);
        assert!(result.confidence > 0.85);
        assert!(
            result
                .reasoning
                .iter()
                .any(|r| r.contains("Market participant"))
        );
    }

    #[test]
    fn test_congressional_authorization() {
        let analysis = CommerceClauseAnalysis::new("CA", "State insurance regulation")
            .with_congressional_authorization()
            .with_subject_matter("Insurance");

        let result = analysis.analyze();

        assert_eq!(result.test, DormantCommerceClauseTest::None);
        assert!(result.valid);
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_pike_balancing_burden_outweighs_benefit() {
        let analysis = CommerceClauseAnalysis::new(
            "AZ",
            "Requirement for truck mudguards (different from federal standard)",
        )
        .with_burden("Requires trucks to change mudguards when entering Arizona, significant cost")
        .with_subject_matter("Truck safety");

        let result = analysis.analyze();

        assert_eq!(result.test, DormantCommerceClauseTest::PikeBalancing);
        assert!(!result.valid); // Burden with no stated benefit
        assert!(result.confidence < 0.7); // Pike balancing is uncertain
    }

    #[test]
    fn test_pike_balancing_with_benefits() {
        let analysis =
            CommerceClauseAnalysis::new("IL", "Inspection requirements for imported produce")
                .with_burden("Requires inspection of out-of-state produce")
                .with_local_benefit("Protects Illinois consumers from contaminated produce")
                .with_subject_matter("Food safety");

        let result = analysis.analyze();

        assert_eq!(result.test, DormantCommerceClauseTest::PikeBalancing);
        assert!(result.valid); // Has stated benefits
    }

    #[test]
    fn test_no_commerce_clause_issue() {
        let analysis = CommerceClauseAnalysis::new("TX", "General criminal law (assault)")
            .with_subject_matter("Criminal law");

        let result = analysis.analyze();

        assert_eq!(result.test, DormantCommerceClauseTest::None);
        assert!(result.valid);
    }

    #[test]
    fn test_commerce_clause_result_summary() {
        let result = CommerceClauseResult {
            test: DormantCommerceClauseTest::Discrimination,
            valid: false,
            confidence: 0.85,
            reasoning: vec!["Discriminates against out-of-state commerce".to_string()],
        };

        let summary = result.summary();

        assert!(summary.contains("STATE LAW IS INVALID"));
        assert!(summary.contains("Discrimination Test"));
        assert!(summary.contains("85.0%"));
    }

    #[test]
    fn test_builder_pattern() {
        let analysis = CommerceClauseAnalysis::new("CA", "State law")
            .with_discrimination("Favors CA businesses")
            .with_subject_matter("Business regulation");

        assert!(analysis.discriminates);
        assert!(analysis.discrimination_basis.is_some());
        assert_eq!(
            analysis.subject_matter,
            Some("Business regulation".to_string())
        );
    }
}
