# eu-gdpr-compliance

This example builds a GDPR compliance checker using Legalis-RS, encoding the General Data Protection Regulation (EU) 2016/679 as machine-readable DSL statutes. It covers all six lawful bases in Article 6 (consent, contract, legal obligation, vital interests, public task, legitimate interests), consent conditions (Art. 7), data subject rights (access Art. 15, erasure Art. 17, portability Art. 20), data breach notification to supervisory authorities within 72 hours (Art. 33), and the Data Protection Impact Assessment requirement (Art. 35). A decision tree is rendered and an audit trail is maintained.

## Usage

```sh
cargo run -p eu-gdpr-compliance --all-features
```

## What It Demonstrates

- All six GDPR Article 6 legal bases for processing as separate statutes
- Data subject rights as GRANT effects evaluated against controller entities
- Breach notification and DPIA obligation modelling
- Decision-tree visualisation with `legalis-viz`
- Audit trail for every compliance determination

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
