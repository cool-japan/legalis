//! Top-level legal research orchestrator.
//!
//! [`LegalResearchAssistant`] ties the research sub-systems together behind a
//! single, ergonomic surface: an in-memory [`ResearchCorpus`], issue spotting,
//! authority ranking, precedent analysis, citation validation and IRAC memo
//! generation. Every method works offline; where a [`crate::LLMProvider`] is
//! available, [`LegalResearchAssistant::augment_memo`] can optionally enrich a
//! generated memo, but the assistant never depends on a live LLM call.

use super::{
    AuthorityRanker, CitationValidation, CitationValidator, Forum, IssueSpotter, LegalAuthority,
    MemoGenerator, PrecedentAnalyzer, PrecedentAssessment, RankedAuthority, ResearchCorpus,
    ResearchHit, ResearchMemo, ResearchRequest, SpottedIssue,
};
use crate::{Jurisdiction, LLMProvider};
use anyhow::Result;

/// Default number of strongest authorities surfaced in a research report.
const DEFAULT_TOP_AUTHORITIES: usize = 10;

/// A comprehensive research report.
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchReport {
    /// The generated IRAC memo.
    pub memo: ResearchMemo,
    /// Issues spotted in the facts.
    pub issues: Vec<SpottedIssue>,
    /// Validation results for every citation referenced in the memo.
    pub citation_validations: Vec<CitationValidation>,
    /// The strongest authorities in the corpus.
    pub top_authorities: Vec<RankedAuthority>,
}

impl ResearchReport {
    /// Returns whether every cited authority validated cleanly (no errors).
    pub fn all_citations_valid(&self) -> bool {
        self.citation_validations.iter().all(|v| v.is_valid)
    }
}

/// The legal research assistant.
#[derive(Debug, Clone)]
pub struct LegalResearchAssistant {
    corpus: ResearchCorpus,
    spotter: IssueSpotter,
    ranker: AuthorityRanker,
    precedent: PrecedentAnalyzer,
    validator: CitationValidator,
}

impl LegalResearchAssistant {
    /// Builds an assistant with an empty corpus and default components.
    ///
    /// Returns an error only if the citation validator's patterns fail to
    /// compile (which cannot happen with the built-in patterns).
    pub fn new() -> Result<Self> {
        Ok(Self {
            corpus: ResearchCorpus::new(),
            spotter: IssueSpotter::new(),
            ranker: AuthorityRanker::new(),
            precedent: PrecedentAnalyzer::new(),
            validator: CitationValidator::new()?,
        })
    }

    /// Builds an assistant around a pre-populated corpus.
    pub fn with_corpus(corpus: ResearchCorpus) -> Result<Self> {
        let mut assistant = Self::new()?;
        assistant.corpus = corpus;
        Ok(assistant)
    }

    /// Overrides the issue spotter.
    pub fn with_spotter(mut self, spotter: IssueSpotter) -> Self {
        self.spotter = spotter;
        self
    }

    /// Overrides the authority ranker.
    pub fn with_ranker(mut self, ranker: AuthorityRanker) -> Self {
        self.ranker = ranker;
        self
    }

    /// Overrides the precedent analyser.
    pub fn with_precedent_analyzer(mut self, precedent: PrecedentAnalyzer) -> Self {
        self.precedent = precedent;
        self
    }

    /// Returns a reference to the corpus.
    pub fn corpus(&self) -> &ResearchCorpus {
        &self.corpus
    }

    /// Returns a mutable reference to the corpus.
    pub fn corpus_mut(&mut self) -> &mut ResearchCorpus {
        &mut self.corpus
    }

    /// Adds an authority to the corpus.
    pub fn add_authority(&mut self, authority: LegalAuthority) -> Result<()> {
        self.corpus.add(authority)
    }

    /// Adds many authorities to the corpus.
    pub fn add_authorities<I>(&mut self, authorities: I) -> Result<()>
    where
        I: IntoIterator<Item = LegalAuthority>,
    {
        self.corpus.add_many(authorities)
    }

    // --- search -------------------------------------------------------------

    /// Searches the corpus (BM25).
    pub fn search(&self, query: &str, top_k: usize) -> Vec<ResearchHit> {
        self.corpus.search(query, top_k)
    }

    /// Searches case law, optionally restricted to a jurisdiction.
    pub fn find_cases(
        &self,
        query: &str,
        top_k: usize,
        jurisdiction: Option<Jurisdiction>,
    ) -> Vec<ResearchHit> {
        self.corpus.search_cases(query, top_k, jurisdiction)
    }

    /// Searches statutes, optionally restricted to a jurisdiction.
    pub fn find_statutes(
        &self,
        query: &str,
        top_k: usize,
        jurisdiction: Option<Jurisdiction>,
    ) -> Vec<ResearchHit> {
        self.corpus.search_statutes(query, top_k, jurisdiction)
    }

    /// Jurisdiction-specific full-text search across all authority types.
    pub fn search_in_jurisdiction(
        &self,
        query: &str,
        jurisdiction: Jurisdiction,
        top_k: usize,
    ) -> Vec<ResearchHit> {
        let options = super::SearchOptions::new(top_k).with_jurisdiction(jurisdiction);
        self.corpus.search_with(query, &options)
    }

    // --- analysis -----------------------------------------------------------

    /// Spots legal issues in a fact pattern.
    pub fn spot_issues(&self, facts: &str) -> Vec<SpottedIssue> {
        self.spotter.spot(facts)
    }

    /// Ranks the strongest authorities in the corpus.
    pub fn rank_authorities(&self, top_k: usize) -> Vec<RankedAuthority> {
        self.ranker.rank(self.corpus.authorities(), top_k)
    }

    /// Analyses the precedential value of one authority in a forum.
    pub fn analyze_precedent(
        &self,
        authority_id: &str,
        forum: &Forum,
        facts: &str,
    ) -> Option<PrecedentAssessment> {
        let authority = self.corpus.get(authority_id)?;
        Some(self.precedent.analyze(authority, forum, facts))
    }

    /// Ranks all corpus authorities by precedential weight for a forum.
    pub fn rank_precedents(
        &self,
        forum: &Forum,
        facts: &str,
        top_k: usize,
    ) -> Vec<PrecedentAssessment> {
        self.precedent
            .rank_precedents(self.corpus.authorities(), forum, facts, top_k)
    }

    // --- citations ----------------------------------------------------------

    /// Validates a single citation against the corpus.
    pub fn validate_citation(&self, citation: &str) -> CitationValidation {
        self.validator
            .validate_against_corpus(citation, &self.corpus)
    }

    /// Extracts and validates every citation found in a block of text.
    pub fn validate_citations_in(&self, text: &str) -> Vec<CitationValidation> {
        self.validator
            .extract_all(text)
            .into_iter()
            .map(|c| self.validator.validate_against_corpus(&c.raw, &self.corpus))
            .collect()
    }

    /// Returns the citation validator.
    pub fn validator(&self) -> &CitationValidator {
        &self.validator
    }

    // --- memo / report ------------------------------------------------------

    /// Generates an IRAC research memo for the request.
    pub fn generate_memo(&self, request: &ResearchRequest) -> ResearchMemo {
        self.memo_generator().generate(request, &self.corpus)
    }

    /// Runs the full research pipeline and returns a comprehensive report.
    pub fn research(&self, request: &ResearchRequest) -> ResearchReport {
        let combined = format!("{} {}", request.question, request.facts);
        let issues = self.spotter.spot(&combined);
        let memo = self.memo_generator().generate(request, &self.corpus);

        let citation_validations = memo
            .authorities_cited
            .iter()
            .map(|c| self.validator.validate_against_corpus(c, &self.corpus))
            .collect();

        let top_authorities = self.ranker.rank(
            self.corpus.authorities(),
            DEFAULT_TOP_AUTHORITIES.min(self.corpus.len().max(1)),
        );

        ResearchReport {
            memo,
            issues,
            citation_validations,
            top_authorities,
        }
    }

    /// Optionally enriches a generated memo using an LLM provider.
    ///
    /// This is the *only* method that requires a live provider; every other
    /// capability of the assistant works offline. The memo's structure and
    /// citations are passed to the provider with an instruction to refine the
    /// prose while preserving all authorities and the IRAC organisation.
    pub async fn augment_memo<P: LLMProvider>(
        &self,
        memo: &ResearchMemo,
        provider: &P,
    ) -> Result<String> {
        let prompt = build_augmentation_prompt(memo);
        provider.generate_text(&prompt).await
    }

    // --- internals ----------------------------------------------------------

    fn memo_generator(&self) -> MemoGenerator {
        MemoGenerator::with_components(
            self.spotter.clone(),
            self.ranker.clone(),
            self.precedent.clone(),
        )
    }
}

fn build_augmentation_prompt(memo: &ResearchMemo) -> String {
    format!(
        "You are a senior legal research attorney. Refine and expand the following research \
         memorandum. Preserve every citation exactly as written, keep the IRAC structure, and \
         do not introduce authorities that are not already cited. Improve clarity, tighten the \
         legal analysis, and ensure the conclusion follows from the application.\n\n\
         ---\n{}\n---\n\nReturn the improved memorandum in Markdown.",
        memo.to_markdown()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthorityType, CourtLevel, TextStream, TreatmentType};
    use async_trait::async_trait;
    use serde::de::DeserializeOwned;

    fn populated_assistant() -> LegalResearchAssistant {
        let mut assistant = LegalResearchAssistant::new().expect("assistant builds");
        assistant
            .add_authorities(vec![
                LegalAuthority::new(
                    "neg",
                    "Palsgraf v. Long Island Railroad",
                    "248 N.Y. 339 (1928)",
                    "Negligence requires a duty of care, breach, causation, and damages to a \
                     foreseeable plaintiff.",
                    AuthorityType::Case,
                    Jurisdiction::UsFederal,
                )
                .with_court_level(CourtLevel::Supreme)
                .with_year(1928)
                .with_citation_count(5000)
                .with_treatment(TreatmentType::Followed),
                LegalAuthority::new(
                    "stat",
                    "Civil Rights Act",
                    "42 U.S.C. § 1983",
                    "Deprivation of constitutional rights under color of law gives rise to \
                     liability for damages.",
                    AuthorityType::Statute,
                    Jurisdiction::UsFederal,
                )
                .with_year(1871)
                .with_citation_count(12000),
            ])
            .expect("authorities added");
        assistant
    }

    fn negligence_request() -> ResearchRequest {
        ResearchRequest::new(
            "Is the defendant liable in negligence?",
            "The defendant owed a duty of care, breached that duty, and the breach caused \
             damages and injury to the plaintiff.",
        )
        .with_forum(Forum::new(
            Jurisdiction::UsState("Texas".into()),
            CourtLevel::Trial,
        ))
    }

    #[test]
    fn test_assistant_builds_and_searches() {
        let assistant = populated_assistant();
        let hits = assistant.search("duty of care negligence", 5);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].id, "neg");
    }

    #[test]
    fn test_find_statutes_and_jurisdiction_search() {
        let assistant = populated_assistant();
        let statutes = assistant.find_statutes("deprivation constitutional rights", 5, None);
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].id, "stat");

        let federal = assistant.search_in_jurisdiction("liability", Jurisdiction::UsFederal, 10);
        assert!(!federal.is_empty());
    }

    #[test]
    fn test_spot_issues_and_rank_authorities() {
        let assistant = populated_assistant();
        let issues = assistant.spot_issues(
            "The defendant owed a duty of care, breached it, causing damages and injury.",
        );
        assert!(issues.iter().any(|i| i.issue == "Negligence"));

        let ranked = assistant.rank_authorities(10);
        assert_eq!(ranked.len(), 2);
        // The followed, heavily-cited Supreme Court case should rank first.
        assert_eq!(ranked[0].authority_id, "neg");
    }

    #[test]
    fn test_analyze_precedent() {
        let assistant = populated_assistant();
        let forum = Forum::new(Jurisdiction::UsState("Texas".into()), CourtLevel::Trial);
        let assessment = assistant
            .analyze_precedent("neg", &forum, "negligence duty of care")
            .expect("authority exists");
        assert_eq!(assessment.binding, super::super::BindingStatus::Binding);

        assert!(
            assistant
                .analyze_precedent("does_not_exist", &forum, "facts")
                .is_none()
        );
    }

    #[test]
    fn test_validate_citations_in_text() {
        let assistant = populated_assistant();
        let text = "Per 248 N.Y. 339 (1928) and the dangling cite 999 U.S. 111 (2050).";
        let validations = assistant.validate_citations_in(text);
        assert!(validations.len() >= 2);
        // The known authority should not be flagged as dangling.
        let known = validations
            .iter()
            .find(|v| v.normalized.contains("248 N.Y. 339"))
            .expect("known citation parsed");
        assert!(!known.issues.iter().any(|i| i.message.contains("dangling")));
    }

    #[test]
    fn test_full_research_report() {
        let assistant = populated_assistant();
        let report = assistant.research(&negligence_request());
        assert!(report.issues.iter().any(|i| i.issue == "Negligence"));
        assert!(!report.memo.authorities_cited.is_empty());
        assert!(!report.top_authorities.is_empty());
        // Citations come from the corpus, so they should resolve (not dangling).
        assert!(report.all_citations_valid());
    }

    // Minimal offline mock provider for the optional augmentation path.
    struct MockProvider;

    #[async_trait]
    impl LLMProvider for MockProvider {
        async fn generate_text(&self, prompt: &str) -> Result<String> {
            Ok(format!("REFINED MEMO based on {} chars", prompt.len()))
        }

        async fn generate_structured<T: DeserializeOwned + Send>(
            &self,
            _prompt: &str,
        ) -> Result<T> {
            anyhow::bail!("structured output not supported by mock")
        }

        async fn generate_text_stream(&self, _prompt: &str) -> Result<TextStream> {
            anyhow::bail!("streaming not supported by mock")
        }

        fn provider_name(&self) -> &str {
            "mock"
        }

        fn model_name(&self) -> &str {
            "mock-model"
        }
    }

    #[tokio::test]
    async fn test_optional_llm_augmentation() {
        let assistant = populated_assistant();
        let memo = assistant.generate_memo(&negligence_request());
        let provider = MockProvider;
        let augmented = assistant
            .augment_memo(&memo, &provider)
            .await
            .expect("augmentation succeeds");
        assert!(augmented.starts_with("REFINED MEMO"));
        assert!(!provider.supports_streaming());
    }
}
