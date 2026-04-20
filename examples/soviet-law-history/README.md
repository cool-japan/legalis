# soviet-law-history

This example uses Legalis-RS for historical and comparative legal research by modelling the Soviet legal system (Советское право, USSR 1922–1991). It reconstructs key codes as DSL statutes with effective and expiry dates: the 1922 RSFSR Civil Code, the 1926 Criminal Code, the 1964 Fundamentals of Civil Legislation, and the 1977 Soviet Constitution (Art. 1 socialist state definition, Art. 6 CPSU leading role). The example illustrates how Legalis-RS handles defunct legal orders for legal transplantation studies, soft ODA context research, and comparative law analysis.

## Usage

```sh
cargo run -p soviet-law-history --all-features
```

## What It Demonstrates

- Historical statutes with `EXPIRY_DATE` marking dissolution (1991-12-26)
- Collective ownership and socialist law concepts modelled as GRANT statutes
- Comparison of Soviet vs. contemporary civil law structures
- Legal transplantation research context for post-Soviet successor states
- Population simulation and audit trail across a defunct jurisdiction ("SU")

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
