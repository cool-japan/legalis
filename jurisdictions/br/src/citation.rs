//! Brazilian Legal Citation Format
//!
//! This module provides formatting functions for Brazilian legal citations.
//!
//! ## Citation Format
//!
//! Brazilian legal citations follow the format:
//! ```text
//! Lei nº [number]/[year], Art. [article]º, §[paragraph]º, inciso [clause]
//! ```
//!
//! ## Examples
//!
//! - `Lei nº 8.078/1990, Art. 5º` - Consumer Defense Code, Article 5
//! - `Lei nº 5.452/1943, Art. 58, §1º` - CLT Article 58, Paragraph 1
//! - `Constituição Federal, Art. 5º, inciso X` - Federal Constitution Art. 5, Clause X
//! - `Lei nº 6.404/1976, Art. 138, §3º, inciso II` - Corporations Law with paragraph and clause

use serde::{Deserialize, Serialize};
use std::fmt;

/// Format a number with Brazilian thousand separators (dots)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::format_law_number;
///
/// assert_eq!(format_law_number(8078), "8.078");
/// assert_eq!(format_law_number(13709), "13.709");
/// assert_eq!(format_law_number(999), "999");
/// assert_eq!(format_law_number(12345), "12.345");
/// ```
pub fn format_law_number(num: u32) -> String {
    let s = num.to_string();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    if len <= 3 {
        return s;
    }

    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push('.');
        }
        result.push(*c);
    }
    result
}

/// Roman numeral representation for clauses (incisos)
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
}

impl fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomanNumeral::I => write!(f, "I"),
            RomanNumeral::II => write!(f, "II"),
            RomanNumeral::III => write!(f, "III"),
            RomanNumeral::IV => write!(f, "IV"),
            RomanNumeral::V => write!(f, "V"),
            RomanNumeral::VI => write!(f, "VI"),
            RomanNumeral::VII => write!(f, "VII"),
            RomanNumeral::VIII => write!(f, "VIII"),
            RomanNumeral::IX => write!(f, "IX"),
            RomanNumeral::X => write!(f, "X"),
            RomanNumeral::XI => write!(f, "XI"),
            RomanNumeral::XII => write!(f, "XII"),
            RomanNumeral::XIII => write!(f, "XIII"),
            RomanNumeral::XIV => write!(f, "XIV"),
            RomanNumeral::XV => write!(f, "XV"),
            RomanNumeral::XVI => write!(f, "XVI"),
            RomanNumeral::XVII => write!(f, "XVII"),
            RomanNumeral::XVIII => write!(f, "XVIII"),
            RomanNumeral::XIX => write!(f, "XIX"),
            RomanNumeral::XX => write!(f, "XX"),
        }
    }
}

/// Brazilian legal citation structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrazilianCitation {
    /// Law name (e.g., "Lei", "Constituição Federal", "Decreto")
    pub lei_tipo: String,
    /// Law number (e.g., 8078 for CDC)
    pub lei_numero: Option<u32>,
    /// Year of enactment (e.g., 1990)
    pub ano: Option<u32>,
    /// Article number
    pub artigo: u32,
    /// Paragraph number (§)
    pub paragrafo: Option<u32>,
    /// Clause (inciso) - typically Roman numerals
    pub inciso: Option<RomanNumeral>,
    /// Sub-clause (alínea) - typically lowercase letters
    pub alinea: Option<char>,
}

impl fmt::Display for BrazilianCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Start with law type and number
        if let (Some(num), Some(year)) = (self.lei_numero, self.ano) {
            write!(
                f,
                "{} nº {}/{}",
                self.lei_tipo,
                format_law_number(num),
                year
            )?;
        } else {
            write!(f, "{}", self.lei_tipo)?;
        }

        // Add article with ordinal indicator
        write!(f, ", Art. {}º", self.artigo)?;

        // Add paragraph if present
        if let Some(para) = self.paragrafo {
            write!(f, ", §{}º", para)?;
        }

        // Add inciso if present
        if let Some(ref inc) = self.inciso {
            write!(f, ", inciso {}", inc)?;
        }

        // Add alínea if present
        if let Some(alin) = self.alinea {
            write!(f, ", alínea {}", alin)?;
        }

        Result::Ok(())
    }
}

/// Format a citation for the Consumer Defense Code (CDC - Lei 8.078/1990)
///
/// # Arguments
///
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::{format_cdc_citation, RomanNumeral};
///
/// let citation = format_cdc_citation(5, None, None);
/// assert_eq!(citation, "Lei nº 8.078/1990, Art. 5º");
///
/// let citation_with_para = format_cdc_citation(51, Some(1), Some(RomanNumeral::IV));
/// assert_eq!(citation_with_para, "Lei nº 8.078/1990, Art. 51º, §1º, inciso IV");
/// ```
pub fn format_cdc_citation(
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Lei".to_string(),
        lei_numero: Some(8078),
        ano: Some(1990),
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Format a citation for CLT (Consolidação das Leis do Trabalho - Lei 5.452/1943)
///
/// # Arguments
///
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::format_clt_citation;
///
/// let citation = format_clt_citation(58, Some(1), None);
/// assert_eq!(citation, "Lei nº 5.452/1943, Art. 58º, §1º");
/// ```
pub fn format_clt_citation(
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Lei".to_string(),
        lei_numero: Some(5452),
        ano: Some(1943),
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Format a citation for the Civil Code (Código Civil - Lei 10.406/2002)
///
/// # Arguments
///
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::format_civil_code_citation;
///
/// let citation = format_civil_code_citation(421, None, None);
/// assert_eq!(citation, "Lei nº 10.406/2002, Art. 421º");
/// ```
pub fn format_civil_code_citation(
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Lei".to_string(),
        lei_numero: Some(10406),
        ano: Some(2002),
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Format a citation for LGPD (Lei Geral de Proteção de Dados - Lei 13.709/2018)
///
/// # Arguments
///
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::{format_lgpd_citation, RomanNumeral};
///
/// let citation = format_lgpd_citation(7, None, Some(RomanNumeral::I));
/// assert_eq!(citation, "Lei nº 13.709/2018, Art. 7º, inciso I");
/// ```
pub fn format_lgpd_citation(
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Lei".to_string(),
        lei_numero: Some(13709),
        ano: Some(2018),
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Format a citation for Lei das S.A. (Corporations Law - Lei 6.404/1976)
///
/// # Arguments
///
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::{format_lei_das_sa_citation, RomanNumeral};
///
/// let citation = format_lei_das_sa_citation(138, Some(3), Some(RomanNumeral::II));
/// assert_eq!(citation, "Lei nº 6.404/1976, Art. 138º, §3º, inciso II");
/// ```
pub fn format_lei_das_sa_citation(
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Lei".to_string(),
        lei_numero: Some(6404),
        ano: Some(1976),
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Format a citation for the Federal Constitution (Constituição Federal 1988)
///
/// # Arguments
///
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::{format_constitution_citation, RomanNumeral};
///
/// let citation = format_constitution_citation(5, None, Some(RomanNumeral::X));
/// assert_eq!(citation, "Constituição Federal, Art. 5º, inciso X");
/// ```
pub fn format_constitution_citation(
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Constituição Federal".to_string(),
        lei_numero: None,
        ano: None,
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Format a general law citation
///
/// # Arguments
///
/// * `lei_numero` - Law number
/// * `ano` - Year of enactment
/// * `artigo` - Article number
/// * `paragrafo` - Optional paragraph number
/// * `inciso` - Optional clause (inciso)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::format_lei_citation;
///
/// let citation = format_lei_citation(12345, 2020, 10, Some(2), None);
/// assert_eq!(citation, "Lei nº 12.345/2020, Art. 10º, §2º");
/// ```
pub fn format_lei_citation(
    lei_numero: u32,
    ano: u32,
    artigo: u32,
    paragrafo: Option<u32>,
    inciso: Option<RomanNumeral>,
) -> String {
    let citation = BrazilianCitation {
        lei_tipo: "Lei".to_string(),
        lei_numero: Some(lei_numero),
        ano: Some(ano),
        artigo,
        paragrafo,
        inciso,
        alinea: None,
    };
    citation.to_string()
}

/// Parse a Roman numeral string to RomanNumeral enum
///
/// # Examples
///
/// ```
/// use legalis_br::citation::{parse_roman_numeral, RomanNumeral};
///
/// assert_eq!(parse_roman_numeral("I"), Some(RomanNumeral::I));
/// assert_eq!(parse_roman_numeral("IV"), Some(RomanNumeral::IV));
/// assert_eq!(parse_roman_numeral("XX"), Some(RomanNumeral::XX));
/// assert_eq!(parse_roman_numeral("XXI"), None);
/// ```
pub fn parse_roman_numeral(s: &str) -> Option<RomanNumeral> {
    match s {
        "I" => Some(RomanNumeral::I),
        "II" => Some(RomanNumeral::II),
        "III" => Some(RomanNumeral::III),
        "IV" => Some(RomanNumeral::IV),
        "V" => Some(RomanNumeral::V),
        "VI" => Some(RomanNumeral::VI),
        "VII" => Some(RomanNumeral::VII),
        "VIII" => Some(RomanNumeral::VIII),
        "IX" => Some(RomanNumeral::IX),
        "X" => Some(RomanNumeral::X),
        "XI" => Some(RomanNumeral::XI),
        "XII" => Some(RomanNumeral::XII),
        "XIII" => Some(RomanNumeral::XIII),
        "XIV" => Some(RomanNumeral::XIV),
        "XV" => Some(RomanNumeral::XV),
        "XVI" => Some(RomanNumeral::XVI),
        "XVII" => Some(RomanNumeral::XVII),
        "XVIII" => Some(RomanNumeral::XVIII),
        "XIX" => Some(RomanNumeral::XIX),
        "XX" => Some(RomanNumeral::XX),
        _ => None,
    }
}

/// Convert a number to RomanNumeral enum (1-20)
///
/// # Examples
///
/// ```
/// use legalis_br::citation::{to_roman_numeral, RomanNumeral};
///
/// assert_eq!(to_roman_numeral(1), Some(RomanNumeral::I));
/// assert_eq!(to_roman_numeral(4), Some(RomanNumeral::IV));
/// assert_eq!(to_roman_numeral(20), Some(RomanNumeral::XX));
/// assert_eq!(to_roman_numeral(21), None);
/// ```
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
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdc_citation() {
        let citation = format_cdc_citation(5, None, None);
        assert_eq!(citation, "Lei nº 8.078/1990, Art. 5º");
    }

    #[test]
    fn test_cdc_citation_with_paragraph() {
        let citation = format_cdc_citation(51, Some(1), None);
        assert_eq!(citation, "Lei nº 8.078/1990, Art. 51º, §1º");
    }

    #[test]
    fn test_cdc_citation_with_inciso() {
        let citation = format_cdc_citation(51, Some(1), Some(RomanNumeral::IV));
        assert_eq!(citation, "Lei nº 8.078/1990, Art. 51º, §1º, inciso IV");
    }

    #[test]
    fn test_constitution_citation() {
        let citation = format_constitution_citation(5, None, Some(RomanNumeral::X));
        assert_eq!(citation, "Constituição Federal, Art. 5º, inciso X");
    }

    #[test]
    fn test_clt_citation() {
        let citation = format_clt_citation(58, Some(1), None);
        assert_eq!(citation, "Lei nº 5.452/1943, Art. 58º, §1º");
    }

    #[test]
    fn test_civil_code_citation() {
        let citation = format_civil_code_citation(421, None, None);
        assert_eq!(citation, "Lei nº 10.406/2002, Art. 421º");
    }

    #[test]
    fn test_lgpd_citation() {
        let citation = format_lgpd_citation(7, None, Some(RomanNumeral::I));
        assert_eq!(citation, "Lei nº 13.709/2018, Art. 7º, inciso I");
    }

    #[test]
    fn test_lei_das_sa_citation() {
        let citation = format_lei_das_sa_citation(138, Some(3), Some(RomanNumeral::II));
        assert_eq!(citation, "Lei nº 6.404/1976, Art. 138º, §3º, inciso II");
    }

    #[test]
    fn test_general_lei_citation() {
        let citation = format_lei_citation(12345, 2020, 10, Some(2), None);
        assert_eq!(citation, "Lei nº 12.345/2020, Art. 10º, §2º");
    }

    #[test]
    fn test_roman_numeral_display() {
        assert_eq!(RomanNumeral::I.to_string(), "I");
        assert_eq!(RomanNumeral::IV.to_string(), "IV");
        assert_eq!(RomanNumeral::XX.to_string(), "XX");
    }

    #[test]
    fn test_parse_roman_numeral() {
        assert_eq!(parse_roman_numeral("I"), Some(RomanNumeral::I));
        assert_eq!(parse_roman_numeral("IV"), Some(RomanNumeral::IV));
        assert_eq!(parse_roman_numeral("XX"), Some(RomanNumeral::XX));
        assert_eq!(parse_roman_numeral("XXI"), None);
    }

    #[test]
    fn test_to_roman_numeral() {
        assert_eq!(to_roman_numeral(1), Some(RomanNumeral::I));
        assert_eq!(to_roman_numeral(4), Some(RomanNumeral::IV));
        assert_eq!(to_roman_numeral(20), Some(RomanNumeral::XX));
        assert_eq!(to_roman_numeral(21), None);
    }

    #[test]
    fn test_brazilian_citation_struct() {
        let citation = BrazilianCitation {
            lei_tipo: "Lei".to_string(),
            lei_numero: Some(8078),
            ano: Some(1990),
            artigo: 5,
            paragrafo: None,
            inciso: None,
            alinea: None,
        };
        assert_eq!(citation.to_string(), "Lei nº 8.078/1990, Art. 5º");
    }

    #[test]
    fn test_citation_with_all_components() {
        let citation = BrazilianCitation {
            lei_tipo: "Lei".to_string(),
            lei_numero: Some(8078),
            ano: Some(1990),
            artigo: 51,
            paragrafo: Some(1),
            inciso: Some(RomanNumeral::IV),
            alinea: Some('a'),
        };
        assert_eq!(
            citation.to_string(),
            "Lei nº 8.078/1990, Art. 51º, §1º, inciso IV, alínea a"
        );
    }
}
