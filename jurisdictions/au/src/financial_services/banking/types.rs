//! Banking Types (APRA Prudential Standards)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Authorised Deposit-taking Institution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedDepositInstitution {
    /// ADI name
    pub name: String,
    /// ABN
    pub abn: String,
    /// APRA registration number
    pub apra_registration: String,
    /// ADI category
    pub category: AdiCategory,
    /// Status
    pub status: AdiStatus,
    /// Authorization date
    pub authorization_date: NaiveDate,
    /// Capital adequacy
    pub capital: CapitalRequirement,
    /// Liquidity
    pub liquidity: LiquidityRequirement,
}

/// ADI categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdiCategory {
    /// Major bank (Big Four)
    MajorBank,
    /// Other domestic bank
    OtherDomesticBank,
    /// Foreign subsidiary bank
    ForeignSubsidiary,
    /// Foreign branch
    ForeignBranch,
    /// Building society
    BuildingSociety,
    /// Credit union
    CreditUnion,
    /// Restricted ADI
    RestrictedAdi,
}

/// ADI status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdiStatus {
    /// Active authorization
    Authorized,
    /// Under supervision
    UnderSupervision,
    /// Authorization suspended
    Suspended,
    /// Authorization revoked
    Revoked,
}

/// Capital requirement (APS 110)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapitalRequirement {
    /// Common Equity Tier 1 (CET1) ratio (%)
    pub cet1_ratio: f64,
    /// Tier 1 capital ratio (%)
    pub tier1_ratio: f64,
    /// Total capital ratio (%)
    pub total_capital_ratio: f64,
    /// Risk-weighted assets (AUD millions)
    pub rwa_aud_millions: f64,
    /// Capital conservation buffer (%)
    pub conservation_buffer: f64,
    /// D-SIB buffer (if applicable) (%)
    pub dsib_buffer: Option<f64>,
    /// Countercyclical buffer (%)
    pub countercyclical_buffer: f64,
    /// Meets minimum requirements
    pub meets_requirements: bool,
}

impl CapitalRequirement {
    /// Get minimum CET1 requirement (including buffers)
    pub fn minimum_cet1(&self) -> f64 {
        let mut min = 4.5; // Base CET1
        min += self.conservation_buffer;
        min += self.countercyclical_buffer;
        if let Some(dsib) = self.dsib_buffer {
            min += dsib;
        }
        min
    }

    /// Check if CET1 ratio meets requirement
    pub fn cet1_adequate(&self) -> bool {
        self.cet1_ratio >= self.minimum_cet1()
    }

    /// Get minimum Tier 1 requirement
    pub fn minimum_tier1(&self) -> f64 {
        6.0 + self.conservation_buffer
            + self.countercyclical_buffer
            + self.dsib_buffer.unwrap_or(0.0)
    }

    /// Get minimum total capital requirement
    pub fn minimum_total(&self) -> f64 {
        8.0 + self.conservation_buffer
            + self.countercyclical_buffer
            + self.dsib_buffer.unwrap_or(0.0)
    }
}

/// Liquidity requirement (APS 210)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidityRequirement {
    /// Liquidity Coverage Ratio (%)
    pub lcr: f64,
    /// Net Stable Funding Ratio (%)
    pub nsfr: f64,
    /// High Quality Liquid Assets (AUD millions)
    pub hqla_aud_millions: f64,
    /// Total net cash outflows (AUD millions)
    pub net_cash_outflows_aud_millions: f64,
    /// Meets LCR requirement (100%+)
    pub meets_lcr: bool,
    /// Meets NSFR requirement (100%+)
    pub meets_nsfr: bool,
}

impl LiquidityRequirement {
    /// Check if LCR meets minimum (100%)
    pub fn lcr_adequate(&self) -> bool {
        self.lcr >= 100.0
    }

    /// Check if NSFR meets minimum (100%)
    pub fn nsfr_adequate(&self) -> bool {
        self.nsfr >= 100.0
    }
}

/// APRA Prudential Standards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrudentialStandard {
    /// APS 110 Capital Adequacy
    Aps110CapitalAdequacy,
    /// APS 210 Liquidity
    Aps210Liquidity,
    /// APS 220 Credit Risk Management
    Aps220CreditRisk,
    /// APS 310 Audit
    Aps310Audit,
    /// APS 330 Public Disclosure
    Aps330PublicDisclosure,
    /// APS 510 Governance
    Aps510Governance,
    /// APS 520 Fit and Proper
    Aps520FitAndProper,
}

impl PrudentialStandard {
    /// Get standard reference
    pub fn reference(&self) -> &'static str {
        match self {
            PrudentialStandard::Aps110CapitalAdequacy => "APS 110",
            PrudentialStandard::Aps210Liquidity => "APS 210",
            PrudentialStandard::Aps220CreditRisk => "APS 220",
            PrudentialStandard::Aps310Audit => "APS 310",
            PrudentialStandard::Aps330PublicDisclosure => "APS 330",
            PrudentialStandard::Aps510Governance => "APS 510",
            PrudentialStandard::Aps520FitAndProper => "APS 520",
        }
    }
}

/// APRA requirement compliance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApraRequirement {
    /// Standard
    pub standard: PrudentialStandard,
    /// Compliant
    pub compliant: bool,
    /// Compliance details
    pub details: String,
    /// Last assessment date
    pub assessment_date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capital_requirement_minimum() {
        let capital = CapitalRequirement {
            cet1_ratio: 12.0,
            tier1_ratio: 14.0,
            total_capital_ratio: 16.0,
            rwa_aud_millions: 100_000.0,
            conservation_buffer: 2.5,
            dsib_buffer: Some(1.0), // D-SIB
            countercyclical_buffer: 0.0,
            meets_requirements: true,
        };

        // Base 4.5 + 2.5 buffer + 1.0 D-SIB = 8.0%
        assert_eq!(capital.minimum_cet1(), 8.0);
        assert!(capital.cet1_adequate());
    }

    #[test]
    fn test_liquidity_requirement() {
        let liquidity = LiquidityRequirement {
            lcr: 125.0,
            nsfr: 110.0,
            hqla_aud_millions: 50_000.0,
            net_cash_outflows_aud_millions: 40_000.0,
            meets_lcr: true,
            meets_nsfr: true,
        };

        assert!(liquidity.lcr_adequate());
        assert!(liquidity.nsfr_adequate());
    }
}
