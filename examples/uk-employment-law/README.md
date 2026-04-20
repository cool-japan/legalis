# uk-employment-law

This example models UK employment law using Legalis-RS, covering five major statutes. Employment Rights Act 1996: unfair dismissal protection after two years' continuous service (s.94), statutory notice periods by service length (s.86), and written statement of particulars. Working Time Regulations 1998: the 48-hour average weekly limit and 28 days' paid annual leave entitlement. National Minimum Wage Act 1998: age-banded minimum wage rates. Equality Act 2010: nine protected characteristics. Statutory Sick Pay and maternity/paternity leave are also modelled.

## Usage

```sh
cargo run -p uk-employment-law --all-features
```

## What It Demonstrates

- Unfair dismissal protection with two-year service threshold and gross misconduct exception
- Notice period rules scaling with continuous service length
- 48-hour working-time limit and 28-day paid leave as PROHIBITION and GRANT statutes
- Age-banded minimum wage obligations
- Equality Act protected characteristics encoded as conditions
- Decision-tree visualisation and population simulation

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
