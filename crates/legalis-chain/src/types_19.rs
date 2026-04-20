//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use thiserror::Error;

use super::types::{
    AiModelType, AllocationConstraint, CelestialBody, ComplianceMode, EnvironmentalMetric,
    FailureHandling, InferenceMode, IntentConditionType, IntentSpecification, PerformanceTarget,
    RegulatoryRule, TreasuryStrategy,
};

/// Intent constraint types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntentConstraintType {
    /// Maximum gas cost
    MaxGasCost,
    /// Minimum output amount
    MinOutput,
    /// Maximum slippage
    MaxSlippage,
    /// MEV protection level
    MevProtection,
    /// Privacy requirement
    Privacy,
    /// Custom constraint
    Custom,
}
/// Reputation calculation methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReputationCalculation {
    /// Transaction volume-based
    VolumeBased,
    /// Time-based (tenure)
    TimeBased,
    /// Activity-based
    ActivityBased,
    /// Staking-based
    StakingBased,
    /// Peer-endorsed
    PeerEndorsed,
    /// AI-computed
    AiComputed,
}
/// Delay-tolerant verification methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelayTolerantMethod {
    /// Asynchronous verification with eventual consistency
    AsyncEventual,
    /// Checkpoint-based verification
    Checkpoint,
    /// Merkle proof aggregation
    MerkleAggregation,
    /// Bundle and verify
    BundleAndVerify,
    /// Optimistic with fraud proofs
    OptimisticFraud,
}
/// Carbon credit types for tokenization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarbonCreditType {
    /// Verified carbon reduction
    Reduction,
    /// Carbon removal
    Removal,
    /// Renewable energy
    RenewableEnergy,
    /// Nature-based solutions
    NatureBased,
}
/// Quantum key distribution protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QkdProtocol {
    /// BB84 protocol
    Bb84,
    /// E91 protocol (entanglement-based)
    E91,
    /// B92 protocol
    B92,
    /// SARG04 protocol
    Sarg04,
}
/// Adaptation strategy for contract parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaptationStrategy {
    /// Market-based adaptation
    MarketBased,
    /// Usage-based adaptation
    UsageBased,
    /// AI-driven adaptation
    AiDriven,
    /// Governance-based adaptation
    GovernanceBased,
    /// Hybrid adaptation
    Hybrid,
}
/// Planetary jurisdiction specification.
#[derive(Debug, Clone)]
pub struct PlanetaryJurisdiction {
    /// Jurisdiction name (e.g., "Earth-EU", "Mars-Colony-Alpha")
    pub name: String,
    /// Celestial body
    pub celestial_body: CelestialBody,
    /// Legal framework
    pub legal_framework: String,
    /// Applicable laws
    pub applicable_laws: Vec<String>,
}
/// GDPR compliance configuration.
#[derive(Debug, Clone)]
pub struct GdprComplianceConfig {
    /// Right to erasure (right to be forgotten)
    pub right_to_erasure: bool,
    /// Right to data portability
    pub right_to_portability: bool,
    /// Right to rectification
    pub right_to_rectification: bool,
    /// Purpose limitation
    pub purpose_limitation: bool,
    /// Data minimization
    pub data_minimization: bool,
    /// Consent management
    pub consent_management: bool,
}
/// Execution order for composed intents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionOrder {
    /// Sequential execution
    Sequential,
    /// Parallel execution
    Parallel,
    /// Dependency-based execution
    DependencyBased,
}
/// Errors during contract generation.
#[derive(Debug, Error)]
pub enum ChainError {
    #[error("Statute '{0}' contains discretionary elements and cannot be exported")]
    DiscretionaryStatute(String),
    #[error("Unsupported condition type: {0}")]
    UnsupportedCondition(String),
    #[error("Unsupported effect type: {0}")]
    UnsupportedEffect(String),
    #[error("Generation error: {0}")]
    GenerationError(String),
}
/// Legal clause types for optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClauseType {
    /// Liability clauses
    Liability,
    /// Payment terms
    Payment,
    /// Termination conditions
    Termination,
    /// Dispute resolution
    DisputeResolution,
    /// Force majeure
    ForceMajeure,
}
/// Quantum key distribution configuration.
#[derive(Debug, Clone)]
pub struct QkdConfig {
    /// Protocol to use
    pub protocol: QkdProtocol,
    /// Key refresh interval (in blocks)
    pub refresh_interval: u64,
    /// Enable quantum random number generation
    pub qrng_enabled: bool,
    /// Oracle address for quantum entropy
    pub oracle_address: Option<String>,
}
/// MiCA compliance configuration.
#[derive(Debug, Clone)]
pub struct MicaComplianceConfig {
    /// Asset reference token (ART) compliance
    pub art_compliance: bool,
    /// E-money token (EMT) compliance
    pub emt_compliance: bool,
    /// White paper requirements
    pub whitepaper_required: bool,
    /// Reserve requirements
    pub reserve_requirements: bool,
    /// Redemption rights
    pub redemption_rights: bool,
}
/// DNA-based identity configuration.
#[derive(Debug, Clone)]
pub struct DnaIdentityConfig {
    /// Enable privacy-preserving DNA matching
    pub privacy_preserving: bool,
    /// Number of genetic markers to use
    pub marker_count: u32,
    /// Enable ancestry verification
    pub ancestry_verification: bool,
    /// Oracle address for DNA verification
    pub oracle_address: Option<String>,
}
/// Life event types for trigger contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifeEventType {
    /// Birth
    Birth,
    /// Marriage
    Marriage,
    /// Divorce
    Divorce,
    /// Death
    Death,
    /// Medical diagnosis
    MedicalDiagnosis,
    /// Recovery from condition
    Recovery,
}
/// Intelligent contract auditing configuration.
#[derive(Debug, Clone)]
pub struct IntelligentAuditConfig {
    /// Enable AI-powered analysis
    pub ai_powered: bool,
    /// Minimum severity to report
    pub min_severity: AuditSeverity,
    /// Enable automated fixes
    pub auto_fix: bool,
    /// Enable comparative analysis
    pub comparative_analysis: bool,
    /// Enable best practices checking
    pub best_practices: bool,
}
/// MEV protection strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MevProtectionStrategy {
    /// Commit-reveal scheme
    CommitReveal,
    /// Private mempool
    PrivateMempool,
    /// Threshold encryption
    ThresholdEncryption,
    /// Batch auction
    BatchAuction,
    /// Time-weighted average price
    Twap,
}
/// Verification priority levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationPriority {
    /// Critical - highest priority
    Critical,
    /// High priority
    High,
    /// Normal priority
    Normal,
    /// Low priority
    Low,
    /// Background - lowest priority
    Background,
}
/// Liquidation cascade prevention configuration.
#[derive(Debug, Clone)]
pub struct LiquidationCascadeConfig {
    /// Enable circuit breaker
    pub circuit_breaker: bool,
    /// Maximum liquidation per block (percentage)
    pub max_liquidation_per_block: u8,
    /// Price impact threshold (percentage)
    pub price_impact_threshold: u8,
    /// Emergency pause trigger
    pub emergency_pause: bool,
    /// Gradual liquidation
    pub gradual_liquidation: bool,
}
/// Multi-network deployment configuration.
#[derive(Debug, Clone)]
pub struct MultiNetworkConfig {
    /// Network configurations
    pub networks: Vec<NetworkConfig>,
    /// Default network name
    pub default_network: String,
}
/// Circular economy tracking configuration.
#[derive(Debug, Clone)]
pub struct CircularEconomyConfig {
    /// Enable material tracking
    pub material_tracking: bool,
    /// Enable recycling verification
    pub recycling_verification: bool,
    /// Enable product lifecycle tracking
    pub lifecycle_tracking: bool,
    /// Enable supply chain transparency
    pub supply_chain_transparency: bool,
}
/// Lazy evaluation configuration.
#[derive(Debug, Clone)]
pub struct LazyEvaluationConfig {
    /// Enable lazy evaluation
    pub enabled: bool,
    /// Contract size threshold (bytes)
    pub size_threshold: usize,
    /// Enable on-demand generation
    pub on_demand: bool,
}
/// Intent-based architecture configuration.
#[derive(Debug, Clone)]
pub struct IntentConfig {
    /// Contract name
    pub name: String,
    /// Enable intent verification
    pub verify_intents: bool,
    /// Enable solver integration
    pub solver_integration: bool,
    /// Maximum intent validity period (in seconds)
    pub max_validity: u64,
    /// Enable partial fills
    pub partial_fills: bool,
}
/// Generated smart contract.
#[derive(Debug, Clone)]
pub struct GeneratedContract {
    /// Name of the contract
    pub name: String,
    /// Source code
    pub source: String,
    /// Target platform
    pub platform: TargetPlatform,
    /// ABI (for Solidity)
    pub abi: Option<String>,
    /// Deployment script
    pub deployment_script: Option<String>,
}
/// Intent composition for complex transactions.
#[derive(Debug, Clone)]
pub struct IntentComposition {
    /// Composition ID
    pub id: String,
    /// Child intents
    pub intents: Vec<IntentSpecification>,
    /// Execution order
    pub execution_order: ExecutionOrder,
    /// Atomic execution requirement
    pub atomic: bool,
    /// Failure handling
    pub failure_handling: FailureHandling,
}
/// Layer 2 platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer2Platform {
    /// Optimism
    Optimism,
    /// Arbitrum
    Arbitrum,
    /// zkSync Era
    ZkSyncEra,
    /// Polygon zkEVM
    PolygonZkEvm,
    /// Base
    Base,
}
/// AI model configuration for on-chain integration.
#[derive(Debug, Clone)]
pub struct AiModelConfig {
    /// Model identifier
    pub model_id: String,
    /// Model type
    pub model_type: AiModelType,
    /// Input parameters
    pub input_params: Vec<String>,
    /// Output type
    pub output_type: String,
    /// Inference mode
    pub inference_mode: InferenceMode,
    /// Oracle address (for oracle-based inference)
    pub oracle_address: Option<String>,
}
/// SLA (Service Level Agreement) enforcement configuration.
#[derive(Debug, Clone)]
pub struct SlaEnforcementConfig {
    /// Uptime requirement (percentage)
    pub uptime_percentage: u8,
    /// Response time SLA in milliseconds
    pub response_time_ms: u32,
    /// Penalty for violation
    pub violation_penalty: bool,
    /// Auto-compensation
    pub auto_compensation: bool,
    /// Performance monitoring
    pub performance_monitoring: bool,
    /// Escalation mechanism
    pub escalation: bool,
}
/// Predictive compliance monitoring configuration.
#[derive(Debug, Clone)]
pub struct PredictiveComplianceConfig {
    /// Monitoring mode
    pub mode: ComplianceMode,
    /// Enable ML predictions
    pub ml_predictions: bool,
    /// Prediction horizon (in days)
    pub prediction_horizon: u32,
    /// Alert threshold
    pub alert_threshold: f64,
    /// Enable automated remediation
    pub auto_remediation: bool,
}
/// Self-executing regulatory contract configuration.
#[derive(Debug, Clone)]
pub struct SelfExecutingRegulatoryConfig {
    /// Regulatory framework name
    pub framework_name: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Regulatory rules
    pub rules: Vec<RegulatoryRule>,
    /// Compliance interval (in blocks)
    pub compliance_interval: u64,
    /// Auto-remediation enabled
    pub auto_remediation: bool,
    /// Audit trail required
    pub audit_trail: bool,
    /// Reporting frequency (in blocks)
    pub reporting_frequency: u64,
}
/// Single network configuration.
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Network name (e.g., "mainnet", "goerli", "polygon")
    pub name: String,
    /// RPC URL
    pub rpc_url: String,
    /// Chain ID
    pub chain_id: u64,
    /// Gas limit override
    pub gas_limit: Option<u64>,
    /// Gas price in gwei
    pub gas_price: Option<u64>,
    /// Etherscan API key for verification
    pub etherscan_api_key: Option<String>,
}
/// Carbon credit tokenization configuration.
#[derive(Debug, Clone)]
pub struct CarbonCreditConfig {
    /// Type of carbon credit
    pub credit_type: CarbonCreditType,
    /// Enable verification oracle
    pub verification_oracle: bool,
    /// Enable retirement tracking
    pub retirement_tracking: bool,
    /// CO2 equivalent per token (in kg)
    pub co2_per_token: u64,
    /// Oracle address
    pub oracle_address: Option<String>,
}
/// Modern testing tools configuration.
#[derive(Debug, Clone)]
pub struct ModernTestingConfig {
    /// Enable Echidna fuzzing
    pub echidna: bool,
    /// Enable Medusa fuzzing
    pub medusa: bool,
    /// Enable Foundry invariant tests
    pub foundry_invariants: bool,
    /// Enable mutation testing
    pub mutation_testing: bool,
    /// Enable differential testing
    pub differential_testing: bool,
}
/// Risk assessment types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskType {
    /// Security vulnerabilities
    Security,
    /// Economic/financial risks
    Economic,
    /// Compliance risks
    Compliance,
    /// Operational risks
    Operational,
    /// Reputational risks
    Reputational,
}
/// Life event trigger configuration.
#[derive(Debug, Clone)]
pub struct LifeEventTriggerConfig {
    /// Event type
    pub event_type: LifeEventType,
    /// Enable automatic execution
    pub auto_execute: bool,
    /// Require multiple attestations
    pub require_attestations: bool,
    /// Minimum number of attestations
    pub min_attestations: u32,
}
/// Audit preparation configuration.
#[derive(Debug, Clone)]
pub struct AuditPreparationConfig {
    /// Include code documentation review
    pub include_docs_review: bool,
    /// Include test coverage analysis
    pub include_coverage: bool,
    /// Include security checklist
    pub include_checklist: bool,
    /// Include architecture diagrams
    pub include_diagrams: bool,
    /// Audit firm name
    pub audit_firm: Option<String>,
}
/// Intent condition specification.
#[derive(Debug, Clone)]
pub struct IntentCondition {
    /// Condition type
    pub condition_type: IntentConditionType,
    /// Target value or state
    pub target: String,
    /// Comparison operator
    pub operator: String,
    /// Expected value
    pub value: String,
}
/// AI-managed treasury configuration.
#[derive(Debug, Clone)]
pub struct AiManagedTreasuryConfig {
    /// Treasury name
    pub treasury_name: String,
    /// Management strategy
    pub strategy: TreasuryStrategy,
    /// Risk tolerance (0-100)
    pub risk_tolerance: u8,
    /// Rebalancing frequency (in blocks)
    pub rebalancing_frequency: u64,
    /// Asset allocation constraints
    pub allocation_constraints: Vec<AllocationConstraint>,
    /// Performance targets
    pub performance_targets: Vec<PerformanceTarget>,
    /// Emergency withdrawal enabled
    pub emergency_withdrawal: bool,
}
/// Personal legal agent configuration.
#[derive(Debug, Clone)]
pub struct PersonalLegalAgentConfig {
    /// Enable automated compliance monitoring
    pub auto_compliance: bool,
    /// Enable contract review
    pub contract_review: bool,
    /// Enable risk assessment
    pub risk_assessment: bool,
    /// AI model for legal analysis
    pub ai_model_address: Option<String>,
}
/// Jurisdiction type for regulatory compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jurisdiction {
    /// United States
    Us,
    /// European Union
    Eu,
    /// United Kingdom
    Uk,
    /// Singapore
    Sg,
    /// Japan
    Jp,
    /// Switzerland
    Ch,
    /// Custom jurisdiction
    Custom,
}
/// Delay-tolerant verification configuration.
#[derive(Debug, Clone)]
pub struct DelayTolerantVerificationConfig {
    /// Verification name
    pub verification_name: String,
    /// Maximum delay tolerance (in seconds)
    pub max_delay: u64,
    /// Verification method
    pub method: DelayTolerantMethod,
    /// Store intermediate results
    pub store_intermediate: bool,
    /// Batch verification enabled
    pub batch_verification: bool,
    /// Priority level
    pub priority: VerificationPriority,
}
/// Virtual governance configuration.
#[derive(Debug, Clone)]
pub struct VirtualGovernanceConfig {
    /// Enable DAO governance
    pub dao_enabled: bool,
    /// Voting power calculation method
    pub voting_power_method: String,
    /// Enable proposal system
    pub proposal_system: bool,
    /// Quorum percentage (0-100)
    pub quorum_percentage: u8,
}
/// DAO (Decentralized Autonomous Organization) configuration.
#[derive(Debug, Clone)]
pub struct DaoConfig {
    /// DAO name
    pub name: String,
    /// Governance token address
    pub governance_token: String,
    /// Minimum quorum percentage (0-100)
    pub quorum_percentage: u8,
    /// Voting period in blocks
    pub voting_period: u64,
    /// Execution delay in blocks
    pub execution_delay: u64,
    /// Proposal threshold (minimum tokens needed to propose)
    pub proposal_threshold: u64,
}
/// Bundler-compatible entry point configuration (ERC-4337).
#[derive(Debug, Clone)]
pub struct BundlerConfig {
    /// Entry point contract address
    pub entry_point: String,
    /// Enable bundler compatibility
    pub bundler_compatible: bool,
    /// Support user operation batching
    pub batch_operations: bool,
    /// Enable gas sponsorship
    pub gas_sponsorship: bool,
}
/// Audit severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSeverity {
    /// Critical issues
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
    /// Informational
    Info,
}
/// Compliance severity levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceSeverity {
    /// Critical violation
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
    /// Informational
    Info,
}
/// IoT sensor types for environmental monitoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoTSensorType {
    /// Air quality sensor
    AirQuality,
    /// Water quality sensor
    WaterQuality,
    /// Temperature sensor
    Temperature,
    /// Emissions sensor
    Emissions,
    /// Energy consumption
    EnergyConsumption,
}
/// DAO-based statute governance configuration.
#[derive(Debug, Clone)]
pub struct DaoStatuteGovernanceConfig {
    /// Statute identifier
    pub statute_id: String,
    /// Voting period (in blocks)
    pub voting_period: u64,
    /// Quorum percentage (0-100)
    pub quorum_percentage: u8,
    /// Approval threshold percentage (0-100)
    pub approval_threshold: u8,
    /// Proposal cooldown period (in blocks)
    pub proposal_cooldown: u64,
    /// Emergency action enabled
    pub emergency_enabled: bool,
    /// Timelock delay (in seconds)
    pub timelock_delay: u64,
}
/// Verification methods for regulatory compliance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationMethod {
    /// On-chain verification
    OnChain,
    /// Oracle-based verification
    Oracle,
    /// Zero-knowledge proof
    ZkProof,
    /// Multi-signature attestation
    Multisig,
    /// AI-assisted verification
    AiAssisted,
}
/// Lattice-based cryptography patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatticeCryptoPattern {
    /// NTRU lattice-based encryption
    Ntru,
    /// Ring Learning With Errors
    RingLwe,
    /// Module Learning With Errors
    ModuleLwe,
    /// NTRU Prime (optimized variant)
    NtruPrime,
}
/// Intent constraint.
#[derive(Debug, Clone)]
pub struct IntentConstraint {
    /// Constraint type
    pub constraint_type: IntentConstraintType,
    /// Constraint value
    pub value: String,
    /// Strict enforcement flag
    pub strict: bool,
}
/// Target platform for contract generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    /// Solidity for Ethereum/EVM
    Solidity,
    /// Rust for WASM
    RustWasm,
    /// Ink! for Substrate
    Ink,
    /// Vyper for Ethereum/EVM
    Vyper,
    /// Move for Aptos/Sui
    Move,
    /// Cairo for StarkNet
    Cairo,
    /// CosmWasm for Cosmos
    CosmWasm,
    /// FunC for TON
    Ton,
    /// Teal for Algorand
    Teal,
    /// Sway for Fuel Network
    Sway,
    /// Clarity for Stacks (Bitcoin L2)
    Clarity,
    /// Noir for Aztec zkRollup
    Noir,
    /// Leo for Aleo
    Leo,
    /// Circom for ZK circuits
    Circom,
    /// zkSync Era (zkEVM L2)
    ZkSyncEra,
    /// Base (Coinbase L2 - Optimism stack)
    Base,
    /// Arbitrum Stylus (Rust native)
    ArbitrumStylus,
    /// Solana (BPF programs)
    Solana,
    /// Polygon zkEVM
    PolygonZkEvm,
    /// Scroll (zkEVM L2)
    Scroll,
    /// Linea (ConsenSys zkEVM)
    Linea,
    /// Polkadot Asset Hub
    PolkadotAssetHub,
    /// Avalanche Subnet
    AvalancheSubnet,
    /// NEAR Protocol (Rust contracts)
    Near,
}
/// Health data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthDataType {
    /// Vital signs (heart rate, blood pressure, etc.)
    VitalSigns,
    /// Medical records
    MedicalRecords,
    /// Vaccination status
    VaccinationStatus,
    /// Genetic health markers
    GeneticMarkers,
    /// Fitness and activity data
    FitnessData,
}
/// Genetic privacy protection levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneticPrivacyLevel {
    /// Full anonymization
    FullAnonymization,
    /// Pseudonymization
    Pseudonymization,
    /// Controlled access
    ControlledAccess,
    /// Zero-knowledge proofs
    ZeroKnowledge,
}
/// Genetic privacy configuration.
#[derive(Debug, Clone)]
pub struct GeneticPrivacyConfig {
    /// Privacy level
    pub privacy_level: GeneticPrivacyLevel,
    /// Enable consent management
    pub consent_management: bool,
    /// Data retention period (in days)
    pub retention_period: u64,
    /// Enable audit logging
    pub audit_logging: bool,
}
/// Real-time environmental monitoring configuration.
#[derive(Debug, Clone)]
pub struct EnvironmentalMonitoringConfig {
    /// Metrics to monitor
    pub metrics: Vec<EnvironmentalMetric>,
    /// Enable automated compliance checking
    pub auto_compliance: bool,
    /// Enable alerts
    pub alerts_enabled: bool,
    /// Reporting interval in seconds
    pub reporting_interval: u64,
}
/// Biodiversity offset configuration.
#[derive(Debug, Clone)]
pub struct BiodiversityOffsetConfig {
    /// Enable habitat tracking
    pub habitat_tracking: bool,
    /// Enable species monitoring
    pub species_monitoring: bool,
    /// Offset ratio (e.g., 2:1 means 2 units offset per 1 unit impact)
    pub offset_ratio: (u32, u32),
    /// Enable verification system
    pub verification_enabled: bool,
}
/// Quantum-resistant cryptographic patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumResistantPattern {
    /// CRYSTALS-Dilithium signature scheme
    Dilithium,
    /// CRYSTALS-Kyber key encapsulation
    Kyber,
    /// SPHINCS+ hash-based signatures
    SphincsPlus,
    /// Falcon signature scheme
    Falcon,
}
/// Zero-knowledge proof system types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkProofSystem {
    /// Groth16 (fast verification, trusted setup)
    Groth16,
    /// PLONK (universal setup)
    Plonk,
    /// STARK (no trusted setup, larger proofs)
    Stark,
}
/// Dispute resolution configuration.
#[derive(Debug, Clone)]
pub struct DisputeResolutionConfig {
    /// Dispute type
    pub dispute_type: String,
    /// AI model for resolution
    pub model_config: AiModelConfig,
    /// Evidence requirements
    pub evidence_types: Vec<String>,
    /// Resolution threshold (confidence level)
    pub resolution_threshold: u8,
    /// Appeal mechanism enabled
    pub allow_appeal: bool,
    /// Escalation address (for human arbitration)
    pub escalation_address: Option<String>,
}
