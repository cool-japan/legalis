//! SMT solver integration for formal verification.
//!
//! This module provides OxiZ SMT solver integration for:
//! - Satisfiability checking of conditions
//! - Contradiction detection
//! - Tautology verification
//! - Counterexample generation

use anyhow::Result;
use chrono::NaiveDate;
use legalis_core::{ComparisonOp, Condition};
use num_bigint::BigInt;
use oxiz_core::{TermId, TermKind, TermManager};
use oxiz_solver::{Solver, SolverResult};
use std::collections::HashMap;

/// SMT-based verifier for legal conditions.
pub struct SmtVerifier {
    solver: Solver,
    tm: TermManager,
    /// Maps entity attribute names to OxiZ integer variables
    int_vars: HashMap<String, TermId>,
    /// Maps boolean attributes to OxiZ boolean variables
    bool_vars: HashMap<String, TermId>,
    /// Maps array names to OxiZ array variables (Int -> Int arrays)
    int_arrays: HashMap<String, TermId>,
    /// Maps bitvector names to OxiZ bitvector variables
    bv_vars: HashMap<String, TermId>,
}

impl Default for SmtVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl SmtVerifier {
    /// Creates a new SMT verifier.
    #[must_use]
    pub fn new() -> Self {
        let mut solver = Solver::new();
        solver.set_logic("QF_LIA"); // Quantifier-free linear integer arithmetic
        let tm = TermManager::new();
        Self {
            solver,
            tm,
            int_vars: HashMap::new(),
            bool_vars: HashMap::new(),
            int_arrays: HashMap::new(),
            bv_vars: HashMap::new(),
        }
    }

    /// Reset the solver and restore the QF_LIA logic setting
    fn reset_solver(&mut self) {
        self.solver.reset();
        self.solver.set_logic("QF_LIA");
        self.clear_vars();
    }

    /// Checks if a condition is satisfiable.
    ///
    /// Returns `Ok(true)` if the condition can be satisfied by some assignment,
    /// `Ok(false)` if the condition is unsatisfiable (always false).
    pub fn is_satisfiable(&mut self, condition: &Condition) -> Result<bool> {
        self.reset_solver();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(formula, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks if a condition is a tautology (always true).
    ///
    /// A condition is a tautology if its negation is unsatisfiable.
    pub fn is_tautology(&mut self, condition: &Condition) -> Result<bool> {
        self.reset_solver();
        let formula = self.translate_condition(condition)?;
        let negated = self.tm.mk_not(formula);
        self.solver.assert(negated, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(false), // Negation is satisfiable, so not a tautology
            SolverResult::Unsat => Ok(true), // Negation is unsatisfiable, so it's a tautology
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks if two conditions contradict each other.
    ///
    /// Returns `Ok(true)` if (cond1 AND cond2) is unsatisfiable.
    pub fn contradict(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        self.reset_solver();
        let formula1 = self.translate_condition(cond1)?;
        let formula2 = self.translate_condition(cond2)?;
        let conjunction = self.tm.mk_and([formula1, formula2]);
        self.solver.assert(conjunction, &mut self.tm);

        let result = self.solver.check(&mut self.tm);
        match result {
            SolverResult::Sat => Ok(false), // Conjunction is satisfiable, no contradiction
            SolverResult::Unsat => Ok(true), // Conjunction is unsatisfiable, they contradict
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks if cond1 implies cond2 (cond1 → cond2).
    ///
    /// Returns `Ok(true)` if cond1 => cond2 is a tautology.
    pub fn implies(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        self.reset_solver();
        let formula1 = self.translate_condition(cond1)?;
        let formula2 = self.translate_condition(cond2)?;

        // Check if !(cond1 => cond2) is unsatisfiable
        // !(cond1 => cond2) = cond1 ∧ ¬cond2
        let not_formula2 = self.tm.mk_not(formula2);
        let not_implies = self.tm.mk_and([formula1, not_formula2]);
        self.solver.assert(not_implies, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(false), // Found counterexample where cond1 is true but cond2 is false
            SolverResult::Unsat => Ok(true), // Implication holds
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Gets a counterexample (model) if the condition is satisfiable.
    ///
    /// Returns a mapping of variable names to their values.
    pub fn get_model(&mut self, condition: &Condition) -> Result<Option<HashMap<String, i64>>> {
        self.reset_solver();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(formula, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => {
                let model = self.solver.model();
                let mut result = HashMap::new();

                if let Some(model) = model {
                    // Extract values for integer variables
                    for (name, var) in &self.int_vars {
                        if let Some(value_term) = model.get(*var)
                            && let Some(term) = self.tm.get(value_term)
                            && let TermKind::IntConst(ref val) = term.kind
                            && let Some(i) = val.to_i64()
                        {
                            result.insert(name.clone(), i);
                        }
                    }
                }

                Ok(Some(result))
            }
            SolverResult::Unsat => Ok(None),
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks equivalence of two conditions.
    ///
    /// Returns `Ok(true)` if the two conditions are logically equivalent.
    pub fn equivalent(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        // Two conditions are equivalent if cond1 <=> cond2
        // which is (cond1 => cond2) AND (cond2 => cond1)
        let implies_forward = self.implies(cond1, cond2)?;
        let implies_backward = self.implies(cond2, cond1)?;
        Ok(implies_forward && implies_backward)
    }

    /// Alias for equivalent() for API compatibility
    pub fn are_equivalent(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        self.equivalent(cond1, cond2)
    }

    /// Finds the minimal unsatisfiable core of conditions.
    ///
    /// Given a set of conditions that are unsatisfiable together,
    /// returns a minimal subset that is still unsatisfiable.
    pub fn find_unsat_core(&mut self, conditions: &[Condition]) -> Result<Vec<usize>> {
        self.reset_solver();
        self.solver.set_produce_unsat_cores(true);

        // Assert each condition with a tracking literal
        let mut tracking_literals = Vec::new();
        for (i, cond) in conditions.iter().enumerate() {
            let formula = self.translate_condition(cond)?;
            let track_name = format!("track_{}", i);
            let track_lit = self.tm.mk_var(&track_name, self.tm.sorts.bool_sort);
            tracking_literals.push((i, track_lit));

            // Assert: track_lit => formula
            let not_track = self.tm.mk_not(track_lit);
            let implication = self.tm.mk_or([not_track, formula]);
            self.solver.assert(implication, &mut self.tm);
        }

        // Assert all tracking literals
        for (_, lit) in &tracking_literals {
            self.solver.assert(*lit, &mut self.tm);
        }

        match self.solver.check(&mut self.tm) {
            SolverResult::Unsat => {
                // Get unsat core
                let mut core_indices = Vec::new();

                if let Some(core) = self.solver.get_unsat_core() {
                    // Use the names to find tracking literals
                    for name in &core.names {
                        if let Some(idx_str) = name.strip_prefix("track_")
                            && let Ok(idx) = idx_str.parse::<usize>()
                        {
                            core_indices.push(idx);
                        }
                    }
                }

                // If core is empty, return all indices as a fallback
                if core_indices.is_empty() {
                    core_indices = (0..conditions.len()).collect();
                }

                Ok(core_indices)
            }
            SolverResult::Sat => Err(anyhow::anyhow!("Conditions are satisfiable, no unsat core")),
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Resets the solver state, clearing all assertions.
    pub fn reset(&mut self) {
        self.reset_solver();
    }

    /// Clears variable mappings
    fn clear_vars(&mut self) {
        self.int_vars.clear();
        self.bool_vars.clear();
        self.int_arrays.clear();
        self.bv_vars.clear();
    }

    /// Pushes a new scope for incremental solving.
    pub fn push(&mut self) {
        self.solver.push();
    }

    /// Pops the most recent scope.
    pub fn pop(&mut self) {
        self.solver.pop();
    }

    /// Asserts a condition without checking satisfiability.
    ///
    /// Useful for incremental solving where multiple conditions
    /// are added before checking.
    pub fn assert_condition(&mut self, condition: &Condition) -> Result<()> {
        let formula = self.translate_condition(condition)?;
        self.solver.assert(formula, &mut self.tm);
        Ok(())
    }

    /// Checks the current set of assertions for satisfiability.
    pub fn check(&mut self) -> Result<bool> {
        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Gets statistics about the current solver state.
    #[must_use]
    pub fn get_statistics(&self) -> HashMap<String, String> {
        let stats = self.solver.get_statistics();
        let mut result = HashMap::new();

        result.insert("conflicts".to_string(), stats.conflicts.to_string());
        result.insert("decisions".to_string(), stats.decisions.to_string());
        result.insert("propagations".to_string(), stats.propagations.to_string());
        result.insert("restarts".to_string(), stats.restarts.to_string());

        result
    }

    /// Generates a proof for an unsatisfiable formula.
    ///
    /// Returns a proof object that can be exported or analyzed.
    pub fn get_proof(&mut self, condition: &Condition) -> Result<Option<String>> {
        self.reset_solver();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(formula, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Unsat => {
                // Get proof from OxiZ
                if let Some(proof) = self.solver.get_proof() {
                    Ok(Some(format!("{:?}", proof)))
                } else {
                    Ok(None)
                }
            }
            SolverResult::Sat => Err(anyhow::anyhow!(
                "Formula is satisfiable, no proof available"
            )),
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Generates a human-readable proof explanation.
    ///
    /// This produces a textual explanation of why a condition is unsatisfiable,
    /// using the unsat core and other information from the solver.
    pub fn explain_unsat(&mut self, conditions: &[Condition]) -> Result<String> {
        if conditions.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"));
        }

        let core_indices = self.find_unsat_core(conditions)?;

        let mut explanation = String::new();
        explanation.push_str("Unsatisfiability Explanation:\n\n");
        explanation.push_str(&format!("Total conditions: {}\n", conditions.len()));
        explanation.push_str(&format!(
            "Core conditions causing unsatisfiability: {}\n\n",
            core_indices.len()
        ));

        explanation.push_str("The following conditions cannot be satisfied together:\n");
        for idx in &core_indices {
            if let Some(cond) = conditions.get(*idx) {
                explanation.push_str(&format!("  [{}] {}\n", idx, cond));
            }
        }

        explanation.push_str("\nExplanation:\n");
        explanation.push_str("These conditions form a minimal unsatisfiable core, meaning that\n");
        explanation.push_str("there is no possible assignment of values that would satisfy all of them simultaneously.\n");

        Ok(explanation)
    }

    /// Translates a Legalis condition to an OxiZ formula.
    fn translate_condition(&mut self, condition: &Condition) -> Result<TermId> {
        match condition {
            Condition::Age { operator, value } => {
                let age_var = self.get_or_create_int_var("age");
                self.translate_comparison(age_var, operator, i64::from(*value))
            }

            Condition::Income { operator, value } => {
                let income_var = self.get_or_create_int_var("income");
                self.translate_comparison(income_var, operator, *value as i64)
            }

            Condition::HasAttribute { key } => {
                // For HasAttribute, we create a boolean variable
                let bool_var = self.get_or_create_bool_var(&format!("has_{}", key));
                Ok(bool_var)
            }

            Condition::AttributeEquals { key, value } => {
                // Create an integer variable for the attribute and check equality
                let attr_var = self.get_or_create_int_var(&format!("attr_{}", key));
                // Hash the string value to an integer for comparison
                let hash_value = Self::hash_string(value);
                let hash_term = self.tm.mk_int(BigInt::from(hash_value));
                Ok(self.tm.mk_eq(attr_var, hash_term))
            }

            Condition::DateRange { start, end } => {
                let date_var = self.get_or_create_int_var("date");
                let mut constraints = Vec::new();

                if let Some(start_date) = start {
                    let start_days = Self::date_to_days(start_date);
                    let start_term = self.tm.mk_int(BigInt::from(start_days));
                    constraints.push(self.tm.mk_ge(date_var, start_term));
                }

                if let Some(end_date) = end {
                    let end_days = Self::date_to_days(end_date);
                    let end_term = self.tm.mk_int(BigInt::from(end_days));
                    constraints.push(self.tm.mk_le(date_var, end_term));
                }

                if constraints.is_empty() {
                    Ok(self.tm.mk_true())
                } else {
                    Ok(self.tm.mk_and(constraints))
                }
            }

            Condition::Geographic {
                region_type,
                region_id,
            } => {
                // For geographic checks, we use a boolean variable
                let region_var = self
                    .get_or_create_bool_var(&format!("in_region_{:?}_{}", region_type, region_id));
                Ok(region_var)
            }

            Condition::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => {
                // Encode relationship as a boolean variable
                let target_str = target_entity_id.as_deref().unwrap_or("any");
                let rel_var = self
                    .get_or_create_bool_var(&format!("rel_{:?}_{}", relationship_type, target_str));
                Ok(rel_var)
            }

            Condition::ResidencyDuration { operator, months } => {
                let duration_var = self.get_or_create_int_var("residency_months");
                self.translate_comparison(duration_var, operator, i64::from(*months))
            }

            Condition::Duration {
                operator,
                value,
                unit,
            } => {
                // Convert duration to a normalized unit (e.g., days)
                let normalized_value = match unit {
                    legalis_core::DurationUnit::Days => i64::from(*value),
                    legalis_core::DurationUnit::Weeks => i64::from(*value) * 7,
                    legalis_core::DurationUnit::Months => i64::from(*value) * 30,
                    legalis_core::DurationUnit::Years => i64::from(*value) * 365,
                };
                let duration_var = self.get_or_create_int_var("duration_days");
                self.translate_comparison(duration_var, operator, normalized_value)
            }

            Condition::Percentage {
                operator,
                value,
                context,
            } => {
                // Create a percentage variable specific to the context
                let percentage_var = self.get_or_create_int_var(&format!("percentage_{}", context));
                self.translate_comparison(percentage_var, operator, i64::from(*value))
            }

            Condition::SetMembership {
                attribute,
                values,
                negated,
            } => {
                // For set membership, we create a disjunction of equality checks
                let attr_var = self.get_or_create_int_var(&format!("attr_{}", attribute));
                let mut membership_checks = Vec::new();

                for value in values {
                    let hash_value = Self::hash_string(value);
                    let hash_term = self.tm.mk_int(BigInt::from(hash_value));
                    let eq_check = self.tm.mk_eq(attr_var, hash_term);
                    membership_checks.push(eq_check);
                }

                let membership = if membership_checks.is_empty() {
                    self.tm.mk_false()
                } else if membership_checks.len() == 1 {
                    membership_checks[0]
                } else {
                    self.tm.mk_or(membership_checks)
                };

                if *negated {
                    Ok(self.tm.mk_not(membership))
                } else {
                    Ok(membership)
                }
            }

            Condition::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                // For pattern matching, we use a boolean variable representing the match result
                let pattern_hash = Self::hash_string(&format!("{}:{}", attribute, pattern));
                let pattern_var =
                    self.get_or_create_bool_var(&format!("pattern_match_{}", pattern_hash));

                if *negated {
                    Ok(self.tm.mk_not(pattern_var))
                } else {
                    Ok(pattern_var)
                }
            }

            Condition::Calculation {
                formula,
                operator,
                value,
            } => {
                // For calculations, create a variable representing the formula result
                let calc_var =
                    self.get_or_create_int_var(&format!("calc_{}", Self::hash_string(formula)));
                self.translate_comparison(calc_var, operator, *value as i64)
            }

            Condition::Composite {
                conditions,
                threshold,
            } => {
                // For composite conditions, we need to sum the weights of satisfied conditions
                // and check if the sum meets the threshold.

                if conditions.is_empty() {
                    // No conditions means the composite is always false
                    return Ok(self.tm.mk_false());
                }

                // Create boolean variables for each condition
                let mut sum_terms = Vec::new();
                for (weight, cond) in conditions {
                    let cond_bool = self.translate_condition(cond)?;
                    // Convert weight to integer (scale by 1000 to preserve precision)
                    let weight_val = (*weight * 1000.0) as i64;
                    let weight_int = self.tm.mk_int(BigInt::from(weight_val));
                    let zero = self.tm.mk_int(BigInt::from(0));
                    // If condition is true, add weight; otherwise add 0
                    let term = self.tm.mk_ite(cond_bool, weight_int, zero);
                    sum_terms.push(term);
                }

                // Sum all terms using mk_add which takes an iterator
                let total = self.tm.mk_add(sum_terms);

                // Compare against threshold (scaled)
                let threshold_int = self.tm.mk_int(BigInt::from((*threshold * 1000.0) as i64));
                Ok(self.tm.mk_ge(total, threshold_int))
            }

            Condition::And(left, right) => {
                let left_formula = self.translate_condition(left)?;
                let right_formula = self.translate_condition(right)?;
                Ok(self.tm.mk_and([left_formula, right_formula]))
            }

            Condition::Or(left, right) => {
                let left_formula = self.translate_condition(left)?;
                let right_formula = self.translate_condition(right)?;
                Ok(self.tm.mk_or([left_formula, right_formula]))
            }

            Condition::Not(inner) => {
                let inner_formula = self.translate_condition(inner)?;
                Ok(self.tm.mk_not(inner_formula))
            }

            Condition::Threshold {
                attributes,
                operator,
                value,
            } => {
                // Sum all attribute values with multipliers
                if attributes.is_empty() {
                    return Ok(self.tm.mk_false());
                }

                let mut sum_terms = Vec::new();
                for (attr_name, multiplier) in attributes {
                    let attr_var = self.get_or_create_int_var(&format!("attr_{}", attr_name));
                    // Scale multiplier to integer (multiply by 1000)
                    let mult_val = (*multiplier * 1000.0) as i64;
                    let mult_term = self.tm.mk_int(BigInt::from(mult_val));
                    let scaled = self.tm.mk_mul([attr_var, mult_term]);
                    sum_terms.push(scaled);
                }

                let total = self.tm.mk_add(sum_terms);
                // Scale threshold value similarly
                let threshold_scaled = (*value * 1000.0) as i64;
                self.translate_comparison(total, operator, threshold_scaled)
            }

            Condition::Fuzzy {
                attribute,
                membership_points: _,
                min_membership,
            } => {
                // For fuzzy conditions, we model as a boolean representing
                // whether the membership degree meets the minimum threshold.
                // This is a simplification for SMT purposes.
                let fuzzy_var = self.get_or_create_bool_var(&format!(
                    "fuzzy_{}_{:.2}",
                    Self::hash_string(attribute),
                    min_membership
                ));
                Ok(fuzzy_var)
            }

            Condition::Probabilistic {
                condition,
                probability: _,
                threshold: _,
            } => {
                // For probabilistic conditions, we model the underlying condition.
                // The probability aspect is abstracted away for SMT verification.
                self.translate_condition(condition)
            }

            Condition::Temporal {
                base_value,
                reference_time: _,
                rate: _,
                operator,
                target_value,
            } => {
                // For temporal conditions, we create a variable representing the
                // current value and compare against the target.
                // The time-based decay/growth is abstracted for SMT purposes.
                let temporal_var = self.get_or_create_int_var(&format!(
                    "temporal_{}",
                    Self::hash_string(&base_value.to_string())
                ));
                self.translate_comparison(temporal_var, operator, *target_value as i64)
            }

            Condition::Custom { description } => {
                // Custom conditions are modeled as boolean variables
                let custom_var = self
                    .get_or_create_bool_var(&format!("custom_{}", Self::hash_string(description)));
                Ok(custom_var)
            }
        }
    }

    /// Translates a comparison operation to an OxiZ formula.
    fn translate_comparison(
        &mut self,
        var: TermId,
        op: &ComparisonOp,
        value: i64,
    ) -> Result<TermId> {
        let value_term = self.tm.mk_int(BigInt::from(value));
        let result = match op {
            ComparisonOp::Equal => self.tm.mk_eq(var, value_term),
            ComparisonOp::NotEqual => {
                let eq = self.tm.mk_eq(var, value_term);
                self.tm.mk_not(eq)
            }
            ComparisonOp::LessThan => self.tm.mk_lt(var, value_term),
            ComparisonOp::LessOrEqual => self.tm.mk_le(var, value_term),
            ComparisonOp::GreaterThan => self.tm.mk_gt(var, value_term),
            ComparisonOp::GreaterOrEqual => self.tm.mk_ge(var, value_term),
        };
        Ok(result)
    }

    /// Gets or creates an integer variable.
    fn get_or_create_int_var(&mut self, name: &str) -> TermId {
        if let Some(&var) = self.int_vars.get(name) {
            var
        } else {
            let var = self.tm.mk_var(name, self.tm.sorts.int_sort);
            self.int_vars.insert(name.to_string(), var);
            var
        }
    }

    /// Gets or creates a boolean variable.
    fn get_or_create_bool_var(&mut self, name: &str) -> TermId {
        if let Some(&var) = self.bool_vars.get(name) {
            var
        } else {
            let var = self.tm.mk_var(name, self.tm.sorts.bool_sort);
            self.bool_vars.insert(name.to_string(), var);
            var
        }
    }

    /// Hashes a string to an i64 value for SMT comparison.
    fn hash_string(s: &str) -> i64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish() as i64
    }

    /// Converts a NaiveDate to days since epoch.
    fn date_to_days(date: &NaiveDate) -> i64 {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default();
        (*date - epoch).num_days()
    }

    /// Simplifies a condition using SMT-based techniques.
    ///
    /// Returns the simplified condition and whether any changes were made.
    pub fn simplify(&mut self, condition: &Condition) -> Result<(Condition, bool)> {
        let mut changed = false;
        let simplified = self.simplify_inner(condition, &mut changed)?;
        Ok((simplified, changed))
    }

    fn simplify_inner(&mut self, condition: &Condition, changed: &mut bool) -> Result<Condition> {
        match condition {
            Condition::Not(inner) => {
                // Double negation elimination
                if let Condition::Not(inner_inner) = inner.as_ref() {
                    *changed = true;
                    return self.simplify_inner(inner_inner, changed);
                }
                let simplified_inner = self.simplify_inner(inner, changed)?;
                Ok(Condition::Not(Box::new(simplified_inner)))
            }

            Condition::And(left, right) => {
                let simplified_left = self.simplify_inner(left, changed)?;
                let simplified_right = self.simplify_inner(right, changed)?;

                // Check if one implies the other
                if self.implies(&simplified_left, &simplified_right)? {
                    *changed = true;
                    return Ok(simplified_left);
                }
                if self.implies(&simplified_right, &simplified_left)? {
                    *changed = true;
                    return Ok(simplified_right);
                }

                Ok(Condition::And(
                    Box::new(simplified_left),
                    Box::new(simplified_right),
                ))
            }

            Condition::Or(left, right) => {
                let simplified_left = self.simplify_inner(left, changed)?;
                let simplified_right = self.simplify_inner(right, changed)?;

                // Check if one implies the other
                if self.implies(&simplified_left, &simplified_right)? {
                    *changed = true;
                    return Ok(simplified_right);
                }
                if self.implies(&simplified_right, &simplified_left)? {
                    *changed = true;
                    return Ok(simplified_left);
                }

                Ok(Condition::Or(
                    Box::new(simplified_left),
                    Box::new(simplified_right),
                ))
            }

            // Pass through other conditions unchanged
            other => Ok(other.clone()),
        }
    }

    /// Analyzes the complexity of a condition.
    ///
    /// Returns a complexity score and a list of suggestions for simplification.
    #[must_use]
    pub fn analyze_complexity(&self, condition: &Condition) -> (usize, Vec<String>) {
        let mut complexity = 0;
        let mut suggestions = Vec::new();
        self.analyze_complexity_inner(condition, &mut complexity, &mut suggestions);
        (complexity, suggestions)
    }

    fn analyze_complexity_inner(
        &self,
        condition: &Condition,
        complexity: &mut usize,
        suggestions: &mut Vec<String>,
    ) {
        *complexity += 1;

        match condition {
            Condition::Not(inner) => {
                // Check for double negation
                if let Condition::Not(_) = inner.as_ref() {
                    suggestions.push("Contains double negation that can be simplified".to_string());
                }
                self.analyze_complexity_inner(inner, complexity, suggestions);
            }

            Condition::And(left, right) => {
                self.analyze_complexity_inner(left, complexity, suggestions);
                self.analyze_complexity_inner(right, complexity, suggestions);

                // Check for potentially redundant conditions
                if Self::conditions_similar(left, right) {
                    suggestions
                        .push("Contains similar conditions that might be simplified".to_string());
                }
            }

            Condition::Or(left, right) => {
                self.analyze_complexity_inner(left, complexity, suggestions);
                self.analyze_complexity_inner(right, complexity, suggestions);
            }

            Condition::Composite { conditions, .. } => {
                for (_, cond) in conditions {
                    self.analyze_complexity_inner(cond, complexity, suggestions);
                }
            }

            _ => {}
        }
    }

    fn conditions_similar(cond1: &Condition, cond2: &Condition) -> bool {
        matches!(
            (cond1, cond2),
            (Condition::Age { .. }, Condition::Age { .. })
                | (Condition::Income { .. }, Condition::Income { .. })
        )
    }

    /// Creates a universally quantified formula (forall).
    ///
    /// Checks if the condition holds for all assignments of the specified variables.
    pub fn check_forall(&mut self, var_names: &[&str], condition: &Condition) -> Result<bool> {
        self.reset_solver();

        // Create fresh bound variables for quantification
        for name in var_names {
            self.get_or_create_int_var(name);
        }

        // Translate the condition with the bound variables
        let body = self.translate_condition(condition)?;

        // For forall: check if the negation is unsatisfiable
        let negated_body = self.tm.mk_not(body);
        self.solver.assert(negated_body, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(false),  // Found counterexample
            SolverResult::Unsat => Ok(true), // Valid for all assignments
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Creates an existentially quantified formula (exists).
    ///
    /// Checks if there exists at least one assignment of the specified variables
    /// that satisfies the condition.
    pub fn check_exists(&mut self, var_names: &[&str], condition: &Condition) -> Result<bool> {
        self.reset_solver();

        // Create fresh bound variables for quantification
        for name in var_names {
            self.get_or_create_int_var(name);
        }

        // Translate the condition with the bound variables
        let body = self.translate_condition(condition)?;
        self.solver.assert(body, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(true),    // Exists a satisfying assignment
            SolverResult::Unsat => Ok(false), // No satisfying assignment exists
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Creates or retrieves an array variable (Int -> Int mapping).
    pub fn get_or_create_int_array(&mut self, name: &str) -> TermId {
        if let Some(&arr) = self.int_arrays.get(name) {
            arr
        } else {
            let int_sort = self.tm.sorts.int_sort;
            let array_sort = self.tm.sorts.array(int_sort, int_sort);
            let arr = self.tm.mk_var(name, array_sort);
            self.int_arrays.insert(name.to_string(), arr);
            arr
        }
    }

    /// Asserts that an array element at a specific index has a specific value.
    pub fn assert_array_element(&mut self, array_name: &str, index: i64, value: i64) -> Result<()> {
        let arr = self.get_or_create_int_array(array_name);
        let idx = self.tm.mk_int(BigInt::from(index));
        let val = self.tm.mk_int(BigInt::from(value));

        let select = self.tm.mk_select(arr, idx);
        let constraint = self.tm.mk_eq(select, val);
        self.solver.assert(constraint, &mut self.tm);

        Ok(())
    }

    /// Creates or retrieves a bitvector variable.
    pub fn get_or_create_bitvector(&mut self, name: &str, size: u32) -> TermId {
        let key = format!("{}_{}", name, size);
        if let Some(&bv) = self.bv_vars.get(&key) {
            bv
        } else {
            let bv_sort = self.tm.sorts.bitvec(size);
            let bv = self.tm.mk_var(&key, bv_sort);
            self.bv_vars.insert(key.clone(), bv);

            // Add range constraints for bitvector: 0 <= bv < 2^size
            // Only for small bitvectors (size <= 8) to avoid overflow issues
            if size <= 8 {
                // Lower bound: bv >= 0 (always true for unsigned)
                let zero = self.tm.mk_bitvec(BigInt::from(0), size);
                let lower_bound = self.tm.mk_bv_ule(zero, bv); // 0 <= bv
                self.solver.assert(lower_bound, &mut self.tm);

                // Upper bound: bv <= 2^size - 1
                let max_val = (1u64 << size) - 1; // 2^size - 1
                let upper = self.tm.mk_bitvec(BigInt::from(max_val), size);
                let upper_bound = self.tm.mk_bv_ule(bv, upper); // bv <= max
                self.solver.assert(upper_bound, &mut self.tm);
            }

            bv
        }
    }

    /// Asserts a bitvector comparison constraint.
    pub fn assert_bitvector_constraint(
        &mut self,
        name: &str,
        size: u32,
        operator: ComparisonOp,
        value: u64,
    ) -> Result<()> {
        let bv = self.get_or_create_bitvector(name, size);
        let val_bv = self.tm.mk_bitvec(BigInt::from(value), size);

        let constraint = match operator {
            ComparisonOp::Equal => self.tm.mk_eq(bv, val_bv),
            ComparisonOp::NotEqual => {
                let eq = self.tm.mk_eq(bv, val_bv);
                self.tm.mk_not(eq)
            }
            ComparisonOp::LessThan => self.tm.mk_bv_ult(bv, val_bv),
            ComparisonOp::LessOrEqual => self.tm.mk_bv_ule(bv, val_bv),
            ComparisonOp::GreaterThan => {
                let ule = self.tm.mk_bv_ule(bv, val_bv);
                self.tm.mk_not(ule)
            }
            ComparisonOp::GreaterOrEqual => {
                let ult = self.tm.mk_bv_ult(bv, val_bv);
                self.tm.mk_not(ult)
            }
        };

        self.solver.assert(constraint, &mut self.tm);
        Ok(())
    }

    /// Checks if a bitvector satisfies certain bit patterns.
    pub fn check_bitvector_mask(
        &mut self,
        name: &str,
        size: u32,
        mask: u64,
        expected: u64,
    ) -> Result<bool> {
        self.reset_solver();

        let bv = self.get_or_create_bitvector(name, size);
        let mask_bv = self.tm.mk_bitvec(BigInt::from(mask), size);
        let expected_bv = self.tm.mk_bitvec(BigInt::from(expected), size);

        // Assert: (bv & mask) == expected
        let masked = self.tm.mk_bv_and(bv, mask_bv);
        let constraint = self.tm.mk_eq(masked, expected_bv);
        self.solver.assert(constraint, &mut self.tm);

        match self.solver.check(&mut self.tm) {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }
}

// Convert BigInt method for num-bigint compatibility
trait BigIntExt {
    fn to_i64(&self) -> Option<i64>;
}

impl BigIntExt for BigInt {
    fn to_i64(&self) -> Option<i64> {
        use std::convert::TryFrom;
        i64::try_from(self.clone()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_satisfiability() {
        let mut verifier = SmtVerifier::new();

        // Age >= 18 should be satisfiable
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_contradiction() {
        let mut verifier = SmtVerifier::new();

        // Age >= 65 AND Age < 18 should be unsatisfiable
        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        };

        let cond2 = Condition::Age {
            operator: ComparisonOp::LessThan,
            value: 18,
        };

        assert!(verifier.contradict(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_tautology() {
        let mut verifier = SmtVerifier::new();

        // (Age >= 18) OR (Age < 18) is a tautology
        let condition = Condition::Or(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 18,
            }),
        );

        assert!(verifier.is_tautology(&condition).unwrap());
    }

    #[test]
    fn test_implication() {
        let mut verifier = SmtVerifier::new();

        // Age >= 21 implies Age >= 18
        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        let cond2 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        assert!(verifier.implies(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_not_implication() {
        let mut verifier = SmtVerifier::new();

        // Age >= 18 does NOT imply Age >= 21
        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let cond2 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        assert!(!verifier.implies(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_complex_condition() {
        let mut verifier = SmtVerifier::new();

        // (Age >= 18 AND Age < 65) OR Income > 50000
        let condition = Condition::Or(
            Box::new(Condition::And(
                Box::new(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                }),
                Box::new(Condition::Age {
                    operator: ComparisonOp::LessThan,
                    value: 65,
                }),
            )),
            Box::new(Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 50000,
            }),
        );

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_incremental_solving() {
        let mut verifier = SmtVerifier::new();

        // First assertion
        verifier
            .assert_condition(&Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .unwrap();

        assert!(verifier.check().unwrap());

        // Push new scope
        verifier.push();

        // Add contradicting condition
        verifier
            .assert_condition(&Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 10,
            })
            .unwrap();

        assert!(!verifier.check().unwrap());

        // Pop and check again
        verifier.pop();
        verifier.reset();
        verifier
            .assert_condition(&Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .unwrap();
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_duration_satisfiability() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::Duration {
            operator: ComparisonOp::GreaterOrEqual,
            value: 5,
            unit: legalis_core::DurationUnit::Years,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_duration_contradiction() {
        let mut verifier = SmtVerifier::new();

        let cond1 = Condition::Duration {
            operator: ComparisonOp::GreaterOrEqual,
            value: 10,
            unit: legalis_core::DurationUnit::Years,
        };

        let cond2 = Condition::Duration {
            operator: ComparisonOp::LessThan,
            value: 1,
            unit: legalis_core::DurationUnit::Years,
        };

        let combined = Condition::And(Box::new(cond1), Box::new(cond2));
        assert!(!verifier.is_satisfiable(&combined).unwrap());
    }

    #[test]
    fn test_percentage_satisfiability() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::Percentage {
            operator: ComparisonOp::GreaterOrEqual,
            value: 25,
            context: "ownership".to_string(),
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_percentage_contradiction() {
        let mut verifier = SmtVerifier::new();

        let cond1 = Condition::Percentage {
            operator: ComparisonOp::GreaterThan,
            value: 75,
            context: "ownership".to_string(),
        };

        let cond2 = Condition::Percentage {
            operator: ComparisonOp::LessThan,
            value: 25,
            context: "ownership".to_string(),
        };

        assert!(verifier.contradict(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_set_membership_satisfiability() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec!["active".to_string(), "pending".to_string()],
            negated: false,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_set_membership_empty_set() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec![],
            negated: false,
        };

        assert!(!verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_set_membership_negated_empty_set() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec![],
            negated: true,
        };

        assert!(verifier.is_tautology(&condition).unwrap());
    }

    #[test]
    fn test_pattern_satisfiability() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::Pattern {
            attribute: "employee_id".to_string(),
            pattern: r"^E\d{6}$".to_string(),
            negated: false,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_pattern_contradiction() {
        let mut verifier = SmtVerifier::new();

        let cond1 = Condition::Pattern {
            attribute: "code".to_string(),
            pattern: r"^[A-Z]{3}\d{3}$".to_string(),
            negated: false,
        };

        let cond2 = Condition::Pattern {
            attribute: "code".to_string(),
            pattern: r"^[A-Z]{3}\d{3}$".to_string(),
            negated: true,
        };

        assert!(verifier.contradict(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_condition_equivalence() {
        let mut verifier = SmtVerifier::new();

        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let cond2 = Condition::Not(Box::new(Condition::Age {
            operator: ComparisonOp::LessThan,
            value: 18,
        }));

        assert!(verifier.are_equivalent(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_double_negation_simplification() {
        let mut verifier = SmtVerifier::new();

        let complex = Condition::Not(Box::new(Condition::Not(Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }))));

        let (simplified, changed) = verifier.simplify(&complex).unwrap();
        assert!(changed);

        if let Condition::Age { operator, value } = simplified {
            assert_eq!(operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(value, 18);
        } else {
            panic!("Expected Age condition after simplification");
        }
    }

    #[test]
    fn test_redundant_and_simplification() {
        let mut verifier = SmtVerifier::new();

        let complex = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
        );

        let (simplified, changed) = verifier.simplify(&complex).unwrap();
        assert!(changed);

        if let Condition::Age { operator, value } = simplified {
            assert_eq!(operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(value, 21);
        } else {
            panic!("Expected simplified to age >= 21");
        }
    }

    #[test]
    fn test_complexity_analysis() {
        let verifier = SmtVerifier::new();

        let condition = Condition::Not(Box::new(Condition::Not(Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }))));

        let (complexity, suggestions) = verifier.analyze_complexity(&condition);
        assert!(complexity >= 3);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("double negation"));
    }

    #[test]
    fn test_quantifier_forall_valid() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::Or(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 0,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 0,
            }),
        );

        let result = verifier.check_forall(&["age"], &condition);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_quantifier_forall_invalid() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let result = verifier.check_forall(&["age"], &condition);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_quantifier_exists_satisfiable() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 21,
            }),
        );

        let result = verifier.check_exists(&["age"], &condition);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_quantifier_exists_unsatisfiable() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterThan,
                value: 100,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 50,
            }),
        );

        let result = verifier.check_exists(&["age"], &condition);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_array_basic_operations() {
        let mut verifier = SmtVerifier::new();

        let result = verifier.assert_array_element("test_array", 0, 100);
        assert!(result.is_ok());

        let result = verifier.assert_array_element("test_array", 1, 200);
        assert!(result.is_ok());

        assert!(verifier.check().is_ok());
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_basic_operations() {
        let mut verifier = SmtVerifier::new();

        let result = verifier.assert_bitvector_constraint("flags", 8, ComparisonOp::Equal, 42);
        assert!(result.is_ok());

        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_comparisons() {
        let mut verifier = SmtVerifier::new();

        verifier
            .assert_bitvector_constraint("value", 32, ComparisonOp::GreaterOrEqual, 100)
            .unwrap();
        verifier
            .assert_bitvector_constraint("value", 32, ComparisonOp::LessThan, 200)
            .unwrap();

        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_mask_operation() {
        let mut verifier = SmtVerifier::new();

        let result = verifier.check_bitvector_mask("bv", 16, 0xFF00, 0x1200);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_bitvector_unsatisfiable_constraints() {
        let mut verifier = SmtVerifier::new();

        verifier
            .assert_bitvector_constraint("val", 8, ComparisonOp::GreaterThan, 200)
            .unwrap();
        verifier
            .assert_bitvector_constraint("val", 8, ComparisonOp::LessThan, 50)
            .unwrap();

        assert!(!verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_overflow() {
        let mut verifier = SmtVerifier::new();

        verifier
            .assert_bitvector_constraint("small", 8, ComparisonOp::GreaterThan, 255)
            .unwrap();

        assert!(!verifier.check().unwrap());
    }

    #[test]
    fn test_quantifier_multiple_variables() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 50000,
            }),
        );

        let result = verifier.check_exists(&["age", "income"], &condition);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_explain_unsat() {
        let mut verifier = SmtVerifier::new();

        let conditions = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterThan,
                value: 65,
            },
            Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 18,
            },
        ];

        let explanation = verifier.explain_unsat(&conditions).unwrap();

        assert!(explanation.contains("Unsatisfiability Explanation"));
        assert!(explanation.contains("Total conditions:"));
        assert!(explanation.contains("Core conditions"));
    }

    #[test]
    fn test_calculation_satisfiable() {
        let mut verifier = SmtVerifier::new();

        let condition = Condition::Calculation {
            formula: "income * 0.2".to_string(),
            operator: ComparisonOp::GreaterThan,
            value: 1000.0,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_calculation_contradiction() {
        let mut verifier = SmtVerifier::new();

        let cond1 = Condition::Calculation {
            formula: "net_worth / 12".to_string(),
            operator: ComparisonOp::GreaterThan,
            value: 5000.0,
        };

        let cond2 = Condition::Calculation {
            formula: "net_worth / 12".to_string(),
            operator: ComparisonOp::LessThan,
            value: 1000.0,
        };

        let combined = Condition::And(Box::new(cond1), Box::new(cond2));
        assert!(!verifier.is_satisfiable(&combined).unwrap());
    }
}
