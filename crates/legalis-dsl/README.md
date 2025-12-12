# legalis-dsl

Domain Specific Language parser for legal documents in Legalis-RS.

## Overview

This crate provides a parser and AST representation for the Legal DSL, enabling structured representation of statutes and legal rules from a human-readable text format.

## DSL Grammar

```
STATUTE ::= "STATUTE" ID ":" TITLE "{" BODY "}"
BODY    ::= (WHEN | THEN | DISCRETION)*
WHEN    ::= "WHEN" CONDITION
THEN    ::= "THEN" EFFECT
DISCRETION ::= "DISCRETION" STRING

CONDITION ::= AGE_COND | INCOME_COND | ATTR_COND
AGE_COND ::= "AGE" OPERATOR NUMBER
INCOME_COND ::= "INCOME" OPERATOR NUMBER

EFFECT ::= "GRANT" | "REVOKE" | "OBLIGATION" STRING

OPERATOR ::= ">=" | "<=" | ">" | "<" | "==" | "!="
```

## Usage

### Parsing a Statute

```rust
use legalis_dsl::LegalDslParser;

let parser = LegalDslParser::new();
let statute = parser.parse_statute(r#"
    STATUTE adult-rights: "Adult Rights Act" {
        WHEN AGE >= 18
        THEN GRANT "Full legal capacity"
    }
"#)?;

assert_eq!(statute.id, "adult-rights");
assert_eq!(statute.title, "Adult Rights Act");
```

### Parsing with Discretion

```rust
let statute = parser.parse_statute(r#"
    STATUTE housing-subsidy: "Housing Subsidy Act" {
        WHEN INCOME <= 5000000
        THEN GRANT "Housing subsidy eligibility"
        DISCRETION "Consider special family circumstances"
    }
"#)?;

assert!(statute.discretion_logic.is_some());
```

## AST Types

### Token

Lexer tokens for DSL parsing:

```rust
pub enum Token {
    Statute, When, Then, Discretion,
    Age, Income, Grant, Revoke, Obligation,
    LBrace, RBrace, Colon,
    Ident(String), StringLit(String),
    Number(u64), Operator(String),
}
```

### ConditionNode

AST representation of conditions:

```rust
pub enum ConditionNode {
    Comparison { field: String, operator: String, value: ConditionValue },
    HasAttribute { key: String },
    And(Box<ConditionNode>, Box<ConditionNode>),
    Or(Box<ConditionNode>, Box<ConditionNode>),
    Not(Box<ConditionNode>),
}
```

## Conversion to Core Types

Use the `ToCore` trait to convert AST nodes to `legalis_core` types:

```rust
use legalis_dsl::parser::ToCore;

let condition_node = ConditionNode::Comparison {
    field: "age".to_string(),
    operator: ">=".to_string(),
    value: ConditionValue::Number(18),
};

let core_condition = condition_node.to_core()?;
```

## Error Handling

```rust
pub enum DslError {
    ParseError { position: usize, message: String },
    InvalidCondition(String),
    InvalidEffect(String),
    UnexpectedEof,
}
```

## License

MIT OR Apache-2.0
