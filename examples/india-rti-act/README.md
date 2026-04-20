# india-rti-act

This example models India's Right to Information Act, 2005 using Legalis-RS. The RTI Act empowers citizens to request information from public authorities, and this example encodes its key provisions as DSL statutes: Section 3 (right to information for citizens), Section 4 (proactive suo motu disclosure obligations), Section 6 (application procedure including fee payment), Section 7 (30-day standard and 48-hour life/liberty response timelines), Section 8 (exemptions from disclosure), and Section 19 (the two-tier appeal mechanism). A population simulation and audit trail complete the demonstration.

## Usage

```sh
cargo run -p india-rti-act --all-features
```

## What It Demonstrates

- Dual response timelines (30-day standard, 48-hour life/liberty) as separate statutes
- Proactive disclosure obligation modelling for public authorities
- Exemption clauses and their interaction with the base right
- Decision-tree rendering and audit trail recording
- Population simulation across diverse citizen and authority scenarios

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
