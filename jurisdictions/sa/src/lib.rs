//! # Legalis-SA: Kingdom of Saudi Arabia Jurisdiction Support
//!
//! Comprehensive Saudi legal system implementation for Legalis-RS.
//!
//! ## Legal System Overview
//!
//! Saudi Arabia operates under an **Islamic Legal System** based on:
//! - **القرآن الكريم** - Holy Quran (Primary source)
//! - **السنة النبوية** - Sunnah (Prophetic tradition)
//! - **الأنظمة** - Royal Decrees and Regulations
//! - **قرارات مجلس الوزراء** - Council of Ministers Resolutions
//!
//! The legal system is primarily based on **Sharia (Islamic Law)** with the
//! **Hanbali school** (المذهب الحنبلي) as the predominant jurisprudence.
//!
//! ## Hierarchy of Laws
//!
//! 1. **النظام الأساسي للحكم** - Basic Law of Governance (1992)
//! 2. **القرآن والسنة** - Quran and Sunnah (Supreme authority)
//! 3. **الأنظمة الملكية** - Royal Decrees (نظام ملكي)
//! 4. **قرارات مجلس الوزراء** - Council of Ministers Resolutions
//! 5. **القرارات الوزارية** - Ministerial Decisions
//! 6. **التعاميم** - Circulars
//!
//! ## Vision 2030 Reforms
//!
//! Since 2016, Saudi Arabia has undergone significant legal reforms:
//! - Economic diversification away from oil
//! - Women's rights expansion (driving, guardianship reform)
//! - Entertainment and tourism sector development
//! - Foreign investment liberalization
//! - Capital market modernization
//!
//! ## Key Legal Areas
//!
//! ### [`basic_law`] - النظام الأساسي للحكم (1992)
//!
//! Saudi Arabia's constitutional document:
//! - Government structure (Monarchy)
//! - Rights and duties
//! - Sharia as supreme law
//! - Consultation (Shura) Council
//!
//! ### [`islamic_law`] - الشريعة الإسلامية
//!
//! Islamic law integration covering:
//! - Commercial transactions (المعاملات)
//! - Family law (الأحوال الشخصية)
//! - Inheritance (المواريث)
//! - Hanbali jurisprudence principles
//!
//! ### [`company_law`] - نظام الشركات (2015)
//!
//! Companies Law (Royal Decree M/3 dated 28/1/1437H):
//! - LLC, JSC, and other company types
//! - Corporate governance
//! - Foreign investment (100% ownership in many sectors)
//! - SAMA regulations for financial companies
//!
//! ### [`labor_law`] - نظام العمل (2005)
//!
//! Labor Law (Royal Decree M/51 dated 23/8/1426H):
//! - 8 hours/day, 48 hours/week maximum
//! - End of Service Award (مكافأة نهاية الخدمة)
//! - Nitaqat (Saudization) program
//! - Working hours during Ramadan (6 hours/day)
//!
//! ### [`capital_markets`] - نظام السوق المالية
//!
//! Capital Market Law (Royal Decree M/30 dated 2/6/1424H):
//! - CMA (هيئة السوق المالية) authority
//! - Tadawul (Saudi Stock Exchange)
//! - Securities regulations
//! - Foreign investor access
//!
//! ### [`tax_law`] - الأنظمة الضريبية
//!
//! Taxation system including:
//! - **VAT (ضريبة القيمة المضافة)**: 15% (increased from 5% in 2020)
//! - **Zakat (الزكاة)**: 2.5% for Saudi/GCC nationals
//! - **Corporate Income Tax (ضريبة الدخل)**: 20% for foreign companies
//! - **Withholding Tax**: Various rates
//!
//! ### [`data_protection`] - نظام حماية البيانات الشخصية (2021)
//!
//! Personal Data Protection Law (PDPL):
//! - GDPR-inspired framework
//! - SDAIA (الهيئة السعودية للبيانات والذكاء الاصطناعي) enforcement
//! - Data localization requirements
//! - Cross-border transfer restrictions
//!
//! ## Quick Start
//!
//! ```rust
//! use legalis_sa::{
//!     citation::{SaudiCitation, common_citations},
//!     labor_law::{WorkingHours, EndOfServiceAward, calculate_eosa},
//!     company_law::{CompanyType, validate_registration},
//!     common::{Sar, HijriDate},
//!     tax_law::{VatRate, ZakatRate},
//! };
//!
//! // Citation example
//! let citation = SaudiCitation::royal_decree("M/3", "28/1/1437")
//!     .with_title_en("Companies Law")
//!     .with_article(3);
//! println!("{}", citation);
//!
//! // Currency formatting
//! let salary = Sar::from_riyals(15000);
//! println!("{}", salary.format_en()); // SAR 15,000.00
//! println!("{}", salary.format_ar()); // 15,000.00 ر.س
//!
//! // EOSA calculation (End of Service Award)
//! let eosa = calculate_eosa(5, Sar::from_riyals(10000));
//! println!("Award: {}", eosa.award_amount);
//!
//! // VAT rate
//! assert_eq!(VatRate::Standard.rate(), 15.0);
//!
//! // Zakat rate
//! assert_eq!(ZakatRate::Standard.rate(), 2.5);
//! ```
//!
//! ## Major Laws Covered
//!
//! | Law | Name (AR) | Name (EN) | Royal Decree |
//! |-----|-----------|-----------|--------------|
//! | Basic Law | النظام الأساسي للحكم | Basic Law of Governance | A/90 (1992) |
//! | Companies | نظام الشركات | Companies Law | M/3 (2015) |
//! | Labor | نظام العمل | Labor Law | M/51 (2005) |
//! | Capital Markets | نظام السوق المالية | Capital Market Law | M/30 (2003) |
//! | PDPL | نظام حماية البيانات الشخصية | Personal Data Protection | M/19 (2021) |
//! | VAT | نظام ضريبة القيمة المضافة | VAT Law | M/113 (2017) |
//! | Income Tax | نظام ضريبة الدخل | Income Tax Law | M/1 (2004) |
//!
//! ## Calendar System
//!
//! Saudi Arabia officially uses the **Hijri (Islamic) calendar**:
//! - 12 lunar months
//! - Official documents use Hijri dates
//! - Conversion to/from Gregorian available
//! - Fiscal year based on Hijri calendar
//!
//! ## Language
//!
//! - **Arabic** is the official language
//! - All laws are originally in Arabic
//! - English translations available but Arabic is authoritative
//!
//! ## Disclaimer
//!
//! This library is for educational and informational purposes. For legal matters,
//! consult qualified Saudi legal professionals (محامي سعودي).

pub mod arbitration;
pub mod basic_law;
pub mod capital_markets;
pub mod citation;
pub mod common;
pub mod company_law;
pub mod data_protection;
pub mod intellectual_property;
pub mod islamic_law;
pub mod labor_law;
pub mod reasoning;
pub mod tax_law;

// Re-export citation types
pub use citation::{DecreeType, SaudiCitation, common_citations};

// Re-export common types
pub use common::{
    HijriDate, HijriMonth, Sar, SaudiHoliday, SaudiHolidayType, convert_gregorian_to_hijri,
    convert_hijri_to_gregorian, get_public_holidays, is_public_holiday, is_working_day,
    working_days_between,
};

// Re-export basic law types
pub use basic_law::{
    BasicLawArticle, BasicLawError, BasicLawResult, GovernmentBranch, get_basic_law_principles,
};

// Re-export Islamic law types
pub use islamic_law::{
    CommercialTransaction, CommercialTransactionError, CommercialTransactionResult, FamilyLaw,
    FamilyLawError, FamilyLawResult, HanbaliPrinciple, HeritageRelationship, InheritanceShare,
    IslamicFinanceContract, calculate_inheritance, check_sharia_compliance, get_hanbali_principles,
};

// Re-export company law types
pub use company_law::{
    CompanyError, CompanyRegistration, CompanyResult, CompanyType, GovernanceRequirements,
    get_company_checklist, validate_registration,
};

// Re-export labor law types
pub use labor_law::{
    ContractType, EndOfServiceAward, LaborError, LaborResult, LeaveType, NitaqatCategory,
    TerminationReason, WorkingHours, calculate_eosa, get_labor_checklist, validate_working_hours,
};

// Re-export capital markets types
pub use capital_markets::{
    CmaError, CmaLicense, CmaResult, InvestorType, SecurityType, get_cma_checklist,
    validate_listing,
};

// Re-export tax law types
pub use tax_law::{
    CorporateIncomeTax, TaxError, TaxResult, VatRate, VatRegistration, ZakatRate,
    calculate_corporate_tax, calculate_vat, calculate_zakat, get_tax_checklist,
};

// Re-export data protection types
pub use data_protection::{
    DataCategory, DataProtectionError, DataProtectionResult, DataSubjectRight, LegalBasis,
    ProcessingPurpose, get_pdpl_checklist, validate_processing,
};

// Re-export intellectual property types
pub use intellectual_property::{
    IpError, IpRegistration, IpResult, IpType, PatentType, TrademarkClass, get_ip_checklist,
};

// Re-export arbitration types
pub use arbitration::{
    ArbitrationAgreement, ArbitrationError, ArbitrationResult, ArbitrationType, DisputeResolution,
    get_arbitration_checklist,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_display() {
        let citation = SaudiCitation::royal_decree("M/3", "28/1/1437")
            .with_title_en("Companies Law")
            .with_article(3);

        let formatted = citation.to_string();
        assert!(formatted.contains("M/3"));
        assert!(formatted.contains("Article 3"));
    }

    #[test]
    fn test_sar_formatting() {
        let amount = Sar::from_riyals(15000);
        assert_eq!(amount.format_en(), "SAR 15,000.00");
    }

    #[test]
    fn test_eosa_calculation() {
        let eosa = calculate_eosa(5, Sar::from_riyals(10000));
        // 5 years of service award
        assert!(eosa.award_amount.riyals() > 0);
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 48);
    }

    #[test]
    fn test_company_types() {
        let llc = CompanyType::Llc;
        assert!(llc.has_limited_liability());
        assert!(llc.minimum_shareholders() >= 1);
    }

    #[test]
    fn test_vat_rate() {
        assert_eq!(VatRate::Standard.rate(), 15.0);
        assert_eq!(VatRate::Zero.rate(), 0.0);
    }

    #[test]
    fn test_zakat_rate() {
        assert_eq!(ZakatRate::Standard.rate(), 2.5);
    }

    #[test]
    fn test_data_protection() {
        let result = validate_processing(&DataCategory::General, &LegalBasis::Consent, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_public_holidays() {
        let holidays = get_public_holidays(1446); // Hijri year
        assert!(!holidays.is_empty());
    }

    #[test]
    fn test_common_citations() {
        let companies = common_citations::companies_law_2015();
        assert_eq!(companies.decree_number, "M/3");

        let labor = common_citations::labor_law_2005();
        assert_eq!(labor.decree_number, "M/51");
    }

    #[test]
    fn test_leave_entitlements() {
        assert_eq!(LeaveType::Annual.statutory_days(), 21);
    }

    #[test]
    fn test_hijri_month_names() {
        assert_eq!(HijriMonth::Muharram.name_ar(), "محرم");
        assert_eq!(HijriMonth::Ramadan.name_en(), "Ramadan");
    }
}
