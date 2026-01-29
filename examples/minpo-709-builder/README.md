# Article 709 Builder API Demo

Demonstration of the enhanced builder API for Japanese Civil Code Article 709 (不法行為 / Tort Liability).

## Overview

This example showcases the ergonomic builder pattern for constructing and validating tort claims under Article 709 of the Japanese Civil Code. The builder API provides type-safe construction with compile-time guarantees that all required elements are present.

## What is Article 709?

Article 709 of the Japanese Civil Code establishes the general principle of tort liability:

> A person who intentionally or negligently infringes upon another person's rights or legally protected interests and thereby causes damage shall be liable for damages arising therefrom.

## Features

### Builder Pattern Advantages

- **Type Safety**: Rust's type system ensures all required fields are provided
- **Ergonomic API**: Fluent interface for natural claim construction
- **Automatic Validation**: Built-in validation of legal requirements
- **Clear Error Messages**: Descriptive errors when requirements aren't met

### Three Demonstration Scenarios

1. **Basic Tort Check** - Simple traffic accident with property damage
2. **Detailed Validation** - Bicycle-pedestrian accident with injury
3. **Supervisor Liability** - Child's tort and parent's responsibility (Article 715/714 connection)

### Requirements Validated

- **Act** (行為) - Tortious conduct
- **Intent or Negligence** (故意・過失) - Mental element
- **Protected Interest** (権利または法律上保護される利益) - Rights infringement
- **Damages** (損害) - Quantifiable harm
- **Causation** (因果関係) - Causal link
- **Responsibility Capacity** (責任能力) - Legal capacity to bear liability

## Usage

```bash
cargo run --bin minpo-709-builder
```

Or from the subcrate directory:

```bash
cargo run
```

## Builder API Examples

### Simple Construction

```rust
let claim = Article709::new()
    .with_act("交通事故で相手の車に衝突")
    .with_intent(Intent::Negligence)
    .with_victim_interest(ProtectedInterest::Property("車両所有権"))
    .with_damage(Damage::new(500_000, "修理費 + レッカー代"))
    .with_causal_link(CausalLink::Direct);

let result = validate_tort_claim(&claim);
```

### Detailed Validation

```rust
let claim = Article709::builder()
    .act("歩行者を自転車でひいた")
    .intent(Intent::NegligenceWithDuty {
        duty_of_care: "前方不注視".to_string(),
    })
    .injured_interest(ProtectedInterest::BodyAndHealth)
    .damage(Damage::new(3_000_000, "治療費 + 慰謝料 + 休業損害"))
    .causal_link(CausalLink::Adequate("事故がなければ損害発生せず"))
    .responsibility_capacity(true)
    .build();
```

## Key Concepts

### Intent Types

- **Intentional** (故意) - Deliberate harmful act
- **Negligence** (過失) - Failure to exercise reasonable care
- **NegligenceWithDuty** (注意義務違反) - Breach of specific duty of care

### Protected Interests

- **BodyAndHealth** (身体・健康) - Physical integrity
- **Property** (財産権) - Property rights
- **Privacy** (プライバシー) - Privacy rights
- **Reputation** (名誉) - Reputation

### Causation Types

- **Direct** (直接因果関係) - Immediate causation
- **Adequate** (相当因果関係) - Adequate/foreseeable causation

### Responsibility Capacity

Japanese law recognizes that children below a certain age (typically 12) lack responsibility capacity (責任能力). In such cases, parents may be liable under Article 714 (parental liability) or Article 715 (supervisor liability).

## Related Examples

- `minpo-709-tort` - Full tort simulation with population testing
- `minpo-715-employer-liability` - Employer/supervisor liability
- `minpo-integrated-tort-damages` - Comprehensive integrated scenarios

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under either of MIT or Apache-2.0 at your option.
