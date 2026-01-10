# Contributing to legalis-eu

Thank you for your interest in contributing to legalis-eu! This document provides guidelines and instructions for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Documentation Standards](#documentation-standards)
- [Submission Guidelines](#submission-guidelines)

## Code of Conduct

- Be respectful and constructive
- Focus on what is best for the community
- Show empathy towards other community members

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Code sample** demonstrating the issue
- **Rust version** (`rustc --version`)
- **Crate version**

**Template:**

```markdown
## Bug Description
Clear description of the bug

## Steps to Reproduce
1. Create DataProcessing with...
2. Call validate()
3. See error

## Expected Behavior
Should validate successfully

## Actual Behavior
Returns error: ...

## Code Sample
\`\`\`rust
let processing = DataProcessing::new()
    .with_controller("Test");
// ...
\`\`\`

## Environment
- Rust version: 1.75.0
- legalis-eu version: 0.5.9
```

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. Include:

- **Clear title and description**
- **Use case** explaining why this would be useful
- **Proposed API** if applicable
- **Example code** showing intended usage

### Contributing Code

Areas where we especially welcome contributions:

1. **Language Translations**
   - French, Spanish, Italian translations
   - EUR-Lex verified text only

2. **Member State Implementations**
   - National GDPR implementations (BDSG, etc.)
   - Member state-specific requirements

3. **Additional CJEU Cases**
   - Landmark case law
   - Recent decisions

4. **Performance Optimizations**
   - Benchmarks showing improvement
   - Profiling data

5. **Documentation Improvements**
   - Guides, examples, API docs
   - Fixing typos and clarifications

## Development Setup

### Prerequisites

```bash
# Rust toolchain (1.70.0 or later)
rustup update stable

# Nextest (recommended for testing)
cargo install cargo-nextest

# Criterion (for benchmarks)
# Already in dev-dependencies
```

### Clone and Build

```bash
git clone https://github.com/cool-japan/legalis.git
cd legalis/jurisdictions/eu

# Build
cargo build

# Run tests
cargo nextest run

# Run examples
cargo run --example gdpr_consent_validation
```

## Coding Standards

### Zero Warnings Policy

**All code must compile with zero warnings:**

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

This is enforced in CI. Fix all warnings before submitting.

### Rust Idioms

Follow standard Rust conventions:

```rust
// ✅ Good: Builder pattern
impl DataProcessing {
    pub fn with_controller(mut self, controller: impl Into<String>) -> Self {
        self.controller = Some(controller.into());
        self
    }
}

// ✅ Good: Result for fallible operations
pub fn validate(&self) -> Result<Validation, GdprError> {
    // ...
}

// ✅ Good: Descriptive error types
#[derive(Debug, Error)]
pub enum GdprError {
    #[error("Missing required field: {field}")]
    MissingField { field: String },
}
```

### Naming Conventions

- **Types**: `PascalCase` - `DataProcessing`, `LawfulBasis`
- **Functions**: `snake_case` - `validate()`, `with_controller()`
- **Constants**: `SCREAMING_SNAKE_CASE` - `MAX_FINE_AMOUNT`
- **Module files**: `snake_case.rs` - `article6.rs`, `cross_border.rs`

### Builder Pattern

All complex types must use the builder pattern:

```rust
pub struct MyType {
    field1: Option<String>,
    field2: bool,
}

impl MyType {
    pub fn new() -> Self {
        Self {
            field1: None,
            field2: false,
        }
    }

    pub fn with_field1(mut self, value: impl Into<String>) -> Self {
        self.field1 = Some(value.into());
        self
    }

    pub fn with_field2(mut self, value: bool) -> Self {
        self.field2 = value;
        self
    }
}

impl Default for MyType {
    fn default() -> Self {
        Self::new()
    }
}
```

## Testing Requirements

### Test Coverage

All new features must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_processing() {
        let processing = DataProcessing::new()
            .with_controller("Test Corp")
            .with_purpose("Testing")
            .with_lawful_basis(LawfulBasis::Contract {
                necessary_for_performance: true,
            });

        let result = processing.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_lawful_basis() {
        let processing = DataProcessing::new()
            .with_controller("Test Corp");

        let result = processing.validate();
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# Run all tests
cargo nextest run

# Run specific test
cargo test test_valid_processing

# Run with output
cargo test -- --nocapture

# Test specific package
cargo test -p legalis-eu
```

### Test Quality Standards

- **One assertion per test** (when possible)
- **Descriptive test names**: `test_consent_requires_all_four_criteria`
- **Test both success and failure cases**
- **Use meaningful test data**

## Documentation Standards

### Code Documentation

All public items must be documented:

```rust
/// Validates GDPR data processing operation
///
/// Checks:
/// - Article 6: Lawful basis is present and valid
/// - Article 9: Special category exceptions if applicable
/// - Purpose limitation and data minimization
///
/// # Example
///
/// ```
/// use legalis_eu::gdpr::*;
///
/// let processing = DataProcessing::new()
///     .with_controller("My Company")
///     .with_lawful_basis(LawfulBasis::Consent {
///         freely_given: true,
///         specific: true,
///         informed: true,
///         unambiguous: true,
///     });
///
/// match processing.validate() {
///     Ok(v) => println!("Valid"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # Errors
///
/// Returns [`GdprError::MissingLawfulBasis`] if no lawful basis specified
pub fn validate(&self) -> Result<ProcessingValidation, GdprError> {
    // ...
}
```

### EUR-Lex References

Always cite legal sources:

```rust
/// Article 6(1)(a) - Consent
///
/// GDPR requires consent to be:
/// - Freely given (Recital 42)
/// - Specific (Recital 43)
/// - Informed (Article 7)
/// - Unambiguous (Article 4(11))
///
/// Source: Regulation (EU) 2016/679, CELEX:32016R0679
```

### Examples

Add examples for significant features:

```bash
# Create example in examples/ directory
touch examples/my_new_feature.rs
```

Example structure:

```rust
//! Demonstrates [feature name]
//!
//! Run with: cargo run --example my_new_feature

use legalis_eu::gdpr::*;

fn main() {
    println!("=== [Feature Name] Example ===\n");

    // Scenario 1
    println!("Scenario 1: [Description]");
    // ...

    // Scenario 2
    println!("\nScenario 2: [Description]");
    // ...
}
```

## Submission Guidelines

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/my-feature
   ```

3. **Make your changes**
   - Follow coding standards
   - Add tests
   - Update documentation

4. **Run quality checks**
   ```bash
   # Tests must pass
   cargo nextest run

   # No warnings
   cargo clippy --all-targets --all-features -- -D warnings

   # Format code
   cargo fmt

   # Check docs
   cargo doc --no-deps
   ```

5. **Commit with clear messages**
   ```bash
   git commit -m "Add Article 99 validation

   - Implement validation logic for Article 99
   - Add 5 unit tests
   - Add example demonstrating usage
   - Update GDPR_GUIDE.md

   Closes #123"
   ```

6. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   ```

7. **Fill out PR template** (describe changes, reference issues)

### Commit Message Guidelines

Format:

```
<type>: <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `chore`: Maintenance tasks

Examples:

```
feat: Add Article 26 joint controllers validation

Implement validation for joint controller arrangements under
Article 26 GDPR. Includes builder pattern API and comprehensive
tests.

Closes #45

---

fix: Correct breach notification deadline calculation

The 72-hour deadline was incorrectly calculated from breach
occurrence rather than discovery. Fixed to comply with Article
33(1).

Fixes #67

---

docs: Add cross-border transfer examples

Add 3 comprehensive examples demonstrating:
- Adequacy decisions
- Standard contractual clauses
- Schrems II transfer impact assessment

```

### PR Checklist

Before submitting, ensure:

- [ ] Tests pass (`cargo nextest run`)
- [ ] No warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Documentation updated
- [ ] Examples added (if applicable)
- [ ] EUR-Lex sources cited
- [ ] CHANGELOG.md updated (for significant changes)

### Code Review Process

1. Maintainer reviews within 7 days
2. Address feedback
3. Maintainer approves
4. Maintainer merges (or contributor with permissions)

## Language Translation Guidelines

### EUR-Lex Verification

All legal text must come from official EUR-Lex sources:

```rust
// ✅ Good: Verified EUR-Lex text
let text = MultilingualText::from_eurlex(
    "Data Controller".to_string(),
    "Verantwortlicher".to_string(),  // From official German GDPR
    "CELEX:32016R0679".to_string(),
);

// ❌ Bad: Machine translation
let text = MultilingualText {
    en: "Data Controller".to_string(),
    de: Some("Datencontroller".to_string()),  // Not official term!
    source: None,
};
```

### Translation Checklist

- [ ] Text from official EUR-Lex version
- [ ] CELEX number documented
- [ ] Legal terminology verified (use official dictionaries)
- [ ] Tested with `cargo test`
- [ ] Added to abbreviation registry if needed

## Questions?

- Open an issue with the `question` label
- Check existing issues and documentation
- Ask in pull request comments

## Thank You!

Your contributions make legalis-eu better for everyone! 🚀
