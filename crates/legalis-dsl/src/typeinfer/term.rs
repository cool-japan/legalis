//! The typed intermediate representation that DSL conditions and effects lower
//! into, over which Algorithm W runs.
//!
//! This is a small, explicitly-typed lambda calculus extended with records
//! (for row polymorphism), algebraic-data constructors and `match` (for ADTs),
//! and `let` (for let-generalization / polymorphism).

use super::types::Lit;

/// A typed term.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A literal value.
    Lit(Lit),
    /// A variable / function reference resolved against the environment.
    Var(String),
    /// Lambda abstraction `\param. body`.
    Abs(String, Box<Term>),
    /// Application `func arg`.
    App(Box<Term>, Box<Term>),
    /// Polymorphic `let name = bound in body` (generalizes `bound`).
    Let(String, Box<Term>, Box<Term>),
    /// Conditional `if cond then then_branch else else_branch`.
    If(Box<Term>, Box<Term>, Box<Term>),
    /// A closed record literal `{ l1 = e1, l2 = e2, ... }`.
    Record(Vec<(String, Term)>),
    /// Record extension `{ label = value | rest }`.
    RecordExtend(String, Box<Term>, Box<Term>),
    /// Record field selection `record.label`.
    RecordSelect(String, Box<Term>),
    /// Record field removal `record - label`.
    RecordRestrict(String, Box<Term>),
    /// Data constructor application `Ctor arg0 arg1 ...`.
    Construct(String, Vec<Term>),
    /// Pattern match `match scrutinee { arms }`.
    Match(Box<Term>, Vec<MatchArm>),
}

impl Term {
    /// Builds a variable term.
    pub fn var(name: impl Into<String>) -> Term {
        Term::Var(name.into())
    }

    /// Builds a lambda.
    pub fn abs(param: impl Into<String>, body: Term) -> Term {
        Term::Abs(param.into(), Box::new(body))
    }

    /// Builds an application.
    pub fn app(func: Term, arg: Term) -> Term {
        Term::App(Box::new(func), Box::new(arg))
    }

    /// Builds a left-associated application of `func` to many arguments.
    pub fn apply_many(func: Term, args: impl IntoIterator<Item = Term>) -> Term {
        args.into_iter().fold(func, Term::app)
    }

    /// Builds a `let`.
    pub fn let_in(name: impl Into<String>, bound: Term, body: Term) -> Term {
        Term::Let(name.into(), Box::new(bound), Box::new(body))
    }

    /// Builds an `if`.
    pub fn if_then_else(cond: Term, then_branch: Term, else_branch: Term) -> Term {
        Term::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch))
    }

    /// Builds a record selection.
    pub fn select(label: impl Into<String>, record: Term) -> Term {
        Term::RecordSelect(label.into(), Box::new(record))
    }
}

/// A single arm of a `match`.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    /// Pattern matched by this arm.
    pub pattern: Pattern,
    /// Body evaluated when the pattern matches.
    pub body: Term,
}

impl MatchArm {
    /// Builds a match arm.
    pub fn new(pattern: Pattern, body: Term) -> Self {
        MatchArm { pattern, body }
    }
}

/// A pattern in a `match` arm.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard `_` (matches anything, binds nothing).
    Wildcard,
    /// Variable pattern (matches anything, binds the name).
    Var(String),
    /// Literal pattern (matches an equal literal).
    Lit(Lit),
    /// Constructor pattern `Ctor sub0 sub1 ...`.
    Constructor {
        /// Constructor name.
        name: String,
        /// Sub-patterns for the constructor's fields.
        args: Vec<Pattern>,
    },
}

impl Pattern {
    /// Builds a constructor pattern.
    pub fn constructor(name: impl Into<String>, args: Vec<Pattern>) -> Pattern {
        Pattern::Constructor {
            name: name.into(),
            args,
        }
    }

    /// Builds a nullary constructor pattern.
    pub fn nullary(name: impl Into<String>) -> Pattern {
        Pattern::Constructor {
            name: name.into(),
            args: Vec::new(),
        }
    }

    /// `true` when the pattern is an irrefutable binder (wildcard or variable).
    pub fn is_irrefutable(&self) -> bool {
        matches!(self, Pattern::Wildcard | Pattern::Var(_))
    }
}
