# legalis-za

South Africa / Suid-Afrika - Legal System Support for Legalis-RS

**Version 0.1.6** - Constitution, Companies Act, POPIA, Labour Relations

## Overview / Oorsig

`legalis-za` provides comprehensive support for the South African legal system within the Legalis-RS framework. South Africa has a mixed legal system combining Roman-Dutch civil law, English common law, and indigenous customary law, all subject to the supreme Constitution.

## South African Legal System / Suid-Afrikaanse Regstelsel

The South African legal system is characterized by:
- **Mixed legal system** - Roman-Dutch civil law and English common law
- **Constitutional supremacy** - Constitution of 1996 is the supreme law
- **Bill of Rights** - Chapter 2 contains justiciable fundamental rights
- **Customary law** - Recognized and applied subject to the Constitution
- **Transformative constitutionalism** - Aimed at addressing historical injustices

### Comparison with Other Legal Systems

| Feature | South Africa | UK | Netherlands | USA |
|---------|--------------|-----|-------------|-----|
| Legal Family | Mixed (Civil/Common) | Common Law | Civil Law | Common Law |
| Main Source | Constitution & Statutes | Case Law | Codes | Case Law |
| Constitution | 1996 | Uncodified | 1814 | 1787 |
| Court System | Constitutional Court → SCA → High Courts | Supreme Court | Hoge Raad | Federal & State |
| Bill of Rights | Chapter 2 (Justiciable) | HRA 1998 | ECHR incorporated | Amendments 1-10 |

## Implemented Features / Geïmplementeerde Kenmerke

### ✅ Constitution / Grondwet

Constitution of the Republic of South Africa, 1996
- ✅ Chapter 2: Bill of Rights (ss 7-39)
  - Equality (s 9)
  - Human dignity (s 10)
  - Life (s 11)
  - Freedom and security (s 12)
  - Privacy (s 14)
  - Freedom of expression (s 16)
  - Labour rights (s 23)
  - Property (s 25)
- ✅ Limitation of rights (s 36)
- ✅ Interpretation (s 39)
- ✅ Horizontal application (s 8)

```rust
use legalis_za::constitution::{FundamentalRight, LimitationAnalysis};

let claim = ConstitutionalClaim::new()
    .right(FundamentalRight::Equality) // Section 9
    .applicant("Affected Person")
    .respondent("State Department")
    .build()?;

// Section 36 limitation analysis
let limitation = LimitationAnalysis::new()
    .nature_of_right(RightNature::Core)
    .importance_of_purpose(true)
    .nature_and_extent_of_limitation(LimitationExtent::Significant)
    .relation_between_limitation_and_purpose(true)
    .less_restrictive_means_available(false)
    .build()?;

assert!(limitation.is_justified());
```

### ✅ Companies Act / Maatskappywet

Companies Act 71 of 2008
- ✅ Company types (Profit: Ltd, Pty Ltd; Non-profit: NPC)
- ✅ Incorporation and registration (CIPC)
- ✅ Directors' duties (s 76)
- ✅ Business rescue (Chapter 6)
- ✅ Shareholder remedies (oppression remedy s 163)
- ✅ MOI (Memorandum of Incorporation)

```rust
use legalis_za::companies::{Company, CompanyType, validate_incorporation};

let company = Company::new()
    .name("Tech Solutions (Pty) Ltd")
    .company_type(CompanyType::PrivateCompany) // (Pty) Ltd
    .directors(vec!["Director 1"]) // Minimum 1 for Pty Ltd
    .registered_office("Johannesburg, Gauteng")
    .build()?;

assert!(validate_incorporation(&company).is_ok());
```

### ✅ POPIA (Protection of Personal Information Act)

Protection of Personal Information Act 4 of 2013
- ✅ 8 Conditions for lawful processing
- ✅ Data subject rights
- ✅ Responsible party obligations
- ✅ Information Officer registration
- ✅ Trans-border information flows
- ✅ Special personal information

```rust
use legalis_za::data_protection::{DataProcessing, LawfulBasis, validate_processing};

let processing = DataProcessing::new()
    .responsible_party("Data Company (Pty) Ltd")
    .purpose("Customer service")
    .lawful_basis(LawfulBasis::Consent)
    .data_categories(vec!["name", "ID number", "address"])
    .information_officer_registered(true)
    .build()?;

assert!(validate_processing(&processing).is_ok());
```

### ✅ Labour Relations / Arbeidsverhoudinge

Labour Relations Act 66 of 1995
- ✅ Unfair dismissal protection
- ✅ Unfair labour practices
- ✅ CCMA dispute resolution
- ✅ Collective bargaining
- ✅ Strikes and lockouts
- ✅ Organisational rights

Basic Conditions of Employment Act 75 of 1997
- ✅ Working hours (45 hours/week maximum)
- ✅ Leave (annual, sick, family responsibility, maternity)
- ✅ Overtime and pay
- ✅ Notice periods

Employment Equity Act 55 of 1998
- ✅ Prohibition of unfair discrimination
- ✅ Affirmative action
- ✅ Employment equity plans

```rust
use legalis_za::labor::{Employment, DismissalAnalysis};

let employment = Employment::new()
    .employee_name("John Mokoena")
    .employer("Manufacturing Co (Pty) Ltd")
    .monthly_salary(25_000) // ZAR
    .tenure_years(5)
    .build()?;

// Analyse dismissal fairness (LRA)
let dismissal = DismissalAnalysis::new()
    .reason(DismissalReason::Misconduct)
    .fair_procedure_followed(true)
    .substantively_fair(true)
    .build()?;

assert!(dismissal.is_fair());
```

### ✅ B-BBEE (Broad-Based Black Economic Empowerment)

Broad-Based Black Economic Empowerment Act 53 of 2003
- ✅ Scorecard elements
- ✅ B-BBEE levels (1-8 + Non-compliant)
- ✅ Ownership requirements
- ✅ Skills development
- ✅ Enterprise and supplier development

## 📊 Current Implementation Status

**Version 0.1.6 Statistics:**
- ✅ **Modules**: 16 modules (citation, common, companies, competition_law, constitution, criminal_law, customary_law, data_protection, environmental_law, financial_services, insolvency, intellectual_property, labor, property_law, statutes, tax_law)
- ✅ **Tests**: 162 tests passing, 0 warnings
- ✅ **Constitution**: Bill of Rights and limitation analysis
- ✅ **POPIA**: Full compliance framework
- ✅ **Companies Act**: Incorporation and governance
- ✅ **Labour Law**: LRA, BCEA, EEA framework

## 🚧 Planned Features

- 📋 Consumer Protection Act
- 📋 National Credit Act
- 📋 Tax legislation (Income Tax Act, VAT Act)
- 📋 Environmental law (NEMA)
- 📋 Mining law (MPRDA)

## Dependencies / Afhanklikhede

- `chrono` - Date/time handling
- `serde` - Serialization
- `thiserror` - Error handling

## License / Lisensie

Apache-2.0

## Related Links / Verwante Skakels

- [Government Gazette](https://www.gov.za/documents/government-gazette)
- [Constitutional Court](https://www.concourt.org.za/)
- [CIPC (Companies)](https://www.cipc.co.za/)
- [CCMA](https://www.ccma.org.za/)
- [Information Regulator (POPIA)](https://inforegulator.org.za/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
