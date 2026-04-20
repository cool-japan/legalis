# canada-healthcare

This example models the Canadian healthcare system under the Canada Health Act (1984) and provincial health insurance plans using Legalis-RS. It encodes the five CHA criteria — public administration, comprehensiveness, universality, portability, and accessibility — as DSL obligations and prohibitions, then adds province-specific eligibility rules for OHIP (Ontario), MSP (British Columbia), RAMQ (Quebec), and other plans. Applicant scenarios covering residents, newcomers, and inter-provincial movers are evaluated, followed by a population simulation and audit trail.

## Usage

```sh
cargo run -p canada-healthcare --all-features
```

## What It Demonstrates

- Encoding federal framework obligations alongside provincial eligibility rules
- Portability coverage during three-month inter-provincial waiting periods
- Prohibition on user charges that impede accessibility
- Population simulation and audit trail recording with `legalis-sim` and `legalis-audit`

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
