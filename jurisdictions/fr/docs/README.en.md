# Legalis-FR Documentation

Welcome to the **legalis-fr** documentation - a comprehensive Rust implementation of French law for legal reasoning and compliance applications.

## 📚 Documentation Structure

- **[Getting Started](./getting-started.md)** - Quick start guide to add legalis-fr to your project
- **[User Guide](./user-guide.md)** - Comprehensive guide with examples for all legal domains
- **[API Patterns](./api-patterns.md)** - Best practices for working with the legalis-fr API
- **[Legal Domains](./legal-domains.md)** - Overview of all 11 available legal domains

## 🎯 What is Legalis-FR?

Legalis-FR is a production-ready Rust crate providing:

- **11 legal domains** covering French civil law, labor law, constitutional law, and more
- **524 comprehensive tests** ensuring legal accuracy
- **Bilingual documentation** (French/English) with 69.7% doc-to-code ratio
- **Legal Reasoning Engine** for advanced legal analysis and case evaluation
- **Type-safe API** preventing invalid legal states at compile time

## 🚀 Quick Example

```rust
use legalis_fr::labor::{Employment, TerminationReason, validate_termination};
use chrono::NaiveDate;

// Create an employment contract
let employment = Employment::builder()
    .employee_name("Marie Dupont")
    .employer_name("TechCorp SARL")
    .start_date(NaiveDate::from_ymd_opt(2020, 1, 15).unwrap())
    .position("Software Engineer")
    .monthly_salary(3500)
    .build()
    .unwrap();

// Validate a termination under French labor law (Article L1234-1)
let termination = validate_termination(
    &employment,
    TerminationReason::EconomicDismissal,
    NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
);

match termination {
    Ok(result) => println!("Valid termination: {:?}", result),
    Err(e) => println!("Invalid: {}", e),
}
```

## 🌍 Language Support

All documentation and error messages are available in **both French and English**:

```rust
use legalis_fr::contract::ContractLawError;

let error = ContractLawError::InvalidConsent {
    reason: "Duress detected".to_string(),
};

// English error message
println!("{}", error);

// French error message
println!("{}", error.message_fr());
```

## 📖 Core Concepts

### 1. Legal Domains

Legalis-FR organizes French law into 11 specialized domains:

- **Contract Law** (Code civil, Book III)
- **Labor Law** (Code du travail)
- **Family Law** (Code civil, Book I)
- **Inheritance Law** (Code civil, Book III)
- **Property Law** (Code civil, Book II)
- **Company Law** (Code de commerce)
- **Evidence Law** (Code civil, Book III, Title XX)
- **Intellectual Property** (Code de la propriété intellectuelle)
- **Constitutional Law** (Constitution of 1958)
- **Administrative Law** (Code de justice administrative)
- **Tort Law** (Code civil, Articles 1240-1244)

### 2. Legal Reasoning Engine

The **Reasoning Engine** provides advanced legal analysis:

```rust
use legalis_fr::reasoning::{LegalCase, apply_legal_reasoning};

let case = LegalCase::builder()
    .facts(vec!["Contract signed under duress".to_string()])
    .legal_question("Is the contract valid?")
    .build()
    .unwrap();

let result = apply_legal_reasoning(case);
println!("Legal conclusion: {}", result.conclusion);
```

### 3. Type Safety

Legalis-FR uses Rust's type system to enforce legal validity:

```rust
// This won't compile - invalid state prevented at compile time
let invalid_marriage = Marriage {
    spouse1_age: 15,  // Error: Age must be at least 18
    // ...
};

// Use builders with validation
let valid_marriage = Marriage::builder()
    .spouse1("Jean Martin", 25)
    .spouse2("Sophie Dubois", 23)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .build()?;  // Returns Result<Marriage, FamilyLawError>
```

## 🔗 Related Resources

- **[Main README](../README.md)** - Project overview and statistics
- **[Cargo.toml](../Cargo.toml)** - Dependencies and crate metadata
- **[Source Code](../src/)** - Implementation details
- **[Tests](../tests/)** - Integration tests and examples

## 💡 Use Cases

Legalis-FR is designed for:

- **Legal Tech Applications** - Contract analysis, compliance checking
- **HR Systems** - Employment law validation, termination procedures
- **Real Estate Platforms** - Property transactions, easement validation
- **Estate Planning Tools** - Inheritance calculations, will validation
- **IP Management Systems** - Patent/trademark validation, copyright analysis
- **Academic Research** - Comparative law studies, legal reasoning research

## 🤝 Contributing

Found an issue or want to improve the documentation? Contributions are welcome!

1. Check the [main repository](https://github.com/your-org/legalis-rs)
2. Review existing issues and pull requests
3. Follow the contribution guidelines

## 📄 License

Legalis-FR is part of the legalis-rs framework. See the main repository for license information.

---

**Ready to get started?** → [Getting Started Guide](./getting-started.md)

**Need examples?** → [User Guide](./user-guide.md)

**Want to understand the API?** → [API Patterns](./api-patterns.md)
