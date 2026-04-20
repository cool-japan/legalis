# Article 709 Tort Liability Simulation

Comprehensive simulation of Japanese Civil Code Article 709 (民法第709条 / Tort Liability) using the Legalis-RS framework.

## Overview

This example demonstrates the full capabilities of the Legalis-RS simulation engine for modeling tort law cases under Article 709 of the Japanese Civil Code. It includes individual scenario testing and population-based simulations to analyze patterns in tort liability.

## What is Article 709?

Article 709 of the Japanese Civil Code is the fundamental provision for tort liability in Japan:

> A person who intentionally or negligently infringes upon another person's rights or legally protected interests and thereby causes damage shall be liable for damages arising therefrom.
>
> 故意又ハ過失ニ因リテ他人ノ権利ヲ侵害シタル者ハ之ニ因リテ生シタル損害ヲ賠償スル責ニ任ス

## Features

### Five Test Scenarios

1. **Intentional Tort (故意の不法行為)** - Clear intent with all elements present
2. **Negligent Tort (過失の不法行為)** - Negligence causing accident
3. **Borderline Case (境界的事例)** - Unclear fault requiring judicial discretion
4. **No Tort (不法行為なし)** - Missing infringement element
5. **Missing Causation (因果関係なし)** - Damages exist but no causal link

### Population Simulation

The example runs a population simulation with 5 agents representing different fact patterns:
- Tracks deterministic outcomes vs. judicial discretion cases
- Calculates deterministic ratio
- Demonstrates separation of computation and discretion

## Core Philosophy

This example demonstrates Legalis-RS's core philosophy:

> **計算可能性と裁量の分離**
> (Separation of Computation and Discretion)

The framework distinguishes between:
- **Deterministic outcomes** - Cases where the law provides clear answers
- **Judicial discretion** - Cases requiring human judgment

## Usage

```bash
cargo run --example minpo-709-tort
```

Or from the subcrate directory:

```bash
cargo run
```

## Three Possible Results

### 1. Deterministic Liability (✅)

All requirements are clearly met:
- Intent OR Negligence: ✓
- Rights Infringement: ✓
- Causation: ✓
- Damages: ✓

Result: **Tortfeasor is LIABLE** (損害賠償責任あり)

### 2. Judicial Discretion (⚖️)

Some elements are unclear or require factual judgment:
- Was there really negligence?
- Was the interest "legally protected"?
- Is causation adequate?

Result: **Requires judicial review** (司法判断が必要)

### 3. Void/No Liability (❌)

Clear precondition not met:
- No rights infringement
- No causation
- No damages

Result: **NO LIABILITY** (責任なし)

## Article 709 Requirements

1. **行為** (Act) - A volitional act by the tortfeasor
2. **故意または過失** (Intent or Negligence) - Mental element
3. **権利侵害** (Rights Infringement) - Violation of rights or legally protected interests
4. **損害** (Damage) - Actual harm occurred
5. **因果関係** (Causation) - Causal link between act and damage

## Population Simulation Metrics

The simulation tracks:
- Total applications
- Deterministic outcomes (clear liability)
- Judicial discretion cases
- Void cases (no liability)
- Deterministic ratio (percentage of clear outcomes)

## Example Output

For each scenario:
```
📌 Scenario 1: Intentional Tort (故意の不法行為)
   Facts: A punched B intentionally, causing injury
   事実: Aが故意にBを殴打し、怪我を負わせた

   ✅ Result: DETERMINISTIC
   Effect: Tortfeasor is LIABLE for damages (損害賠償責任あり)
```

## Related Examples

- `minpo-709-builder` - Builder API demonstration
- `minpo-710-damages-builder` - Non-pecuniary damages
- `minpo-715-employer-liability` - Employer vicarious liability
- `comparative-tort-law` - Cross-jurisdictional comparison

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under the Apache License, Version 2.0.
