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
}
