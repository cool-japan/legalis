//! FBA Validation Functions

use super::error::{FbaError, FbaResult};
use super::types::*;
use serde::{Deserialize, Serialize};

/// Validate foreign ownership for a business activity
pub fn validate_foreign_ownership(
    activity: &BusinessActivity,
    ownership: &ForeignOwnership,
) -> FbaResult<()> {
    // Check if activity is restricted
    match activity.restriction_list {
        BusinessRestrictionList::List1Prohibited => {
            if ownership.total_foreign_percentage > 0.0 {
                return Err(FbaError::ProhibitedActivity {
                    activity: activity.name_th.clone(),
                });
            }
        }
        BusinessRestrictionList::List2CabinetApproval => {
            if ownership.total_foreign_percentage > 49.0 && !ownership.has_treaty_exemption() {
                return Err(FbaError::RequiresCabinetApproval {
                    activity: activity.name_th.clone(),
                });
            }
        }
        BusinessRestrictionList::List3License => {
            if ownership.total_foreign_percentage > 49.0
                && !ownership.has_treaty_exemption()
                && !ownership.has_boi_promotion()
            {
                return Err(FbaError::RequiresLicense {
                    activity: activity.name_th.clone(),
                });
            }
        }
        BusinessRestrictionList::NotRestricted => {
            // No restrictions
        }
    }

    // Check nominee structure
    if ownership.uses_nominees {
        return Err(FbaError::NomineeStructure {
            description: "ตรวจพบการใช้ผู้ถือหุ้นแทน (นอมินี)".to_string(),
        });
    }

    Ok(())
}

/// Validate ownership structure
pub fn validate_ownership_structure(structure: &OwnershipStructure) -> FbaResult<()> {
    // Check capital requirement
    if !structure.meets_capital_requirement() {
        return Err(FbaError::ExcessiveForeignOwnership {
            percentage: structure.foreign_ownership.total_foreign_percentage,
            limit: 49.0,
        });
    }

    // Check Thai shareholders
    if structure.thai_shareholder_count < 3 {
        return Err(FbaError::InsufficientThaiShareholders {
            count: structure.thai_shareholder_count,
            required: 3,
        });
    }

    Ok(())
}

/// Validate treaty exemption eligibility
pub fn validate_treaty_exemption(
    investor: &ForeignInvestor,
    treaty: TreatyExemption,
) -> FbaResult<()> {
    let valid_nationality = match treaty {
        TreatyExemption::UsTreatyOfAmity => {
            investor.nationality.to_lowercase().contains("usa")
                || investor
                    .nationality
                    .to_lowercase()
                    .contains("united states")
                || investor.nationality == "สหรัฐอเมริกา"
        }
        TreatyExemption::AseanFramework => {
            let asean_countries = [
                "brunei",
                "cambodia",
                "indonesia",
                "laos",
                "malaysia",
                "myanmar",
                "philippines",
                "singapore",
                "vietnam",
                "บรูไน",
                "กัมพูชา",
                "อินโดนีเซีย",
                "ลาว",
                "มาเลเซีย",
                "เมียนมา",
                "ฟิลิปปินส์",
                "สิงคโปร์",
                "เวียดนาม",
            ];
            asean_countries
                .iter()
                .any(|c| investor.nationality.to_lowercase().contains(c))
        }
        TreatyExemption::JapanThailandEpa => {
            investor.nationality.to_lowercase().contains("japan") || investor.nationality == "ญี่ปุ่น"
        }
        TreatyExemption::AustraliaThailandFta => {
            investor.nationality.to_lowercase().contains("australia")
                || investor.nationality == "ออสเตรเลีย"
        }
        TreatyExemption::OtherBilateral => true,
    };

    if !valid_nationality {
        return Err(FbaError::TreatyExemptionInvalid {
            reason: format!(
                "สัญชาติ '{}' ไม่สามารถใช้สิทธิตาม{}",
                investor.nationality,
                treaty.name_th()
            ),
        });
    }

    Ok(())
}

/// FBA compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FbaCompliance {
    /// Overall compliance status
    pub compliant: bool,

    /// Activity restriction list
    pub restriction_list: BusinessRestrictionList,

    /// Foreign ownership percentage
    pub foreign_ownership_percentage: f64,

    /// Whether license is required
    pub license_required: bool,

    /// Whether has valid license
    pub has_valid_license: bool,

    /// Whether has treaty exemption
    pub has_treaty_exemption: bool,

    /// Whether has BOI promotion
    pub has_boi_promotion: bool,

    /// List of compliance issues
    pub issues: Vec<String>,

    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive FBA compliance check
pub fn validate_fba_compliance(
    activity: &BusinessActivity,
    ownership: &ForeignOwnership,
    has_valid_license: bool,
) -> FbaCompliance {
    let mut compliance = FbaCompliance {
        compliant: true,
        restriction_list: activity.restriction_list,
        foreign_ownership_percentage: ownership.total_foreign_percentage,
        license_required: matches!(
            activity.restriction_list,
            BusinessRestrictionList::List2CabinetApproval | BusinessRestrictionList::List3License
        ),
        has_valid_license,
        has_treaty_exemption: ownership.has_treaty_exemption(),
        has_boi_promotion: ownership.has_boi_promotion(),
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check restriction list
    match activity.restriction_list {
        BusinessRestrictionList::List1Prohibited => {
            if ownership.total_foreign_percentage > 0.0 {
                compliance.compliant = false;
                compliance
                    .issues
                    .push(format!("ธุรกิจ '{}' ห้ามมิให้คนต่างด้าวประกอบ", activity.name_th));
                compliance
                    .recommendations
                    .push("ยุติการดำเนินธุรกิจหรือโอนหุ้นให้คนไทยทั้งหมด".to_string());
            }
        }
        BusinessRestrictionList::List2CabinetApproval => {
            if ownership.total_foreign_percentage > 49.0
                && !compliance.has_treaty_exemption
                && !compliance.has_boi_promotion
            {
                compliance.compliant = false;
                compliance.issues.push(format!(
                    "ธุรกิจ '{}' ต้องได้รับอนุมัติจากคณะรัฐมนตรี",
                    activity.name_th
                ));
                compliance
                    .recommendations
                    .push("ยื่นคำขออนุมัติต่อคณะรัฐมนตรีหรือลดสัดส่วนการถือหุ้นต่างด้าว".to_string());
            }
        }
        BusinessRestrictionList::List3License => {
            if ownership.total_foreign_percentage > 49.0
                && !compliance.has_treaty_exemption
                && !compliance.has_boi_promotion
                && !has_valid_license
            {
                compliance.compliant = false;
                compliance.issues.push(format!(
                    "ธุรกิจ '{}' ต้องมีใบอนุญาตประกอบธุรกิจของคนต่างด้าว",
                    activity.name_th
                ));
                compliance
                    .recommendations
                    .push("ยื่นคำขอใบอนุญาตต่อกรมพัฒนาธุรกิจการค้า".to_string());
            }
        }
        BusinessRestrictionList::NotRestricted => {
            // No restrictions
        }
    }

    // Check nominee structure
    if ownership.uses_nominees {
        compliance.compliant = false;
        compliance
            .issues
            .push("ตรวจพบโครงสร้างนอมินีที่ผิดกฎหมาย".to_string());
        compliance
            .recommendations
            .push("ปรับโครงสร้างผู้ถือหุ้นให้ถูกต้องตามกฎหมาย".to_string());
    }

    compliance
}

/// Get FBA checklist
pub fn get_fba_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ตรวจสอบบัญชีธุรกิจที่จำกัด", "Check business restriction list"),
        ("สัดส่วนหุ้นต่างด้าวไม่เกิน 49%", "Foreign ownership ≤49%"),
        ("ผู้ถือหุ้นไทยไม่น้อยกว่า 3 คน", "At least 3 Thai shareholders"),
        ("ทุนจดทะเบียนขั้นต่ำ 3 ล้านบาท", "Minimum capital 3M THB"),
        ("ไม่มีโครงสร้างนอมินี", "No nominee structure"),
        ("ใบอนุญาต FBA (ถ้าจำเป็น)", "FBA License (if required)"),
        ("สิทธิตามสนธิสัญญา (ถ้ามี)", "Treaty exemption (if applicable)"),
        ("สิทธิประโยชน์ BOI (ถ้ามี)", "BOI promotion (if applicable)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_foreign_ownership_prohibited() {
        let activity = BusinessActivity::new(
            "1.1",
            "การค้าที่ดิน",
            "Land Trading",
            BusinessRestrictionList::List1Prohibited,
        );
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 10.0)];
        let ownership = ForeignOwnership::new(investors);

        assert!(validate_foreign_ownership(&activity, &ownership).is_err());
    }

    #[test]
    fn test_validate_foreign_ownership_list3_within_limit() {
        let activity = BusinessActivity::new(
            "3.1",
            "การค้าปลีก",
            "Retail",
            BusinessRestrictionList::List3License,
        );
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 40.0)];
        let ownership = ForeignOwnership::new(investors);

        assert!(validate_foreign_ownership(&activity, &ownership).is_ok());
    }

    #[test]
    fn test_validate_foreign_ownership_list3_exceeds() {
        let activity = BusinessActivity::new(
            "3.1",
            "การค้าปลีก",
            "Retail",
            BusinessRestrictionList::List3License,
        );
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 60.0)];
        let ownership = ForeignOwnership::new(investors);

        assert!(validate_foreign_ownership(&activity, &ownership).is_err());
    }

    #[test]
    fn test_validate_foreign_ownership_with_treaty() {
        let activity = BusinessActivity::new(
            "3.1",
            "การค้าปลีก",
            "Retail",
            BusinessRestrictionList::List3License,
        );
        let investors = vec![
            ForeignInvestor::new("US Corp", "USA", 100.0)
                .with_treaty(TreatyExemption::UsTreatyOfAmity),
        ];
        let ownership = ForeignOwnership::new(investors);

        assert!(validate_foreign_ownership(&activity, &ownership).is_ok());
    }

    #[test]
    fn test_validate_treaty_exemption_valid() {
        let investor = ForeignInvestor::new("US Corp", "USA", 100.0);
        assert!(validate_treaty_exemption(&investor, TreatyExemption::UsTreatyOfAmity).is_ok());
    }

    #[test]
    fn test_validate_treaty_exemption_invalid() {
        let investor = ForeignInvestor::new("German Corp", "Germany", 100.0);
        assert!(validate_treaty_exemption(&investor, TreatyExemption::UsTreatyOfAmity).is_err());
    }

    #[test]
    fn test_fba_compliance_check() {
        let activity = BusinessActivity::new(
            "3.1",
            "การค้าปลีก",
            "Retail",
            BusinessRestrictionList::List3License,
        );
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 40.0)];
        let ownership = ForeignOwnership::new(investors);

        let compliance = validate_fba_compliance(&activity, &ownership, false);
        assert!(compliance.compliant);
    }

    #[test]
    fn test_fba_compliance_check_non_compliant() {
        let activity = BusinessActivity::new(
            "3.1",
            "การค้าปลีก",
            "Retail",
            BusinessRestrictionList::List3License,
        );
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 60.0)];
        let ownership = ForeignOwnership::new(investors);

        let compliance = validate_fba_compliance(&activity, &ownership, false);
        assert!(!compliance.compliant);
    }

    #[test]
    fn test_fba_checklist() {
        let checklist = get_fba_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(th, _)| th.contains("49%")));
    }
}
