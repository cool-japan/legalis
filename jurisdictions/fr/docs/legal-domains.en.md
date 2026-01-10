# Legal Domains Overview

Comprehensive reference for all 11 legal domains in legalis-fr, organized by French legal codes.

## Overview

Legalis-FR implements **11 major legal domains** covering French civil, commercial, labor, and constitutional law:

| Domain | Code | Articles | Tests | Lines |
|--------|------|----------|-------|-------|
| **Contract Law** | Code civil III | 4 | 33 | 1,816 |
| **Labor Law** | Code du travail | 15 | 80 | 2,946 |
| **Family Law** | Code civil I | 19 | 71 | 3,350 |
| **Inheritance Law** | Code civil III | 12 | 63 | 1,711 |
| **Property Law** | Code civil II-III | 13 | 77 | 1,967 |
| **IP Law** | CPI | 8 | 56 | 1,897 |
| **Evidence Law** | Code civil III | 8 | 42 | 1,132 |
| **Company Law** | Code de commerce | 3 | 19 | 1,557 |
| **Constitutional Law** | Constitution 1958 | 1 | 6 | 755 |
| **Administrative Law** | CJA | 1 | 4 | 391 |
| **Tort Law** | Code civil | 3 | 9 | 391 |

**Total**: 87 articles, 460+ tests, 17,539 lines

---

## 1. Contract Law (Droit des contrats)

**Legal Basis**: Code civil, Book III, Articles 1101-1231

### Scope

Implementation of the 2016 reform of French contract law, covering:
- Formation of contracts (Articles 1101-1171)
- Performance and non-performance (Articles 1217-1231)
- Termination and damages

### Key Articles Implemented

#### Article 1128: Essential Elements

```rust
use legalis_fr::contract::{Contract, ContractType, validate_contract};

// Validates three essential elements:
// 1. Consent of parties (consentement)
// 2. Capacity to contract (capacité)
// 3. Lawful and certain content (contenu licite et certain)

let contract = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Seller".to_string(), "Buyer".to_string()])
    .object("Clearly defined object")
    .price(100_000)
    .build()?;

validate_contract(&contract)?;  // Validates Article 1128
```

#### Article 1217: Non-Performance Remedies

```rust
use legalis_fr::contract::{assess_contract_performance, PerformanceStatus};

// Implements Article 1217 remedies:
// - Exception for non-performance (exception d'inexécution)
// - Price reduction (réduction du prix)
// - Resolution (résolution du contrat)
// - Performance in kind (exécution forcée en nature)
// - Damages (dommages et intérêts)

let remedies = assess_contract_performance(
    &contract,
    PerformanceStatus::PartialFailure,
)?;
```

### Use Cases

- **E-commerce platforms**: Contract formation and validation
- **Real estate**: Sale and lease agreements
- **B2B platforms**: Service contracts and SLAs
- **Legal tech**: Contract analysis and risk assessment

### Related Modules

- **Labor Law**: Employment contracts (special contract type)
- **Property Law**: Real estate sale contracts
- **Company Law**: Partnership agreements

---

## 2. Labor Law (Droit du travail)

**Legal Basis**: Code du travail, Articles L1221-1 to L5422-3

### Scope

Comprehensive implementation of French employment law:
- Employment contracts (CDI, CDD, temps partiel)
- Working time (35-hour workweek, overtime)
- Minimum wage (SMIC)
- Termination procedures
- Collective bargaining

### Key Articles Implemented

#### Article L1221-1: Employment Contract

```rust
use legalis_fr::labor::{Employment, EmploymentType, validate_employment};

let employment = Employment::builder()
    .employee_name("Marie Dupont")
    .employer_name("TechCorp SARL")
    .employment_type(EmploymentType::Indefinite)  // CDI
    .start_date(NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
    .position("Software Engineer")
    .monthly_salary(3_500)
    .weekly_hours(35.0)
    .build()?;

validate_employment(&employment)?;
```

#### Article L3121-27: Maximum Working Hours

```rust
use legalis_fr::labor::validate_working_hours;

// Maximum 35 hours/week (legal workweek)
// Collective agreements may allow up to 48 hours
validate_working_hours(35.0)?;  // OK
validate_working_hours(50.0)?;  // Error: exceeds legal maximum
```

#### Article L3231-2: Minimum Wage (SMIC)

```rust
use legalis_fr::labor::{validate_minimum_wage, SMIC_2024};

// SMIC 2024: €1,766.92/month for 35h/week
validate_minimum_wage(1_800, 35.0)?;  // OK
validate_minimum_wage(1_500, 35.0)?;  // Error: below SMIC
```

#### Article L1234-1: Termination

```rust
use legalis_fr::labor::{validate_termination, TerminationReason};

let termination = validate_termination(
    &employment,
    TerminationReason::EconomicDismissal,
    NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
)?;

println!("Notice period: {} months", termination.notice_period_months);
println!("Severance pay: €{}", termination.severance_pay);
```

### Use Cases

- **HR systems**: Employment contract management
- **Payroll systems**: SMIC compliance, overtime calculation
- **Workforce management**: Working hours tracking
- **Legal compliance**: Termination procedure validation

### Special Features

- **2024 SMIC values**: Automatically updated
- **Collective agreement support**: Extensible for industry-specific rules
- **Bilingual documentation**: French labor law terminology

---

## 3. Family Law (Droit de la famille)

**Legal Basis**: Code civil, Book I, Articles 143-515-13

### Scope

Comprehensive family law implementation:
- Marriage (mariage) and PACS
- Divorce (divorce)
- Filiation and adoption
- Parental authority (autorité parentale)
- Name changes

### Key Articles Implemented

#### Articles 143-144: Marriage Requirements

```rust
use legalis_fr::family::{Marriage, MarriageRegime, validate_marriage};

let marriage = Marriage::builder()
    .spouse1("Jean Martin", 28)  // Minimum age: 18
    .spouse2("Sophie Dubois", 26)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .regime(MarriageRegime::CommunityOfProperty)
    .build()?;

validate_marriage(&marriage)?;
```

#### Articles 229-232: Divorce Types

```rust
use legalis_fr::family::{Divorce, DivorceType};

// Four types of divorce:
// 1. Mutual consent (consentement mutuel)
// 2. Accepted divorce (divorce accepté)
// 3. Fault divorce (faute)
// 4. Definitive alteration (altération définitive du lien conjugal)

let divorce = Divorce::builder()
    .marriage(marriage)
    .divorce_type(DivorceType::MutualConsent)
    .filing_date(NaiveDate::from_ymd_opt(2025, 3, 10).unwrap())
    .build()?;
```

#### Article 371-1: Parental Authority

```rust
use legalis_fr::family::{ParentalAuthority, assess_parental_authority};

let authority = ParentalAuthority::builder()
    .child_name("Emma Martin")
    .child_birthdate(NaiveDate::from_ymd_opt(2015, 4, 20).unwrap())
    .parents(vec!["Jean Martin".to_string(), "Sophie Dubois".to_string()])
    .joint_authority(true)  // Joint authority is the rule
    .build()?;

assess_parental_authority(&authority)?;
```

#### Articles 515-1 to 515-7: PACS

```rust
use legalis_fr::family::{PACS, validate_pacs};

let pacs = PACS::builder()
    .partner1("Alice Moreau", 30)
    .partner2("Bob Lefebvre", 32)
    .registration_date(NaiveDate::from_ymd_opt(2023, 9, 1).unwrap())
    .build()?;

validate_pacs(&pacs)?;
```

### Use Cases

- **Civil status management**: Marriage, PACS, divorce records
- **Notary systems**: Marriage contracts, estate planning
- **Family courts**: Case management systems
- **Legal advice platforms**: Family law guidance

### Special Features

- **2013 same-sex marriage law**: Full implementation
- **Marriage regimes**: Community, separation, participation
- **Bilingual terms**: French legal terminology with English translations

---

## 4. Inheritance Law (Droit des successions)

**Legal Basis**: Code civil, Book III, Title I, Articles 720-892

### Scope

Complete succession and will framework:
- Succession opening and devolution
- Reserved portions (réserve héréditaire)
- Available portion (quotité disponible)
- Wills (testaments)
- Estate division

### Key Articles Implemented

#### Article 720: Succession Opening

```rust
use legalis_fr::inheritance::{Succession, Heir, Relationship};

let succession = Succession::builder()
    .deceased("Jean Martin")
    .death_date(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap())
    .heirs(vec![
        Heir::new("Marie Martin", Relationship::Child, None),
        Heir::new("Pierre Martin", Relationship::Child, None),
        Heir::new("Sophie Martin", Relationship::Spouse, None),
    ])
    .estate_value(500_000)
    .build()?;
```

#### Articles 912-913: Reserved Portions

```rust
use legalis_fr::inheritance::calculate_reserved_portions;

// Reserved portions protect children and spouse:
// - 1 child: reserved portion = 1/2 (available = 1/2)
// - 2 children: reserved portion = 2/3 (available = 1/3)
// - 3+ children: reserved portion = 3/4 (available = 1/4)

let portions = calculate_reserved_portions(2)?;
println!("Reserved: {:.2}%", portions.reserved_portion * 100.0);  // 66.67%
println!("Available: {:.2}%", portions.available_portion * 100.0);  // 33.33%
```

#### Articles 774-792: Wills

```rust
use legalis_fr::inheritance::{Will, WillType, validate_will};

// Three types of wills:
// 1. Holographic (olographe): handwritten, dated, signed
// 2. Authentic (authentique): notarized
// 3. Mystic (mystique): sealed, presented to notary

let will = Will::builder()
    .testator("Jean Martin")
    .will_type(WillType::Holographic {
        handwritten: true,
        dated: true,
        signed: true,
    })
    .date(NaiveDate::from_ymd_opt(2023, 1, 10).unwrap())
    .dispositions(vec![
        "Leave apartment to Marie".to_string(),
        "Leave car to Pierre".to_string(),
    ])
    .build()?;

validate_will(&will)?;
```

### Use Cases

- **Estate planning tools**: Will creation and validation
- **Notary systems**: Succession management
- **Wealth management**: Inheritance tax calculation
- **Legal advice**: Reserved portion compliance

### Special Features

- **Reserved portion calculator**: Automatic calculation
- **Succession order**: Full implementation of French succession rules
- **Will validation**: Three types with specific requirements

---

## 5. Property Law (Droit des biens)

**Legal Basis**: Code civil, Books II-III, Articles 490-734

### Scope

Real property and servitudes:
- Ownership rights (droit de propriété)
- Easements (servitudes)
- Water rights
- Right of way (passage)
- Real estate transactions

### Key Articles Implemented

#### Article 544: Absolute Ownership

```rust
use legalis_fr::property::{Property, PropertyType, validate_ownership};

let property = Property::builder()
    .property_type(PropertyType::Immovable {
        land_area: 500.0,
        building_area: Some(150.0),
    })
    .owner("Marie Dupont")
    .location("12 Rue de la Paix, Paris")
    .value(750_000)
    .build()?;

// Article 544: Right to use, enjoy, and dispose (usus, fructus, abusus)
validate_ownership(&property)?;
```

#### Articles 637-710: Easements

```rust
use legalis_fr::property::{Easement, EasementType, validate_easement};

let easement = Easement::builder()
    .easement_type(EasementType::RightOfWay)
    .dominant_estate(Some("Parcel A"))
    .servient_estate("Parcel B")
    .description("3-meter path for vehicle access")
    .build()?;

validate_easement(&easement)?;
```

#### Article 555: Water Rights

```rust
// Mandatory easement for water access (cattle, irrigation)
let water_easement = Easement::builder()
    .easement_type(EasementType::WaterRights)
    .dominant_estate(Some("Farm"))
    .servient_estate("Property with stream")
    .description("Cattle watering rights")
    .build()?;
```

### Use Cases

- **Real estate platforms**: Property transactions
- **Land registry systems**: Easement tracking
- **Agriculture tech**: Water rights management
- **Urban planning**: Servitude compliance

### Special Features

- **Legal easements**: Automatic water rights, drainage, support
- **Conventional easements**: Custom servitudes
- **Property types**: Immovable (real estate) vs. movable (personal property)

---

## 6. Intellectual Property Law (Droit de la propriété intellectuelle)

**Legal Basis**: Code de la propriété intellectuelle (CPI)

### Scope

Industrial and literary property:
- Patents (brevets)
- Copyrights (droits d'auteur)
- Trademarks (marques)
- Designs (dessins et modèles)

### Key Articles Implemented

#### Articles L611-10, L611-11: Patents

```rust
use legalis_fr::intellectual_property::{Patent, validate_patent};

let patent = Patent::builder()
    .title("Novel Solar Panel Design")
    .inventor("Dr. Marie Curie")
    .filing_date(NaiveDate::from_ymd_opt(2023, 3, 15).unwrap())
    .novelty(true)
    .inventive_step(true)
    .industrial_applicability(true)
    .build()?;

validate_patent(&patent)?;
// Protection: 20 years from filing date
```

#### Articles L122-1, L123-1: Copyright

```rust
use legalis_fr::intellectual_property::{Copyright, WorkType};

let copyright = Copyright::builder()
    .work_title("Les Misérables")
    .author("Victor Hugo")
    .creation_date(NaiveDate::from_ymd_opt(1862, 4, 3).unwrap())
    .work_type(WorkType::Literary)
    .build()?;

// Protection: Life of author + 70 years
println!("Expiry: {}", copyright.expiry_date());
```

#### Articles L711-1, L712-1: Trademarks

```rust
use legalis_fr::intellectual_property::{Trademark, validate_trademark};

let trademark = Trademark::builder()
    .mark("LEGALIS™")
    .owner("Legalis SAS")
    .registration_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
    .distinctiveness(true)
    .build()?;

// Protection: 10 years, renewable indefinitely
```

### Use Cases

- **IP management systems**: Portfolio tracking
- **Patent offices**: Application processing
- **Publishing platforms**: Copyright management
- **Brand protection**: Trademark monitoring

### Special Features

- **Duration calculator**: Automatic expiry date calculation
- **Three-fold requirements**: Novelty, inventive step, industrial applicability
- **Renewal tracking**: Trademark renewal dates

---

## 7. Evidence Law (Droit de la preuve)

**Legal Basis**: Code civil, Book III, Title XX, Articles 1353-1378

### Scope

Proof and evidence in civil proceedings:
- Burden of proof (charge de la preuve)
- Means of proof (modes de preuve)
- Electronic evidence (preuves électroniques)
- Presumptions (présomptions)

### Key Articles Implemented

#### Article 1353: Burden of Proof

```rust
use legalis_fr::evidence::{BurdenOfProof, assess_burden_of_proof};

let burden = BurdenOfProof::builder()
    .claimant_must_prove(vec![
        "Contract was signed".to_string(),
        "Payment was made".to_string(),
    ])
    .defendant_must_prove(vec![
        "Goods were delivered".to_string(),
    ])
    .build()?;

assess_burden_of_proof(&burden)?;
```

#### Articles 1366-1378: Electronic Evidence

```rust
use legalis_fr::evidence::{Evidence, EvidenceType};

// Electronic evidence has same force as written evidence
let evidence = Evidence::builder()
    .evidence_type(EvidenceType::WrittenDocument {
        electronic: true,
        signed: true,  // Electronic signature
    })
    .description("Electronically signed contract")
    .authenticity_verified(true)
    .build()?;
```

#### Article 1354: Presumptions

```rust
use legalis_fr::evidence::{PresumptionType, assess_presumption};

// Three types:
// 1. Simple (rebuttable)
// 2. Mixed
// 3. Irrebuttable (absolute)

let simple = assess_presumption(PresumptionType::Simple)?;
assert!(simple.rebuttable);

let irrebuttable = assess_presumption(PresumptionType::Irrebuttable)?;
assert!(!irrebuttable.rebuttable);
```

### Use Cases

- **Litigation support**: Evidence assessment
- **E-signature platforms**: Electronic evidence validation
- **Legal research**: Burden of proof analysis
- **Court systems**: Evidence admissibility

### Special Features

- **Electronic evidence**: Full 2016 reform implementation
- **Presumption types**: Three-tier classification
- **Bilingual terminology**: French legal proof concepts

---

## 8. Company Law (Droit des sociétés)

**Legal Basis**: Code de commerce, Articles L210-1 to L247-1

### Scope

Business entity formation and management:
- SARL (Société à responsabilité limitée)
- SAS (Société par actions simplifiée)
- SA (Société anonyme)
- Formation requirements
- Capital requirements

### Key Articles Implemented

#### Article L210-2: Company Formation

```rust
use legalis_fr::company::{Company, CompanyType, validate_company_formation};

// SARL: minimum €1 capital
let sarl = Company::builder()
    .name("TechCorp SARL")
    .company_type(CompanyType::SARL)
    .capital(10_000)
    .shareholders(vec![
        "Jean Martin (60%)".to_string(),
        "Sophie Dubois (40%)".to_string(),
    ])
    .registered_office("45 Rue de Rivoli, Paris")
    .build()?;

validate_company_formation(&sarl)?;

// SAS: minimum €1 capital, more flexible
let sas = Company::builder()
    .name("InnovateSAS")
    .company_type(CompanyType::SAS)
    .capital(50_000)
    .build()?;
```

### Use Cases

- **Business registration**: Company formation
- **Corporate management**: Governance compliance
- **Startup platforms**: Entity selection guidance
- **Legal automation**: Formation document generation

### Special Features

- **Three main types**: SARL, SAS, SA
- **Capital requirements**: Type-specific minimums
- **Shareholder validation**: Ownership distribution checks

---

## 9. Constitutional Law (Droit constitutionnel)

**Legal Basis**: Constitution of the Fifth Republic (1958)

### Scope

Constitutional review and fundamental rights:
- Constitutionality review (contrôle de constitutionnalité)
- Fundamental rights (droits fondamentaux)
- Separation of powers

### Key Articles Implemented

#### Article 61: Constitutionality Review

```rust
use legalis_fr::constitution::{assess_constitutionality, ConstitutionalIssue};

let issue = ConstitutionalIssue::builder()
    .law_text("New data retention law")
    .challenged_provisions(vec![
        "Mandatory 5-year data retention".to_string(),
    ])
    .constitutional_rights_at_stake(vec![
        "Right to privacy (Article 2)".to_string(),
        "Freedom of communication (Article 11)".to_string(),
    ])
    .build()?;

let assessment = assess_constitutionality(&issue)?;
```

### Use Cases

- **Constitutional courts**: Constitutionality review
- **Legislative drafting**: Pre-review compliance
- **Legal research**: Constitutional analysis
- **Academic tools**: Teaching constitutional law

---

## 10. Administrative Law (Droit administratif)

**Legal Basis**: Code de justice administrative

### Scope

Administrative acts and procedures:
- Administrative acts (actes administratifs)
- Judicial review (recours administratifs)
- Public service obligations

### Use Cases

- **Public administration**: Act validation
- **Administrative courts**: Judicial review
- **Government systems**: Compliance checking

---

## 11. Tort Law (Responsabilité délictuelle)

**Legal Basis**: Code civil, Articles 1240-1244

### Scope

Civil liability and damages:
- Article 1240: General tort liability
- Article 1241: Negligence liability
- Article 1242: Vicarious liability

### Use Cases

- **Insurance systems**: Liability assessment
- **Litigation support**: Damages calculation
- **Legal advice**: Tort claim validation

---

## Domain Relationships

```
┌────────────────────────────────────────────────┐
│         Legal Reasoning Engine                 │
│    (Meta-layer for all domains)                │
└────────────────────────────────────────────────┘
                     ▲
                     │
        ┌────────────┼────────────┐
        │            │            │
   ┌────▼───┐   ┌───▼────┐   ┌──▼─────┐
   │Contract│   │ Labor  │   │ Family │
   └────┬───┘   └───┬────┘   └──┬─────┘
        │           │            │
        ├───────────┴────────────┤
        │                        │
   ┌────▼──────┐          ┌─────▼────┐
   │Inheritance│          │ Property │
   └───────────┘          └──────────┘
```

### Cross-Domain Interactions

1. **Contract + Labor**: Employment contracts are special contracts
2. **Family + Inheritance**: Spouse rights in succession
3. **Property + Inheritance**: Real estate succession
4. **Evidence + All**: Proof requirements across domains
5. **Constitutional + All**: Fundamental rights protection

---

## Comparative Law Features

### vs. German Law (legalis-de)

| Feature | French | German |
|---------|--------|--------|
| **Employment** | 35h workweek | No federal limit |
| **Marriage age** | 18 years | 18 years |
| **Reserved portions** | 1/2 to 3/4 | 1/2 |
| **Patent term** | 20 years | 20 years |

### vs. Japanese Law (legalis-jp)

| Feature | French | Japanese |
|---------|--------|----------|
| **Divorce types** | 4 types | 2 types |
| **Copyright term** | Life + 70 | Life + 70 |
| **Company types** | SARL, SAS, SA | KK, GK |

---

## Next Steps

- **[Getting Started](./getting-started.md)** - Begin using legalis-fr
- **[User Guide](./user-guide.md)** - Practical examples for each domain
- **[API Patterns](./api-patterns.md)** - Best practices and patterns

---

**Questions?** Check the main [README](../README.md) or API documentation.
