# welfare-benefits

This example demonstrates a comprehensive welfare benefits eligibility determination system using Legalis-RS. Four benefit programs are encoded as DSL statutes: Basic Welfare Assistance (income ≤ $30,000 → $500/month), Senior Citizens Pension Supplement (age ≥ 65 and income ≤ $50,000 → $300/month), Child Support Benefit (dependent children and income ≤ $60,000 → $200/child/month), and Disability Support. The system parses statutes, verifies consistency, evaluates sample individuals across all programs, runs a population simulation, renders a decision tree, and maintains a full audit trail.

## Usage

```sh
cargo run -p welfare-benefits --all-features
```

## What It Demonstrates

- Multi-program eligibility: income, age, and attribute conditions combined
- `LegalDslParser` parsing, `StatuteVerifier` consistency checking
- Population simulation via `SimEngine` and `PopulationBuilder`
- Decision-tree visualisation with `legalis-viz`
- Audit trail recording with `AuditTrail`, `AuditRecord`, and `DecisionResult`
- DISCRETION clause modelling for case-worker cost-of-living adjustments

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
