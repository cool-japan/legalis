//! Syntax backward-compatibility layers (roadmap v0.3.4).
//!
//! A [`CompatibilityLayer`] is a version-aware normaliser: it rewrites older
//! keyword spellings into their current equivalents *before* the core parser
//! runs, emitting [`DslWarning::DeprecatedSyntax`] for forms that are deprecated
//! at the target version and a hard error for forms that were *removed* at or
//! before it. Rewriting is whole-word and quote/comment-aware, so deprecated
//! spellings inside string literals or comments are never touched.
//!
//! The built-in rules mirror the lexer's own deprecation handling
//! (`EXCEPT`→`EXCEPTION`, `AMENDS`→`AMENDMENT`, `REPLACES`→`SUPERSEDES`).

use crate::{DslError, DslResult, DslWarning, SourceLocation};

/// A `major.minor.patch` syntax version, ordered for comparisons.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct SyntaxVersion {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

impl SyntaxVersion {
    /// Creates a version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parses a `major.minor[.patch]` string.
    pub fn parse(text: &str) -> Option<Self> {
        let mut parts = text.split('.');
        let major = parts.next()?.trim().parse().ok()?;
        let minor = parts.next()?.trim().parse().ok()?;
        let patch = match parts.next() {
            Some(p) => p.trim().parse().ok()?,
            None => 0,
        };
        if parts.next().is_some() {
            return None;
        }
        Some(Self::new(major, minor, patch))
    }
}

impl std::fmt::Display for SyntaxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A rule rewriting a deprecated keyword to its replacement.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeprecationRule {
    /// The old keyword spelling.
    pub old: String,
    /// The replacement spelling.
    pub new: String,
    /// The version at which `old` became deprecated.
    pub deprecated_since: SyntaxVersion,
    /// The version at which `old` was removed (no longer accepted), if any.
    pub removed_in: Option<SyntaxVersion>,
    /// Human-readable guidance.
    pub message: String,
}

impl DeprecationRule {
    /// Creates a deprecation rule.
    pub fn new(
        old: impl Into<String>,
        new: impl Into<String>,
        deprecated_since: SyntaxVersion,
    ) -> Self {
        let old = old.into();
        let new = new.into();
        let message = format!("use '{new}' instead of '{old}'");
        Self {
            old,
            new,
            deprecated_since,
            removed_in: None,
            message,
        }
    }

    /// Marks the version at which the old form was removed.
    pub fn removed_in(mut self, version: SyntaxVersion) -> Self {
        self.removed_in = Some(version);
        self
    }

    /// Overrides the guidance message.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

/// A version-aware backward-compatibility normaliser.
#[derive(Debug, Clone)]
pub struct CompatibilityLayer {
    rules: Vec<DeprecationRule>,
    target: SyntaxVersion,
}

impl CompatibilityLayer {
    /// Creates a layer targeting `target` with no rules.
    pub fn new(target: SyntaxVersion) -> Self {
        Self {
            rules: Vec::new(),
            target,
        }
    }

    /// Creates a layer targeting `target` pre-loaded with the built-in rules
    /// (`EXCEPT`/`AMENDS`/`REPLACES`).
    pub fn with_builtin_rules(target: SyntaxVersion) -> Self {
        let v010 = SyntaxVersion::new(0, 1, 0);
        let mut layer = Self::new(target);
        layer.add_rule(DeprecationRule::new("EXCEPT", "EXCEPTION", v010));
        layer.add_rule(DeprecationRule::new("AMENDS", "AMENDMENT", v010));
        layer.add_rule(DeprecationRule::new("REPLACES", "SUPERSEDES", v010));
        layer
    }

    /// Adds a rule.
    pub fn add_rule(&mut self, rule: DeprecationRule) {
        self.rules.push(rule);
    }

    /// The target version.
    pub fn target(&self) -> SyntaxVersion {
        self.target
    }

    /// Returns the configured rules.
    pub fn rules(&self) -> &[DeprecationRule] {
        &self.rules
    }

    /// Finds the rule (if any) that applies to `word` at the target version.
    /// A rule applies only once its `deprecated_since` is at or before the target.
    fn applicable_rule(&self, word: &str) -> Option<&DeprecationRule> {
        self.rules.iter().find(|rule| {
            rule.old.eq_ignore_ascii_case(word) && self.target >= rule.deprecated_since
        })
    }

    /// Normalizes `input`, rewriting deprecated keywords to their replacements.
    ///
    /// Returns the rewritten source plus a deprecation warning per rewrite. A
    /// keyword whose rule was *removed* at or before the target version is a hard
    /// error. String literals and comments are copied verbatim.
    pub fn normalize(&self, input: &str) -> DslResult<(String, Vec<DslWarning>)> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = String::with_capacity(input.len());
        let mut warnings = Vec::new();
        let mut i = 0usize;
        let mut line = 1usize;
        let mut col = 1usize;

        while i < chars.len() {
            let c = chars[i];

            // Line comment: copy verbatim to end of line.
            if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                while i < chars.len() && chars[i] != '\n' {
                    out.push(chars[i]);
                    col += 1;
                    i += 1;
                }
                continue;
            }

            // Block comment: copy verbatim through the closing `*/`.
            if c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                out.push('/');
                out.push('*');
                i += 2;
                col += 2;
                while i < chars.len() {
                    if chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                        out.push('*');
                        out.push('/');
                        i += 2;
                        col += 2;
                        break;
                    }
                    if chars[i] == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                    out.push(chars[i]);
                    i += 1;
                }
                continue;
            }

            // String literal: copy verbatim through the closing quote.
            if c == '"' {
                out.push('"');
                i += 1;
                col += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                    out.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    out.push('"');
                    i += 1;
                    col += 1;
                }
                continue;
            }

            // Identifier word: candidate for keyword rewriting.
            if c.is_ascii_alphabetic() || c == '_' {
                let start = i;
                let start_line = line;
                let start_col = col;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
                {
                    i += 1;
                    col += 1;
                }
                let word: String = chars[start..i].iter().collect();
                if let Some(rule) = self.applicable_rule(&word) {
                    if let Some(removed) = rule.removed_in
                        && self.target >= removed
                    {
                        return Err(DslError::syntax_error(
                            SourceLocation::new(start_line, start_col, start),
                            format!(
                                "syntax '{}' was removed in version {} ({})",
                                word, removed, rule.message
                            ),
                            rule.new.clone(),
                            word.clone(),
                            Some(rule.message.clone()),
                        ));
                    }
                    out.push_str(&rule.new);
                    warnings.push(DslWarning::DeprecatedSyntax {
                        location: SourceLocation::new(start_line, start_col, start),
                        old_syntax: word,
                        new_syntax: rule.new.clone(),
                        message: rule.message.clone(),
                    });
                } else {
                    out.push_str(&word);
                }
                continue;
            }

            // Any other character: copy and advance, tracking position.
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            out.push(c);
            i += 1;
        }

        Ok((out, warnings))
    }
}
