//! Unification, including extensible-record (row) unification.
//!
//! [`unify`] computes a most general unifier (mgu) of two monomorphic types,
//! performing the occurs-check on every variable binding. Records are unified
//! by [`unify_rows`] using the standard algorithm for rows with unique labels:
//! shared labels are unified pointwise, labels present on only one side are
//! absorbed by the other side's row variable, and a fresh row variable is
//! introduced when both rows carry extra fields *and* an open tail.

use std::collections::BTreeMap;

use super::error::{InferResult, TypeInferError};
use super::subst::Subst;
use super::types::{MonoType, Row, VarId, VarSupply};

/// Computes the most general unifier of two types.
///
/// The caller is expected to have applied any ambient substitution to both
/// arguments first; the returned substitution should then be composed onto the
/// ambient one.
pub fn unify(supply: &mut VarSupply, lhs: &MonoType, rhs: &MonoType) -> InferResult<Subst> {
    match (lhs, rhs) {
        (MonoType::Var(a), MonoType::Var(b)) if a == b => Ok(Subst::new()),
        (MonoType::Var(a), ty) | (ty, MonoType::Var(a)) => bind_type(*a, ty),
        (MonoType::Con { name: n1, args: a1 }, MonoType::Con { name: n2, args: a2 }) => {
            if n1 != n2 || a1.len() != a2.len() {
                return Err(mismatch(lhs, rhs));
            }
            let mut subst = Subst::new();
            for (x, y) in a1.iter().zip(a2.iter()) {
                let xs = subst.apply_type(x);
                let ys = subst.apply_type(y);
                let s = unify(supply, &xs, &ys)?;
                subst = s.compose(&subst);
            }
            Ok(subst)
        }
        (MonoType::Fun(f1, t1), MonoType::Fun(f2, t2)) => {
            let s1 = unify(supply, f1, f2)?;
            let t1s = s1.apply_type(t1);
            let t2s = s1.apply_type(t2);
            let s2 = unify(supply, &t1s, &t2s)?;
            Ok(s2.compose(&s1))
        }
        (MonoType::Record(r1), MonoType::Record(r2)) => unify_rows(supply, r1, r2),
        _ => Err(mismatch(lhs, rhs)),
    }
}

/// Binds a type variable to a type after an occurs-check.
fn bind_type(var: VarId, ty: &MonoType) -> InferResult<Subst> {
    if matches!(ty, MonoType::Var(other) if *other == var) {
        return Ok(Subst::new());
    }
    if ty.free_vars().contains_type(var) {
        return Err(TypeInferError::OccursCheck {
            var: format!("t{var}"),
            ty: ty.to_string(),
        });
    }
    Ok(Subst::singleton_type(var, ty.clone()))
}

/// Binds a row variable to a row after an occurs-check.
fn bind_row(var: VarId, row: &Row) -> InferResult<Subst> {
    if row.fields.is_empty() && row.tail == Some(var) {
        return Ok(Subst::new());
    }
    if row.free_vars().contains_row(var) {
        return Err(TypeInferError::OccursCheck {
            var: format!("r{var}"),
            ty: row.to_string(),
        });
    }
    Ok(Subst::singleton_row(var, row.clone()))
}

/// Unifies two record rows.
pub fn unify_rows(supply: &mut VarSupply, r1: &Row, r2: &Row) -> InferResult<Subst> {
    let mut subst = Subst::new();

    // 1. Unify the field types of labels common to both rows.
    for (label, ty1) in &r1.fields {
        if let Some(ty2) = r2.fields.get(label) {
            let t1 = subst.apply_type(ty1);
            let t2 = subst.apply_type(ty2);
            let s = unify(supply, &t1, &t2)?;
            subst = s.compose(&subst);
        }
    }

    // 2. Partition the remaining labels (applying the field substitution).
    let only1 = leftover_fields(&subst, &r1.fields, &r2.fields);
    let only2 = leftover_fields(&subst, &r2.fields, &r1.fields);
    let tail1 = r1.tail;
    let tail2 = r2.tail;

    match (only1.is_empty(), only2.is_empty()) {
        (true, true) => {
            let s = unify_tails(tail1, tail2)?;
            Ok(s.compose(&subst))
        }
        (true, false) => {
            // r1 must absorb the extra labels of r2, so r1 needs an open tail.
            let var = tail1.ok_or_else(|| missing_labels(&only2, r1))?;
            let absorbed = Row::new(only2, tail2);
            let s = bind_row(var, &absorbed)?;
            Ok(s.compose(&subst))
        }
        (false, true) => {
            let var = tail2.ok_or_else(|| missing_labels(&only1, r2))?;
            let absorbed = Row::new(only1, tail1);
            let s = bind_row(var, &absorbed)?;
            Ok(s.compose(&subst))
        }
        (false, false) => {
            // Both rows have extra labels: each tail must be open and the two
            // tails must differ, otherwise the row would be infinite.
            let var1 = tail1.ok_or_else(|| missing_labels(&only2, r1))?;
            let var2 = tail2.ok_or_else(|| missing_labels(&only1, r2))?;
            if var1 == var2 {
                return Err(TypeInferError::RowMismatch {
                    message: format!(
                        "cannot unify rows sharing tail r{var1} with differing fields"
                    ),
                });
            }
            let fresh = supply.fresh_row_var();
            let s1 = bind_row(var1, &Row::new(only2, Some(fresh)))?;
            let s2 = bind_row(var2, &Row::new(only1, Some(fresh)))?;
            Ok(s2.compose(&s1).compose(&subst))
        }
    }
}

/// Collects the substituted field types present in `from` but not in `other`.
fn leftover_fields(
    subst: &Subst,
    from: &BTreeMap<String, MonoType>,
    other: &BTreeMap<String, MonoType>,
) -> BTreeMap<String, MonoType> {
    from.iter()
        .filter(|(label, _)| !other.contains_key(*label))
        .map(|(label, ty)| (label.clone(), subst.apply_type(ty)))
        .collect()
}

/// Unifies two (optional) row-variable tails.
fn unify_tails(tail1: Option<VarId>, tail2: Option<VarId>) -> InferResult<Subst> {
    match (tail1, tail2) {
        (None, None) => Ok(Subst::new()),
        (Some(var), None) | (None, Some(var)) => bind_row(var, &Row::empty()),
        (Some(a), Some(b)) if a == b => Ok(Subst::new()),
        (Some(a), Some(b)) => bind_row(a, &Row::open(b)),
    }
}

/// Builds a `MissingLabel` error for the first absent label.
fn missing_labels(missing: &BTreeMap<String, MonoType>, closed: &Row) -> TypeInferError {
    let label = missing
        .keys()
        .next()
        .cloned()
        .unwrap_or_else(|| "?".to_string());
    TypeInferError::MissingLabel {
        label,
        row: closed.to_string(),
    }
}

/// Builds a `Mismatch` error from two types.
fn mismatch(lhs: &MonoType, rhs: &MonoType) -> TypeInferError {
    TypeInferError::Mismatch {
        expected: lhs.to_string(),
        found: rhs.to_string(),
    }
}
