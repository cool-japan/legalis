//! Legal citation parsing, normalisation and validation.
//!
//! [`CitationValidator`] recognises the most common United States citation
//! formats - case reporters, the U.S. Code, the Code of Federal Regulations,
//! constitutional provisions, public laws and the Statutes at Large - parses
//! them into structured [`CitationComponents`], normalises their surface form
//! and flags malformed citations. It can additionally cross-check a citation
//! against a [`ResearchCorpus`] to detect dangling references.
//!
//! The recogniser is built from compiled regular expressions. Because regex
//! compilation is fallible, [`CitationValidator::new`] returns a [`Result`];
//! the patterns are static so a failure is not expected in practice, but the
//! error is propagated rather than panicked on.

use super::ResearchCorpus;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Earliest plausible four-digit year for a citation.
const MIN_PLAUSIBLE_YEAR: i32 = 1600;
/// Latest plausible four-digit year for a citation.
const MAX_PLAUSIBLE_YEAR: i32 = 2100;

/// Known case reporter abbreviations recognised by the parser.
///
/// Ordered loosely from federal to regional to state reporters; the parser
/// sorts them by descending length internally so that longer reporters (e.g.
/// `F. Supp. 2d`) win over their prefixes (`F.`).
const KNOWN_REPORTERS: &[&str] = &[
    // Federal
    "U.S.",
    "S. Ct.",
    "L. Ed. 2d",
    "L. Ed.",
    "F.4th",
    "F.3d",
    "F.2d",
    "F. Supp. 3d",
    "F. Supp. 2d",
    "F. Supp.",
    "F.R.D.",
    "Fed. Cl.",
    "B.R.",
    "F.",
    // Regional reporters
    "N.E.3d",
    "N.E.2d",
    "N.E.",
    "N.W.3d",
    "N.W.2d",
    "N.W.",
    "S.E.2d",
    "S.E.",
    "S.W.3d",
    "S.W.2d",
    "S.W.",
    "So.3d",
    "So.2d",
    "So.",
    "P.3d",
    "P.2d",
    "P.",
    "A.3d",
    "A.2d",
    "A.",
    // State reporters
    "Cal. App. 5th",
    "Cal. App. 4th",
    "Cal. App. 3d",
    "Cal. App. 2d",
    "Cal. App.",
    "Cal. 5th",
    "Cal. 4th",
    "Cal. 3d",
    "Cal. 2d",
    "Cal.",
    "N.Y.S.3d",
    "N.Y.S.2d",
    "N.Y.3d",
    "N.Y.2d",
    "N.Y.",
    "Mass.",
    "Ill.",
    "Tex.",
    "Pa.",
    "Ohio St. 3d",
    "Ohio St. 2d",
];

/// The kind of legal citation recognised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationKind {
    /// A case reporter citation (e.g. `347 U.S. 483`).
    CaseReporter,
    /// A U.S. Code citation (e.g. `42 U.S.C. § 1983`).
    Statute,
    /// A Code of Federal Regulations citation (e.g. `29 C.F.R. § 1604.11`).
    Regulation,
    /// A constitutional citation (e.g. `U.S. Const. amend. XIV`).
    Constitution,
    /// A public law citation (e.g. `Pub. L. No. 116-136`).
    PublicLaw,
    /// A Statutes at Large citation (e.g. `134 Stat. 281`).
    StatutesAtLarge,
    /// An unrecognised citation.
    Unknown,
}

/// Structured components extracted from a citation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CitationComponents {
    /// Case name (for full case citations).
    pub case_name: Option<String>,
    /// Reporter volume / Statutes at Large volume.
    pub volume: Option<u32>,
    /// Reporter abbreviation.
    pub reporter: Option<String>,
    /// First page / page in Statutes at Large.
    pub page: Option<u32>,
    /// Code title (for U.S.C. / C.F.R.).
    pub title: Option<u32>,
    /// Section identifier.
    pub section: Option<String>,
    /// Year of decision / enactment.
    pub year: Option<i32>,
    /// Issuing court (parsed from the parenthetical).
    pub court: Option<String>,
}

/// A parsed citation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedCitation {
    /// The original text as supplied.
    pub raw: String,
    /// The normalised surface form.
    pub normalized: String,
    /// Recognised kind.
    pub kind: CitationKind,
    /// Extracted components.
    pub components: CitationComponents,
}

/// Severity of a citation validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationSeverity {
    /// A defect that makes the citation invalid.
    Error,
    /// A non-fatal concern.
    Warning,
    /// Informational note.
    Info,
}

/// A single validation finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitationIssue {
    /// Severity of the issue.
    pub severity: CitationSeverity,
    /// Human-readable description.
    pub message: String,
}

impl CitationIssue {
    fn error(message: impl Into<String>) -> Self {
        Self {
            severity: CitationSeverity::Error,
            message: message.into(),
        }
    }

    fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: CitationSeverity::Warning,
            message: message.into(),
        }
    }
}

/// The result of validating a citation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitationValidation {
    /// The citation as supplied.
    pub raw: String,
    /// Normalised form.
    pub normalized: String,
    /// Recognised kind.
    pub kind: CitationKind,
    /// Whether the citation is valid (no error-level issues).
    pub is_valid: bool,
    /// All issues found.
    pub issues: Vec<CitationIssue>,
}

/// Parses, normalises and validates legal citations.
#[derive(Debug, Clone)]
pub struct CitationValidator {
    reporter: Regex,
    generic_reporter: Regex,
    statute: Regex,
    regulation: Regex,
    constitution: Regex,
    public_law: Regex,
    statutes_at_large: Regex,
    year4: Regex,
    ws: Regex,
    v_abbrev: Regex,
    section_double: Regex,
    section_single: Regex,
    case_name: Regex,
}

impl CitationValidator {
    /// Builds a validator, compiling all recognition patterns.
    pub fn new() -> Result<Self> {
        let mut reporters: Vec<String> = KNOWN_REPORTERS.iter().map(|r| regex::escape(r)).collect();
        reporters.sort_by_key(|r| std::cmp::Reverse(r.len()));
        let reporter_alt = reporters.join("|");

        let reporter = Regex::new(&format!(
            r"(?P<vol>\d+)\s+(?P<rep>(?:{reporter_alt}))\s+(?P<page>\d+)(?:\s*\(\s*(?P<paren>[^)]*?)\s*\))?"
        ))?;
        let generic_reporter = Regex::new(
            r"^\s*(?P<vol>\d+)\s+(?P<rep>[A-Z][A-Za-z.]*(?:\s+[A-Za-z0-9.]+){0,4}?)\s+(?P<page>\d+)\s*(?:\(\s*(?P<paren>[^)]*?)\s*\))?\s*$",
        )?;
        let statute = Regex::new(
            r"(?P<title>\d+)\s+U\.?\s*S\.?\s*C\.?(?:\s*[AS]\.?)?\s+§{1,2}\s*(?P<sec>\d+[A-Za-z0-9.\u{2010}-]*)",
        )?;
        let regulation =
            Regex::new(r"(?P<title>\d+)\s+C\.?\s*F\.?\s*R\.?\s+§{1,2}\s*(?P<sec>\d+(?:\.\d+)*)")?;
        let constitution = Regex::new(
            r"(?P<doc>U\.?\s*S\.?|[A-Z][a-z]+\.?)\s+Const\.?(?P<rest>(?:\s*,?\s*(?:art\.?|amend\.?|§{1,2}|cl\.?|pt\.?)\s*[IVXLCDM0-9]+)*)",
        )?;
        let public_law =
            Regex::new(r"Pub\.?\s*L\.?\s*(?:No\.?)?\s*(?P<cong>\d+)\s*[\u{2010}-]\s*(?P<num>\d+)")?;
        let statutes_at_large = Regex::new(r"(?P<vol>\d+)\s+Stat\.?\s+(?P<page>\d+)")?;
        let year4 = Regex::new(r"\d{4}")?;
        let ws = Regex::new(r"\s+")?;
        let v_abbrev = Regex::new(r"\s+[vV][sS]?\.?\s+")?;
        let section_double = Regex::new(r"§§\s*")?;
        let section_single = Regex::new(r"§\s*")?;
        let case_name = Regex::new(
            r"([A-Z][A-Za-z.&'\-]*(?:\s+(?:of|the|and|in|for|&|[A-Z][A-Za-z.&'\-]*))*\s+v\.?\s+[A-Z][A-Za-z.&'\-]*(?:\s+(?:of|the|and|in|for|&|[A-Z][A-Za-z.&'\-]*))*)\s*,\s*$",
        )?;

        Ok(Self {
            reporter,
            generic_reporter,
            statute,
            regulation,
            constitution,
            public_law,
            statutes_at_large,
            year4,
            ws,
            v_abbrev,
            section_double,
            section_single,
            case_name,
        })
    }

    /// Normalises the surface form of a citation.
    ///
    /// Collapses whitespace, standardises the `v.` separator, the section
    /// symbol spacing and reporter ordinals, and trims stray trailing
    /// separators.
    pub fn normalize(&self, raw: &str) -> String {
        let mut s = self.ws.replace_all(raw.trim(), " ").to_string();
        s = self.v_abbrev.replace_all(&s, " v. ").to_string();
        s = self.section_double.replace_all(&s, "§§ ").to_string();
        s = self.section_single.replace_all(&s, "§ ").to_string();
        s = self.ws.replace_all(&s, " ").to_string();
        s.trim().trim_end_matches([',', ';']).trim().to_string()
    }

    /// Parses a single citation string into structured form.
    pub fn parse(&self, raw: &str) -> ParsedCitation {
        let normalized = self.normalize(raw);

        if let Some(parsed) = self.try_specific(&normalized) {
            return ParsedCitation {
                raw: raw.to_string(),
                normalized,
                kind: parsed.0,
                components: parsed.1,
            };
        }

        if let Some(caps) = self.generic_reporter.captures(&normalized) {
            let mut components = CitationComponents::default();
            components.volume = caps.name("vol").and_then(|m| m.as_str().parse().ok());
            components.reporter = caps.name("rep").map(|m| m.as_str().trim().to_string());
            components.page = caps.name("page").and_then(|m| m.as_str().parse().ok());
            if let Some(paren) = caps.name("paren") {
                let (year, court) = self.parse_paren(paren.as_str());
                components.year = year;
                components.court = court;
            }
            return ParsedCitation {
                raw: raw.to_string(),
                normalized,
                kind: CitationKind::CaseReporter,
                components,
            };
        }

        ParsedCitation {
            raw: raw.to_string(),
            normalized,
            kind: CitationKind::Unknown,
            components: CitationComponents::default(),
        }
    }

    /// Validates a citation, returning any defects found.
    pub fn validate(&self, raw: &str) -> CitationValidation {
        let parsed = self.parse(raw);
        let mut issues = Vec::new();

        match parsed.kind {
            CitationKind::Unknown => {
                issues.push(CitationIssue::error("unrecognised citation format"));
            }
            CitationKind::CaseReporter => {
                let c = &parsed.components;
                if !self.is_known_reporter(c.reporter.as_deref()) {
                    issues.push(CitationIssue::warning(format!(
                        "unrecognised reporter abbreviation: {}",
                        c.reporter.as_deref().unwrap_or("<none>")
                    )));
                }
                if matches!(c.volume, Some(0)) {
                    issues.push(CitationIssue::error("reporter volume cannot be zero"));
                }
                if matches!(c.page, Some(0)) {
                    issues.push(CitationIssue::error("reporter page cannot be zero"));
                }
                if c.year.is_none() {
                    issues.push(CitationIssue::warning("missing year of decision"));
                }
                self.check_year(c.year, &mut issues);
            }
            CitationKind::Statute | CitationKind::Regulation => {
                if parsed.components.title.is_none() {
                    issues.push(CitationIssue::error("missing code title"));
                }
                if parsed.components.section.is_none() {
                    issues.push(CitationIssue::error("missing section reference"));
                }
            }
            CitationKind::PublicLaw | CitationKind::StatutesAtLarge => {
                self.check_year(parsed.components.year, &mut issues);
            }
            CitationKind::Constitution => {}
        }

        let is_valid = !issues.iter().any(|i| i.severity == CitationSeverity::Error);

        CitationValidation {
            raw: raw.to_string(),
            normalized: parsed.normalized,
            kind: parsed.kind,
            is_valid,
            issues,
        }
    }

    /// Validates a citation against a corpus, additionally flagging dangling
    /// references (citations whose normalised form is not present in the
    /// corpus).
    pub fn validate_against_corpus(
        &self,
        raw: &str,
        corpus: &ResearchCorpus,
    ) -> CitationValidation {
        let mut validation = self.validate(raw);
        if validation.kind == CitationKind::Unknown {
            return validation;
        }

        let found = corpus
            .authorities()
            .iter()
            .any(|a| self.normalize(&a.citation) == validation.normalized);
        if !found {
            validation.issues.push(CitationIssue::warning(
                "citation not found in corpus (possible dangling reference)",
            ));
        }
        validation
    }

    /// Extracts every recognised citation from a block of free text.
    pub fn extract_all(&self, text: &str) -> Vec<ParsedCitation> {
        let mut spans: Vec<(usize, usize, CitationKind, CitationComponents)> = Vec::new();

        for caps in self.statute.captures_iter(text) {
            if let Some(m) = caps.get(0) {
                let mut c = CitationComponents::default();
                c.title = caps.name("title").and_then(|x| x.as_str().parse().ok());
                c.section = caps.name("sec").map(|x| x.as_str().to_string());
                spans.push((m.start(), m.end(), CitationKind::Statute, c));
            }
        }
        for caps in self.regulation.captures_iter(text) {
            if let Some(m) = caps.get(0) {
                let mut c = CitationComponents::default();
                c.title = caps.name("title").and_then(|x| x.as_str().parse().ok());
                c.section = caps.name("sec").map(|x| x.as_str().to_string());
                spans.push((m.start(), m.end(), CitationKind::Regulation, c));
            }
        }
        for caps in self.constitution.captures_iter(text) {
            if let Some(m) = caps.get(0) {
                let mut c = CitationComponents::default();
                c.section = caps
                    .name("rest")
                    .map(|x| x.as_str().trim().trim_start_matches(',').trim().to_string())
                    .filter(|s| !s.is_empty());
                spans.push((m.start(), m.end(), CitationKind::Constitution, c));
            }
        }
        for caps in self.public_law.captures_iter(text) {
            if let Some(m) = caps.get(0) {
                let mut c = CitationComponents::default();
                c.title = caps.name("cong").and_then(|x| x.as_str().parse().ok());
                c.section = caps.name("num").map(|x| x.as_str().to_string());
                spans.push((m.start(), m.end(), CitationKind::PublicLaw, c));
            }
        }
        for caps in self.reporter.captures_iter(text) {
            if let Some(m) = caps.get(0) {
                let mut c = CitationComponents::default();
                c.volume = caps.name("vol").and_then(|x| x.as_str().parse().ok());
                c.reporter = caps.name("rep").map(|x| x.as_str().trim().to_string());
                c.page = caps.name("page").and_then(|x| x.as_str().parse().ok());
                if let Some(paren) = caps.name("paren") {
                    let (year, court) = self.parse_paren(paren.as_str());
                    c.year = year;
                    c.court = court;
                }
                c.case_name = self.lookback_case_name(text, m.start());
                spans.push((m.start(), m.end(), CitationKind::CaseReporter, c));
            }
        }
        for caps in self.statutes_at_large.captures_iter(text) {
            if let Some(m) = caps.get(0) {
                let mut c = CitationComponents::default();
                c.volume = caps.name("vol").and_then(|x| x.as_str().parse().ok());
                c.page = caps.name("page").and_then(|x| x.as_str().parse().ok());
                spans.push((m.start(), m.end(), CitationKind::StatutesAtLarge, c));
            }
        }

        // Resolve overlaps: sort by start, then by descending length, and keep
        // the first non-overlapping span at each position.
        spans.sort_by(|a, b| a.0.cmp(&b.0).then((b.1 - b.0).cmp(&(a.1 - a.0))));
        let mut result = Vec::new();
        let mut cursor = 0usize;
        for (start, end, kind, components) in spans {
            if start < cursor {
                continue;
            }
            cursor = end;
            let raw = text[start..end].to_string();
            result.push(ParsedCitation {
                normalized: self.normalize(&raw),
                raw,
                kind,
                components,
            });
        }
        result
    }

    // --- internals ----------------------------------------------------------

    fn try_specific(&self, text: &str) -> Option<(CitationKind, CitationComponents)> {
        if let Some(caps) = self.constitution.captures(text) {
            // The pattern requires the literal "Const", so any match is a
            // constitutional citation.
            let mut c = CitationComponents::default();
            c.section = caps
                .name("rest")
                .map(|x| x.as_str().trim().trim_start_matches(',').trim().to_string())
                .filter(|s| !s.is_empty());
            return Some((CitationKind::Constitution, c));
        }
        if let Some(caps) = self.public_law.captures(text) {
            let mut c = CitationComponents::default();
            c.title = caps.name("cong").and_then(|x| x.as_str().parse().ok());
            c.section = caps.name("num").map(|x| x.as_str().to_string());
            return Some((CitationKind::PublicLaw, c));
        }
        if let Some(caps) = self.statute.captures(text) {
            let mut c = CitationComponents::default();
            c.title = caps.name("title").and_then(|x| x.as_str().parse().ok());
            c.section = caps.name("sec").map(|x| x.as_str().to_string());
            return Some((CitationKind::Statute, c));
        }
        if let Some(caps) = self.regulation.captures(text) {
            let mut c = CitationComponents::default();
            c.title = caps.name("title").and_then(|x| x.as_str().parse().ok());
            c.section = caps.name("sec").map(|x| x.as_str().to_string());
            return Some((CitationKind::Regulation, c));
        }
        if let Some(caps) = self.reporter.captures(text) {
            let mut c = CitationComponents::default();
            c.volume = caps.name("vol").and_then(|x| x.as_str().parse().ok());
            c.reporter = caps.name("rep").map(|x| x.as_str().trim().to_string());
            c.page = caps.name("page").and_then(|x| x.as_str().parse().ok());
            if let Some(paren) = caps.name("paren") {
                let (year, court) = self.parse_paren(paren.as_str());
                c.year = year;
                c.court = court;
            }
            if let Some(m) = caps.get(0) {
                c.case_name = self.lookback_case_name(text, m.start());
            }
            return Some((CitationKind::CaseReporter, c));
        }
        if let Some(caps) = self.statutes_at_large.captures(text) {
            let mut c = CitationComponents::default();
            c.volume = caps.name("vol").and_then(|x| x.as_str().parse().ok());
            c.page = caps.name("page").and_then(|x| x.as_str().parse().ok());
            return Some((CitationKind::StatutesAtLarge, c));
        }
        None
    }

    fn parse_paren(&self, paren: &str) -> (Option<i32>, Option<String>) {
        if let Some(m) = self.year4.find(paren) {
            let year = m.as_str().parse::<i32>().ok();
            let court_part = paren[..m.start()].trim().trim_end_matches(',').trim();
            let court = if court_part.is_empty() {
                None
            } else {
                Some(court_part.to_string())
            };
            (year, court)
        } else {
            let court = paren.trim();
            (
                None,
                if court.is_empty() {
                    None
                } else {
                    Some(court.to_string())
                },
            )
        }
    }

    fn lookback_case_name(&self, text: &str, start: usize) -> Option<String> {
        let prefix = &text[..start];
        self.case_name
            .captures(prefix)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
    }

    fn is_known_reporter(&self, reporter: Option<&str>) -> bool {
        match reporter {
            Some(r) => KNOWN_REPORTERS.iter().any(|k| k.eq_ignore_ascii_case(r)),
            None => false,
        }
    }

    fn check_year(&self, year: Option<i32>, issues: &mut Vec<CitationIssue>) {
        if let Some(y) = year.filter(|y| !(MIN_PLAUSIBLE_YEAR..=MAX_PLAUSIBLE_YEAR).contains(y)) {
            issues.push(CitationIssue::warning(format!("implausible year: {y}")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthorityType, Jurisdiction, LegalAuthority};

    fn validator() -> CitationValidator {
        CitationValidator::new().expect("validator builds")
    }

    #[test]
    fn test_parse_case_reporter_with_year() {
        let v = validator();
        let parsed = v.parse("Brown v. Board of Education, 347 U.S. 483 (1954)");
        assert_eq!(parsed.kind, CitationKind::CaseReporter);
        assert_eq!(parsed.components.volume, Some(347));
        assert_eq!(parsed.components.reporter.as_deref(), Some("U.S."));
        assert_eq!(parsed.components.page, Some(483));
        assert_eq!(parsed.components.year, Some(1954));
        assert_eq!(
            parsed.components.case_name.as_deref(),
            Some("Brown v. Board of Education")
        );
    }

    #[test]
    fn test_parse_federal_supplement() {
        let v = validator();
        let parsed = v.parse("123 F. Supp. 2d 456");
        assert_eq!(parsed.kind, CitationKind::CaseReporter);
        assert_eq!(parsed.components.reporter.as_deref(), Some("F. Supp. 2d"));
        assert_eq!(parsed.components.page, Some(456));
    }

    #[test]
    fn test_parse_statute_and_regulation() {
        let v = validator();
        let statute = v.parse("42 U.S.C. § 1983");
        assert_eq!(statute.kind, CitationKind::Statute);
        assert_eq!(statute.components.title, Some(42));
        assert_eq!(statute.components.section.as_deref(), Some("1983"));

        let reg = v.parse("29 C.F.R. § 1604.11");
        assert_eq!(reg.kind, CitationKind::Regulation);
        assert_eq!(reg.components.title, Some(29));
        assert_eq!(reg.components.section.as_deref(), Some("1604.11"));
    }

    #[test]
    fn test_parse_constitution_and_public_law() {
        let v = validator();
        let con = v.parse("U.S. Const. amend. XIV");
        assert_eq!(con.kind, CitationKind::Constitution);

        let pl = v.parse("Pub. L. No. 116-136");
        assert_eq!(pl.kind, CitationKind::PublicLaw);
        assert_eq!(pl.components.title, Some(116));
        assert_eq!(pl.components.section.as_deref(), Some("136"));
    }

    #[test]
    fn test_normalize() {
        let v = validator();
        assert_eq!(v.normalize("Brown   v Board , "), "Brown v. Board");
        assert_eq!(v.normalize("42 U.S.C. §1983"), "42 U.S.C. § 1983");
        assert_eq!(v.normalize("410   F.3d   1066"), "410 F.3d 1066");
    }

    #[test]
    fn test_validate_malformed() {
        let v = validator();
        let bad = v.validate("not a citation at all");
        assert!(!bad.is_valid);
        assert_eq!(bad.kind, CitationKind::Unknown);

        let zero_page = v.validate("347 U.S. 0 (1954)");
        assert!(!zero_page.is_valid);
        assert!(
            zero_page
                .issues
                .iter()
                .any(|i| i.message.contains("page cannot be zero"))
        );
    }

    #[test]
    fn test_validate_missing_year_warns_but_valid() {
        let v = validator();
        let result = v.validate("347 U.S. 483");
        assert!(result.is_valid);
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.severity == CitationSeverity::Warning
                    && i.message.contains("missing year"))
        );
    }

    #[test]
    fn test_unknown_reporter_warning() {
        let v = validator();
        let result = v.validate("100 Zz. 200 (1990)");
        assert_eq!(result.kind, CitationKind::CaseReporter);
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.message.contains("unrecognised reporter"))
        );
    }

    #[test]
    fn test_extract_all_mixed_text() {
        let v = validator();
        let text = "As held in Brown v. Board of Education, 347 U.S. 483 (1954), and under \
                    42 U.S.C. § 1983, the rule from 29 C.F.R. § 1604.11 applies.";
        let cites = v.extract_all(text);
        let kinds: Vec<CitationKind> = cites.iter().map(|c| c.kind).collect();
        assert!(kinds.contains(&CitationKind::CaseReporter));
        assert!(kinds.contains(&CitationKind::Statute));
        assert!(kinds.contains(&CitationKind::Regulation));
        assert!(cites.len() >= 3);
    }

    #[test]
    fn test_dangling_citation_detection() {
        let v = validator();
        let mut corpus = ResearchCorpus::new();
        corpus
            .add(LegalAuthority::new(
                "a1",
                "Brown v. Board of Education",
                "347 U.S. 483 (1954)",
                "equal protection",
                AuthorityType::Case,
                Jurisdiction::UsFederal,
            ))
            .expect("add ok");

        let present = v.validate_against_corpus("347 U.S. 483 (1954)", &corpus);
        assert!(
            !present
                .issues
                .iter()
                .any(|i| i.message.contains("dangling"))
        );

        let dangling = v.validate_against_corpus("999 U.S. 111 (2001)", &corpus);
        assert!(
            dangling
                .issues
                .iter()
                .any(|i| i.message.contains("dangling"))
        );
    }
}
