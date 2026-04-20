# mexico-tax-law

This example models Mexican federal tax obligations under the Codigo Fiscal de la Federacion (CFF) and related laws using Legalis-RS. Encoded statutes include: CFF Art. 27 RFC registration obligation for any taxpayer over 18 or with taxable activity, CFF Art. 29 mandatory CFDI (Comprobante Fiscal Digital por Internet) electronic invoicing for all business transactions, LISR individual progressive income tax (ISR, 1.92%–35%), corporate income tax, and LIVA value-added tax (IVA, 16%). The example evaluates individuals and businesses, runs a population simulation, and records an audit trail.

## Usage

```sh
cargo run -p mexico-tax-law --all-features
```

## What It Demonstrates

- RFC registration obligations triggered by age and taxable activity
- Mandatory CFDI e-invoicing modelled as a compliance obligation
- Progressive ISR rate structure and corporate flat-rate tax
- IVA (VAT) obligation on business transactions
- Population simulation and audit trail for tax compliance scenarios

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
