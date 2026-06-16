//! IRAC-style research memo generation.
//!
//! [`MemoGenerator`] turns a [`ResearchRequest`] (a legal question plus a fact
//! pattern and forum) into a structured [`ResearchMemo`] by combining issue
//! spotting, corpus search, authority ranking and precedent analysis. The
//! generated memo follows the classic Issue / Rule / Application / Conclusion
//! structure for each spotted issue and renders to Markdown. The whole process
//! is deterministic and requires no LLM.

use super::{
    AuthorityRanker, BindingStatus, Forum, IssueSpotter, PrecedentAnalyzer, ResearchCorpus,
    SearchOptions, SpottedIssue,
};
use serde::{Deserialize, Serialize};

/// A request for legal research.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResearchRequest {
    /// The legal question presented.
    pub question: String,
    /// The fact pattern.
    pub facts: String,
    /// The forum in which the matter is being litigated, if known.
    pub forum: Option<Forum>,
    /// Additional search keywords.
    pub keywords: Vec<String>,
    /// Maximum authorities to cite per issue.
    pub max_authorities_per_issue: usize,
    /// Maximum number of issues to analyse.
    pub max_issues: usize,
}

impl ResearchRequest {
    /// Creates a request from a question and facts.
    pub fn new(question: impl Into<String>, facts: impl Into<String>) -> Self {
        Self {
            question: question.into(),
            facts: facts.into(),
            forum: None,
            keywords: Vec::new(),
            max_authorities_per_issue: 3,
            max_issues: 5,
        }
    }

    /// Sets the forum.
    pub fn with_forum(mut self, forum: Forum) -> Self {
        self.forum = Some(forum);
        self
    }

    /// Adds a search keyword.
    pub fn with_keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keywords.push(keyword.into());
        self
    }

    /// Sets the maximum number of authorities cited per issue.
    pub fn with_max_authorities_per_issue(mut self, max: usize) -> Self {
        self.max_authorities_per_issue = max;
        self
    }

    /// Sets the maximum number of issues analysed.
    pub fn with_max_issues(mut self, max: usize) -> Self {
        self.max_issues = max;
        self
    }
}

/// A reference to an authority cited in support of an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorityReference {
    /// Authority identifier.
    pub id: String,
    /// Authority title.
    pub title: String,
    /// Canonical citation.
    pub citation: String,
    /// Binding status in the forum (if a forum was supplied).
    pub binding: Option<BindingStatus>,
    /// Overall authority strength in `[0, 1]`.
    pub strength: f64,
    /// Relevance score from the search.
    pub relevance: f64,
}

/// The IRAC analysis of a single issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueAnalysis {
    /// The spotted issue.
    pub issue: SpottedIssue,
    /// Authorities cited for this issue.
    pub authorities: Vec<AuthorityReference>,
    /// Statement of the issue.
    pub issue_statement: String,
    /// The governing rule.
    pub rule: String,
    /// Application of the rule to the facts.
    pub application: String,
    /// The conclusion.
    pub conclusion: String,
}

/// A generated research memorandum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResearchMemo {
    /// Memo heading.
    pub heading: String,
    /// Restatement of the question presented.
    pub question_presented: String,
    /// A short overall answer.
    pub brief_answer: String,
    /// Per-issue IRAC analyses.
    pub issues: Vec<IssueAnalysis>,
    /// De-duplicated list of all citations referenced.
    pub authorities_cited: Vec<String>,
    /// Practical recommendations.
    pub recommendations: Vec<String>,
}

impl ResearchMemo {
    /// Renders the memo as a Markdown document.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.heading));
        out.push_str("## Question Presented\n\n");
        out.push_str(&format!("{}\n\n", self.question_presented));
        out.push_str("## Brief Answer\n\n");
        out.push_str(&format!("{}\n\n", self.brief_answer));

        out.push_str("## Discussion\n\n");
        for (idx, analysis) in self.issues.iter().enumerate() {
            out.push_str(&format!(
                "### {}. {} ({})\n\n",
                idx + 1,
                analysis.issue.issue,
                analysis.issue.area.label()
            ));
            out.push_str(&format!("**Issue.** {}\n\n", analysis.issue_statement));
            out.push_str(&format!("**Rule.** {}\n\n", analysis.rule));
            out.push_str(&format!("**Application.** {}\n\n", analysis.application));
            out.push_str(&format!("**Conclusion.** {}\n\n", analysis.conclusion));
        }

        if !self.authorities_cited.is_empty() {
            out.push_str("## Authorities Cited\n\n");
            for citation in &self.authorities_cited {
                out.push_str(&format!("- {citation}\n"));
            }
            out.push('\n');
        }

        if !self.recommendations.is_empty() {
            out.push_str("## Recommendations\n\n");
            for rec in &self.recommendations {
                out.push_str(&format!("- {rec}\n"));
            }
            out.push('\n');
        }

        out
    }
}

/// Generates IRAC research memos.
#[derive(Debug, Clone)]
pub struct MemoGenerator {
    spotter: IssueSpotter,
    ranker: AuthorityRanker,
    precedent: PrecedentAnalyzer,
}

impl Default for MemoGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoGenerator {
    /// Creates a generator with default sub-components.
    pub fn new() -> Self {
        Self {
            spotter: IssueSpotter::new(),
            ranker: AuthorityRanker::new(),
            precedent: PrecedentAnalyzer::new(),
        }
    }

    /// Creates a generator with custom sub-components.
    pub fn with_components(
        spotter: IssueSpotter,
        ranker: AuthorityRanker,
        precedent: PrecedentAnalyzer,
    ) -> Self {
        Self {
            spotter,
            ranker,
            precedent,
        }
    }

    /// Generates a research memo for the given request over the corpus.
    pub fn generate(&self, request: &ResearchRequest, corpus: &ResearchCorpus) -> ResearchMemo {
        let combined = format!("{} {}", request.question, request.facts);
        let mut spotted = self.spotter.spot(&combined);
        spotted.truncate(request.max_issues);

        let mut issue_analyses = Vec::new();
        let mut authorities_cited: Vec<String> = Vec::new();
        let mut recommendations: Vec<String> = Vec::new();

        for issue in spotted {
            let references = self.gather_authorities(&issue, request, corpus);

            for reference in &references {
                if !authorities_cited.contains(&reference.citation) {
                    authorities_cited.push(reference.citation.clone());
                }
            }

            if references.is_empty() {
                recommendations.push(format!(
                    "No on-point authority found in the corpus for '{}'; broaden the search \
                     or supplement the corpus.",
                    issue.issue
                ));
            }
            if !issue.missing_elements.is_empty() {
                recommendations.push(format!(
                    "Develop additional facts for '{}' regarding: {}.",
                    issue.issue,
                    issue.missing_elements.join(", ")
                ));
            }
            for reference in &references {
                if matches!(reference.binding, Some(BindingStatus::NoLongerGoodLaw)) {
                    recommendations.push(format!(
                        "Caution: {} appears to no longer be good law; verify before relying on it.",
                        reference.citation
                    ));
                }
            }

            let issue_statement = self.issue_statement(&issue, request);
            let rule = self.rule(&issue, &references);
            let application = self.application(&issue, &references);
            let conclusion = self.conclusion(&issue, &references);

            issue_analyses.push(IssueAnalysis {
                issue,
                authorities: references,
                issue_statement,
                rule,
                application,
                conclusion,
            });
        }

        let brief_answer = self.brief_answer(&issue_analyses);
        let heading = format!(
            "Research Memorandum: {}",
            truncate_words(&request.question, 12)
        );

        ResearchMemo {
            heading,
            question_presented: request.question.clone(),
            brief_answer,
            issues: issue_analyses,
            authorities_cited,
            recommendations,
        }
    }

    // --- internals ----------------------------------------------------------

    fn gather_authorities(
        &self,
        issue: &SpottedIssue,
        request: &ResearchRequest,
        corpus: &ResearchCorpus,
    ) -> Vec<AuthorityReference> {
        let mut query = format!("{} {}", issue.issue, issue.matched_elements.join(" "));
        if !request.keywords.is_empty() {
            query.push(' ');
            query.push_str(&request.keywords.join(" "));
        }

        let options = SearchOptions::new(request.max_authorities_per_issue);
        let mut hits = corpus.search_with(&query, &options);
        if hits.is_empty() {
            // Fall back to searching with the raw facts.
            hits = corpus.search(&request.facts, request.max_authorities_per_issue);
        }

        hits.into_iter()
            .filter_map(|hit| {
                let authority = corpus.get(&hit.id)?;
                let strength = self.ranker.score(authority).total;
                let binding = request.forum.as_ref().map(|forum| {
                    self.precedent
                        .analyze(authority, forum, &request.facts)
                        .binding
                });
                Some(AuthorityReference {
                    id: authority.id.clone(),
                    title: authority.title.clone(),
                    citation: authority.citation.clone(),
                    binding,
                    strength,
                    relevance: hit.score,
                })
            })
            .collect()
    }

    fn issue_statement(&self, issue: &SpottedIssue, request: &ResearchRequest) -> String {
        let forum = request
            .forum
            .as_ref()
            .map(|f| format!(" under {}", f.jurisdiction.description()))
            .unwrap_or_default();
        format!(
            "Whether the facts establish {}{}, given the required elements: {}.",
            issue.issue,
            forum,
            element_list(issue)
        )
    }

    fn rule(&self, issue: &SpottedIssue, references: &[AuthorityReference]) -> String {
        let mut rule = format!(
            "To establish {}, the following elements must be satisfied: {}.",
            issue.issue,
            element_list(issue)
        );
        if let Some(primary) = references.first() {
            rule.push_str(&format!(
                " The governing authority is {} ({}){}.",
                primary.title,
                primary.citation,
                primary
                    .binding
                    .map(|b| format!(", which is {} in this forum", b.label()))
                    .unwrap_or_default()
            ));
        }
        rule
    }

    fn application(&self, issue: &SpottedIssue, references: &[AuthorityReference]) -> String {
        let satisfied = if issue.matched_elements.is_empty() {
            "none of the required elements appear to be supported".to_string()
        } else {
            format!(
                "the facts support the following elements: {}",
                issue.matched_elements.join(", ")
            )
        };
        let missing = if issue.missing_elements.is_empty() {
            " All required elements are addressed by the facts.".to_string()
        } else {
            format!(
                " The following elements are not yet supported and require further \
                 factual development: {}.",
                issue.missing_elements.join(", ")
            )
        };
        let support = if references.is_empty() {
            String::new()
        } else {
            let binding_cites: Vec<String> = references
                .iter()
                .filter(|r| matches!(r.binding, Some(BindingStatus::Binding)))
                .map(|r| r.citation.clone())
                .collect();
            if binding_cites.is_empty() {
                format!(
                    " Persuasive support is available from {}.",
                    references
                        .iter()
                        .map(|r| r.citation.clone())
                        .collect::<Vec<_>>()
                        .join("; ")
                )
            } else {
                format!(" Binding support is found in {}.", binding_cites.join("; "))
            }
        };
        format!("Applying the rule to the facts, {satisfied}.{missing}{support}")
    }

    fn conclusion(&self, issue: &SpottedIssue, references: &[AuthorityReference]) -> String {
        let has_binding = references
            .iter()
            .any(|r| matches!(r.binding, Some(BindingStatus::Binding)));
        if issue.confidence >= 0.75 && (has_binding || references.is_empty()) {
            format!(
                "The elements of {} are well supported by the facts; a court would likely \
                 find in favour of the claim.",
                issue.issue
            )
        } else if issue.confidence >= 0.5 {
            format!(
                "There is a reasonable basis for {}, although some elements require further \
                 development before the outcome can be predicted with confidence.",
                issue.issue
            )
        } else {
            format!(
                "On the current facts, {} is unlikely to succeed because key elements ({}) \
                 are unaddressed.",
                issue.issue,
                issue.missing_elements.join(", ")
            )
        }
    }

    fn brief_answer(&self, analyses: &[IssueAnalysis]) -> String {
        if analyses.is_empty() {
            return "No recognised legal issues were identified in the stated facts.".to_string();
        }
        let strong: Vec<&str> = analyses
            .iter()
            .filter(|a| a.issue.confidence >= 0.75)
            .map(|a| a.issue.issue.as_str())
            .collect();
        let weak: Vec<&str> = analyses
            .iter()
            .filter(|a| a.issue.confidence < 0.5)
            .map(|a| a.issue.issue.as_str())
            .collect();

        let mut answer = format!("The facts raise {} potential issue(s).", analyses.len());
        if !strong.is_empty() {
            answer.push_str(&format!(
                " The strongest claim(s) are: {}.",
                strong.join(", ")
            ));
        }
        if !weak.is_empty() {
            answer.push_str(&format!(
                " The following are weak on the current record: {}.",
                weak.join(", ")
            ));
        }
        answer
    }
}

fn element_list(issue: &SpottedIssue) -> String {
    let mut all: Vec<String> = issue.matched_elements.clone();
    all.extend(issue.missing_elements.iter().cloned());
    if all.is_empty() {
        "(no elements defined)".to_string()
    } else {
        all.join(", ")
    }
}

fn truncate_words(text: &str, max_words: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() <= max_words {
        text.trim().to_string()
    } else {
        format!("{}...", words[..max_words].join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthorityType, CourtLevel, Jurisdiction, LegalAuthority};

    fn corpus() -> ResearchCorpus {
        let mut corpus = ResearchCorpus::new();
        corpus
            .add_many(vec![
                LegalAuthority::new(
                    "neg",
                    "Palsgraf v. Long Island Railroad",
                    "248 N.Y. 339 (1928)",
                    "Negligence requires a duty of care, breach of that duty, causation, and \
                     damages to a foreseeable plaintiff.",
                    AuthorityType::Case,
                    Jurisdiction::UsFederal,
                )
                .with_court_level(CourtLevel::Supreme)
                .with_year(1928)
                .with_citation_count(5000),
                LegalAuthority::new(
                    "con",
                    "Lucy v. Zehmer",
                    "84 S.E.2d 516 (1954)",
                    "A binding contract requires offer, acceptance and consideration; breach \
                     of the agreement gives rise to damages.",
                    AuthorityType::Case,
                    Jurisdiction::UsFederal,
                )
                .with_court_level(CourtLevel::Appellate)
                .with_year(1954)
                .with_citation_count(800),
            ])
            .expect("corpus builds");
        corpus
    }

    fn negligence_request() -> ResearchRequest {
        ResearchRequest::new(
            "Is the defendant liable in negligence?",
            "The defendant owed a duty of care to the plaintiff, breached that duty, and \
             the breach caused damages and injury.",
        )
        .with_forum(Forum::new(
            Jurisdiction::UsState("Texas".into()),
            CourtLevel::Trial,
        ))
    }

    #[test]
    fn test_generate_memo_structure() {
        let generator = MemoGenerator::new();
        let memo = generator.generate(&negligence_request(), &corpus());
        assert!(!memo.issues.is_empty());
        assert!(memo.issues.iter().any(|i| i.issue.issue == "Negligence"));
        assert!(!memo.brief_answer.is_empty());
        assert!(!memo.heading.is_empty());
    }

    #[test]
    fn test_memo_cites_authorities() {
        let generator = MemoGenerator::new();
        let memo = generator.generate(&negligence_request(), &corpus());
        assert!(
            memo.authorities_cited
                .iter()
                .any(|c| c.contains("248 N.Y. 339"))
        );
        let neg = memo
            .issues
            .iter()
            .find(|i| i.issue.issue == "Negligence")
            .expect("negligence issue present");
        assert!(!neg.authorities.is_empty());
        // U.S. Supreme Court authority binds the Texas state forum.
        assert!(
            neg.authorities
                .iter()
                .any(|a| a.binding == Some(BindingStatus::Binding))
        );
    }

    #[test]
    fn test_irac_sections_present() {
        let generator = MemoGenerator::new();
        let memo = generator.generate(&negligence_request(), &corpus());
        let neg = memo
            .issues
            .iter()
            .find(|i| i.issue.issue == "Negligence")
            .expect("negligence issue present");
        assert!(neg.issue_statement.contains("Negligence"));
        assert!(neg.rule.contains("elements"));
        assert!(neg.application.to_lowercase().contains("appl") || !neg.application.is_empty());
        assert!(neg.conclusion.contains("Negligence"));
    }

    #[test]
    fn test_to_markdown_renders_sections() {
        let generator = MemoGenerator::new();
        let memo = generator.generate(&negligence_request(), &corpus());
        let md = memo.to_markdown();
        assert!(md.contains("# Research Memorandum"));
        assert!(md.contains("## Question Presented"));
        assert!(md.contains("## Brief Answer"));
        assert!(md.contains("## Discussion"));
        assert!(md.contains("**Rule.**"));
    }

    #[test]
    fn test_recommendations_for_missing_elements() {
        let generator = MemoGenerator::new();
        // Facts only mention a duty - other negligence elements missing.
        let request =
            ResearchRequest::new("Negligence?", "The defendant arguably owed a duty of care.");
        let memo = generator.generate(&request, &corpus());
        assert!(
            memo.recommendations
                .iter()
                .any(|r| r.contains("Develop additional facts"))
        );
    }

    #[test]
    fn test_empty_facts_no_issues() {
        let generator = MemoGenerator::new();
        let request = ResearchRequest::new("Anything?", "The weather was pleasant that day.");
        let memo = generator.generate(&request, &corpus());
        assert!(memo.brief_answer.contains("No recognised legal issues"));
    }
}
