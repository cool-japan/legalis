//! # Legalis-VN: Vietnam Jurisdiction Support
//!
//! Comprehensive Vietnamese legal system implementation for Legalis-RS.
//!
//! ## Legal System Overview
//!
//! Vietnam uses a **Socialist Civil Law** system with market economy characteristics
//! (Đổi Mới - economic renovation). Key features:
//!
//! - Single-party state (Communist Party of Vietnam)
//! - State ownership of all land (land use rights only)
//! - Mandatory trade union involvement
//! - French civil law influence
//!
//! ## Hierarchy of Laws
//!
//! 1. **Hiến pháp** - Constitution (2013)
//! 2. **Luật / Bộ luật** - Laws/Codes (passed by National Assembly)
//! 3. **Pháp lệnh** - Ordinances (Standing Committee)
//! 4. **Nghị định** - Decrees (Government)
//! 5. **Thông tư** - Circulars (Ministries)
//! 6. **Quyết định** - Decisions
//!
//! ## Modules
//!
//! ### [`citation`] - Vietnamese Legal Citation System
//!
//! Supports all Vietnamese legal instrument types:
//! - `Luật số [num]/[year]/QH[session]` - Laws
//! - `Nghị định số [num]/[year]/NĐ-CP` - Decrees
//! - `Thông tư số [num]/[year]/TT-[ministry]` - Circulars
//!
//! ### [`common`] - Shared Utilities
//!
//! - Vietnamese public holidays (Tết, national days)
//! - VND currency formatting
//! - Wage regions (Vùng I-IV)
//! - Province information
//!
//! ### [`labor_code`] - Bộ luật Lao động 2019
//!
//! Vietnam's Labor Code (Law No. 45/2019/QH14):
//! - 48 hours/week maximum (Article 105)
//! - Contract types: indefinite, fixed-term, seasonal
//! - Severance: 0.5 months per year (Article 46)
//! - Social insurance mandatory (BHXH, BHYT, BHTN)
//!
//! ### [`enterprise`] - Luật Doanh nghiệp 2020
//!
//! Enterprise Law (Law No. 59/2020/QH14):
//! - Enterprise types (LLC, JSC, Partnership)
//! - Registration requirements
//! - Corporate governance
//!
//! ### [`investment`] - Luật Đầu tư 2020
//!
//! Investment Law (Law No. 61/2020/QH14):
//! - Conditional investment sectors
//! - Foreign investment rules
//! - Investment incentives (tax, land)
//! - Special Economic Zones
//!
//! ## Key Legal Concepts
//!
//! ### State Land Ownership
//!
//! All land in Vietnam is owned by the state. Investors obtain:
//! - Land Use Rights (Quyền sử dụng đất) - transferable
//! - Red Book (Sổ đỏ) - residential land use certificate
//! - Pink Book (Sổ hồng) - apartment ownership certificate
//!
//! ### Social Insurance (Bảo hiểm xã hội)
//!
//! Mandatory contributions:
//! - BHXH (Social Insurance): 25.5% total
//! - BHYT (Health Insurance): 4.5% total
//! - BHTN (Unemployment): 2% total
//!
//! ## Quick Start
//!
//! ```rust
//! use legalis_vn::{
//!     citation::{VietnameseCitation, common_citations},
//!     labor_code::{validate_contract, EmploymentContract, WorkingHours, ContractType},
//!     enterprise::{EnterpriseType, EnterpriseRegistration, validate_registration},
//!     common::{Vnd, WageRegion, Province},
//! };
//!
//! // Citation example
//! let citation = VietnameseCitation::luat(45, 2019, 14)
//!     .with_title_vi("Lao động")
//!     .with_article(105);
//! println!("{}", citation); // Luật Lao động số 45/2019/QH14, Điều 105
//!
//! // Currency formatting
//! let salary = Vnd::from_trieu(10);
//! println!("{}", salary); // 10.000.000 đ
//!
//! // Minimum wage by region
//! let min_wage = WageRegion::Region1.minimum_wage_2024();
//! println!("Vùng I: {}", min_wage);
//! ```
//!
//! ## Major Laws Covered
//!
//! | Law | Name (VI) | Name (EN) |
//! |-----|-----------|-----------|
//! | 45/2019/QH14 | Bộ luật Lao động | Labor Code |
//! | 59/2020/QH14 | Luật Doanh nghiệp | Enterprise Law |
//! | 61/2020/QH14 | Luật Đầu tư | Investment Law |
//! | 91/2015/QH13 | Bộ luật Dân sự | Civil Code |
//! | 45/2013/QH13 | Luật Đất đai | Land Law |
//!
//! ## Disclaimer
//!
//! This library is for educational and informational purposes. For legal matters,
//! consult qualified Vietnamese legal professionals (luật sư).

pub mod citation;
pub mod common;
pub mod enterprise;
pub mod investment;
pub mod labor_code;

// Re-export commonly used items
pub use citation::{
    CodeType, Issuer, LegalInstrumentType, Ministry, VietnameseCitation, common_citations,
};

pub use common::{
    Province, VietnameseHoliday, VietnameseHolidayType, Vnd, WageRegion, get_public_holidays,
    is_public_holiday, is_working_day, working_days_between,
};

pub use labor_code::{
    ContractType, EmploymentContract, LaborCodeError, LaborCodeResult, LaborCompliance, LeaveType,
    Severance, SocialInsurance, TerminationType, WorkingHours, calculate_severance,
    calculate_social_insurance, get_labor_checklist, validate_contract, validate_labor_compliance,
    validate_minimum_wage, validate_working_hours,
};

pub use enterprise::{
    EnterpriseError, EnterpriseRegistration, EnterpriseResult, EnterpriseType,
    get_enterprise_checklist, validate_registration,
};

pub use investment::{
    InvestmentError, InvestmentIncentive, InvestmentProject, InvestmentResult, InvestmentSector,
    SpecialEconomicZone, check_sector_eligibility, get_investment_checklist,
    validate_investment_project,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_display() {
        let citation = VietnameseCitation::luat(45, 2019, 14)
            .with_title_vi("Lao động")
            .with_article(105);

        assert!(citation.to_string().contains("45/2019/QH14"));
        assert!(citation.to_string().contains("Điều 105"));
    }

    #[test]
    fn test_vnd_formatting() {
        let amount = Vnd::from_trieu(10);
        assert_eq!(amount.format_vi(), "10.000.000 đ");
        assert_eq!(amount.format_trieu(), "10 triệu đồng");
    }

    #[test]
    fn test_wage_region() {
        let region1 = WageRegion::Region1;
        assert!(region1.minimum_wage_2024().amount() > 4_000_000);
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 48);
    }

    #[test]
    fn test_severance_calculation() {
        let severance = calculate_severance(10, 10_000_000, TerminationType::ContractExpiry);
        assert_eq!(severance.severance_months, 5.0);
        assert_eq!(severance.total_amount, 50_000_000);
    }

    #[test]
    fn test_social_insurance() {
        let insurance = calculate_social_insurance(10_000_000);
        assert!(insurance.total > 0);
        assert!(insurance.total_employer > insurance.total_employee);
    }

    #[test]
    fn test_enterprise_types() {
        let llc = EnterpriseType::MultiMemberLlc;
        assert!(llc.has_limited_liability());
        assert_eq!(llc.minimum_members(), Some(2));
    }

    #[test]
    fn test_common_citations() {
        let labor = common_citations::labor_code_2019();
        assert_eq!(labor.year, 2019);

        let enterprise = common_citations::enterprise_law_2020();
        assert_eq!(enterprise.number, 59);
    }

    #[test]
    fn test_public_holidays() {
        let holidays = get_public_holidays(2024);
        assert!(!holidays.is_empty());

        // Check National Day
        let national_day = chrono::NaiveDate::from_ymd_opt(2024, 9, 2).unwrap();
        assert!(is_public_holiday(national_day));
    }
}
