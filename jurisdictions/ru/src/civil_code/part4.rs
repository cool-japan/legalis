//! Civil Code Part 4: Intellectual Property Rights (2006).
//!
//! Federal Law No. 231-FZ of December 18, 2006
//!
//! This part covers:
//! - General provisions on intellectual property (Articles 1225-1254)
//! - Copyright and related rights (Articles 1255-1302)
//! - Patent law (Articles 1345-1407)
//! - Trademark law (Articles 1477-1515)
//! - Trade secrets (Articles 1465-1472)

use serde::{Deserialize, Serialize};

use super::CivilCodeError;

/// Types of intellectual property objects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntellectualPropertyRight {
    /// Copyright (авторское право)
    Copyright,
    /// Related rights (смежные права)
    RelatedRights,
    /// Patent (патент)
    Patent,
    /// Trademark (товарный знак)
    Trademark,
    /// Trade secret (коммерческая тайна)
    TradeSecret,
    /// Industrial design (промышленный образец)
    IndustrialDesign,
    /// Utility model (полезная модель)
    UtilityModel,
    /// Trade name (фирменное наименование)
    TradeName,
    /// Appellation of origin (наименование места происхождения товара)
    AppellationOfOrigin,
}

/// Types of copyrighted works
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkType {
    /// Literary work
    Literary,
    /// Musical work
    Musical,
    /// Dramatic work
    Dramatic,
    /// Choreographic work
    Choreographic,
    /// Audiovisual work
    Audiovisual,
    /// Painting, sculpture, graphic work
    ArtisticWork,
    /// Architectural work
    Architectural,
    /// Photographic work
    Photographic,
    /// Computer program
    ComputerProgram,
    /// Database
    Database,
    /// Derivative work
    Derivative,
    /// Composite work
    Composite,
}

/// Copyright holder representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyrightHolder {
    /// Name of copyright holder
    pub name: String,
    /// Is original author
    pub is_original_author: bool,
    /// Date of acquisition
    pub acquisition_date: chrono::NaiveDate,
    /// Rights held
    pub rights: Vec<CopyrightRight>,
}

/// Copyright rights under Article 1255-1270
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CopyrightRight {
    /// Right of authorship (право авторства)
    Authorship,
    /// Right to name (право автора на имя)
    Name,
    /// Right to inviolability (право на неприкосновенность произведения)
    Inviolability,
    /// Right to publication (право на обнародование)
    Publication,
    /// Exclusive right (исключительное право)
    ExclusiveRight,
    /// Right to remuneration (право на вознаграждение)
    Remuneration,
}

/// Patent representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patent {
    /// Patent number
    pub number: String,
    /// Patent holder
    pub holder: String,
    /// Inventor(s)
    pub inventors: Vec<String>,
    /// Date of filing
    pub filing_date: chrono::NaiveDate,
    /// Date of grant
    pub grant_date: Option<chrono::NaiveDate>,
    /// Patent type
    pub patent_type: PatentType,
    /// Claims
    pub claims: Vec<String>,
    /// Is valid
    pub is_valid: bool,
}

/// Types of patents under Russian law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatentType {
    /// Invention (изобретение) - 20 years
    Invention,
    /// Utility model (полезная модель) - 10 years
    UtilityModel,
    /// Industrial design (промышленный образец) - 5 years, renewable
    IndustrialDesign,
}

impl PatentType {
    /// Returns the protection period in years
    pub fn protection_period_years(&self) -> u32 {
        match self {
            Self::Invention => 20,
            Self::UtilityModel => 10,
            Self::IndustrialDesign => 5,
        }
    }
}

/// Trademark representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trademark {
    /// Registration number
    pub registration_number: String,
    /// Trademark holder
    pub holder: String,
    /// Registration date
    pub registration_date: chrono::NaiveDate,
    /// Trademark text/image description
    pub description: String,
    /// Classes of goods and services (Nice Classification)
    pub classes: Vec<u32>,
    /// Is valid
    pub is_valid: bool,
}

impl Trademark {
    /// Checks if the trademark protection is still valid
    pub fn is_protection_valid(&self, current_date: &chrono::NaiveDate) -> bool {
        if !self.is_valid {
            return false;
        }

        // Trademark protection is 10 years, renewable indefinitely
        let years_since_registration = current_date
            .years_since(self.registration_date)
            .unwrap_or(0);

        // Assuming renewal every 10 years
        years_since_registration < 10 || self.is_valid
    }
}

/// Trade secret (коммерческая тайна) representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSecret {
    /// Owner of the trade secret
    pub owner: String,
    /// Description of the secret
    pub description: String,
    /// Date of establishment
    pub established_date: chrono::NaiveDate,
    /// Has economic value
    pub has_economic_value: bool,
    /// Confidentiality measures taken
    pub confidentiality_measures: Vec<String>,
}

impl TradeSecret {
    /// Validates trade secret protection requirements (Article 1465)
    pub fn validate(&self) -> Result<(), CivilCodeError> {
        if !self.has_economic_value {
            return Err(CivilCodeError::InvalidIntellectualProperty(
                "Trade secret must have economic value".to_string(),
            ));
        }

        if self.confidentiality_measures.is_empty() {
            return Err(CivilCodeError::InvalidIntellectualProperty(
                "Trade secret must have confidentiality measures".to_string(),
            ));
        }

        Ok(())
    }
}

/// License agreement for intellectual property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseAgreement {
    /// Licensor
    pub licensor: String,
    /// Licensee
    pub licensee: String,
    /// Type of IP right
    pub ip_right: IntellectualPropertyRight,
    /// Is exclusive license
    pub exclusive: bool,
    /// License period
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    /// Territory
    pub territory: String,
    /// Royalty or lump sum
    pub compensation: Option<crate::common::Currency>,
}

impl LicenseAgreement {
    /// Validates the license agreement
    pub fn validate(&self) -> Result<(), CivilCodeError> {
        // License agreement must be in writing (Article 1235)
        // This is assumed to be true if we have a structured object

        // Check that end date is after start date if specified
        if let Some(end_date) = self.end_date
            && end_date <= self.start_date
        {
            return Err(CivilCodeError::InvalidIntellectualProperty(
                "License end date must be after start date".to_string(),
            ));
        }

        Ok(())
    }
}

/// Article 1259: Objects of copyright
pub fn is_copyrightable_work(work_type: &WorkType) -> bool {
    // All work types in the enum are copyrightable
    matches!(
        work_type,
        WorkType::Literary
            | WorkType::Musical
            | WorkType::Dramatic
            | WorkType::Choreographic
            | WorkType::Audiovisual
            | WorkType::ArtisticWork
            | WorkType::Architectural
            | WorkType::Photographic
            | WorkType::ComputerProgram
            | WorkType::Database
            | WorkType::Derivative
            | WorkType::Composite
    )
}

/// Article 1350: Patentability requirements
pub fn check_patentability(
    is_novel: bool,
    has_inventive_step: bool,
    is_industrially_applicable: bool,
) -> Result<(), CivilCodeError> {
    if !is_novel {
        return Err(CivilCodeError::InvalidIntellectualProperty(
            "Invention must be novel".to_string(),
        ));
    }

    if !has_inventive_step {
        return Err(CivilCodeError::InvalidIntellectualProperty(
            "Invention must have inventive step".to_string(),
        ));
    }

    if !is_industrially_applicable {
        return Err(CivilCodeError::InvalidIntellectualProperty(
            "Invention must be industrially applicable".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_type_protection_period() {
        assert_eq!(PatentType::Invention.protection_period_years(), 20);
        assert_eq!(PatentType::UtilityModel.protection_period_years(), 10);
        assert_eq!(PatentType::IndustrialDesign.protection_period_years(), 5);
    }

    #[test]
    fn test_copyright_work_types() {
        assert!(is_copyrightable_work(&WorkType::Literary));
        assert!(is_copyrightable_work(&WorkType::ComputerProgram));
        assert!(is_copyrightable_work(&WorkType::Database));
    }

    #[test]
    fn test_patentability_check() {
        // All requirements met
        assert!(check_patentability(true, true, true).is_ok());

        // Missing novelty
        assert!(check_patentability(false, true, true).is_err());

        // Missing inventive step
        assert!(check_patentability(true, false, true).is_err());

        // Missing industrial applicability
        assert!(check_patentability(true, true, false).is_err());
    }

    #[test]
    fn test_trade_secret_validation() {
        let valid_secret = TradeSecret {
            owner: "ООО Компания".to_string(),
            description: "Secret formula".to_string(),
            established_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
            has_economic_value: true,
            confidentiality_measures: vec!["NDA".to_string(), "Access control".to_string()],
        };
        assert!(valid_secret.validate().is_ok());

        let invalid_secret = TradeSecret {
            owner: "ООО Компания".to_string(),
            description: "Not secret".to_string(),
            established_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
            has_economic_value: false,
            confidentiality_measures: vec![],
        };
        assert!(invalid_secret.validate().is_err());
    }

    #[test]
    fn test_license_agreement_validation() {
        let valid_license = LicenseAgreement {
            licensor: "Лицензиар".to_string(),
            licensee: "Лицензиат".to_string(),
            ip_right: IntellectualPropertyRight::Patent,
            exclusive: true,
            start_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            end_date: Some(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).expect("Valid date")),
            territory: "Российская Федерация".to_string(),
            compensation: Some(crate::common::Currency::from_rubles(100000)),
        };
        assert!(valid_license.validate().is_ok());

        let invalid_license = LicenseAgreement {
            licensor: "Лицензиар".to_string(),
            licensee: "Лицензиат".to_string(),
            ip_right: IntellectualPropertyRight::Patent,
            exclusive: true,
            start_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            end_date: Some(chrono::NaiveDate::from_ymd_opt(2023, 1, 1).expect("Valid date")),
            territory: "Российская Федерация".to_string(),
            compensation: None,
        };
        assert!(invalid_license.validate().is_err());
    }

    #[test]
    fn test_trademark_validity() {
        let trademark = Trademark {
            registration_number: "123456".to_string(),
            holder: "ООО Компания".to_string(),
            registration_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
            description: "BRAND".to_string(),
            classes: vec![9, 35, 42],
            is_valid: true,
        };

        let current = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).expect("Valid date");
        assert!(trademark.is_protection_valid(&current));
    }
}
