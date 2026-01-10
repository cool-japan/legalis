//! Risk Analysis System (リスク分析システム)
//!
//! This module provides comprehensive risk analysis for contracts and legal documents,
//! detecting unfair clauses, legal violations, and compliance issues.
//!
//! # Features
//!
//! - **Multi-Dimensional Risk Detection** (多次元リスク検出)
//!   - Consumer protection violations (消費者保護法違反)
//!   - Labor law violations (労働法違反)
//!   - Unfair contract terms (不当契約条項)
//!   - Ambiguous clauses (曖昧な条項)
//!   - Data protection issues (個人情報保護問題)
//!
//! - **Severity Classification** (深刻度分類)
//!   - Critical: Severe legal violations requiring immediate action
//!   - High: Serious issues that must be corrected
//!   - Medium: Notable issues requiring attention
//!   - Low: Minor issues with recommendations
//!
//! - **Rule-Based Detection Engine** (ルールベース検出エンジン)
//!   - Extensible rule system
//!   - Contract-type specific rules
//!   - Confidence scoring
//!   - Legal reference tracking
//!
//! - **Comprehensive Reporting** (包括的レポート)
//!   - Risk severity breakdown
//!   - Overall risk scoring (0-100)
//!   - Detailed findings with locations
//!   - Actionable recommendations
//!
//! # Legal Coverage
//!
//! ## Consumer Contract Act (消費者契約法)
//! - Article 8: Liability exemption clause restrictions
//! - Article 9: Excessive penalty limitations
//! - Article 10: General unfair terms prohibition
//!
//! ## Labor Standards Act (労働基準法)
//! - Article 16: Prohibition of penalty stipulations
//! - Article 18: Prohibition of forced savings
//! - Illegal non-compete clauses
//!
//! ## Other Legal Areas
//! - Personal Information Protection Act (個人情報保護法)
//! - Civil Code provisions (民法)
//! - General contract principles
//!
//! # Risk Categories
//!
//! 1. **Consumer Protection** (消費者保護法違反)
//!    - Full exemption clauses
//!    - Excessive cancellation fees
//!    - Consumer disadvantage clauses
//!
//! 2. **Labor Law** (労働法違反)
//!    - Illegal penalty deductions
//!    - Overly broad non-compete clauses
//!    - Forced savings provisions
//!
//! 3. **Unfair Terms** (不当契約条項)
//!    - One-sided modification rights
//!    - Unreasonable burdens
//!    - Unconscionable terms
//!
//! 4. **Ambiguous Clauses** (曖昧な条項)
//!    - Vague terminology
//!    - Missing definitions
//!    - Unclear obligations
//!
//! 5. **Data Protection** (個人情報保護)
//!    - Missing consent provisions
//!    - Inadequate privacy protections
//!
//! # Examples
//!
//! ## Basic Risk Analysis
//!
//! ```rust
//! use legalis_jp::risk_analysis::*;
//!
//! // Create a contract document
//! let mut document = ContractDocument::new("Employment Contract", ContractType::Employment);
//! document.add_clause("Article 5", "退職時には違約金を定める。");
//! document.add_clause("Article 10", "個人情報を第三者に提供する。");
//!
//! // Analyze for risks
//! let detector = RiskDetector::new();
//! let report = detector.analyze(&document).unwrap();
//!
//! // Check results
//! println!("Overall Risk Score: {}/100", report.overall_risk_score);
//! println!("Critical Risks: {}", report.critical_count());
//! println!("Total Findings: {}", report.findings.len());
//!
//! // Review findings
//! for finding in &report.findings {
//!     println!("{} {}: {}",
//!         finding.severity.emoji(),
//!         finding.severity.english_name(),
//!         finding.issue_description
//!     );
//! }
//! ```
//!
//! ## Quick Analysis
//!
//! ```rust
//! use legalis_jp::risk_analysis::*;
//!
//! let report = quick_analyze(
//!     "Consumer Sales Contract",
//!     ContractType::Consumer,
//!     "当社は一切責任を負いません。解約時には全額を違約金として支払うものとします。"
//! ).unwrap();
//!
//! if report.has_serious_risks() {
//!     println!("WARNING: Contract has serious legal issues!");
//!     for finding in report.findings {
//!         if finding.severity == RiskSeverity::Critical {
//!             println!("❌ {}", finding.issue_description);
//!             println!("   Recommendation: {}", finding.recommendation);
//!         }
//!     }
//! }
//! ```
//!
//! ## Custom Rule Engine
//!
//! ```rust
//! use legalis_jp::risk_analysis::*;
//!
//! // Create custom rule engine
//! let mut engine = RuleEngine::empty();
//! // Add specific rules as needed
//! // engine.add_rule(Box::new(MyCustomRule));
//!
//! let detector = RiskDetector::with_rule_engine(engine);
//! ```
//!
//! ## Analyzing Multiple Documents
//!
//! ```rust
//! use legalis_jp::risk_analysis::*;
//!
//! let detector = RiskDetector::new();
//! let contracts = vec![
//!     ("Employment Contract A", ContractType::Employment, "contract text A"),
//!     ("Employment Contract B", ContractType::Employment, "contract text B"),
//! ];
//!
//! for (title, contract_type, text) in contracts {
//!     match quick_analyze(title, contract_type, text) {
//!         Ok(report) => {
//!             println!("{}: Risk Score {}/100", title, report.overall_risk_score);
//!         }
//!         Err(e) => eprintln!("Analysis failed: {}", e),
//!     }
//! }
//! ```
//!
//! # Risk Severity Levels
//!
//! | Severity | Score | Description | Example |
//! |----------|-------|-------------|---------|
//! | **Critical** | 4 | Severe legal violations | Full liability exemption in consumer contract |
//! | **High** | 3 | Serious issues requiring correction | Illegal penalty clause in employment contract |
//! | **Medium** | 2 | Notable issues requiring attention | Ambiguous jurisdiction clause |
//! | **Low** | 1 | Minor issues with recommendations | Vague terminology |
//!
//! # Detection Rules
//!
//! The system includes built-in detection rules:
//!
//! - `ConsumerProtectionRule`: Detects Consumer Contract Act violations
//! - `LaborLawRule`: Detects Labor Standards Act violations
//! - `AmbiguousClauseRule`: Detects unclear or vague clauses
//! - `JurisdictionClauseRule`: Detects unfair jurisdiction provisions
//! - `DataProtectionRule`: Detects privacy and data protection issues
//!
//! # Best Practices
//!
//! 1. **Always review high and critical findings** before signing contracts
//! 2. **Consult legal experts** for critical violations
//! 3. **Use appropriate contract types** for accurate rule application
//! 4. **Review recommendations** for each finding
//! 5. **Perform regular analysis** of template contracts
//!
//! # Performance Considerations
//!
//! - Rule-based detection is fast (milliseconds for typical contracts)
//! - Memory usage scales linearly with document size
//! - Can analyze multiple documents in parallel
//! - Suitable for real-time validation in contract editors

pub mod detector;
pub mod error;
pub mod rules;
pub mod types;

// Re-export commonly used types and functions
pub use detector::{ContractClause, ContractDocument, RiskDetector, quick_analyze};
pub use error::{Result, RiskAnalysisError};
pub use rules::{DetectionRule, RuleEngine};
pub use types::*;
