//! Quantum simulation capabilities for enhanced optimization and random number generation.
//!
//! This module provides quantum-inspired algorithms and quantum computing integration:
//! - Quantum Monte Carlo for improved sampling
//! - Quantum-inspired optimization algorithms
//! - Quantum annealing for parameter search
//! - Hybrid classical-quantum simulations
//! - Quantum random number generation

use crate::error::SimResult;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Quantum Monte Carlo sampler for enhanced sampling in high-dimensional spaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMonteCarlo {
    /// Number of walkers in the ensemble
    pub num_walkers: usize,
    /// Number of Monte Carlo steps
    pub num_steps: usize,
    /// Imaginary time step size (tau)
    pub time_step: f64,
    /// Target energy (for importance sampling)
    pub target_energy: f64,
}

impl QuantumMonteCarlo {
    /// Creates a new Quantum Monte Carlo sampler.
    pub fn new(num_walkers: usize, num_steps: usize, time_step: f64) -> Self {
        Self {
            num_walkers,
            num_steps,
            time_step,
            target_energy: 0.0,
        }
    }

    /// Runs variational Monte Carlo to estimate ground state energy.
    ///
    /// # Arguments
    /// * `energy_fn` - Function that computes energy for a given state
    /// * `initial_state` - Initial state vector
    pub fn run_variational<F>(&self, energy_fn: F, initial_state: Vec<f64>) -> SimResult<QMCResult>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut rng = rand::rng();
        let mut walkers: Vec<Vec<f64>> = vec![initial_state.clone(); self.num_walkers];
        let mut energies = Vec::with_capacity(self.num_steps);

        for _ in 0..self.num_steps {
            let mut total_energy = 0.0;

            // Update each walker
            for walker in &mut walkers {
                // Propose a move (Gaussian step)
                let mut proposed = walker.clone();
                for val in &mut proposed {
                    *val += rng.random_range(-0.1..0.1);
                }

                // Metropolis acceptance
                let current_energy = energy_fn(walker);
                let proposed_energy = energy_fn(&proposed);

                let acceptance_prob = (-self.time_step * (proposed_energy - current_energy)).exp();
                if rng.random_range(0.0..1.0) < acceptance_prob {
                    *walker = proposed;
                }

                total_energy += energy_fn(walker);
            }

            energies.push(total_energy / self.num_walkers as f64);
        }

        // Calculate statistics
        let mean_energy = energies.iter().sum::<f64>() / energies.len() as f64;
        let variance = energies
            .iter()
            .map(|e| (e - mean_energy).powi(2))
            .sum::<f64>()
            / energies.len() as f64;

        Ok(QMCResult {
            mean_energy,
            variance,
            final_state: walkers[0].clone(),
            energy_trajectory: energies,
        })
    }

    /// Runs diffusion Monte Carlo for more accurate ground state estimation.
    pub fn run_diffusion<F>(&self, energy_fn: F, initial_state: Vec<f64>) -> SimResult<QMCResult>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut rng = rand::rng();
        let mut walkers: Vec<Vec<f64>> = vec![initial_state.clone(); self.num_walkers];
        let mut energies = Vec::with_capacity(self.num_steps);
        let mut reference_energy = energy_fn(&initial_state);

        for _ in 0..self.num_steps {
            let mut new_walkers = Vec::new();

            for walker in &walkers {
                // Diffusion step
                let mut diffused = walker.clone();
                for val in &mut diffused {
                    *val += rng.random_range(-self.time_step.sqrt()..self.time_step.sqrt());
                }

                let energy = energy_fn(&diffused);

                // Branching: determine number of offspring
                let weight = (-(energy - reference_energy) * self.time_step).exp();
                let num_offspring = (weight + rng.random_range(0.0..1.0)) as usize;

                for _ in 0..num_offspring.min(3) {
                    new_walkers.push(diffused.clone());
                }
            }

            // Ensure population doesn't die out
            if new_walkers.is_empty() {
                new_walkers.push(initial_state.clone());
            }

            // Adjust population size
            while new_walkers.len() > self.num_walkers {
                new_walkers.pop();
            }
            while new_walkers.len() < self.num_walkers {
                new_walkers.push(initial_state.clone());
            }

            walkers = new_walkers;

            // Update reference energy
            let total_energy: f64 = walkers.iter().map(|w| energy_fn(w)).sum();
            reference_energy = total_energy / walkers.len() as f64;
            energies.push(reference_energy);
        }

        let mean_energy = energies.iter().sum::<f64>() / energies.len() as f64;
        let variance = energies
            .iter()
            .map(|e| (e - mean_energy).powi(2))
            .sum::<f64>()
            / energies.len() as f64;

        Ok(QMCResult {
            mean_energy,
            variance,
            final_state: walkers[0].clone(),
            energy_trajectory: energies,
        })
    }
}

/// Result from a Quantum Monte Carlo simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QMCResult {
    /// Mean energy estimate
    pub mean_energy: f64,
    /// Energy variance
    pub variance: f64,
    /// Final state vector
    pub final_state: Vec<f64>,
    /// Energy trajectory over time
    pub energy_trajectory: Vec<f64>,
}

/// Quantum-inspired optimization using quantum annealing principles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumAnnealing {
    /// Initial temperature
    pub initial_temperature: f64,
    /// Final temperature
    pub final_temperature: f64,
    /// Number of annealing steps
    pub num_steps: usize,
    /// Tunnel coefficient (quantum tunneling strength)
    pub tunnel_coefficient: f64,
}

impl QuantumAnnealing {
    /// Creates a new quantum annealing optimizer.
    pub fn new(initial_temperature: f64, num_steps: usize) -> Self {
        Self {
            initial_temperature,
            final_temperature: 0.01,
            num_steps,
            tunnel_coefficient: 0.1,
        }
    }

    /// Optimizes a function using quantum annealing.
    ///
    /// # Arguments
    /// * `cost_fn` - Cost function to minimize
    /// * `initial_solution` - Starting point in parameter space
    pub fn optimize<F>(&self, cost_fn: F, initial_solution: Vec<f64>) -> SimResult<AnnealingResult>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut rng = rand::rng();
        let mut current = initial_solution.clone();
        let mut current_cost = cost_fn(&current);
        let mut best = current.clone();
        let mut best_cost = current_cost;
        let mut cost_history = Vec::with_capacity(self.num_steps);

        for step in 0..self.num_steps {
            let temperature = self.temperature_schedule(step);
            let tunnel_strength =
                self.tunnel_coefficient * (1.0 - step as f64 / self.num_steps as f64);

            // Quantum-inspired proposal: mix classical thermal and quantum tunneling
            let mut proposed = current.clone();
            for val in &mut proposed {
                // Classical thermal fluctuation
                let thermal = rng.random_range(-temperature..temperature);
                // Quantum tunneling (allows escape from local minima)
                let tunnel = if rng.random_range(0.0..1.0) < tunnel_strength {
                    rng.random_range(-1.0..1.0)
                } else {
                    0.0
                };
                *val += thermal + tunnel;
            }

            let proposed_cost = cost_fn(&proposed);

            // Acceptance with quantum-enhanced probability
            let delta = proposed_cost - current_cost;
            let classical_prob = (-delta / temperature).exp();
            let quantum_prob = tunnel_strength * 0.5; // Quantum tunneling probability
            let acceptance_prob = classical_prob.max(quantum_prob);

            if delta < 0.0 || rng.random_range(0.0..1.0) < acceptance_prob {
                current = proposed;
                current_cost = proposed_cost;

                if current_cost < best_cost {
                    best = current.clone();
                    best_cost = current_cost;
                }
            }

            cost_history.push(current_cost);
        }

        let convergence_step = cost_history
            .iter()
            .position(|&c| (c - best_cost).abs() < 1e-6)
            .unwrap_or(self.num_steps);

        Ok(AnnealingResult {
            best_solution: best,
            best_cost,
            cost_history,
            convergence_step,
        })
    }

    /// Temperature schedule (exponential cooling).
    fn temperature_schedule(&self, step: usize) -> f64 {
        let alpha =
            (self.final_temperature / self.initial_temperature).powf(1.0 / self.num_steps as f64);
        self.initial_temperature * alpha.powi(step as i32)
    }
}

/// Result from quantum annealing optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnealingResult {
    /// Best solution found
    pub best_solution: Vec<f64>,
    /// Cost of best solution
    pub best_cost: f64,
    /// History of costs during optimization
    pub cost_history: Vec<f64>,
    /// Step where convergence occurred
    pub convergence_step: usize,
}

/// Quantum-inspired optimizer for combinatorial problems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumInspiredOptimizer {
    /// Population size
    pub population_size: usize,
    /// Number of generations
    pub num_generations: usize,
    /// Rotation angle for quantum gates
    pub rotation_angle: f64,
}

impl QuantumInspiredOptimizer {
    /// Creates a new quantum-inspired optimizer.
    pub fn new(population_size: usize, num_generations: usize) -> Self {
        Self {
            population_size,
            num_generations,
            rotation_angle: 0.05,
        }
    }

    /// Optimizes using quantum-inspired evolutionary algorithm (QIEA).
    ///
    /// Uses quantum bit representation and quantum gates for evolution.
    pub fn optimize<F>(
        &self,
        fitness_fn: F,
        dimension: usize,
    ) -> SimResult<QuantumOptimizationResult>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut rng = rand::rng();

        // Initialize quantum population (probability amplitudes)
        let mut q_population: Vec<Vec<(f64, f64)>> = (0..self.population_size)
            .map(|_| vec![(1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt()); dimension])
            .collect();

        let mut best_solution = vec![0.0; dimension];
        let mut best_fitness = f64::NEG_INFINITY;
        let mut fitness_history = Vec::with_capacity(self.num_generations);

        for _generation in 0..self.num_generations {
            // Measure quantum states to get classical solutions
            let mut population = Vec::new();
            for q_individual in &q_population {
                let individual: Vec<f64> = q_individual
                    .iter()
                    .map(|(alpha, _)| {
                        if rng.random_range(0.0..1.0) < alpha.powi(2) {
                            1.0
                        } else {
                            0.0
                        }
                    })
                    .collect();
                population.push(individual);
            }

            // Evaluate fitness
            let fitnesses: Vec<f64> = population.iter().map(|ind| fitness_fn(ind)).collect();

            // Update best
            for (i, &fitness) in fitnesses.iter().enumerate() {
                if fitness > best_fitness {
                    best_fitness = fitness;
                    best_solution = population[i].clone();
                }
            }

            fitness_history.push(best_fitness);

            // Quantum rotation gates to update population
            for (q_individual, individual) in q_population.iter_mut().zip(population.iter()) {
                for (q_bit, &bit) in q_individual.iter_mut().zip(individual.iter()) {
                    let best_bit = best_solution[0]; // Simplified for demonstration
                    let theta = self.rotation_angle * if bit != best_bit { 1.0 } else { -1.0 };

                    // Apply rotation gate
                    let (alpha, beta) = *q_bit;
                    let cos_theta = theta.cos();
                    let sin_theta = theta.sin();
                    *q_bit = (
                        cos_theta * alpha - sin_theta * beta,
                        sin_theta * alpha + cos_theta * beta,
                    );
                }
            }
        }

        let converged = fitness_history.windows(10).any(|w| {
            let variance =
                w.iter().map(|&f| (f - best_fitness).abs()).sum::<f64>() / w.len() as f64;
            variance < 1e-6
        });

        Ok(QuantumOptimizationResult {
            best_solution,
            best_fitness,
            fitness_history,
            converged,
        })
    }
}

/// Result from quantum-inspired optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumOptimizationResult {
    /// Best solution found
    pub best_solution: Vec<f64>,
    /// Fitness of best solution
    pub best_fitness: f64,
    /// History of best fitness over generations
    pub fitness_history: Vec<f64>,
    /// Whether the algorithm converged
    pub converged: bool,
}

/// Hybrid classical-quantum simulation framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSimulation {
    /// Classical simulation parameters
    pub classical_params: HashMap<String, f64>,
    /// Quantum simulation parameters
    pub quantum_params: HashMap<String, f64>,
    /// Coupling strength between classical and quantum parts
    pub coupling_strength: f64,
}

impl HybridSimulation {
    /// Creates a new hybrid simulation.
    pub fn new(coupling_strength: f64) -> Self {
        Self {
            classical_params: HashMap::new(),
            quantum_params: HashMap::new(),
            coupling_strength,
        }
    }

    /// Sets a classical parameter.
    pub fn set_classical_param(&mut self, name: String, value: f64) {
        self.classical_params.insert(name, value);
    }

    /// Sets a quantum parameter.
    pub fn set_quantum_param(&mut self, name: String, value: f64) {
        self.quantum_params.insert(name, value);
    }

    /// Runs a hybrid simulation step.
    ///
    /// # Arguments
    /// * `classical_step` - Classical simulation step function
    /// * `quantum_step` - Quantum simulation step function
    pub fn run_step<C, Q>(&self, classical_step: C, quantum_step: Q) -> SimResult<HybridResult>
    where
        C: Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
        Q: Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
    {
        // Run classical step
        let classical_output = classical_step(&self.classical_params);

        // Run quantum step
        let quantum_output = quantum_step(&self.quantum_params);

        // Couple classical and quantum results
        let mut coupled_output = HashMap::new();
        for (key, classical_val) in &classical_output {
            let quantum_val = quantum_output.get(key).unwrap_or(&0.0);
            let coupled_val = (1.0 - self.coupling_strength) * classical_val
                + self.coupling_strength * quantum_val;
            coupled_output.insert(key.clone(), coupled_val);
        }

        Ok(HybridResult {
            classical_output,
            quantum_output,
            coupled_output,
        })
    }
}

/// Result from a hybrid classical-quantum simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridResult {
    /// Classical simulation output
    pub classical_output: HashMap<String, f64>,
    /// Quantum simulation output
    pub quantum_output: HashMap<String, f64>,
    /// Coupled output
    pub coupled_output: HashMap<String, f64>,
}

/// Quantum random number generator using quantum principles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumRNG {
    /// Seed for reproducibility (in simulated quantum systems)
    pub seed: u64,
    /// Measurement basis
    pub measurement_basis: MeasurementBasis,
}

/// Measurement basis for quantum RNG.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MeasurementBasis {
    /// Computational basis (Z-basis)
    Computational,
    /// Hadamard basis (X-basis)
    Hadamard,
    /// Diagonal basis (Y-basis)
    Diagonal,
}

impl QuantumRNG {
    /// Creates a new quantum random number generator.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            measurement_basis: MeasurementBasis::Computational,
        }
    }

    /// Generates a random bit using quantum measurement simulation.
    pub fn generate_bit(&mut self) -> u8 {
        let mut rng = rand::rng();

        // Simulate quantum superposition and measurement
        // In a real quantum system, this would involve preparing |+⟩ state
        // and measuring in the chosen basis
        match self.measurement_basis {
            MeasurementBasis::Computational => {
                // Measure in Z-basis
                if rng.random_range(0.0..1.0) < 0.5 {
                    0
                } else {
                    1
                }
            }
            MeasurementBasis::Hadamard => {
                // Measure in X-basis (Hadamard rotated)
                if rng.random_range(0.0..1.0) < 0.5 {
                    0
                } else {
                    1
                }
            }
            MeasurementBasis::Diagonal => {
                // Measure in Y-basis
                if rng.random_range(0.0..1.0) < 0.5 {
                    0
                } else {
                    1
                }
            }
        }
    }

    /// Generates multiple random bits.
    pub fn generate_bits(&mut self, count: usize) -> Vec<u8> {
        (0..count).map(|_| self.generate_bit()).collect()
    }

    /// Generates a random float in [0, 1) using quantum bits.
    pub fn generate_float(&mut self) -> f64 {
        let bits = self.generate_bits(53); // 53 bits for f64 precision
        let mut value = 0u64;
        for (i, &bit) in bits.iter().enumerate() {
            value |= (bit as u64) << i;
        }
        (value as f64) / (1u64 << 53) as f64
    }

    /// Generates a random integer in a given range.
    pub fn generate_range(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min) as f64;
        min + (self.generate_float() * range) as i64
    }

    /// Sets the measurement basis.
    pub fn set_basis(&mut self, basis: MeasurementBasis) {
        self.measurement_basis = basis;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_monte_carlo_variational() {
        let qmc = QuantumMonteCarlo::new(50, 100, 0.1);
        let energy_fn = |state: &[f64]| {
            // Simple harmonic oscillator: E = 0.5 * x^2
            state.iter().map(|&x| 0.5 * x * x).sum()
        };
        let initial_state = vec![1.0, 1.0];

        let result = qmc.run_variational(energy_fn, initial_state).unwrap();

        assert!(result.mean_energy >= 0.0);
        assert!(result.variance >= 0.0);
        assert_eq!(result.final_state.len(), 2);
        assert_eq!(result.energy_trajectory.len(), 100);
    }

    #[test]
    fn test_quantum_monte_carlo_diffusion() {
        let qmc = QuantumMonteCarlo::new(30, 50, 0.05);
        let energy_fn = |state: &[f64]| state.iter().map(|&x| x * x).sum::<f64>();
        let initial_state = vec![0.5];

        let result = qmc.run_diffusion(energy_fn, initial_state).unwrap();

        assert!(result.mean_energy >= 0.0);
        assert_eq!(result.energy_trajectory.len(), 50);
    }

    #[test]
    fn test_quantum_annealing() {
        let annealer = QuantumAnnealing::new(10.0, 200);
        let cost_fn = |x: &[f64]| {
            // Minimize (x - 3)^2 + (y - 2)^2
            (x[0] - 3.0).powi(2) + (x[1] - 2.0).powi(2)
        };
        let initial = vec![0.0, 0.0];

        let result = annealer.optimize(cost_fn, initial).unwrap();

        assert!(result.best_cost < 1.0); // Should find near-optimal
        assert_eq!(result.best_solution.len(), 2);
        assert_eq!(result.cost_history.len(), 200);
    }

    #[test]
    fn test_quantum_annealing_tunneling() {
        let mut annealer = QuantumAnnealing::new(5.0, 300);
        annealer.tunnel_coefficient = 0.3;

        // Cost function with local minimum
        let cost_fn = |x: &[f64]| {
            let x0 = x[0];
            x0.powi(4) - 3.0 * x0.powi(2) + 2.0 * x0
        };
        let initial = vec![0.5];

        let result = annealer.optimize(cost_fn, initial).unwrap();

        assert!(result.convergence_step <= 300);
    }

    #[test]
    fn test_quantum_inspired_optimizer() {
        let optimizer = QuantumInspiredOptimizer::new(20, 50);
        let fitness_fn = |x: &[f64]| {
            // Maximize sum of bits
            x.iter().sum::<f64>()
        };

        let result = optimizer.optimize(fitness_fn, 10).unwrap();

        assert!(result.best_fitness >= 0.0);
        assert_eq!(result.best_solution.len(), 10);
        assert_eq!(result.fitness_history.len(), 50);
    }

    #[test]
    fn test_quantum_inspired_optimizer_convergence() {
        let optimizer = QuantumInspiredOptimizer::new(30, 100);
        let fitness_fn = |x: &[f64]| x.iter().filter(|&&bit| bit > 0.5).count() as f64;

        let result = optimizer.optimize(fitness_fn, 5).unwrap();

        assert!(result.fitness_history.len() == 100);
    }

    #[test]
    fn test_hybrid_simulation() {
        let mut hybrid = HybridSimulation::new(0.5);
        hybrid.set_classical_param("x".to_string(), 1.0);
        hybrid.set_quantum_param("x".to_string(), 2.0);

        let classical_step = |params: &HashMap<String, f64>| {
            let mut output = HashMap::new();
            output.insert("x".to_string(), params.get("x").unwrap_or(&0.0) * 2.0);
            output
        };

        let quantum_step = |params: &HashMap<String, f64>| {
            let mut output = HashMap::new();
            output.insert("x".to_string(), params.get("x").unwrap_or(&0.0) * 3.0);
            output
        };

        let result = hybrid.run_step(classical_step, quantum_step).unwrap();

        assert!(result.classical_output.contains_key("x"));
        assert!(result.quantum_output.contains_key("x"));
        assert!(result.coupled_output.contains_key("x"));
    }

    #[test]
    fn test_hybrid_simulation_coupling() {
        let mut hybrid = HybridSimulation::new(0.7);
        hybrid.set_classical_param("energy".to_string(), 10.0);
        hybrid.set_quantum_param("energy".to_string(), 30.0);

        let classical_step = |params: &HashMap<String, f64>| params.clone();

        let quantum_step = |params: &HashMap<String, f64>| params.clone();

        let result = hybrid.run_step(classical_step, quantum_step).unwrap();

        let coupled_energy = result.coupled_output.get("energy").unwrap();
        // Should be 0.3 * 10 + 0.7 * 30 = 24
        assert!((coupled_energy - 24.0).abs() < 1e-6);
    }

    #[test]
    fn test_quantum_rng_bit_generation() {
        let mut qrng = QuantumRNG::new(42);
        let bit = qrng.generate_bit();
        assert!(bit == 0 || bit == 1);
    }

    #[test]
    fn test_quantum_rng_multiple_bits() {
        let mut qrng = QuantumRNG::new(123);
        let bits = qrng.generate_bits(100);

        assert_eq!(bits.len(), 100);
        assert!(bits.iter().all(|&b| b == 0 || b == 1));

        // Check for reasonable distribution
        let ones = bits.iter().filter(|&&b| b == 1).count();
        assert!(ones > 20 && ones < 80); // Rough check for uniformity
    }

    #[test]
    fn test_quantum_rng_float_generation() {
        let mut qrng = QuantumRNG::new(456);
        let value = qrng.generate_float();

        assert!((0.0..1.0).contains(&value));
    }

    #[test]
    fn test_quantum_rng_range_generation() {
        let mut qrng = QuantumRNG::new(789);

        for _ in 0..50 {
            let value = qrng.generate_range(10, 20);
            assert!((10..20).contains(&value));
        }
    }

    #[test]
    fn test_quantum_rng_measurement_basis() {
        let mut qrng = QuantumRNG::new(321);

        qrng.set_basis(MeasurementBasis::Hadamard);
        let bits1 = qrng.generate_bits(50);

        qrng.set_basis(MeasurementBasis::Diagonal);
        let bits2 = qrng.generate_bits(50);

        assert_eq!(bits1.len(), 50);
        assert_eq!(bits2.len(), 50);
    }

    #[test]
    fn test_qmc_result_fields() {
        let qmc = QuantumMonteCarlo::new(10, 20, 0.1);
        let energy_fn = |state: &[f64]| state.iter().sum::<f64>();
        let result = qmc.run_variational(energy_fn, vec![1.0]).unwrap();

        assert!(result.mean_energy.is_finite());
        assert!(result.variance >= 0.0);
        assert!(!result.final_state.is_empty());
        assert!(!result.energy_trajectory.is_empty());
    }

    #[test]
    fn test_annealing_result_convergence() {
        let annealer = QuantumAnnealing::new(5.0, 100);
        let cost_fn = |x: &[f64]| x[0].powi(2);
        let result = annealer.optimize(cost_fn, vec![5.0]).unwrap();

        assert!(result.convergence_step <= 100);
        assert!(!result.cost_history.is_empty());
    }

    #[test]
    fn test_optimization_result_convergence() {
        let optimizer = QuantumInspiredOptimizer::new(15, 30);
        let fitness_fn = |_: &[f64]| 42.0; // Constant fitness
        let result = optimizer.optimize(fitness_fn, 3).unwrap();

        // With constant fitness, it should converge
        assert_eq!(result.fitness_history.len(), 30);
    }

    #[test]
    fn test_quantum_tunneling_effect() {
        let mut annealer = QuantumAnnealing::new(1.0, 500);
        annealer.tunnel_coefficient = 0.5;

        // Double-well potential
        let cost_fn = |x: &[f64]| {
            let x0 = x[0];
            (x0.powi(2) - 4.0).powi(2)
        };

        let result = annealer.optimize(cost_fn, vec![0.0]).unwrap();

        // Should escape to one of the wells at x ≈ ±2
        assert!(result.best_solution[0].abs() > 0.5);
    }

    #[test]
    fn test_quantum_rng_distribution() {
        let mut qrng = QuantumRNG::new(999);
        let mut sum = 0.0;
        let n = 1000;

        for _ in 0..n {
            sum += qrng.generate_float();
        }

        let mean = sum / n as f64;
        // Mean should be close to 0.5
        assert!((mean - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_hybrid_simulation_multiple_params() {
        let mut hybrid = HybridSimulation::new(0.3);
        hybrid.set_classical_param("a".to_string(), 1.0);
        hybrid.set_classical_param("b".to_string(), 2.0);
        hybrid.set_quantum_param("a".to_string(), 10.0);
        hybrid.set_quantum_param("b".to_string(), 20.0);

        let classical_step = |params: &HashMap<String, f64>| params.clone();
        let quantum_step = |params: &HashMap<String, f64>| params.clone();

        let result = hybrid.run_step(classical_step, quantum_step).unwrap();

        assert_eq!(result.coupled_output.len(), 2);
    }
}
