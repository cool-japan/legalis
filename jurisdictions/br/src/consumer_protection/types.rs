//! CDC Consumer Protection Types

use crate::citation::{RomanNumeral, format_cdc_citation};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Consumer definition per CDC Art. 2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Consumer {
    /// Consumer identifier (CPF for individual, CNPJ for legal entity)
    pub identifier: String,
    /// Name in Portuguese
    pub nome_pt: String,
    /// Name in English
    pub name_en: String,
    /// Whether consumer is a legal entity acquiring as final destination
    pub is_legal_entity: bool,
    /// Whether considered vulnerable consumer (elderly, child, etc.)
    pub is_hypervulnerable: bool,
}

impl Consumer {
    /// Create a new individual consumer
    pub fn individual(cpf: impl Into<String>, nome: impl Into<String>) -> Self {
        Self {
            identifier: cpf.into(),
            nome_pt: nome.into(),
            name_en: String::new(),
            is_legal_entity: false,
            is_hypervulnerable: false,
        }
    }

    /// Create a legal entity consumer
    pub fn legal_entity(cnpj: impl Into<String>, nome: impl Into<String>) -> Self {
        Self {
            identifier: cnpj.into(),
            nome_pt: nome.into(),
            name_en: String::new(),
            is_legal_entity: true,
            is_hypervulnerable: false,
        }
    }

    /// Mark as hypervulnerable (elderly, child, illiterate, sick)
    pub fn with_hypervulnerability(mut self) -> Self {
        self.is_hypervulnerable = true;
        self
    }
}

/// Provider/Supplier definition per CDC Art. 3
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provider {
    /// Provider identifier (CNPJ)
    pub cnpj: String,
    /// Company name in Portuguese
    pub nome_pt: String,
    /// Trade name
    pub nome_fantasia: Option<String>,
    /// Provider type
    pub provider_type: ProviderType,
    /// Main activity
    pub atividade_principal: String,
}

/// Provider type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    /// Product manufacturer
    Manufacturer,
    /// Service provider
    ServiceProvider,
    /// Retailer
    Retailer,
    /// Importer
    Importer,
    /// Distributor
    Distributor,
}

impl ProviderType {
    /// Get liability level under CDC
    pub fn liability_level(&self) -> &'static str {
        match self {
            Self::Manufacturer | Self::Importer => "Primary (solidary)",
            Self::Retailer => "Subsidiary (when manufacturer unknown)",
            Self::Distributor => "Subsidiary",
            Self::ServiceProvider => "Primary (strict liability)",
        }
    }
}

/// Product definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Product {
    /// Product name
    pub nome_pt: String,
    /// Product description
    pub description: String,
    /// Whether product is durable (affects warranty)
    pub is_durable: bool,
    /// Whether product is essential
    pub is_essential: bool,
    /// Product category
    pub category: ProductCategory,
}

/// Product category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductCategory {
    /// Food products
    Food,
    /// Medicine/pharmaceutical
    Pharmaceutical,
    /// Electronics
    Electronics,
    /// Vehicle
    Vehicle,
    /// Appliance
    Appliance,
    /// Clothing
    Clothing,
    /// Furniture
    Furniture,
    /// Other
    Other,
}

impl Product {
    /// Get warranty period in days (Art. 26)
    pub fn warranty_period_days(&self) -> u32 {
        if self.is_durable {
            90 // Durable goods: 90 days
        } else {
            30 // Non-durable goods: 30 days
        }
    }

    /// Get statutory citation for warranty
    pub fn warranty_citation(&self) -> String {
        format_cdc_citation(26, None, None)
    }
}

/// Product defect types (Art. 12)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductDefect {
    /// Design defect - flaw in product conception
    Design { description: String },
    /// Manufacturing defect - flaw in production
    Manufacturing { description: String },
    /// Information defect - inadequate warnings/instructions
    Information { missing_info: String },
}

impl ProductDefect {
    /// Get legal citation for defect type
    pub fn citation(&self) -> String {
        format_cdc_citation(12, Some(1), None)
    }

    /// Get defect type name in Portuguese
    pub fn nome_pt(&self) -> &'static str {
        match self {
            Self::Design { .. } => "Defeito de concepção",
            Self::Manufacturing { .. } => "Defeito de fabricação",
            Self::Information { .. } => "Defeito de informação",
        }
    }
}

/// Provider liability determination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLiability {
    /// Whether liability applies
    pub is_liable: bool,
    /// Liability type
    pub liability_type: LiabilityType,
    /// Grounds for liability exclusion (Art. 12, §3)
    pub exclusion_ground: Option<LiabilityExclusion>,
}

/// Liability type under CDC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityType {
    /// Strict/objective liability (responsabilidade objetiva)
    Strict,
    /// Solidary liability (responsabilidade solidária)
    Solidary,
    /// Subsidiary liability
    Subsidiary,
}

/// Grounds for liability exclusion (Art. 12, §3)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityExclusion {
    /// Provider did not place product in market
    NotPlacedInMarket,
    /// Product has no defect
    NoDefect,
    /// Exclusive fault of consumer or third party
    ExclusiveFault { description: String },
}

impl LiabilityExclusion {
    /// Get citation for exclusion ground
    pub fn citation(&self) -> String {
        let inciso = match self {
            Self::NotPlacedInMarket => RomanNumeral::I,
            Self::NoDefect => RomanNumeral::II,
            Self::ExclusiveFault { .. } => RomanNumeral::III,
        };
        format_cdc_citation(12, Some(3), Some(inciso))
    }
}

/// Consumer rights enumeration (Art. 6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumerRight {
    /// Protection of life, health, safety (Art. 6, I)
    LifeHealthSafety,
    /// Education about consumption (Art. 6, II)
    Education,
    /// Adequate information (Art. 6, III)
    Information,
    /// Protection against misleading advertising (Art. 6, IV)
    AdvertisingProtection,
    /// Contract modification (Art. 6, V)
    ContractModification,
    /// Damage reparation (Art. 6, VI)
    DamageReparation,
    /// Access to justice (Art. 6, VII)
    AccessToJustice,
    /// Rights defense facilitation (Art. 6, VIII)
    RightsDefenseFacilitation,
    /// Public service quality (Art. 6, X)
    PublicServiceQuality,
}

impl ConsumerRight {
    /// Get article citation
    pub fn citation(&self) -> String {
        let inciso = match self {
            Self::LifeHealthSafety => RomanNumeral::I,
            Self::Education => RomanNumeral::II,
            Self::Information => RomanNumeral::III,
            Self::AdvertisingProtection => RomanNumeral::IV,
            Self::ContractModification => RomanNumeral::V,
            Self::DamageReparation => RomanNumeral::VI,
            Self::AccessToJustice => RomanNumeral::VII,
            Self::RightsDefenseFacilitation => RomanNumeral::VIII,
            Self::PublicServiceQuality => RomanNumeral::X,
        };
        format_cdc_citation(6, None, Some(inciso))
    }

    /// Get right description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::LifeHealthSafety => "Proteção da vida, saúde e segurança",
            Self::Education => "Educação para o consumo",
            Self::Information => "Informação adequada e clara",
            Self::AdvertisingProtection => "Proteção contra publicidade enganosa",
            Self::ContractModification => "Modificação de cláusulas desproporcionais",
            Self::DamageReparation => "Reparação de danos",
            Self::AccessToJustice => "Acesso à justiça",
            Self::RightsDefenseFacilitation => "Facilitação da defesa de direitos",
            Self::PublicServiceQuality => "Qualidade dos serviços públicos",
        }
    }
}

/// Abusive clause types (Art. 51)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbusiveClauseType {
    /// Exclusion/limitation of liability (Art. 51, I)
    LiabilityExclusion,
    /// Withdrawal of indemnification option (Art. 51, II)
    IndemnificationWithdrawal,
    /// Transferring responsibility to third party (Art. 51, III)
    ResponsibilityTransfer,
    /// Consumer disadvantage obligations (Art. 51, IV)
    DisadvantageousObligation,
    /// Mandatory arbitration (Art. 51, VII)
    MandatoryArbitration,
    /// Supplier representative designation (Art. 51, VIII)
    SupplierRepresentative,
    /// Unilateral contract modification (Art. 51, XIII)
    UnilateralModification,
    /// Non-compliance penalty violation (Art. 51, XIV)
    PenaltyViolation,
    /// Unilateral termination (Art. 51, XI)
    UnilateralTermination,
}

impl AbusiveClauseType {
    /// Get legal citation
    pub fn citation(&self) -> String {
        let inciso = match self {
            Self::LiabilityExclusion => RomanNumeral::I,
            Self::IndemnificationWithdrawal => RomanNumeral::II,
            Self::ResponsibilityTransfer => RomanNumeral::III,
            Self::DisadvantageousObligation => RomanNumeral::IV,
            Self::MandatoryArbitration => RomanNumeral::VII,
            Self::SupplierRepresentative => RomanNumeral::VIII,
            Self::UnilateralModification => RomanNumeral::XIII,
            Self::PenaltyViolation => RomanNumeral::XIV,
            Self::UnilateralTermination => RomanNumeral::XI,
        };
        format_cdc_citation(51, None, Some(inciso))
    }

    /// Get description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::LiabilityExclusion => "Cláusula que exclui responsabilidade do fornecedor",
            Self::IndemnificationWithdrawal => "Retirada da opção de reembolso",
            Self::ResponsibilityTransfer => "Transferência de responsabilidade a terceiros",
            Self::DisadvantageousObligation => "Obrigações iníquas ou desvantajosas",
            Self::MandatoryArbitration => "Arbitragem compulsória",
            Self::SupplierRepresentative => "Representante imposto pelo fornecedor",
            Self::UnilateralModification => "Modificação unilateral do contrato",
            Self::PenaltyViolation => "Multa excessiva ao consumidor",
            Self::UnilateralTermination => "Rescisão unilateral pelo fornecedor",
        }
    }
}

/// Abusive clause instance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbusiveClause {
    /// Clause type
    pub clause_type: AbusiveClauseType,
    /// Actual clause text
    pub text: String,
    /// Contract location
    pub location: Option<String>,
}

impl AbusiveClause {
    /// Create a new abusive clause
    pub fn new(clause_type: AbusiveClauseType, text: String) -> Self {
        Self {
            clause_type,
            text,
            location: None,
        }
    }

    /// All abusive clauses are null and void (nulas de pleno direito)
    pub fn is_null_and_void(&self) -> bool {
        true // Art. 51 caput: all listed clauses are automatically null
    }

    /// Get legal consequence
    pub fn legal_consequence_pt(&self) -> &'static str {
        "Nula de pleno direito (Art. 51, caput)"
    }
}

/// Contract type for consumer relations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Standard adhesion contract
    Adhesion,
    /// Distance contract (internet, phone)
    Distance,
    /// Doorstep sale
    Doorstep,
    /// Credit/financing contract
    Credit,
    /// Service contract
    Service,
    /// Purchase and sale
    Sale,
}

impl ContractType {
    /// Check if withdrawal right applies (Art. 49)
    pub fn has_withdrawal_right(&self) -> bool {
        matches!(self, Self::Distance | Self::Doorstep)
    }

    /// Get description
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::Adhesion => "Contrato de adesão",
            Self::Distance => "Contratação à distância",
            Self::Doorstep => "Venda fora do estabelecimento comercial",
            Self::Credit => "Contrato de crédito/financiamento",
            Self::Service => "Contrato de prestação de serviços",
            Self::Sale => "Contrato de compra e venda",
        }
    }
}

/// Consumer contract
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerContract {
    /// Contract type
    pub contract_type: ContractType,
    /// Contract date
    pub data_contratacao: NaiveDate,
    /// Consumer
    pub consumer: Consumer,
    /// Provider
    pub provider: Provider,
    /// Contract value in centavos
    pub valor_centavos: i64,
    /// Whether contract was signed outside business premises
    pub fora_estabelecimento: bool,
}

impl ConsumerContract {
    /// Check if 7-day withdrawal right applies (Art. 49)
    pub fn has_withdrawal_right(&self) -> bool {
        self.fora_estabelecimento || self.contract_type.has_withdrawal_right()
    }

    /// Get withdrawal deadline
    pub fn withdrawal_deadline(&self) -> NaiveDate {
        self.data_contratacao + chrono::Duration::days(7)
    }

    /// Get citation for withdrawal right
    pub fn withdrawal_citation(&self) -> String {
        format_cdc_citation(49, None, None)
    }
}

/// Withdrawal right (direito de arrependimento - Art. 49)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalRight {
    /// Days since purchase/contract
    pub days_since_purchase: u32,
    /// Whether product was returned
    pub product_returned: bool,
    /// Whether refund was requested
    pub refund_requested: bool,
}

impl WithdrawalRight {
    /// Create withdrawal right with days elapsed
    pub fn new(days_since_purchase: u32) -> Self {
        Self {
            days_since_purchase,
            product_returned: false,
            refund_requested: false,
        }
    }

    /// Check if withdrawal is still valid (within 7 days)
    pub fn is_valid(&self) -> bool {
        self.days_since_purchase <= 7
    }

    /// Days remaining for withdrawal
    pub fn days_remaining(&self) -> Option<u32> {
        if self.days_since_purchase <= 7 {
            Some(7 - self.days_since_purchase)
        } else {
            None
        }
    }

    /// Get legal citation
    pub fn citation(&self) -> String {
        format_cdc_citation(49, None, None)
    }
}

/// Product recall information (Art. 10)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recall {
    /// Product name
    pub produto: String,
    /// Manufacturer
    pub fabricante: String,
    /// Recall reason
    pub motivo: String,
    /// Recall start date
    pub data_inicio: NaiveDate,
    /// Affected units
    pub unidades_afetadas: Option<u64>,
    /// Whether authorities were notified
    pub autoridades_notificadas: bool,
}

impl Recall {
    /// Check if recall complies with CDC requirements
    pub fn is_compliant(&self) -> bool {
        self.autoridades_notificadas
    }

    /// Get citation for recall obligations
    pub fn citation(&self) -> String {
        format_cdc_citation(10, Some(1), None)
    }
}

/// Consumer compliance status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsumerCompliance {
    /// Overall compliance
    pub compliant: bool,
    /// Information adequacy (Art. 6, III)
    pub information_adequate: bool,
    /// No abusive clauses
    pub no_abusive_clauses: bool,
    /// Liability properly attributed
    pub liability_proper: bool,
    /// Withdrawal right respected
    pub withdrawal_respected: bool,
    /// Warranty terms compliant
    pub warranty_compliant: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_creation() {
        let consumer = Consumer::individual("12345678909", "João da Silva");
        assert!(!consumer.is_legal_entity);
        assert!(!consumer.is_hypervulnerable);
    }

    #[test]
    fn test_hypervulnerable_consumer() {
        let consumer = Consumer::individual("12345678909", "Maria Idosa").with_hypervulnerability();
        assert!(consumer.is_hypervulnerable);
    }

    #[test]
    fn test_product_warranty_durable() {
        let product = Product {
            nome_pt: "Televisor".to_string(),
            description: "TV LED 55 polegadas".to_string(),
            is_durable: true,
            is_essential: false,
            category: ProductCategory::Electronics,
        };
        assert_eq!(product.warranty_period_days(), 90);
    }

    #[test]
    fn test_product_warranty_nondurable() {
        let product = Product {
            nome_pt: "Alimento".to_string(),
            description: "Produto alimentício".to_string(),
            is_durable: false,
            is_essential: true,
            category: ProductCategory::Food,
        };
        assert_eq!(product.warranty_period_days(), 30);
    }

    #[test]
    fn test_abusive_clause_null() {
        let clause = AbusiveClause::new(
            AbusiveClauseType::LiabilityExclusion,
            "O fornecedor não se responsabiliza por defeitos".to_string(),
        );
        assert!(clause.is_null_and_void());
    }

    #[test]
    fn test_withdrawal_right_valid() {
        let withdrawal = WithdrawalRight::new(5);
        assert!(withdrawal.is_valid());
        assert_eq!(withdrawal.days_remaining(), Some(2));
    }

    #[test]
    fn test_withdrawal_right_expired() {
        let withdrawal = WithdrawalRight::new(10);
        assert!(!withdrawal.is_valid());
        assert_eq!(withdrawal.days_remaining(), None);
    }

    #[test]
    fn test_contract_withdrawal_distance() {
        let contract = ConsumerContract {
            contract_type: ContractType::Distance,
            data_contratacao: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            consumer: Consumer::individual("12345678909", "Test"),
            provider: Provider {
                cnpj: "11222333000181".to_string(),
                nome_pt: "Loja Online Ltda".to_string(),
                nome_fantasia: Some("Loja Online".to_string()),
                provider_type: ProviderType::Retailer,
                atividade_principal: "Comércio eletrônico".to_string(),
            },
            valor_centavos: 10000,
            fora_estabelecimento: true,
        };
        assert!(contract.has_withdrawal_right());
    }

    #[test]
    fn test_consumer_right_citation() {
        let right = ConsumerRight::Information;
        assert!(right.citation().contains("Art. 6"));
    }

    #[test]
    fn test_abusive_clause_citation() {
        let clause_type = AbusiveClauseType::MandatoryArbitration;
        assert!(clause_type.citation().contains("Art. 51"));
    }

    #[test]
    fn test_product_defect() {
        let defect = ProductDefect::Manufacturing {
            description: "Peça mal encaixada".to_string(),
        };
        assert_eq!(defect.nome_pt(), "Defeito de fabricação");
    }

    #[test]
    fn test_liability_exclusion_citation() {
        let exclusion = LiabilityExclusion::NoDefect;
        let citation = exclusion.citation();
        assert!(citation.contains("Art. 12"));
        assert!(citation.contains("§3"));
    }
}
