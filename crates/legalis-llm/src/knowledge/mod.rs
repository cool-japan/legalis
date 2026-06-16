//! Legal Knowledge Management
//!
//! A self-contained, pure-Rust knowledge-management toolkit that operates over
//! caller-supplied data with no live LLM dependency. It provides the four
//! actionable v0.5.9 capabilities as deterministic engines and data structures:
//!
//! * [`precedent_library`] - a precedent library: store, index (full-text +
//!   citation) and retrieve precedents, with parsed citation handling and
//!   topic/jurisdiction filters.
//! * [`templating`] - versioned legal document/clause templates with a complete
//!   revision history and a line-level diff between any two revisions.
//! * [`search_index`] - an in-memory inverted-index ranked search over arbitrary
//!   documents (BM25 / TF-IDF), independent of the precedent domain.
//! * [`graph`] - a typed legal concept graph (nodes are concepts, edges are
//!   typed relations) with traversal and reasoning queries (ancestors,
//!   descendants, shortest path, neighbourhood, transitive closure).
//!
//! Knowledge-graph *visualisation*, firm-knowledge-base integration,
//! collaborative annotation and expertise location are intentionally **not**
//! implemented here: the first needs a renderer and the rest require an external
//! multi-user firm system. See `TODO.md` for the deferral rationale.

mod graph;
mod precedent_library;
mod search_index;
mod templating;

pub use graph::*;
pub use precedent_library::*;
pub use search_index::*;
pub use templating::*;

// ============================================================================
// Shared text utilities (tokeniser + conservative stemmer)
//
// A self-contained copy so the knowledge suite does not depend on the private
// helpers in the `research` module; the two can evolve independently.
// ============================================================================

/// Returns whether a lowercase token is a stopword.
///
/// As in the research tokeniser, legally-loaded modal verbs (`shall`, `may`,
/// `must`, `will`) are deliberately retained.
pub(crate) fn is_stopword(token: &str) -> bool {
    const STOPWORDS: &[&str] = &[
        "a", "an", "and", "are", "as", "at", "be", "been", "being", "but", "by", "for", "from",
        "had", "has", "have", "he", "her", "his", "i", "if", "in", "into", "is", "it", "its", "of",
        "on", "or", "our", "out", "she", "so", "than", "that", "the", "their", "them", "then",
        "there", "these", "they", "this", "those", "to", "up", "was", "we", "were", "what", "when",
        "where", "which", "who", "whom", "whose", "why", "with", "would", "you", "your",
    ];
    STOPWORDS.contains(&token)
}

/// Tokenises text into normalised, stemmed terms.
pub(crate) fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        if raw.is_empty() {
            continue;
        }
        let lower = raw.to_lowercase();
        if lower.chars().count() < 2 {
            continue;
        }
        if is_stopword(&lower) {
            continue;
        }
        let stemmed = stem(&lower);
        if !stemmed.is_empty() {
            tokens.push(stemmed);
        }
    }
    tokens
}

/// A conservative, deterministic inflectional stemmer.
pub(crate) fn stem(token: &str) -> String {
    let len = token.chars().count();
    if len <= 3 {
        return token.to_string();
    }
    if len > 4 && (token.ends_with("ies") || token.ends_with("ied")) {
        let base = &token[..token.len() - 3];
        return format!("{base}y");
    }
    if token.ends_with("sses") {
        return token[..token.len() - 2].to_string();
    }
    if len > 4 && token.ends_with("es") {
        let base = &token[..token.len() - 2];
        if base.ends_with('s')
            || base.ends_with('x')
            || base.ends_with('z')
            || base.ends_with("ch")
            || base.ends_with("sh")
        {
            return base.to_string();
        }
    }
    if len > 3 && token.ends_with('s') && !token.ends_with("ss") {
        return token[..token.len() - 1].to_string();
    }
    if len > 5 && token.ends_with("ing") {
        return token[..token.len() - 3].to_string();
    }
    if len > 4 && token.ends_with("ed") {
        return token[..token.len() - 2].to_string();
    }
    token.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_and_stem() {
        let tokens = tokenize("The defendant breached the contracts.");
        assert!(!tokens.iter().any(|t| t == "the"));
        assert!(tokens.contains(&"breach".to_string()));
        assert!(tokens.contains(&"contract".to_string()));
        assert_eq!(stem("duties"), "duty");
        assert_eq!(stem("class"), "class");
        assert_eq!(stem("law"), "law");
    }

    #[test]
    fn test_modal_verbs_preserved() {
        let tokens = tokenize("the party shall and may but must not");
        assert!(tokens.contains(&"shall".to_string()));
        assert!(tokens.contains(&"may".to_string()));
        assert!(tokens.contains(&"must".to_string()));
    }
}
