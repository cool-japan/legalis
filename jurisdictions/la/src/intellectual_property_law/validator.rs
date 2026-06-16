//! Intellectual Property Law Validators (ການກວດສອບກົດໝາຍຊັບສິນທາງປັນຍາ)
//!
//! Validation functions for Lao intellectual property law based on the
//! **Law on Intellectual Property (Lao PDR), No. 38/NA, 2017**.
//!
//! Each validator returns `Ok(())` on compliance, or an [`IpLawError`] carrying
//! bilingual messages and the governing statute citation.

use crate::intellectual_property_law::error::{IpLawError, IpResult};
use crate::intellectual_property_law::types::*;

// ============================================================================
// Patent Validators - ການກວດສອບສິດທິບັດ
// ============================================================================

/// Validate that an invention satisfies the patentability requirements.
/// ກວດສອບເງື່ອນໄຂການອອກສິດທິບັດ
///
/// Patentability requires all three of: novelty, an inventive step, and
/// industrial applicability.
pub fn validate_patentability(application: &PatentApplication) -> IpResult<()> {
    if application.title.trim().is_empty() {
        return Err(IpLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ການປະດິດ".to_string(),
            message_en: "The invention title is required".to_string(),
        });
    }

    if !application.is_novel {
        return Err(IpLawError::NotPatentable {
            provision: "novelty requirement",
            message_lao: "ການປະດິດບໍ່ມີຄວາມໃໝ່".to_string(),
            message_en: "The invention lacks novelty".to_string(),
        });
    }

    if !application.has_inventive_step {
        return Err(IpLawError::NotPatentable {
            provision: "inventive step requirement",
            message_lao: "ການປະດິດບໍ່ມີຂັ້ນຕອນການປະດິດສ້າງ".to_string(),
            message_en: "The invention lacks an inventive step".to_string(),
        });
    }

    if !application.is_industrially_applicable {
        return Err(IpLawError::NotPatentable {
            provision: "industrial applicability requirement",
            message_lao: "ການປະດິດບໍ່ສາມາດນຳໃຊ້ທາງອຸດສາຫະກຳໄດ້".to_string(),
            message_en: "The invention is not industrially applicable".to_string(),
        });
    }

    Ok(())
}

/// Validate that a patent is still within its term of protection.
/// ກວດສອບອາຍຸການປົກປ້ອງສິດທິບັດ
///
/// The patent term runs for [`PATENT_TERM_YEARS`] years from the filing year.
pub fn validate_patent_term(filing_year: u32, current_year: u32) -> IpResult<()> {
    if current_year < filing_year {
        return Err(IpLawError::ValidationError {
            message_lao: "ປີປັດຈຸບັນຕ້ອງບໍ່ກ່ອນປີທີ່ຍື່ນຄຳຮ້ອງ".to_string(),
            message_en: "Current year cannot precede the filing year".to_string(),
        });
    }

    let expiry_year = filing_year.saturating_add(PATENT_TERM_YEARS);
    if current_year > expiry_year {
        return Err(IpLawError::PatentExpired {
            message_lao: format!("ສິດທິບັດໝົດອາຍຸໃນປີ {} (ຍື່ນປີ {})", expiry_year, filing_year),
            message_en: format!(
                "The patent expired in {} ({}-year term from filing year {})",
                expiry_year, PATENT_TERM_YEARS, filing_year
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Trademark Validators - ການກວດສອບເຄື່ອງໝາຍການຄ້າ
// ============================================================================

/// Validate that a mark satisfies the registrability requirements.
/// ກວດສອບເງື່ອນໄຂການຈົດທະບຽນເຄື່ອງໝາຍການຄ້າ
///
/// A mark is registrable if it is distinctive, not deceptive/misleading, and
/// does not conflict with a prior registered mark.
pub fn validate_trademark_registrability(registration: &TrademarkRegistration) -> IpResult<()> {
    if registration.mark.trim().is_empty() {
        return Err(IpLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸເຄື່ອງໝາຍ".to_string(),
            message_en: "The mark is required".to_string(),
        });
    }

    if !registration.is_distinctive {
        return Err(IpLawError::TrademarkNotRegistrable {
            provision: "distinctiveness requirement",
            message_lao: "ເຄື່ອງໝາຍບໍ່ມີຄຸນລັກສະນະທີ່ໂດດເດັ່ນ".to_string(),
            message_en: "The mark is not distinctive".to_string(),
        });
    }

    if registration.is_deceptive {
        return Err(IpLawError::TrademarkNotRegistrable {
            provision: "prohibition on deceptive marks",
            message_lao: "ເຄື່ອງໝາຍຫຼອກລວງ ຫຼື ເຮັດໃຫ້ເຂົ້າໃຈຜິດ".to_string(),
            message_en: "The mark is deceptive or misleading".to_string(),
        });
    }

    if registration.conflicts_with_prior_mark {
        return Err(IpLawError::TrademarkNotRegistrable {
            provision: "conflict with a prior registered mark",
            message_lao: "ເຄື່ອງໝາຍຂັດກັບເຄື່ອງໝາຍທີ່ຈົດທະບຽນກ່ອນ".to_string(),
            message_en: "The mark conflicts with a prior registered mark".to_string(),
        });
    }

    Ok(())
}

/// Validate that a trademark registration is still in force (not lapsed).
/// ກວດສອບການຕໍ່ອາຍຸ ແລະ ສະຖານະຂອງເຄື່ອງໝາຍການຄ້າ
///
/// A registration is valid while the current year falls within
/// [`TRADEMARK_TERM_YEARS`] of the registration year extended by each completed
/// renewal period; once that term elapses without renewal the mark has lapsed.
pub fn validate_trademark_renewal(
    registration: &TrademarkRegistration,
    current_year: u32,
) -> IpResult<()> {
    if current_year < registration.registration_year {
        return Err(IpLawError::ValidationError {
            message_lao: "ປີປັດຈຸບັນຕ້ອງບໍ່ກ່ອນປີທີ່ຈົດທະບຽນ".to_string(),
            message_en: "Current year cannot precede the registration year".to_string(),
        });
    }

    let expiry_year = registration.expiry_year();
    if current_year > expiry_year {
        return Err(IpLawError::TrademarkLapsed {
            message_lao: format!(
                "ການຈົດທະບຽນເຄື່ອງໝາຍ '{}' ໝົດອາຍຸໃນປີ {}",
                registration.mark, expiry_year
            ),
            message_en: format!(
                "Trademark '{}' registration lapsed in {} and must be renewed",
                registration.mark, expiry_year
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Copyright Validators - ການກວດສອບລິຂະສິດ
// ============================================================================

/// Validate copyright protection of a work.
/// ກວດສອບການປົກປ້ອງລິຂະສິດ
///
/// Copyright subsists in original works without registration. Where the author
/// is deceased, the work remains protected until the year of death plus
/// [`COPYRIGHT_TERM_AFTER_DEATH_YEARS`]; thereafter it enters the public domain.
pub fn validate_copyright(work: &CopyrightWork) -> IpResult<()> {
    if work.title.trim().is_empty() {
        return Err(IpLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ຜົນງານ".to_string(),
            message_en: "The work title is required".to_string(),
        });
    }

    if !work.is_original {
        return Err(IpLawError::CopyrightNotProtected {
            provision: "originality requirement",
            message_lao: "ຜົນງານບໍ່ມີຄວາມເປັນຕົ້ນສະບັບ ຈຶ່ງບໍ່ໄດ້ຮັບການປົກປ້ອງລິຂະສິດ".to_string(),
            message_en: "The work is not original and does not attract copyright".to_string(),
        });
    }

    if let Some(public_domain_year) = work.public_domain_year()
        && work.current_year > public_domain_year
    {
        return Err(IpLawError::CopyrightExpired {
            message_lao: format!("ລິຂະສິດໝົດອາຍຸໃນປີ {} (ເຂົ້າສູ່ສາທາລະນະສົມບັດ)", public_domain_year),
            message_en: format!(
                "Copyright expired in {} (life of author + {} years); the work is in the public domain",
                public_domain_year, COPYRIGHT_TERM_AFTER_DEATH_YEARS
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Trade Secret Validators - ການກວດສອບຄວາມລັບທາງການຄ້າ
// ============================================================================

/// Validate that information qualifies for trade-secret protection.
/// ກວດສອບເງື່ອນໄຂການປົກປ້ອງຄວາມລັບທາງການຄ້າ
///
/// All three criteria must hold: the information is secret, has commercial value,
/// and the holder has taken reasonable steps to keep it secret.
pub fn validate_trade_secret(secret: &TradeSecret) -> IpResult<()> {
    if !secret.is_secret {
        return Err(IpLawError::InvalidTradeSecret {
            provision: "secrecy requirement",
            message_lao: "ຂໍ້ມູນບໍ່ເປັນຄວາມລັບ (ເປັນທີ່ຮູ້ກັນທົ່ວໄປ)".to_string(),
            message_en: "The information is not secret (it is generally known)".to_string(),
        });
    }

    if !secret.has_commercial_value {
        return Err(IpLawError::InvalidTradeSecret {
            provision: "commercial value requirement",
            message_lao: "ຂໍ້ມູນບໍ່ມີມູນຄ່າທາງການຄ້າ".to_string(),
            message_en: "The information has no commercial value".to_string(),
        });
    }

    if !secret.reasonable_protection_steps {
        return Err(IpLawError::InvalidTradeSecret {
            provision: "reasonable steps requirement",
            message_lao: "ຜູ້ຖືຄອງບໍ່ໄດ້ໃຊ້ມາດຕະການທີ່ສົມເຫດສົມຜົນເພື່ອຮັກສາຄວາມລັບ".to_string(),
            message_en: "The holder did not take reasonable steps to keep it secret".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Industrial Design Validators - ການກວດສອບແບບອຸດສາຫະກຳ
// ============================================================================

/// Validate that an industrial design is new / original.
/// ກວດສອບຄວາມໃໝ່ຂອງແບບອຸດສາຫະກຳ
pub fn validate_industrial_design(design: &IndustrialDesign) -> IpResult<()> {
    if design.title.trim().is_empty() {
        return Err(IpLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ແບບ".to_string(),
            message_en: "The design title is required".to_string(),
        });
    }

    if !design.is_new {
        return Err(IpLawError::IndustrialDesignNotNew {
            message_lao: format!("ແບບອຸດສາຫະກຳ '{}' ບໍ່ໃໝ່", design.title),
            message_en: format!("Industrial design '{}' is not new", design.title),
        });
    }

    Ok(())
}

// ============================================================================
// Geographical Indication Validators - ການກວດສອບສິ່ງບົ່ງຊີ້ທາງພູມສາດ
// ============================================================================

/// Validate that a geographical indication is registrable.
/// ກວດສອບເງື່ອນໄຂການຈົດທະບຽນສິ່ງບົ່ງຊີ້ທາງພູມສາດ
///
/// A GI is registrable where the product's quality or reputation is essentially
/// attributable to its geographical origin.
pub fn validate_geographical_indication(gi: &GeographicalIndication) -> IpResult<()> {
    if gi.name.trim().is_empty() || gi.region.trim().is_empty() {
        return Err(IpLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ ແລະ ເຂດແຫຼ່ງກຳເນີດ".to_string(),
            message_en: "Both the name and the region of origin are required".to_string(),
        });
    }

    if !gi.quality_linked_to_origin {
        return Err(IpLawError::InvalidGeographicalIndication {
            provision: "quality-origin link requirement",
            message_lao: format!(
                "ຄຸນນະພາບ/ຊື່ສຽງຂອງ '{}' ບໍ່ເຊື່ອມໂຍງກັບແຫຼ່ງກຳເນີດ '{}'",
                gi.name, gi.region
            ),
            message_en: format!(
                "The quality/reputation of '{}' is not attributable to its origin '{}'",
                gi.name, gi.region
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Plant Variety Validators - ການກວດສອບພັນພືດໃໝ່
// ============================================================================

/// Validate that a new plant variety satisfies novelty plus the DUS criteria.
/// ກວດສອບເງື່ອນໄຂພັນພືດໃໝ່ (ໃໝ່ + ແຕກຕ່າງ + ສະໝ່ຳສະເໝີ + ໝັ້ນຄົງ)
pub fn validate_plant_variety(variety: &PlantVariety) -> IpResult<()> {
    if variety.denomination.trim().is_empty() {
        return Err(IpLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ພັນພືດ".to_string(),
            message_en: "The variety denomination is required".to_string(),
        });
    }

    if !variety.is_new {
        return Err(IpLawError::PlantVarietyRequirementsNotMet {
            provision: "novelty requirement",
            message_lao: "ພັນພືດບໍ່ໃໝ່".to_string(),
            message_en: "The plant variety is not new".to_string(),
        });
    }

    if !variety.is_distinct {
        return Err(IpLawError::PlantVarietyRequirementsNotMet {
            provision: "distinctness requirement",
            message_lao: "ພັນພືດບໍ່ມີຄວາມແຕກຕ່າງ".to_string(),
            message_en: "The plant variety is not distinct".to_string(),
        });
    }

    if !variety.is_uniform {
        return Err(IpLawError::PlantVarietyRequirementsNotMet {
            provision: "uniformity requirement",
            message_lao: "ພັນພືດບໍ່ມີຄວາມສະໝ່ຳສະເໝີ".to_string(),
            message_en: "The plant variety is not uniform".to_string(),
        });
    }

    if !variety.is_stable {
        return Err(IpLawError::PlantVarietyRequirementsNotMet {
            provision: "stability requirement",
            message_lao: "ພັນພືດບໍ່ມີຄວາມໝັ້ນຄົງ".to_string(),
            message_en: "The plant variety is not stable".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Infringement Validators - ການກວດສອບການລະເມີດ
// ============================================================================

/// Validate the use of a protected IP right for infringement.
/// ກວດສອບການລະເມີດສິດຊັບສິນທາງປັນຍາ
///
/// Any unauthorised use of a protected right constitutes infringement.
pub fn validate_infringement(infringement: &IpInfringement) -> IpResult<()> {
    if !infringement.authorized {
        return Err(IpLawError::IpInfringement {
            right: infringement.right_type.english_name(),
            message_lao: format!(
                "ການນຳໃຊ້ {} ໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ: {}",
                infringement.right_type.lao_name(),
                infringement.description
            ),
            message_en: format!(
                "Unauthorised use of a protected {}: {}",
                infringement.right_type.english_name(),
                infringement.description
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_patent() -> PatentApplication {
        PatentApplication {
            title: "Solar-powered irrigation pump".to_string(),
            is_novel: true,
            has_inventive_step: true,
            is_industrially_applicable: true,
            filing_year: 2020,
        }
    }

    fn good_trademark() -> TrademarkRegistration {
        TrademarkRegistration {
            mark: "LaoSilk".to_string(),
            is_distinctive: true,
            is_deceptive: false,
            conflicts_with_prior_mark: false,
            registration_year: 2020,
            renewal_count: 0,
        }
    }

    fn good_copyright() -> CopyrightWork {
        CopyrightWork {
            title: "Lao folk anthology".to_string(),
            author: "Somchai".to_string(),
            is_original: true,
            author_death_year: None,
            current_year: 2025,
        }
    }

    fn good_trade_secret() -> TradeSecret {
        TradeSecret {
            description: "Beverage formulation".to_string(),
            is_secret: true,
            has_commercial_value: true,
            reasonable_protection_steps: true,
        }
    }

    fn good_plant_variety() -> PlantVariety {
        PlantVariety {
            denomination: "LaoJasmine-1".to_string(),
            is_new: true,
            is_distinct: true,
            is_uniform: true,
            is_stable: true,
        }
    }

    // -- Patentability --------------------------------------------------------

    #[test]
    fn test_patentability_ok() {
        assert!(validate_patentability(&good_patent()).is_ok());
    }

    #[test]
    fn test_patentability_not_novel_fails() {
        let mut app = good_patent();
        app.is_novel = false;
        assert!(matches!(
            validate_patentability(&app).unwrap_err(),
            IpLawError::NotPatentable { .. }
        ));
    }

    #[test]
    fn test_patentability_no_inventive_step_fails() {
        let mut app = good_patent();
        app.has_inventive_step = false;
        assert!(validate_patentability(&app).is_err());
    }

    #[test]
    fn test_patentability_not_applicable_fails() {
        let mut app = good_patent();
        app.is_industrially_applicable = false;
        assert!(validate_patentability(&app).is_err());
    }

    // -- Patent term ----------------------------------------------------------

    #[test]
    fn test_patent_term_ok() {
        assert!(validate_patent_term(2010, 2025).is_ok());
    }

    #[test]
    fn test_patent_term_expired_fails() {
        assert!(matches!(
            validate_patent_term(2000, 2025).unwrap_err(),
            IpLawError::PatentExpired { .. }
        ));
    }

    // -- Trademark registrability --------------------------------------------

    #[test]
    fn test_trademark_registrable_ok() {
        assert!(validate_trademark_registrability(&good_trademark()).is_ok());
    }

    #[test]
    fn test_trademark_not_distinctive_fails() {
        let mut mark = good_trademark();
        mark.is_distinctive = false;
        assert!(matches!(
            validate_trademark_registrability(&mark).unwrap_err(),
            IpLawError::TrademarkNotRegistrable { .. }
        ));
    }

    #[test]
    fn test_trademark_deceptive_fails() {
        let mut mark = good_trademark();
        mark.is_deceptive = true;
        assert!(validate_trademark_registrability(&mark).is_err());
    }

    #[test]
    fn test_trademark_conflict_fails() {
        let mut mark = good_trademark();
        mark.conflicts_with_prior_mark = true;
        assert!(validate_trademark_registrability(&mark).is_err());
    }

    // -- Trademark renewal ----------------------------------------------------

    #[test]
    fn test_trademark_renewal_ok() {
        assert!(validate_trademark_renewal(&good_trademark(), 2025).is_ok());
    }

    #[test]
    fn test_trademark_renewal_lapsed_fails() {
        let mut mark = good_trademark();
        mark.registration_year = 2000;
        mark.renewal_count = 0;
        assert!(matches!(
            validate_trademark_renewal(&mark, 2025).unwrap_err(),
            IpLawError::TrademarkLapsed { .. }
        ));
    }

    #[test]
    fn test_trademark_renewal_extends_term_ok() {
        let mut mark = good_trademark();
        mark.registration_year = 2000;
        mark.renewal_count = 2; // expiry 2000 + 3*10 = 2030
        assert!(validate_trademark_renewal(&mark, 2025).is_ok());
    }

    // -- Copyright ------------------------------------------------------------

    #[test]
    fn test_copyright_living_author_ok() {
        assert!(validate_copyright(&good_copyright()).is_ok());
    }

    #[test]
    fn test_copyright_not_original_fails() {
        let mut work = good_copyright();
        work.is_original = false;
        assert!(matches!(
            validate_copyright(&work).unwrap_err(),
            IpLawError::CopyrightNotProtected { .. }
        ));
    }

    #[test]
    fn test_copyright_within_term_ok() {
        let mut work = good_copyright();
        work.author_death_year = Some(1990); // expiry 2040
        assert!(validate_copyright(&work).is_ok());
    }

    #[test]
    fn test_copyright_expired_fails() {
        let mut work = good_copyright();
        work.author_death_year = Some(1950); // expiry 2000
        assert!(matches!(
            validate_copyright(&work).unwrap_err(),
            IpLawError::CopyrightExpired { .. }
        ));
    }

    // -- Trade secret ---------------------------------------------------------

    #[test]
    fn test_trade_secret_ok() {
        assert!(validate_trade_secret(&good_trade_secret()).is_ok());
    }

    #[test]
    fn test_trade_secret_not_secret_fails() {
        let mut secret = good_trade_secret();
        secret.is_secret = false;
        assert!(matches!(
            validate_trade_secret(&secret).unwrap_err(),
            IpLawError::InvalidTradeSecret { .. }
        ));
    }

    #[test]
    fn test_trade_secret_no_value_fails() {
        let mut secret = good_trade_secret();
        secret.has_commercial_value = false;
        assert!(validate_trade_secret(&secret).is_err());
    }

    #[test]
    fn test_trade_secret_no_steps_fails() {
        let mut secret = good_trade_secret();
        secret.reasonable_protection_steps = false;
        assert!(validate_trade_secret(&secret).is_err());
    }

    // -- Industrial design ----------------------------------------------------

    #[test]
    fn test_industrial_design_ok() {
        let design = IndustrialDesign {
            title: "Ergonomic kettle".to_string(),
            is_new: true,
            filing_year: 2022,
        };
        assert!(validate_industrial_design(&design).is_ok());
    }

    #[test]
    fn test_industrial_design_not_new_fails() {
        let design = IndustrialDesign {
            title: "Plain box".to_string(),
            is_new: false,
            filing_year: 2022,
        };
        assert!(matches!(
            validate_industrial_design(&design).unwrap_err(),
            IpLawError::IndustrialDesignNotNew { .. }
        ));
    }

    // -- Geographical indication ---------------------------------------------

    #[test]
    fn test_geographical_indication_ok() {
        let gi = GeographicalIndication {
            name: "Bolaven Coffee".to_string(),
            region: "Bolaven Plateau".to_string(),
            quality_linked_to_origin: true,
        };
        assert!(validate_geographical_indication(&gi).is_ok());
    }

    #[test]
    fn test_geographical_indication_no_link_fails() {
        let gi = GeographicalIndication {
            name: "Generic Coffee".to_string(),
            region: "Anywhere".to_string(),
            quality_linked_to_origin: false,
        };
        assert!(matches!(
            validate_geographical_indication(&gi).unwrap_err(),
            IpLawError::InvalidGeographicalIndication { .. }
        ));
    }

    // -- Plant variety --------------------------------------------------------

    #[test]
    fn test_plant_variety_ok() {
        assert!(validate_plant_variety(&good_plant_variety()).is_ok());
    }

    #[test]
    fn test_plant_variety_not_new_fails() {
        let mut variety = good_plant_variety();
        variety.is_new = false;
        assert!(matches!(
            validate_plant_variety(&variety).unwrap_err(),
            IpLawError::PlantVarietyRequirementsNotMet { .. }
        ));
    }

    #[test]
    fn test_plant_variety_not_distinct_fails() {
        let mut variety = good_plant_variety();
        variety.is_distinct = false;
        assert!(validate_plant_variety(&variety).is_err());
    }

    #[test]
    fn test_plant_variety_not_stable_fails() {
        let mut variety = good_plant_variety();
        variety.is_stable = false;
        assert!(validate_plant_variety(&variety).is_err());
    }

    // -- Infringement ---------------------------------------------------------

    #[test]
    fn test_infringement_unauthorized_fails() {
        let infringement = IpInfringement {
            right_type: IpRightType::Patent,
            authorized: false,
            description: "Manufacturing the patented pump without a licence".to_string(),
        };
        assert!(matches!(
            validate_infringement(&infringement).unwrap_err(),
            IpLawError::IpInfringement { .. }
        ));
    }

    #[test]
    fn test_infringement_authorized_ok() {
        let infringement = IpInfringement {
            right_type: IpRightType::Trademark,
            authorized: true,
            description: "Licensed use of the mark".to_string(),
        };
        assert!(validate_infringement(&infringement).is_ok());
    }
}
