//! Internationalization (i18n) support for EU legal texts
//!
//! The EU has 24 official languages, and all legislation is equally authentic in each.
//! This module provides structures for storing and accessing legal text in multiple languages.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Multilingual legal text supporting multiple EU languages
///
/// ## Example
///
/// ```rust
/// use legalis_eu::MultilingualText;
///
/// let text = MultilingualText::from_eurlex(
///     "Data Controller".to_string(),
///     "Verantwortlicher".to_string(),
///     "CELEX:32016R0679".to_string(),
/// );
///
/// assert_eq!(text.in_language("en"), "Data Controller");
/// assert_eq!(text.in_language("de"), "Verantwortlicher");
/// assert_eq!(text.in_language("fr"), "Data Controller");  // Falls back to EN
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultilingualText {
    /// English text (primary/fallback)
    pub en: String,

    /// German text - Datenschutz-Grundverordnung (DSGVO)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub de: Option<String>,

    /// French text - Règlement Général sur la Protection des Données (RGPD)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fr: Option<String>,

    /// Spanish text - Reglamento General de Protección de Datos (RGPD)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub es: Option<String>,

    /// Italian text - Regolamento Generale sulla Protezione dei Dati (GDPR)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub it: Option<String>,

    /// Polish text - Ogólne Rozporządzenie o Ochronie Danych (RODO)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pl: Option<String>,

    /// Dutch text - Algemene Verordening Gegevensbescherming (AVG)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub nl: Option<String>,

    /// Portuguese text - Regulamento Geral sobre a Proteção de Dados (RGPD)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pt: Option<String>,

    /// Swedish text - Dataskyddsförordningen (GDPR)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sv: Option<String>,

    /// Czech text - Obecné nařízení o ochraně osobních údajů (GDPR)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cs: Option<String>,

    /// Greek text - Γενικός Κανονισμός για την Προστασία Δεδομένων (GDPR)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub el: Option<String>,

    /// Source attribution (EUR-Lex CELEX number)
    pub source: Option<String>,
}

impl MultilingualText {
    /// Create a new multilingual text with only English
    pub fn new(en: impl Into<String>) -> Self {
        Self {
            en: en.into(),
            de: None,
            fr: None,
            es: None,
            it: None,
            pl: None,
            nl: None,
            pt: None,
            sv: None,
            cs: None,
            el: None,
            source: None,
        }
    }

    /// Get text in preferred language with fallback chain
    ///
    /// Falls back to English if the requested language is not available.
    /// Supports all 24 EU official languages (currently 11 implemented).
    pub fn in_language(&self, lang: &str) -> &str {
        match lang {
            "de" | "DE" => self.de.as_deref().unwrap_or(&self.en),
            "fr" | "FR" => self.fr.as_deref().unwrap_or(&self.en),
            "es" | "ES" => self.es.as_deref().unwrap_or(&self.en),
            "it" | "IT" => self.it.as_deref().unwrap_or(&self.en),
            "pl" | "PL" => self.pl.as_deref().unwrap_or(&self.en),
            "nl" | "NL" => self.nl.as_deref().unwrap_or(&self.en),
            "pt" | "PT" => self.pt.as_deref().unwrap_or(&self.en),
            "sv" | "SV" => self.sv.as_deref().unwrap_or(&self.en),
            "cs" | "CS" => self.cs.as_deref().unwrap_or(&self.en),
            "el" | "EL" => self.el.as_deref().unwrap_or(&self.en),
            _ => &self.en,
        }
    }

    /// Create from EUR-Lex official text with source attribution
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::MultilingualText;
    ///
    /// let text = MultilingualText::from_eurlex(
    ///     "General Data Protection Regulation".to_string(),
    ///     "Datenschutz-Grundverordnung".to_string(),
    ///     "CELEX:32016R0679".to_string(),
    /// );
    ///
    /// assert_eq!(text.source.as_deref(), Some("CELEX:32016R0679"));
    /// ```
    pub fn from_eurlex(en: String, de: String, celex: String) -> Self {
        Self {
            en,
            de: Some(de),
            fr: None,
            es: None,
            it: None,
            pl: None,
            nl: None,
            pt: None,
            sv: None,
            cs: None,
            el: None,
            source: Some(celex),
        }
    }

    /// Add German translation
    pub fn with_de(mut self, de: impl Into<String>) -> Self {
        self.de = Some(de.into());
        self
    }

    /// Add French translation
    pub fn with_fr(mut self, fr: impl Into<String>) -> Self {
        self.fr = Some(fr.into());
        self
    }

    /// Add Spanish translation
    pub fn with_es(mut self, es: impl Into<String>) -> Self {
        self.es = Some(es.into());
        self
    }

    /// Add Italian translation
    pub fn with_it(mut self, it: impl Into<String>) -> Self {
        self.it = Some(it.into());
        self
    }

    /// Add Polish translation
    pub fn with_pl(mut self, pl: impl Into<String>) -> Self {
        self.pl = Some(pl.into());
        self
    }

    /// Add Dutch translation
    pub fn with_nl(mut self, nl: impl Into<String>) -> Self {
        self.nl = Some(nl.into());
        self
    }

    /// Add Portuguese translation
    pub fn with_pt(mut self, pt: impl Into<String>) -> Self {
        self.pt = Some(pt.into());
        self
    }

    /// Add Swedish translation
    pub fn with_sv(mut self, sv: impl Into<String>) -> Self {
        self.sv = Some(sv.into());
        self
    }

    /// Add Czech translation
    pub fn with_cs(mut self, cs: impl Into<String>) -> Self {
        self.cs = Some(cs.into());
        self
    }

    /// Add Greek translation
    pub fn with_el(mut self, el: impl Into<String>) -> Self {
        self.el = Some(el.into());
        self
    }

    /// Add source attribution
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multilingual_text_creation() {
        let text = MultilingualText::new("Data Controller");
        assert_eq!(text.en, "Data Controller");
        assert!(text.de.is_none());
        assert!(text.fr.is_none());
    }

    #[test]
    fn test_multilingual_text_fallback() {
        let text = MultilingualText {
            en: "Data Controller".to_string(),
            de: Some("Verantwortlicher".to_string()),
            fr: None,
            es: None,
            it: None,
            pl: None,
            nl: None,
            pt: None,
            sv: None,
            cs: None,
            el: None,
            source: Some("CELEX:32016R0679".to_string()),
        };

        assert_eq!(text.in_language("en"), "Data Controller");
        assert_eq!(text.in_language("de"), "Verantwortlicher");
        assert_eq!(text.in_language("fr"), "Data Controller"); // Falls back to EN
    }

    #[test]
    fn test_from_eurlex() {
        let text = MultilingualText::from_eurlex(
            "General Data Protection Regulation".to_string(),
            "Datenschutz-Grundverordnung".to_string(),
            "CELEX:32016R0679".to_string(),
        );

        assert_eq!(text.en, "General Data Protection Regulation");
        assert_eq!(text.de.as_deref(), Some("Datenschutz-Grundverordnung"));
        assert_eq!(text.source.as_deref(), Some("CELEX:32016R0679"));
    }

    #[test]
    fn test_builder_pattern() {
        let text = MultilingualText::new("Data Controller")
            .with_de("Verantwortlicher")
            .with_fr("Responsable du traitement")
            .with_source("CELEX:32016R0679");

        assert_eq!(text.in_language("en"), "Data Controller");
        assert_eq!(text.in_language("de"), "Verantwortlicher");
        assert_eq!(text.in_language("fr"), "Responsable du traitement");
    }

    #[test]
    fn test_all_11_languages_builder() {
        let text = MultilingualText::new("Data Controller")
            .with_de("Verantwortlicher")
            .with_fr("Responsable du traitement")
            .with_es("Responsable del tratamiento")
            .with_it("Titolare del trattamento")
            .with_pl("Administrator danych")
            .with_nl("Verwerkingsverantwoordelijke")
            .with_pt("Responsável pelo tratamento")
            .with_sv("Personuppgiftsansvarig")
            .with_cs("Správce")
            .with_el("Υπεύθυνος επεξεργασίας")
            .with_source("CELEX:32016R0679");

        // Verify all languages are set correctly
        assert_eq!(text.in_language("en"), "Data Controller");
        assert_eq!(text.in_language("de"), "Verantwortlicher");
        assert_eq!(text.in_language("fr"), "Responsable du traitement");
        assert_eq!(text.in_language("es"), "Responsable del tratamiento");
        assert_eq!(text.in_language("it"), "Titolare del trattamento");
        assert_eq!(text.in_language("pl"), "Administrator danych");
        assert_eq!(text.in_language("nl"), "Verwerkingsverantwoordelijke");
        assert_eq!(text.in_language("pt"), "Responsável pelo tratamento");
        assert_eq!(text.in_language("sv"), "Personuppgiftsansvarig");
        assert_eq!(text.in_language("cs"), "Správce");
        assert_eq!(text.in_language("el"), "Υπεύθυνος επεξεργασίας");

        // Verify source attribution
        assert_eq!(text.source.as_deref(), Some("CELEX:32016R0679"));
    }

    #[test]
    fn test_case_insensitive_language_codes() {
        let text = MultilingualText::new("Data Controller")
            .with_de("Verantwortlicher")
            .with_pl("Administrator danych")
            .with_el("Υπεύθυνος επεξεργασίας");

        // Test case-insensitive matching
        assert_eq!(text.in_language("DE"), "Verantwortlicher");
        assert_eq!(text.in_language("de"), "Verantwortlicher");
        assert_eq!(text.in_language("PL"), "Administrator danych");
        assert_eq!(text.in_language("pl"), "Administrator danych");
        assert_eq!(text.in_language("EL"), "Υπεύθυνος επεξεργασίας");
        assert_eq!(text.in_language("el"), "Υπεύθυνος επεξεργασίας");
    }

    #[test]
    fn test_partial_translations_with_fallback() {
        let text = MultilingualText::new("Data Controller")
            .with_de("Verantwortlicher")
            .with_es("Responsable del tratamiento");
        // Only EN, DE, ES are set - others should fall back to EN

        assert_eq!(text.in_language("en"), "Data Controller");
        assert_eq!(text.in_language("de"), "Verantwortlicher");
        assert_eq!(text.in_language("es"), "Responsable del tratamiento");

        // These should all fall back to English
        assert_eq!(text.in_language("fr"), "Data Controller");
        assert_eq!(text.in_language("it"), "Data Controller");
        assert_eq!(text.in_language("pl"), "Data Controller");
        assert_eq!(text.in_language("nl"), "Data Controller");
        assert_eq!(text.in_language("pt"), "Data Controller");
        assert_eq!(text.in_language("sv"), "Data Controller");
        assert_eq!(text.in_language("cs"), "Data Controller");
        assert_eq!(text.in_language("el"), "Data Controller");
    }

    #[test]
    fn test_unsupported_language_fallback() {
        let text = MultilingualText::new("Data Controller").with_de("Verantwortlicher");

        // Non-EU languages should fall back to English
        assert_eq!(text.in_language("ja"), "Data Controller");
        assert_eq!(text.in_language("zh"), "Data Controller");
        assert_eq!(text.in_language("ar"), "Data Controller");
        assert_eq!(text.in_language("ru"), "Data Controller");
        assert_eq!(text.in_language("unknown"), "Data Controller");
    }
}
