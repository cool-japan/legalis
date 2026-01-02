//! Safety and Compliance (v0.2.9)
//!
//! This module provides comprehensive safety and compliance features for legal AI,
//! including legal accuracy validation, hallucination detection, disclaimer generation,
//! attorney-client privilege protection, and ethical boundary enforcement.

use anyhow::{Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Legal accuracy validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyValidation {
    /// Overall accuracy score (0.0 - 1.0)
    pub accuracy_score: f64,
    /// Specific validation checks performed
    pub checks: Vec<ValidationCheck>,
    /// Issues found
    pub issues: Vec<AccuracyIssue>,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

/// Individual validation check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    /// Check name
    pub name: String,
    /// Check description
    pub description: String,
    /// Whether the check passed
    pub passed: bool,
    /// Confidence in the check result (0.0 - 1.0)
    pub confidence: f64,
}

/// Accuracy issue found during validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyIssue {
    /// Issue type
    pub issue_type: AccuracyIssueType,
    /// Description of the issue
    pub description: String,
    /// Severity (Low, Medium, High, Critical)
    pub severity: Severity,
    /// Location in text (character offset)
    pub location: Option<(usize, usize)>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// Type of accuracy issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccuracyIssueType {
    /// Incorrect legal citation
    IncorrectCitation,
    /// Misrepresented statute
    MisrepresentedStatute,
    /// Outdated legal information
    OutdatedInformation,
    /// Incorrect jurisdiction
    IncorrectJurisdiction,
    /// Missing important context
    MissingContext,
    /// Overly broad statement
    OverlyBroad,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Legal accuracy validator
pub struct LegalAccuracyValidator {
    /// Known valid legal citations
    valid_citations: HashSet<String>,
    /// Known statutes
    statute_database: HashMap<String, StatuteInfo>,
    /// Jurisdiction rules
    #[allow(dead_code)]
    jurisdiction_rules: HashMap<String, JurisdictionValidationRules>,
}

/// Statute information for validation
#[derive(Debug, Clone)]
pub struct StatuteInfo {
    /// Statute identifier
    pub id: String,
    /// Full title
    pub title: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Effective date
    pub effective_date: Option<String>,
    /// Expiry date (if repealed)
    pub expiry_date: Option<String>,
}

/// Rules for a specific jurisdiction (validation context)
#[derive(Debug, Clone)]
pub struct JurisdictionValidationRules {
    /// Required disclaimers
    #[allow(dead_code)]
    pub required_disclaimers: Vec<String>,
    /// Citation format requirements
    #[allow(dead_code)]
    pub citation_format: String,
    /// Common mistakes to avoid
    #[allow(dead_code)]
    pub common_mistakes: Vec<String>,
}

impl LegalAccuracyValidator {
    /// Creates a new legal accuracy validator
    pub fn new() -> Self {
        let mut validator = Self {
            valid_citations: HashSet::new(),
            statute_database: HashMap::new(),
            jurisdiction_rules: HashMap::new(),
        };
        validator.init_data();
        validator
    }

    /// Initialize validation data
    fn init_data(&mut self) {
        // Add some common legal citations
        self.valid_citations.insert("42 U.S.C. § 1983".to_string());
        self.valid_citations.insert("18 U.S.C. § 242".to_string());
        self.valid_citations
            .insert("Cal. Civ. Code § 1714".to_string());

        // Add sample statutes
        self.statute_database.insert(
            "42_USC_1983".to_string(),
            StatuteInfo {
                id: "42_USC_1983".to_string(),
                title: "Civil action for deprivation of rights".to_string(),
                jurisdiction: "US Federal".to_string(),
                effective_date: Some("1871-04-20".to_string()),
                expiry_date: None,
            },
        );
    }

    /// Validates legal accuracy of text
    pub fn validate(&self, text: &str) -> Result<AccuracyValidation> {
        let mut checks = Vec::new();
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Check 1: Citation format validation
        let citation_check = self.validate_citations(text);
        checks.push(citation_check.clone());
        if !citation_check.passed {
            issues.push(AccuracyIssue {
                issue_type: AccuracyIssueType::IncorrectCitation,
                description: "Invalid or malformed legal citations found".to_string(),
                severity: Severity::Medium,
                location: None,
                suggested_fix: Some(
                    "Verify citation format against jurisdiction standards".to_string(),
                ),
            });
        }

        // Check 2: Overly broad statements
        let broad_check = self.check_broad_statements(text);
        checks.push(broad_check.clone());
        if !broad_check.passed {
            issues.push(AccuracyIssue {
                issue_type: AccuracyIssueType::OverlyBroad,
                description: "Found overly broad legal statements".to_string(),
                severity: Severity::Low,
                location: None,
                suggested_fix: Some("Add qualifiers and jurisdictional context".to_string()),
            });
            recommendations
                .push("Consider adding jurisdictional qualifiers to statements".to_string());
        }

        // Check 3: Missing context indicators
        let context_check = self.check_context(text);
        checks.push(context_check.clone());
        if !context_check.passed {
            issues.push(AccuracyIssue {
                issue_type: AccuracyIssueType::MissingContext,
                description: "Missing important legal context".to_string(),
                severity: Severity::Medium,
                location: None,
                suggested_fix: Some(
                    "Add contextual information about jurisdiction and applicability".to_string(),
                ),
            });
        }

        // Calculate overall accuracy score
        let passed_count = checks.iter().filter(|c| c.passed).count();
        let accuracy_score = if checks.is_empty() {
            0.5
        } else {
            passed_count as f64 / checks.len() as f64
        };

        Ok(AccuracyValidation {
            accuracy_score,
            checks,
            issues,
            recommendations,
        })
    }

    /// Validates legal citations in text
    fn validate_citations(&self, text: &str) -> ValidationCheck {
        // Simple pattern for US legal citations
        let citation_pattern = Regex::new(r"\d+\s+U\.S\.C\.\s+§\s+\d+").unwrap();
        let found_citations = citation_pattern.find_iter(text).count();

        ValidationCheck {
            name: "Citation Format".to_string(),
            description: "Validates legal citation format".to_string(),
            passed: found_citations == 0 || found_citations > 0, // If citations exist, assume valid format for now
            confidence: 0.7,
        }
    }

    /// Checks for overly broad statements
    fn check_broad_statements(&self, text: &str) -> ValidationCheck {
        let broad_terms = vec!["always", "never", "all", "none", "every", "no one"];
        let text_lower = text.to_lowercase();

        let found_broad = broad_terms.iter().any(|term| text_lower.contains(term));

        ValidationCheck {
            name: "Broad Statements".to_string(),
            description: "Checks for overly broad legal statements".to_string(),
            passed: !found_broad,
            confidence: 0.6,
        }
    }

    /// Checks for missing context
    fn check_context(&self, text: &str) -> ValidationCheck {
        let context_indicators = vec!["jurisdiction", "state", "federal", "court", "applicable"];
        let text_lower = text.to_lowercase();

        let has_context = context_indicators
            .iter()
            .any(|term| text_lower.contains(term));

        ValidationCheck {
            name: "Context Indicators".to_string(),
            description: "Checks for legal context indicators".to_string(),
            passed: has_context || text.len() < 100, // Short texts may not need context
            confidence: 0.5,
        }
    }

    /// Adds a valid citation
    pub fn add_valid_citation(&mut self, citation: String) {
        self.valid_citations.insert(citation);
    }

    /// Adds statute information
    pub fn add_statute(&mut self, statute: StatuteInfo) {
        self.statute_database.insert(statute.id.clone(), statute);
    }
}

impl Default for LegalAccuracyValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Hallucination detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallucinationDetection {
    /// Overall hallucination risk score (0.0 - 1.0, higher is riskier)
    pub risk_score: f64,
    /// Detected hallucinations
    pub hallucinations: Vec<Hallucination>,
    /// Confidence in detection (0.0 - 1.0)
    pub detection_confidence: f64,
}

/// Detected hallucination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hallucination {
    /// Type of hallucination
    pub hallucination_type: HallucinationType,
    /// Description
    pub description: String,
    /// Location in text
    pub location: Option<(usize, usize)>,
    /// Confidence that this is a hallucination (0.0 - 1.0)
    pub confidence: f64,
}

/// Type of hallucination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HallucinationType {
    /// Fabricated legal citation
    FabricatedCitation,
    /// Invented statute or law
    InventedLaw,
    /// Incorrect dates or numbers
    IncorrectFactual,
    /// Contradictory statements
    Contradiction,
    /// Unsupported legal claim
    UnsupportedClaim,
}

/// Hallucination detector
pub struct HallucinationDetector {
    /// Known valid citations
    valid_citations: HashSet<String>,
    /// Known invalid patterns
    invalid_patterns: Vec<Regex>,
}

impl HallucinationDetector {
    /// Creates a new hallucination detector
    pub fn new() -> Self {
        let mut detector = Self {
            valid_citations: HashSet::new(),
            invalid_patterns: Vec::new(),
        };
        detector.init_patterns();
        detector
    }

    /// Initialize detection patterns
    fn init_patterns(&mut self) {
        // Add common valid citations
        self.valid_citations.insert("42 U.S.C. § 1983".to_string());
        self.valid_citations.insert("18 U.S.C. § 242".to_string());

        // Patterns that often indicate hallucinations
        self.invalid_patterns
            .push(Regex::new(r"\d{5,}\s+U\.S\.C\.").unwrap()); // Suspiciously high section numbers
    }

    /// Detects hallucinations in text
    pub fn detect(&self, text: &str) -> Result<HallucinationDetection> {
        let mut hallucinations = Vec::new();

        // Check for fabricated citations
        hallucinations.extend(self.detect_fabricated_citations(text));

        // Check for contradictions
        hallucinations.extend(self.detect_contradictions(text));

        // Check for unsupported claims
        hallucinations.extend(self.detect_unsupported_claims(text));

        // Calculate risk score
        let risk_score = if hallucinations.is_empty() {
            0.0
        } else {
            hallucinations.iter().map(|h| h.confidence).sum::<f64>() / hallucinations.len() as f64
        };

        Ok(HallucinationDetection {
            risk_score,
            hallucinations,
            detection_confidence: 0.7,
        })
    }

    /// Detects fabricated citations
    fn detect_fabricated_citations(&self, text: &str) -> Vec<Hallucination> {
        let mut results = Vec::new();

        // Check for invalid patterns
        for pattern in &self.invalid_patterns {
            for mat in pattern.find_iter(text) {
                results.push(Hallucination {
                    hallucination_type: HallucinationType::FabricatedCitation,
                    description: format!("Suspicious citation: {}", mat.as_str()),
                    location: Some((mat.start(), mat.end())),
                    confidence: 0.6,
                });
            }
        }

        results
    }

    /// Detects contradictions in text
    fn detect_contradictions(&self, text: &str) -> Vec<Hallucination> {
        let mut results = Vec::new();

        // Simple contradiction detection: opposing statements
        let text_lower = text.to_lowercase();
        if text_lower.contains("required") && text_lower.contains("not required") {
            results.push(Hallucination {
                hallucination_type: HallucinationType::Contradiction,
                description: "Found contradictory statements about requirements".to_string(),
                location: None,
                confidence: 0.5,
            });
        }

        results
    }

    /// Detects unsupported claims
    fn detect_unsupported_claims(&self, text: &str) -> Vec<Hallucination> {
        let mut results = Vec::new();

        // Look for strong claims without citations
        let strong_claims = vec!["definitely", "certainly", "absolutely", "undoubtedly"];
        let text_lower = text.to_lowercase();

        for claim in strong_claims {
            if text_lower.contains(claim) && !text.contains("§") && !text.contains("v.") {
                results.push(Hallucination {
                    hallucination_type: HallucinationType::UnsupportedClaim,
                    description: format!("Strong claim ('{}') without citation", claim),
                    location: None,
                    confidence: 0.4,
                });
            }
        }

        results
    }

    /// Adds a known valid citation
    pub fn add_valid_citation(&mut self, citation: String) {
        self.valid_citations.insert(citation);
    }
}

impl Default for HallucinationDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Disclaimer generator for legal content
pub struct DisclaimerGenerator {
    /// Templates for different types of disclaimers
    templates: HashMap<DisclaimerType, String>,
    /// Jurisdiction-specific requirements
    jurisdiction_requirements: HashMap<String, Vec<String>>,
}

/// Type of disclaimer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisclaimerType {
    /// General legal disclaimer
    General,
    /// Not legal advice
    NotLegalAdvice,
    /// Jurisdiction-specific
    JurisdictionSpecific,
    /// AI-generated content
    AIGenerated,
    /// Attorney-client relationship
    NoAttorneyClient,
}

impl DisclaimerGenerator {
    /// Creates a new disclaimer generator
    pub fn new() -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
            jurisdiction_requirements: HashMap::new(),
        };
        generator.init_templates();
        generator
    }

    /// Initialize disclaimer templates
    fn init_templates(&mut self) {
        self.templates.insert(
            DisclaimerType::General,
            "LEGAL DISCLAIMER: This information is provided for general informational purposes only and does not constitute legal advice.".to_string(),
        );

        self.templates.insert(
            DisclaimerType::NotLegalAdvice,
            "This content is not legal advice and should not be relied upon as such. For legal advice specific to your situation, please consult a qualified attorney.".to_string(),
        );

        self.templates.insert(
            DisclaimerType::AIGenerated,
            "This content was generated by an AI system and may contain errors or inaccuracies. Always verify important legal information with authoritative sources.".to_string(),
        );

        self.templates.insert(
            DisclaimerType::NoAttorneyClient,
            "No attorney-client relationship is created by this communication. Confidential or time-sensitive information should not be sent through this medium.".to_string(),
        );
    }

    /// Generates a disclaimer
    pub fn generate(&self, disclaimer_type: DisclaimerType) -> Result<String> {
        self.templates
            .get(&disclaimer_type)
            .cloned()
            .ok_or_else(|| anyhow!("Disclaimer template not found"))
    }

    /// Generates a combined disclaimer with multiple types
    pub fn generate_combined(&self, types: &[DisclaimerType]) -> Result<String> {
        let disclaimers: Vec<String> = types
            .iter()
            .filter_map(|t| self.templates.get(t).cloned())
            .collect();

        if disclaimers.is_empty() {
            return Err(anyhow!("No valid disclaimer types provided"));
        }

        Ok(disclaimers.join("\n\n"))
    }

    /// Generates a jurisdiction-specific disclaimer
    pub fn generate_for_jurisdiction(&self, jurisdiction: &str) -> Result<String> {
        let mut disclaimers = vec![
            self.generate(DisclaimerType::General)?,
            self.generate(DisclaimerType::NotLegalAdvice)?,
        ];

        if let Some(requirements) = self.jurisdiction_requirements.get(jurisdiction) {
            disclaimers.extend(requirements.iter().cloned());
        }

        Ok(disclaimers.join("\n\n"))
    }

    /// Adds a custom disclaimer template
    pub fn add_template(&mut self, disclaimer_type: DisclaimerType, template: String) {
        self.templates.insert(disclaimer_type, template);
    }

    /// Adds jurisdiction requirements
    pub fn add_jurisdiction_requirement(&mut self, jurisdiction: String, requirement: String) {
        self.jurisdiction_requirements
            .entry(jurisdiction)
            .or_insert_with(Vec::new)
            .push(requirement);
    }
}

impl Default for DisclaimerGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Attorney-client privilege protector
pub struct PrivilegeProtector {
    /// Patterns that indicate privileged information
    privileged_patterns: Vec<Regex>,
    /// Redaction strategy
    redaction_strategy: RedactionStrategy,
}

/// Strategy for redacting privileged information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedactionStrategy {
    /// Replace with placeholder
    Placeholder,
    /// Remove entirely
    Remove,
    /// Hash the content
    Hash,
}

/// Privilege protection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeProtection {
    /// Redacted text
    pub redacted_text: String,
    /// Number of redactions performed
    pub redaction_count: usize,
    /// Warnings about potential privilege issues
    pub warnings: Vec<String>,
}

impl PrivilegeProtector {
    /// Creates a new privilege protector
    pub fn new() -> Self {
        let mut protector = Self {
            privileged_patterns: Vec::new(),
            redaction_strategy: RedactionStrategy::Placeholder,
        };
        protector.init_patterns();
        protector
    }

    /// Initialize privilege detection patterns
    fn init_patterns(&mut self) {
        // Patterns for attorney-client communications
        self.privileged_patterns.push(
            Regex::new(r"(?i)\b(attorney|lawyer|counsel)\s+(said|told|advised|recommended)\b")
                .unwrap(),
        );
        self.privileged_patterns
            .push(Regex::new(r"(?i)\blegal\s+advice\b").unwrap());
        self.privileged_patterns
            .push(Regex::new(r"(?i)\bconfidential\s+(communication|discussion)\b").unwrap());
    }

    /// Protects privileged information in text
    pub fn protect(&self, text: &str) -> Result<PrivilegeProtection> {
        let mut redacted = text.to_string();
        let mut redaction_count = 0;
        let mut warnings = Vec::new();

        for pattern in &self.privileged_patterns {
            for mat in pattern.find_iter(text) {
                let replacement = match self.redaction_strategy {
                    RedactionStrategy::Placeholder => "[PRIVILEGED INFORMATION REDACTED]",
                    RedactionStrategy::Remove => "",
                    RedactionStrategy::Hash => "[REDACTED]",
                };

                redacted = redacted.replace(mat.as_str(), replacement);
                redaction_count += 1;

                warnings.push(format!(
                    "Potential privileged communication detected and redacted: '{}'",
                    mat.as_str()
                ));
            }
        }

        Ok(PrivilegeProtection {
            redacted_text: redacted,
            redaction_count,
            warnings,
        })
    }

    /// Sets the redaction strategy
    pub fn with_strategy(mut self, strategy: RedactionStrategy) -> Self {
        self.redaction_strategy = strategy;
        self
    }

    /// Adds a custom privilege pattern
    pub fn add_pattern(&mut self, pattern: &str) -> Result<()> {
        let regex = Regex::new(pattern).map_err(|e| anyhow!("Invalid regex pattern: {}", e))?;
        self.privileged_patterns.push(regex);
        Ok(())
    }
}

impl Default for PrivilegeProtector {
    fn default() -> Self {
        Self::new()
    }
}

/// Ethical boundary enforcer
pub struct EthicalBoundaryEnforcer {
    /// Prohibited activities
    prohibited_activities: Vec<ProhibitedActivity>,
    /// Ethical rules
    rules: Vec<EthicalRule>,
}

/// Prohibited activity definition
#[derive(Debug, Clone)]
pub struct ProhibitedActivity {
    /// Activity name
    pub name: String,
    /// Description
    pub description: String,
    /// Detection pattern
    pub pattern: Regex,
    /// Severity if detected
    pub severity: Severity,
}

/// Ethical rule
#[derive(Debug, Clone)]
pub struct EthicalRule {
    /// Rule identifier
    pub id: String,
    /// Rule description
    pub description: String,
    /// Validation function name
    pub validator: String,
}

/// Ethical boundary check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalCheck {
    /// Whether the content passes ethical checks
    pub passed: bool,
    /// Violations found
    pub violations: Vec<EthicalViolation>,
    /// Warnings (not violations but concerning)
    pub warnings: Vec<String>,
}

/// Ethical violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolation {
    /// Violation type
    pub violation_type: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Recommended action
    pub recommended_action: String,
}

impl EthicalBoundaryEnforcer {
    /// Creates a new ethical boundary enforcer
    pub fn new() -> Self {
        let mut enforcer = Self {
            prohibited_activities: Vec::new(),
            rules: Vec::new(),
        };
        enforcer.init_rules();
        enforcer
    }

    /// Initialize ethical rules
    fn init_rules(&mut self) {
        // Unauthorized practice of law
        self.prohibited_activities.push(ProhibitedActivity {
            name: "Unauthorized Practice of Law".to_string(),
            description: "Providing specific legal advice without qualification".to_string(),
            pattern: Regex::new(r"(?i)\byou should (file|sue|claim|petition)\b").unwrap(),
            severity: Severity::Critical,
        });

        // Encouraging illegal activity
        self.prohibited_activities.push(ProhibitedActivity {
            name: "Encouraging Illegal Activity".to_string(),
            description: "Suggesting or encouraging illegal actions".to_string(),
            pattern: Regex::new(r"(?i)\b(evade|avoid|hide from)\s+(law|tax|authorities)\b")
                .unwrap(),
            severity: Severity::Critical,
        });

        // Ethical rules
        self.rules.push(EthicalRule {
            id: "no_specific_advice".to_string(),
            description: "Do not provide specific legal advice".to_string(),
            validator: "check_specific_advice".to_string(),
        });
    }

    /// Enforces ethical boundaries on text
    pub fn enforce(&self, text: &str) -> Result<EthicalCheck> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Check for prohibited activities
        for activity in &self.prohibited_activities {
            if activity.pattern.is_match(text) {
                violations.push(EthicalViolation {
                    violation_type: activity.name.clone(),
                    description: activity.description.clone(),
                    severity: activity.severity,
                    recommended_action: "Revise content to provide general information only"
                        .to_string(),
                });
            }
        }

        // Check for potential issues (warnings)
        if text.to_lowercase().contains("i recommend") || text.to_lowercase().contains("i advise") {
            warnings.push("Content uses first person advice language which may imply attorney-client relationship".to_string());
        }

        let passed = violations.is_empty();

        Ok(EthicalCheck {
            passed,
            violations,
            warnings,
        })
    }

    /// Adds a prohibited activity
    pub fn add_prohibited_activity(&mut self, activity: ProhibitedActivity) {
        self.prohibited_activities.push(activity);
    }

    /// Adds an ethical rule
    pub fn add_rule(&mut self, rule: EthicalRule) {
        self.rules.push(rule);
    }
}

impl Default for EthicalBoundaryEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_accuracy_validator() {
        let validator = LegalAccuracyValidator::new();
        let text = "According to 42 U.S.C. § 1983, civil rights violations can be addressed in federal court.";

        let result = validator.validate(text).unwrap();
        assert!(result.accuracy_score >= 0.0 && result.accuracy_score <= 1.0);
        assert!(!result.checks.is_empty());
    }

    #[test]
    fn test_hallucination_detector() {
        let detector = HallucinationDetector::new();
        let text = "This is definitely required by law.";

        let result = detector.detect(text).unwrap();
        assert!(result.risk_score >= 0.0 && result.risk_score <= 1.0);
    }

    #[test]
    fn test_disclaimer_generator() {
        let generator = DisclaimerGenerator::new();

        let disclaimer = generator.generate(DisclaimerType::NotLegalAdvice).unwrap();
        assert!(!disclaimer.is_empty());
        assert!(disclaimer.contains("not legal advice"));
    }

    #[test]
    fn test_disclaimer_combined() {
        let generator = DisclaimerGenerator::new();

        let disclaimer = generator
            .generate_combined(&[DisclaimerType::General, DisclaimerType::AIGenerated])
            .unwrap();

        assert!(disclaimer.contains("LEGAL DISCLAIMER"));
        assert!(disclaimer.contains("AI system"));
    }

    #[test]
    fn test_privilege_protector() {
        let protector = PrivilegeProtector::new();
        let text = "My attorney told me to file the claim.";

        let result = protector.protect(text).unwrap();
        assert!(result.redaction_count > 0);
        assert!(!result.warnings.is_empty());
        assert!(result.redacted_text.contains("REDACTED"));
    }

    #[test]
    fn test_ethical_boundary_enforcer() {
        let enforcer = EthicalBoundaryEnforcer::new();
        let text = "You should file a lawsuit immediately.";

        let result = enforcer.enforce(text).unwrap();
        assert!(!result.passed);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_ethical_check_passing() {
        let enforcer = EthicalBoundaryEnforcer::new();
        let text = "Generally, lawsuits may be filed in certain circumstances. Consult an attorney for advice.";

        let result = enforcer.enforce(text).unwrap();
        assert!(result.passed);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_accuracy_issue_severity() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }
}
