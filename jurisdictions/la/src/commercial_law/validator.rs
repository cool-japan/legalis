//! Commercial Law Validators
//!
//! This module provides validation functions for commercial law compliance,
//! including enterprise formation, investment regulations, and IP registration.

use super::error::{CommercialLawError, Result};
use super::types::*;

/// Validate enterprise formation according to Enterprise Law 2013
pub fn validate_enterprise_formation(
    enterprise_type: &EnterpriseType,
    name_en: &str,
    name_lo: &str,
    capital: u64,
    shareholders: Option<&[Shareholder]>,
) -> Result<()> {
    // Validate business name
    validate_business_name(name_en, name_lo)?;

    // Validate capital requirements
    validate_capital_requirements(enterprise_type, capital)?;

    // Validate shareholders for company types
    match enterprise_type {
        EnterpriseType::LimitedCompany => {
            if let Some(shs) = shareholders {
                validate_limited_company_shareholders(shs)?;
            } else {
                return Err(CommercialLawError::invalid_enterprise_formation(
                    "Limited company must have shareholders",
                    "ບໍລິສັດຈໍາກັດຕ້ອງມີຜູ້ຖືຮຸ້ນ",
                ));
            }
        }
        EnterpriseType::PublicCompany => {
            if let Some(shs) = shareholders {
                validate_public_company_shareholders(shs)?;
            } else {
                return Err(CommercialLawError::invalid_enterprise_formation(
                    "Public company must have shareholders",
                    "ບໍລິສັດມະຫາຊົນຕ້ອງມີຜູ້ຖືຮຸ້ນ",
                ));
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate capital requirements by enterprise type
pub fn validate_capital_requirements(enterprise_type: &EnterpriseType, capital: u64) -> Result<()> {
    if let Some(min_capital) = CapitalRequirements::get_minimum_capital(enterprise_type)
        && capital < min_capital
    {
        return Err(CommercialLawError::insufficient_capital(
            format!(
                "Capital {} LAK is below minimum requirement of {} LAK",
                capital, min_capital
            ),
            format!("ທຶນ {} ກີບ ຕ່ໍາກວ່າຄວາມຕ້ອງການຂັ້ນຕ່ໍາ {} ກີບ", capital, min_capital),
        ));
    }

    Ok(())
}

/// Validate business name
pub fn validate_business_name(name_en: &str, name_lo: &str) -> Result<()> {
    // English name validation
    if name_en.is_empty() {
        return Err(CommercialLawError::invalid_business_name(
            "English business name cannot be empty",
            "ຊື່ທຸລະກິດພາສາອັງກິດບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if name_en.len() < 3 {
        return Err(CommercialLawError::invalid_business_name(
            "English business name must be at least 3 characters",
            "ຊື່ທຸລະກິດພາສາອັງກິດຕ້ອງມີຢ່າງໜ້ອຍ 3 ຕົວອັກສອນ",
        ));
    }

    // Lao name validation
    if name_lo.is_empty() {
        return Err(CommercialLawError::invalid_business_name(
            "Lao business name cannot be empty",
            "ຊື່ທຸລະກິດພາສາລາວບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    // Check for prohibited words
    let prohibited_words = [
        "government",
        "ລັດຖະບານ",
        "ministry",
        "ກະຊວງ",
        "national",
        "ແຫ່ງຊາດ",
    ];
    for word in &prohibited_words {
        if name_en.to_lowercase().contains(&word.to_lowercase()) || name_lo.contains(word) {
            return Err(CommercialLawError::invalid_business_name(
                format!("Business name cannot contain prohibited word: {}", word),
                format!("ຊື່ທຸລະກິດບໍ່ສາມາດມີຄໍາຫ້າມ: {}", word),
            ));
        }
    }

    Ok(())
}

/// Validate limited company shareholders (1-30 shareholders)
fn validate_limited_company_shareholders(shareholders: &[Shareholder]) -> Result<()> {
    if shareholders.is_empty() {
        return Err(CommercialLawError::shareholder_error(
            "Limited company must have at least 1 shareholder",
            "ບໍລິສັດຈໍາກັດຕ້ອງມີຢ່າງໜ້ອຍ 1 ຜູ້ຖືຮຸ້ນ",
        ));
    }

    if shareholders.len() > 30 {
        return Err(CommercialLawError::shareholder_error(
            "Limited company cannot have more than 30 shareholders",
            "ບໍລິສັດຈໍາກັດບໍ່ສາມາດມີຜູ້ຖືຮຸ້ນເກີນ 30 ຄົນ",
        ));
    }

    // Validate ownership percentages sum to 100%
    let total_ownership: f64 = shareholders.iter().map(|s| s.ownership_percentage).sum();
    if (total_ownership - 100.0).abs() > 0.01 {
        return Err(CommercialLawError::shareholder_error(
            format!(
                "Total ownership must equal 100%, got {:.2}%",
                total_ownership
            ),
            format!("ສ່ວນແບ່ງການຖືຮຸ້ນທັງໝົດຕ້ອງເທົ່າກັບ 100%, ໄດ້ {:.2}%", total_ownership),
        ));
    }

    Ok(())
}

/// Validate public company shareholders (minimum 15 shareholders)
fn validate_public_company_shareholders(shareholders: &[Shareholder]) -> Result<()> {
    if shareholders.len() < 15 {
        return Err(CommercialLawError::shareholder_error(
            "Public company must have at least 15 shareholders",
            "ບໍລິສັດມະຫາຊົນຕ້ອງມີຢ່າງໜ້ອຍ 15 ຜູ້ຖືຮຸ້ນ",
        ));
    }

    // Validate ownership percentages sum to 100%
    let total_ownership: f64 = shareholders.iter().map(|s| s.ownership_percentage).sum();
    if (total_ownership - 100.0).abs() > 0.01 {
        return Err(CommercialLawError::shareholder_error(
            format!(
                "Total ownership must equal 100%, got {:.2}%",
                total_ownership
            ),
            format!("ສ່ວນແບ່ງການຖືຮຸ້ນທັງໝົດຕ້ອງເທົ່າກັບ 100%, ໄດ້ {:.2}%", total_ownership),
        ));
    }

    Ok(())
}

/// Validate board of directors composition
pub fn validate_board_composition(board: &BoardOfDirectors) -> Result<()> {
    // Minimum 3 directors
    if board.directors.len() < 3 {
        return Err(CommercialLawError::invalid_board_composition(
            "Board must have at least 3 directors",
            "ຄະນະກໍາມະການຕ້ອງມີຢ່າງໜ້ອຍ 3 ກໍາມະການ",
        ));
    }

    // Maximum 15 directors (best practice)
    if board.directors.len() > 15 {
        return Err(CommercialLawError::invalid_board_composition(
            "Board should not have more than 15 directors",
            "ຄະນະກໍາມະການບໍ່ຄວນມີກໍາມະການເກີນ 15 ຄົນ",
        ));
    }

    // Must have exactly one chairperson
    let chairpersons = board
        .directors
        .iter()
        .filter(|d| d.position == DirectorPosition::Chairperson)
        .count();
    if chairpersons != 1 {
        return Err(CommercialLawError::invalid_board_composition(
            format!(
                "Board must have exactly 1 chairperson, found {}",
                chairpersons
            ),
            format!("ຄະນະກໍາມະການຕ້ອງມີປະທານພຽງ 1 ຄົນ, ພົບ {} ຄົນ", chairpersons),
        ));
    }

    // At least one managing director
    let managing_directors = board
        .directors
        .iter()
        .filter(|d| d.position == DirectorPosition::ManagingDirector)
        .count();
    if managing_directors == 0 {
        return Err(CommercialLawError::invalid_board_composition(
            "Board must have at least 1 managing director",
            "ຄະນະກໍາມະການຕ້ອງມີຢ່າງໜ້ອຍ 1 ຜູ້ຈັດການ",
        ));
    }

    // Minimum 1 board meeting per year
    if board.meetings_per_year < 1 {
        return Err(CommercialLawError::corporate_governance_violation(
            "Board must hold at least 1 meeting per year",
            "ຄະນະກໍາມະການຕ້ອງຈັດກອງປະຊຸມຢ່າງໜ້ອຍປີລະ 1 ຄັ້ງ",
        ));
    }

    Ok(())
}

/// Validate foreign investment compliance
pub fn validate_foreign_investment(investment: &ForeignInvestment) -> Result<()> {
    // Check restricted sectors
    validate_restricted_sector(&investment.sector, investment.foreign_ownership_percentage)?;

    // Validate foreign ownership percentage
    if investment.foreign_ownership_percentage < 0.0
        || investment.foreign_ownership_percentage > 100.0
    {
        return Err(CommercialLawError::foreign_investment_violation(
            "Foreign ownership percentage must be between 0% and 100%",
            "ສ່ວນແບ່ງການຖືຮຸ້ນຂອງຕ່າງປະເທດຕ້ອງຢູ່ລະຫວ່າງ 0% ແລະ 100%",
        ));
    }

    // Check if approval is required for large investments (>= 2 billion LAK)
    if investment.investment_amount >= 2_000_000_000 && !investment.requires_approval {
        return Err(CommercialLawError::investment_approval_required(
            "Investments >= 2 billion LAK require government approval",
            "ການລົງທຶນ >= 2 ພັນລ້ານກີບ ຕ້ອງການການອະນຸມັດຈາກລັດຖະບານ",
        ));
    }

    // If approval required, check status
    if investment.requires_approval && investment.approval_status.is_none() {
        return Err(CommercialLawError::investment_approval_required(
            "Approval status must be specified for investments requiring approval",
            "ຕ້ອງລະບຸສະຖານະການອະນຸມັດສໍາລັບການລົງທຶນທີ່ຕ້ອງການການອະນຸມັດ",
        ));
    }

    // Validate concession if present
    if let Some(concession) = &investment.concession {
        validate_concession(concession)?;
    }

    Ok(())
}

/// Validate restricted sector compliance
pub fn validate_restricted_sector(
    sector: &BusinessSector,
    foreign_ownership_percentage: f64,
) -> Result<()> {
    match sector {
        // Banking, Insurance, Telecom: max 49% foreign ownership
        BusinessSector::Finance | BusinessSector::Telecommunications => {
            if foreign_ownership_percentage > 49.0 {
                return Err(CommercialLawError::restricted_sector_violation(
                    format!(
                        "{:?} sector allows maximum 49% foreign ownership, got {:.2}%",
                        sector, foreign_ownership_percentage
                    ),
                    format!(
                        "ຂະແໜງ {:?} ອະນຸຍາດການຖືຮຸ້ນຂອງຕ່າງປະເທດສູງສຸດ 49%, ໄດ້ {:.2}%",
                        sector, foreign_ownership_percentage
                    ),
                ));
            }
        }

        // Education and Healthcare: require special approval
        BusinessSector::Education | BusinessSector::Healthcare => {
            if foreign_ownership_percentage > 0.0 {
                // Special approval required - checked in validate_foreign_investment
            }
        }

        // Other sectors: generally open to foreign investment
        _ => {}
    }

    Ok(())
}

/// Validate concession agreement
fn validate_concession(concession: &Concession) -> Result<()> {
    // Duration must be reasonable (typically 10-99 years)
    if concession.duration_years < 10 {
        return Err(CommercialLawError::concession_error(
            "Concession duration must be at least 10 years",
            "ໄລຍະເວລາສໍາປະທານຕ້ອງມີຢ່າງໜ້ອຍ 10 ປີ",
        ));
    }

    if concession.duration_years > 99 {
        return Err(CommercialLawError::concession_error(
            "Concession duration cannot exceed 99 years",
            "ໄລຍະເວລາສໍາປະທານບໍ່ສາມາດເກີນ 99 ປີ",
        ));
    }

    // Royalty rate must be reasonable (0-100%)
    if concession.royalty_rate < 0.0 || concession.royalty_rate > 100.0 {
        return Err(CommercialLawError::concession_error(
            "Royalty rate must be between 0% and 100%",
            "ອັດຕາຄ່າລິຂະສິດຕ້ອງຢູ່ລະຫວ່າງ 0% ແລະ 100%",
        ));
    }

    // For land concessions, validate area
    if let Some(area) = concession.area_hectares {
        if area <= 0.0 {
            return Err(CommercialLawError::concession_error(
                "Concession area must be positive",
                "ພື້ນທີ່ສໍາປະທານຕ້ອງເປັນບວກ",
            ));
        }

        // Large land concessions (> 10,000 ha) require National Assembly approval
        if area > 10_000.0 {
            // This should be checked elsewhere, but we can warn here
        }
    }

    Ok(())
}

/// Validate patent application
pub fn validate_patent(patent: &Patent) -> Result<()> {
    if patent.title_en.is_empty() || patent.title_lo.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Patent title in both English and Lao is required",
            "ຕ້ອງມີຊື່ສິດທິບັດທັງພາສາອັງກິດແລະລາວ",
        ));
    }

    if patent.inventors.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Patent must have at least one inventor",
            "ສິດທິບັດຕ້ອງມີຢ່າງໜ້ອຍ 1 ຜູ້ປະດິດ",
        ));
    }

    if patent.applicant.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Patent applicant must be specified",
            "ຕ້ອງລະບຸຜູ້ສະຫມັກສິດທິບັດ",
        ));
    }

    if patent.description.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Patent description cannot be empty",
            "ຄໍາອະທິບາຍສິດທິບັດບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    Ok(())
}

/// Validate trademark application
pub fn validate_trademark(trademark: &Trademark) -> Result<()> {
    // Must have either text or logo
    if trademark.trademark_text.is_none() && trademark.trademark_logo.is_none() {
        return Err(CommercialLawError::intellectual_property_error(
            "Trademark must have either text or logo",
            "ເຄື່ອງໝາຍການຄ້າຕ້ອງມີຂໍ້ຄວາມຫຼືໂລໂກ້",
        ));
    }

    if trademark.owner.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Trademark owner must be specified",
            "ຕ້ອງລະບຸເຈົ້າຂອງເຄື່ອງໝາຍການຄ້າ",
        ));
    }

    // Must have at least one Nice classification class
    if trademark.classes.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Trademark must have at least one Nice classification class",
            "ເຄື່ອງໝາຍການຄ້າຕ້ອງມີຢ່າງໜ້ອຍ 1 ປະເພດການຈັດແບ່ງ Nice",
        ));
    }

    // Validate Nice classes (1-45)
    for class in &trademark.classes {
        if *class < 1 || *class > 45 {
            return Err(CommercialLawError::intellectual_property_error(
                format!("Invalid Nice classification class: {}", class),
                format!("ປະເພດການຈັດແບ່ງ Nice ບໍ່ຖືກຕ້ອງ: {}", class),
            ));
        }
    }

    Ok(())
}

/// Validate copyright registration
pub fn validate_copyright(copyright: &Copyright) -> Result<()> {
    if copyright.title_en.is_empty() || copyright.title_lo.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Copyright work title in both English and Lao is required",
            "ຕ້ອງມີຊື່ງານລິຂະສິດທັງພາສາອັງກິດແລະລາວ",
        ));
    }

    if copyright.authors.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Copyright must have at least one author",
            "ລິຂະສິດຕ້ອງມີຢ່າງໜ້ອຍ 1 ຜູ້ຂຽນ",
        ));
    }

    if copyright.holder.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Copyright holder must be specified",
            "ຕ້ອງລະບຸຜູ້ຖືລິຂະສິດ",
        ));
    }

    Ok(())
}

/// Validate industrial design registration
pub fn validate_industrial_design(design: &IndustrialDesign) -> Result<()> {
    if design.title_en.is_empty() || design.title_lo.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Industrial design title in both English and Lao is required",
            "ຕ້ອງມີຊື່ແບບອຸດສາຫະກໍາທັງພາສາອັງກິດແລະລາວ",
        ));
    }

    if design.designers.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Industrial design must have at least one designer",
            "ແບບອຸດສາຫະກໍາຕ້ອງມີຢ່າງໜ້ອຍ 1 ຜູ້ອອກແບບ",
        ));
    }

    if design.applicant.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Industrial design applicant must be specified",
            "ຕ້ອງລະບຸຜູ້ສະຫມັກແບບອຸດສາຫະກໍາ",
        ));
    }

    if design.description.is_empty() {
        return Err(CommercialLawError::intellectual_property_error(
            "Industrial design description cannot be empty",
            "ຄໍາອະທິບາຍແບບອຸດສາຫະກໍາບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    Ok(())
}

/// Validate IP registration based on type
pub fn validate_ip_registration(ip: &IntellectualProperty) -> Result<()> {
    match ip {
        IntellectualProperty::Patent(patent) => validate_patent(patent),
        IntellectualProperty::Trademark(trademark) => validate_trademark(trademark),
        IntellectualProperty::Copyright(copyright) => validate_copyright(copyright),
        IntellectualProperty::IndustrialDesign(design) => validate_industrial_design(design),
    }
}

/// Validate partnership structure
pub fn validate_partnership(partnership: &Partnership) -> Result<()> {
    // Must have at least 2 partners
    if partnership.partners.len() < 2 {
        return Err(CommercialLawError::partnership_error(
            "Partnership must have at least 2 partners",
            "ຫ້າງຫຸ້ນສ່ວນຕ້ອງມີຢ່າງໜ້ອຍ 2 ຫຸ້ນສ່ວນ",
        ));
    }

    // For limited partnership, must have at least 1 general partner
    if partnership.partnership_type == PartnershipType::Limited {
        let general_partners = partnership
            .partners
            .iter()
            .filter(|p| p.partner_type == PartnerType::General)
            .count();
        if general_partners == 0 {
            return Err(CommercialLawError::partnership_error(
                "Limited partnership must have at least 1 general partner",
                "ຫ້າງຫຸ້ນສ່ວນຈໍາກັດຕ້ອງມີຢ່າງໜ້ອຍ 1 ຫຸ້ນສ່ວນທົ່ວໄປ",
            ));
        }
    }

    // Validate total ownership adds up to 100%
    let total_ownership: f64 = partnership
        .partners
        .iter()
        .map(|p| p.ownership_percentage)
        .sum();
    if (total_ownership - 100.0).abs() > 0.01 {
        return Err(CommercialLawError::partnership_error(
            format!(
                "Total ownership must equal 100%, got {:.2}%",
                total_ownership
            ),
            format!(
                "ສ່ວນແບ່ງການເປັນເຈົ້າຂອງທັງໝົດຕ້ອງເທົ່າກັບ 100%, ໄດ້ {:.2}%",
                total_ownership
            ),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_capital_requirements_limited_company() {
        let result = validate_capital_requirements(&EnterpriseType::LimitedCompany, 100_000_000);
        assert!(result.is_ok());

        let result = validate_capital_requirements(&EnterpriseType::LimitedCompany, 10_000_000);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_business_name_valid() {
        let result = validate_business_name("ABC Company Limited", "ບໍລິສັດ ABC ຈໍາກັດ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_business_name_prohibited_words() {
        let result = validate_business_name("Government Services Ltd", "ບໍລິການ");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_board_composition_valid() {
        let board = BoardOfDirectors {
            directors: vec![
                Director {
                    name: "John Doe".to_string(),
                    id: "P123".to_string(),
                    position: DirectorPosition::Chairperson,
                    nationality: "LAO".to_string(),
                    is_foreign: false,
                    appointed_at: Utc::now(),
                },
                Director {
                    name: "Jane Smith".to_string(),
                    id: "P456".to_string(),
                    position: DirectorPosition::ManagingDirector,
                    nationality: "LAO".to_string(),
                    is_foreign: false,
                    appointed_at: Utc::now(),
                },
                Director {
                    name: "Bob Johnson".to_string(),
                    id: "P789".to_string(),
                    position: DirectorPosition::Director,
                    nationality: "LAO".to_string(),
                    is_foreign: false,
                    appointed_at: Utc::now(),
                },
            ],
            meetings_per_year: 4,
            last_meeting: Some(Utc::now()),
        };

        assert!(validate_board_composition(&board).is_ok());
    }

    #[test]
    fn test_validate_board_composition_too_few_directors() {
        let board = BoardOfDirectors {
            directors: vec![Director {
                name: "John Doe".to_string(),
                id: "P123".to_string(),
                position: DirectorPosition::Chairperson,
                nationality: "LAO".to_string(),
                is_foreign: false,
                appointed_at: Utc::now(),
            }],
            meetings_per_year: 1,
            last_meeting: None,
        };

        assert!(validate_board_composition(&board).is_err());
    }

    #[test]
    fn test_validate_restricted_sector_finance() {
        let result = validate_restricted_sector(&BusinessSector::Finance, 30.0);
        assert!(result.is_ok());

        let result = validate_restricted_sector(&BusinessSector::Finance, 60.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_trademark_valid() {
        let trademark = Trademark {
            trademark_text: Some("ABC Brand".to_string()),
            trademark_logo: None,
            owner: "ABC Company".to_string(),
            application_number: "TM-2024-001".to_string(),
            application_date: Utc::now(),
            registration_date: None,
            registration_number: None,
            classes: vec![25, 35],
            expiry_date: Utc::now(),
            status: IPStatus::Pending,
        };

        assert!(validate_trademark(&trademark).is_ok());
    }

    #[test]
    fn test_validate_trademark_invalid_class() {
        let trademark = Trademark {
            trademark_text: Some("ABC Brand".to_string()),
            trademark_logo: None,
            owner: "ABC Company".to_string(),
            application_number: "TM-2024-001".to_string(),
            application_date: Utc::now(),
            registration_date: None,
            registration_number: None,
            classes: vec![50], // Invalid class
            expiry_date: Utc::now(),
            status: IPStatus::Pending,
        };

        assert!(validate_trademark(&trademark).is_err());
    }
}
