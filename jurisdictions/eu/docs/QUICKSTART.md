# Quick Start Guide - legalis-eu

Get started with EU law modeling in under 5 minutes!

## Table of Contents

- [Installation](#installation)
- [Your First GDPR Validation](#your-first-gdpr-validation)
- [Your First IP Validation](#your-first-ip-validation)
- [Common Patterns](#common-patterns)
- [Next Steps](#next-steps)

## Installation

Add `legalis-eu` to your `Cargo.toml`:

```toml
[dependencies]
legalis-eu = "0.5.9"
chrono = "0.4"  # Required for date/time handling
```

Or use cargo add:

```bash
cargo add legalis-eu chrono
```

## Your First GDPR Validation

Let's validate a simple GDPR data processing operation based on consent:

```rust
use legalis_eu::gdpr::*;

fn main() {
    // Create a data processing operation
    let processing = DataProcessing::new()
        .with_controller("My Company Ltd")
        .with_purpose("Send marketing newsletters")
        .add_data_category(PersonalDataCategory::Regular("email".to_string()))
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        });

    // Validate compliance
    match processing.validate() {
        Ok(validation) => {
            if validation.is_compliant() {
                println!("✅ Your processing is GDPR compliant!");
                println!("   Lawful basis: Consent (Article 6(1)(a))");
            } else {
                println!("⚠️ Compliance issues found");
            }
        }
        Err(e) => {
            println!("❌ Validation error: {}", e);
        }
    }
}
```

**Run it:**
```bash
cargo run
```

**Output:**
```
✅ Your processing is GDPR compliant!
   Lawful basis: Consent (Article 6(1)(a))
```

## Your First IP Validation

Let's validate an EU trademark application:

```rust
use legalis_eu::intellectual_property::*;

fn main() {
    // Create a trademark application
    let trademark = EuTrademark::new()
        .with_mark_text("INNOVATECH")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("Tech Innovations GmbH")
        .add_nice_class(9)   // Computer software
        .unwrap()
        .add_nice_class(42)  // Software services
        .unwrap()
        .add_goods_services("Computer software for data analysis");

    // Validate registrability
    match trademark.validate() {
        Ok(validation) => {
            if validation.is_registrable {
                println!("✅ Trademark is registrable!");
                println!("   Nice Classes: 9, 42");
                println!("   Distinctiveness: {}", validation.distinctiveness_established);
            }
        }
        Err(e) => {
            println!("❌ Trademark issue: {}", e);
        }
    }
}
```

## Common Patterns

### 1. Builder Pattern

All major types use the fluent builder pattern:

```rust
let processing = DataProcessing::new()
    .with_controller("Company")
    .with_purpose("Analytics")
    .add_data_category(PersonalDataCategory::Regular("user_id".into()))
    .with_lawful_basis(LawfulBasis::LegitimateInterests {
        controller_interest: "Improve service quality".into(),
        balancing_test_passed: true,
    });
```

### 2. Validation Pattern

All validations return `Result<ValidationStruct, ErrorType>`:

```rust
match entity.validate() {
    Ok(validation) => {
        // Check validation results
        if validation.is_compliant() {
            // Success case
        }
    }
    Err(e) => {
        // Handle error
        eprintln!("Error: {}", e);
    }
}
```

### 3. Error Handling

Errors are strongly typed and descriptive:

```rust
use legalis_eu::gdpr::GdprError;

match processing.validate() {
    Ok(v) => println!("Valid"),
    Err(GdprError::MissingLawfulBasis { message }) => {
        println!("No lawful basis specified: {}", message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Common Use Cases

### Check if Data Breach Must Be Reported

```rust
use legalis_eu::gdpr::*;
use chrono::Utc;

let breach = DataBreach::new()
    .with_controller("My Company")
    .with_breach_category(BreachCategory::ConfidentialityBreach)
    .with_discovered_at(Utc::now() - chrono::Duration::hours(60))
    .with_affected_count(1000);

match breach.validate() {
    Ok(validation) => {
        if validation.supervisory_authority_notification_required {
            let hours = validation.hours_since_discovery;
            if hours > 72 {
                println!("⚠️ 72-hour deadline EXCEEDED!");
            } else {
                println!("⏰ {} hours remaining to notify authority", 72 - hours);
            }
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Calculate Consumer Withdrawal Period

```rust
use legalis_eu::consumer_rights::*;
use chrono::Utc;

let contract = DistanceContract::new()
    .with_trader("Online Shop GmbH")
    .with_consumer("Customer Name")
    .with_contract_date(Utc::now() - chrono::Duration::days(5))
    .with_goods_description("Laptop computer");

match contract.calculate_withdrawal_period() {
    Ok(period) => {
        println!("Days remaining: {}", period.days_remaining);
        println!("Deadline: {}", period.deadline);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Validate Trade Secret Protection

```rust
use legalis_eu::intellectual_property::*;

let secret = TradeSecret::new()
    .with_description("Proprietary algorithm")
    .with_holder("Tech Corp")
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: true,
        has_commercial_value: true,
        reasonable_steps_taken: true,
    })
    .add_protective_measure("NDA with all employees")
    .add_protective_measure("Access control to source code");

match secret.validate() {
    Ok(validation) => {
        if validation.three_part_test_passed {
            println!("✅ Protected as trade secret under Directive 2016/943");
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Next Steps

### 📚 Read Detailed Guides

- [GDPR Guide](GDPR_GUIDE.md) - Complete GDPR implementation guide
- [IP Guide](IP_GUIDE.md) - Intellectual Property protection guide
- [FAQ](FAQ.md) - Frequently asked questions
- [Contributing Guide](CONTRIBUTING.md) - How to contribute to the project

### 🔍 Explore Examples

Run the 23 included examples:

```bash
# GDPR examples
cargo run --example gdpr_consent_validation
cargo run --example gdpr_dpia_workflow
cargo run --example gdpr_cross_border_transfers

# IP examples
cargo run --example ip_eu_trademark
cargo run --example ip_copyright
cargo run --example ip_comprehensive

# See all examples
ls examples/
```

### 📖 API Documentation

Generate and browse the full API documentation:

```bash
cargo doc --open
```

### 🧪 Run Tests

See the crate in action with the comprehensive test suite:

```bash
cargo test
# or with nextest (recommended)
cargo nextest run
```

## Common Questions

**Q: Which GDPR lawful basis should I use?**

A: It depends on your use case:
- **Consent**: Marketing, optional features (must be freely given)
- **Contract**: Necessary for performing a contract (e.g., order fulfillment)
- **Legal obligation**: Required by law (e.g., tax records)
- **Legitimate interests**: Requires balancing test (e.g., fraud prevention)

See the [GDPR Guide](GDPR_GUIDE.md#choosing-a-lawful-basis) for details.

**Q: How do I handle special categories of data?**

A: Use `Article9Processing` with appropriate exceptions:

```rust
let processing = Article9Processing::new()
    .with_controller("Hospital")
    .add_special_category(SpecialCategory::HealthData)
    .with_exception(Article9Exception::ExplicitConsent {
        purposes: vec!["Medical treatment".into()],
        consent_documented: true,
    });
```

**Q: Is this production-ready?**

A: Yes! The crate has:
- 173 tests passing (0 warnings)
- Sub-microsecond performance
- Comprehensive validation logic
- Active maintenance

**Q: What EU languages are supported?**

A: Currently English and German. French, Spanish, and Italian support planned for v0.7.0.

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/cool-japan/legalis/issues)
- **Documentation**: Run `cargo doc --open`
- **Examples**: Check the `examples/` directory
- **Guides**: Read the docs/ directory

## Quick Reference

### Import Paths

```rust
use legalis_eu::gdpr::*;                    // GDPR types
use legalis_eu::intellectual_property::*;   // IP types
use legalis_eu::consumer_rights::*;         // Consumer protection
use legalis_eu::competition::*;             // Competition law
use legalis_eu::treaty::*;                  // Treaty framework
```

### Key Types

- **GDPR**: `DataProcessing`, `DataBreach`, `DataSubjectRequest`, `CrossBorderTransfer`
- **IP**: `EuTrademark`, `CommunityDesign`, `CopyrightWork`, `TradeSecret`
- **Consumer**: `DistanceContract`, `WithdrawalRight`
- **Competition**: `Article101Agreement`, `Article102Conduct`

Happy coding! 🚀
