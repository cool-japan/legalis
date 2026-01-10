# Getting Started with Legalis-FR

This guide will help you add legalis-fr to your Rust project and write your first legal validation code.

## 📦 Installation

### Add to Cargo.toml

Add legalis-fr to your `Cargo.toml` dependencies:

```toml
[dependencies]
legalis-fr = "0.2.0"
legalis-core = "0.2.0"  # Core types and traits
chrono = "0.4"          # Date handling (required)
```

### Optional Dependencies

For specific use cases, you may want:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"                                   # JSON support
```

### Verify Installation

Create a simple test file to verify the installation:

```rust
// src/main.rs or tests/test_install.rs
use legalis_fr::contract::{Contract, ContractType};

fn main() {
    println!("Legalis-FR installed successfully!");

    let contract = Contract::builder()
        .contract_type(ContractType::Sale)
        .build();

    println!("Contract created: {:?}", contract);
}
```

Run with:
```bash
cargo run
# or
cargo test test_install
```

## 🎯 Your First Legal Validation

Let's create a simple program that validates a French employment contract.

### Example: Employment Contract Validation

```rust
use legalis_fr::labor::{Employment, validate_employment};
use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an employment contract
    let employment = Employment::builder()
        .employee_name("Marie Dupont")
        .employer_name("TechCorp SARL")
        .start_date(NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
        .end_date(None)  // Indefinite contract (CDI)
        .position("Software Engineer")
        .monthly_salary(3500)
        .weekly_hours(35.0)  // Legal 35-hour workweek
        .probation_period_months(Some(3))
        .build()?;

    // Validate the employment under French labor law
    match validate_employment(&employment) {
        Ok(_) => {
            println!("✅ Employment contract is valid under French law");
            println!("   Employee: {}", employment.employee_name);
            println!("   Salary: €{}/month", employment.monthly_salary);
            println!("   Weekly hours: {}", employment.weekly_hours);
        }
        Err(e) => {
            println!("❌ Invalid employment contract: {}", e);
        }
    }

    Ok(())
}
```

### What This Does

1. **Creates an employment contract** using the builder pattern
2. **Validates** it against French labor law (Code du travail)
3. **Checks**:
   - Minimum wage compliance (Article L3231-2)
   - Maximum working hours (Article L3121-27: 35h/week)
   - Probation period limits (Article L1221-19)
   - Required contract elements (Article L1221-1)

## 🔨 Common Patterns

### Pattern 1: Builder Pattern

All major types use builders for safe construction:

```rust
use legalis_fr::family::{Marriage, MarriageRegime};
use chrono::NaiveDate;

let marriage = Marriage::builder()
    .spouse1("Jean Martin", 28)
    .spouse2("Sophie Dubois", 26)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .regime(MarriageRegime::CommunityOfProperty)
    .build()?;  // Returns Result<Marriage, FamilyLawError>
```

### Pattern 2: Validation Functions

Each module provides validation functions:

```rust
use legalis_fr::contract::{Contract, validate_contract};

let contract = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Seller".to_string(), "Buyer".to_string()])
    .object("Apartment in Paris")
    .price(450_000)
    .build()?;

// Validate under Article 1128 (essential elements)
match validate_contract(&contract) {
    Ok(_) => println!("Valid contract"),
    Err(e) => println!("Invalid: {}", e),
}
```

### Pattern 3: Error Handling

All errors are bilingual and descriptive:

```rust
use legalis_fr::labor::LaborLawError;

match validate_employment(&employment) {
    Err(LaborLawError::MinimumWageViolation {
        actual,
        minimum,
        article
    }) => {
        println!("Salary €{} is below minimum €{}", actual, minimum);
        println!("Violates: {}", article);
        println!("French: {}",
            LaborLawError::MinimumWageViolation {
                actual,
                minimum,
                article: article.clone()
            }.message_fr()
        );
    }
    _ => {}
}
```

## 📚 Next Steps

### Explore Legal Domains

Choose the domain relevant to your use case:

```rust
// Contract Law
use legalis_fr::contract::{Contract, validate_contract};

// Labor Law
use legalis_fr::labor::{Employment, validate_employment};

// Family Law
use legalis_fr::family::{Marriage, validate_marriage};

// Inheritance Law
use legalis_fr::inheritance::{Succession, calculate_reserved_portions};

// Property Law
use legalis_fr::property::{Property, validate_easement};

// Intellectual Property
use legalis_fr::intellectual_property::{Patent, Copyright, Trademark};

// Evidence Law
use legalis_fr::evidence::{Evidence, assess_burden_of_proof};

// Company Law
use legalis_fr::company::{Company, validate_company_formation};

// Constitutional Law
use legalis_fr::constitution::{assess_constitutionality};

// Administrative Law
use legalis_fr::administrative::{AdministrativeAct, validate_act};

// Tort Law (Code civil Articles 1240-1244)
use legalis_fr::code_civil::{assess_tort_liability};
```

### Advanced Features

Once comfortable with basics, explore:

1. **[Legal Reasoning Engine](./user-guide.md#legal-reasoning-engine)** - Advanced case analysis
2. **[Custom Validators](./api-patterns.md#custom-validators)** - Extend validation logic
3. **[Serialization](./api-patterns.md#serialization)** - Save/load legal data
4. **[Comparative Law](./user-guide.md#comparative-law)** - Compare with German/Japanese law

## 🐛 Troubleshooting

### Issue: "Cannot find module legalis_fr"

**Solution**: Ensure you've added the dependency correctly:
```toml
[dependencies]
legalis-fr = "0.2.0"  # Note the hyphen, not underscore
```

Import with underscore:
```rust
use legalis_fr::contract::Contract;  // Underscore in code
```

### Issue: "Builder pattern returns an error"

**Solution**: Use the `?` operator or `match` to handle `Result`:

```rust
// Option 1: Use ? operator
let employment = Employment::builder()
    .employee_name("Marie Dupont")
    .build()?;  // Propagates error

// Option 2: Match on Result
match Employment::builder().build() {
    Ok(emp) => println!("Success: {:?}", emp),
    Err(e) => println!("Error: {}", e),
}
```

### Issue: "Missing required fields"

**Solution**: Check builder requirements. Most types require:
- Names/identifiers
- Dates (use `chrono::NaiveDate`)
- Numeric values (amounts, ages, etc.)

Use the type's documentation to see all required fields:
```bash
cargo doc --open -p legalis-fr
```

### Issue: "Date parsing fails"

**Solution**: Use `NaiveDate::from_ymd_opt()` and handle `Option`:

```rust
use chrono::NaiveDate;

// Correct
let date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();

// Or handle gracefully
let date = NaiveDate::from_ymd_opt(2023, 6, 15)
    .ok_or("Invalid date")?;
```

## 💡 Tips

1. **Start simple**: Begin with one legal domain relevant to your use case
2. **Read the docs**: Each module has extensive documentation with examples
3. **Check tests**: The `tests/` directory contains real-world examples
4. **Use type inference**: Let Rust infer types where possible
5. **Enable clippy**: `cargo clippy` helps catch common mistakes

## 📖 Learning Resources

- **[User Guide](./user-guide.md)** - Comprehensive examples for all domains
- **[API Patterns](./api-patterns.md)** - Best practices and design patterns
- **[Legal Domains](./legal-domains.md)** - Detailed overview of each domain
- **[API Documentation](https://docs.rs/legalis-fr)** - Full API reference

## ✅ Checklist

Before moving to the User Guide, ensure you can:

- [ ] Add legalis-fr to your `Cargo.toml`
- [ ] Import modules (e.g., `use legalis_fr::contract::Contract`)
- [ ] Create a type using the builder pattern
- [ ] Call a validation function
- [ ] Handle errors using `Result` and `?`
- [ ] Work with `chrono::NaiveDate` for dates

---

**Ready for more?** → Continue to the **[User Guide](./user-guide.md)** for comprehensive examples of all legal domains.
