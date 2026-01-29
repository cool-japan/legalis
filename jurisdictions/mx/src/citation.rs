//! Mexican Legal Citation Format
//!
//! This module provides formatting functions for Mexican legal citations.
//!
//! ## Citation Format
//!
//! Mexican legal citations follow the format:
//! ```text
//! [Law Name], Artículo [article], fracción [fraction]
//! ```
//!
//! ## Examples
//!
//! - `Código Civil Federal, Artículo 1792` - Federal Civil Code, Article 1792
//! - `Ley Federal del Trabajo, Artículo 61, fracción I` - Labor Law Art. 61, Fraction I
//! - `Constitución Política, Artículo 123, Apartado A, fracción VI` - Constitution

use serde::{Deserialize, Serialize};
use std::fmt;

/// Roman numeral representation for fractions (fracciones)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RomanNumeral {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
    XI,
    XII,
    XIII,
    XIV,
    XV,
    XVI,
    XVII,
    XVIII,
    XIX,
    XX,
    XXI,
    XXII,
    XXIII,
    XXIV,
    XXV,
    XXVI,
    XXVII,
    XXVIII,
    XXIX,
    XXX,
}

impl fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RomanNumeral::I => "I",
            RomanNumeral::II => "II",
            RomanNumeral::III => "III",
            RomanNumeral::IV => "IV",
            RomanNumeral::V => "V",
            RomanNumeral::VI => "VI",
            RomanNumeral::VII => "VII",
            RomanNumeral::VIII => "VIII",
            RomanNumeral::IX => "IX",
            RomanNumeral::X => "X",
            RomanNumeral::XI => "XI",
            RomanNumeral::XII => "XII",
            RomanNumeral::XIII => "XIII",
            RomanNumeral::XIV => "XIV",
            RomanNumeral::XV => "XV",
            RomanNumeral::XVI => "XVI",
            RomanNumeral::XVII => "XVII",
            RomanNumeral::XVIII => "XVIII",
            RomanNumeral::XIX => "XIX",
            RomanNumeral::XX => "XX",
            RomanNumeral::XXI => "XXI",
            RomanNumeral::XXII => "XXII",
            RomanNumeral::XXIII => "XXIII",
            RomanNumeral::XXIV => "XXIV",
            RomanNumeral::XXV => "XXV",
            RomanNumeral::XXVI => "XXVI",
            RomanNumeral::XXVII => "XXVII",
            RomanNumeral::XXVIII => "XXVIII",
            RomanNumeral::XXIX => "XXIX",
            RomanNumeral::XXX => "XXX",
        };
        write!(f, "{}", s)
    }
}

/// Mexican legal citation structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MexicanCitation {
    /// Law name (e.g., "Código Civil Federal", "Ley Federal del Trabajo")
    pub ley_nombre: String,
    /// Article number (Artículo)
    pub articulo: u32,
    /// Paragraph (párrafo)
    pub parrafo: Option<u32>,
    /// Fraction (fracción) - typically Roman numerals
    pub fraccion: Option<RomanNumeral>,
    /// Section (apartado) - typically uppercase letters
    pub apartado: Option<char>,
}

impl fmt::Display for MexicanCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, Artículo {}", self.ley_nombre, self.articulo)?;

        if let Some(ref apartado) = self.apartado {
            write!(f, ", Apartado {}", apartado)?;
        }

        if let Some(ref parrafo) = self.parrafo {
            write!(f, ", párrafo {}", parrafo)?;
        }

        if let Some(ref fraccion) = self.fraccion {
            write!(f, ", fracción {}", fraccion)?;
        }

        Ok(())
    }
}

/// Format a citation for the Federal Civil Code (Código Civil Federal)
pub fn format_civil_code_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Código Civil Federal".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for the Federal Labor Law (Ley Federal del Trabajo)
pub fn format_labor_law_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Ley Federal del Trabajo".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for the Federal Criminal Code (Código Penal Federal)
pub fn format_criminal_code_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Código Penal Federal".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for LFPDPPP (Data Protection Law)
pub fn format_lfpdppp_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Ley Federal de Protección de Datos Personales en Posesión de los Particulares"
            .to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for the Tax Code (Código Fiscal de la Federación)
pub fn format_tax_code_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Código Fiscal de la Federación".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for ISR (Income Tax Law)
pub fn format_isr_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Ley del Impuesto Sobre la Renta".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for IVA (Value Added Tax Law)
pub fn format_iva_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Ley del Impuesto al Valor Agregado".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for LGSM (General Law of Commercial Companies)
pub fn format_lgsm_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Ley General de Sociedades Mercantiles".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for LFCE (Federal Economic Competition Law)
pub fn format_lfce_citation(
    articulo: u32,
    parrafo: Option<u32>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Ley Federal de Competencia Económica".to_string(),
        articulo,
        parrafo,
        fraccion,
        apartado: None,
    };
    citation.to_string()
}

/// Format a citation for the Constitution (Constitución Política)
pub fn format_constitution_citation(
    articulo: u32,
    apartado: Option<char>,
    fraccion: Option<RomanNumeral>,
) -> String {
    let citation = MexicanCitation {
        ley_nombre: "Constitución Política de los Estados Unidos Mexicanos".to_string(),
        articulo,
        parrafo: None,
        fraccion,
        apartado,
    };
    citation.to_string()
}

/// Convert a number to RomanNumeral enum (1-30)
pub fn to_roman_numeral(n: u32) -> Option<RomanNumeral> {
    match n {
        1 => Some(RomanNumeral::I),
        2 => Some(RomanNumeral::II),
        3 => Some(RomanNumeral::III),
        4 => Some(RomanNumeral::IV),
        5 => Some(RomanNumeral::V),
        6 => Some(RomanNumeral::VI),
        7 => Some(RomanNumeral::VII),
        8 => Some(RomanNumeral::VIII),
        9 => Some(RomanNumeral::IX),
        10 => Some(RomanNumeral::X),
        11 => Some(RomanNumeral::XI),
        12 => Some(RomanNumeral::XII),
        13 => Some(RomanNumeral::XIII),
        14 => Some(RomanNumeral::XIV),
        15 => Some(RomanNumeral::XV),
        16 => Some(RomanNumeral::XVI),
        17 => Some(RomanNumeral::XVII),
        18 => Some(RomanNumeral::XVIII),
        19 => Some(RomanNumeral::XIX),
        20 => Some(RomanNumeral::XX),
        21 => Some(RomanNumeral::XXI),
        22 => Some(RomanNumeral::XXII),
        23 => Some(RomanNumeral::XXIII),
        24 => Some(RomanNumeral::XXIV),
        25 => Some(RomanNumeral::XXV),
        26 => Some(RomanNumeral::XXVI),
        27 => Some(RomanNumeral::XXVII),
        28 => Some(RomanNumeral::XXVIII),
        29 => Some(RomanNumeral::XXIX),
        30 => Some(RomanNumeral::XXX),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_citation() {
        let citation = format_civil_code_citation(1792, None, None);
        assert_eq!(citation, "Código Civil Federal, Artículo 1792");
    }

    #[test]
    fn test_labor_law_citation() {
        let citation = format_labor_law_citation(61, None, Some(RomanNumeral::I));
        assert_eq!(citation, "Ley Federal del Trabajo, Artículo 61, fracción I");
    }

    #[test]
    fn test_constitution_citation() {
        let citation = format_constitution_citation(123, Some('A'), Some(RomanNumeral::VI));
        assert_eq!(
            citation,
            "Constitución Política de los Estados Unidos Mexicanos, Artículo 123, Apartado A, fracción VI"
        );
    }

    #[test]
    fn test_to_roman_numeral() {
        assert_eq!(to_roman_numeral(1), Some(RomanNumeral::I));
        assert_eq!(to_roman_numeral(30), Some(RomanNumeral::XXX));
        assert_eq!(to_roman_numeral(31), None);
    }
}
