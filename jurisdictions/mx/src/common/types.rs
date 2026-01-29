//! Common Mexican legal types

use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Mexican currency (Peso - MXN)
///
/// Represents monetary values in Mexican Pesos.
/// Uses integer representation for centavos (cents) to avoid floating-point errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MexicanCurrency {
    /// Amount in centavos (cents). 100 centavos = 1 Peso
    pub centavos: i64,
}

impl MexicanCurrency {
    /// Create a new Mexican currency value from pesos
    pub fn from_pesos(pesos: i64) -> Self {
        Self {
            centavos: pesos * 100,
        }
    }

    /// Create a new Mexican currency value from centavos
    pub fn from_centavos(centavos: i64) -> Self {
        Self { centavos }
    }

    /// Get the value in pesos (integer part)
    pub fn pesos(&self) -> i64 {
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

impl fmt::Display for MexicanCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}. MXN", self.pesos())
    }
}

/// Mexican date with deadline calculation utilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MexicanDate {
    pub date: DateTime<Utc>,
}

impl MexicanDate {
    /// Create a new Mexican date
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
    pub fn add_business_days(&self, days: i64) -> Self {
        let mut current = self.date;
        let mut remaining_days = days;

        while remaining_days > 0 {
            current += Duration::days(1);
            let weekday = current.weekday();
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

/// Mexican document types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// RFC - Registro Federal de Contribuyentes (Tax ID)
    RFC(String),
    /// CURP - Clave Única de Registro de Población (Population Registry ID)
    CURP(String),
    /// NSS - Número de Seguro Social (Social Security Number)
    NSS(String),
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentType::RFC(rfc) => write!(f, "RFC: {}", rfc),
            DocumentType::CURP(curp) => write!(f, "CURP: {}", curp),
            DocumentType::NSS(nss) => write!(f, "NSS: {}", nss),
        }
    }
}

/// Mexican document structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MexicanDocument {
    /// Document type and number
    pub doc_type: DocumentType,
    /// Name (full name for individuals, company name for corporations)
    pub nombre_es: String,
    /// English translation of name
    pub name_en: String,
}

/// Document validation error
#[derive(Debug, Error, PartialEq)]
pub enum DocumentError {
    #[error("Invalid RFC format: {0}")]
    InvalidRFCFormat(String),
    #[error("Invalid CURP format: {0}")]
    InvalidCURPFormat(String),
    #[error("Invalid NSS format: {0}")]
    InvalidNSSFormat(String),
}

/// Validate RFC format (13 characters for individuals, 12 for companies)
///
/// # Examples
///
/// ```
/// use legalis_mx::common::validate_rfc;
///
/// assert!(validate_rfc("XAXX010101000").is_ok()); // Individual
/// assert!(validate_rfc("ABC010101ABC").is_ok()); // Company
/// assert!(validate_rfc("123").is_err());
/// ```
pub fn validate_rfc(rfc: &str) -> Result<(), DocumentError> {
    let rfc = rfc.to_uppercase().replace([' ', '-'], "");

    if rfc.len() != 12 && rfc.len() != 13 {
        return Err(DocumentError::InvalidRFCFormat(
            "RFC must be 12 or 13 characters".to_string(),
        ));
    }

    // Basic validation: first characters should be letters
    if !rfc.chars().take(3).all(|c| c.is_ascii_alphabetic()) {
        return Err(DocumentError::InvalidRFCFormat(
            "RFC must start with letters".to_string(),
        ));
    }

    Ok(())
}

/// Validate CURP format (18 characters)
///
/// # Examples
///
/// ```
/// use legalis_mx::common::validate_curp;
///
/// assert!(validate_curp("XAXX010101HDFRRL00").is_ok());
/// assert!(validate_curp("123").is_err());
/// ```
pub fn validate_curp(curp: &str) -> Result<(), DocumentError> {
    let curp = curp.to_uppercase().replace([' ', '-'], "");

    if curp.len() != 18 {
        return Err(DocumentError::InvalidCURPFormat(
            "CURP must be 18 characters".to_string(),
        ));
    }

    // First 4 characters should be letters
    if !curp.chars().take(4).all(|c| c.is_ascii_alphabetic()) {
        return Err(DocumentError::InvalidCURPFormat(
            "CURP must start with 4 letters".to_string(),
        ));
    }

    Ok(())
}

/// Validate NSS format (11 digits)
///
/// # Examples
///
/// ```
/// use legalis_mx::common::validate_nss;
///
/// assert!(validate_nss("12345678901").is_ok());
/// assert!(validate_nss("123").is_err());
/// ```
pub fn validate_nss(nss: &str) -> Result<(), DocumentError> {
    let digits: String = nss.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() != 11 {
        return Err(DocumentError::InvalidNSSFormat(
            "NSS must be 11 digits".to_string(),
        ));
    }

    Ok(())
}

/// Mexican states (Estados)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MexicanState {
    AGS, // Aguascalientes
    BC,  // Baja California
    BCS, // Baja California Sur
    CAM, // Campeche
    CHP, // Chiapas
    CHH, // Chihuahua
    CMX, // Ciudad de México (Mexico City)
    COA, // Coahuila
    COL, // Colima
    DUR, // Durango
    GTO, // Guanajuato
    GRO, // Guerrero
    HGO, // Hidalgo
    JAL, // Jalisco
    MEX, // México (State of Mexico)
    MIC, // Michoacán
    MOR, // Morelos
    NAY, // Nayarit
    NLE, // Nuevo León
    OAX, // Oaxaca
    PUE, // Puebla
    QRO, // Querétaro
    ROO, // Quintana Roo
    SLP, // San Luis Potosí
    SIN, // Sinaloa
    SON, // Sonora
    TAB, // Tabasco
    TAM, // Tamaulipas
    TLA, // Tlaxcala
    VER, // Veracruz
    YUC, // Yucatán
    ZAC, // Zacatecas
}

impl MexicanState {
    /// Get the full name in Spanish
    pub fn nombre_es(&self) -> &'static str {
        match self {
            MexicanState::AGS => "Aguascalientes",
            MexicanState::BC => "Baja California",
            MexicanState::BCS => "Baja California Sur",
            MexicanState::CAM => "Campeche",
            MexicanState::CHP => "Chiapas",
            MexicanState::CHH => "Chihuahua",
            MexicanState::CMX => "Ciudad de México",
            MexicanState::COA => "Coahuila",
            MexicanState::COL => "Colima",
            MexicanState::DUR => "Durango",
            MexicanState::GTO => "Guanajuato",
            MexicanState::GRO => "Guerrero",
            MexicanState::HGO => "Hidalgo",
            MexicanState::JAL => "Jalisco",
            MexicanState::MEX => "México",
            MexicanState::MIC => "Michoacán",
            MexicanState::MOR => "Morelos",
            MexicanState::NAY => "Nayarit",
            MexicanState::NLE => "Nuevo León",
            MexicanState::OAX => "Oaxaca",
            MexicanState::PUE => "Puebla",
            MexicanState::QRO => "Querétaro",
            MexicanState::ROO => "Quintana Roo",
            MexicanState::SLP => "San Luis Potosí",
            MexicanState::SIN => "Sinaloa",
            MexicanState::SON => "Sonora",
            MexicanState::TAB => "Tabasco",
            MexicanState::TAM => "Tamaulipas",
            MexicanState::TLA => "Tlaxcala",
            MexicanState::VER => "Veracruz",
            MexicanState::YUC => "Yucatán",
            MexicanState::ZAC => "Zacatecas",
        }
    }

    /// Get the abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            MexicanState::AGS => "AGS",
            MexicanState::BC => "BC",
            MexicanState::BCS => "BCS",
            MexicanState::CAM => "CAM",
            MexicanState::CHP => "CHP",
            MexicanState::CHH => "CHH",
            MexicanState::CMX => "CMX",
            MexicanState::COA => "COA",
            MexicanState::COL => "COL",
            MexicanState::DUR => "DUR",
            MexicanState::GTO => "GTO",
            MexicanState::GRO => "GRO",
            MexicanState::HGO => "HGO",
            MexicanState::JAL => "JAL",
            MexicanState::MEX => "MEX",
            MexicanState::MIC => "MIC",
            MexicanState::MOR => "MOR",
            MexicanState::NAY => "NAY",
            MexicanState::NLE => "NLE",
            MexicanState::OAX => "OAX",
            MexicanState::PUE => "PUE",
            MexicanState::QRO => "QRO",
            MexicanState::ROO => "ROO",
            MexicanState::SLP => "SLP",
            MexicanState::SIN => "SIN",
            MexicanState::SON => "SON",
            MexicanState::TAB => "TAB",
            MexicanState::TAM => "TAM",
            MexicanState::TLA => "TLA",
            MexicanState::VER => "VER",
            MexicanState::YUC => "YUC",
            MexicanState::ZAC => "ZAC",
        }
    }

    /// Get the region
    pub fn region_es(&self) -> &'static str {
        match self {
            MexicanState::BC
            | MexicanState::BCS
            | MexicanState::CHH
            | MexicanState::DUR
            | MexicanState::SIN
            | MexicanState::SON => "Norte",

            MexicanState::COA | MexicanState::NLE | MexicanState::TAM => "Noreste",

            MexicanState::AGS
            | MexicanState::COL
            | MexicanState::GTO
            | MexicanState::JAL
            | MexicanState::MIC
            | MexicanState::NAY
            | MexicanState::QRO
            | MexicanState::SLP
            | MexicanState::ZAC => "Centro-Occidente",

            MexicanState::CMX
            | MexicanState::HGO
            | MexicanState::MEX
            | MexicanState::MOR
            | MexicanState::PUE
            | MexicanState::TLA => "Centro",

            MexicanState::GRO | MexicanState::OAX | MexicanState::VER => "Sur",

            MexicanState::CAM
            | MexicanState::CHP
            | MexicanState::ROO
            | MexicanState::TAB
            | MexicanState::YUC => "Sureste",
        }
    }
}

impl fmt::Display for MexicanState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

/// Municipality information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Municipality {
    /// Municipality name in Spanish
    pub nombre_es: String,
    /// English translation
    pub name_en: String,
    /// State
    pub state: MexicanState,
    /// INEGI code (Instituto Nacional de Estadística y Geografía)
    pub inegi_code: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mexican_currency() {
        let amount = MexicanCurrency::from_pesos(100);
        assert_eq!(amount.pesos(), 100);
        assert_eq!(amount.cents(), 0);
    }

    #[test]
    fn test_mexican_currency_with_cents() {
        let amount = MexicanCurrency::from_centavos(12345);
        assert_eq!(amount.pesos(), 123);
        assert_eq!(amount.cents(), 45);
    }

    #[test]
    fn test_rfc_validation() {
        assert!(validate_rfc("XAXX010101000").is_ok());
        assert!(validate_rfc("ABC010101ABC").is_ok());
        assert!(validate_rfc("123").is_err());
    }

    #[test]
    fn test_curp_validation() {
        assert!(validate_curp("XAXX010101HDFRRL00").is_ok());
        assert!(validate_curp("123").is_err());
    }

    #[test]
    fn test_nss_validation() {
        assert!(validate_nss("12345678901").is_ok());
        assert!(validate_nss("123").is_err());
    }

    #[test]
    fn test_mexican_state_names() {
        assert_eq!(MexicanState::CMX.nombre_es(), "Ciudad de México");
        assert_eq!(MexicanState::JAL.nombre_es(), "Jalisco");
        assert_eq!(MexicanState::NLE.nombre_es(), "Nuevo León");
    }

    #[test]
    fn test_mexican_state_region() {
        assert_eq!(MexicanState::CMX.region_es(), "Centro");
        assert_eq!(MexicanState::NLE.region_es(), "Noreste");
        assert_eq!(MexicanState::YUC.region_es(), "Sureste");
    }
}
