# jp-drone-regulation

This example models Japan's comprehensive drone regulatory framework under the Aviation Act (航空法) using Legalis-RS. It captures the evolution of regulations from the 2015 initial amendments through the 2022 registration and Remote ID requirements (100 g weight threshold), the 2022 national pilot licensing (技能証明) and aircraft certification (機体認証) system, and the December 2025 revision that abolished HP-listed aircraft simplification and private skill certifications — leaving only national licences and certified aircraft as valid simplification pathways. The example evaluates drone operators against Category I/II/III flight rules and maintains an audit trail.

## Usage

```sh
cargo run -p jp-drone-regulation --all-features
```

## What It Demonstrates

- Jurisdiction-specific DSL statutes with `EXPIRY_DATE` for superseded rules
- Weight-threshold conditions (≥100 g) for unmanned aircraft classification
- Category-based flight permission requirements (特定飛行)
- National licence (一等/二等 技能証明) and aircraft certification checks
- December 2025 regulatory revision: abolition of private certification simplification
- Audit trail recording for flight-permission decisions

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
