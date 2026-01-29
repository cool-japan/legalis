//! # Criminal Code - Código Penal (Decreto-Lei nº 2.848/1940)
//!
//! Brazil's criminal code, establishing crimes and penalties.
//!
//! ## Overview
//!
//! The Brazilian Criminal Code was enacted in 1940 and has undergone numerous reforms.
//! It is divided into two parts:
//!
//! | Part | Content |
//! |------|---------|
//! | General Part | Criminal responsibility, penalties, extinction of punishability |
//! | Special Part | Specific crimes against persons, property, public administration, etc. |
//!
//! ## Key Principles
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | Legality | No crime without prior law (Art. 1) |
//! | Culpability | No penalty without guilt (nullum crimen sine culpa) |
//! | Presumption of Innocence | Constitutional guarantee (CF Art. 5, LVII) |
//! | Proportionality | Penalty proportional to offense |
//!
//! ## Penalties
//!
//! | Type | Range | Description |
//! |------|-------|-------------|
//! | Reclusão | 1-30 years | More severe imprisonment |
//! | Detenção | 15 days-30 years | Less severe imprisonment |
//! | Multa | Variable | Fine (days-fine system) |
//!
//! ## Major Crime Categories
//!
//! - Crimes against persons (Arts. 121-154)
//! - Property crimes (Arts. 155-183)
//! - Sexual crimes (Arts. 213-234)
//! - Crimes against public administration (Arts. 312-359)
//! - Economic crimes (Lei 8.137/1990)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Crime (crime/delito)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crime {
    /// Crime name
    pub nome: String,
    /// Criminal code article
    pub artigo: u16,
    /// Crime category
    pub categoria: CrimeCategory,
    /// Penalty
    pub pena: Penalty,
    /// Whether crime allows bail
    pub afiancavel: bool,
}

/// Crime categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrimeCategory {
    /// Crimes against persons (Arts. 121-154)
    AgainstPersons,
    /// Property crimes (Arts. 155-183)
    AgainstProperty,
    /// Sexual crimes (Arts. 213-234)
    Sexual,
    /// Against public administration (Arts. 312-359)
    AgainstPublicAdministration,
    /// Against public faith (Arts. 289-311)
    AgainstPublicFaith,
    /// Economic crimes
    Economic,
    /// Drug crimes (Lei 11.343/2006)
    DrugRelated,
    /// Environmental crimes (Lei 9.605/1998)
    Environmental,
    /// Cybercrimes (Lei 12.737/2012)
    Cyber,
}

/// Penalty (pena)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Penalty {
    /// Penalty type
    pub tipo: PenaltyType,
    /// Minimum duration
    pub minimo: PenaltyDuration,
    /// Maximum duration
    pub maximo: PenaltyDuration,
}

/// Penalty types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Reclusão (more severe imprisonment)
    Reclusao,
    /// Detenção (less severe imprisonment)
    Detencao,
    /// Fine (multa)
    Fine,
    /// Reclusão + Fine
    ReclusaoAndFine,
    /// Detenção + Fine
    DetencaoAndFine,
}

/// Penalty duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PenaltyDuration {
    /// Years
    pub anos: u8,
    /// Months
    pub meses: u8,
    /// Days
    pub dias: u8,
}

impl PenaltyDuration {
    /// Create duration from years
    pub fn from_years(anos: u8) -> Self {
        Self {
            anos,
            meses: 0,
            dias: 0,
        }
    }

    /// Create duration from months
    pub fn from_months(meses: u8) -> Self {
        Self {
            anos: 0,
            meses,
            dias: 0,
        }
    }

    /// Create duration from days
    pub fn from_days(dias: u8) -> Self {
        Self {
            anos: 0,
            meses: 0,
            dias,
        }
    }

    /// Convert to total days (approximate)
    pub fn to_days(&self) -> u32 {
        (self.anos as u32 * 365) + (self.meses as u32 * 30) + (self.dias as u32)
    }
}

/// Common crimes definitions
impl Crime {
    /// Homicide (homicídio) - Art. 121
    pub fn homicide_simple() -> Self {
        Self {
            nome: "Homicídio simples".to_string(),
            artigo: 121,
            categoria: CrimeCategory::AgainstPersons,
            pena: Penalty {
                tipo: PenaltyType::Reclusao,
                minimo: PenaltyDuration::from_years(6),
                maximo: PenaltyDuration::from_years(20),
            },
            afiancavel: false,
        }
    }

    /// Theft (furto) - Art. 155
    pub fn theft() -> Self {
        Self {
            nome: "Furto".to_string(),
            artigo: 155,
            categoria: CrimeCategory::AgainstProperty,
            pena: Penalty {
                tipo: PenaltyType::ReclusaoAndFine,
                minimo: PenaltyDuration::from_years(1),
                maximo: PenaltyDuration::from_years(4),
            },
            afiancavel: true,
        }
    }

    /// Robbery (roubo) - Art. 157
    pub fn robbery() -> Self {
        Self {
            nome: "Roubo".to_string(),
            artigo: 157,
            categoria: CrimeCategory::AgainstProperty,
            pena: Penalty {
                tipo: PenaltyType::ReclusaoAndFine,
                minimo: PenaltyDuration::from_years(4),
                maximo: PenaltyDuration::from_years(10),
            },
            afiancavel: false,
        }
    }

    /// Embezzlement (peculato) - Art. 312
    pub fn embezzlement() -> Self {
        Self {
            nome: "Peculato".to_string(),
            artigo: 312,
            categoria: CrimeCategory::AgainstPublicAdministration,
            pena: Penalty {
                tipo: PenaltyType::ReclusaoAndFine,
                minimo: PenaltyDuration::from_years(2),
                maximo: PenaltyDuration::from_years(12),
            },
            afiancavel: false,
        }
    }

    /// Corruption (corrupção passiva) - Art. 317
    pub fn passive_corruption() -> Self {
        Self {
            nome: "Corrupção passiva".to_string(),
            artigo: 317,
            categoria: CrimeCategory::AgainstPublicAdministration,
            pena: Penalty {
                tipo: PenaltyType::ReclusaoAndFine,
                minimo: PenaltyDuration::from_years(2),
                maximo: PenaltyDuration::from_years(12),
            },
            afiancavel: false,
        }
    }
}

/// Criminal procedural status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriminalProcedure {
    /// Defendant (réu)
    pub reu: String,
    /// Crime charged
    pub crime: Crime,
    /// Procedure start date
    pub data_inicio: NaiveDate,
    /// Current status
    pub status: ProcedureStatus,
    /// Whether defendant is in custody
    pub preso: bool,
}

/// Procedure status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcedureStatus {
    /// Investigation phase (inquérito)
    Investigation,
    /// Charges filed (denúncia oferecida)
    ChargesFiled,
    /// Trial phase (instrução)
    Trial,
    /// Sentenced (sentenciado)
    Sentenced,
    /// Appeal (recurso)
    Appeal,
    /// Final judgment (trânsito em julgado)
    FinalJudgment,
}

/// Aggravating and mitigating circumstances
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Circumstance {
    /// Aggravating (agravantes) - Art. 61
    Aggravating { description: String },
    /// Mitigating (atenuantes) - Art. 65
    Mitigating { description: String },
}

/// Causes of extinction of punishability (Art. 107)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtinctionOfPunishability {
    /// Death of agent
    DeathOfAgent,
    /// Amnesty
    Amnesty,
    /// Pardon (indulto)
    Pardon,
    /// Retroactive decriminalization (abolitio criminis)
    AbolitioCriminis,
    /// Statute of limitations (prescrição)
    StatuteOfLimitations,
    /// Judgment (sentença)
    Judgment,
    /// Payment (in specific cases)
    Payment,
}

/// Criminal code errors
#[derive(Debug, Clone, Error)]
pub enum CriminalCodeError {
    /// No crime without law (Art. 1)
    #[error("Não há crime sem lei anterior que o defina (Art. 1º)")]
    NoCrimeWithoutLaw,

    /// Invalid penalty
    #[error("Pena inválida: {reason}")]
    InvalidPenalty { reason: String },

    /// Statute of limitations expired
    #[error("Prescrição da pretensão punitiva (Art. 109)")]
    StatuteOfLimitations,

    /// Extinction of punishability
    #[error("Extinção da punibilidade (Art. 107): {cause:?}")]
    ExtinctionOfPunishability { cause: ExtinctionOfPunishability },

    /// Procedural error
    #[error("Erro processual: {description}")]
    ProceduralError { description: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for criminal code operations
pub type CriminalCodeResult<T> = Result<T, CriminalCodeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homicide_crime() {
        let crime = Crime::homicide_simple();
        assert_eq!(crime.artigo, 121);
        assert!(!crime.afiancavel);
        assert_eq!(crime.pena.tipo, PenaltyType::Reclusao);
    }

    #[test]
    fn test_theft_crime() {
        let crime = Crime::theft();
        assert_eq!(crime.artigo, 155);
        assert!(crime.afiancavel);
    }

    #[test]
    fn test_penalty_duration() {
        let duration = PenaltyDuration::from_years(5);
        assert_eq!(duration.to_days(), 1825); // 5 * 365

        let months = PenaltyDuration::from_months(6);
        assert_eq!(months.to_days(), 180); // 6 * 30
    }

    #[test]
    fn test_corruption_crime() {
        let crime = Crime::passive_corruption();
        assert_eq!(crime.artigo, 317);
        assert_eq!(crime.categoria, CrimeCategory::AgainstPublicAdministration);
    }
}
