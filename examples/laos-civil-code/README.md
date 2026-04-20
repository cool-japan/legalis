# laos-civil-code

This example applies Legalis-RS to the Laos Civil Code 2020 (ປະມວນກົດໝາຍແພ່ງ) as a case study in Japan's legal technical assistance (法整備支援 / Soft ODA). Drafted with JICA support from 1996 and enacted by the Lao National Assembly in May 2020, the Civil Code draws on Japanese and German civil law traditions. The example encodes provisions from all seven parts — General Provisions, Persons, Things, Obligations, Real Rights, Family, and Inheritance — as DSL statutes, evaluates legal scenarios, runs a population simulation, and records audit decisions.

## Usage

```sh
cargo run -p laos-civil-code --all-features
```

## What It Demonstrates

- Civil code structure (General Provisions through Inheritance) as hierarchical DSL statutes
- Japan Soft ODA context: comparative law approach without direct transplantation
- Obligations (ໜີ້) and real rights (ກຳມະສິດ) modelled alongside family and succession provisions
- Statute verification for consistency across all seven code parts
- Population simulation and audit trail for civil law scenarios

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
