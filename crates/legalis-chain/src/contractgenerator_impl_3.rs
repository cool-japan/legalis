//! # ContractGenerator - generate_vyper_group Methods
//!
//! This module contains method implementations for `ContractGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::{ComparisonOp, Condition, EffectType, Statute};

use super::functions::{ChainResult, to_pascal_case, to_snake_case};
use super::types::DeploymentConfig;
use super::types_19::{GeneratedContract, TargetPlatform};

use super::contractgenerator_type::ContractGenerator;

impl ContractGenerator {
    pub fn generate_vyper(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str("# @version ^0.3.0\n");
        source.push_str(&format!("# @title {}\n", statute.title));
        source.push_str("# @notice Auto-generated from Legalis-RS\n\n");
        source.push_str("owner: public(address)\n");
        source.push_str("eligible: public(HashMap[address, bool])\n\n");
        source.push_str("event EligibilityChecked:\n");
        source.push_str("    entity: indexed(address)\n");
        source.push_str("    result: bool\n\n");
        source.push_str("event EffectApplied:\n");
        source.push_str("    beneficiary: indexed(address)\n");
        source.push_str("    effect_type: String[100]\n\n");
        source.push_str("@external\n");
        source.push_str("def __init__():\n");
        source.push_str("    self.owner = msg.sender\n\n");
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
    pub fn condition_to_vyper(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn condition_to_vyper_expr(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn generate_move(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
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
        source.push_str("    public entry fun initialize(account: &signer) {\n");
        source.push_str("        let owner_addr = signer::address_of(account);\n");
        source.push_str("        move_to(account, StatuteContract {\n");
        source.push_str("            owner: owner_addr,\n");
        source.push_str("            eligible_count: 0,\n");
        source.push_str("        });\n");
        source.push_str("    }\n\n");
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
        source
            .push_str(
                "    public entry fun apply_effect(account: &signer, beneficiary: address) acquires StatuteContract {\n",
            );
        source
            .push_str(
                "        let contract = borrow_global_mut<StatuteContract>(signer::address_of(account));\n",
            );
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
    pub fn condition_to_move(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn generate_cairo(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
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
    pub fn condition_to_cairo(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn generate_cosmwasm(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str("use cosmwasm_std::{\n");
        source.push_str("    entry_point, to_json_binary, Binary, Deps, DepsMut, Env,\n");
        source.push_str("    MessageInfo, Response, StdResult, Addr,\n");
        source.push_str("};\n");
        source.push_str("use serde::{Deserialize, Serialize};\n\n");
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("pub struct State {\n");
        source.push_str("    pub owner: Addr,\n");
        source.push_str("    pub eligible_count: u64,\n");
        source.push_str("}\n\n");
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("pub struct InstantiateMsg {}\n\n");
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("#[serde(rename_all = \"snake_case\")]\n");
        source.push_str("pub enum ExecuteMsg {\n");
        source.push_str("    ApplyEffect { beneficiary: String },\n");
        source.push_str("}\n\n");
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
    pub fn condition_to_cosmwasm(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn generate_solidity_factory(
        &self,
        statute_ids: &[&str],
    ) -> ChainResult<GeneratedContract> {
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
        source
            .push_str(
                "    function getContractsByType(string memory contractType) public view returns (address[] memory) {\n",
            );
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
    pub fn generate_vyper_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
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
    pub fn generate_solidity_proxy(&self, contract_name: &str) -> ChainResult<GeneratedContract> {
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
    pub fn generate_solidity_deployment(
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
    pub fn generate_vyper_deployment(
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
    pub fn generate_move_deployment(
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
    pub fn generate_cairo_deployment(
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
    pub fn generate_wasm_deployment(
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
    pub fn generate_ink_deployment(
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
    pub fn generate_cosmwasm_deployment(
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
        script
            .push_str(
                "  --mount type=volume,source=\"$(basename \"$(pwd)\")_cache\",target=/code/target \\\n",
            );
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
        script
            .push_str(
                "CODE_ID=$(echo $RES | jq -r '.logs[0].events[] | select(.type==\"store_code\") | .attributes[] | select(.key==\"code_id\") | .value')\n",
            );
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
    pub fn generate_ton(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str(";; FunC contract for TON\n");
        source.push_str(&format!(";; {}\n\n", statute.title));
        source.push_str("#include \"imports/stdlib.fc\";\n\n");
        source.push_str("global int owner;\n");
        source.push_str("global int eligible_count;\n\n");
        source.push_str("() load_data() impure {\n");
        source.push_str("    var ds = get_data().begin_parse();\n");
        source.push_str("    owner = ds~load_uint(256);\n");
        source.push_str("    eligible_count = ds~load_uint(64);\n");
        source.push_str("}\n\n");
        source.push_str("() save_data() impure {\n");
        source.push_str("    set_data(begin_cell()\n");
        source.push_str("        .store_uint(owner, 256)\n");
        source.push_str("        .store_uint(eligible_count, 64)\n");
        source.push_str("        .end_cell());\n");
        source.push_str("}\n\n");
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
    pub fn condition_to_ton(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn generate_teal(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str("#pragma version 8\n");
        source.push_str(&format!("// {}\n\n", statute.title));
        source.push_str("// Handle application calls\n");
        source.push_str("txn ApplicationID\n");
        source.push_str("int 0\n");
        source.push_str("==\n");
        source.push_str("bnz create_app\n\n");
        source.push_str("// Check eligibility\n");
        source.push_str("txn OnCompletion\n");
        source.push_str("int NoOp\n");
        source.push_str("==\n");
        source.push_str("bnz check_eligibility\n\n");
        source.push_str("check_eligibility:\n");
        for (idx, condition) in statute.preconditions.iter().enumerate() {
            source.push_str(&format!("    // Condition {}\n", idx + 1));
            source.push_str(&self.condition_to_teal(condition)?);
        }
        source.push_str("    int 1  // Return true\n");
        source.push_str("    return\n\n");
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
    pub fn condition_to_teal(&self, condition: &Condition) -> ChainResult<String> {
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
    pub fn generate_sway(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();
        source.push_str("contract;\n\n");
        source.push_str(&format!("// {}\n", statute.title));
        source.push_str(&format!("// Contract: {}\n\n", contract_name));
        source.push_str("use std::{\n");
        source.push_str("    auth::msg_sender,\n");
        source.push_str("    context::msg_amount,\n");
        source.push_str("};\n\n");
        source.push_str("storage {\n");
        source.push_str("    owner: Identity = Identity::Address(Address::zero()),\n");
        source.push_str("}\n\n");
        source.push_str("abi Statute {\n");
        source.push_str("    #[storage(read)]\n");
        source.push_str("    fn check_eligibility(age: u64, income: u64) -> bool;\n");
        source.push_str("    \n");
        source.push_str("    #[storage(read, write)]\n");
        source.push_str("    fn apply_effect(applicant: Identity) -> bool;\n");
        source.push_str("}\n\n");
        source.push_str("impl Statute for Contract {\n");
        source.push_str("    #[storage(read)]\n");
        source.push_str("    fn check_eligibility(age: u64, income: u64) -> bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&format!(
                "        // {}\n",
                self.condition_to_sway_comment(condition)
            ));
            source.push_str(&self.condition_to_sway(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n\n");
        source.push_str("    #[storage(read, write)]\n");
        source.push_str("    fn apply_effect(applicant: Identity) -> bool {\n");
        source
            .push_str(
                "        require(msg_sender().unwrap() == storage.owner.read(), \"Only owner can apply effect\");\n",
            );
        source.push_str(&format!("        // {}\n", statute.effect.description));
        source.push_str("        true\n");
        source.push_str("    }\n");
        source.push_str("}\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Sway,
            abi: None,
            deployment_script: None,
        })
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_sway(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!(
                    "        require(age {} {}, \"Age requirement not met\");\n",
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
                    "        require(income {} {}, \"Income requirement not met\");\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_sway(left)?;
                result.push_str(&self.condition_to_sway(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_sway_comment(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "Age {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "Income {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::And(left, right) => {
                format!(
                    "{} AND {}",
                    self.condition_to_sway_comment(left),
                    self.condition_to_sway_comment(right)
                )
            }
            _ => "Custom condition".to_string(),
        }
    }
    pub fn generate_clarity(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str(&format!(";; {}\n", statute.title));
        source.push_str(&format!(";; Contract: {}\n\n", contract_name));
        source.push_str(";; Define contract owner\n");
        source.push_str("(define-data-var owner principal tx-sender)\n\n");
        source.push_str(";; Define error codes\n");
        source.push_str("(define-constant ERR-NOT-AUTHORIZED (err u100))\n");
        source.push_str("(define-constant ERR-INVALID-PARAM (err u101))\n\n");
        source.push_str(";; Check eligibility based on conditions\n");
        source.push_str("(define-read-only (check-eligibility (age uint) (income uint))\n");
        source.push_str("  (begin\n");
        for condition in &statute.preconditions {
            source.push_str(&format!(
                "    ;; {}\n",
                self.condition_to_clarity_comment(condition)
            ));
            source.push_str(&self.condition_to_clarity(condition)?);
        }
        source.push_str("    (ok true)\n");
        source.push_str("  )\n");
        source.push_str(")\n\n");
        source.push_str(";; Apply effect (only owner)\n");
        source.push_str("(define-public (apply-effect (applicant principal))\n");
        source.push_str("  (begin\n");
        source.push_str("    (asserts! (is-eq tx-sender (var-get owner)) ERR-NOT-AUTHORIZED)\n");
        source.push_str(&format!("    ;; {}\n", statute.effect.description));
        source.push_str("    (ok true)\n");
        source.push_str("  )\n");
        source.push_str(")\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Clarity,
            abi: None,
            deployment_script: None,
        })
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_clarity(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "is-eq",
                    _ => ">=",
                };
                Ok(format!(
                    "    (asserts! ({} age u{}) ERR-INVALID-PARAM)\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "is-eq",
                    _ => ">=",
                };
                Ok(format!(
                    "    (asserts! ({} income u{}) ERR-INVALID-PARAM)\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_clarity(left)?;
                result.push_str(&self.condition_to_clarity(right)?);
                Ok(result)
            }
            _ => Ok("    ;; Custom condition\n".to_string()),
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_clarity_comment(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "Age {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "Income {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::And(left, right) => {
                format!(
                    "{} AND {}",
                    self.condition_to_clarity_comment(left),
                    self.condition_to_clarity_comment(right)
                )
            }
            _ => "Custom condition".to_string(),
        }
    }
    pub fn generate_noir(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str(&format!("// {}\n", statute.title));
        source.push_str(&format!("// Contract: {}\n\n", contract_name));
        source.push_str("use dep::std;\n\n");
        source.push_str("// Check eligibility based on private inputs\n");
        source.push_str("fn check_eligibility(\n");
        source.push_str("    age: Field,\n");
        source.push_str("    income: Field,\n");
        source.push_str(") -> pub bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&format!(
                "    // {}\n",
                self.condition_to_noir_comment(condition)
            ));
            source.push_str(&self.condition_to_noir(condition)?);
        }
        source.push_str("    true\n");
        source.push_str("}\n\n");
        source.push_str("// Main circuit\n");
        source.push_str("fn main(\n");
        source.push_str("    age: Field,\n");
        source.push_str("    income: Field,\n");
        source.push_str("    pub result: pub bool,\n");
        source.push_str(") {\n");
        source.push_str("    let eligible = check_eligibility(age, income);\n");
        source.push_str("    assert(eligible == result);\n");
        source.push_str("}\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Noir,
            abi: None,
            deployment_script: None,
        })
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_noir(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!("    assert(age {} {});\n", op, value))
            }
            Condition::Income { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!("    assert(income {} {});\n", op, value))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_noir(left)?;
                result.push_str(&self.condition_to_noir(right)?);
                Ok(result)
            }
            _ => Ok("    // Custom condition\n".to_string()),
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_noir_comment(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "Age {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "Income {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::And(left, right) => {
                format!(
                    "{} AND {}",
                    self.condition_to_noir_comment(left),
                    self.condition_to_noir_comment(right)
                )
            }
            _ => "Custom condition".to_string(),
        }
    }
    pub fn generate_leo(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();
        source.push_str(&format!("// {}\n", statute.title));
        source.push_str(&format!("// Contract: {}\n\n", contract_name));
        source.push_str("program statute.aleo {\n\n");
        source.push_str("    // Check eligibility transition\n");
        source.push_str("    transition check_eligibility(\n");
        source.push_str("        public age: u64,\n");
        source.push_str("        public income: u64\n");
        source.push_str("    ) -> bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&format!(
                "        // {}\n",
                self.condition_to_leo_comment(condition)
            ));
            source.push_str(&self.condition_to_leo(condition)?);
        }
        source.push_str("        return true;\n");
        source.push_str("    }\n\n");
        source.push_str("    // Apply effect transition\n");
        source.push_str("    transition apply_effect(public applicant: address) -> bool {\n");
        source.push_str(&format!("        // {}\n", statute.effect.description));
        source.push_str("        return true;\n");
        source.push_str("    }\n");
        source.push_str("}\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Leo,
            abi: None,
            deployment_script: None,
        })
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_leo(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!("        assert(age {} {}u64);\n", op, value))
            }
            Condition::Income { operator, value } => {
                let op = match operator {
                    ComparisonOp::GreaterOrEqual => ">=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::Equal => "==",
                    _ => ">=",
                };
                Ok(format!("        assert(income {} {}u64);\n", op, value))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_leo(left)?;
                result.push_str(&self.condition_to_leo(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_leo_comment(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "Age {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "Income {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::And(left, right) => {
                format!(
                    "{} AND {}",
                    self.condition_to_leo_comment(left),
                    self.condition_to_leo_comment(right)
                )
            }
            _ => "Custom condition".to_string(),
        }
    }
    pub fn generate_circom(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();
        source.push_str("pragma circom 2.0.0;\n\n");
        source.push_str(&format!("// {}\n", statute.title));
        source.push_str(&format!("// Circuit: {}\n\n", contract_name));
        source.push_str("template StatuteChecker() {\n");
        source.push_str("    // Input signals (private)\n");
        source.push_str("    signal input age;\n");
        source.push_str("    signal input income;\n\n");
        source.push_str("    // Output signal (public)\n");
        source.push_str("    signal output eligible;\n\n");
        source.push_str("    // Intermediate signals for conditions\n");
        let num_conditions = statute.preconditions.len();
        for i in 0..num_conditions {
            source.push_str(&format!("    signal condition_{};\n", i + 1));
        }
        source.push('\n');
        for (idx, condition) in statute.preconditions.iter().enumerate() {
            source.push_str(&format!(
                "    // Condition {}: {}\n",
                idx + 1,
                self.condition_to_circom_comment(condition)
            ));
            source.push_str(&self.condition_to_circom(condition, idx + 1)?);
        }
        source.push_str("    // All conditions must be true\n");
        if num_conditions > 0 {
            source.push_str("    signal all_conditions;\n");
            if num_conditions == 1 {
                source.push_str("    all_conditions <== condition_1;\n");
            } else {
                source.push_str("    all_conditions <== condition_1 * condition_2");
                for i in 3..=num_conditions {
                    source.push_str(&format!(" * condition_{}", i));
                }
                source.push_str(";\n");
            }
            source.push_str("    eligible <== all_conditions;\n");
        } else {
            source.push_str("    eligible <== 1;\n");
        }
        source.push_str("}\n\n");
        source.push_str("component main = StatuteChecker();\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Circom,
            abi: None,
            deployment_script: None,
        })
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_circom(&self, condition: &Condition, idx: usize) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => match operator {
                ComparisonOp::GreaterOrEqual => {
                    Ok(format!("    condition_{} <== age >= {};\n", idx, value))
                }
                ComparisonOp::LessThan => {
                    Ok(format!("    condition_{} <== age < {};\n", idx, value))
                }
                ComparisonOp::Equal => Ok(format!("    condition_{} <== age == {};\n", idx, value)),
                _ => Ok(format!("    condition_{} <== age >= {};\n", idx, value)),
            },
            Condition::Income { operator, value } => match operator {
                ComparisonOp::GreaterOrEqual => {
                    Ok(format!("    condition_{} <== income >= {};\n", idx, value))
                }
                ComparisonOp::LessThan => {
                    Ok(format!("    condition_{} <== income < {};\n", idx, value))
                }
                ComparisonOp::Equal => {
                    Ok(format!("    condition_{} <== income == {};\n", idx, value))
                }
                _ => Ok(format!("    condition_{} <== income >= {};\n", idx, value)),
            },
            _ => Ok(format!(
                "    condition_{} <== 1; // Custom condition\n",
                idx
            )),
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn condition_to_circom_comment(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "Age {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "Income {} {}",
                    match operator {
                        ComparisonOp::GreaterOrEqual => ">=",
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::Equal => "==",
                        _ => ">=",
                    },
                    value
                )
            }
            Condition::And(left, right) => {
                format!(
                    "{} AND {}",
                    self.condition_to_circom_comment(left),
                    self.condition_to_circom_comment(right)
                )
            }
            _ => "Custom condition".to_string(),
        }
    }
    pub fn generate_zksync_era(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();
        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!("/// @title {}\n", statute.title));
        source.push_str("/// @notice Auto-generated for zkSync Era (zkEVM L2)\n");
        source.push_str("/// @dev Optimized for zkSync Era with custom gas metering\n");
        source.push_str(&format!("contract {} {{\n", contract_name));
        source.push_str("    // zkSync Era specific optimizations\n");
        source.push_str("    event EligibilityChecked(address indexed entity, bool result);\n");
        source.push_str(
            "    event EffectApplied(address indexed beneficiary, string effectType);\n\n",
        );
        source.push_str("    address public immutable owner;\n");
        source.push_str("    mapping(address => bool) public eligible;\n\n");
        source.push_str("    constructor() {\n");
        source.push_str("        owner = msg.sender;\n");
        source.push_str("    }\n\n");
        source.push_str(
            "    function checkEligibility(uint256 age, uint256 income) public returns (bool) {\n",
        );
        source.push_str("        // zkSync Era gas optimizations\n");
        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_solidity(condition)?);
        }
        source.push_str("        emit EligibilityChecked(msg.sender, true);\n");
        source.push_str("        return true;\n");
        source.push_str("    }\n\n");
        source.push_str("    function apply(address beneficiary) public returns (bool) {\n");
        source.push_str("        require(msg.sender == owner, \"Only owner\");\n");
        source.push_str(&format!(
            "        emit EffectApplied(beneficiary, \"{}\");\n",
            statute.effect.effect_type
        ));
        source.push_str("        return true;\n");
        source.push_str("    }\n");
        source.push_str("}\n");
        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::ZkSyncEra,
            abi: None,
            deployment_script: None,
        })
    }
}
