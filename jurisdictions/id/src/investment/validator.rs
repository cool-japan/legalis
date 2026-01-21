//! Validation functions for Indonesian Investment Law

use super::error::{InvestmentError, InvestmentResult};
use super::types::*;
use serde::{Deserialize, Serialize};

/// Check foreign ownership limit for a sector
pub fn check_ownership_limit(kbli_code: &str, foreign_percentage: u32) -> InvestmentResult<()> {
    // Simplified DNI check - in reality would check full database
    let limit = get_ownership_limit_for_kbli(kbli_code);

    match limit {
        Some(0) => Err(InvestmentError::SectorClosed {
            sector: kbli_code.to_string(),
        }),
        Some(max_percent) if foreign_percentage > max_percent => {
            Err(InvestmentError::OwnershipExceedsLimit {
                sector: kbli_code.to_string(),
                actual: foreign_percentage,
                limit: max_percent,
            })
        }
        _ => Ok(()),
    }
}

/// Get ownership limit for KBLI code (simplified)
fn get_ownership_limit_for_kbli(kbli_code: &str) -> Option<u32> {
    // Simplified mapping - real implementation would use full DNI database
    match kbli_code {
        // Closed sectors
        "01700" | "02100" => Some(0), // Natural forest exploitation, logging
        "41010" => Some(0),           // Public housing development (MSME)

        // Restricted sectors
        "47111" | "47112" => Some(0), // Mini market, traditional retail (MSME)
        "46100" => Some(67),          // Wholesale trade
        "55111" => Some(67),          // Star hotel
        "60100" => Some(49),          // Broadcasting

        // Open sectors (100% foreign allowed)
        "62011" | "62012" => Some(100), // Software development
        "70100" => Some(100),           // Headquarters
        "72100" => Some(100),           // R&D
        _ => Some(100),                 // Default open
    }
}

/// Validate sector eligibility for foreign investment
pub fn validate_sector_eligibility(
    kbli_codes: &[String],
    is_foreign_investment: bool,
    foreign_percentage: u32,
) -> InvestmentResult<()> {
    if !is_foreign_investment {
        return Ok(());
    }

    for kbli in kbli_codes {
        check_ownership_limit(kbli, foreign_percentage)?;
    }

    Ok(())
}

/// Validate foreign investment requirements
pub fn validate_foreign_investment(
    investment: &ForeignInvestment,
    exchange_rate: f64,
) -> InvestmentResult<()> {
    // Check minimum capital
    let investment_idr = (investment.investment_usd as f64 * exchange_rate) as i64;
    if investment_idr < ForeignInvestment::minimum_capital_idr() {
        return Err(InvestmentError::MinimumCapitalNotMet {
            actual: investment_idr,
            required: ForeignInvestment::minimum_capital_idr(),
        });
    }

    // Check sector eligibility
    validate_sector_eligibility(
        &investment.kbli_codes,
        true,
        investment.foreign_ownership_percent,
    )?;

    // Check partnership requirements for restricted sectors
    for kbli in &investment.kbli_codes {
        if requires_local_partnership(kbli) && investment.local_partner.is_none() {
            return Err(InvestmentError::PartnershipRequired {
                sector: kbli.clone(),
            });
        }
    }

    Ok(())
}

/// Check if sector requires local partnership
fn requires_local_partnership(kbli_code: &str) -> bool {
    // Sectors requiring partnership with MSME
    matches!(kbli_code, "46410" | "46420" | "46510") // Various wholesale requiring partnership
}

/// Validate business license
pub fn validate_business_license(license: &BusinessLicense) -> InvestmentResult<()> {
    // Check NIB present
    if license.nib.is_empty() {
        return Err(InvestmentError::MissingNib);
    }

    // Check license type matches risk level
    let required_license = license.risk_level.required_license();
    if license.license_type != required_license {
        return Err(InvestmentError::LicenseTypeMismatch {
            required: format!("{:?}", required_license),
            actual: format!("{:?}", license.license_type),
        });
    }

    // Check certificates for medium/high risk
    match license.risk_level {
        BusinessRisk::MediumLow | BusinessRisk::MediumHigh => {
            if license.certificates.is_empty() {
                return Err(InvestmentError::MissingCertificate {
                    risk_level: format!("{:?}", license.risk_level),
                });
            }
        }
        BusinessRisk::High => {
            if license.certificates.is_empty() {
                return Err(InvestmentError::MissingCertificate {
                    risk_level: "High".to_string(),
                });
            }
        }
        _ => {}
    }

    // Check foreign ownership if PMA
    if license.is_pma
        && let Some(foreign_percent) = license.foreign_ownership_percent
    {
        for kbli in &license.kbli_codes {
            check_ownership_limit(kbli, foreign_percent)?;
        }
    }

    Ok(())
}

/// Investment compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentCompliance {
    /// Overall compliance status
    pub compliant: bool,
    /// NIB obtained
    pub nib_obtained: bool,
    /// Sector eligibility confirmed
    pub sector_eligible: bool,
    /// Capital requirements met
    pub capital_adequate: bool,
    /// License type appropriate
    pub license_appropriate: bool,
    /// Partnership requirements met (if applicable)
    pub partnership_met: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Available incentives
    pub available_incentives: Vec<String>,
}

/// Comprehensive investment compliance check
pub fn validate_investment_compliance(
    investment: &ForeignInvestment,
    has_nib: bool,
    exchange_rate: f64,
) -> InvestmentCompliance {
    let mut compliance = InvestmentCompliance {
        compliant: true,
        nib_obtained: has_nib,
        sector_eligible: true,
        capital_adequate: true,
        license_appropriate: true,
        partnership_met: true,
        issues: Vec::new(),
        recommendations: Vec::new(),
        available_incentives: Vec::new(),
    };

    // Check NIB
    if !has_nib {
        compliance.compliant = false;
        compliance.issues.push("NIB belum diperoleh".to_string());
        compliance
            .recommendations
            .push("Ajukan NIB melalui OSS (oss.go.id)".to_string());
    }

    // Check capital
    let investment_idr = (investment.investment_usd as f64 * exchange_rate) as i64;
    if investment_idr < ForeignInvestment::minimum_capital_idr() {
        compliance.compliant = false;
        compliance.capital_adequate = false;
        compliance.issues.push(format!(
            "Modal Rp {} kurang dari minimum Rp 10 miliar (PP 5/2021)",
            investment_idr
        ));
    }

    // Check sector eligibility
    for kbli in &investment.kbli_codes {
        if let Err(e) = check_ownership_limit(kbli, investment.foreign_ownership_percent) {
            compliance.compliant = false;
            compliance.sector_eligible = false;
            compliance.issues.push(format!("{}", e));
        }

        if requires_local_partnership(kbli) && investment.local_partner.is_none() {
            compliance.compliant = false;
            compliance.partnership_met = false;
            compliance
                .issues
                .push(format!("Kemitraan UMKM diperlukan untuk KBLI {}", kbli));
        }
    }

    // Add incentive recommendations if priority sector
    if investment.is_priority_sector {
        compliance
            .available_incentives
            .push("Tax Holiday (5-20 tahun)".to_string());
        compliance
            .available_incentives
            .push("Investment Allowance".to_string());
        compliance
            .available_incentives
            .push("Accelerated Depreciation".to_string());
        compliance
            .available_incentives
            .push("Import Duty Exemption".to_string());
    }

    // SEZ incentives
    if investment.in_sez {
        compliance
            .available_incentives
            .push("KEK Tax Incentives".to_string());
        compliance
            .available_incentives
            .push("Customs facilities".to_string());
    }

    compliance
}

/// Get investment compliance checklist
pub fn get_investment_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "NIB (Nomor Induk Berusaha) diperoleh",
            "NIB obtained",
            "OSS",
        ),
        (
            "Modal minimum Rp 10 miliar",
            "Min capital Rp 10B",
            "PP 5/2021",
        ),
        (
            "Modal disetor minimum Rp 2.5 miliar",
            "Paid-up min Rp 2.5B",
            "PP 5/2021",
        ),
        (
            "Sektor tidak dalam DNI tertutup",
            "Sector not in closed DNI",
            "PP 10/2021",
        ),
        (
            "Kepemilikan asing sesuai batas DNI",
            "Foreign ownership within DNI limit",
            "PP 10/2021",
        ),
        (
            "Sertifikat standar (jika diperlukan)",
            "Standard certificate (if required)",
            "PP 5/2021",
        ),
        (
            "RPTKA untuk TKA",
            "RPTKA for foreign workers",
            "UU Ketenagakerjaan",
        ),
        (
            "Kemitraan UMKM (jika diperlukan)",
            "MSME partnership (if required)",
            "PP 7/2021",
        ),
        ("Izin lokasi/pertanahan", "Location/land permit", "Varies"),
        (
            "Izin lingkungan (AMDAL/UKL-UPL)",
            "Environmental permit",
            "UU PPLH",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ownership_limit_open() {
        let result = check_ownership_limit("62011", 100); // Software dev - open
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_ownership_limit_restricted() {
        let result = check_ownership_limit("46100", 100); // Wholesale - 67% max
        assert!(matches!(
            result,
            Err(InvestmentError::OwnershipExceedsLimit { .. })
        ));
    }

    #[test]
    fn test_check_ownership_limit_closed() {
        let result = check_ownership_limit("47111", 100); // Mini market - closed
        assert!(matches!(result, Err(InvestmentError::SectorClosed { .. })));
    }

    #[test]
    fn test_validate_foreign_investment() {
        let investment = ForeignInvestment {
            company_name: "PT Example".to_string(),
            investor_country: "Singapore".to_string(),
            investment_usd: 1_000_000,
            foreign_ownership_percent: 100,
            local_partner: None,
            kbli_codes: vec!["62011".to_string()],
            location: "Jakarta".to_string(),
            is_priority_sector: true,
            in_sez: false,
            planned_local_employment: 50,
            planned_foreign_employment: 2,
        };

        let result = validate_foreign_investment(&investment, 15_000.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_foreign_investment_insufficient_capital() {
        let investment = ForeignInvestment {
            company_name: "PT Example".to_string(),
            investor_country: "Singapore".to_string(),
            investment_usd: 100_000, // Too low
            foreign_ownership_percent: 100,
            local_partner: None,
            kbli_codes: vec!["62011".to_string()],
            location: "Jakarta".to_string(),
            is_priority_sector: false,
            in_sez: false,
            planned_local_employment: 10,
            planned_foreign_employment: 1,
        };

        let result = validate_foreign_investment(&investment, 15_000.0);
        assert!(matches!(
            result,
            Err(InvestmentError::MinimumCapitalNotMet { .. })
        ));
    }

    #[test]
    fn test_validate_business_license() {
        let license = BusinessLicense {
            nib: "1234567890".to_string(),
            company_name: "PT Example".to_string(),
            kbli_codes: vec!["62011".to_string()],
            license_type: LicenseType::NibOnly,
            risk_level: BusinessRisk::Low,
            is_pma: true,
            foreign_ownership_percent: Some(100),
            investment_value: 15_000_000_000,
            location: "Jakarta".to_string(),
            sez_location: None,
            certificates: vec![],
        };

        assert!(validate_business_license(&license).is_ok());
    }

    #[test]
    fn test_investment_compliance() {
        let investment = ForeignInvestment {
            company_name: "PT Priority".to_string(),
            investor_country: "Japan".to_string(),
            investment_usd: 1_000_000,
            foreign_ownership_percent: 100,
            local_partner: None,
            kbli_codes: vec!["62011".to_string()],
            location: "Jakarta".to_string(),
            is_priority_sector: true,
            in_sez: true,
            planned_local_employment: 100,
            planned_foreign_employment: 5,
        };

        let compliance = validate_investment_compliance(&investment, true, 15_000.0);
        assert!(compliance.compliant);
        assert!(!compliance.available_incentives.is_empty());
    }

    #[test]
    fn test_investment_checklist() {
        let checklist = get_investment_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(id, _, _)| id.contains("NIB")));
    }
}
