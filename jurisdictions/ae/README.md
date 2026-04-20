# legalis-ae

الإمارات العربية المتحدة (UAE) Legal System Support for Legalis-RS

**Version 0.1.3** - Federal Laws, Labor, Commercial Companies, Data Protection, Free Zones

## نظرة عامة (Overview)

`legalis-ae` provides comprehensive support for the United Arab Emirates legal system within the Legalis-RS framework. The UAE has a unique mixed legal system combining civil law, Islamic law (Sharia), and common law elements in its financial free zones.

## النظام القانوني الإماراتي (UAE Legal System)

The UAE legal system is characterized by:
- **Federal civil law** - Based on Egyptian civil code traditions
- **Islamic law (Sharia)** - Governs family law and personal status
- **Free zone common law** - DIFC and ADGM use English common law
- **Federal structure** - 7 Emirates with both federal and local laws

### Comparison with Other Legal Systems

| Feature | UAE | Egypt | UK | USA |
|---------|-----|-------|-----|-----|
| Legal Family | Civil/Islamic/Common | Civil | Common Law | Common Law |
| Main Source | Codes & Sharia | Codes | Case Law | Case Law |
| Constitution | 1971 (amended 2009) | 2014 | Uncodified | 1787 |
| Court System | Federal & Local | 3-tier | Supreme Court | Federal & State |
| Free Zones | DIFC, ADGM (Common Law) | N/A | N/A | N/A |

## الميزات المنفذة (Implemented Features)

### ✅ قانون العمل (Labor Law)

UAE Federal Decree-Law No. 33/2021 - Comprehensive labor regulations
- ✅ Employment contract types (limited/unlimited term)
- ✅ Working hours (8 hours/day, 48 hours/week)
- ✅ Leave entitlements (annual, sick, maternity, paternity)
- ✅ End of service benefits (gratuity calculation)
- ✅ Wage protection system
- ✅ Termination procedures

```rust
use legalis_ae::labor_law::{EmploymentContract, ContractType, GratuityCalculator};

let contract = EmploymentContract::new()
    .employee_name("أحمد محمد")
    .contract_type(ContractType::LimitedTerm { months: 24 })
    .monthly_salary(15_000) // AED
    .start_date("2022-01-01")
    .build()?;

// Calculate end of service gratuity
let gratuity = GratuityCalculator::calculate(&contract, 5 /* years */)?;
// First 5 years: 21 days salary per year
```

### ✅ قانون الشركات التجارية (Commercial Companies Law)

Federal Decree-Law No. 32/2021 - Company formation and governance
- ✅ Company types (LLC, PJSC, PrJSC, Partnership, Sole Proprietorship)
- ✅ Capital requirements
- ✅ Foreign ownership rules (up to 100% in non-strategic sectors)
- ✅ Corporate governance requirements
- ✅ Board composition rules

```rust
use legalis_ae::commercial_companies::{Company, CompanyType, validate_formation};

let company = Company::new()
    .name("شركة التكنولوجيا المحدودة")
    .company_type(CompanyType::LLC)
    .capital(300_000) // AED (minimum for LLC)
    .shareholders(vec!["Shareholder 1", "Shareholder 2"])
    .foreign_ownership_percentage(100) // Now allowed
    .build()?;

assert!(validate_formation(&company).is_ok());
```

### ✅ حماية البيانات (Data Protection)

Federal Decree-Law No. 45/2021 - Personal Data Protection Law
- ✅ Data subject rights (access, rectification, erasure)
- ✅ Lawful processing grounds
- ✅ Cross-border transfer restrictions
- ✅ Data controller obligations
- ✅ Data breach notification requirements
- ✅ Special categories (sensitive data)

```rust
use legalis_ae::data_protection::{DataProcessing, LawfulBasis, validate_processing};

let processing = DataProcessing::new()
    .controller("شركة البيانات")
    .purpose("Customer relationship management")
    .lawful_basis(LawfulBasis::Consent)
    .data_categories(vec!["name", "email", "phone"])
    .cross_border_transfer(false)
    .build()?;

assert!(validate_processing(&processing).is_ok());
```

### ✅ المناطق الحرة (Free Zones)

DIFC (Dubai International Financial Centre) and ADGM (Abu Dhabi Global Market)
- ✅ Common law jurisdiction recognition
- ✅ DIFC Courts system
- ✅ ADGM Courts system
- ✅ Financial services regulations
- ✅ Employment regulations (different from federal)

## 📊 Current Implementation Status

**Version 0.1.3 Statistics:**
- ✅ **Labor Law**: Federal Decree-Law No. 33/2021
- ✅ **Commercial Companies**: Federal Decree-Law No. 32/2021
- ✅ **Data Protection**: Federal Decree-Law No. 45/2021
- ✅ **Free Zones**: DIFC and ADGM frameworks
- ✅ **Modules**: 5 modules (labor_law, commercial_companies, data_protection, common, citation)

## Dependencies

- `chrono` - Date/time handling
- `serde` - Serialization
- `thiserror` - Error handling

## License

Apache-2.0

## Related Links

- [UAE Government Portal](https://u.ae/)
- [Ministry of Human Resources (MOHRE)](https://www.mohre.gov.ae/)
- [DIFC](https://www.difc.ae/)
- [ADGM](https://www.adgm.com/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
