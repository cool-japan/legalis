//! Integration tests for citation formatting

use legalis_mx::citation::*;

#[test]
fn test_civil_code_citation() {
    let cite = format_civil_code_citation(1792, None, None);
    assert_eq!(cite, "Código Civil Federal, Artículo 1792");
}

#[test]
fn test_labor_law_citation() {
    let cite = format_labor_law_citation(61, None, Some(RomanNumeral::I));
    assert_eq!(cite, "Ley Federal del Trabajo, Artículo 61, fracción I");
}

#[test]
fn test_criminal_code_citation() {
    let cite = format_criminal_code_citation(302, None, None);
    assert_eq!(cite, "Código Penal Federal, Artículo 302");
}

#[test]
fn test_constitution_citation() {
    let cite = format_constitution_citation(123, Some('A'), Some(RomanNumeral::VI));
    assert_eq!(
        cite,
        "Constitución Política de los Estados Unidos Mexicanos, Artículo 123, Apartado A, fracción VI"
    );
}

#[test]
fn test_tax_code_citation() {
    let cite = format_tax_code_citation(5, None, Some(RomanNumeral::II));
    assert_eq!(
        cite,
        "Código Fiscal de la Federación, Artículo 5, fracción II"
    );
}

#[test]
fn test_isr_citation() {
    let cite = format_isr_citation(9, None, None);
    assert_eq!(cite, "Ley del Impuesto Sobre la Renta, Artículo 9");
}

#[test]
fn test_iva_citation() {
    let cite = format_iva_citation(1, None, None);
    assert_eq!(cite, "Ley del Impuesto al Valor Agregado, Artículo 1");
}

#[test]
fn test_lgsm_citation() {
    let cite = format_lgsm_citation(89, None, None);
    assert_eq!(cite, "Ley General de Sociedades Mercantiles, Artículo 89");
}

#[test]
fn test_lfce_citation() {
    let cite = format_lfce_citation(53, None, Some(RomanNumeral::I));
    assert_eq!(
        cite,
        "Ley Federal de Competencia Económica, Artículo 53, fracción I"
    );
}

#[test]
fn test_to_roman_numeral() {
    assert_eq!(to_roman_numeral(1), Some(RomanNumeral::I));
    assert_eq!(to_roman_numeral(10), Some(RomanNumeral::X));
    assert_eq!(to_roman_numeral(30), Some(RomanNumeral::XXX));
    assert_eq!(to_roman_numeral(31), None);
}
