# legalis-dsl

Domain Specific Language parser for legal documents in Legalis-RS.

## Overview

This crate provides a parser, AST representation, and pretty-printer for the Legal DSL, enabling structured representation of statutes and legal rules from a human-readable text format.

## Features

- **Full Parser**: Tokenizer and recursive descent parser
- **Multi-Statute Support**: Parse multiple statutes from a single document
- **Import System**: Reference other statute files
- **Comments**: Support for `//` and `/* */` comments
- **Metadata**: Jurisdiction, version, effective dates
- **Logical Operators**: AND, OR, NOT with proper precedence
- **Pretty-Printer**: Format and roundtrip DSL code
- **Error Tracking**: Line/column information for parse errors

## DSL Grammar

```
DOCUMENT  ::= IMPORT* STATUTE*
IMPORT    ::= "IMPORT" STRING ("AS" IDENT)?
STATUTE   ::= "STATUTE" ID ":" TITLE "{" BODY "}"
BODY      ::= (METADATA | WHEN | THEN | DISCRETION)*
METADATA  ::= JURISDICTION | VERSION | EFFECTIVE_DATE | EXPIRY_DATE
WHEN      ::= "WHEN" CONDITION
THEN      ::= "THEN" EFFECT
DISCRETION ::= "DISCRETION" STRING

CONDITION ::= OR_EXPR
OR_EXPR   ::= AND_EXPR ("OR" AND_EXPR)*
AND_EXPR  ::= UNARY_EXPR ("AND" UNARY_EXPR)*
UNARY_EXPR ::= "NOT" UNARY_EXPR | "(" OR_EXPR ")" | PRIMARY
PRIMARY   ::= AGE_COND | INCOME_COND | HAS_COND | IDENT ("." IDENT)?

EFFECT    ::= "GRANT" | "REVOKE" | "OBLIGATION" | "PROHIBITION" STRING
OPERATOR  ::= ">=" | "<=" | ">" | "<" | "==" | "!="
```

## Usage

### Parsing a Single Statute

```rust
use legalis_dsl::LegalDslParser;

let parser = LegalDslParser::new();
let statute = parser.parse_statute(r#"
    STATUTE adult-rights: "Adult Rights Act" {
        JURISDICTION "US"
        VERSION 1
        EFFECTIVE_DATE 2024-01-01

        WHEN AGE >= 18
        THEN GRANT "Full legal capacity"

        DISCRETION "Consider emancipation cases"
    }
"#)?;

assert_eq!(statute.id, "adult-rights");
assert_eq!(statute.jurisdiction, Some("US".to_string()));
```

### Parsing Multiple Statutes

```rust
let statutes = parser.parse_statutes(r#"
    // First statute
    STATUTE law-1: "First Law" {
        WHEN AGE >= 18
        THEN GRANT "Right A"
    }

    /* Second statute with
       block comment */
    STATUTE law-2: "Second Law" {
        WHEN INCOME <= 50000
        THEN GRANT "Right B"
    }
"#)?;

assert_eq!(statutes.len(), 2);
```

### Parsing with Imports

```rust
let doc = parser.parse_document(r#"
    IMPORT "base/common.legalis" AS common
    IMPORT "extensions/special.legalis"

    STATUTE derived: "Derived Law" {
        WHEN common.eligibility AND AGE >= 21
        THEN GRANT "Extended rights"
    }
"#)?;

assert_eq!(doc.imports.len(), 2);
assert_eq!(doc.statutes.len(), 1);
```

### Complex Conditions

```rust
let statute = parser.parse_statute(r#"
    STATUTE complex: "Complex Eligibility" {
        WHEN (AGE >= 65 OR HAS disability) AND INCOME <= 50000
        THEN GRANT "Benefits"
    }
"#)?;
```

### Pretty-Printing

```rust
use legalis_dsl::printer::{DslPrinter, PrinterConfig};

let printer = DslPrinter::new(PrinterConfig::default());
let formatted = printer.format_statute(&statute);
println!("{}", formatted);
```

## AST Types

### Token

```rust
pub enum Token {
    // Keywords
    Statute, When, Then, Discretion, Import, As,
    Age, Income, Grant, Revoke, Obligation, Prohibition,
    And, Or, Not, Has,
    // Metadata
    EffectiveDate, ExpiryDate, Jurisdiction, Version,
    // Structural
    LParen, RParen, LBrace, RBrace, Colon, Dash, Dot,
    // Literals
    Ident(String), StringLit(String), Number(u64), Operator(String),
}
```

### ConditionNode

```rust
pub enum ConditionNode {
    Comparison { field: String, operator: String, value: ConditionValue },
    HasAttribute { key: String },
    And(Box<ConditionNode>, Box<ConditionNode>),
    Or(Box<ConditionNode>, Box<ConditionNode>),
    Not(Box<ConditionNode>),
}
```

### ImportNode

```rust
pub struct ImportNode {
    pub path: String,
    pub alias: Option<String>,
}
```

## Error Handling

```rust
pub enum DslError {
    ParseError { location: Option<SourceLocation>, message: String },
    InvalidCondition(String),
    InvalidEffect(String),
    UnexpectedEof,
    UnclosedComment(Option<SourceLocation>),
    UnmatchedParen(Option<SourceLocation>),
}
```

Errors include source location information for better diagnostics:

```rust
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}
```

## CLI Integration

```bash
# Parse and validate
legalis parse input.legalis

# Format code
legalis format input.legalis --inplace

# Parse as JSON
legalis parse input.legalis --format json
```

## License

MIT OR Apache-2.0
