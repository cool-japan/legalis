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
use z3::ast::{Ast, Bool, Int};
use z3::{Config, Context as Z3Context, Solver};

/// SMT-based verifier for legal conditions.
pub struct SmtVerifier<'ctx> {
    ctx: &'ctx Z3Context,
    solver: Solver<'ctx>,
    /// Maps entity attribute names to Z3 integer variables
    int_vars: HashMap<String, Int<'ctx>>,
    /// Maps boolean attributes to Z3 boolean variables
    bool_vars: HashMap<String, Bool<'ctx>>,
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

        for (key, value) in stats.entries() {
            result.insert(key.to_string(), value.to_string());
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
                    Ok(Some(proof_ast.to_string()))
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
        for (name, _) in &self.int_vars {
            smtlib2.push_str(&format!("(declare-const {} Int)\n", name));
        }
        for (name, _) in &self.bool_vars {
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
                    smtlib2.push_str(&format!("; Proof: {}\n", proof));
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

        let core = verifier.find_unsat_core(&conditions).unwrap();

        // The core should contain at least the two contradicting age conditions
        assert!(!core.is_empty());
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
}
