# Legalis-KR: South Korea Jurisdiction Support

Comprehensive South Korean legal system implementation for the Legalis-RS ecosystem.

## Overview

Legalis-KR (대한민국 법률 프레임워크) provides full support for South Korean law, covering civil, commercial, labor, criminal, data protection, tax, competition, and intellectual property law. The crate offers bilingual support (Korean/English) and implements Korea's civil law system with its unique characteristics.

## Legal System

South Korea operates under a **civil law system** influenced by:
- **German law** (BGB tradition)
- **Japanese law** (historical influence)
- **Modern Korean legal developments**

### Legal Hierarchy

```
헌법 (Constitution) - Supreme law
    ↓
법률 (Acts) - National Assembly legislation
    ↓
대통령령 (Presidential Decrees)
    ↓
총리령/부령 (Ministerial Orders)
    ↓
조례 (Local Ordinances)
```

## Major Legal Areas Covered

### 1. Civil Code (민법) - Enacted 1958

- **General Provisions** (총칙편) - Legal capacity, juristic acts
- **Property Rights** (물권법) - Ownership, security interests
- **Obligations** (채권법) - Contracts, torts
- **Family Law** (가족법) - Marriage, parent-child relations
- **Succession** (상속법) - Inheritance, wills

### 2. Commercial Code (상법) - Enacted 1962

- Company formation and governance
- Stock companies (주식회사), limited companies
- Bills and notes
- Maritime commerce

### 3. Labor Law (노동법)

#### Labor Standards Act (근로기준법) - 1953
- **Working hours**: 40 hours/week maximum
- **Overtime**: Extra compensation required
- **Severance pay**: After 1+ years employment
- **Leave entitlements**: Annual, sick, maternity

#### Employment Insurance Act (고용보험법) - 1993
- Unemployment benefits
- Job training support

#### Workers' Compensation Act (산재보상보험법) - 1963
- Industrial accident insurance

### 4. Data Protection

#### PIPA (개인정보 보호법) - Personal Information Protection Act - 2011
- Comprehensive privacy law
- Consent requirements
- **24-hour breach notification** rule
- Cross-border transfer restrictions
- Modeled after EU GDPR

### 5. Tax Law (세법)

- **Income Tax Act** (소득세법) - Progressive rates for individuals
- **Corporate Tax Act** (법인세법) - Corporate income tax
- **VAT Act** (부가가치세법) - **10% standard rate**
- Withholding tax regulations

### 6. Competition Law (공정거래법)

#### Fair Trade Act (독점규제 및 공정거래에 관한 법률) - 1980
- Anti-monopoly regulation
- Merger control
- Abuse of dominance
- Unfair trade practices
- KFTC (공정거래위원회) enforcement

### 7. Criminal Code (형법) - Enacted 1953

- Criminal offenses and penalties
- Sentencing guidelines
- Criminal liability

### 8. Intellectual Property (지식재산권법)

- **Patent Law**: 20-year protection
- **Trademark Law**: 10-year renewable protection
- **Copyright Law**: Life + 70 years
- Trade secrets

### 9. Financial Services (금융 서비스법)

- Banking regulations
- Securities law
- Insurance regulation
- FSC (금융위원회) oversight

## Bilingual Support

All types support both **Korean (한국어)** and **English**:

```rust
use legalis_kr::i18n::BilingualText;

let text = BilingualText::new("개인정보 보호법", "PIPA");
assert_eq!(text.ko, "개인정보 보호법");
assert_eq!(text.en, "PIPA");
```

Korean text is **authoritative** in legal interpretation.

## Citation Format

Korean legal citations follow this format:

```
법률명 제X조 제Y항 제Z호
(Law Name, Article X, Paragraph Y, Item Z)
```

Example:

```rust
use legalis_kr::citation::{cite, Citation};

let citation = cite::civil_code(103);
assert_eq!(citation.format_korean(), "민법 제103조");
assert_eq!(citation.format_english(), "Civil Code, Article 103");
```

## Currency Support

Korean Won (KRW / 원) handling:

```rust
use legalis_kr::common::KrwAmount;

let amount = KrwAmount::from_won(1_000_000); // 1 million won
let amount2 = KrwAmount::from_man(100.0);    // 100만원

assert_eq!(amount.format_korean(), "100.00만원");
assert_eq!(amount.format_english(), "KRW 1,000,000");
```

## Key Features

### Korean Naming Conventions

```rust
use legalis_kr::common::KoreanName;

let name = KoreanName::new("김", "철수"); // 김철수 (Kim Cheol-soo)
println!("{}", name.full_name());         // "김철수"
println!("{}", name.family_first());      // "Kim Cheol-soo"
```

### Company Types

```rust
use legalis_kr::CompanyType;

// 주식회사 (Stock Company)
let company = CompanyType::StockCompany;
assert_eq!(company.name_ko(), "주식회사");
assert_eq!(company.suffix_ko(), "주식회사");

// 유한회사 (Limited Company)
let llc = CompanyType::LimitedCompany;
assert_eq!(llc.name_ko(), "유한회사");
```

### PIPA Data Protection

```rust
use legalis_kr::data_protection::{PersonalInfoCategory, ProcessingBasis};

let category = PersonalInfoCategory::General;
let basis = ProcessingBasis::Consent;

// Validate processing legality
assert!(basis.is_lawful());
```

### Tax Calculations

```rust
use legalis_kr::tax_law::vat::{VatRate, calculate_vat};

let rate = VatRate::Standard; // 10%
let vat = calculate_vat(10_000, rate);
assert_eq!(vat, 1_000);
```

## Major Laws by Date

| Law | Korean Name | Effective Date | Articles |
|-----|-------------|----------------|----------|
| Civil Code | 민법 | 1958-01-01 | 1,118 |
| Criminal Code | 형법 | 1953-10-03 | 372 |
| Commercial Code | 상법 | 1962-01-20 | 924 |
| Labor Standards Act | 근로기준법 | 1953-05-10 | 116 |
| PIPA | 개인정보 보호법 | 2011-09-30 | 76 |
| Fair Trade Act | 공정거래법 | 1980-12-31 | 133 |

## Regulatory Bodies

- **법원 (Courts)**: Supreme Court, High Courts, District Courts
- **KFTC (공정거래위원회)**: Korea Fair Trade Commission
- **PIPC (개인정보보호위원회)**: Personal Information Protection Commission
- **FSC (금융위원회)**: Financial Services Commission
- **MOEL (고용노동부)**: Ministry of Employment and Labor
- **NTS (국세청)**: National Tax Service

## Usage Examples

### Civil Code Contract Validation

```rust
use legalis_kr::civil_code::obligations::*;

let contract = Contract::new()
    .with_parties("갑", "을")
    .with_consideration(1_000_000)
    .with_purpose("상품 판매");

assert!(contract.is_valid());
```

### Labor Law Compliance

```rust
use legalis_kr::labor_law::labor_standards::*;

let hours = WorkingHours::new(40); // 40 hours per week
assert!(hours.is_within_legal_limit());
```

### Company Registration

```rust
use legalis_kr::company_law::*;

let company = Company::builder()
    .name("테크혁신 주식회사")
    .company_type(CompanyType::StockCompany)
    .capital(100_000_000)
    .build();
```

## Documentation

- [Full API Documentation](https://docs.rs/legalis-kr)
- [Main Legalis-RS Project](../../README.md)
- [Korean Legal Resources](https://www.law.go.kr) - 국가법령정보센터

## Related Crates

- `legalis-core` - Core framework
- `legalis-jp` - Japanese law (historically related)
- `legalis-de` - German law (legal family influence)

## License

Licensed under either of MIT or Apache-2.0 at your option.

## Disclaimer

This library is for educational and informational purposes. For legal matters in South Korea, consult qualified Korean legal professionals (변호사).

---

**이 라이브러리는 교육 및 정보 제공 목적입니다. 한국 법률 문제는 반드시 자격을 갖춘 변호사와 상담하십시오.**
