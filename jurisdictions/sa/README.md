# Legalis-SA: Kingdom of Saudi Arabia Jurisdiction Support

Comprehensive Saudi Arabian legal system implementation for the Legalis-RS ecosystem, based on **Islamic Law (Sharia)** and modern regulatory frameworks.

## Overview

Legalis-SA (المملكة العربية السعودية) provides full support for Saudi Arabia's unique Islamic legal system, covering Basic Law, Sharia principles, company law, labor law, capital markets, tax law (VAT and Zakat), data protection, intellectual property, and arbitration. The crate offers bilingual support (Arabic/English) with Arabic as the authoritative language.

## Legal System: Islamic Law Foundation

Saudi Arabia operates under an **Islamic Legal System** based on:

### Primary Sources (المصادر الأساسية)

1. **القرآن الكريم** (Holy Quran) - Supreme divine source
2. **السنة النبوية** (Sunnah) - Prophetic tradition
3. **الإجماع** (Ijma) - Consensus of scholars
4. **القياس** (Qiyas) - Analogical reasoning

### Secondary Sources

5. **الأنظمة الملكية** (Royal Decrees) - نظام ملكي
6. **قرارات مجلس الوزراء** (Council of Ministers Resolutions)
7. **القرارات الوزارية** (Ministerial Decisions)

### Jurisprudence School

Saudi Arabia follows the **Hanbali school** (المذهب الحنبلي), one of the four major Sunni schools of Islamic jurisprudence, known for strict textual interpretation.

## Legal Hierarchy

```
القرآن الكريم والسنة النبوية
(Quran and Sunnah - Supreme Authority)
    ↓
النظام الأساسي للحكم (Basic Law of Governance - 1992)
    ↓
الأنظمة الملكية (Royal Decrees)
    • نظام الشركات (Companies Law)
    • نظام العمل (Labor Law)
    • نظام السوق المالية (Capital Market Law)
    • نظام ضريبة القيمة المضافة (VAT Law)
    ↓
قرارات مجلس الوزراء (Council of Ministers Resolutions)
    ↓
القرارات الوزارية (Ministerial Decisions)
```

## Vision 2030 Reforms

Since 2016, Saudi Arabia has undergone major legal and social reforms:
- Economic diversification beyond oil
- Women's rights expansion (driving, reduced guardianship)
- Entertainment and tourism development
- Foreign investment liberalization (100% ownership in many sectors)
- Capital market modernization
- Digital economy development

## Major Legal Areas Covered

### 1. Basic Law of Governance (النظام الأساسي للحكم) - 1992

Royal Decree A/90 dated 27/8/1412H:
- Saudi Arabia's constitutional document
- Government structure (Monarchy)
- Rights and duties of citizens
- Sharia as supreme law
- Consultative Council (مجلس الشورى)

```rust
use legalis_sa::basic_law::*;

let principles = get_basic_law_principles();
assert!(principles.contains(&"Sharia supremacy"));
```

### 2. Islamic Law (الشريعة الإسلامية)

#### Commercial Transactions (المعاملات التجارية)
- **Murabaha** (مرابحة) - Cost-plus financing
- **Musharakah** (مشاركة) - Partnership
- **Mudarabah** (مضاربة) - Profit-sharing
- **Ijarah** (إجارة) - Leasing
- Riba (usury) prohibition

#### Family Law (الأحوال الشخصية)
- Marriage (نكاح)
- Divorce (طلاق)
- Inheritance (الميراث / Faraid)

#### Hanbali Principles
- Strict adherence to Quran and Sunnah
- Minimal use of personal opinion (ra'y)
- Preservation of public interest (مصلحة)

```rust
use legalis_sa::islamic_law::*;

let contract = IslamicFinanceContract::builder()
    .contract_type(ContractType::Murabaha)
    .profit_rate(5.0)
    .build()?;

assert!(check_sharia_compliance(&contract).is_ok());
```

### 3. Companies Law (نظام الشركات) - 2015

Royal Decree M/3 dated 28/1/1437H:

#### Company Types
- **LLC** (شركة ذات مسؤولية محدودة) - Limited Liability Company
- **JSC** (شركة مساهمة) - Joint Stock Company
- **General Partnership** (شركة تضامن)
- **Limited Partnership** (شركة توصية بسيطة)
- **Professional Company** (شركة مهنية)

#### Foreign Investment
- 100% foreign ownership allowed in many sectors
- MISA (هيئة الاستثمار) licensing

```rust
use legalis_sa::company_law::*;

let company = CompanyRegistration::builder()
    .company_type(CompanyType::Llc)
    .name_ar("شركة التقنية المحدودة")
    .name_en("Tech Company LLC")
    .capital(Sar::from_riyals(500_000)) // Minimum SAR 500,000
    .build()?;

assert!(validate_registration(&company).is_ok());
```

### 4. Labor Law (نظام العمل) - 2005

Royal Decree M/51 dated 23/8/1426H:

#### Working Hours
- **Standard**: 8 hours/day, 48 hours/week maximum
- **Ramadan**: 6 hours/day for Muslims
- **Friday**: Weekly rest day (الجمعة)

#### Leave Entitlements
- **Annual leave**: 21 days (up to 30 after 5 years)
- **Sick leave**: 30 days full pay + 60 days 75% pay + 30 days unpaid
- **Eid holidays**: Eid al-Fitr and Eid al-Adha

#### End of Service Award (مكافأة نهاية الخدمة)
- **First 5 years**: Half month salary per year
- **After 5 years**: Full month salary per year

#### Saudization (Nitaqat / نطاقات)
- Mandatory Saudi employment quotas
- Color-coded compliance system (Platinum, Green, Yellow, Red)

```rust
use legalis_sa::labor_law::*;

let hours = WorkingHours::standard();
assert_eq!(hours.total_weekly_hours(), 48);

let eosa = calculate_eosa(7, Sar::from_riyals(10_000));
// 5 years × 0.5 month + 2 years × 1 month = 4.5 months
// 4.5 × SAR 10,000 = SAR 45,000
```

### 5. Capital Markets Law (نظام السوق المالية) - 2003

Royal Decree M/30 dated 2/6/1424H:

- **CMA** (هيئة السوق المالية) - Capital Market Authority
- **Tadawul** (تداول) - Saudi Stock Exchange
- Securities regulation
- Foreign investor access (Qualified Foreign Investors)

```rust
use legalis_sa::capital_markets::*;

let listing = validate_listing(
    SecurityType::Shares,
    Sar::from_riyals(100_000_000), // Market cap
)?;
```

### 6. Tax Law (الأنظمة الضريبية)

#### VAT (ضريبة القيمة المضافة)
- **Current rate**: **15%** (increased from 5% in July 2020)
- Registration threshold: SAR 375,000 annual turnover
- ZATCA (هيئة الزكاة والضريبة والجمارك) enforcement

#### Zakat (الزكاة)
- **Rate**: **2.5%** on net worth
- Applies to Saudi and GCC nationals
- Calculated on Hijri lunar year
- Religious obligation with legal enforcement

#### Corporate Income Tax (ضريبة الدخل)
- **Rate**: **20%** for foreign companies
- Saudi/GCC nationals pay Zakat, not income tax
- Withholding tax on various payments

```rust
use legalis_sa::tax_law::*;

// VAT calculation
let vat = calculate_vat(Sar::from_riyals(1_000), VatRate::Standard)?;
assert_eq!(vat.riyals(), 150); // 15%

// Zakat calculation
let zakat = calculate_zakat(Sar::from_riyals(100_000))?;
assert_eq!(zakat.riyals(), 2_500); // 2.5%
```

### 7. Personal Data Protection Law (نظام حماية البيانات الشخصية) - 2021

Royal Decree M/19 dated 9/2/1443H (PDPL):

- GDPR-inspired framework
- **SDAIA** (الهيئة السعودية للبيانات والذكاء الاصطناعي) enforcement
- Consent requirements
- Data localization (certain categories)
- Cross-border transfer restrictions
- Data subject rights

```rust
use legalis_sa::data_protection::*;

let processing = validate_processing(
    &DataCategory::Sensitive,
    &LegalBasis::ExplicitConsent,
    false, // Not cross-border
)?;
```

### 8. Intellectual Property

- **Patents** (براءات الاختراع): 20 years
- **Trademarks** (العلامات التجارية): 10 years, renewable
- **Copyright** (حقوق التأليف): Life + 50 years
- **Trade Secrets** (الأسرار التجارية)
- SAIP (الهيئة السعودية للملكية الفكرية) administration

```rust
use legalis_sa::intellectual_property::*;

let trademark = IpRegistration::new()
    .ip_type(IpType::Trademark)
    .protection_term(10)
    .register()?;
```

### 9. Arbitration (التحكيم) - 2012

Saudi Arbitration Law (Royal Decree M/34):
- Domestic and international arbitration
- SCCA (مركز التحكيم التجاري السعودي)
- New York Convention ratification
- Enforcement of foreign awards

```rust
use legalis_sa::arbitration::*;

let agreement = ArbitrationAgreement::builder()
    .arbitration_type(ArbitrationType::International)
    .seat("Riyadh")
    .governing_law("Saudi Arabian Law")
    .build()?;
```

## Hijri Calendar System

Saudi Arabia officially uses the **Islamic (Hijri) calendar** (التقويم الهجري):

```rust
use legalis_sa::common::HijriDate;

let date = HijriDate::new(1446, HijriMonth::Muharram, 1);
println!("{}", date.format_ar()); // "1 محرم 1446"

// Conversion to/from Gregorian
let gregorian = convert_hijri_to_gregorian(&date)?;
```

### Hijri Months (الشهور الهجرية)

1. محرم (Muharram)
2. صفر (Safar)
3. ربيع الأول (Rabi' al-Awwal)
4. ربيع الثاني (Rabi' al-Thani)
5. جمادى الأولى (Jumada al-Ula)
6. جمادى الآخرة (Jumada al-Akhirah)
7. رجب (Rajab)
8. شعبان (Sha'ban)
9. **رمضان (Ramadan)** - Fasting month
10. شوال (Shawwal)
11. ذو القعدة (Dhu al-Qi'dah)
12. ذو الحجة (Dhu al-Hijjah) - Hajj month

## Currency Support

Saudi Riyal (SAR / ر.س) handling:

```rust
use legalis_sa::common::Sar;

let amount = Sar::from_riyals(15_000);
println!("{}", amount.format_en()); // "SAR 15,000.00"
println!("{}", amount.format_ar()); // "15,000.00 ر.س"

// Halalas (fils) - 100 halalas = 1 riyal
let halalas = amount.halalas(); // 1,500,000
```

## Public Holidays

```rust
use legalis_sa::common::*;

let holidays = get_public_holidays(1446); // Hijri year

// Major holidays:
// - Eid al-Fitr (عيد الفطر) - 4 days
// - Eid al-Adha (عيد الأضحى) - 4 days
// - Saudi National Day (اليوم الوطني) - September 23
// - Founding Day (يوم التأسيس) - February 22

let is_holiday = is_public_holiday("1446-10-01"); // Eid al-Fitr
```

## Citation Format

Saudi legal citations use Royal Decree numbers:

```
المرسوم الملكي رقم م/3 بتاريخ 28/1/1437هـ
Royal Decree No. M/3 dated 28/1/1437H
```

```rust
use legalis_sa::citation::*;

let citation = SaudiCitation::royal_decree("M/3", "28/1/1437")
    .with_title_en("Companies Law")
    .with_title_ar("نظام الشركات")
    .with_article(3)
    .build();

println!("{}", citation);
```

## Bilingual Support

All types support **Arabic (العربية)** and **English**:

```rust
use legalis_sa::common::*;

let company_name_ar = "شركة التقنية المحدودة";
let company_name_en = "Tech Company LLC";

// Arabic is authoritative in legal interpretation
```

## Key Regulatory Bodies

- **مجلس الوزراء** - Council of Ministers
- **مجلس الشورى** - Shura Council (Consultative Council)
- **وزارة التجارة** - Ministry of Commerce (company registration)
- **وزارة العمل** - Ministry of Labor
- **هيئة السوق المالية** (CMA) - Capital Market Authority
- **هيئة الزكاة والضريبة والجمارك** (ZATCA) - Zakat, Tax and Customs Authority
- **الهيئة السعودية للبيانات والذكاء الاصطناعي** (SDAIA) - Data & AI Authority
- **الهيئة السعودية للملكية الفكرية** (SAIP) - IP Authority

## Documentation

- [Full API Documentation](https://docs.rs/legalis-sa)
- [Main Legalis-RS Project](../../README.md)
- [Saudi Legal Portal](https://laws.boe.gov.sa)
- [Vision 2030](https://www.vision2030.gov.sa)

## Related Crates

- `legalis-core` - Core framework
- `legalis-ae` - UAE law (similar GCC system)
- `legalis-my` - Malaysia (Islamic law comparison)

## License

Licensed under either of MIT or Apache-2.0 at your option.

## Disclaimer

This library is for educational and informational purposes. For legal matters in the Kingdom of Saudi Arabia, consult qualified Saudi legal professionals (محامي سعودي).

---

**إخلاء المسؤولية: هذه المكتبة لأغراض تعليمية وإعلامية فقط. للمسائل القانونية في المملكة العربية السعودية، استشر محامياً سعودياً مؤهلاً.**
