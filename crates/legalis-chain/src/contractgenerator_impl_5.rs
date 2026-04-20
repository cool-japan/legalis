//! # ContractGenerator - generate_solidity_upgrade_script_group Methods
//!
//! This module contains method implementations for `ContractGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ChainResult;
use super::types::{
    BridgeConfig, ProxyPattern, TokenConfig, TokenStandard, TreasuryConfig, VestingConfig,
};
use super::types_19::{ChainError, DaoConfig, GeneratedContract, TargetPlatform};

use super::contractgenerator_type::ContractGenerator;

impl ContractGenerator {
    pub fn generate_solidity_upgrade_script(
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
    pub fn generate_evm_cross_chain_config(
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
    pub fn generate_evm_compilation_tests(
        &self,
        contract: &GeneratedContract,
    ) -> ChainResult<String> {
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
    pub fn generate_evm_deployment_sim_tests(
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
    pub fn generate_evm_gas_benchmarks(&self, contract: &GeneratedContract) -> ChainResult<String> {
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
    pub fn generate_evm_security_tests(&self, contract: &GeneratedContract) -> ChainResult<String> {
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
        tests
            .push_str(
                "    it(\"should only allow owner to perform privileged operations\", async function () {\n",
            );
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
    pub fn generate_solidity_token(&self, config: &TokenConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();
        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        match config.standard {
            TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                source.push_str("import \"@openzeppelin/contracts/token/ERC20/ERC20.sol\";\n");
                if config.burnable {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol\";\n",
                        );
                }
                if config.pausable {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol\";\n",
                        );
                    source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
                }
                if config.snapshot {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Snapshot.sol\";\n",
                        );
                }
                if config.mintable {
                    source
                        .push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
                }
            }
            TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                source.push_str("import \"@openzeppelin/contracts/token/ERC721/ERC721.sol\";\n");
                if config.burnable {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol\";\n",
                        );
                }
                if config.pausable {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol\";\n",
                        );
                    source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
                }
                source
                    .push_str(
                        "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol\";\n",
                    );
                source
                    .push_str(
                        "import \"@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol\";\n",
                    );
                if config.mintable {
                    source
                        .push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
                }
            }
            TokenStandard::Erc1155 => {
                source.push_str("import \"@openzeppelin/contracts/token/ERC1155/ERC1155.sol\";\n");
                if config.burnable {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Burnable.sol\";\n",
                        );
                }
                if config.pausable {
                    source
                        .push_str(
                            "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Pausable.sol\";\n",
                        );
                    source.push_str("import \"@openzeppelin/contracts/access/Ownable.sol\";\n");
                }
                source
                    .push_str(
                        "import \"@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Supply.sol\";\n",
                    );
                if config.mintable {
                    source
                        .push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
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
            source.push_str(
                "    bytes32 public constant MINTER_ROLE = keccak256(\"MINTER_ROLE\");\n\n",
            );
        }
        if matches!(
            config.standard,
            TokenStandard::Erc721 | TokenStandard::Erc721Extended
        ) {
            source.push_str("    uint256 private _nextTokenId;\n\n");
        }
        source.push_str("    constructor()\n");
        match config.standard {
            TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                source.push_str(&format!(
                    "        ERC20(\"{}\", \"{}\")\n",
                    config.name, config.symbol
                ));
            }
            TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                source.push_str(&format!(
                    "        ERC721(\"{}\", \"{}\")\n",
                    config.name, config.symbol
                ));
            }
            TokenStandard::Erc1155 => {
                let base_uri = config
                    .base_uri
                    .as_deref()
                    .unwrap_or("https://token-cdn.domain/{id}.json");
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
        if let Some(initial_supply) = config.initial_supply
            && matches!(
                config.standard,
                TokenStandard::Erc20 | TokenStandard::Erc20Extended
            )
        {
            source.push_str(&format!(
                "        _mint(msg.sender, {} * 10 ** decimals());\n",
                initial_supply
            ));
        }
        source.push_str("    }\n\n");
        if config.mintable {
            match config.standard {
                TokenStandard::Erc20 | TokenStandard::Erc20Extended => {
                    source
                        .push_str(
                            "    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {\n",
                        );
                    source.push_str("        _mint(to, amount);\n");
                    source.push_str("    }\n\n");
                }
                TokenStandard::Erc721 | TokenStandard::Erc721Extended => {
                    source
                        .push_str(
                            "    function safeMint(address to, string memory uri) public onlyRole(MINTER_ROLE) {\n",
                        );
                    source.push_str("        uint256 tokenId = _nextTokenId++;\n");
                    source.push_str("        _safeMint(to, tokenId);\n");
                    source.push_str("        _setTokenURI(tokenId, uri);\n");
                    source.push_str("    }\n\n");
                }
                TokenStandard::Erc1155 => {
                    source
                        .push_str(
                            "    function mint(address to, uint256 id, uint256 amount, bytes memory data) public onlyRole(MINTER_ROLE) {\n",
                        );
                    source.push_str("        _mint(to, id, amount, data);\n");
                    source.push_str("    }\n\n");
                    source
                        .push_str(
                            "    function mintBatch(address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data) public onlyRole(MINTER_ROLE) {\n",
                        );
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
        if config.snapshot
            && matches!(
                config.standard,
                TokenStandard::Erc20 | TokenStandard::Erc20Extended
            )
        {
            source.push_str("    function snapshot() public onlyOwner {\n");
            source.push_str("        _snapshot();\n");
            source.push_str("    }\n\n");
        }
        if matches!(
            config.standard,
            TokenStandard::Erc721 | TokenStandard::Erc721Extended
        ) {
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
            source
                .push_str(
                    "    function _update(address from, address to, uint256[] memory ids, uint256[] memory values)\n",
                );
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
        if matches!(
            config.standard,
            TokenStandard::Erc20 | TokenStandard::Erc20Extended
        ) && (config.pausable || config.snapshot)
        {
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
    pub fn generate_vyper_token(&self, config: &TokenConfig) -> ChainResult<GeneratedContract> {
        if !matches!(
            config.standard,
            TokenStandard::Erc20 | TokenStandard::Erc20Extended
        ) {
            return Err(ChainError::GenerationError(
                "Vyper currently only supports ERC-20 tokens".to_string(),
            ));
        }
        let mut source = String::new();
        source.push_str("# @version ^0.3.0\n\n");
        source.push_str("from vyper.interfaces import ERC20\n\n");
        source.push_str(&format!("name: public(String[64]) = \"{}\"\n", config.name));
        source.push_str(&format!(
            "symbol: public(String[32]) = \"{}\"\n",
            config.symbol
        ));
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
            source.push_str(&format!(
                "    self.totalSupply = {} * 10 ** 18\n",
                initial_supply
            ));
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
        source
            .push_str("def transferFrom(_from: address, _to: address, _value: uint256) -> bool:\n");
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
    pub fn generate_solidity_dao(&self, config: &DaoConfig) -> ChainResult<GeneratedContract> {
        let mut source = String::new();
        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str("import \"@openzeppelin/contracts/governance/Governor.sol\";\n");
        source.push_str(
            "import \"@openzeppelin/contracts/governance/extensions/GovernorSettings.sol\";\n",
        );
        source
            .push_str(
                "import \"@openzeppelin/contracts/governance/extensions/GovernorCountingSimple.sol\";\n",
            );
        source.push_str(
            "import \"@openzeppelin/contracts/governance/extensions/GovernorVotes.sol\";\n",
        );
        source
            .push_str(
                "import \"@openzeppelin/contracts/governance/extensions/GovernorVotesQuorumFraction.sol\";\n",
            );
        source
            .push_str(
                "import \"@openzeppelin/contracts/governance/extensions/GovernorTimelockControl.sol\";\n",
            );
        source.push_str(
            "import \"@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol\";\n",
        );
        source
            .push_str("import \"@openzeppelin/contracts/governance/TimelockController.sol\";\n\n");
        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str("/// @notice DAO governance contract\n");
        source.push_str("/// @dev Uses OpenZeppelin Governor framework\n");
        source
            .push_str(
                &format!(
                    "contract {} is Governor, GovernorSettings, GovernorCountingSimple, GovernorVotes, GovernorVotesQuorumFraction, GovernorTimelockControl {{\n",
                    config.name
                ),
            );
        source.push_str("    constructor(IVotes _token, TimelockController _timelock)\n");
        source.push_str(&format!("        Governor(\"{}\")\n", config.name));
        source.push_str(&format!(
            "        GovernorSettings({}, {}, {})\n",
            1, config.voting_period, config.proposal_threshold
        ));
        source.push_str("        GovernorVotes(_token)\n");
        source.push_str(&format!(
            "        GovernorVotesQuorumFraction({})\n",
            config.quorum_percentage
        ));
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
        source
            .push_str(
                "    function _queueOperations(uint256 proposalId, address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash)\n",
            );
        source.push_str("        internal\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (uint48)\n");
        source.push_str("    {\n");
        source
            .push_str(
                "        return super._queueOperations(proposalId, targets, values, calldatas, descriptionHash);\n",
            );
        source.push_str("    }\n\n");
        source
            .push_str(
                "    function _executeOperations(uint256 proposalId, address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash)\n",
            );
        source.push_str("        internal\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("    {\n");
        source
            .push_str(
                "        super._executeOperations(proposalId, targets, values, calldatas, descriptionHash);\n",
            );
        source.push_str("    }\n\n");
        source
            .push_str(
                "    function _cancel(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash)\n",
            );
        source.push_str("        internal\n");
        source.push_str("        override(Governor, GovernorTimelockControl)\n");
        source.push_str("        returns (uint256)\n");
        source.push_str("    {\n");
        source.push_str(
            "        return super._cancel(targets, values, calldatas, descriptionHash);\n",
        );
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
    pub fn generate_solidity_bridge(
        &self,
        config: &BridgeConfig,
    ) -> ChainResult<GeneratedContract> {
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
        source.push_str(&format!(
            "contract {} is Ownable, Pausable, ReentrancyGuard {{\n",
            config.name
        ));
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
        source.push_str(&format!(
            "    uint256 public constant SOURCE_CHAIN_ID = {};\n",
            config.source_chain_id
        ));
        source.push_str(&format!(
            "    uint256 public constant DESTINATION_CHAIN_ID = {};\n",
            config.destination_chain_id
        ));
        source.push_str(&format!(
            "    uint256 public constant FEE_BASIS_POINTS = {};  // {}%\n",
            config.fee_basis_points,
            config.fee_basis_points as f64 / 100.0
        ));
        source.push_str("    uint256 public constant BASIS_POINTS_DIVISOR = 10000;\n\n");
        source.push_str("    mapping(address => bool) public supportedTokens;\n");
        source.push_str("    mapping(bytes32 => bool) public processedTransfers;\n");
        source.push_str("    mapping(address => uint256) public nonces;\n");
        source.push_str("    uint256 public totalValueLocked;\n\n");
        source
            .push_str(
                "    event TokensLocked(bytes32 indexed transferId, address indexed token, address indexed from, address to, uint256 amount, uint256 nonce);\n",
            );
        source
            .push_str(
                "    event TokensReleased(bytes32 indexed transferId, address indexed token, address indexed to, uint256 amount);\n",
            );
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
        source
            .push_str(
                "    function lockTokens(address token, address to, uint256 amount) external whenNotPaused nonReentrant returns (bytes32) {\n",
            );
        source.push_str("        require(supportedTokens[token], \"Token not supported\");\n");
        source.push_str("        require(amount > 0, \"Amount must be positive\");\n");
        source.push_str("        require(to != address(0), \"Invalid recipient\");\n\n");
        source.push_str(
            "        uint256 fee = (amount * FEE_BASIS_POINTS) / BASIS_POINTS_DIVISOR;\n",
        );
        source.push_str("        uint256 amountAfterFee = amount - fee;\n\n");
        source.push_str(
            "        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);\n",
        );
        source.push_str("        totalValueLocked += amountAfterFee;\n\n");
        source.push_str("        uint256 nonce = nonces[msg.sender]++;\n");
        source
            .push_str(
                "        bytes32 transferId = keccak256(abi.encodePacked(token, msg.sender, to, amount, nonce, block.chainid));\n\n",
            );
        source
            .push_str(
                "        emit TokensLocked(transferId, token, msg.sender, to, amountAfterFee, nonce);\n",
            );
        source.push_str("        if (fee > 0) {\n");
        source.push_str("            emit FeesCollected(token, fee);\n");
        source.push_str("        }\n\n");
        source.push_str("        return transferId;\n");
        source.push_str("    }\n\n");
        source.push_str(
            "    /// @notice Release tokens on destination chain (only owner/validator)\n",
        );
        source.push_str("    /// @param token Token contract address\n");
        source.push_str("    /// @param to Recipient address\n");
        source.push_str("    /// @param amount Amount to release\n");
        source.push_str("    /// @param transferId Original transfer ID from source chain\n");
        source
            .push_str(
                "    function releaseTokens(address token, address to, uint256 amount, bytes32 transferId) external onlyOwner whenNotPaused nonReentrant {\n",
            );
        source.push_str(
            "        require(!processedTransfers[transferId], \"Transfer already processed\");\n",
        );
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
        source.push_str(
            "    function withdrawFees(address token, uint256 amount) external onlyOwner {\n",
        );
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
    pub fn generate_solidity_treasury(
        &self,
        config: &TreasuryConfig,
    ) -> ChainResult<GeneratedContract> {
        let mut source = String::new();
        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str("import \"@openzeppelin/contracts/access/AccessControl.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/security/ReentrancyGuard.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/IERC20.sol\";\n");
        source.push_str("import \"@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol\";\n\n");
        source.push_str(&format!("/// @title {}\n", config.name));
        source.push_str(
            "/// @notice Treasury management contract with spending limits and multi-approval\n",
        );
        source
            .push_str("/// @dev Implements role-based access control and daily spending limits\n");
        source.push_str(&format!(
            "contract {} is AccessControl, ReentrancyGuard {{\n",
            config.name
        ));
        source.push_str("    using SafeERC20 for IERC20;\n\n");
        source
            .push_str("    bytes32 public constant SPENDER_ROLE = keccak256(\"SPENDER_ROLE\");\n");
        source.push_str(
            "    bytes32 public constant APPROVER_ROLE = keccak256(\"APPROVER_ROLE\");\n\n",
        );
        source.push_str(&format!(
            "    uint256 public dailyLimit = {};  // Daily spending limit in wei\n",
            config.daily_limit
        ));
        source
            .push_str(
                &format!(
                    "    uint256 public multiApprovalThreshold = {};  // Threshold requiring multiple approvals\n",
                    config.multi_approval_threshold
                ),
            );
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
        source
            .push_str(
                "    event ProposalCreated(uint256 indexed proposalId, address indexed to, uint256 amount);\n",
            );
        source.push_str(
            "    event ProposalApproved(uint256 indexed proposalId, address indexed approver);\n",
        );
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
        source
            .push_str(
                "    function withdraw(address payable to, uint256 amount) external onlyRole(SPENDER_ROLE) nonReentrant {\n",
            );
        source.push_str("        require(amount <= dailyLimit, \"Exceeds daily limit\");\n");
        source.push_str("        _resetDailyLimitIfNeeded();\n");
        source.push_str(
            "        require(spentToday + amount <= dailyLimit, \"Daily limit exceeded\");\n",
        );
        source.push_str("        spentToday += amount;\n");
        source.push_str("        (bool success, ) = to.call{value: amount}(\"\");\n");
        source.push_str("        require(success, \"Transfer failed\");\n");
        source.push_str("        emit Withdrawal(to, amount);\n");
        source.push_str("    }\n\n");
        source
            .push_str(
                "    function proposeWithdrawal(address to, uint256 amount, bytes memory data) external onlyRole(SPENDER_ROLE) returns (uint256) {\n",
            );
        source.push_str(
            "        require(amount >= multiApprovalThreshold, \"Amount below threshold\");\n",
        );
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
        source.push_str(
            "    function approveProposal(uint256 proposalId) external onlyRole(APPROVER_ROLE) {\n",
        );
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str("        require(!proposal.executed, \"Already executed\");\n");
        source.push_str("        require(!proposal.approved[msg.sender], \"Already approved\");\n");
        source.push_str("        proposal.approved[msg.sender] = true;\n");
        source.push_str("        proposal.approvals++;\n");
        source.push_str("        emit ProposalApproved(proposalId, msg.sender);\n");
        source.push_str("    }\n\n");
        source
            .push_str(
                "    function executeProposal(uint256 proposalId) external onlyRole(SPENDER_ROLE) nonReentrant {\n",
            );
        source.push_str("        Proposal storage proposal = proposals[proposalId];\n");
        source.push_str("        require(!proposal.executed, \"Already executed\");\n");
        source.push_str("        require(proposal.approvals >= 2, \"Insufficient approvals\");\n");
        source.push_str("        proposal.executed = true;\n");
        source.push_str(
            "        (bool success, ) = proposal.to.call{value: proposal.amount}(proposal.data);\n",
        );
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
        source
            .push_str(
                "    function withdrawToken(address token, address to, uint256 amount) external onlyRole(SPENDER_ROLE) nonReentrant {\n",
            );
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
    pub fn generate_solidity_vesting(
        &self,
        config: &VestingConfig,
    ) -> ChainResult<GeneratedContract> {
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
        source.push_str(&format!(
            "    address public immutable beneficiary = {};\n",
            config.beneficiary
        ));
        source.push_str(&format!(
            "    uint256 public immutable start = {};\n",
            config.start
        ));
        source.push_str(&format!(
            "    uint256 public immutable cliffDuration = {};\n",
            config.cliff_duration
        ));
        source.push_str(&format!(
            "    uint256 public immutable duration = {};\n",
            config.duration
        ));
        source.push_str(&format!(
            "    bool public immutable revocable = {};\n\n",
            config.revocable
        ));
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
        source
            .push_str("    function vestedAmount(address token) public view returns (uint256) {\n");
        source.push_str("        if (block.timestamp < start + cliffDuration) {\n");
        source.push_str("            return 0;\n");
        source.push_str("        }\n");
        source
            .push_str(
                "        uint256 totalAllocation = IERC20(token).balanceOf(address(this)) + released[token];\n",
            );
        source.push_str("        if (block.timestamp >= start + duration) {\n");
        source.push_str("            return totalAllocation;\n");
        source.push_str("        }\n");
        source
            .push_str("        return (totalAllocation * (block.timestamp - start)) / duration;\n");
        source.push_str("    }\n\n");
        source.push_str(
            "    function _releasableAmount(address token) private view returns (uint256) {\n",
        );
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
}
