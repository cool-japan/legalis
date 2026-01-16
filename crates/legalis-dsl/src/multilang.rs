//! Multi-Language DSL Support
//!
//! This module provides support for writing legal DSL in multiple natural languages.
//! Keywords can be translated while maintaining the same AST structure.

use std::collections::HashMap;

/// Supported languages for the DSL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DslLanguage {
    /// English (default)
    #[default]
    English,
    /// Japanese (日本語)
    Japanese,
    /// German (Deutsch)
    German,
    /// French (Français)
    French,
    /// Chinese (中文)
    Chinese,
}

impl DslLanguage {
    /// Returns the human-readable name of the language
    pub fn name(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Japanese => "日本語 (Japanese)",
            Self::German => "Deutsch (German)",
            Self::French => "Français (French)",
            Self::Chinese => "中文 (Chinese)",
        }
    }

    /// Returns all supported languages
    pub fn all() -> Vec<DslLanguage> {
        vec![
            Self::English,
            Self::Japanese,
            Self::German,
            Self::French,
            Self::Chinese,
        ]
    }
}

/// Keyword mapping for a specific language
#[derive(Debug, Clone)]
pub struct KeywordMapping {
    /// The language this mapping represents
    pub language: DslLanguage,
    /// Map from English keywords to localized keywords
    keywords: HashMap<&'static str, &'static str>,
    /// Reverse map from localized keywords to English
    reverse_map: HashMap<String, &'static str>,
}

impl KeywordMapping {
    /// Creates a new keyword mapping for a language
    pub fn new(language: DslLanguage) -> Self {
        let mut mapping = Self {
            language,
            keywords: HashMap::new(),
            reverse_map: HashMap::new(),
        };
        mapping.load_keywords();
        mapping
    }

    /// Loads keywords for the specific language
    fn load_keywords(&mut self) {
        match self.language {
            DslLanguage::English => self.load_english(),
            DslLanguage::Japanese => self.load_japanese(),
            DslLanguage::German => self.load_german(),
            DslLanguage::French => self.load_french(),
            DslLanguage::Chinese => self.load_chinese(),
        }
    }

    /// Loads English keywords (identity mapping)
    fn load_english(&mut self) {
        let keywords = [
            "STATUTE",
            "WHEN",
            "THEN",
            "DEFAULT",
            "GRANT",
            "REVOKE",
            "OBLIGATION",
            "PROHIBITION",
            "DISCRETION",
            "EXCEPTION",
            "AMENDMENT",
            "SUPERSEDES",
            "IMPORT",
            "EFFECTIVE_DATE",
            "EXPIRY_DATE",
            "JURISDICTION",
            "VERSION",
            "HAS",
            "AND",
            "OR",
            "NOT",
            "BETWEEN",
            "IN",
            "LIKE",
            "MATCHES",
            "UNLESS",
            "REQUIRES",
            "DELEGATE",
            "PRIORITY",
            "SCOPE",
            "CONSTRAINT",
            "NAMESPACE",
            "FROM",
            "PUBLIC",
            "PRIVATE",
            "EXPORT",
            "MACRO",
            "IF",
            "ELSE",
            "AGE",
            "INCOME",
        ];

        for keyword in &keywords {
            self.add_mapping(keyword, keyword);
        }
    }

    /// Loads Japanese keywords
    fn load_japanese(&mut self) {
        self.add_mapping("STATUTE", "法令");
        self.add_mapping("WHEN", "条件");
        self.add_mapping("THEN", "効果");
        self.add_mapping("DEFAULT", "既定値");
        self.add_mapping("GRANT", "付与");
        self.add_mapping("REVOKE", "取消");
        self.add_mapping("OBLIGATION", "義務");
        self.add_mapping("PROHIBITION", "禁止");
        self.add_mapping("DISCRETION", "裁量");
        self.add_mapping("EXCEPTION", "例外");
        self.add_mapping("AMENDMENT", "改正");
        self.add_mapping("SUPERSEDES", "優先");
        self.add_mapping("IMPORT", "参照");
        self.add_mapping("EFFECTIVE_DATE", "施行日");
        self.add_mapping("EXPIRY_DATE", "失効日");
        self.add_mapping("JURISDICTION", "管轄");
        self.add_mapping("VERSION", "版");
        self.add_mapping("HAS", "有");
        self.add_mapping("AND", "且つ");
        self.add_mapping("OR", "又は");
        self.add_mapping("NOT", "否");
        self.add_mapping("BETWEEN", "範囲");
        self.add_mapping("IN", "含");
        self.add_mapping("LIKE", "類似");
        self.add_mapping("MATCHES", "一致");
        self.add_mapping("UNLESS", "除外");
        self.add_mapping("REQUIRES", "要求");
        self.add_mapping("DELEGATE", "委任");
        self.add_mapping("PRIORITY", "優先度");
        self.add_mapping("SCOPE", "適用範囲");
        self.add_mapping("CONSTRAINT", "制約");
        self.add_mapping("NAMESPACE", "名前空間");
        self.add_mapping("FROM", "元");
        self.add_mapping("PUBLIC", "公開");
        self.add_mapping("PRIVATE", "非公開");
        self.add_mapping("EXPORT", "公開");
        self.add_mapping("MACRO", "定義");
        self.add_mapping("IF", "もし");
        self.add_mapping("ELSE", "他");
        self.add_mapping("AGE", "年齢");
        self.add_mapping("INCOME", "所得");
    }

    /// Loads German keywords
    fn load_german(&mut self) {
        self.add_mapping("STATUTE", "GESETZ");
        self.add_mapping("WHEN", "WENN");
        self.add_mapping("THEN", "DANN");
        self.add_mapping("DEFAULT", "STANDARD");
        self.add_mapping("GRANT", "GEWÄHREN");
        self.add_mapping("REVOKE", "WIDERRUFEN");
        self.add_mapping("OBLIGATION", "PFLICHT");
        self.add_mapping("PROHIBITION", "VERBOT");
        self.add_mapping("DISCRETION", "ERMESSEN");
        self.add_mapping("EXCEPTION", "AUSNAHME");
        self.add_mapping("AMENDMENT", "ÄNDERUNG");
        self.add_mapping("SUPERSEDES", "ERSETZT");
        self.add_mapping("IMPORT", "IMPORTIEREN");
        self.add_mapping("EFFECTIVE_DATE", "INKRAFTTRETEN");
        self.add_mapping("EXPIRY_DATE", "ABLAUFDATUM");
        self.add_mapping("JURISDICTION", "ZUSTÄNDIGKEIT");
        self.add_mapping("VERSION", "VERSION");
        self.add_mapping("HAS", "HAT");
        self.add_mapping("AND", "UND");
        self.add_mapping("OR", "ODER");
        self.add_mapping("NOT", "NICHT");
        self.add_mapping("BETWEEN", "ZWISCHEN");
        self.add_mapping("IN", "IN");
        self.add_mapping("LIKE", "WIE");
        self.add_mapping("MATCHES", "ENTSPRICHT");
        self.add_mapping("UNLESS", "AUSSER");
        self.add_mapping("REQUIRES", "ERFORDERT");
        self.add_mapping("DELEGATE", "DELEGIEREN");
        self.add_mapping("PRIORITY", "PRIORITÄT");
        self.add_mapping("SCOPE", "GELTUNGSBEREICH");
        self.add_mapping("CONSTRAINT", "EINSCHRÄNKUNG");
        self.add_mapping("NAMESPACE", "NAMENSRAUM");
        self.add_mapping("FROM", "VON");
        self.add_mapping("PUBLIC", "ÖFFENTLICH");
        self.add_mapping("PRIVATE", "PRIVAT");
        self.add_mapping("EXPORT", "EXPORTIEREN");
        self.add_mapping("MACRO", "MAKRO");
        self.add_mapping("IF", "FALLS");
        self.add_mapping("ELSE", "SONST");
        self.add_mapping("AGE", "ALTER");
        self.add_mapping("INCOME", "EINKOMMEN");
    }

    /// Loads French keywords
    fn load_french(&mut self) {
        self.add_mapping("STATUTE", "LOI");
        self.add_mapping("WHEN", "QUAND");
        self.add_mapping("THEN", "ALORS");
        self.add_mapping("DEFAULT", "DÉFAUT");
        self.add_mapping("GRANT", "ACCORDER");
        self.add_mapping("REVOKE", "RÉVOQUER");
        self.add_mapping("OBLIGATION", "OBLIGATION");
        self.add_mapping("PROHIBITION", "INTERDICTION");
        self.add_mapping("DISCRETION", "DISCRÉTION");
        self.add_mapping("EXCEPTION", "EXCEPTION");
        self.add_mapping("AMENDMENT", "AMENDEMENT");
        self.add_mapping("SUPERSEDES", "REMPLACE");
        self.add_mapping("IMPORT", "IMPORTER");
        self.add_mapping("EFFECTIVE_DATE", "DATE_EFFET");
        self.add_mapping("EXPIRY_DATE", "DATE_EXPIRATION");
        self.add_mapping("JURISDICTION", "JURIDICTION");
        self.add_mapping("VERSION", "VERSION");
        self.add_mapping("HAS", "A");
        self.add_mapping("AND", "ET");
        self.add_mapping("OR", "OU");
        self.add_mapping("NOT", "NON");
        self.add_mapping("BETWEEN", "ENTRE");
        self.add_mapping("IN", "DANS");
        self.add_mapping("LIKE", "COMME");
        self.add_mapping("MATCHES", "CORRESPOND");
        self.add_mapping("UNLESS", "SAUF");
        self.add_mapping("REQUIRES", "NÉCESSITE");
        self.add_mapping("DELEGATE", "DÉLÉGUER");
        self.add_mapping("PRIORITY", "PRIORITÉ");
        self.add_mapping("SCOPE", "PORTÉE");
        self.add_mapping("CONSTRAINT", "CONTRAINTE");
        self.add_mapping("NAMESPACE", "ESPACE_NOMS");
        self.add_mapping("FROM", "DE");
        self.add_mapping("PUBLIC", "PUBLIC");
        self.add_mapping("PRIVATE", "PRIVÉ");
        self.add_mapping("EXPORT", "EXPORTER");
        self.add_mapping("MACRO", "MACRO");
        self.add_mapping("IF", "SI");
        self.add_mapping("ELSE", "SINON");
        self.add_mapping("AGE", "ÂGE");
        self.add_mapping("INCOME", "REVENU");
    }

    /// Loads Chinese keywords
    fn load_chinese(&mut self) {
        self.add_mapping("STATUTE", "法规");
        self.add_mapping("WHEN", "当");
        self.add_mapping("THEN", "则");
        self.add_mapping("DEFAULT", "默认");
        self.add_mapping("GRANT", "授予");
        self.add_mapping("REVOKE", "撤销");
        self.add_mapping("OBLIGATION", "义务");
        self.add_mapping("PROHIBITION", "禁止");
        self.add_mapping("DISCRETION", "酌情");
        self.add_mapping("EXCEPTION", "例外");
        self.add_mapping("AMENDMENT", "修正");
        self.add_mapping("SUPERSEDES", "取代");
        self.add_mapping("IMPORT", "引用");
        self.add_mapping("EFFECTIVE_DATE", "生效日期");
        self.add_mapping("EXPIRY_DATE", "失效日期");
        self.add_mapping("JURISDICTION", "管辖");
        self.add_mapping("VERSION", "版本");
        self.add_mapping("HAS", "有");
        self.add_mapping("AND", "且");
        self.add_mapping("OR", "或");
        self.add_mapping("NOT", "非");
        self.add_mapping("BETWEEN", "范围");
        self.add_mapping("IN", "属于");
        self.add_mapping("LIKE", "类似");
        self.add_mapping("MATCHES", "匹配");
        self.add_mapping("UNLESS", "除非");
        self.add_mapping("REQUIRES", "需要");
        self.add_mapping("DELEGATE", "委托");
        self.add_mapping("PRIORITY", "优先级");
        self.add_mapping("SCOPE", "适用范围");
        self.add_mapping("CONSTRAINT", "约束");
        self.add_mapping("NAMESPACE", "命名空间");
        self.add_mapping("FROM", "从");
        self.add_mapping("PUBLIC", "公开");
        self.add_mapping("PRIVATE", "私有");
        self.add_mapping("EXPORT", "导出");
        self.add_mapping("MACRO", "宏");
        self.add_mapping("IF", "如果");
        self.add_mapping("ELSE", "否则");
        self.add_mapping("AGE", "年龄");
        self.add_mapping("INCOME", "收入");
    }

    /// Adds a keyword mapping
    fn add_mapping(&mut self, english: &'static str, localized: &'static str) {
        self.keywords.insert(english, localized);
        self.reverse_map.insert(localized.to_string(), english);
    }

    /// Translates an English keyword to the target language
    pub fn to_localized(&self, english_keyword: &str) -> Option<&str> {
        self.keywords.get(english_keyword).copied()
    }

    /// Translates a localized keyword back to English
    pub fn to_english(&self, localized_keyword: &str) -> Option<&str> {
        self.reverse_map.get(localized_keyword).copied()
    }

    /// Checks if a string is a keyword in this language
    pub fn is_keyword(&self, word: &str) -> bool {
        self.reverse_map.contains_key(word) || self.keywords.contains_key(word)
    }

    /// Gets all localized keywords for this language
    pub fn all_keywords(&self) -> Vec<&str> {
        self.reverse_map.keys().map(|s| s.as_str()).collect()
    }
}

/// Multi-language DSL translator
#[derive(Debug)]
pub struct MultiLangTranslator {
    mappings: HashMap<DslLanguage, KeywordMapping>,
}

impl Default for MultiLangTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiLangTranslator {
    /// Creates a new multi-language translator
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        for lang in DslLanguage::all() {
            mappings.insert(lang, KeywordMapping::new(lang));
        }

        Self { mappings }
    }

    /// Gets the keyword mapping for a specific language
    pub fn get_mapping(&self, language: DslLanguage) -> Option<&KeywordMapping> {
        self.mappings.get(&language)
    }

    /// Translates DSL code from one language to another
    pub fn translate(&self, code: &str, from: DslLanguage, to: DslLanguage) -> String {
        if from == to {
            return code.to_string();
        }

        let from_mapping = self
            .mappings
            .get(&from)
            .expect("Unsupported source language");
        let to_mapping = self.mappings.get(&to).expect("Unsupported target language");

        let mut result = code.to_string();

        // Replace keywords from source language to target language
        for (english, localized_from) in &from_mapping.keywords {
            if let Some(localized_to) = to_mapping.to_localized(english) {
                // Use word boundaries to avoid partial replacements
                let pattern = format!(r"\b{}\b", regex::escape(localized_from));
                if let Ok(re) = regex::Regex::new(&pattern) {
                    result = re.replace_all(&result, localized_to).to_string();
                }
            }
        }

        result
    }

    /// Normalizes DSL code to English (canonical form)
    pub fn normalize_to_english(&self, code: &str, from: DslLanguage) -> String {
        self.translate(code, from, DslLanguage::English)
    }

    /// Detects the likely language of DSL code
    pub fn detect_language(&self, code: &str) -> DslLanguage {
        let mut scores: HashMap<DslLanguage, usize> = HashMap::new();

        for (lang, mapping) in &self.mappings {
            let mut score = 0;
            for keyword in mapping.all_keywords() {
                if code.contains(keyword) {
                    score += 1;
                }
            }
            scores.insert(*lang, score);
        }

        // Return language with highest score
        scores
            .into_iter()
            .max_by_key(|(_, score)| *score)
            .map(|(lang, _)| lang)
            .unwrap_or(DslLanguage::English)
    }
}

/// Example DSL code generator for different languages
pub struct LanguageExamples;

impl LanguageExamples {
    /// Generates a simple voting rights statute in the specified language
    pub fn voting_rights_example(language: DslLanguage) -> String {
        let translator = MultiLangTranslator::new();
        let english_code = r#"STATUTE voting-rights: "Voting Rights" {
    JURISDICTION "US-CA"
    VERSION 1
    EFFECTIVE_DATE 2024-01-01
    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Right to vote"
    EXCEPTION WHEN HAS felony_conviction "Felons are excluded"
}"#;

        translator.translate(english_code, DslLanguage::English, language)
    }

    /// Generates a tax benefit statute in the specified language
    pub fn tax_benefit_example(language: DslLanguage) -> String {
        let translator = MultiLangTranslator::new();
        let english_code = r#"STATUTE child-tax-credit: "Child Tax Credit" {
    JURISDICTION "US"
    VERSION 2
    WHEN AGE < 17 AND HAS dependent
    WHEN INCOME < 200000
    THEN GRANT "Tax credit of $2000"
}"#;

        translator.translate(english_code, DslLanguage::English, language)
    }

    /// Generates a driver's license statute in the specified language
    pub fn drivers_license_example(language: DslLanguage) -> String {
        let translator = MultiLangTranslator::new();
        let english_code = r#"STATUTE drivers-license: "Driver's License Eligibility" {
    VERSION 1
    WHEN AGE >= 16 AND HAS drivers_ed_certificate
    THEN GRANT "Learner's permit"
    WHEN AGE >= 18
    THEN GRANT "Full driver's license"
}"#;

        translator.translate(english_code, DslLanguage::English, language)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_mapping_japanese() {
        let mapping = KeywordMapping::new(DslLanguage::Japanese);

        assert_eq!(mapping.to_localized("STATUTE"), Some("法令"));
        assert_eq!(mapping.to_localized("WHEN"), Some("条件"));
        assert_eq!(mapping.to_localized("THEN"), Some("効果"));
        assert_eq!(mapping.to_english("法令"), Some("STATUTE"));
        assert_eq!(mapping.to_english("条件"), Some("WHEN"));
    }

    #[test]
    fn test_keyword_mapping_german() {
        let mapping = KeywordMapping::new(DslLanguage::German);

        assert_eq!(mapping.to_localized("STATUTE"), Some("GESETZ"));
        assert_eq!(mapping.to_localized("WHEN"), Some("WENN"));
        assert_eq!(mapping.to_localized("AND"), Some("UND"));
        assert_eq!(mapping.to_english("GESETZ"), Some("STATUTE"));
    }

    #[test]
    fn test_keyword_mapping_french() {
        let mapping = KeywordMapping::new(DslLanguage::French);

        assert_eq!(mapping.to_localized("STATUTE"), Some("LOI"));
        assert_eq!(mapping.to_localized("WHEN"), Some("QUAND"));
        assert_eq!(mapping.to_localized("AND"), Some("ET"));
        assert_eq!(mapping.to_english("LOI"), Some("STATUTE"));
    }

    #[test]
    fn test_keyword_mapping_chinese() {
        let mapping = KeywordMapping::new(DslLanguage::Chinese);

        assert_eq!(mapping.to_localized("STATUTE"), Some("法规"));
        assert_eq!(mapping.to_localized("WHEN"), Some("当"));
        assert_eq!(mapping.to_localized("GRANT"), Some("授予"));
        assert_eq!(mapping.to_english("法规"), Some("STATUTE"));
    }

    #[test]
    fn test_is_keyword() {
        let mapping = KeywordMapping::new(DslLanguage::Japanese);

        assert!(mapping.is_keyword("法令"));
        assert!(mapping.is_keyword("条件"));
        assert!(!mapping.is_keyword("random_word"));
    }

    #[test]
    fn test_translate_english_to_japanese() {
        let translator = MultiLangTranslator::new();
        let english = "STATUTE test WHEN AGE >= 18 THEN GRANT rights";
        let japanese = translator.translate(english, DslLanguage::English, DslLanguage::Japanese);

        assert!(japanese.contains("法令"));
        assert!(japanese.contains("条件"));
        assert!(japanese.contains("年齢"));
        assert!(japanese.contains("付与"));
    }

    #[test]
    fn test_translate_english_to_german() {
        let translator = MultiLangTranslator::new();
        let english = "STATUTE test WHEN AGE >= 18 THEN GRANT rights";
        let german = translator.translate(english, DslLanguage::English, DslLanguage::German);

        assert!(german.contains("GESETZ"));
        assert!(german.contains("WENN"));
        assert!(german.contains("ALTER"));
        assert!(german.contains("GEWÄHREN"));
    }

    #[test]
    fn test_translate_roundtrip() {
        let translator = MultiLangTranslator::new();
        let english = "STATUTE test WHEN AGE >= 18";
        let japanese = translator.translate(english, DslLanguage::English, DslLanguage::Japanese);
        let back_to_english =
            translator.translate(&japanese, DslLanguage::Japanese, DslLanguage::English);

        // Keywords should be preserved
        assert!(back_to_english.contains("STATUTE"));
        assert!(back_to_english.contains("WHEN"));
        assert!(back_to_english.contains("AGE"));
    }

    #[test]
    fn test_normalize_to_english() {
        let translator = MultiLangTranslator::new();
        let japanese = "法令 test 条件 年齢 >= 18";
        let english = translator.normalize_to_english(japanese, DslLanguage::Japanese);

        assert!(english.contains("STATUTE"));
        assert!(english.contains("WHEN"));
        assert!(english.contains("AGE"));
    }

    #[test]
    fn test_detect_language_english() {
        let translator = MultiLangTranslator::new();
        let code = "STATUTE test WHEN AGE >= 18 THEN GRANT rights";
        let detected = translator.detect_language(code);

        assert_eq!(detected, DslLanguage::English);
    }

    #[test]
    fn test_detect_language_japanese() {
        let translator = MultiLangTranslator::new();
        let code = "法令 test 条件 年齢 >= 18 効果 付与 rights";
        let detected = translator.detect_language(code);

        assert_eq!(detected, DslLanguage::Japanese);
    }

    #[test]
    fn test_detect_language_german() {
        let translator = MultiLangTranslator::new();
        let code = "GESETZ test WENN ALTER >= 18 DANN GEWÄHREN rights";
        let detected = translator.detect_language(code);

        assert_eq!(detected, DslLanguage::German);
    }

    #[test]
    fn test_language_examples_japanese() {
        let example = LanguageExamples::voting_rights_example(DslLanguage::Japanese);

        assert!(example.contains("法令"));
        assert!(example.contains("管轄"));
        assert!(example.contains("条件"));
        assert!(example.contains("年齢"));
    }

    #[test]
    fn test_language_examples_german() {
        let example = LanguageExamples::voting_rights_example(DslLanguage::German);

        assert!(example.contains("GESETZ"));
        assert!(example.contains("ZUSTÄNDIGKEIT"));
        assert!(example.contains("WENN"));
        assert!(example.contains("ALTER"));
    }

    #[test]
    fn test_language_examples_french() {
        let example = LanguageExamples::voting_rights_example(DslLanguage::French);

        assert!(example.contains("LOI"));
        assert!(example.contains("JURIDICTION"));
        assert!(example.contains("QUAND"));
        assert!(example.contains("ÂGE"));
    }

    #[test]
    fn test_language_examples_chinese() {
        let example = LanguageExamples::voting_rights_example(DslLanguage::Chinese);

        assert!(example.contains("法规"));
        assert!(example.contains("管辖"));
        assert!(example.contains("当"));
        assert!(example.contains("年龄"));
    }

    #[test]
    fn test_all_languages_available() {
        let translator = MultiLangTranslator::new();

        for lang in DslLanguage::all() {
            assert!(translator.get_mapping(lang).is_some());
        }
    }

    #[test]
    fn test_language_names() {
        assert_eq!(DslLanguage::English.name(), "English");
        assert_eq!(DslLanguage::Japanese.name(), "日本語 (Japanese)");
        assert_eq!(DslLanguage::German.name(), "Deutsch (German)");
        assert_eq!(DslLanguage::French.name(), "Français (French)");
        assert_eq!(DslLanguage::Chinese.name(), "中文 (Chinese)");
    }
}
