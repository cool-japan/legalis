//! Capital Market Law (نظام السوق المالية)
//!
//! Royal Decree No. M/30 dated 2/6/1424H (2003)
//!
//! Establishes the Capital Market Authority (CMA - هيئة السوق المالية)
//! and regulates securities and financial markets in Saudi Arabia.

use crate::common::Sar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for CMA operations
pub type CmaResult<T> = Result<T, CmaError>;

/// CMA errors
#[derive(Debug, Error)]
pub enum CmaError {
    /// Invalid listing requirements
    #[error("متطلبات الإدراج غير مستوفاة: {reason}")]
    InvalidListing { reason: String },

    /// Unlicensed activity
    #[error("نشاط غير مرخص: {activity}")]
    UnlicensedActivity { activity: String },

    /// Disclosure violation
    #[error("انتهاك الإفصاح: {description}")]
    DisclosureViolation { description: String },
}

/// Types of CMA licenses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CmaLicense {
    /// Dealing (التعامل)
    Dealing,
    /// Managing (الإدارة)
    Managing,
    /// Arranging (الترتيب)
    Arranging,
    /// Advising (المشورة)
    Advising,
    /// Custody (الحفظ)
    Custody,
}

impl CmaLicense {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Dealing => "التعامل",
            Self::Managing => "الإدارة",
            Self::Arranging => "الترتيب",
            Self::Advising => "المشورة",
            Self::Custody => "الحفظ",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Dealing => "Dealing",
            Self::Managing => "Managing",
            Self::Arranging => "Arranging",
            Self::Advising => "Advising",
            Self::Custody => "Custody",
        }
    }
}

/// Types of securities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityType {
    /// Shares (أسهم)
    Shares,
    /// Debt Instruments (أدوات دين)
    DebtInstruments,
    /// Investment Funds Units (وحدات صناديق استثمار)
    FundUnits,
    /// Sukuk (صكوك)
    Sukuk,
    /// Derivatives (مشتقات)
    Derivatives,
}

impl SecurityType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Shares => "أسهم",
            Self::DebtInstruments => "أدوات دين",
            Self::FundUnits => "وحدات صناديق استثمار",
            Self::Sukuk => "صكوك",
            Self::Derivatives => "مشتقات",
        }
    }
}

/// Types of investors
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestorType {
    /// Qualified investor (مستثمر مؤهل)
    Qualified,
    /// Retail investor (مستثمر عادي)
    Retail,
    /// Institutional investor (مستثمر مؤسسي)
    Institutional,
}

impl InvestorType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Qualified => "مستثمر مؤهل",
            Self::Retail => "مستثمر عادي",
            Self::Institutional => "مستثمر مؤسسي",
        }
    }

    /// Check if can invest in private placements
    pub fn can_invest_private_placement(&self) -> bool {
        matches!(self, Self::Qualified | Self::Institutional)
    }
}

/// Listing requirements for Tadawul
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingRequirements {
    /// Minimum market capitalization
    pub minimum_market_cap: Sar,
    /// Minimum public float percentage
    pub minimum_public_float_pct: f64,
    /// Minimum number of shareholders
    pub minimum_shareholders: u32,
    /// Track record years required
    pub track_record_years: u32,
}

impl ListingRequirements {
    /// Main Market listing requirements
    pub fn main_market() -> Self {
        Self {
            minimum_market_cap: Sar::from_riyals(300_000_000), // 300 million SAR
            minimum_public_float_pct: 30.0,
            minimum_shareholders: 200,
            track_record_years: 3,
        }
    }

    /// Nomu (Parallel Market) listing requirements
    pub fn nomu_market() -> Self {
        Self {
            minimum_market_cap: Sar::from_riyals(10_000_000), // 10 million SAR
            minimum_public_float_pct: 20.0,
            minimum_shareholders: 50,
            track_record_years: 2,
        }
    }
}

/// Validate listing eligibility
pub fn validate_listing(
    market_cap: Sar,
    public_float_pct: f64,
    shareholders: u32,
    track_record: u32,
    is_main_market: bool,
) -> CmaResult<()> {
    let requirements = if is_main_market {
        ListingRequirements::main_market()
    } else {
        ListingRequirements::nomu_market()
    };

    if market_cap < requirements.minimum_market_cap {
        return Err(CmaError::InvalidListing {
            reason: format!(
                "Market cap {} below minimum {}",
                market_cap, requirements.minimum_market_cap
            ),
        });
    }

    if public_float_pct < requirements.minimum_public_float_pct {
        return Err(CmaError::InvalidListing {
            reason: format!(
                "Public float {}% below minimum {}%",
                public_float_pct, requirements.minimum_public_float_pct
            ),
        });
    }

    if shareholders < requirements.minimum_shareholders {
        return Err(CmaError::InvalidListing {
            reason: format!(
                "Shareholders {} below minimum {}",
                shareholders, requirements.minimum_shareholders
            ),
        });
    }

    if track_record < requirements.track_record_years {
        return Err(CmaError::InvalidListing {
            reason: format!(
                "Track record {} years below minimum {} years",
                track_record, requirements.track_record_years
            ),
        });
    }

    Ok(())
}

/// Get CMA compliance checklist
pub fn get_cma_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("التسجيل في هيئة السوق المالية", "CMA registration"),
        ("الحصول على الترخيص", "Obtain appropriate license"),
        ("متطلبات رأس المال", "Capital requirements"),
        ("الإفصاح والشفافية", "Disclosure and transparency"),
        ("حوكمة الشركات", "Corporate governance"),
        ("مكافحة غسل الأموال", "Anti-money laundering (AML)"),
        ("حماية المستثمرين", "Investor protection"),
        ("التقارير الدورية", "Periodic reporting"),
        ("الالتزام بمعايير السوق", "Market conduct standards"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cma_licenses() {
        assert_eq!(CmaLicense::Dealing.name_ar(), "التعامل");
        assert_eq!(CmaLicense::Managing.name_en(), "Managing");
    }

    #[test]
    fn test_investor_types() {
        assert!(InvestorType::Qualified.can_invest_private_placement());
        assert!(InvestorType::Institutional.can_invest_private_placement());
        assert!(!InvestorType::Retail.can_invest_private_placement());
    }

    #[test]
    fn test_main_market_requirements() {
        let req = ListingRequirements::main_market();
        assert_eq!(req.minimum_market_cap.riyals(), 300_000_000);
        assert_eq!(req.minimum_public_float_pct, 30.0);
        assert_eq!(req.minimum_shareholders, 200);
    }

    #[test]
    fn test_nomu_market_requirements() {
        let req = ListingRequirements::nomu_market();
        assert_eq!(req.minimum_market_cap.riyals(), 10_000_000);
        assert_eq!(req.track_record_years, 2);
    }

    #[test]
    fn test_valid_listing() {
        let result = validate_listing(
            Sar::from_riyals(500_000_000),
            35.0,
            250,
            3,
            true, // Main market
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_listing_low_market_cap() {
        let result = validate_listing(
            Sar::from_riyals(100_000_000), // Too low
            35.0,
            250,
            3,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_checklist() {
        let checklist = get_cma_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 9);
    }
}
