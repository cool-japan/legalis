//! # Legalis-KR: South Korea Jurisdiction Support
//!
//! # 대한민국 법률 프레임워크 / South Korean Legal Framework
//!
//! Comprehensive implementation of Korean law for the Legalis ecosystem.
//!
//! ## Legal System Overview / 법률 체계 개요
//!
//! South Korea operates under a civil law system influenced by German and Japanese law.
//! The legal hierarchy is:
//!
//! 1. **헌법 (Constitution)** - Supreme law
//! 2. **법률 (Acts)** - Enacted by National Assembly
//! 3. **대통령령 (Presidential Decrees)** - Issued by President
//! 4. **총리령/부령 (Prime Ministerial/Ministerial Orders)** - Issued by ministries
//! 5. **조례 (Local Ordinances)** - Local government regulations
//!
//! ## Implemented Modules / 구현된 모듈
//!
//! ### Civil Law / 민법
//!
//! - **Civil Code (민법)** - Comprehensive civil code enacted 1958
//!   - General Provisions (총칙편)
//!   - Property Rights (물권법)
//!   - Obligations (채권법)
//!   - Family Law (가족법)
//!   - Succession (상속법)
//!
//! ### Commercial Law / 상법
//!
//! - **Commercial Code (상법)** - Company law and commercial transactions
//! - Company formation and governance
//! - Bills and notes
//! - Maritime commerce
//!
//! ### Labor Law / 노동법
//!
//! - **Labor Standards Act (근로기준법)** - 40 hours/week, severance pay
//! - **Employment Insurance Act (고용보험법)** - Unemployment benefits
//! - **Workers' Compensation Act (산재보상보험법)** - Industrial accident insurance
//!
//! ### Data Protection / 개인정보 보호
//!
//! - **PIPA (개인정보 보호법)** - Personal Information Protection Act
//! - Consent requirements
//! - Data breach notification (24 hours)
//!
//! ### Tax Law / 세법
//!
//! - **Income Tax Act (소득세법)** - Progressive individual taxation
//! - **Corporate Tax Act (법인세법)** - Corporate income tax
//! - **VAT Act (부가가치세법)** - 10% standard rate
//!
//! ### Competition Law / 공정거래법
//!
//! - **Fair Trade Act (공정거래법)** - Anti-monopoly regulation
//! - Merger control
//! - Abuse of dominance
//!
//! ### Other Areas
//!
//! - **Criminal Code (형법)** - Criminal offenses and penalties
//! - **Intellectual Property (지식재산권법)** - Patents, trademarks, copyrights
//! - **Financial Services (금융 서비스법)** - Financial regulation
//!
//! ## Bilingual Support / 이중 언어 지원
//!
//! All types support both Korean (한국어) and English text.
//! Korean text is authoritative in legal interpretation.
//!
//! ```rust
//! use legalis_kr::i18n::BilingualText;
//!
//! let text = BilingualText::new("개인정보 보호법", "PIPA");
//! assert_eq!(text.ko, "개인정보 보호법");
//! assert_eq!(text.en, "PIPA");
//! ```
//!
//! ## Citation Format / 인용 형식
//!
//! Korean legal citations follow the format: 법률명 제X조 제Y항 제Z호
//!
//! ```rust
//! use legalis_kr::citation::{cite, Citation};
//!
//! let citation = cite::civil_code(103);
//! assert_eq!(citation.format_korean(), "민법 제103조");
//! ```
//!
//! ## Key Legislation / 주요 법률
//!
//! | Law (법률) | Effective Date | Description |
//! |------------|----------------|-------------|
//! | 민법 | 1958-01-01 | Civil Code (1,118 articles) |
//! | 형법 | 1953-10-03 | Criminal Code |
//! | 상법 | 1962-01-20 | Commercial Code |
//! | 근로기준법 | 1953-05-10 | Labor Standards Act |
//! | 개인정보 보호법 | 2011-09-30 | Personal Information Protection Act |
//! | 공정거래법 | 1980-12-31 | Monopoly Regulation and Fair Trade Act |

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod administrative_law;
pub mod citation;
pub mod civil_code;
pub mod commercial_code;
pub mod common;
pub mod company_law;
pub mod competition_law;
pub mod criminal_code;
pub mod data_protection;
pub mod financial_services;
pub mod i18n;
pub mod intellectual_property;
pub mod labor_law;
pub mod procedure_law;
pub mod real_estate;
pub mod reasoning;
pub mod tax_law;

// Re-export commonly used types
pub use citation::{Citation, cite, laws};
pub use common::{
    CompanyName, KoreanName, KrwAmount, OrganizationForm, currency, dates, holidays, names,
};
pub use i18n::{BilingualText, Locale, terms};

// Re-export civil code types
pub use civil_code::{
    family::*, general_provisions::*, obligations::*, property::*, succession::*,
};

// Re-export commercial code types
pub use commercial_code::{
    BoardOfDirectors, CommercialCodeError, CommercialCodeResult, Company, CompanyType, Director,
};

// Re-export labor law types
pub use labor_law::{employment_insurance::*, labor_standards::*, workers_compensation::*};

// Re-export data protection types
pub use data_protection::{
    ConsentRecord, DataProtectionError, DataProtectionResult, PersonalInfoCategory, ProcessingBasis,
};

// Re-export tax law types
pub use tax_law::{corporate_tax::*, income_tax::*, vat::*};

// Re-export competition law types
pub use competition_law::{
    AbuseType, CompetitionLawError, CompetitionLawResult, merger_filing_threshold,
    requires_merger_filing,
};

// Re-export criminal code types
pub use criminal_code::{
    CriminalCase, CriminalCodeError, CriminalCodeResult, CriminalOffense, OffenseCategory,
    PenaltyType,
};

// Re-export IP types
pub use intellectual_property::{
    IpError, IpRegistration, IpResult, IpRightType, PATENT_TERM_YEARS, TRADEMARK_TERM_YEARS,
};

// Re-export financial services types
pub use financial_services::{
    FinancialProductType, FinancialServicesError, FinancialServicesResult,
};

// Re-export reasoning types
pub use reasoning::{
    InterpretationMethod, LegalProvision, PrecedentLevel, ReasoningError, ReasoningResult,
};

// Re-export administrative law types
pub use administrative_law::{administrative_procedure::*, information_disclosure::*};

// Re-export procedure law types
pub use procedure_law::{civil_procedure::*, criminal_procedure::*};

// Re-export real estate types
pub use real_estate::{housing_lease::*, real_estate_transaction::*};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Civil Code statute
pub fn create_civil_code_statute() -> Statute {
    Statute::new(
        "KR-CC-1958",
        "민법 / Civil Code",
        Effect::new(
            EffectType::Grant,
            "민사법률관계 / Civil legal relationships",
        ),
    )
    .with_jurisdiction("KR")
}

/// Create Criminal Code statute
pub fn create_criminal_code_statute() -> Statute {
    Statute::new(
        "KR-CrimC-1953",
        "형법 / Criminal Code",
        Effect::new(
            EffectType::Prohibition,
            "범죄행위 금지 / Prohibition of criminal acts",
        ),
    )
    .with_jurisdiction("KR")
}

/// Create Commercial Code statute
pub fn create_commercial_code_statute() -> Statute {
    Statute::new(
        "KR-ComC-1962",
        "상법 / Commercial Code",
        Effect::new(
            EffectType::Grant,
            "상사법률관계 / Commercial legal relationships",
        ),
    )
    .with_jurisdiction("KR")
}

/// Create Labor Standards Act statute
pub fn create_labor_standards_statute() -> Statute {
    Statute::new(
        "KR-LSA-1953",
        "근로기준법 / Labor Standards Act",
        Effect::new(
            EffectType::Obligation,
            "근로기준 준수 / Compliance with labor standards",
        ),
    )
    .with_jurisdiction("KR")
}

/// Create Personal Information Protection Act statute
pub fn create_pipa_statute() -> Statute {
    Statute::new(
        "KR-PIPA-2011",
        "개인정보 보호법 / Personal Information Protection Act",
        Effect::new(
            EffectType::Obligation,
            "개인정보 보호 / Personal information protection",
        ),
    )
    .with_jurisdiction("KR")
}

/// Create Employment Insurance Act statute
pub fn create_employment_insurance_statute() -> Statute {
    Statute::new(
        "KR-EIA-1993",
        "고용보험법 / Employment Insurance Act",
        Effect::new(EffectType::Grant, "실업급여 / Unemployment benefits"),
    )
    .with_jurisdiction("KR")
}

/// Create Workers' Compensation Act statute
pub fn create_workers_compensation_statute() -> Statute {
    Statute::new(
        "KR-IACCIA-1963",
        "산업재해보상보험법 / Industrial Accident Compensation Insurance Act",
        Effect::new(EffectType::Grant, "산재보상 / Workers' compensation"),
    )
    .with_jurisdiction("KR")
}

/// Create Income Tax Act statute
pub fn create_income_tax_statute() -> Statute {
    Statute::new(
        "KR-ITA-1949",
        "소득세법 / Income Tax Act",
        Effect::new(EffectType::Obligation, "소득세 납부 / Income tax payment"),
    )
    .with_jurisdiction("KR")
}

/// Create Corporate Tax Act statute
pub fn create_corporate_tax_statute() -> Statute {
    Statute::new(
        "KR-CTA-1949",
        "법인세법 / Corporate Tax Act",
        Effect::new(
            EffectType::Obligation,
            "법인세 납부 / Corporate tax payment",
        ),
    )
    .with_jurisdiction("KR")
}

/// Create VAT Act statute
pub fn create_vat_statute() -> Statute {
    Statute::new(
        "KR-VATA-1976",
        "부가가치세법 / Value-Added Tax Act",
        Effect::new(EffectType::Obligation, "부가가치세 납부 / VAT payment"),
    )
    .with_jurisdiction("KR")
}

/// Create Fair Trade Act statute
pub fn create_fair_trade_statute() -> Statute {
    Statute::new(
        "KR-MRFTA-1980",
        "독점규제 및 공정거래에 관한 법률 / Monopoly Regulation and Fair Trade Act",
        Effect::new(
            EffectType::Prohibition,
            "독점행위 금지 / Prohibition of monopolistic conduct",
        ),
    )
    .with_jurisdiction("KR")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_civil_code_statute() {
        let statute = create_civil_code_statute();
        assert!(statute.id.contains("CC"));
        assert!(statute.title.contains("민법"));
    }

    #[test]
    fn test_bilingual_text() {
        let text = BilingualText::new("테스트", "Test");
        assert_eq!(text.ko, "테스트");
        assert_eq!(text.en, "Test");
    }

    #[test]
    fn test_citation_format() {
        let citation = cite::civil_code(103);
        let formatted = citation.format_korean();
        assert!(formatted.contains("민법"));
        assert!(formatted.contains("제103조"));
    }

    #[test]
    fn test_krw_amount() {
        let amount = KrwAmount::from_man(100.0); // 100만원
        assert_eq!(amount.format_korean(), "100.00만원");
    }

    #[test]
    fn test_organization_form() {
        let form = OrganizationForm::StockCompany;
        assert_eq!(form.name_ko(), "주식회사");
    }
}
