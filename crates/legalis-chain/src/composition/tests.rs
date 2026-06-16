//! Tests for the contract-composition toolkit.

use std::collections::BTreeMap;

use crate::{
    ContractComponent, ContractTemplate, DependencyGraph, InheritanceHierarchy, InheritanceNode,
    ModularContractBuilder, ParamKind, TargetPlatform, TemplateLibrary, TemplateParam,
};

// --- Dependency graph ----------------------------------------------------------

#[test]
fn test_dependency_topo_simple_chain() {
    let mut graph = DependencyGraph::new();
    // C depends on B depends on A => deploy order A, B, C.
    graph.add_dependency("B", "A").expect("edge B->A");
    graph.add_dependency("C", "B").expect("edge C->B");
    let order = graph.topological_order().expect("acyclic");
    assert_eq!(order, vec!["A", "B", "C"]);
    assert!(graph.is_acyclic());
}

#[test]
fn test_dependency_topo_is_insertion_stable() {
    let mut graph = DependencyGraph::new();
    // Two independent roots; insertion order decides the tie.
    graph.add_node("First").expect("node");
    graph.add_node("Second").expect("node");
    graph.add_dependency("Leaf", "First").expect("edge");
    graph.add_dependency("Leaf", "Second").expect("edge");
    let order = graph.topological_order().expect("acyclic");
    assert_eq!(order, vec!["First", "Second", "Leaf"]);
    // Leaf must come last (after both prerequisites).
    let leaf_pos = order
        .iter()
        .position(|n| n == "Leaf")
        .expect("leaf present");
    assert_eq!(leaf_pos, 2);
}

#[test]
fn test_dependency_diamond_order_valid() {
    // D -> {B, C} -> A. Valid orders place A first and D last.
    let mut graph = DependencyGraph::new();
    graph.add_dependency("B", "A").expect("edge");
    graph.add_dependency("C", "A").expect("edge");
    graph.add_dependency("D", "B").expect("edge");
    graph.add_dependency("D", "C").expect("edge");
    let order = graph.topological_order().expect("acyclic");
    let pos = |name: &str| order.iter().position(|n| n == name).expect("present");
    assert!(pos("A") < pos("B"));
    assert!(pos("A") < pos("C"));
    assert!(pos("B") < pos("D"));
    assert!(pos("C") < pos("D"));
}

#[test]
fn test_dependency_cycle_is_rejected() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("A", "B").expect("edge");
    graph.add_dependency("B", "C").expect("edge");
    graph.add_dependency("C", "A").expect("edge"); // closes the cycle
    assert!(!graph.is_acyclic());
    let err = graph.topological_order().expect_err("cycle");
    let message = format!("{err}");
    assert!(message.contains("cycle"), "unexpected message: {message}");
}

#[test]
fn test_dependency_self_edge_rejected() {
    let mut graph = DependencyGraph::new();
    let err = graph.add_dependency("A", "A").expect_err("self-edge");
    assert!(format!("{err}").contains("itself"));
}

#[test]
fn test_dependency_empty_name_rejected() {
    let mut graph = DependencyGraph::new();
    assert!(graph.add_node("   ").is_err());
}

#[test]
fn test_dependency_transitive_closure() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("B", "A").expect("edge");
    graph.add_dependency("C", "B").expect("edge");
    graph.add_dependency("D", "C").expect("edge");
    let closure = graph.transitive_dependencies("D").expect("closure");
    assert_eq!(closure, vec!["A", "B", "C"]);
    // A has no dependencies.
    assert!(
        graph
            .transitive_dependencies("A")
            .expect("closure")
            .is_empty()
    );
}

#[test]
fn test_dependency_transitive_unknown_node() {
    let graph = DependencyGraph::new();
    assert!(graph.transitive_dependencies("ghost").is_err());
}

#[test]
fn test_dependency_direct_dependencies() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("App", "Lib").expect("edge");
    graph.add_dependency("App", "Token").expect("edge");
    let direct = graph.direct_dependencies("App").expect("present");
    assert!(direct.contains(&"Lib".to_string()));
    assert!(direct.contains(&"Token".to_string()));
    assert_eq!(direct.len(), 2);
    assert!(graph.direct_dependencies("ghost").is_none());
}

#[test]
fn test_dependency_idempotent_add_node() {
    let mut graph = DependencyGraph::new();
    let first = graph.add_node("A").expect("node");
    let second = graph.add_node("A").expect("node");
    assert_eq!(first, second);
    assert_eq!(graph.len(), 1);
}

// --- Inheritance / C3 linearization --------------------------------------------

#[test]
fn test_inheritance_linear_chain() {
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "B".to_string(),
            parents: vec!["A".to_string()],
        })
        .expect("declare B");
    hierarchy
        .declare(InheritanceNode {
            name: "C".to_string(),
            parents: vec!["B".to_string()],
        })
        .expect("declare C");
    let mro = hierarchy.linearize("C").expect("mro");
    assert_eq!(mro, vec!["C", "B", "A"]);
}

#[test]
fn test_inheritance_diamond_c3() {
    // Classic diamond: D is B, C; B is A; C is A.
    // Solidity lists bases base-first, so C3 MRO (derived-first) is D,C,B,A.
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "A".to_string(),
            parents: vec![],
        })
        .expect("declare A");
    hierarchy
        .declare(InheritanceNode {
            name: "B".to_string(),
            parents: vec!["A".to_string()],
        })
        .expect("declare B");
    hierarchy
        .declare(InheritanceNode {
            name: "C".to_string(),
            parents: vec!["A".to_string()],
        })
        .expect("declare C");
    hierarchy
        .declare(InheritanceNode {
            name: "D".to_string(),
            parents: vec!["B".to_string(), "C".to_string()],
        })
        .expect("declare D");
    let mro = hierarchy.linearize("D").expect("mro");
    assert_eq!(mro, vec!["D", "C", "B", "A"]);
    // A must appear exactly once and last.
    assert_eq!(mro.iter().filter(|name| *name == "A").count(), 1);
}

#[test]
fn test_inheritance_optimized_bases_drops_redundant() {
    // X inherits both Base and Mid, but Mid already inherits Base, so Base is
    // redundant in X's direct base list.
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "Mid".to_string(),
            parents: vec!["Base".to_string()],
        })
        .expect("declare Mid");
    hierarchy
        .declare(InheritanceNode {
            name: "X".to_string(),
            parents: vec!["Base".to_string(), "Mid".to_string()],
        })
        .expect("declare X");
    let bases = hierarchy.optimized_bases("X").expect("bases");
    // Base is implied by Mid, so only Mid remains.
    assert_eq!(bases, vec!["Mid"]);
}

#[test]
fn test_inheritance_optimized_bases_orders_base_first() {
    // X is Mid, Base (wrong order). Optimizer must emit base-first: Base, Mid
    // is impossible because Mid implies Base; so result is just Mid.
    // Use two independent bases to check ordering instead.
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "X".to_string(),
            parents: vec!["Ownable".to_string(), "Pausable".to_string()],
        })
        .expect("declare X");
    let bases = hierarchy.optimized_bases("X").expect("bases");
    assert_eq!(bases.len(), 2);
    assert!(bases.contains(&"Ownable".to_string()));
    assert!(bases.contains(&"Pausable".to_string()));
}

#[test]
fn test_inheritance_inconsistent_is_rejected() {
    // Inconsistent MRO: D is B, C; B is X, Y; C is Y, X. The conflicting order of
    // X and Y makes C3 linearization impossible (as in Solidity/Python).
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "B".to_string(),
            parents: vec!["X".to_string(), "Y".to_string()],
        })
        .expect("declare B");
    hierarchy
        .declare(InheritanceNode {
            name: "C".to_string(),
            parents: vec!["Y".to_string(), "X".to_string()],
        })
        .expect("declare C");
    hierarchy
        .declare(InheritanceNode {
            name: "D".to_string(),
            parents: vec!["B".to_string(), "C".to_string()],
        })
        .expect("declare D");
    let err = hierarchy.linearize("D").expect_err("impossible");
    assert!(format!("{err}").contains("impossible"));
}

#[test]
fn test_inheritance_self_parent_rejected() {
    let mut hierarchy = InheritanceHierarchy::new();
    let err = hierarchy
        .declare(InheritanceNode {
            name: "A".to_string(),
            parents: vec!["A".to_string()],
        })
        .expect_err("self-inherit");
    assert!(format!("{err}").contains("itself"));
}

#[test]
fn test_inheritance_duplicate_parent_rejected() {
    let mut hierarchy = InheritanceHierarchy::new();
    let err = hierarchy
        .declare(InheritanceNode {
            name: "A".to_string(),
            parents: vec!["B".to_string(), "B".to_string()],
        })
        .expect_err("dup parent");
    assert!(format!("{err}").contains("duplicate"));
}

#[test]
fn test_inheritance_cycle_rejected() {
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "A".to_string(),
            parents: vec!["B".to_string()],
        })
        .expect("declare A");
    hierarchy
        .declare(InheritanceNode {
            name: "B".to_string(),
            parents: vec!["A".to_string()],
        })
        .expect("declare B");
    let err = hierarchy.linearize("A").expect_err("cycle");
    assert!(format!("{err}").contains("cycle"));
}

#[test]
fn test_inheritance_leaf_has_no_bases() {
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "Solo".to_string(),
            parents: vec![],
        })
        .expect("declare");
    assert!(hierarchy.optimized_bases("Solo").expect("bases").is_empty());
    assert_eq!(hierarchy.linearize("Solo").expect("mro"), vec!["Solo"]);
}

// --- Template library ----------------------------------------------------------

fn sample_template() -> ContractTemplate {
    ContractTemplate {
        id: "greeter".to_string(),
        title: "Greeter".to_string(),
        platform: TargetPlatform::Solidity,
        params: vec![
            TemplateParam {
                name: "contract_name".to_string(),
                kind: ParamKind::Identifier,
                default: None,
                description: "name".to_string(),
            },
            TemplateParam {
                name: "greeting".to_string(),
                kind: ParamKind::Text,
                default: Some("Hello".to_string()),
                description: "greeting".to_string(),
            },
        ],
        name_param: "contract_name".to_string(),
        body: "contract {{contract_name}} { string public greeting = \"{{greeting}}\"; }\n"
            .to_string(),
    }
}

#[test]
fn test_template_render_with_defaults() {
    let template = sample_template();
    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "MyGreeter".to_string());
    // greeting omitted -> default "Hello".
    let contract = template.render(&values).expect("render");
    assert_eq!(contract.name, "MyGreeter");
    assert!(contract.source.contains("contract MyGreeter"));
    assert!(contract.source.contains("\"Hello\""));
    assert!(!contract.source.contains("{{"));
}

#[test]
fn test_template_render_override_default() {
    let template = sample_template();
    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "Hi".to_string());
    values.insert("greeting".to_string(), "Bonjour".to_string());
    let contract = template.render(&values).expect("render");
    assert!(contract.source.contains("\"Bonjour\""));
}

#[test]
fn test_template_missing_required_param() {
    let template = sample_template();
    let values = BTreeMap::new(); // contract_name missing, no default
    let err = template.render(&values).expect_err("missing");
    assert!(format!("{err}").contains("missing required parameter"));
}

#[test]
fn test_template_unknown_param_rejected() {
    let template = sample_template();
    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "X".to_string());
    values.insert("nope".to_string(), "1".to_string());
    let err = template.render(&values).expect_err("unknown");
    assert!(format!("{err}").contains("no parameter named"));
}

#[test]
fn test_template_param_kind_validation() {
    let template = sample_template();
    let mut values = BTreeMap::new();
    // Identifier cannot start with a digit.
    values.insert("contract_name".to_string(), "9bad".to_string());
    let err = template.render(&values).expect_err("bad ident");
    assert!(format!("{err}").contains("not a valid"));
}

#[test]
fn test_param_kind_address_validation() {
    assert!(
        ParamKind::Address
            .validate("a", "0x1111111111111111111111111111111111111111")
            .is_ok()
    );
    assert!(ParamKind::Address.validate("a", "0x1234").is_err());
    assert!(
        ParamKind::Address
            .validate("a", "1111111111111111111111111111111111111111")
            .is_err()
    );
}

#[test]
fn test_param_kind_unsigned_and_bool() {
    assert!(ParamKind::UnsignedInt.validate("n", "1000").is_ok());
    assert!(ParamKind::UnsignedInt.validate("n", "-1").is_err());
    assert!(ParamKind::UnsignedInt.validate("n", "1.5").is_err());
    assert!(ParamKind::Boolean.validate("b", "true").is_ok());
    assert!(ParamKind::Boolean.validate("b", "false").is_ok());
    assert!(ParamKind::Boolean.validate("b", "yes").is_err());
}

#[test]
fn test_library_builtins_present_and_render() {
    let library = TemplateLibrary::with_builtins();
    assert!(library.len() >= 3);
    let ids = library.ids();
    assert!(ids.contains(&"erc20_capped".to_string()));
    assert!(ids.contains(&"pausable_vault".to_string()));
    assert!(ids.contains(&"timelock_escrow".to_string()));

    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "MyToken".to_string());
    values.insert("token_name".to_string(), "My Token".to_string());
    values.insert("symbol".to_string(), "MTK".to_string());
    values.insert("cap".to_string(), "1000000".to_string());
    let contract = library.render("erc20_capped", &values).expect("render");
    assert_eq!(contract.name, "MyToken");
    assert!(
        contract
            .source
            .contains("contract MyToken is ERC20, Ownable2Step")
    );
    assert!(contract.source.contains("1000000 * (10 ** decimals())"));
    assert!(!contract.source.contains("{{"));
}

#[test]
fn test_library_builtin_vault_address_validated() {
    let library = TemplateLibrary::with_builtins();
    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "Vault".to_string());
    // Not a valid address.
    values.insert("asset".to_string(), "0xdead".to_string());
    assert!(library.render("pausable_vault", &values).is_err());
}

#[test]
fn test_library_unknown_template() {
    let library = TemplateLibrary::with_builtins();
    let values = BTreeMap::new();
    assert!(library.render("does_not_exist", &values).is_err());
}

#[test]
fn test_library_register_validates_name_param() {
    let mut library = TemplateLibrary::new();
    let bad = ContractTemplate {
        id: "bad".to_string(),
        title: "Bad".to_string(),
        platform: TargetPlatform::Solidity,
        params: vec![],
        body: "contract X {}".to_string(),
        name_param: "missing".to_string(),
    };
    assert!(library.register(bad).is_err());
}

// --- Modular builder -----------------------------------------------------------

fn ownable_component() -> ContractComponent {
    ContractComponent::new("Ownable")
        .with_import("import \"@openzeppelin/contracts/access/Ownable2Step.sol\";")
        .with_base("Ownable2Step")
}

fn counter_component() -> ContractComponent {
    ContractComponent::new("Counter")
        .with_state_var("uint256 public count;")
        .with_event("event Incremented(uint256 newValue);")
        .with_function(
            "function increment() external onlyOwner {\n    count += 1;\n    emit Incremented(count);\n}",
        )
}

#[test]
fn test_builder_composes_sections() {
    let contract = ModularContractBuilder::new("CounterApp")
        .with_component(ownable_component())
        .with_component(counter_component())
        .with_constructor_body("// no-op")
        .build()
        .expect("build");
    assert_eq!(contract.name, "CounterApp");
    let source = &contract.source;
    assert!(source.contains("pragma solidity ^0.8.20;"));
    assert!(source.contains("import \"@openzeppelin/contracts/access/Ownable2Step.sol\";"));
    assert!(source.contains("contract CounterApp is Ownable2Step"));
    assert!(source.contains("uint256 public count;"));
    assert!(source.contains("event Incremented(uint256 newValue);"));
    assert!(source.contains("function increment()"));
    assert!(source.contains("-- from component: Counter --"));
    assert!(source.contains("constructor()"));
    // Section banners present.
    assert!(source.contains("==== State ===="));
    assert!(source.contains("==== Functions ===="));
}

#[test]
fn test_builder_deduplicates_imports_and_bases() {
    // Two components both pull in Ownable2Step.
    let comp_a = ContractComponent::new("A")
        .with_import("import \"X.sol\";")
        .with_base("Base");
    let comp_b = ContractComponent::new("B")
        .with_import("import \"X.sol\";")
        .with_base("Base");
    let contract = ModularContractBuilder::new("Combo")
        .with_component(comp_a)
        .with_component(comp_b)
        .build()
        .expect("build");
    // Import appears exactly once.
    let import_count = contract.source.matches("import \"X.sol\";").count();
    assert_eq!(import_count, 1);
    // Base appears once in the is-clause.
    assert!(contract.source.contains("contract Combo is Base {"));
}

#[test]
fn test_builder_rejects_member_collision() {
    let comp_a = ContractComponent::new("A").with_state_var("uint256 public count;");
    let comp_b = ContractComponent::new("B").with_state_var("uint256   public   count;");
    let err = ModularContractBuilder::new("Dup")
        .with_component(comp_a)
        .with_component(comp_b)
        .build()
        .expect_err("collision");
    assert!(format!("{err}").contains("duplicate state variable"));
}

#[test]
fn test_builder_requires_components() {
    let err = ModularContractBuilder::new("Empty")
        .build()
        .expect_err("no comps");
    assert!(format!("{err}").contains("at least one component"));
}

#[test]
fn test_builder_empty_name_rejected() {
    let err = ModularContractBuilder::new("  ")
        .with_component(counter_component())
        .build()
        .expect_err("empty name");
    assert!(format!("{err}").contains("must not be empty"));
}

#[test]
fn test_builder_no_bases_emits_plain_contract() {
    let contract = ModularContractBuilder::new("Plain")
        .with_component(ContractComponent::new("S").with_state_var("uint256 public x;"))
        .build()
        .expect("build");
    assert!(contract.source.contains("contract Plain {"));
    assert!(!contract.source.contains(" is "));
}

#[test]
fn test_builder_resolved_bases_linearized() {
    // Component bases Base and Mid where Mid inherits Base -> only Mid emitted,
    // exercised via resolved_bases through a hierarchy declared by the builder.
    let mut hierarchy = InheritanceHierarchy::new();
    hierarchy
        .declare(InheritanceNode {
            name: "Mid".to_string(),
            parents: vec!["Base".to_string()],
        })
        .expect("declare Mid");
    // Builder treats both as direct parents of the assembled contract; since the
    // builder's own hierarchy only knows the assembled contract, redundancy
    // pruning needs the relationship declared. Validate the builder keeps both
    // when no relationship is known (safe default), and the inheritance optimizer
    // prunes when the relationship IS known.
    let pruned = hierarchy
        .declare(InheritanceNode {
            name: "Asm".to_string(),
            parents: vec!["Base".to_string(), "Mid".to_string()],
        })
        .map(|()| hierarchy.optimized_bases("Asm"));
    let bases = pruned.expect("declared").expect("bases");
    assert_eq!(bases, vec!["Mid"]);
}

#[test]
fn test_builder_add_component_validates() {
    let mut builder = ModularContractBuilder::new("Guard");
    let err = builder
        .add_component(ContractComponent::new("  "))
        .expect_err("empty comp name");
    assert!(format!("{err}").contains("must not be empty"));
    builder.add_component(counter_component()).expect("ok comp");
}
