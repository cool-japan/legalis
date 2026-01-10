# legalis-uk

United Kingdom Jurisdiction Support for Legalis-RS

## Overview

`legalis-uk` provides comprehensive support for the UK legal system within the Legalis-RS framework, covering employment law, data protection, consumer rights, contract law, and company law for England and Wales.

## Features

### Employment Law (Employment Rights Act 1996)

Implementation of core UK employment rights and protections:

- **ERA 1996 s.1** - Written particulars of employment
- **ERA 1996 s.86** - Statutory notice periods
- **ERA 1996 s.98** - Unfair dismissal (2-year qualifying period)
- **ERA 1996 s.162** - Redundancy payment calculation
- **Working Time Regulations 1998** - 48-hour week, rest breaks
- **National Minimum Wage Act 1998** - Age-based wage rates

```rust
use legalis_uk::employment::{EmploymentContract, ContractType, validate_employment_contract};
use chrono::NaiveDate;

let contract = EmploymentContract::builder()
    .with_employee_name("John Smith")
    .with_employer_name("Acme Ltd")
    .with_contract_type(ContractType::Permanent)
    .with_start_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
    .with_written_particulars(true);

match validate_employment_contract(&contract) {
    Ok(()) => println!("Contract complies with ERA 1996"),
    Err(e) => println!("Contract non-compliant: {}", e),
}
```

### Data Protection (UK GDPR / Data Protection Act 2018)

UK GDPR implementation with 80% reuse from EU GDPR plus UK-specific adaptations:

- **Article 6** - Lawfulness of processing (identical to EU)
- **Article 9** - Special categories processing (identical to EU)
- **Articles 15-22** - Data subject rights (identical to EU)
- **Article 32** - Security of processing (identical to EU)
- **UK-specific**: ICO enforcement, UK adequacy decisions, DPA 2018 exemptions

```rust
use legalis_uk::data_protection::{
    // Re-exported from EU GDPR
    DataProcessing, LawfulBasis,
    // UK-specific
    IcoEnforcement, UkAdequacyDecision,
};

// Use EU GDPR types directly
let processing = DataProcessing::new()
    .with_lawful_basis(LawfulBasis::Consent { /* ... */ });

// UK-specific enforcement
let ico_action = IcoEnforcement::information_notice(/* ... */);
```

### Consumer Rights (Consumer Rights Act 2015)

CRA 2015 implementation with tiered remedies for goods, services, and digital content:

- **CRA s.9-11** - Goods: satisfactory quality, fit for purpose, as described
- **CRA s.49-52** - Services: reasonable care and skill, price, time
- **CRA s.34-47** - Digital content rights
- **CRA s.22-24** - Tiered remedies (short-term reject → repair/replace → price reduction/final reject)
- **CRA Part 2** - Unfair terms test

```rust
use legalis_uk::consumer_rights::{GoodsContract, Remedy, calculate_available_remedies};

let contract = GoodsContract::new("Laptop")
    .with_price(1299.99)
    .with_delivery_date(30.days_ago())
    .breach_discovered_on(10.days_ago());

let remedies = calculate_available_remedies(&contract)?;
// Returns: ShortTermRightToReject (within 30 days) or tiered remedies
```

### Contract Law (Common Law Principles)

Common law contract formation with case law integration:

- **Formation**: Offer, acceptance, consideration, intention
- **Mirror image rule** - Counter-offer destroys original offer (Hyde v Wrench 1840)
- **Postal rule** - Acceptance complete on posting (Adams v Lindsell 1818)
- **Consideration** - Must move from promisee, not past (Tweddle v Atkinson 1861)
- **Damages** - Hadley v Baxendale test (1854)
- **Terms** - Condition, warranty, innominate term (Hong Kong Fir Shipping 1962)

```rust
use legalis_uk::contract::{
    ContractFormation, Offer, Acceptance, Consideration,
    validate_contract_formation,
};

let formation = ContractFormation::new()
    .with_offer(Offer::new("Sale of goods for £100"))
    .with_acceptance(Acceptance::unqualified("I accept"))
    .with_consideration(Consideration::money(100.0))
    .with_commercial_intention(true);

match validate_contract_formation(&formation) {
    Ok(_) => println!("Valid contract formed"),
    Err(e) => println!("Contract invalid: {}", e),
}
```

### Company Law (Companies Act 2006)

CA 2006 implementation for company formation and governance:

- **CA 2006 Part 2** - Company formation (name, office, capital, directors)
- **CA 2006 ss.53-81** - Company name restrictions
- **CA 2006 ss.171-177** - Seven statutory director duties
- **CA 2006 s.763** - Minimum capital (£50k for plc, none for private)
- **CA 2006 s.586** - 25% paid up for public companies

```rust
use legalis_uk::company::{CompanyFormation, CompanyType, validate_company_formation};

let formation = CompanyFormation::new("Acme Ltd")
    .with_company_type(CompanyType::PrivateLimitedByShares)
    .with_registered_office(/* ... */)
    .with_director(/* ... */)
    .with_share_capital(10000.0);

validate_company_formation(&formation)?;
```

## Legal System Features

The UK legal system (England & Wales) follows the **Common Law** tradition with these characteristics:

- **Precedent-based**: Judicial decisions (case law) are primary source
- **Stare decisis**: Binding precedent from higher courts
- **Inductive reasoning**: Case-to-case reasoning by analogy
- **Statutory law**: Acts of Parliament supplemented by common law
- **Flexibility**: Courts can distinguish cases and develop law

### Comparison with Civil Law

| Feature | Common Law (UK) | Civil Law (Germany) |
|---------|----------------|---------------------|
| Primary source | Case law (precedent) | Codified statutes |
| Court role | Law-making | Law-applying |
| Reasoning | Inductive (case → case) | Deductive (code → case) |
| Binding force | Stare decisis | Statutory text |
| Flexibility | High (courts distinguish) | Low (legislature must amend) |
| Contract formation | Requires consideration | No consideration required |
| Legal professions | Barristers & solicitors | Single legal profession |

## UK-Specific Considerations

### Regional Variations

- **England & Wales**: Single legal jurisdiction (covered by this crate)
- **Scotland**: Separate legal system (hybrid civil/common law) - not yet implemented
- **Northern Ireland**: Separate but similar to E&W - not yet implemented

### Statute Referencing

UK statutes are referenced as:
- `ERA 1996 s.86` (section 86 of Employment Rights Act 1996)
- `CA 2006 ss.171-177` (sections 171 to 177 of Companies Act 2006)
- `CRA 2015 s.9` (section 9 of Consumer Rights Act 2015)

Not European-style article numbers (e.g., "Article 6").

### Court Hierarchy

- **Supreme Court** (highest, binds all lower courts)
- **Court of Appeal** (binds High Court and below)
- **High Court** (binds County Court and tribunals)
- **County Court** / **Employment Tribunal**

## Modules

- `employment` - Employment Rights Act 1996, Working Time Regulations 1998
- `data_protection` - UK GDPR, Data Protection Act 2018 (80% reuse from EU)
- `consumer_rights` - Consumer Rights Act 2015
- `contract` - Common law contract principles
- `company` - Companies Act 2006

## Dependencies

- `legalis-core` - Core legal framework
- `legalis-eu` - EU GDPR implementation (reused for UK GDPR)
- `legalis-i18n` - Internationalization support
- `chrono` - Date and time handling
- `thiserror` - Error handling
- `uuid` - Unique identifiers
- `serde` - Serialization (optional feature)

## Examples

See the `examples/` directory for comprehensive usage examples:

- `employment-contract-validation.rs` - Employment contract compliance
- `uk-gdpr-consent-validation.rs` - UK GDPR consent validation
- `consumer-goods-remedy.rs` - Consumer rights tiered remedies
- `contract-formation.rs` - Common law contract formation
- `company-formation.rs` - Companies Act 2006 company formation

## Implementation Status

- ✅ Project structure and dependencies
- 🚧 Employment Law module (in progress)
- ⏳ Data Protection module (planned)
- ⏳ Consumer Rights module (planned)
- ⏳ Contract Law module (planned)
- ⏳ Company Law module (planned)

## Contributing

Contributions are welcome! Please see the main Legalis-RS repository for contribution guidelines.

## License

See the LICENSE file in the main repository.

## References

- [UK Legislation](https://www.legislation.gov.uk/)
- [Employment Rights Act 1996](https://www.legislation.gov.uk/ukpga/1996/18)
- [Companies Act 2006](https://www.legislation.gov.uk/ukpga/2006/46)
- [Consumer Rights Act 2015](https://www.legislation.gov.uk/ukpga/2015/15)
- [Data Protection Act 2018](https://www.legislation.gov.uk/ukpga/2018/12)
- [UK GDPR](https://www.legislation.gov.uk/eur/2016/679)
- [ICO (Information Commissioner's Office)](https://ico.org.uk/)
