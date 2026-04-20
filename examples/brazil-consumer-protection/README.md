# brazil-consumer-protection

This example models Brazil's Consumer Defense Code (Codigo de Defesa do Consumidor — CDC, Lei 8.078/1990) using Legalis-RS. Key articles are encoded as DSL statutes: Art. 6 basic consumer rights, Art. 12–14 strict product and service liability, Art. 18–20 defect remedies, Art. 30–35 binding offer rules, Art. 39 abusive practices, Art. 49 seven-day right of withdrawal for remote purchases, and Art. 51 abusive clause prohibition. The example evaluates consumer scenarios, runs a population simulation, renders decision trees, and records a full audit trail.

## Usage

```sh
cargo run -p brazil-consumer-protection --all-features
```

## What It Demonstrates

- Modelling strict-liability statutes (no-fault for product defects)
- Exception clauses for liberal professionals requiring proof of fault
- Right-of-withdrawal rules triggered by remote purchase conditions
- Decision-tree visualisation with `legalis-viz`
- Population simulation and audit trail recording

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
