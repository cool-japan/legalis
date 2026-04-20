//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types_3::SubtitlePosition;
use super::types_4::{HistoricalPeriod, InterpretedSegment, SimultaneousInterpreter};
use super::types_5::MultilingualEmbedder;
use super::types_6::{CourtProceedingTranslator, RiskFactor};
use super::types_8::{CourtParticipant, RiskLevel};
use super::types_9::TranscriptionSegment;
use super::types_10::{LegalCase, Locale};
use super::types_12::SearchResult;

/// Multilingual hearing support with channel management.
#[derive(Debug, Clone)]
pub struct MultilingualHearing {
    /// Hearing title/case name.
    pub title: String,
    /// Primary language of the hearing.
    pub primary_language: Locale,
    /// All languages being used in the hearing.
    pub active_languages: Vec<Locale>,
    /// Interpreters by channel (language pair).
    pub interpretation_channels: HashMap<String, SimultaneousInterpreter>,
    /// Court proceeding translator.
    pub court_translator: CourtProceedingTranslator,
    /// Whether to provide closed captions.
    pub closed_captions: bool,
}
impl MultilingualHearing {
    /// Creates a new multilingual hearing.
    pub fn new(title: impl Into<String>, primary_language: Locale) -> Self {
        Self {
            title: title.into(),
            court_translator: CourtProceedingTranslator::new(primary_language.clone()),
            primary_language: primary_language.clone(),
            active_languages: vec![primary_language],
            interpretation_channels: HashMap::new(),
            closed_captions: true,
        }
    }
    /// Adds a language to the hearing.
    pub fn add_language(&mut self, locale: Locale) {
        if !self.active_languages.contains(&locale) {
            let channel_key = format!("{}_to_{}", self.primary_language.tag(), locale.tag());
            let interpreter = SimultaneousInterpreter::for_court_proceedings(
                self.primary_language.clone(),
                locale.clone(),
            );
            self.interpretation_channels
                .insert(channel_key, interpreter);
            self.active_languages.push(locale);
        }
    }
    /// Adds a participant to the hearing.
    pub fn add_participant(&mut self, participant: CourtParticipant) {
        if participant.requires_interpretation {
            self.add_language(participant.primary_language.clone());
        }
        self.court_translator.add_participant(participant);
    }
    /// Gets the total number of active interpretation channels.
    pub fn channel_count(&self) -> usize {
        self.interpretation_channels.len()
    }
    /// Enables or disables closed captions.
    pub fn with_closed_captions(mut self, enable: bool) -> Self {
        self.closed_captions = enable;
        self
    }
    /// Processes an utterance and returns interpretations for all channels.
    pub fn process_multilingual_utterance(
        &self,
        segment: TranscriptionSegment,
    ) -> HashMap<String, InterpretedSegment> {
        let mut all_interpretations = HashMap::new();
        for (channel_key, interpreter) in &self.interpretation_channels {
            let interpreted = interpreter.interpret_segment(segment.clone());
            all_interpretations.insert(channel_key.clone(), interpreted);
        }
        all_interpretations
    }
}
/// Cross-lingual case search engine.
#[derive(Debug, Clone)]
pub struct CrossLingualCaseSearch {
    /// Multilingual embedder.
    pub embedder: MultilingualEmbedder,
    /// Indexed cases with embeddings.
    pub cases: Vec<LegalCase>,
    /// Minimum similarity threshold for results.
    pub min_similarity: f32,
}
impl CrossLingualCaseSearch {
    /// Creates a new cross-lingual case search engine.
    pub fn new(embedder: MultilingualEmbedder) -> Self {
        Self {
            embedder,
            cases: Vec::new(),
            min_similarity: 0.5,
        }
    }
    /// Sets the minimum similarity threshold.
    pub fn with_min_similarity(mut self, min_similarity: f32) -> Self {
        self.min_similarity = min_similarity.clamp(0.0, 1.0);
        self
    }
    /// Adds a case to the search index.
    pub fn add_case(&mut self, mut case: LegalCase) {
        if case.embedding.is_none() {
            let embedding = self.embedder.embed(&case.summary, case.locale.clone());
            case.embedding = Some(embedding);
        }
        self.cases.push(case);
    }
    /// Searches for similar cases across languages.
    pub fn search(
        &self,
        query: &str,
        query_locale: Locale,
        max_results: usize,
    ) -> Vec<SearchResult> {
        let query_embedding = self.embedder.embed(query, query_locale);
        let mut results: Vec<SearchResult> = self
            .cases
            .iter()
            .filter_map(|case| {
                if let Some(ref case_embedding) = case.embedding {
                    let similarity = query_embedding.cosine_similarity(case_embedding);
                    if similarity >= self.min_similarity {
                        Some((case.clone(), similarity))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
            .map(|(rank, (case, similarity))| SearchResult::new(case, similarity, rank + 1))
            .collect();
        results.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for (i, result) in results.iter_mut().enumerate() {
            result.rank = i + 1;
        }
        results.into_iter().take(max_results).collect()
    }
    /// Searches for cases in a specific jurisdiction.
    pub fn search_by_jurisdiction(
        &self,
        query: &str,
        query_locale: Locale,
        jurisdiction: &str,
        max_results: usize,
    ) -> Vec<SearchResult> {
        let all_results = self.search(query, query_locale, self.cases.len());
        all_results
            .into_iter()
            .filter(|result| result.case.jurisdiction == jurisdiction)
            .take(max_results)
            .enumerate()
            .map(|(i, mut result)| {
                result.rank = i + 1;
                result
            })
            .collect()
    }
    /// Returns the total number of indexed cases.
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }
}
/// Template section that can be conditionally included.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Section name
    pub name: String,
    /// Section content with placeholders
    pub content: String,
    /// Condition for including this section (e.g., "jurisdiction == US")
    pub condition: Option<String>,
}
impl TemplateSection {
    /// Creates a new template section.
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
            condition: None,
        }
    }
    /// Adds a condition for including this section.
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    /// Checks if the condition is met given the context.
    pub fn should_include(&self, context: &HashMap<String, String>) -> bool {
        if let Some(ref condition) = self.condition {
            if let Some((key, rest)) = condition.split_once("==") {
                let key = key.trim();
                let value = rest.trim();
                return context.get(key).map(|v| v == value).unwrap_or(false);
            } else if let Some((key, rest)) = condition.split_once("!=") {
                let key = key.trim();
                let value = rest.trim();
                return context.get(key).map(|v| v != value).unwrap_or(true);
            }
        }
        true
    }
}
/// Local custom type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomType {
    /// Marriage and family customs
    Marriage,
    /// Inheritance and succession
    Inheritance,
    /// Property ownership
    Property,
    /// Business practices
    Business,
    /// Dispute resolution
    DisputeResolution,
    /// Contract formation
    Contract,
}
/// Unicode CLDR legal extension type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegalExtensionType {
    /// Legal system type (u-legal).
    LegalSystem,
    /// Citation style (u-cite).
    CitationStyle,
    /// Court type (u-court).
    CourtType,
    /// Legal formality level (u-formality).
    FormalityLevel,
}
/// Subtitle timing and styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubtitleCue {
    /// Subtitle text content.
    pub text: String,
    /// Start time in milliseconds.
    pub start_ms: u64,
    /// End time in milliseconds.
    pub end_ms: u64,
    /// Language locale of the subtitle.
    pub locale: Locale,
    /// Speaker name (optional).
    pub speaker: Option<String>,
    /// Position on screen (optional, for multi-language subtitles).
    pub position: Option<SubtitlePosition>,
}
impl SubtitleCue {
    /// Creates a new subtitle cue.
    pub fn new(text: impl Into<String>, start_ms: u64, end_ms: u64, locale: Locale) -> Self {
        Self {
            text: text.into(),
            start_ms,
            end_ms,
            locale,
            speaker: None,
            position: None,
        }
    }
    /// Sets the speaker.
    pub fn with_speaker(mut self, speaker: impl Into<String>) -> Self {
        self.speaker = Some(speaker.into());
        self
    }
    /// Sets the position.
    pub fn with_position(mut self, position: SubtitlePosition) -> Self {
        self.position = Some(position);
        self
    }
    /// Formats as WebVTT cue.
    pub fn to_webvtt(&self) -> String {
        let start = Self::format_timestamp(self.start_ms);
        let end = Self::format_timestamp(self.end_ms);
        let mut vtt = format!("{} --> {}\n", start, end);
        if let Some(ref speaker) = self.speaker {
            vtt.push_str(&format!("<v {}>{}</v>\n", speaker, self.text));
        } else {
            vtt.push_str(&format!("{}\n", self.text));
        }
        vtt
    }
    /// Formats as SRT subtitle.
    pub fn to_srt(&self, index: u32) -> String {
        let start = Self::format_timestamp_srt(self.start_ms);
        let end = Self::format_timestamp_srt(self.end_ms);
        let text = if let Some(ref speaker) = self.speaker {
            format!("{}: {}", speaker, self.text)
        } else {
            self.text.clone()
        };
        format!("{}\n{} --> {}\n{}\n\n", index, start, end, text)
    }
    fn format_timestamp(ms: u64) -> String {
        let total_seconds = ms / 1000;
        let milliseconds = ms % 1000;
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = total_seconds / 3600;
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        )
    }
    fn format_timestamp_srt(ms: u64) -> String {
        let total_seconds = ms / 1000;
        let milliseconds = ms % 1000;
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = total_seconds / 3600;
        format!(
            "{:02}:{:02}:{:02},{:03}",
            hours, minutes, seconds, milliseconds
        )
    }
}
/// Legal risk scorer for documents.
#[derive(Debug, Default)]
pub struct LegalRiskScorer {
    /// Risk indicators and their severity
    indicators: HashMap<String, RiskLevel>,
}
impl LegalRiskScorer {
    /// Creates a new legal risk scorer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a risk scorer with default indicators.
    pub fn with_defaults() -> Self {
        let mut scorer = Self::new();
        scorer.add_indicator("unlimited liability", RiskLevel::Critical);
        scorer.add_indicator("no limitation of liability", RiskLevel::Critical);
        scorer.add_indicator("personal guarantee", RiskLevel::High);
        scorer.add_indicator("waive", RiskLevel::High);
        scorer.add_indicator("automatic renewal", RiskLevel::Medium);
        scorer.add_indicator("non-refundable", RiskLevel::Medium);
        scorer.add_indicator("as-is", RiskLevel::Medium);
        scorer.add_indicator("no warranty", RiskLevel::Medium);
        scorer.add_indicator("limitation of liability", RiskLevel::Low);
        scorer.add_indicator("indemnification", RiskLevel::Low);
        scorer.add_indicator("insurance", RiskLevel::Low);
        scorer
    }
    /// Adds a risk indicator.
    pub fn add_indicator(&mut self, indicator: impl Into<String>, level: RiskLevel) {
        self.indicators.insert(indicator.into(), level);
    }
    /// Scores document risk and returns identified factors.
    pub fn score(&self, text: &str) -> (RiskLevel, Vec<RiskFactor>) {
        let mut risk_factors = Vec::new();
        let text_lower = text.to_lowercase();
        let mut overall_score = 0.0;
        for (indicator, level) in &self.indicators {
            if text_lower.contains(&indicator.to_lowercase()) {
                let score_value = match level {
                    RiskLevel::Low => 1.0,
                    RiskLevel::Medium => 2.0,
                    RiskLevel::High => 3.0,
                    RiskLevel::Critical => 4.0,
                };
                overall_score += score_value;
                if let Some(pos) = text_lower.find(&indicator.to_lowercase()) {
                    risk_factors.push(RiskFactor {
                        description: format!("Found: {}", indicator),
                        level: *level,
                        position: pos,
                        mitigation: self.suggest_mitigation(indicator, level),
                    });
                }
            }
        }
        let overall_level = if overall_score >= 10.0 {
            RiskLevel::Critical
        } else if overall_score >= 6.0 {
            RiskLevel::High
        } else if overall_score >= 3.0 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };
        (overall_level, risk_factors)
    }
    fn suggest_mitigation(&self, indicator: &str, level: &RiskLevel) -> Option<String> {
        match (indicator, level) {
            ("unlimited liability", RiskLevel::Critical) => {
                Some("Add limitation of liability clause to cap damages".to_string())
            }
            ("personal guarantee", RiskLevel::High) => {
                Some("Consider corporate guarantee instead of personal".to_string())
            }
            ("automatic renewal", RiskLevel::Medium) => {
                Some("Add notice period for cancellation before renewal".to_string())
            }
            _ => None,
        }
    }
}
/// Archaic legal term with historical context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchaicTerm {
    /// The archaic term
    pub term: String,
    /// Historical period when the term was used
    pub period: HistoricalPeriod,
    /// Modern equivalent term
    pub modern_equivalent: String,
    /// Definition of the term
    pub definition: String,
    /// Example usage in historical context
    pub example: Option<String>,
    /// Locale of the term
    pub locale: Locale,
}
impl ArchaicTerm {
    /// Creates a new archaic term.
    pub fn new(
        term: impl Into<String>,
        period: HistoricalPeriod,
        modern_equivalent: impl Into<String>,
        definition: impl Into<String>,
        locale: Locale,
    ) -> Self {
        Self {
            term: term.into(),
            period,
            modern_equivalent: modern_equivalent.into(),
            definition: definition.into(),
            example: None,
            locale,
        }
    }
    /// Adds an example usage.
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.example = Some(example.into());
        self
    }
}
/// Entry for context-aware translations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ContextualTranslationEntry {
    pub(crate) key: String,
    pub(crate) context: String,
    pub(crate) translation: String,
}
