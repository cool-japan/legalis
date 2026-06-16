//! Parameterized contract-template library.
//!
//! Part of the `composition` module. A [`ContractTemplate`] is a named source
//! skeleton with a typed parameter list and `{{placeholder}}` substitution slots.
//! Rendering validates that every required parameter is supplied, that supplied
//! values satisfy each parameter's [`ParamKind`], and that no unknown parameters
//! were passed — then expands the body into finished source.
//!
//! [`TemplateLibrary`] holds a collection of templates and ships a curated set of
//! production-ready EVM building blocks via [`TemplateLibrary::with_builtins`].

use std::collections::BTreeMap;

use crate::functions::ChainResult;
use crate::types_19::{ChainError, GeneratedContract, TargetPlatform};

/// The value-domain a template parameter accepts.
///
/// Validation is performed when a template is rendered so that malformed values
/// are rejected before any source is produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    /// A Solidity identifier (`[A-Za-z_][A-Za-z0-9_]*`) — contract/symbol names.
    Identifier,
    /// A non-negative decimal integer literal.
    UnsignedInt,
    /// A 20-byte `0x`-prefixed hex address.
    Address,
    /// Free-form text (e.g. a token name); only emptiness is rejected.
    Text,
    /// A boolean rendered as `true`/`false`.
    Boolean,
}

impl ParamKind {
    /// Validates `value` against this kind.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] describing the mismatch.
    pub fn validate(self, name: &str, value: &str) -> ChainResult<()> {
        let ok = match self {
            ParamKind::Identifier => is_solidity_identifier(value),
            ParamKind::UnsignedInt => {
                !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
            }
            ParamKind::Address => is_hex_address(value),
            ParamKind::Text => !value.trim().is_empty(),
            ParamKind::Boolean => value == "true" || value == "false",
        };
        if ok {
            Ok(())
        } else {
            Err(ChainError::GenerationError(format!(
                "template parameter '{name}' value '{value}' is not a valid {self:?}"
            )))
        }
    }
}

/// Declaration of a single template parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateParam {
    /// Placeholder name (without braces).
    pub name: String,
    /// Accepted value domain.
    pub kind: ParamKind,
    /// Optional default applied when the caller omits the parameter.
    pub default: Option<String>,
    /// Human-readable description for tooling/UIs.
    pub description: String,
}

/// A parameterized contract template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractTemplate {
    /// Unique template id (e.g. `"erc20_capped"`).
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Target platform of the rendered output.
    pub platform: TargetPlatform,
    /// Declared parameters.
    pub params: Vec<TemplateParam>,
    /// Source body containing `{{param}}` placeholders.
    pub body: String,
    /// Placeholder whose rendered value names the produced contract.
    pub name_param: String,
}

impl ContractTemplate {
    /// Renders the template into a [`GeneratedContract`] using `values`.
    ///
    /// Supplied values override defaults; omitted parameters fall back to their
    /// default if one exists. Validation enforces: no unknown keys, every
    /// parameter resolvable, each value matching its [`ParamKind`], and no
    /// `{{...}}` placeholders left unexpanded.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] on any validation failure.
    pub fn render(&self, values: &BTreeMap<String, String>) -> ChainResult<GeneratedContract> {
        // Reject unknown keys early so typos surface immediately.
        for key in values.keys() {
            if !self.params.iter().any(|param| &param.name == key) {
                return Err(ChainError::GenerationError(format!(
                    "template '{}' has no parameter named '{key}'",
                    self.id
                )));
            }
        }

        // Resolve every parameter to a concrete, validated value.
        let mut resolved: BTreeMap<String, String> = BTreeMap::new();
        for param in &self.params {
            let value = match values.get(&param.name) {
                Some(supplied) => supplied.clone(),
                None => param.default.clone().ok_or_else(|| {
                    ChainError::GenerationError(format!(
                        "template '{}' is missing required parameter '{}'",
                        self.id, param.name
                    ))
                })?,
            };
            param.kind.validate(&param.name, &value)?;
            resolved.insert(param.name.clone(), value);
        }

        // Expand placeholders.
        let mut source = self.body.clone();
        for (name, value) in &resolved {
            source = source.replace(&format!("{{{{{name}}}}}"), value);
        }
        if let Some(unfilled) = find_remaining_placeholder(&source) {
            return Err(ChainError::GenerationError(format!(
                "template '{}' left placeholder '{{{{{unfilled}}}}}' unexpanded",
                self.id
            )));
        }

        let name = resolved.get(&self.name_param).cloned().ok_or_else(|| {
            ChainError::GenerationError(format!(
                "template '{}' name_param '{}' is not a declared parameter",
                self.id, self.name_param
            ))
        })?;

        Ok(GeneratedContract {
            name,
            source,
            platform: self.platform,
            abi: None,
            deployment_script: None,
        })
    }
}

/// A searchable collection of [`ContractTemplate`]s.
#[derive(Debug, Clone, Default)]
pub struct TemplateLibrary {
    templates: BTreeMap<String, ContractTemplate>,
}

impl TemplateLibrary {
    /// Creates an empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers `template`, replacing any existing entry with the same id.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the template id is empty or if
    /// the `name_param` is not among the declared parameters.
    pub fn register(&mut self, template: ContractTemplate) -> ChainResult<()> {
        if template.id.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "template id must not be empty".to_string(),
            ));
        }
        if !template
            .params
            .iter()
            .any(|param| param.name == template.name_param)
        {
            return Err(ChainError::GenerationError(format!(
                "template '{}' name_param '{}' is not a declared parameter",
                template.id, template.name_param
            )));
        }
        self.templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// Looks up a template by id.
    pub fn get(&self, id: &str) -> Option<&ContractTemplate> {
        self.templates.get(id)
    }

    /// Returns the number of registered templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Returns whether the library is empty.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    /// Lists all template ids in sorted order.
    pub fn ids(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Renders the template `id` with `values`.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if no such template exists or if
    /// rendering fails.
    pub fn render(
        &self,
        id: &str,
        values: &BTreeMap<String, String>,
    ) -> ChainResult<GeneratedContract> {
        let template = self
            .get(id)
            .ok_or_else(|| ChainError::GenerationError(format!("unknown template: '{id}'")))?;
        template.render(values)
    }

    /// Builds a library pre-loaded with a curated set of production EVM templates.
    ///
    /// Included: a capped/ownable ERC-20, a role-gated pausable vault, and a
    /// simple timelocked escrow. Each is parameterized and emits compile-ready
    /// Solidity with NatSpec.
    pub fn with_builtins() -> Self {
        let mut library = Self::new();
        // These registrations cannot fail (the name_param of each builtin is a
        // declared parameter), but we propagate defensively rather than panic.
        let builtins = [
            builtin_erc20_capped(),
            builtin_pausable_vault(),
            builtin_escrow(),
        ];
        for template in builtins {
            // A failed builtin registration would be a programming error; skip it
            // rather than poison the whole library.
            let _ = library.register(template);
        }
        library
    }
}

/// Returns whether `value` is a valid Solidity identifier.
fn is_solidity_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

/// Returns whether `value` is a `0x`-prefixed 20-byte hex address.
fn is_hex_address(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("0x") else {
        return false;
    };
    hex.len() == 40 && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Finds the first remaining `{{name}}` placeholder, if any.
fn find_remaining_placeholder(source: &str) -> Option<String> {
    let start = source.find("{{")?;
    let rest = &source[start + 2..];
    let end = rest.find("}}")?;
    Some(rest[..end].to_string())
}

/// Builtin: capped, ownable ERC-20.
fn builtin_erc20_capped() -> ContractTemplate {
    ContractTemplate {
        id: "erc20_capped".to_string(),
        title: "Capped Ownable ERC-20".to_string(),
        platform: TargetPlatform::Solidity,
        params: vec![
            TemplateParam {
                name: "contract_name".to_string(),
                kind: ParamKind::Identifier,
                default: None,
                description: "Solidity contract name".to_string(),
            },
            TemplateParam {
                name: "token_name".to_string(),
                kind: ParamKind::Text,
                default: None,
                description: "ERC-20 token name".to_string(),
            },
            TemplateParam {
                name: "symbol".to_string(),
                kind: ParamKind::Identifier,
                default: None,
                description: "ERC-20 symbol".to_string(),
            },
            TemplateParam {
                name: "cap".to_string(),
                kind: ParamKind::UnsignedInt,
                default: None,
                description: "Maximum total supply (whole tokens, pre-decimals)".to_string(),
            },
        ],
        name_param: "contract_name".to_string(),
        body: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @title {{contract_name}}
/// @notice Capped, owner-mintable ERC-20 generated by Legalis-Chain.
/// @dev Minting reverts once the immutable cap is reached; ownership transfer is two-step.
contract {{contract_name}} is ERC20, Ownable2Step {
    uint256 public immutable cap;

    constructor() ERC20("{{token_name}}", "{{symbol}}") Ownable(msg.sender) {
        cap = {{cap}} * (10 ** decimals());
    }

    /// @notice Mint new tokens up to the cap (owner only).
    function mint(address to, uint256 amount) external onlyOwner {
        require(totalSupply() + amount <= cap, "Cap exceeded");
        _mint(to, amount);
    }
}
"#
        .to_string(),
    }
}

/// Builtin: role-gated, pausable deposit vault.
fn builtin_pausable_vault() -> ContractTemplate {
    ContractTemplate {
        id: "pausable_vault".to_string(),
        title: "Pausable Role-Gated Vault".to_string(),
        platform: TargetPlatform::Solidity,
        params: vec![
            TemplateParam {
                name: "contract_name".to_string(),
                kind: ParamKind::Identifier,
                default: None,
                description: "Solidity contract name".to_string(),
            },
            TemplateParam {
                name: "asset".to_string(),
                kind: ParamKind::Address,
                default: None,
                description: "ERC-20 asset held by the vault".to_string(),
            },
        ],
        name_param: "contract_name".to_string(),
        body: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/// @title {{contract_name}}
/// @notice Pausable, role-gated deposit vault generated by Legalis-Chain.
/// @dev Deposits/withdrawals follow checks-effects-interactions and are reentrancy-guarded.
contract {{contract_name}} is AccessControl, Pausable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");
    IERC20 public constant ASSET = IERC20({{asset}});

    mapping(address => uint256) public balanceOf;

    event Deposited(address indexed account, uint256 amount);
    event Withdrawn(address indexed account, uint256 amount);

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(GUARDIAN_ROLE, msg.sender);
    }

    /// @notice Deposit `amount` of the underlying asset.
    function deposit(uint256 amount) external whenNotPaused nonReentrant {
        balanceOf[msg.sender] += amount; // effects before interaction
        ASSET.safeTransferFrom(msg.sender, address(this), amount);
        emit Deposited(msg.sender, amount);
    }

    /// @notice Withdraw `amount` of the underlying asset.
    function withdraw(uint256 amount) external nonReentrant {
        require(balanceOf[msg.sender] >= amount, "Insufficient balance");
        balanceOf[msg.sender] -= amount; // effects before interaction
        ASSET.safeTransfer(msg.sender, amount);
        emit Withdrawn(msg.sender, amount);
    }

    /// @notice Pause deposits (guardian only).
    function pause() external onlyRole(GUARDIAN_ROLE) {
        _pause();
    }

    /// @notice Resume deposits (admin only).
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
"#
        .to_string(),
    }
}

/// Builtin: single-beneficiary timelocked escrow.
fn builtin_escrow() -> ContractTemplate {
    ContractTemplate {
        id: "timelock_escrow".to_string(),
        title: "Timelocked Escrow".to_string(),
        platform: TargetPlatform::Solidity,
        params: vec![
            TemplateParam {
                name: "contract_name".to_string(),
                kind: ParamKind::Identifier,
                default: None,
                description: "Solidity contract name".to_string(),
            },
            TemplateParam {
                name: "beneficiary".to_string(),
                kind: ParamKind::Address,
                default: None,
                description: "Address entitled to the escrowed funds".to_string(),
            },
            TemplateParam {
                name: "release_after".to_string(),
                kind: ParamKind::UnsignedInt,
                default: Some("0".to_string()),
                description: "Seconds after deployment before funds can be released".to_string(),
            },
        ],
        name_param: "contract_name".to_string(),
        body: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/// @title {{contract_name}}
/// @notice Single-beneficiary timelocked ETH escrow generated by Legalis-Chain.
/// @dev Release is reentrancy-guarded and follows checks-effects-interactions.
contract {{contract_name}} is ReentrancyGuard {
    address payable public immutable beneficiary;
    uint256 public immutable releaseTime;
    bool public released;

    event Released(uint256 amount);

    constructor() payable {
        beneficiary = payable({{beneficiary}});
        releaseTime = block.timestamp + {{release_after}} seconds;
    }

    receive() external payable {}

    /// @notice Release the escrowed balance to the beneficiary after the timelock.
    function release() external nonReentrant {
        require(!released, "Already released");
        require(block.timestamp >= releaseTime, "Timelock active");
        released = true; // effects before interaction
        uint256 amount = address(this).balance;
        (bool ok, ) = beneficiary.call{value: amount}("");
        require(ok, "Transfer failed");
        emit Released(amount);
    }
}
"#
        .to_string(),
    }
}
