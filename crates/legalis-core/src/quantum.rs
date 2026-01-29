//! Quantum-Ready Legal Logic (v0.3.3)
//!
//! This module provides quantum computing primitives and quantum-inspired algorithms
//! for legal reasoning. It includes:
//!
//! - Quantum circuit generation for legal decision problems
//! - Quantum-inspired optimization (annealing, QAOA)
//! - Hybrid classical-quantum evaluation pipelines
//! - Post-quantum cryptographic proofs
//! - Quantum constraint satisfaction solvers
//!
//! **Note**: This module provides simulation frameworks and interfaces. Actual quantum
//! hardware integration requires additional backend implementations.

use crate::{Condition, Statute};
use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// 1. QUANTUM CIRCUIT GENERATION FOR LEGAL PROBLEMS
// ============================================================================

/// Quantum gate types for legal circuit construction
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QuantumGate {
    /// Hadamard gate - creates superposition
    H(usize),
    /// Pauli-X gate - NOT operation
    X(usize),
    /// Pauli-Y gate
    Y(usize),
    /// Pauli-Z gate
    Z(usize),
    /// Controlled-NOT gate
    CNOT(usize, usize),
    /// Rotation around Z-axis
    RZ(usize, f64),
    /// Rotation around Y-axis
    RY(usize, f64),
    /// Toffoli gate (CCNOT)
    Toffoli(usize, usize, usize),
    /// Measurement gate
    Measure(usize),
}

impl fmt::Display for QuantumGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantumGate::H(q) => write!(f, "H(q{})", q),
            QuantumGate::X(q) => write!(f, "X(q{})", q),
            QuantumGate::Y(q) => write!(f, "Y(q{})", q),
            QuantumGate::Z(q) => write!(f, "Z(q{})", q),
            QuantumGate::CNOT(c, t) => write!(f, "CNOT(q{}, q{})", c, t),
            QuantumGate::RZ(q, theta) => write!(f, "RZ(q{}, {})", q, theta),
            QuantumGate::RY(q, theta) => write!(f, "RY(q{}, {})", q, theta),
            QuantumGate::Toffoli(c1, c2, t) => write!(f, "Toffoli(q{}, q{}, q{})", c1, c2, t),
            QuantumGate::Measure(q) => write!(f, "Measure(q{})", q),
        }
    }
}

/// Quantum circuit for legal decision problems
///
/// # Example
///
/// ```
/// use legalis_core::quantum::{QuantumCircuit, QuantumGate};
///
/// let mut circuit = QuantumCircuit::new(2);
/// circuit.add_gate(QuantumGate::H(0)); // Superposition for uncertainty
/// circuit.add_gate(QuantumGate::CNOT(0, 1)); // Entangle conditions
/// circuit.add_gate(QuantumGate::Measure(0));
///
/// assert_eq!(circuit.num_qubits(), 2);
/// assert_eq!(circuit.num_gates(), 3);
///
/// let qiskit = circuit.to_qiskit();
/// assert!(qiskit.contains("QuantumCircuit(2)"));
/// assert!(qiskit.contains("qc.h(0)"));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QuantumCircuit {
    num_qubits: usize,
    gates: Vec<QuantumGate>,
    metadata: HashMap<String, String>,
}

impl QuantumCircuit {
    /// Create a new quantum circuit with specified number of qubits
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a quantum gate to the circuit
    pub fn add_gate(&mut self, gate: QuantumGate) {
        self.gates.push(gate);
    }

    /// Get the number of qubits in the circuit
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the number of gates in the circuit
    pub fn num_gates(&self) -> usize {
        self.gates.len()
    }

    /// Add metadata to the circuit
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Convert condition to quantum gates (superposition for uncertainty)
    pub fn from_condition(condition: &Condition) -> Self {
        let mut circuit = Self::new(2);
        circuit.add_metadata("source".to_string(), "legal_condition".to_string());

        // Create superposition for uncertain conditions
        circuit.add_gate(QuantumGate::H(0));

        // Apply condition-specific gates
        match condition {
            Condition::And(..) | Condition::Or(..) => {
                // Multi-qubit operations for compound conditions
                circuit.add_gate(QuantumGate::CNOT(0, 1));
            }
            Condition::Not(_) => {
                // Negation via X gate
                circuit.add_gate(QuantumGate::X(0));
            }
            _ => {
                // Simple condition - single qubit rotation
                circuit.add_gate(QuantumGate::RZ(0, std::f64::consts::PI / 4.0));
            }
        }

        circuit.add_gate(QuantumGate::Measure(0));
        circuit
    }

    /// Export to Qiskit-compatible Python code
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::quantum::{QuantumCircuit, QuantumGate};
    ///
    /// let mut circuit = QuantumCircuit::new(3);
    /// circuit.add_gate(QuantumGate::H(0));
    /// circuit.add_gate(QuantumGate::CNOT(0, 1));
    /// circuit.add_gate(QuantumGate::X(2));
    ///
    /// let qiskit_code = circuit.to_qiskit();
    /// assert!(qiskit_code.contains("from qiskit import QuantumCircuit"));
    /// assert!(qiskit_code.contains("qc = QuantumCircuit(3)"));
    /// assert!(qiskit_code.contains("qc.h(0)"));
    /// assert!(qiskit_code.contains("qc.cx(0, 1)"));
    /// assert!(qiskit_code.contains("qc.x(2)"));
    /// ```
    pub fn to_qiskit(&self) -> String {
        let mut code = String::from("from qiskit import QuantumCircuit\n\n");
        code.push_str(&format!("qc = QuantumCircuit({})\n", self.num_qubits));

        for gate in &self.gates {
            match gate {
                QuantumGate::H(q) => code.push_str(&format!("qc.h({})\n", q)),
                QuantumGate::X(q) => code.push_str(&format!("qc.x({})\n", q)),
                QuantumGate::Y(q) => code.push_str(&format!("qc.y({})\n", q)),
                QuantumGate::Z(q) => code.push_str(&format!("qc.z({})\n", q)),
                QuantumGate::CNOT(c, t) => code.push_str(&format!("qc.cx({}, {})\n", c, t)),
                QuantumGate::RZ(q, theta) => code.push_str(&format!("qc.rz({}, {})\n", theta, q)),
                QuantumGate::RY(q, theta) => code.push_str(&format!("qc.ry({}, {})\n", theta, q)),
                QuantumGate::Toffoli(c1, c2, t) => {
                    code.push_str(&format!("qc.ccx({}, {}, {})\n", c1, c2, t))
                }
                QuantumGate::Measure(q) => code.push_str(&format!("qc.measure({}, {})\n", q, q)),
            }
        }

        code
    }
}

// ============================================================================
// 2. QUANTUM-INSPIRED OPTIMIZATION ALGORITHMS
// ============================================================================

/// QUBO (Quadratic Unconstrained Binary Optimization) problem representation
///
/// Used to convert legal constraint satisfaction problems to quantum annealing format.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QuboProblem {
    /// Quadratic coefficients matrix
    pub q_matrix: HashMap<(usize, usize), f64>,
    /// Number of variables
    pub num_vars: usize,
}

impl QuboProblem {
    /// Create a new QUBO problem
    pub fn new(num_vars: usize) -> Self {
        Self {
            q_matrix: HashMap::new(),
            num_vars,
        }
    }

    /// Add a quadratic coefficient
    pub fn add_coefficient(&mut self, i: usize, j: usize, value: f64) {
        self.q_matrix.insert((i, j), value);
    }

    /// Evaluate the QUBO energy for a given binary solution
    pub fn evaluate(&self, solution: &[bool]) -> f64 {
        let mut energy = 0.0;

        for ((i, j), coeff) in &self.q_matrix {
            let x_i = if solution[*i] { 1.0 } else { 0.0 };
            let x_j = if solution[*j] { 1.0 } else { 0.0 };
            energy += coeff * x_i * x_j;
        }

        energy
    }
}

/// Quantum-inspired annealing optimizer for constraint satisfaction
///
/// # Example
///
/// ```
/// use legalis_core::quantum::QuantumAnnealingOptimizer;
///
/// let optimizer = QuantumAnnealingOptimizer::new(10, 100);
/// assert_eq!(optimizer.num_vars(), 10);
/// assert_eq!(optimizer.max_iterations(), 100);
///
/// // Minimize x0 AND x1 (both should be 0)
/// let mut problem = optimizer.create_problem();
/// problem.add_coefficient(0, 0, -1.0); // Prefer x0 = 1
/// problem.add_coefficient(1, 1, -1.0); // Prefer x1 = 1
/// problem.add_coefficient(0, 1, 2.0);  // Penalty for both = 1
///
/// let solution = optimizer.solve(&problem);
/// assert_eq!(solution.len(), 10);
/// ```
#[derive(Debug, Clone)]
pub struct QuantumAnnealingOptimizer {
    num_vars: usize,
    max_iterations: usize,
    temperature_initial: f64,
    temperature_final: f64,
}

impl QuantumAnnealingOptimizer {
    /// Create a new quantum annealing optimizer
    pub fn new(num_vars: usize, max_iterations: usize) -> Self {
        Self {
            num_vars,
            max_iterations,
            temperature_initial: 100.0,
            temperature_final: 0.01,
        }
    }

    /// Create a QUBO problem compatible with this optimizer
    pub fn create_problem(&self) -> QuboProblem {
        QuboProblem::new(self.num_vars)
    }

    /// Get number of variables
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Get maximum iterations
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    /// Solve the QUBO problem using simulated annealing
    pub fn solve(&self, problem: &QuboProblem) -> Vec<bool> {
        // Initialize random solution
        let mut solution: Vec<bool> = (0..self.num_vars).map(|i| i % 2 == 0).collect();
        let mut best_solution = solution.clone();
        let mut best_energy = problem.evaluate(&solution);

        // Simulated annealing loop
        for iteration in 0..self.max_iterations {
            let temperature = self.temperature_schedule(iteration);

            // Flip a random bit
            let flip_idx = iteration % self.num_vars;
            solution[flip_idx] = !solution[flip_idx];

            let new_energy = problem.evaluate(&solution);
            let delta = new_energy - best_energy;

            // Accept if better, or probabilistically if worse
            if delta < 0.0 || self.acceptance_probability(delta, temperature) > 0.5 {
                best_energy = new_energy;
                best_solution = solution.clone();
            } else {
                // Revert flip
                solution[flip_idx] = !solution[flip_idx];
            }
        }

        best_solution
    }

    fn temperature_schedule(&self, iteration: usize) -> f64 {
        let progress = iteration as f64 / self.max_iterations as f64;
        self.temperature_initial * (1.0 - progress) + self.temperature_final * progress
    }

    fn acceptance_probability(&self, delta: f64, temperature: f64) -> f64 {
        (-delta / temperature).exp()
    }

    /// Convert statute constraints to QUBO problem
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    /// use legalis_core::quantum::QuantumAnnealingOptimizer;
    ///
    /// let statute = Statute::new(
    ///     "tax-credit",
    ///     "Tax Credit",
    ///     Effect::new(EffectType::Grant, "Tax benefit")
    /// )
    /// .with_precondition(Condition::Income {
    ///     operator: ComparisonOp::LessThan,
    ///     value: 50000
    /// });
    ///
    /// let optimizer = QuantumAnnealingOptimizer::new(5, 50);
    /// let qubo = optimizer.statute_to_qubo(&statute);
    /// assert_eq!(qubo.num_vars, 5);
    /// ```
    pub fn statute_to_qubo(&self, _statute: &Statute) -> QuboProblem {
        let mut problem = QuboProblem::new(self.num_vars);

        // Convert statute preconditions to QUBO constraints
        // For simulation: add simple constraint favoring certain patterns
        for i in 0..self.num_vars {
            problem.add_coefficient(i, i, -1.0); // Prefer 1s
        }

        // Add pairwise constraints
        for i in 0..self.num_vars.saturating_sub(1) {
            problem.add_coefficient(i, i + 1, 0.5);
        }

        problem
    }
}

/// QAOA (Quantum Approximate Optimization Algorithm) framework
#[derive(Debug, Clone)]
pub struct QaoaOptimizer {
    num_qubits: usize,
    num_layers: usize,
}

impl QaoaOptimizer {
    /// Create a new QAOA optimizer
    pub fn new(num_qubits: usize, num_layers: usize) -> Self {
        Self {
            num_qubits,
            num_layers,
        }
    }

    /// Generate QAOA circuit for optimization problem
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::quantum::QaoaOptimizer;
    ///
    /// let qaoa = QaoaOptimizer::new(4, 2);
    /// let circuit = qaoa.generate_circuit();
    ///
    /// assert_eq!(circuit.num_qubits(), 4);
    /// assert!(circuit.num_gates() > 0);
    /// ```
    pub fn generate_circuit(&self) -> QuantumCircuit {
        let mut circuit = QuantumCircuit::new(self.num_qubits);

        // Initial superposition layer
        for i in 0..self.num_qubits {
            circuit.add_gate(QuantumGate::H(i));
        }

        // QAOA layers
        for _layer in 0..self.num_layers {
            // Problem Hamiltonian layer (ZZ interactions)
            for i in 0..self.num_qubits.saturating_sub(1) {
                circuit.add_gate(QuantumGate::CNOT(i, i + 1));
                circuit.add_gate(QuantumGate::RZ(i + 1, 0.5));
                circuit.add_gate(QuantumGate::CNOT(i, i + 1));
            }

            // Mixer Hamiltonian layer (X rotations)
            for i in 0..self.num_qubits {
                circuit.add_gate(QuantumGate::RY(i, 0.3));
            }
        }

        // Measurement
        for i in 0..self.num_qubits {
            circuit.add_gate(QuantumGate::Measure(i));
        }

        circuit
    }
}

// ============================================================================
// 3. HYBRID CLASSICAL-QUANTUM EVALUATION
// ============================================================================

/// Result from hybrid quantum-classical evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HybridEvaluationResult {
    pub classical_result: bool,
    pub quantum_result: Option<bool>,
    pub confidence: f64,
    pub processing_time_ms: u64,
}

/// Trait for hybrid quantum-classical evaluation
///
/// Enables preprocessing with classical logic, quantum evaluation,
/// and classical post-processing.
pub trait HybridQuantumEvaluator {
    /// Classical preprocessing step
    fn classical_preprocess(&self, input: &Condition) -> Vec<bool>;

    /// Quantum evaluation step (simulated)
    fn quantum_evaluate(&self, preprocessed: &[bool]) -> Option<bool>;

    /// Classical post-processing step
    fn classical_postprocess(&self, quantum_result: Option<bool>) -> bool;

    /// Full hybrid evaluation pipeline
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::quantum::{HybridQuantumEvaluator, DefaultHybridEvaluator};
    /// use legalis_core::Condition;
    ///
    /// let evaluator = DefaultHybridEvaluator::new();
    /// let condition = Condition::custom("test_condition".to_string());
    ///
    /// let result = evaluator.evaluate_hybrid(&condition);
    /// assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    /// ```
    fn evaluate_hybrid(&self, condition: &Condition) -> HybridEvaluationResult {
        let start = std::time::Instant::now();

        let preprocessed = self.classical_preprocess(condition);
        let quantum_result = self.quantum_evaluate(&preprocessed);
        let classical_result = self.classical_postprocess(quantum_result);

        let confidence = if quantum_result.is_some() { 0.95 } else { 0.75 };

        HybridEvaluationResult {
            classical_result,
            quantum_result,
            confidence,
            processing_time_ms: start.elapsed().as_millis() as u64,
        }
    }
}

/// Default implementation of hybrid evaluator
#[derive(Debug, Clone)]
pub struct DefaultHybridEvaluator {
    use_quantum_fallback: bool,
}

impl DefaultHybridEvaluator {
    /// Create a new default hybrid evaluator
    pub fn new() -> Self {
        Self {
            use_quantum_fallback: true,
        }
    }
}

impl Default for DefaultHybridEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridQuantumEvaluator for DefaultHybridEvaluator {
    fn classical_preprocess(&self, condition: &Condition) -> Vec<bool> {
        // Convert condition to binary vector
        match condition {
            Condition::And(..) => vec![true; 2],
            Condition::Or(..) => vec![false; 2],
            _ => vec![true],
        }
    }

    fn quantum_evaluate(&self, preprocessed: &[bool]) -> Option<bool> {
        if !self.use_quantum_fallback {
            return None;
        }

        // Simulated quantum evaluation
        // In real implementation, this would interface with quantum backend
        Some(preprocessed.iter().any(|&x| x))
    }

    fn classical_postprocess(&self, quantum_result: Option<bool>) -> bool {
        quantum_result.unwrap_or(false)
    }
}

/// VQE (Variational Quantum Eigensolver) for legal entailment
#[derive(Debug, Clone)]
pub struct VqeEntailmentSolver {
    num_qubits: usize,
    num_iterations: usize,
}

impl VqeEntailmentSolver {
    /// Create a new VQE entailment solver
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::quantum::VqeEntailmentSolver;
    ///
    /// let vqe = VqeEntailmentSolver::new(3, 100);
    /// let circuit = vqe.variational_circuit();
    /// assert_eq!(circuit.num_qubits(), 3);
    /// ```
    pub fn new(num_qubits: usize, num_iterations: usize) -> Self {
        Self {
            num_qubits,
            num_iterations,
        }
    }

    /// Generate variational quantum circuit
    pub fn variational_circuit(&self) -> QuantumCircuit {
        let mut circuit = QuantumCircuit::new(self.num_qubits);

        // Variational ansatz
        for i in 0..self.num_qubits {
            circuit.add_gate(QuantumGate::RY(i, 0.5));
        }

        for i in 0..self.num_qubits.saturating_sub(1) {
            circuit.add_gate(QuantumGate::CNOT(i, i + 1));
        }

        circuit
    }

    /// Check if premise entails hypothesis (simulated)
    pub fn entails(&self, _premise: &Condition, _hypothesis: &Condition) -> bool {
        // Simulated VQE entailment check
        // Real implementation would run VQE optimization for self.num_iterations
        true
    }

    /// Get the number of iterations configured
    pub fn num_iterations(&self) -> usize {
        self.num_iterations
    }
}

// ============================================================================
// 4. QUANTUM-SAFE CRYPTOGRAPHIC PROOFS
// ============================================================================

/// Post-quantum signature schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PostQuantumScheme {
    /// NIST Dilithium (lattice-based)
    Dilithium,
    /// NIST Falcon (lattice-based)
    Falcon,
    /// NIST SPHINCS+ (hash-based)
    SphincsPlus,
    /// Kyber (lattice-based encryption)
    Kyber,
}

impl fmt::Display for PostQuantumScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostQuantumScheme::Dilithium => write!(f, "Dilithium"),
            PostQuantumScheme::Falcon => write!(f, "Falcon"),
            PostQuantumScheme::SphincsPlus => write!(f, "SPHINCS+"),
            PostQuantumScheme::Kyber => write!(f, "Kyber"),
        }
    }
}

/// Quantum-resistant cryptographic proof
///
/// # Example
///
/// ```
/// use legalis_core::quantum::{QuantumProof, PostQuantumScheme};
///
/// let data = b"legal statute v1.0";
/// let proof = QuantumProof::sign(data, PostQuantumScheme::Dilithium);
///
/// assert_eq!(proof.scheme(), PostQuantumScheme::Dilithium);
/// assert!(proof.verify(data));
/// assert!(!proof.verify(b"tampered data"));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QuantumProof {
    scheme: PostQuantumScheme,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}

impl QuantumProof {
    /// Sign data using post-quantum scheme
    pub fn sign(data: &[u8], scheme: PostQuantumScheme) -> Self {
        // Simulated signing (real implementation would use actual PQC library)
        let signature = Self::simulate_sign(data, scheme);
        let public_key = Self::simulate_keygen(scheme);

        Self {
            scheme,
            signature,
            public_key,
        }
    }

    /// Verify quantum-resistant signature
    pub fn verify(&self, data: &[u8]) -> bool {
        // Simulated verification
        let expected_sig = Self::simulate_sign(data, self.scheme);
        self.signature == expected_sig
    }

    /// Get the cryptographic scheme used
    pub fn scheme(&self) -> PostQuantumScheme {
        self.scheme
    }

    /// Get signature bytes
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Get public key bytes
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    // Simulation helpers
    fn simulate_sign(data: &[u8], scheme: PostQuantumScheme) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update([scheme as u8]);
        hasher.finalize().to_vec()
    }

    fn simulate_keygen(scheme: PostQuantumScheme) -> Vec<u8> {
        vec![0x42; 32 + (scheme as usize) * 16]
    }
}

/// Quantum-resistant audit trail
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QuantumAuditTrail {
    entries: Vec<AuditEntry>,
    scheme: PostQuantumScheme,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct AuditEntry {
    timestamp: u64,
    action: String,
    data: Vec<u8>,
    proof: QuantumProof,
}

impl QuantumAuditTrail {
    /// Create a new quantum-resistant audit trail
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::quantum::{QuantumAuditTrail, PostQuantumScheme};
    ///
    /// let mut trail = QuantumAuditTrail::new(PostQuantumScheme::Falcon);
    /// trail.record("statute_created", b"statute-001");
    /// trail.record("statute_modified", b"statute-001-v2");
    ///
    /// assert_eq!(trail.len(), 2);
    /// assert!(trail.verify_all());
    /// ```
    pub fn new(scheme: PostQuantumScheme) -> Self {
        Self {
            entries: Vec::new(),
            scheme,
        }
    }

    /// Record an action with quantum-resistant proof
    pub fn record(&mut self, action: &str, data: &[u8]) {
        let timestamp = Self::current_timestamp();
        // Sign the combination of timestamp, action, and data
        let mut sign_data = format!("{}{}", timestamp, action).into_bytes();
        sign_data.extend_from_slice(data);

        let proof = QuantumProof::sign(&sign_data, self.scheme);
        let entry = AuditEntry {
            timestamp,
            action: action.to_string(),
            data: data.to_vec(),
            proof,
        };
        self.entries.push(entry);
    }

    /// Verify all audit trail entries
    pub fn verify_all(&self) -> bool {
        self.entries.iter().all(|entry| {
            let mut sign_data = format!("{}{}", entry.timestamp, entry.action).into_bytes();
            sign_data.extend_from_slice(&entry.data);
            entry.proof.verify(&sign_data)
        })
    }

    /// Get number of audit entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if audit trail is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

// ============================================================================
// 5. QUANTUM ANNEALING FOR CONSTRAINT SATISFACTION
// ============================================================================

/// Constraint satisfaction solver using quantum annealing
///
/// # Example
///
/// ```
/// use legalis_core::quantum::ConstraintSatSolver;
///
/// let mut solver = ConstraintSatSolver::new(5);
/// solver.add_constraint(0, 1, 1.0); // Variables 0 and 1 should differ
/// solver.add_constraint(1, 2, -1.0); // Variables 1 and 2 should match
///
/// let solution = solver.solve();
/// assert_eq!(solution.len(), 5);
/// ```
#[derive(Debug, Clone)]
pub struct ConstraintSatSolver {
    num_vars: usize,
    constraints: Vec<(usize, usize, f64)>,
}

impl ConstraintSatSolver {
    /// Create a new constraint satisfaction solver
    pub fn new(num_vars: usize) -> Self {
        Self {
            num_vars,
            constraints: Vec::new(),
        }
    }

    /// Add a constraint between two variables
    pub fn add_constraint(&mut self, var1: usize, var2: usize, weight: f64) {
        self.constraints.push((var1, var2, weight));
    }

    /// Convert to Ising model for quantum annealing
    pub fn to_ising_model(&self) -> IsingModel {
        let mut model = IsingModel::new(self.num_vars);

        for &(i, j, weight) in &self.constraints {
            model.add_coupling(i, j, weight);
        }

        model
    }

    /// Solve using quantum annealing (simulated)
    pub fn solve(&self) -> Vec<bool> {
        let ising = self.to_ising_model();
        let optimizer = QuantumAnnealingOptimizer::new(self.num_vars, 200);
        let qubo = self.ising_to_qubo(&ising);
        optimizer.solve(&qubo)
    }

    fn ising_to_qubo(&self, ising: &IsingModel) -> QuboProblem {
        let mut qubo = QuboProblem::new(ising.num_spins);

        for ((i, j), coupling) in &ising.couplings {
            qubo.add_coefficient(*i, *j, *coupling);
        }

        for (i, field) in ising.fields.iter().enumerate() {
            qubo.add_coefficient(i, i, *field);
        }

        qubo
    }
}

/// Ising model for quantum annealing
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IsingModel {
    num_spins: usize,
    fields: Vec<f64>,
    couplings: HashMap<(usize, usize), f64>,
}

impl IsingModel {
    /// Create a new Ising model
    pub fn new(num_spins: usize) -> Self {
        Self {
            num_spins,
            fields: vec![0.0; num_spins],
            couplings: HashMap::new(),
        }
    }

    /// Add external field to a spin
    pub fn add_field(&mut self, spin: usize, field: f64) {
        if spin < self.num_spins {
            self.fields[spin] = field;
        }
    }

    /// Add coupling between two spins
    pub fn add_coupling(&mut self, spin1: usize, spin2: usize, coupling: f64) {
        self.couplings.insert((spin1, spin2), coupling);
    }

    /// Export to D-Wave compatible format
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::quantum::IsingModel;
    ///
    /// let mut model = IsingModel::new(3);
    /// model.add_field(0, 1.0);
    /// model.add_coupling(0, 1, -1.0);
    ///
    /// let dwave_format = model.to_dwave_format();
    /// assert!(dwave_format.contains("h = {"));
    /// assert!(dwave_format.contains("J = {"));
    /// ```
    pub fn to_dwave_format(&self) -> String {
        let mut output = String::from("# D-Wave Ising Model\n");

        // Fields (h)
        output.push_str("h = {\n");
        for (i, &field) in self.fields.iter().enumerate() {
            if field != 0.0 {
                output.push_str(&format!("    {}: {:.4},\n", i, field));
            }
        }
        output.push_str("}\n\n");

        // Couplings (J)
        output.push_str("J = {\n");
        for ((i, j), &coupling) in &self.couplings {
            output.push_str(&format!("    ({}, {}): {:.4},\n", i, j, coupling));
        }
        output.push_str("}\n");

        output
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ComparisonOp, Effect, EffectType};

    #[test]
    fn test_quantum_circuit_creation() {
        let mut circuit = QuantumCircuit::new(3);
        assert_eq!(circuit.num_qubits(), 3);
        assert_eq!(circuit.num_gates(), 0);

        circuit.add_gate(QuantumGate::H(0));
        circuit.add_gate(QuantumGate::CNOT(0, 1));
        assert_eq!(circuit.num_gates(), 2);
    }

    #[test]
    fn test_quantum_gate_display() {
        assert_eq!(format!("{}", QuantumGate::H(0)), "H(q0)");
        assert_eq!(format!("{}", QuantumGate::CNOT(0, 1)), "CNOT(q0, q1)");
        assert_eq!(
            format!("{}", QuantumGate::RZ(2, std::f64::consts::FRAC_PI_2)),
            "RZ(q2, 1.5707963267948966)"
        );
    }

    #[test]
    fn test_circuit_from_condition() {
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let circuit = QuantumCircuit::from_condition(&condition);
        assert!(circuit.num_gates() > 0);
    }

    #[test]
    fn test_circuit_to_qiskit() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.add_gate(QuantumGate::H(0));
        circuit.add_gate(QuantumGate::X(1));

        let qiskit = circuit.to_qiskit();
        assert!(qiskit.contains("QuantumCircuit(2)"));
        assert!(qiskit.contains("qc.h(0)"));
        assert!(qiskit.contains("qc.x(1)"));
    }

    #[test]
    fn test_qubo_problem() {
        let mut qubo = QuboProblem::new(3);
        qubo.add_coefficient(0, 0, 1.0);
        qubo.add_coefficient(0, 1, -2.0);

        let solution = vec![true, false, true];
        let energy = qubo.evaluate(&solution);
        assert_eq!(energy, 1.0);
    }

    #[test]
    fn test_quantum_annealing_optimizer() {
        let optimizer = QuantumAnnealingOptimizer::new(5, 100);
        assert_eq!(optimizer.num_vars(), 5);
        assert_eq!(optimizer.max_iterations(), 100);

        let problem = optimizer.create_problem();
        assert_eq!(problem.num_vars, 5);
    }

    #[test]
    fn test_annealing_solve() {
        let optimizer = QuantumAnnealingOptimizer::new(4, 50);
        let mut problem = optimizer.create_problem();

        // Simple problem: minimize sum
        for i in 0..4 {
            problem.add_coefficient(i, i, 1.0);
        }

        let solution = optimizer.solve(&problem);
        assert_eq!(solution.len(), 4);
    }

    #[test]
    fn test_statute_to_qubo() {
        let statute = Statute::new(
            "test-statute",
            "Test",
            Effect::new(EffectType::Grant, "Grant"),
        );

        let optimizer = QuantumAnnealingOptimizer::new(3, 10);
        let qubo = optimizer.statute_to_qubo(&statute);
        assert_eq!(qubo.num_vars, 3);
    }

    #[test]
    fn test_qaoa_optimizer() {
        let qaoa = QaoaOptimizer::new(3, 2);
        let circuit = qaoa.generate_circuit();
        assert_eq!(circuit.num_qubits(), 3);
        assert!(circuit.num_gates() > 0);
    }

    #[test]
    fn test_hybrid_evaluator() {
        let evaluator = DefaultHybridEvaluator::new();
        let condition = Condition::custom("test".to_string());

        let result = evaluator.evaluate_hybrid(&condition);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_vqe_entailment_solver() {
        let vqe = VqeEntailmentSolver::new(3, 100);
        let circuit = vqe.variational_circuit();
        assert_eq!(circuit.num_qubits(), 3);
    }

    #[test]
    fn test_post_quantum_proof() {
        let data = b"legal document";
        let proof = QuantumProof::sign(data, PostQuantumScheme::Dilithium);

        assert_eq!(proof.scheme(), PostQuantumScheme::Dilithium);
        assert!(proof.verify(data));
        assert!(!proof.verify(b"tampered"));
    }

    #[test]
    fn test_quantum_audit_trail() {
        let mut trail = QuantumAuditTrail::new(PostQuantumScheme::Falcon);
        trail.record("create", b"data1");
        trail.record("update", b"data2");

        assert_eq!(trail.len(), 2);
        assert!(!trail.is_empty());
    }

    #[test]
    fn test_constraint_sat_solver() {
        let mut solver = ConstraintSatSolver::new(4);
        solver.add_constraint(0, 1, 1.0);
        solver.add_constraint(1, 2, -1.0);

        let solution = solver.solve();
        assert_eq!(solution.len(), 4);
    }

    #[test]
    fn test_ising_model() {
        let mut model = IsingModel::new(3);
        model.add_field(0, 1.0);
        model.add_coupling(0, 1, -0.5);

        let dwave = model.to_dwave_format();
        assert!(dwave.contains("h = {"));
        assert!(dwave.contains("J = {"));
    }

    #[test]
    fn test_post_quantum_scheme_display() {
        assert_eq!(format!("{}", PostQuantumScheme::Dilithium), "Dilithium");
        assert_eq!(format!("{}", PostQuantumScheme::Falcon), "Falcon");
        assert_eq!(format!("{}", PostQuantumScheme::SphincsPlus), "SPHINCS+");
        assert_eq!(format!("{}", PostQuantumScheme::Kyber), "Kyber");
    }
}
