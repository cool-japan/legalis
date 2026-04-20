# legal-dsl-interop

This example demonstrates cross-format interoperability of legal statutes using `legalis-interop`. Legalis-RS statutes (voting eligibility, welfare benefit) are converted to and from 27+ legal domain-specific languages and standards including Catala (Inria literate programming), L4 (Singapore deontic logic with MUST/MAY/SHANT), Stipula (University of Bologna smart contract agreements), Akoma Ntoso (OASIS legislative XML), LegalRuleML, and business process standards such as BPMN, DMN, and CMMN.

## Usage

```sh
cargo run -p legal-dsl-interop --all-features
```

## What It Demonstrates

- Round-trip conversion between Legalis DSL and 27+ external legal formats
- Catala output for literate-programming-style legal rules
- L4 deontic logic output (MUST/MAY/SHANT operators)
- Stipula smart-contract agreement generation
- Akoma Ntoso and LegalRuleML XML serialisation
- BPMN/DMN/CMMN business process and decision model output

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
