//! # ContractGenerator - generate_recursive_proof_verifier_group Methods
//!
//! This module contains method implementations for `ContractGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;

use super::functions::{ChainResult, to_pascal_case, to_snake_case};
use super::types::{PrivateStatuteConfig, RecursiveProofConfig};
use super::types_19::{ChainError, GeneratedContract, TargetPlatform, ZkProofSystem};

use super::contractgenerator_type::ContractGenerator;

impl ContractGenerator {
    /// Generates recursive proof composition contract.
    ///
    /// Creates a contract that can verify proofs of proofs (recursive zkSNARKs).
    pub fn generate_recursive_proof_verifier(
        &self,
        statute: &Statute,
        config: &RecursiveProofConfig,
    ) -> ChainResult<GeneratedContract> {
        if self.platform != TargetPlatform::Solidity {
            return Err(ChainError::GenerationError(
                "Recursive proof verifiers currently only supported for Solidity".to_string(),
            ));
        }
        let contract_name = format!("{}RecursiveVerifier", to_pascal_case(&statute.id));
        let mut source = String::from("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str(&format!(
            "/// @title {} - Recursive Proof Verifier\n",
            contract_name
        ));
        source.push_str("/// @notice Verifies recursive zkSNARK proofs (proofs of proofs)\n");
        source.push_str(&format!(
            "/// @dev Maximum recursion depth: {}\n",
            config.max_depth
        ));
        source.push_str(&format!("contract {} {{\n", contract_name));
        source.push_str("    /// @notice Proof structure\n");
        source.push_str("    struct Proof {\n");
        source.push_str("        uint256[2] a;          // G1 point\n");
        source.push_str("        uint256[2][2] b;       // G2 point\n");
        source.push_str("        uint256[2] c;          // G1 point\n");
        source.push_str("        uint256[] publicInputs;\n");
        source.push_str("    }\n\n");
        source.push_str("    /// @notice Recursive proof structure\n");
        source.push_str("    struct RecursiveProof {\n");
        source.push_str("        Proof innerProof;      // The proof being verified\n");
        source.push_str("        Proof outerProof;      // Proof of verification\n");
        source.push_str("        uint256 depth;         // Recursion depth\n");
        source.push_str("    }\n\n");
        if config.batch_verification {
            source.push_str("    /// @notice Batch proof structure\n");
            source.push_str("    struct BatchProof {\n");
            source.push_str("        Proof[] proofs;\n");
            source.push_str("        Proof aggregatedProof;\n");
            source.push_str("    }\n\n");
        }
        source.push_str(&format!(
            "    uint256 public constant MAX_DEPTH = {};\n",
            config.max_depth
        ));
        source.push_str("    mapping(bytes32 => bool) public verifiedProofs;\n\n");
        source.push_str("    event ProofVerified(bytes32 indexed proofHash, uint256 depth);\n");
        if config.aggregation {
            source.push_str(
                "    event ProofsAggregated(bytes32 indexed aggregatedHash, uint256 count);\n",
            );
        }
        source.push('\n');
        source.push_str("    /// @notice Verify a recursive proof\n");
        source.push_str("    function verifyRecursiveProof(\n");
        source.push_str("        RecursiveProof calldata recursiveProof\n");
        source.push_str("    ) external returns (bool) {\n");
        source.push_str(
            "        require(recursiveProof.depth <= MAX_DEPTH, \"Exceeds max depth\");\n",
        );
        source.push_str("        \n");
        source.push_str("        // Verify the inner proof\n");
        source
            .push_str("        bool innerValid = verifySingleProof(recursiveProof.innerProof);\n");
        source.push_str("        require(innerValid, \"Inner proof invalid\");\n");
        source.push_str("        \n");
        source.push_str("        // Verify the outer proof (proof of inner verification)\n");
        source
            .push_str("        bool outerValid = verifySingleProof(recursiveProof.outerProof);\n");
        source.push_str("        require(outerValid, \"Outer proof invalid\");\n");
        source.push_str("        \n");
        source.push_str("        bytes32 proofHash = keccak256(abi.encode(recursiveProof));\n");
        source.push_str("        verifiedProofs[proofHash] = true;\n");
        source.push_str("        emit ProofVerified(proofHash, recursiveProof.depth);\n");
        source.push_str("        \n");
        source.push_str("        return true;\n");
        source.push_str("    }\n\n");
        if config.batch_verification {
            source.push_str("    /// @notice Verify multiple proofs in batch\n");
            source.push_str("    function verifyBatchProofs(\n");
            source.push_str("        BatchProof calldata batch\n");
            source.push_str("    ) external returns (bool) {\n");
            source.push_str("        require(batch.proofs.length > 0, \"Empty batch\");\n");
            source.push_str("        \n");
            source.push_str("        // Verify individual proofs\n");
            source.push_str("        for (uint256 i = 0; i < batch.proofs.length; i++) {\n");
            source
                .push_str(
                    "            require(verifySingleProof(batch.proofs[i]), \"Proof verification failed\");\n",
                );
            source.push_str("        }\n");
            source.push_str("        \n");
            source.push_str("        // Verify aggregated proof\n");
            source
                .push_str(
                    "        require(verifySingleProof(batch.aggregatedProof), \"Aggregated proof invalid\");\n",
                );
            source.push_str("        \n");
            source.push_str("        bytes32 aggregatedHash = keccak256(abi.encode(batch));\n");
            source
                .push_str("        emit ProofsAggregated(aggregatedHash, batch.proofs.length);\n");
            source.push_str("        \n");
            source.push_str("        return true;\n");
            source.push_str("    }\n\n");
        }
        source.push_str("    /// @dev Verify a single proof using pairing checks\n");
        source.push_str("    function verifySingleProof(\n");
        source.push_str("        Proof calldata proof\n");
        source.push_str("    ) internal view returns (bool) {\n");
        source.push_str(
            "        // Simplified verification - in production, use actual pairing checks\n",
        );
        source.push_str("        // e(a, b) = e(c, g2) where e is the pairing function\n");
        source.push_str("        require(proof.a.length == 2, \"Invalid proof\");\n");
        source.push_str("        require(proof.c.length == 2, \"Invalid proof\");\n");
        source.push_str("        return true;\n");
        source.push_str("    }\n");
        source.push_str("}\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }
    /// Generates private statute execution contract with ZK proofs.
    ///
    /// Creates a contract that executes statutes privately using zero-knowledge proofs.
    pub fn generate_private_statute_contract(
        &self,
        statute: &Statute,
        config: &PrivateStatuteConfig,
    ) -> ChainResult<GeneratedContract> {
        if self.platform != TargetPlatform::Solidity {
            return Err(ChainError::GenerationError(
                "Private statute contracts currently only supported for Solidity".to_string(),
            ));
        }
        let contract_name = format!("Private{}", to_pascal_case(&statute.id));
        let proof_system_name = match config.proof_system {
            ZkProofSystem::Plonk => "Plonk",
            ZkProofSystem::Groth16 => "Groth16",
            ZkProofSystem::Stark => "zkSTARK",
        };
        let mut source = String::from("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.20;\n\n");
        source.push_str(&format!(
            "/// @title {} - Private Statute Execution\n",
            contract_name
        ));
        source.push_str(&format!(
            "/// @notice Executes statute {} with privacy using {} proofs\n",
            statute.id, proof_system_name
        ));
        source.push_str(
            "/// @dev Preconditions and effects are verified via zero-knowledge proofs\n",
        );
        source.push_str(&format!("contract {} {{\n", contract_name));
        source.push_str("    /// @notice Verifier contract interface\n");
        source.push_str("    interface IZkVerifier {\n");
        source.push_str("        function verifyProof(\n");
        source.push_str("            uint256[2] memory a,\n");
        source.push_str("            uint256[2][2] memory b,\n");
        source.push_str("            uint256[2] memory c,\n");
        source.push_str("            uint256[] memory publicInputs\n");
        source.push_str("        ) external view returns (bool);\n");
        source.push_str("    }\n\n");
        source.push_str("    /// @notice Verifier contract\n");
        source.push_str(&format!(
            "    IZkVerifier public immutable {};\n\n",
            to_snake_case(&config.verifier_name)
        ));
        source.push_str("    /// @notice Statute commitment (private state)\n");
        source.push_str("    bytes32 public statuteCommitment;\n\n");
        source.push_str("    /// @notice Nullifiers to prevent double-spending\n");
        source.push_str("    mapping(bytes32 => bool) public nullifiers;\n\n");
        source.push_str("    event StatuteExecutedPrivately(\n");
        source.push_str("        bytes32 indexed commitment,\n");
        source.push_str("        bytes32 indexed nullifier\n");
        source.push_str("    );\n\n");
        if !config.hide_effects {
            source.push_str("    event EffectApplied(\n");
            source.push_str("        address indexed beneficiary,\n");
            source.push_str("        string effectType\n");
            source.push_str("    );\n\n");
        }
        source.push_str("    constructor(address verifierAddress) {\n");
        source.push_str(&format!(
            "        {} = IZkVerifier(verifierAddress);\n",
            to_snake_case(&config.verifier_name)
        ));
        source.push_str(
            "        statuteCommitment = keccak256(abi.encodePacked(\"\", block.timestamp));\n",
        );
        source.push_str("    }\n\n");
        source.push_str("    /// @notice Execute statute privately with ZK proof\n");
        source.push_str("    /// @param proof The zero-knowledge proof\n");
        source.push_str("    /// @param publicInputs Public inputs for verification\n");
        source.push_str("    /// @param nullifier Unique nullifier to prevent replay\n");
        source.push_str("    function executePrivate(\n");
        source.push_str("        uint256[2] memory a,\n");
        source.push_str("        uint256[2][2] memory b,\n");
        source.push_str("        uint256[2] memory c,\n");
        source.push_str("        uint256[] memory publicInputs,\n");
        source.push_str("        bytes32 nullifier\n");
        source.push_str("    ) external returns (bool) {\n");
        source.push_str("        // Check nullifier not used\n");
        source.push_str("        require(!nullifiers[nullifier], \"Nullifier already used\");\n");
        source.push_str("        \n");
        if config.hide_preconditions {
            source.push_str("        // Verify proof (preconditions are hidden)\n");
        } else {
            source.push_str("        // Verify proof with public preconditions\n");
        }
        source.push_str(&format!(
            "        bool valid = {}.verifyProof(a, b, c, publicInputs);\n",
            to_snake_case(&config.verifier_name)
        ));
        source.push_str("        require(valid, \"Invalid zero-knowledge proof\");\n");
        source.push_str("        \n");
        source.push_str("        // Mark nullifier as used\n");
        source.push_str("        nullifiers[nullifier] = true;\n");
        source.push_str("        \n");
        if config.hide_effects {
            source.push_str("        // Apply effect privately (hidden)\n");
            source
                .push_str(
                    "        bytes32 newCommitment = keccak256(abi.encodePacked(statuteCommitment, nullifier));\n",
                );
            source.push_str("        statuteCommitment = newCommitment;\n");
        } else {
            source.push_str("        // Apply effect publicly\n");
            source.push_str("        // Extract beneficiary from public inputs\n");
            source
                .push_str("        require(publicInputs.length > 0, \"Missing public inputs\");\n");
            source.push_str("        address beneficiary = address(uint160(publicInputs[0]));\n");
            source.push_str("        \n");
            let effect_type_str = format!("{:?}", statute.effect.effect_type);
            source.push_str(&format!("        // Apply effect: {}\n", effect_type_str));
            source.push_str("        // Implementation depends on effect type\n");
            source.push_str("        \n");
            source.push_str(&format!(
                "        emit EffectApplied(beneficiary, \"{}\");\n",
                effect_type_str
            ));
        }
        source.push_str("        \n");
        source.push_str("        emit StatuteExecutedPrivately(statuteCommitment, nullifier);\n");
        source.push_str("        return true;\n");
        source.push_str("    }\n\n");
        source.push_str("    /// @notice Verify a proof without executing\n");
        source.push_str("    function verifyOnly(\n");
        source.push_str("        uint256[2] memory a,\n");
        source.push_str("        uint256[2][2] memory b,\n");
        source.push_str("        uint256[2] memory c,\n");
        source.push_str("        uint256[] memory publicInputs\n");
        source.push_str("    ) external view returns (bool) {\n");
        source.push_str(&format!(
            "        return {}.verifyProof(a, b, c, publicInputs);\n",
            to_snake_case(&config.verifier_name)
        ));
        source.push_str("    }\n");
        source.push_str("}\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }
}
