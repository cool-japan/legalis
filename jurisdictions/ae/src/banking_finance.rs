//! UAE Banking and Finance Law
//!
//! Governing the UAE banking sector and financial institutions.
//!
//! ## Key Laws
//!
//! - **Federal Decree-Law No. 14/2018** - Central Bank and Organization of Financial Institutions
//! - **Federal Law No. 6/1985** - Islamic Banks, Financial Institutions and Investment Companies
//! - **Federal Decree-Law No. 20/2018** - Anti-Money Laundering and Combating Terrorist Financing
//!
//! ## Central Bank of UAE (CBUAE)
//!
//! The CBUAE is the principal banking regulator, supervising:
//! - Commercial banks
//! - Islamic banks
//! - Exchange houses
//! - Finance companies

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for banking operations
pub type BankingResult<T> = Result<T, BankingError>;

/// Types of financial institutions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialInstitution {
    /// Commercial bank (بنك تجاري)
    CommercialBank,
    /// Islamic bank (بنك إسلامي)
    IslamicBank,
    /// Investment bank (بنك استثماري)
    InvestmentBank,
    /// Finance company (شركة تمويل)
    FinanceCompany,
    /// Exchange house (صرافة)
    ExchangeHouse,
    /// Insurance company (شركة تأمين)
    Insurance,
}

impl FinancialInstitution {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::CommercialBank => "بنك تجاري",
            Self::IslamicBank => "بنك إسلامي",
            Self::InvestmentBank => "بنك استثماري",
            Self::FinanceCompany => "شركة تمويل",
            Self::ExchangeHouse => "صرافة",
            Self::Insurance => "شركة تأمين",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::CommercialBank => "Commercial Bank",
            Self::IslamicBank => "Islamic Bank",
            Self::InvestmentBank => "Investment Bank",
            Self::FinanceCompany => "Finance Company",
            Self::ExchangeHouse => "Exchange House",
            Self::Insurance => "Insurance Company",
        }
    }

    /// Minimum capital requirement (AED)
    pub fn minimum_capital(&self) -> Aed {
        match self {
            Self::CommercialBank => Aed::from_dirhams(1_000_000_000), // AED 1 billion
            Self::IslamicBank => Aed::from_dirhams(1_000_000_000),
            Self::InvestmentBank => Aed::from_dirhams(500_000_000), // AED 500 million
            Self::FinanceCompany => Aed::from_dirhams(30_000_000),  // AED 30 million
            Self::ExchangeHouse => Aed::from_dirhams(5_000_000),    // AED 5 million
            Self::Insurance => Aed::from_dirhams(50_000_000),       // AED 50 million
        }
    }

    /// Check if CBUAE license required
    pub fn requires_cbuae_license(&self) -> bool {
        !matches!(self, Self::Insurance)
    }
}

/// Banking regulatory requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryCompliance {
    /// Capital adequacy ratio (minimum 13%)
    pub capital_adequacy_ratio: f64,
    /// Liquidity coverage ratio
    pub liquidity_coverage_ratio: f64,
    /// Has AML/CFT program
    pub has_aml_program: bool,
    /// Has compliance officer
    pub has_compliance_officer: bool,
    /// Submits regulatory returns
    pub submits_returns: bool,
}

impl RegulatoryCompliance {
    /// Minimum capital adequacy ratio
    pub const MIN_CAR: f64 = 13.0;

    /// Minimum liquidity coverage ratio
    pub const MIN_LCR: f64 = 100.0;

    /// Check if compliant with CBUAE requirements
    pub fn is_compliant(&self) -> BankingResult<()> {
        if self.capital_adequacy_ratio < Self::MIN_CAR {
            return Err(BankingError::CapitalAdequacyBreach {
                ratio: self.capital_adequacy_ratio,
                minimum: Self::MIN_CAR,
            });
        }

        if self.liquidity_coverage_ratio < Self::MIN_LCR {
            return Err(BankingError::LiquidityBreach {
                ratio: self.liquidity_coverage_ratio,
                minimum: Self::MIN_LCR,
            });
        }

        if !self.has_aml_program {
            return Err(BankingError::AmlProgramRequired);
        }

        if !self.has_compliance_officer {
            return Err(BankingError::ComplianceOfficerRequired);
        }

        Ok(())
    }
}

/// Anti-Money Laundering (AML) requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlCompliance {
    /// Customer Due Diligence (CDD) performed
    pub cdd_performed: bool,
    /// Enhanced Due Diligence (EDD) for high-risk
    pub edd_for_high_risk: bool,
    /// Transaction monitoring
    pub transaction_monitoring: bool,
    /// Suspicious Activity Reports (SAR) filed
    pub sar_filing: bool,
    /// MLRO appointed (Money Laundering Reporting Officer)
    pub mlro_appointed: bool,
}

impl AmlCompliance {
    /// Check if AML compliant
    pub fn is_compliant(&self) -> BankingResult<()> {
        if !self.mlro_appointed {
            return Err(BankingError::MlroRequired);
        }

        if !self.cdd_performed {
            return Err(BankingError::CddRequired);
        }

        if !self.transaction_monitoring {
            return Err(BankingError::TransactionMonitoringRequired);
        }

        Ok(())
    }
}

/// Consumer protection regulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerProtection {
    /// Interest rate cap (if applicable)
    pub interest_rate_cap: Option<f64>,
    /// Cooling-off period (days)
    pub cooling_off_days: u32,
    /// Disclosure requirements met
    pub disclosures_met: bool,
    /// Fair treatment of customers
    pub fair_treatment: bool,
}

impl ConsumerProtection {
    /// CBUAE standard cooling-off period
    pub const STANDARD_COOLING_OFF: u32 = 5;

    /// Check consumer protection compliance
    pub fn is_compliant(&self) -> BankingResult<()> {
        if self.cooling_off_days < Self::STANDARD_COOLING_OFF {
            return Err(BankingError::InsufficientCoolingOff {
                provided: self.cooling_off_days,
                required: Self::STANDARD_COOLING_OFF,
            });
        }

        if !self.disclosures_met {
            return Err(BankingError::DisclosureRequired);
        }

        Ok(())
    }
}

/// Banking errors
#[derive(Debug, Error)]
pub enum BankingError {
    /// Capital adequacy breach
    #[error("نسبة كفاية رأس المال أقل من الحد الأدنى: {ratio}% (المطلوب {minimum}%)")]
    CapitalAdequacyBreach { ratio: f64, minimum: f64 },

    /// Liquidity breach
    #[error("نسبة تغطية السيولة أقل من الحد الأدنى: {ratio}% (المطلوب {minimum}%)")]
    LiquidityBreach { ratio: f64, minimum: f64 },

    /// AML program required
    #[error("يجب تطبيق برنامج مكافحة غسل الأموال")]
    AmlProgramRequired,

    /// Compliance officer required
    #[error("يجب تعيين مسؤول امتثال")]
    ComplianceOfficerRequired,

    /// MLRO required
    #[error("يجب تعيين مسؤول الإبلاغ عن غسل الأموال (MLRO)")]
    MlroRequired,

    /// CDD required
    #[error("يجب إجراء العناية الواجبة للعملاء (CDD)")]
    CddRequired,

    /// Transaction monitoring required
    #[error("يجب مراقبة المعاملات")]
    TransactionMonitoringRequired,

    /// Insufficient cooling-off period
    #[error("فترة التراجع غير كافية: {provided} يوم (المطلوب {required})")]
    InsufficientCoolingOff { provided: u32, required: u32 },

    /// Disclosure required
    #[error("يجب الإفصاح عن المعلومات للعملاء")]
    DisclosureRequired,
}

/// Get banking compliance checklist
pub fn get_banking_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ترخيص المصرف المركزي", "CBUAE License"),
        ("رأس المال الأدنى", "Minimum capital requirement"),
        ("نسبة كفاية رأس المال", "Capital adequacy ratio (13%)"),
        ("نسبة تغطية السيولة", "Liquidity coverage ratio"),
        ("برنامج مكافحة غسل الأموال", "AML/CFT program"),
        ("مسؤول الامتثال", "Compliance officer"),
        ("العناية الواجبة للعملاء", "Customer Due Diligence"),
        ("مراقبة المعاملات", "Transaction monitoring"),
        ("الإبلاغ عن الأنشطة المشبوهة", "Suspicious Activity Reporting"),
        ("حماية المستهلك", "Consumer protection"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_institutions() {
        let bank = FinancialInstitution::CommercialBank;
        assert_eq!(bank.name_ar(), "بنك تجاري");
        assert!(bank.requires_cbuae_license());
        assert_eq!(bank.minimum_capital().dirhams(), 1_000_000_000);

        let islamic = FinancialInstitution::IslamicBank;
        assert_eq!(islamic.name_en(), "Islamic Bank");
    }

    #[test]
    fn test_regulatory_compliance_valid() {
        let compliance = RegulatoryCompliance {
            capital_adequacy_ratio: 15.0,
            liquidity_coverage_ratio: 120.0,
            has_aml_program: true,
            has_compliance_officer: true,
            submits_returns: true,
        };

        assert!(compliance.is_compliant().is_ok());
    }

    #[test]
    fn test_regulatory_compliance_car_breach() {
        let compliance = RegulatoryCompliance {
            capital_adequacy_ratio: 10.0, // Below 13%
            liquidity_coverage_ratio: 120.0,
            has_aml_program: true,
            has_compliance_officer: true,
            submits_returns: true,
        };

        assert!(compliance.is_compliant().is_err());
    }

    #[test]
    fn test_aml_compliance() {
        let aml = AmlCompliance {
            cdd_performed: true,
            edd_for_high_risk: true,
            transaction_monitoring: true,
            sar_filing: true,
            mlro_appointed: true,
        };

        assert!(aml.is_compliant().is_ok());
    }

    #[test]
    fn test_aml_compliance_mlro_missing() {
        let aml = AmlCompliance {
            cdd_performed: true,
            edd_for_high_risk: true,
            transaction_monitoring: true,
            sar_filing: true,
            mlro_appointed: false,
        };

        assert!(aml.is_compliant().is_err());
    }

    #[test]
    fn test_consumer_protection() {
        let protection = ConsumerProtection {
            interest_rate_cap: Some(15.0),
            cooling_off_days: 5,
            disclosures_met: true,
            fair_treatment: true,
        };

        assert!(protection.is_compliant().is_ok());
    }

    #[test]
    fn test_consumer_protection_insufficient_cooling_off() {
        let protection = ConsumerProtection {
            interest_rate_cap: None,
            cooling_off_days: 2,
            disclosures_met: true,
            fair_treatment: true,
        };

        assert!(protection.is_compliant().is_err());
    }

    #[test]
    fn test_banking_checklist() {
        let checklist = get_banking_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
