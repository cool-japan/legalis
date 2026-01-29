//! Cryptoassets Types (FSMA 2023, FCA PS19/22, MLR 2017 Reg 14A)

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Cryptoasset classification (FCA PS19/22)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CryptoassetClassification {
    /// Security Token (specified investment under RAO 2001)
    /// Full FCA authorization required (FSMA 2000 s.19)
    SecurityToken {
        /// Type of security
        investment_type: SecurityTokenType,

        /// Whether prospectus required (public offer)
        prospectus_required: bool,
    },

    /// E-Money Token (meets e-money definition, EMR 2011)
    /// E-money institution authorization required
    EMoneyToken {
        /// Whether issuer authorized as EMI
        issuer_authorized: bool,

        /// Whether redeemable at par (1:1 with fiat)
        redeemable_at_par: bool,
    },

    /// Utility Token (unregulated, provides access to goods/services)
    /// AML regulation applies if exchange/custodian (MLR 2017 Reg 14A)
    UtilityToken {
        /// Description of utility provided
        utility_description: String,
    },

    /// Stablecoin (may be regulated as e-money or payment system)
    /// FSMA 2023 Part 5 stablecoin regime
    Stablecoin {
        /// Peg mechanism
        peg: StablecoinPeg,

        /// Reserve backing ratio
        reserve_backing_ratio: f64,
    },
}

/// Security token type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SecurityTokenType {
    /// Tokenized equity (share token)
    Equity,

    /// Tokenized debt (bond token)
    Debt,

    /// Tokenized fund units
    CollectiveInvestmentScheme,

    /// Derivative token
    Derivative,
}

/// Stablecoin peg mechanism
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StablecoinPeg {
    /// Fiat currency peg (e.g., GBP, USD)
    FiatCurrency {
        /// Currency code (ISO 4217)
        currency: String,
    },

    /// Commodity peg (e.g., gold)
    Commodity {
        /// Commodity name
        commodity: String,
    },

    /// Algorithmic stabilization (no collateral)
    Algorithmic,
}

/// Stablecoin (FSMA 2023 Part 5)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stablecoin {
    /// Token name
    pub token_name: String,

    /// Issuer name
    pub issuer: String,

    /// Peg mechanism
    pub peg: StablecoinPeg,

    /// Reserve backing details
    pub reserve_backing: ReserveBacking,

    /// Whether reserve audited
    pub reserve_audited: bool,

    /// Audit date
    pub audit_date: Option<NaiveDate>,

    /// Redemption rights (FSMA 2023 requirement)
    pub redemption_rights: RedemptionRights,

    /// FCA authorization status
    pub fca_authorized: bool,

    /// Authorization type if authorized
    pub authorization_type: Option<String>,
}

impl Stablecoin {
    /// Check if stablecoin is fully reserved (1:1 backing, FSMA 2023)
    pub fn is_fully_reserved(&self) -> bool {
        self.reserve_backing.backing_ratio >= 1.0
    }

    /// Check if compliant with FSMA 2023 stablecoin requirements
    pub fn is_fsma_2023_compliant(&self) -> bool {
        self.fca_authorized
            && self.is_fully_reserved()
            && self.reserve_backing.segregated
            && self.redemption_rights.redeemable
            && self.reserve_audited
    }

    /// Check if algorithmic (high risk)
    pub fn is_algorithmic(&self) -> bool {
        matches!(self.peg, StablecoinPeg::Algorithmic)
    }
}

/// Reserve backing details
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReserveBacking {
    /// Backing ratio (should be >= 1.0 for 100%+ backing)
    pub backing_ratio: f64,

    /// Description of reserve assets
    pub assets_description: String,

    /// Custody arrangements
    pub custody_arrangements: String,

    /// Whether reserves segregated from issuer's assets
    pub segregated: bool,
}

/// Redemption rights
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RedemptionRights {
    /// Whether token redeemable for fiat/asset
    pub redeemable: bool,

    /// Redemption fee (if any)
    pub redemption_fee: Option<f64>,

    /// Redemption timeframe
    pub redemption_timeframe: Option<String>,
}

/// Cryptoasset financial promotion (FSMA 2000 s.21, effective Oct 8, 2023)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CryptoassetPromotion {
    /// Promotion content
    pub promotion_content: String,

    /// Promotion date
    pub promotion_date: NaiveDate,

    /// Medium (social media, website, email, etc.)
    pub medium: PromotionMedium,

    /// Whether approved by FCA-authorized person (required from Oct 8, 2023)
    pub approved_by_authorized_person: bool,

    /// Approver's FRN if approved
    pub approver_frn: Option<String>,

    /// Whether risk warning included
    pub risk_warning_included: bool,

    /// Whether risk warning prominent (clear, fair, not misleading)
    pub risk_warning_prominent: bool,

    /// Fair, clear and not misleading (COBS 4.2.1R)
    pub fair_clear_not_misleading: bool,

    /// Target audience
    pub target_audience: PromotionAudience,
}

impl CryptoassetPromotion {
    /// Check if compliant with FSMA s.21 financial promotions regime (post Oct 8, 2023)
    pub fn is_compliant(&self) -> bool {
        // From Oct 8, 2023, cryptoasset promotions must be approved
        let approval_compliant = if self.promotion_date
            >= NaiveDate::from_ymd_opt(2023, 10, 8).expect("valid date constant: October 8, 2023")
        {
            self.approved_by_authorized_person && self.approver_frn.is_some()
        } else {
            true // Before Oct 8, 2023, approval not required
        };

        approval_compliant
            && self.risk_warning_included
            && self.risk_warning_prominent
            && self.fair_clear_not_misleading
    }
}

/// Promotion medium
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PromotionMedium {
    /// Social media (Twitter, Facebook, Instagram, etc.)
    SocialMedia,

    /// Website
    Website,

    /// Email
    Email,

    /// Television
    Television,

    /// Radio
    Radio,

    /// Print (newspaper, magazine)
    Print,

    /// Outdoor advertising (billboard, bus, etc.)
    Outdoor,

    /// Other medium type
    Other {
        /// Description of medium
        description: String,
    },
}

/// Promotion target audience
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PromotionAudience {
    /// General public (retail)
    Retail,

    /// Professional investors
    Professional,

    /// Certified high net worth individuals
    HighNetWorth,

    /// Sophisticated investors
    Sophisticated,

    /// Restricted (specific group)
    Restricted,
}

/// Cryptoasset exchange provider (MLR 2017 Reg 14A)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CryptoassetExchangeProvider {
    /// Provider name
    pub provider_name: String,

    /// FCA registered under MLR 2017 Reg 14A
    pub fca_registered: bool,

    /// Registration number
    pub registration_number: Option<String>,

    /// Registration date
    pub registration_date: Option<NaiveDate>,

    /// AML/CTF policies in place
    pub aml_policies: bool,

    /// Customer Due Diligence performed
    pub cdd_performed: bool,

    /// SAR procedures in place
    pub sar_procedures: bool,

    /// Travel Rule compliance (MLR 2017 Reg 14A - transfers ≥£1,000)
    pub travel_rule_compliant: bool,

    /// Sanctions screening performed
    pub sanctions_screening: bool,
}

impl CryptoassetExchangeProvider {
    /// Check if MLR 2017 Reg 14A compliant
    pub fn is_mlr_compliant(&self) -> bool {
        self.fca_registered
            && self.registration_number.is_some()
            && self.aml_policies
            && self.cdd_performed
            && self.sar_procedures
            && self.travel_rule_compliant
            && self.sanctions_screening
    }
}

/// Security token assessment (Howey Test adapted)
///
/// FCA PS19/22 uses a modified Howey Test to determine if token is security:
/// 1. Investment of money
/// 2. Common enterprise
/// 3. Expectation of profit
/// 4. From efforts of others
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecurityTokenAssessment {
    /// Token name
    pub token_name: String,

    /// Whether involves investment of money
    pub investment_of_money: bool,

    /// Whether involves common enterprise (pooled funds, shared profits)
    pub common_enterprise: bool,

    /// Whether expectation of profit
    pub expectation_of_profit: bool,

    /// Whether profit from efforts of others (not token holder's own efforts)
    pub profit_from_others_efforts: bool,

    /// Whether provides equity rights (voting, dividends)
    pub equity_rights: bool,

    /// Whether provides debt rights (interest, principal repayment)
    pub debt_rights: bool,

    /// Whether derivative characteristics
    pub derivative_characteristics: bool,
}

impl SecurityTokenAssessment {
    /// Check if token meets Howey Test (likely a security)
    pub fn is_likely_security(&self) -> bool {
        // All four Howey Test elements must be present
        let howey_test = self.investment_of_money
            && self.common_enterprise
            && self.expectation_of_profit
            && self.profit_from_others_efforts;

        // Or token has explicit equity/debt/derivative rights
        let explicit_security_rights =
            self.equity_rights || self.debt_rights || self.derivative_characteristics;

        howey_test || explicit_security_rights
    }

    /// Get classification recommendation
    pub fn recommended_classification(&self) -> CryptoassetClassification {
        if self.is_likely_security() {
            let investment_type = if self.equity_rights {
                SecurityTokenType::Equity
            } else if self.debt_rights {
                SecurityTokenType::Debt
            } else if self.derivative_characteristics {
                SecurityTokenType::Derivative
            } else {
                SecurityTokenType::CollectiveInvestmentScheme
            };

            CryptoassetClassification::SecurityToken {
                investment_type,
                prospectus_required: true, // Public offer requires prospectus
            }
        } else {
            // If not security, likely utility token
            CryptoassetClassification::UtilityToken {
                utility_description: "Token provides access to goods/services".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stablecoin_fully_reserved() {
        let stablecoin = Stablecoin {
            token_name: "GBPT".to_string(),
            issuer: "UK Stablecoin Ltd".to_string(),
            peg: StablecoinPeg::FiatCurrency {
                currency: "GBP".to_string(),
            },
            reserve_backing: ReserveBacking {
                backing_ratio: 1.0,
                assets_description: "Bank deposits".to_string(),
                custody_arrangements: "UK bank".to_string(),
                segregated: true,
            },
            reserve_audited: true,
            audit_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            redemption_rights: RedemptionRights {
                redeemable: true,
                redemption_fee: None,
                redemption_timeframe: Some("1 business day".to_string()),
            },
            fca_authorized: true,
            authorization_type: Some("E-Money Institution".to_string()),
        };

        assert!(stablecoin.is_fully_reserved());
        assert!(stablecoin.is_fsma_2023_compliant());
        assert!(!stablecoin.is_algorithmic());
    }

    #[test]
    fn test_stablecoin_algorithmic() {
        let stablecoin = Stablecoin {
            token_name: "AlgoStable".to_string(),
            issuer: "Algo Protocol".to_string(),
            peg: StablecoinPeg::Algorithmic,
            reserve_backing: ReserveBacking {
                backing_ratio: 0.0, // No reserves
                assets_description: "Algorithmic".to_string(),
                custody_arrangements: "N/A".to_string(),
                segregated: false,
            },
            reserve_audited: false,
            audit_date: None,
            redemption_rights: RedemptionRights {
                redeemable: false,
                redemption_fee: None,
                redemption_timeframe: None,
            },
            fca_authorized: false,
            authorization_type: None,
        };

        assert!(!stablecoin.is_fully_reserved());
        assert!(!stablecoin.is_fsma_2023_compliant());
        assert!(stablecoin.is_algorithmic());
    }

    #[test]
    fn test_cryptoasset_promotion_compliant_post_oct_2023() {
        let promotion = CryptoassetPromotion {
            promotion_content: "Invest in BTC".to_string(),
            promotion_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            medium: PromotionMedium::SocialMedia,
            approved_by_authorized_person: true,
            approver_frn: Some("123456".to_string()),
            risk_warning_included: true,
            risk_warning_prominent: true,
            fair_clear_not_misleading: true,
            target_audience: PromotionAudience::Retail,
        };

        assert!(promotion.is_compliant());
    }

    #[test]
    fn test_cryptoasset_promotion_non_compliant_no_approval() {
        let promotion = CryptoassetPromotion {
            promotion_content: "Buy crypto now!".to_string(),
            promotion_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            medium: PromotionMedium::SocialMedia,
            approved_by_authorized_person: false, // NOT APPROVED
            approver_frn: None,
            risk_warning_included: true,
            risk_warning_prominent: true,
            fair_clear_not_misleading: true,
            target_audience: PromotionAudience::Retail,
        };

        assert!(!promotion.is_compliant());
    }

    #[test]
    fn test_exchange_provider_mlr_compliant() {
        let exchange = CryptoassetExchangeProvider {
            provider_name: "UK Crypto Exchange Ltd".to_string(),
            fca_registered: true,
            registration_number: Some("12345678".to_string()),
            registration_date: Some(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()),
            aml_policies: true,
            cdd_performed: true,
            sar_procedures: true,
            travel_rule_compliant: true,
            sanctions_screening: true,
        };

        assert!(exchange.is_mlr_compliant());
    }

    #[test]
    fn test_security_token_assessment_howey_test() {
        let assessment = SecurityTokenAssessment {
            token_name: "InvestToken".to_string(),
            investment_of_money: true,
            common_enterprise: true,
            expectation_of_profit: true,
            profit_from_others_efforts: true,
            equity_rights: false,
            debt_rights: false,
            derivative_characteristics: false,
        };

        assert!(assessment.is_likely_security());
        assert!(matches!(
            assessment.recommended_classification(),
            CryptoassetClassification::SecurityToken { .. }
        ));
    }

    #[test]
    fn test_security_token_assessment_equity_rights() {
        let assessment = SecurityTokenAssessment {
            token_name: "ShareToken".to_string(),
            investment_of_money: true,
            common_enterprise: false,
            expectation_of_profit: true,
            profit_from_others_efforts: false,
            equity_rights: true, // Explicit equity rights
            debt_rights: false,
            derivative_characteristics: false,
        };

        assert!(assessment.is_likely_security());

        match assessment.recommended_classification() {
            CryptoassetClassification::SecurityToken {
                investment_type, ..
            } => {
                assert_eq!(investment_type, SecurityTokenType::Equity);
            }
            _ => panic!("Expected SecurityToken classification"),
        }
    }

    #[test]
    fn test_utility_token_not_security() {
        let assessment = SecurityTokenAssessment {
            token_name: "GameToken".to_string(),
            investment_of_money: true,
            common_enterprise: false,
            expectation_of_profit: false, // No profit expectation
            profit_from_others_efforts: false,
            equity_rights: false,
            debt_rights: false,
            derivative_characteristics: false,
        };

        assert!(!assessment.is_likely_security());
        assert!(matches!(
            assessment.recommended_classification(),
            CryptoassetClassification::UtilityToken { .. }
        ));
    }
}
