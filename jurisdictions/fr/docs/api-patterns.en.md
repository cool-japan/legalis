# API Patterns and Best Practices

Learn how to effectively use the legalis-fr API with proven patterns and best practices.

## Table of Contents

1. [Builder Pattern](#1-builder-pattern)
2. [Error Handling](#2-error-handling)
3. [Validation Patterns](#3-validation-patterns)
4. [Serialization](#4-serialization)
5. [Custom Validators](#5-custom-validators)
6. [Integration Patterns](#6-integration-patterns)
7. [Performance Optimization](#7-performance-optimization)
8. [Testing Patterns](#8-testing-patterns)

---

## 1. Builder Pattern

All major types in legalis-fr use the builder pattern for safe, ergonomic construction.

### 1.1 Basic Builder Usage

```rust
use legalis_fr::contract::{Contract, ContractType};
use chrono::NaiveDate;

// Fluent, chainable API
let contract = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Alice".to_string(), "Bob".to_string()])
    .object("Apartment")
    .price(450_000)
    .formation_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .build()?;  // Returns Result<Contract, ContractLawError>
```

### 1.2 Optional Fields

Builders handle optional fields elegantly:

```rust
use legalis_fr::labor::Employment;

// With optional probation period
let employment1 = Employment::builder()
    .employee_name("Marie")
    .probation_period_months(Some(3))
    .build()?;

// Without probation period
let employment2 = Employment::builder()
    .employee_name("Pierre")
    .probation_period_months(None)
    .build()?;

// Or simply omit optional fields
let employment3 = Employment::builder()
    .employee_name("Sophie")
    .build()?;
```

### 1.3 Validation in Builders

Builders perform validation automatically:

```rust
use legalis_fr::family::Marriage;

// This will fail at build time
let result = Marriage::builder()
    .spouse1("Jean", 15)  // Too young (minimum age is 18)
    .spouse2("Sophie", 25)
    .build();

match result {
    Err(e) => {
        println!("Validation error: {}", e);
        // Error: "Spouse1 must be at least 18 years old (Article 144)"
    }
    _ => {}
}
```

### 1.4 Builder Pattern Benefits

1. **Type Safety**: Can't create invalid objects
2. **Ergonomics**: Fluent, readable API
3. **Validation**: Built-in checks at construction time
4. **Backwards Compatibility**: Easy to add new optional fields
5. **Self-Documenting**: Clear what fields are required

---

## 2. Error Handling

Legalis-FR provides rich, bilingual error types for all legal domains.

### 2.1 Error Types

Each module has its own error type:

```rust
use legalis_fr::contract::ContractLawError;
use legalis_fr::labor::LaborLawError;
use legalis_fr::family::FamilyLawError;
use legalis_fr::inheritance::InheritanceLawError;
// ... etc
```

### 2.2 Bilingual Error Messages

All errors support both English and French:

```rust
use legalis_fr::labor::{validate_minimum_wage, LaborLawError};

match validate_minimum_wage(1200, 35.0) {
    Err(LaborLawError::MinimumWageViolation { actual, minimum, article }) => {
        // English (default)
        println!("{}", LaborLawError::MinimumWageViolation {
            actual,
            minimum,
            article: article.clone()
        });

        // French
        println!("{}", LaborLawError::MinimumWageViolation {
            actual,
            minimum,
            article
        }.message_fr());
    }
    _ => {}
}
```

### 2.3 Structured Error Handling

Use pattern matching for precise error handling:

```rust
use legalis_fr::contract::{validate_contract, ContractLawError};

fn process_contract(contract: Contract) -> Result<(), Box<dyn std::error::Error>> {
    match validate_contract(&contract) {
        Ok(_) => {
            println!("✅ Contract valid");
            Ok(())
        }
        Err(ContractLawError::MissingEssentialElement { element }) => {
            eprintln!("❌ Missing: {}", element);
            // Handle missing element specifically
            Err("Contract incomplete".into())
        }
        Err(ContractLawError::InvalidConsent { reason }) => {
            eprintln!("❌ Invalid consent: {}", reason);
            // Handle consent issues specifically
            Err("Consent issue".into())
        }
        Err(e) => {
            eprintln!("❌ Other error: {}", e);
            Err(e.into())
        }
    }
}
```

### 2.4 Error Conversion

Convert between error types when needed:

```rust
use std::error::Error;

fn validate_employment_contract(
    employment: Employment,
) -> Result<(), Box<dyn Error>> {
    validate_employment(&employment)?;  // LaborLawError converted to Box<dyn Error>
    Ok(())
}
```

### 2.5 Error Logging

Integrate with logging frameworks:

```rust
use log::{error, warn, info};

match validate_contract(&contract) {
    Ok(_) => {
        info!("Contract validated successfully: {}", contract.object);
    }
    Err(e) => {
        error!("Contract validation failed: {}", e);
        error!("French: {}", e.message_fr());
        warn!("Contract: {:?}", contract);
    }
}
```

---

## 3. Validation Patterns

### 3.1 Immediate Validation

Validate immediately after construction:

```rust
// Pattern 1: Validate separately
let contract = Contract::builder().build()?;
validate_contract(&contract)?;

// Pattern 2: Validate in a single expression
let contract = {
    let c = Contract::builder().build()?;
    validate_contract(&c)?;
    c
};
```

### 3.2 Lazy Validation

Defer validation until needed:

```rust
struct ContractDraft {
    contract: Contract,
    validated: bool,
}

impl ContractDraft {
    fn validate(&mut self) -> Result<(), ContractLawError> {
        if !self.validated {
            validate_contract(&self.contract)?;
            self.validated = true;
        }
        Ok(())
    }

    fn finalize(mut self) -> Result<Contract, ContractLawError> {
        self.validate()?;
        Ok(self.contract)
    }
}
```

### 3.3 Batch Validation

Validate multiple items and collect errors:

```rust
fn validate_all_contracts(
    contracts: Vec<Contract>
) -> (Vec<Contract>, Vec<(usize, ContractLawError)>) {
    let mut valid = Vec::new();
    let mut errors = Vec::new();

    for (i, contract) in contracts.into_iter().enumerate() {
        match validate_contract(&contract) {
            Ok(_) => valid.push(contract),
            Err(e) => errors.push((i, e)),
        }
    }

    (valid, errors)
}
```

### 3.4 Conditional Validation

Apply different validation rules based on context:

```rust
fn validate_contract_context(
    contract: &Contract,
    is_online: bool,
) -> Result<(), ContractLawError> {
    // Always validate essentials
    validate_contract(contract)?;

    // Additional validation for online contracts
    if is_online {
        if contract.formation_date > chrono::Utc::now().naive_utc().date() {
            return Err(ContractLawError::InvalidFormationDate {
                date: contract.formation_date,
            });
        }
    }

    Ok(())
}
```

---

## 4. Serialization

### 4.1 JSON Serialization

Add `serde` support to your types:

```rust
use serde::{Serialize, Deserialize};
use legalis_fr::contract::Contract;

#[derive(Serialize, Deserialize)]
struct ContractData {
    #[serde(flatten)]
    contract: Contract,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    created_at: String,
    created_by: String,
}

// Serialize to JSON
let json = serde_json::to_string(&contract_data)?;

// Deserialize from JSON
let loaded: ContractData = serde_json::from_str(&json)?;
```

### 4.2 Database Integration

Store legal data in databases:

```rust
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct ContractRow {
    id: i32,
    contract_type: String,
    parties: Vec<String>,
    object: String,
    price: i64,
    formation_date: chrono::NaiveDate,
}

async fn save_contract(
    pool: &PgPool,
    contract: &Contract,
) -> Result<i32, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO contracts (contract_type, parties, object, price, formation_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        format!("{:?}", contract.contract_type),
        &contract.parties,
        contract.object,
        contract.price as i64,
        contract.formation_date,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
```

### 4.3 File Storage

Save/load legal data from files:

```rust
use std::fs;

fn save_to_file(contract: &Contract, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(contract)?;
    fs::write(path, json)?;
    Ok(())
}

fn load_from_file(path: &str) -> Result<Contract, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let contract = serde_json::from_str(&json)?;
    Ok(contract)
}
```

---

## 5. Custom Validators

### 5.1 Extending Validation

Add custom validation logic:

```rust
use legalis_fr::contract::{Contract, validate_contract, ContractLawError};

fn validate_contract_extended(
    contract: &Contract,
) -> Result<(), ContractLawError> {
    // Standard validation
    validate_contract(contract)?;

    // Custom business rules
    if contract.price > 1_000_000 {
        // Require notary for high-value contracts
        if !contract.notarized {
            return Err(ContractLawError::CustomValidation {
                reason: "Contracts over €1M require notarization".to_string(),
            });
        }
    }

    Ok(())
}
```

### 5.2 Validator Composition

Combine multiple validators:

```rust
fn validate_all(contract: &Contract) -> Result<(), ContractLawError> {
    validate_contract(contract)?;
    validate_contract_extended(contract)?;
    validate_parties(contract)?;
    validate_jurisdiction(contract)?;
    Ok(())
}

fn validate_parties(contract: &Contract) -> Result<(), ContractLawError> {
    if contract.parties.len() < 2 {
        return Err(ContractLawError::InvalidParties {
            reason: "At least 2 parties required".to_string(),
        });
    }
    Ok(())
}

fn validate_jurisdiction(contract: &Contract) -> Result<(), ContractLawError> {
    // Custom jurisdiction validation
    Ok(())
}
```

### 5.3 Validation Pipeline

Create a validation pipeline:

```rust
type ValidationFn = fn(&Contract) -> Result<(), ContractLawError>;

struct ValidationPipeline {
    validators: Vec<ValidationFn>,
}

impl ValidationPipeline {
    fn new() -> Self {
        Self { validators: Vec::new() }
    }

    fn add(mut self, validator: ValidationFn) -> Self {
        self.validators.push(validator);
        self
    }

    fn validate(&self, contract: &Contract) -> Result<(), ContractLawError> {
        for validator in &self.validators {
            validator(contract)?;
        }
        Ok(())
    }
}

// Usage
let pipeline = ValidationPipeline::new()
    .add(validate_contract)
    .add(validate_contract_extended)
    .add(validate_parties);

pipeline.validate(&contract)?;
```

---

## 6. Integration Patterns

### 6.1 Web API Integration

```rust
use actix_web::{web, HttpResponse, Result};
use legalis_fr::contract::{Contract, validate_contract};

async fn validate_contract_endpoint(
    contract: web::Json<Contract>,
) -> Result<HttpResponse> {
    match validate_contract(&contract) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "valid": true,
            "message": "Contract is valid under French law"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "valid": false,
            "error": e.to_string(),
            "error_fr": e.message_fr(),
        }))),
    }
}
```

### 6.2 gRPC Integration

```rust
use tonic::{Request, Response, Status};

pub struct LegalService;

#[tonic::async_trait]
impl legal_service_server::LegalService for LegalService {
    async fn validate_contract(
        &self,
        request: Request<ValidateContractRequest>,
    ) -> Result<Response<ValidateContractResponse>, Status> {
        let contract = request.into_inner().contract
            .ok_or_else(|| Status::invalid_argument("Contract required"))?;

        match validate_contract(&contract) {
            Ok(_) => Ok(Response::new(ValidateContractResponse {
                valid: true,
                errors: vec![],
            })),
            Err(e) => Ok(Response::new(ValidateContractResponse {
                valid: false,
                errors: vec![e.to_string()],
            })),
        }
    }
}
```

### 6.3 Event-Driven Architecture

```rust
use tokio::sync::mpsc;

enum LegalEvent {
    ContractCreated(Contract),
    ContractValidated(Contract),
    ContractRejected(Contract, ContractLawError),
}

async fn process_contracts(mut rx: mpsc::Receiver<Contract>) {
    while let Some(contract) = rx.recv().await {
        match validate_contract(&contract) {
            Ok(_) => {
                // Publish validated event
                publish_event(LegalEvent::ContractValidated(contract)).await;
            }
            Err(e) => {
                // Publish rejection event
                publish_event(LegalEvent::ContractRejected(contract, e)).await;
            }
        }
    }
}
```

---

## 7. Performance Optimization

### 7.1 Lazy Evaluation

Defer expensive operations:

```rust
struct LazyContract {
    contract: Contract,
    validated: Option<Result<(), ContractLawError>>,
}

impl LazyContract {
    fn new(contract: Contract) -> Self {
        Self {
            contract,
            validated: None,
        }
    }

    fn is_valid(&mut self) -> bool {
        if self.validated.is_none() {
            self.validated = Some(validate_contract(&self.contract));
        }
        self.validated.as_ref().unwrap().is_ok()
    }
}
```

### 7.2 Caching

Cache validation results:

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct ValidationCache {
    cache: HashMap<u64, Result<(), ContractLawError>>,
}

impl ValidationCache {
    fn validate(&mut self, contract: &Contract) -> Result<(), ContractLawError> {
        let key = calculate_hash(contract);

        if let Some(cached) = self.cache.get(&key) {
            return cached.clone();
        }

        let result = validate_contract(contract);
        self.cache.insert(key, result.clone());
        result
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
```

### 7.3 Parallel Validation

Validate multiple items in parallel:

```rust
use rayon::prelude::*;

fn validate_contracts_parallel(
    contracts: Vec<Contract>
) -> Vec<Result<(), ContractLawError>> {
    contracts
        .par_iter()
        .map(|c| validate_contract(c))
        .collect()
}
```

---

## 8. Testing Patterns

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use legalis_fr::contract::{Contract, ContractType};

    #[test]
    fn test_valid_contract() {
        let contract = Contract::builder()
            .contract_type(ContractType::Sale)
            .parties(vec!["Alice".to_string(), "Bob".to_string()])
            .object("Test object")
            .price(1000)
            .build()
            .unwrap();

        assert!(validate_contract(&contract).is_ok());
    }

    #[test]
    fn test_invalid_contract_missing_parties() {
        let contract = Contract::builder()
            .contract_type(ContractType::Sale)
            .parties(vec![])
            .build();

        assert!(contract.is_err());
    }
}
```

### 8.2 Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use legalis_fr::contract::{Contract, validate_contract};
    use legalis_fr::labor::{Employment, validate_employment};

    #[test]
    fn test_complete_workflow() {
        // Create contract
        let contract = Contract::builder()
            .build()
            .unwrap();

        // Validate
        validate_contract(&contract).unwrap();

        // Create employment from contract
        let employment = Employment::from_contract(&contract)
            .unwrap();

        // Validate employment
        validate_employment(&employment).unwrap();
    }
}
```

### 8.3 Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use legalis_fr::contract::Contract;

    proptest! {
        #[test]
        fn test_contract_price_never_negative(price in 0u64..1_000_000) {
            let contract = Contract::builder()
                .price(price)
                .build()
                .unwrap();

            assert!(contract.price >= 0);
        }
    }
}
```

---

## Best Practices Summary

1. **Always use builders** for type construction
2. **Handle errors explicitly** with pattern matching
3. **Validate early** to catch issues quickly
4. **Use bilingual errors** for international applications
5. **Leverage type safety** to prevent invalid states
6. **Test thoroughly** with unit, integration, and property tests
7. **Cache results** for expensive validations
8. **Document custom validators** for maintainability

---

**Next**: [Legal Domains Overview](./legal-domains.md)

**Previous**: [User Guide](./user-guide.md)
