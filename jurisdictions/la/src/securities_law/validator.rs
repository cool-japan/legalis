//! Securities Law Validators (ການກວດສອບກົດໝາຍຫຼັກຊັບ)
//!
//! Validation functions for Lao securities and capital-markets law based on the
//! **Law on Securities (Lao PDR), 2012**.
//!
//! Each validator returns `Ok(())` on compliance, or a [`SecuritiesLawError`]
//! carrying bilingual messages and the governing statute citation.

use crate::securities_law::error::{SecuritiesLawError, SecuritiesResult};
use crate::securities_law::types::*;

// ============================================================================
// Public Offering Validators - ການກວດສອບການສະເໜີຂາຍຕໍ່ສາທາລະນະ
// ============================================================================

/// Validate a public offering of securities.
/// ກວດສອບການສະເໜີຂາຍຫຼັກຊັບ
///
/// The issuer must be identified and the total offering value must be positive.
/// Where the offering type requires a prospectus (any public offering), the
/// offering must have a complete prospectus and Lao SEC approval; a private
/// placement is exempt from those public-offering requirements.
pub fn validate_public_offering(offering: &PublicOffering) -> SecuritiesResult<()> {
    if offering.issuer.trim().is_empty() {
        return Err(SecuritiesLawError::InvalidPublicOffering {
            provision: "public offering — issuer identification",
            message_lao: "ຕ້ອງລະບຸຊື່ຜູ້ອອກຫຼັກຊັບ".to_string(),
            message_en: "The issuer of the offering must be identified".to_string(),
        });
    }

    if offering.total_value_lak == 0 {
        return Err(SecuritiesLawError::InvalidPublicOffering {
            provision: "public offering — offering value",
            message_lao: "ມູນຄ່າການສະເໜີຂາຍຕ້ອງຫຼາຍກວ່າ 0".to_string(),
            message_en: "The total offering value must be greater than zero".to_string(),
        });
    }

    if offering.offering_type.requires_prospectus() {
        if !offering.has_prospectus || !offering.prospectus_complete {
            return Err(SecuritiesLawError::IncompleteProspectus {
                provision: "public offering — prospectus and disclosure",
                message_lao: "ການສະເໜີຂາຍຕໍ່ສາທາລະນະຕ້ອງມີໜັງສືຊີ້ຊວນທີ່ສົມບູນພ້ອມການເປີດເຜີຍຂໍ້ມູນຄົບຖ້ວນ".to_string(),
                message_en: "A public offering requires a complete prospectus with full disclosure"
                    .to_string(),
            });
        }

        if !offering.sec_approved {
            return Err(SecuritiesLawError::OfferingNotApproved {
                message_lao: "ການສະເໜີຂາຍຕໍ່ສາທາລະນະຕ້ອງໄດ້ຮັບອະນຸມັດຈາກຄະນະກຳມະການຄຸ້ມຄອງຫຼັກຊັບ".to_string(),
                message_en: "A public offering requires approval by the Lao SEC".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate that an offering's prospectus is present and complete.
/// ກວດສອບໜັງສືຊີ້ຊວນ
///
/// The prospectus must exist and disclose full and accurate information.
pub fn validate_prospectus(offering: &PublicOffering) -> SecuritiesResult<()> {
    if !offering.has_prospectus {
        return Err(SecuritiesLawError::IncompleteProspectus {
            provision: "public offering — prospectus requirement",
            message_lao: "ການສະເໜີຂາຍຕ້ອງມີໜັງສືຊີ້ຊວນ".to_string(),
            message_en: "The offering must be accompanied by a prospectus".to_string(),
        });
    }

    if !offering.prospectus_complete {
        return Err(SecuritiesLawError::IncompleteProspectus {
            provision: "public offering — full and accurate disclosure",
            message_lao: "ໜັງສືຊີ້ຊວນຕ້ອງມີຂໍ້ມູນຄົບຖ້ວນ ແລະ ຖືກຕ້ອງ".to_string(),
            message_en: "The prospectus must contain full and accurate disclosure".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Listing Validators - ການກວດສອບການຈົດທະບຽນ
// ============================================================================

/// Validate that a company meets the listing requirements.
/// ກວດສອບເງື່ອນໄຂການຈົດທະບຽນ
///
/// A listed company must have at least the minimum public float and must keep its
/// periodic financial reporting current (continuous disclosure).
pub fn validate_listing(company: &ListedCompany) -> SecuritiesResult<()> {
    if company.public_float_percent < MIN_PUBLIC_FLOAT_PERCENT {
        return Err(SecuritiesLawError::ListingRequirementNotMet {
            provision: "listing — minimum public float",
            message_lao: format!("ສັດສ່ວນຮຸ້ນສ່ວນສາທາລະນະຕ້ອງບໍ່ໜ້ອຍກວ່າ {}%", MIN_PUBLIC_FLOAT_PERCENT),
            message_en: format!(
                "The public float must be at least {}%",
                MIN_PUBLIC_FLOAT_PERCENT
            ),
        });
    }

    if !company.financial_reports_current {
        return Err(SecuritiesLawError::ListingRequirementNotMet {
            provision: "listing — continuous disclosure and periodic reporting",
            message_lao: "ບໍລິສັດຈົດທະບຽນຕ້ອງລາຍງານການເງິນເປັນປະຈຸບັນ".to_string(),
            message_en: "A listed company must keep its periodic financial reporting current"
                .to_string(),
        });
    }

    Ok(())
}

/// Validate that foreign ownership of a listed company is within the cap.
/// ກວດສອບການຖືຄອງຮຸ້ນຂອງນັກລົງທຶນຕ່າງປະເທດ
pub fn validate_foreign_ownership(company: &ListedCompany) -> SecuritiesResult<()> {
    if company.foreign_ownership_percent > FOREIGN_OWNERSHIP_LIMIT_PERCENT {
        return Err(SecuritiesLawError::ForeignOwnershipExceeded {
            message_lao: format!(
                "ການຖືຄອງຮຸ້ນຂອງນັກລົງທຶນຕ່າງປະເທດຕ້ອງບໍ່ເກີນ {}%",
                FOREIGN_OWNERSHIP_LIMIT_PERCENT
            ),
            message_en: format!(
                "Foreign ownership must not exceed {}%",
                FOREIGN_OWNERSHIP_LIMIT_PERCENT
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Securities Company Validators - ການກວດສອບບໍລິສັດຫຼັກຊັບ
// ============================================================================

/// Validate that a securities company is licensed and adequately capitalised.
/// ກວດສອບໃບອະນຸຍາດ ແລະ ທຶນຈົດທະບຽນຂອງບໍລິສັດຫຼັກຊັບ
pub fn validate_securities_company_license(company: &SecuritiesCompany) -> SecuritiesResult<()> {
    if !company.licensed {
        return Err(SecuritiesLawError::UnlicensedSecuritiesCompany {
            provision: "securities intermediary — licensing",
            message_lao: format!(
                "{} ຕ້ອງໄດ້ຮັບໃບອະນຸຍາດຈາກຄະນະກຳມະການຄຸ້ມຄອງຫຼັກຊັບ",
                company.participant_type.lao_name()
            ),
            message_en: format!(
                "A {} must be licensed by the Lao SEC",
                company.participant_type.english_name()
            ),
        });
    }

    if company.registered_capital_lak == 0 {
        return Err(SecuritiesLawError::UnlicensedSecuritiesCompany {
            provision: "securities intermediary — minimum capital",
            message_lao: "ບໍລິສັດຫຼັກຊັບຕ້ອງມີທຶນຈົດທະບຽນພຽງພໍ".to_string(),
            message_en: "A securities company must be adequately capitalised".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Prohibited Conduct Validators - ການກວດສອບການກະທຳທີ່ຖືກຫ້າມ
// ============================================================================

/// Validate a securities trade for prohibited conduct.
/// ກວດສອບການຊື້ຂາຍຫຼັກຊັບ
///
/// Trading on material non-public information (insider trading) and manipulative
/// trading are both prohibited.
pub fn validate_trade(trade: &SecuritiesTrade) -> SecuritiesResult<()> {
    if trade.used_inside_information {
        return Err(SecuritiesLawError::InsiderTrading {
            provision: "prohibited conduct — insider trading",
            message_lao: "ຫ້າມຊື້ຂາຍຫຼັກຊັບໂດຍໃຊ້ຂໍ້ມູນພາຍໃນທີ່ສຳຄັນ".to_string(),
            message_en: "Trading on material non-public information is prohibited".to_string(),
        });
    }

    if trade.manipulative {
        return Err(SecuritiesLawError::MarketManipulation {
            provision: "prohibited conduct — market manipulation",
            message_lao: "ຫ້າມການກະທຳທີ່ປັ່ນປ່ວນຕະຫຼາດຫຼັກຊັບ".to_string(),
            message_en: "Market manipulation is prohibited".to_string(),
        });
    }

    Ok(())
}

/// Validate a securities trade specifically for insider trading.
/// ກວດສອບການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນ
pub fn validate_insider_trading(trade: &SecuritiesTrade) -> SecuritiesResult<()> {
    if trade.used_inside_information {
        return Err(SecuritiesLawError::InsiderTrading {
            provision: "prohibited conduct — insider trading",
            message_lao: "ການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນທີ່ສຳຄັນຖືກຫ້າມ".to_string(),
            message_en: "Trading on material non-public information constitutes insider trading"
                .to_string(),
        });
    }

    Ok(())
}

/// Validate a securities trade specifically for market manipulation.
/// ກວດສອບການປັ່ນປ່ວນຕະຫຼາດ
pub fn validate_market_manipulation(trade: &SecuritiesTrade) -> SecuritiesResult<()> {
    if trade.manipulative {
        return Err(SecuritiesLawError::MarketManipulation {
            provision: "prohibited conduct — market manipulation",
            message_lao: "ການກະທຳທີ່ປັ່ນປ່ວນຕະຫຼາດຖືກຫ້າມ".to_string(),
            message_en: "Manipulative trading conduct is prohibited".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Disclosure Validators - ການກວດສອບການເປີດເຜີຍຂໍ້ມູນ
// ============================================================================

/// Validate a disclosure event under the continuous-disclosure obligation.
/// ກວດສອບການເປີດເຜີຍຂໍ້ມູນ
///
/// The event must carry a description. A material event must be disclosed within
/// the disclosure deadline ([`MATERIAL_DISCLOSURE_DEADLINE_DAYS`]).
pub fn validate_disclosure(event: &DisclosureEvent) -> SecuritiesResult<()> {
    if event.description.trim().is_empty() {
        return Err(SecuritiesLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸລາຍລະອຽດຂອງເຫດການ".to_string(),
            message_en: "The disclosure event must include a description".to_string(),
        });
    }

    if event.material && !event.disclosed_within_deadline {
        return Err(SecuritiesLawError::DisclosureViolation {
            provision: "continuous disclosure — material information",
            message_lao: format!(
                "ຕ້ອງເປີດເຜີຍຂໍ້ມູນທີ່ສຳຄັນພາຍໃນ {} ມື້",
                MATERIAL_DISCLOSURE_DEADLINE_DAYS
            ),
            message_en: format!(
                "Material information must be disclosed within {} days",
                MATERIAL_DISCLOSURE_DEADLINE_DAYS
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compliant_offering() -> PublicOffering {
        PublicOffering {
            issuer: "Lao Brewery Co.".to_string(),
            offering_type: OfferingType::Ipo,
            has_prospectus: true,
            prospectus_complete: true,
            sec_approved: true,
            total_value_lak: 50_000_000_000,
        }
    }

    fn compliant_listing() -> ListedCompany {
        ListedCompany {
            name: "EDL-Generation".to_string(),
            public_float_percent: 20,
            foreign_ownership_percent: 5,
            financial_reports_current: true,
            status: ListingStatus::Listed,
        }
    }

    fn licensed_company() -> SecuritiesCompany {
        SecuritiesCompany {
            name: "Lao-China Securities".to_string(),
            participant_type: MarketParticipantType::BrokerDealer,
            licensed: true,
            registered_capital_lak: 30_000_000_000,
        }
    }

    fn clean_trade() -> SecuritiesTrade {
        SecuritiesTrade {
            security: SecurityType::OrdinaryShares,
            used_inside_information: false,
            manipulative: false,
        }
    }

    // ---- Public offering ----

    #[test]
    fn test_valid_public_offering_ok() {
        assert!(validate_public_offering(&compliant_offering()).is_ok());
    }

    #[test]
    fn test_public_offering_empty_issuer_fails() {
        let mut offering = compliant_offering();
        offering.issuer = String::new();
        assert!(matches!(
            validate_public_offering(&offering).unwrap_err(),
            SecuritiesLawError::InvalidPublicOffering { .. }
        ));
    }

    #[test]
    fn test_public_offering_zero_value_fails() {
        let mut offering = compliant_offering();
        offering.total_value_lak = 0;
        assert!(validate_public_offering(&offering).is_err());
    }

    #[test]
    fn test_public_offering_missing_prospectus_fails() {
        let mut offering = compliant_offering();
        offering.has_prospectus = false;
        assert!(matches!(
            validate_public_offering(&offering).unwrap_err(),
            SecuritiesLawError::IncompleteProspectus { .. }
        ));
    }

    #[test]
    fn test_public_offering_not_approved_fails() {
        let mut offering = compliant_offering();
        offering.sec_approved = false;
        assert!(matches!(
            validate_public_offering(&offering).unwrap_err(),
            SecuritiesLawError::OfferingNotApproved { .. }
        ));
    }

    #[test]
    fn test_private_placement_without_prospectus_ok() {
        let offering = PublicOffering {
            issuer: "Mekong Capital".to_string(),
            offering_type: OfferingType::PrivatePlacement,
            has_prospectus: false,
            prospectus_complete: false,
            sec_approved: false,
            total_value_lak: 10_000_000_000,
        };
        assert!(validate_public_offering(&offering).is_ok());
    }

    // ---- Prospectus ----

    #[test]
    fn test_validate_prospectus_ok() {
        assert!(validate_prospectus(&compliant_offering()).is_ok());
    }

    #[test]
    fn test_validate_prospectus_incomplete_fails() {
        let mut offering = compliant_offering();
        offering.prospectus_complete = false;
        assert!(validate_prospectus(&offering).is_err());
    }

    // ---- Listing ----

    #[test]
    fn test_valid_listing_ok() {
        assert!(validate_listing(&compliant_listing()).is_ok());
    }

    #[test]
    fn test_listing_below_min_float_fails() {
        let mut company = compliant_listing();
        company.public_float_percent = MIN_PUBLIC_FLOAT_PERCENT - 1;
        assert!(matches!(
            validate_listing(&company).unwrap_err(),
            SecuritiesLawError::ListingRequirementNotMet { .. }
        ));
    }

    #[test]
    fn test_listing_stale_reports_fails() {
        let mut company = compliant_listing();
        company.financial_reports_current = false;
        assert!(validate_listing(&company).is_err());
    }

    // ---- Foreign ownership ----

    #[test]
    fn test_foreign_ownership_ok() {
        assert!(validate_foreign_ownership(&compliant_listing()).is_ok());
    }

    #[test]
    fn test_foreign_ownership_exceeded_fails() {
        let mut company = compliant_listing();
        company.foreign_ownership_percent = FOREIGN_OWNERSHIP_LIMIT_PERCENT + 1;
        assert!(matches!(
            validate_foreign_ownership(&company).unwrap_err(),
            SecuritiesLawError::ForeignOwnershipExceeded { .. }
        ));
    }

    // ---- Securities company licensing ----

    #[test]
    fn test_licensed_company_ok() {
        assert!(validate_securities_company_license(&licensed_company()).is_ok());
    }

    #[test]
    fn test_unlicensed_company_fails() {
        let mut company = licensed_company();
        company.licensed = false;
        assert!(matches!(
            validate_securities_company_license(&company).unwrap_err(),
            SecuritiesLawError::UnlicensedSecuritiesCompany { .. }
        ));
    }

    #[test]
    fn test_company_zero_capital_fails() {
        let mut company = licensed_company();
        company.registered_capital_lak = 0;
        assert!(validate_securities_company_license(&company).is_err());
    }

    // ---- Trading / prohibited conduct ----

    #[test]
    fn test_clean_trade_ok() {
        assert!(validate_trade(&clean_trade()).is_ok());
    }

    #[test]
    fn test_insider_trade_fails() {
        let mut trade = clean_trade();
        trade.used_inside_information = true;
        assert!(matches!(
            validate_trade(&trade).unwrap_err(),
            SecuritiesLawError::InsiderTrading { .. }
        ));
    }

    #[test]
    fn test_manipulative_trade_fails() {
        let mut trade = clean_trade();
        trade.manipulative = true;
        assert!(matches!(
            validate_trade(&trade).unwrap_err(),
            SecuritiesLawError::MarketManipulation { .. }
        ));
    }

    #[test]
    fn test_validate_insider_trading_ok_and_fail() {
        assert!(validate_insider_trading(&clean_trade()).is_ok());
        let mut trade = clean_trade();
        trade.used_inside_information = true;
        assert!(validate_insider_trading(&trade).is_err());
    }

    #[test]
    fn test_validate_market_manipulation_ok_and_fail() {
        assert!(validate_market_manipulation(&clean_trade()).is_ok());
        let mut trade = clean_trade();
        trade.manipulative = true;
        assert!(validate_market_manipulation(&trade).is_err());
    }

    // ---- Disclosure ----

    #[test]
    fn test_disclosure_material_within_deadline_ok() {
        let event = DisclosureEvent {
            description: "Quarterly earnings release".to_string(),
            material: true,
            disclosed_within_deadline: true,
        };
        assert!(validate_disclosure(&event).is_ok());
    }

    #[test]
    fn test_disclosure_material_late_fails() {
        let event = DisclosureEvent {
            description: "Major acquisition".to_string(),
            material: true,
            disclosed_within_deadline: false,
        };
        assert!(matches!(
            validate_disclosure(&event).unwrap_err(),
            SecuritiesLawError::DisclosureViolation { .. }
        ));
    }

    #[test]
    fn test_disclosure_immaterial_late_ok() {
        let event = DisclosureEvent {
            description: "Minor office relocation".to_string(),
            material: false,
            disclosed_within_deadline: false,
        };
        assert!(validate_disclosure(&event).is_ok());
    }

    #[test]
    fn test_disclosure_empty_description_fails() {
        let event = DisclosureEvent {
            description: "   ".to_string(),
            material: true,
            disclosed_within_deadline: true,
        };
        assert!(matches!(
            validate_disclosure(&event).unwrap_err(),
            SecuritiesLawError::ValidationError { .. }
        ));
    }
}
