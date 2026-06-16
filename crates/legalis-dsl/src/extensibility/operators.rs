//! Domain-specific operator definitions (roadmap v0.3.4).
//!
//! An [`OperatorTable`] holds user-registered operators, each with a precedence
//! and associativity, and drives a precedence-climbing (Pratt-style) parser over
//! a self-contained expression lexer. Because the core tokenizer drops unknown
//! symbol characters, this module ships its own small lexer so that *arbitrary*
//! operator symbols (`^`, `~>`, `<=>`, word operators like `AND`, …) can be
//! recognized and given meaning without touching the base grammar.
//!
//! The produced [`ExprNode`] tree can be evaluated numerically with
//! [`ExprNode::eval`], which makes operator precedence/associativity directly
//! testable.

use crate::{DslError, DslResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Associativity of an infix operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Associativity {
    /// Left-associative: `a - b - c` = `(a - b) - c`.
    Left,
    /// Right-associative: `a ^ b ^ c` = `a ^ (b ^ c)`.
    Right,
    /// Non-associative: chaining is a parse error.
    NonAssoc,
}

/// Where an operator may appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorFixity {
    /// Binary infix operator (`a + b`).
    Infix,
    /// Unary prefix operator (`- a`, `! a`).
    Prefix,
}

/// A registered operator definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorDef {
    /// The operator symbol (e.g. `+`, `^`, `~>`, `AND`).
    pub symbol: String,
    /// Binding power; higher binds tighter.
    pub precedence: u8,
    /// Associativity (ignored for prefix operators).
    pub associativity: Associativity,
    /// Fixity.
    pub fixity: OperatorFixity,
    /// Human-readable description.
    pub description: String,
}

impl OperatorDef {
    /// Creates an infix operator definition.
    pub fn infix(symbol: impl Into<String>, precedence: u8, associativity: Associativity) -> Self {
        Self {
            symbol: symbol.into(),
            precedence,
            associativity,
            fixity: OperatorFixity::Infix,
            description: String::new(),
        }
    }

    /// Creates a prefix operator definition.
    pub fn prefix(symbol: impl Into<String>, precedence: u8) -> Self {
        Self {
            symbol: symbol.into(),
            precedence,
            associativity: Associativity::Right,
            fixity: OperatorFixity::Prefix,
            description: String::new(),
        }
    }

    /// Attaches a description.
    pub fn described(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

/// A parsed expression over registered operators.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
    /// A numeric literal.
    Number(f64),
    /// An identifier (variable reference).
    Ident(String),
    /// A string literal.
    Str(String),
    /// A binary application.
    Binary {
        /// Operator symbol.
        op: String,
        /// Left operand.
        left: Box<ExprNode>,
        /// Right operand.
        right: Box<ExprNode>,
    },
    /// A prefix-unary application.
    Unary {
        /// Operator symbol.
        op: String,
        /// Operand.
        operand: Box<ExprNode>,
    },
}

impl ExprNode {
    /// Evaluates the expression numerically. Identifiers are looked up in `env`;
    /// the standard arithmetic operators (`+ - * / ^` and prefix `-`/`+`) are
    /// understood. Returns `None` for unknown identifiers or operators, or for a
    /// division by zero.
    pub fn eval(&self, env: &HashMap<String, f64>) -> Option<f64> {
        match self {
            ExprNode::Number(n) => Some(*n),
            ExprNode::Ident(name) => env.get(name).copied(),
            ExprNode::Str(_) => None,
            ExprNode::Unary { op, operand } => {
                let v = operand.eval(env)?;
                match op.as_str() {
                    "-" => Some(-v),
                    "+" => Some(v),
                    _ => None,
                }
            }
            ExprNode::Binary { op, left, right } => {
                let l = left.eval(env)?;
                let r = right.eval(env)?;
                match op.as_str() {
                    "+" => Some(l + r),
                    "-" => Some(l - r),
                    "*" => Some(l * r),
                    "/" => {
                        if r == 0.0 {
                            None
                        } else {
                            Some(l / r)
                        }
                    }
                    "^" => Some(l.powf(r)),
                    _ => None,
                }
            }
        }
    }
}

/// A table of registered operators driving the expression parser.
#[derive(Debug, Clone, Default)]
pub struct OperatorTable {
    infix: BTreeMap<String, OperatorDef>,
    prefix: BTreeMap<String, OperatorDef>,
}

impl OperatorTable {
    /// Creates an empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a table pre-populated with the standard arithmetic operators:
    /// `+`/`-` (precedence 10, left), `*`/`/` (20, left), `^` (30, right) and
    /// prefix `-`/`+` (40).
    pub fn standard() -> Self {
        let mut table = Self::new();
        table.register(OperatorDef::infix("+", 10, Associativity::Left).described("addition"));
        table.register(OperatorDef::infix("-", 10, Associativity::Left).described("subtraction"));
        table
            .register(OperatorDef::infix("*", 20, Associativity::Left).described("multiplication"));
        table.register(OperatorDef::infix("/", 20, Associativity::Left).described("division"));
        table.register(
            OperatorDef::infix("^", 30, Associativity::Right).described("exponentiation"),
        );
        table.register(OperatorDef::prefix("-", 40).described("negation"));
        table.register(OperatorDef::prefix("+", 40).described("unary plus"));
        table
    }

    /// Registers an operator (replacing any existing one of the same symbol and
    /// fixity).
    pub fn register(&mut self, def: OperatorDef) {
        match def.fixity {
            OperatorFixity::Infix => {
                self.infix.insert(def.symbol.clone(), def);
            }
            OperatorFixity::Prefix => {
                self.prefix.insert(def.symbol.clone(), def);
            }
        }
    }

    /// Looks up an infix operator.
    pub fn infix(&self, symbol: &str) -> Option<&OperatorDef> {
        self.infix.get(symbol)
    }

    /// Looks up a prefix operator.
    pub fn prefix(&self, symbol: &str) -> Option<&OperatorDef> {
        self.prefix.get(symbol)
    }

    /// Returns every registered operator definition (infix then prefix), sorted.
    pub fn all(&self) -> Vec<OperatorDef> {
        self.infix
            .values()
            .chain(self.prefix.values())
            .cloned()
            .collect()
    }

    /// Returns true if `symbol` names any registered operator.
    pub fn is_operator_symbol(&self, symbol: &str) -> bool {
        self.infix.contains_key(symbol) || self.prefix.contains_key(symbol)
    }

    /// Parses an expression string into an [`ExprNode`] using this table.
    pub fn parse(&self, input: &str) -> DslResult<ExprNode> {
        let tokens = lex_expression(input, self)?;
        let mut parser = ExprParser {
            tokens: &tokens,
            pos: 0,
            table: self,
        };
        let expr = parser.parse_expression(0)?;
        if !parser.is_eof() {
            return Err(DslError::parse_error(
                "Unexpected trailing tokens in expression",
            ));
        }
        Ok(expr)
    }
}

/// A token in the expression lexer.
#[derive(Debug, Clone, PartialEq)]
enum ExprToken {
    Number(f64),
    Ident(String),
    Str(String),
    LParen,
    RParen,
    Op(String),
}

/// Tokenizes an expression, recognizing registered operator symbols (longest
/// match), identifiers, numbers, strings and parentheses.
fn lex_expression(input: &str, table: &OperatorTable) -> DslResult<Vec<ExprToken>> {
    // Operator symbols sorted by descending length for greedy longest-match.
    let mut symbols: Vec<String> = table
        .infix
        .keys()
        .chain(table.prefix.keys())
        .cloned()
        .collect();
    symbols.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if ch.is_whitespace() {
            i += 1;
            continue;
        }
        match ch {
            '(' => {
                tokens.push(ExprToken::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(ExprToken::RParen);
                i += 1;
            }
            '"' => {
                i += 1;
                let mut s = String::new();
                while i < chars.len() && chars[i] != '"' {
                    s.push(chars[i]);
                    i += 1;
                }
                if i >= chars.len() {
                    return Err(DslError::parse_error("Unterminated string in expression"));
                }
                i += 1; // closing quote
                tokens.push(ExprToken::Str(s));
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let lexeme: String = chars[start..i].iter().collect();
                let value = lexeme
                    .parse::<f64>()
                    .map_err(|_| DslError::parse_error(format!("Invalid number '{lexeme}'")))?;
                tokens.push(ExprToken::Number(value));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
                {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                // A word that is a registered operator (e.g. `AND`) lexes as an
                // operator; otherwise it is an identifier.
                if table.is_operator_symbol(&word) {
                    tokens.push(ExprToken::Op(word));
                } else {
                    tokens.push(ExprToken::Ident(word));
                }
            }
            _ => {
                // A maximal run of operator characters, split greedily into the
                // longest registered symbols.
                let start = i;
                while i < chars.len() && is_op_char(chars[i]) {
                    i += 1;
                }
                let run: String = chars[start..i].iter().collect();
                split_operators(&run, &symbols, &mut tokens)?;
            }
        }
    }
    Ok(tokens)
}

/// Returns true for characters that may appear in an operator symbol.
fn is_op_char(c: char) -> bool {
    !c.is_alphanumeric() && !c.is_whitespace() && !matches!(c, '(' | ')' | '"' | '_')
}

/// Greedily splits a run of operator characters into registered operator tokens.
fn split_operators(run: &str, symbols: &[String], tokens: &mut Vec<ExprToken>) -> DslResult<()> {
    let run_chars: Vec<char> = run.chars().collect();
    let mut i = 0;
    while i < run_chars.len() {
        let remaining: String = run_chars[i..].iter().collect();
        let matched = symbols
            .iter()
            .find(|sym| !sym.is_empty() && remaining.starts_with(sym.as_str()));
        match matched {
            Some(sym) => {
                tokens.push(ExprToken::Op(sym.clone()));
                i += sym.chars().count();
            }
            None => {
                return Err(DslError::parse_error(format!(
                    "Unknown operator near '{remaining}'"
                )));
            }
        }
    }
    Ok(())
}

/// The precedence-climbing parser.
struct ExprParser<'a> {
    tokens: &'a [ExprToken],
    pos: usize,
    table: &'a OperatorTable,
}

impl ExprParser<'_> {
    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> Option<&ExprToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&ExprToken> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    /// Precedence-climbing core: parses operators with binding power >= `min_bp`.
    fn parse_expression(&mut self, min_bp: u8) -> DslResult<ExprNode> {
        let mut left = self.parse_prefix()?;

        while let Some(ExprToken::Op(sym)) = self.peek() {
            let Some(def) = self.table.infix(sym).cloned() else {
                break;
            };
            if def.precedence < min_bp {
                break;
            }
            self.advance(); // consume operator

            let next_min = match def.associativity {
                Associativity::Left | Associativity::NonAssoc => def.precedence + 1,
                Associativity::Right => def.precedence,
            };
            let right = self.parse_expression(next_min)?;

            // Reject chaining of non-associative operators of equal precedence.
            if def.associativity == Associativity::NonAssoc
                && let Some(ExprToken::Op(next_sym)) = self.peek()
                && self
                    .table
                    .infix(next_sym)
                    .is_some_and(|d| d.precedence == def.precedence)
            {
                return Err(DslError::parse_error(format!(
                    "Operator '{}' is non-associative and cannot be chained",
                    def.symbol
                )));
            }

            left = ExprNode::Binary {
                op: def.symbol.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses an optional prefix operator followed by a primary.
    fn parse_prefix(&mut self) -> DslResult<ExprNode> {
        if let Some(ExprToken::Op(sym)) = self.peek()
            && let Some(def) = self.table.prefix(sym).cloned()
        {
            self.advance();
            let operand = self.parse_expression(def.precedence)?;
            return Ok(ExprNode::Unary {
                op: def.symbol,
                operand: Box::new(operand),
            });
        }
        self.parse_primary()
    }

    /// Parses a primary expression (literal, identifier or parenthesized group).
    fn parse_primary(&mut self) -> DslResult<ExprNode> {
        match self.advance() {
            Some(ExprToken::Number(n)) => Ok(ExprNode::Number(*n)),
            Some(ExprToken::Ident(s)) => Ok(ExprNode::Ident(s.clone())),
            Some(ExprToken::Str(s)) => Ok(ExprNode::Str(s.clone())),
            Some(ExprToken::LParen) => {
                let inner = self.parse_expression(0)?;
                match self.advance() {
                    Some(ExprToken::RParen) => Ok(inner),
                    _ => Err(DslError::parse_error("Expected ')' in expression")),
                }
            }
            Some(ExprToken::Op(sym)) => Err(DslError::parse_error(format!(
                "Unexpected operator '{sym}' where an operand was expected"
            ))),
            Some(ExprToken::RParen) => Err(DslError::parse_error("Unexpected ')' in expression")),
            None => Err(DslError::parse_error("Unexpected end of expression")),
        }
    }
}
