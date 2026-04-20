# singapore-business

This example models Singapore's business regulatory framework using Legalis-RS, covering four key statutes: the Companies Act (Cap. 50) — ACRA registration within 14 days, minimum locally resident director, company secretary appointment within six months, and annual return filing; the Personal Data Protection Act 2012 (PDPA) with its consent and data breach notification obligations; the Employment Act (Cap. 91) for employment contract and leave entitlements; and Board of Investment-equivalent business licence requirements. Sample company scenarios are evaluated, followed by a population simulation and audit trail.

## Usage

```sh
cargo run -p singapore-business --all-features
```

## What It Demonstrates

- Companies Act obligations: ACRA registration, director residency, company secretary
- PDPA consent-based data processing and breach notification rules
- Employment Act entitlements as GRANT and OBLIGATION statutes
- Population simulation for business compliance scenario modelling
- Audit trail for regulatory decision recording

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
