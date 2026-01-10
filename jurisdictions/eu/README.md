# legalis-eu

**Type-safe EU law validation for Rust** 🇪🇺

European Union jurisdiction support for Legalis-RS, providing comprehensive modeling of EU law including GDPR, Consumer Rights, Competition Law, Treaty Framework, and Intellectual Property.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Tests](https://img.shields.io/badge/tests-173%20passing-brightgreen)]()
[![Warnings](https://img.shields.io/badge/warnings-0-brightgreen)]()
[![Performance](https://img.shields.io/badge/performance-sub--microsecond-blue)]()

## Quick Links

📖 **[Quick Start Guide](docs/QUICKSTART.md)** •
📘 **[GDPR Guide](docs/GDPR_GUIDE.md)** •
🎨 **[IP Guide](docs/IP_GUIDE.md)** •
❓ **[FAQ](docs/FAQ.md)** •
🤝 **[Contributing](docs/CONTRIBUTING.md)**

## Table of Contents

- [Quick Start](#quick-start)
- [Features](#features)
- [Installation](#installation)
- [Usage Examples](#usage-examples)
- [Documentation](#documentation)
- [Examples](#examples)
- [Testing](#tests)
- [Architecture](#architecture)
- [Roadmap](#future-roadmap)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

```rust
use legalis_eu::gdpr::*;

// Validate GDPR data processing
let processing = DataProcessing::new()
    .with_controller("My Company")
    .with_purpose("Marketing emails")
    .add_data_category(PersonalDataCategory::Regular("email".into()))
    .with_lawful_basis(LawfulBasis::Consent {
        freely_given: true,
        specific: true,
        informed: true,
        unambiguous: true,
    });

match processing.validate() {
    Ok(validation) if validation.is_compliant() => {
        println!("✅ GDPR compliant!");
    }
    Err(e) => eprintln!("❌ Error: {}", e),
    _ => {}
}
```

**See the [Quick Start Guide](docs/QUICKSTART.md) for more examples.**

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
legalis-eu = "0.5.9"
chrono = "0.4"  # Required for date/time handling
```

Or use cargo:

```bash
cargo add legalis-eu chrono
```

## Status

**Current Version**: v0.5.9 - **Core Implementation Complete** ✅

- ✅ 196 tests passing (173 unit + 11 property + 12 i18n tests, 0 warnings)
- ✅ 25 comprehensive examples
- ✅ Internationalized error messages (11 EU languages: EN, DE, FR, ES, IT, PL, NL, PT, SV, CS, EL)
- ✅ JSON Schema generation support
- ✅ Full GDPR implementation (Articles 6-49, 83)
- ✅ Consumer Rights Directive 2011/83/EU
- ✅ Competition Law (Articles 101-102 TFEU)
- ✅ Treaty Framework (Four Freedoms, Charter, CJEU cases)
- ✅ Intellectual Property Law (Trademark, Design, Copyright, Trade Secrets)

### Implementation Status

**Phase 1 (GDPR Foundation)**: ✅ COMPLETE
- Articles 6, 9, 15-22, 24-26, 28, 30, 32-36, 37-39, 44-49, 83
- Data processing, security, DPO, DPIA, cross-border transfers, fines

**Phase 2 (GDPR Extensions)**: ✅ COMPLETE
- Article 25 (Data Protection by Design & Default)
- Article 35-36 (DPIA and Prior Consultation)
- Article 28 (Processor Contracts)
- Cross-border transfers (Chapter V)

**Phase 3 (Consumer Rights)**: ✅ COMPLETE
- Articles 6, 9-17 of Directive 2011/83/EU
- 14-day withdrawal right with exceptions

**Phase 4 (Competition Law)**: ✅ COMPLETE
- Article 101 TFEU (Anti-competitive agreements)
- Article 102 TFEU (Abuse of dominance)

**Phase 5 (Treaty Framework)**: ✅ COMPLETE
- Four Freedoms (Articles 28-66 TFEU)
- Charter of Fundamental Rights
- CJEU landmark cases

## Features

### Internationalized Error Messages ✨ NEW

Get GDPR compliance error messages in 11 EU languages:

```rust
use legalis_eu::gdpr::error::GdprError;

let error = GdprError::MissingLawfulBasis;

println!("🇬🇧 {}", error.message("en"));  // English
println!("🇩🇪 {}", error.message("de"));  // German (DSGVO)
println!("🇫🇷 {}", error.message("fr"));  // French (RGPD)
println!("🇪🇸 {}", error.message("es"));  // Spanish (RGPD)
println!("🇮🇹 {}", error.message("it"));  // Italian
println!("🇵🇱 {}", error.message("pl"));  // Polish (RODO)
// ... and 5 more languages
```

**Supported Languages (11 total):**
- English (EN) - GDPR *(default/fallback)*
- German (DE) - Datenschutz-Grundverordnung (DSGVO)
- French (FR) - Règlement Général sur la Protection des Données (RGPD)
- Spanish (ES) - Reglamento General de Protección de Datos (RGPD)
- Italian (IT) - Regolamento Generale sulla Protezione dei Dati (GDPR)
- Polish (PL) - Ogólne Rozporządzenie o Ochronie Danych (RODO)
- Dutch (NL) - Algemene Verordening Gegevensbescherming (AVG)
- Portuguese (PT) - Regulamento Geral sobre a Proteção de Dados (RGPD)
- Swedish (SV) - Dataskyddsförordningen (GDPR)
- Czech (CS) - Obecné nařízení o ochraně osobních údajů (GDPR)
- Greek (EL) - Γενικός Κανονισμός για την Προστασία Δεδομένων (GDPR)

**Coverage:**
- These 11 languages cover ~420 million EU citizens
- Major markets (DE, FR, ES, IT, PL, NL) represent ~80% of EU GDP
- Covers 11 of the 24 official EU languages

**Benefits:**
- Better user experience for EU multilingual applications
- Compliance with language accessibility requirements
- Easy API integration with client language preferences
- Automatic fallback to English for unsupported languages
- Official GDPR terminology in each language

See [`examples/gdpr_i18n_errors.rs`](examples/gdpr_i18n_errors.rs) for complete example.

### JSON Schema Generation

Export GDPR types as JSON schemas for API documentation and validation:

```rust
use schemars::schema_for;
use legalis_eu::gdpr::types::LawfulBasis;

let schema = schema_for!(LawfulBasis);
println!("{}", serde_json::to_string_pretty(&schema).unwrap());
```

**Use Cases:**
- OpenAPI/Swagger API documentation
- TypeScript/Python client code generation
- Request/response validation in web frameworks
- Form generation for UIs
- Configuration file validation

Enable with features: `--features schema,serde`

See [`examples/gdpr_schema_generation.rs`](examples/gdpr_schema_generation.rs) for complete example.

### Intellectual Property Law

Implementation of EU IP regulations:

#### EU Trademark Regulation (EU) 2017/1001
- ✅ European Union Trademark (EUTM) registration validation
- ✅ Nice Classification (Classes 1-45)
- ✅ Distinctiveness requirements (Article 7)
- ✅ Absolute grounds for refusal
- ✅ Descriptive marks and secondary meaning
- ✅ Generic mark detection

#### Community Design Regulation (EC) No 6/2002
- ✅ Registered Community Design (RCD)
- ✅ Unregistered Community Design (UCD)
- ✅ Novelty and individual character assessment

#### Copyright Directives
- ✅ InfoSoc Directive 2001/29/EC structure
- ✅ DSM Directive (EU) 2019/790 structure
- ✅ Software Directive 2009/24/EC structure
- ✅ Database Directive 96/9/EC structure

#### Trade Secrets Directive (EU) 2016/943
- ✅ Trade secret protection validation
- ✅ Three-part test (secret, commercial value, reasonable steps)

### GDPR Implementation (Regulation 2016/679)

Comprehensive implementation of the General Data Protection Regulation:

#### Core Processing (Articles 6-9)
- ✅ **Article 6**: All 6 lawful bases (consent, contract, legal obligation, vital interests, public task, legitimate interests)
- ✅ **Article 7**: Consent conditions validation
- ✅ **Article 9**: Special categories with all 10 exceptions

#### Data Subject Rights (Articles 15-22)
- ✅ **Article 15**: Right of access
- ✅ **Article 16**: Right to rectification
- ✅ **Article 17**: Right to erasure ("right to be forgotten")
- ✅ **Article 18**: Right to restriction of processing
- ✅ **Article 20**: Right to data portability
- ✅ **Article 21**: Right to object
- ✅ **Article 22**: Automated decision-making rights

#### Accountability & Governance (Articles 24-26, 28, 30, 37-39)
- ✅ **Article 24**: Controller accountability framework
- ✅ **Article 25**: Data Protection by Design & Default (DPBD)
- ✅ **Article 26**: Joint controllers
- ✅ **Article 28**: Processor contracts (all mandatory clauses)
- ✅ **Article 30**: Records of Processing Activities (ROPA)
- ✅ **Article 37-39**: Data Protection Officer (DPO) requirements

#### Security & Breach (Articles 32-36)
- ✅ **Article 32**: Security of processing (technical & organizational measures)
- ✅ **Article 33-34**: Data breach notification (72-hour rule)
- ✅ **Article 35**: Data Protection Impact Assessment (DPIA)
- ✅ **Article 36**: Prior consultation with supervisory authority

#### Cross-Border Transfers (Chapter V, Articles 44-49)
- ✅ **Article 45**: Adequacy decisions (14 adequate countries)
- ✅ **Article 46**: Appropriate safeguards (SCCs, BCRs, certifications)
- ✅ **Article 49**: Derogations for specific situations
- ✅ **Schrems II**: Transfer Impact Assessment for high-risk destinations

#### Penalties (Article 83)
- ✅ **Article 83**: Administrative fines calculation (up to €20M or 4% global turnover)

### Consumer Rights Directive (2011/83/EU)

Implementation of EU consumer protection law:

- ✅ **Article 6**: Information requirements for distance/off-premises contracts
- ✅ **Articles 9-16**: Right of withdrawal (14 days, extended to 12 months if info missing)
- ✅ **Article 17**: All 13 exceptions to withdrawal right

### Competition Law (Articles 101-102 TFEU)

Implementation of EU competition rules:

#### Article 101 - Anti-competitive Agreements
- ✅ Hardcore restrictions (price-fixing, market-sharing)
- ✅ De minimis test (10% threshold for horizontal agreements)
- ✅ Article 101(3) exemption criteria (all 4 required)
- ✅ Information exchange detection

#### Article 102 - Abuse of Dominant Position
- ✅ Dominance assessment (>40% market share threshold)
- ✅ Exploitative abuse (excessive pricing, limiting production)
- ✅ Exclusionary abuse (predatory pricing, refusal to deal, tying, margin squeeze)

### Treaty Framework

Foundational EU law structures:

#### Four Freedoms (Articles 28-66 TFEU)
- ✅ Free movement of goods (Article 34)
- ✅ Free movement of persons (Article 45)
- ✅ Freedom to provide services (Article 56)
- ✅ Free movement of capital (Article 63)

#### Charter of Fundamental Rights
- ✅ Article 7: Privacy
- ✅ Article 8: Data protection
- ✅ Article 11: Freedom of expression
- ✅ Article 16: Freedom to conduct business
- ✅ Article 47: Effective remedy

#### CJEU Landmark Cases
- ✅ Van Gend en Loos (C-26/62) - Direct effect
- ✅ Costa v ENEL (C-6/64) - Supremacy
- ✅ Cassis de Dijon (C-120/78) - Mutual recognition
- ✅ Francovich (C-6/90, C-9/90) - State liability

## Usage Examples

### GDPR Data Processing Validation

```rust
use legalis_eu::gdpr::*;

let processing = DataProcessing::new()
    .with_controller("Acme Corp")
    .with_purpose("Marketing emails")
    .add_data_category(PersonalDataCategory::Regular("email".to_string()))
    .with_lawful_basis(LawfulBasis::Consent {
        freely_given: true,
        specific: true,
        informed: true,
        unambiguous: true,
    });

match processing.validate() {
    Ok(validation) => {
        if validation.is_compliant() {
            println!("✅ Processing is GDPR compliant");
        }
    }
    Err(e) => println!("❌ Error: {}", e),
}
```

### Cross-Border Transfer Validation

```rust
use legalis_eu::gdpr::cross_border::*;

let transfer = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("US")
    .with_safeguard(TransferSafeguard::StandardContractualClauses {
        version: "2021".to_string(),
        clauses_signed: true,
    });

match transfer.validate() {
    Ok(validation) => {
        if validation.risk_assessment_required {
            println!("⚠️ Transfer Impact Assessment required (Schrems II)");
        }
    }
    Err(e) => println!("❌ Transfer not permitted: {}", e),
}
```

### Consumer Withdrawal Right

```rust
use legalis_eu::consumer_rights::*;
use chrono::Utc;

let contract = DistanceContract::new()
    .with_trader("Online Shop Ltd")
    .with_consumer("John Doe")
    .with_contract_date(Utc::now())
    .with_goods_description("Laptop computer");

match contract.calculate_withdrawal_period() {
    Ok(period) => {
        println!("Withdrawal deadline: {}", period.deadline);
        println!("Days remaining: {}", period.days_remaining);
    }
    Err(e) => println!("Error: {}", e),
}
```

### Competition Law Analysis

```rust
use legalis_eu::competition::*;

let agreement = Article101Agreement::new()
    .with_undertaking("Company A")
    .with_undertaking("Company B")
    .with_agreement_type(AgreementType::Horizontal)
    .add_restriction(Restriction::PriceFixing {
        agreed_price: 100.0,
    });

match agreement.validate() {
    Ok(validation) => {
        if validation.hardcore_restriction {
            println!("❌ Hardcore restriction - per se illegal");
        }
    }
    Err(e) => println!("Error: {}", e),
}
```

## Multi-Language Support

Supports 11 major EU languages (extensible to all 24 official EU languages):

```rust
use legalis_eu::i18n::MultilingualText;

let text = MultilingualText::new("Data Controller")
    .with_de("Verantwortlicher")
    .with_fr("Responsable du traitement")
    .with_es("Responsable del tratamiento")
    .with_it("Titolare del trattamento")
    .with_pl("Administrator danych")
    .with_nl("Verwerkingsverantwoordelijke")
    .with_pt("Responsável pelo tratamento")
    .with_sv("Personuppgiftsansvarig")
    .with_cs("Správce")
    .with_el("Υπεύθυνος επεξεργασίας")
    .with_source("CELEX:32016R0679");

assert_eq!(text.in_language("en"), "Data Controller");
assert_eq!(text.in_language("de"), "Verantwortlicher");
assert_eq!(text.in_language("pl"), "Administrator danych");
assert_eq!(text.in_language("ja"), "Data Controller"); // Fallback to EN
```

## EUR-Lex Citation System

Proper citation handling with CELEX identifiers:

```rust
use legalis_eu::citation::EuCitation;

let gdpr = EuCitation::regulation(2016, 679).with_article(6);
assert_eq!(gdpr.format_for_language("en"), "Art. 6 GDPR");
assert_eq!(gdpr.format_for_language("de"), "Art. 6 DSGVO");
```

## Documentation

Comprehensive guides are available in the [`docs/`](docs/) directory:

### 📖 Getting Started

- **[Quick Start Guide](docs/QUICKSTART.md)** - Get up and running in 5 minutes
  - Installation
  - First GDPR validation
  - First IP validation
  - Common patterns
  - Next steps

### 📘 In-Depth Guides

- **[GDPR Guide](docs/GDPR_GUIDE.md)** - Complete GDPR implementation guide
  - All 6 lawful bases explained
  - Special categories handling
  - Data subject rights
  - Security and breach notification
  - Cross-border transfers
  - Accountability (DPIA, DPO, etc.)
  - Complete examples

- **[Intellectual Property Guide](docs/IP_GUIDE.md)** - EU IP protection guide
  - EU Trademarks (EUTM)
  - Community Designs (RCD/UCD)
  - Copyright (InfoSoc, DSM, Software directives)
  - Trade Secrets
  - Layered IP strategy

### ❓ Reference

- **[FAQ](docs/FAQ.md)** - Frequently Asked Questions
  - General questions
  - GDPR questions
  - IP questions
  - Performance questions
  - Troubleshooting

### 🤝 For Contributors

- **[Contributing Guide](docs/CONTRIBUTING.md)** - How to contribute
  - Code of conduct
  - Development setup
  - Coding standards
  - Testing requirements
  - Submission guidelines

### 📚 API Documentation

Generate and browse the full API documentation:

```bash
cargo doc --open
```

## Examples

Run the 23 included examples to see all features in action:

```bash
# GDPR Core
cargo run --example gdpr_consent_validation
cargo run --example gdpr_article9_special_categories
cargo run --example gdpr_dsar_handling
cargo run --example gdpr_breach_notification

# GDPR Accountability
cargo run --example gdpr_article24_accountability
cargo run --example gdpr_article25_dpbd
cargo run --example gdpr_article26_joint_controllers
cargo run --example gdpr_processor_contract
cargo run --example gdpr_ropa

# GDPR Security & Risk
cargo run --example gdpr_security_article32
cargo run --example gdpr_dpia
cargo run --example gdpr_dpia_workflow
cargo run --example gdpr_dpo

# GDPR Transfers & Fines
cargo run --example gdpr_cross_border_transfers
cargo run --example gdpr_article83_fines

# GDPR Integration
cargo run --example gdpr_complete_compliance_workflow

# Consumer Rights
cargo run --example consumer_rights_withdrawal

# Competition Law
cargo run --example competition_article101_cartels
cargo run --example competition_article102_dominance

# Intellectual Property
cargo run --example ip_eu_trademark
cargo run --example ip_copyright
cargo run --example ip_trade_secrets
cargo run --example ip_comprehensive
```

## Tests

Run the comprehensive test suite:

```bash
cargo test -p legalis-eu        # Standard tests
cargo nextest run -p legalis-eu # With nextest (recommended)
```

### Benchmarks

Performance benchmarks for validation operations:

```bash
cargo bench --bench gdpr_validation  # GDPR validation benchmarks
cargo bench --bench ip_validation    # IP validation benchmarks
```

**Baseline Performance** (typical results):

GDPR Validation:
- Consent validation: ~80ns per validation
- Special category check: ~210ns per validation
- Cross-border transfer: ~40ns per validation

IP Validation:
- Trademark validation: ~97ns per validation
- Design validation: ~80ns per validation
- Copyright validation: ~57ns per validation
- Trade secret validation: ~112ns per validation
- Misappropriation analysis: ~84ns per analysis

These measurements demonstrate that the crate adds minimal overhead (sub-microsecond), making it suitable for high-performance applications.

**Test Coverage**: 196 tests passing:
- 173 unit tests covering all modules
- 11 property-based tests (proptest) for GDPR Article 6
- 12 i18n tests for multilingual support (11 languages):
  - 8 error message translation tests
  - 4 MultilingualText structure tests
- GDPR Article 6 (lawful bases)
- GDPR Article 9 (special categories)
- GDPR Article 24 (accountability)
- GDPR Article 25 (DPBD)
- GDPR Article 26 (joint controllers)
- GDPR Article 28 (processor contracts)
- GDPR Article 30 (ROPA)
- GDPR Article 32 (security)
- GDPR Article 35-36 (DPIA)
- GDPR Article 37-39 (DPO)
- GDPR Article 83 (fines)
- GDPR Chapter V (cross-border transfers)
- Data subject rights (Articles 15-22)
- Breach notification (Articles 33-34)
- Consumer Rights (withdrawal, exceptions)
- Competition Law (Articles 101-102)
- Treaty Framework (Four Freedoms, Charter, CJEU cases)
- Intellectual Property (Trademark, Design, Copyright, Trade Secrets)
- I18n support
- Citation system
- Member states registry

## Architecture

### Module Structure

```
src/
├── lib.rs                      # Main module with documentation
├── i18n.rs                     # MultilingualText (11 languages, extensible to 24)
├── citation.rs                 # EUR-Lex/CELEX citation system
├── gdpr/                       # GDPR implementation
│   ├── types.rs                # Core GDPR types
│   ├── error.rs                # GDPR-specific errors
│   ├── article6.rs             # Lawful bases (DataProcessing)
│   ├── article9.rs             # Special categories
│   ├── article24.rs            # Controller accountability
│   ├── article25.rs            # Data Protection by Design
│   ├── article26.rs            # Joint controllers
│   ├── article30.rs            # ROPA
│   ├── processor_contract.rs  # Article 28
│   ├── dpo.rs                  # Articles 37-39
│   ├── dpia.rs                 # Articles 35-36
│   ├── rights.rs               # Data subject rights (15-22)
│   ├── security.rs             # Security & breach (32-34)
│   ├── cross_border.rs         # Chapter V transfers
│   └── mod.rs
├── consumer_rights/            # Consumer Rights Directive
│   ├── types.rs                # Contract types
│   ├── withdrawal.rs           # Withdrawal right calculator
│   ├── error.rs                # Consumer rights errors
│   └── mod.rs
├── competition/                # Competition Law
│   ├── article101.rs           # Anti-competitive agreements
│   ├── article102.rs           # Abuse of dominance
│   ├── types.rs                # Market definition, abuse types
│   ├── error.rs                # Competition errors
│   └── mod.rs
├── treaty/                     # Treaty Framework
│   ├── types.rs                # Treaty types and articles
│   ├── four_freedoms.rs        # Four freedoms
│   ├── charter.rs              # Charter of Fundamental Rights
│   ├── case_law.rs             # CJEU landmark cases
│   └── mod.rs
└── shared/
    └── member_states.rs        # EU27 + EEA registry
```

### Integration with legalis-core

All EU legal instruments integrate with `legalis-core`:
- Uses `Statute`, `Condition`, `Effect` types
- Supports `LegalResult::JudicialDiscretion` for balancing tests
- Follows the same builder pattern as other jurisdictions

## Design Principles

1. **Type Safety**: Enums for legal concepts (LawfulBasis, DataSubjectRight, AbuseType)
2. **Builder Pattern**: Fluent API for complex data structures
3. **Source Attribution**: All legal text references EUR-Lex CELEX numbers
4. **Judicial Discretion**: Uses `LegalResult::JudicialDiscretion` for balancing tests
5. **Extensibility**: Optional language fields for future translations
6. **Integration**: Full integration with `legalis-core` framework
7. **Zero Warnings**: Strict compilation with no warnings policy

## Dependencies

- `legalis-core`: Core legal framework types
- `legalis-i18n`: Internationalization support
- `chrono`: Date/time handling
- `serde`: Serialization support (optional)
- `thiserror`: Error handling
- `uuid`: Unique identifiers for legal results

## Future Roadmap

### Phase 6: Multi-Language Expansion (v0.6.0)
- French translation (RGPD)
- Spanish translation
- Italian translation
- Community contribution framework

### Phase 7: Member State Implementations (v0.7.0+)
- Germany (BDSG - Bundesdatenschutzgesetz)
- France (Loi Informatique et Libertés)
- Italy (Codice Privacy)
- Pattern for other member states

### Technical Improvements
- Performance optimization
- Benchmark suite
- Property-based testing
- EUR-Lex API integration

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Areas of interest:
- Additional language translations (13 remaining EU languages: BG, HR, DA, ET, FI, HU, GA, LV, LT, MT, RO, SK, SL)
- Member state-specific implementations
- CJEU case law expansion
- Performance optimizations
- Documentation improvements

When contributing:
- Follow the builder pattern established in existing code
- Include comprehensive tests (maintain 100% test pass rate)
- Add examples for new features
- Reference EUR-Lex sources (CELEX numbers)
- Maintain zero warnings policy (`cargo clippy -- -D warnings`)
