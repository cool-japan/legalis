# legalis-sg - Singapore Jurisdiction Support

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](../../../LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.86%2B-orange.svg)](https://www.rust-lang.org)

Comprehensive modeling of Singapore law for the Legalis-RS legal framework.

## Features

This crate provides type-safe representations and validation logic for seven major legal domains in Singapore:

### 1. Companies Act (Cap. 50) ✅
- Company formation and structure
- ACRA (Accounting and Corporate Regulatory Authority) registration
- Director eligibility and resident director requirements (s. 145)
- Share capital and shareholding structures
- AGM (Annual General Meeting) compliance (s. 175)
- Annual return filing deadlines (s. 197)
- Company secretary requirements (s. 171)
- Corporate governance

### 2. Employment Act (Cap. 91) ✅
- Employment contract validation
- Working hours and overtime (s. 38)
- CPF (Central Provident Fund) contribution calculations
- Leave entitlements (annual, sick, maternity, paternity)
- Termination notice requirements (s. 10/11)
- Salary and allowance structures
- EA coverage determination

### 3. Personal Data Protection Act 2012 (PDPA) ✅
- Consent management (s. 13/14)
- Purpose limitation (s. 18)
- Data breach notification (s. 26B/26C/26D)
- DNC (Do Not Call) Registry compliance (Part IX)
- DPO (Data Protection Officer) requirements
- Cross-border data transfers (s. 26)
- Data subject access requests (s. 21)

### 4. Consumer Protection ✅
- Sale of Goods Act (Cap. 393) - Implied terms (s. 13/14)
- Consumer Protection (Fair Trading) Act (Cap. 52A)
- Unfair practice detection (s. 4-7)
- Contract term analysis
- Merchantability and fitness for purpose validation

### 5. Intellectual Property ✅
- **Patents Act (Cap. 221)** - 20-year term, novelty/inventive step/industrial application
- **Trade Marks Act (Cap. 332)** - 10-year renewable terms, Nice Classification (45 classes)
- **Copyright Act 2021** - Life + 70 years, fair dealing exceptions
- **Registered Designs Act (Cap. 266)** - 15-year maximum term, novelty/individual character

### 6. Banking Act (Cap. 19) ✅
- **Basel III Capital Adequacy** - CET1 ≥6.5%, Tier 1 ≥8.0%, Total ≥10.0%
- **Banking Licenses** - Full Bank, Wholesale Bank, Merchant Bank
- **AML/CFT Compliance** - MAS Notice 626 (Customer Due Diligence, STR, CTR)
- **MAS Notices** - 637 (Capital), 626 (AML), 610 (Returns)

### 7. Payment Services Act 2019 ✅
- **7 Payment Service Types** - Account issuance, e-money, remittance, merchant acquisition, DPT (crypto), money-changing
- **License Tiers** - Money-changing, Standard Payment Institution (SPI), Major Payment Institution (MPI)
- **Safeguarding Requirements** - 100-110% of float for e-money/account issuance
- **Digital Payment Token (DPT) Regulation** - Cryptocurrency exchange, wallet custody
- **KYC/AML** - Enhanced verification for accounts > SGD 5,000

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
legalis-sg = "0.1.1"
```

### Example: Company Formation

```rust
use legalis_sg::companies::*;

// Create a Singapore Pte Ltd company
let company = Company::builder()
    .name("Tech Innovations Pte Ltd")
    .company_type(CompanyType::PrivateLimited)
    .share_capital(ShareCapital::new(100_000_00)) // SGD 100,000
    .add_director(Director::new("John Tan", "S1234567A", true)) // Resident
    .registered_address(Address::singapore("1 Raffles Place", "048616"))
    .build()?;

// Validate
match validate_company_formation(&company) {
    Ok(report) => println!("✅ Company formation valid"),
    Err(CompaniesError::NoResidentDirector) => {
        eprintln!("❌ No resident director (s. 145 violation)");
    }
    Err(e) => eprintln!("❌ {}", e),
}
```

### Example: CPF Contribution

```rust
use legalis_sg::employment::*;

// Calculate CPF for 30-year-old earning SGD 5,000/month
let cpf = CpfContribution::new(30, 5_000_00);
let breakdown = cpf.calculate()?;

println!("Employer: SGD {:.2} (17%)", breakdown.employer_amount_sgd());
println!("Employee: SGD {:.2} (20%)", breakdown.employee_amount_sgd());
// Output: Employer: SGD 850.00, Employee: SGD 1,000.00
```

### Example: PDPA Consent

```rust
use legalis_sg::pdpa::*;

// Record marketing consent
let consent = ConsentRecord::builder()
    .data_subject_id("customer@example.com")
    .purpose(PurposeOfCollection::Marketing)
    .consent_method(ConsentMethod::Electronic)
    .add_data_category(PersonalDataCategory::Email)
    .timestamp_now()
    .build()?;

validate_consent(&consent)?;
```

### Example: Unfair Practice Detection

```rust
use legalis_sg::consumer::*;

let contract = ConsumerContract::new()
    .business_name("Electronics Store")
    .consumer_name("Jane Lim")
    .add_term("No refunds under any circumstances");

let practices = detect_unfair_practices(&contract);
for practice in practices {
    println!("⚠️  {:?}: {}", practice.practice_type, practice.description);
}
```

### Example: Basel III Capital Adequacy

```rust
use legalis_sg::banking::*;
use chrono::Utc;

// Create capital adequacy data
let capital = CapitalAdequacy {
    cet1_capital_sgd: 1_500_000_000_00,      // SGD 15M
    at1_capital_sgd: 300_000_000_00,         // SGD 3M
    tier2_capital_sgd: 500_000_000_00,       // SGD 5M
    risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
    calculation_date: Utc::now(),
};

// CET1: 15M / 100M = 15.0% (> 6.5% ✓)
// Tier 1: 18M / 100M = 18.0% (> 8.0% ✓)
// Total: 23M / 100M = 23.0% (> 10.0% ✓)
assert!(capital.meets_regulatory_minimum());

let bank = Bank::new(
    "197700001E".to_string(),
    "Singapore Commercial Bank Ltd".to_string(),
    BankLicenseType::FullBank,
    Utc::now(),
    true,
    "Singapore".to_string(),
    capital,
);

let report = validate_bank(&bank)?;
println!("CET1: {:.2}%", report.capital_status.cet1_ratio);
```

### Example: Payment Service Provider (DPT/Crypto)

```rust
use legalis_sg::payment::*;
use chrono::Utc;

// Create a cryptocurrency exchange
let provider = PaymentServiceProviderBuilder::new()
    .uen("202098765B".to_string())
    .name("Singapore Crypto Exchange Pte Ltd".to_string())
    .license_type(PaymentLicenseType::MajorPaymentInstitution)
    .license_date(Utc::now())
    .add_service(PaymentServiceType::DigitalPaymentToken)
    .add_dpt_service(DptServiceType::Exchange)
    .add_dpt_service(DptServiceType::Custody)
    .monthly_volume_sgd(500_000_000) // SGD 5M/month
    .has_aml_officer(true)
    .build()?;

let report = validate_payment_provider(&provider)?;
if report.is_compliant {
    println!("✅ DPT service provider compliant");
}
```

## Architecture

```
legalis-sg/
├── src/
│   ├── lib.rs              # Public API, module exports
│   ├── citation.rs         # Singapore legal citation system
│   ├── companies/          # Companies Act (Cap. 50)
│   │   ├── mod.rs
│   │   ├── types.rs        # Company, Director, ShareCapital
│   │   ├── validator.rs    # Formation, compliance validation
│   │   ├── error.rs        # CompaniesError enum
│   │   ├── acra.rs         # ACRA-specific logic
│   │   └── governance.rs   # AGM, annual returns
│   ├── employment/         # Employment Act (Cap. 91)
│   │   ├── mod.rs
│   │   ├── types.rs        # EmploymentContract, CpfContribution
│   │   ├── validator.rs    # Contract, CPF, leave validation
│   │   ├── error.rs        # EmploymentError enum
│   │   ├── cpf.rs          # CPF calculations
│   │   ├── leave.rs        # Leave entitlement logic
│   │   └── termination.rs  # Notice period calculations
│   ├── pdpa/               # Personal Data Protection Act 2012
│   │   ├── mod.rs
│   │   ├── types.rs        # ConsentRecord, DataBreachNotification
│   │   ├── validator.rs    # Consent, breach validation
│   │   ├── error.rs        # PdpaError enum
│   │   ├── consent.rs      # Consent management
│   │   ├── breach.rs       # Breach notification workflow
│   │   ├── dnc.rs          # DNC Registry logic
│   │   └── dpo.rs          # DPO requirement assessment
│   ├── consumer/           # Consumer Protection
│   │   ├── mod.rs
│   │   ├── types.rs        # ConsumerContract, UnfairPractice
│   │   ├── validator.rs    # Practice detection
│   │   ├── error.rs        # ConsumerError enum
│   │   ├── sale_of_goods.rs    # Implied terms
│   │   └── unfair_practices.rs # Detection algorithms
│   ├── ip/                 # Intellectual Property
│   │   ├── mod.rs
│   │   ├── types.rs        # Patent, Trademark, Copyright, Design
│   │   ├── validator.rs    # IP validation, similarity, fair dealing
│   │   └── error.rs        # IpError enum
│   ├── banking/            # Banking Act (Cap. 19)
│   │   ├── mod.rs
│   │   ├── types.rs        # Bank, CapitalAdequacy, AML types
│   │   ├── validator.rs    # Basel III, license, AML validation
│   │   └── error.rs        # BankingError enum
│   └── payment/            # Payment Services Act 2019
│       ├── mod.rs
│       ├── types.rs        # PaymentServiceProvider, DPT, Safeguarding
│       ├── validator.rs    # License, safeguarding, DPT validation
│       └── error.rs        # PaymentError enum
├── examples/               # 10 comprehensive examples
│   ├── acra_company_registration.rs
│   ├── employment_contract_validation.rs
│   ├── cpf_contribution_calculator.rs
│   ├── banking_capital_adequacy.rs
│   └── ...
└── tests/                  # Integration tests
    ├── companies_validation_tests.rs
    ├── employment_cpf_tests.rs
    ├── pdpa_consent_tests.rs
    └── ...
```

## Examples

Run any of the 10 comprehensive examples:

```bash
# Companies Act
cargo run --example acra_company_registration

# Employment Act
cargo run --example employment_contract_validation
cargo run --example cpf_contribution_calculator
cargo run --example leave_entitlement_calculator
cargo run --example termination_notice_checker

# Consumer Protection
cargo run --example consumer_contract_analysis
cargo run --example sale_of_goods_validation

# Intellectual Property
cargo run --example ip_comprehensive_validation

# Banking Act
cargo run --example banking_capital_adequacy

# Payment Services Act
cargo run --example payment_services_dpt
```

## Testing

```bash
# Run all tests
cargo nextest run --package legalis-sg

# Run specific domain tests
cargo test --package legalis-sg --test companies_validation_tests
cargo test --package legalis-sg --test employment_contract_tests
cargo test --package legalis-sg --test pdpa_consent_tests
cargo test --package legalis-sg --test consumer_protection_tests

# Run with coverage
cargo nextest run --package legalis-sg --all-features
```

## Singapore-Specific Features

### UEN (Unique Entity Number)
All business entities are assigned a 9-10 digit UEN by ACRA for identification.

### CPF (Central Provident Fund)
Mandatory retirement savings for citizens/PRs:
- **Employer**: 17% (age ≤55), graduated decrease for older workers
- **Employee**: 20% (age ≤55), graduated decrease for older workers
- **Wage Ceiling**: SGD 6,000/month (Ordinary Wage)
- **Additional Wage Ceiling**: SGD 102,000/year

### DNC Registry
Do Not Call Registry allows opt-out of marketing communications:
- Voice calls
- Text messages (SMS/MMS)
- Fax messages

### ACRA BizFile+
Electronic filing system for:
- Company registration
- Annual returns (within 7 months of FYE)
- Changes to company particulars
- Director appointments/resignations

## Multilingual Support

In accordance with Singapore's **multilingual context**, error messages are provided in:

1. **English** - Primary language for legal and business
2. **Chinese (中文/华语)** - Simplified Chinese for the Chinese community (74% of population)
3. **Malay (Bahasa Melayu)** - National language of Singapore (13% of population)

### Example Error Message

```rust
ExcessiveWorkingHours { actual: 50.0, limit: 44.0 }
```

Displays as:
```
Working hours 50h/week exceeds limit 44h/week (Employment Act s. 38)
工作时间每周50小时超过法定限制每周44小时 (雇佣法第38条)
Waktu bekerja 50j/minggu melebihi had 44j/minggu (Akta Pekerjaan s. 38)
```

**Note**: While Singapore has four official languages (English, Chinese, Malay, Tamil), the trilingual pattern (English/Chinese/Malay) covers the primary languages used in business and legal contexts. Tamil support can be added optionally. A production deployment would require verification by native speakers and legal professionals familiar with terminology in each language.

## PDPA vs GDPR

Key differences between Singapore PDPA and EU GDPR:

| Feature | GDPR | PDPA |
|---------|------|------|
| **Scope** | EU/EEA + extraterritorial | Singapore organizations |
| **Legal Basis** | 6 lawful bases | Consent-centric |
| **DPO** | Mandatory (certain cases) | Recommended only |
| **Breach Notification** | 72 hours | 3 calendar days |
| **Max Fine** | €20M or 4% revenue | SGD 1M |
| **Right to Erasure** | Yes (Art. 17) | Limited |
| **Marketing Opt-Out** | GDPR opt-in | DNC Registry |

## Design Principles

### 1. Type Safety
- Use **enums** instead of strings (e.g., `CompanyType`, `LeaveType`)
- Money stored as **u64 cents** to avoid floating-point errors
- Dates as **chrono::DateTime<Utc>** for timezone correctness

### 2. Bilingual Error Messages
```rust
#[error("Working hours {actual}h exceeds limit {limit}h (EA s. 38)\n工作时间 {actual}小时超过限制 {limit}小时 (雇佣法第38条)")]
ExcessiveWorkingHours { actual: f64, limit: f64 }
```

### 3. Statute References
All errors and documentation include legal citations:
- Companies Act: "CA s. 145(1)" or "Cap. 50, s. 145"
- PDPA: "PDPA s. 13"
- Employment Act: "EA s. 38" or "Cap. 91, s. 38"

### 4. Builder Pattern
Complex types use builders for ergonomic construction:
```rust
let company = Company::builder()
    .name("Acme Pte Ltd")
    .company_type(CompanyType::PrivateLimited)
    .share_capital(ShareCapital::new(100_000_00))
    .build()?;
```

### 5. Comprehensive Validation
Multi-stage validation:
1. Field-level validation
2. Cross-field relationships
3. Type-specific business rules
4. Singapore-specific statutory requirements

## Dependencies

- `legalis-core` - Core legal framework types
- `legalis-i18n` - Internationalization support
- `chrono` - Date/time handling
- `serde` - Serialization/deserialization
- `thiserror` - Error handling
- `uuid` - Unique identifiers

## Roadmap

Future enhancements:

- [ ] Insolvency Act (winding up, judicial management)
- [ ] Intellectual Property (Patents Act, Copyright Act, Trade Marks Act)
- [ ] Banking Act (MAS regulations, licensing)
- [ ] Securities and Futures Act (SFA)
- [ ] Competition Act (competition law)
- [ ] Contract law (common law principles)
- [ ] Tort law (negligence, defamation)
- [ ] Property law (land titles, conveyancing)

## Contributing

Contributions welcome! Please ensure:

1. Zero compiler warnings (`cargo nextest run --no-run`)
2. Zero clippy warnings (`cargo clippy -- -D warnings`)
3. All tests pass (`cargo nextest run`)
4. Code formatted (`cargo fmt`)
5. Documentation for public items
6. Examples for new features
7. Statute references in errors/docs

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](../../../LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))

at your option.

## Resources

### Official Resources
- [ACRA BizFile+](https://www.acra.gov.sg/)
- [Ministry of Manpower](https://www.mom.gov.sg/)
- [Personal Data Protection Commission](https://www.pdpc.gov.sg/)
- [Singapore Statutes Online](https://sso.agc.gov.sg/)
- [Singapore Law Watch](https://www.lawnet.sg/)

### Key Statutes
- [Companies Act (Cap. 50)](https://sso.agc.gov.sg/Act/CoA1967)
- [Employment Act (Cap. 91)](https://sso.agc.gov.sg/Act/EmA1968)
- [Personal Data Protection Act 2012](https://sso.agc.gov.sg/Act/PDPA2012)
- [Sale of Goods Act (Cap. 393)](https://sso.agc.gov.sg/Act/SOGA1979)
- [Consumer Protection (Fair Trading) Act (Cap. 52A)](https://sso.agc.gov.sg/Act/CPFTA2003)
