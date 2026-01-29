//! # Intellectual Property Law - Propriedade Industrial
//!
//! Brazilian IP law (Lei nº 9.279/1996 - LPI).
//!
//! ## Overview
//!
//! Brazil's Industrial Property Law covers:
//! - Patents (patentes)
//! - Trademarks (marcas)
//! - Industrial designs (desenhos industriais)
//! - Geographical indications (indicações geográficas)
//!
//! ## Authority
//!
//! INPI (Instituto Nacional da Propriedade Industrial) - National Institute of Industrial Property
//!
//! ## Key Rights
//!
//! | Type | Duration | Protection |
//! |------|----------|------------|
//! | Invention Patent | 20 years | Min 10 years from grant |
//! | Utility Model | 15 years | Min 7 years from grant |
//! | Trademark | 10 years | Renewable indefinitely |
//! | Industrial Design | 10 years | Max 25 years (5+5+5+5) |

#[cfg(test)]
use chrono::Datelike;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Intellectual property right
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntellectualProperty {
    /// IP type
    pub tipo: IpType,
    /// Title/name
    pub titulo: String,
    /// Owner (titular)
    pub titular: String,
    /// Registration number
    pub numero_registro: Option<String>,
    /// Filing date
    pub data_deposito: NaiveDate,
    /// Grant date
    pub data_concessao: Option<NaiveDate>,
    /// Whether active
    pub vigente: bool,
}

/// Types of intellectual property
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpType {
    /// Invention patent (patente de invenção) - Art. 40
    InventionPatent,
    /// Utility model (modelo de utilidade) - Art. 40
    UtilityModel,
    /// Trademark (marca) - Art. 133
    Trademark {
        /// Trademark type
        tipo_marca: TrademarkType,
        /// Nice classification class
        classe_nice: u8,
    },
    /// Industrial design (desenho industrial) - Art. 108
    IndustrialDesign,
    /// Geographical indication (indicação geográfica) - Arts. 176-182
    GeographicalIndication {
        /// GI type
        tipo_ig: GiType,
    },
}

/// Trademark types (Art. 123)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrademarkType {
    /// Product trademark (marca de produto)
    Product,
    /// Service trademark (marca de serviço)
    Service,
    /// Collective trademark (marca coletiva)
    Collective,
    /// Certification trademark (marca de certificação)
    Certification,
}

/// Geographical indication types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GiType {
    /// Indication of source (indicação de procedência)
    IndicationOfSource,
    /// Appellation of origin (denominação de origem)
    AppellationOfOrigin,
}

impl IntellectualProperty {
    /// Create a new invention patent
    pub fn invention_patent(
        titulo: impl Into<String>,
        titular: impl Into<String>,
        data_deposito: NaiveDate,
    ) -> Self {
        Self {
            tipo: IpType::InventionPatent,
            titulo: titulo.into(),
            titular: titular.into(),
            numero_registro: None,
            data_deposito,
            data_concessao: None,
            vigente: false,
        }
    }

    /// Create a new trademark
    pub fn trademark(
        titulo: impl Into<String>,
        titular: impl Into<String>,
        tipo_marca: TrademarkType,
        classe: u8,
        data_deposito: NaiveDate,
    ) -> Self {
        Self {
            tipo: IpType::Trademark {
                tipo_marca,
                classe_nice: classe,
            },
            titulo: titulo.into(),
            titular: titular.into(),
            numero_registro: None,
            data_deposito,
            data_concessao: None,
            vigente: false,
        }
    }

    /// Calculate expiration date based on IP type
    pub fn calculate_expiration(&self) -> Option<NaiveDate> {
        let concessao = self.data_concessao?;

        match &self.tipo {
            IpType::InventionPatent => {
                // 20 years from filing, min 10 from grant
                let from_filing = self
                    .data_deposito
                    .checked_add_signed(chrono::Duration::days(365 * 20))?;
                let from_grant = concessao.checked_add_signed(chrono::Duration::days(365 * 10))?;
                Some(from_filing.max(from_grant))
            }
            IpType::UtilityModel => {
                // 15 years from filing, min 7 from grant
                let from_filing = self
                    .data_deposito
                    .checked_add_signed(chrono::Duration::days(365 * 15))?;
                let from_grant = concessao.checked_add_signed(chrono::Duration::days(365 * 7))?;
                Some(from_filing.max(from_grant))
            }
            IpType::Trademark { .. } => {
                // 10 years from grant, renewable
                concessao.checked_add_signed(chrono::Duration::days(365 * 10))
            }
            IpType::IndustrialDesign => {
                // 10 years from filing
                self.data_deposito
                    .checked_add_signed(chrono::Duration::days(365 * 10))
            }
            IpType::GeographicalIndication { .. } => {
                // No expiration
                None
            }
        }
    }

    /// Check if IP is still valid
    pub fn is_valid(&self, reference_date: NaiveDate) -> bool {
        if !self.vigente {
            return false;
        }

        if let Some(expiration) = self.calculate_expiration() {
            reference_date <= expiration
        } else {
            true // No expiration (e.g., GI)
        }
    }

    /// Mark as granted
    pub fn grant(mut self, data_concessao: NaiveDate, numero: impl Into<String>) -> Self {
        self.data_concessao = Some(data_concessao);
        self.numero_registro = Some(numero.into());
        self.vigente = true;
        self
    }
}

/// Patentability requirements (Arts. 8-11)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatentabilityCheck {
    /// Whether invention is new (novelty)
    pub novidade: bool,
    /// Whether invention has inventive step
    pub atividade_inventiva: bool,
    /// Whether invention has industrial application
    pub aplicacao_industrial: bool,
}

impl PatentabilityCheck {
    /// Check if meets all patentability requirements (Art. 8)
    pub fn is_patentable(&self) -> bool {
        self.novidade && self.atividade_inventiva && self.aplicacao_industrial
    }
}

/// Non-patentable subject matter (Arts. 10, 18)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonPatentable {
    /// Scientific discoveries (Art. 10, I)
    ScientificDiscovery,
    /// Mathematical methods (Art. 10, I)
    MathematicalMethod,
    /// Therapeutic methods (Art. 10, VIII)
    TherapeuticMethod,
    /// Living beings (except microorganisms) - Art. 10, IX
    LivingBeings,
    /// Against morality (Art. 18, I)
    AgainstMorality,
    /// Against public health (Art. 18, II)
    AgainstPublicHealth,
}

/// Trademark infringement (Art. 189)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrademarkInfringement {
    /// Infringing mark
    pub marca_infratora: String,
    /// Protected mark
    pub marca_protegida: String,
    /// Type of infringement
    pub tipo: InfringementType,
}

/// Types of infringement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfringementType {
    /// Reproduction (reprodução)
    Reproduction,
    /// Imitation (imitação)
    Imitation,
    /// Importation (importação)
    Importation,
    /// Counterfeiting (falsificação)
    Counterfeiting,
}

/// IP crimes (Arts. 183-195)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpCrime {
    /// Patent infringement (Art. 183)
    PatentInfringement {
        /// Patent number
        patente: String,
    },
    /// Trademark infringement (Art. 189)
    TrademarkInfringement {
        /// Trademark
        marca: String,
    },
    /// False geographical indication (Art. 192)
    FalseGeographicalIndication {
        /// GI name
        indicacao: String,
    },
    /// Unfair competition (Art. 195)
    UnfairCompetition {
        /// Description
        descricao: String,
    },
}

/// IP errors
#[derive(Debug, Clone, Error)]
pub enum IpError {
    /// Not patentable
    #[error("Invenção não patenteável (Arts. 8, 10, 18): {reason}")]
    NotPatentable { reason: String },

    /// Trademark conflict
    #[error("Conflito de marca (Art. 124): {conflicting_mark}")]
    TrademarkConflict { conflicting_mark: String },

    /// Expired IP
    #[error("Direito de propriedade industrial expirado: {expiration_date}")]
    ExpiredIp { expiration_date: NaiveDate },

    /// Infringement detected
    #[error("Violação de propriedade industrial (Arts. 183-195): {infringement:?}")]
    Infringement { infringement: IpCrime },

    /// Invalid registration
    #[error("Registro inválido: {reason}")]
    InvalidRegistration { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for IP operations
pub type IpResult<T> = Result<T, IpError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invention_patent() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let patent =
            IntellectualProperty::invention_patent("Novo método de produção", "ACME Ltda.", date);
        assert!(matches!(patent.tipo, IpType::InventionPatent));
    }

    #[test]
    fn test_trademark() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let trademark =
            IntellectualProperty::trademark("ACME", "ACME Ltda.", TrademarkType::Product, 35, date);
        if let IpType::Trademark {
            tipo_marca,
            classe_nice,
        } = trademark.tipo
        {
            assert_eq!(tipo_marca, TrademarkType::Product);
            assert_eq!(classe_nice, 35);
        } else {
            panic!("Expected trademark type");
        }
    }

    #[test]
    fn test_patentability() {
        let check = PatentabilityCheck {
            novidade: true,
            atividade_inventiva: true,
            aplicacao_industrial: true,
        };
        assert!(check.is_patentable());

        let not_patentable = PatentabilityCheck {
            novidade: false,
            atividade_inventiva: true,
            aplicacao_industrial: true,
        };
        assert!(!not_patentable.is_patentable());
    }

    #[test]
    fn test_patent_expiration() {
        let filing = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        let grant = NaiveDate::from_ymd_opt(2023, 1, 1).expect("valid date");

        let patent = IntellectualProperty::invention_patent("Invenção", "Titular", filing)
            .grant(grant, "BR1234567");

        let expiration = patent
            .calculate_expiration()
            .expect("should have expiration");
        // Should be 20 years from filing (2039-12-31) or 10 years from grant (2032-12-31), whichever is later
        assert_eq!(expiration.year(), 2039);
    }
}
