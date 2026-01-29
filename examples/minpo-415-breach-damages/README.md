# Article 415 Breach of Obligation Demo

Demonstration of Japanese Civil Code Article 415 (債務不履行 / Breach of Obligation) using the Legalis-RS framework.

## Overview

This example demonstrates how to model and validate breach of contract claims under Article 415 of the Japanese Civil Code. It covers the five requirements for establishing breach of obligation liability and explores practical issues like foreseeability and duty to mitigate damages.

## What is Article 415?

Article 415 of the Japanese Civil Code provides the fundamental framework for contract breach damages:

> A person who fails to perform an obligation in accordance with its nature and the purposes of the contract is liable for damages arising from such failure if the failure is attributable to that person.

Unlike tort law (Article 709), contract law does not require "intent" or "negligence" - only that the breach is "attributable" to the debtor (帰責事由).

## Features

### Three Demonstration Scenarios

1. **Basic Breach Liability** - Seller fails to deliver goods without justification
2. **Foreseeability and Damage Scope** - Delivery delay causing factory shutdown (Hadley v. Baxendale principle)
3. **Duty to Mitigate Damages** - Lease termination and landlord's obligation to re-let

### Five Requirements Validated

1. **Obligation exists** (債務の存在) - Valid contractual duty
2. **Non-performance** (不履行) - Breach occurred
3. **Attribution** (帰責事由) - Breach attributable to debtor
4. **Causation** (因果関係) - Link between breach and damages
5. **Damages** (損害) - Quantifiable harm

## Usage

```bash
cargo run --bin minpo-415-breach-damages
```

Or from the subcrate directory:

```bash
cargo run
```

## Key Legal Concepts Demonstrated

### Attribution vs. Fault

Article 415 requires only "attribution to the debtor" (帰責事由), which is broader than the tort law requirement of intent/negligence. The debtor can escape liability only if the breach was due to circumstances beyond their control.

### Foreseeability (Hadley v. Baxendale)

Japanese courts apply the English common law principle: damages are recoverable only if they were:
- Normally foreseeable at contract time, OR
- Within the contemplation of both parties due to special circumstances communicated

### Duty to Mitigate

Though not explicitly stated in Article 415, Japanese case law recognizes that creditors have a duty to prevent damage expansion based on the good faith principle (Article 1(2)).

## Example Output

Each scenario displays:
- Facts of the case (in Japanese and English)
- Validation of the 5 requirements
- Estimated damages
- Legal analysis and practical notes

## Comparison with Tort Law

| Aspect | Article 415 (Contract) | Article 709 (Tort) |
|--------|------------------------|---------------------|
| Basis | Contractual relationship | Violation of rights |
| Fault | Attribution (帰責事由) | Intent/Negligence (故意・過失) |
| Protected interests | Contract performance | Rights/legally protected interests |
| Damage scope | Foreseeable damages | Causally linked damages |

## Related Examples

- `minpo-709-tort` - Tort liability under Article 709
- `minpo-integrated-tort-damages` - Integration of contract and tort law

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under either of MIT or Apache-2.0 at your option.
