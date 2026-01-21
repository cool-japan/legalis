# Legalis-RS v0.1.3 Release Notes

**Release Date**: January 21, 2026
**Type**: Major Expansion Release
**Focus**: Global Jurisdiction Coverage

---

## Overview

Legalis-RS v0.1.3 is a major expansion release that dramatically expands jurisdiction coverage from 9 to **18 jurisdictions**, adding comprehensive legal frameworks for major economies across Asia-Pacific, Middle East, Latin America, and Africa. This release includes 11 new jurisdiction crates with full implementations of civil, corporate, labor, and data protection laws.

## Highlights

### New Jurisdictions (11)

| Jurisdiction | Code | Coverage |
|-------------|------|----------|
| **United Arab Emirates** | `legalis-ae` | Federal Law, Commercial, Labor, PDPL, Free Zones |
| **Australia** | `legalis-au` | Contract, Corporate, Criminal, Consumer, Family, Immigration, IP, Mining, Privacy, Property, Superannuation, Tax, Tort |
| **Brazil** | `legalis-br` | Civil Code, Consumer Protection, LGPD, Labor (CLT), Tax |
| **China** | `legalis-cn` | Civil Code, Contract, Corporate, Data Protection, IP, Labor |
| **Indonesia** | `legalis-id` | Civil Code, Investment, Labor, Tax |
| **India** | `legalis-in` | Constitution, Contract, Criminal (IPC/BNS), DPDP, Consumer Protection, Corporate, IP, Labor, Tax |
| **Laos** | `legalis-la` | Civil Code, Investment, Labor |
| **Thailand** | `legalis-th` | Civil Code, Labor, Investment, Data Protection |
| **Vietnam** | `legalis-vn` | Civil Code, Investment, Labor, Cybersecurity |
| **South Africa** | `legalis-za` | Companies Act, Labor (LRA/BCEA), POPIA, BBBEE |

### Test Coverage

- **13,083 Tests Passing** (up from 11,365 in v0.1.2)
- **1,718 New Tests** added across new jurisdictions
- **Zero Warnings**: Clean clippy run with all features

### Statistics

```
Total Lines:    930,368
Code Lines:     732,506
Rust Code:      720,271 (across 1,651 files)
Comments:       66,754
Tests:          13,083
Jurisdictions:  18
Crates:         41 (17 core + 24 jurisdictions)
```

---

## Changes by Category

### New Jurisdiction Implementations

#### Asia-Pacific

**legalis-au (Australia)**
- Contract law including building contracts and unconscionable conduct
- Corporate law with company registration and director duties
- Criminal law with common defences
- Consumer law (ACL) with product safety and guarantees
- Family law including child support
- Immigration law
- Intellectual property (patents, trademarks, copyright, designs)
- Mining and resources law
- Privacy Act with breach notification
- Property law including strata
- Superannuation law
- Tax law (income, GST, CGT)
- Tort law including product liability

**legalis-cn (China)**
- Civil Code implementation
- Contract law
- Corporate law
- Data protection (PIPL)
- Intellectual property
- Labor law

**legalis-id (Indonesia)**
- Civil Code
- Investment law (BKPM)
- Labor law
- Tax regulations

**legalis-in (India)**
- Constitutional law
- Contract Act
- Criminal law (IPC/BNS transition)
- Digital Personal Data Protection (DPDP) Act
- Consumer Protection Act
- Companies Act
- IP laws (Patents, Trademarks, Copyright)
- Labor codes (2020 reforms)
- Income Tax Act, GST

**legalis-th (Thailand)**
- Civil and Commercial Code
- Labor Protection Act
- Investment promotion (BOI)
- Personal Data Protection Act (PDPA)

**legalis-vn (Vietnam)**
- Civil Code
- Enterprise Law
- Investment Law
- Labor Code 2019
- Cybersecurity Law

**legalis-la (Laos)**
- Civil Code
- Investment Promotion Law
- Labor Law

#### Middle East

**legalis-ae (United Arab Emirates)**
- Federal law structure
- Commercial law
- Labor law (Ministerial Decree)
- Personal Data Protection Law (PDPL)
- Free zone regulations (DIFC, ADGM, DMCC, JAFZA)

#### Latin America

**legalis-br (Brazil)**
- Civil Code (CC/2002)
- Consumer Defense Code (CDC)
- LGPD (Data Protection)
- CLT (Labor Law)
- Tax system (IRPJ, CSLL, PIS/COFINS, ICMS)

#### Africa

**legalis-za (South Africa)**
- Companies Act 71 of 2008
- Labour Relations Act (LRA)
- Basic Conditions of Employment Act (BCEA)
- POPIA (Data Protection)
- BBBEE (Broad-Based Black Economic Empowerment)

### Bug Fixes

- **legalis-in**: Fixed boolean logic bug in criminal/validator.rs
- **legalis-au**: Fixed rustdoc broken intra-doc links in consumer_law/types.rs
- **legalis-br**: Fixed doc test failures (CNPJ validation, currency format, severance calculation)
- **legalis-eu**: Fixed always-true assertions in unfair_practices.rs tests
- **legalis-id**: Fixed useless vec! in civil_code/types.rs
- **Code Quality**: Fixed 20+ clippy warnings across multiple jurisdiction crates

---

## Breaking Changes

**None**. This is a fully backward-compatible release.

---

## Migration Guide

### From v0.1.2 to v0.1.3

Simply update your `Cargo.toml`:

```toml
[dependencies]
legalis-core = "0.1.3"
legalis-jp = "0.1.3"
legalis-au = "0.1.3"  # New
legalis-in = "0.1.3"  # New
# ... etc
```

Or use workspace inheritance:

```toml
legalis-core.workspace = true
```

No code changes required in your projects.

---

## Installation

### Via Cargo

```bash
cargo install legalis-cli --version 0.1.3
```

### Via Git

```bash
git clone https://github.com/cool-japan/legalis-rs.git
cd legalis-rs
git checkout v0.1.3
cargo build --release
```

---

## Crate Versions

All crates are released at version **0.1.3**:

### Core Crates (17)
- legalis-core, legalis-dsl, legalis-registry
- legalis-llm, legalis-verifier, legalis-sim
- legalis-diff, legalis-i18n, legalis-porting
- legalis-viz, legalis-chain, legalis-lod
- legalis-audit, legalis-interop, legalis-api
- legalis-cli, legalis (workspace)

### Jurisdiction Crates (24)
| Region | Crates |
|--------|--------|
| **Asia-Pacific** | legalis-au, legalis-cn, legalis-id, legalis-in, legalis-jp, legalis-la, legalis-sg, legalis-th, legalis-vn |
| **Europe** | legalis-de, legalis-eu, legalis-fr, legalis-uk |
| **Americas** | legalis-br, legalis-ca, legalis-us |
| **Middle East** | legalis-ae |
| **Africa** | legalis-za |

---

## Testing

### Run All Tests

```bash
cargo nextest run --all-features
```

### Run Clippy

```bash
cargo clippy --all-features --all-targets
```

### Build Project

```bash
cargo build --all-features
```

---

## Documentation

- **[Full Documentation](https://docs.rs/legalis-core/0.1.3)**
- **[README](README.md)**
- **[TODO & Roadmap](TODO.md)**
- **[Examples](examples/)**

---

## Contributors

This release was made possible by the Legalis-RS development team at COOLJAPAN OU (Team Kitasan).

**Special Thanks**:
- Comprehensive jurisdiction research and implementation
- Legal framework modeling for 11 new countries
- Cross-jurisdictional consistency improvements

---

## Links

- **Repository**: https://github.com/cool-japan/legalis-rs
- **Full Changelog**: https://github.com/cool-japan/legalis-rs/compare/v0.1.2...v0.1.3
- **Previous Release**: [v0.1.2](RELEASE-0.1.2.md)
- **Issues**: https://github.com/cool-japan/legalis-rs/issues

---

## What's Next?

See [TODO.md](TODO.md) for the roadmap. Upcoming features include:
- Additional ASEAN jurisdictions (Malaysia, Philippines, Myanmar, Cambodia)
- Enhanced cross-border legal analysis
- Expanded treaty and international law coverage
- Performance optimizations for large-scale analysis

---

**Version**: 0.1.3
**License**: MIT OR Apache-2.0
**Rust Version**: 1.86+
