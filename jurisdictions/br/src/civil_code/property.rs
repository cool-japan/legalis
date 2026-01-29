//! Property Rights (Direito das Coisas) - Articles 1196-1510
//!
//! Real rights including possession, ownership, and limited real rights.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Possession (posse) - Arts. 1196-1224
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Possession {
    /// Possessor (possuidor)
    pub possuidor: String,
    /// Object of possession
    pub objeto: String,
    /// Type of possession
    pub tipo: PossessionType,
    /// Whether possession is in good faith
    pub boa_fe: bool,
    /// Whether possession is with just title
    pub justo_titulo: bool,
}

/// Types of possession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PossessionType {
    /// Direct possession (posse direta)
    Direct,
    /// Indirect possession (posse indireta)
    Indirect,
    /// Possession as owner (posse ad usucapionem)
    AsOwner,
    /// Precarious possession (posse precária)
    Precarious,
}

impl Possession {
    /// Create a new possession
    pub fn new(
        possuidor: impl Into<String>,
        objeto: impl Into<String>,
        tipo: PossessionType,
    ) -> Self {
        Self {
            possuidor: possuidor.into(),
            objeto: objeto.into(),
            tipo,
            boa_fe: true,
            justo_titulo: false,
        }
    }

    /// Check if possession can lead to adverse possession (usucapião)
    /// Requires: continuous, peaceful, public possession as owner
    pub fn can_lead_to_adverse_possession(&self) -> bool {
        matches!(self.tipo, PossessionType::AsOwner) && self.boa_fe
    }

    /// Set good faith status
    pub fn with_good_faith(mut self, boa_fe: bool) -> Self {
        self.boa_fe = boa_fe;
        self
    }

    /// Set just title status
    pub fn with_just_title(mut self) -> Self {
        self.justo_titulo = true;
        self
    }
}

/// Ownership (propriedade) - Arts. 1228-1368
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ownership {
    /// Owner (proprietário)
    pub proprietario: String,
    /// Property description
    pub imovel: String,
    /// Property type
    pub tipo: PropertyKind,
    /// Whether property serves social function
    pub funcao_social: bool,
}

/// Property kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyKind {
    /// Urban property
    Urban,
    /// Rural property
    Rural,
    /// Movable property
    Movable,
}

impl Ownership {
    /// Create a new ownership
    pub fn new(
        proprietario: impl Into<String>,
        imovel: impl Into<String>,
        tipo: PropertyKind,
    ) -> Self {
        Self {
            proprietario: proprietario.into(),
            imovel: imovel.into(),
            tipo,
            funcao_social: true,
        }
    }

    /// Check if property serves social function (Art. 1228, §1)
    /// Required for all property
    pub fn serves_social_function(&self) -> bool {
        self.funcao_social
    }

    /// Get owner's faculties (Art. 1228)
    /// Use, enjoy, dispose, recover (usar, gozar, dispor, reivindicar)
    pub fn owner_faculties(&self) -> [&'static str; 4] {
        ["usar", "gozar", "dispor", "reivindicar"]
    }
}

/// Adverse possession (usucapião) - Arts. 1238-1244
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdversePossession {
    /// Type of adverse possession
    pub tipo: AdversePossessionType,
    /// Years of possession
    pub anos_posse: u8,
    /// Whether possession meets requirements
    pub requisitos_atendidos: bool,
}

/// Types of adverse possession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdversePossessionType {
    /// Ordinary adverse possession - 10 years with just title and good faith (Art. 1242)
    Ordinary,
    /// Extraordinary adverse possession - 15 years (Art. 1238)
    Extraordinary,
    /// Special urban adverse possession - 5 years (Art. 1240)
    SpecialUrban,
    /// Special rural adverse possession - 5 years (Art. 1239)
    SpecialRural,
    /// Family homestead adverse possession - 2 years (Art. 1240-A)
    FamilyHomestead,
}

impl AdversePossession {
    /// Get required years for adverse possession
    pub fn required_years(&self) -> u8 {
        match self.tipo {
            AdversePossessionType::FamilyHomestead => 2,
            AdversePossessionType::SpecialUrban | AdversePossessionType::SpecialRural => 5,
            AdversePossessionType::Ordinary => 10,
            AdversePossessionType::Extraordinary => 15,
        }
    }

    /// Check if adverse possession is complete
    pub fn is_complete(&self) -> bool {
        self.anos_posse >= self.required_years() && self.requisitos_atendidos
    }
}

/// Limited real rights (direitos reais limitados) - Arts. 1369-1510
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitedRealRight {
    /// Surface right (direito de superfície) - Arts. 1369-1377
    SurfaceRight {
        /// Surface holder
        superficiario: String,
        /// Property owner
        proprietario: String,
        /// Duration in years
        prazo_anos: Option<u8>,
    },
    /// Servitude (servidão) - Arts. 1378-1389
    Servitude {
        /// Dominant property
        predio_dominante: String,
        /// Servant property
        predio_serviente: String,
        /// Type of servitude
        tipo: String,
    },
    /// Usufruct (usufruto) - Arts. 1390-1411
    Usufruct {
        /// Usufructuary
        usufrutuario: String,
        /// Bare owner (nu-proprietário)
        nu_proprietario: String,
        /// Duration in years
        prazo_anos: Option<u8>,
    },
    /// Use (uso) - Arts. 1412-1413
    Use {
        /// User
        usuario: String,
        /// Owner
        proprietario: String,
    },
    /// Habitation (habitação) - Arts. 1414-1416
    Habitation {
        /// Habitant
        habitante: String,
        /// Owner
        proprietario: String,
    },
    /// Right of way (direito do promitente comprador) - Arts. 1417-1418
    RightOfWay {
        /// Promissory buyer
        promitente_comprador: String,
        /// Promissory seller
        promitente_vendedor: String,
    },
    /// Mortgage (hipoteca) - Arts. 1473-1505
    Mortgage {
        /// Mortgagor
        devedor: String,
        /// Mortgagee
        credor: String,
        /// Mortgaged property
        imovel: String,
    },
    /// Pledge (penhor) - Arts. 1431-1472
    Pledge {
        /// Pledgor
        devedor: String,
        /// Pledgee
        credor: String,
        /// Pledged object
        coisa: String,
    },
    /// Antichresis (anticrese) - Arts. 1506-1510
    Antichresis {
        /// Debtor
        devedor: String,
        /// Creditor
        credor: String,
        /// Property
        imovel: String,
    },
}

/// Property rights errors
#[derive(Debug, Clone, Error)]
pub enum PropertyError {
    /// Violation of social function (Art. 1228, §1)
    #[error("Violação da função social da propriedade (Art. 1228, §1º)")]
    SocialFunctionViolation,

    /// Invalid possession
    #[error("Posse inválida: {reason}")]
    InvalidPossession { reason: String },

    /// Adverse possession requirements not met
    #[error("Requisitos de usucapião não atendidos (Art. {article}): {reason}")]
    AdversePossessionRequirements { article: u16, reason: String },

    /// Invalid limited real right
    #[error("Direito real limitado inválido: {reason}")]
    InvalidLimitedRight { reason: String },

    /// Ownership dispute
    #[error("Conflito de propriedade: {description}")]
    OwnershipDispute { description: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for property operations
pub type PropertyResult<T> = Result<T, PropertyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possession_creation() {
        let possession = Possession::new("João", "Terreno", PossessionType::AsOwner)
            .with_good_faith(true)
            .with_just_title();
        assert!(possession.can_lead_to_adverse_possession());
    }

    #[test]
    fn test_ownership_social_function() {
        let ownership = Ownership::new("Maria", "Casa", PropertyKind::Urban);
        assert!(ownership.serves_social_function());
        assert_eq!(ownership.owner_faculties().len(), 4);
    }

    #[test]
    fn test_adverse_possession_years() {
        let extraordinary = AdversePossession {
            tipo: AdversePossessionType::Extraordinary,
            anos_posse: 15,
            requisitos_atendidos: true,
        };
        assert_eq!(extraordinary.required_years(), 15);
        assert!(extraordinary.is_complete());

        let special_urban = AdversePossession {
            tipo: AdversePossessionType::SpecialUrban,
            anos_posse: 5,
            requisitos_atendidos: true,
        };
        assert_eq!(special_urban.required_years(), 5);
        assert!(special_urban.is_complete());
    }

    #[test]
    fn test_limited_real_rights() {
        let usufruct = LimitedRealRight::Usufruct {
            usufrutuario: "Filho".to_string(),
            nu_proprietario: "Pai".to_string(),
            prazo_anos: Some(10),
        };
        assert!(matches!(usufruct, LimitedRealRight::Usufruct { .. }));
    }
}
