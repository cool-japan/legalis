# API Reference

Practical guide to using the Legalis-DE API.

## Installation

```toml
[dependencies]
legalis-de = "0.1.1"
chrono = "0.4"  # For date values
serde = { version = "1.0", features = ["derive"] }  # Optional: Serialization
```

## Basic Usage

### Import Modules

```rust
// Company Law
use legalis_de::gmbhg::*;
use legalis_de::hgb::*;
use legalis_de::aktg::*;

// Civil Code
use legalis_de::bgb::schuldrecht::*;
use legalis_de::bgb::unerlaubte_handlungen::*;

// Constitutional Law
use legalis_de::grundgesetz::*;

// Labor Law
use legalis_de::arbeitsrecht::*;

// Utilities
use chrono::NaiveDate;
```

### Create Capital Amount

```rust
use legalis_de::gmbhg::Capital;

// From euro amount
let capital = Capital::from_euros(25_000);
assert_eq!(capital.to_euros(), 25_000.0);

// From cent amount
let capital = Capital::from_cents(2_500_000);
assert_eq!(capital.to_cents(), 2_500_000);

// Operations
let sum = capital1.add(&capital2);
let diff = capital1.subtract(&capital2)?; // Error if negative
```

## Validation Patterns

### Simple Validation

```rust
use legalis_de::gmbhg::*;

let capital = Capital::from_euros(25_000);

match validate_capital(&capital, CompanyType::GmbH) {
    Ok(()) => println!("✅ Valid"),
    Err(e) => {
        println!("❌ Error: {}", e);
        println!("   Legal basis: {}", e.article_reference());
    }
}
```

### Structured Error Handling

```rust
use legalis_de::gmbhg::{GmbHError, Result};

fn process_gmbh(capital: &Capital) -> Result<()> {
    validate_capital(capital, CompanyType::GmbH)?;
    // Further processing...
    Ok(())
}

match process_gmbh(&capital) {
    Ok(()) => println!("Success"),
    Err(GmbHError::InsufficientCapital { actual, required }) => {
        eprintln!("Capital too low: €{:.2} < €{:.2}",
                  actual.to_euros(), required.to_euros());
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Error Methods

All error types implement helper methods:

```rust
let error = GmbHError::InsufficientCapital {
    actual: Capital::from_euros(20_000),
    required: Capital::from_euros(25_000),
};

// Statutory reference
assert_eq!(error.article_reference(), "§5 para. 1 GmbHG");

// Check validity
if error.makes_contract_void() {
    println!("Contract void");
} else if error.makes_contract_voidable() {
    println!("Contract voidable");
}
```

## Builder Patterns

### TortClaim Builder

```rust
use legalis_de::bgb::unerlaubte_handlungen::*;

let claim = TortClaim823_1Builder::new()
    .plaintiff("John Doe".to_string())
    .defendant("Tortfeasor GmbH".to_string())
    .protected_interest(ProtectedInterest::Property)
    .unlawful_act("Negligent property damage".to_string())
    .fault(Verschulden::OrdinaryNegligence)
    .damage_amount(Capital::from_euros(5_000))
    .build()?;

validate_tort_claim_823_1(&claim)?;
```

### Movable Property Builder

```rust
use legalis_de::bgb::sachenrecht::*;

let transfer = MovableTransferBuilder::new()
    .transferor("Seller".to_string())
    .transferee("Buyer".to_string())
    .item_description("Used car VW Golf".to_string())
    .transfer_agreement_exists(true)
    .delivery_completed(true)
    .transferor_is_owner(true)
    .build()?;
```

## Serialization

All types support Serde:

```rust
use legalis_de::gmbhg::*;
use serde_json;

let capital = Capital::from_euros(25_000);

// To JSON
let json = serde_json::to_string(&capital)?;
println!("{}", json); // {"cents":2500000}

// From JSON
let deserialized: Capital = serde_json::from_str(&json)?;
assert_eq!(capital, deserialized);
```

## Date Handling

```rust
use chrono::NaiveDate;

// Create date
let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

// Comparisons
let later = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
assert!(later > date);

// Calculations
use chrono::Duration;
let future = date + Duration::days(30);
```

## Typical Workflows

### Workflow 1: GmbH Formation

```rust
use legalis_de::gmbhg::*;

// 1. Check capital
let capital = Capital::from_euros(25_000);
validate_capital(&capital, CompanyType::GmbH)?;

// 2. Articles of association
let articles = ArticlesOfAssociation {
    company_name: "My GmbH".to_string(),
    share_capital: capital,
    // ... more fields
};
validate_articles_of_association(&articles)?;

// 3. Managing directors
let directors = ManagingDirectors {
    // ...
};
validate_managing_directors(&directors)?;

println!("✅ GmbH formation validated");
```

### Workflow 2: Contract Formation

```rust
use legalis_de::bgb::schuldrecht::*;

// 1. Create parties
let seller = Party {
    name: "Seller".to_string(),
    legal_capacity: LegalCapacity::Full,
    // ...
};

// 2. Offer
let offer = Offer {
    offeror: seller.clone(),
    offeree: buyer.clone(),
    terms: ContractTerms { /* ... */ },
    // ...
};
validate_offer(&offer)?;

// 3. Acceptance
let acceptance = Acceptance {
    offer: offer.clone(),
    accepted_at: Utc::now(),
    // ...
};
validate_acceptance(&acceptance, &offer)?;

// 4. Contract
let contract = Contract {
    parties: (seller, buyer),
    terms: offer.terms.clone(),
    // ...
};
validate_contract(&contract)?;
```

### Workflow 3: Dismissal Check

```rust
use legalis_de::arbeitsrecht::*;

let dismissal = Dismissal {
    employee_name: "Employee".to_string(),
    dismissal_type: DismissalType::Ordinary,
    dismissal_ground: DismissalGround::Operational("Site closure".to_string()),
    notice_period_weeks: 4,
    works_council_consulted: true,
    written_form: true,
    // ...
};

// Comprehensive validation
validate_dismissal(&dismissal)?;
```

## Common Patterns

### Pattern Matching

```rust
match validate_employment_contract(&contract) {
    Ok(()) => {
        // Success
    }
    Err(LaborLawError::WorkingHoursExceedLimit { hours, limit }) => {
        println!("Working time {}h exceeds limit {}h", hours, limit);
    }
    Err(LaborLawError::InsufficientLeave { provided, required }) => {
        println!("Leave insufficient: {} < {} days", provided, required);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

### Conditional Checks

```rust
// Check capital
if capital.is_valid_for_gmbh() {
    println!("Suitable for GmbH");
}

// Works council required?
if WorksCouncil::is_required(employee_count) {
    let size = WorksCouncil::required_size(employee_count);
    // Form works council
}

// Working hours compliant?
if working_hours.complies_with_arbzg() {
    println!("ArbZG-compliant");
}
```

### Iterations

```rust
// Over shareholders
for shareholder in &articles.shareholders {
    println!("{}: {}%",
             shareholder.name,
             shareholder.share_allocation.percentage);
}

// Co-determination rights
for right in &codetermination_rights.rights {
    println!("{:?}: {}", right.right_type, right.legal_basis);
}
```

## Performance Tips

### Clone vs. Borrow

```rust
// Good: Borrow when possible
fn validate(capital: &Capital) -> Result<()> {
    validate_capital(capital, CompanyType::GmbH)
}

// Less good: Clone when necessary
fn store(capital: Capital) -> Capital {
    capital // Transfer ownership
}
```

### Error Propagation

```rust
// Efficient with ?
fn process() -> Result<()> {
    validate_capital(&capital, CompanyType::GmbH)?;
    validate_articles(&articles)?;
    validate_directors(&directors)?;
    Ok(())
}
```

### Pre-computations

```rust
// Cache values if used multiple times
let required_size = WorksCouncil::required_size(employee_count);
let required_type = SupervisoryBoard::required_codetermination(employee_count);

// Instead of calling multiple times
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gmbh_capital() {
        let capital = Capital::from_euros(25_000);
        assert!(validate_capital(&capital, CompanyType::GmbH).is_ok());
    }

    #[test]
    fn test_invalid_gmbh_capital() {
        let capital = Capital::from_euros(20_000);
        let result = validate_capital(&capital, CompanyType::GmbH);
        assert!(result.is_err());

        match result {
            Err(GmbHError::InsufficientCapital { .. }) => {},
            _ => panic!("Wrong error type"),
        }
    }
}
```

### Integration Tests

```rust
#[test]
fn test_complete_gmbh_formation() -> Result<()> {
    let articles = create_test_articles();
    validate_articles_of_association(&articles)?;

    let directors = create_test_directors();
    validate_managing_directors(&directors)?;

    Ok(())
}
```

## Further Resources

- [Examples](../examples/)
- [Tests](../tests/)
- [TODO](../TODO.md)
- [Rust Documentation](https://docs.rs/legalis-de/)
