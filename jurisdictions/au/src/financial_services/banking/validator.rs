//! Banking Validators

use super::error::{BankingError, Result};
use super::types::*;

/// Validate ADI compliance
pub fn validate_adi_compliance(adi: &AuthorizedDepositInstitution) -> Result<()> {
    // Check authorization status
    if adi.status != AdiStatus::Authorized {
        return Err(BankingError::AuthorizationNotCurrent {
            adi_name: adi.name.clone(),
            status: format!("{:?}", adi.status),
        });
    }

    // Validate capital
    validate_capital_adequacy(&adi.capital)?;

    // Validate liquidity
    validate_liquidity(&adi.liquidity)?;

    Ok(())
}

/// Validate capital adequacy (APS 110)
pub fn validate_capital_adequacy(capital: &CapitalRequirement) -> Result<()> {
    // Check CET1
    let min_cet1 = capital.minimum_cet1();
    if capital.cet1_ratio < min_cet1 {
        return Err(BankingError::CapitalInadequacy {
            ratio_type: "CET1".to_string(),
            actual: capital.cet1_ratio,
            required: min_cet1,
        });
    }

    // Check Tier 1
    let min_tier1 = capital.minimum_tier1();
    if capital.tier1_ratio < min_tier1 {
        return Err(BankingError::CapitalInadequacy {
            ratio_type: "Tier 1".to_string(),
            actual: capital.tier1_ratio,
            required: min_tier1,
        });
    }

    // Check Total Capital
    let min_total = capital.minimum_total();
    if capital.total_capital_ratio < min_total {
        return Err(BankingError::CapitalInadequacy {
            ratio_type: "Total Capital".to_string(),
            actual: capital.total_capital_ratio,
            required: min_total,
        });
    }

    Ok(())
}

/// Validate liquidity (APS 210)
pub fn validate_liquidity(liquidity: &LiquidityRequirement) -> Result<()> {
    // Check LCR (minimum 100%)
    if liquidity.lcr < 100.0 {
        return Err(BankingError::LiquidityInadequacy {
            ratio_type: "LCR".to_string(),
            actual: liquidity.lcr,
        });
    }

    // Check NSFR (minimum 100%)
    if liquidity.nsfr < 100.0 {
        return Err(BankingError::LiquidityInadequacy {
            ratio_type: "NSFR".to_string(),
            actual: liquidity.nsfr,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_compliant_adi() -> AuthorizedDepositInstitution {
        AuthorizedDepositInstitution {
            name: "Test Bank".to_string(),
            abn: "12345678901".to_string(),
            apra_registration: "APRA-001".to_string(),
            category: AdiCategory::OtherDomesticBank,
            status: AdiStatus::Authorized,
            authorization_date: NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(),
            capital: CapitalRequirement {
                cet1_ratio: 12.0,
                tier1_ratio: 14.0,
                total_capital_ratio: 16.0,
                rwa_aud_millions: 50_000.0,
                conservation_buffer: 2.5,
                dsib_buffer: None,
                countercyclical_buffer: 0.0,
                meets_requirements: true,
            },
            liquidity: LiquidityRequirement {
                lcr: 130.0,
                nsfr: 115.0,
                hqla_aud_millions: 10_000.0,
                net_cash_outflows_aud_millions: 7_700.0,
                meets_lcr: true,
                meets_nsfr: true,
            },
        }
    }

    #[test]
    fn test_validate_adi_compliance_success() {
        let adi = create_compliant_adi();
        assert!(validate_adi_compliance(&adi).is_ok());
    }

    #[test]
    fn test_validate_adi_compliance_not_authorized() {
        let mut adi = create_compliant_adi();
        adi.status = AdiStatus::Suspended;

        let result = validate_adi_compliance(&adi);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(BankingError::AuthorizationNotCurrent { .. })
        ));
    }

    #[test]
    fn test_validate_capital_inadequacy() {
        let capital = CapitalRequirement {
            cet1_ratio: 5.0, // Below minimum
            tier1_ratio: 7.0,
            total_capital_ratio: 9.0,
            rwa_aud_millions: 50_000.0,
            conservation_buffer: 2.5,
            dsib_buffer: None,
            countercyclical_buffer: 0.0,
            meets_requirements: false,
        };

        let result = validate_capital_adequacy(&capital);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(BankingError::CapitalInadequacy { .. })
        ));
    }

    #[test]
    fn test_validate_liquidity_inadequacy() {
        let liquidity = LiquidityRequirement {
            lcr: 85.0, // Below 100%
            nsfr: 110.0,
            hqla_aud_millions: 8_500.0,
            net_cash_outflows_aud_millions: 10_000.0,
            meets_lcr: false,
            meets_nsfr: true,
        };

        let result = validate_liquidity(&liquidity);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(BankingError::LiquidityInadequacy { .. })
        ));
    }
}
