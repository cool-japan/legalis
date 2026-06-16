//! Substitutions over the type language.
//!
//! A [`Subst`] maps type variables to [`MonoType`]s and row variables to
//! [`Row`]s. Substitutions are applied idempotently (the unifier keeps them so
//! by construction) and composed with [`Subst::compose`].

use std::collections::HashMap;

use super::types::{MonoType, Pred, QualType, Row, TypeScheme, VarId};

/// A simultaneous substitution for both type- and row-kinded variables.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Subst {
    types: HashMap<VarId, MonoType>,
    rows: HashMap<VarId, Row>,
}

impl Subst {
    /// The empty substitution.
    pub fn new() -> Self {
        Subst::default()
    }

    /// A substitution binding a single type variable.
    pub fn singleton_type(var: VarId, ty: MonoType) -> Self {
        let mut subst = Subst::new();
        subst.insert_type(var, ty);
        subst
    }

    /// A substitution binding a single row variable.
    pub fn singleton_row(var: VarId, row: Row) -> Self {
        let mut subst = Subst::new();
        subst.insert_row(var, row);
        subst
    }

    /// `true` when the substitution binds nothing.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty() && self.rows.is_empty()
    }

    /// Records a type-variable binding.
    pub fn insert_type(&mut self, var: VarId, ty: MonoType) {
        self.types.insert(var, ty);
    }

    /// Records a row-variable binding.
    pub fn insert_row(&mut self, var: VarId, row: Row) {
        self.rows.insert(var, row);
    }

    /// Looks up a type-variable binding.
    pub fn lookup_type(&self, var: VarId) -> Option<&MonoType> {
        self.types.get(&var)
    }

    /// Looks up a row-variable binding.
    pub fn lookup_row(&self, var: VarId) -> Option<&Row> {
        self.rows.get(&var)
    }

    /// Applies the substitution throughout a type, resolving variables
    /// recursively (the occurs-check guarantees this terminates).
    pub fn apply_type(&self, ty: &MonoType) -> MonoType {
        match ty {
            MonoType::Var(id) => match self.types.get(id) {
                Some(bound) => self.apply_type(bound),
                None => MonoType::Var(*id),
            },
            MonoType::Con { name, args } => MonoType::Con {
                name: name.clone(),
                args: args.iter().map(|arg| self.apply_type(arg)).collect(),
            },
            MonoType::Fun(from, to) => MonoType::Fun(
                Box::new(self.apply_type(from)),
                Box::new(self.apply_type(to)),
            ),
            MonoType::Record(row) => MonoType::Record(self.apply_row(row)),
        }
    }

    /// Applies the substitution throughout a row.
    ///
    /// When the tail variable is bound to another row, that row's fields are
    /// merged in (labels are disjoint between a row and its tail by
    /// construction, so this never silently drops a field).
    pub fn apply_row(&self, row: &Row) -> Row {
        let mut fields = std::collections::BTreeMap::new();
        for (label, ty) in &row.fields {
            fields.insert(label.clone(), self.apply_type(ty));
        }
        match row.tail {
            None => Row::new(fields, None),
            Some(tail) => match self.rows.get(&tail) {
                None => Row::new(fields, Some(tail)),
                Some(bound) => {
                    let resolved = self.apply_row(bound);
                    for (label, ty) in resolved.fields {
                        fields.entry(label).or_insert(ty);
                    }
                    Row::new(fields, resolved.tail)
                }
            },
        }
    }

    /// Applies the substitution to a predicate.
    pub fn apply_pred(&self, pred: &Pred) -> Pred {
        Pred::new(pred.class.clone(), self.apply_type(&pred.ty))
    }

    /// Applies the substitution to a qualified type.
    pub fn apply_qual(&self, qual: &QualType) -> QualType {
        QualType::new(
            qual.preds.iter().map(|p| self.apply_pred(p)).collect(),
            self.apply_type(&qual.ty),
        )
    }

    /// Applies the substitution to a scheme, skipping its quantified variables.
    ///
    /// The quantified variables shadow any outer binding, so they are removed
    /// from the substitution before it is pushed under the binder.
    pub fn apply_scheme(&self, scheme: &TypeScheme) -> TypeScheme {
        let mut restricted = self.clone();
        for id in &scheme.type_vars {
            restricted.types.remove(id);
        }
        for id in &scheme.row_vars {
            restricted.rows.remove(id);
        }
        TypeScheme::new(
            scheme.type_vars.clone(),
            scheme.row_vars.clone(),
            restricted.apply_qual(&scheme.qual),
        )
    }

    /// Composes two substitutions: `self.compose(other)` behaves as applying
    /// `other` first and then `self`.
    pub fn compose(&self, other: &Subst) -> Subst {
        let mut types = HashMap::new();
        for (var, ty) in &other.types {
            types.insert(*var, self.apply_type(ty));
        }
        for (var, ty) in &self.types {
            types.entry(*var).or_insert_with(|| ty.clone());
        }

        let mut rows = HashMap::new();
        for (var, row) in &other.rows {
            rows.insert(*var, self.apply_row(row));
        }
        for (var, row) in &self.rows {
            rows.entry(*var).or_insert_with(|| row.clone());
        }

        Subst { types, rows }
    }
}
