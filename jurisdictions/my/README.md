# Legalis-MY: Malaysia Jurisdiction Support

Comprehensive Malaysian legal system implementation for the Legalis-RS ecosystem.

## Overview

Legalis-MY provides full support for Malaysia's unique **dual legal system**, combining Common Law heritage with Islamic Law (Syariah). The crate covers company law, employment law, data protection (PDPA), contract law, Islamic law, tax, intellectual property, competition law, and securities regulation.

## Legal System: Dual Framework

Malaysia operates under a **distinctive dual legal system**:

### 1. Common Law System (Civil Courts)

Inherited from British colonial rule, applicable to all citizens:
- Contract law
- Tort law
- Property law
- Criminal law
- Commercial law

### 2. Islamic Law (Syariah Courts)

Applies **only to Muslims** in specific matters:
- Family law (marriage, divorce)
- Inheritance
- Islamic finance
- Religious offenses

### Legal Hierarchy

```
Federal Constitution (1957)
    ├── Fundamental Liberties (Part II)
    ├── Federal-State Division (13 states + 3 territories)
    └── Islamic Law jurisdiction (List II, Schedule 9)
         ↓
Federal Laws (Acts of Parliament)
    ├── Companies Act 2016
    ├── Employment Act 1955
    ├── PDPA 2010
    └── Contracts Act 1950
         ↓
State Laws
    ├── Islamic Family Law (States)
    └── Land Law (States)
         ↓
Case Law (Judicial Precedents)
    ├── Federal Court (apex)
    ├── Court of Appeal
    └── High Courts (Malaya & Sabah/Sarawak)

Parallel: Syariah Courts (for Muslims only)
    ├── Syariah Appeal Court
    ├── Syariah High Court
    └── Syariah Subordinate Court
```

## Major Legal Areas Covered

### 1. Federal Constitution (1957)

- Constitutional supremacy
- Fundamental liberties
- Federal-state relations
- Islamic law jurisdiction

### 2. Companies Act 2016

- **Sdn Bhd** (Sendirian Berhad) - Private limited company
- **Bhd** (Berhad) - Public limited company
- **LLP** - Limited liability partnership
- **SSM registration** (Suruhanjaya Syarikat Malaysia)
- Corporate governance

### 3. Employment Act 1955

- **Working hours**: 8 hours/day, 48 hours/week maximum
- **Leave entitlements**: Annual leave, sick leave, maternity
- **Termination**: Notice periods, severance
- **EPF**: Employees Provident Fund (mandatory retirement savings)
  - Employer: 12-13% contribution
  - Employee: 11% contribution

### 4. Personal Data Protection Act (PDPA) 2010

- Consent-based framework
- Seven data protection principles
- Data breach notification
- Cross-border transfer restrictions
- PDPD (Personal Data Protection Department) enforcement

### 5. Contracts Act 1950

- Contract formation
- Validity requirements
- Void and voidable contracts
- Remedies for breach
- Based on Indian Contract Act 1872

### 6. Islamic Law (Syariah)

#### For Muslims Only:
- **Family law**: Marriage (nikah), divorce (talak)
- **Inheritance** (Faraid): Islamic inheritance distribution
- **Islamic finance**: Syariah-compliant banking
  - Murabaha (cost-plus financing)
  - Musharakah (partnership)
  - Ijarah (leasing)
- Islamic Banking Act 1983

### 7. Tax Law

#### Income Tax Act 1967
- Progressive individual tax (0-30%)
- Corporate tax (24% standard)

#### Sales and Service Tax (SST)
- **Sales Tax**: 5-10% on manufactured goods
- **Service Tax**: 6% on prescribed services
- Replaced GST in 2018

#### Stamp Duty
- Property transfers
- Share transfers
- Loan agreements

### 8. Intellectual Property

- **Patents Act 1983**: 20-year protection
- **Trademarks Act 2019**: 10-year renewable
- **Copyright Act 1987**: Life + 50 years
- Industrial designs protection

### 9. Competition Act 2010

- Anti-competitive agreements prohibition
- Abuse of dominant position
- MyCC (Malaysia Competition Commission) enforcement
- Merger control

### 10. Capital Markets and Services Act 2007

- Securities regulation
- Bursa Malaysia (stock exchange)
- SC (Securities Commission) oversight
- Foreign investor access

## Trilingual Support

Malaysian legal system uses **three languages**:

```rust
// Malay (Bahasa Malaysia) - Official language, authoritative
// English - Widely used in courts and commerce
// Chinese - Commercial documentation

use legalis_my::common::*;

let company_name = "Tech Inovasi Sdn Bhd";  // Malay
let company_name_en = "Tech Innovation Sdn Bhd"; // English
```

## Key Regulatory Bodies

- **SSM** (Suruhanjaya Syarikat Malaysia) - Companies Commission of Malaysia
- **LHDN** (Lembaga Hasil Dalam Negeri) - Inland Revenue Board
- **BNM** (Bank Negara Malaysia) - Central Bank
- **SC** (Securities Commission Malaysia) - Securities regulator
- **PDPD** (Personal Data Protection Department) - Privacy regulator
- **MyCC** (Malaysia Competition Commission) - Competition authority
- **JAKIM** (Jabatan Kemajuan Islam Malaysia) - Islamic Development Department

## Currency Support

Malaysian Ringgit (MYR / RM) handling:

```rust
use legalis_my::common::MalaysianCurrency;

let salary = MalaysianCurrency::from_ringgit(5000);
assert_eq!(salary.format(), "RM 5,000.00");

// Sen (cents) support
let amount = MalaysianCurrency::from_sen(500_000); // RM 5,000.00
```

## Company Formation

### Sdn Bhd (Private Limited Company)

```rust
use legalis_my::company_law::*;

let company = Company::builder()
    .name("Tech Innovations Sdn Bhd")
    .company_type(CompanyType::PrivateLimited)
    .share_capital(ShareCapital::new(1_000_000)) // RM 10,000
    .add_director(Director::new("Ahmad bin Ali", "850123-01-5678", true))
    .registered_address("Kuala Lumpur")
    .build()?;

// SSM registration required
assert!(company.requires_ssm_registration());
```

## Employment Law

### EPF Calculation

```rust
use legalis_my::employment_law::*;

// Employee: Age 30, Salary RM 3,000
let epf = EpfContribution::new(30, 300_000); // Amount in sen
let breakdown = epf.calculate()?;

println!("Employer: RM {:.2}", breakdown.employer_amount());
println!("Employee: RM {:.2}", breakdown.employee_amount());
```

### Working Hours

```rust
let hours = WorkingHours::new(8, 48); // 8 hours/day, 48 hours/week
assert!(hours.is_compliant());
```

## PDPA Compliance

### Consent Management

```rust
use legalis_my::data_protection::*;

let consent = ConsentRecord::builder()
    .data_subject_id("customer@example.com")
    .purpose(PurposeOfCollection::Marketing)
    .consent_method(ConsentMethod::Written)
    .timestamp(Utc::now())
    .build()?;

assert!(consent.is_valid());
```

### Data Breach Notification

```rust
let breach = DataBreachNotification::new()
    .affected_count(1000)
    .breach_date(Utc::now())
    .notify_pdpd()?; // Must notify PDPD
```

## Islamic Law Support

### Syariah Compliance

```rust
use legalis_my::islamic_law::*;

let product = IslamicFinanceProduct::builder()
    .product_type(IslamicProductType::Murabaha)
    .profit_rate(5.0)
    .build()?;

assert!(validate_shariah_compliance(&product).is_ok());
```

### Islamic Inheritance (Faraid)

```rust
let inheritance = FaraidCalculation::new()
    .estate_value(100_000_00) // RM 1,000,000
    .add_heir(Heir::Wife)
    .add_heir(Heir::Son)
    .add_heir(Heir::Daughter)
    .calculate()?;

for distribution in inheritance.distributions() {
    println!("{}: RM {}", distribution.heir, distribution.amount);
}
```

## Tax Calculations

### Income Tax

```rust
use legalis_my::tax_law::IncomeTax;

let tax = IncomeTax::calculate_individual(100_000_00)?; // RM 1,000,000 income
println!("Tax payable: RM {}", tax.amount());
```

### SST (Sales and Service Tax)

```rust
use legalis_my::tax_law::{SalesTax, ServiceTax};

let sales_tax = SalesTax::calculate(10_000_00, 10.0)?; // 10% rate
let service_tax = ServiceTax::calculate(5_000_00, 6.0)?; // 6% rate
```

## Citation Format

Malaysian legal citations:

```
Companies Act 2016, s. 241(1)
Employment Act 1955, s. 60D
[2024] 1 MLJ 123 (Malayan Law Journal)
[2023] 5 CLJ 456 (Current Law Journal)
```

```rust
use legalis_my::citation::*;

let citation = Citation::statute()
    .act("Companies Act 2016")
    .section("241")
    .subsection("1")
    .build();

println!("{}", citation); // "Companies Act 2016, s. 241(1)"
```

## Public Holidays

```rust
use legalis_my::common::*;

let is_holiday = is_malaysian_holiday("2024-08-31"); // Merdeka Day
let is_working = is_working_day("2024-08-31");

assert!(is_holiday);
assert!(!is_working);
```

## Documentation

- [Full API Documentation](https://docs.rs/legalis-my)
- [Main Legalis-RS Project](../../README.md)
- [SSM Portal](https://www.ssm.com.my)
- [PDPA Guidelines](https://www.pdp.gov.my)

## Related Crates

- `legalis-core` - Core framework
- `legalis-sg` - Singapore law (similar legal system)
- `legalis-uk` - UK law (common law heritage)
- `legalis-sa` - Saudi Arabia (Islamic law comparison)

## License

Licensed under either of MIT or Apache-2.0 at your option.

## Disclaimer

This library is for educational and informational purposes. For legal matters in Malaysia, consult qualified Malaysian legal professionals (Peguam / Lawyer).

---

**Penafian: Perpustakaan ini adalah untuk tujuan pendidikan dan maklumat sahaja. Untuk perkara undang-undang di Malaysia, rujuk peguam yang berkelayakan.**
