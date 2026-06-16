//! Algebraic data types: declarations, constructor schemes, and an
//! exhaustiveness checker for `match`.
//!
//! A [`DataDecl`] introduces a (possibly parameterised) sum-of-products type.
//! Each [`Constructor`] is a product of field types. Constructors are turned
//! into polymorphic [`TypeScheme`]s — e.g. `Just : forall a. a -> Maybe a` —
//! so Algorithm W can treat constructor application uniformly with ordinary
//! function application.
//!
//! [`DataEnv::missing_constructors`] implements Maranget-style usefulness:
//! a column of patterns is *complete* only when every constructor of the data
//! type appears, so a `match` is exhaustive iff a wildcard row would be
//! redundant against the arm matrix.

use std::collections::HashMap;

use super::error::{InferResult, TypeInferError};
use super::term::Pattern;
use super::types::{MonoType, QualType, TypeScheme, VarId};

/// One constructor of an algebraic data type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    /// Constructor name (globally unique across the data environment).
    pub name: String,
    /// Field types, written in terms of the data type's parameters.
    pub fields: Vec<MonoType>,
}

impl Constructor {
    /// A nullary constructor (an enum variant).
    pub fn nullary(name: impl Into<String>) -> Self {
        Constructor {
            name: name.into(),
            fields: Vec::new(),
        }
    }

    /// A constructor with the given field types.
    pub fn new(name: impl Into<String>, fields: Vec<MonoType>) -> Self {
        Constructor {
            name: name.into(),
            fields,
        }
    }

    /// The constructor's arity (number of fields).
    pub fn arity(&self) -> usize {
        self.fields.len()
    }
}

/// A data-type declaration: a name, its type parameters, and its constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataDecl {
    /// Type name (e.g. `Maybe`).
    pub name: String,
    /// Parameter variable ids, in order (e.g. the `a` in `Maybe a`).
    pub params: Vec<VarId>,
    /// Constructors of the type.
    pub constructors: Vec<Constructor>,
}

impl DataDecl {
    /// A non-parameterised enum from a name and a list of nullary variants.
    ///
    /// This bridges the existing DSL enum types (`status: Active | Inactive`)
    /// into the algebraic-data machinery.
    pub fn enumeration(
        name: impl Into<String>,
        variants: impl IntoIterator<Item = String>,
    ) -> Self {
        let constructors = variants
            .into_iter()
            .map(Constructor::nullary)
            .collect::<Vec<_>>();
        DataDecl {
            name: name.into(),
            params: Vec::new(),
            constructors,
        }
    }

    /// Builds a parameterised declaration.
    pub fn new(
        name: impl Into<String>,
        params: Vec<VarId>,
        constructors: Vec<Constructor>,
    ) -> Self {
        DataDecl {
            name: name.into(),
            params,
            constructors,
        }
    }
}

/// The environment of declared data types and constructors.
#[derive(Debug, Clone, Default)]
pub struct DataEnv {
    decls: HashMap<String, DataDecl>,
    /// Maps a constructor name to the data type that owns it.
    ctor_owner: HashMap<String, String>,
}

impl DataEnv {
    /// An empty data environment.
    pub fn new() -> Self {
        DataEnv::default()
    }

    /// Registers a data declaration, indexing each of its constructors.
    pub fn declare(&mut self, decl: DataDecl) {
        for ctor in &decl.constructors {
            self.ctor_owner.insert(ctor.name.clone(), decl.name.clone());
        }
        self.decls.insert(decl.name.clone(), decl);
    }

    /// Looks up a declaration by type name.
    pub fn decl(&self, name: &str) -> Option<&DataDecl> {
        self.decls.get(name)
    }

    /// Looks up the constructor record by constructor name.
    pub fn constructor(&self, name: &str) -> Option<&Constructor> {
        let owner = self.ctor_owner.get(name)?;
        let decl = self.decls.get(owner)?;
        decl.constructors.iter().find(|c| c.name == name)
    }

    /// Returns the data type owning the named constructor.
    pub fn owner_of(&self, ctor: &str) -> Option<&str> {
        self.ctor_owner.get(ctor).map(String::as_str)
    }

    /// Returns all constructor names of the data type owning `ctor`.
    pub fn sibling_constructors(&self, ctor: &str) -> Option<Vec<String>> {
        let owner = self.ctor_owner.get(ctor)?;
        let decl = self.decls.get(owner)?;
        Some(decl.constructors.iter().map(|c| c.name.clone()).collect())
    }

    /// Produces a fresh-instantiated scheme for a constructor:
    /// `forall params. field0 -> ... -> fieldN -> Data params`.
    ///
    /// The scheme quantifies over the data type's parameters so each use site
    /// gets independent type variables.
    pub fn constructor_scheme(&self, name: &str) -> InferResult<TypeScheme> {
        let owner = self
            .ctor_owner
            .get(name)
            .ok_or_else(|| TypeInferError::UnknownConstructor(name.to_string()))?;
        let decl = self
            .decls
            .get(owner)
            .ok_or_else(|| TypeInferError::UnknownDataType(owner.clone()))?;
        let ctor = decl
            .constructors
            .iter()
            .find(|c| c.name == name)
            .ok_or_else(|| TypeInferError::UnknownConstructor(name.to_string()))?;

        let data_type = MonoType::app(
            decl.name.clone(),
            decl.params.iter().copied().map(MonoType::Var).collect(),
        );
        let body = MonoType::arrow(ctor.fields.clone(), data_type);
        Ok(TypeScheme::new(
            decl.params.clone(),
            Vec::new(),
            QualType::plain(body),
        ))
    }

    /// Checks a list of top-level `match` patterns for exhaustiveness and
    /// returns the names of any uncovered constructors (empty when exhaustive).
    ///
    /// `data_name` is the type of the scrutinee, used to know the full
    /// constructor set.
    pub fn missing_constructors(&self, data_name: &str, patterns: &[Pattern]) -> Vec<String> {
        let decl = match self.decls.get(data_name) {
            Some(decl) => decl,
            // Unknown type (e.g. matching on a builtin): only a wildcard makes
            // it exhaustive.
            None => {
                if patterns.iter().any(Pattern::is_irrefutable) {
                    return Vec::new();
                }
                return vec!["_".to_string()];
            }
        };

        // A single irrefutable pattern covers everything.
        if patterns.iter().any(Pattern::is_irrefutable) {
            return Vec::new();
        }

        let matrix: Vec<Vec<Pattern>> = patterns.iter().map(|p| vec![p.clone()]).collect();
        let mut missing = Vec::new();
        for ctor in &decl.constructors {
            let sub = wildcards(ctor.arity());
            if is_useful(self, &matrix, &cons_query(&ctor.name, &sub)) {
                missing.push(ctor.name.clone());
            }
        }
        missing
    }
}

/// Builds a query row for `ctor` with the given sub-patterns.
fn cons_query(ctor: &str, sub: &[Pattern]) -> Vec<Pattern> {
    vec![Pattern::constructor(ctor, sub.to_vec())]
}

/// Builds `n` wildcard patterns.
fn wildcards(n: usize) -> Vec<Pattern> {
    vec![Pattern::Wildcard; n]
}

/// Maranget's usefulness predicate `U(matrix, query)`: is `query` matched by at
/// least one value not already matched by some row of `matrix`?
///
/// The match is exhaustive exactly when no constructor query is useful.
fn is_useful(env: &DataEnv, matrix: &[Vec<Pattern>], query: &[Pattern]) -> bool {
    // Base case: zero columns.
    if query.is_empty() {
        return matrix.is_empty();
    }

    let head = &query[0];
    let rest = &query[1..];

    match head {
        Pattern::Constructor { name, args } => {
            let specialized = specialize(env, matrix, name, args.len());
            let mut new_query = args.clone();
            new_query.extend_from_slice(rest);
            is_useful(env, &specialized, &new_query)
        }
        Pattern::Lit(lit) => {
            let specialized = specialize_lit(matrix, lit);
            is_useful(env, &specialized, rest)
        }
        Pattern::Wildcard | Pattern::Var(_) => {
            let head_ctors = column_constructors(matrix);
            if let Some(complete) = complete_signature(env, &head_ctors) {
                // The first column is a complete constructor signature: the
                // query is useful iff it is useful for some constructor.
                for (ctor, arity) in complete {
                    let specialized = specialize(env, matrix, &ctor, arity);
                    let mut new_query = wildcards(arity);
                    new_query.extend_from_slice(rest);
                    if is_useful(env, &specialized, &new_query) {
                        return true;
                    }
                }
                false
            } else {
                // Incomplete (or literal / unknown) signature: fall through to
                // the default matrix.
                let defaulted = default_matrix(matrix);
                is_useful(env, &defaulted, rest)
            }
        }
    }
}

/// Collects the distinct constructor names appearing in the first column.
fn column_constructors(matrix: &[Vec<Pattern>]) -> Vec<String> {
    let mut names = Vec::new();
    for row in matrix {
        let Some(Pattern::Constructor { name, .. }) = row.first() else {
            continue;
        };
        if !names.contains(name) {
            names.push(name.clone());
        }
    }
    names
}

/// Returns the full constructor set with arities if `head_ctors` already covers
/// every constructor of a single data type, otherwise `None`.
fn complete_signature(env: &DataEnv, head_ctors: &[String]) -> Option<Vec<(String, usize)>> {
    let first = head_ctors.first()?;
    let owner = env.owner_of(first)?;
    let decl = env.decl(owner)?;
    let all_present = decl
        .constructors
        .iter()
        .all(|c| head_ctors.contains(&c.name));
    if all_present {
        Some(
            decl.constructors
                .iter()
                .map(|c| (c.name.clone(), c.arity()))
                .collect(),
        )
    } else {
        None
    }
}

/// Specialization `S(c, matrix)` for a constructor with `arity` fields.
fn specialize(
    _env: &DataEnv,
    matrix: &[Vec<Pattern>],
    ctor: &str,
    arity: usize,
) -> Vec<Vec<Pattern>> {
    let mut out = Vec::new();
    for row in matrix {
        let Some((head, tail)) = row.split_first() else {
            continue;
        };
        match head {
            Pattern::Constructor { name, args } if name == ctor => {
                let mut new_row = args.clone();
                new_row.extend_from_slice(tail);
                out.push(new_row);
            }
            Pattern::Wildcard | Pattern::Var(_) => {
                let mut new_row = wildcards(arity);
                new_row.extend_from_slice(tail);
                out.push(new_row);
            }
            _ => {}
        }
    }
    out
}

/// Specialization for a literal head pattern.
fn specialize_lit(matrix: &[Vec<Pattern>], lit: &super::types::Lit) -> Vec<Vec<Pattern>> {
    let mut out = Vec::new();
    for row in matrix {
        let Some((head, tail)) = row.split_first() else {
            continue;
        };
        match head {
            Pattern::Lit(other) if other == lit => out.push(tail.to_vec()),
            Pattern::Wildcard | Pattern::Var(_) => out.push(tail.to_vec()),
            _ => {}
        }
    }
    out
}

/// The default matrix `D(matrix)`: rows whose head is irrefutable, with the
/// head column dropped.
fn default_matrix(matrix: &[Vec<Pattern>]) -> Vec<Vec<Pattern>> {
    let mut out = Vec::new();
    for row in matrix {
        let Some((head, tail)) = row.split_first() else {
            continue;
        };
        if head.is_irrefutable() {
            out.push(tail.to_vec());
        }
    }
    out
}
