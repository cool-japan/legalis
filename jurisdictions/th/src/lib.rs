//! # Legalis-TH: Thai Legal Framework
//!
//! Comprehensive Thai legal framework implementation in Pure Rust.
//!
//! ## Overview
//!
//! Thailand operates under a **Civil Law** system with Buddhist influence.
//! This library provides validation, types, and utilities for key Thai legislation:
//!
//! | Domain | Law | Year (B.E./CE) | Key Provisions |
//! |--------|-----|----------------|----------------|
//! | Constitution | รัฐธรรมนูญ | 2560/2017 | Fundamental rights, monarchy, Buddhism |
//! | Civil & Commercial | ประมวลกฎหมายแพ่งและพาณิชย์ | 2535/1992 | Obligations, contracts, property, family |
//! | Foreign Business | พ.ร.บ. ประกอบธุรกิจของคนต่างด้าว | 2542/1999 | Foreign ownership restrictions |
//! | Labor | พ.ร.บ. คุ้มครองแรงงาน | 2541/1998 | 48h/week, severance, SSF |
//! | Data Protection | พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล (PDPA) | 2562/2019 | Thailand's GDPR |
//! | Investment | พ.ร.บ. ส่งเสริมการลงทุน (BOI) | 2520/1977 | Tax incentives, EEC |
//! | Land | ประมวลกฎหมายที่ดิน | 2497/1954 | Foreign ownership restrictions |
//!
//! ## Buddhist Era Calendar (พุทธศักราช - พ.ศ.)
//!
//! Thailand uses the Buddhist Era calendar for all legal documents.
//! The Buddhist Era is 543 years ahead of the Common Era.
//!
//! ```rust
//! use legalis_th::calendar::{ce_to_be, be_to_ce, BuddhistYear, BuddhistDate};
//!
//! // Convert between calendars
//! assert_eq!(ce_to_be(2024), 2567);
//! assert_eq!(be_to_ce(2562), 2019); // PDPA year
//!
//! // Create Buddhist dates
//! let date = BuddhistDate::new(15, 6, 2567).expect("valid date");
//! assert_eq!(date.format_th(), "15 มิถุนายน พ.ศ. 2567");
//! ```
//!
//! ## Thai Legal Citation Format
//!
//! Thai legal citations follow specific formats:
//!
//! ```rust
//! use legalis_th::citation::{ThaiAct, ThaiConstitution};
//! use legalis_th::calendar::BuddhistYear;
//!
//! // Act citation
//! let pdpa = ThaiAct::new(
//!     "คุ้มครองข้อมูลส่วนบุคคล",
//!     "Personal Data Protection Act",
//!     BuddhistYear::from_be(2562),
//! );
//! assert_eq!(pdpa.format_th(), "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562");
//!
//! // Section reference
//! let section = pdpa.section(26);
//! assert_eq!(section.format_th(), "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26");
//!
//! // Constitution citation
//! let constitution = ThaiConstitution::current(); // 2560/2017
//! assert_eq!(constitution.format_th(), "รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. 2560");
//! ```
//!
//! ## Key Legal Principles
//!
//! ### Foreign Business Act (FBA) - พ.ร.บ. ประกอบธุรกิจของคนต่างด้าว พ.ศ. 2542
//!
//! Three-tier restriction system for foreign businesses:
//!
//! | List | Restriction | Examples |
//! |------|-------------|----------|
//! | List 1 | Prohibited | Media, land trading, forestry |
//! | List 2 | Cabinet approval required | Arms, domestic transport |
//! | List 3 | License required | Retail, construction, legal services |
//!
//! Exemptions available for:
//! - BOI-promoted investments
//! - ASEAN treaty benefits
//! - US Treaty of Amity
//!
//! ### Labor Protection Act (LPA) - พ.ร.บ. คุ้มครองแรงงาน พ.ศ. 2541
//!
//! | Provision | Details |
//! |-----------|---------|
//! | Working Hours | 48h/week maximum (8h/day, 6 days) |
//! | Overtime | Max 36h/week with consent |
//! | Rest Days | Minimum 1 day/week |
//! | Public Holidays | Minimum 13 days/year |
//! | Annual Leave | 6+ days after 1 year |
//! | Sick Leave | Up to 30 days/year with pay |
//! | Severance | 30-400 days based on tenure |
//!
//! ### Personal Data Protection Act (PDPA) - พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562
//!
//! | Aspect | PDPA (Thailand) |
//! |--------|-----------------|
//! | Legal Bases | 6 (consent, contract, legal obligation, vital interests, public task, legitimate interests) |
//! | Data Subject Rights | 8 (access, rectification, erasure, restriction, portability, objection, automated decisions) |
//! | Breach Notification | 72 hours to PDPC |
//! | Authority | PDPC (Personal Data Protection Committee) |
//! | Penalties | Up to 5M THB (admin), 1M THB (criminal) |
//!
//! ### Board of Investment (BOI) - คณะกรรมการส่งเสริมการลงทุน
//!
//! | Incentive Category | CIT Exemption | Import Duty | Other |
//! |--------------------|---------------|-------------|-------|
//! | A1 (High Priority) | 8 years + 50% 5 years | 0% | Zone benefits |
//! | A2 | 8 years | 0% | Zone benefits |
//! | A3 | 5 years | 0% | - |
//! | A4 | 3 years | 0% | - |
//! | B1-B2 | 0 years | 0% | Import duty only |
//!
//! ## Module Structure
//!
//! - [`calendar`] - Buddhist Era calendar system
//! - [`citation`] - Thai legal citation formatting
//! - [`labor_law`] - Labor Protection Act
//! - [`data_protection`] - PDPA
//! - [`foreign_business`] - Foreign Business Act
//!
//! ## Bilingual Support
//!
//! All types support both Thai (authoritative) and English:
//!
//! ```rust
//! use legalis_th::citation::ThaiCourtLevel;
//!
//! let court = ThaiCourtLevel::SupremeCourt;
//! assert_eq!(court.name_th(), "ศาลฎีกา");
//! assert_eq!(court.name_en(), "Supreme Court");
//! ```
//!
//! ## References
//!
//! - [Royal Thai Government Gazette](http://www.ratchakitcha.soc.go.th/)
//! - [Office of the Council of State](http://web.krisdika.go.th/)
//! - [Board of Investment](https://www.boi.go.th/)
//! - [Department of Labour Protection](https://www.labour.go.th/)
//! - [PDPC - Personal Data Protection Committee](https://www.pdpc.or.th/)

pub mod calendar;
pub mod citation;
pub mod data_protection;
pub mod foreign_business;
pub mod labor_law;

// Re-export calendar types
pub use calendar::{
    BE_CE_OFFSET, BuddhistDate, BuddhistYear, ThaiEra, be_to_ce, ce_to_be, format_buddhist_year,
    format_buddhist_year_en,
};

// Re-export citation types
pub use citation::{
    ThaiAct, ThaiActSection, ThaiCitation, ThaiConstitution, ThaiConstitutionSection,
    ThaiCourtDecision, ThaiCourtLevel, ThaiLegalInstrumentType,
};

// Re-export data protection types
pub use data_protection::{
    DataCategory, DataSubjectRight, LegalBasis, PdpaCompliance, PdpaError, PdpaResult,
    PersonalDataProcessing, ProcessingPurpose,
};

// Re-export foreign business types
pub use foreign_business::{
    BusinessActivity, BusinessRestrictionList, FbaCompliance, FbaError, FbaResult,
    ForeignBusinessLicense, OwnershipStructure,
};

// Re-export labor law types
pub use labor_law::{
    EmploymentContract, EmploymentType, LaborCompliance, LpaError, LpaResult, Severance,
    TerminationType, WorkingHours,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_conversion() {
        assert_eq!(ce_to_be(2024), 2567);
        assert_eq!(be_to_ce(2562), 2019);
    }

    #[test]
    fn test_buddhist_year() {
        let year = BuddhistYear::from_ce(2019);
        assert_eq!(year.be_year, 2562);
        assert_eq!(year.format_th(), "พ.ศ. 2562");
    }

    #[test]
    fn test_thai_act_citation() {
        let pdpa = ThaiAct::new(
            "คุ้มครองข้อมูลส่วนบุคคล",
            "Personal Data Protection Act",
            BuddhistYear::from_be(2562),
        );
        assert_eq!(pdpa.format_th(), "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562");
    }

    #[test]
    fn test_constitution() {
        let constitution = ThaiConstitution::current();
        assert_eq!(constitution.year.be_year, 2560);
    }

    #[test]
    fn test_court_level() {
        let court = ThaiCourtLevel::SupremeCourt;
        assert_eq!(court.name_th(), "ศาลฎีกา");
        assert_eq!(court.name_en(), "Supreme Court");
    }
}
