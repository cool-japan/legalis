# Legalis-RU: Russian Federation Jurisdiction Support

Comprehensive Russian legal system implementation for the Legalis-RS ecosystem.

## Overview

Legalis-RU (Российская Федерация) provides full support for Russian law, covering civil, criminal, labor, tax, company, competition, data protection, and intellectual property law. The crate offers bilingual support (Russian/English) and implements Russia's civil law system with post-Soviet characteristics.

## Legal System

The Russian Federation operates under a **civil law system** based on:
- **Civil Code** (Гражданский кодекс РФ) - 4 parts, foundation of private law
- **Criminal Code** (Уголовный кодекс РФ) - Penal law
- **Labor Code** (Трудовой кодекс РФ) - Employment relations
- **Tax Code** (Налоговый кодекс РФ) - Taxation
- **Federal laws** - Specialized legislation

### Legal Hierarchy

```
Конституция РФ (Constitution of the Russian Federation)
    ↓
Федеральные конституционные законы (Federal Constitutional Laws)
    ↓
Федеральные законы (Federal Laws)
    • Гражданский кодекс (Civil Code)
    • Уголовный кодекс (Criminal Code)
    • Трудовой кодекс (Labor Code)
    • Налоговый кодекс (Tax Code)
    ↓
Указы Президента (Presidential Decrees)
    ↓
Постановления Правительства (Government Resolutions)
    ↓
Ведомственные акты (Ministerial Acts)
```

## Major Legal Areas Covered

### 1. Civil Code (Гражданский кодекс РФ)

Enacted in 4 parts (1994-2006), the Civil Code governs:

#### Part I (Часть первая)
- General provisions
- Legal persons
- Juristic acts
- Representation
- Limitation periods

#### Part II (Часть вторая)
- Obligations
- Contract types
- Tort liability
- Unjust enrichment

#### Part III (Часть третья)
- Succession (inheritance)
- International private law

#### Part IV (Часть четвертая)
- Intellectual property rights

```rust
use legalis_ru::civil_code::*;

let contract = CivilLaw::validate_contract(
    ContractType::Sale,
    ObligationType::TransferOwnership,
)?;
```

### 2. Criminal Code (Уголовный кодекс РФ) - 1996

- Criminal offenses classification
- Penalties and sanctions
- Criminal liability principles

```rust
use legalis_ru::criminal_code::*;

let crime = Crime::new()
    .category(CrimeCategory::Grave)
    .liability(CriminalLiability::General);
```

### 3. Labor Code (Трудовой кодекс РФ) - 2001

- **Working time**: 40 hours/week standard
- **Overtime**: Limited to 4 hours/day, 120 hours/year
- **Leave**: Minimum 28 calendar days annual leave
- **Termination**: Strict procedures, severance requirements
- **Collective bargaining**: Trade union rights

```rust
use legalis_ru::labor_code::*;

let contract = EmploymentContract::builder()
    .employment_type(EmploymentType::Indefinite)
    .working_time(WorkingTimeRegime::Standard40Hours)
    .build()?;
```

### 4. Tax Code (Налоговый кодекс РФ)

#### VAT (НДС - Налог на добавленную стоимость)
- **Standard rate**: 20%
- **Reduced rate**: 10% (food, children's goods)
- **Zero rate**: 0% (exports)

#### Income Tax (НДФЛ - Налог на доходы физических лиц)
- **Standard rate**: 13% (residents on income up to 5M RUB)
- **Progressive**: 15% (above 5M RUB since 2021)
- **Non-residents**: 30%

#### Corporate Tax (Налог на прибыль организаций)
- **Federal rate**: 3%
- **Regional rate**: 17%
- **Total**: 20%

```rust
use legalis_ru::tax_code::*;

// VAT calculation
let vat = calculate_vat(100_000, VatRate::Standard)?; // 20,000 RUB

// Income tax
let tax = calculate_income_tax(1_000_000, true)?; // Resident
```

### 5. Company Law

#### Federal Law 14-FZ on Limited Liability Companies (ООО)
- Minimum 1 founder
- Charter capital: 10,000 RUB minimum
- General meeting of participants
- Director (единоличный исполнительный орган)

#### Joint-Stock Companies (АО)
- Public (ПАО) and Non-Public (АО)
- Board of directors
- Shareholder rights

```rust
use legalis_ru::company_law::*;

let llc = LimitedLiabilityCompany::builder()
    .name("ООО Технологии Будущего")
    .charter_capital(10_000_00) // 100,000 RUB
    .add_founder("Иванов И.И.", 50.0)
    .add_founder("Петров П.П.", 50.0)
    .build()?;
```

### 6. Data Protection - Federal Law 152-FZ (О персональных данных) - 2006

Russia's personal data protection law:
- Consent requirements
- Data subject rights
- Security measures
- **Data localization**: Personal data of Russian citizens must be stored in Russia
- Cross-border transfer restrictions

```rust
use legalis_ru::data_protection::*;

let processing = PersonalDataOperator::new()
    .data_category(DataCategory::General)
    .consent_type(ConsentType::Written)
    .security_measures(vec![
        SecurityMeasure::Encryption,
        SecurityMeasure::AccessControl,
    ])
    .validate()?;
```

### 7. Competition Law - Federal Law 135-FZ (О защите конкуренции) - 2006

- Prohibition of anti-competitive agreements
- Abuse of dominant position (>35% market share)
- Merger control
- FAS (ФАС - Федеральная антимонопольная служба) enforcement

```rust
use legalis_ru::competition_law::*;

let dominance = DominantPosition::new()
    .market_share(MarketShare::new(45.0))
    .relevant_market("Telecommunications");

assert!(dominance.requires_notification());
```

### 8. Intellectual Property

- **Patents** (Патенты): 20 years
- **Trademarks** (Товарные знаки): 10 years, renewable
- **Copyright** (Авторское право): Life + 70 years
- **Trade secrets** (Коммерческая тайна)

```rust
use legalis_ru::intellectual_property::*;

let patent = Patent::new()
    .patent_type(PatentType::Invention)
    .protection_term(20)
    .register()?;
```

## Bilingual Support

All types support both **Russian (русский)** and **English**:

```rust
use legalis_ru::common::Currency;

let amount = Currency::from_rubles(100_000);
println!("{}", amount.format_ru()); // "100 000,00 ₽"
println!("{}", amount.format_en()); // "RUB 100,000.00"
```

Russian text is **authoritative** in legal interpretation.

## Currency Support

Russian Ruble (RUB / ₽) handling:

```rust
use legalis_ru::common::Currency;

let salary = Currency::from_rubles(150_000);
let kopecks = salary.kopecks(); // 15,000,000 kopecks

assert_eq!(salary.format_ru(), "150 000,00 ₽");
```

## Citation Format

Russian legal citations:

```
Гражданский кодекс РФ, ст. 421 (Civil Code, Article 421)
ФЗ № 14-ФЗ от 08.02.1998 (Federal Law No. 14-FZ of 08.02.1998)
Постановление ВС РФ (Supreme Court Resolution)
```

```rust
use legalis_ru::citation::*;

let citation = Citation::statute()
    .document_type(DocumentType::FederalLaw)
    .number("14-FZ")
    .date("08.02.1998")
    .article(21)
    .build();
```

## Legal Calendar

Russian holidays and business days:

```rust
use legalis_ru::common::*;

let is_holiday = is_russian_holiday("2024-01-01"); // New Year
let working_days = calculate_business_days("2024-01-01", "2024-01-31");

assert!(is_holiday);
```

## Key Regulatory Bodies

- **ФАС** (FAS) - Federal Antimonopoly Service
- **Роскомнадзор** (Roskomnadzor) - Communications regulator, data protection
- **ЦБ РФ** (Central Bank of Russia) - Banking and financial regulation
- **Росреестр** (Rosreestr) - Property registration
- **ФНС** (Federal Tax Service) - Tax administration

## Code Examples

### Employment Contract

```rust
use legalis_ru::labor_code::*;

let contract = EmploymentContract::builder()
    .employer("ООО Прогресс")
    .employee("Сидоров А.В.")
    .position("Программист")
    .salary(Currency::from_rubles(200_000))
    .employment_type(EmploymentType::Indefinite)
    .probation_period(3) // 3 months
    .build()?;

assert!(contract.is_valid());
```

### Civil Contract

```rust
use legalis_ru::civil_code::*;

let contract = CivilLaw::create_contract()
    .contract_type(ContractType::Sale)
    .parties("Продавец ООО А", "Покупатель ООО Б")
    .subject("Программное обеспечение")
    .price(Currency::from_rubles(500_000))
    .validate()?;
```

### Tax Compliance

```rust
use legalis_ru::tax_code::*;

// Corporate tax on 1,000,000 RUB profit
let corporate_tax = calculate_corporate_tax(1_000_000_00)?;
assert_eq!(corporate_tax.rate(), 20.0);
assert_eq!(corporate_tax.amount(), 200_000_00); // 200,000 RUB
```

## Documentation

- [Full API Documentation](https://docs.rs/legalis-ru)
- [Main Legalis-RS Project](../../README.md)
- [Russian Legal Portal](http://www.consultant.ru) - КонсультантПлюс

## Related Crates

- `legalis-core` - Core framework
- `legalis-de` - German law (civil law family)
- `legalis-fr` - French law (civil law family)

## License

Licensed under either of MIT or Apache-2.0 at your option.

## Disclaimer

This library is for educational and informational purposes. For legal matters in the Russian Federation, consult qualified Russian legal professionals (Юрист / Адвокат).

---

**Отказ от ответственности: Эта библиотека предназначена только для образовательных и информационных целей. Для юридических вопросов в Российской Федерации обратитесь к квалифицированным российским юристам.**
