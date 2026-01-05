//! Multilingual support for diff summaries and NLP.
//!
//! This module provides multi-language support for generating summaries
//! in different languages.

use crate::{StatuteDiff, nlp::NaturalLanguageSummary};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// English.
    English,
    /// French.
    French,
    /// Spanish.
    Spanish,
    /// German.
    German,
    /// Japanese.
    Japanese,
    /// Chinese (Simplified).
    ChineseSimplified,
}

impl Language {
    /// Gets the ISO 639-1 language code.
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Spanish => "es",
            Language::German => "de",
            Language::Japanese => "ja",
            Language::ChineseSimplified => "zh",
        }
    }

    /// Gets the language name.
    pub fn name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
            Language::Japanese => "日本語",
            Language::ChineseSimplified => "中文",
        }
    }
}

/// Translation dictionary for common terms.
#[derive(Debug, Clone)]
pub struct TranslationDictionary {
    translations: HashMap<Language, HashMap<String, String>>,
}

impl Default for TranslationDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslationDictionary {
    /// Creates a new translation dictionary with default translations.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::multilingual::{TranslationDictionary, Language};
    ///
    /// let dict = TranslationDictionary::new();
    /// let translated = dict.translate("statute", Language::French);
    /// assert!(translated.contains("statut") || translated.contains("loi"));
    /// ```
    pub fn new() -> Self {
        let mut dict = Self {
            translations: HashMap::new(),
        };

        // Add French translations
        let mut fr = HashMap::new();
        fr.insert("statute".to_string(), "loi".to_string());
        fr.insert("modified".to_string(), "modifié".to_string());
        fr.insert("added".to_string(), "ajouté".to_string());
        fr.insert("removed".to_string(), "supprimé".to_string());
        fr.insert("change".to_string(), "changement".to_string());
        fr.insert("impact".to_string(), "impact".to_string());
        fr.insert("severity".to_string(), "gravité".to_string());
        dict.translations.insert(Language::French, fr);

        // Add Spanish translations
        let mut es = HashMap::new();
        es.insert("statute".to_string(), "estatuto".to_string());
        es.insert("modified".to_string(), "modificado".to_string());
        es.insert("added".to_string(), "añadido".to_string());
        es.insert("removed".to_string(), "eliminado".to_string());
        es.insert("change".to_string(), "cambio".to_string());
        es.insert("impact".to_string(), "impacto".to_string());
        es.insert("severity".to_string(), "gravedad".to_string());
        dict.translations.insert(Language::Spanish, es);

        // Add German translations
        let mut de = HashMap::new();
        de.insert("statute".to_string(), "Gesetz".to_string());
        de.insert("modified".to_string(), "geändert".to_string());
        de.insert("added".to_string(), "hinzugefügt".to_string());
        de.insert("removed".to_string(), "entfernt".to_string());
        de.insert("change".to_string(), "Änderung".to_string());
        de.insert("impact".to_string(), "Auswirkung".to_string());
        de.insert("severity".to_string(), "Schweregrad".to_string());
        dict.translations.insert(Language::German, de);

        // Add Japanese translations
        let mut ja = HashMap::new();
        ja.insert("statute".to_string(), "法令".to_string());
        ja.insert("modified".to_string(), "修正".to_string());
        ja.insert("added".to_string(), "追加".to_string());
        ja.insert("removed".to_string(), "削除".to_string());
        ja.insert("change".to_string(), "変更".to_string());
        ja.insert("impact".to_string(), "影響".to_string());
        ja.insert("severity".to_string(), "重大度".to_string());
        dict.translations.insert(Language::Japanese, ja);

        // Add Chinese translations
        let mut zh = HashMap::new();
        zh.insert("statute".to_string(), "法规".to_string());
        zh.insert("modified".to_string(), "修改".to_string());
        zh.insert("added".to_string(), "添加".to_string());
        zh.insert("removed".to_string(), "删除".to_string());
        zh.insert("change".to_string(), "变更".to_string());
        zh.insert("impact".to_string(), "影响".to_string());
        zh.insert("severity".to_string(), "严重性".to_string());
        dict.translations.insert(Language::ChineseSimplified, zh);

        dict
    }

    /// Translates a term to the target language.
    pub fn translate(&self, term: &str, language: Language) -> String {
        if language == Language::English {
            return term.to_string();
        }

        self.translations
            .get(&language)
            .and_then(|lang_dict| lang_dict.get(term))
            .cloned()
            .unwrap_or_else(|| term.to_string())
    }

    /// Adds a custom translation.
    pub fn add_translation(&mut self, language: Language, term: &str, translation: &str) {
        self.translations
            .entry(language)
            .or_default()
            .insert(term.to_string(), translation.to_string());
    }
}

/// Multilingual summary generator.
#[derive(Debug, Clone)]
pub struct MultilingualSummaryGenerator {
    dictionary: TranslationDictionary,
}

impl Default for MultilingualSummaryGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MultilingualSummaryGenerator {
    /// Creates a new multilingual summary generator.
    pub fn new() -> Self {
        Self {
            dictionary: TranslationDictionary::new(),
        }
    }

    /// Generates a summary in the specified language.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, multilingual::{MultilingualSummaryGenerator, Language}};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let generator = MultilingualSummaryGenerator::new();
    /// let summary = generator.generate_summary(&diff_result, Language::French);
    ///
    /// assert!(summary.language == Language::French);
    /// ```
    pub fn generate_summary(&self, diff: &StatuteDiff, language: Language) -> MultilingualSummary {
        let change_count = diff.changes.len();
        let severity_text = format!("{:?}", diff.impact.severity);

        // Generate summary in target language
        let summary_text = match language {
            Language::English => {
                format!(
                    "The statute '{}' has been modified with {} change(s) and {} severity.",
                    diff.statute_id, change_count, severity_text
                )
            }
            Language::French => {
                format!(
                    "La {} '{}' a été modifiée avec {} changement(s) de gravité {}.",
                    self.dictionary.translate("statute", language),
                    diff.statute_id,
                    change_count,
                    severity_text
                )
            }
            Language::Spanish => {
                format!(
                    "El {} '{}' ha sido modificado con {} cambio(s) de gravedad {}.",
                    self.dictionary.translate("statute", language),
                    diff.statute_id,
                    change_count,
                    severity_text
                )
            }
            Language::German => {
                format!(
                    "Das {} '{}' wurde mit {} Änderung(en) mit Schweregrad {} geändert.",
                    self.dictionary.translate("statute", language),
                    diff.statute_id,
                    change_count,
                    severity_text
                )
            }
            Language::Japanese => {
                format!(
                    "{}「{}」は{}件の変更があり、重大度は{}です。",
                    self.dictionary.translate("statute", language),
                    diff.statute_id,
                    change_count,
                    severity_text
                )
            }
            Language::ChineseSimplified => {
                format!(
                    "{} '{}' 已被修改，共{}处变更，严重性为{}。",
                    self.dictionary.translate("statute", language),
                    diff.statute_id,
                    change_count,
                    severity_text
                )
            }
        };

        // Generate detailed explanations
        let mut explanations = Vec::new();
        for change in &diff.changes {
            let explanation = self.translate_change_description(change, language);
            explanations.push(explanation);
        }

        MultilingualSummary {
            statute_id: diff.statute_id.clone(),
            language,
            summary: summary_text,
            explanations,
            change_count,
        }
    }

    fn translate_change_description(&self, change: &crate::Change, language: Language) -> String {
        let change_type_text = self.dictionary.translate(
            match change.change_type {
                crate::ChangeType::Added => "added",
                crate::ChangeType::Removed => "removed",
                crate::ChangeType::Modified => "modified",
                crate::ChangeType::Reordered => "reordered",
            },
            language,
        );

        match language {
            Language::English => format!("{}: {}", change.target, change_type_text),
            Language::French => format!("{} a été {}", change.target, change_type_text),
            Language::Spanish => format!("{} ha sido {}", change.target, change_type_text),
            Language::German => format!("{} wurde {}", change.target, change_type_text),
            Language::Japanese => format!("{}が{}", change.target, change_type_text),
            Language::ChineseSimplified => format!("{}已{}", change.target, change_type_text),
        }
    }

    /// Gets the translation dictionary.
    pub fn dictionary_mut(&mut self) -> &mut TranslationDictionary {
        &mut self.dictionary
    }
}

/// A multilingual summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualSummary {
    /// Statute ID.
    pub statute_id: String,
    /// Language of the summary.
    pub language: Language,
    /// Summary text.
    pub summary: String,
    /// Detailed explanations.
    pub explanations: Vec<String>,
    /// Number of changes.
    pub change_count: usize,
}

impl MultilingualSummary {
    /// Converts to a natural language summary.
    pub fn to_natural_language_summary(&self) -> NaturalLanguageSummary {
        NaturalLanguageSummary {
            statute_id: self.statute_id.clone(),
            summary: self.summary.clone(),
            explanation: self.explanations.clone(),
            impacts: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_diff() -> StatuteDiff {
        let old = Statute::new(
            "law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let new = Statute::new(
            "law",
            "New Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        diff(&old, &new).unwrap()
    }

    #[test]
    fn test_translation_dictionary() {
        let dict = TranslationDictionary::new();

        assert_eq!(dict.translate("statute", Language::French), "loi");
        assert_eq!(dict.translate("statute", Language::Spanish), "estatuto");
        assert_eq!(dict.translate("statute", Language::English), "statute");
    }

    #[test]
    fn test_language_code() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::Japanese.code(), "ja");
    }

    #[test]
    fn test_multilingual_summary_english() {
        let generator = MultilingualSummaryGenerator::new();
        let diff_result = test_diff();

        let summary = generator.generate_summary(&diff_result, Language::English);
        assert_eq!(summary.language, Language::English);
        assert!(summary.summary.contains("statute"));
    }

    #[test]
    fn test_multilingual_summary_french() {
        let generator = MultilingualSummaryGenerator::new();
        let diff_result = test_diff();

        let summary = generator.generate_summary(&diff_result, Language::French);
        assert_eq!(summary.language, Language::French);
        assert!(summary.summary.contains("loi"));
    }

    #[test]
    fn test_multilingual_summary_spanish() {
        let generator = MultilingualSummaryGenerator::new();
        let diff_result = test_diff();

        let summary = generator.generate_summary(&diff_result, Language::Spanish);
        assert_eq!(summary.language, Language::Spanish);
    }

    #[test]
    fn test_custom_translation() {
        let mut dict = TranslationDictionary::new();
        dict.add_translation(Language::French, "custom", "personnalisé");

        assert_eq!(dict.translate("custom", Language::French), "personnalisé");
    }
}
