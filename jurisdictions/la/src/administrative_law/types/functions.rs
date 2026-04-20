//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// Administrative appeal deadline: 30 days from notification
/// ກຳນົດເວລາການອຸທອນບໍລິຫານ: 30 ວັນນັບແຕ່ວັນທີ່ໄດ້ຮັບແຈ້ງ
pub const ADMINISTRATIVE_APPEAL_DEADLINE_DAYS: u8 = 30;
/// Court appeal deadline: 60 days from administrative decision
/// ກຳນົດເວລາການຟ້ອງຕໍ່ສານ: 60 ວັນນັບແຕ່ວັນທີ່ມີການຕັດສິນໃຈບໍລິຫານ
pub const COURT_APPEAL_DEADLINE_DAYS: u8 = 60;
/// State liability claim deadline: 2 years from wrongful act
/// ກຳນົດເວລາການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ: 2 ປີນັບແຕ່ວັນທີ່ມີການກະທຳຜິດ
pub const STATE_LIABILITY_CLAIM_DEADLINE_YEARS: u8 = 2;
/// Minimum fine amount in LAK
/// ຈຳນວນເງິນປັບຂັ້ນຕ່ຳເປັນກີບ
pub const MINIMUM_FINE_AMOUNT_LAK: u64 = 100_000;
/// Maximum suspension days for licenses
/// ຈຳນວນວັນສູງສຸດສຳລັບການລະງັບໃບອະນຸຍາດ
pub const MAXIMUM_SUSPENSION_DAYS: u32 = 365;
/// Maximum license revocation permanent ban years
/// ຈຳນວນປີສູງສຸດສຳລັບການຫ້າມຖາວອນ
pub const MAXIMUM_BAN_YEARS: u32 = 10;
/// Default notification period in days
/// ໄລຍະເວລາແຈ້ງເຕືອນເລີ່ມຕົ້ນເປັນວັນ
pub const DEFAULT_NOTIFICATION_PERIOD_DAYS: u8 = 15;
/// Village level jurisdiction limit in LAK
/// ຂອບເຂດອຳນາດລະດັບບ້ານເປັນກີບ
pub const VILLAGE_JURISDICTION_LIMIT_LAK: u64 = 5_000_000;
/// District level jurisdiction limit in LAK
/// ຂອບເຂດອຳນາດລະດັບເມືອງເປັນກີບ
pub const DISTRICT_JURISDICTION_LIMIT_LAK: u64 = 50_000_000;
/// Provincial level jurisdiction limit in LAK
/// ຂອບເຂດອຳນາດລະດັບແຂວງເປັນກີບ
pub const PROVINCIAL_JURISDICTION_LIMIT_LAK: u64 = 500_000_000;
#[cfg(test)]
mod tests {
    use super::super::decision_types::*;
    use super::super::types_2::*;
    use super::*;
    #[test]
    fn test_administrative_level_hierarchy() {
        let central = AdministrativeLevel::Central {
            ministry: "Ministry of Justice".to_string(),
        };
        let provincial = AdministrativeLevel::Provincial {
            province: "Vientiane".to_string(),
        };
        let district = AdministrativeLevel::District {
            district: "Sisattanak".to_string(),
        };
        let village = AdministrativeLevel::Village {
            village: "Ban Nongbone".to_string(),
        };
        assert!(central.is_superior_to(&provincial));
        assert!(provincial.is_superior_to(&district));
        assert!(district.is_superior_to(&village));
        assert!(!village.is_superior_to(&central));
    }
    #[test]
    fn test_administrative_decision_builder() {
        let decision = AdministrativeDecision::builder()
            .decision_number("DEC-2024-001".to_string())
            .issuing_authority(AdministrativeLevel::Central {
                ministry: "Ministry of Industry and Commerce".to_string(),
            })
            .decision_date("2024-01-15".to_string())
            .subject_lao("ການອອກໃບອະນຸຍາດປະກອບທຸລະກິດ".to_string())
            .subject_en("Business License Issuance".to_string())
            .decision_type(DecisionType::License {
                license_type: LicenseType::BusinessLicense,
            })
            .legal_basis(LegalBasis::new(
                "ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ",
                "Enterprise Law",
                15,
                Some(1),
            ))
            .affected_party(AffectedParty::new(
                "ABC Company Ltd.",
                PartyType::LegalEntity,
            ))
            .is_final(false)
            .appeal_deadline_days(Some(30))
            .build();
        assert!(decision.is_ok());
        let decision = decision.expect("Failed to build decision");
        assert_eq!(decision.decision_number, "DEC-2024-001");
    }
    #[test]
    fn test_administrative_sanction_builder() {
        let sanction = AdministrativeSanction::builder()
            .sanction_id("SANC-2024-001".to_string())
            .sanction_type(SanctionType::Fine {
                amount_lak: 5_000_000,
                payment_deadline: "2024-02-15".to_string(),
            })
            .issuing_authority(AdministrativeLevel::Provincial {
                province: "Vientiane".to_string(),
            })
            .legal_basis(LegalBasis::new("ກົດໝາຍວ່າດ້ວຍພາສີ", "Tax Law", 50, None))
            .violation_description_lao("ການຍື່ນພາສີຊ້າ".to_string())
            .violation_description_en("Late tax filing".to_string())
            .sanction_date("2024-01-20".to_string())
            .appeal_available(true)
            .subject(AffectedParty::new("Company XYZ", PartyType::LegalEntity))
            .appeal_deadline_days(30)
            .build();
        assert!(sanction.is_ok());
    }
    #[test]
    fn test_administrative_appeal_builder() {
        let appeal = AdministrativeAppeal::builder()
            .appeal_number("APP-2024-001".to_string())
            .original_decision("DEC-2024-001".to_string())
            .appellant(AffectedParty::new("John Doe", PartyType::Individual))
            .appeal_ground(AppealGround::ProceduralError {
                description: "Not properly notified".to_string(),
            })
            .filing_date("2024-02-01".to_string())
            .appeal_level(AppealLevel::SuperiorAuthority {
                authority: "Ministry of Justice".to_string(),
            })
            .deadline_date("2024-02-15".to_string())
            .build();
        assert!(appeal.is_ok());
    }
    #[test]
    fn test_state_liability_creation() {
        let claim = StateLiability::new(
            "SLC-2024-001",
            AffectedParty::new("Jane Smith", PartyType::Individual),
            AdministrativeLevel::Provincial {
                province: "Savannakhet".to_string(),
            },
            LiabilityType::WrongfulDecision,
            "ຄວາມເສຍຫາຍຈາກການຕັດສິນໃຈທີ່ຜິດກົດໝາຍ",
            "Damage from wrongful administrative decision",
            50_000_000,
        )
        .with_wrongful_act_date("2023-11-01")
        .with_filing_date("2024-01-15")
        .with_evidence("Witness statement");
        assert_eq!(claim.claim_number, "SLC-2024-001");
        assert_eq!(claim.claimed_amount_lak, 50_000_000);
    }
    #[test]
    fn test_sanction_severity_levels() {
        let warning = SanctionType::Warning { written: false };
        let small_fine = SanctionType::Fine {
            amount_lak: 500_000,
            payment_deadline: "2024-02-15".to_string(),
        };
        let revocation = SanctionType::LicenseRevocation;
        assert_eq!(warning.severity_level(), 1);
        assert_eq!(small_fine.severity_level(), 2);
        assert_eq!(revocation.severity_level(), 5);
    }
    #[test]
    fn test_legal_basis_citations() {
        let basis = LegalBasis::new("ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ", "Enterprise Law", 15, Some(2));
        assert!(basis.citation_lao().contains("ມາດຕາ 15"));
        assert!(basis.citation_lao().contains("ວັກ 2"));
        assert!(basis.citation_en().contains("Article 15"));
        assert!(basis.citation_en().contains("Paragraph 2"));
    }
    #[test]
    fn test_appeal_level_deadlines() {
        assert_eq!(
            AppealLevel::SameAuthority.deadline_days(),
            ADMINISTRATIVE_APPEAL_DEADLINE_DAYS
        );
        assert_eq!(
            AppealLevel::AdministrativeCourt.deadline_days(),
            COURT_APPEAL_DEADLINE_DAYS
        );
    }
    #[test]
    fn test_license_type_authority_levels() {
        assert_eq!(LicenseType::MiningLicense.minimum_authority_level(), 0);
        assert_eq!(
            LicenseType::EnvironmentalLicense.minimum_authority_level(),
            1
        );
        assert_eq!(LicenseType::BusinessLicense.minimum_authority_level(), 2);
    }
}
