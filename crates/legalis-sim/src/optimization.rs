//! Policy optimization for finding optimal statute parameters.
//!
//! This module provides tools for optimizing policy parameters to achieve desired
//! outcomes while respecting constraints.

use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameter bounds for optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterBounds {
    /// Lower bound.
    pub lower: f64,
    /// Upper bound.
    pub upper: f64,
}

impl ParameterBounds {
    /// Create new parameter bounds.
    pub fn new(lower: f64, upper: f64) -> SimResult<Self> {
        if lower > upper {
            return Err(SimulationError::ConfigurationError(
                "Lower bound must be <= upper bound".to_string(),
            ));
        }
        Ok(Self { lower, upper })
    }

    /// Clamp value to bounds.
    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.lower).min(self.upper)
    }
}

/// Optimization objective.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Objective {
    /// Maximize the objective.
    Maximize,
    /// Minimize the objective.
    Minimize,
}

/// Optimization result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// Best parameters found.
    pub best_parameters: HashMap<String, f64>,
    /// Best objective value.
    pub best_objective: f64,
    /// Number of iterations/evaluations.
    pub iterations: usize,
    /// Whether optimization converged.
    pub converged: bool,
    /// Optimization history.
    pub history: Vec<(HashMap<String, f64>, f64)>,
}

/// Grid search optimizer.
pub struct GridSearchOptimizer {
    /// Number of grid points per parameter.
    pub grid_points: usize,
    /// Objective direction.
    pub objective: Objective,
}

impl GridSearchOptimizer {
    /// Create new grid search optimizer.
    pub fn new(grid_points: usize, objective: Objective) -> Self {
        Self {
            grid_points,
            objective,
        }
    }

    /// Optimize using grid search.
    pub fn optimize<F>(
        &self,
        parameter_bounds: &HashMap<String, ParameterBounds>,
        objective_fn: F,
    ) -> SimResult<OptimizationResult>
    where
        F: Fn(&HashMap<String, f64>) -> f64,
    {
        if parameter_bounds.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "No parameters to optimize".to_string(),
            ));
        }

        let param_names: Vec<_> = parameter_bounds.keys().cloned().collect();
        let mut grid_values = Vec::new();

        for param_name in &param_names {
            let bounds = &parameter_bounds[param_name];
            let step = (bounds.upper - bounds.lower) / (self.grid_points - 1) as f64;

            let values: Vec<f64> = (0..self.grid_points)
                .map(|i| bounds.lower + i as f64 * step)
                .collect();

            grid_values.push(values);
        }

        let mut best_params = HashMap::new();
        let mut best_objective = match self.objective {
            Objective::Maximize => f64::NEG_INFINITY,
            Objective::Minimize => f64::INFINITY,
        };

        let mut history = Vec::new();
        let mut iterations = 0;

        // Generate all combinations
        Self::grid_search_recursive(
            &param_names,
            &grid_values,
            0,
            &mut HashMap::new(),
            &objective_fn,
            &mut best_params,
            &mut best_objective,
            &self.objective,
            &mut history,
            &mut iterations,
        );

        Ok(OptimizationResult {
            best_parameters: best_params,
            best_objective,
            iterations,
            converged: true,
            history,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn grid_search_recursive<F>(
        param_names: &[String],
        grid_values: &[Vec<f64>],
        depth: usize,
        current_params: &mut HashMap<String, f64>,
        objective_fn: &F,
        best_params: &mut HashMap<String, f64>,
        best_objective: &mut f64,
        objective: &Objective,
        history: &mut Vec<(HashMap<String, f64>, f64)>,
        iterations: &mut usize,
    ) where
        F: Fn(&HashMap<String, f64>) -> f64,
    {
        if depth == param_names.len() {
            let objective_value = objective_fn(current_params);
            history.push((current_params.clone(), objective_value));
            *iterations += 1;

            let is_better = match objective {
                Objective::Maximize => objective_value > *best_objective,
                Objective::Minimize => objective_value < *best_objective,
            };

            if is_better {
                *best_objective = objective_value;
                *best_params = current_params.clone();
            }

            return;
        }

        let param_name = &param_names[depth];
        let values = &grid_values[depth];

        for &value in values {
            current_params.insert(param_name.clone(), value);
            Self::grid_search_recursive(
                param_names,
                grid_values,
                depth + 1,
                current_params,
                objective_fn,
                best_params,
                best_objective,
                objective,
                history,
                iterations,
            );
        }

        current_params.remove(param_name);
    }
}

/// Nelder-Mead simplex optimizer (gradient-free).
pub struct NelderMeadOptimizer {
    /// Maximum iterations.
    pub max_iterations: usize,
    /// Convergence tolerance.
    pub tolerance: f64,
    /// Objective direction.
    pub objective: Objective,
}

impl NelderMeadOptimizer {
    /// Create new Nelder-Mead optimizer.
    pub fn new(max_iterations: usize, tolerance: f64, objective: Objective) -> Self {
        Self {
            max_iterations,
            tolerance,
            objective,
        }
    }

    /// Optimize using Nelder-Mead algorithm.
    pub fn optimize<F>(
        &self,
        initial_params: &HashMap<String, f64>,
        parameter_bounds: &HashMap<String, ParameterBounds>,
        objective_fn: F,
    ) -> SimResult<OptimizationResult>
    where
        F: Fn(&HashMap<String, f64>) -> f64,
    {
        if initial_params.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "No initial parameters provided".to_string(),
            ));
        }

        let param_names: Vec<_> = initial_params.keys().cloned().collect();
        let n = param_names.len();

        // Create initial simplex
        let mut simplex = vec![initial_params.clone()];

        for i in 0..n {
            let mut vertex = initial_params.clone();
            let param_name = &param_names[i];
            let current = vertex[param_name];
            let step = if let Some(bounds) = parameter_bounds.get(param_name) {
                (bounds.upper - bounds.lower) * 0.1
            } else {
                current.abs() * 0.1 + 0.1
            };

            vertex.insert(param_name.clone(), current + step);
            simplex.push(vertex);
        }

        let mut history = Vec::new();
        let mut iterations = 0;
        let mut converged = false;

        // Nelder-Mead constants
        let alpha = 1.0; // Reflection
        let gamma = 2.0; // Expansion
        let rho = 0.5; // Contraction
        let sigma = 0.5; // Shrink

        for _ in 0..self.max_iterations {
            // Evaluate simplex
            let mut values: Vec<_> = simplex
                .iter()
                .map(|params| {
                    let value = objective_fn(params);
                    (params.clone(), value)
                })
                .collect();

            // Sort by objective (best to worst)
            values.sort_by(|a, b| {
                match self.objective {
                    Objective::Maximize => b.1.partial_cmp(&a.1),
                    Objective::Minimize => a.1.partial_cmp(&b.1),
                }
                .unwrap_or(std::cmp::Ordering::Equal)
            });

            let best = &values[0];
            let worst = &values[n];

            history.push((best.0.clone(), best.1));
            iterations += 1;

            // Check convergence
            let range = (worst.1 - best.1).abs();
            if range < self.tolerance {
                converged = true;
                break;
            }

            // Calculate centroid (excluding worst point)
            let centroid = Self::calculate_centroid(&values[0..n]);

            // Reflection
            let reflected = Self::reflect(&centroid, &worst.0, alpha);
            let reflected = Self::clamp_params(&reflected, parameter_bounds);
            let reflected_value = objective_fn(&reflected);

            let second_worst_value = values[n - 1].1;

            let is_better_than_worst = match self.objective {
                Objective::Maximize => reflected_value > worst.1,
                Objective::Minimize => reflected_value < worst.1,
            };

            let is_better_than_second_worst = match self.objective {
                Objective::Maximize => reflected_value > second_worst_value,
                Objective::Minimize => reflected_value < second_worst_value,
            };

            if is_better_than_worst && is_better_than_second_worst {
                simplex[n] = reflected;
                continue;
            }

            // Expansion
            if match self.objective {
                Objective::Maximize => reflected_value > best.1,
                Objective::Minimize => reflected_value < best.1,
            } {
                let expanded = Self::reflect(&centroid, &worst.0, gamma);
                let expanded = Self::clamp_params(&expanded, parameter_bounds);
                let expanded_value = objective_fn(&expanded);

                if match self.objective {
                    Objective::Maximize => expanded_value > reflected_value,
                    Objective::Minimize => expanded_value < reflected_value,
                } {
                    simplex[n] = expanded;
                } else {
                    simplex[n] = reflected;
                }
                continue;
            }

            // Contraction
            let contracted = Self::contract(&centroid, &worst.0, rho);
            let contracted = Self::clamp_params(&contracted, parameter_bounds);
            let contracted_value = objective_fn(&contracted);

            if match self.objective {
                Objective::Maximize => contracted_value > worst.1,
                Objective::Minimize => contracted_value < worst.1,
            } {
                simplex[n] = contracted;
                continue;
            }

            // Shrink
            for i in 1..=n {
                simplex[i] = Self::shrink(&best.0, &simplex[i], sigma);
                simplex[i] = Self::clamp_params(&simplex[i], parameter_bounds);
            }
        }

        let final_values: Vec<_> = simplex.iter().map(|p| objective_fn(p)).collect();
        let (best_idx, &best_objective) = final_values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                match self.objective {
                    Objective::Maximize => a.partial_cmp(b),
                    Objective::Minimize => b.partial_cmp(a),
                }
                .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap();

        Ok(OptimizationResult {
            best_parameters: simplex[best_idx].clone(),
            best_objective,
            iterations,
            converged,
            history,
        })
    }

    fn calculate_centroid(points: &[(HashMap<String, f64>, f64)]) -> HashMap<String, f64> {
        if points.is_empty() {
            return HashMap::new();
        }

        let mut centroid = HashMap::new();
        let n = points.len() as f64;

        for (params, _) in points {
            for (key, value) in params {
                *centroid.entry(key.clone()).or_insert(0.0) += value / n;
            }
        }

        centroid
    }

    fn reflect(
        centroid: &HashMap<String, f64>,
        point: &HashMap<String, f64>,
        alpha: f64,
    ) -> HashMap<String, f64> {
        point
            .iter()
            .map(|(key, value)| {
                let c = centroid.get(key).copied().unwrap_or(0.0);
                (key.clone(), c + alpha * (c - value))
            })
            .collect()
    }

    fn contract(
        centroid: &HashMap<String, f64>,
        point: &HashMap<String, f64>,
        rho: f64,
    ) -> HashMap<String, f64> {
        point
            .iter()
            .map(|(key, value)| {
                let c = centroid.get(key).copied().unwrap_or(0.0);
                (key.clone(), c + rho * (value - c))
            })
            .collect()
    }

    fn shrink(
        best: &HashMap<String, f64>,
        point: &HashMap<String, f64>,
        sigma: f64,
    ) -> HashMap<String, f64> {
        point
            .iter()
            .map(|(key, value)| {
                let b = best.get(key).copied().unwrap_or(0.0);
                (key.clone(), b + sigma * (value - b))
            })
            .collect()
    }

    fn clamp_params(
        params: &HashMap<String, f64>,
        bounds: &HashMap<String, ParameterBounds>,
    ) -> HashMap<String, f64> {
        params
            .iter()
            .map(|(key, value)| {
                let clamped = if let Some(bound) = bounds.get(key) {
                    bound.clamp(*value)
                } else {
                    *value
                };
                (key.clone(), clamped)
            })
            .collect()
    }
}

/// Pareto frontier for multi-objective optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParetoFrontier {
    /// Non-dominated solutions.
    pub solutions: Vec<(HashMap<String, f64>, Vec<f64>)>,
}

impl ParetoFrontier {
    /// Create new empty Pareto frontier.
    pub fn new() -> Self {
        Self {
            solutions: Vec::new(),
        }
    }

    /// Add solution to frontier (if non-dominated).
    pub fn add_solution(&mut self, params: HashMap<String, f64>, objectives: Vec<f64>) {
        // Check if dominated by existing solutions
        let dominated = self
            .solutions
            .iter()
            .any(|(_, existing_objs)| Self::dominates(existing_objs, &objectives));

        if !dominated {
            // Remove solutions dominated by new solution
            self.solutions
                .retain(|(_, existing_objs)| !Self::dominates(&objectives, existing_objs));

            self.solutions.push((params, objectives));
        }
    }

    fn dominates(a: &[f64], b: &[f64]) -> bool {
        let mut at_least_one_better = false;

        for (a_val, b_val) in a.iter().zip(b.iter()) {
            if a_val < b_val {
                return false; // Assuming maximization; a is worse in this objective
            }
            if a_val > b_val {
                at_least_one_better = true;
            }
        }

        at_least_one_better
    }
}

impl Default for ParetoFrontier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_bounds() {
        let bounds = ParameterBounds::new(0.0, 10.0).unwrap();
        assert_eq!(bounds.clamp(-5.0), 0.0);
        assert_eq!(bounds.clamp(5.0), 5.0);
        assert_eq!(bounds.clamp(15.0), 10.0);
    }

    #[test]
    fn test_grid_search() {
        let optimizer = GridSearchOptimizer::new(5, Objective::Minimize);

        let mut bounds = HashMap::new();
        bounds.insert("x".to_string(), ParameterBounds::new(-10.0, 10.0).unwrap());
        bounds.insert("y".to_string(), ParameterBounds::new(-10.0, 10.0).unwrap());

        // Minimize (x-3)^2 + (y-4)^2
        let objective = |params: &HashMap<String, f64>| {
            let x = params.get("x").copied().unwrap_or(0.0);
            let y = params.get("y").copied().unwrap_or(0.0);
            (x - 3.0).powi(2) + (y - 4.0).powi(2)
        };

        let result = optimizer.optimize(&bounds, objective).unwrap();

        // Should find minimum near (3, 4)
        let x = result.best_parameters.get("x").unwrap();
        let y = result.best_parameters.get("y").unwrap();

        assert!((x - 3.0).abs() < 3.0);
        assert!((y - 4.0).abs() < 3.0);
        assert!(result.converged);
    }

    #[test]
    fn test_nelder_mead() {
        let optimizer = NelderMeadOptimizer::new(100, 1e-6, Objective::Minimize);

        let mut initial = HashMap::new();
        initial.insert("x".to_string(), 0.0);
        initial.insert("y".to_string(), 0.0);

        let mut bounds = HashMap::new();
        bounds.insert("x".to_string(), ParameterBounds::new(-10.0, 10.0).unwrap());
        bounds.insert("y".to_string(), ParameterBounds::new(-10.0, 10.0).unwrap());

        // Minimize (x-2)^2 + (y-3)^2
        let objective = |params: &HashMap<String, f64>| {
            let x = params.get("x").copied().unwrap_or(0.0);
            let y = params.get("y").copied().unwrap_or(0.0);
            (x - 2.0).powi(2) + (y - 3.0).powi(2)
        };

        let result = optimizer.optimize(&initial, &bounds, objective).unwrap();

        let x = result.best_parameters.get("x").unwrap();
        let y = result.best_parameters.get("y").unwrap();

        assert!((x - 2.0).abs() < 0.1);
        assert!((y - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_pareto_frontier() {
        let mut frontier = ParetoFrontier::new();

        let mut p1 = HashMap::new();
        p1.insert("x".to_string(), 1.0);
        frontier.add_solution(p1.clone(), vec![10.0, 5.0]);

        let mut p2 = HashMap::new();
        p2.insert("x".to_string(), 2.0);
        frontier.add_solution(p2.clone(), vec![8.0, 8.0]);

        let mut p3 = HashMap::new();
        p3.insert("x".to_string(), 3.0);
        frontier.add_solution(p3.clone(), vec![5.0, 10.0]);

        // Dominated solution should not be added
        let mut p4 = HashMap::new();
        p4.insert("x".to_string(), 4.0);
        frontier.add_solution(p4.clone(), vec![7.0, 6.0]);

        assert_eq!(frontier.solutions.len(), 3);
    }
}
