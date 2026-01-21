//! Common Brazilian legal types

use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Brazilian currency (Real - BRL)
///
/// Represents monetary values in Brazilian Reais.
/// Uses integer representation for centavos (cents) to avoid floating-point errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BrazilianCurrency {
    /// Amount in centavos (cents). 100 centavos = 1 Real
    pub centavos: i64,
}

impl BrazilianCurrency {
    /// Create a new Brazilian currency value from reais
    pub fn from_reais(reais: i64) -> Self {
        Self {
            centavos: reais * 100,
        }
    }

    /// Create a new Brazilian currency value from centavos
    pub fn from_centavos(centavos: i64) -> Self {
        Self { centavos }
    }

    /// Get the value in reais (integer part)
    pub fn reais(&self) -> i64 {
        self.centavos / 100
    }

    /// Get the centavos part (remainder)
    pub fn cents(&self) -> i64 {
        self.centavos % 100
    }

    /// Convert to float (for display purposes only)
    pub fn to_f64(&self) -> f64 {
        self.centavos as f64 / 100.0
    }
}

impl fmt::Display for BrazilianCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R$ {},{:02}", self.reais(), self.cents().abs())
    }
}

/// Brazilian date with deadline calculation utilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrazilianDate {
    pub date: DateTime<Utc>,
}

impl BrazilianDate {
    /// Create a new Brazilian date
    pub fn new(date: DateTime<Utc>) -> Self {
        Self { date }
    }

    /// Create from current time
    pub fn now() -> Self {
        Self { date: Utc::now() }
    }

    /// Add calendar days (working + non-working days)
    pub fn add_calendar_days(&self, days: i64) -> Self {
        Self {
            date: self.date + Duration::days(days),
        }
    }

    /// Add business days (Monday-Friday, excluding holidays - simplified)
    /// Note: This is a simplified version that doesn't account for Brazilian holidays
    pub fn add_business_days(&self, days: i64) -> Self {
        let mut current = self.date;
        let mut remaining_days = days;

        while remaining_days > 0 {
            current += Duration::days(1);
            let weekday = current.weekday();
            // Skip weekends
            if weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun {
                remaining_days -= 1;
            }
        }

        Self { date: current }
    }

    /// Check if the deadline has passed
    pub fn is_expired(&self) -> bool {
        self.date < Utc::now()
    }

    /// Get days until this date
    pub fn days_until(&self) -> i64 {
        let duration = self.date.signed_duration_since(Utc::now());
        duration.num_days()
    }
}

/// Brazilian document types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// CPF - Cadastro de Pessoas Físicas (Individual Taxpayer ID)
    CPF(String),
    /// CNPJ - Cadastro Nacional da Pessoa Jurídica (Corporate Taxpayer ID)
    CNPJ(String),
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentType::CPF(cpf) => write!(f, "CPF: {}", cpf),
            DocumentType::CNPJ(cnpj) => write!(f, "CNPJ: {}", cnpj),
        }
    }
}

/// Brazilian document structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrazilianDocument {
    /// Document type and number
    pub doc_type: DocumentType,
    /// Name (full name for individuals, company name for corporations)
    pub nome_pt: String,
    /// English translation of name
    pub name_en: String,
}

/// Document validation error
#[derive(Debug, Error, PartialEq)]
pub enum DocumentError {
    #[error("Invalid CPF format: {0}")]
    InvalidCPFFormat(String),
    #[error("Invalid CNPJ format: {0}")]
    InvalidCNPJFormat(String),
    #[error("Invalid CPF checksum")]
    InvalidCPFChecksum,
    #[error("Invalid CNPJ checksum")]
    InvalidCNPJChecksum,
}

/// Validate CPF format (XXX.XXX.XXX-XX or XXXXXXXXXXX)
///
/// # Examples
///
/// ```
/// use legalis_br::common::validate_cpf;
///
/// assert!(validate_cpf("123.456.789-09").is_ok());
/// assert!(validate_cpf("12345678909").is_ok());
/// assert!(validate_cpf("123").is_err());
/// ```
pub fn validate_cpf(cpf: &str) -> Result<(), DocumentError> {
    // Remove formatting characters
    let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    // Check length
    if digits.len() != 11 {
        return Err(DocumentError::InvalidCPFFormat(
            "CPF must have 11 digits".to_string(),
        ));
    }

    // Check if all digits are the same (invalid CPF)
    if digits
        .chars()
        .all(|c| c == digits.chars().next().unwrap_or('0'))
    {
        return Err(DocumentError::InvalidCPFChecksum);
    }

    // Validate checksum digits
    let digits_vec: Vec<u32> = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();

    // First check digit
    let sum: u32 = digits_vec
        .iter()
        .take(9)
        .enumerate()
        .map(|(i, &d)| d * (10 - i as u32))
        .sum();
    let remainder = sum % 11;
    let check1 = if remainder < 2 { 0 } else { 11 - remainder };
    if check1 != digits_vec[9] {
        return Err(DocumentError::InvalidCPFChecksum);
    }

    // Second check digit
    let sum: u32 = digits_vec
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, &d)| d * (11 - i as u32))
        .sum();
    let remainder = sum % 11;
    let check2 = if remainder < 2 { 0 } else { 11 - remainder };
    if check2 != digits_vec[10] {
        return Err(DocumentError::InvalidCPFChecksum);
    }

    Result::Ok(())
}

/// Validate CNPJ format (XX.XXX.XXX/XXXX-XX or XXXXXXXXXXXXXX)
///
/// # Examples
///
/// ```
/// use legalis_br::common::validate_cnpj;
///
/// // Valid CNPJ with correct check digits
/// assert!(validate_cnpj("11.444.777/0001-61").is_ok());
/// assert!(validate_cnpj("11444777000161").is_ok());
/// assert!(validate_cnpj("123").is_err());
/// ```
pub fn validate_cnpj(cnpj: &str) -> Result<(), DocumentError> {
    // Remove formatting characters
    let digits: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

    // Check length
    if digits.len() != 14 {
        return Err(DocumentError::InvalidCNPJFormat(
            "CNPJ must have 14 digits".to_string(),
        ));
    }

    // Check if all digits are the same (invalid CNPJ)
    if digits
        .chars()
        .all(|c| c == digits.chars().next().unwrap_or('0'))
    {
        return Err(DocumentError::InvalidCNPJChecksum);
    }

    let digits_vec: Vec<u32> = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();

    // First check digit
    let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum = 0;
    for i in 0..12 {
        sum += digits_vec[i] * weights1[i];
    }
    let remainder = sum % 11;
    let check1 = if remainder < 2 { 0 } else { 11 - remainder };
    if check1 != digits_vec[12] {
        return Err(DocumentError::InvalidCNPJChecksum);
    }

    // Second check digit
    let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    sum = 0;
    for i in 0..13 {
        sum += digits_vec[i] * weights2[i];
    }
    let remainder = sum % 11;
    let check2 = if remainder < 2 { 0 } else { 11 - remainder };
    if check2 != digits_vec[13] {
        return Err(DocumentError::InvalidCNPJChecksum);
    }

    Result::Ok(())
}

/// Brazilian states (Estados)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrazilianState {
    // North Region (Norte)
    AC, // Acre
    AP, // Amapá
    AM, // Amazonas
    PA, // Pará
    RO, // Rondônia
    RR, // Roraima
    TO, // Tocantins

    // Northeast Region (Nordeste)
    AL, // Alagoas
    BA, // Bahia
    CE, // Ceará
    MA, // Maranhão
    PB, // Paraíba
    PE, // Pernambuco
    PI, // Piauí
    RN, // Rio Grande do Norte
    SE, // Sergipe

    // Central-West Region (Centro-Oeste)
    DF, // Distrito Federal (Federal District)
    GO, // Goiás
    MT, // Mato Grosso
    MS, // Mato Grosso do Sul

    // Southeast Region (Sudeste)
    ES, // Espírito Santo
    MG, // Minas Gerais
    RJ, // Rio de Janeiro
    SP, // São Paulo

    // South Region (Sul)
    PR, // Paraná
    RS, // Rio Grande do Sul
    SC, // Santa Catarina
}

impl BrazilianState {
    /// Get the full name in Portuguese
    pub fn nome_pt(&self) -> &'static str {
        match self {
            BrazilianState::AC => "Acre",
            BrazilianState::AL => "Alagoas",
            BrazilianState::AP => "Amapá",
            BrazilianState::AM => "Amazonas",
            BrazilianState::BA => "Bahia",
            BrazilianState::CE => "Ceará",
            BrazilianState::DF => "Distrito Federal",
            BrazilianState::ES => "Espírito Santo",
            BrazilianState::GO => "Goiás",
            BrazilianState::MA => "Maranhão",
            BrazilianState::MT => "Mato Grosso",
            BrazilianState::MS => "Mato Grosso do Sul",
            BrazilianState::MG => "Minas Gerais",
            BrazilianState::PA => "Pará",
            BrazilianState::PB => "Paraíba",
            BrazilianState::PR => "Paraná",
            BrazilianState::PE => "Pernambuco",
            BrazilianState::PI => "Piauí",
            BrazilianState::RJ => "Rio de Janeiro",
            BrazilianState::RN => "Rio Grande do Norte",
            BrazilianState::RS => "Rio Grande do Sul",
            BrazilianState::RO => "Rondônia",
            BrazilianState::RR => "Roraima",
            BrazilianState::SC => "Santa Catarina",
            BrazilianState::SP => "São Paulo",
            BrazilianState::SE => "Sergipe",
            BrazilianState::TO => "Tocantins",
        }
    }

    /// Get the abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            BrazilianState::AC => "AC",
            BrazilianState::AL => "AL",
            BrazilianState::AP => "AP",
            BrazilianState::AM => "AM",
            BrazilianState::BA => "BA",
            BrazilianState::CE => "CE",
            BrazilianState::DF => "DF",
            BrazilianState::ES => "ES",
            BrazilianState::GO => "GO",
            BrazilianState::MA => "MA",
            BrazilianState::MT => "MT",
            BrazilianState::MS => "MS",
            BrazilianState::MG => "MG",
            BrazilianState::PA => "PA",
            BrazilianState::PB => "PB",
            BrazilianState::PR => "PR",
            BrazilianState::PE => "PE",
            BrazilianState::PI => "PI",
            BrazilianState::RJ => "RJ",
            BrazilianState::RN => "RN",
            BrazilianState::RS => "RS",
            BrazilianState::RO => "RO",
            BrazilianState::RR => "RR",
            BrazilianState::SC => "SC",
            BrazilianState::SP => "SP",
            BrazilianState::SE => "SE",
            BrazilianState::TO => "TO",
        }
    }

    /// Get the region
    pub fn region_pt(&self) -> &'static str {
        match self {
            BrazilianState::AC
            | BrazilianState::AP
            | BrazilianState::AM
            | BrazilianState::PA
            | BrazilianState::RO
            | BrazilianState::RR
            | BrazilianState::TO => "Norte",
            BrazilianState::AL
            | BrazilianState::BA
            | BrazilianState::CE
            | BrazilianState::MA
            | BrazilianState::PB
            | BrazilianState::PE
            | BrazilianState::PI
            | BrazilianState::RN
            | BrazilianState::SE => "Nordeste",
            BrazilianState::DF | BrazilianState::GO | BrazilianState::MT | BrazilianState::MS => {
                "Centro-Oeste"
            }
            BrazilianState::ES | BrazilianState::MG | BrazilianState::RJ | BrazilianState::SP => {
                "Sudeste"
            }
            BrazilianState::PR | BrazilianState::RS | BrazilianState::SC => "Sul",
        }
    }
}

impl fmt::Display for BrazilianState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

/// Federal entity (União, Estados, Municípios)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederalEntity {
    /// União (Federal government)
    Union,
    /// Estado (State)
    State(BrazilianState),
    /// Município (Municipality)
    Municipality(Municipality),
}

/// Municipality information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Municipality {
    /// Municipality name in Portuguese
    pub nome_pt: String,
    /// English translation
    pub name_en: String,
    /// State
    pub state: BrazilianState,
    /// IBGE code (Instituto Brasileiro de Geografia e Estatística)
    pub ibge_code: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brazilian_currency() {
        let amount = BrazilianCurrency::from_reais(100);
        assert_eq!(amount.reais(), 100);
        assert_eq!(amount.cents(), 0);
        assert_eq!(amount.to_string(), "R$ 100,00");
    }

    #[test]
    fn test_brazilian_currency_with_cents() {
        let amount = BrazilianCurrency::from_centavos(12345);
        assert_eq!(amount.reais(), 123);
        assert_eq!(amount.cents(), 45);
        assert_eq!(amount.to_string(), "R$ 123,45");
    }

    #[test]
    fn test_cpf_validation_valid() {
        // Valid CPF: 123.456.789-09
        assert!(validate_cpf("12345678909").is_ok());
        assert!(validate_cpf("123.456.789-09").is_ok());
    }

    #[test]
    fn test_cpf_validation_invalid_length() {
        assert!(validate_cpf("123").is_err());
        assert!(validate_cpf("12345678901234").is_err());
    }

    #[test]
    fn test_cpf_validation_invalid_checksum() {
        assert!(validate_cpf("12345678900").is_err());
    }

    #[test]
    fn test_cpf_validation_all_same_digits() {
        assert!(validate_cpf("11111111111").is_err());
        assert!(validate_cpf("00000000000").is_err());
    }

    #[test]
    fn test_cnpj_validation_valid() {
        // Valid CNPJ: 11.222.333/0001-81
        assert!(validate_cnpj("11222333000181").is_ok());
        assert!(validate_cnpj("11.222.333/0001-81").is_ok());
    }

    #[test]
    fn test_cnpj_validation_invalid_length() {
        assert!(validate_cnpj("123").is_err());
    }

    #[test]
    fn test_cnpj_validation_all_same_digits() {
        assert!(validate_cnpj("11111111111111").is_err());
        assert!(validate_cnpj("00000000000000").is_err());
    }

    #[test]
    fn test_brazilian_state_names() {
        assert_eq!(BrazilianState::SP.nome_pt(), "São Paulo");
        assert_eq!(BrazilianState::RJ.nome_pt(), "Rio de Janeiro");
        assert_eq!(BrazilianState::DF.nome_pt(), "Distrito Federal");
    }

    #[test]
    fn test_brazilian_state_abbreviation() {
        assert_eq!(BrazilianState::SP.abbreviation(), "SP");
        assert_eq!(BrazilianState::RJ.abbreviation(), "RJ");
    }

    #[test]
    fn test_brazilian_state_region() {
        assert_eq!(BrazilianState::SP.region_pt(), "Sudeste");
        assert_eq!(BrazilianState::BA.region_pt(), "Nordeste");
        assert_eq!(BrazilianState::AM.region_pt(), "Norte");
        assert_eq!(BrazilianState::RS.region_pt(), "Sul");
        assert_eq!(BrazilianState::GO.region_pt(), "Centro-Oeste");
    }

    #[test]
    fn test_brazilian_date_add_calendar_days() {
        let start = BrazilianDate::now();
        let future = start.add_calendar_days(7);
        assert!(future.date > start.date);
    }

    #[test]
    fn test_document_type_display() {
        let cpf = DocumentType::CPF("123.456.789-09".to_string());
        assert_eq!(cpf.to_string(), "CPF: 123.456.789-09");

        let cnpj = DocumentType::CNPJ("11.222.333/0001-81".to_string());
        assert_eq!(cnpj.to_string(), "CNPJ: 11.222.333/0001-81");
    }

    #[test]
    fn test_municipality() {
        let municipality = Municipality {
            nome_pt: "São Paulo".to_string(),
            name_en: "São Paulo".to_string(),
            state: BrazilianState::SP,
            ibge_code: 3550308,
        };
        assert_eq!(municipality.state, BrazilianState::SP);
        assert_eq!(municipality.ibge_code, 3550308);
    }
}
