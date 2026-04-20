# korea-labor-law

This example models South Korea's Labor Standards Act (근로기준법) using Legalis-RS. Key provisions are encoded as DSL statutes: Article 2 employee definition based on subordination and wage relationship, Article 17 written employment contract obligation, Article 23 unfair dismissal prohibition (void without justifiable reason), the 52-hour weekly working-hours limit, mandatory annual leave entitlements, and severance pay obligations (one month per year of service). The example evaluates employer and employee scenarios, runs a population simulation, and records an audit trail.

## Usage

```sh
cargo run -p korea-labor-law --all-features
```

## What It Demonstrates

- Subordination-based employee-status determination
- 52-hour workweek and overtime restrictions as PROHIBITION statutes
- Dismissal protection with DISCRETION for Labour Relations Commission assessment
- Severance pay obligation modelling
- Population simulation and audit trail with `legalis-sim` and `legalis-audit`

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
