# australia-immigration

This example demonstrates Australian visa eligibility assessment under the Migration Act 1958 using Legalis-RS. It parses over a dozen visa subclasses — Skilled Independent (189), Skilled Nominated (190), Skilled Work Regional (491), Student (500), Working Holiday (417), Partner (820/801), Temporary Skill Shortage (482), and Employer Nomination Scheme (186) — as DSL statutes, verifies them for consistency, evaluates sample applicants against each provision, and runs a population simulation of 500 applicants to measure deterministic versus discretionary decision rates. An audit trail records every eligibility determination made by the automated system.

## Usage

```sh
cargo run -p australia-immigration --all-features
```

## What It Demonstrates

- Parsing multi-statute DSL strings with `LegalDslParser`
- Points-based visa eligibility logic (age, English, skills, sponsorship)
- Mandatory character (s.501) and health (PIC 4005/4007) requirement modelling
- Population simulation via `SimEngine` and `PopulationBuilder`
- Deterministic audit recording with `AuditTrail` and `AuditRecord`
- Handling `DISCRETION` clauses alongside hard eligibility rules

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
