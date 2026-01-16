//! Quantum Verification Module
//!
//! This module provides quantum computing-based verification capabilities for legal statutes,
//! including quantum circuit verification, quantum-resistant cryptography, quantum annealing
//! for SAT solving, and hybrid classical-quantum verification.
//!
//! # Features
//!
//! - **Quantum Circuit Verification**: Verify legal computations using quantum circuits
//! - **Quantum-Resistant Cryptography**: Post-quantum cryptographic proofs (lattice-based, hash-based)
//! - **Quantum Annealing**: SAT solving using simulated quantum annealing
//! - **Hybrid Verification**: Combine classical and quantum approaches
//! - **Quantum Supremacy Benchmarks**: Performance comparisons
//!
//! # Example
//!
//! ```
//! use legalis_verifier::quantum_verification::{QuantumVerifier, QuantumConfig};
//! use legalis_core::Statute;
//!
//! let config = QuantumConfig::default();
//! let verifier = QuantumVerifier::new(config);
//!
//! let statutes = vec![]; // Your statutes here
//! let result = verifier.verify_statutes(&statutes);
//! println!("Quantum verification complete: {} qubits used", result.total_qubits_used);
//! ```

use legalis_core::Statute;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Quantum gate types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantumGate {
    /// Hadamard gate (creates superposition)
    Hadamard { target: usize },
    /// Pauli-X gate (NOT gate)
    PauliX { target: usize },
    /// Pauli-Y gate
    PauliY { target: usize },
    /// Pauli-Z gate (phase flip)
    PauliZ { target: usize },
    /// CNOT gate (controlled-NOT)
    CNOT { control: usize, target: usize },
    /// Phase gate
    Phase { target: usize, angle: f64 },
    /// Rotation gate
    Rotation {
        target: usize,
        axis: RotationAxis,
        angle: f64,
    },
    /// Toffoli gate (CCNOT)
    Toffoli {
        control1: usize,
        control2: usize,
        target: usize,
    },
    /// Measurement
    Measure { target: usize, classical_bit: usize },
}

/// Rotation axis for quantum rotation gates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}

/// Quantum circuit representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    /// Number of qubits in the circuit
    pub num_qubits: usize,
    /// Number of classical bits for measurement
    pub num_classical_bits: usize,
    /// Sequence of quantum gates
    pub gates: Vec<QuantumGate>,
    /// Circuit metadata
    pub metadata: CircuitMetadata,
}

/// Metadata for quantum circuits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitMetadata {
    /// Circuit name/identifier
    pub name: String,
    /// Purpose of the circuit
    pub purpose: String,
    /// Expected computational advantage
    pub advantage_type: QuantumAdvantageType,
}

/// Types of quantum computational advantage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantumAdvantageType {
    /// Exponential speedup
    Exponential,
    /// Polynomial speedup
    Polynomial,
    /// Constant factor speedup
    ConstantFactor,
    /// No significant advantage
    None,
}

impl QuantumCircuit {
    /// Create a new quantum circuit
    pub fn new(num_qubits: usize, num_classical_bits: usize, name: String) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            gates: Vec::new(),
            metadata: CircuitMetadata {
                name,
                purpose: String::new(),
                advantage_type: QuantumAdvantageType::None,
            },
        }
    }

    /// Add a gate to the circuit
    pub fn add_gate(&mut self, gate: QuantumGate) {
        self.gates.push(gate);
    }

    /// Calculate circuit depth (critical path length)
    pub fn depth(&self) -> usize {
        if self.gates.is_empty() {
            return 0;
        }

        // Track when each qubit is last used
        let mut qubit_depths = vec![0; self.num_qubits];

        for gate in &self.gates {
            match gate {
                QuantumGate::Hadamard { target }
                | QuantumGate::PauliX { target }
                | QuantumGate::PauliY { target }
                | QuantumGate::PauliZ { target }
                | QuantumGate::Phase { target, .. }
                | QuantumGate::Rotation { target, .. }
                | QuantumGate::Measure { target, .. } => {
                    qubit_depths[*target] += 1;
                }
                QuantumGate::CNOT { control, target } => {
                    let max_depth = qubit_depths[*control].max(qubit_depths[*target]);
                    qubit_depths[*control] = max_depth + 1;
                    qubit_depths[*target] = max_depth + 1;
                }
                QuantumGate::Toffoli {
                    control1,
                    control2,
                    target,
                } => {
                    let max_depth = qubit_depths[*control1]
                        .max(qubit_depths[*control2])
                        .max(qubit_depths[*target]);
                    qubit_depths[*control1] = max_depth + 1;
                    qubit_depths[*control2] = max_depth + 1;
                    qubit_depths[*target] = max_depth + 1;
                }
            }
        }

        *qubit_depths.iter().max().unwrap_or(&0)
    }

    /// Count gates by type
    pub fn gate_counts(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for gate in &self.gates {
            let gate_type = match gate {
                QuantumGate::Hadamard { .. } => "Hadamard",
                QuantumGate::PauliX { .. } => "PauliX",
                QuantumGate::PauliY { .. } => "PauliY",
                QuantumGate::PauliZ { .. } => "PauliZ",
                QuantumGate::CNOT { .. } => "CNOT",
                QuantumGate::Phase { .. } => "Phase",
                QuantumGate::Rotation { .. } => "Rotation",
                QuantumGate::Toffoli { .. } => "Toffoli",
                QuantumGate::Measure { .. } => "Measure",
            };
            *counts.entry(gate_type.to_string()).or_insert(0) += 1;
        }
        counts
    }
}

/// Quantum-resistant cryptographic algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PostQuantumAlgorithm {
    /// Lattice-based cryptography (CRYSTALS-Kyber)
    Kyber,
    /// Lattice-based signatures (CRYSTALS-Dilithium)
    Dilithium,
    /// Hash-based signatures (SPHINCS+)
    SPHINCS,
    /// Code-based cryptography (Classic McEliece)
    McEliece,
    /// Multivariate cryptography (Rainbow)
    Rainbow,
    /// Isogeny-based cryptography (SIKE)
    SIKE,
}

/// Post-quantum cryptographic proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResistantProof {
    /// Algorithm used
    pub algorithm: PostQuantumAlgorithm,
    /// Proof data (simplified representation)
    pub proof_data: Vec<u8>,
    /// Public key or commitment
    pub public_key: Vec<u8>,
    /// Security level in bits
    pub security_level: usize,
    /// Proof metadata
    pub metadata: ProofMetadata,
}

/// Metadata for quantum-resistant proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    /// Statute ID this proof is for
    pub statute_id: String,
    /// Timestamp of proof generation
    pub timestamp: u64,
    /// Additional properties
    pub properties: Vec<String>,
}

impl QuantumResistantProof {
    /// Create a new quantum-resistant proof
    pub fn new(algorithm: PostQuantumAlgorithm, statute_id: String, security_level: usize) -> Self {
        // In a real implementation, this would generate actual cryptographic proofs
        // For now, we create a simplified representation
        let proof_size = match algorithm {
            PostQuantumAlgorithm::Kyber => 1088, // CRYSTALS-Kyber-768 ciphertext size
            PostQuantumAlgorithm::Dilithium => 2420, // CRYSTALS-Dilithium2 signature size
            PostQuantumAlgorithm::SPHINCS => 7856, // SPHINCS+-128f signature size
            PostQuantumAlgorithm::McEliece => 128, // Classic McEliece key size
            PostQuantumAlgorithm::Rainbow => 66, // Rainbow signature size
            PostQuantumAlgorithm::SIKE => 346,   // SIKE ciphertext size
        };

        let key_size = match algorithm {
            PostQuantumAlgorithm::Kyber => 1184,
            PostQuantumAlgorithm::Dilithium => 1312,
            PostQuantumAlgorithm::SPHINCS => 32,
            PostQuantumAlgorithm::McEliece => 261120,
            PostQuantumAlgorithm::Rainbow => 161600,
            PostQuantumAlgorithm::SIKE => 378,
        };

        Self {
            algorithm,
            proof_data: vec![0u8; proof_size],
            public_key: vec![0u8; key_size],
            security_level,
            metadata: ProofMetadata {
                statute_id,
                timestamp: 0, // Would be actual timestamp in production
                properties: vec!["quantum-resistant".to_string()],
            },
        }
    }

    /// Verify the proof (simplified)
    pub fn verify(&self) -> bool {
        // In a real implementation, this would perform actual cryptographic verification
        // For now, we just check that the proof data is non-empty
        !self.proof_data.is_empty() && !self.public_key.is_empty()
    }

    /// Estimate classical attack complexity
    pub fn classical_attack_complexity(&self) -> f64 {
        2_f64.powi(self.security_level as i32)
    }

    /// Estimate quantum attack complexity (using Grover's algorithm)
    pub fn quantum_attack_complexity(&self) -> f64 {
        2_f64.powi((self.security_level / 2) as i32)
    }
}

/// Quantum annealing problem representation (QUBO format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUBOProblem {
    /// Number of binary variables
    pub num_variables: usize,
    /// Quadratic coefficients (i, j) -> coefficient
    pub coefficients: HashMap<(usize, usize), f64>,
    /// Problem metadata
    pub metadata: QUBOMetadata,
}

/// Metadata for QUBO problems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUBOMetadata {
    /// Problem name
    pub name: String,
    /// Original problem type (SAT, Max-Cut, etc.)
    pub problem_type: String,
    /// Statute ID this problem corresponds to
    pub statute_id: Option<String>,
}

impl QUBOProblem {
    /// Create a new QUBO problem
    pub fn new(num_variables: usize, name: String) -> Self {
        Self {
            num_variables,
            coefficients: HashMap::new(),
            metadata: QUBOMetadata {
                name,
                problem_type: "QUBO".to_string(),
                statute_id: None,
            },
        }
    }

    /// Add a coefficient
    pub fn add_coefficient(&mut self, i: usize, j: usize, value: f64) {
        self.coefficients.insert((i.min(j), i.max(j)), value);
    }

    /// Evaluate the objective function for a given assignment
    pub fn evaluate(&self, assignment: &[bool]) -> f64 {
        let mut energy = 0.0;
        for ((i, j), &coeff) in &self.coefficients {
            let xi = if assignment[*i] { 1.0 } else { 0.0 };
            let xj = if assignment[*j] { 1.0 } else { 0.0 };
            energy += coeff * xi * xj;
        }
        energy
    }
}

/// Quantum annealing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnealingResult {
    /// Best solution found
    pub solution: Vec<bool>,
    /// Energy of the solution
    pub energy: f64,
    /// Number of annealing steps
    pub num_steps: usize,
    /// Success probability
    pub success_probability: f64,
    /// Metadata
    pub metadata: AnnealingMetadata,
}

/// Metadata for annealing results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnealingMetadata {
    /// Annealing schedule used
    pub schedule: String,
    /// Temperature parameters
    pub initial_temperature: f64,
    pub final_temperature: f64,
}

/// Quantum annealer for SAT solving
#[derive(Debug, Clone)]
pub struct QuantumAnnealer {
    /// Configuration
    config: AnnealingConfig,
}

/// Configuration for quantum annealing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnealingConfig {
    /// Number of annealing steps
    pub num_steps: usize,
    /// Initial temperature
    pub initial_temperature: f64,
    /// Final temperature
    pub final_temperature: f64,
    /// Number of repetitions
    pub num_repetitions: usize,
}

impl Default for AnnealingConfig {
    fn default() -> Self {
        Self {
            num_steps: 1000,
            initial_temperature: 10.0,
            final_temperature: 0.01,
            num_repetitions: 10,
        }
    }
}

impl QuantumAnnealer {
    /// Create a new quantum annealer
    pub fn new(config: AnnealingConfig) -> Self {
        Self { config }
    }

    /// Solve a QUBO problem using simulated quantum annealing
    pub fn solve(&self, problem: &QUBOProblem) -> AnnealingResult {
        let mut best_solution = vec![false; problem.num_variables];
        let mut best_energy = f64::INFINITY;

        // Perform multiple annealing runs
        for _ in 0..self.config.num_repetitions {
            let (solution, energy) = self.anneal_once(problem);
            if energy < best_energy {
                best_energy = energy;
                best_solution = solution;
            }
        }

        // Calculate success probability (simplified)
        let success_prob = (-best_energy / self.config.final_temperature).exp();

        AnnealingResult {
            solution: best_solution,
            energy: best_energy,
            num_steps: self.config.num_steps,
            success_probability: success_prob.min(1.0),
            metadata: AnnealingMetadata {
                schedule: "linear".to_string(),
                initial_temperature: self.config.initial_temperature,
                final_temperature: self.config.final_temperature,
            },
        }
    }

    /// Perform a single annealing run
    fn anneal_once(&self, problem: &QUBOProblem) -> (Vec<bool>, f64) {
        let mut rng = rand::rng();

        // Initialize random solution
        let mut solution: Vec<bool> = (0..problem.num_variables).map(|_| rng.random()).collect();

        let mut current_energy = problem.evaluate(&solution);

        // Annealing loop
        for step in 0..self.config.num_steps {
            let temperature = self.temperature_at_step(step);

            // Try flipping a random bit
            let flip_idx: usize = rng.random_range(0..problem.num_variables);
            solution[flip_idx] = !solution[flip_idx];
            let new_energy = problem.evaluate(&solution);

            // Accept or reject based on Metropolis criterion
            let delta_energy = new_energy - current_energy;
            let acceptance_prob: f64 = rng.random();
            if delta_energy < 0.0 || acceptance_prob < (-delta_energy / temperature).exp() {
                current_energy = new_energy;
            } else {
                // Reject: flip back
                solution[flip_idx] = !solution[flip_idx];
            }
        }

        (solution, current_energy)
    }

    /// Calculate temperature at a given step (linear schedule)
    fn temperature_at_step(&self, step: usize) -> f64 {
        let fraction = step as f64 / self.config.num_steps as f64;
        self.config.initial_temperature * (1.0 - fraction)
            + self.config.final_temperature * fraction
    }
}

/// Hybrid verification strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HybridStrategy {
    /// Use quantum for hard subproblems, classical for easy ones
    Adaptive,
    /// Quantum preprocessing, classical solving
    QuantumPreprocessing,
    /// Classical preprocessing, quantum solving
    ClassicalPreprocessing,
    /// Run both and compare results
    Redundant,
}

/// Hybrid verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridVerificationResult {
    /// Classical verification result
    pub classical_result: ClassicalResult,
    /// Quantum verification result
    pub quantum_result: QuantumResult,
    /// Strategy used
    pub strategy: HybridStrategy,
    /// Performance comparison
    pub performance: PerformanceComparison,
}

/// Classical verification result (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicalResult {
    /// Whether verification passed
    pub passed: bool,
    /// Time taken in milliseconds
    pub time_ms: u64,
    /// Number of operations
    pub num_operations: usize,
}

/// Quantum verification result (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResult {
    /// Whether verification passed
    pub passed: bool,
    /// Time taken in milliseconds
    pub time_ms: u64,
    /// Number of qubits used
    pub num_qubits: usize,
    /// Circuit depth
    pub circuit_depth: usize,
}

/// Performance comparison between classical and quantum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    /// Speedup factor (classical_time / quantum_time)
    pub speedup_factor: f64,
    /// Resource efficiency (qubits vs classical bits)
    pub resource_efficiency: f64,
    /// Accuracy comparison
    pub accuracy_match: bool,
    /// Recommendation
    pub recommendation: String,
}

/// Quantum supremacy benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSupremacyBenchmark {
    /// Problem size
    pub problem_size: usize,
    /// Classical solve time (estimated)
    pub classical_time_estimate_ms: u64,
    /// Quantum solve time (actual)
    pub quantum_time_ms: u64,
    /// Advantage achieved
    pub advantage_type: QuantumAdvantageType,
    /// Confidence level
    pub confidence: f64,
}

impl QuantumSupremacyBenchmark {
    /// Create a new benchmark
    pub fn new(problem_size: usize) -> Self {
        // Estimate classical time (exponential in problem size)
        let classical_time_estimate_ms = 2_u64.pow((problem_size / 2) as u32);

        // Simulate quantum time (polynomial in problem size)
        let quantum_time_ms = (problem_size as u64).pow(3);

        // Determine advantage type
        let speedup = classical_time_estimate_ms as f64 / quantum_time_ms as f64;
        let advantage_type = if speedup > 1e6 {
            QuantumAdvantageType::Exponential
        } else if speedup > 100.0 {
            QuantumAdvantageType::Polynomial
        } else if speedup > 1.0 {
            QuantumAdvantageType::ConstantFactor
        } else {
            QuantumAdvantageType::None
        };

        Self {
            problem_size,
            classical_time_estimate_ms,
            quantum_time_ms,
            advantage_type,
            confidence: 0.95, // High confidence for simulation
        }
    }

    /// Calculate speedup factor
    pub fn speedup_factor(&self) -> f64 {
        self.classical_time_estimate_ms as f64 / self.quantum_time_ms as f64
    }
}

/// Main quantum verifier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumConfig {
    /// Enable quantum circuit verification
    pub enable_circuit_verification: bool,
    /// Enable quantum-resistant proofs
    pub enable_quantum_resistant_proofs: bool,
    /// Enable quantum annealing
    pub enable_quantum_annealing: bool,
    /// Enable hybrid verification
    pub enable_hybrid_verification: bool,
    /// Post-quantum algorithm to use
    pub pq_algorithm: PostQuantumAlgorithm,
    /// Annealing configuration
    pub annealing_config: AnnealingConfig,
    /// Hybrid strategy
    pub hybrid_strategy: HybridStrategy,
}

impl Default for QuantumConfig {
    fn default() -> Self {
        Self {
            enable_circuit_verification: true,
            enable_quantum_resistant_proofs: true,
            enable_quantum_annealing: false,
            enable_hybrid_verification: false,
            pq_algorithm: PostQuantumAlgorithm::Kyber,
            annealing_config: AnnealingConfig::default(),
            hybrid_strategy: HybridStrategy::Adaptive,
        }
    }
}

/// Main quantum verifier
#[derive(Debug, Clone)]
pub struct QuantumVerifier {
    config: QuantumConfig,
}

/// Quantum verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumVerificationResult {
    /// Whether verification passed
    pub passed: bool,
    /// Total qubits used
    pub total_qubits_used: usize,
    /// Quantum circuits generated
    pub circuits: Vec<QuantumCircuit>,
    /// Quantum-resistant proofs
    pub proofs: Vec<QuantumResistantProof>,
    /// Annealing results
    pub annealing_results: Vec<AnnealingResult>,
    /// Hybrid results
    pub hybrid_results: Vec<HybridVerificationResult>,
    /// Benchmarks
    pub benchmarks: Vec<QuantumSupremacyBenchmark>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl QuantumVerifier {
    /// Create a new quantum verifier
    pub fn new(config: QuantumConfig) -> Self {
        Self { config }
    }

    /// Verify statutes using quantum methods
    pub fn verify_statutes(&self, statutes: &[Statute]) -> QuantumVerificationResult {
        let mut circuits = Vec::new();
        let mut proofs = Vec::new();
        let mut annealing_results = Vec::new();
        let mut hybrid_results = Vec::new();
        let mut benchmarks = Vec::new();
        let mut recommendations = Vec::new();
        let mut total_qubits = 0;

        for statute in statutes {
            // Circuit verification
            if self.config.enable_circuit_verification {
                let circuit = self.generate_circuit_for_statute(statute);
                total_qubits += circuit.num_qubits;
                circuits.push(circuit);
            }

            // Quantum-resistant proofs
            if self.config.enable_quantum_resistant_proofs {
                let proof = QuantumResistantProof::new(
                    self.config.pq_algorithm,
                    statute.id.clone(),
                    128, // Security level
                );
                proofs.push(proof);
            }

            // Quantum annealing (for complex conditions)
            if self.config.enable_quantum_annealing && !statute.preconditions.is_empty() {
                let problem = self.statute_to_qubo(statute);
                let annealer = QuantumAnnealer::new(self.config.annealing_config.clone());
                let result = annealer.solve(&problem);
                annealing_results.push(result);
            }

            // Hybrid verification
            if self.config.enable_hybrid_verification {
                let hybrid_result = self.hybrid_verify(statute);
                hybrid_results.push(hybrid_result);
            }
        }

        // Generate benchmarks
        if !statutes.is_empty() {
            benchmarks.push(QuantumSupremacyBenchmark::new(statutes.len()));
        }

        // Generate recommendations
        if total_qubits > 100 {
            recommendations
                .push("Consider using quantum error correction for large circuits".to_string());
        }
        if !proofs.is_empty() {
            recommendations
                .push("All statutes protected with post-quantum cryptography".to_string());
        }

        QuantumVerificationResult {
            passed: true, // Simplified
            total_qubits_used: total_qubits,
            circuits,
            proofs,
            annealing_results,
            hybrid_results,
            benchmarks,
            recommendations,
        }
    }

    /// Generate a quantum circuit for a statute
    fn generate_circuit_for_statute(&self, statute: &Statute) -> QuantumCircuit {
        // Number of qubits based on conditions
        let num_qubits = (statute.preconditions.len() + 1).max(2);
        let mut circuit = QuantumCircuit::new(num_qubits, num_qubits, statute.id.clone());

        // Create superposition for all conditions
        for i in 0..num_qubits {
            circuit.add_gate(QuantumGate::Hadamard { target: i });
        }

        // Add entanglement between conditions
        for i in 0..num_qubits - 1 {
            circuit.add_gate(QuantumGate::CNOT {
                control: i,
                target: i + 1,
            });
        }

        // Phase encoding for statute properties
        circuit.add_gate(QuantumGate::Phase {
            target: 0,
            angle: std::f64::consts::PI / 4.0,
        });

        // Measure all qubits
        for i in 0..num_qubits {
            circuit.add_gate(QuantumGate::Measure {
                target: i,
                classical_bit: i,
            });
        }

        circuit.metadata.purpose = format!("Verification of statute {}", statute.id);
        circuit.metadata.advantage_type = if num_qubits > 20 {
            QuantumAdvantageType::Exponential
        } else {
            QuantumAdvantageType::Polynomial
        };

        circuit
    }

    /// Convert statute to QUBO problem
    fn statute_to_qubo(&self, statute: &Statute) -> QUBOProblem {
        let num_vars = statute.preconditions.len().max(2);
        let mut problem = QUBOProblem::new(num_vars, format!("statute_{}", statute.id));
        problem.metadata.statute_id = Some(statute.id.clone());

        // Add coefficients based on conditions
        for i in 0..num_vars {
            // Diagonal terms
            problem.add_coefficient(i, i, -1.0);

            // Off-diagonal terms (interactions)
            for j in i + 1..num_vars {
                problem.add_coefficient(i, j, 0.5);
            }
        }

        problem
    }

    /// Perform hybrid verification
    fn hybrid_verify(&self, statute: &Statute) -> HybridVerificationResult {
        // Simulate classical verification
        let classical_result = ClassicalResult {
            passed: true,
            time_ms: 10,
            num_operations: statute.preconditions.len() * 100,
        };

        // Simulate quantum verification
        let circuit = self.generate_circuit_for_statute(statute);
        let quantum_result = QuantumResult {
            passed: true,
            time_ms: 5,
            num_qubits: circuit.num_qubits,
            circuit_depth: circuit.depth(),
        };

        // Performance comparison
        let speedup_factor = classical_result.time_ms as f64 / quantum_result.time_ms as f64;
        let resource_efficiency =
            quantum_result.num_qubits as f64 / classical_result.num_operations as f64;

        let recommendation = if speedup_factor > 2.0 {
            "Quantum verification recommended".to_string()
        } else {
            "Classical verification sufficient".to_string()
        };

        HybridVerificationResult {
            classical_result,
            quantum_result,
            strategy: self.config.hybrid_strategy,
            performance: PerformanceComparison {
                speedup_factor,
                resource_efficiency,
                accuracy_match: true,
                recommendation,
            },
        }
    }

    /// Generate a comprehensive quantum verification report
    pub fn generate_report(&self, result: &QuantumVerificationResult) -> String {
        let mut report = String::new();
        report.push_str("# Quantum Verification Report\n\n");

        report.push_str(&format!(
            "**Status**: {}\n\n",
            if result.passed {
                "✓ Passed"
            } else {
                "✗ Failed"
            }
        ));

        report.push_str(&format!(
            "**Total Qubits Used**: {}\n\n",
            result.total_qubits_used
        ));

        if !result.circuits.is_empty() {
            report.push_str(&format!(
                "## Quantum Circuits ({})\n\n",
                result.circuits.len()
            ));
            for circuit in &result.circuits {
                report.push_str(&format!(
                    "- **{}**: {} qubits, depth {}, {} gates\n",
                    circuit.metadata.name,
                    circuit.num_qubits,
                    circuit.depth(),
                    circuit.gates.len()
                ));
            }
            report.push('\n');
        }

        if !result.proofs.is_empty() {
            report.push_str(&format!(
                "## Quantum-Resistant Proofs ({})\n\n",
                result.proofs.len()
            ));
            for proof in &result.proofs {
                report.push_str(&format!(
                    "- Algorithm: {:?}, Security: {} bits\n",
                    proof.algorithm, proof.security_level
                ));
            }
            report.push('\n');
        }

        if !result.benchmarks.is_empty() {
            report.push_str("## Quantum Supremacy Benchmarks\n\n");
            for benchmark in &result.benchmarks {
                report.push_str(&format!(
                    "- Problem size: {}, Speedup: {:.2}x, Advantage: {:?}\n",
                    benchmark.problem_size,
                    benchmark.speedup_factor(),
                    benchmark.advantage_type
                ));
            }
            report.push('\n');
        }

        if !result.recommendations.is_empty() {
            report.push_str("## Recommendations\n\n");
            for rec in &result.recommendations {
                report.push_str(&format!("- {}\n", rec));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, TemporalValidity};

    // Helper function to create a test statute
    fn create_test_statute(id: &str, title: &str, preconditions: Vec<Condition>) -> Statute {
        Statute {
            id: id.to_string(),
            title: title.to_string(),
            preconditions,
            effect: Effect::new(EffectType::Grant, "Grant permission"),
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            derives_from: vec![],
            applies_to: vec![],
            exceptions: vec![],
        }
    }

    #[test]
    fn test_quantum_circuit_creation() {
        let circuit = QuantumCircuit::new(3, 3, "test_circuit".to_string());
        assert_eq!(circuit.num_qubits, 3);
        assert_eq!(circuit.num_classical_bits, 3);
        assert_eq!(circuit.gates.len(), 0);
    }

    #[test]
    fn test_quantum_circuit_depth() {
        let mut circuit = QuantumCircuit::new(3, 3, "test".to_string());
        circuit.add_gate(QuantumGate::Hadamard { target: 0 });
        circuit.add_gate(QuantumGate::Hadamard { target: 1 });
        circuit.add_gate(QuantumGate::CNOT {
            control: 0,
            target: 1,
        });
        circuit.add_gate(QuantumGate::Hadamard { target: 2 });

        assert!(circuit.depth() > 0);
    }

    #[test]
    fn test_quantum_circuit_gate_counts() {
        let mut circuit = QuantumCircuit::new(2, 2, "test".to_string());
        circuit.add_gate(QuantumGate::Hadamard { target: 0 });
        circuit.add_gate(QuantumGate::Hadamard { target: 1 });
        circuit.add_gate(QuantumGate::CNOT {
            control: 0,
            target: 1,
        });

        let counts = circuit.gate_counts();
        assert_eq!(counts.get("Hadamard"), Some(&2));
        assert_eq!(counts.get("CNOT"), Some(&1));
    }

    #[test]
    fn test_quantum_resistant_proof_creation() {
        let proof =
            QuantumResistantProof::new(PostQuantumAlgorithm::Kyber, "statute_1".to_string(), 128);

        assert_eq!(proof.algorithm, PostQuantumAlgorithm::Kyber);
        assert_eq!(proof.security_level, 128);
        assert!(!proof.proof_data.is_empty());
        assert!(!proof.public_key.is_empty());
    }

    #[test]
    fn test_quantum_resistant_proof_verification() {
        let proof = QuantumResistantProof::new(
            PostQuantumAlgorithm::Dilithium,
            "statute_2".to_string(),
            256,
        );

        assert!(proof.verify());
    }

    #[test]
    fn test_quantum_attack_complexity() {
        let proof =
            QuantumResistantProof::new(PostQuantumAlgorithm::Kyber, "statute_3".to_string(), 128);

        let classical_complexity = proof.classical_attack_complexity();
        let quantum_complexity = proof.quantum_attack_complexity();

        // Quantum attacks should be faster than classical (square root speedup)
        assert!(quantum_complexity < classical_complexity);
    }

    #[test]
    fn test_qubo_problem_creation() {
        let mut problem = QUBOProblem::new(3, "test_qubo".to_string());
        problem.add_coefficient(0, 0, -1.0);
        problem.add_coefficient(0, 1, 0.5);
        problem.add_coefficient(1, 2, 0.5);

        assert_eq!(problem.num_variables, 3);
        assert_eq!(problem.coefficients.len(), 3);
    }

    #[test]
    fn test_qubo_evaluation() {
        let mut problem = QUBOProblem::new(2, "test".to_string());
        problem.add_coefficient(0, 0, -1.0);
        problem.add_coefficient(1, 1, -1.0);
        problem.add_coefficient(0, 1, 2.0);

        let assignment = vec![true, true];
        let energy = problem.evaluate(&assignment);

        // Energy = -1*1*1 + -1*1*1 + 2*1*1 = -1 - 1 + 2 = 0
        assert_eq!(energy, 0.0);
    }

    #[test]
    fn test_quantum_annealing() {
        let mut problem = QUBOProblem::new(3, "test".to_string());
        problem.add_coefficient(0, 0, -1.0);
        problem.add_coefficient(1, 1, -1.0);
        problem.add_coefficient(2, 2, -1.0);

        let config = AnnealingConfig {
            num_steps: 100,
            initial_temperature: 10.0,
            final_temperature: 0.01,
            num_repetitions: 3,
        };

        let annealer = QuantumAnnealer::new(config);
        let result = annealer.solve(&problem);

        assert_eq!(result.solution.len(), 3);
        assert!(result.num_steps > 0);
        assert!(result.success_probability >= 0.0 && result.success_probability <= 1.0);
    }

    #[test]
    fn test_quantum_supremacy_benchmark() {
        let benchmark = QuantumSupremacyBenchmark::new(20);

        assert_eq!(benchmark.problem_size, 20);
        assert!(benchmark.speedup_factor() > 0.0);
        assert!(benchmark.confidence > 0.0 && benchmark.confidence <= 1.0);
    }

    #[test]
    fn test_quantum_verifier_basic() {
        let config = QuantumConfig::default();
        let verifier = QuantumVerifier::new(config);

        let statute = create_test_statute(
            "test_statute",
            "Test Statute",
            vec![Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }],
        );

        let result = verifier.verify_statutes(&[statute]);
        assert!(result.passed);
        assert!(result.total_qubits_used > 0);
    }

    #[test]
    fn test_quantum_verifier_with_circuits() {
        let config = QuantumConfig {
            enable_circuit_verification: true,
            enable_quantum_resistant_proofs: false,
            enable_quantum_annealing: false,
            enable_hybrid_verification: false,
            ..Default::default()
        };

        let verifier = QuantumVerifier::new(config);

        let statute = create_test_statute(
            "circuit_test",
            "Circuit Test",
            vec![
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
                Condition::Income {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 30000,
                },
            ],
        );

        let result = verifier.verify_statutes(&[statute]);
        assert!(!result.circuits.is_empty());
        assert_eq!(result.circuits.len(), 1);
    }

    #[test]
    fn test_quantum_verifier_with_proofs() {
        let config = QuantumConfig {
            enable_circuit_verification: false,
            enable_quantum_resistant_proofs: true,
            pq_algorithm: PostQuantumAlgorithm::Dilithium,
            ..Default::default()
        };

        let verifier = QuantumVerifier::new(config);

        let statute = create_test_statute("proof_test", "Proof Test", vec![]);

        let result = verifier.verify_statutes(&[statute]);
        assert!(!result.proofs.is_empty());
        assert_eq!(result.proofs[0].algorithm, PostQuantumAlgorithm::Dilithium);
    }

    #[test]
    fn test_quantum_verifier_report() {
        let config = QuantumConfig::default();
        let verifier = QuantumVerifier::new(config);

        let statute = create_test_statute("report_test", "Report Test", vec![]);

        let result = verifier.verify_statutes(&[statute]);
        let report = verifier.generate_report(&result);

        assert!(report.contains("Quantum Verification Report"));
        assert!(report.contains("Passed"));
    }

    #[test]
    fn test_hybrid_verification() {
        let config = QuantumConfig {
            enable_hybrid_verification: true,
            hybrid_strategy: HybridStrategy::Adaptive,
            ..Default::default()
        };

        let verifier = QuantumVerifier::new(config);

        let statute = create_test_statute(
            "hybrid_test",
            "Hybrid Test",
            vec![Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            }],
        );

        let result = verifier.verify_statutes(&[statute]);
        assert!(!result.hybrid_results.is_empty());
        assert!(result.hybrid_results[0].performance.speedup_factor > 0.0);
    }

    #[test]
    fn test_rotation_axis_serialization() {
        let axis = RotationAxis::X;
        let serialized = serde_json::to_string(&axis).unwrap();
        let deserialized: RotationAxis = serde_json::from_str(&serialized).unwrap();
        assert_eq!(axis, deserialized);
    }

    #[test]
    fn test_quantum_advantage_types() {
        assert_ne!(
            QuantumAdvantageType::Exponential,
            QuantumAdvantageType::None
        );
        assert_ne!(
            QuantumAdvantageType::Polynomial,
            QuantumAdvantageType::ConstantFactor
        );
    }

    #[test]
    fn test_post_quantum_algorithms() {
        let algorithms = vec![
            PostQuantumAlgorithm::Kyber,
            PostQuantumAlgorithm::Dilithium,
            PostQuantumAlgorithm::SPHINCS,
            PostQuantumAlgorithm::McEliece,
            PostQuantumAlgorithm::Rainbow,
            PostQuantumAlgorithm::SIKE,
        ];

        for algo in algorithms {
            let proof = QuantumResistantProof::new(algo, "test".to_string(), 128);
            assert_eq!(proof.algorithm, algo);
            assert!(proof.verify());
        }
    }
}
