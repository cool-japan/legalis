# legalis-us

United States Jurisdiction Support for Legalis-RS

## Overview

`legalis-us` provides Common Law system support for the Legalis-RS framework, including the Restatement of Torts (American Law Institute) and landmark tort cases.

## Features

### Restatement of Torts (ALI)

The American Law Institute's Restatement of Torts synthesizes Common Law principles from case law into structured rules. This crate provides:

- **§ 158 - Battery**: Harmful or offensive contact with another's person
- **§ 46 - Intentional Infliction of Emotional Distress (IIED)**: Extreme and outrageous conduct causing severe emotional distress
- **§ 402A - Products Liability**: Strict liability for defective products

```rust
use legalis_us::{section_158_battery, section_46_iied};

let battery = section_158_battery();
let iied = section_46_iied();
```

### Landmark Tort Cases

Famous cases that established key precedents in tort law:

- **Palsgraf v. Long Island Railroad (1928)** - Foreseeability in negligence
- **Donoghue v. Stevenson (1932)** - Neighbor principle and duty of care
- **Garratt v. Dailey (1955)** - Intent in battery
- **Vosburg v. Putney (1891)** - Transferred intent doctrine

```rust
use legalis_us::{palsgraf_v_long_island, donoghue_v_stevenson};

let palsgraf = palsgraf_v_long_island();
assert_eq!(palsgraf.year, 1928);
assert!(palsgraf.holding.contains("foreseeable"));
```

## Common Law vs Civil Law

The US legal system (derived from English Common Law) differs fundamentally from Civil Law systems (Japan, Germany, France):

### Civil Law Approach (大陸法)

```text
Legislature
    ↓
Code/Statute (e.g., 民法709条, BGB §823, Code civil 1240)
    ↓
Courts apply statute to cases
```

### Common Law Approach (英米法)

```text
Case 1 → Precedent A
    ↓
Case 2 cites Case 1 → Refines Precedent A
    ↓
Case 3 distinguishes → Exception to Precedent A
    ↓
Restatement synthesizes → § X: Rule A (non-binding)
    ↓
Case 4 adopts Restatement § X
```

## Key Differences

| Feature | Civil Law | Common Law |
|---------|-----------|------------|
| Primary Source | Statutes/Codes | Cases/Precedents |
| Court Role | Apply code | Make law |
| Reasoning | Deductive (code → case) | Analogical (case → case) |
| Binding Force | Statute text | Prior holdings (stare decisis) |
| Flexibility | Low (legislature must amend) | High (courts distinguish) |

## Why This Matters for Legalis-RS

- Civil Law modeling uses `Statute` objects (e.g., 民法709条)
- Common Law modeling uses `Case` objects with `precedent_weight()`

The same tort concept appears differently:
- **Civil Law**: Article 709 (statute) → "intent OR negligence"
- **Common Law**: Palsgraf (case) → "duty to foreseeable plaintiff"

## Dependencies

- `legalis-core` - Core types and traits
- `serde` - Serialization
- `chrono` - Date/time handling
- `uuid` - Unique identifiers

## License

MIT OR Apache-2.0

## Links

- [American Law Institute](https://www.ali.org/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
