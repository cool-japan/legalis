//! Tokenizer (lexer) for the Legalis DSL.
//!
//! This module provides standalone tokenization functions used by `LegalDslParser`.
//! Separated from parser_impl.rs to keep files under 2000 lines.

use crate::ast::{SpannedToken, Token};
use crate::{DslError, DslResult, DslWarning, SourceLocation};

/// Removes comments from input (both `//` line comments and `/* */` block comments).
pub(crate) fn strip_comments(input: &str) -> DslResult<String> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut position = 0;

    while let Some(ch) = chars.next() {
        position += 1;
        if ch == '/'
            && let Some(&next) = chars.peek()
        {
            if next == '/' {
                // Line comment: skip until newline
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                result.push('\n');
                continue;
            } else if next == '*' {
                // Block comment: skip until */
                chars.next();
                let comment_start = position;
                let mut found_end = false;
                while let Some(c) = chars.next() {
                    if c == '*'
                        && let Some(&next_c) = chars.peek()
                        && next_c == '/'
                    {
                        chars.next();
                        found_end = true;
                        break;
                    }
                }
                if !found_end {
                    return Err(DslError::UnclosedComment(Some(
                        SourceLocation::from_offset(comment_start, input),
                    )));
                }
                result.push(' ');
                continue;
            }
        }
        result.push(ch);
    }

    Ok(result)
}

/// Tokenizes input DSL text into a sequence of spanned tokens.
///
/// Returns both the token list and any deprecation warnings encountered.
pub(crate) fn tokenize_input(input: &str) -> DslResult<(Vec<SpannedToken>, Vec<DslWarning>)> {
    let stripped = strip_comments(input)?;
    let mut warnings = Vec::new();
    // Pre-allocate capacity: estimate ~10 tokens per 100 bytes
    let estimated_tokens = (stripped.len() / 10).max(16);
    let mut tokens = Vec::with_capacity(estimated_tokens);
    let mut chars = stripped.chars().peekable();
    let mut offset = 0;
    let mut line = 1;
    let mut column = 1;

    while let Some(&ch) = chars.peek() {
        let token_start = SourceLocation::new(line, column, offset);
        match ch {
            '\n' => {
                chars.next();
                offset += 1;
                line += 1;
                column = 1;
            }
            // Optimize: skip multiple whitespace characters at once
            ' ' | '\t' | '\r' => {
                chars.next();
                offset += 1;
                column += 1;
                // Fast-path: skip additional whitespace
                while let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        ' ' | '\t' | '\r' => {
                            chars.next();
                            offset += 1;
                            column += 1;
                        }
                        _ => break,
                    }
                }
            }
            '(' => {
                tokens.push(SpannedToken::new(Token::LParen, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            ')' => {
                tokens.push(SpannedToken::new(Token::RParen, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            '{' => {
                tokens.push(SpannedToken::new(Token::LBrace, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            '}' => {
                tokens.push(SpannedToken::new(Token::RBrace, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            ':' => {
                tokens.push(SpannedToken::new(Token::Colon, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            ',' => {
                tokens.push(SpannedToken::new(Token::Comma, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            '"' => {
                chars.next();
                offset += 1;
                column += 1;
                // Pre-allocate for typical string length
                let mut s = String::with_capacity(32);
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        offset += 1;
                        column += 1;
                        break;
                    }
                    if c == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                    s.push(c);
                    chars.next();
                    offset += 1;
                }
                tokens.push(SpannedToken::new(Token::StringLit(s), token_start));
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                // Pre-allocate for typical keyword/identifier length
                let mut word = String::with_capacity(16);
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '-' {
                        word.push(c);
                        chars.next();
                        offset += 1;
                        column += 1;
                    } else {
                        break;
                    }
                }
                let upper = word.to_uppercase();

                // Check for deprecated syntax and collect warnings
                match upper.as_str() {
                    "EXCEPT" => {
                        warnings.push(DslWarning::DeprecatedSyntax {
                            location: token_start,
                            old_syntax: "EXCEPT".to_string(),
                            new_syntax: "EXCEPTION".to_string(),
                            message: "Please use 'EXCEPTION' instead of 'EXCEPT'".to_string(),
                        });
                    }
                    "AMENDS" => {
                        warnings.push(DslWarning::DeprecatedSyntax {
                            location: token_start,
                            old_syntax: "AMENDS".to_string(),
                            new_syntax: "AMENDMENT".to_string(),
                            message: "Please use 'AMENDMENT' instead of 'AMENDS'".to_string(),
                        });
                    }
                    "REPLACES" => {
                        warnings.push(DslWarning::DeprecatedSyntax {
                            location: token_start,
                            old_syntax: "REPLACES".to_string(),
                            new_syntax: "SUPERSEDES".to_string(),
                            message: "Please use 'SUPERSEDES' instead of 'REPLACES'".to_string(),
                        });
                    }
                    _ => {}
                }

                let token = match upper.as_str() {
                    "STATUTE" => Token::Statute,
                    "WHEN" => Token::When,
                    "UNLESS" => Token::Unless,
                    "REQUIRES" => Token::Requires,
                    "THEN" => Token::Then,
                    "DISCRETION" => Token::Discretion,
                    "AGE" => Token::Age,
                    "INCOME" => Token::Income,
                    "GRANT" => Token::Grant,
                    "REVOKE" => Token::Revoke,
                    "OBLIGATION" => Token::Obligation,
                    "PROHIBITION" => Token::Prohibition,
                    "IMPORT" => Token::Import,
                    "AS" => Token::As,
                    "EXCEPTION" | "EXCEPT" => Token::Exception,
                    "AMENDMENT" | "AMENDS" => Token::Amendment,
                    "SUPERSEDES" | "REPLACES" => Token::Supersedes,
                    "DELEGATE" | "DELEGATES" => Token::Delegate,
                    "PRIORITY" => Token::Priority,
                    "SCOPE" => Token::Scope,
                    "CONSTRAINT" | "CONSTRAINTS" | "INVARIANT" => Token::Constraint,
                    // Contract / compliance / test clause keywords (v0.2.5 - v0.2.7).
                    "CONTRACT" => Token::Contract,
                    "PARTY" | "PARTIES" => Token::Party,
                    "RIGHT" | "RIGHTS" => Token::Right,
                    "PERFORMANCE" => Token::Performance,
                    "CLAUSE" => Token::Clause,
                    "COMPLIANCE" => Token::Compliance,
                    "PENALTY" | "PENALTIES" => Token::Penalty,
                    "REPORT" | "REPORTING" => Token::Report,
                    "INSPECT" | "INSPECTION" | "AUDIT" => Token::Inspect,
                    "DEADLINE" => Token::Deadline,
                    "TIMELINE" => Token::Timeline,
                    // Module system keywords (v0.1.4)
                    "NAMESPACE" => Token::Namespace,
                    "FROM" => Token::From,
                    "PUBLIC" => Token::Public,
                    "PRIVATE" => Token::Private,
                    "EXPORT" => Token::Export,
                    "AND" => Token::And,
                    "OR" => Token::Or,
                    "NOT" => Token::Not,
                    "HAS" => Token::Has,
                    "BETWEEN" => Token::Between,
                    "IN" => Token::In,
                    "LIKE" => Token::Like,
                    "MATCHES" | "MATCH" | "REGEX" => Token::Matches,
                    "IN_RANGE" | "INRANGE" => Token::InRange,
                    "NOT_IN_RANGE" | "NOTINRANGE" => Token::NotInRange,
                    "DEFAULT" => Token::Default,
                    "UNION" => Token::Union,
                    "INTERSECT" | "INTERSECTION" => Token::Intersect,
                    "DIFFERENCE" | "SETMINUS" => Token::Difference,
                    "EFFECTIVE_DATE" | "EFFECTIVE" => Token::EffectiveDate,
                    "EXPIRY_DATE" | "EXPIRY" | "EXPIRES" => Token::ExpiryDate,
                    "JURISDICTION" => Token::Jurisdiction,
                    "VERSION" => Token::Version,
                    "CURRENT_DATE" | "CURRENTDATE" | "NOW" | "TODAY" => Token::CurrentDate,
                    "DATE_FIELD" | "DATEFIELD" => Token::DateField,
                    _ => Token::Ident(word),
                };
                tokens.push(SpannedToken::new(token, token_start));
            }
            _ if ch.is_numeric() => {
                // Pre-allocate for typical number length
                let mut num = String::with_capacity(8);
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        num.push(c);
                        chars.next();
                        offset += 1;
                        column += 1;
                    } else {
                        break;
                    }
                }
                // A `.` followed by a digit makes this a float literal (e.g. `0.05`).
                // The original decimal text is preserved so precision/leading zeros
                // survive the lexer (unlike `Number . Number`).
                let is_float = matches!(chars.peek(), Some('.')) && {
                    let mut lookahead = chars.clone();
                    lookahead.next();
                    matches!(lookahead.peek(), Some(c) if c.is_numeric())
                };
                if is_float {
                    num.push('.');
                    chars.next();
                    offset += 1;
                    column += 1;
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            num.push(c);
                            chars.next();
                            offset += 1;
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(SpannedToken::new(
                        Token::Float(num.parse().unwrap_or(0.0)),
                        token_start,
                    ));
                } else {
                    tokens.push(SpannedToken::new(
                        Token::Number(num.parse().unwrap_or(0)),
                        token_start,
                    ));
                }
            }
            '-' => {
                tokens.push(SpannedToken::new(Token::Dash, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            '.' => {
                tokens.push(SpannedToken::new(Token::Dot, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            '>' | '<' | '=' | '!' => {
                let mut op = String::new();
                op.push(ch);
                chars.next();
                offset += 1;
                column += 1;
                if let Some(&next) = chars.peek()
                    && next == '='
                {
                    op.push(next);
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                tokens.push(SpannedToken::new(Token::Operator(op), token_start));
            }
            '*' => {
                tokens.push(SpannedToken::new(Token::Star, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            '@' => {
                tokens.push(SpannedToken::new(Token::At, token_start));
                chars.next();
                offset += 1;
                column += 1;
            }
            _ => {
                chars.next();
                offset += 1;
                column += 1;
            }
        }
    }

    Ok((tokens, warnings))
}
