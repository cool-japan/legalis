# Legalis-DE: German Law Library

Comprehensive Rust library for German law with structured types, validators, and examples.

## Overview

Legalis-DE provides type-safe Rust implementations of German legal statutes with:

- **Comprehensive Coverage**: 20+ German statutes implemented
- **Type Safety**: Enums and structs for all legal concepts
- **Bilingual**: German and English error messages
- **Validation**: Multi-stage validation with detailed errors
- **Examples**: 20+ working examples for all major features
- **Production Ready**: 365 tests, 0 warnings, ~25,000 lines of code

## Legal Areas Covered

### 1. Company Law (7,240 lines)

| Statute | Description | Module |
|---------|-------------|--------|
| **GmbHG** | Limited Liability Company (GmbH & UG) | `gmbhg` |
| **HGB** | Commercial Code (Partnerships: OHG, KG, GmbH & Co. KG) | `hgb` |
| **AktG** | Stock Corporation Act (AG) | `aktg` |

### 2. Civil Code - BGB (11,913 lines)

| Area | Sections | Module |
|------|----------|--------|
| **Contract Law** | §§104-361 (Obligations) | `bgb::schuldrecht` |
| **Tort Law** | §§823, 826 (Torts) | `bgb::unerlaubte_handlungen` |
| **Property Law** | §§873-1259 (Ownership, Pledges) | `bgb::sachenrecht` |
| **Family Law** | §§1303-1698 (Marriage, Divorce, Custody) | `bgb::familienrecht` |
| **Succession Law** | §§1922-2385 (Wills, Compulsory Portion) | `bgb::erbrecht` |

### 3. Constitutional Law - Grundgesetz (2,845 lines)

| Area | Articles | Module |
|------|----------|--------|
| **Basic Rights** | Art. 1-19 (Human Dignity, Freedoms) | `grundgesetz` |
| **State Organization** | Art. 20-146 (Parliament, Government) | `grundgesetz` |

### 4. Labor Law (3,057 lines)

| Statute | Area | Module |
|---------|------|--------|
| **BGB** | Individual Employment Law (§§611-630) | `arbeitsrecht` |
| **ArbZG** | Working Hours Act | `arbeitsrecht` |
| **BUrlG** | Federal Leave Act | `arbeitsrecht` |
| **KSchG** | Protection Against Dismissal Act | `arbeitsrecht` |
| **TVG** | Collective Bargaining Act | `arbeitsrecht` |
| **BetrVG** | Works Constitution Act | `arbeitsrecht` |
| **MitbestG** | Co-Determination Act | `arbeitsrecht` |

## Quick Start

### Installation

```toml
[dependencies]
legalis-de = "0.1.1"
```

### Example: GmbH Formation

```rust
use legalis_de::gmbhg::*;

// Create share capital of €25,000
let capital = Capital::from_euros(25_000);

// Validate
match validate_capital(&capital, CompanyType::GmbH) {
    Ok(()) => println!("✅ Share capital valid"),
    Err(e) => println!("❌ Error: {}", e),
}
```

### Example: Employment Contract

```rust
use legalis_de::arbeitsrecht::*;

let contract = EmploymentContract {
    employee_name: "John Doe".to_string(),
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    salary: Salary::Monthly {
        gross_amount: Capital::from_euros(4_500),
    },
    working_hours: WorkingHours {
        hours_per_week: 40,
        days_per_week: 5,
    },
    // ... more fields
};

validate_employment_contract(&contract)?;
```

## Documentation Structure

| File | Description |
|------|-------------|
| [GESELLSCHAFTSRECHT.en.md](GESELLSCHAFTSRECHT.en.md) | Company Law Guide |
| [BGB.en.md](BGB.en.md) | Civil Code Guide |
| [GRUNDGESETZ.en.md](GRUNDGESETZ.en.md) | Constitutional Law Guide |
| [ARBEITSRECHT.en.md](ARBEITSRECHT.en.md) | Labor Law Guide |
| [API.en.md](API.en.md) | API Reference and Usage |

German versions: `.md` extension (e.g. `README.md`)

## Key Features

### Type Safety

```rust
// Compiler prevents invalid states
let capital = Capital::from_cents(2_499_900); // €24,999
assert!(!capital.is_valid_for_gmbh()); // Too low for GmbH

let capital = Capital::from_euros(25_000);
assert!(capital.is_valid_for_gmbh()); // ✅ Valid
```

### Bilingual Errors

```rust
match validate_capital(&capital, CompanyType::GmbH) {
    Err(GmbHError::InsufficientCapital { actual, required }) => {
        // German error message with article reference
        println!("{}", e);
        // "Share capital of €24,999.00 is insufficient for GmbH.
        //  Required: €25,000.00 (§5 para. 1 GmbHG)"
    }
    Ok(()) => println!("✅ Valid"),
}
```

### Builder Patterns

```rust
use legalis_de::bgb::unerlaubte_handlungen::*;

let claim = TortClaim823_1Builder::new()
    .plaintiff("John Doe".to_string())
    .defendant("Accident GmbH".to_string())
    .protected_interest(ProtectedInterest::Body)
    .unlawful_act("Negligent traffic accident".to_string())
    .fault(Verschulden::GrosseNachlaessigkeit)
    .damage_amount(Capital::from_euros(15_000))
    .build()?;
```

## Legal Accuracy

**Important**: This library is for educational and development purposes. It does **not** constitute legal advice.

### Sources

All implementations based on:
- Official legal texts (dejure.org, gesetze-im-internet.de)
- Federal Court of Justice (BGH) decisions
- Constitutional Court (BVerfG) rulings
- Legal academic literature

### Article References

Every error contains precise statutory references:

```rust
pub fn article_reference(&self) -> &'static str {
    match self {
        GmbHError::InsufficientCapital { .. } => "§5 para. 1 GmbHG",
        GmbHError::InvalidCompanyName { .. } => "§4 GmbHG",
        // ...
    }
}
```

## Project Structure

```
jurisdictions/de/
├── src/
│   ├── gmbhg/          # GmbH Act
│   ├── hgb/            # Commercial Code
│   ├── aktg/           # Stock Corporation Act
│   ├── bgb/            # Civil Code
│   │   ├── schuldrecht/           # Contract Law
│   │   ├── unerlaubte_handlungen/ # Tort Law
│   │   ├── sachenrecht/           # Property Law
│   │   ├── familienrecht/         # Family Law
│   │   └── erbrecht/              # Succession Law
│   ├── grundgesetz/    # Constitutional Law
│   └── arbeitsrecht/   # Labor Law
├── examples/           # 20+ working examples
├── tests/             # 365 tests
├── docs/              # Documentation (DE/EN)
└── TODO.md            # Project roadmap
```

## Examples

All examples can be run with:

```bash
# GmbH formation
cargo run --example gmbh-formation-valid

# Contract formation
cargo run --example contract-formation

# Tort liability
cargo run --example tort-claim-823-1

# Employment contract
cargo run --example employment-contract-validation

# Collective bargaining agreement
cargo run --example collective-bargaining-agreement

# Supervisory board
cargo run --example supervisory-board-codetermination
```

## Testing

```bash
# Run all tests
cargo nextest run

# Specific module
cargo test gmbhg_validation_tests

# With coverage
cargo tarpaulin --out Html
```

## Quality Metrics

- **Tests**: 365 tests (365 passed, 0 failed)
- **Warnings**: 0 (cargo clippy --all-targets)
- **Lines of Code**: ~25,000 (source + tests + examples)
- **Documentation**: 100% public API documented
- **Statutes**: 20+ German statutes covered

## Contributing

Contributions welcome! Please note:

1. **Legal Accuracy**: All changes must cite official sources
2. **Tests**: New features require tests
3. **Documentation**: Update both German and English documentation
4. **Zero Warnings**: `cargo clippy` must pass cleanly

## License

See LICENSE file in main directory.

## Disclaimer

This software is for educational and development purposes only. It does not constitute legal advice and does not replace consultation with an attorney. The authors assume no liability for legal consequences arising from use of this software.

## Further Resources

- [API Documentation](API.en.md)
- [Examples](../examples/)
- [Project Roadmap](../TODO.md)
- [Changelog](../CHANGELOG.md)
