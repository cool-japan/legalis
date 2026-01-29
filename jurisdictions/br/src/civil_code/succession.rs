//! Succession Law (Direito das Sucessões) - Articles 1784-2027
//!
//! Inheritance, wills, and estate administration.

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Inheritance (herança) - Arts. 1784-1828
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inheritance {
    /// Deceased (de cujus)
    pub falecido: String,
    /// Date of death
    pub data_obito: NaiveDate,
    /// Estate value
    pub valor_patrimonio: BrazilianCurrency,
    /// List of heirs
    pub herdeiros: Vec<Heir>,
    /// Type of succession
    pub tipo_sucessao: SuccessionType,
}

/// Types of succession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccessionType {
    /// Testamentary succession (sucessão testamentária)
    Testamentary,
    /// Legitimate/legal succession (sucessão legítima)
    Legitimate,
    /// Mixed succession (both will and legal)
    Mixed,
}

impl Inheritance {
    /// Create a new inheritance
    pub fn new(
        falecido: impl Into<String>,
        data_obito: NaiveDate,
        valor: BrazilianCurrency,
    ) -> Self {
        Self {
            falecido: falecido.into(),
            data_obito,
            valor_patrimonio: valor,
            herdeiros: Vec::new(),
            tipo_sucessao: SuccessionType::Legitimate,
        }
    }

    /// Add heir to inheritance
    pub fn add_heir(mut self, heir: Heir) -> Self {
        self.herdeiros.push(heir);
        self
    }

    /// Opening of succession (abertura da sucessão) - Art. 1784
    /// Succession opens at moment of death
    pub fn succession_opens_at(&self) -> NaiveDate {
        self.data_obito
    }

    /// Calculate reserved portion (legítima) - Art. 1846
    /// 50% of estate is reserved for necessary heirs
    pub fn calculate_reserved_portion(&self) -> BrazilianCurrency {
        BrazilianCurrency::from_centavos(self.valor_patrimonio.centavos / 2)
    }

    /// Calculate available portion (quota disponível) - Art. 1789
    /// 50% can be freely disposed by will
    pub fn calculate_available_portion(&self) -> BrazilianCurrency {
        BrazilianCurrency::from_centavos(self.valor_patrimonio.centavos / 2)
    }
}

/// Heir (herdeiro)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heir {
    /// Heir name
    pub nome: String,
    /// Heir type
    pub tipo: HeirType,
    /// Succession order/class
    pub ordem: SuccessionOrder,
    /// Share/quota
    pub quota: Option<Fraction>,
}

/// Types of heirs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeirType {
    /// Necessary heir (herdeiro necessário) - Art. 1845
    /// Descendants, ascendants, surviving spouse
    Necessary,
    /// Testamentary heir (herdeiro testamentário)
    Testamentary,
    /// Legatee (legatário) - receives specific bequest
    Legatee,
}

/// Succession order (ordem da vocação hereditária) - Arts. 1829-1844
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccessionOrder {
    /// 1st class: Descendants + surviving spouse
    FirstClass,
    /// 2nd class: Ascendants + surviving spouse
    SecondClass,
    /// 3rd class: Surviving spouse alone
    ThirdClass,
    /// 4th class: Collateral relatives (up to 4th degree)
    FourthClass,
}

impl SuccessionOrder {
    /// Get order priority (lower is higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            Self::FirstClass => 1,
            Self::SecondClass => 2,
            Self::ThirdClass => 3,
            Self::FourthClass => 4,
        }
    }

    /// Get description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::FirstClass => "Descendentes com cônjuge sobrevivente",
            Self::SecondClass => "Ascendentes com cônjuge sobrevivente",
            Self::ThirdClass => "Cônjuge sobrevivente",
            Self::FourthClass => "Colaterais até o quarto grau",
        }
    }
}

/// Fraction representation for inheritance shares
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fraction {
    /// Numerator
    pub numerador: u32,
    /// Denominator
    pub denominador: u32,
}

impl Fraction {
    /// Create a new fraction
    pub fn new(numerador: u32, denominador: u32) -> Self {
        Self {
            numerador,
            denominador,
        }
    }

    /// Calculate fraction value
    pub fn value(&self) -> f64 {
        self.numerador as f64 / self.denominador as f64
    }
}

/// Will/Testament (testamento) - Arts. 1857-1990
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Will {
    /// Testator (testador)
    pub testador: String,
    /// Will type
    pub tipo: WillType,
    /// Date of execution
    pub data_execucao: NaiveDate,
    /// Testamentary dispositions
    pub disposicoes: Vec<TestamentaryDisposition>,
    /// Whether will is valid
    pub valido: bool,
}

/// Types of wills (Art. 1862)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WillType {
    /// Public will (testamento público) - Arts. 1864-1867
    Public,
    /// Closed will (testamento cerrado) - Arts. 1868-1875
    Closed,
    /// Holographic will (testamento particular) - Arts. 1876-1880
    Holographic,
    /// Maritime will (testamento marítimo) - Arts. 1888-1892
    Maritime,
    /// Aeronautic will (testamento aeronáutico) - Arts. 1889-1892
    Aeronautic,
    /// Military will (testamento militar) - Arts. 1893-1896
    Military,
}

impl Will {
    /// Create a new will
    pub fn new(testador: impl Into<String>, tipo: WillType, data: NaiveDate) -> Self {
        Self {
            testador: testador.into(),
            tipo,
            data_execucao: data,
            disposicoes: Vec::new(),
            valido: true,
        }
    }

    /// Check testamentary capacity (capacidade testamentária) - Art. 1860
    /// Must be at least 16 years old
    pub fn check_testamentary_capacity(idade: u8) -> Result<(), SuccessionError> {
        if idade < 16 {
            return Err(SuccessionError::InsufficientTestamentaryCapacity { idade });
        }
        Ok(())
    }

    /// Add testamentary disposition
    pub fn add_disposition(mut self, disposition: TestamentaryDisposition) -> Self {
        self.disposicoes.push(disposition);
        self
    }

    /// Check if will respects reserved portion (Art. 1846)
    pub fn respects_reserved_portion(&self) -> bool {
        // Simplified check - would need full estate calculation
        self.valido
    }
}

/// Testamentary disposition (disposição testamentária)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestamentaryDisposition {
    /// Beneficiary
    pub beneficiario: String,
    /// Object/property disposed
    pub objeto: String,
    /// Disposition type
    pub tipo: DispositionType,
}

/// Types of testamentary dispositions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispositionType {
    /// Bequest/Legacy (legado)
    Legacy,
    /// Institution of heir (instituição de herdeiro)
    InstitutionOfHeir,
    /// Substitution (substituição)
    Substitution,
    /// Condition/term/encumbrance (condição/termo/encargo)
    Condition,
}

/// Inventory (inventário) - Arts. 1991-2027
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    /// Deceased
    pub falecido: String,
    /// Inventory type
    pub tipo: InventoryType,
    /// Start date
    pub data_inicio: NaiveDate,
    /// Whether there are minor heirs
    pub herdeiros_menores: bool,
    /// Estate administrator
    pub inventariante: String,
}

/// Types of inventory proceedings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventoryType {
    /// Judicial inventory (inventário judicial)
    Judicial,
    /// Extrajudicial inventory (inventário extrajudicial) - Art. 982, CPC
    /// Requires: all heirs capable and in agreement, no will
    Extrajudicial,
}

impl Inventory {
    /// Check if extrajudicial inventory is possible
    /// Requires: no minors, all heirs agree, no disputed will
    pub fn can_be_extrajudicial(herdeiros_menores: bool, testamento: bool) -> bool {
        !herdeiros_menores && !testamento
    }
}

/// Succession errors
#[derive(Debug, Clone, Error)]
pub enum SuccessionError {
    /// Insufficient testamentary capacity
    #[error("Incapacidade testamentária (Art. 1860): idade {idade} anos")]
    InsufficientTestamentaryCapacity { idade: u8 },

    /// Violation of reserved portion
    #[error("Violação da legítima (Art. 1846): {description}")]
    ReservedPortionViolation { description: String },

    /// Invalid will
    #[error("Testamento inválido: {reason}")]
    InvalidWill { reason: String },

    /// Succession order violation
    #[error("Violação da ordem de vocação hereditária (Arts. 1829-1844)")]
    SuccessionOrderViolation,

    /// Disqualified heir (indignidade) - Art. 1814
    #[error("Herdeiro indigno (Art. 1814): {reason}")]
    DisqualifiedHeir { reason: String },

    /// Disinheritance (deserdação) - Art. 1961
    #[error("Deserdação (Art. 1961): {reason}")]
    Disinheritance { reason: String },

    /// Invalid inventory
    #[error("Inventário inválido: {reason}")]
    InvalidInventory { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for succession operations
pub type SuccessionResult<T> = Result<T, SuccessionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inheritance_creation() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let inheritance =
            Inheritance::new("João Silva", date, BrazilianCurrency::from_reais(1000000));
        assert_eq!(inheritance.succession_opens_at(), date);
    }

    #[test]
    fn test_reserved_portion() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let inheritance =
            Inheritance::new("Maria Santos", date, BrazilianCurrency::from_reais(1000000));
        let reserved = inheritance.calculate_reserved_portion();
        let available = inheritance.calculate_available_portion();
        assert_eq!(reserved.reais(), 500000);
        assert_eq!(available.reais(), 500000);
    }

    #[test]
    fn test_succession_order_priority() {
        assert_eq!(SuccessionOrder::FirstClass.priority(), 1);
        assert_eq!(SuccessionOrder::FourthClass.priority(), 4);
    }

    #[test]
    fn test_will_capacity() {
        assert!(Will::check_testamentary_capacity(18).is_ok());
        assert!(Will::check_testamentary_capacity(16).is_ok());
        assert!(Will::check_testamentary_capacity(15).is_err());
    }

    #[test]
    fn test_fraction_calculation() {
        let half = Fraction::new(1, 2);
        assert!((half.value() - 0.5).abs() < f64::EPSILON);

        let third = Fraction::new(1, 3);
        assert!((third.value() - 0.333333).abs() < 0.001);
    }

    #[test]
    fn test_inventory_extrajudicial() {
        assert!(Inventory::can_be_extrajudicial(false, false));
        assert!(!Inventory::can_be_extrajudicial(true, false));
        assert!(!Inventory::can_be_extrajudicial(false, true));
    }
}
