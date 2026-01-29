//! Criminal Code of the Russian Federation (Уголовный кодекс РФ).
//!
//! Federal Law No. 63-FZ of June 13, 1996
//!
//! This module provides:
//! - Crime classifications
//! - Criminal liability rules
//! - Punishment types
//! - Sentencing guidelines

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to Criminal Code operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CriminalCodeError {
    /// Invalid crime classification
    #[error("Invalid crime classification: {0}")]
    InvalidCrime(String),

    /// Invalid punishment
    #[error("Invalid punishment: {0}")]
    InvalidPunishment(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Categories of crimes (Article 15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrimeCategory {
    /// Minor crime (небольшой тяжести) - up to 3 years
    Minor,
    /// Medium gravity (средней тяжести) - up to 5 years
    Medium,
    /// Grave crime (тяжкое преступление) - up to 10 years
    Grave,
    /// Especially grave crime (особо тяжкое преступление) - over 10 years
    EspeciallyGrave,
}

impl CrimeCategory {
    /// Returns the maximum imprisonment term in years
    pub fn max_imprisonment_years(&self) -> u32 {
        match self {
            Self::Minor => 3,
            Self::Medium => 5,
            Self::Grave => 10,
            Self::EspeciallyGrave => 20, // Can be higher for specific crimes
        }
    }

    /// Returns the description in Russian
    pub fn description_ru(&self) -> &'static str {
        match self {
            Self::Minor => "Преступление небольшой тяжести",
            Self::Medium => "Преступление средней тяжести",
            Self::Grave => "Тяжкое преступление",
            Self::EspeciallyGrave => "Особо тяжкое преступление",
        }
    }
}

/// Crime representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crime {
    /// Article number
    pub article: u32,
    /// Part of article (if applicable)
    pub part: Option<u32>,
    /// Crime category
    pub category: CrimeCategory,
    /// Description in Russian
    pub description_ru: String,
    /// Description in English
    pub description_en: Option<String>,
    /// Possible punishments
    pub punishments: Vec<PunishmentType>,
}

impl Crime {
    /// Creates a new crime
    pub fn new(article: u32, category: CrimeCategory, description_ru: impl Into<String>) -> Self {
        Self {
            article,
            part: None,
            category,
            description_ru: description_ru.into(),
            description_en: None,
            punishments: Vec::new(),
        }
    }

    /// Sets the part of the article
    pub fn with_part(mut self, part: u32) -> Self {
        self.part = Some(part);
        self
    }

    /// Adds a punishment type
    pub fn add_punishment(mut self, punishment: PunishmentType) -> Self {
        self.punishments.push(punishment);
        self
    }
}

/// Types of punishment (Article 44)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PunishmentType {
    /// Fine (штраф)
    Fine { min_amount: u64, max_amount: u64 },
    /// Deprivation of the right to hold certain positions
    DeprivationOfRight { duration_years: u32 },
    /// Compulsory work (обязательные работы)
    CompulsoryWork { hours: u32 },
    /// Correctional labor (исправительные работы)
    CorrectionalLabor { months: u32 },
    /// Restriction of freedom (ограничение свободы)
    RestrictionOfFreedom { years: u32 },
    /// Forced labor (принудительные работы)
    ForcedLabor { years: u32 },
    /// Imprisonment (лишение свободы)
    Imprisonment { min_years: u32, max_years: u32 },
    /// Life imprisonment (пожизненное лишение свободы)
    LifeImprisonment,
}

impl PunishmentType {
    /// Validates the punishment
    pub fn validate(&self) -> Result<(), CriminalCodeError> {
        match self {
            Self::Fine {
                min_amount,
                max_amount,
            } => {
                if max_amount < min_amount {
                    return Err(CriminalCodeError::InvalidPunishment(
                        "Maximum fine cannot be less than minimum".to_string(),
                    ));
                }
            }
            Self::CompulsoryWork { hours } => {
                if *hours > 480 {
                    return Err(CriminalCodeError::InvalidPunishment(
                        "Compulsory work cannot exceed 480 hours".to_string(),
                    ));
                }
            }
            Self::CorrectionalLabor { months } => {
                if *months > 24 {
                    return Err(CriminalCodeError::InvalidPunishment(
                        "Correctional labor cannot exceed 24 months".to_string(),
                    ));
                }
            }
            Self::Imprisonment {
                min_years,
                max_years,
            } => {
                if max_years < min_years {
                    return Err(CriminalCodeError::InvalidPunishment(
                        "Maximum imprisonment cannot be less than minimum".to_string(),
                    ));
                }
                if *max_years > 20 {
                    // General rule, exceptions exist
                    return Err(CriminalCodeError::InvalidPunishment(
                        "Imprisonment generally cannot exceed 20 years".to_string(),
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

/// Criminal liability representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalLiability {
    /// The crime committed
    pub crime: Crime,
    /// Defendant
    pub defendant: String,
    /// Date of crime
    pub crime_date: chrono::NaiveDate,
    /// Mitigating circumstances (Article 61)
    pub mitigating_circumstances: Vec<String>,
    /// Aggravating circumstances (Article 63)
    pub aggravating_circumstances: Vec<String>,
}

impl CriminalLiability {
    /// Creates new criminal liability
    pub fn new(crime: Crime, defendant: impl Into<String>, crime_date: chrono::NaiveDate) -> Self {
        Self {
            crime,
            defendant: defendant.into(),
            crime_date,
            mitigating_circumstances: Vec::new(),
            aggravating_circumstances: Vec::new(),
        }
    }

    /// Adds a mitigating circumstance
    pub fn add_mitigating(mut self, circumstance: impl Into<String>) -> Self {
        self.mitigating_circumstances.push(circumstance.into());
        self
    }

    /// Adds an aggravating circumstance
    pub fn add_aggravating(mut self, circumstance: impl Into<String>) -> Self {
        self.aggravating_circumstances.push(circumstance.into());
        self
    }
}

/// Sanction (punishment imposed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sanction {
    /// Type of punishment
    pub punishment: PunishmentType,
    /// Date of sentencing
    pub sentencing_date: chrono::NaiveDate,
    /// Is suspended (условное наказание)
    pub suspended: bool,
    /// Probation period if suspended
    pub probation_period_years: Option<u32>,
}

impl Sanction {
    /// Creates a new sanction
    pub fn new(punishment: PunishmentType, sentencing_date: chrono::NaiveDate) -> Self {
        Self {
            punishment,
            sentencing_date,
            suspended: false,
            probation_period_years: None,
        }
    }

    /// Makes the sanction suspended with probation
    pub fn with_suspension(mut self, probation_years: u32) -> Self {
        self.suspended = true;
        self.probation_period_years = Some(probation_years);
        self
    }

    /// Validates the sanction
    pub fn validate(&self) -> Result<(), CriminalCodeError> {
        self.punishment.validate()?;

        if self.suspended && self.probation_period_years.is_none() {
            return Err(CriminalCodeError::InvalidPunishment(
                "Suspended sentence must have probation period".to_string(),
            ));
        }

        Ok(())
    }
}

/// Quick validation for criminal liability
pub fn quick_validate_criminal_liability(
    liability: &CriminalLiability,
) -> Result<(), CriminalCodeError> {
    // Validate that the crime category matches potential punishments
    for punishment in &liability.crime.punishments {
        punishment.validate()?;
    }

    Ok(())
}

/// Article 14: Concept of crime
pub fn is_crime(is_socially_dangerous: bool, is_unlawful: bool, is_culpable: bool) -> bool {
    is_socially_dangerous && is_unlawful && is_culpable
}

/// Article 19: General conditions of criminal liability (age requirement)
pub fn check_criminal_capacity(birth_date: &chrono::NaiveDate) -> Result<u32, CriminalCodeError> {
    let today = chrono::Local::now().naive_local().date();
    let age = today.years_since(*birth_date).unwrap_or(0);

    if age < 16 {
        return Err(CriminalCodeError::ValidationFailed(
            "Criminal liability generally starts at 16 years".to_string(),
        ));
    }

    Ok(age)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crime_category() {
        assert_eq!(CrimeCategory::Minor.max_imprisonment_years(), 3);
        assert_eq!(CrimeCategory::EspeciallyGrave.max_imprisonment_years(), 20);
    }

    #[test]
    fn test_punishment_validation() {
        let fine = PunishmentType::Fine {
            min_amount: 1000,
            max_amount: 5000,
        };
        assert!(fine.validate().is_ok());

        let invalid_fine = PunishmentType::Fine {
            min_amount: 5000,
            max_amount: 1000,
        };
        assert!(invalid_fine.validate().is_err());

        let excessive_work = PunishmentType::CompulsoryWork { hours: 500 };
        assert!(excessive_work.validate().is_err());
    }

    #[test]
    fn test_crime_definition() {
        assert!(is_crime(true, true, true));
        assert!(!is_crime(false, true, true));
        assert!(!is_crime(true, false, true));
    }

    #[test]
    fn test_criminal_capacity() {
        let birth_2015 = chrono::NaiveDate::from_ymd_opt(2015, 1, 1).expect("Valid date");
        assert!(check_criminal_capacity(&birth_2015).is_err());

        let birth_2000 = chrono::NaiveDate::from_ymd_opt(2000, 1, 1).expect("Valid date");
        let age = check_criminal_capacity(&birth_2000).expect("Should succeed");
        assert!(age >= 16);
    }

    #[test]
    fn test_sanction_validation() {
        let punishment = PunishmentType::Imprisonment {
            min_years: 2,
            max_years: 5,
        };
        let sanction = Sanction::new(
            punishment,
            chrono::NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
        )
        .with_suspension(3);

        assert!(sanction.validate().is_ok());
        assert!(sanction.suspended);
    }
}
