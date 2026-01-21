# legalis-la

ປະເທດລາວ (Lao PDR) Legal System Support for Legalis-RS

**Version 0.1.3** - Civil Code 2020 Foundation with Comparative Law Analysis (introduced in v0.1.0)

## ພາບລວມ (Overview)

`legalis-la` provides comprehensive support for the Lao People's Democratic Republic legal system within the Legalis-RS framework. This crate implements major legal codes with bilingual support (Lao/English) and includes comparative law analysis showing Japanese and French legal influences through Japan's ODA legal institutional development assistance programs.

## ລະບົບກົດໝາຍລາວ (Lao Legal System)

The Lao legal system is a **civil law system** with unique characteristics shaped by:
- **Socialist legal tradition** - Adapted for market economy transition since 1986
- **Japanese Civil Code influence** - Through JICA's ODA legal assistance (1990s-present)
- **French colonial legacy** - Historical influence from French Indochina period (1893-1953)
- **Customary law integration** - Traditional dispute resolution mechanisms

### Comparison with Other Legal Systems

| Feature | Lao PDR | Japan | France | USA |
|---------|---------|-------|--------|-----|
| Legal Family | Civil Law (Socialist) | Civil Law | Civil Law | Common Law |
| Main Source | Codes & Statutes | Codes & Statutes | Codes & Statutes | Case Law |
| Constitution | 1991 (amended 2015) | 1946 | 1958 | 1787 |
| Court System | 3-tier (People's Court) | 4-tier (Supreme-District) | 3-tier (Cassation-Appeal-First) | Federal & State |
| Legal Capacity Age | 18 years | 18 years (2022) | 18 years | 18-21 years (varies) |

## ລະບຽບການທີ່ປະຕິບັດໄດ້ (Implemented Features)

### ✅ ປະມວນກົດໝາຍແພ່ງ 2020 (Civil Code 2020)

The Lao Civil Code (Law No. 66/NA, effective July 9, 2021) - **1,087 Articles** across 6 Books

#### Book I: General Provisions (ບົດບັນຍັດທົ່ວໄປ) - Articles 1-161

Basic principles, legal capacity, juristic acts, agency, period of time
- ✅ Article 1: Good faith principle (ຫຼັກການສຸດຈິງຈິດ)
- ✅ Article 3: Abuse of rights prohibition (ການຫ້າມໃຊ້ສິດເກີນຂອບເຂດ)
- ✅ Article 20-21: Legal capacity framework (ຄວາມສາມາດທາງກົດໝາຍ)
- ✅ Juristic act validation system
- ✅ Agency relationship framework
- ✅ Period and prescription rules

```rust
use legalis_la::{LegalCapacity, validate_legal_capacity};

let capacity = LegalCapacity::Full { age: 18 };
assert!(validate_legal_capacity(&capacity).is_ok());
```

#### Book II: Property (ຊັບສິນ) - Articles 162-431

Real rights, ownership, possession, co-ownership, servitudes
- ✅ Article 162-163: Property classification (State/Collective/Private)
- ✅ Article 200+: Ownership framework (ສິດເປັນເຈົ້າຂອງ)
- ✅ Possession rules (ການຄອບຄອງ)
- ✅ Co-ownership structures
- ✅ Servitude and real property rights

```rust
use legalis_la::{Property, Ownership, validate_ownership};

let property = Property {
    property_type: PropertyType::Private,
    description: "Residential land in Vientiane".to_string(),
    area_sqm: 500.0,
};

let ownership = Ownership {
    owner: "Khamla Sounthala".to_string(),
    property: property,
    registration_number: Some("VTE-2024-12345".to_string()),
};

assert!(validate_ownership(&ownership).is_ok());
```

#### Book III: Obligations (ພັນທະ) - Articles 432-672

General obligations, contracts, torts, unjust enrichment
- ✅ Article 432+: Obligation framework (ພັນທະທົ່ວໄປ)
- ✅ Article 500+: Contract law (ສັນຍາ)
  - Contract formation requirements (offer, acceptance, consideration)
  - Contract types: Sale, Lease, Loan, Service, Work
  - Contract performance and breach remedies
- ✅ Article 600+: Tort law (ການລະເມິດສິດ)
  - General tort liability
  - Damage compensation framework
- ✅ Unjust enrichment rules

```rust
use legalis_la::obligations::{Contract, ContractType, validate_contract_formation};
use chrono::Utc;

let contract = Contract {
    parties: vec!["Buyer".to_string(), "Seller".to_string()],
    contract_type: ContractType::Sale {
        price: 100_000_000,
        subject: "Land in Luang Prabang".to_string(),
    },
    offer: "Sale of land for 100,000,000 LAK".to_string(),
    acceptance: true,
    consideration: Some(100_000_000),
    lawful_purpose: true,
    capacity_verified: true,
    free_consent: true,
    concluded_at: Utc::now(),
};

assert!(validate_contract_formation(&contract).is_ok());
```

#### Book IV: Family Law (ກົດໝາຍຄອບຄົວ) - Articles 673-909

Marriage, divorce, parent-child relations, adoption, guardianship
- ✅ Article 673+: Marriage requirements (ການແຕ່ງງານ)
  - Minimum age: 18 years (both genders)
  - Registration requirements
  - Prohibited marriages
- ✅ Article 700+: Divorce framework (ການຢ່າຮ້າງ)
  - Mutual consent divorce
  - Fault-based divorce grounds
  - Property division rules
- ✅ Parent-child relations and parental authority
- ✅ Adoption procedures (ການຮັບເປັນບຸດບຸນທຳ)
- ✅ Guardianship framework

```rust
use legalis_la::family::{Marriage, validate_marriage};

let marriage = Marriage {
    spouse1_name: "Bounmy".to_string(),
    spouse1_age: 25,
    spouse2_name: "Vanida".to_string(),
    spouse2_age: 23,
    registration_date: "2024-01-15".to_string(),
    registration_office: "Vientiane Capital".to_string(),
    prohibited_relationship: false,
    free_consent: true,
};

assert!(validate_marriage(&marriage).is_ok());
```

#### Book V: Inheritance (ມໍລະດົກ) - Articles 910-1078

Succession, wills, forced heirship, estate administration
- ✅ Article 910+: Succession framework (ການສືບທອດ)
  - Legal succession order (descendants, spouse, parents, siblings)
  - Testamentary succession
- ✅ Article 950+: Will requirements (ພິນທະສົມ)
  - Holographic will (handwritten, signed, dated)
  - Public will (notarized)
  - Will revocation rules
- ✅ Article 1000+: Forced heirship (reserved portion)
- ✅ Estate administration framework

```rust
use legalis_la::inheritance::{Will, validate_will};

let will = Will {
    testator_name: "Phouvong Soulivong".to_string(),
    testator_age: 65,
    will_type: WillType::Holographic,
    date: "2024-03-10".to_string(),
    beneficiaries: vec!["Son".to_string(), "Daughter".to_string()],
    property_description: "House and land in Savannakhet".to_string(),
    signed: true,
    witnessed: false,
};

assert!(validate_will(&will).is_ok());
```

### ✅ Comparative Law Analysis (ວິໄຈຂຽບທຽບກົດໝາຍ)

Cross-references to Japanese and French legal systems
- ✅ Japanese Civil Code equivalents (明治民法・平成民法)
- ✅ French Code civil equivalents (Code Napoléon)
- ✅ Legal transplantation analysis
- ✅ ODA legal assistance documentation

```rust
use legalis_la::comparative::{compare_with_japanese_law, compare_with_french_law};

// Compare Lao Article 500 (Contract formation) with Japanese Civil Code
let jp_comparison = compare_with_japanese_law("Article 500");
// Returns: Japanese Civil Code Article 521-526 (Contract formation)

// Compare with French Code civil
let fr_comparison = compare_with_french_law("Article 500");
// Returns: Code civil Articles 1113-1122 (Formation du contrat)
```

### ✅ Japan's ODA Legal Assistance (ການຊ່ວຍເຫຼືອທາງກົດໝາຍ ODA)

Documentation of JICA's legal institutional development programs
- ✅ Historical timeline (1990s-2020s)
- ✅ Legal expert missions
- ✅ Civil Code drafting assistance
- ✅ Judicial training programs

```rust
use legalis_la::oda::{get_oda_history, get_legal_assistance_projects};

let history = get_oda_history();
// Returns: Chronological list of JICA legal assistance activities

let projects = get_legal_assistance_projects();
// Returns: Detailed project information and contributions
```

## 📊 Current Implementation Status

**Version 0.1.3 Statistics:**
- ✅ **Civil Code 2020**: 1,087 articles across 6 books
- ✅ **Implementation**: ~2,188 lines of production code
- ✅ **Modules**: 7 modules (general provisions, property, obligations, family, inheritance, comparative, ODA)
- ✅ **Validators**: 12+ validation functions
- ✅ **Bilingual Support**: Lao/English throughout
- ✅ **Tests**: Comprehensive unit tests (in progress)
- ✅ **Documentation**: Extensive doc comments with legal context

## 🚧 Planned Features (See TODO.md)

### Constitution of Lao PDR (ລັດຖະທຳມະນູນ)
- 📋 1991 Constitution (as amended in 2003, 2015)
- 📋 State structure and powers
- 📋 Fundamental rights and duties of citizens
- 📋 National Assembly framework

### Criminal Code (ກົດໝາຍອາຍາ)
- 📋 Criminal Code 2017 (Law No. 26/NA)
- 📋 General provisions (criminal liability, penalties)
- 📋 Specific crimes (property crimes, crimes against persons, corruption)
- 📋 Criminal procedure framework

### Commercial Law (ກົດໝາຍການຄ້າ)
- 📋 Enterprise Law 2013
- 📋 Investment Promotion Law 2016
- 📋 Contract Law for Economic Activities
- 📋 Intellectual Property Law

### Land Law (ກົດໝາຍທີ່ດິນ)
- 📋 Land Law 2019
- 📋 Land registration and titling
- 📋 Land use rights and concessions
- 📋 Land dispute resolution

### Labor Law (ກົດໝາຍແຮງງານ)
- 📋 Labor Law 2013
- 📋 Employment contracts
- 📋 Working hours and leave
- 📋 Social security and benefits

## Dependencies

- `legalis-core` - Core types and traits
- `legalis-i18n` - Internationalization support
- `legalis-verifier` - Validation framework
- `legalis-sim` - Simulation support
- `chrono` - Date/time handling
- `serde` - Serialization
- `thiserror` - Error handling

## License

MIT OR Apache-2.0

## Related Links

- [Ministry of Justice, Lao PDR](http://www.moj.gov.la/)
- [JICA Legal and Judicial Development Project](https://www.jica.go.jp/laos/english/)
- [Asian Development Bank - Lao PDR](https://www.adb.org/countries/lao-pdr/main)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)

## Acknowledgments

This implementation was developed with reference to:
- Official Lao Civil Code 2020 (Law No. 66/NA)
- JICA's legal assistance documentation
- Comparative law research on Japanese and French civil codes
- Legal scholarship on Southeast Asian legal systems
