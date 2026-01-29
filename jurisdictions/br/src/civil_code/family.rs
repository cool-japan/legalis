//! Family Law (Direito de Família) - Articles 1511-1783
//!
//! Family relationships, marriage, kinship, and protection.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Marriage (casamento) - Arts. 1511-1590
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Marriage {
    /// Spouse 1
    pub conjuge1: String,
    /// Spouse 2
    pub conjuge2: String,
    /// Marriage date
    pub data_casamento: NaiveDate,
    /// Property regime
    pub regime_bens: PropertyRegime,
    /// Marriage certificate number
    pub certidao: Option<String>,
}

/// Property regimes in marriage (regime de bens) - Arts. 1639-1688
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyRegime {
    /// Community of property (comunhão universal) - Arts. 1667-1671
    /// All property is shared
    UniversalCommunity,
    /// Partial community (comunhão parcial) - Arts. 1658-1666
    /// DEFAULT regime - property acquired during marriage is shared
    PartialCommunity,
    /// Separation of property (separação de bens) - Arts. 1687-1688
    /// Each spouse keeps their own property
    SeparationOfProperty,
    /// Participation in final acquests (participação final nos aquestos) - Arts. 1672-1686
    /// Each spouse owns their property, but shares appreciation at dissolution
    FinalAcquests,
}

impl PropertyRegime {
    /// Check if regime is the default (Art. 1640)
    pub fn is_default(&self) -> bool {
        matches!(self, Self::PartialCommunity)
    }

    /// Get regime description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::UniversalCommunity => "Comunhão Universal de Bens",
            Self::PartialCommunity => "Comunhão Parcial de Bens",
            Self::SeparationOfProperty => "Separação de Bens",
            Self::FinalAcquests => "Participação Final nos Aquestos",
        }
    }
}

impl Marriage {
    /// Create a new marriage with default partial community regime
    pub fn new(conjuge1: impl Into<String>, conjuge2: impl Into<String>, data: NaiveDate) -> Self {
        Self {
            conjuge1: conjuge1.into(),
            conjuge2: conjuge2.into(),
            data_casamento: data,
            regime_bens: PropertyRegime::PartialCommunity,
            certidao: None,
        }
    }

    /// Set property regime
    pub fn with_regime(mut self, regime: PropertyRegime) -> Self {
        self.regime_bens = regime;
        self
    }

    /// Check marriage capacity requirements (Art. 1517)
    /// Minimum age: 16 years (with parental consent)
    /// Full capacity: 18 years
    pub fn check_minimum_age(idade: u8) -> Result<(), FamilyError> {
        if idade < 16 {
            return Err(FamilyError::BelowMinimumAge { idade });
        }
        Ok(())
    }
}

/// Stable union (união estável) - Arts. 1723-1727
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableUnion {
    /// Partner 1
    pub companheiro1: String,
    /// Partner 2
    pub companheiro2: String,
    /// Start date
    pub inicio: NaiveDate,
    /// Whether union is public (notório)
    pub publica: bool,
    /// Whether union is continuous (contínua)
    pub continua: bool,
    /// Whether union has family constitution objective
    pub objetivo_familiar: bool,
}

impl StableUnion {
    /// Create a new stable union
    pub fn new(
        companheiro1: impl Into<String>,
        companheiro2: impl Into<String>,
        inicio: NaiveDate,
    ) -> Self {
        Self {
            companheiro1: companheiro1.into(),
            companheiro2: companheiro2.into(),
            inicio,
            publica: true,
            continua: true,
            objetivo_familiar: true,
        }
    }

    /// Check if stable union requirements are met (Art. 1723)
    /// Requires: public, continuous, durable, with family objective
    pub fn is_valid(&self) -> bool {
        self.publica && self.continua && self.objetivo_familiar
    }

    /// Convert to marriage (Art. 1726)
    pub fn convert_to_marriage(&self, data: NaiveDate) -> Marriage {
        Marriage::new(self.companheiro1.clone(), self.companheiro2.clone(), data)
    }
}

/// Divorce (divórcio) - Arts. 1571-1582
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Divorce {
    /// Divorce type
    pub tipo: DivorceType,
    /// Divorce date
    pub data: NaiveDate,
    /// Whether there are minor children
    pub filhos_menores: bool,
    /// Alimony arrangement
    pub pensao_alimenticia: Option<AlimonyArrangement>,
}

/// Divorce types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceType {
    /// Consensual divorce (divórcio consensual)
    Consensual,
    /// Litigious divorce (divórcio litigioso)
    Litigious,
    /// Direct conversion (without prior separation)
    Direct,
}

/// Alimony arrangement (pensão alimentícia)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlimonyArrangement {
    /// Payer (alimentante)
    pub pagador: String,
    /// Recipient (alimentando)
    pub beneficiario: String,
    /// Amount in BRL (monthly)
    pub valor_mensal: u64,
    /// Whether amount is percentage of income
    pub percentual_renda: Option<u8>,
}

/// Kinship (parentesco) - Arts. 1591-1595
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kinship {
    /// Direct line (linha reta): ascendants/descendants
    Direct { degree: u8 },
    /// Collateral line (linha colateral): up to 4th degree
    Collateral { degree: u8 },
    /// Affinity (afinidade): relationship through marriage
    Affinity { degree: u8 },
}

impl Kinship {
    /// Check if kinship is within prohibited degrees for marriage (Art. 1521)
    pub fn is_prohibited_for_marriage(&self) -> bool {
        match self {
            Self::Direct { .. } => true, // All direct line prohibited
            Self::Collateral { degree } => *degree <= 2, // Up to siblings
            Self::Affinity { degree } => *degree <= 1, // Direct line affinity
        }
    }
}

/// Parental authority (poder familiar) - Arts. 1630-1638
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentalAuthority {
    /// Parent(s)
    pub pais: Vec<String>,
    /// Child
    pub filho: String,
    /// Whether child is minor
    pub menor: bool,
    /// Whether authority is suspended
    pub suspenso: bool,
    /// Whether authority is lost
    pub perdido: bool,
}

impl ParentalAuthority {
    /// Create new parental authority
    pub fn new(pais: Vec<String>, filho: impl Into<String>, menor: bool) -> Self {
        Self {
            pais,
            filho: filho.into(),
            menor,
            suspenso: false,
            perdido: false,
        }
    }

    /// Check if parental authority is active
    pub fn is_active(&self) -> bool {
        self.menor && !self.suspenso && !self.perdido
    }

    /// Grounds for loss of parental authority (Art. 1638)
    pub fn grounds_for_loss() -> [&'static str; 4] {
        [
            "Castigar imoderadamente o filho",
            "Deixar o filho em abandono",
            "Praticar atos contrários à moral e aos bons costumes",
            "Incidir, reiteradamente, nas faltas previstas no artigo antecedente",
        ]
    }
}

/// Family law errors
#[derive(Debug, Clone, Error)]
pub enum FamilyError {
    /// Below minimum marriage age
    #[error("Idade inferior ao mínimo legal para casamento (Art. 1517): {idade} anos")]
    BelowMinimumAge { idade: u8 },

    /// Prohibited kinship for marriage
    #[error("Parentesco em grau proibido para casamento (Art. 1521): {kinship:?}")]
    ProhibitedKinship { kinship: Kinship },

    /// Invalid stable union
    #[error("União estável inválida (Art. 1723): {reason}")]
    InvalidStableUnion { reason: String },

    /// Divorce requirements not met
    #[error("Requisitos de divórcio não atendidos: {reason}")]
    DivorceRequirements { reason: String },

    /// Invalid property regime
    #[error("Regime de bens inválido: {reason}")]
    InvalidPropertyRegime { reason: String },

    /// Parental authority violation
    #[error("Violação do poder familiar (Arts. 1630-1638): {reason}")]
    ParentalAuthorityViolation { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for family law operations
pub type FamilyResult<T> = Result<T, FamilyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marriage_creation() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let marriage =
            Marriage::new("João", "Maria", date).with_regime(PropertyRegime::UniversalCommunity);
        assert_eq!(marriage.regime_bens, PropertyRegime::UniversalCommunity);
    }

    #[test]
    fn test_default_property_regime() {
        assert!(PropertyRegime::PartialCommunity.is_default());
    }

    #[test]
    fn test_stable_union() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        let union = StableUnion::new("Pedro", "Ana", date);
        assert!(union.is_valid());
    }

    #[test]
    fn test_kinship_prohibition() {
        let direct = Kinship::Direct { degree: 1 }; // Parent-child
        assert!(direct.is_prohibited_for_marriage());

        let collateral = Kinship::Collateral { degree: 4 }; // Cousins
        assert!(!collateral.is_prohibited_for_marriage());
    }

    #[test]
    fn test_parental_authority() {
        let authority =
            ParentalAuthority::new(vec!["Mãe".to_string(), "Pai".to_string()], "Filho", true);
        assert!(authority.is_active());
        assert_eq!(ParentalAuthority::grounds_for_loss().len(), 4);
    }

    #[test]
    fn test_minimum_marriage_age() {
        assert!(Marriage::check_minimum_age(18).is_ok());
        assert!(Marriage::check_minimum_age(16).is_ok());
        assert!(Marriage::check_minimum_age(15).is_err());
    }
}
