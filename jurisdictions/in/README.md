# legalis-in

भारत (India) - Legal System Support for Legalis-RS

**Version 0.1.3** - Constitution, Companies Act, DPDPA, Labour Codes, IT Act

## अवलोकन (Overview)

`legalis-in` provides comprehensive support for the Indian legal system within the Legalis-RS framework. India has a common law system inherited from British colonial rule, with a written Constitution as the supreme law.

## भारतीय कानूनी प्रणाली (Indian Legal System)

The Indian legal system is characterized by:
- **Common law tradition** - Inherited from British India
- **Written Constitution** - World's longest constitution (1950)
- **Federal structure** - Union, State, and Concurrent lists
- **Personal laws** - Hindu, Muslim, Christian, Parsi personal laws
- **Codified statutes** - Indian Penal Code, Civil Procedure Code, etc.

### अन्य कानूनी प्रणालियों के साथ तुलना (Comparison)

| विशेषता (Feature) | भारत (India) | UK | USA | जापान (Japan) |
|-------------------|--------------|-----|-----|---------------|
| कानूनी परिवार (Legal Family) | Common Law | Common Law | Common Law | Civil Law |
| मुख्य स्रोत (Main Source) | Statutes & Case Law | Case Law | Case Law | Codes |
| संविधान (Constitution) | 1950 | Uncodified | 1787 | 1946 |
| न्यायालय प्रणाली (Court System) | Supreme → High → District | Supreme Court | Federal & State | 4-tier |
| मौलिक अधिकार (Fundamental Rights) | Part III | HRA 1998 | Bill of Rights | Chapter III |

## कार्यान्वित विशेषताएं (Implemented Features)

### ✅ संविधान (Constitution)

Constitution of India, 1950
- ✅ Part III: Fundamental Rights (Articles 12-35)
- ✅ Part IV: Directive Principles (Articles 36-51)
- ✅ Part IVA: Fundamental Duties (Article 51A)
- ✅ Seventh Schedule (Union, State, Concurrent Lists)
- ✅ Constitutional remedies (Article 32, 226)

```rust
use legalis_in::constitution::{FundamentalRight, validate_state_action};

let claim = ConstitutionalClaim::new()
    .right(FundamentalRight::RightToEquality) // Article 14
    .petitioner("Citizen")
    .respondent("State of Maharashtra")
    .state_action("Discriminatory policy")
    .build()?;

// Check if state action violates fundamental rights
let result = validate_state_action(&claim)?;
```

### ✅ कंपनी अधिनियम (Companies Act)

Companies Act, 2013
- ✅ Company types (Private, Public, OPC, Section 8)
- ✅ Incorporation requirements
- ✅ Directors' duties (Section 166)
- ✅ Shareholder remedies (oppression, mismanagement)
- ✅ Corporate governance (Board, AGM, EGM)
- ✅ CSR obligations (Section 135)

```rust
use legalis_in::companies::{Company, CompanyType, validate_incorporation};

let company = Company::new()
    .name("Tech Solutions Private Limited")
    .company_type(CompanyType::Private)
    .authorized_capital(10_00_000) // ₹10 lakhs
    .paid_up_capital(1_00_000) // ₹1 lakh
    .directors(vec!["Director 1", "Director 2"]) // Minimum 2 for Private
    .registered_office("Mumbai, Maharashtra")
    .build()?;

assert!(validate_incorporation(&company).is_ok());
```

### ✅ डिजिटल व्यक्तिगत डेटा संरक्षण (DPDPA)

Digital Personal Data Protection Act, 2023
- ✅ Data principal rights
- ✅ Data fiduciary obligations
- ✅ Consent requirements
- ✅ Cross-border data transfer
- ✅ Significant data fiduciaries
- ✅ Data Protection Board

```rust
use legalis_in::data_protection::{DataProcessing, LawfulBasis, validate_processing};

let processing = DataProcessing::new()
    .data_fiduciary("Tech Company Pvt Ltd")
    .purpose("Service delivery")
    .lawful_basis(LawfulBasis::Consent)
    .data_categories(vec!["name", "Aadhaar", "phone"])
    .cross_border_transfer(true)
    .adequate_jurisdiction(true) // Notified country
    .build()?;

assert!(validate_processing(&processing).is_ok());
```

### ✅ श्रम संहिताएं (Labour Codes)

Four Labour Codes (2019-2020)
- ✅ Code on Wages, 2019
- ✅ Industrial Relations Code, 2020
- ✅ Social Security Code, 2020
- ✅ Occupational Safety, Health and Working Conditions Code, 2020

```rust
use legalis_in::labour_codes::{Employment, WageCalculator};

let employment = Employment::new()
    .employee_name("राजेश कुमार")
    .employer("ABC Manufacturing Ltd")
    .monthly_wage(25_000) // ₹
    .working_hours_per_day(8)
    .state("Karnataka")
    .build()?;

// Calculate overtime (2x for hours beyond 8)
let overtime_pay = WageCalculator::overtime(&employment, 4)?;
```

### ✅ सूचना प्रौद्योगिकी अधिनियम (IT Act)

Information Technology Act, 2000 (as amended)
- ✅ Electronic records and signatures
- ✅ Cybercrimes (Sections 43, 66, 66A-66F)
- ✅ Intermediary liability (Section 79)
- ✅ Data protection rules (IT Rules 2011)

```rust
use legalis_in::it_act::{Intermediary, validate_safe_harbour};

let platform = Intermediary::new()
    .name("Social Media Platform")
    .user_count(50_00_000) // 50 lakhs (SSMI threshold)
    .grievance_officer_appointed(true)
    .compliance_officer_appointed(true)
    .content_moderation_policy(true)
    .build()?;

// Check safe harbour compliance
assert!(validate_safe_harbour(&platform).is_ok());
```

### ✅ जीएसटी (GST)

Goods and Services Tax Acts, 2017
- ✅ Registration requirements
- ✅ Tax rates (0%, 5%, 12%, 18%, 28%)
- ✅ Input tax credit
- ✅ Returns filing (GSTR-1, GSTR-3B)
- ✅ E-way bills

```rust
use legalis_in::gst::{GSTRegistration, TaxCalculator};

let registration = GSTRegistration::new()
    .business_name("Traders Pvt Ltd")
    .annual_turnover(50_00_000) // ₹50 lakhs (threshold)
    .state("Delhi")
    .build()?;

// Calculate GST
let gst = TaxCalculator::calculate(10_000, GSTRate::Eighteen)?; // 18%
```

### ✅ अनुबंध (Contract Law)

Indian Contract Act, 1872
- ✅ Formation (offer, acceptance, consideration)
- ✅ Capacity and free consent
- ✅ Void and voidable contracts
- ✅ Breach and remedies

### ✅ आपराधिक कानून (Criminal Law)

- ✅ Indian Penal Code, 1860 / Bharatiya Nyaya Sanhita, 2023
- ✅ Code of Criminal Procedure, 1973 / Bharatiya Nagarik Suraksha Sanhita, 2023
- ✅ Indian Evidence Act, 1872 / Bharatiya Sakshya Adhiniyam, 2023

## 📊 वर्तमान कार्यान्वयन स्थिति (Current Status)

**Version 0.1.3 Statistics:**
- ✅ **Modules**: 11 modules (constitution, companies, data_protection, labour_codes, it_act, gst, contract, criminal, common, citation)
- ✅ **Constitution**: Fundamental Rights and Directive Principles
- ✅ **DPDPA**: Full compliance framework
- ✅ **Companies Act**: Incorporation and governance
- ✅ **Labour Codes**: All four codes supported

## निर्भरताएं (Dependencies)

- `legalis-core` - Core types and traits
- `legalis-i18n` - Internationalization support
- `chrono` - Date/time handling
- `serde` - Serialization
- `thiserror` - Error handling

## लाइसेंस (License)

MIT OR Apache-2.0

## संबंधित लिंक (Related Links)

- [India Code](https://www.indiacode.nic.in/)
- [Supreme Court of India](https://main.sci.gov.in/)
- [Ministry of Law and Justice](https://lawmin.gov.in/)
- [MCA (Companies)](https://www.mca.gov.in/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
