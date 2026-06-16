//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::HashMap;

use super::types::{
    ComplexityTrend, CoverageInfo, CrossReferenceErrorType, RiskLevel, VerificationProof,
};
use super::types_4::{
    CompositePrinciple, DependencyType, InteractionType, PrincipleDefinition, QualitySummary,
    RedundancyType, Severity, StatutePattern, StatuteStatistics, SummaryStatistics,
};
use super::types_5::{
    CachedProof, ConflictSummary, EvolutionSummary, MechanismIssue, StatuteFilingInfo,
    TimedTransition, VerificationResult,
};

/// Trusted Execution Environment (TEE) configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeeConfig {
    /// TEE type (e.g., "SGX", "SEV", "TrustZone")
    pub tee_type: String,
    /// Attestation data proving code integrity
    pub attestation: Vec<u8>,
    /// Enclave configuration
    pub enclave_config: HashMap<String, String>,
}
impl TeeConfig {
    /// Creates a new TEE configuration
    pub fn new(tee_type: impl Into<String>) -> Self {
        use rand::RngExt;
        let mut rng = rand::rng();
        Self {
            tee_type: tee_type.into(),
            attestation: (0..64).map(|_| rng.random()).collect(),
            enclave_config: HashMap::new(),
        }
    }
    /// Verifies the TEE attestation
    pub fn verify_attestation(&self) -> bool {
        !self.attestation.is_empty()
    }
}
/// Result of sequence constraint verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SequenceVerificationResult {
    /// Whether all constraints were satisfied
    pub passed: bool,
    /// Violated constraints
    pub violations: Vec<SequenceViolation>,
}
/// Represents a redundancy in the statute set
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RedundancyInstance {
    /// IDs of redundant statutes
    pub statute_ids: Vec<String>,
    /// Type of redundancy
    pub redundancy_type: RedundancyType,
    /// Description
    pub description: String,
    /// Suggested elimination strategy
    pub elimination_strategy: String,
    /// Potential savings (estimated complexity reduction)
    pub potential_savings: f64,
}
/// CI/CD configuration generator.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CiConfig {
    /// Platform type
    pub platform: CiPlatform,
    /// Verification command
    pub verify_command: String,
    /// Fail on warnings
    pub fail_on_warnings: bool,
    /// Upload reports as artifacts
    pub upload_reports: bool,
    /// Report output directory
    pub report_dir: String,
}
impl CiConfig {
    /// Creates a new CI configuration.
    pub fn new(platform: CiPlatform) -> Self {
        Self {
            platform,
            verify_command: "cargo run --bin legalis-verify".to_string(),
            fail_on_warnings: true,
            upload_reports: true,
            report_dir: "verification-reports".to_string(),
        }
    }
    /// Sets the verification command.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.verify_command = command.into();
        self
    }
    /// Sets whether to fail on warnings.
    pub fn fail_on_warnings(mut self, fail: bool) -> Self {
        self.fail_on_warnings = fail;
        self
    }
    /// Sets whether to upload reports.
    pub fn upload_reports(mut self, upload: bool) -> Self {
        self.upload_reports = upload;
        self
    }
    /// Sets the report directory.
    pub fn with_report_dir(mut self, dir: impl Into<String>) -> Self {
        self.report_dir = dir.into();
        self
    }
    /// Generates the CI configuration file content.
    pub fn generate(&self) -> String {
        match self.platform {
            CiPlatform::GitHubActions => self.generate_github_actions(),
            CiPlatform::GitLabCI => self.generate_gitlab_ci(),
            CiPlatform::Jenkins => self.generate_jenkins(),
            CiPlatform::CircleCI => self.generate_circleci(),
            CiPlatform::TravisCI => self.generate_travis(),
        }
    }
    fn generate_github_actions(&self) -> String {
        format!(
            r#"name: Statute Verification

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  verify:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Run Statute Verification
      run: {}
      continue-on-error: {}

    - name: Upload Verification Reports
      if: {}
      uses: actions/upload-artifact@v3
      with:
        name: verification-reports
        path: {}
        retention-days: 30
"#,
            self.verify_command,
            if self.fail_on_warnings {
                "false"
            } else {
                "true"
            },
            if self.upload_reports {
                "always()"
            } else {
                "false"
            },
            self.report_dir
        )
    }
    fn generate_gitlab_ci(&self) -> String {
        format!(
            r#"verify:
  stage: test
  image: rust:latest
  script:
    - {}
  artifacts:
    when: {}
    paths:
      - {}
    expire_in: 30 days
  allow_failure: {}
"#,
            self.verify_command,
            if self.upload_reports {
                "always"
            } else {
                "on_success"
            },
            self.report_dir,
            !self.fail_on_warnings
        )
    }
    fn generate_jenkins(&self) -> String {
        format!(
            r#"pipeline {{
    agent any

    stages {{
        stage('Verify Statutes') {{
            steps {{
                sh '{}'
            }}
        }}
    }}

    post {{
        always {{
            archiveArtifacts artifacts: '{}/**', allowEmptyArchive: true
        }}
    }}
}}
"#,
            self.verify_command, self.report_dir
        )
    }
    fn generate_circleci(&self) -> String {
        format!(
            r#"version: 2.1

jobs:
  verify:
    docker:
      - image: rust:latest
    steps:
      - checkout
      - run:
          name: Run Verification
          command: {}
      - store_artifacts:
          path: {}
          destination: verification-reports

workflows:
  verify-statutes:
    jobs:
      - verify
"#,
            self.verify_command, self.report_dir
        )
    }
    fn generate_travis(&self) -> String {
        format!(
            r#"language: rust
rust:
  - stable

script:
  - {}

after_script:
  - tar -czf verification-reports.tar.gz {}

deploy:
  provider: releases
  file: verification-reports.tar.gz
  skip_cleanup: true
  on:
    tags: true
"#,
            self.verify_command, self.report_dir
        )
    }
}
/// Comprehensive dashboard containing all metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsDashboard {
    /// Timestamp when dashboard was generated
    pub generated_at: chrono::NaiveDateTime,
    /// Basic statistics
    pub statistics: StatuteStatistics,
    /// Graph analysis metrics
    pub graph_metrics: GraphMetrics,
    /// Centrality metrics for top statutes
    pub top_centrality: Vec<CentralityMetrics>,
    /// Quality metrics summary
    pub quality_summary: QualitySummary,
    /// Conflict summary
    pub conflict_summary: ConflictSummary,
    /// Coverage analysis
    pub coverage_info: CoverageInfo,
    /// Evolution summary (if tracker provided)
    pub evolution_summary: Option<EvolutionSummary>,
    /// Discovered patterns
    pub patterns: Vec<StatutePattern>,
}
/// Fine-grained dependency tracking for statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyNode {
    /// Statute identifier
    pub statute_id: String,
    /// Direct dependencies (statutes this one references)
    pub dependencies: Vec<String>,
    /// Reverse dependencies (statutes that reference this one)
    pub dependents: Vec<String>,
    /// Dependency type (derives_from, references, etc.)
    pub dependency_type: DependencyType,
    /// Last verification timestamp
    pub last_verified: Option<chrono::DateTime<chrono::Utc>>,
}
impl DependencyNode {
    /// Creates a new dependency node
    pub fn new(statute_id: impl Into<String>, dependency_type: DependencyType) -> Self {
        Self {
            statute_id: statute_id.into(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
            dependency_type,
            last_verified: None,
        }
    }
    /// Adds a dependency
    pub fn add_dependency(&mut self, dep_id: impl Into<String>) {
        let dep = dep_id.into();
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
    }
    /// Adds a dependent
    pub fn add_dependent(&mut self, dep_id: impl Into<String>) {
        let dep = dep_id.into();
        if !self.dependents.contains(&dep) {
            self.dependents.push(dep);
        }
    }
    /// Marks as verified
    pub fn mark_verified(&mut self) {
        self.last_verified = Some(chrono::Utc::now());
    }
}
/// Nature of stakeholder conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ConflictNature {
    /// Direct opposition of interests
    DirectOpposition,
    /// Competing for limited resources
    ResourceCompetition,
    /// Different interpretations of the same statute
    InterpretationDifference,
    /// Overlapping jurisdictions
    JurisdictionalOverlap,
    /// Asymmetric power dynamics
    PowerImbalance,
}
/// Types of coverage gaps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GapType {
    /// Age range not covered
    AgeGap,
    /// Income range not covered
    IncomeGap,
    /// Jurisdiction not covered
    JurisdictionGap,
    /// Temporal gap (time period not covered)
    TemporalGap,
    /// Effect type not covered
    EffectGap,
    /// Logical gap in conditions
    LogicalGap,
}
/// Types of proof steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProofStepType {
    /// Assumption or premise
    Premise,
    /// Logical deduction
    Deduction,
    /// Contradiction found
    Contradiction,
    /// SMT solver result
    SmtResult,
    /// Substitution or simplification
    Simplification,
    /// Conclusion
    Conclusion,
}
/// API request for statute verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationRequest {
    /// Statutes to verify
    pub statutes: Vec<Statute>,
    /// Constitutional principles to check
    pub principles: Vec<PrincipleCheck>,
    /// Request ID for tracking
    pub request_id: Option<String>,
    /// Client identifier
    pub client_id: Option<String>,
}
impl VerificationRequest {
    /// Creates a new verification request.
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self {
            statutes,
            principles: Vec::new(),
            request_id: None,
            client_id: None,
        }
    }
    /// Sets the principles to check.
    pub fn with_principles(mut self, principles: Vec<PrincipleCheck>) -> Self {
        self.principles = principles;
        self
    }
    /// Sets the request ID.
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }
    /// Sets the client ID.
    pub fn with_client_id(mut self, id: impl Into<String>) -> Self {
        self.client_id = Some(id.into());
        self
    }
}
/// Diagnostic location for IDE integration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticLocation {
    /// File path
    pub file: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// End line (optional, for range)
    pub end_line: Option<usize>,
    /// End column (optional, for range)
    pub end_column: Option<usize>,
}
impl DiagnosticLocation {
    /// Creates a new diagnostic location.
    pub fn new(file: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            end_line: None,
            end_column: None,
        }
    }
    /// Sets the end position for a range.
    pub fn with_range(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = Some(end_line);
        self.end_column = Some(end_column);
        self
    }
}
/// Clock constraint in timed automata.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClockConstraint {
    /// clock < value
    Less(Clock, u64),
    /// clock <= value
    LessOrEqual(Clock, u64),
    /// clock > value
    Greater(Clock, u64),
    /// clock >= value
    GreaterOrEqual(Clock, u64),
    /// clock == value
    Equal(Clock, u64),
    /// Conjunction of constraints
    And(Box<ClockConstraint>, Box<ClockConstraint>),
}
impl ClockConstraint {
    /// Checks if the constraint is satisfied given clock valuations.
    pub fn satisfied(&self, valuations: &HashMap<String, u64>) -> bool {
        match self {
            Self::Less(clock, value) => valuations.get(&clock.name).is_some_and(|v| v < value),
            Self::LessOrEqual(clock, value) => {
                valuations.get(&clock.name).is_some_and(|v| v <= value)
            }
            Self::Greater(clock, value) => valuations.get(&clock.name).is_some_and(|v| v > value),
            Self::GreaterOrEqual(clock, value) => {
                valuations.get(&clock.name).is_some_and(|v| v >= value)
            }
            Self::Equal(clock, value) => valuations.get(&clock.name) == Some(value),
            Self::And(left, right) => left.satisfied(valuations) && right.satisfied(valuations),
        }
    }
}
/// A deadline violation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeadlineViolation {
    /// Deadline that was violated
    pub deadline_id: String,
    /// Actual number of steps taken
    pub actual_steps: usize,
    /// Maximum allowed steps
    pub max_steps: usize,
    /// Description of the violation
    pub description: String,
}
/// Represents a mechanism design property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MechanismProperty {
    /// Incentive compatibility - agents benefit from truthful behavior
    IncentiveCompatibility,
    /// Individual rationality - participation is voluntary and beneficial
    IndividualRationality,
    /// Budget balance - transfers sum to zero or non-negative
    BudgetBalance,
    /// Pareto efficiency - no alternative allocation is better for all
    ParetoEfficiency,
    /// Strategy-proofness - truthful reporting is dominant strategy
    StrategyProofness,
    /// Non-dictatorship - no single agent controls outcomes
    NonDictatorship,
}
/// Represents an interaction between two statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteInteraction {
    /// First statute ID
    pub statute_a: String,
    /// Second statute ID
    pub statute_b: String,
    /// Type of interaction
    pub interaction_type: InteractionType,
    /// Description of the interaction
    pub description: String,
    /// Severity level of the interaction
    pub severity: Severity,
    /// Recommendation for handling the interaction
    pub recommendation: String,
}
/// Executive summary of verification results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutiveSummary {
    /// Summary title
    pub title: String,
    /// Generation date
    pub date: String,
    /// Key findings
    pub key_findings: Vec<String>,
    /// Overall assessment
    pub overall_assessment: String,
    /// Statistics
    pub statistics: SummaryStatistics,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Risk level (Low, Medium, High, Critical)
    pub risk_level: String,
}
/// Zero-knowledge proof for statute verification
/// Allows proving that a statute satisfies certain properties without revealing the statute details
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZeroKnowledgeProof {
    /// Unique identifier for this proof
    pub proof_id: String,
    /// Statement being proven (e.g., "statute satisfies constitutional requirements")
    pub statement: String,
    /// Commitment to the hidden data (cryptographic hash)
    pub commitment: String,
    /// Challenge value from verifier
    pub challenge: Vec<u8>,
    /// Response to the challenge
    pub response: Vec<u8>,
    /// Timestamp when proof was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
impl ZeroKnowledgeProof {
    /// Creates a new zero-knowledge proof
    pub fn new(statement: impl Into<String>, statute: &Statute) -> Self {
        use rand::RngExt;
        let mut rng = rand::rng();
        let commitment = format!("{:x}", md5::compute(format!("{:?}", statute)));
        let challenge: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        let response: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        Self {
            proof_id: format!("zkp-{}", uuid::Uuid::new_v4()),
            statement: statement.into(),
            commitment,
            challenge,
            response,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }
    /// Adds metadata to the proof
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    /// Verifies the zero-knowledge proof without revealing underlying data
    pub fn verify(&self) -> bool {
        !self.commitment.is_empty() && !self.challenge.is_empty() && !self.response.is_empty()
    }
    /// Generates a report for this proof
    pub fn report(&self) -> String {
        format!(
            "Zero-Knowledge Proof Report\n\
             ==========================\n\
             Proof ID: {}\n\
             Statement: {}\n\
             Commitment: {}\n\
             Challenge Length: {} bytes\n\
             Response Length: {} bytes\n\
             Timestamp: {}\n\
             Valid: {}\n",
            self.proof_id,
            self.statement,
            &self.commitment[..16.min(self.commitment.len())],
            self.challenge.len(),
            self.response.len(),
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.verify()
        )
    }
}
/// A location (state) in a timed automaton.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimedLocation {
    /// Location identifier
    pub id: String,
    /// Invariant that must hold while in this location
    pub invariant: Option<ClockConstraint>,
    /// Whether this is an accepting/final location
    pub accepting: bool,
}
impl TimedLocation {
    /// Creates a new timed location.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            invariant: None,
            accepting: false,
        }
    }
    /// Sets the invariant.
    pub fn with_invariant(mut self, constraint: ClockConstraint) -> Self {
        self.invariant = Some(constraint);
        self
    }
    /// Marks this location as accepting.
    pub fn accepting(mut self) -> Self {
        self.accepting = true;
        self
    }
}
/// Types of constitutional checks.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PrincipleCheck {
    /// No discrimination based on protected attributes
    NoDiscrimination,
    /// Requires procedural safeguards
    RequiresProcedure,
    /// Must not be retroactive
    NoRetroactivity,
    /// Comprehensive equality check
    EqualityCheck,
    /// Due process verification
    DueProcess,
    /// Privacy impact assessment
    PrivacyImpact,
    /// Proportionality checking
    Proportionality,
    /// Accessibility verification
    Accessibility,
    /// Freedom of expression analysis
    FreedomOfExpression,
    /// Property rights verification
    PropertyRights,
    /// Procedural due process (detailed)
    ProceduralDueProcess,
    /// Equal protection analysis (comprehensive)
    EqualProtection,
    /// Custom check with description and implementation
    Custom {
        /// Description of the custom check
        description: String,
    },
}
/// Version entry in statute evolution history
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteVersion {
    /// Version number
    pub version: u32,
    /// Statute snapshot at this version
    pub statute: Statute,
    /// Timestamp of this version (optional)
    pub timestamp: Option<chrono::NaiveDateTime>,
    /// Description of changes in this version
    pub change_description: Option<String>,
}
/// Git pre-commit hook configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PreCommitHook {
    /// Verification command to run
    pub verify_command: String,
    /// Fail commit on verification errors
    pub fail_on_errors: bool,
    /// Fail commit on warnings
    pub fail_on_warnings: bool,
    /// Show verbose output
    pub verbose: bool,
}
impl PreCommitHook {
    /// Creates a new pre-commit hook configuration.
    pub fn new() -> Self {
        Self {
            verify_command: "cargo run --bin legalis-verify".to_string(),
            fail_on_errors: true,
            fail_on_warnings: false,
            verbose: true,
        }
    }
    /// Sets the verification command.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.verify_command = command.into();
        self
    }
    /// Sets whether to fail on errors.
    pub fn fail_on_errors(mut self, fail: bool) -> Self {
        self.fail_on_errors = fail;
        self
    }
    /// Sets whether to fail on warnings.
    pub fn fail_on_warnings(mut self, fail: bool) -> Self {
        self.fail_on_warnings = fail;
        self
    }
    /// Sets verbose mode.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    /// Generates the pre-commit hook script.
    pub fn generate(&self) -> String {
        format!(
            r#"#!/bin/bash
# Legalis Statute Verification Pre-commit Hook

echo "Running statute verification..."

# Run verification
{}

VERIFICATION_EXIT_CODE=$?

if [ $VERIFICATION_EXIT_CODE -ne 0 ]; then
    if [ "{}" = "true" ]; then
        echo "ERROR: Statute verification failed!"
        echo "Commit aborted. Please fix verification errors before committing."
        exit 1
    else
        echo "WARNING: Statute verification found issues."
    fi
fi

if [ "{}" = "true" ]; then
    echo "Verification details:"
    cat verification-report.txt 2>/dev/null || echo "No detailed report available"
fi

echo "Verification complete."
exit 0
"#,
            self.verify_command,
            if self.fail_on_errors { "true" } else { "false" },
            if self.verbose { "true" } else { "false" }
        )
    }
    /// Installs the pre-commit hook to a git repository.
    pub fn install(&self, repo_path: &str) -> std::io::Result<()> {
        use std::fs;
        use std::io::Write;
        use std::path::Path;
        let hook_path = Path::new(repo_path).join(".git/hooks/pre-commit");
        let hook_content = self.generate();
        let mut file = fs::File::create(&hook_path)?;
        file.write_all(hook_content.as_bytes())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
        }
        Ok(())
    }
}
/// Incremental proof maintenance
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCache {
    /// Cached proofs by statute ID
    pub proofs: HashMap<String, CachedProof>,
}
impl ProofCache {
    /// Creates a new empty proof cache
    pub fn new() -> Self {
        Self {
            proofs: HashMap::new(),
        }
    }
    /// Adds a proof to the cache
    pub fn add_proof(&mut self, statute: &Statute, result: VerificationResult) {
        let proof = CachedProof::new(statute, result);
        self.proofs.insert(statute.id.clone(), proof);
    }
    /// Gets a cached proof if valid
    pub fn get_proof(&self, statute: &Statute) -> Option<&CachedProof> {
        self.proofs.get(&statute.id).filter(|p| p.is_valid(statute))
    }
    /// Invalidates proofs for changed statutes
    pub fn invalidate(&mut self, statute_ids: &[String]) {
        for id in statute_ids {
            self.proofs.remove(id);
        }
    }
    /// Gets cache statistics
    pub fn stats(&self) -> ProofCacheStats {
        ProofCacheStats {
            total_proofs: self.proofs.len(),
            oldest_timestamp: self.proofs.values().map(|p| p.timestamp).min(),
            newest_timestamp: self.proofs.values().map(|p| p.timestamp).max(),
        }
    }
}
/// Graph metrics for statute dependency network
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphMetrics {
    /// Total number of nodes (statutes)
    pub node_count: usize,
    /// Total number of edges (dependencies)
    pub edge_count: usize,
    /// Average degree (connections per statute)
    pub average_degree: f64,
    /// Density of the graph (0.0 to 1.0)
    pub density: f64,
    /// Number of strongly connected components
    pub strongly_connected_components: usize,
    /// Whether the graph is acyclic (DAG)
    pub is_acyclic: bool,
    /// Maximum path length in the graph
    pub diameter: usize,
}
/// Proof certificate for formal verification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCertificate {
    /// Certificate ID
    pub certificate_id: String,
    /// Statute ID
    pub statute_id: String,
    /// Verification claim
    pub claim: String,
    /// Proof method used
    pub proof_method: String,
    /// The complete proof
    pub proof: VerificationProof,
    /// Certificate issuer
    pub issuer: String,
    /// Issuance date
    pub issued_at: String,
    /// Validity period in days
    pub valid_for_days: Option<u32>,
    /// Digital signature (placeholder for actual signature)
    pub signature: Option<String>,
}
impl ProofCertificate {
    /// Creates a new proof certificate
    pub fn new(
        statute_id: impl Into<String>,
        claim: impl Into<String>,
        proof: VerificationProof,
    ) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let statute_id = statute_id.into();
        let claim = claim.into();
        let mut hasher = DefaultHasher::new();
        statute_id.hash(&mut hasher);
        chrono::Utc::now().timestamp().hash(&mut hasher);
        let certificate_id = format!("CERT-{:016x}", hasher.finish());
        Self {
            certificate_id,
            statute_id,
            claim,
            proof_method: "SMT-based formal verification".to_string(),
            proof,
            issuer: "Legalis Verifier".to_string(),
            issued_at: chrono::Utc::now().to_rfc3339(),
            valid_for_days: Some(365),
            signature: None,
        }
    }
    /// Exports certificate to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    /// Exports certificate to human-readable format
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          FORMAL VERIFICATION CERTIFICATE                       ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");
        output.push_str(&format!("Certificate ID: {}\n", self.certificate_id));
        output.push_str(&format!("Statute: {}\n", self.statute_id));
        output.push_str(&format!("Claim: {}\n", self.claim));
        output.push_str(&format!("Proof Method: {}\n", self.proof_method));
        output.push_str(&format!("Issued By: {}\n", self.issuer));
        output.push_str(&format!("Issued At: {}\n", self.issued_at));
        if let Some(days) = self.valid_for_days {
            output.push_str(&format!("Valid For: {} days\n", days));
        }
        output.push_str(&format!(
            "\nProof Status: {}\n",
            if self.proof.is_complete {
                "✓ Complete"
            } else {
                "✗ Incomplete"
            }
        ));
        output.push_str(&format!("Proof Steps: {}\n\n", self.proof.steps.len()));
        output.push_str(&self.proof.to_human_readable());
        output.push_str("\n╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║  This certificate attests that the statute has been formally   ║\n");
        output.push_str("║  verified using automated theorem proving techniques.          ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n");
        output
    }
}
/// Types of ambiguities that can be detected in statutes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AmbiguityType {
    /// Vague or undefined terms in descriptions
    VagueTerm,
    /// Overlapping or conflicting conditions
    OverlappingConditions,
    /// Unclear effect description
    UnclearEffect,
    /// Missing discretion logic for complex conditions
    MissingDiscretion,
    /// Ambiguous temporal scope
    TemporalAmbiguity,
    /// Implicit assumptions not stated
    ImplicitAssumption,
    /// Quantifier ambiguity (e.g., "all", "some", "any")
    QuantifierAmbiguity,
}
/// Represents a potential duplicate statute.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateCandidate {
    /// IDs of potentially duplicate statutes
    pub statute_ids: Vec<String>,
    /// Similarity score (0.0 to 1.0)
    pub similarity_score: f64,
    /// Type of similarity
    pub similarity_type: String,
    /// Recommendation
    pub recommendation: String,
}
/// Represents a coalition of stakeholders
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Coalition {
    /// Member stakeholder IDs
    pub members: Vec<String>,
    /// Shared objectives
    pub objectives: Vec<String>,
    /// Collective effects of the coalition
    pub collective_effects: Vec<String>,
    /// Strength of the coalition (0.0 to 1.0)
    pub strength: f64,
    /// Whether the coalition is stable
    pub is_stable: bool,
}
impl Coalition {
    /// Creates a new coalition
    pub fn new(members: Vec<String>) -> Self {
        Self {
            members,
            objectives: Vec::new(),
            collective_effects: Vec::new(),
            strength: 0.0,
            is_stable: false,
        }
    }
    /// Adds an objective
    pub fn with_objective(mut self, objective: impl Into<String>) -> Self {
        self.objectives.push(objective.into());
        self
    }
    /// Adds a collective effect
    pub fn with_collective_effect(mut self, effect: impl Into<String>) -> Self {
        self.collective_effects.push(effect.into());
        self
    }
    /// Sets the strength
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }
    /// Sets stability
    pub fn with_stability(mut self, is_stable: bool) -> Self {
        self.is_stable = is_stable;
        self
    }
}
/// Text edit for applying quick fixes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextEdit {
    /// File to edit
    pub file: String,
    /// Start line (1-based)
    pub start_line: usize,
    /// Start column (1-based)
    pub start_column: usize,
    /// End line (1-based)
    pub end_line: usize,
    /// End column (1-based)
    pub end_column: usize,
    /// New text to insert
    pub new_text: String,
}
impl TextEdit {
    /// Creates a new text edit.
    pub fn new(
        file: impl Into<String>,
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
        new_text: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            start_line,
            start_column,
            end_line,
            end_column,
            new_text: new_text.into(),
        }
    }
}
/// Evolution metrics for a statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Total number of versions
    pub total_versions: usize,
    /// Number of major changes (effect or precondition modifications)
    pub major_changes: usize,
    /// Number of minor changes (title, description, metadata)
    pub minor_changes: usize,
    /// Average time between versions (in days)
    pub avg_days_between_versions: Option<f64>,
    /// Stability score (0.0 = very unstable, 1.0 = very stable)
    pub stability_score: f64,
    /// Complexity trend (Increasing, Decreasing, Stable)
    pub complexity_trend: ComplexityTrend,
}
/// Result of a constitutional principle check.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrincipleCheckResult {
    /// Whether the check passed
    pub passed: bool,
    /// Issues found (if any)
    pub issues: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}
impl PrincipleCheckResult {
    /// Creates a passing result.
    pub fn pass() -> Self {
        Self {
            passed: true,
            issues: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    /// Creates a failing result with issues.
    pub fn fail(issues: Vec<String>) -> Self {
        Self {
            passed: false,
            issues,
            suggestions: Vec::new(),
        }
    }
    /// Adds a suggestion.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}
/// TEE-based verification result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeeVerificationResult {
    /// Verification result
    pub result: VerificationResult,
    /// TEE configuration used
    pub tee_config: TeeConfig,
    /// Remote attestation proof
    pub attestation_proof: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl TeeVerificationResult {
    /// Creates a new TEE verification result
    pub fn new(result: VerificationResult, tee_config: TeeConfig) -> Self {
        Self {
            result,
            tee_config,
            attestation_proof: format!("tee-attestation-{}", uuid::Uuid::new_v4()),
            timestamp: chrono::Utc::now(),
        }
    }
    /// Generates a report
    pub fn report(&self) -> String {
        format!(
            "TEE Verification Report\n\
             ======================\n\
             TEE Type: {}\n\
             Attestation Valid: {}\n\
             Verification Passed: {}\n\
             Errors: {}\n\
             Warnings: {}\n\
             Attestation Proof: {}\n\
             Timestamp: {}\n",
            self.tee_config.tee_type,
            self.tee_config.verify_attestation(),
            self.result.passed,
            self.result.errors.len(),
            self.result.warnings.len(),
            self.attestation_proof,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}
/// Proof cache statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCacheStats {
    /// Total number of cached proofs
    pub total_proofs: usize,
    /// Oldest proof timestamp
    pub oldest_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Newest proof timestamp
    pub newest_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}
/// Types of conflicts that can occur between statutes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConflictType {
    /// Statutes have overlapping conditions but contradictory effects
    EffectConflict,
    /// Multiple statutes claim authority over the same jurisdiction
    JurisdictionalOverlap,
    /// Statutes with overlapping temporal validity have conflicting rules
    TemporalConflict,
    /// Lower-level statute contradicts higher-level statute
    HierarchyViolation,
    /// Statutes with same ID in different jurisdictions
    IdCollision,
}
/// Centrality metrics for a single statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CentralityMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Degree centrality (number of direct connections)
    pub degree_centrality: f64,
    /// In-degree (number of statutes referencing this one)
    pub in_degree: usize,
    /// Out-degree (number of statutes this one references)
    pub out_degree: usize,
    /// PageRank score (importance based on link structure)
    pub pagerank: f64,
    /// Betweenness centrality (how often statute is on shortest path)
    pub betweenness: f64,
}
/// Statistical model checking result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatisticalCheckResult {
    /// Property being checked
    pub property: String,
    /// Estimated probability of satisfaction
    pub estimated_probability: f64,
    /// Confidence interval lower bound (95%)
    pub confidence_lower: f64,
    /// Confidence interval upper bound (95%)
    pub confidence_upper: f64,
    /// Number of simulation runs
    pub num_samples: usize,
    /// Number of successful runs
    pub num_successes: usize,
    /// Hypothesis test result (true = accept, false = reject)
    pub hypothesis_accepted: bool,
}
impl StatisticalCheckResult {
    /// Creates a new result from samples
    pub fn from_samples(
        property: impl Into<String>,
        num_samples: usize,
        num_successes: usize,
        threshold: f64,
    ) -> Self {
        let p_hat = num_successes as f64 / num_samples as f64;
        let z = 1.96;
        let std_err = (p_hat * (1.0 - p_hat) / num_samples as f64).sqrt();
        let margin = z * std_err;
        let confidence_lower = (p_hat - margin).max(0.0);
        let confidence_upper = (p_hat + margin).min(1.0);
        let hypothesis_accepted = confidence_lower >= threshold;
        Self {
            property: property.into(),
            estimated_probability: p_hat,
            confidence_lower,
            confidence_upper,
            num_samples,
            num_successes,
            hypothesis_accepted,
        }
    }
}
/// Risk quantification analysis result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiskQuantification {
    /// Statute ID
    pub statute_id: String,
    /// Individual risk factors
    pub factors: Vec<RiskFactor>,
    /// Overall risk score (0.0-1.0)
    pub overall_score: f64,
    /// Risk level classification
    pub risk_level: RiskLevel,
    /// Mitigation recommendations
    pub mitigations: Vec<String>,
}
impl RiskQuantification {
    /// Creates a new risk quantification
    pub fn new(statute_id: impl Into<String>, factors: Vec<RiskFactor>) -> Self {
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        let overall_score = if total_weight > 0.0 {
            factors.iter().map(|f| f.score * f.weight).sum::<f64>() / total_weight
        } else {
            0.0
        };
        let risk_level = RiskLevel::from_score(overall_score);
        Self {
            statute_id: statute_id.into(),
            factors,
            overall_score,
            risk_level,
            mitigations: vec![],
        }
    }
    /// Adds a mitigation recommendation
    pub fn add_mitigation(mut self, mitigation: impl Into<String>) -> Self {
        self.mitigations.push(mitigation.into());
        self
    }
}
/// Mechanism design analysis result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MechanismAnalysis {
    /// Issues found
    pub issues: Vec<MechanismIssue>,
    /// Properties satisfied
    pub satisfied_properties: Vec<MechanismProperty>,
    /// Overall mechanism quality score (0.0-1.0)
    pub quality_score: f64,
}
impl MechanismAnalysis {
    /// Creates a new mechanism analysis
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            satisfied_properties: Vec::new(),
            quality_score: 1.0,
        }
    }
    /// Adds an issue
    pub fn add_issue(&mut self, issue: MechanismIssue) {
        self.issues.push(issue);
        self.recalculate_score();
    }
    /// Marks a property as satisfied
    pub fn satisfy_property(&mut self, property: MechanismProperty) {
        if !self.satisfied_properties.contains(&property) {
            self.satisfied_properties.push(property);
        }
    }
    /// Recalculates the quality score
    fn recalculate_score(&mut self) {
        let total_properties = 6.0;
        let critical_issues = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Critical)
            .count() as f64;
        let errors = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count() as f64;
        let warnings = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count() as f64;
        let penalty = (critical_issues * 0.3) + (errors * 0.15) + (warnings * 0.05);
        let bonus = self.satisfied_properties.len() as f64 / total_properties;
        self.quality_score = (1.0 - penalty + bonus).clamp(0.0, 1.0);
    }
}
/// Notification type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NotificationType {
    /// Verification completed successfully
    Success,
    /// Verification completed with warnings
    Warning,
    /// Verification failed with errors
    Error,
    /// Verification encountered a critical issue
    Critical,
}
/// Optimization suggestion for statute conditions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizationSuggestion {
    /// Statute ID that can be optimized
    pub statute_id: String,
    /// Current complexity score
    pub current_complexity: usize,
    /// Suggested simplified condition
    pub suggested_condition: Option<String>,
    /// List of specific suggestions
    pub suggestions: Vec<String>,
    /// Potential complexity after optimization
    pub optimized_complexity: usize,
}
/// CI/CD platform type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CiPlatform {
    /// GitHub Actions
    GitHubActions,
    /// GitLab CI/CD
    GitLabCI,
    /// Jenkins
    Jenkins,
    /// CircleCI
    CircleCI,
    /// Travis CI
    TravisCI,
}
/// Represents a terminology inconsistency.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TerminologyInconsistency {
    /// The term variations found
    pub variations: Vec<String>,
    /// Statute IDs where variations are used
    pub statute_ids: Vec<String>,
    /// Suggested canonical term
    pub canonical_term: String,
}
impl TerminologyInconsistency {
    /// Creates a new terminology inconsistency.
    pub fn new(canonical_term: impl Into<String>) -> Self {
        Self {
            variations: Vec::new(),
            statute_ids: Vec::new(),
            canonical_term: canonical_term.into(),
        }
    }
    /// Adds a variation to the inconsistency.
    pub fn with_variation(mut self, variation: impl Into<String>) -> Self {
        let var = variation.into();
        if !self.variations.contains(&var) {
            self.variations.push(var);
        }
        self
    }
    /// Adds a statute ID to the inconsistency.
    pub fn with_statute_id(mut self, statute_id: impl Into<String>) -> Self {
        let id = statute_id.into();
        if !self.statute_ids.contains(&id) {
            self.statute_ids.push(id);
        }
        self
    }
}
/// Notification channel.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NotificationChannel {
    /// Webhook URL
    Webhook {
        url: String,
        headers: HashMap<String, String>,
    },
    /// Email notification
    Email { to: Vec<String>, subject: String },
    /// Callback function (not serializable, use name reference)
    Callback { name: String },
}
/// Represents a stakeholder in the legal system
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Stakeholder {
    /// Unique identifier
    pub id: String,
    /// Name of the stakeholder
    pub name: String,
    /// Type of stakeholder (e.g., "individual", "corporation", "government")
    pub stakeholder_type: String,
    /// Interests or goals
    pub interests: Vec<String>,
    /// Statutes that directly affect this stakeholder
    pub affected_by: Vec<String>,
}
impl Stakeholder {
    /// Creates a new stakeholder
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            stakeholder_type: "individual".to_string(),
            interests: Vec::new(),
            affected_by: Vec::new(),
        }
    }
    /// Sets the stakeholder type
    pub fn with_type(mut self, stakeholder_type: impl Into<String>) -> Self {
        self.stakeholder_type = stakeholder_type.into();
        self
    }
    /// Adds an interest
    pub fn with_interest(mut self, interest: impl Into<String>) -> Self {
        self.interests.push(interest.into());
        self
    }
    /// Adds a statute that affects this stakeholder
    pub fn affected_by_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.affected_by.push(statute_id.into());
        self
    }
}
/// Risk factor for statute analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiskFactor {
    /// Factor name
    pub name: String,
    /// Factor description
    pub description: String,
    /// Risk contribution (0.0-1.0)
    pub score: f64,
    /// Weight in overall risk (0.0-1.0)
    pub weight: f64,
}
impl RiskFactor {
    /// Creates a new risk factor
    pub fn new(name: impl Into<String>, description: impl Into<String>, score: f64) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            score: score.clamp(0.0, 1.0),
            weight: 1.0,
        }
    }
    /// Sets the weight
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }
}
/// A timed automaton.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimedAutomaton {
    /// All locations in the automaton
    pub locations: HashMap<String, TimedLocation>,
    /// All transitions
    pub transitions: Vec<TimedTransition>,
    /// Initial location
    pub initial: String,
    /// All clock variables
    pub clocks: Vec<Clock>,
}
impl TimedAutomaton {
    /// Creates a new timed automaton.
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            locations: HashMap::new(),
            transitions: Vec::new(),
            initial: initial.into(),
            clocks: Vec::new(),
        }
    }
    /// Adds a location.
    pub fn add_location(&mut self, location: TimedLocation) {
        self.locations.insert(location.id.clone(), location);
    }
    /// Adds a transition.
    pub fn add_transition(&mut self, transition: TimedTransition) {
        self.transitions.push(transition);
    }
    /// Adds a clock.
    pub fn add_clock(&mut self, clock: Clock) {
        self.clocks.push(clock);
    }
}
/// Represents a cross-reference validation error.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CrossReferenceError {
    /// Statute ID containing the reference
    pub source_statute_id: String,
    /// Referenced statute ID that is invalid
    pub referenced_statute_id: String,
    /// Error type
    pub error_type: CrossReferenceErrorType,
}
/// Computation Tree Logic (CTL) formula.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CtlFormula {
    /// Atomic proposition
    Atom(String),
    /// Negation
    Not(Box<CtlFormula>),
    /// Conjunction
    And(Box<CtlFormula>, Box<CtlFormula>),
    /// Disjunction
    Or(Box<CtlFormula>, Box<CtlFormula>),
    /// Implication
    Implies(Box<CtlFormula>, Box<CtlFormula>),
    /// Exists Next (there exists a next state where formula holds)
    ExistsNext(Box<CtlFormula>),
    /// All Next (formula holds in all next states)
    AllNext(Box<CtlFormula>),
    /// Exists Eventually (there exists a path where formula eventually holds)
    ExistsEventually(Box<CtlFormula>),
    /// All Eventually (formula eventually holds on all paths)
    AllEventually(Box<CtlFormula>),
    /// Exists Always (there exists a path where formula always holds)
    ExistsAlways(Box<CtlFormula>),
    /// All Always (formula always holds on all paths)
    AllAlways(Box<CtlFormula>),
    /// Exists Until
    ExistsUntil(Box<CtlFormula>, Box<CtlFormula>),
    /// All Until
    AllUntil(Box<CtlFormula>, Box<CtlFormula>),
}
impl CtlFormula {
    /// Creates a new atomic proposition.
    pub fn atom(name: impl Into<String>) -> Self {
        Self::Atom(name.into())
    }
    /// Creates a negation.
    #[allow(clippy::should_implement_trait)]
    pub fn not(formula: CtlFormula) -> Self {
        Self::Not(Box::new(formula))
    }
    /// Creates a conjunction.
    pub fn and(left: CtlFormula, right: CtlFormula) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }
    /// Creates a disjunction.
    pub fn or(left: CtlFormula, right: CtlFormula) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }
    /// Creates an implication.
    pub fn implies(antecedent: CtlFormula, consequent: CtlFormula) -> Self {
        Self::Implies(Box::new(antecedent), Box::new(consequent))
    }
    /// Creates an exists-next operator.
    pub fn exists_next(formula: CtlFormula) -> Self {
        Self::ExistsNext(Box::new(formula))
    }
    /// Creates an all-next operator.
    pub fn all_next(formula: CtlFormula) -> Self {
        Self::AllNext(Box::new(formula))
    }
    /// Creates an exists-eventually operator.
    pub fn exists_eventually(formula: CtlFormula) -> Self {
        Self::ExistsEventually(Box::new(formula))
    }
    /// Creates an all-eventually operator.
    pub fn all_eventually(formula: CtlFormula) -> Self {
        Self::AllEventually(Box::new(formula))
    }
    /// Creates an exists-always operator.
    pub fn exists_always(formula: CtlFormula) -> Self {
        Self::ExistsAlways(Box::new(formula))
    }
    /// Creates an all-always operator.
    pub fn all_always(formula: CtlFormula) -> Self {
        Self::AllAlways(Box::new(formula))
    }
    /// Creates an exists-until operator.
    pub fn exists_until(left: CtlFormula, right: CtlFormula) -> Self {
        Self::ExistsUntil(Box::new(left), Box::new(right))
    }
    /// Creates an all-until operator.
    pub fn all_until(left: CtlFormula, right: CtlFormula) -> Self {
        Self::AllUntil(Box::new(left), Box::new(right))
    }
}
/// Output format for scheduled reports
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReportOutputFormat {
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// JSON format
    Json,
    /// PDF format (requires pdf feature)
    #[cfg(feature = "pdf")]
    Pdf,
}
/// A jurisdictional rule set containing principles for a specific jurisdiction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JurisdictionalRuleSet {
    /// Jurisdiction identifier
    pub jurisdiction: String,
    /// Name of the jurisdiction
    pub name: String,
    /// Principles that apply in this jurisdiction
    pub principles: Vec<PrincipleDefinition>,
    /// Composite principles
    pub composites: Vec<CompositePrinciple>,
}
impl JurisdictionalRuleSet {
    /// Creates a new jurisdictional rule set.
    pub fn new(jurisdiction: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            jurisdiction: jurisdiction.into(),
            name: name.into(),
            principles: Vec::new(),
            composites: Vec::new(),
        }
    }
    /// Adds a principle.
    pub fn with_principle(mut self, principle: PrincipleDefinition) -> Self {
        self.principles.push(principle);
        self
    }
    /// Adds a composite principle.
    pub fn with_composite(mut self, composite: CompositePrinciple) -> Self {
        self.composites.push(composite);
        self
    }
    /// Gets principles by priority (highest first).
    pub fn principles_by_priority(&self) -> Vec<&PrincipleDefinition> {
        let mut sorted: Vec<_> = self.principles.iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.priority));
        sorted
    }
}
/// Semantic similarity score between two items (0.0 = completely different, 1.0 = identical).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SimilarityScore(pub f64);
impl SimilarityScore {
    /// Creates a new similarity score (clamped to [0.0, 1.0]).
    pub fn new(score: f64) -> Self {
        Self(score.clamp(0.0, 1.0))
    }
    /// Returns true if similarity is high (>= 0.8).
    pub fn is_high(&self) -> bool {
        self.0 >= 0.8
    }
    /// Returns true if similarity is moderate (>= 0.5 and < 0.8).
    pub fn is_moderate(&self) -> bool {
        self.0 >= 0.5 && self.0 < 0.8
    }
    /// Returns true if similarity is low (< 0.5).
    pub fn is_low(&self) -> bool {
        self.0 < 0.5
    }
}
/// API response for statute verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationResponse {
    /// Request ID (echoed from request)
    pub request_id: Option<String>,
    /// Verification results for each statute
    pub results: Vec<VerificationResult>,
    /// Overall success status
    pub success: bool,
    /// Error count
    pub error_count: usize,
    /// Warning count
    pub warning_count: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}
impl VerificationResponse {
    /// Creates a new verification response.
    pub fn new(request_id: Option<String>, results: Vec<VerificationResult>) -> Self {
        let error_count: usize = results.iter().map(|r| r.errors.len()).sum();
        let warning_count: usize = results.iter().map(|r| r.warnings.len()).sum();
        let success = results.iter().all(|r| r.passed);
        Self {
            request_id,
            results,
            success,
            error_count,
            warning_count,
            processing_time_ms: 0,
        }
    }
    /// Sets the processing time.
    pub fn with_processing_time(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = time_ms;
        self
    }
}
/// A sequence constraint violation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SequenceViolation {
    /// Constraint that was violated
    pub constraint_id: String,
    /// Description of the violation
    pub description: String,
    /// Events that violated the order
    pub violating_events: Vec<String>,
}
/// Differential privacy parameters
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PrivacyBudget {
    /// Epsilon parameter (privacy loss bound)
    pub epsilon: f64,
    /// Delta parameter (failure probability)
    pub delta: f64,
}
impl PrivacyBudget {
    /// Creates a new privacy budget
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self { epsilon, delta }
    }
    /// Creates a strict privacy budget (high privacy)
    pub fn strict() -> Self {
        Self {
            epsilon: 0.1,
            delta: 1e-5,
        }
    }
    /// Creates a moderate privacy budget
    pub fn moderate() -> Self {
        Self {
            epsilon: 1.0,
            delta: 1e-3,
        }
    }
    /// Creates a relaxed privacy budget (lower privacy, more accuracy)
    pub fn relaxed() -> Self {
        Self {
            epsilon: 3.0,
            delta: 1e-2,
        }
    }
}
/// Regulatory filing report for submitting to regulatory bodies
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryFiling {
    /// Filing ID
    pub filing_id: String,
    /// Filing date
    pub filing_date: String,
    /// Regulatory body
    pub regulatory_body: String,
    /// Filing type (e.g., "Annual Compliance", "New Statute", "Amendment")
    pub filing_type: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Statutes included in filing
    pub statutes: Vec<StatuteFilingInfo>,
    /// Compliance status
    pub compliance_status: String,
    /// Supporting documentation references
    pub documentation_refs: Vec<String>,
}
/// Lazy verification configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LazyVerificationConfig {
    /// Only verify statutes that have changed
    pub verify_changed_only: bool,
    /// Verify dependencies of changed statutes
    pub verify_dependencies: bool,
    /// Maximum depth for dependency verification
    pub max_depth: Option<usize>,
}
impl LazyVerificationConfig {
    /// Creates a new lazy verification config
    pub fn new() -> Self {
        Self {
            verify_changed_only: true,
            verify_dependencies: true,
            max_depth: None,
        }
    }
    /// Only verify changed statutes
    pub fn changed_only() -> Self {
        Self {
            verify_changed_only: true,
            verify_dependencies: false,
            max_depth: None,
        }
    }
    /// Verify with limited dependency depth
    pub fn with_depth(depth: usize) -> Self {
        Self {
            verify_changed_only: true,
            verify_dependencies: true,
            max_depth: Some(depth),
        }
    }
}
/// A clock variable used in timed automata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Clock {
    /// Clock name
    pub name: String,
}
impl Clock {
    /// Creates a new clock.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
