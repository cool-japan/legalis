# thailand-business

This example models Thailand's foreign business regulatory framework using Legalis-RS. Three restriction tiers under the Foreign Business Act B.E. 2542 (1999) are encoded: List 1 (absolutely prohibited for foreigners — newspapers, radio/TV, rice farming, land trading), List 2 (Cabinet approval required — arms production, domestic transport, Thai antiques), and List 3 (Foreign Business Licence required — most service sectors). BOI promotion zone incentives are modelled as exceptions that override baseline restrictions, and the Civil and Commercial Code and Labour Protection Act provisions are also included.

## Usage

```sh
cargo run -p thailand-business --all-features
```

## What It Demonstrates

- Three-tier foreign investment restriction structure as PROHIBITION and OBLIGATION statutes
- BOI promotion exceptions overriding base FBA restrictions
- Civil and Commercial Code provisions for company formation
- Labour Protection Act compliance obligations
- Population simulation and audit trail for foreign investor scenarios

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
