# legalis-au

Australia Legal System Support for Legalis-RS

**Version 0.1.3** - Commonwealth Constitution, ACL, Fair Work, Native Title

## Overview

`legalis-au` provides comprehensive support for the Australian legal system within the Legalis-RS framework. Australia operates under a federal common law system with both Commonwealth (federal) and State/Territory laws.

## Australian Legal System

The Australian legal system is characterized by:
- **Common law tradition** - Inherited from English law
- **Federal structure** - Commonwealth, 6 States, 2 Territories
- **Constitutional framework** - 1901 Constitution with implied rights
- **Parliamentary sovereignty** - Subject to constitutional limits
- **Native title recognition** - Since Mabo v Queensland (1992)

### Comparison with Other Legal Systems

| Feature | Australia | UK | USA | Japan |
|---------|-----------|-----|-----|-------|
| Legal Family | Common Law | Common Law | Common Law | Civil Law |
| Main Source | Case Law & Statutes | Case Law & Statutes | Case Law | Codes |
| Constitution | 1901 (Written) | Uncodified | 1787 | 1946 |
| Court System | High Court → Federal/State | Supreme Court | Federal & State | 4-tier |
| Bill of Rights | No (implied rights) | HRA 1998 | Yes (Amendments) | Chapter III |

## Implemented Features

### ✅ Constitution

Commonwealth of Australia Constitution Act 1901
- ✅ Federal structure (ss 51-52, 107-109)
- ✅ Separation of powers (Chapters I-III)
- ✅ Implied rights (freedom of political communication)
- ✅ Section 92 (free trade between States)
- ✅ External affairs power (s 51(xxix))
- ✅ Corporations power (s 51(xx))

```rust
use legalis_au::constitution::{ConstitutionalPower, validate_federal_law};

let law = FederalLaw::new()
    .title("Corporations Act 2001")
    .head_of_power(ConstitutionalPower::Corporations) // s 51(xx)
    .build()?;

assert!(validate_federal_law(&law).is_ok());
```

### ✅ Consumer Law (ACL)

Australian Consumer Law (Schedule 2 to Competition and Consumer Act 2010)
- ✅ Consumer guarantees (ss 51-59)
- ✅ Unfair contract terms (Part 2-3)
- ✅ Misleading and deceptive conduct (s 18)
- ✅ Unconscionable conduct (ss 20-22)
- ✅ Product safety standards

```rust
use legalis_au::consumer_law::{ConsumerGuarantee, validate_guarantee};

let guarantee = ConsumerGuarantee::new()
    .guarantee_type(GuaranteeType::AcceptableQuality)
    .goods_value(500.00) // AUD
    .build()?;

assert!(validate_guarantee(&guarantee).is_ok());
```

### ✅ Employment Law (Fair Work)

Fair Work Act 2009 - National workplace relations framework
- ✅ National Employment Standards (NES)
- ✅ Modern awards system
- ✅ Enterprise agreements
- ✅ Unfair dismissal protection
- ✅ General protections
- ✅ Minimum wage

```rust
use legalis_au::employment::{EmploymentContract, NationalEmploymentStandards};

let contract = EmploymentContract::new()
    .employee_name("John Smith")
    .award("Clerks Private Sector Award 2020")
    .classification("Level 3")
    .hours_per_week(38.0) // Standard full-time
    .build()?;

let nes = NationalEmploymentStandards::validate(&contract)?;
assert!(nes.meets_minimum_standards());
```

### ✅ Tort Law

Common law torts with statutory modifications
- ✅ Negligence (duty, breach, causation, damage)
- ✅ Civil Liability Acts (State variations)
- ✅ Defamation (uniform defamation laws)
- ✅ Nuisance and trespass

```rust
use legalis_au::tort::{Negligence, DutyOfCare};

let claim = Negligence::new()
    .plaintiff("Injured Party")
    .defendant("Negligent Party")
    .duty(DutyOfCare::Established)
    .breach(true)
    .causation(CausationType::ButFor)
    .damage(50_000.00)
    .build()?;
```

### ✅ Property Law

Real property and native title
- ✅ Torrens title system
- ✅ Native title (Native Title Act 1993)
- ✅ Leasehold interests
- ✅ Easements and covenants

### ✅ Corporate Law

Corporations Act 2001 (Cth)
- ✅ Company types (Pty Ltd, Ltd, No Liability)
- ✅ Directors' duties (ss 180-184)
- ✅ Shareholder remedies (oppression, derivative actions)
- ✅ Insolvent trading prohibition

### ✅ Privacy Law

Privacy Act 1988 (Cth) and Australian Privacy Principles (APPs)
- ✅ 13 Australian Privacy Principles
- ✅ Notifiable Data Breaches scheme
- ✅ Cross-border data transfer
- ✅ Credit reporting

### ✅ Superannuation

Superannuation Guarantee (Administration) Act 1992
- ✅ SG contribution rates (11.5% in 2024-25)
- ✅ Choice of fund
- ✅ Preservation rules

## 📊 Current Implementation Status

**Version 0.1.3 Statistics:**
- ✅ **Modules**: 20 modules (constitution, consumer_law, employment, tort, property, corporate, privacy, etc.)
- ✅ **Constitution**: Key sections with implied rights
- ✅ **ACL**: Consumer guarantees and unfair terms
- ✅ **Fair Work**: NES and modern awards
- ✅ **Comprehensive coverage** of major Commonwealth legislation

## Dependencies

- `legalis-core` - Core types and traits
- `legalis-i18n` - Internationalization support
- `legalis-verifier` - Validation framework
- `legalis-sim` - Simulation support
- `chrono` - Date/time handling
- `serde` - Serialization
- `thiserror` - Error handling

## License

Apache-2.0

## Related Links

- [Federal Register of Legislation](https://www.legislation.gov.au/)
- [High Court of Australia](https://www.hcourt.gov.au/)
- [Fair Work Commission](https://www.fwc.gov.au/)
- [ACCC (Consumer Law)](https://www.accc.gov.au/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
