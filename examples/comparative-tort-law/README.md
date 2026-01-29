# Comparative Tort Law Example

Comparative legal analysis of tort law across four major legal systems: Japan, Germany, France, and the United States.

## Overview

This example demonstrates how Legalis-RS can be used to perform empirical comparative legal analysis by simulating identical tort scenarios under different legal systems. It provides insights into how civil law (statute-based) and common law (case-based) systems handle the same factual situations differently.

## Legal Systems Covered

### Civil Law Systems (大陸法)

- **Japan (🇯🇵)**: Article 709 of the Civil Code (民法第709条) - Medium abstraction level
- **Germany (🇩🇪)**: BGB § 823 Abs. 1 & § 826 - Concrete enumeration approach
- **France (🇫🇷)**: Code civil Article 1240 (ex-1382) - Ultra-abstract universal clause

### Common Law System (英米法)

- **United States (🇺🇸)**: Restatement (Second) of Torts - Case-based reasoning with specific tort categories

## Features

### Six Test Scenarios

1. **Battery** - Physical assault with clear intent
2. **Fraud** - Economic loss through fraudulent inducement
3. **Defamation** - Personality rights violation
4. **Pure Economic Loss** - Negligent advice causing financial harm
5. **Intentional Infliction of Emotional Distress (IIED)** - Extreme conduct causing severe distress
6. **Products Liability** - Defective product causing injury

### Legal Philosophy Spectrum

```
CIVIL LAW
Concrete ←―――――――――――――→ Abstract
(Certainty)              (Flexibility)

   🇩🇪 BGB § 823    🇯🇵 Art. 709    🇫🇷 Art. 1240
   (Enumeration)    (Medium)        (Universal)

COMMON LAW
   🇺🇸 Restatement
   (Case synthesis)
```

## Usage

```bash
cargo run --example comparative-tort-law
```

Or from the subcrate directory:

```bash
cargo run
```

## What You'll Learn

- How different legal systems handle identical factual scenarios
- Trade-offs between legal certainty (concrete rules) and flexibility (abstract principles)
- Structural gaps in enumeration-based systems (German BGB § 823(1) vs § 826)
- Differences between civil law general tort provisions and common law specific torts
- Role of judicial discretion across legal systems
- Unique common law features like punitive damages and binding precedent

## Example Output

For each test case, the example shows:
- Factual scenario in English and Japanese
- How each legal system evaluates the case
- Whether liability is deterministic or requires judicial discretion
- Comparative analysis highlighting key differences

## Related Examples

- `minpo-709-tort` - Japanese tort law simulation
- `minpo-integrated-tort-damages` - Comprehensive Japanese tort scenarios

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under either of MIT or Apache-2.0 at your option.
