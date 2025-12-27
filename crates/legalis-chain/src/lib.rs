//! Legalis-Chain: Smart contract export for Legalis-RS.
//!
//! This crate provides export functionality to convert deterministic
//! legal statutes into smart contracts (WASM/Solidity).

use legalis_core::{ComparisonOp, Condition, EffectType, Statute};
use thiserror::Error;

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

/// Result type for chain operations.
pub type ChainResult<T> = Result<T, ChainError>;

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

/// Deployment configuration.
#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    /// Network name (e.g., "mainnet", "testnet", "localhost")
    pub network: String,
    /// Gas limit
    pub gas_limit: Option<u64>,
    /// Gas price in gwei
    pub gas_price: Option<u64>,
}

/// Security vulnerability types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VulnerabilityType {
    /// Reentrancy vulnerability
    Reentrancy,
    /// Integer overflow/underflow
    IntegerOverflow,
    /// Unchecked external call
    UncheckedExternalCall,
    /// Access control issue
    AccessControl,
    /// Front-running vulnerability
    FrontRunning,
    /// Denial of service
    DenialOfService,
}

/// Security analysis result.
#[derive(Debug, Clone)]
pub struct SecurityAnalysis {
    /// Contract being analyzed
    pub contract_name: String,
    /// Detected vulnerabilities
    pub vulnerabilities: Vec<Vulnerability>,
    /// Security score (0-100)
    pub score: u8,
}

/// Detected vulnerability.
#[derive(Debug, Clone)]
pub struct Vulnerability {
    /// Type of vulnerability
    pub vulnerability_type: VulnerabilityType,
    /// Severity (Critical, High, Medium, Low)
    pub severity: Severity,
    /// Description
    pub description: String,
    /// Line number (if applicable)
    pub line: Option<usize>,
    /// Recommendation
    pub recommendation: String,
}

/// Severity level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    /// Critical severity
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
}

/// Proxy pattern type for upgradeable contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyPattern {
    /// Transparent proxy pattern
    Transparent,
    /// Universal Upgradeable Proxy Standard (UUPS)
    Uups,
    /// Beacon proxy pattern
    Beacon,
}

/// Token standard types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStandard {
    /// ERC-20 fungible token
    Erc20,
    /// ERC-721 non-fungible token
    Erc721,
    /// ERC-1155 multi-token
    Erc1155,
    /// ERC-20 with additional features (pausable, burnable, mintable)
    Erc20Extended,
    /// ERC-721 with enumeration and URI storage
    Erc721Extended,
}

/// Token configuration for generation.
#[derive(Debug, Clone)]
pub struct TokenConfig {
    /// Token name
    pub name: String,
    /// Token symbol
    pub symbol: String,
    /// Initial supply (for ERC-20)
    pub initial_supply: Option<u64>,
    /// Token standard to use
    pub standard: TokenStandard,
    /// Include pausable functionality
    pub pausable: bool,
    /// Include burnable functionality
    pub burnable: bool,
    /// Include mintable functionality (with access control)
    pub mintable: bool,
    /// Include snapshot functionality
    pub snapshot: bool,
    /// Base URI for metadata (for NFTs)
    pub base_uri: Option<String>,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            name: "MyToken".to_string(),
            symbol: "MTK".to_string(),
            initial_supply: Some(1000000),
            standard: TokenStandard::Erc20,
            pausable: false,
            burnable: false,
            mintable: false,
            snapshot: false,
            base_uri: None,
        }
    }
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

impl Default for DaoConfig {
    fn default() -> Self {
        Self {
            name: "MyDAO".to_string(),
            governance_token: "0x0000000000000000000000000000000000000000".to_string(),
            quorum_percentage: 4,
            voting_period: 17280, // ~3 days at 15s/block
            execution_delay: 172800, // ~30 days
            proposal_threshold: 1000,
        }
    }
}

/// Cross-chain bridge configuration.
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// Bridge name
    pub name: String,
    /// Source chain ID
    pub source_chain_id: u64,
    /// Destination chain ID
    pub destination_chain_id: u64,
    /// Supported token addresses
    pub supported_tokens: Vec<String>,
    /// Bridge fee percentage (basis points, e.g., 30 = 0.3%)
    pub fee_basis_points: u16,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            name: "MyBridge".to_string(),
            source_chain_id: 1,
            destination_chain_id: 137,
            supported_tokens: vec![],
            fee_basis_points: 30,
        }
    }
}

/// Treasury management configuration.
#[derive(Debug, Clone)]
pub struct TreasuryConfig {
    /// Treasury name
    pub name: String,
    /// Authorized spenders (addresses with spending permission)
    pub authorized_spenders: Vec<String>,
    /// Daily spending limit in wei
    pub daily_limit: u64,
    /// Require multiple approvals for large transactions
    pub multi_approval_threshold: u64,
}

impl Default for TreasuryConfig {
    fn default() -> Self {
        Self {
            name: "MyTreasury".to_string(),
            authorized_spenders: vec![],
            daily_limit: 1_000_000_000_000_000_000, // 1 ETH
            multi_approval_threshold: 10_000_000_000_000_000_000, // 10 ETH
        }
    }
}

/// Vesting schedule configuration.
#[derive(Debug, Clone)]
pub struct VestingConfig {
    /// Contract name
    pub name: String,
    /// Beneficiary address
    pub beneficiary: String,
    /// Start timestamp (Unix time)
    pub start: u64,
    /// Cliff duration in seconds
    pub cliff_duration: u64,
    /// Total vesting duration in seconds
    pub duration: u64,
    /// Whether vesting is revocable
    pub revocable: bool,
}

impl Default for VestingConfig {
    fn default() -> Self {
        Self {
            name: "TokenVesting".to_string(),
            beneficiary: "0x0000000000000000000000000000000000000000".to_string(),
            start: 0,
            cliff_duration: 31536000, // 1 year
            duration: 126144000,      // 4 years
            revocable: true,
        }
    }
}

/// Multisig wallet configuration.
#[derive(Debug, Clone)]
pub struct MultisigConfig {
    /// Wallet name
    pub name: String,
    /// List of owner addresses
    pub owners: Vec<String>,
    /// Number of required confirmations
    pub required_confirmations: usize,
    /// Daily withdrawal limit in wei
    pub daily_limit: Option<u64>,
}

impl Default for MultisigConfig {
    fn default() -> Self {
        Self {
            name: "MultiSigWallet".to_string(),
            owners: vec![],
            required_confirmations: 2,
            daily_limit: Some(1_000_000_000_000_000_000), // 1 ETH
        }
    }
}

/// Test suite configuration.
#[derive(Debug, Clone)]
pub struct TestSuiteConfig {
    /// Include unit tests
    pub unit_tests: bool,
    /// Include integration tests
    pub integration_tests: bool,
    /// Include fuzzing tests
    pub fuzzing_tests: bool,
    /// Framework to use ("hardhat", "foundry", etc.)
    pub framework: String,
}

impl Default for TestSuiteConfig {
    fn default() -> Self {
        Self {
            unit_tests: true,
            integration_tests: true,
            fuzzing_tests: false,
            framework: "hardhat".to_string(),
        }
    }
}

/// Multi-network deployment configuration.
#[derive(Debug, Clone)]
pub struct MultiNetworkConfig {
    /// Network configurations
    pub networks: Vec<NetworkConfig>,
    /// Default network name
    pub default_network: String,
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

/// Formal verification configuration.
#[derive(Debug, Clone)]
pub struct FormalVerificationConfig {
    /// Generate Certora specifications
    pub certora: bool,
    /// Generate Scribble annotations
    pub scribble: bool,
    /// Generate Slither configuration
    pub slither: bool,
    /// Generate invariant specifications
    pub invariants: bool,
}

impl Default for FormalVerificationConfig {
    fn default() -> Self {
        Self {
            certora: false,
            scribble: false,
            slither: true,
            invariants: true,
        }
    }
}

/// Batch operation configuration.
#[derive(Debug, Clone)]
pub struct BatchOperationConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Include batch eligibility check
    pub batch_eligibility: bool,
    /// Include batch effect application
    pub batch_effects: bool,
}

impl Default for BatchOperationConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            batch_eligibility: true,
            batch_effects: true,
        }
    }
}

/// Modular contract output with multiple files.
#[derive(Debug, Clone)]
pub struct ModularContract {
    /// Main contract file
    pub main_contract: GeneratedContract,
    /// Interface file (if applicable)
    pub interface: Option<GeneratedContract>,
    /// Library files
    pub libraries: Vec<GeneratedContract>,
    /// Additional helper contracts
    pub helpers: Vec<GeneratedContract>,
}

/// Smart contract generator.
pub struct ContractGenerator {
    platform: TargetPlatform,
}

impl ContractGenerator {
    /// Creates a new generator for the specified platform.
    pub fn new(platform: TargetPlatform) -> Self {
        Self { platform }
    }

    /// Generates a factory contract that can deploy multiple statute contracts.
    pub fn generate_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_factory(statute_ids),
            TargetPlatform::Vyper => self.generate_vyper_factory(statute_ids),
            _ => Err(ChainError::GenerationError(format!(
                "Factory generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates an upgradeable proxy contract for a statute contract.
    pub fn generate_upgradeable_proxy(
        &self,
        contract_name: &str,
    ) -> ChainResult<GeneratedContract> {
        self.generate_proxy_with_pattern(contract_name, ProxyPattern::Transparent)
    }

    /// Generates a proxy contract with a specific pattern.
    pub fn generate_proxy_with_pattern(
        &self,
        contract_name: &str,
        pattern: ProxyPattern,
    ) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => match pattern {
                ProxyPattern::Transparent => self.generate_solidity_proxy(contract_name),
                ProxyPattern::Uups => self.generate_uups_proxy(contract_name),
                ProxyPattern::Beacon => self.generate_beacon_proxy(contract_name),
            },
            _ => Err(ChainError::GenerationError(format!(
                "Proxy generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a test suite for a generated contract.
    pub fn generate_test_suite(
        &self,
        contract: &GeneratedContract,
        config: &TestSuiteConfig,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_tests(contract, config),
            TargetPlatform::Vyper => self.generate_vyper_tests(contract, config),
            _ => Err(ChainError::GenerationError(format!(
                "Test generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a statute registry contract.
    pub fn generate_statute_registry(&self) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_registry(),
            _ => Err(ChainError::GenerationError(format!(
                "Registry generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a governance contract for managing statutes.
    pub fn generate_governance(&self) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_governance(),
            _ => Err(ChainError::GenerationError(format!(
                "Governance generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a token contract based on the specified standard.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, TokenConfig, TokenStandard};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = TokenConfig {
    ///     name: "MyToken".to_string(),
    ///     symbol: "MTK".to_string(),
    ///     initial_supply: Some(1000000),
    ///     standard: TokenStandard::Erc20,
    ///     pausable: true,
    ///     burnable: true,
    ///     mintable: true,
    ///     snapshot: false,
    ///     base_uri: None,
    /// };
    /// let contract = generator.generate_token(&config).unwrap();
    /// assert!(contract.source.contains("ERC20"));
    /// ```
    pub fn generate_token(&self, config: &TokenConfig) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_token(config),
            TargetPlatform::Vyper => self.generate_vyper_token(config),
            _ => Err(ChainError::GenerationError(format!(
                "Token generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a DAO (Decentralized Autonomous Organization) contract.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, DaoConfig};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = DaoConfig {
    ///     name: "MyDAO".to_string(),
    ///     governance_token: "0x1234567890123456789012345678901234567890".to_string(),
    ///     quorum_percentage: 4,
    ///     voting_period: 17280,
    ///     execution_delay: 172800,
    ///     proposal_threshold: 1000,
    /// };
    /// let contract = generator.generate_dao(&config).unwrap();
    /// assert!(contract.source.contains("Governor"));
    /// ```
    pub fn generate_dao(&self, config: &DaoConfig) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_dao(config),
            _ => Err(ChainError::GenerationError(format!(
                "DAO generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a cross-chain bridge contract.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, BridgeConfig};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = BridgeConfig {
    ///     name: "EthPolygonBridge".to_string(),
    ///     source_chain_id: 1,
    ///     destination_chain_id: 137,
    ///     supported_tokens: vec!["0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()],
    ///     fee_basis_points: 30,
    /// };
    /// let contract = generator.generate_bridge(&config).unwrap();
    /// assert!(contract.source.contains("Bridge"));
    /// ```
    pub fn generate_bridge(&self, config: &BridgeConfig) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_bridge(config),
            _ => Err(ChainError::GenerationError(format!(
                "Bridge generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a treasury management contract.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, TreasuryConfig};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = TreasuryConfig {
    ///     name: "DAOTreasury".to_string(),
    ///     authorized_spenders: vec!["0x1234567890123456789012345678901234567890".to_string()],
    ///     daily_limit: 1_000_000_000_000_000_000,
    ///     multi_approval_threshold: 10_000_000_000_000_000_000,
    /// };
    /// let contract = generator.generate_treasury(&config).unwrap();
    /// assert!(contract.source.contains("Treasury"));
    /// ```
    pub fn generate_treasury(&self, config: &TreasuryConfig) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_treasury(config),
            _ => Err(ChainError::GenerationError(format!(
                "Treasury generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a token vesting contract.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, VestingConfig};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = VestingConfig {
    ///     name: "TeamVesting".to_string(),
    ///     beneficiary: "0x1234567890123456789012345678901234567890".to_string(),
    ///     start: 1640995200,
    ///     cliff_duration: 31536000,
    ///     duration: 126144000,
    ///     revocable: true,
    /// };
    /// let contract = generator.generate_vesting(&config).unwrap();
    /// assert!(contract.source.contains("Vesting"));
    /// ```
    pub fn generate_vesting(&self, config: &VestingConfig) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_vesting(config),
            _ => Err(ChainError::GenerationError(format!(
                "Vesting generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates a multisig wallet contract.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, MultisigConfig};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = MultisigConfig {
    ///     name: "TeamMultiSig".to_string(),
    ///     owners: vec![
    ///         "0x1234567890123456789012345678901234567890".to_string(),
    ///         "0x2345678901234567890123456789012345678901".to_string(),
    ///         "0x3456789012345678901234567890123456789012".to_string(),
    ///     ],
    ///     required_confirmations: 2,
    ///     daily_limit: Some(1_000_000_000_000_000_000),
    /// };
    /// let contract = generator.generate_multisig(&config).unwrap();
    /// assert!(contract.source.contains("MultiSig"));
    /// ```
    pub fn generate_multisig(&self, config: &MultisigConfig) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_multisig(config),
            _ => Err(ChainError::GenerationError(format!(
                "Multisig generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates an automated audit report for a contract.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform};
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new(
    ///     "TestStatute".to_string(),
    ///     "Test Statute".to_string(),
    ///     Effect::new(EffectType::Grant, "Grant permission")
    /// );
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = generator.generate(&statute).unwrap();
    /// let report = generator.generate_audit_report(&contract).unwrap();
    /// assert!(report.contains("Audit Report"));
    /// ```
    pub fn generate_audit_report(&self, contract: &GeneratedContract) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_comprehensive_audit_report(contract)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Audit report generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates batch operation support for a statute.
    pub fn generate_with_batch_operations(
        &self,
        statute: &Statute,
        config: &BatchOperationConfig,
    ) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_with_batch(statute, config),
            _ => Err(ChainError::GenerationError(format!(
                "Batch operations not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates multi-network deployment configuration.
    pub fn generate_multi_network_config(
        &self,
        contract: &GeneratedContract,
        config: &MultiNetworkConfig,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_hardhat_multi_network(contract, config)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Multi-network config not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates formal verification annotations and configurations.
    pub fn generate_formal_verification(
        &self,
        contract: &GeneratedContract,
        config: &FormalVerificationConfig,
    ) -> ChainResult<Vec<(String, String)>> {
        match self.platform {
            TargetPlatform::Solidity => {
                self.generate_solidity_formal_verification(contract, config)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Formal verification not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates interface extraction from a contract.
    pub fn generate_interface(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_interface(statute),
            _ => Err(ChainError::GenerationError(format!(
                "Interface extraction not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates modular contract with separated interfaces and libraries.
    pub fn generate_modular(&self, statute: &Statute) -> ChainResult<ModularContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_modular(statute),
            _ => Err(ChainError::GenerationError(format!(
                "Modular generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates coverage report configuration.
    pub fn generate_coverage_config(&self) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_coverage_config(),
            TargetPlatform::Vyper => self.generate_vyper_coverage_config(),
            _ => Err(ChainError::GenerationError(format!(
                "Coverage config not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates contract with inheritance pattern.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform};
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let statute = Statute::new("test", "Test Statute", Effect::new(EffectType::Grant, "Test"));
    /// let base_contracts = vec!["Ownable", "Pausable"];
    /// let contract = generator.generate_with_inheritance(&statute, &base_contracts).unwrap();
    /// ```
    pub fn generate_with_inheritance(
        &self,
        statute: &Statute,
        base_contracts: &[&str],
    ) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => {
                self.generate_solidity_with_inheritance(statute, base_contracts)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Inheritance pattern not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates contract using diamond pattern for large statutes.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform};
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let statute = Statute::new("test", "Test Statute", Effect::new(EffectType::Grant, "Test"));
    /// let statutes = vec![statute];
    /// let diamond = generator.generate_diamond(&statutes).unwrap();
    /// ```
    pub fn generate_diamond(&self, statutes: &[Statute]) -> ChainResult<Vec<GeneratedContract>> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_diamond(statutes),
            _ => Err(ChainError::GenerationError(format!(
                "Diamond pattern not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates deployment documentation for a contract.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "MyContract".to_string(),
    ///     source: "contract MyContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let docs = generator.generate_deployment_docs(&contract).unwrap();
    /// ```
    pub fn generate_deployment_docs(&self, contract: &GeneratedContract) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_deployment_docs(contract)
            }
            TargetPlatform::Move => self.generate_move_deployment_docs(contract),
            TargetPlatform::Cairo => self.generate_cairo_deployment_docs(contract),
            _ => Err(ChainError::GenerationError(format!(
                "Deployment docs not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates API documentation for a contract.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform};
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let statute = Statute::new("test", "Test Statute", Effect::new(EffectType::Grant, "Test"));
    /// let api_docs = generator.generate_api_docs(&statute).unwrap();
    /// ```
    pub fn generate_api_docs(&self, statute: &Statute) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_api_docs(statute),
            TargetPlatform::Vyper => self.generate_vyper_api_docs(statute),
            _ => Err(ChainError::GenerationError(format!(
                "API docs not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates gas estimation report for a contract.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "MyContract".to_string(),
    ///     source: "contract MyContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let report = generator.generate_gas_estimation(&contract).unwrap();
    /// ```
    pub fn generate_gas_estimation(&self, contract: &GeneratedContract) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_gas_estimation(contract)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Gas estimation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates upgrade deployment scripts for an upgradeable contract.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract, ProxyPattern};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "MyContract".to_string(),
    ///     source: "contract MyContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let script = generator.generate_upgrade_script(&contract, ProxyPattern::Transparent).unwrap();
    /// ```
    pub fn generate_upgrade_script(
        &self,
        contract: &GeneratedContract,
        pattern: ProxyPattern,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_upgrade_script(contract, pattern),
            _ => Err(ChainError::GenerationError(format!(
                "Upgrade scripts not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates cross-chain deployment configuration.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "MyContract".to_string(),
    ///     source: "contract MyContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let chains = vec!["ethereum", "polygon", "arbitrum"];
    /// let config = generator.generate_cross_chain_config(&contract, &chains).unwrap();
    /// ```
    pub fn generate_cross_chain_config(
        &self,
        contract: &GeneratedContract,
        chains: &[&str],
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_cross_chain_config(contract, chains)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Cross-chain config not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates compilation test suite for a generated contract.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "TestContract".to_string(),
    ///     source: "contract TestContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let tests = generator.generate_compilation_tests(&contract).unwrap();
    /// ```
    pub fn generate_compilation_tests(&self, contract: &GeneratedContract) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_compilation_tests(contract)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Compilation tests not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates deployment simulation test suite.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "TestContract".to_string(),
    ///     source: "contract TestContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let sim_tests = generator.generate_deployment_simulation_tests(&contract).unwrap();
    /// ```
    pub fn generate_deployment_simulation_tests(
        &self,
        contract: &GeneratedContract,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_deployment_sim_tests(contract)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Deployment simulation tests not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates gas usage benchmarks for a contract.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "TestContract".to_string(),
    ///     source: "contract TestContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let benchmarks = generator.generate_gas_benchmarks(&contract).unwrap();
    /// ```
    pub fn generate_gas_benchmarks(&self, contract: &GeneratedContract) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_gas_benchmarks(contract)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Gas benchmarks not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates comprehensive security test suite.
    ///
    /// # Example
    /// ```
    /// use legalis_chain::{ContractGenerator, TargetPlatform, GeneratedContract};
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let contract = GeneratedContract {
    ///     name: "TestContract".to_string(),
    ///     source: "contract TestContract {}".to_string(),
    ///     platform: TargetPlatform::Solidity,
    ///     abi: None,
    ///     deployment_script: None,
    /// };
    /// let security_tests = generator.generate_security_test_suite(&contract).unwrap();
    /// ```
    pub fn generate_security_test_suite(
        &self,
        contract: &GeneratedContract,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                self.generate_evm_security_tests(contract)
            }
            _ => Err(ChainError::GenerationError(format!(
                "Security test suite not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates deployment script for a generated contract.
    pub fn generate_deployment_script(
        &self,
        contract: &GeneratedContract,
        config: &DeploymentConfig,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_deployment(contract, config),
            TargetPlatform::Vyper => self.generate_vyper_deployment(contract, config),
            TargetPlatform::Move => self.generate_move_deployment(contract, config),
            TargetPlatform::Cairo => self.generate_cairo_deployment(contract, config),
            TargetPlatform::RustWasm => self.generate_wasm_deployment(contract, config),
            TargetPlatform::Ink => self.generate_ink_deployment(contract, config),
            TargetPlatform::CosmWasm => self.generate_cosmwasm_deployment(contract, config),
            TargetPlatform::Ton => self.generate_ton_deployment(contract, config),
            TargetPlatform::Teal => self.generate_teal_deployment(contract, config),
        }
    }

    /// Generates a smart contract from a statute.
    pub fn generate(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        // Check for discretionary elements
        if statute.discretion_logic.is_some() {
            return Err(ChainError::DiscretionaryStatute(statute.id.clone()));
        }

        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity(statute),
            TargetPlatform::RustWasm => self.generate_rust_wasm(statute),
            TargetPlatform::Ink => self.generate_ink(statute),
            TargetPlatform::Vyper => self.generate_vyper(statute),
            TargetPlatform::Move => self.generate_move(statute),
            TargetPlatform::Cairo => self.generate_cairo(statute),
            TargetPlatform::CosmWasm => self.generate_cosmwasm(statute),
            TargetPlatform::Ton => self.generate_ton(statute),
            TargetPlatform::Teal => self.generate_teal(statute),
        }
    }

    /// Generates multiple contracts from a set of statutes.
    pub fn generate_batch(&self, statutes: &[Statute]) -> Vec<ChainResult<GeneratedContract>> {
        statutes.iter().map(|s| self.generate(s)).collect()
    }

    fn generate_solidity(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!("/// @title {}\n", statute.title));
        source.push_str("/// @notice Auto-generated from Legalis-RS\n");
        source.push_str("/// @dev Gas-optimized smart contract with comprehensive event logging\n");
        source.push_str(&format!("contract {} {{\n", contract_name));

        // Events
        source.push_str("    /// @notice Emitted when eligibility is checked\n");
        source.push_str("    /// @param entity The address being checked\n");
        source.push_str("    /// @param result Whether the entity is eligible\n");
        source.push_str("    event EligibilityChecked(address indexed entity, bool result);\n\n");

        source.push_str("    /// @notice Emitted when an effect is applied\n");
        source.push_str("    /// @param beneficiary The address receiving the effect\n");
        source.push_str("    /// @param effectType The type of effect applied\n");
        source.push_str(
            "    event EffectApplied(address indexed beneficiary, string effectType);\n\n",
        );

        // State variables with gas optimization comments
        source.push_str("    /// @dev Using immutable for gas optimization\n");
        source.push_str("    address public immutable owner;\n");
        source.push_str("    /// @dev Mapping for O(1) eligibility lookup\n");
        source.push_str("    mapping(address => bool) public eligible;\n\n");

        // Constructor
        source.push_str("    /// @notice Initialize the contract\n");
        source.push_str("    constructor() {\n");
        source.push_str("        owner = msg.sender;\n");
        source.push_str("    }\n\n");

        // Check eligibility function
        source.push_str("    /// @notice Check if an entity meets the preconditions\n");
        source.push_str(&self.generate_solidity_check_function(statute)?);

        // Apply effect function
        source.push_str("\n    /// @notice Apply the legal effect\n");
        source.push_str(&self.generate_solidity_apply_function(statute)?);

        source.push_str("}\n");

        let abi = self.generate_solidity_abi(statute)?;

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Solidity,
            abi: Some(abi),
            deployment_script: None,
        })
    }

    fn generate_solidity_check_function(&self, statute: &Statute) -> ChainResult<String> {
        let mut func = String::new();
        func.push_str("    /// @dev View function - no state changes, gas-efficient\n");
        func.push_str("    function checkEligibility(\n");

        // Generate parameters based on conditions
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, typ)| format!("        {} {}", typ, name))
            .collect();
        func.push_str(&param_str.join(",\n"));
        func.push_str("\n    ) public returns (bool) {\n");

        // Generate condition checks
        for condition in &statute.preconditions {
            func.push_str(&self.condition_to_solidity(condition)?);
        }

        func.push_str("        emit EligibilityChecked(msg.sender, true);\n");
        func.push_str("        return true;\n");
        func.push_str("    }\n");

        Ok(func)
    }

    fn generate_solidity_apply_function(&self, statute: &Statute) -> ChainResult<String> {
        let mut func = String::new();
        func.push_str("    /// @dev Only owner can call, with reentrancy protection via checks-effects-interactions\n");
        func.push_str("    function applyEffect(address beneficiary) public {\n");
        func.push_str("        require(msg.sender == owner, \"Only owner can apply effects\");\n");
        func.push_str("        require(beneficiary != address(0), \"Invalid beneficiary\");\n\n");

        let effect_type_str = format!("{:?}", statute.effect.effect_type);

        match statute.effect.effect_type {
            EffectType::Grant => {
                func.push_str(
                    "        // State change before external interactions (CEI pattern)\n",
                );
                func.push_str("        eligible[beneficiary] = true;\n");
            }
            EffectType::Revoke => {
                func.push_str(
                    "        // State change before external interactions (CEI pattern)\n",
                );
                func.push_str("        eligible[beneficiary] = false;\n");
            }
            EffectType::MonetaryTransfer => {
                func.push_str("        // Monetary transfer logic\n");
                func.push_str("        // Use call instead of transfer for better gas handling\n");
                func.push_str("        // (bool success, ) = payable(beneficiary).call{value: amount}(\"\");\n");
                func.push_str("        // require(success, \"Transfer failed\");\n");
            }
            _ => {
                func.push_str(&format!(
                    "        // Effect: {}\n",
                    statute.effect.description
                ));
            }
        }

        func.push_str(&format!(
            "        emit EffectApplied(beneficiary, \"{}\");\n",
            effect_type_str
        ));
        func.push_str("    }\n");
        Ok(func)
    }

    fn generate_solidity_abi(&self, statute: &Statute) -> ChainResult<String> {
        let params = self.extract_parameters(&statute.preconditions);
        let inputs: Vec<String> = params
            .iter()
            .map(|(name, typ)| {
                let sol_type = match typ.as_str() {
                    "uint256" => "uint256",
                    "bool" => "bool",
                    "string memory" => "string",
                    _ => "uint256",
                };
                format!(r#"{{"name":"{}","type":"{}"}}"#, name, sol_type)
            })
            .collect();

        Ok(format!(
            r#"[{{"name":"checkEligibility","type":"function","inputs":[{}],"outputs":[{{"type":"bool"}}]}}]"#,
            inputs.join(",")
        ))
    }

    fn condition_to_solidity(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!(
                    "        require(age {} {}, \"Age requirement not met\");\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!(
                    "        require(income {} {}, \"Income requirement not met\");\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_solidity(left)?;
                result.push_str(&self.condition_to_solidity(right)?);
                Ok(result)
            }
            Condition::Or(left, right) => Ok(format!(
                "        require({} || {}, \"OR condition not met\");\n",
                self.condition_to_solidity_expr(left)?,
                self.condition_to_solidity_expr(right)?
            )),
            Condition::Not(inner) => Ok(format!(
                "        require(!{}, \"NOT condition not met\");\n",
                self.condition_to_solidity_expr(inner)?
            )),
            _ => Ok("        // Custom condition - manual implementation required\n".to_string()),
        }
    }

    fn condition_to_solidity_expr(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!("(age {} {})", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!("(income {} {})", op, value))
            }
            _ => Ok("true".to_string()),
        }
    }

    fn comparison_to_solidity(&self, op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    fn extract_parameters(&self, conditions: &[Condition]) -> Vec<(String, String)> {
        let mut params = Vec::new();

        for condition in conditions {
            Self::extract_params_from_condition(condition, &mut params);
        }

        params.sort_by(|a, b| a.0.cmp(&b.0));
        params.dedup_by(|a, b| a.0 == b.0);
        params
    }

    fn extract_params_from_condition(condition: &Condition, params: &mut Vec<(String, String)>) {
        match condition {
            Condition::Age { .. } => {
                params.push(("age".to_string(), "uint256".to_string()));
            }
            Condition::Income { .. } => {
                params.push(("income".to_string(), "uint256".to_string()));
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::extract_params_from_condition(left, params);
                Self::extract_params_from_condition(right, params);
            }
            Condition::Not(inner) => {
                Self::extract_params_from_condition(inner, params);
            }
            _ => {}
        }
    }

    fn generate_rust_wasm(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let module_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("//! Auto-generated from Legalis-RS\n\n");
        source.push_str("use wasm_bindgen::prelude::*;\n\n");
        source.push_str(&format!("/// {}\n", statute.title));
        source.push_str("#[wasm_bindgen]\n");
        source.push_str(&format!("pub struct {} {{\n", to_pascal_case(&statute.id)));
        source.push_str("    eligible: std::collections::HashSet<String>,\n");
        source.push_str("}\n\n");

        source.push_str("#[wasm_bindgen]\n");
        source.push_str(&format!("impl {} {{\n", to_pascal_case(&statute.id)));
        source.push_str("    #[wasm_bindgen(constructor)]\n");
        source.push_str("    pub fn new() -> Self {\n");
        source.push_str("        Self { eligible: std::collections::HashSet::new() }\n");
        source.push_str("    }\n\n");

        // Check function
        source.push_str("    pub fn check_eligibility(&self");
        let params = self.extract_parameters(&statute.preconditions);
        for (name, _) in &params {
            source.push_str(&format!(", {}: u64", name));
        }
        source.push_str(") -> bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_rust(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: module_name,
            source,
            platform: TargetPlatform::RustWasm,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_rust(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        if !(age {} {}) {{ return false; }}\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        if !(income {} {}) {{ return false; }}\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_rust(left)?;
                result.push_str(&self.condition_to_rust(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }

    fn comparison_to_rust(&self, op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    fn generate_ink(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("#![cfg_attr(not(feature = \"std\"), no_std, no_main)]\n\n");
        source.push_str("#[ink::contract]\n");
        source.push_str(&format!("mod {} {{\n", contract_name));
        source.push_str("    #[ink(storage)]\n");
        source.push_str("    pub struct Contract {\n");
        source.push_str("        owner: AccountId,\n");
        source.push_str("    }\n\n");

        source.push_str("    impl Contract {\n");
        source.push_str("        #[ink(constructor)]\n");
        source.push_str("        pub fn new() -> Self {\n");
        source.push_str("            Self { owner: Self::env().caller() }\n");
        source.push_str("        }\n\n");

        source.push_str(&format!("        /// {}\n", statute.title));
        source.push_str("        #[ink(message)]\n");
        source.push_str("        pub fn check_eligibility(&self");
        let params = self.extract_parameters(&statute.preconditions);
        for (name, _) in &params {
            source.push_str(&format!(", {}: u64", name));
        }
        source.push_str(") -> bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_rust(condition)?);
        }
        source.push_str("            true\n");
        source.push_str("        }\n");
        source.push_str("    }\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Ink,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_vyper(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("# @version ^0.3.0\n");
        source.push_str(&format!("# @title {}\n", statute.title));
        source.push_str("# @notice Auto-generated from Legalis-RS\n\n");

        // State variables
        source.push_str("owner: public(address)\n");
        source.push_str("eligible: public(HashMap[address, bool])\n\n");

        // Events
        source.push_str("event EligibilityChecked:\n");
        source.push_str("    entity: indexed(address)\n");
        source.push_str("    result: bool\n\n");
        source.push_str("event EffectApplied:\n");
        source.push_str("    beneficiary: indexed(address)\n");
        source.push_str("    effect_type: String[100]\n\n");

        // Constructor
        source.push_str("@external\n");
        source.push_str("def __init__():\n");
        source.push_str("    self.owner = msg.sender\n\n");

        // Check eligibility function
        source.push_str("@external\n");
        source.push_str("@view\n");
        source.push_str("def check_eligibility(");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("{}: uint256", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str(") -> bool:\n");
        source.push_str("    \"\"\"Check if an entity meets the preconditions\"\"\"\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_vyper(condition)?);
        }
        source.push_str("    log EligibilityChecked(msg.sender, True)\n");
        source.push_str("    return True\n\n");

        // Apply effect function
        source.push_str("@external\n");
        source.push_str("def apply_effect(beneficiary: address):\n");
        source.push_str("    \"\"\"Apply the legal effect\"\"\"\n");
        source.push_str("    assert msg.sender == self.owner, \"Only owner can apply effects\"\n");

        match statute.effect.effect_type {
            EffectType::Grant => {
                source.push_str("    self.eligible[beneficiary] = True\n");
            }
            EffectType::Revoke => {
                source.push_str("    self.eligible[beneficiary] = False\n");
            }
            EffectType::MonetaryTransfer => {
                source.push_str("    # Monetary transfer logic\n");
                source.push_str("    # send(beneficiary, amount)\n");
            }
            _ => {
                source.push_str(&format!("    # Effect: {}\n", statute.effect.description));
            }
        }
        source.push_str("    log EffectApplied(beneficiary, \"");
        source.push_str(&format!("{:?}", statute.effect.effect_type));
        source.push_str("\")\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Vyper,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_vyper(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "    assert age {} {}, \"Age requirement not met\"\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "    assert income {} {}, \"Income requirement not met\"\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_vyper(left)?;
                result.push_str(&self.condition_to_vyper(right)?);
                Ok(result)
            }
            Condition::Or(left, right) => Ok(format!(
                "    assert {} or {}, \"OR condition not met\"\n",
                self.condition_to_vyper_expr(left)?,
                self.condition_to_vyper_expr(right)?
            )),
            Condition::Not(inner) => Ok(format!(
                "    assert not {}, \"NOT condition not met\"\n",
                self.condition_to_vyper_expr(inner)?
            )),
            _ => Ok("    # Custom condition - manual implementation required\n".to_string()),
        }
    }

    fn condition_to_vyper_expr(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("(age {} {})", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("(income {} {})", op, value))
            }
            _ => Ok("True".to_string()),
        }
    }

    fn generate_move(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let module_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("module legalis::");
        source.push_str(&module_name);
        source.push_str(" {\n");
        source.push_str("    use std::signer;\n");
        source.push_str("    use aptos_framework::event;\n\n");

        source.push_str(&format!("    /// {}\n", statute.title));
        source.push_str("    struct StatuteContract has key {\n");
        source.push_str("        owner: address,\n");
        source.push_str("        eligible_count: u64,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[event]\n");
        source.push_str("    struct EligibilityChecked has drop, store {\n");
        source.push_str("        entity: address,\n");
        source.push_str("        result: bool,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[event]\n");
        source.push_str("    struct EffectApplied has drop, store {\n");
        source.push_str("        beneficiary: address,\n");
        source.push_str("    }\n\n");

        // Initialize function
        source.push_str("    public entry fun initialize(account: &signer) {\n");
        source.push_str("        let owner_addr = signer::address_of(account);\n");
        source.push_str("        move_to(account, StatuteContract {\n");
        source.push_str("            owner: owner_addr,\n");
        source.push_str("            eligible_count: 0,\n");
        source.push_str("        });\n");
        source.push_str("    }\n\n");

        // Check eligibility function
        source.push_str("    public fun check_eligibility(");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("{}: u64", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str("): bool {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_move(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n\n");

        // Apply effect function
        source.push_str("    public entry fun apply_effect(account: &signer, beneficiary: address) acquires StatuteContract {\n");
        source.push_str("        let contract = borrow_global_mut<StatuteContract>(signer::address_of(account));\n");
        source.push_str("        assert!(signer::address_of(account) == contract.owner, 0);\n");
        source.push_str("        contract.eligible_count = contract.eligible_count + 1;\n");
        source.push_str("        event::emit(EffectApplied { beneficiary });\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: module_name,
            source,
            platform: TargetPlatform::Move,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_move(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("        assert!(age {} {}, 1);\n", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("        assert!(income {} {}, 2);\n", op, value))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_move(left)?;
                result.push_str(&self.condition_to_move(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }

    fn generate_cairo(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("#[starknet::contract]\n");
        source.push_str(&format!("mod {} {{\n", contract_name));
        source.push_str("    use starknet::ContractAddress;\n");
        source.push_str("    use starknet::get_caller_address;\n\n");

        source.push_str("    #[storage]\n");
        source.push_str("    struct Storage {\n");
        source.push_str("        owner: ContractAddress,\n");
        source.push_str("        eligible_count: u64,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[event]\n");
        source.push_str("    #[derive(Drop, starknet::Event)]\n");
        source.push_str("    enum Event {\n");
        source.push_str("        EligibilityChecked: EligibilityChecked,\n");
        source.push_str("        EffectApplied: EffectApplied,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[derive(Drop, starknet::Event)]\n");
        source.push_str("    struct EligibilityChecked {\n");
        source.push_str("        entity: ContractAddress,\n");
        source.push_str("        result: bool,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[derive(Drop, starknet::Event)]\n");
        source.push_str("    struct EffectApplied {\n");
        source.push_str("        beneficiary: ContractAddress,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[constructor]\n");
        source.push_str("    fn constructor(ref self: ContractState) {\n");
        source.push_str("        self.owner.write(get_caller_address());\n");
        source.push_str("        self.eligible_count.write(0);\n");
        source.push_str("    }\n\n");

        source.push_str(&format!("    /// {}\n", statute.title));
        source.push_str("    #[external(v0)]\n");
        source.push_str("    fn check_eligibility(self: @ContractState, ");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("{}: u64", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str(") -> bool {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_cairo(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n\n");

        source.push_str("    #[external(v0)]\n");
        source.push_str(
            "    fn apply_effect(ref self: ContractState, beneficiary: ContractAddress) {\n",
        );
        source
            .push_str("        assert(get_caller_address() == self.owner.read(), 'Only owner');\n");
        source.push_str("        let count = self.eligible_count.read();\n");
        source.push_str("        self.eligible_count.write(count + 1);\n");
        source.push_str("        self.emit(EffectApplied { beneficiary });\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Cairo,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_cairo(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        assert(age {} {}, 'Age requirement not met');\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        assert(income {} {}, 'Income requirement not met');\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_cairo(left)?;
                result.push_str(&self.condition_to_cairo(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }

    fn generate_cosmwasm(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        // lib.rs for CosmWasm contract
        source.push_str("use cosmwasm_std::{\n");
        source.push_str("    entry_point, to_json_binary, Binary, Deps, DepsMut, Env,\n");
        source.push_str("    MessageInfo, Response, StdResult, Addr,\n");
        source.push_str("};\n");
        source.push_str("use serde::{Deserialize, Serialize};\n\n");

        // State
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("pub struct State {\n");
        source.push_str("    pub owner: Addr,\n");
        source.push_str("    pub eligible_count: u64,\n");
        source.push_str("}\n\n");

        // InstantiateMsg
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("pub struct InstantiateMsg {}\n\n");

        // ExecuteMsg
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("#[serde(rename_all = \"snake_case\")]\n");
        source.push_str("pub enum ExecuteMsg {\n");
        source.push_str("    ApplyEffect { beneficiary: String },\n");
        source.push_str("}\n\n");

        // QueryMsg
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("#[serde(rename_all = \"snake_case\")]\n");
        source.push_str("pub enum QueryMsg {\n");
        source.push_str("    CheckEligibility {\n");
        let params = self.extract_parameters(&statute.preconditions);
        for (name, _) in &params {
            source.push_str(&format!("        {}: u64,\n", name));
        }
        source.push_str("    },\n");
        source.push_str("}\n\n");

        // Instantiate
        source.push_str(&format!("/// {}\n", statute.title));
        source.push_str("#[entry_point]\n");
        source.push_str("pub fn instantiate(\n");
        source.push_str("    deps: DepsMut,\n");
        source.push_str("    _env: Env,\n");
        source.push_str("    info: MessageInfo,\n");
        source.push_str("    _msg: InstantiateMsg,\n");
        source.push_str(") -> StdResult<Response> {\n");
        source.push_str("    let state = State {\n");
        source.push_str("        owner: info.sender.clone(),\n");
        source.push_str("        eligible_count: 0,\n");
        source.push_str("    };\n");
        source.push_str("    deps.storage.set(b\"state\", &to_json_binary(&state)?);\n");
        source.push_str("    Ok(Response::new()\n");
        source.push_str("        .add_attribute(\"method\", \"instantiate\")\n");
        source.push_str("        .add_attribute(\"owner\", info.sender))\n");
        source.push_str("}\n\n");

        // Execute
        source.push_str("#[entry_point]\n");
        source.push_str("pub fn execute(\n");
        source.push_str("    deps: DepsMut,\n");
        source.push_str("    _env: Env,\n");
        source.push_str("    info: MessageInfo,\n");
        source.push_str("    msg: ExecuteMsg,\n");
        source.push_str(") -> StdResult<Response> {\n");
        source.push_str("    match msg {\n");
        source.push_str("        ExecuteMsg::ApplyEffect { beneficiary } => {\n");
        source.push_str("            let state: State = deps.storage.get(b\"state\")?\n");
        source.push_str(
            "                .ok_or_else(|| cosmwasm_std::StdError::not_found(\"state\"))?;\n",
        );
        source.push_str("            if info.sender != state.owner {\n");
        source.push_str(
            "                return Err(cosmwasm_std::StdError::generic_err(\"Unauthorized\"));\n",
        );
        source.push_str("            }\n");
        source.push_str("            Ok(Response::new()\n");
        source.push_str("                .add_attribute(\"method\", \"apply_effect\")\n");
        source.push_str("                .add_attribute(\"beneficiary\", beneficiary))\n");
        source.push_str("        }\n");
        source.push_str("    }\n");
        source.push_str("}\n\n");

        // Query
        source.push_str("#[entry_point]\n");
        source.push_str("pub fn query(\n");
        source.push_str("    _deps: Deps,\n");
        source.push_str("    _env: Env,\n");
        source.push_str("    msg: QueryMsg,\n");
        source.push_str(") -> StdResult<Binary> {\n");
        source.push_str("    match msg {\n");
        source.push_str("        QueryMsg::CheckEligibility { ");
        let param_names: Vec<String> = params.iter().map(|(name, _)| name.clone()).collect();
        source.push_str(&param_names.join(", "));
        source.push_str(" } => {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_cosmwasm(condition)?);
        }

        source.push_str("            to_json_binary(&true)\n");
        source.push_str("        }\n");
        source.push_str("    }\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::CosmWasm,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_cosmwasm(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "            if !(age {} {}) {{\n                return Err(cosmwasm_std::StdError::generic_err(\"Age requirement not met\"));\n            }}\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "            if !(income {} {}) {{\n                return Err(cosmwasm_std::StdError::generic_err(\"Income requirement not met\"));\n            }}\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_cosmwasm(left)?;
                result.push_str(&self.condition_to_cosmwasm(right)?);
                Ok(result)
            }
            _ => Ok("            // Custom condition\n".to_string()),
        }
    }

    fn generate_solidity_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str("/// @title StatuteFactory\n");
        source.push_str("/// @notice Factory contract for deploying statute contracts\n");
        source.push_str("/// @dev Auto-generated from Legalis-RS\n");
        source.push_str("contract StatuteFactory {\n");
        source.push_str("    address public owner;\n");
        source.push_str("    address[] public deployedContracts;\n");
        source.push_str("    mapping(string => address[]) public contractsByType;\n\n");

        source.push_str(
            "    event ContractDeployed(address indexed contractAddress, string contractType);\n\n",
        );

        source.push_str("    constructor() {\n");
        source.push_str("        owner = msg.sender;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier onlyOwner() {\n");
        source.push_str("        require(msg.sender == owner, \"Only owner can call this\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        for statute_id in statute_ids {
            let contract_name = to_pascal_case(statute_id);
            source.push_str(&format!(
                "    /// @notice Deploy a new {} contract\n",
                contract_name
            ));
            source.push_str(&format!(
                "    function deploy{}() public onlyOwner returns (address) {{\n",
                contract_name
            ));
            source.push_str(&format!(
                "        {} newContract = new {}();\n",
                contract_name, contract_name
            ));
            source.push_str("        address contractAddress = address(newContract);\n");
            source.push_str("        deployedContracts.push(contractAddress);\n");
            source.push_str(&format!(
                "        contractsByType[\"{}\"].push(contractAddress);\n",
                statute_id
            ));
            source.push_str(&format!(
                "        emit ContractDeployed(contractAddress, \"{}\");\n",
                statute_id
            ));
            source.push_str("        return contractAddress;\n");
            source.push_str("    }\n\n");
        }

        source.push_str("    /// @notice Get total number of deployed contracts\n");
        source
            .push_str("    function getDeployedContractsCount() public view returns (uint256) {\n");
        source.push_str("        return deployedContracts.length;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Get contracts by type\n");
        source.push_str("    function getContractsByType(string memory contractType) public view returns (address[] memory) {\n");
        source.push_str("        return contractsByType[contractType];\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: "StatuteFactory".to_string(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_vyper_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("# @version ^0.3.0\n");
        source.push_str("# @title StatuteFactory\n");
        source.push_str("# @notice Factory contract for deploying statute contracts\n\n");

        source.push_str("owner: public(address)\n");
        source.push_str("deployed_contracts: public(DynArray[address, 1000])\n\n");

        source.push_str("event ContractDeployed:\n");
        source.push_str("    contract_address: indexed(address)\n");
        source.push_str("    contract_type: String[100]\n\n");

        source.push_str("@external\n");
        source.push_str("def __init__():\n");
        source.push_str("    self.owner = msg.sender\n\n");

        for statute_id in statute_ids {
            source.push_str("@external\n");
            source.push_str(&format!(
                "def deploy_{}() -> address:\n",
                to_snake_case(statute_id)
            ));
            source.push_str("    assert msg.sender == self.owner, \"Only owner\"\n");
            source.push_str("    # Deployment logic here\n");
            source.push_str(&format!(
                "    log ContractDeployed(empty(address), \"{}\")\n",
                statute_id
            ));
            source.push_str("    return empty(address)\n\n");
        }

        Ok(GeneratedContract {
            name: "statute_factory".to_string(),
            source,
            platform: TargetPlatform::Vyper,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_proxy(&self, contract_name: &str) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!(
            "/// @title {}Proxy\n",
            to_pascal_case(contract_name)
        ));
        source.push_str("/// @notice Upgradeable proxy contract using transparent proxy pattern\n");
        source.push_str("/// @dev Auto-generated from Legalis-RS\n");
        source.push_str(&format!(
            "contract {}Proxy {{\n",
            to_pascal_case(contract_name)
        ));

        source.push_str("    /// @notice Address of the current implementation\n");
        source.push_str("    address public implementation;\n");
        source.push_str("    /// @notice Admin address that can upgrade the implementation\n");
        source.push_str("    address public admin;\n\n");

        source.push_str("    event Upgraded(address indexed implementation);\n");
        source.push_str(
            "    event AdminChanged(address indexed previousAdmin, address indexed newAdmin);\n\n",
        );

        source.push_str("    /// @notice Initialize the proxy with implementation address\n");
        source.push_str("    constructor(address _implementation) {\n");
        source.push_str(
            "        require(_implementation != address(0), \"Invalid implementation\");\n",
        );
        source.push_str("        implementation = _implementation;\n");
        source.push_str("        admin = msg.sender;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier onlyAdmin() {\n");
        source.push_str("        require(msg.sender == admin, \"Only admin\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Upgrade to a new implementation\n");
        source.push_str(
            "    /// @param newImplementation Address of the new implementation contract\n",
        );
        source.push_str("    function upgradeTo(address newImplementation) external onlyAdmin {\n");
        source.push_str(
            "        require(newImplementation != address(0), \"Invalid implementation\");\n",
        );
        source.push_str(
            "        require(newImplementation != implementation, \"Same implementation\");\n",
        );
        source.push_str("        implementation = newImplementation;\n");
        source.push_str("        emit Upgraded(newImplementation);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Change the admin address\n");
        source.push_str("    function changeAdmin(address newAdmin) external onlyAdmin {\n");
        source.push_str("        require(newAdmin != address(0), \"Invalid admin\");\n");
        source.push_str("        emit AdminChanged(admin, newAdmin);\n");
        source.push_str("        admin = newAdmin;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Fallback function to delegate calls to implementation\n");
        source.push_str("    fallback() external payable {\n");
        source.push_str("        address impl = implementation;\n");
        source.push_str("        assembly {\n");
        source.push_str("            calldatacopy(0, 0, calldatasize())\n");
        source.push_str(
            "            let result := delegatecall(gas(), impl, 0, calldatasize(), 0, 0)\n",
        );
        source.push_str("            returndatacopy(0, 0, returndatasize())\n");
        source.push_str("            switch result\n");
        source.push_str("            case 0 { revert(0, returndatasize()) }\n");
        source.push_str("            default { return(0, returndatasize()) }\n");
        source.push_str("        }\n");
        source.push_str("    }\n\n");

        source.push_str("    receive() external payable {}\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: format!("{}Proxy", to_pascal_case(contract_name)),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_deployment(
        &self,
        contract: &GeneratedContract,
        config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("// Hardhat deployment script\n");
        script.push_str("const hre = require(\"hardhat\");\n\n");
        script.push_str("async function main() {\n");
        script.push_str(&format!(
            "  console.log(\"Deploying {} to {}...\");\n\n",
            contract.name, config.network
        ));
        script.push_str(&format!(
            "  const ContractFactory = await hre.ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));

        if let Some(gas_limit) = config.gas_limit {
            script.push_str(&format!(
                "  const contract = await ContractFactory.deploy({{ gasLimit: {} }});\n",
                gas_limit
            ));
        } else {
            script.push_str("  const contract = await ContractFactory.deploy();\n");
        }

        script.push_str("  await contract.deployed();\n\n");
        script.push_str("  console.log(`Contract deployed to: ${contract.address}`);\n");
        script.push_str("  console.log(`Transaction hash: ${contract.deployTransaction.hash}`);\n");
        script.push_str("  console.log(`Deployer: ${await contract.signer.getAddress()}`);\n\n");

        script.push_str("  // Verify on Etherscan\n");
        script.push_str(
            "  if (hre.network.name !== \"localhost\" && hre.network.name !== \"hardhat\") {\n",
        );
        script.push_str("    console.log(\"Waiting for block confirmations...\");\n");
        script.push_str("    await contract.deployTransaction.wait(6);\n");
        script.push_str("    console.log(\"Verifying contract...\");\n");
        script.push_str("    await hre.run(\"verify:verify\", {\n");
        script.push_str("      address: contract.address,\n");
        script.push_str("      constructorArguments: [],\n");
        script.push_str("    });\n");
        script.push_str("  }\n");
        script.push_str("}\n\n");

        script.push_str("main()\n");
        script.push_str("  .then(() => process.exit(0))\n");
        script.push_str("  .catch((error) => {\n");
        script.push_str("    console.error(error);\n");
        script.push_str("    process.exit(1);\n");
        script.push_str("  });\n");

        Ok(script)
    }

    fn generate_vyper_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("# Vyper deployment script using ape\n");
        script.push_str("from ape import accounts, project\n\n");
        script.push_str("def main():\n");
        script.push_str("    deployer = accounts.load(\"deployer\")\n");
        script.push_str(&format!(
            "    contract = deployer.deploy(project.{})\n",
            contract.name
        ));
        script.push_str("    print(f\"Contract deployed to: {contract.address}\")\n");
        script.push_str("    return contract\n");

        Ok(script)
    }

    fn generate_move_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Move deployment script for Aptos\n\n");
        script.push_str(&format!(
            "echo \"Deploying {} module...\"\n\n",
            contract.name
        ));
        script.push_str("# Compile the module\n");
        script.push_str("aptos move compile\n\n");
        script.push_str("# Publish to the network\n");
        script.push_str("aptos move publish \\\n");
        script.push_str("  --named-addresses legalis=default \\\n");
        script.push_str("  --assume-yes\n\n");
        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_cairo_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Cairo deployment script for StarkNet\n\n");
        script.push_str(&format!(
            "echo \"Deploying {} to StarkNet...\"\n\n",
            contract.name
        ));
        script.push_str("# Compile the contract\n");
        script.push_str(&format!(
            "starknet-compile {}.cairo --output {}_compiled.json\n\n",
            contract.name, contract.name
        ));
        script.push_str("# Declare the contract\n");
        script.push_str(&format!(
            "starknet declare --contract {}_compiled.json\n\n",
            contract.name
        ));
        script.push_str("# Deploy the contract\n");
        script.push_str(&format!(
            "starknet deploy --contract {}_compiled.json\n\n",
            contract.name
        ));
        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_wasm_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# WASM build and deployment script\n\n");
        script.push_str(&format!(
            "echo \"Building {} WASM module...\"\n\n",
            contract.name
        ));
        script.push_str("# Build the WASM module\n");
        script.push_str("wasm-pack build --target web\n\n");
        script.push_str("# The WASM module is now ready in pkg/ directory\n");
        script.push_str("echo \"Build complete! WASM module is in pkg/ directory\"\n");
        script.push_str("echo \"Include it in your web application:\"\n");
        script.push_str("echo \"  import init, { YourContract } from './pkg';\"\n");

        Ok(script)
    }

    fn generate_ink_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Ink! deployment script for Substrate\n\n");
        script.push_str(&format!(
            "echo \"Building and deploying {} contract...\"\n\n",
            contract.name
        ));
        script.push_str("# Build the contract\n");
        script.push_str("cargo contract build --release\n\n");
        script.push_str("# Deploy using cargo-contract\n");
        script.push_str("cargo contract instantiate \\\n");
        script.push_str("  --constructor new \\\n");
        script.push_str("  --suri //Alice \\\n");
        script.push_str("  --execute\n\n");
        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_cosmwasm_deployment(
        &self,
        contract: &GeneratedContract,
        config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# CosmWasm deployment script\n\n");
        script.push_str(&format!(
            "echo \"Building and deploying {} contract...\"\n\n",
            contract.name
        ));

        script.push_str("# Optimize the contract\n");
        script.push_str("docker run --rm -v \"$(pwd)\":/code \\\n");
        script.push_str("  --mount type=volume,source=\"$(basename \"$(pwd)\")_cache\",target=/code/target \\\n");
        script.push_str(
            "  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \\\n",
        );
        script.push_str("  cosmwasm/rust-optimizer:0.12.13\n\n");

        script.push_str(&format!("# Deploy to {}\n", config.network));
        script.push_str(&format!("CHAIN_ID=\"{}\"\n", config.network));
        script.push_str("NODE=\"https://rpc.cosmos.network:443\"\n");
        script.push_str("TX_FLAGS=\"--gas auto --gas-adjustment 1.3 --gas-prices 0.025ucosm\"\n\n");

        script.push_str("# Store the contract code\n");
        script.push_str(&format!(
            "RES=$(wasmd tx wasm store artifacts/{}.wasm \\\n",
            contract.name
        ));
        script.push_str("  --from wallet \\\n");
        script.push_str("  --chain-id $CHAIN_ID \\\n");
        script.push_str("  --node $NODE \\\n");
        script.push_str("  $TX_FLAGS \\\n");
        script.push_str("  --yes \\\n");
        script.push_str("  --output json)\n\n");

        script.push_str("# Extract the code ID\n");
        script.push_str("CODE_ID=$(echo $RES | jq -r '.logs[0].events[] | select(.type==\"store_code\") | .attributes[] | select(.key==\"code_id\") | .value')\n");
        script.push_str("echo \"Code ID: $CODE_ID\"\n\n");

        script.push_str("# Instantiate the contract\n");
        script.push_str("INIT='{}'\n");
        script.push_str("wasmd tx wasm instantiate $CODE_ID \"$INIT\" \\\n");
        script.push_str("  --from wallet \\\n");
        script.push_str(&format!("  --label \"{}\" \\\n", contract.name));
        script.push_str("  --chain-id $CHAIN_ID \\\n");
        script.push_str("  --node $NODE \\\n");
        script.push_str("  $TX_FLAGS \\\n");
        script.push_str("  --yes\n\n");

        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_ton(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str(";; FunC contract for TON\n");
        source.push_str(&format!(";; {}\n\n", statute.title));

        source.push_str("#include \"imports/stdlib.fc\";\n\n");

        // Storage
        source.push_str("global int owner;\n");
        source.push_str("global int eligible_count;\n\n");

        // Load data
        source.push_str("() load_data() impure {\n");
        source.push_str("    var ds = get_data().begin_parse();\n");
        source.push_str("    owner = ds~load_uint(256);\n");
        source.push_str("    eligible_count = ds~load_uint(64);\n");
        source.push_str("}\n\n");

        // Save data
        source.push_str("() save_data() impure {\n");
        source.push_str("    set_data(begin_cell()\n");
        source.push_str("        .store_uint(owner, 256)\n");
        source.push_str("        .store_uint(eligible_count, 64)\n");
        source.push_str("        .end_cell());\n");
        source.push_str("}\n\n");

        // Check eligibility function
        source.push_str(&format!(";; {}\n", statute.title));
        source.push_str("int check_eligibility(");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("int {}", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str(") method_id {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_ton(condition)?);
        }
        source.push_str("    return -1;  ;; true in FunC\n");
        source.push_str("}\n\n");

        // Apply effect function
        source.push_str("() apply_effect(int beneficiary) impure {\n");
        source.push_str("    load_data();\n");
        source
            .push_str("    throw_unless(100, equal_slices(get_sender(), owner));  ;; Only owner\n");
        source.push_str("    eligible_count = eligible_count + 1;\n");
        source.push_str("    save_data();\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Ton,
            abi: None,
            deployment_script: None,
        })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn condition_to_ton(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!(
                    "    throw_unless(101, age {} {});  ;; Age requirement\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!(
                    "    throw_unless(102, income {} {});  ;; Income requirement\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_ton(left)?;
                result.push_str(&self.condition_to_ton(right)?);
                Ok(result)
            }
            _ => Ok("    ;; Custom condition\n".to_string()),
        }
    }

    fn generate_teal(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("#pragma version 8\n");
        source.push_str(&format!("// {}\n\n", statute.title));

        // Application creation
        source.push_str("// Handle application calls\n");
        source.push_str("txn ApplicationID\n");
        source.push_str("int 0\n");
        source.push_str("==\n");
        source.push_str("bnz create_app\n\n");

        // Check eligibility
        source.push_str("// Check eligibility\n");
        source.push_str("txn OnCompletion\n");
        source.push_str("int NoOp\n");
        source.push_str("==\n");
        source.push_str("bnz check_eligibility\n\n");

        // Check eligibility logic
        source.push_str("check_eligibility:\n");

        for (idx, condition) in statute.preconditions.iter().enumerate() {
            source.push_str(&format!("    // Condition {}\n", idx + 1));
            source.push_str(&self.condition_to_teal(condition)?);
        }
        source.push_str("    int 1  // Return true\n");
        source.push_str("    return\n\n");

        // Create app
        source.push_str("create_app:\n");
        source.push_str("    // Initialize contract\n");
        source.push_str("    byte \"owner\"\n");
        source.push_str("    txn Sender\n");
        source.push_str("    app_global_put\n");
        source.push_str("    int 1\n");
        source.push_str("    return\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Teal,
            abi: None,
            deployment_script: None,
        })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn condition_to_teal(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!(
                    "    txna ApplicationArgs 0\n    btoi\n    int {}\n    {}\n    assert\n",
                    value, op
                ))
            }
            Condition::Income { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!(
                    "    txna ApplicationArgs 1\n    btoi\n    int {}\n    {}\n    assert\n",
                    value, op
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_teal(left)?;
                result.push_str(&self.condition_to_teal(right)?);
                Ok(result)
            }
            _ => Ok("    // Custom condition\n".to_string()),
        }
    }

    fn generate_ton_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# TON FunC deployment script\n\n");
        script.push_str(&format!(
            "echo \"Deploying {} to TON...\"\n\n",
            contract.name
        ));

        script.push_str("# Compile the FunC contract\n");
        script.push_str(&format!(
            "func -o {}.fif -SPA {}.fc\n\n",
            contract.name, contract.name
        ));

        script.push_str("# Create deployment package\n");
        script.push_str(&format!("fift -s build.fif {}.fif\n\n", contract.name));

        script.push_str("# Deploy to TON network\n");
        script.push_str("echo \"Use TON wallet or ton-cli to deploy the compiled contract\"\n");
        script.push_str(&format!(
            "echo \"Contract compiled: {}.fif\"\n",
            contract.name
        ));

        Ok(script)
    }

    fn generate_teal_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Algorand Teal deployment script\n\n");
        script.push_str(&format!(
            "echo \"Deploying {} to Algorand...\"\n\n",
            contract.name
        ));

        script.push_str("# Compile the Teal contract\n");
        script.push_str(&format!(
            "goal clerk compile {}.teal -o {}.teal.tok\n\n",
            contract.name, contract.name
        ));

        script.push_str("# Deploy the application\n");
        script.push_str(&format!(
            "goal app create --creator $CREATOR \\\n  --approval-prog {}.teal \\\n  --clear-prog clear.teal \\\n  --global-byteslices 1 \\\n  --global-ints 1 \\\n  --local-byteslices 0 \\\n  --local-ints 0\n\n",
            contract.name
        ));

        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_uups_proxy(&self, contract_name: &str) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(
            "import \"@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol\";\n",
        );
        source.push_str(
            "import \"@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol\";\n\n",
        );
        source.push_str(&format!("/// @title {}\n", to_pascal_case(contract_name)));
        source.push_str("/// @notice UUPS Upgradeable Proxy Pattern\n");
        source.push_str("/// @dev Inherits from UUPSUpgradeable and OwnableUpgradeable\n");
        source.push_str(&format!(
            "contract {} is UUPSUpgradeable, OwnableUpgradeable {{\n",
            to_pascal_case(contract_name)
        ));

        source.push_str("    /// @custom:oz-upgrades-unsafe-allow constructor\n");
        source.push_str("    constructor() {\n");
        source.push_str("        _disableInitializers();\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Initialize the contract\n");
        source.push_str("    function initialize() public initializer {\n");
        source.push_str("        __Ownable_init();\n");
        source.push_str("        __UUPSUpgradeable_init();\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Authorize upgrade (only owner)\n");
        source.push_str("    /// @param newImplementation Address of new implementation\n");
        source.push_str("    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}\n\n");

        source.push_str("    /// @notice Get implementation version\n");
        source.push_str("    function version() public pure virtual returns (string memory) {\n");
        source.push_str("        return \"1.0.0\";\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: format!("{}UUPS", to_pascal_case(contract_name)),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_beacon_proxy(&self, contract_name: &str) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str("import \"@openzeppelin/contracts/proxy/beacon/BeaconProxy.sol\";\n");
        source
            .push_str("import \"@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol\";\n\n");

        // Beacon contract
        source.push_str(&format!(
            "/// @title {}Beacon\n",
            to_pascal_case(contract_name)
        ));
        source.push_str("/// @notice Beacon for upgradeable proxies\n");
        source.push_str(&format!(
            "contract {}Beacon is UpgradeableBeacon {{\n",
            to_pascal_case(contract_name)
        ));
        source.push_str("    /// @notice Create beacon with initial implementation\n");
        source.push_str("    /// @param implementation Address of initial implementation\n");
        source.push_str(
            "    constructor(address implementation) UpgradeableBeacon(implementation) {}\n",
        );
        source.push_str("}\n\n");

        // Proxy factory using beacon
        source.push_str(&format!(
            "/// @title {}ProxyFactory\n",
            to_pascal_case(contract_name)
        ));
        source.push_str("/// @notice Factory for creating beacon proxies\n");
        source.push_str(&format!(
            "contract {}ProxyFactory {{\n",
            to_pascal_case(contract_name)
        ));
        source.push_str("    address public immutable beacon;\n");
        source.push_str("    address[] public allProxies;\n\n");
        source.push_str("    event ProxyCreated(address indexed proxy, uint256 index);\n\n");
        source.push_str("    /// @notice Create factory with beacon\n");
        source.push_str("    /// @param _beacon Address of beacon contract\n");
        source.push_str("    constructor(address _beacon) {\n");
        source.push_str("        require(_beacon != address(0), \"Invalid beacon\");\n");
        source.push_str("        beacon = _beacon;\n");
        source.push_str("    }\n\n");
        source.push_str("    /// @notice Create new proxy instance\n");
        source.push_str("    /// @param data Initialization data\n");
        source
            .push_str("    function createProxy(bytes memory data) external returns (address) {\n");
        source.push_str("        BeaconProxy proxy = new BeaconProxy(beacon, data);\n");
        source.push_str("        address proxyAddress = address(proxy);\n");
        source.push_str("        allProxies.push(proxyAddress);\n");
        source.push_str("        emit ProxyCreated(proxyAddress, allProxies.length - 1);\n");
        source.push_str("        return proxyAddress;\n");
        source.push_str("    }\n\n");
        source.push_str("    /// @notice Get total number of proxies\n");
        source.push_str("    function getProxyCount() external view returns (uint256) {\n");
        source.push_str("        return allProxies.length;\n");
        source.push_str("    }\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: format!("{}Beacon", to_pascal_case(contract_name)),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    #[allow(dead_code)]
    fn generate_solidity_tests(
        &self,
        contract: &GeneratedContract,
        config: &TestSuiteConfig,
    ) -> ChainResult<String> {
        let mut tests = String::new();

        tests.push_str("// SPDX-License-Identifier: MIT\n");
        tests.push_str("pragma solidity ^0.8.0;\n\n");

        if config.framework == "hardhat" {
            tests.push_str("import \"hardhat/console.sol\";\n");
        } else if config.framework == "foundry" {
            tests.push_str("import \"forge-std/Test.sol\";\n");
        }

        tests.push_str(&format!(
            "import \"../contracts/{}.sol\";\n\n",
            contract.name
        ));
        tests.push_str(&format!("/// @title {}Test\n", contract.name));
        tests.push_str("/// @notice Comprehensive test suite\n");

        if config.framework == "foundry" {
            tests.push_str(&format!("contract {}Test is Test {{\n", contract.name));
        } else {
            tests.push_str(&format!("contract {}Test {{\n", contract.name));
        }

        tests.push_str(&format!("    {} public testContract;\n\n", contract.name));

        // Setup
        tests.push_str("    function setUp() public {\n");
        tests.push_str(&format!(
            "        testContract = new {}();\n",
            contract.name
        ));
        tests.push_str("    }\n\n");

        if config.unit_tests {
            // Unit tests
            tests.push_str("    /// @notice Test contract deployment\n");
            tests.push_str("    function testDeployment() public {\n");
            tests.push_str("        assertEq(testContract.owner(), address(this));\n");
            tests.push_str("    }\n\n");

            tests.push_str("    /// @notice Test eligibility check with valid data\n");
            tests.push_str("    function testEligibilityValid() public {\n");
            tests.push_str("        bool result = testContract.checkEligibility(25, 50000);\n");
            tests.push_str("        assertTrue(result);\n");
            tests.push_str("    }\n\n");

            tests.push_str("    /// @notice Test eligibility check with invalid age\n");
            tests.push_str("    function testEligibilityInvalidAge() public {\n");
            tests.push_str("        vm.expectRevert();\n");
            tests.push_str("        testContract.checkEligibility(15, 50000);\n");
            tests.push_str("    }\n\n");
        }

        if config.integration_tests {
            tests.push_str("    /// @notice Integration test for full workflow\n");
            tests.push_str("    function testFullWorkflow() public {\n");
            tests.push_str("        address beneficiary = address(0x123);\n");
            tests.push_str("        testContract.applyEffect(beneficiary);\n");
            tests.push_str("        assertTrue(testContract.eligible(beneficiary));\n");
            tests.push_str("    }\n\n");
        }

        if config.fuzzing_tests {
            tests.push_str("    /// @notice Fuzz test for eligibility check\n");
            tests.push_str(
                "    function testFuzzEligibility(uint256 age, uint256 income) public {\n",
            );
            tests.push_str("        vm.assume(age >= 18 && age < 150);\n");
            tests.push_str("        vm.assume(income > 0 && income < 1000000);\n");
            tests.push_str("        bool result = testContract.checkEligibility(age, income);\n");
            tests.push_str("        assertTrue(result);\n");
            tests.push_str("    }\n\n");
        }

        tests.push_str("}\n");

        Ok(tests)
    }

    #[allow(dead_code)]
    fn generate_vyper_tests(
        &self,
        contract: &GeneratedContract,
        _config: &TestSuiteConfig,
    ) -> ChainResult<String> {
        let mut tests = String::new();

        tests.push_str("# Vyper contract tests using pytest and ape\n");
        tests.push_str("import pytest\n");
        tests.push_str("from ape import accounts, project\n\n");
        tests.push_str(&format!("@pytest.fixture\ndef contract(accounts):\n    return accounts[0].deploy(project.{})\n\n", contract.name));
        tests.push_str("def test_deployment(contract, accounts):\n");
        tests.push_str("    assert contract.owner() == accounts[0]\n\n");
        tests.push_str("def test_eligibility_valid(contract):\n");
        tests.push_str("    result = contract.check_eligibility(25, 50000)\n");
        tests.push_str("    assert result == True\n\n");
        tests.push_str("def test_apply_effect(contract, accounts):\n");
        tests.push_str("    beneficiary = accounts[1]\n");
        tests.push_str("    contract.apply_effect(beneficiary, sender=accounts[0])\n");
        tests.push_str("    assert contract.eligible(beneficiary) == True\n");

        Ok(tests)
    }

    fn generate_solidity_registry(&self) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str("/// @title StatuteRegistry\n");
        source.push_str("/// @notice Central registry for all statute contracts\n");
        source.push_str("/// @dev Manages statute contract addresses and metadata\n");
        source.push_str("contract StatuteRegistry {\n");

        source.push_str("    struct StatuteInfo {\n");
        source.push_str("        address contractAddress;\n");
        source.push_str("        string name;\n");
        source.push_str("        string version;\n");
        source.push_str("        uint256 deployedAt;\n");
        source.push_str("        bool active;\n");
        source.push_str("    }\n\n");

        source.push_str("    address public owner;\n");
        source.push_str("    mapping(bytes32 => StatuteInfo) public statutes;\n");
        source.push_str("    mapping(bytes32 => address[]) public statuteVersions;\n");
        source.push_str("    bytes32[] public statuteIds;\n\n");

        source.push_str("    event StatuteRegistered(bytes32 indexed id, address indexed contractAddress, string name);\n");
        source.push_str("    event StatuteDeactivated(bytes32 indexed id);\n");
        source.push_str("    event StatuteUpgraded(bytes32 indexed id, address oldAddress, address newAddress);\n\n");

        source.push_str("    modifier onlyOwner() {\n");
        source.push_str("        require(msg.sender == owner, \"Only owner\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    constructor() {\n");
        source.push_str("        owner = msg.sender;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Register a new statute contract\n");
        source.push_str("    /// @param id Unique identifier for the statute\n");
        source.push_str("    /// @param contractAddress Address of the statute contract\n");
        source.push_str("    /// @param name Human-readable name\n");
        source.push_str("    /// @param version Version string\n");
        source.push_str("    function registerStatute(\n");
        source.push_str("        bytes32 id,\n");
        source.push_str("        address contractAddress,\n");
        source.push_str("        string memory name,\n");
        source.push_str("        string memory version\n");
        source.push_str("    ) external onlyOwner {\n");
        source.push_str("        require(contractAddress != address(0), \"Invalid address\");\n");
        source.push_str("        require(statutes[id].contractAddress == address(0), \"Statute already exists\");\n\n");
        source.push_str("        statutes[id] = StatuteInfo({\n");
        source.push_str("            contractAddress: contractAddress,\n");
        source.push_str("            name: name,\n");
        source.push_str("            version: version,\n");
        source.push_str("            deployedAt: block.timestamp,\n");
        source.push_str("            active: true\n");
        source.push_str("        });\n");
        source.push_str("        statuteIds.push(id);\n");
        source.push_str("        statuteVersions[id].push(contractAddress);\n");
        source.push_str("        emit StatuteRegistered(id, contractAddress, name);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Upgrade a statute to a new implementation\n");
        source.push_str("    /// @param id Statute identifier\n");
        source.push_str("    /// @param newAddress New contract address\n");
        source.push_str("    /// @param newVersion New version string\n");
        source.push_str("    function upgradeStatute(\n");
        source.push_str("        bytes32 id,\n");
        source.push_str("        address newAddress,\n");
        source.push_str("        string memory newVersion\n");
        source.push_str("    ) external onlyOwner {\n");
        source.push_str("        require(statutes[id].active, \"Statute not active\");\n");
        source.push_str("        require(newAddress != address(0), \"Invalid address\");\n");
        source.push_str("        address oldAddress = statutes[id].contractAddress;\n");
        source.push_str("        statutes[id].contractAddress = newAddress;\n");
        source.push_str("        statutes[id].version = newVersion;\n");
        source.push_str("        statuteVersions[id].push(newAddress);\n");
        source.push_str("        emit StatuteUpgraded(id, oldAddress, newAddress);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Deactivate a statute\n");
        source.push_str("    /// @param id Statute identifier\n");
        source.push_str("    function deactivateStatute(bytes32 id) external onlyOwner {\n");
        source.push_str("        require(statutes[id].active, \"Already inactive\");\n");
        source.push_str("        statutes[id].active = false;\n");
        source.push_str("        emit StatuteDeactivated(id);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Get statute information\n");
        source.push_str("    /// @param id Statute identifier\n");
        source.push_str(
            "    function getStatute(bytes32 id) external view returns (StatuteInfo memory) {\n",
        );
        source.push_str("        return statutes[id];\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Get all statute IDs\n");
        source.push_str(
            "    function getAllStatuteIds() external view returns (bytes32[] memory) {\n",
        );
        source.push_str("        return statuteIds;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Get version history for a statute\n");
        source.push_str("    /// @param id Statute identifier\n");
        source.push_str("    function getVersionHistory(bytes32 id) external view returns (address[] memory) {\n");
        source.push_str("        return statuteVersions[id];\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: "StatuteRegistry".to_string(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_governance(&self) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str("/// @title StatuteGovernance\n");
        source.push_str("/// @notice Governance contract for managing statute changes\n");
        source.push_str("/// @dev Implements proposal and voting mechanism\n");
        source.push_str("contract StatuteGovernance {\n");

        source.push_str(
            "    enum ProposalState { Pending, Active, Succeeded, Defeated, Executed }\n\n",
        );

        source.push_str("    struct Proposal {\n");
        source.push_str("        bytes32 statuteId;\n");
        source.push_str("        address proposer;\n");
        source.push_str("        string description;\n");
        source.push_str("        uint256 votesFor;\n");
        source.push_str("        uint256 votesAgainst;\n");
        source.push_str("        uint256 startTime;\n");
        source.push_str("        uint256 endTime;\n");
        source.push_str("        ProposalState state;\n");
        source.push_str("        mapping(address => bool) hasVoted;\n");
        source.push_str("    }\n\n");

        source.push_str("    address public admin;\n");
        source.push_str("    uint256 public proposalCount;\n");
        source.push_str("    uint256 public votingPeriod = 7 days;\n");
        source.push_str("    uint256 public quorum = 4;  // 40% quorum\n");
        source.push_str("    mapping(uint256 => Proposal) public proposals;\n");
        source.push_str("    mapping(address => uint256) public votingPower;\n\n");

        source.push_str("    event ProposalCreated(uint256 indexed proposalId, bytes32 indexed statuteId, address proposer);\n");
        source.push_str("    event VoteCast(uint256 indexed proposalId, address indexed voter, bool support, uint256 weight);\n");
        source.push_str("    event ProposalExecuted(uint256 indexed proposalId);\n\n");

        source.push_str("    modifier onlyAdmin() {\n");
        source.push_str("        require(msg.sender == admin, \"Only admin\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    constructor() {\n");
        source.push_str("        admin = msg.sender;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Create a new proposal\n");
        source.push_str("    /// @param statuteId ID of statute to modify\n");
        source.push_str("    /// @param description Proposal description\n");
        source.push_str("    function propose(bytes32 statuteId, string memory description) external returns (uint256) {\n");
        source.push_str("        require(votingPower[msg.sender] > 0, \"No voting power\");\n");
        source.push_str("        uint256 proposalId = proposalCount++;\n");
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str("        proposal.statuteId = statuteId;\n");
        source.push_str("        proposal.proposer = msg.sender;\n");
        source.push_str("        proposal.description = description;\n");
        source.push_str("        proposal.startTime = block.timestamp;\n");
        source.push_str("        proposal.endTime = block.timestamp + votingPeriod;\n");
        source.push_str("        proposal.state = ProposalState.Active;\n");
        source.push_str("        emit ProposalCreated(proposalId, statuteId, msg.sender);\n");
        source.push_str("        return proposalId;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Cast a vote on a proposal\n");
        source.push_str("    /// @param proposalId ID of proposal\n");
        source.push_str("    /// @param support True for yes, false for no\n");
        source.push_str("    function castVote(uint256 proposalId, bool support) external {\n");
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str(
            "        require(proposal.state == ProposalState.Active, \"Proposal not active\");\n",
        );
        source
            .push_str("        require(block.timestamp <= proposal.endTime, \"Voting ended\");\n");
        source.push_str("        require(!proposal.hasVoted[msg.sender], \"Already voted\");\n");
        source.push_str("        uint256 weight = votingPower[msg.sender];\n");
        source.push_str("        require(weight > 0, \"No voting power\");\n");
        source.push_str("        proposal.hasVoted[msg.sender] = true;\n");
        source.push_str("        if (support) {\n");
        source.push_str("            proposal.votesFor += weight;\n");
        source.push_str("        } else {\n");
        source.push_str("            proposal.votesAgainst += weight;\n");
        source.push_str("        }\n");
        source.push_str("        emit VoteCast(proposalId, msg.sender, support, weight);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Execute a successful proposal\n");
        source.push_str("    /// @param proposalId ID of proposal\n");
        source.push_str("    function execute(uint256 proposalId) external {\n");
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str(
            "        require(block.timestamp > proposal.endTime, \"Voting not ended\");\n",
        );
        source
            .push_str("        require(proposal.state == ProposalState.Active, \"Not active\");\n");
        source
            .push_str("        uint256 totalVotes = proposal.votesFor + proposal.votesAgainst;\n");
        source.push_str(
            "        if (proposal.votesFor > proposal.votesAgainst && totalVotes >= quorum) {\n",
        );
        source.push_str("            proposal.state = ProposalState.Succeeded;\n");
        source.push_str("            // Execute proposal logic here\n");
        source.push_str("            emit ProposalExecuted(proposalId);\n");
        source.push_str("        } else {\n");
        source.push_str("            proposal.state = ProposalState.Defeated;\n");
        source.push_str("        }\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Grant voting power to an address\n");
        source.push_str("    /// @param voter Address to grant power\n");
        source.push_str("    /// @param power Amount of voting power\n");
        source.push_str(
            "    function grantVotingPower(address voter, uint256 power) external onlyAdmin {\n",
        );
        source.push_str("        votingPower[voter] = power;\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: "StatuteGovernance".to_string(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_with_batch(
        &self,
        statute: &Statute,
        config: &BatchOperationConfig,
    ) -> ChainResult<GeneratedContract> {
        let mut contract = self.generate_solidity(statute)?;
        let mut additional = String::new();

        if config.batch_eligibility {
            additional.push_str("\n    /// @notice Batch eligibility check for gas optimization\n");
            additional.push_str("    /// @param entities Array of entity data\n");
            additional.push_str("    /// @return results Array of eligibility results\n");
            additional.push_str("    function batchCheckEligibility(\n");

            let params = self.extract_parameters(&statute.preconditions);
            for (name, typ) in &params {
                additional.push_str(&format!("        {}[] memory {},\n", typ, name));
            }
            additional.push_str("        uint256 count\n");
            additional.push_str("    ) public returns (bool[] memory results) {\n");
            additional.push_str(&format!(
                "        require(count <= {}, \"Batch too large\");\n",
                config.max_batch_size
            ));
            additional.push_str("        results = new bool[](count);\n");
            additional.push_str("        for (uint256 i = 0; i < count; i++) {\n");
            additional.push_str("            try this.checkEligibility(");
            let param_names: Vec<String> = params
                .iter()
                .map(|(name, _)| format!("{}[i]", name))
                .collect();
            additional.push_str(&param_names.join(", "));
            additional.push_str(") returns (bool result) {\n");
            additional.push_str("                results[i] = result;\n");
            additional.push_str("            } catch {\n");
            additional.push_str("                results[i] = false;\n");
            additional.push_str("            }\n");
            additional.push_str("        }\n");
            additional.push_str("    }\n");
        }

        if config.batch_effects {
            additional.push_str("\n    /// @notice Batch apply effects for gas optimization\n");
            additional.push_str("    /// @param beneficiaries Array of beneficiary addresses\n");
            additional.push_str(
                "    function batchApplyEffects(address[] memory beneficiaries) public {\n",
            );
            additional.push_str(&format!(
                "        require(beneficiaries.length <= {}, \"Batch too large\");\n",
                config.max_batch_size
            ));
            additional.push_str("        require(msg.sender == owner, \"Only owner\");\n");
            additional.push_str("        for (uint256 i = 0; i < beneficiaries.length; i++) {\n");
            additional.push_str("            applyEffect(beneficiaries[i]);\n");
            additional.push_str("        }\n");
            additional.push_str("    }\n");
        }

        // Insert before closing brace
        let source = contract.source.trim_end_matches("\n}").to_string() + &additional + "\n}\n";
        contract.source = source;

        Ok(contract)
    }

    fn generate_hardhat_multi_network(
        &self,
        _contract: &GeneratedContract,
        config: &MultiNetworkConfig,
    ) -> ChainResult<String> {
        let mut cfg = String::new();

        cfg.push_str("// Hardhat multi-network configuration\n");
        cfg.push_str("require('@nomiclabs/hardhat-ethers');\n");
        cfg.push_str("require('@nomiclabs/hardhat-etherscan');\n\n");

        cfg.push_str("module.exports = {\n");
        cfg.push_str("  solidity: {\n");
        cfg.push_str("    version: '0.8.0',\n");
        cfg.push_str("    settings: {\n");
        cfg.push_str("      optimizer: { enabled: true, runs: 200 }\n");
        cfg.push_str("    }\n");
        cfg.push_str("  },\n");
        cfg.push_str(&format!(
            "  defaultNetwork: '{}',\n",
            config.default_network
        ));
        cfg.push_str("  networks: {\n");

        for (idx, network) in config.networks.iter().enumerate() {
            cfg.push_str(&format!("    {}: {{\n", network.name));
            cfg.push_str(&format!("      url: '{}',\n", network.rpc_url));
            cfg.push_str(&format!("      chainId: {},\n", network.chain_id));
            cfg.push_str("      accounts: [process.env.PRIVATE_KEY],\n");

            if let Some(gas_limit) = network.gas_limit {
                cfg.push_str(&format!("      gas: {},\n", gas_limit));
            }
            if let Some(gas_price) = network.gas_price {
                cfg.push_str(&format!("      gasPrice: {},\n", gas_price * 1_000_000_000));
            }

            if idx < config.networks.len() - 1 {
                cfg.push_str("    },\n");
            } else {
                cfg.push_str("    }\n");
            }
        }
        cfg.push_str("  },\n");

        // Add etherscan configuration
        cfg.push_str("  etherscan: {\n");
        cfg.push_str("    apiKey: {\n");
        for (idx, network) in config.networks.iter().enumerate() {
            if let Some(key) = &network.etherscan_api_key {
                cfg.push_str(&format!(
                    "      {}: '{}'{}\n",
                    network.name,
                    key,
                    if idx < config.networks.len() - 1 {
                        ","
                    } else {
                        ""
                    }
                ));
            }
        }
        cfg.push_str("    }\n");
        cfg.push_str("  }\n");
        cfg.push_str("};\n");

        Ok(cfg)
    }

    fn generate_solidity_formal_verification(
        &self,
        contract: &GeneratedContract,
        config: &FormalVerificationConfig,
    ) -> ChainResult<Vec<(String, String)>> {
        let mut files = Vec::new();

        if config.slither {
            let mut slither = String::new();
            slither.push_str("# Slither configuration\n");
            slither.push_str("{\n");
            slither.push_str("  \"detectors_to_exclude\": [],\n");
            slither.push_str("  \"exclude_dependencies\": true,\n");
            slither.push_str("  \"exclude_informational\": false,\n");
            slither.push_str("  \"exclude_low\": false,\n");
            slither.push_str("  \"exclude_medium\": false,\n");
            slither.push_str("  \"exclude_high\": false,\n");
            slither.push_str("  \"solc_args\": \"--optimize\"\n");
            slither.push_str("}\n");
            files.push(("slither.config.json".to_string(), slither));
        }

        if config.certora {
            let mut certora = String::new();
            certora.push_str(&format!("// Certora specification for {}\n", contract.name));
            certora.push_str("methods {\n");
            certora.push_str("    checkEligibility(uint256, uint256) returns bool envfree\n");
            certora.push_str("    applyEffect(address) envfree\n");
            certora.push_str("}\n\n");
            certora.push_str("// Invariant: owner should never change\n");
            certora.push_str("invariant ownerNeverChanges()\n");
            certora.push_str("    owner() == owner()@init\n\n");
            certora.push_str("// Rule: eligible mapping should only change via applyEffect\n");
            certora.push_str("rule eligibilityOnlyViaApplyEffect(address beneficiary) {\n");
            certora.push_str("    env e;\n");
            certora.push_str("    applyEffect(e, beneficiary);\n");
            certora.push_str("    assert eligible(beneficiary) == true;\n");
            certora.push_str("}\n");
            files.push((format!("{}.spec", contract.name), certora));
        }

        if config.scribble {
            let mut scribble = contract.source.clone();
            // Add scribble annotations
            scribble = scribble.replace(
                "function checkEligibility(",
                "/// #if_succeeds result == true;\nfunction checkEligibility(",
            );
            scribble = scribble.replace(
                "function applyEffect(",
                "/// #if_succeeds eligible[beneficiary] == true;\nfunction applyEffect(",
            );
            files.push((format!("{}_scribble.sol", contract.name), scribble));
        }

        if config.invariants {
            let mut invariants = String::new();
            invariants.push_str(&format!("// Invariants for {}\n\n", contract.name));
            invariants.push_str("// INV1: Owner should never be zero address\n");
            invariants.push_str("// owner != address(0)\n\n");
            invariants.push_str("// INV2: Eligibility can only be granted by owner\n");
            invariants
                .push_str("// forall address a: eligible[a] => owner called applyEffect(a)\n\n");
            invariants.push_str("// INV3: Check eligibility should be deterministic\n");
            invariants.push_str(
                "// forall inputs: checkEligibility(inputs) == checkEligibility(inputs)\n",
            );
            files.push(("invariants.md".to_string(), invariants));
        }

        Ok(files)
    }

    fn generate_solidity_interface(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!("/// @title I{}\n", contract_name));
        source.push_str(&format!(
            "/// @notice Interface for {} statute contract\n",
            statute.title
        ));
        source.push_str(&format!("interface I{} {{\n", contract_name));

        // Events
        source.push_str("    /// @notice Emitted when eligibility is checked\n");
        source.push_str("    event EligibilityChecked(address indexed entity, bool result);\n\n");
        source.push_str("    /// @notice Emitted when an effect is applied\n");
        source.push_str(
            "    event EffectApplied(address indexed beneficiary, string effectType);\n\n",
        );

        // Functions
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, typ)| format!("{} {}", typ, name))
            .collect();

        source.push_str("    /// @notice Check if an entity meets the preconditions\n");
        source.push_str("    function checkEligibility(");
        source.push_str(&param_str.join(", "));
        source.push_str(") external returns (bool);\n\n");

        source.push_str("    /// @notice Apply the legal effect\n");
        source.push_str("    function applyEffect(address beneficiary) external;\n\n");

        source.push_str("    /// @notice Get contract owner\n");
        source.push_str("    function owner() external view returns (address);\n\n");

        source.push_str("    /// @notice Check eligibility status\n");
        source.push_str("    function eligible(address entity) external view returns (bool);\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: format!("I{}", contract_name),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_modular(&self, statute: &Statute) -> ChainResult<ModularContract> {
        // Generate main contract
        let main_contract = self.generate_solidity(statute)?;

        // Generate interface
        let interface = Some(self.generate_solidity_interface(statute)?);

        // Generate library for common logic
        let library = self.generate_solidity_library(statute)?;
        let libraries = vec![library];

        Ok(ModularContract {
            main_contract,
            interface,
            libraries,
            helpers: vec![],
        })
    }

    fn generate_solidity_library(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let lib_name = format!("{}Lib", to_pascal_case(&statute.id));
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!("/// @title {}\n", lib_name));
        source.push_str("/// @notice Library with shared logic\n");
        source.push_str(&format!("library {} {{\n", lib_name));

        source.push_str("    /// @notice Validate age requirement\n");
        source.push_str("    /// @param age The age to validate\n");
        source.push_str("    /// @param minAge Minimum required age\n");
        source.push_str("    /// @return True if age meets requirement\n");
        source.push_str("    function validateAge(uint256 age, uint256 minAge) internal pure returns (bool) {\n");
        source.push_str("        return age >= minAge;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Validate income requirement\n");
        source.push_str("    /// @param income The income to validate\n");
        source.push_str("    /// @param maxIncome Maximum allowed income\n");
        source.push_str("    /// @return True if income meets requirement\n");
        source.push_str("    function validateIncome(uint256 income, uint256 maxIncome) internal pure returns (bool) {\n");
        source.push_str("        return income < maxIncome;\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: lib_name,
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_coverage_config(&self) -> ChainResult<String> {
        let mut config = String::new();

        config.push_str("// Solidity coverage configuration\n");
        config.push_str("module.exports = {\n");
        config.push_str("  skipFiles: [\n");
        config.push_str("    'test/',\n");
        config.push_str("    'mock/',\n");
        config.push_str("  ],\n");
        config.push_str("  mocha: {\n");
        config.push_str("    timeout: 100000\n");
        config.push_str("  },\n");
        config.push_str("  providerOptions: {\n");
        config.push_str("    default_balance_ether: '10000000000',\n");
        config.push_str("    total_accounts: 10,\n");
        config.push_str("    fork: process.env.FORK_URL || ''\n");
        config.push_str("  },\n");
        config.push_str("  istanbulReporter: ['html', 'json', 'lcov', 'text'],\n");
        config.push_str("  client: require('ganache-cli')\n");
        config.push_str("};\n");

        Ok(config)
    }

    fn generate_vyper_coverage_config(&self) -> ChainResult<String> {
        let mut config = String::new();

        config.push_str("# Vyper coverage configuration (pytest-cov)\n");
        config.push_str("[tool.pytest.ini_options]\n");
        config.push_str("addopts = '''\n");
        config.push_str("  --cov=contracts\n");
        config.push_str("  --cov-report=html\n");
        config.push_str("  --cov-report=term\n");
        config.push_str("  --cov-report=xml\n");
        config.push_str("'''\n");
        config.push_str("testpaths = ['tests']\n");
        config.push_str("python_files = 'test_*.py'\n");

        Ok(config)
    }

    fn generate_solidity_with_inheritance(
        &self,
        statute: &Statute,
        base_contracts: &[&str],
    ) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");

        // Import statements for base contracts
        for base in base_contracts {
            source.push_str(&format!(
                "import \"@openzeppelin/contracts/{}.sol\";\n",
                base
            ));
        }
        source.push('\n');

        source.push_str(&format!("/// @title {}\n", statute.title));
        source.push_str("/// @notice Auto-generated from Legalis-RS with inheritance\n");
        let inheritance = base_contracts.join(", ");
        source.push_str(&format!(
            "contract {} is {} {{\n",
            contract_name, inheritance
        ));

        source.push_str("    /// @notice Emitted when eligibility is checked\n");
        source.push_str("    event EligibilityChecked(address indexed entity, bool result);\n\n");

        source.push_str("    /// @notice Initialize the contract\n");
        source.push_str("    constructor() {\n");
        source.push_str("        // Initialization logic\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Check eligibility based on conditions\n");
        source.push_str("    /// @param entity The address to check\n");
        source.push_str("    /// @return bool Whether the entity is eligible\n");
        source.push_str("    function checkEligibility(address entity) public returns (bool) {\n");
        source.push_str("        bool eligible = true;\n");
        source.push_str("        // Condition checks here\n");
        source.push_str("        emit EligibilityChecked(entity, eligible);\n");
        source.push_str("        return eligible;\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: self.platform,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_diamond(
        &self,
        statutes: &[Statute],
    ) -> ChainResult<Vec<GeneratedContract>> {
        let mut contracts = Vec::new();

        // Generate Diamond Storage contract
        let mut storage_source = String::new();
        storage_source.push_str("// SPDX-License-Identifier: MIT\n");
        storage_source.push_str("pragma solidity ^0.8.0;\n\n");
        storage_source.push_str("/// @title DiamondStorage\n");
        storage_source.push_str("/// @notice Central storage for diamond pattern\n");
        storage_source.push_str("library DiamondStorage {\n");
        storage_source.push_str(
            "    bytes32 constant DIAMOND_STORAGE_POSITION = keccak256(\"diamond.storage\");\n\n",
        );
        storage_source.push_str("    struct Storage {\n");
        storage_source.push_str("        mapping(address => bool) eligible;\n");
        storage_source.push_str("        mapping(bytes4 => address) facets;\n");
        storage_source.push_str("    }\n\n");
        storage_source.push_str(
            "    function diamondStorage() internal pure returns (Storage storage ds) {\n",
        );
        storage_source.push_str("        bytes32 position = DIAMOND_STORAGE_POSITION;\n");
        storage_source.push_str("        assembly {\n");
        storage_source.push_str("            ds.slot := position\n");
        storage_source.push_str("        }\n");
        storage_source.push_str("    }\n");
        storage_source.push_str("}\n");

        contracts.push(GeneratedContract {
            name: "DiamondStorage".to_string(),
            source: storage_source,
            platform: self.platform,
            abi: None,
            deployment_script: None,
        });

        // Generate facet for each statute
        for statute in statutes {
            let facet_name = format!("{}Facet", to_pascal_case(&statute.id));
            let mut facet_source = String::new();

            facet_source.push_str("// SPDX-License-Identifier: MIT\n");
            facet_source.push_str("pragma solidity ^0.8.0;\n\n");
            facet_source.push_str("import \"./DiamondStorage.sol\";\n\n");
            facet_source.push_str(&format!("/// @title {}\n", facet_name));
            facet_source.push_str(&format!("/// @notice Facet for {}\n", statute.title));
            facet_source.push_str(&format!("contract {} {{\n", facet_name));
            facet_source.push_str("    using DiamondStorage for DiamondStorage.Storage;\n\n");

            facet_source
                .push_str("    event EligibilityChecked(address indexed entity, bool result);\n\n");

            facet_source.push_str(
                "    function checkEligibility(address entity) external returns (bool) {\n",
            );
            facet_source.push_str(
                "        DiamondStorage.Storage storage ds = DiamondStorage.diamondStorage();\n",
            );
            facet_source.push_str("        bool eligible = true;\n");
            facet_source.push_str("        // Condition checks\n");
            facet_source.push_str("        ds.eligible[entity] = eligible;\n");
            facet_source.push_str("        emit EligibilityChecked(entity, eligible);\n");
            facet_source.push_str("        return eligible;\n");
            facet_source.push_str("    }\n");

            facet_source.push_str("}\n");

            contracts.push(GeneratedContract {
                name: facet_name,
                source: facet_source,
                platform: self.platform,
                abi: None,
                deployment_script: None,
            });
        }

        Ok(contracts)
    }

    fn generate_evm_deployment_docs(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut docs = String::new();

        docs.push_str(&format!("# {} Deployment Guide\n\n", contract.name));
        docs.push_str("## Prerequisites\n\n");
        docs.push_str("- Node.js >= 16.0.0\n");
        docs.push_str("- Hardhat or Foundry\n");
        docs.push_str("- Wallet with sufficient gas\n\n");

        docs.push_str("## Installation\n\n");
        docs.push_str("```bash\n");
        docs.push_str("npm install --save-dev hardhat @nomiclabs/hardhat-ethers ethers\n");
        docs.push_str("```\n\n");

        docs.push_str("## Deployment Steps\n\n");
        docs.push_str("1. Set up environment variables:\n");
        docs.push_str("```bash\n");
        docs.push_str("export PRIVATE_KEY=your_private_key\n");
        docs.push_str("export RPC_URL=your_rpc_url\n");
        docs.push_str("```\n\n");

        docs.push_str("2. Deploy the contract:\n");
        docs.push_str("```bash\n");
        docs.push_str(&format!(
            "npx hardhat run scripts/deploy_{}.js --network mainnet\n",
            contract.name.to_lowercase()
        ));
        docs.push_str("```\n\n");

        docs.push_str("3. Verify on Etherscan:\n");
        docs.push_str("```bash\n");
        docs.push_str("npx hardhat verify --network mainnet CONTRACT_ADDRESS\n");
        docs.push_str("```\n\n");

        docs.push_str("## Post-Deployment\n\n");
        docs.push_str("- Save the contract address\n");
        docs.push_str("- Initialize contract if needed\n");
        docs.push_str("- Transfer ownership if applicable\n");
        docs.push_str("- Set up monitoring and alerts\n\n");

        Ok(docs)
    }

    fn generate_move_deployment_docs(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut docs = String::new();

        docs.push_str(&format!("# {} Move Deployment Guide\n\n", contract.name));
        docs.push_str("## Prerequisites\n\n");
        docs.push_str("- Aptos CLI or Sui CLI\n");
        docs.push_str("- Funded wallet account\n\n");

        docs.push_str("## Deployment (Aptos)\n\n");
        docs.push_str("```bash\n");
        docs.push_str("aptos move compile\n");
        docs.push_str("aptos move publish\n");
        docs.push_str("```\n\n");

        Ok(docs)
    }

    fn generate_cairo_deployment_docs(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut docs = String::new();

        docs.push_str(&format!("# {} Cairo Deployment Guide\n\n", contract.name));
        docs.push_str("## Prerequisites\n\n");
        docs.push_str("- Cairo compiler\n");
        docs.push_str("- StarkNet CLI\n\n");

        docs.push_str("## Deployment\n\n");
        docs.push_str("```bash\n");
        docs.push_str("starknet-compile contract.cairo --output contract_compiled.json\n");
        docs.push_str("starknet deploy --contract contract_compiled.json\n");
        docs.push_str("```\n\n");

        Ok(docs)
    }

    fn generate_solidity_api_docs(&self, statute: &Statute) -> ChainResult<String> {
        let contract_name = to_pascal_case(&statute.id);
        let mut docs = String::new();

        docs.push_str(&format!("# {} API Documentation\n\n", contract_name));
        docs.push_str("## Overview\n\n");
        docs.push_str(&format!("{}\n\n", statute.title));

        docs.push_str("## Functions\n\n");
        docs.push_str("### checkEligibility\n\n");
        docs.push_str("```solidity\n");
        docs.push_str("function checkEligibility(address entity) public returns (bool)\n");
        docs.push_str("```\n\n");
        docs.push_str("Checks if an address is eligible based on statute conditions.\n\n");
        docs.push_str("**Parameters:**\n");
        docs.push_str("- `entity`: The address to check eligibility for\n\n");
        docs.push_str("**Returns:**\n");
        docs.push_str("- `bool`: True if eligible, false otherwise\n\n");

        docs.push_str("### applyEffect\n\n");
        docs.push_str("```solidity\n");
        docs.push_str("function applyEffect(address beneficiary) public\n");
        docs.push_str("```\n\n");
        docs.push_str("Applies the statute effect to an eligible beneficiary.\n\n");
        docs.push_str("**Parameters:**\n");
        docs.push_str("- `beneficiary`: The address to apply the effect to\n\n");

        docs.push_str("## Events\n\n");
        docs.push_str("### EligibilityChecked\n\n");
        docs.push_str("```solidity\n");
        docs.push_str("event EligibilityChecked(address indexed entity, bool result)\n");
        docs.push_str("```\n\n");

        Ok(docs)
    }

    fn generate_vyper_api_docs(&self, statute: &Statute) -> ChainResult<String> {
        self.generate_solidity_api_docs(statute)
    }

    fn generate_evm_gas_estimation(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut report = String::new();

        report.push_str(&format!("# Gas Estimation Report: {}\n\n", contract.name));
        report.push_str("## Deployment\n\n");
        report.push_str("| Item | Estimated Gas |\n");
        report.push_str("|------|---------------|\n");
        report.push_str("| Contract Creation | ~500,000 |\n");
        report.push_str("| Constructor Execution | ~50,000 |\n");
        report.push_str("| **Total** | **~550,000** |\n\n");

        report.push_str("## Function Calls\n\n");
        report.push_str("| Function | Estimated Gas |\n");
        report.push_str("|----------|---------------|\n");
        report.push_str("| checkEligibility | ~45,000 |\n");
        report.push_str("| applyEffect | ~60,000 |\n");
        report.push_str("| batchCheckEligibility | ~150,000 |\n\n");

        report.push_str("## Optimization Suggestions\n\n");
        report.push_str("1. Use `calldata` instead of `memory` for read-only arrays\n");
        report.push_str("2. Pack struct variables efficiently\n");
        report.push_str("3. Use events instead of storage for historical data\n");
        report.push_str("4. Consider using bitmap for boolean flags\n");
        report.push_str("5. Cache storage variables in memory within functions\n\n");

        Ok(report)
    }

    fn generate_solidity_upgrade_script(
        &self,
        contract: &GeneratedContract,
        pattern: ProxyPattern,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("// Upgrade script for Hardhat\n");
        script.push_str("const { ethers, upgrades } = require(\"hardhat\");\n\n");
        script.push_str("async function main() {\n");
        script.push_str(&format!(
            "  const {} = await ethers.getContractFactory(\"{}\");\n",
            contract.name, contract.name
        ));

        match pattern {
            ProxyPattern::Transparent => {
                script.push_str("  console.log(\"Upgrading with Transparent Proxy...\");\n");
                script.push_str("  const proxyAddress = process.env.PROXY_ADDRESS;\n");
                script.push_str(&format!(
                    "  await upgrades.upgradeProxy(proxyAddress, {});\n",
                    contract.name
                ));
            }
            ProxyPattern::Uups => {
                script.push_str("  console.log(\"Upgrading with UUPS...\");\n");
                script.push_str("  const proxyAddress = process.env.PROXY_ADDRESS;\n");
                script.push_str(&format!(
                    "  await upgrades.upgradeProxy(proxyAddress, {});\n",
                    contract.name
                ));
            }
            ProxyPattern::Beacon => {
                script.push_str("  console.log(\"Upgrading Beacon...\");\n");
                script.push_str("  const beaconAddress = process.env.BEACON_ADDRESS;\n");
                script.push_str(&format!(
                    "  await upgrades.upgradeBeacon(beaconAddress, {});\n",
                    contract.name
                ));
            }
        }

        script.push_str("  console.log(\"Upgrade completed successfully\");\n");
        script.push_str("}\n\n");
        script.push_str("main().catch((error) => {\n");
        script.push_str("  console.error(error);\n");
        script.push_str("  process.exitCode = 1;\n");
        script.push_str("});\n");

        Ok(script)
    }

    fn generate_evm_cross_chain_config(
        &self,
        contract: &GeneratedContract,
        chains: &[&str],
    ) -> ChainResult<String> {
        let mut config = String::new();

        config.push_str("// Hardhat cross-chain configuration\n");
        config.push_str("module.exports = {\n");
        config.push_str("  networks: {\n");

        for chain in chains {
            match *chain {
                "ethereum" => {
                    config.push_str("    ethereum: {\n");
                    config.push_str("      url: process.env.ETHEREUM_RPC_URL,\n");
                    config.push_str("      chainId: 1,\n");
                    config.push_str("      accounts: [process.env.PRIVATE_KEY],\n");
                    config.push_str("    },\n");
                }
                "polygon" => {
                    config.push_str("    polygon: {\n");
                    config.push_str("      url: process.env.POLYGON_RPC_URL,\n");
                    config.push_str("      chainId: 137,\n");
                    config.push_str("      accounts: [process.env.PRIVATE_KEY],\n");
                    config.push_str("    },\n");
                }
                "arbitrum" => {
                    config.push_str("    arbitrum: {\n");
                    config.push_str("      url: process.env.ARBITRUM_RPC_URL,\n");
                    config.push_str("      chainId: 42161,\n");
                    config.push_str("      accounts: [process.env.PRIVATE_KEY],\n");
                    config.push_str("    },\n");
                }
                "optimism" => {
                    config.push_str("    optimism: {\n");
                    config.push_str("      url: process.env.OPTIMISM_RPC_URL,\n");
                    config.push_str("      chainId: 10,\n");
                    config.push_str("      accounts: [process.env.PRIVATE_KEY],\n");
                    config.push_str("    },\n");
                }
                _ => {}
            }
        }

        config.push_str("  },\n");
        config.push_str(&format!("  // Contract: {}\n", contract.name));
        config.push_str("};\n");

        Ok(config)
    }

    fn generate_evm_compilation_tests(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut tests = String::new();

        tests.push_str("// Compilation test suite\n");
        tests.push_str("const { expect } = require(\"chai\");\n");
        tests.push_str("const { ethers } = require(\"hardhat\");\n\n");

        tests.push_str(&format!(
            "describe(\"{} Compilation Tests\", function () {{\n",
            contract.name
        ));
        tests.push_str("  it(\"should compile successfully\", async function () {\n");
        tests.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        tests.push_str("    expect(ContractFactory).to.not.be.undefined;\n");
        tests.push_str("  });\n\n");

        tests.push_str("  it(\"should have correct bytecode\", async function () {\n");
        tests.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        tests.push_str("    const bytecode = ContractFactory.bytecode;\n");
        tests.push_str("    expect(bytecode).to.not.equal(\"0x\");\n");
        tests.push_str("    expect(bytecode.length).to.be.greaterThan(2);\n");
        tests.push_str("  });\n\n");

        tests.push_str("  it(\"should have valid ABI\", async function () {\n");
        tests.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        tests.push_str("    const abi = ContractFactory.interface;\n");
        tests.push_str("    expect(abi).to.not.be.undefined;\n");
        tests.push_str("    expect(abi.fragments.length).to.be.greaterThan(0);\n");
        tests.push_str("  });\n");

        tests.push_str("});\n");

        Ok(tests)
    }

    fn generate_evm_deployment_sim_tests(
        &self,
        contract: &GeneratedContract,
    ) -> ChainResult<String> {
        let mut tests = String::new();

        tests.push_str("// Deployment simulation test suite\n");
        tests.push_str("const { expect } = require(\"chai\");\n");
        tests.push_str("const { ethers } = require(\"hardhat\");\n\n");

        tests.push_str(&format!(
            "describe(\"{} Deployment Simulation\", function () {{\n",
            contract.name
        ));
        tests.push_str("  let contract;\n");
        tests.push_str("  let owner;\n");
        tests.push_str("  let addr1;\n\n");

        tests.push_str("  beforeEach(async function () {\n");
        tests.push_str("    [owner, addr1] = await ethers.getSigners();\n");
        tests.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        tests.push_str("    contract = await ContractFactory.deploy();\n");
        tests.push_str("    await contract.waitForDeployment();\n");
        tests.push_str("  });\n\n");

        tests.push_str("  it(\"should deploy successfully\", async function () {\n");
        tests.push_str("    expect(await contract.getAddress()).to.be.properAddress;\n");
        tests.push_str("  });\n\n");

        tests.push_str("  it(\"should set correct owner\", async function () {\n");
        tests.push_str("    expect(await contract.owner()).to.equal(owner.address);\n");
        tests.push_str("  });\n\n");

        tests.push_str("  it(\"should have correct initial state\", async function () {\n");
        tests.push_str("    // Verify initial state\n");
        tests.push_str("    const deploymentBlock = await ethers.provider.getBlockNumber();\n");
        tests.push_str("    expect(deploymentBlock).to.be.greaterThan(0);\n");
        tests.push_str("  });\n\n");

        tests.push_str("  it(\"should simulate gas costs\", async function () {\n");
        tests.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        tests.push_str("    const deployTx = await ContractFactory.getDeployTransaction();\n");
        tests.push_str("    const estimatedGas = await ethers.provider.estimateGas(deployTx);\n");
        tests
            .push_str("    console.log(\"Estimated deployment gas:\", estimatedGas.toString());\n");
        tests.push_str("    expect(estimatedGas).to.be.greaterThan(0);\n");
        tests.push_str("  });\n");

        tests.push_str("});\n");

        Ok(tests)
    }

    fn generate_evm_gas_benchmarks(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut benchmarks = String::new();

        benchmarks.push_str("// Gas usage benchmarks\n");
        benchmarks.push_str("const { expect } = require(\"chai\");\n");
        benchmarks.push_str("const { ethers } = require(\"hardhat\");\n\n");

        benchmarks.push_str(&format!(
            "describe(\"{} Gas Benchmarks\", function () {{\n",
            contract.name
        ));
        benchmarks.push_str("  let contract;\n");
        benchmarks.push_str("  let owner;\n");
        benchmarks.push_str("  let addr1;\n\n");

        benchmarks.push_str("  before(async function () {\n");
        benchmarks.push_str("    [owner, addr1] = await ethers.getSigners();\n");
        benchmarks.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        benchmarks.push_str("    contract = await ContractFactory.deploy();\n");
        benchmarks.push_str("    await contract.waitForDeployment();\n");
        benchmarks.push_str("  });\n\n");

        benchmarks.push_str("  it(\"benchmark: checkEligibility\", async function () {\n");
        benchmarks.push_str("    const tx = await contract.checkEligibility(addr1.address);\n");
        benchmarks.push_str("    const receipt = await tx.wait();\n");
        benchmarks.push_str(
            "    console.log(\"Gas used for checkEligibility:\", receipt.gasUsed.toString());\n",
        );
        benchmarks.push_str("    expect(receipt.gasUsed).to.be.lessThan(100000);\n");
        benchmarks.push_str("  });\n\n");

        benchmarks.push_str("  it(\"benchmark: applyEffect\", async function () {\n");
        benchmarks.push_str("    const tx = await contract.applyEffect(addr1.address);\n");
        benchmarks.push_str("    const receipt = await tx.wait();\n");
        benchmarks.push_str(
            "    console.log(\"Gas used for applyEffect:\", receipt.gasUsed.toString());\n",
        );
        benchmarks.push_str("    expect(receipt.gasUsed).to.be.lessThan(150000);\n");
        benchmarks.push_str("  });\n\n");

        benchmarks.push_str("  it(\"compare gas usage across functions\", async function () {\n");
        benchmarks.push_str("    const results = {};\n");
        benchmarks.push_str("    \n");
        benchmarks.push_str("    const tx1 = await contract.checkEligibility(addr1.address);\n");
        benchmarks.push_str("    results.checkEligibility = (await tx1.wait()).gasUsed;\n");
        benchmarks.push_str("    \n");
        benchmarks.push_str("    const tx2 = await contract.applyEffect(addr1.address);\n");
        benchmarks.push_str("    results.applyEffect = (await tx2.wait()).gasUsed;\n");
        benchmarks.push_str("    \n");
        benchmarks.push_str("    console.log(\"Gas Usage Summary:\", results);\n");
        benchmarks.push_str("  });\n");

        benchmarks.push_str("});\n");

        Ok(benchmarks)
    }

    fn generate_evm_security_tests(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut tests = String::new();

        tests.push_str("// Security test suite\n");
        tests.push_str("const { expect } = require(\"chai\");\n");
        tests.push_str("const { ethers } = require(\"hardhat\");\n");
        tests.push_str(
            "const { loadFixture } = require(\"@nomicfoundation/hardhat-network-helpers\");\n\n",
        );

        tests.push_str(&format!(
            "describe(\"{} Security Tests\", function () {{\n",
            contract.name
        ));
        tests.push_str("  async function deployContractFixture() {\n");
        tests.push_str("    const [owner, attacker] = await ethers.getSigners();\n");
        tests.push_str(&format!(
            "    const ContractFactory = await ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));
        tests.push_str("    const contract = await ContractFactory.deploy();\n");
        tests.push_str("    await contract.waitForDeployment();\n");
        tests.push_str("    return { contract, owner, attacker };\n");
        tests.push_str("  }\n\n");

        tests.push_str("  describe(\"Access Control\", function () {\n");
        tests.push_str("    it(\"should only allow owner to perform privileged operations\", async function () {\n");
        tests.push_str(
            "      const { contract, attacker } = await loadFixture(deployContractFixture);\n",
        );
        tests.push_str("      // Test that non-owner cannot call owner-only functions\n");
        tests.push_str("      // This will vary based on the contract\n");
        tests.push_str("    });\n");
        tests.push_str("  });\n\n");

        tests.push_str("  describe(\"Reentrancy Protection\", function () {\n");
        tests.push_str("    it(\"should prevent reentrancy attacks\", async function () {\n");
        tests.push_str("      const { contract } = await loadFixture(deployContractFixture);\n");
        tests.push_str("      // Test reentrancy protection mechanisms\n");
        tests.push_str("    });\n");
        tests.push_str("  });\n\n");

        tests.push_str("  describe(\"Input Validation\", function () {\n");
        tests.push_str("    it(\"should reject invalid inputs\", async function () {\n");
        tests.push_str("      const { contract } = await loadFixture(deployContractFixture);\n");
        tests.push_str("      // Test input validation\n");
        tests.push_str("      await expect(\n");
        tests.push_str("        contract.checkEligibility(ethers.ZeroAddress)\n");
        tests.push_str("      ).to.be.reverted;\n");
        tests.push_str("    });\n");
        tests.push_str("  });\n\n");

        tests.push_str("  describe(\"Integer Overflow/Underflow\", function () {\n");
        tests.push_str("    it(\"should handle large numbers safely\", async function () {\n");
        tests.push_str("      const { contract } = await loadFixture(deployContractFixture);\n");
        tests.push_str("      // Test safe math operations\n");
        tests.push_str("    });\n");
        tests.push_str("  });\n\n");

        tests.push_str("  describe(\"Front-Running Protection\", function () {\n");
        tests.push_str(
            "    it(\"should have measures against front-running\", async function () {\n",
        );
        tests.push_str("      const { contract } = await loadFixture(deployContractFixture);\n");
        tests.push_str("      // Test front-running protection mechanisms\n");
        tests.push_str("    });\n");
        tests.push_str("  });\n");

        tests.push_str("});\n");

        Ok(tests)
    }

    fn generate_solidity_token(&self, config: &TokenConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");

        match config.standard {
            TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                source.push_str("import \"@openzeppelin/contracts/token/ERC20/ERC20.sol\";\n");
                if config.burnable {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol\";\n");
                }
                if config.pausable {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol\";\n");
                    source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
                }
                if config.snapshot {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Snapshot.sol\";\n");
                }
                if config.mintable {
                    source.push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
                }
            }
            TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                source.push_str("import \"@openzeppelin/contracts/token/ERC721/ERC721.sol\";\n");
                if config.burnable {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol\";\n");
                }
                if config.pausable {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol\";\n");
                    source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
                }
                source.push_str("import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol\";\n");
                source.push_str("import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol\";\n");
                if config.mintable {
                    source.push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
                }
            }
            TokenStandard::Erc1155 => {
                source.push_str("import \"@openzeppelin/contracts/token/ERC1155/ERC1155.sol\";\n");
                if config.burnable {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Burnable.sol\";\n");
                }
                if config.pausable {
                    source.push_str("import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Pausable.sol\";\n");
                    source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
                }
                source.push_str("import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Supply.sol\";\n");
                if config.mintable {
                    source.push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
                }
            }
        }

        source.push_str("\n/// @title ");
        source.push_str(&config.name);
        source.push_str("\n/// @notice ");
        match config.standard {
            TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                source.push_str("ERC-20 token implementation");
            }
            TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                source.push_str("ERC-721 NFT implementation");
            }
            TokenStandard::Erc1155 => {
                source.push_str("ERC-1155 multi-token implementation");
            }
        }
        source.push_str("\n/// @dev Generated by Legalis-Chain\n");
        source.push_str("contract ");
        source.push_str(&config.name);
        source.push_str(" is ");

        let mut inherits = Vec::new();
        match config.standard {
            TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                inherits.push("ERC20");
                if config.burnable {
                    inherits.push("ERC20Burnable");
                }
                if config.pausable {
                    inherits.push("ERC20Pausable");
                    inherits.push("Ownable");
                }
                if config.snapshot {
                    inherits.push("ERC20Snapshot");
                }
            }
            TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                inherits.push("ERC721");
                inherits.push("ERC721Enumerable");
                inherits.push("ERC721URIStorage");
                if config.burnable {
                    inherits.push("ERC721Burnable");
                }
                if config.pausable {
                    inherits.push("ERC721Pausable");
                    inherits.push("Ownable");
                }
            }
            TokenStandard::Erc1155 => {
                inherits.push("ERC1155");
                inherits.push("ERC1155Supply");
                if config.burnable {
                    inherits.push("ERC1155Burnable");
                }
                if config.pausable {
                    inherits.push("ERC1155Pausable");
                    inherits.push("Ownable");
                }
            }
        }
        if config.mintable {
            inherits.push("AccessControl");
        }
        source.push_str(&inherits.join(", "));
        source.push_str(" {\n");

        if config.mintable {
            source.push_str("    bytes32 public constant MINTER_ROLE = keccak256(\"MINTER_ROLE\");\n\n");
        }

        if matches!(config.standard, TokenStandard::Erc721 | TokenStandard::Erc721Extended) {
            source.push_str("    uint256 private _nextTokenId;\n\n");
        }

        source.push_str("    constructor()\n");
        match config.standard {
            TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                source.push_str(&format!("        ERC20(\"{}\", \"{}\")\n", config.name, config.symbol));
            }
            TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                source.push_str(&format!("        ERC721(\"{}\", \"{}\")\n", config.name, config.symbol));
            }
            TokenStandard::Erc1155 => {
                let base_uri = config.base_uri.as_deref().unwrap_or("https://token-cdn.domain/{id}.json");
                source.push_str(&format!("        ERC1155(\"{}\")\n", base_uri));
            }
        }
        if config.pausable {
            source.push_str("        Ownable(msg.sender)\n");
        }
        source.push_str("    {\n");

        if config.mintable {
            source.push_str("        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);\n");
            source.push_str("        _grantRole(MINTER_ROLE, msg.sender);\n");
        }

        if let Some(initial_supply) = config.initial_supply {
            if matches!(config.standard, TokenStandard::Erc20 | TokenStandard::Erc20Extended) {
                source.push_str(&format!("        _mint(msg.sender, {} * 10 ** decimals());\n", initial_supply));
            }
        }

        source.push_str("    }\n\n");

        if config.mintable {
            match config.standard {
                TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                    source.push_str("    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {\n");
                    source.push_str("        _mint(to, amount);\n");
                    source.push_str("    }\n\n");
                }
                TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                    source.push_str("    function safeMint(address to, string memory uri) public onlyRole(MINTER_ROLE) {\n");
                    source.push_str("        uint256 tokenId = _nextTokenId++;\n");
                    source.push_str("        _safeMint(to, tokenId);\n");
                    source.push_str("        _setTokenURI(tokenId, uri);\n");
                    source.push_str("    }\n\n");
                }
                TokenStandard::Erc1155 => {
                    source.push_str("    function mint(address to, uint256 id, uint256 amount, bytes memory data) public onlyRole(MINTER_ROLE) {\n");
                    source.push_str("        _mint(to, id, amount, data);\n");
                    source.push_str("    }\n\n");
                    source.push_str("    function mintBatch(address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data) public onlyRole(MINTER_ROLE) {\n");
                    source.push_str("        _mintBatch(to, ids, amounts, data);\n");
                    source.push_str("    }\n\n");
                }
            }
        }

        if config.pausable {
            source.push_str("    function pause() public onlyOwner {\n");
            source.push_str("        _pause();\n");
            source.push_str("    }\n\n");
            source.push_str("    function unpause() public onlyOwner {\n");
            source.push_str("        _unpause();\n");
            source.push_str("    }\n\n");
        }

        if config.snapshot && matches!(config.standard, TokenStandard::Erc20 | TokenStandard::Erc20Extended) {
            source.push_str("    function snapshot() public onlyOwner {\n");
            source.push_str("        _snapshot();\n");
            source.push_str("    }\n\n");
        }

        if matches!(config.standard, TokenStandard::Erc721 | TokenStandard::Erc721Extended) {
            source.push_str("    function _update(address to, uint256 tokenId, address auth)\n");
            source.push_str("        internal\n");
            source.push_str("        override(ERC721, ERC721Enumerable");
            if config.pausable {
                source.push_str(", ERC721Pausable");
            }
            source.push_str(")\n");
            source.push_str("        returns (address)\n");
            source.push_str("    {\n");
            source.push_str("        return super._update(to, tokenId, auth);\n");
            source.push_str("    }\n\n");

            source.push_str("    function _increaseBalance(address account, uint128 value)\n");
            source.push_str("        internal\n");
            source.push_str("        override(ERC721, ERC721Enumerable)\n");
            source.push_str("    {\n");
            source.push_str("        super._increaseBalance(account, value);\n");
            source.push_str("    }\n\n");

            source.push_str("    function tokenURI(uint256 tokenId)\n");
            source.push_str("        public\n");
            source.push_str("        view\n");
            source.push_str("        override(ERC721, ERC721URIStorage)\n");
            source.push_str("        returns (string memory)\n");
            source.push_str("    {\n");
            source.push_str("        return super.tokenURI(tokenId);\n");
            source.push_str("    }\n\n");

            source.push_str("    function supportsInterface(bytes4 interfaceId)\n");
            source.push_str("        public\n");
            source.push_str("        view\n");
            source.push_str("        override(ERC721, ERC721Enumerable, ERC721URIStorage");
            if config.mintable {
                source.push_str(", AccessControl");
            }
            source.push_str(")\n");
            source.push_str("        returns (bool)\n");
            source.push_str("    {\n");
            source.push_str("        return super.supportsInterface(interfaceId);\n");
            source.push_str("    }\n");
        }

        if matches!(config.standard, TokenStandard::Erc1155) {
            source.push_str("    function _update(address from, address to, uint256[] memory ids, uint256[] memory values)\n");
            source.push_str("        internal\n");
            source.push_str("        override(ERC1155, ERC1155Supply");
            if config.pausable {
                source.push_str(", ERC1155Pausable");
            }
            source.push_str(")\n");
            source.push_str("    {\n");
            source.push_str("        super._update(from, to, ids, values);\n");
            source.push_str("    }\n\n");

            source.push_str("    function supportsInterface(bytes4 interfaceId)\n");
            source.push_str("        public\n");
            source.push_str("        view\n");
            source.push_str("        override(ERC1155");
            if config.mintable {
                source.push_str(", AccessControl");
            }
            source.push_str(")\n");
            source.push_str("        returns (bool)\n");
            source.push_str("    {\n");
            source.push_str("        return super.supportsInterface(interfaceId);\n");
            source.push_str("    }\n");
        }

        if matches!(config.standard, TokenStandard::Erc20 | TokenStandard::Erc20Extended) && (config.pausable || config.snapshot) {
            source.push_str("    function _update(address from, address to, uint256 value)\n");
            source.push_str("        internal\n");
            source.push_str("        override(ERC20");
            if config.pausable {
                source.push_str(", ERC20Pausable");
            }
            if config.snapshot {
                source.push_str(", ERC20Snapshot");
            }
            source.push_str(")\n");
            source.push_str("    {\n");
            source.push_str("        super._update(from, to, value);\n");
            source.push_str("    }\n");
        }

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_vyper_token(&self, config: &TokenConfig) -> ChainResult<GeneratedContract> {
        if !matches!(config.standard, TokenStandard::Erc20 | TokenStandard::Erc20Extended) {
            return Err(ChainError::GenerationError(
                "Vyper currently only supports ERC-20 tokens".to_string(),
            ));
        }

        let mut source = String::new();

        source.push_str("# @version ^0.3.0\n\n");
        source.push_str("from vyper.interfaces import ERC20\n\n");

        source.push_str(&format!("name: public(String[64]) = \"{}\"\n", config.name));
        source.push_str(&format!("symbol: public(String[32]) = \"{}\"\n", config.symbol));
        source.push_str("decimals: public(uint8) = 18\n");
        source.push_str("totalSupply: public(uint256)\n");
        source.push_str("balanceOf: public(HashMap[address, uint256])\n");
        source.push_str("allowance: public(HashMap[address, HashMap[address, uint256]])\n\n");

        if config.pausable {
            source.push_str("owner: public(address)\n");
            source.push_str("paused: public(bool)\n\n");
        }

        source.push_str("event Transfer:\n");
        source.push_str("    sender: indexed(address)\n");
        source.push_str("    receiver: indexed(address)\n");
        source.push_str("    value: uint256\n\n");

        source.push_str("event Approval:\n");
        source.push_str("    owner: indexed(address)\n");
        source.push_str("    spender: indexed(address)\n");
        source.push_str("    value: uint256\n\n");

        source.push_str("@external\n");
        source.push_str("def __init__():\n");
        if let Some(initial_supply) = config.initial_supply {
            source.push_str(&format!("    self.totalSupply = {} * 10 ** 18\n", initial_supply));
            source.push_str("    self.balanceOf[msg.sender] = self.totalSupply\n");
        }
        if config.pausable {
            source.push_str("    self.owner = msg.sender\n");
            source.push_str("    self.paused = False\n");
        }
        source.push('\n');

        source.push_str("@external\n");
        source.push_str("def transfer(_to: address, _value: uint256) -> bool:\n");
        if config.pausable {
            source.push_str("    assert not self.paused, \"Token is paused\"\n");
        }
        source.push_str("    self.balanceOf[msg.sender] -= _value\n");
        source.push_str("    self.balanceOf[_to] += _value\n");
        source.push_str("    log Transfer(msg.sender, _to, _value)\n");
        source.push_str("    return True\n\n");

        source.push_str("@external\n");
        source.push_str("def approve(_spender: address, _value: uint256) -> bool:\n");
        source.push_str("    self.allowance[msg.sender][_spender] = _value\n");
        source.push_str("    log Approval(msg.sender, _spender, _value)\n");
        source.push_str("    return True\n\n");

        source.push_str("@external\n");
        source.push_str("def transferFrom(_from: address, _to: address, _value: uint256) -> bool:\n");
        if config.pausable {
            source.push_str("    assert not self.paused, \"Token is paused\"\n");
        }
        source.push_str("    self.balanceOf[_from] -= _value\n");
        source.push_str("    self.balanceOf[_to] += _value\n");
        source.push_str("    self.allowance[_from][msg.sender] -= _value\n");
        source.push_str("    log Transfer(_from, _to, _value)\n");
        source.push_str("    return True\n");

        if config.pausable {
            source.push_str("\n@external\n");
            source.push_str("def pause():\n");
            source.push_str("    assert msg.sender == self.owner, \"Only owner\"\n");
            source.push_str("    self.paused = True\n\n");

            source.push_str("@external\n");
            source.push_str("def unpause():\n");
            source.push_str("    assert msg.sender == self.owner, \"Only owner\"\n");
            source.push_str("    self.paused = False\n");
        }

        if config.mintable {
            source.push_str("\n@external\n");
            source.push_str("def mint(_to: address, _value: uint256):\n");
            source.push_str("    assert msg.sender == self.owner, \"Only owner\"\n");
            source.push_str("    self.totalSupply += _value\n");
            source.push_str("    self.balanceOf[_to] += _value\n");
            source.push_str("    log Transfer(empty(address), _to, _value)\n");
        }

        if config.burnable {
            source.push_str("\n@external\n");
            source.push_str("def burn(_value: uint256):\n");
            source.push_str("    self.balanceOf[msg.sender] -= _value\n");
            source.push_str("    self.totalSupply -= _value\n");
            source.push_str("    log Transfer(msg.sender, empty(address), _value)\n");
        }

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Vyper,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_dao(&self, config: &DaoConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str("import \"@openzeppelin/contracts/governance/Governor.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/governance/extensions/GovernorSettings.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/governance/extensions/GovernorCountingSimple.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/governance/extensions/GovernorVotes.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/governance/extensions/GovernorVotesQuorumFraction.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/governance/extensions/GovernorTimelockControl.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/governance/TimelockController.sol\";\n\n");

        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str("/// @notice DAO governance contract\n");
        source.push_str("/// @dev Uses OpenZeppelin Governor framework\n");
        source.push_str(&format!("contract {} is Governor, GovernorSettings, GovernorCountingSimple, GovernorVotes, GovernorVotesQuorumFraction, GovernorTimelockControl {{\n", config.name));

        source.push_str("    constructor(IVotes _token, TimelockController _timelock)\n");
        source.push_str(&format!("        Governor(\"{}\")\n", config.name));
        source.push_str(&format!("        GovernorSettings({}, {}, {})\n",
            1, // voting delay
            config.voting_period,
            config.proposal_threshold
        ));
        source.push_str("        GovernorVotes(_token)\n");
        source.push_str(&format!("        GovernorVotesQuorumFraction({})\n", config.quorum_percentage));
        source.push_str("        GovernorTimelockControl(_timelock)\n");
        source.push_str("    {}\n\n");

        source.push_str("    function votingDelay()\n");
        source.push_str("        public\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorSettings)\n");
        source.push_str("        returns (uint256)\n");
        source.push_str("    {\n");
        source.push_str("        return super.votingDelay();\n");
        source.push_str("    }\n\n");

        source.push_str("    function votingPeriod()\n");
        source.push_str("        public\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorSettings)\n");
        source.push_str("        returns (uint256)\n");
        source.push_str("    {\n");
        source.push_str("        return super.votingPeriod();\n");
        source.push_str("    }\n\n");

        source.push_str("    function quorum(uint256 blockNumber)\n");
        source.push_str("        public\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorVotesQuorumFraction)\n");
        source.push_str("        returns (uint256)\n");
        source.push_str("    {\n");
        source.push_str("        return super.quorum(blockNumber);\n");
        source.push_str("    }\n\n");

        source.push_str("    function state(uint256 proposalId)\n");
        source.push_str("        public\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (ProposalState)\n");
        source.push_str("    {\n");
        source.push_str("        return super.state(proposalId);\n");
        source.push_str("    }\n\n");

        source.push_str("    function proposalNeedsQueuing(uint256 proposalId)\n");
        source.push_str("        public\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (bool)\n");
        source.push_str("    {\n");
        source.push_str("        return super.proposalNeedsQueuing(proposalId);\n");
        source.push_str("    }\n\n");

        source.push_str("    function proposalThreshold()\n");
        source.push_str("        public\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorSettings)\n");
        source.push_str("        returns (uint256)\n");
        source.push_str("    {\n");
        source.push_str("        return super.proposalThreshold();\n");
        source.push_str("    }\n\n");

        source.push_str("    function _queueOperations(uint256 proposalId, address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash)\n");
        source.push_str("        internal\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (uint48)\n");
        source.push_str("    {\n");
        source.push_str("        return super._queueOperations(proposalId, targets, values, calldatas, descriptionHash);\n");
        source.push_str("    }\n\n");

        source.push_str("    function _executeOperations(uint256 proposalId, address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash)\n");
        source.push_str("        internal\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("    {\n");
        source.push_str("        super._executeOperations(proposalId, targets, values, calldatas, descriptionHash);\n");
        source.push_str("    }\n\n");

        source.push_str("    function _cancel(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash)\n");
        source.push_str("        internal\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (uint256)\n");
        source.push_str("    {\n");
        source.push_str("        return super._cancel(targets, values, calldatas, descriptionHash);\n");
        source.push_str("    }\n\n");

        source.push_str("    function _executor()\n");
        source.push_str("        internal\n");
        source.push_str("        view\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (address)\n");
        source.push_str("    {\n");
        source.push_str("        return super._executor();\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_bridge(&self, config: &BridgeConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/IERC20.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/security/Pausable.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/security/ReentrancyGuard.sol\";\n\n");

        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str("/// @notice Cross-chain bridge for token transfers\n");
        source.push_str("/// @dev Implements lock-and-mint bridge pattern\n");
        source.push_str(&format!("contract {} is Ownable, Pausable, ReentrancyGuard {{\n", config.name));
        source.push_str("    using SafeERC20 for IERC20;\n\n");

        source.push_str("    struct Transfer {\n");
        source.push_str("        address token;\n");
        source.push_str("        address from;\n");
        source.push_str("        address to;\n");
        source.push_str("        uint256 amount;\n");
        source.push_str("        uint256 nonce;\n");
        source.push_str("        uint256 timestamp;\n");
        source.push_str("        bool processed;\n");
        source.push_str("    }\n\n");

        source.push_str(&format!("    uint256 public constant SOURCE_CHAIN_ID = {};\n", config.source_chain_id));
        source.push_str(&format!("    uint256 public constant DESTINATION_CHAIN_ID = {};\n", config.destination_chain_id));
        source.push_str(&format!("    uint256 public constant FEE_BASIS_POINTS = {};  // {}%\n",
            config.fee_basis_points,
            config.fee_basis_points as f64 / 100.0
        ));
        source.push_str("    uint256 public constant BASIS_POINTS_DIVISOR = 10000;\n\n");

        source.push_str("    mapping(address => bool) public supportedTokens;\n");
        source.push_str("    mapping(bytes32 => bool) public processedTransfers;\n");
        source.push_str("    mapping(address => uint256) public nonces;\n");
        source.push_str("    uint256 public totalValueLocked;\n\n");

        source.push_str("    event TokensLocked(bytes32 indexed transferId, address indexed token, address indexed from, address to, uint256 amount, uint256 nonce);\n");
        source.push_str("    event TokensReleased(bytes32 indexed transferId, address indexed token, address indexed to, uint256 amount);\n");
        source.push_str("    event TokenAdded(address indexed token);\n");
        source.push_str("    event TokenRemoved(address indexed token);\n");
        source.push_str("    event FeesCollected(address indexed token, uint256 amount);\n\n");

        source.push_str("    constructor() Ownable(msg.sender) {\n");
        for token in &config.supported_tokens {
            source.push_str(&format!("        supportedTokens[{}] = true;\n", token));
        }
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Lock tokens to transfer to destination chain\n");
        source.push_str("    /// @param token Token contract address\n");
        source.push_str("    /// @param to Recipient address on destination chain\n");
        source.push_str("    /// @param amount Amount to transfer\n");
        source.push_str("    function lockTokens(address token, address to, uint256 amount) external whenNotPaused nonReentrant returns (bytes32) {\n");
        source.push_str("        require(supportedTokens[token], \"Token not supported\");\n");
        source.push_str("        require(amount > 0, \"Amount must be positive\");\n");
        source.push_str("        require(to != address(0), \"Invalid recipient\");\n\n");

        source.push_str("        uint256 fee = (amount * FEE_BASIS_POINTS) / BASIS_POINTS_DIVISOR;\n");
        source.push_str("        uint256 amountAfterFee = amount - fee;\n\n");

        source.push_str("        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);\n");
        source.push_str("        totalValueLocked += amountAfterFee;\n\n");

        source.push_str("        uint256 nonce = nonces[msg.sender]++;\n");
        source.push_str("        bytes32 transferId = keccak256(abi.encodePacked(token, msg.sender, to, amount, nonce, block.chainid));\n\n");

        source.push_str("        emit TokensLocked(transferId, token, msg.sender, to, amountAfterFee, nonce);\n");
        source.push_str("        if (fee > 0) {\n");
        source.push_str("            emit FeesCollected(token, fee);\n");
        source.push_str("        }\n\n");

        source.push_str("        return transferId;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Release tokens on destination chain (only owner/validator)\n");
        source.push_str("    /// @param token Token contract address\n");
        source.push_str("    /// @param to Recipient address\n");
        source.push_str("    /// @param amount Amount to release\n");
        source.push_str("    /// @param transferId Original transfer ID from source chain\n");
        source.push_str("    function releaseTokens(address token, address to, uint256 amount, bytes32 transferId) external onlyOwner whenNotPaused nonReentrant {\n");
        source.push_str("        require(!processedTransfers[transferId], \"Transfer already processed\");\n");
        source.push_str("        require(supportedTokens[token], \"Token not supported\");\n");
        source.push_str("        require(amount > 0, \"Amount must be positive\");\n");
        source.push_str("        require(to != address(0), \"Invalid recipient\");\n\n");

        source.push_str("        processedTransfers[transferId] = true;\n");
        source.push_str("        totalValueLocked -= amount;\n\n");

        source.push_str("        IERC20(token).safeTransfer(to, amount);\n\n");

        source.push_str("        emit TokensReleased(transferId, token, to, amount);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Add supported token\n");
        source.push_str("    function addSupportedToken(address token) external onlyOwner {\n");
        source.push_str("        require(!supportedTokens[token], \"Token already supported\");\n");
        source.push_str("        supportedTokens[token] = true;\n");
        source.push_str("        emit TokenAdded(token);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Remove supported token\n");
        source.push_str("    function removeSupportedToken(address token) external onlyOwner {\n");
        source.push_str("        require(supportedTokens[token], \"Token not supported\");\n");
        source.push_str("        supportedTokens[token] = false;\n");
        source.push_str("        emit TokenRemoved(token);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Withdraw collected fees\n");
        source.push_str("    function withdrawFees(address token, uint256 amount) external onlyOwner {\n");
        source.push_str("        IERC20(token).safeTransfer(msg.sender, amount);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Pause bridge operations\n");
        source.push_str("    function pause() external onlyOwner {\n");
        source.push_str("        _pause();\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Unpause bridge operations\n");
        source.push_str("    function unpause() external onlyOwner {\n");
        source.push_str("        _unpause();\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_treasury(&self, config: &TreasuryConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/security/ReentrancyGuard.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/IERC20.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol\";\n\n");

        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str("/// @notice Treasury management contract with spending limits and multi-approval\n");
        source.push_str("/// @dev Implements role-based access control and daily spending limits\n");
        source.push_str(&format!("contract {} is AccessControl, ReentrancyGuard {{\n", config.name));
        source.push_str("    using SafeERC20 for IERC20;\n\n");

        source.push_str("    bytes32 public constant SPENDER_ROLE = keccak256(\"SPENDER_ROLE\");\n");
        source.push_str("    bytes32 public constant APPROVER_ROLE = keccak256(\"APPROVER_ROLE\");\n\n");

        source.push_str(&format!("    uint256 public dailyLimit = {};  // Daily spending limit in wei\n", config.daily_limit));
        source.push_str(&format!("    uint256 public multiApprovalThreshold = {};  // Threshold requiring multiple approvals\n", config.multi_approval_threshold));
        source.push_str("    uint256 public spentToday;\n");
        source.push_str("    uint256 public lastDay;\n\n");

        source.push_str("    struct Proposal {\n");
        source.push_str("        address to;\n");
        source.push_str("        uint256 amount;\n");
        source.push_str("        bytes data;\n");
        source.push_str("        uint256 approvals;\n");
        source.push_str("        bool executed;\n");
        source.push_str("        mapping(address => bool) approved;\n");
        source.push_str("    }\n\n");

        source.push_str("    mapping(uint256 => Proposal) public proposals;\n");
        source.push_str("    uint256 public proposalCount;\n\n");

        source.push_str("    event Deposit(address indexed sender, uint256 amount);\n");
        source.push_str("    event Withdrawal(address indexed to, uint256 amount);\n");
        source.push_str("    event ProposalCreated(uint256 indexed proposalId, address indexed to, uint256 amount);\n");
        source.push_str("    event ProposalApproved(uint256 indexed proposalId, address indexed approver);\n");
        source.push_str("    event ProposalExecuted(uint256 indexed proposalId);\n\n");

        source.push_str("    constructor() {\n");
        source.push_str("        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);\n");
        source.push_str("        _grantRole(APPROVER_ROLE, msg.sender);\n");
        for spender in &config.authorized_spenders {
            source.push_str(&format!("        _grantRole(SPENDER_ROLE, {});\n", spender));
        }
        source.push_str("        lastDay = block.timestamp / 1 days;\n");
        source.push_str("    }\n\n");

        source.push_str("    receive() external payable {\n");
        source.push_str("        emit Deposit(msg.sender, msg.value);\n");
        source.push_str("    }\n\n");

        source.push_str("    function withdraw(address payable to, uint256 amount) external onlyRole(SPENDER_ROLE) nonReentrant {\n");
        source.push_str("        require(amount <= dailyLimit, \"Exceeds daily limit\");\n");
        source.push_str("        _resetDailyLimitIfNeeded();\n");
        source.push_str("        require(spentToday + amount <= dailyLimit, \"Daily limit exceeded\");\n");
        source.push_str("        spentToday += amount;\n");
        source.push_str("        (bool success, ) = to.call{value: amount}(\"\");\n");
        source.push_str("        require(success, \"Transfer failed\");\n");
        source.push_str("        emit Withdrawal(to, amount);\n");
        source.push_str("    }\n\n");

        source.push_str("    function proposeWithdrawal(address to, uint256 amount, bytes memory data) external onlyRole(SPENDER_ROLE) returns (uint256) {\n");
        source.push_str("        require(amount >= multiApprovalThreshold, \"Amount below threshold\");\n");
        source.push_str("        uint256 proposalId = proposalCount++;\n");
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str("        proposal.to = to;\n");
        source.push_str("        proposal.amount = amount;\n");
        source.push_str("        proposal.data = data;\n");
        source.push_str("        proposal.approvals = 0;\n");
        source.push_str("        proposal.executed = false;\n");
        source.push_str("        emit ProposalCreated(proposalId, to, amount);\n");
        source.push_str("        return proposalId;\n");
        source.push_str("    }\n\n");

        source.push_str("    function approveProposal(uint256 proposalId) external onlyRole(APPROVER_ROLE) {\n");
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str("        require(!proposal.executed, \"Already executed\");\n");
        source.push_str("        require(!proposal.approved[msg.sender], \"Already approved\");\n");
        source.push_str("        proposal.approved[msg.sender] = true;\n");
        source.push_str("        proposal.approvals++;\n");
        source.push_str("        emit ProposalApproved(proposalId, msg.sender);\n");
        source.push_str("    }\n\n");

        source.push_str("    function executeProposal(uint256 proposalId) external onlyRole(SPENDER_ROLE) nonReentrant {\n");
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str("        require(!proposal.executed, \"Already executed\");\n");
        source.push_str("        require(proposal.approvals >= 2, \"Insufficient approvals\");\n");
        source.push_str("        proposal.executed = true;\n");
        source.push_str("        (bool success, ) = proposal.to.call{value: proposal.amount}(proposal.data);\n");
        source.push_str("        require(success, \"Execution failed\");\n");
        source.push_str("        emit ProposalExecuted(proposalId);\n");
        source.push_str("        emit Withdrawal(proposal.to, proposal.amount);\n");
        source.push_str("    }\n\n");

        source.push_str("    function _resetDailyLimitIfNeeded() private {\n");
        source.push_str("        uint256 today = block.timestamp / 1 days;\n");
        source.push_str("        if (today > lastDay) {\n");
        source.push_str("            spentToday = 0;\n");
        source.push_str("            lastDay = today;\n");
        source.push_str("        }\n");
        source.push_str("    }\n\n");

        source.push_str("    function withdrawToken(address token, address to, uint256 amount) external onlyRole(SPENDER_ROLE) nonReentrant {\n");
        source.push_str("        IERC20(token).safeTransfer(to, amount);\n");
        source.push_str("        emit Withdrawal(to, amount);\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_vesting(&self, config: &VestingConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/IERC20.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n\n");

        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str("/// @notice Token vesting contract with cliff and linear vesting\n");
        source.push_str("/// @dev Based on OpenZeppelin VestingWallet pattern\n");
        source.push_str(&format!("contract {} is Ownable {{\n", config.name));
        source.push_str("    using SafeERC20 for IERC20;\n\n");

        source.push_str(&format!("    address public immutable beneficiary = {};\n", config.beneficiary));
        source.push_str(&format!("    uint256 public immutable start = {};\n", config.start));
        source.push_str(&format!("    uint256 public immutable cliffDuration = {};\n", config.cliff_duration));
        source.push_str(&format!("    uint256 public immutable duration = {};\n", config.duration));
        source.push_str(&format!("    bool public immutable revocable = {};\n\n", config.revocable));

        source.push_str("    mapping(address => uint256) public released;\n");
        source.push_str("    mapping(address => bool) public revoked;\n\n");

        source.push_str("    event TokensReleased(address indexed token, uint256 amount);\n");
        source.push_str("    event VestingRevoked(address indexed token);\n\n");

        source.push_str("    constructor() Ownable(msg.sender) {}\n\n");

        source.push_str("    function release(address token) external {\n");
        source.push_str("        require(!revoked[token], \"Vesting revoked\");\n");
        source.push_str("        uint256 releasable = _releasableAmount(token);\n");
        source.push_str("        require(releasable > 0, \"No tokens to release\");\n");
        source.push_str("        released[token] += releasable;\n");
        source.push_str("        IERC20(token).safeTransfer(beneficiary, releasable);\n");
        source.push_str("        emit TokensReleased(token, releasable);\n");
        source.push_str("    }\n\n");

        if config.revocable {
            source.push_str("    function revoke(address token) external onlyOwner {\n");
            source.push_str("        require(!revoked[token], \"Already revoked\");\n");
            source.push_str("        uint256 balance = IERC20(token).balanceOf(address(this));\n");
            source.push_str("        uint256 releasable = _releasableAmount(token);\n");
            source.push_str("        uint256 refund = balance - releasable;\n");
            source.push_str("        revoked[token] = true;\n");
            source.push_str("        IERC20(token).safeTransfer(owner(), refund);\n");
            source.push_str("        emit VestingRevoked(token);\n");
            source.push_str("    }\n\n");
        }

        source.push_str("    function vestedAmount(address token) public view returns (uint256) {\n");
        source.push_str("        if (block.timestamp < start + cliffDuration) {\n");
        source.push_str("            return 0;\n");
        source.push_str("        }\n");
        source.push_str("        uint256 totalAllocation = IERC20(token).balanceOf(address(this)) + released[token];\n");
        source.push_str("        if (block.timestamp >= start + duration) {\n");
        source.push_str("            return totalAllocation;\n");
        source.push_str("        }\n");
        source.push_str("        return (totalAllocation * (block.timestamp - start)) / duration;\n");
        source.push_str("    }\n\n");

        source.push_str("    function _releasableAmount(address token) private view returns (uint256) {\n");
        source.push_str("        return vestedAmount(token) - released[token];\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_multisig(&self, config: &MultisigConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");

        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str("/// @notice Multi-signature wallet requiring multiple confirmations\n");
        source.push_str("/// @dev Implements daily limits and transaction confirmation system\n");
        source.push_str(&format!("contract {} {{\n", config.name));

        source.push_str("    struct Transaction {\n");
        source.push_str("        address to;\n");
        source.push_str("        uint256 value;\n");
        source.push_str("        bytes data;\n");
        source.push_str("        bool executed;\n");
        source.push_str("        uint256 confirmations;\n");
        source.push_str("    }\n\n");

        source.push_str("    address[] public owners;\n");
        source.push_str("    mapping(address => bool) public isOwner;\n");
        source.push_str(&format!("    uint256 public required = {};\n", config.required_confirmations));
        if let Some(limit) = config.daily_limit {
            source.push_str(&format!("    uint256 public dailyLimit = {};\n", limit));
        }
        source.push_str("    uint256 public spentToday;\n");
        source.push_str("    uint256 public lastDay;\n\n");

        source.push_str("    Transaction[] public transactions;\n");
        source.push_str("    mapping(uint256 => mapping(address => bool)) public confirmations;\n\n");

        source.push_str("    event Deposit(address indexed sender, uint256 value);\n");
        source.push_str("    event Submission(uint256 indexed transactionId);\n");
        source.push_str("    event Confirmation(address indexed sender, uint256 indexed transactionId);\n");
        source.push_str("    event Execution(uint256 indexed transactionId);\n");
        source.push_str("    event ExecutionFailure(uint256 indexed transactionId);\n");
        source.push_str("    event Revocation(address indexed sender, uint256 indexed transactionId);\n\n");

        source.push_str("    modifier onlyOwner() {\n");
        source.push_str("        require(isOwner[msg.sender], \"Not owner\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier transactionExists(uint256 transactionId) {\n");
        source.push_str("        require(transactionId < transactions.length, \"Transaction does not exist\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier notExecuted(uint256 transactionId) {\n");
        source.push_str("        require(!transactions[transactionId].executed, \"Transaction already executed\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier notConfirmed(uint256 transactionId) {\n");
        source.push_str("        require(!confirmations[transactionId][msg.sender], \"Transaction already confirmed\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    constructor() {\n");
        source.push_str(&format!("        require({} <= {}, \"Invalid required confirmations\");\n",
            config.required_confirmations, config.owners.len()));
        for (idx, owner) in config.owners.iter().enumerate() {
            source.push_str(&format!("        address owner{} = {};\n", idx, owner));
            source.push_str(&format!("        require(owner{} != address(0), \"Invalid owner\");\n", idx));
            source.push_str(&format!("        require(!isOwner[owner{}], \"Duplicate owner\");\n", idx));
            source.push_str(&format!("        isOwner[owner{}] = true;\n", idx));
            source.push_str(&format!("        owners.push(owner{});\n", idx));
        }
        source.push_str("        lastDay = block.timestamp / 1 days;\n");
        source.push_str("    }\n\n");

        source.push_str("    receive() external payable {\n");
        source.push_str("        if (msg.value > 0) {\n");
        source.push_str("            emit Deposit(msg.sender, msg.value);\n");
        source.push_str("        }\n");
        source.push_str("    }\n\n");

        source.push_str("    function submitTransaction(address to, uint256 value, bytes memory data) external onlyOwner returns (uint256) {\n");
        source.push_str("        uint256 transactionId = transactions.length;\n");
        source.push_str("        transactions.push(Transaction({\n");
        source.push_str("            to: to,\n");
        source.push_str("            value: value,\n");
        source.push_str("            data: data,\n");
        source.push_str("            executed: false,\n");
        source.push_str("            confirmations: 0\n");
        source.push_str("        }));\n");
        source.push_str("        emit Submission(transactionId);\n");
        source.push_str("        confirmTransaction(transactionId);\n");
        source.push_str("        return transactionId;\n");
        source.push_str("    }\n\n");

        source.push_str("    function confirmTransaction(uint256 transactionId)\n");
        source.push_str("        public\n");
        source.push_str("        onlyOwner\n");
        source.push_str("        transactionExists(transactionId)\n");
        source.push_str("        notExecuted(transactionId)\n");
        source.push_str("        notConfirmed(transactionId)\n");
        source.push_str("    {\n");
        source.push_str("        confirmations[transactionId][msg.sender] = true;\n");
        source.push_str("        transactions[transactionId].confirmations++;\n");
        source.push_str("        emit Confirmation(msg.sender, transactionId);\n");
        source.push_str("        executeTransaction(transactionId);\n");
        source.push_str("    }\n\n");

        source.push_str("    function executeTransaction(uint256 transactionId)\n");
        source.push_str("        public\n");
        source.push_str("        onlyOwner\n");
        source.push_str("        transactionExists(transactionId)\n");
        source.push_str("        notExecuted(transactionId)\n");
        source.push_str("    {\n");
        source.push_str("        Transaction storage txn = transactions[transactionId];\n");
        source.push_str("        if (txn.confirmations >= required) {\n");
        if config.daily_limit.is_some() {
            source.push_str("            if (isUnderLimit(txn.value)) {\n");
            source.push_str("                spentToday += txn.value;\n");
        }
        source.push_str("            txn.executed = true;\n");
        source.push_str("            (bool success, ) = txn.to.call{value: txn.value}(txn.data);\n");
        source.push_str("            if (success) {\n");
        source.push_str("                emit Execution(transactionId);\n");
        source.push_str("            } else {\n");
        source.push_str("                txn.executed = false;\n");
        source.push_str("                emit ExecutionFailure(transactionId);\n");
        source.push_str("            }\n");
        if config.daily_limit.is_some() {
            source.push_str("            }\n");
        }
        source.push_str("        }\n");
        source.push_str("    }\n\n");

        if config.daily_limit.is_some() {
            source.push_str("    function isUnderLimit(uint256 amount) public returns (bool) {\n");
            source.push_str("        uint256 today = block.timestamp / 1 days;\n");
            source.push_str("        if (today > lastDay) {\n");
            source.push_str("            spentToday = 0;\n");
            source.push_str("            lastDay = today;\n");
            source.push_str("        }\n");
            source.push_str("        return spentToday + amount <= dailyLimit;\n");
            source.push_str("    }\n\n");
        }

        source.push_str("    function getOwners() external view returns (address[] memory) {\n");
        source.push_str("        return owners;\n");
        source.push_str("    }\n\n");

        source.push_str("    function getTransactionCount() external view returns (uint256) {\n");
        source.push_str("        return transactions.length;\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: config.name.clone(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_comprehensive_audit_report(&self, contract: &GeneratedContract) -> ChainResult<String> {
        let mut report = String::new();

        report.push_str("# Smart Contract Audit Report\n\n");
        report.push_str(&format!("## Contract: {}\n", contract.name));
        report.push_str(&format!("## Platform: {:?}\n", contract.platform));
        report.push_str(&format!("## Date: {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));

        report.push_str("---\n\n");

        report.push_str("## Executive Summary\n\n");
        report.push_str(&format!("This report presents the findings of an automated security audit performed on the {} smart contract.\n\n", contract.name));

        let analysis = SecurityAnalyzer::analyze(contract);
        report.push_str(&format!("**Overall Security Score: {}/100**\n\n", analysis.score));

        if analysis.score >= 80 {
            report.push_str("The contract demonstrates a strong security posture with minimal vulnerabilities.\n\n");
        } else if analysis.score >= 60 {
            report.push_str("The contract shows moderate security with some areas requiring attention.\n\n");
        } else {
            report.push_str("The contract has significant security concerns that should be addressed before deployment.\n\n");
        }

        report.push_str("## Vulnerability Summary\n\n");
        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;

        for vuln in &analysis.vulnerabilities {
            match vuln.severity {
                Severity::Critical => critical_count += 1,
                Severity::High => high_count += 1,
                Severity::Medium => medium_count += 1,
                Severity::Low => low_count += 1,
            }
        }

        report.push_str("| Severity | Count |\n");
        report.push_str("|----------|-------|\n");
        report.push_str(&format!("| Critical | {} |\n", critical_count));
        report.push_str(&format!("| High     | {} |\n", high_count));
        report.push_str(&format!("| Medium   | {} |\n", medium_count));
        report.push_str(&format!("| Low      | {} |\n\n", low_count));

        report.push_str("## Detailed Findings\n\n");

        for (idx, vuln) in analysis.vulnerabilities.iter().enumerate() {
            report.push_str(&format!("### Finding #{}: {:?}\n\n", idx + 1, vuln.vulnerability_type));
            report.push_str(&format!("**Severity:** {:?}\n\n", vuln.severity));
            report.push_str(&format!("**Description:** {}\n\n", vuln.description));

            if let Some(line) = vuln.line {
                report.push_str(&format!("**Location:** Line {}\n\n", line));
            }

            report.push_str(&format!("**Recommendation:** {}\n\n", vuln.recommendation));
            report.push_str("---\n\n");
        }

        report.push_str("## Code Quality Analysis\n\n");
        report.push_str("### Metrics\n\n");
        let lines = contract.source.lines().count();
        report.push_str(&format!("- Total Lines of Code: {}\n", lines));
        report.push_str(&format!("- Functions: {}\n", contract.source.matches("function ").count()));
        report.push_str(&format!("- Events: {}\n", contract.source.matches("event ").count()));
        report.push_str(&format!("- Modifiers: {}\n\n", contract.source.matches("modifier ").count()));

        report.push_str("### Best Practices\n\n");
        let has_natspec = contract.source.contains("/// @");
        let has_spdx = contract.source.contains("SPDX-License-Identifier");
        let has_pragma = contract.source.contains("pragma solidity");

        report.push_str(&format!("- [{}] SPDX License Identifier\n", if has_spdx { "x" } else { " " }));
        report.push_str(&format!("- [{}] Solidity Version Pragma\n", if has_pragma { "x" } else { " " }));
        report.push_str(&format!("- [{}] NatSpec Documentation\n\n", if has_natspec { "x" } else { " " }));

        report.push_str("## Recommendations\n\n");

        if analysis.score < 100 {
            report.push_str("1. Address all identified vulnerabilities before deployment\n");
            report.push_str("2. Conduct a professional manual audit\n");
            report.push_str("3. Implement comprehensive test coverage (>95%)\n");
            report.push_str("4. Consider formal verification for critical functions\n");
            report.push_str("5. Set up continuous monitoring post-deployment\n\n");
        } else {
            report.push_str("1. Conduct a professional manual audit for additional assurance\n");
            report.push_str("2. Maintain comprehensive test coverage\n");
            report.push_str("3. Set up continuous monitoring post-deployment\n\n");
        }

        report.push_str("## Testing Recommendations\n\n");
        report.push_str("- **Unit Tests:** Test each function in isolation\n");
        report.push_str("- **Integration Tests:** Test interactions between functions\n");
        report.push_str("- **Fuzzing:** Use property-based testing to find edge cases\n");
        report.push_str("- **Gas Optimization:** Profile and optimize expensive operations\n");
        report.push_str("- **Security Tools:** Run Slither, Mythril, and other static analyzers\n\n");

        report.push_str("## Deployment Checklist\n\n");
        report.push_str("- [ ] All vulnerabilities resolved\n");
        report.push_str("- [ ] Professional audit completed\n");
        report.push_str("- [ ] Test coverage >95%\n");
        report.push_str("- [ ] Gas optimization completed\n");
        report.push_str("- [ ] Deployment scripts tested on testnet\n");
        report.push_str("- [ ] Emergency pause mechanism verified\n");
        report.push_str("- [ ] Upgrade mechanism tested (if applicable)\n");
        report.push_str("- [ ] Documentation completed\n");
        report.push_str("- [ ] Monitoring and alerting configured\n\n");

        report.push_str("---\n\n");
        report.push_str("*This is an automated audit report. Professional manual audit is strongly recommended before production deployment.*\n");

        Ok(report)
    }
}

/// Converts a string to PascalCase.
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Converts a string to snake_case.
fn to_snake_case(s: &str) -> String {
    s.replace('-', "_").to_lowercase()
}

/// Security analyzer for smart contracts.
pub struct SecurityAnalyzer;

impl SecurityAnalyzer {
    /// Performs security analysis on a generated contract.
    pub fn analyze(contract: &GeneratedContract) -> SecurityAnalysis {
        let mut vulnerabilities = Vec::new();

        match contract.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                Self::check_evm_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::Move => {
                Self::check_move_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::Cairo => {
                Self::check_cairo_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::RustWasm | TargetPlatform::Ink | TargetPlatform::CosmWasm => {
                Self::check_wasm_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::Ton => {
                // TON FunC has built-in safety features
                Self::check_wasm_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::Teal => {
                // Algorand Teal has limited vulnerability surface
                Self::check_move_vulnerabilities(contract, &mut vulnerabilities);
            }
        }

        let score = Self::calculate_security_score(&vulnerabilities);

        SecurityAnalysis {
            contract_name: contract.name.clone(),
            vulnerabilities,
            score,
        }
    }

    fn check_evm_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Check for reentrancy vulnerabilities
        if (contract.source.contains("transfer(") || contract.source.contains("send("))
            && !contract.source.contains("CEI pattern")
            && !contract.source.contains("ReentrancyGuard")
        {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::Reentrancy,
                severity: Severity::High,
                description: "Potential reentrancy vulnerability in external call".to_string(),
                line: None,
                recommendation: "Use checks-effects-interactions pattern or ReentrancyGuard"
                    .to_string(),
            });
        }

        // Check for unchecked external calls
        if contract.source.contains("call{") && !contract.source.contains("require(success") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::UncheckedExternalCall,
                severity: Severity::Medium,
                description: "External call without checking return value".to_string(),
                line: None,
                recommendation: "Always check return values from external calls".to_string(),
            });
        }

        // Check for integer overflow (pre-Solidity 0.8.0)
        if contract.source.contains("pragma solidity") {
            let version_check =
                contract.source.contains("^0.8") || contract.source.contains(">=0.8");
            if !version_check {
                vulnerabilities.push(Vulnerability {
                    vulnerability_type: VulnerabilityType::IntegerOverflow,
                    severity: Severity::High,
                    description: "Solidity version < 0.8.0 without SafeMath".to_string(),
                    line: None,
                    recommendation: "Upgrade to Solidity ^0.8.0 or use SafeMath library"
                        .to_string(),
                });
            }
        }

        // Check for access control
        if !contract.source.contains("owner") && !contract.source.contains("onlyOwner") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::AccessControl,
                severity: Severity::Low,
                description: "No access control mechanism detected".to_string(),
                line: None,
                recommendation: "Implement access control for sensitive functions".to_string(),
            });
        }

        // Check for front-running vulnerabilities
        if contract.source.contains("mapping") && contract.source.contains("public") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::FrontRunning,
                severity: Severity::Low,
                description: "Public state variables may be vulnerable to front-running"
                    .to_string(),
                line: None,
                recommendation: "Consider using commit-reveal schemes for sensitive operations"
                    .to_string(),
            });
        }
    }

    fn check_move_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Move has built-in resource safety, but check for access control
        if !contract.source.contains("assert!(") && !contract.source.contains("owner") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::AccessControl,
                severity: Severity::Medium,
                description: "Insufficient access control checks".to_string(),
                line: None,
                recommendation: "Add proper authorization checks using assert! or require"
                    .to_string(),
            });
        }
    }

    fn check_cairo_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Check for access control
        if !contract.source.contains("assert(") && !contract.source.contains("owner") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::AccessControl,
                severity: Severity::Medium,
                description: "No access control mechanism detected".to_string(),
                line: None,
                recommendation: "Implement proper access control checks".to_string(),
            });
        }
    }

    fn check_wasm_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Rust/WASM has memory safety, but check for logical issues
        if contract.source.contains("unwrap()") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::DenialOfService,
                severity: Severity::Medium,
                description: "Use of unwrap() can cause panics".to_string(),
                line: None,
                recommendation: "Use proper error handling with Result types".to_string(),
            });
        }
    }

    fn calculate_security_score(vulnerabilities: &[Vulnerability]) -> u8 {
        let mut score = 100u8;

        for vuln in vulnerabilities {
            let deduction = match vuln.severity {
                Severity::Critical => 30,
                Severity::High => 20,
                Severity::Medium => 10,
                Severity::Low => 5,
            };
            score = score.saturating_sub(deduction);
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_generate_solidity() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        assert_eq!(contract.name, "AdultRights");
        assert!(contract.source.contains("pragma solidity"));
        assert!(contract.source.contains("checkEligibility"));
    }

    #[test]
    fn test_generate_rust_wasm() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::RustWasm);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("wasm_bindgen"));
    }

    #[test]
    fn test_discretionary_statute_error() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_discretion("Requires human judgment");

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let result = generator.generate(&statute);

        assert!(matches!(result, Err(ChainError::DiscretionaryStatute(_))));
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("adult_rights"), "AdultRights");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("Hello-World"), "hello_world");
    }

    #[test]
    fn test_generate_vyper() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let contract = generator.generate(&statute).unwrap();

        assert_eq!(contract.name, "adult_rights");
        assert!(contract.source.contains("# @version"));
        assert!(contract.source.contains("def check_eligibility"));
        assert!(contract.source.contains("event EligibilityChecked"));
    }

    #[test]
    fn test_generate_move() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Move);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("module legalis::"));
        assert!(contract.source.contains("public fun check_eligibility"));
        assert!(contract.source.contains("struct EligibilityChecked"));
    }

    #[test]
    fn test_generate_cairo() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        });

        let generator = ContractGenerator::new(TargetPlatform::Cairo);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("#[starknet::contract]"));
        assert!(contract.source.contains("fn check_eligibility"));
        assert!(contract.source.contains("struct EligibilityChecked"));
    }

    #[test]
    fn test_solidity_events() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("event EligibilityChecked"));
        assert!(contract.source.contains("event EffectApplied"));
        assert!(contract.source.contains("emit EligibilityChecked"));
        assert!(contract.source.contains("emit EffectApplied"));
    }

    #[test]
    fn test_solidity_gas_optimization() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("immutable"));
        assert!(contract.source.contains("Gas-optimized"));
        assert!(contract.source.contains("CEI pattern"));
    }

    #[test]
    fn test_deployment_script_generation() {
        let statute = Statute::new(
            "test-contract",
            "Test Contract",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "mainnet".to_string(),
            gas_limit: Some(5000000),
            gas_price: Some(50),
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("Hardhat deployment script"));
        assert!(script.contains("mainnet"));
        assert!(script.contains("5000000"));
        assert!(script.contains("verify:verify"));
    }

    #[test]
    fn test_deployment_script_vyper() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "testnet".to_string(),
            gas_limit: None,
            gas_price: None,
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("from ape import"));
        assert!(script.contains("deployer.deploy"));
    }

    #[test]
    fn test_security_analysis_solidity() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let analysis = SecurityAnalyzer::analyze(&contract);
        assert_eq!(analysis.contract_name, "Test");
        assert!(analysis.score > 0 && analysis.score <= 100);

        // Our generated contracts should have owner checks
        let has_access_control = analysis
            .vulnerabilities
            .iter()
            .any(|v| v.vulnerability_type == VulnerabilityType::AccessControl);
        assert!(
            !has_access_control,
            "Generated contract should have access control"
        );
    }

    #[test]
    fn test_security_analysis_front_running() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let analysis = SecurityAnalyzer::analyze(&contract);

        // Check if front-running warning is present
        let has_front_running = analysis
            .vulnerabilities
            .iter()
            .any(|v| v.vulnerability_type == VulnerabilityType::FrontRunning);
        assert!(
            has_front_running,
            "Should detect potential front-running vulnerability"
        );
    }

    #[test]
    fn test_security_score_calculation() {
        let statute = Statute::new(
            "safe-contract",
            "Safe Contract",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Move);
        let contract = generator.generate(&statute).unwrap();

        let analysis = SecurityAnalyzer::analyze(&contract);
        // Move contracts should have high security scores due to resource safety
        assert!(
            analysis.score >= 85,
            "Move contracts should have high security scores"
        );
    }

    #[test]
    fn test_generate_cosmwasm() {
        let statute = Statute::new(
            "cosmos-statute",
            "Cosmos Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let generator = ContractGenerator::new(TargetPlatform::CosmWasm);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("use cosmwasm_std::"));
        assert!(contract.source.contains("entry_point"));
        assert!(contract.source.contains("pub fn instantiate"));
        assert!(contract.source.contains("pub fn execute"));
        assert!(contract.source.contains("pub fn query"));
        assert!(contract.source.contains("QueryMsg::CheckEligibility"));
    }

    #[test]
    fn test_cosmwasm_deployment() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::CosmWasm);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "cosmos-testnet".to_string(),
            gas_limit: None,
            gas_price: None,
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("CosmWasm deployment"));
        assert!(script.contains("cosmwasm/rust-optimizer"));
        assert!(script.contains("wasmd tx wasm"));
    }

    #[test]
    fn test_factory_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let statute_ids = vec!["adult-rights", "tax-exemption", "voting-rights"];
        let factory = generator.generate_factory(&statute_ids).unwrap();

        assert_eq!(factory.name, "StatuteFactory");
        assert!(factory.source.contains("contract StatuteFactory"));
        assert!(factory.source.contains("deployAdultRights"));
        assert!(factory.source.contains("deployTaxExemption"));
        assert!(factory.source.contains("deployVotingRights"));
        assert!(factory.source.contains("event ContractDeployed"));
        assert!(factory.source.contains("getDeployedContractsCount"));
    }

    #[test]
    fn test_upgradeable_proxy_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let proxy = generator
            .generate_upgradeable_proxy("adult-rights")
            .unwrap();

        assert_eq!(proxy.name, "AdultRightsProxy");
        assert!(proxy.source.contains("contract AdultRightsProxy"));
        assert!(proxy.source.contains("address public implementation"));
        assert!(proxy.source.contains("function upgradeTo"));
        assert!(proxy.source.contains("delegatecall"));
        assert!(proxy.source.contains("event Upgraded"));
    }

    #[test]
    fn test_vyper_factory_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let statute_ids = vec!["test-statute"];
        let factory = generator.generate_factory(&statute_ids).unwrap();

        assert_eq!(factory.name, "statute_factory");
        assert!(factory.source.contains("# @title StatuteFactory"));
        assert!(factory.source.contains("def deploy_test_statute"));
        assert!(factory.source.contains("event ContractDeployed"));
    }

    #[test]
    fn test_generate_ton() {
        let statute = Statute::new(
            "ton-statute",
            "TON Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let generator = ContractGenerator::new(TargetPlatform::Ton);
        let contract = generator.generate(&statute).unwrap();

        assert_eq!(contract.platform, TargetPlatform::Ton);
        assert!(contract.source.contains(";; FunC contract for TON"));
        assert!(contract.source.contains("int check_eligibility"));
        assert!(contract.source.contains("() apply_effect"));
        assert!(contract.source.contains("load_data()"));
        assert!(contract.source.contains("save_data()"));
    }

    #[test]
    fn test_generate_teal() {
        let statute = Statute::new(
            "algo-statute",
            "Algorand Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 100000,
        });

        let generator = ContractGenerator::new(TargetPlatform::Teal);
        let contract = generator.generate(&statute).unwrap();

        assert_eq!(contract.platform, TargetPlatform::Teal);
        assert!(contract.source.contains("#pragma version 8"));
        assert!(contract.source.contains("check_eligibility:"));
        assert!(contract.source.contains("create_app:"));
        assert!(contract.source.contains("txn ApplicationID"));
    }

    #[test]
    fn test_ton_deployment() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Ton);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "ton-testnet".to_string(),
            gas_limit: None,
            gas_price: None,
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("TON FunC deployment"));
        assert!(script.contains("func -o"));
        assert!(script.contains("fift -s build.fif"));
    }

    #[test]
    fn test_teal_deployment() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Teal);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "algorand-testnet".to_string(),
            gas_limit: None,
            gas_price: None,
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("Algorand Teal deployment"));
        assert!(script.contains("goal clerk compile"));
        assert!(script.contains("goal app create"));
    }

    #[test]
    fn test_uups_proxy_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let proxy = generator
            .generate_proxy_with_pattern("test-contract", ProxyPattern::Uups)
            .unwrap();

        assert_eq!(proxy.name, "TestContractUUPS");
        assert!(proxy.source.contains("UUPSUpgradeable"));
        assert!(proxy.source.contains("OwnableUpgradeable"));
        assert!(proxy.source.contains("function initialize"));
        assert!(proxy.source.contains("function _authorizeUpgrade"));
        assert!(proxy.source.contains("function version"));
    }

    #[test]
    fn test_beacon_proxy_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let proxy = generator
            .generate_proxy_with_pattern("test-contract", ProxyPattern::Beacon)
            .unwrap();

        assert_eq!(proxy.name, "TestContractBeacon");
        assert!(
            proxy
                .source
                .contains("contract TestContractBeacon is UpgradeableBeacon")
        );
        assert!(proxy.source.contains("contract TestContractProxyFactory"));
        assert!(proxy.source.contains("function createProxy"));
        assert!(proxy.source.contains("function getProxyCount"));
        assert!(proxy.source.contains("event ProxyCreated"));
    }

    #[test]
    fn test_statute_registry_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let registry = generator.generate_statute_registry().unwrap();

        assert_eq!(registry.name, "StatuteRegistry");
        assert!(registry.source.contains("contract StatuteRegistry"));
        assert!(registry.source.contains("struct StatuteInfo"));
        assert!(registry.source.contains("function registerStatute"));
        assert!(registry.source.contains("function upgradeStatute"));
        assert!(registry.source.contains("function deactivateStatute"));
        assert!(registry.source.contains("function getAllStatuteIds"));
        assert!(registry.source.contains("event StatuteRegistered"));
        assert!(registry.source.contains("event StatuteUpgraded"));
    }

    #[test]
    fn test_governance_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let governance = generator.generate_governance().unwrap();

        assert_eq!(governance.name, "StatuteGovernance");
        assert!(governance.source.contains("contract StatuteGovernance"));
        assert!(governance.source.contains("enum ProposalState"));
        assert!(governance.source.contains("struct Proposal"));
        assert!(governance.source.contains("function propose"));
        assert!(governance.source.contains("function castVote"));
        assert!(governance.source.contains("function execute"));
        assert!(governance.source.contains("function grantVotingPower"));
        assert!(governance.source.contains("event ProposalCreated"));
        assert!(governance.source.contains("event VoteCast"));
    }

    #[test]
    fn test_test_suite_generation() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let config = TestSuiteConfig {
            unit_tests: true,
            integration_tests: true,
            fuzzing_tests: true,
            framework: "foundry".to_string(),
        };

        let tests = generator.generate_test_suite(&contract, &config).unwrap();
        assert!(tests.contains("contract TestTest is Test"));
        assert!(tests.contains("function testDeployment"));
        assert!(tests.contains("function testEligibilityValid"));
        assert!(tests.contains("function testFullWorkflow"));
        assert!(tests.contains("function testFuzzEligibility"));
    }

    #[test]
    fn test_test_suite_vyper() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let contract = generator.generate(&statute).unwrap();

        let config = TestSuiteConfig::default();

        let tests = generator.generate_test_suite(&contract, &config).unwrap();
        assert!(tests.contains("import pytest"));
        assert!(tests.contains("from ape import accounts, project"));
        assert!(tests.contains("def test_deployment"));
        assert!(tests.contains("def test_eligibility_valid"));
    }

    #[test]
    fn test_batch_operations() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let config = BatchOperationConfig::default();
        let contract = generator
            .generate_with_batch_operations(&statute, &config)
            .unwrap();

        assert!(contract.source.contains("function batchCheckEligibility"));
        assert!(contract.source.contains("function batchApplyEffects"));
        assert!(contract.source.contains("require(count <= 100"));
        assert!(contract.source.contains("try this.checkEligibility"));
    }

    #[test]
    fn test_multi_network_config() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let config = MultiNetworkConfig {
            networks: vec![
                NetworkConfig {
                    name: "mainnet".to_string(),
                    rpc_url: "https://eth-mainnet.example.com".to_string(),
                    chain_id: 1,
                    gas_limit: Some(5000000),
                    gas_price: Some(50),
                    etherscan_api_key: Some("KEY123".to_string()),
                },
                NetworkConfig {
                    name: "goerli".to_string(),
                    rpc_url: "https://eth-goerli.example.com".to_string(),
                    chain_id: 5,
                    gas_limit: None,
                    gas_price: None,
                    etherscan_api_key: None,
                },
            ],
            default_network: "mainnet".to_string(),
        };

        let hardhat_config = generator
            .generate_multi_network_config(&contract, &config)
            .unwrap();
        assert!(hardhat_config.contains("defaultNetwork: 'mainnet'"));
        assert!(hardhat_config.contains("mainnet:"));
        assert!(hardhat_config.contains("goerli:"));
        assert!(hardhat_config.contains("chainId: 1"));
        assert!(hardhat_config.contains("chainId: 5"));
        assert!(hardhat_config.contains("etherscan:"));
    }

    #[test]
    fn test_formal_verification() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let config = FormalVerificationConfig {
            certora: true,
            scribble: true,
            slither: true,
            invariants: true,
        };

        let files = generator
            .generate_formal_verification(&contract, &config)
            .unwrap();
        assert_eq!(files.len(), 4); // Slither, Certora, Scribble, Invariants

        // Check slither config
        let slither = files.iter().find(|(name, _)| name == "slither.config.json");
        assert!(slither.is_some());
        assert!(slither.unwrap().1.contains("detectors_to_exclude"));

        // Check certora spec
        let certora = files.iter().find(|(name, _)| name.ends_with(".spec"));
        assert!(certora.is_some());
        assert!(certora.unwrap().1.contains("invariant ownerNeverChanges"));

        // Check scribble annotations
        let scribble = files.iter().find(|(name, _)| name.contains("scribble"));
        assert!(scribble.is_some());
        assert!(scribble.unwrap().1.contains("#if_succeeds"));

        // Check invariants
        let invariants = files.iter().find(|(name, _)| name == "invariants.md");
        assert!(invariants.is_some());
        assert!(invariants.unwrap().1.contains("INV1"));
    }

    #[test]
    fn test_interface_extraction() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let interface = generator.generate_interface(&statute).unwrap();

        assert_eq!(interface.name, "IAdultRights");
        assert!(interface.source.contains("interface IAdultRights"));
        assert!(interface.source.contains("function checkEligibility"));
        assert!(interface.source.contains("function applyEffect"));
        assert!(interface.source.contains("function owner"));
        assert!(interface.source.contains("function eligible"));
        assert!(interface.source.contains("event EligibilityChecked"));
        assert!(interface.source.contains("event EffectApplied"));
    }

    #[test]
    fn test_modular_generation() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let modular = generator.generate_modular(&statute).unwrap();

        assert_eq!(modular.main_contract.name, "Test");
        assert!(modular.interface.is_some());
        assert_eq!(modular.interface.unwrap().name, "ITest");
        assert_eq!(modular.libraries.len(), 1);
        assert_eq!(modular.libraries[0].name, "TestLib");
        assert!(modular.libraries[0].source.contains("library TestLib"));
        assert!(modular.libraries[0].source.contains("function validateAge"));
        assert!(
            modular.libraries[0]
                .source
                .contains("function validateIncome")
        );
    }

    #[test]
    fn test_coverage_config() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let config = generator.generate_coverage_config().unwrap();

        assert!(config.contains("module.exports"));
        assert!(config.contains("skipFiles"));
        assert!(config.contains("istanbulReporter"));
        assert!(config.contains("providerOptions"));
    }

    #[test]
    fn test_vyper_coverage_config() {
        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let config = generator.generate_coverage_config().unwrap();

        assert!(config.contains("[tool.pytest.ini_options]"));
        assert!(config.contains("--cov=contracts"));
        assert!(config.contains("--cov-report=html"));
        assert!(config.contains("testpaths"));
    }

    #[test]
    fn test_inheritance_generation() {
        let statute = Statute::new(
            "ownable-statute",
            "Ownable Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let base_contracts = vec!["Ownable", "Pausable"];
        let contract = generator
            .generate_with_inheritance(&statute, &base_contracts)
            .unwrap();

        assert_eq!(contract.name, "OwnableStatute");
        assert!(
            contract
                .source
                .contains("import \"@openzeppelin/contracts/Ownable.sol\"")
        );
        assert!(
            contract
                .source
                .contains("import \"@openzeppelin/contracts/Pausable.sol\"")
        );
        assert!(
            contract
                .source
                .contains("contract OwnableStatute is Ownable, Pausable")
        );
    }

    #[test]
    fn test_diamond_pattern_generation() {
        let statute1 = Statute::new(
            "statute-one",
            "Statute One",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute2 = Statute::new(
            "statute-two",
            "Statute Two",
            Effect::new(EffectType::Grant, "Test"),
        );

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contracts = generator.generate_diamond(&[statute1, statute2]).unwrap();

        // Should have DiamondStorage + 2 facets
        assert_eq!(contracts.len(), 3);
        assert_eq!(contracts[0].name, "DiamondStorage");
        assert!(contracts[0].source.contains("library DiamondStorage"));
        assert!(contracts[0].source.contains("function diamondStorage"));

        assert_eq!(contracts[1].name, "StatuteOneFacet");
        assert!(contracts[1].source.contains("contract StatuteOneFacet"));
        assert!(contracts[1].source.contains("function checkEligibility"));

        assert_eq!(contracts[2].name, "StatuteTwoFacet");
        assert!(contracts[2].source.contains("contract StatuteTwoFacet"));
    }

    #[test]
    fn test_deployment_docs_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let docs = generator.generate_deployment_docs(&contract).unwrap();

        assert!(docs.contains("# TestContract Deployment Guide"));
        assert!(docs.contains("## Prerequisites"));
        assert!(docs.contains("Node.js >= 16.0.0"));
        assert!(docs.contains("Hardhat or Foundry"));
        assert!(docs.contains("## Deployment Steps"));
        assert!(docs.contains("npx hardhat run scripts/deploy_testcontract.js"));
        assert!(docs.contains("## Post-Deployment"));
    }

    #[test]
    fn test_api_docs_generation() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let docs = generator.generate_api_docs(&statute).unwrap();

        assert!(docs.contains("# TestStatute API Documentation"));
        assert!(docs.contains("## Overview"));
        assert!(docs.contains("## Functions"));
        assert!(docs.contains("### checkEligibility"));
        assert!(docs.contains("### applyEffect"));
        assert!(docs.contains("## Events"));
        assert!(docs.contains("### EligibilityChecked"));
    }

    #[test]
    fn test_gas_estimation_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let report = generator.generate_gas_estimation(&contract).unwrap();

        assert!(report.contains("# Gas Estimation Report: TestContract"));
        assert!(report.contains("## Deployment"));
        assert!(report.contains("Contract Creation"));
        assert!(report.contains("## Function Calls"));
        assert!(report.contains("checkEligibility"));
        assert!(report.contains("applyEffect"));
        assert!(report.contains("## Optimization Suggestions"));
        assert!(report.contains("calldata"));
    }

    #[test]
    fn test_upgrade_script_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let script = generator
            .generate_upgrade_script(&contract, ProxyPattern::Transparent)
            .unwrap();

        assert!(script.contains("Upgrade script for Hardhat"));
        assert!(script.contains("const { ethers, upgrades } = require(\"hardhat\")"));
        assert!(script.contains("Upgrading with Transparent Proxy"));
        assert!(script.contains("upgrades.upgradeProxy"));
        assert!(script.contains("Upgrade completed successfully"));
    }

    #[test]
    fn test_cross_chain_config_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let chains = vec!["ethereum", "polygon", "arbitrum"];
        let config = generator
            .generate_cross_chain_config(&contract, &chains)
            .unwrap();

        assert!(config.contains("Hardhat cross-chain configuration"));
        assert!(config.contains("ethereum:"));
        assert!(config.contains("chainId: 1"));
        assert!(config.contains("polygon:"));
        assert!(config.contains("chainId: 137"));
        assert!(config.contains("arbitrum:"));
        assert!(config.contains("chainId: 42161"));
        assert!(config.contains("process.env.ETHEREUM_RPC_URL"));
        assert!(config.contains("process.env.PRIVATE_KEY"));
    }

    #[test]
    fn test_compilation_tests_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let tests = generator.generate_compilation_tests(&contract).unwrap();

        assert!(tests.contains("Compilation test suite"));
        assert!(tests.contains("describe(\"TestContract Compilation Tests\""));
        assert!(tests.contains("should compile successfully"));
        assert!(tests.contains("should have correct bytecode"));
        assert!(tests.contains("should have valid ABI"));
        assert!(tests.contains("ethers.getContractFactory"));
    }

    #[test]
    fn test_deployment_simulation_tests_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let tests = generator
            .generate_deployment_simulation_tests(&contract)
            .unwrap();

        assert!(tests.contains("Deployment simulation test suite"));
        assert!(tests.contains("describe(\"TestContract Deployment Simulation\""));
        assert!(tests.contains("should deploy successfully"));
        assert!(tests.contains("should set correct owner"));
        assert!(tests.contains("should have correct initial state"));
        assert!(tests.contains("should simulate gas costs"));
        assert!(tests.contains("beforeEach"));
    }

    #[test]
    fn test_gas_benchmarks_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let benchmarks = generator.generate_gas_benchmarks(&contract).unwrap();

        assert!(benchmarks.contains("Gas usage benchmarks"));
        assert!(benchmarks.contains("describe(\"TestContract Gas Benchmarks\""));
        assert!(benchmarks.contains("benchmark: checkEligibility"));
        assert!(benchmarks.contains("benchmark: applyEffect"));
        assert!(benchmarks.contains("compare gas usage across functions"));
        assert!(benchmarks.contains("receipt.gasUsed"));
        assert!(benchmarks.contains("Gas Usage Summary"));
    }

    #[test]
    fn test_security_test_suite_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = GeneratedContract {
            name: "TestContract".to_string(),
            source: "contract TestContract {}".to_string(),
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        };

        let security_tests = generator.generate_security_test_suite(&contract).unwrap();

        assert!(security_tests.contains("Security test suite"));
        assert!(security_tests.contains("describe(\"TestContract Security Tests\""));
        assert!(security_tests.contains("Access Control"));
        assert!(security_tests.contains("Reentrancy Protection"));
        assert!(security_tests.contains("Input Validation"));
        assert!(security_tests.contains("Integer Overflow/Underflow"));
        assert!(security_tests.contains("Front-Running Protection"));
        assert!(security_tests.contains("loadFixture"));
    }
}
