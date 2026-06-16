//! Core type representation for the Hindley–Milner inference engine.
//!
//! The type language is deliberately small but expressive enough for the five
//! advanced-type-system features this module targets:
//!
//! * [`MonoType`] — monomorphic types: variables, applied constructors,
//!   function arrows and records.
//! * [`Row`] — an extensible record row (a set of labelled fields plus an
//!   optional *row variable* tail) which powers **row polymorphism**.
//! * [`Pred`] / [`QualType`] — qualified types carrying type-class predicates,
//!   the basis of **type classes**.
//! * [`TypeScheme`] — a `∀`-quantified qualified type (over both ordinary type
//!   variables and row variables), the basis of **let-polymorphism**.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Identifier for a unification variable.
///
/// Type-kinded and row-kinded variables share one monotonically increasing
/// id-space (so an id is globally unique within an inference run) but live in
/// separate substitution maps; the [`Kind`] disambiguates them when needed.
pub type VarId = u32;

/// The kind of a unification variable: an ordinary type, or a record row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// An ordinary type variable (kind `*`).
    Type,
    /// A row variable (kind `row`), only valid as a record tail.
    Row,
}

/// A literal value in the typed term IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    /// 64-bit signed integer literal.
    Int(i64),
    /// Boolean literal.
    Bool(bool),
    /// String literal.
    Str(String),
    /// ISO date literal, kept as text.
    Date(String),
    /// Arbitrary-precision decimal literal, kept as text.
    Decimal(String),
}

impl Lit {
    /// The name of the base type constructor this literal inhabits.
    pub fn type_name(&self) -> &'static str {
        match self {
            Lit::Int(_) => "Int",
            Lit::Bool(_) => "Bool",
            Lit::Str(_) => "String",
            Lit::Date(_) => "Date",
            Lit::Decimal(_) => "Decimal",
        }
    }

    /// The [`MonoType`] this literal inhabits.
    pub fn mono_type(&self) -> MonoType {
        MonoType::con(self.type_name())
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(n) => write!(f, "{n}"),
            Lit::Bool(b) => write!(f, "{b}"),
            Lit::Str(s) => write!(f, "{s:?}"),
            Lit::Date(d) => write!(f, "{d}"),
            Lit::Decimal(d) => write!(f, "{d}"),
        }
    }
}

/// A monomorphic type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonoType {
    /// A unification (type) variable.
    Var(VarId),
    /// An applied type constructor, e.g. `Int`, `Bool`, `List a`, or a
    /// user-declared algebraic data type such as `Maybe a`.
    Con {
        /// Constructor name.
        name: String,
        /// Constructor arguments (empty for nullary constructors).
        args: Vec<MonoType>,
    },
    /// A function arrow `from -> to`.
    Fun(Box<MonoType>, Box<MonoType>),
    /// A record type built over a [`Row`].
    Record(Row),
}

impl MonoType {
    /// A nullary type constructor (e.g. `Int`).
    pub fn con(name: impl Into<String>) -> Self {
        MonoType::Con {
            name: name.into(),
            args: Vec::new(),
        }
    }

    /// An applied type constructor (e.g. `List a`).
    pub fn app(name: impl Into<String>, args: Vec<MonoType>) -> Self {
        MonoType::Con {
            name: name.into(),
            args,
        }
    }

    /// The built-in `Int` type.
    pub fn int() -> Self {
        Self::con("Int")
    }

    /// The built-in `Bool` type.
    pub fn boolean() -> Self {
        Self::con("Bool")
    }

    /// The built-in `String` type.
    pub fn string() -> Self {
        Self::con("String")
    }

    /// The built-in `Date` type.
    pub fn date() -> Self {
        Self::con("Date")
    }

    /// The built-in `Decimal` type.
    pub fn decimal() -> Self {
        Self::con("Decimal")
    }

    /// The list type `List elem`.
    pub fn list(elem: MonoType) -> Self {
        Self::app("List", vec![elem])
    }

    /// A single function arrow `from -> to`.
    pub fn func(from: MonoType, to: MonoType) -> Self {
        MonoType::Fun(Box::new(from), Box::new(to))
    }

    /// A curried function type `args[0] -> args[1] -> ... -> ret`.
    pub fn arrow(args: Vec<MonoType>, ret: MonoType) -> Self {
        args.into_iter()
            .rev()
            .fold(ret, |acc, arg| MonoType::func(arg, acc))
    }

    /// A record type over the given row.
    pub fn record(row: Row) -> Self {
        MonoType::Record(row)
    }

    /// Returns the constructor name if this is a (possibly applied) constructor.
    pub fn head_con(&self) -> Option<&str> {
        match self {
            MonoType::Con { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }

    /// Returns `true` when the type is in *head normal form* for the purposes of
    /// type-class context reduction — i.e. its head is a type variable, so no
    /// instance can fire on it yet.
    pub fn is_hnf(&self) -> bool {
        matches!(self, MonoType::Var(_))
    }

    /// Collects the free type and row variables of this type.
    pub fn free_vars(&self) -> FreeVars {
        let mut acc = FreeVars::default();
        self.collect_free(&mut acc);
        acc
    }

    fn collect_free(&self, acc: &mut FreeVars) {
        match self {
            MonoType::Var(id) => {
                acc.types.insert(*id);
            }
            MonoType::Con { args, .. } => {
                for arg in args {
                    arg.collect_free(acc);
                }
            }
            MonoType::Fun(from, to) => {
                from.collect_free(acc);
                to.collect_free(acc);
            }
            MonoType::Record(row) => row.collect_free(acc),
        }
    }
}

impl fmt::Display for MonoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonoType::Var(id) => write!(f, "t{id}"),
            MonoType::Con { name, args } => {
                if args.is_empty() {
                    write!(f, "{name}")
                } else {
                    write!(f, "{name}")?;
                    for arg in args {
                        // Parenthesise composite arguments for readability.
                        if matches!(arg, MonoType::Con { args, .. } if !args.is_empty())
                            || matches!(arg, MonoType::Fun(_, _))
                        {
                            write!(f, " ({arg})")?;
                        } else {
                            write!(f, " {arg}")?;
                        }
                    }
                    Ok(())
                }
            }
            MonoType::Fun(from, to) => {
                // The arrow associates to the right; parenthesise a function on
                // the left of an arrow.
                if matches!(**from, MonoType::Fun(_, _)) {
                    write!(f, "({from}) -> {to}")
                } else {
                    write!(f, "{from} -> {to}")
                }
            }
            MonoType::Record(row) => write!(f, "{row}"),
        }
    }
}

/// An extensible record row: a sorted map of labelled field types plus an
/// optional row-variable tail.
///
/// * `tail == None` is a *closed* row — the record has exactly these fields.
/// * `tail == Some(r)` is an *open* row — the record has at least these fields
///   plus whatever `r` is later unified to. Open rows are what make record
///   types row-polymorphic.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Row {
    /// Present fields, kept sorted by label for deterministic display/equality.
    pub fields: BTreeMap<String, MonoType>,
    /// The row-variable tail, if the row is open.
    pub tail: Option<VarId>,
}

impl Row {
    /// The empty closed row `{}`.
    pub fn empty() -> Self {
        Row::default()
    }

    /// An open row `{ | r }` with no concrete fields.
    pub fn open(tail: VarId) -> Self {
        Row {
            fields: BTreeMap::new(),
            tail: Some(tail),
        }
    }

    /// Builds a row from a field map and optional tail.
    pub fn new(fields: BTreeMap<String, MonoType>, tail: Option<VarId>) -> Self {
        Row { fields, tail }
    }

    /// Returns a copy of this row with `label: ty` added.
    pub fn with(mut self, label: impl Into<String>, ty: MonoType) -> Self {
        self.fields.insert(label.into(), ty);
        self
    }

    /// `true` if the row is closed (has no tail variable).
    pub fn is_closed(&self) -> bool {
        self.tail.is_none()
    }

    /// `true` if the row is open (has a tail variable).
    pub fn is_open(&self) -> bool {
        self.tail.is_some()
    }

    /// Collects free type and row variables of this row.
    pub fn free_vars(&self) -> FreeVars {
        let mut acc = FreeVars::default();
        self.collect_free(&mut acc);
        acc
    }

    fn collect_free(&self, acc: &mut FreeVars) {
        for ty in self.fields.values() {
            ty.collect_free(acc);
        }
        if let Some(tail) = self.tail {
            acc.rows.insert(tail);
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (label, ty) in &self.fields {
            if first {
                write!(f, " ")?;
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{label}: {ty}")?;
        }
        match self.tail {
            Some(tail) => write!(f, " | r{tail} }}"),
            None if first => write!(f, "}}"),
            None => write!(f, " }}"),
        }
    }
}

/// A type-class predicate (constraint), e.g. `Ord Int` or `Eq a`.
///
/// Single-parameter type classes are sufficient for modelling condition
/// behaviours (orderability, equality, numerics, pattern-matchability).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred {
    /// Class name (e.g. `Ord`).
    pub class: String,
    /// The type the constraint ranges over.
    pub ty: MonoType,
}

impl Pred {
    /// Builds a predicate `class ty`.
    pub fn new(class: impl Into<String>, ty: MonoType) -> Self {
        Pred {
            class: class.into(),
            ty,
        }
    }

    /// `true` when the predicate is in head normal form (its type's head is a
    /// variable), meaning no instance can be applied to simplify it further.
    pub fn is_hnf(&self) -> bool {
        self.ty.is_hnf()
    }

    /// Free variables of the predicate.
    pub fn free_vars(&self) -> FreeVars {
        self.ty.free_vars()
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.class, self.ty)
    }
}

/// A qualified type `preds => ty` (a context together with a head type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualType {
    /// Class predicates that must hold.
    pub preds: Vec<Pred>,
    /// The underlying monomorphic type.
    pub ty: MonoType,
}

impl QualType {
    /// A qualified type with no predicates.
    pub fn plain(ty: MonoType) -> Self {
        QualType {
            preds: Vec::new(),
            ty,
        }
    }

    /// A qualified type with the given predicates and head type.
    pub fn new(preds: Vec<Pred>, ty: MonoType) -> Self {
        QualType { preds, ty }
    }

    /// Free variables of both the predicates and the head type.
    pub fn free_vars(&self) -> FreeVars {
        let mut acc = self.ty.free_vars();
        for pred in &self.preds {
            acc.union_with(&pred.free_vars());
        }
        acc
    }
}

impl fmt::Display for QualType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.preds.is_empty() {
            write!(f, "{}", self.ty)
        } else {
            write!(f, "(")?;
            for (i, pred) in self.preds.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{pred}")?;
            }
            write!(f, ") => {}", self.ty)
        }
    }
}

/// A `∀`-quantified qualified type — the polymorphic types stored in the
/// environment and produced by let-generalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScheme {
    /// Quantified ordinary type variables.
    pub type_vars: Vec<VarId>,
    /// Quantified row variables.
    pub row_vars: Vec<VarId>,
    /// The quantified qualified type.
    pub qual: QualType,
}

impl TypeScheme {
    /// A monomorphic scheme (no quantifiers, no predicates).
    pub fn mono(ty: MonoType) -> Self {
        TypeScheme {
            type_vars: Vec::new(),
            row_vars: Vec::new(),
            qual: QualType::plain(ty),
        }
    }

    /// A scheme directly from a qualified type with no quantifiers.
    pub fn qualified(qual: QualType) -> Self {
        TypeScheme {
            type_vars: Vec::new(),
            row_vars: Vec::new(),
            qual,
        }
    }

    /// Builds a fully specified scheme.
    pub fn new(type_vars: Vec<VarId>, row_vars: Vec<VarId>, qual: QualType) -> Self {
        TypeScheme {
            type_vars,
            row_vars,
            qual,
        }
    }

    /// `true` when nothing is quantified.
    pub fn is_monomorphic(&self) -> bool {
        self.type_vars.is_empty() && self.row_vars.is_empty()
    }

    /// Free variables — those of the body minus the quantified variables.
    pub fn free_vars(&self) -> FreeVars {
        let mut acc = self.qual.free_vars();
        for id in &self.type_vars {
            acc.types.remove(id);
        }
        for id in &self.row_vars {
            acc.rows.remove(id);
        }
        acc
    }

    /// Produces an alpha-equivalent scheme whose quantified variables are
    /// renumbered canonically (type variables `0,1,…` and, independently, row
    /// variables `0,1,…`) in order of first appearance.
    ///
    /// Two schemes that differ only by the names of their bound variables share
    /// the same normal form, which makes scheme equality robust in tests.
    pub fn normalized(&self) -> TypeScheme {
        let mut type_map: BTreeMap<VarId, VarId> = BTreeMap::new();
        let mut row_map: BTreeMap<VarId, VarId> = BTreeMap::new();
        let type_set: BTreeSet<VarId> = self.type_vars.iter().copied().collect();
        let row_set: BTreeSet<VarId> = self.row_vars.iter().copied().collect();

        // Walk the body in a fixed order to assign canonical ids.
        assign_canonical(&self.qual, &type_set, &row_set, &mut type_map, &mut row_map);

        // Apply the renaming in a single, non-transitive pass. Using a
        // [`Subst`] here would be wrong: a canonical id may coincide with an
        // original id, and `Subst::apply` resolves bindings transitively,
        // which would corrupt the permutation (or loop on an identity binding).
        let qual = rename_qual(&self.qual, &type_map, &row_map);
        let mut type_vars: Vec<VarId> = type_map.values().copied().collect();
        let mut row_vars: Vec<VarId> = row_map.values().copied().collect();
        type_vars.sort_unstable();
        type_vars.dedup();
        row_vars.sort_unstable();
        row_vars.dedup();
        TypeScheme {
            type_vars,
            row_vars,
            qual,
        }
    }
}

/// Renames the variables of a type according to the (non-transitive) maps,
/// leaving variables absent from the maps untouched.
fn rename_type(
    ty: &MonoType,
    type_map: &BTreeMap<VarId, VarId>,
    row_map: &BTreeMap<VarId, VarId>,
) -> MonoType {
    match ty {
        MonoType::Var(id) => MonoType::Var(*type_map.get(id).unwrap_or(id)),
        MonoType::Con { name, args } => MonoType::Con {
            name: name.clone(),
            args: args
                .iter()
                .map(|arg| rename_type(arg, type_map, row_map))
                .collect(),
        },
        MonoType::Fun(from, to) => MonoType::Fun(
            Box::new(rename_type(from, type_map, row_map)),
            Box::new(rename_type(to, type_map, row_map)),
        ),
        MonoType::Record(row) => MonoType::Record(rename_row(row, type_map, row_map)),
    }
}

/// Renames the variables of a row according to the (non-transitive) maps.
fn rename_row(
    row: &Row,
    type_map: &BTreeMap<VarId, VarId>,
    row_map: &BTreeMap<VarId, VarId>,
) -> Row {
    let fields = row
        .fields
        .iter()
        .map(|(label, ty)| (label.clone(), rename_type(ty, type_map, row_map)))
        .collect();
    let tail = row.tail.map(|t| *row_map.get(&t).unwrap_or(&t));
    Row::new(fields, tail)
}

/// Renames the variables of a qualified type according to the (non-transitive)
/// maps. Used both to normalize schemes and to instantiate them with fresh
/// variables (a single-pass rename avoids the self-binding loop that a
/// transitively-applied [`crate::typeinfer::subst::Subst`] would hit when a
/// fresh id coincides with a bound id).
pub(crate) fn rename_qual(
    qual: &QualType,
    type_map: &BTreeMap<VarId, VarId>,
    row_map: &BTreeMap<VarId, VarId>,
) -> QualType {
    let preds = qual
        .preds
        .iter()
        .map(|pred| Pred::new(pred.class.clone(), rename_type(&pred.ty, type_map, row_map)))
        .collect();
    QualType::new(preds, rename_type(&qual.ty, type_map, row_map))
}

/// Assigns canonical ids to quantified variables in first-appearance order.
fn assign_canonical(
    qual: &QualType,
    type_set: &BTreeSet<VarId>,
    row_set: &BTreeSet<VarId>,
    type_map: &mut BTreeMap<VarId, VarId>,
    row_map: &mut BTreeMap<VarId, VarId>,
) {
    fn walk_type(
        ty: &MonoType,
        type_set: &BTreeSet<VarId>,
        row_set: &BTreeSet<VarId>,
        type_map: &mut BTreeMap<VarId, VarId>,
        row_map: &mut BTreeMap<VarId, VarId>,
    ) {
        match ty {
            MonoType::Var(id) => {
                if type_set.contains(id) && !type_map.contains_key(id) {
                    let next = type_map.len() as VarId;
                    type_map.insert(*id, next);
                }
            }
            MonoType::Con { args, .. } => {
                for arg in args {
                    walk_type(arg, type_set, row_set, type_map, row_map);
                }
            }
            MonoType::Fun(from, to) => {
                walk_type(from, type_set, row_set, type_map, row_map);
                walk_type(to, type_set, row_set, type_map, row_map);
            }
            MonoType::Record(row) => {
                for field in row.fields.values() {
                    walk_type(field, type_set, row_set, type_map, row_map);
                }
                if let Some(tail) = row.tail {
                    let unseen = row_set.contains(&tail) && !row_map.contains_key(&tail);
                    if unseen {
                        let next = row_map.len() as VarId;
                        row_map.insert(tail, next);
                    }
                }
            }
        }
    }

    for pred in &qual.preds {
        walk_type(&pred.ty, type_set, row_set, type_map, row_map);
    }
    walk_type(&qual.ty, type_set, row_set, type_map, row_map);
}

impl fmt::Display for TypeScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_monomorphic() {
            return write!(f, "{}", self.qual);
        }
        write!(f, "forall")?;
        for id in &self.type_vars {
            write!(f, " t{id}")?;
        }
        for id in &self.row_vars {
            write!(f, " r{id}")?;
        }
        write!(f, ". {}", self.qual)
    }
}

/// A pair of free-variable sets, one per kind.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FreeVars {
    /// Free ordinary type variables.
    pub types: BTreeSet<VarId>,
    /// Free row variables.
    pub rows: BTreeSet<VarId>,
}

impl FreeVars {
    /// Merges another set into this one.
    pub fn union_with(&mut self, other: &FreeVars) {
        self.types.extend(other.types.iter().copied());
        self.rows.extend(other.rows.iter().copied());
    }

    /// `true` when both kinds of free-variable sets are empty.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty() && self.rows.is_empty()
    }

    /// `true` if the given type variable is free here.
    pub fn contains_type(&self, id: VarId) -> bool {
        self.types.contains(&id)
    }

    /// `true` if the given row variable is free here.
    pub fn contains_row(&self, id: VarId) -> bool {
        self.rows.contains(&id)
    }
}

/// A monotonic supplier of fresh unification variables.
#[derive(Debug, Clone, Default)]
pub struct VarSupply {
    next: VarId,
}

impl VarSupply {
    /// Creates a fresh supply starting at zero.
    pub fn new() -> Self {
        VarSupply::default()
    }

    /// Returns a fresh, never-before-used variable id.
    pub fn fresh(&mut self) -> VarId {
        let id = self.next;
        self.next += 1;
        id
    }

    /// Returns a fresh type variable.
    pub fn fresh_type(&mut self) -> MonoType {
        MonoType::Var(self.fresh())
    }

    /// Returns a fresh row variable id.
    pub fn fresh_row_var(&mut self) -> VarId {
        self.fresh()
    }
}
