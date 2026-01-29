//! Legal Reasoning Module for Saudi Arabia
//!
//! This module provides legal reasoning capabilities specific to Saudi law,
//! including Sharia-based reasoning and statutory interpretation.

use serde::{Deserialize, Serialize};

/// Types of legal reasoning in Saudi Arabia
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReasoningType {
    /// Quranic interpretation (تفسير القرآن)
    QuranicInterpretation,
    /// Hadith analysis (تحليل الحديث)
    HadithAnalysis,
    /// Fiqh principles (قواعد فقهية)
    FiqhPrinciples,
    /// Statutory interpretation (تفسير الأنظمة)
    StatutoryInterpretation,
    /// Precedent analysis (تحليل السوابق)
    PrecedentAnalysis,
}

impl ReasoningType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::QuranicInterpretation => "تفسير القرآن",
            Self::HadithAnalysis => "تحليل الحديث",
            Self::FiqhPrinciples => "قواعد فقهية",
            Self::StatutoryInterpretation => "تفسير الأنظمة",
            Self::PrecedentAnalysis => "تحليل السوابق",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::QuranicInterpretation => "Quranic Interpretation",
            Self::HadithAnalysis => "Hadith Analysis",
            Self::FiqhPrinciples => "Fiqh Principles",
            Self::StatutoryInterpretation => "Statutory Interpretation",
            Self::PrecedentAnalysis => "Precedent Analysis",
        }
    }

    /// Get priority level (1 = highest)
    pub fn priority(&self) -> u32 {
        match self {
            Self::QuranicInterpretation => 1, // Highest
            Self::HadithAnalysis => 2,
            Self::FiqhPrinciples => 3,
            Self::StatutoryInterpretation => 4,
            Self::PrecedentAnalysis => 5,
        }
    }
}

/// Legal reasoning structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalReasoning {
    /// Type of reasoning
    pub reasoning_type: ReasoningType,
    /// Source text
    pub source: String,
    /// Interpretation
    pub interpretation: String,
    /// Applicable scenarios
    pub applicable_to: Vec<String>,
}

impl LegalReasoning {
    /// Create new legal reasoning
    pub fn new(
        reasoning_type: ReasoningType,
        source: impl Into<String>,
        interpretation: impl Into<String>,
    ) -> Self {
        Self {
            reasoning_type,
            source: source.into(),
            interpretation: interpretation.into(),
            applicable_to: Vec::new(),
        }
    }

    /// Add applicable scenario
    pub fn add_applicable_to(mut self, scenario: impl Into<String>) -> Self {
        self.applicable_to.push(scenario.into());
        self
    }
}

/// Get common Fiqh principles (قواعد فقهية)
pub fn get_fiqh_principles() -> Vec<(&'static str, &'static str)> {
    vec![
        ("الأمور بمقاصدها", "Matters are judged by their intentions"),
        ("اليقين لا يزول بالشك", "Certainty is not removed by doubt"),
        ("المشقة تجلب التيسير", "Hardship begets ease"),
        ("الضرر يزال", "Harm must be removed"),
        ("العادة محكمة", "Custom is authoritative"),
        (
            "الأصل في الأشياء الإباحة",
            "The default in matters is permissibility",
        ),
        (
            "ما حرم أخذه حرم إعطاؤه",
            "What is forbidden to take is forbidden to give",
        ),
        (
            "درء المفاسد أولى من جلب المصالح",
            "Preventing harm takes precedence over bringing benefit",
        ),
    ]
}

/// Interpret statute in light of Sharia principles
pub fn interpret_with_sharia(statute_text: &str, sharia_principle: &str) -> String {
    format!(
        "Interpretation of statute '{}' in light of Sharia principle '{}': \
         The statute must be understood and applied consistent with Islamic law, \
         ensuring harmony between positive law and Sharia values.",
        statute_text, sharia_principle
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_types() {
        assert_eq!(
            ReasoningType::QuranicInterpretation.name_ar(),
            "تفسير القرآن"
        );
        assert_eq!(ReasoningType::FiqhPrinciples.name_en(), "Fiqh Principles");
    }

    #[test]
    fn test_reasoning_priority() {
        assert_eq!(ReasoningType::QuranicInterpretation.priority(), 1);
        assert_eq!(ReasoningType::HadithAnalysis.priority(), 2);
        assert!(
            ReasoningType::QuranicInterpretation.priority()
                < ReasoningType::FiqhPrinciples.priority()
        );
    }

    #[test]
    fn test_legal_reasoning() {
        let reasoning = LegalReasoning::new(
            ReasoningType::FiqhPrinciples,
            "الأمور بمقاصدها",
            "Actions are judged by intentions",
        )
        .add_applicable_to("Contract interpretation")
        .add_applicable_to("Criminal liability");

        assert_eq!(reasoning.applicable_to.len(), 2);
    }

    #[test]
    fn test_fiqh_principles() {
        let principles = get_fiqh_principles();
        assert!(!principles.is_empty());
        assert!(principles.len() >= 8);

        // Check first principle
        assert!(principles[0].0.contains("الأمور"));
        assert!(principles[0].1.contains("intentions"));
    }

    #[test]
    fn test_sharia_interpretation() {
        let interpretation = interpret_with_sharia(
            "Contract formation requires offer and acceptance",
            "الأمور بمقاصدها",
        );
        assert!(!interpretation.is_empty());
        assert!(interpretation.contains("Sharia"));
    }
}
