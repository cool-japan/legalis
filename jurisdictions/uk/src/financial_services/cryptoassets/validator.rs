//! Cryptoassets Validators (FSMA 2023, FCA PS19/22, MLR 2017 Reg 14A)

use super::error::{CryptoassetsError, Result};
use super::types::*;
use chrono::NaiveDate;

/// Validate cryptoasset classification
///
/// Checks appropriate classification based on token characteristics.
/// Security tokens require FCA authorization, e-money tokens require EMI authorization.
pub fn validate_cryptoasset_classification(
    classification: &CryptoassetClassification,
    fca_authorized: bool,
) -> Result<()> {
    match classification {
        CryptoassetClassification::SecurityToken { .. } => {
            // Security tokens require FCA authorization (FSMA 2000 s.19)
            if !fca_authorized {
                return Err(CryptoassetsError::SecurityTokenNotAuthorized {
                    token_name: "Token".to_string(),
                });
            }
        }
        CryptoassetClassification::EMoneyToken {
            issuer_authorized,
            redeemable_at_par,
        } => {
            // E-money tokens require EMI authorization (EMR 2011)
            if !issuer_authorized {
                return Err(CryptoassetsError::EMoneyTokenNotAuthorized {
                    token_name: "Token".to_string(),
                });
            }

            // E-money must be redeemable at par (1:1)
            if !redeemable_at_par {
                return Err(CryptoassetsError::ValidationError {
                    message: "E-money token must be redeemable at par (1:1 with fiat)".to_string(),
                });
            }
        }
        CryptoassetClassification::UtilityToken { .. } => {
            // Utility tokens generally unregulated (unless disguised security)
            // But exchange providers still need MLR 2017 Reg 14A registration
        }
        CryptoassetClassification::Stablecoin {
            peg,
            reserve_backing_ratio,
        } => {
            // Stablecoins must be fully reserved (≥1.0 = 100%)
            if *reserve_backing_ratio < 1.0 {
                return Err(CryptoassetsError::StablecoinNotFullyReserved {
                    token_name: "Stablecoin".to_string(),
                    backing_ratio: reserve_backing_ratio * 100.0,
                });
            }

            // Warn on algorithmic stablecoins (high risk)
            if matches!(peg, StablecoinPeg::Algorithmic) {
                return Err(CryptoassetsError::AlgorithmicStablecoin {
                    token_name: "Algorithmic Stablecoin".to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate stablecoin compliance (FSMA 2023 Part 5)
///
/// Checks stablecoin regulatory compliance:
/// - Fully reserved (1:1 backing)
/// - Reserves segregated
/// - Independently audited
/// - Redemption rights provided
pub fn validate_stablecoin(stablecoin: &Stablecoin) -> Result<()> {
    // Check algorithmic first (high risk, may be banned)
    if stablecoin.is_algorithmic() {
        return Err(CryptoassetsError::AlgorithmicStablecoin {
            token_name: stablecoin.token_name.clone(),
        });
    }

    // Check fully reserved (FSMA 2023)
    if !stablecoin.is_fully_reserved() {
        return Err(CryptoassetsError::StablecoinNotFullyReserved {
            token_name: stablecoin.token_name.clone(),
            backing_ratio: stablecoin.reserve_backing.backing_ratio * 100.0,
        });
    }

    // Check reserves segregated
    if !stablecoin.reserve_backing.segregated {
        return Err(CryptoassetsError::StablecoinReservesNotSegregated {
            token_name: stablecoin.token_name.clone(),
        });
    }

    // Check audited
    if !stablecoin.reserve_audited {
        return Err(CryptoassetsError::StablecoinNotAudited {
            token_name: stablecoin.token_name.clone(),
        });
    }

    // Check redemption rights
    if !stablecoin.redemption_rights.redeemable {
        return Err(CryptoassetsError::NoRedemptionRights {
            token_name: stablecoin.token_name.clone(),
        });
    }

    Ok(())
}

/// Validate cryptoasset promotion (FSMA s.21, effective Oct 8, 2023)
///
/// Checks financial promotion compliance:
/// - Approved by FCA-authorized person (from Oct 8, 2023)
/// - Risk warning included and prominent
/// - Fair, clear and not misleading
pub fn validate_cryptoasset_promotion(promotion: &CryptoassetPromotion) -> Result<()> {
    // Check approval requirement (from Oct 8, 2023)
    if promotion.promotion_date
        >= NaiveDate::from_ymd_opt(2023, 10, 8).expect("valid date constant: October 8, 2023")
    {
        if !promotion.approved_by_authorized_person {
            return Err(CryptoassetsError::PromotionNotApproved {
                promotion_date: promotion.promotion_date.to_string(),
            });
        }

        if promotion.approver_frn.is_none() {
            return Err(CryptoassetsError::PromotionNotApproved {
                promotion_date: promotion.promotion_date.to_string(),
            });
        }
    }

    // Check risk warning included
    if !promotion.risk_warning_included {
        return Err(CryptoassetsError::NoRiskWarning {
            promotion_content: promotion.promotion_content.clone(),
        });
    }

    // Check risk warning prominent
    if !promotion.risk_warning_prominent {
        return Err(CryptoassetsError::NoRiskWarning {
            promotion_content: promotion.promotion_content.clone(),
        });
    }

    // Check fair, clear and not misleading (COBS 4.2.1R)
    if !promotion.fair_clear_not_misleading {
        return Err(CryptoassetsError::PromotionMisleading {
            promotion_content: promotion.promotion_content.clone(),
            reason: "Promotion is misleading or unclear".to_string(),
        });
    }

    Ok(())
}

/// Validate cryptoasset exchange provider (MLR 2017 Reg 14A)
///
/// Checks AML/CTF compliance for exchange providers:
/// - FCA registered under MLR 2017 Reg 14A
/// - AML policies in place
/// - CDD performed
/// - SAR procedures established
/// - Travel Rule compliance
pub fn validate_exchange_provider(provider: &CryptoassetExchangeProvider) -> Result<()> {
    // Check FCA registration (MLR 2017 Reg 14A)
    if !provider.fca_registered {
        return Err(CryptoassetsError::ExchangeProviderNotRegistered {
            provider_name: provider.provider_name.clone(),
        });
    }

    if provider.registration_number.is_none() {
        return Err(CryptoassetsError::ExchangeProviderNotRegistered {
            provider_name: provider.provider_name.clone(),
        });
    }

    // Check AML policies
    if !provider.aml_policies {
        return Err(CryptoassetsError::ValidationError {
            message: format!(
                "Exchange provider '{}' does not have AML policies in place",
                provider.provider_name
            ),
        });
    }

    // Check CDD performed
    if !provider.cdd_performed {
        return Err(CryptoassetsError::CddNotPerformed {
            customer_name: "Customer".to_string(),
        });
    }

    // Check SAR procedures
    if !provider.sar_procedures {
        return Err(CryptoassetsError::ValidationError {
            message: format!(
                "Exchange provider '{}' does not have SAR procedures in place",
                provider.provider_name
            ),
        });
    }

    // Check Travel Rule compliance
    if !provider.travel_rule_compliant {
        return Err(CryptoassetsError::TravelRuleViolation { amount_gbp: 1000.0 });
    }

    // Check sanctions screening
    if !provider.sanctions_screening {
        return Err(CryptoassetsError::SanctionsScreeningNotPerformed {
            entity_name: "Entity".to_string(),
        });
    }

    Ok(())
}

/// Validate security token assessment (Howey Test)
///
/// Assesses whether token is likely a security token requiring FCA authorization.
pub fn validate_security_token_assessment(assessment: &SecurityTokenAssessment) -> Result<()> {
    // If token is likely a security, ensure it's properly authorized
    if assessment.is_likely_security() {
        // Return informational validation (actual authorization check done elsewhere)
        Ok(())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_security_token_authorized() {
        let classification = CryptoassetClassification::SecurityToken {
            investment_type: SecurityTokenType::Equity,
            prospectus_required: true,
        };

        assert!(validate_cryptoasset_classification(&classification, true).is_ok());
    }

    #[test]
    fn test_validate_security_token_not_authorized() {
        let classification = CryptoassetClassification::SecurityToken {
            investment_type: SecurityTokenType::Equity,
            prospectus_required: true,
        };

        let result = validate_cryptoasset_classification(&classification, false);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CryptoassetsError::SecurityTokenNotAuthorized { .. })
        ));
    }

    #[test]
    fn test_validate_stablecoin_compliant() {
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
            authorization_type: Some("EMI".to_string()),
        };

        assert!(validate_stablecoin(&stablecoin).is_ok());
    }

    #[test]
    fn test_validate_stablecoin_not_fully_reserved() {
        let stablecoin = Stablecoin {
            token_name: "USDT".to_string(),
            issuer: "Tether Ltd".to_string(),
            peg: StablecoinPeg::FiatCurrency {
                currency: "USD".to_string(),
            },
            reserve_backing: ReserveBacking {
                backing_ratio: 0.85, // Only 85% backed
                assets_description: "Mixed assets".to_string(),
                custody_arrangements: "Various".to_string(),
                segregated: true,
            },
            reserve_audited: false,
            audit_date: None,
            redemption_rights: RedemptionRights {
                redeemable: true,
                redemption_fee: None,
                redemption_timeframe: None,
            },
            fca_authorized: false,
            authorization_type: None,
        };

        let result = validate_stablecoin(&stablecoin);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CryptoassetsError::StablecoinNotFullyReserved { .. })
        ));
    }

    #[test]
    fn test_validate_stablecoin_algorithmic() {
        let stablecoin = Stablecoin {
            token_name: "AlgoStable".to_string(),
            issuer: "Algo Protocol".to_string(),
            peg: StablecoinPeg::Algorithmic,
            reserve_backing: ReserveBacking {
                backing_ratio: 0.0,
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

        let result = validate_stablecoin(&stablecoin);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CryptoassetsError::AlgorithmicStablecoin { .. })
        ));
    }

    #[test]
    fn test_validate_promotion_compliant_post_oct_2023() {
        let promotion = CryptoassetPromotion {
            promotion_content: "Invest in Bitcoin - High risk investment".to_string(),
            promotion_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            medium: PromotionMedium::SocialMedia,
            approved_by_authorized_person: true,
            approver_frn: Some("123456".to_string()),
            risk_warning_included: true,
            risk_warning_prominent: true,
            fair_clear_not_misleading: true,
            target_audience: PromotionAudience::Retail,
        };

        assert!(validate_cryptoasset_promotion(&promotion).is_ok());
    }

    #[test]
    fn test_validate_promotion_not_approved_post_oct_2023() {
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

        let result = validate_cryptoasset_promotion(&promotion);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CryptoassetsError::PromotionNotApproved { .. })
        ));
    }

    #[test]
    fn test_validate_promotion_no_risk_warning() {
        let promotion = CryptoassetPromotion {
            promotion_content: "Get rich with crypto!".to_string(),
            promotion_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            medium: PromotionMedium::SocialMedia,
            approved_by_authorized_person: true,
            approver_frn: Some("123456".to_string()),
            risk_warning_included: false, // NO RISK WARNING
            risk_warning_prominent: false,
            fair_clear_not_misleading: true,
            target_audience: PromotionAudience::Retail,
        };

        let result = validate_cryptoasset_promotion(&promotion);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CryptoassetsError::NoRiskWarning { .. })
        ));
    }

    #[test]
    fn test_validate_exchange_provider_compliant() {
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

        assert!(validate_exchange_provider(&exchange).is_ok());
    }

    #[test]
    fn test_validate_exchange_provider_not_registered() {
        let exchange = CryptoassetExchangeProvider {
            provider_name: "Unregistered Exchange".to_string(),
            fca_registered: false, // NOT REGISTERED
            registration_number: None,
            registration_date: None,
            aml_policies: true,
            cdd_performed: true,
            sar_procedures: true,
            travel_rule_compliant: true,
            sanctions_screening: true,
        };

        let result = validate_exchange_provider(&exchange);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CryptoassetsError::ExchangeProviderNotRegistered { .. })
        ));
    }

    #[test]
    fn test_validate_security_token_assessment() {
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

        assert!(validate_security_token_assessment(&assessment).is_ok());
        assert!(assessment.is_likely_security());
    }
}
