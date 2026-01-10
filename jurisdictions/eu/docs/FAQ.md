# Frequently Asked Questions (FAQ)

## General Questions

### What is legalis-eu?

legalis-eu is a Rust crate that provides type-safe validation of EU legal requirements, including GDPR, Consumer Rights, Competition Law, Treaty Framework, and Intellectual Property regulations.

### Is this production-ready?

Yes! legalis-eu is production-ready with:
- 173 tests passing (0 warnings)
- Sub-microsecond performance
- Comprehensive validation logic
- Active maintenance

### What Rust version is required?

Rust 1.70.0 or later. The crate follows stable Rust and doesn't require nightly features.

### How do I install it?

Add to your `Cargo.toml`:
```toml
[dependencies]
legalis-eu = "0.5.9"
chrono = "0.4"
```

Or use cargo:
```bash
cargo add legalis-eu chrono
```

## GDPR Questions

### Which lawful basis should I use?

It depends on your use case:

| Use Case | Lawful Basis | Article |
|----------|--------------|---------|
| Marketing emails | Consent | 6(1)(a) |
| Order fulfillment | Contract | 6(1)(b) |
| Legal requirement | Legal Obligation | 6(1)(c) |
| Life-threatening emergency | Vital Interests | 6(1)(d) |
| Government function | Public Task | 6(1)(e) |
| Fraud prevention | Legitimate Interests | 6(1)(f) |

**Rule of thumb:**
- Use **Contract** if processing is necessary to perform a contract
- Use **Consent** for optional, non-essential processing
- Use **Legitimate Interests** when you have a compelling business need (requires balancing test)

### Can I use consent for essential service functionality?

**No!** This is a common mistake. If something is necessary to provide your service, use **Contract** basis, not Consent. Users must be able to withdraw consent without losing access to essential features.

```rust
// ❌ Wrong: Using consent for account creation
.with_lawful_basis(LawfulBasis::Consent { ... })

// ✅ Correct: Use contract for essential functionality
.with_lawful_basis(LawfulBasis::Contract {
    necessary_for_performance: true,
})
```

### How do I handle special categories of data?

Use `Article9Processing` and specify an appropriate exception:

```rust
let processing = Article9Processing::new()
    .add_special_category(SpecialCategory::HealthData)
    .with_exception(Article9Exception::ExplicitConsent {
        purposes: vec!["Medical treatment".into()],
        consent_documented: true,
    });
```

You need BOTH:
1. A lawful basis from Article 6, AND
2. An exception from Article 9

### What's the deadline for data breach notification?

**72 hours** from when you **discovered** the breach (not when it occurred):

```rust
use chrono::Utc;

let breach = DataBreach::new()
    .with_discovered_at(Utc::now() - chrono::Duration::hours(60));

match breach.validate() {
    Ok(validation) => {
        if validation.hours_since_discovery > 72 {
            println!("❌ Deadline exceeded!");
        }
    }
    _ => {}
}
```

### Do I need a Data Protection Officer (DPO)?

You need a DPO if:
1. You're a public authority, OR
2. Your core activities involve large-scale systematic monitoring, OR
3. Your core activities involve large-scale processing of special categories

```rust
let assessment = DpoDesignationAssessment::new()
    .is_public_authority(false)
    .core_activity_systematic_monitoring(true)
    .monitoring_scale(LargeScale::Over100000);

// Will determine if DPO is required
```

### Can I transfer data to the US?

Yes, but you need appropriate safeguards:

1. **Standard Contractual Clauses (SCCs)** - most common
2. **Transfer Impact Assessment** - required for high-risk destinations (Schrems II)
3. **Additional measures** - encryption, data minimization

```rust
let transfer = CrossBorderTransfer::new()
    .with_destination_country("US")
    .with_safeguard(TransferSafeguard::StandardContractualClauses {
        version: "2021".into(),
        clauses_signed: true,
    });

// Will indicate if Transfer Impact Assessment is needed
```

## Intellectual Property Questions

### Should I file for an EU trademark or national trademarks?

**EU Trademark (EUTM)** advantages:
- ✅ Single registration covers all 27 EU member states
- ✅ One application, one fee
- ✅ Centrally managed by EUIPO

**National trademarks** advantages:
- ✅ Can be cheaper for single country
- ✅ Easier to enforce locally
- ✅ Not vulnerable to single opposition

**Recommendation**: For EU-wide business, use EUTM. For single-country focus, use national trademark.

### Can I trademark a descriptive term?

Generally **no**, unless you can prove "secondary meaning" (acquired distinctiveness):

```rust
// ❌ "FAST" for computers - too descriptive
let mark = EuTrademark::new()
    .with_mark_text("FAST")
    .with_descriptive(true);
// Will fail validation

// ✅ "FAST" with secondary meaning (if proven)
let mark = EuTrademark::new()
    .with_mark_text("FAST")
    .with_descriptive(true)
    .with_secondary_meaning(true);  // Must prove this!
```

### What's the difference between Registered and Unregistered Community Design?

| Feature | RCD | UCD |
|---------|-----|-----|
| Registration | Required | Automatic |
| Duration | 25 years | 3 years |
| Cost | Application fees | Free |
| Enforcement | Easier | Harder |
| Protection | All copying | Only slavish copying |

**Use RCD** if: Design is important to your business, long-term protection needed
**Use UCD** if: Short fashion cycles, testing market, no budget for registration

### Does copyright protect my software idea?

**No!** Copyright only protects **expression**, not **ideas**:

- ✅ **Protected**: Source code, object code, comments, variable names
- ❌ **Not protected**: Algorithms, functionality, user interfaces, general concepts

For algorithms and functionality, use:
- **Patents** (if novel and non-obvious)
- **Trade Secrets** (if can be kept secret)

### How do I protect my trade secret?

Implement **reasonable protective measures**:

```rust
let secret = TradeSecret::new()
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: true,
        has_commercial_value: true,
        reasonable_steps_taken: true,  // This is key!
    })
    .add_protective_measure("NDA with all employees")
    .add_protective_measure("Access control to source code")
    .add_protective_measure("Encryption")
    .add_protective_measure("Exit interviews");
```

**Minimum measures**:
- NDAs with anyone who has access
- Access controls (passwords, physical security)
- Marking documents as "Confidential"
- Training employees on confidentiality

## Performance Questions

### How fast is validation?

Very fast! All operations are sub-microsecond:

| Operation | Performance |
|-----------|-------------|
| GDPR consent validation | ~80ns |
| Trademark validation | ~102ns |
| Copyright validation | ~56ns |
| Trade secret validation | ~100ns |

This makes it suitable for high-performance applications.

### Can I use this in async code?

Yes! All validation functions are synchronous and can be used in async contexts:

```rust
async fn validate_processing() -> Result<(), GdprError> {
    let processing = DataProcessing::new()
        .with_controller("Company");

    // Validation is synchronous but very fast
    let validation = processing.validate()?;

    // Continue with async operations
    save_to_database(validation).await?;

    Ok(())
}
```

### Does validation allocate?

Minimal allocations. Most validation logic operates on borrowed data. Validation results may allocate for vectors of warnings/recommendations.

## Language and Localization Questions

### What languages are supported?

Currently:
- ✅ **English** (primary)
- ✅ **German** (secondary)

Planned for v0.7.0:
- French
- Spanish
- Italian

### How do I use German translations?

```rust
use legalis_eu::i18n::MultilingualText;

let text = MultilingualText::from_eurlex(
    "Data Controller".to_string(),
    "Verantwortlicher".to_string(),
    "CELEX:32016R0679".to_string(),
);

// Get in preferred language
let german = text.in_language("de");  // "Verantwortlicher"
let english = text.in_language("en"); // "Data Controller"
```

### Are translations machine-generated?

**No!** All translations come from official EUR-Lex sources. We never use machine translation for legal text.

## Integration Questions

### How does this integrate with legalis-core?

legalis-eu builds on legalis-core types:

```rust
use legalis_core::{Statute, Effect, Condition};

// EU regulations can be converted to Statutes
let statute = gdpr_article_6_1_a();  // Returns Statute
```

All EU legal instruments integrate with the core framework.

### Can I serialize/deserialize validation results?

Yes, with the `serde` feature (enabled by default):

```rust
use serde::{Serialize, Deserialize};

let processing = DataProcessing::new()
    .with_controller("Company");

let validation = processing.validate()?;

// Serialize to JSON
let json = serde_json::to_string(&validation)?;

// Deserialize
let validation: ProcessingValidation = serde_json::from_str(&json)?;
```

### Can I use this with other legal frameworks?

Yes! Check out:
- `legalis-us` - US legal framework
- `legalis-jp` - Japanese law
- `legalis-de` - German law (BDSG)
- `legalis-fr` - French law

All share the same core framework and can be used together.

## Development Questions

### How do I run the tests?

```bash
# All tests
cargo test

# With nextest (recommended)
cargo nextest run

# Specific test
cargo test test_consent_validation

# With output
cargo test -- --nocapture
```

### How do I run the examples?

```bash
# List examples
ls examples/

# Run specific example
cargo run --example gdpr_consent_validation

# Run all examples
for example in examples/*.rs; do
    cargo run --example $(basename $example .rs)
done
```

### How do I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines. Quick steps:

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo clippy -- -D warnings` (must pass)
5. Submit pull request

### Why zero warnings policy?

Clean code is maintainable code. We enforce:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

All PRs must pass this check.

## Troubleshooting

### Error: "Missing lawful basis"

You need to specify a lawful basis for GDPR processing:

```rust
// ❌ Error: No lawful basis
let processing = DataProcessing::new()
    .with_controller("Company");

// ✅ Fixed
let processing = DataProcessing::new()
    .with_controller("Company")
    .with_lawful_basis(LawfulBasis::Consent { ... });
```

### Error: "Nice class out of range"

Nice classes must be 1-45:

```rust
// ❌ Error
.add_nice_class(99)?  // Invalid

// ✅ Fixed
.add_nice_class(9)?   // Valid (Class 9 = Software)
```

### Validation says "judicial discretion required"

Some legal tests cannot be automated and require human judgment:

```rust
let result = processing.validate()?;

match result.lawful_basis_valid {
    LegalResult::JudicialDiscretion { issue, .. } => {
        println!("Manual review required: {}", issue);
        // Perform balancing test manually
    }
    LegalResult::Valid => println!("Automatically valid"),
    LegalResult::Invalid { reason } => println!("Invalid: {}", reason),
}
```

## Still Have Questions?

- Check the [documentation guides](.)
- Browse the [examples](../examples/)
- Open an [issue](https://github.com/cool-japan/legalis/issues)
- Read the API docs: `cargo doc --open`
