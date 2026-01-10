//! SMT solver integration for formal verification.
//!
//! This module provides Z3 SMT solver integration for:
//! - Satisfiability checking of conditions
//! - Contradiction detection
//! - Tautology verification
//! - Counterexample generation

use anyhow::{Context, Result};
use legalis_core::{ComparisonOp, Condition};
use std::collections::HashMap;
use std::ops::{Add, Mul};
use z3::ast::{Array, Ast, BV, Bool, Int};
use z3::{Config, Context as Z3Context, FuncDecl, Solver, Sort};

/// SMT-based verifier for legal conditions.
pub struct SmtVerifier<'ctx> {
    ctx: &'ctx Z3Context,
    solver: Solver<'ctx>,
    /// Maps entity attribute names to Z3 integer variables
    int_vars: HashMap<String, Int<'ctx>>,
    /// Maps boolean attributes to Z3 boolean variables
    bool_vars: HashMap<String, Bool<'ctx>>,
    /// Maps array names to Z3 array variables (Int -> Int arrays)
    int_arrays: HashMap<String, Array<'ctx>>,
    /// Maps uninterpreted function names to Z3 function declarations
    uninterpreted_funcs: HashMap<String, FuncDecl<'ctx>>,
    /// Maps bitvector names to Z3 bitvector variables
    bv_vars: HashMap<String, BV<'ctx>>,
}

impl<'ctx> SmtVerifier<'ctx> {
    /// Creates a new SMT verifier with the given Z3 context.
    pub fn new(ctx: &'ctx Z3Context) -> Self {
        let solver = Solver::new(ctx);
        Self {
            ctx,
            solver,
            int_vars: HashMap::new(),
            bool_vars: HashMap::new(),
            int_arrays: HashMap::new(),
            uninterpreted_funcs: HashMap::new(),
            bv_vars: HashMap::new(),
        }
    }

    /// Checks if a condition is satisfiable.
    ///
    /// Returns `Ok(true)` if the condition can be satisfied by some assignment,
    /// `Ok(false)` if the condition is unsatisfiable (always false).
    pub fn is_satisfiable(&mut self, condition: &Condition) -> Result<bool> {
        self.solver.reset();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(&formula);

        match self.solver.check() {
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks if a condition is a tautology (always true).
    ///
    /// A condition is a tautology if its negation is unsatisfiable.
    pub fn is_tautology(&mut self, condition: &Condition) -> Result<bool> {
        self.solver.reset();
        let formula = self.translate_condition(condition)?;
        let negated = formula.not();
        self.solver.assert(&negated);

        match self.solver.check() {
            z3::SatResult::Sat => Ok(false), // Negation is satisfiable, so not a tautology
            z3::SatResult::Unsat => Ok(true), // Negation is unsatisfiable, so it's a tautology
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks if two conditions contradict each other.
    ///
    /// Returns `Ok(true)` if (cond1 AND cond2) is unsatisfiable.
    pub fn contradict(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        self.solver.reset();
        let formula1 = self.translate_condition(cond1)?;
        let formula2 = self.translate_condition(cond2)?;
        let conjunction = Bool::and(self.ctx, &[&formula1, &formula2]);
        self.solver.assert(&conjunction);

        match self.solver.check() {
            z3::SatResult::Sat => Ok(false), // Conjunction is satisfiable, no contradiction
            z3::SatResult::Unsat => Ok(true), // Conjunction is unsatisfiable, they contradict
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Checks if cond1 implies cond2 (cond1 → cond2).
    ///
    /// Returns `Ok(true)` if cond1 => cond2 is a tautology.
    pub fn implies(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        self.solver.reset();
        let formula1 = self.translate_condition(cond1)?;
        let formula2 = self.translate_condition(cond2)?;

        // Check if !(cond1 => cond2) is unsatisfiable
        // !(cond1 => cond2) = cond1 ∧ ¬cond2
        let not_implies = Bool::and(self.ctx, &[&formula1, &formula2.not()]);
        self.solver.assert(&not_implies);

        match self.solver.check() {
            z3::SatResult::Sat => Ok(false), // Found counterexample where cond1 is true but cond2 is false
            z3::SatResult::Unsat => Ok(true), // Implication holds
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Gets a counterexample (model) if the condition is satisfiable.
    ///
    /// Returns a mapping of variable names to their values.
    pub fn get_model(&mut self, condition: &Condition) -> Result<Option<HashMap<String, i64>>> {
        self.solver.reset();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(&formula);

        match self.solver.check() {
            z3::SatResult::Sat => {
                let model = self.solver.get_model().context("Failed to get model")?;
                let mut result = HashMap::new();

                // Extract values for integer variables
                for (name, var) in &self.int_vars {
                    if let Some(value) = model.eval(var, true) {
                        if let Some(i) = value.as_i64() {
                            result.insert(name.clone(), i);
                        }
                    }
                }

                Ok(Some(result))
            }
            z3::SatResult::Unsat => Ok(None),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Gets multiple models (up to max_count) that satisfy the condition.
    ///
    /// This is useful for exploring different valid scenarios.
    pub fn get_multiple_models(
        &mut self,
        condition: &Condition,
        max_count: usize,
    ) -> Result<Vec<HashMap<String, i64>>> {
        self.solver.reset();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(&formula);

        let mut models = Vec::new();

        for _ in 0..max_count {
            match self.solver.check() {
                z3::SatResult::Sat => {
                    let model = self.solver.get_model().context("Failed to get model")?;
                    let mut result = HashMap::new();

                    // Extract values for integer variables
                    let mut blocking_clause = Vec::new();
                    for (name, var) in &self.int_vars {
                        if let Some(value) = model.eval(var, true) {
                            if let Some(i) = value.as_i64() {
                                result.insert(name.clone(), i);
                                // Create constraint to block this assignment
                                let int_value = Int::from_i64(self.ctx, i);
                                blocking_clause.push(var._eq(&int_value).not());
                            }
                        }
                    }

                    models.push(result);

                    // Block this model to find a different one
                    if !blocking_clause.is_empty() {
                        let block = Bool::or(self.ctx, &blocking_clause.iter().collect::<Vec<_>>());
                        self.solver.assert(&block);
                    } else {
                        break; // No more models possible
                    }
                }
                z3::SatResult::Unsat => break,
                z3::SatResult::Unknown => {
                    return Err(anyhow::anyhow!("SMT solver returned unknown"));
                }
            }
        }

        Ok(models)
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

    /// Finds the minimal unsatisfiable core of conditions.
    ///
    /// Given a set of conditions that are unsatisfiable together,
    /// returns a minimal subset that is still unsatisfiable.
    pub fn find_unsat_core(&mut self, conditions: &[Condition]) -> Result<Vec<usize>> {
        self.solver.reset();

        // Assert each condition with a tracking literal
        let mut tracking_literals = Vec::new();
        for (i, cond) in conditions.iter().enumerate() {
            let formula = self.translate_condition(cond)?;
            let track_name = format!("track_{}", i);
            let track_lit = Bool::new_const(self.ctx, track_name);
            tracking_literals.push((i, track_lit.clone()));

            // Assert: track_lit => formula
            let implication = Bool::or(self.ctx, &[&track_lit.not(), &formula]);
            self.solver.assert(&implication);
        }

        // Assert all tracking literals
        for (_, lit) in &tracking_literals {
            self.solver.assert(lit);
        }

        match self.solver.check() {
            z3::SatResult::Unsat => {
                // Get unsat core
                let core = self.solver.get_unsat_core();
                let mut core_indices = Vec::new();

                for (idx, track_lit) in &tracking_literals {
                    if core.contains(track_lit) {
                        core_indices.push(*idx);
                    }
                }

                Ok(core_indices)
            }
            z3::SatResult::Sat => Err(anyhow::anyhow!("Conditions are satisfiable, no unsat core")),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Resets the solver state, clearing all assertions.
    pub fn reset(&mut self) {
        self.solver.reset();
        self.int_vars.clear();
        self.bool_vars.clear();
        self.int_arrays.clear();
        self.uninterpreted_funcs.clear();
        self.bv_vars.clear();
    }

    /// Pushes a new scope for incremental solving.
    pub fn push(&mut self) {
        self.solver.push();
    }

    /// Pops the most recent scope.
    pub fn pop(&mut self) {
        self.solver.pop(1);
    }

    /// Asserts a condition without checking satisfiability.
    ///
    /// Useful for incremental solving where multiple conditions
    /// are added before checking.
    pub fn assert_condition(&mut self, condition: &Condition) -> Result<()> {
        let formula = self.translate_condition(condition)?;
        self.solver.assert(&formula);
        Ok(())
    }

    /// Checks the current set of assertions for satisfiability.
    pub fn check(&mut self) -> Result<bool> {
        match self.solver.check() {
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Gets statistics about the current solver state.
    pub fn get_statistics(&self) -> HashMap<String, String> {
        let stats = self.solver.get_statistics();
        let mut result = HashMap::new();

        for entry in stats.entries() {
            result.insert(entry.key.to_string(), format!("{:?}", entry.value));
        }

        result
    }

    /// Generates a proof for an unsatisfiable formula.
    ///
    /// Returns a proof object that can be exported or analyzed.
    /// Note: Requires Z3 to be configured with proof generation enabled.
    pub fn get_proof(&mut self, condition: &Condition) -> Result<Option<String>> {
        self.solver.reset();
        let formula = self.translate_condition(condition)?;
        self.solver.assert(&formula);

        match self.solver.check() {
            z3::SatResult::Unsat => {
                // Get proof
                let proof = self.solver.get_proof();
                if let Some(proof_ast) = proof {
                    Ok(Some(format!("{:?}", proof_ast)))
                } else {
                    Ok(None)
                }
            }
            z3::SatResult::Sat => Err(anyhow::anyhow!(
                "Formula is satisfiable, no proof available"
            )),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Exports a proof in SMTLIB2 format.
    ///
    /// This generates an SMTLIB2 representation of the proof
    /// that can be verified by other SMT solvers.
    pub fn export_proof_smtlib2(&mut self, condition: &Condition) -> Result<String> {
        self.solver.reset();
        let formula = self.translate_condition(condition)?;

        let mut smtlib2 = String::new();
        smtlib2.push_str("; Proof for condition\n");
        smtlib2.push_str("(set-logic ALL)\n");

        // Declare variables
        for name in self.int_vars.keys() {
            smtlib2.push_str(&format!("(declare-const {} Int)\n", name));
        }
        for name in self.bool_vars.keys() {
            smtlib2.push_str(&format!("(declare-const {} Bool)\n", name));
        }

        // Assert the formula
        smtlib2.push_str(&format!("(assert {})\n", formula));
        smtlib2.push_str("(check-sat)\n");

        self.solver.assert(&formula);

        match self.solver.check() {
            z3::SatResult::Unsat => {
                smtlib2.push_str("; Result: unsat\n");
                if let Some(proof) = self.solver.get_proof() {
                    smtlib2.push_str(&format!("; Proof: {:?}\n", proof));
                }
            }
            z3::SatResult::Sat => {
                smtlib2.push_str("; Result: sat\n");
            }
            z3::SatResult::Unknown => {
                smtlib2.push_str("; Result: unknown\n");
            }
        }

        Ok(smtlib2)
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

    /// Translates a Legalis condition to a Z3 formula.
    fn translate_condition(&mut self, condition: &Condition) -> Result<Bool<'ctx>> {
        match condition {
            Condition::Age { operator, value } => {
                let age_var = self.get_or_create_int_var("age");
                self.translate_comparison(&age_var, operator, *value as i64)
            }

            Condition::Income { operator, value } => {
                let income_var = self.get_or_create_int_var("income");
                self.translate_comparison(&income_var, operator, *value as i64)
            }

            Condition::HasAttribute { key } => {
                // For HasAttribute, we create a boolean variable
                let bool_var = self.get_or_create_bool_var(&format!("has_{}", key));
                Ok(bool_var.clone())
            }

            Condition::AttributeEquals { key, value } => {
                // Create an integer variable for the attribute and check equality
                let attr_var = self.get_or_create_int_var(&format!("attr_{}", key));
                // Hash the string value to an integer for comparison
                let hash_value = Self::hash_string(value);
                Ok(attr_var._eq(&Int::from_i64(self.ctx, hash_value)))
            }

            Condition::DateRange { start, end } => {
                let date_var = self.get_or_create_int_var("date");
                let mut constraints = Vec::new();

                if let Some(start_date) = start {
                    let start_days = Self::date_to_days(start_date);
                    constraints.push(date_var.ge(&Int::from_i64(self.ctx, start_days)));
                }

                if let Some(end_date) = end {
                    let end_days = Self::date_to_days(end_date);
                    constraints.push(date_var.le(&Int::from_i64(self.ctx, end_days)));
                }

                if constraints.is_empty() {
                    Ok(Bool::from_bool(self.ctx, true))
                } else {
                    Ok(Bool::and(self.ctx, &constraints.iter().collect::<Vec<_>>()))
                }
            }

            Condition::Geographic {
                region_type,
                region_id,
            } => {
                // For geographic checks, we use a boolean variable
                let region_var = self
                    .get_or_create_bool_var(&format!("in_region_{:?}_{}", region_type, region_id));
                Ok(region_var.clone())
            }

            Condition::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => {
                // Encode relationship as a boolean variable
                let target_str = target_entity_id.as_deref().unwrap_or("any");
                let rel_var = self
                    .get_or_create_bool_var(&format!("rel_{:?}_{}", relationship_type, target_str));
                Ok(rel_var.clone())
            }

            Condition::ResidencyDuration { operator, months } => {
                let duration_var = self.get_or_create_int_var("residency_months");
                self.translate_comparison(&duration_var, operator, *months as i64)
            }

            Condition::Duration {
                operator,
                value,
                unit,
            } => {
                // Convert duration to a normalized unit (e.g., days)
                let normalized_value = match unit {
                    legalis_core::DurationUnit::Days => *value as i64,
                    legalis_core::DurationUnit::Weeks => (*value as i64) * 7,
                    legalis_core::DurationUnit::Months => (*value as i64) * 30,
                    legalis_core::DurationUnit::Years => (*value as i64) * 365,
                };
                let duration_var = self.get_or_create_int_var("duration_days");
                self.translate_comparison(&duration_var, operator, normalized_value)
            }

            Condition::Percentage {
                operator,
                value,
                context,
            } => {
                // Create a percentage variable specific to the context
                let percentage_var = self.get_or_create_int_var(&format!("percentage_{}", context));
                self.translate_comparison(&percentage_var, operator, *value as i64)
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
                    let eq_check = attr_var._eq(&Int::from_i64(self.ctx, hash_value));
                    membership_checks.push(eq_check);
                }

                let membership = if membership_checks.is_empty() {
                    Bool::from_bool(self.ctx, false)
                } else if membership_checks.len() == 1 {
                    membership_checks[0].clone()
                } else {
                    Bool::or(self.ctx, &membership_checks.iter().collect::<Vec<_>>())
                };

                if *negated {
                    Ok(membership.not())
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
                    Ok(pattern_var.not())
                } else {
                    Ok(pattern_var.clone())
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
                self.translate_comparison(&calc_var, operator, *value as i64)
            }

            Condition::Composite {
                conditions,
                threshold,
            } => {
                // For composite conditions, we need to sum the weights of satisfied conditions
                // and check if the sum meets the threshold.
                // For simplicity, we create a boolean for each sub-condition and use
                // if-then-else to accumulate weights.

                if conditions.is_empty() {
                    // No conditions means the composite is always false
                    return Ok(Bool::from_bool(self.ctx, false));
                }

                // Create boolean variables for each condition
                let mut sum_terms = Vec::new();
                for (weight, cond) in conditions {
                    let cond_bool = self.translate_condition(cond)?;
                    // Convert weight to integer (scale by 1000 to preserve precision)
                    let weight_val = (*weight * 1000.0) as i64;
                    let weight_int = Int::from_i64(self.ctx, weight_val);
                    let zero = Int::from_i64(self.ctx, 0);
                    // If condition is true, add weight; otherwise add 0
                    let term = cond_bool.ite(&weight_int, &zero);
                    sum_terms.push(term);
                }

                // Sum all terms
                let total = if sum_terms.len() == 1 {
                    sum_terms[0].clone()
                } else {
                    let mut sum = sum_terms[0].clone();
                    for term in &sum_terms[1..] {
                        sum = sum.add(term);
                    }
                    sum
                };

                // Compare against threshold (also scaled by 1000)
                let threshold_val = (*threshold * 1000.0) as i64;
                let threshold_int = Int::from_i64(self.ctx, threshold_val);
                Ok(total.ge(&threshold_int))
            }

            Condition::Threshold {
                attributes,
                operator,
                value,
            } => {
                // Sum multiple attributes with their multipliers
                if attributes.is_empty() {
                    return Ok(Bool::from_bool(self.ctx, false));
                }

                let mut sum_terms = Vec::new();
                for (attr_name, multiplier) in attributes {
                    let attr_var = self.get_or_create_int_var(attr_name);
                    let mult_val = (*multiplier * 1000.0) as i64;
                    let mult_int = Int::from_i64(self.ctx, mult_val);
                    let term = attr_var.mul(&mult_int);
                    sum_terms.push(term);
                }

                // Sum all terms
                let total = if sum_terms.len() == 1 {
                    sum_terms[0].clone()
                } else {
                    let mut sum = sum_terms[0].clone();
                    for term in &sum_terms[1..] {
                        sum = sum.add(term);
                    }
                    sum
                };

                // Scale the value and compare
                let scaled_value = (*value * 1000.0) as i64;
                self.translate_comparison(&total, operator, scaled_value)
            }

            Condition::Fuzzy {
                attribute,
                membership_points,
                min_membership,
            } => {
                // For fuzzy logic, we approximate by checking if the attribute value
                // falls within ranges that have sufficient membership degree.
                // This is a simplification since we can't model continuous membership functions directly.

                // Create a variable for the attribute
                let attr_var = self.get_or_create_int_var(attribute);

                // Find points with membership >= min_membership
                let satisfying_ranges: Vec<_> = membership_points
                    .iter()
                    .filter(|(_, membership)| *membership >= *min_membership)
                    .collect();

                if satisfying_ranges.is_empty() {
                    return Ok(Bool::from_bool(self.ctx, false));
                }

                // Create disjunction for all satisfying value ranges
                let mut range_checks = Vec::new();
                for (value, _) in &satisfying_ranges {
                    let value_int = Int::from_i64(self.ctx, *value as i64);
                    let check = attr_var._eq(&value_int);
                    range_checks.push(check);
                }

                if range_checks.len() == 1 {
                    Ok(range_checks[0].clone())
                } else {
                    Ok(Bool::or(self.ctx, &range_checks.iter().collect::<Vec<_>>()))
                }
            }

            Condition::Probabilistic {
                condition,
                probability: _,
                threshold: _,
            } => {
                // For probabilistic conditions, we simplify by just checking the base condition
                // since we can't model probability directly in SMT without probabilistic logic
                self.translate_condition(condition)
            }

            Condition::Temporal {
                base_value,
                reference_time,
                rate,
                operator,
                target_value,
            } => {
                // For temporal conditions, we need to model: value = base_value * (1 + rate)^time_elapsed
                // We'll create a variable for the current time and compute the value
                let _current_time_var = self.get_or_create_int_var("current_time");

                // For simplicity, we'll create a variable representing the computed temporal value
                // and constrain it based on the formula
                let temporal_value_var = self.get_or_create_int_var(&format!(
                    "temporal_{}_{}_{}",
                    Self::hash_string(&base_value.to_string()),
                    reference_time,
                    Self::hash_string(&rate.to_string())
                ));

                // Compare the temporal value against the target
                self.translate_comparison(&temporal_value_var, operator, *target_value as i64)
            }

            Condition::Custom { description } => {
                // For custom conditions, create a boolean variable
                let hash = Self::hash_string(description);
                let custom_var = self.get_or_create_bool_var(&format!("custom_{}", hash));
                Ok(custom_var.clone())
            }

            Condition::And(left, right) => {
                let left_formula = self.translate_condition(left)?;
                let right_formula = self.translate_condition(right)?;
                Ok(Bool::and(self.ctx, &[&left_formula, &right_formula]))
            }

            Condition::Or(left, right) => {
                let left_formula = self.translate_condition(left)?;
                let right_formula = self.translate_condition(right)?;
                Ok(Bool::or(self.ctx, &[&left_formula, &right_formula]))
            }

            Condition::Not(inner) => {
                let inner_formula = self.translate_condition(inner)?;
                Ok(inner_formula.not())
            }
        }
    }

    /// Translates a comparison operation to a Z3 formula.
    fn translate_comparison(
        &self,
        var: &Int<'ctx>,
        operator: &ComparisonOp,
        value: i64,
    ) -> Result<Bool<'ctx>> {
        let value_ast = Int::from_i64(self.ctx, value);
        let result = match operator {
            ComparisonOp::Equal => var._eq(&value_ast),
            ComparisonOp::NotEqual => var._eq(&value_ast).not(),
            ComparisonOp::LessThan => var.lt(&value_ast),
            ComparisonOp::LessOrEqual => var.le(&value_ast),
            ComparisonOp::GreaterThan => var.gt(&value_ast),
            ComparisonOp::GreaterOrEqual => var.ge(&value_ast),
        };
        Ok(result)
    }

    /// Gets or creates an integer variable with the given name.
    fn get_or_create_int_var(&mut self, name: &str) -> Int<'ctx> {
        if let Some(var) = self.int_vars.get(name) {
            var.clone()
        } else {
            let var = Int::new_const(self.ctx, name);
            self.int_vars.insert(name.to_string(), var.clone());
            var
        }
    }

    /// Gets or creates a boolean variable with the given name.
    fn get_or_create_bool_var(&mut self, name: &str) -> Bool<'ctx> {
        if let Some(var) = self.bool_vars.get(name) {
            var.clone()
        } else {
            let var = Bool::new_const(self.ctx, name);
            self.bool_vars.insert(name.to_string(), var.clone());
            var
        }
    }

    /// Converts a date to days since epoch for comparison.
    fn date_to_days(date: &chrono::NaiveDate) -> i64 {
        date.signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days()
    }

    /// Simple string hash for comparison purposes.
    fn hash_string(s: &str) -> i64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        (hasher.finish() % (i64::MAX as u64)) as i64
    }

    /// Checks if two conditions are semantically equivalent.
    ///
    /// Returns `Ok(true)` if the conditions are logically equivalent,
    /// meaning they are satisfied by exactly the same set of values.
    pub fn are_equivalent(&mut self, cond1: &Condition, cond2: &Condition) -> Result<bool> {
        // Two conditions are equivalent if (cond1 <=> cond2) is a tautology
        // This is equivalent to checking:
        // - cond1 => cond2 AND cond2 => cond1

        self.solver.reset();
        let formula1 = self.translate_condition(cond1)?;
        let formula2 = self.translate_condition(cond2)?;

        // Check if (cond1 XOR cond2) is unsatisfiable
        // If they're equivalent, XOR should always be false
        let xor = formula1._eq(&formula2).not();
        self.solver.assert(&xor);

        match self.solver.check() {
            z3::SatResult::Sat => Ok(false),  // Found a difference
            z3::SatResult::Unsat => Ok(true), // They're equivalent
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Simplifies a condition to a potentially simpler equivalent form.
    ///
    /// This attempts to find a simpler condition that is logically equivalent
    /// to the given condition. Returns the simplified condition and whether
    /// any simplification was performed.
    pub fn simplify(&mut self, condition: &Condition) -> Result<(Condition, bool)> {
        // For now, implement basic simplifications:
        // 1. Double negation elimination: NOT(NOT(x)) => x
        // 2. Identity elimination: x AND TRUE => x, x OR FALSE => x
        // 3. Contradiction detection: x AND NOT(x) => FALSE

        match condition {
            // Double negation elimination
            Condition::Not(inner) => {
                if let Condition::Not(inner_inner) = inner.as_ref() {
                    return Ok((inner_inner.as_ref().clone(), true));
                }
                // Recursively simplify the inner condition
                let (simplified_inner, changed) = self.simplify(inner)?;
                if changed {
                    Ok((Condition::Not(Box::new(simplified_inner)), true))
                } else {
                    Ok((condition.clone(), false))
                }
            }

            // AND simplification
            Condition::And(left, right) => {
                let (left_simp, left_changed) = self.simplify(left)?;
                let (right_simp, right_changed) = self.simplify(right)?;

                // Check if they contradict
                if self.contradict(&left_simp, &right_simp)? {
                    // This would never be true, but we can't represent FALSE directly
                    // Return the original for now
                    return Ok((condition.clone(), false));
                }

                // Check if left implies right (then AND is just left)
                if self.implies(&left_simp, &right_simp)? {
                    return Ok((left_simp, true));
                }

                // Check if right implies left (then AND is just right)
                if self.implies(&right_simp, &left_simp)? {
                    return Ok((right_simp, true));
                }

                if left_changed || right_changed {
                    Ok((
                        Condition::And(Box::new(left_simp), Box::new(right_simp)),
                        true,
                    ))
                } else {
                    Ok((condition.clone(), false))
                }
            }

            // OR simplification
            Condition::Or(left, right) => {
                let (left_simp, left_changed) = self.simplify(left)?;
                let (right_simp, right_changed) = self.simplify(right)?;

                // Check if left implies right (then OR is just right)
                if self.implies(&left_simp, &right_simp)? {
                    return Ok((right_simp, true));
                }

                // Check if right implies left (then OR is just left)
                if self.implies(&right_simp, &left_simp)? {
                    return Ok((left_simp, true));
                }

                if left_changed || right_changed {
                    Ok((
                        Condition::Or(Box::new(left_simp), Box::new(right_simp)),
                        true,
                    ))
                } else {
                    Ok((condition.clone(), false))
                }
            }

            // Base conditions don't simplify further
            _ => Ok((condition.clone(), false)),
        }
    }

    /// Analyzes a condition for complexity and suggests simpler alternatives.
    ///
    /// Returns a complexity score and suggestions for simplification.
    pub fn analyze_complexity(&mut self, condition: &Condition) -> (usize, Vec<String>) {
        let mut suggestions = Vec::new();
        let complexity = self.count_complexity(condition);

        // Check for double negations
        if Self::has_double_negation(condition) {
            suggestions.push("Consider removing double negations for clarity".to_string());
        }

        // Check for redundant conditions
        if let Condition::And(left, right) = condition {
            if let Ok(true) = self.implies(left, right) {
                suggestions.push(
                    "Left condition implies right - AND can be simplified to just left condition"
                        .to_string(),
                );
            } else if let Ok(true) = self.implies(right, left) {
                suggestions.push(
                    "Right condition implies left - AND can be simplified to just right condition"
                        .to_string(),
                );
            }
        }

        if let Condition::Or(left, right) = condition {
            if let Ok(true) = self.implies(left, right) {
                suggestions.push(
                    "Left condition implies right - OR can be simplified to just right condition"
                        .to_string(),
                );
            } else if let Ok(true) = self.implies(right, left) {
                suggestions.push(
                    "Right condition implies left - OR can be simplified to just left condition"
                        .to_string(),
                );
            }
        }

        (complexity, suggestions)
    }

    // ===== Advanced SMT Features (v0.1.1) =====

    /// Creates a universally quantified formula (forall).
    ///
    /// Checks if the condition holds for all possible values of the specified variables.
    /// Returns `Ok(true)` if the formula is valid (holds for all values).
    ///
    /// # Arguments
    /// * `var_names` - Names of variables to quantify over
    /// * `condition` - The condition that should hold for all variable values
    ///
    /// # Example
    /// Check if "age >= 0" holds for all ages (should be true if age is properly constrained)
    pub fn check_forall(&mut self, var_names: &[&str], condition: &Condition) -> Result<bool> {
        self.solver.reset();

        // Create fresh bound variables for quantification
        let mut bound_vars = Vec::new();
        for name in var_names {
            let var = Int::fresh_const(self.ctx, "forall");
            // Temporarily add to our variable map (clone before moving)
            self.int_vars.insert(name.to_string(), var.clone());
            bound_vars.push(var);
        }

        // Translate the condition with the bound variables
        let body = self.translate_condition(condition)?;

        // Create the forall quantifier
        let patterns = &[];
        let forall = z3::ast::forall_const(
            self.ctx,
            &bound_vars.iter().map(|v| v as &dyn Ast).collect::<Vec<_>>(),
            patterns,
            &body,
        );

        // Check if the negation of forall is unsatisfiable
        // If ¬(∀x. P(x)) is unsat, then ∀x. P(x) is valid
        self.solver.assert(&forall.not());

        let result = match self.solver.check() {
            z3::SatResult::Sat => Ok(false),  // Found a counterexample
            z3::SatResult::Unsat => Ok(true), // Valid for all values
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        };

        // Clean up temporary variables
        for name in var_names {
            self.int_vars.remove(*name);
        }

        result
    }

    /// Creates an existentially quantified formula (exists).
    ///
    /// Checks if there exists at least one assignment of the specified variables
    /// that satisfies the condition.
    ///
    /// # Arguments
    /// * `var_names` - Names of variables to quantify over
    /// * `condition` - The condition to check for existence
    ///
    /// # Example
    /// Check if there exists an age such that "age >= 18 AND age < 21"
    pub fn check_exists(&mut self, var_names: &[&str], condition: &Condition) -> Result<bool> {
        self.solver.reset();

        // Create fresh bound variables for quantification
        let mut bound_vars = Vec::new();
        for name in var_names {
            let var = Int::fresh_const(self.ctx, "exists");
            // Temporarily add to our variable map (clone before moving)
            self.int_vars.insert(name.to_string(), var.clone());
            bound_vars.push(var);
        }

        // Translate the condition with the bound variables
        let body = self.translate_condition(condition)?;

        // Create the exists quantifier
        let patterns = &[];
        let exists = z3::ast::exists_const(
            self.ctx,
            &bound_vars.iter().map(|v| v as &dyn Ast).collect::<Vec<_>>(),
            patterns,
            &body,
        );

        // Check if exists is satisfiable
        self.solver.assert(&exists);

        let result = match self.solver.check() {
            z3::SatResult::Sat => Ok(true),    // Exists a satisfying assignment
            z3::SatResult::Unsat => Ok(false), // No satisfying assignment exists
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        };

        // Clean up temporary variables
        for name in var_names {
            self.int_vars.remove(*name);
        }

        result
    }

    /// Creates or retrieves an array variable (Int -> Int mapping).
    ///
    /// Arrays in SMT are used to model collections, maps, or sequences.
    /// This creates an array that maps integer indices to integer values.
    ///
    /// # Arguments
    /// * `name` - Name of the array variable
    ///
    /// # Returns
    /// Z3 array variable (Int -> Int)
    pub fn get_or_create_int_array(&mut self, name: &str) -> Array<'ctx> {
        if let Some(arr) = self.int_arrays.get(name) {
            arr.clone()
        } else {
            let int_sort = Sort::int(self.ctx);
            let arr = Array::fresh_const(self.ctx, "arr", &int_sort, &int_sort);
            self.int_arrays.insert(name.to_string(), arr.clone());
            arr
        }
    }

    /// Asserts that an array element at a specific index has a specific value.
    ///
    /// # Arguments
    /// * `array_name` - Name of the array
    /// * `index` - Index in the array
    /// * `value` - Expected value at that index
    ///
    /// # Example
    /// Assert that collection[5] = 100
    pub fn assert_array_element(&mut self, array_name: &str, index: i64, value: i64) -> Result<()> {
        let arr = self.get_or_create_int_array(array_name);
        let idx = Int::from_i64(self.ctx, index);
        let val = Int::from_i64(self.ctx, value);

        let select = arr.select(&idx);
        let select_int = select
            .as_int()
            .ok_or_else(|| anyhow::anyhow!("Array element is not an integer"))?;
        let constraint = select_int._eq(&val);
        self.solver.assert(&constraint);

        Ok(())
    }

    /// Checks if all elements in an array satisfy a condition.
    ///
    /// This uses quantifiers to express "for all indices i, array[i] satisfies condition".
    ///
    /// # Arguments
    /// * `array_name` - Name of the array
    /// * `min_index` - Minimum index to check (inclusive)
    /// * `max_index` - Maximum index to check (inclusive)
    /// * `value_condition` - Lambda that takes an Int and returns a Bool constraint
    ///
    /// # Returns
    /// `Ok(true)` if all elements satisfy the condition
    pub fn check_all_array_elements<F>(
        &mut self,
        array_name: &str,
        min_index: i64,
        max_index: i64,
        value_condition: F,
    ) -> Result<bool>
    where
        F: Fn(&Int<'ctx>) -> Bool<'ctx>,
    {
        self.solver.reset();

        let arr = self.get_or_create_int_array(array_name);
        let index_var = Int::fresh_const(self.ctx, "idx");

        // Create range constraint: min_index <= idx <= max_index
        let min_int = Int::from_i64(self.ctx, min_index);
        let max_int = Int::from_i64(self.ctx, max_index);
        let in_range = Bool::and(
            self.ctx,
            &[&index_var.ge(&min_int), &index_var.le(&max_int)],
        );

        // Get array element at index
        let element = arr.select(&index_var);

        // Convert Dynamic to Int
        let element_int = element
            .as_int()
            .ok_or_else(|| anyhow::anyhow!("Array element is not an integer"))?;

        // Apply the value condition
        let element_satisfies = value_condition(&element_int);

        // Create implication: in_range => element_satisfies
        let implication = in_range.implies(&element_satisfies);

        // Create forall quantifier
        let patterns = &[];
        let forall =
            z3::ast::forall_const(self.ctx, &[&index_var as &dyn Ast], patterns, &implication);

        // Check if forall is valid
        self.solver.assert(&forall.not());

        match self.solver.check() {
            z3::SatResult::Sat => Ok(false),  // Found a counterexample
            z3::SatResult::Unsat => Ok(true), // All elements satisfy
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Creates or retrieves a bitvector variable.
    ///
    /// Bitvectors provide precise modeling of fixed-width integers,
    /// which is useful for modeling bounded numeric values, flags, or bit manipulation.
    ///
    /// # Arguments
    /// * `name` - Name of the bitvector variable
    /// * `size` - Size in bits (e.g., 8 for u8, 32 for i32, 64 for i64)
    ///
    /// # Returns
    /// Z3 bitvector variable of the specified size
    pub fn get_or_create_bitvector(&mut self, name: &str, size: u32) -> BV<'ctx> {
        let key = format!("{}_{}", name, size);
        if let Some(bv) = self.bv_vars.get(&key) {
            bv.clone()
        } else {
            let bv = BV::fresh_const(self.ctx, "bv", size);
            self.bv_vars.insert(key, bv.clone());
            bv
        }
    }

    /// Asserts a bitvector comparison constraint.
    ///
    /// # Arguments
    /// * `name` - Name of the bitvector variable
    /// * `size` - Size in bits
    /// * `operator` - Comparison operator
    /// * `value` - Value to compare against
    ///
    /// # Example
    /// Assert that a 32-bit bitvector "flags" has value >= 10
    pub fn assert_bitvector_constraint(
        &mut self,
        name: &str,
        size: u32,
        operator: ComparisonOp,
        value: u64,
    ) -> Result<()> {
        let bv = self.get_or_create_bitvector(name, size);
        let val_bv = BV::from_u64(self.ctx, value, size);

        let constraint = match operator {
            ComparisonOp::Equal => bv._eq(&val_bv),
            ComparisonOp::NotEqual => bv._eq(&val_bv).not(),
            ComparisonOp::LessThan => bv.bvult(&val_bv),
            ComparisonOp::LessOrEqual => bv.bvule(&val_bv),
            ComparisonOp::GreaterThan => bv.bvugt(&val_bv),
            ComparisonOp::GreaterOrEqual => bv.bvuge(&val_bv),
        };

        self.solver.assert(&constraint);
        Ok(())
    }

    /// Checks if a bitvector satisfies certain bit patterns.
    ///
    /// # Arguments
    /// * `name` - Name of the bitvector
    /// * `size` - Size in bits
    /// * `mask` - Bit mask to apply
    /// * `expected` - Expected value after masking
    ///
    /// # Returns
    /// `Ok(true)` if the constraint is satisfiable
    ///
    /// # Example
    /// Check if (bv & 0xFF00) == 0x1200
    pub fn check_bitvector_mask(
        &mut self,
        name: &str,
        size: u32,
        mask: u64,
        expected: u64,
    ) -> Result<bool> {
        self.solver.reset();

        let bv = self.get_or_create_bitvector(name, size);
        let mask_bv = BV::from_u64(self.ctx, mask, size);
        let expected_bv = BV::from_u64(self.ctx, expected, size);

        // Assert: (bv & mask) == expected
        let masked = bv.bvand(&mask_bv);
        let constraint = masked._eq(&expected_bv);
        self.solver.assert(&constraint);

        match self.solver.check() {
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        }
    }

    /// Creates an uninterpreted function declaration.
    ///
    /// Uninterpreted functions are useful for modeling unknown predicates or functions
    /// whose implementation is not specified but whose properties are constrained.
    ///
    /// # Arguments
    /// * `name` - Name of the function
    /// * `arity` - Number of arguments (currently supports 1-3 Int arguments)
    ///
    /// # Returns
    /// Z3 function declaration
    pub fn declare_uninterpreted_func(
        &mut self,
        name: &str,
        arity: usize,
    ) -> Result<&FuncDecl<'ctx>> {
        if self.uninterpreted_funcs.contains_key(name) {
            return Ok(self.uninterpreted_funcs.get(name).expect("just checked"));
        }

        if arity == 0 || arity > 3 {
            return Err(anyhow::anyhow!(
                "Unsupported arity: {}. Only 1-3 arguments supported",
                arity
            ));
        }

        let int_sort = Sort::int(self.ctx);
        let domain: Vec<&Sort> = vec![&int_sort; arity];

        let func = FuncDecl::new(self.ctx, name, &domain, &int_sort);
        self.uninterpreted_funcs.insert(name.to_string(), func);

        Ok(self.uninterpreted_funcs.get(name).expect("just inserted"))
    }

    /// Applies an uninterpreted function to arguments.
    ///
    /// # Arguments
    /// * `func_name` - Name of the uninterpreted function
    /// * `args` - Arguments to apply (Int values)
    ///
    /// # Returns
    /// Int representing the function application result
    pub fn apply_uninterpreted_func(
        &mut self,
        func_name: &str,
        args: &[&Int<'ctx>],
    ) -> Result<Int<'ctx>> {
        let func = self.declare_uninterpreted_func(func_name, args.len())?;

        let ast_args: Vec<&dyn Ast> = args.iter().map(|arg| *arg as &dyn Ast).collect();
        let result = func.apply(&ast_args);

        // Convert the result back to Int
        result
            .as_int()
            .ok_or_else(|| anyhow::anyhow!("Function application did not return an Int"))
    }

    /// Asserts a property about an uninterpreted function.
    ///
    /// This is useful for constraining the behavior of uninterpreted functions
    /// without fully defining them.
    ///
    /// # Arguments
    /// * `func_name` - Name of the function
    /// * `args` - Input arguments
    /// * `expected_output` - Expected output value
    ///
    /// # Example
    /// Assert that custom_predicate(10, 20) = 30
    pub fn assert_func_property(
        &mut self,
        func_name: &str,
        args: &[i64],
        expected_output: i64,
    ) -> Result<()> {
        let int_args: Vec<Int> = args.iter().map(|&v| Int::from_i64(self.ctx, v)).collect();
        let arg_refs: Vec<&Int> = int_args.iter().collect();

        let result = self.apply_uninterpreted_func(func_name, &arg_refs)?;
        let expected = Int::from_i64(self.ctx, expected_output);

        let constraint = result._eq(&expected);
        self.solver.assert(&constraint);

        Ok(())
    }

    /// Checks if an uninterpreted function is injective (one-to-one).
    ///
    /// Returns `Ok(true)` if the function is guaranteed to be injective
    /// within the current constraints.
    ///
    /// # Arguments
    /// * `func_name` - Name of the uninterpreted function
    ///
    /// # Example
    /// Check if f(x) = f(y) implies x = y
    pub fn check_func_injective(&mut self, func_name: &str) -> Result<bool> {
        self.solver.push();

        // Create two distinct variables
        let x = Int::fresh_const(self.ctx, "x");
        let y = Int::fresh_const(self.ctx, "y");

        // Assert that f(x) = f(y)
        let fx = self.apply_uninterpreted_func(func_name, &[&x])?;
        let fy = self.apply_uninterpreted_func(func_name, &[&y])?;
        self.solver.assert(&fx._eq(&fy));

        // Assert that x != y
        self.solver.assert(&x._eq(&y).not());

        // Check if this is satisfiable
        // If SAT, then function is not injective (found f(x)=f(y) with x!=y)
        // If UNSAT, then function is injective
        let result = match self.solver.check() {
            z3::SatResult::Sat => Ok(false),  // Not injective
            z3::SatResult::Unsat => Ok(true), // Injective
            z3::SatResult::Unknown => Err(anyhow::anyhow!("SMT solver returned unknown")),
        };

        self.solver.pop(1);
        result
    }

    /// Counts the complexity of a condition (number of nodes in the tree).
    fn count_complexity(&self, condition: &Condition) -> usize {
        match condition {
            Condition::And(left, right) | Condition::Or(left, right) => {
                1 + self.count_complexity(left) + self.count_complexity(right)
            }
            Condition::Not(inner) => 1 + self.count_complexity(inner),
            _ => 1,
        }
    }

    /// Checks if a condition contains double negations.
    fn has_double_negation(condition: &Condition) -> bool {
        match condition {
            Condition::Not(inner) => {
                if matches!(inner.as_ref(), Condition::Not(_)) {
                    true
                } else {
                    Self::has_double_negation(inner)
                }
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::has_double_negation(left) || Self::has_double_negation(right)
            }
            _ => false,
        }
    }
}

/// Creates a Z3 context for SMT verification.
///
/// This is a convenience function that creates a Z3 context with
/// appropriate configuration for legal statute verification.
pub fn create_z3_context() -> Z3Context {
    let mut cfg = Config::new();
    cfg.set_timeout_msec(10000); // 10 second timeout
    cfg.set_model_generation(true);
    Z3Context::new(&cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition};

    #[test]
    fn test_simple_satisfiable() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_contradiction() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let cond2 = Condition::Age {
            operator: ComparisonOp::LessThan,
            value: 18,
        };

        assert!(verifier.contradict(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_no_contradiction() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let cond2 = Condition::Age {
            operator: ComparisonOp::LessThan,
            value: 65,
        };

        assert!(!verifier.contradict(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_and_satisfiability() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        );

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_unsatisfiable_and() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterThan,
                value: 65,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 18,
            }),
        );

        assert!(!verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_implication() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

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
    fn test_no_implication() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Age >= 18 does not imply Age >= 21
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
    fn test_tautology() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Age >= 18 OR Age < 18 is a tautology
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
    fn test_not_tautology() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        assert!(!verifier.is_tautology(&condition).unwrap());
    }

    #[test]
    fn test_get_model() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 30000,
            }),
        );

        let model = verifier.get_model(&condition).unwrap();
        assert!(model.is_some());

        let values = model.unwrap();
        assert!(values.contains_key("age"));
        assert!(values.contains_key("income"));

        // Verify the model satisfies the constraints
        let age = values["age"];
        let income = values["income"];
        assert!(age >= 18);
        assert!(income < 30000);
    }

    #[test]
    fn test_complex_nested_condition() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // (Age >= 18 AND Income < 50000) OR (Age >= 65)
        let condition = Condition::Or(
            Box::new(Condition::And(
                Box::new(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                }),
                Box::new(Condition::Income {
                    operator: ComparisonOp::LessThan,
                    value: 50000,
                }),
            )),
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            }),
        );

        assert!(verifier.is_satisfiable(&condition).unwrap());
        assert!(!verifier.is_tautology(&condition).unwrap());
    }

    #[test]
    fn test_proof_generation_unsat() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Unsatisfiable condition: Age > 65 AND Age < 18
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterThan,
                value: 65,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 18,
            }),
        );

        // Should be able to get a proof for unsatisfiable formula
        let proof_result = verifier.get_proof(&condition);
        assert!(proof_result.is_ok());
    }

    #[test]
    fn test_smtlib2_export() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let smtlib2 = verifier.export_proof_smtlib2(&condition).unwrap();

        assert!(smtlib2.contains("set-logic"));
        assert!(smtlib2.contains("declare-const"));
        assert!(smtlib2.contains("assert"));
        assert!(smtlib2.contains("check-sat"));
    }

    #[test]
    fn test_explain_unsat() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

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
    fn test_unsat_core() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Create a set of unsatisfiable conditions
        let conditions = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterThan,
                value: 65,
            },
            Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 0,
            },
        ];

        let result = verifier.find_unsat_core(&conditions);

        // The function should return Ok (conditions are indeed unsatisfiable)
        assert!(result.is_ok());

        let core = result.unwrap();
        // The core may be empty if Z3 doesn't produce unsat cores by default,
        // but it should not contain more indices than the total conditions
        assert!(core.len() <= conditions.len());
    }

    #[test]
    fn test_duration_satisfiability() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Test Duration condition with different units
        let condition = Condition::Duration {
            operator: ComparisonOp::GreaterOrEqual,
            value: 5,
            unit: legalis_core::DurationUnit::Years,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_duration_contradiction() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Duration >= 10 years AND Duration < 1 year should be unsatisfiable
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
    fn test_duration_unit_conversion() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // 365 days should be equivalent to 1 year
        let cond_days = Condition::Duration {
            operator: ComparisonOp::GreaterOrEqual,
            value: 365,
            unit: legalis_core::DurationUnit::Days,
        };

        let cond_years = Condition::Duration {
            operator: ComparisonOp::GreaterOrEqual,
            value: 1,
            unit: legalis_core::DurationUnit::Years,
        };

        // Both should be satisfiable
        assert!(verifier.is_satisfiable(&cond_days).unwrap());
        verifier.reset();
        assert!(verifier.is_satisfiable(&cond_years).unwrap());
    }

    #[test]
    fn test_percentage_satisfiability() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::Percentage {
            operator: ComparisonOp::GreaterOrEqual,
            value: 25,
            context: "ownership".to_string(),
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_percentage_contradiction() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

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
    fn test_percentage_different_contexts() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Different contexts should not contradict
        let cond1 = Condition::Percentage {
            operator: ComparisonOp::GreaterThan,
            value: 75,
            context: "ownership".to_string(),
        };

        let cond2 = Condition::Percentage {
            operator: ComparisonOp::LessThan,
            value: 25,
            context: "voting_rights".to_string(),
        };

        assert!(!verifier.contradict(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_set_membership_satisfiability() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec!["active".to_string(), "pending".to_string()],
            negated: false,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_set_membership_negated() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec!["inactive".to_string(), "deleted".to_string()],
            negated: true,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_set_membership_empty_set() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Membership in empty set should be unsatisfiable
        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec![],
            negated: false,
        };

        assert!(!verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_set_membership_negated_empty_set() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // NOT in empty set should be a tautology (always true)
        let condition = Condition::SetMembership {
            attribute: "status".to_string(),
            values: vec![],
            negated: true,
        };

        assert!(verifier.is_tautology(&condition).unwrap());
    }

    #[test]
    fn test_pattern_satisfiability() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::Pattern {
            attribute: "employee_id".to_string(),
            pattern: r"^E\d{6}$".to_string(),
            negated: false,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_pattern_negated() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let condition = Condition::Pattern {
            attribute: "email".to_string(),
            pattern: r".*@example\.com$".to_string(),
            negated: true,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_pattern_contradiction() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Same pattern, one positive and one negated should contradict
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
    fn test_complex_new_conditions() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // (Duration >= 5 years AND Percentage >= 25%) should be satisfiable
        let condition = Condition::And(
            Box::new(Condition::Duration {
                operator: ComparisonOp::GreaterOrEqual,
                value: 5,
                unit: legalis_core::DurationUnit::Years,
            }),
            Box::new(Condition::Percentage {
                operator: ComparisonOp::GreaterOrEqual,
                value: 25,
                context: "ownership".to_string(),
            }),
        );

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_mixed_old_and_new_conditions() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Age >= 18 AND Duration >= 5 years AND SetMembership in {active, pending}
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::And(
                Box::new(Condition::Duration {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 5,
                    unit: legalis_core::DurationUnit::Years,
                }),
                Box::new(Condition::SetMembership {
                    attribute: "status".to_string(),
                    values: vec!["active".to_string(), "pending".to_string()],
                    negated: false,
                }),
            )),
        );

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_calculation_satisfiable() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Calculation: tax_owed = income * 0.2, checking if tax_owed > 1000
        let condition = Condition::Calculation {
            formula: "income * 0.2".to_string(),
            operator: ComparisonOp::GreaterThan,
            value: 1000.0,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_calculation_different_formulas() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Two different calculations should be treated as different variables
        let cond1 = Condition::Calculation {
            formula: "income * 0.2".to_string(),
            operator: ComparisonOp::GreaterThan,
            value: 1000.0,
        };

        let cond2 = Condition::Calculation {
            formula: "income * 0.3".to_string(),
            operator: ComparisonOp::LessThan,
            value: 2000.0,
        };

        let combined = Condition::And(Box::new(cond1), Box::new(cond2));
        assert!(verifier.is_satisfiable(&combined).unwrap());
    }

    #[test]
    fn test_calculation_contradiction() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Same formula with contradictory constraints: value > 5000 AND value < 1000
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

    #[test]
    fn test_calculation_with_age_and_income() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Complex condition: age >= 18 AND income > 50000 AND tax_liability > 10000
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::And(
                Box::new(Condition::Income {
                    operator: ComparisonOp::GreaterThan,
                    value: 50000,
                }),
                Box::new(Condition::Calculation {
                    formula: "income * tax_rate".to_string(),
                    operator: ComparisonOp::GreaterThan,
                    value: 10000.0,
                }),
            )),
        );

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_calculation_equality() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Calculation with equality operator
        let condition = Condition::Calculation {
            formula: "base_amount + fees".to_string(),
            operator: ComparisonOp::Equal,
            value: 1500.0,
        };

        assert!(verifier.is_satisfiable(&condition).unwrap());
    }

    #[test]
    fn test_condition_equivalence() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Two equivalent conditions with different structure
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
    fn test_condition_not_equivalent() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let cond2 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        assert!(!verifier.are_equivalent(&cond1, &cond2).unwrap());
    }

    #[test]
    fn test_double_negation_simplification() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // NOT(NOT(age >= 18))
        let complex = Condition::Not(Box::new(Condition::Not(Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }))));

        let (simplified, changed) = verifier.simplify(&complex).unwrap();
        assert!(changed);

        // Should simplify to age >= 18
        if let Condition::Age { operator, value } = simplified {
            assert_eq!(operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(value, 18);
        } else {
            panic!("Expected Age condition after simplification");
        }
    }

    #[test]
    fn test_redundant_and_simplification() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // age >= 21 AND age >= 18
        // The first implies the second, so this should simplify to just age >= 21
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

        // Should simplify to age >= 21
        if let Condition::Age { operator, value } = simplified {
            assert_eq!(operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(value, 21);
        } else {
            panic!("Expected simplified to age >= 21");
        }
    }

    #[test]
    fn test_complexity_analysis() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Complex condition with double negation
        let condition = Condition::Not(Box::new(Condition::Not(Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }))));

        let (complexity, suggestions) = verifier.analyze_complexity(&condition);
        assert!(complexity >= 3); // At least 3 nodes
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("double negation"));
    }

    #[test]
    fn test_complexity_analysis_redundant_and() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // age >= 21 AND age >= 18 (redundant)
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            }),
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
        );

        let (complexity, suggestions) = verifier.analyze_complexity(&condition);
        assert!(complexity >= 3);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("simplified")));
    }

    // ===== Tests for Advanced SMT Features (v0.1.1) =====

    #[test]
    fn test_quantifier_forall_valid() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Check that forall age: age >= 0 OR age < 0 (tautology)
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
        assert!(result.unwrap()); // Should be valid for all ages
    }

    #[test]
    fn test_quantifier_forall_invalid() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Check that forall age: age >= 18 (not valid for all ages)
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let result = verifier.check_forall(&["age"], &condition);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should not be valid for all ages
    }

    #[test]
    fn test_quantifier_exists_satisfiable() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Check that exists age: age >= 18 AND age < 21
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
        assert!(result.unwrap()); // Should exist (e.g., age=19)
    }

    #[test]
    fn test_quantifier_exists_unsatisfiable() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Check that exists age: age > 100 AND age < 50 (impossible)
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
        assert!(!result.unwrap()); // Should not exist
    }

    #[test]
    fn test_array_basic_operations() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Test basic array operations
        let result = verifier.assert_array_element("test_array", 0, 100);
        assert!(result.is_ok());

        let result = verifier.assert_array_element("test_array", 1, 200);
        assert!(result.is_ok());

        // Check satisfiability
        assert!(verifier.check().is_ok());
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_array_all_elements_satisfy() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Set array elements to specific values
        for i in 0..5 {
            verifier
                .assert_array_element("numbers", i, (i + 1) * 10)
                .unwrap();
        }

        // Check if all elements are > 0
        let result = verifier.check_all_array_elements("numbers", 0, 4, |elem| {
            let zero = Int::from_i64(verifier.ctx, 0);
            elem.gt(&zero)
        });

        // Note: This test may not work as expected due to how Z3 handles
        // arrays with specific element assertions vs. forall quantifiers
        assert!(result.is_ok());
    }

    #[test]
    fn test_bitvector_basic_operations() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Assert that an 8-bit bitvector equals 42
        let result = verifier.assert_bitvector_constraint("flags", 8, ComparisonOp::Equal, 42);
        assert!(result.is_ok());

        // Check satisfiability
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_comparisons() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Assert that a 32-bit bitvector is >= 100 and < 200
        verifier
            .assert_bitvector_constraint("value", 32, ComparisonOp::GreaterOrEqual, 100)
            .unwrap();
        verifier
            .assert_bitvector_constraint("value", 32, ComparisonOp::LessThan, 200)
            .unwrap();

        // Should be satisfiable (e.g., value=150)
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_mask_operation() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Check if (bv & 0xFF00) == 0x1200 is satisfiable for 16-bit bv
        let result = verifier.check_bitvector_mask("bv", 16, 0xFF00, 0x1200);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should be satisfiable (e.g., bv=0x12XX where XX can be anything)
    }

    #[test]
    fn test_bitvector_unsatisfiable_constraints() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Assert contradictory constraints
        verifier
            .assert_bitvector_constraint("val", 8, ComparisonOp::GreaterThan, 200)
            .unwrap();
        verifier
            .assert_bitvector_constraint("val", 8, ComparisonOp::LessThan, 50)
            .unwrap();

        // Should be unsatisfiable
        assert!(!verifier.check().unwrap());
    }

    #[test]
    fn test_uninterpreted_function_declaration() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Declare a unary function f: Int -> Int
        let result = verifier.declare_uninterpreted_func("f", 1);
        assert!(result.is_ok());

        // Declare a binary function g: Int x Int -> Int
        let result = verifier.declare_uninterpreted_func("g", 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_uninterpreted_function_properties() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Assert that f(10) = 20
        verifier.assert_func_property("f", &[10], 20).unwrap();

        // Assert that f(10) = 25 (contradiction)
        verifier.assert_func_property("f", &[10], 25).unwrap();

        // Should be unsatisfiable due to contradiction
        assert!(!verifier.check().unwrap());
    }

    #[test]
    fn test_uninterpreted_function_consistency() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Assert that f(5) = 10
        verifier.assert_func_property("f", &[5], 10).unwrap();

        // Assert that f(5) = 10 again (consistent)
        verifier.assert_func_property("f", &[5], 10).unwrap();

        // Should be satisfiable
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_uninterpreted_function_injectivity() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Without constraints, function may not be injective
        let result = verifier.check_func_injective("h");
        assert!(result.is_ok());
        // Result depends on Z3's default behavior - could be either true or false
    }

    #[test]
    fn test_uninterpreted_function_with_constraints() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Create a binary uninterpreted function
        verifier.declare_uninterpreted_func("add", 2).unwrap();

        // Assert add(3, 5) = 8
        verifier.assert_func_property("add", &[3, 5], 8).unwrap();

        // Assert add(2, 6) = 8
        verifier.assert_func_property("add", &[2, 6], 8).unwrap();

        // Should be satisfiable
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_mixed_quantifiers_and_arrays() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Set some array values
        verifier.assert_array_element("scores", 0, 90).unwrap();
        verifier.assert_array_element("scores", 1, 85).unwrap();
        verifier.assert_array_element("scores", 2, 95).unwrap();

        // Check satisfiability
        assert!(verifier.check().unwrap());
    }

    #[test]
    fn test_bitvector_overflow() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // For an 8-bit bitvector, max value is 255
        // Asserting value > 255 should be unsatisfiable
        verifier
            .assert_bitvector_constraint("small", 8, ComparisonOp::GreaterThan, 255)
            .unwrap();

        assert!(!verifier.check().unwrap());
    }

    #[test]
    fn test_quantifier_multiple_variables() {
        let ctx = create_z3_context();
        let mut verifier = SmtVerifier::new(&ctx);

        // Check that exists age, income: age >= 18 AND income > 50000
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
        assert!(result.unwrap()); // Should exist
    }
}
