# private-international-law

This example models Private International Law (PIL) / Conflict of Laws (国際私法) using Legalis-RS, addressing three key PIL questions: jurisdiction, choice of law, and recognition of foreign judgments. Encoded instruments include Japan's Act on General Rules for Application of Laws (Art. 4 legal capacity by nationality, Art. 7 party autonomy for contract governing law, Art. 8 closest connection rule), EU Rome I and Rome II Regulations for contractual and non-contractual obligations, Hague Conventions (service, evidence, child abduction), and the New York Convention on arbitral award enforcement.

## Usage

```sh
cargo run -p private-international-law --all-features
```

## What It Demonstrates

- Jurisdiction analysis (which court has authority)
- Choice-of-law rules: party autonomy, closest connection, habitual residence
- Japanese PIL (法の適用に関する通則法) alongside EU Rome I and II
- Hague Convention and New York Convention modelling
- Exception clauses (e.g., Japanese law exception for foreign incapacity)
- Audit trail for cross-border legal decision recording

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
