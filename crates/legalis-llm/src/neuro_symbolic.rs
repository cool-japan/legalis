//! Neuro-Symbolic Integration for Legal AI
//!
//! This module combines neural network capabilities with symbolic reasoning
//! to create hybrid systems that leverage both pattern recognition and logical inference.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Logical operator for symbolic reasoning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Logical NOT
    Not,
    /// Logical implication (if-then)
    Implies,
    /// Logical equivalence (if and only if)
    Iff,
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "AND"),
            Self::Or => write!(f, "OR"),
            Self::Not => write!(f, "NOT"),
            Self::Implies => write!(f, "IMPLIES"),
            Self::Iff => write!(f, "IFF"),
        }
    }
}

/// Symbolic predicate for logical reasoning
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Predicate {
    /// Predicate name
    pub name: String,
    /// Arguments
    pub arguments: Vec<String>,
}

impl Predicate {
    /// Creates a new predicate
    pub fn new(name: impl Into<String>, arguments: Vec<String>) -> Self {
        Self {
            name: name.into(),
            arguments,
        }
    }

    /// Creates a zero-argument predicate
    pub fn atom(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arguments: Vec::new(),
        }
    }
}

impl std::fmt::Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.arguments.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}({})", self.name, self.arguments.join(", "))
        }
    }
}

/// Logical formula
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Formula {
    /// Atomic predicate
    Atom(Predicate),
    /// Negation
    Not(Box<Formula>),
    /// Conjunction (AND)
    And(Vec<Formula>),
    /// Disjunction (OR)
    Or(Vec<Formula>),
    /// Implication
    Implies(Box<Formula>, Box<Formula>),
    /// Bi-conditional (IFF)
    Iff(Box<Formula>, Box<Formula>),
}

impl Formula {
    /// Creates an atomic formula
    pub fn atom(name: impl Into<String>) -> Self {
        Formula::Atom(Predicate::atom(name))
    }

    /// Creates a negation
    pub fn not(formula: Formula) -> Self {
        Formula::Not(Box::new(formula))
    }

    /// Creates a conjunction
    pub fn and(formulas: Vec<Formula>) -> Self {
        Formula::And(formulas)
    }

    /// Creates a disjunction
    pub fn or(formulas: Vec<Formula>) -> Self {
        Formula::Or(formulas)
    }

    /// Creates an implication
    pub fn implies(antecedent: Formula, consequent: Formula) -> Self {
        Formula::Implies(Box::new(antecedent), Box::new(consequent))
    }

    /// Creates a bi-conditional
    pub fn iff(left: Formula, right: Formula) -> Self {
        Formula::Iff(Box::new(left), Box::new(right))
    }
}

/// Hybrid neuro-symbolic reasoner
#[derive(Debug)]
pub struct HybridReasoner {
    /// Symbolic knowledge base (facts)
    knowledge_base: HashSet<Predicate>,
    /// Logical rules
    rules: Vec<Rule>,
    /// Neural confidence scores
    neural_scores: HashMap<String, f64>,
}

impl HybridReasoner {
    /// Creates a new hybrid reasoner
    pub fn new() -> Self {
        Self {
            knowledge_base: HashSet::new(),
            rules: Vec::new(),
            neural_scores: HashMap::new(),
        }
    }

    /// Adds a fact to the knowledge base
    pub fn add_fact(&mut self, predicate: Predicate) {
        self.knowledge_base.insert(predicate);
    }

    /// Adds a logical rule
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Sets a neural confidence score for a predicate
    pub fn set_neural_score(&mut self, predicate_name: impl Into<String>, score: f64) {
        self.neural_scores
            .insert(predicate_name.into(), score.clamp(0.0, 1.0));
    }

    /// Performs hybrid reasoning combining neural and symbolic approaches
    pub fn reason(&self, query: &Predicate) -> Result<ReasoningResult> {
        // Check if query is a direct fact
        if self.knowledge_base.contains(query) {
            return Ok(ReasoningResult {
                conclusion: true,
                confidence: 1.0,
                explanation: format!("Direct fact: {}", query),
                method: ReasoningMethod::Symbolic,
            });
        }

        // Try to derive using rules
        for rule in &self.rules {
            if rule.head.name == query.name {
                let can_derive = self.can_derive_from_rule(rule)?;
                if can_derive {
                    return Ok(ReasoningResult {
                        conclusion: true,
                        confidence: 0.9,
                        explanation: format!("Derived using rule: {}", rule.name),
                        method: ReasoningMethod::Symbolic,
                    });
                }
            }
        }

        // Fall back to neural confidence
        if let Some(&score) = self.neural_scores.get(&query.name) {
            return Ok(ReasoningResult {
                conclusion: score > 0.5,
                confidence: score,
                explanation: format!("Neural prediction for {}", query),
                method: ReasoningMethod::Neural,
            });
        }

        // Hybrid approach: combine symbolic and neural
        Ok(ReasoningResult {
            conclusion: false,
            confidence: 0.0,
            explanation: format!("Cannot derive: {}", query),
            method: ReasoningMethod::Hybrid,
        })
    }

    fn can_derive_from_rule(&self, rule: &Rule) -> Result<bool> {
        // Simple backward chaining
        for condition in &rule.body {
            if !self.knowledge_base.contains(condition) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Gets all facts in the knowledge base
    pub fn facts(&self) -> &HashSet<Predicate> {
        &self.knowledge_base
    }

    /// Gets all rules
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}

impl Default for HybridReasoner {
    fn default() -> Self {
        Self::new()
    }
}

/// Logical rule (Horn clause)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Rule name
    pub name: String,
    /// Rule head (conclusion)
    pub head: Predicate,
    /// Rule body (conditions)
    pub body: Vec<Predicate>,
}

impl Rule {
    /// Creates a new rule
    pub fn new(name: impl Into<String>, head: Predicate, body: Vec<Predicate>) -> Self {
        Self {
            name: name.into(),
            head,
            body,
        }
    }
}

/// Reasoning method used
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasoningMethod {
    /// Pure symbolic reasoning
    Symbolic,
    /// Pure neural reasoning
    Neural,
    /// Hybrid approach
    Hybrid,
}

/// Result of reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    /// Whether the conclusion holds
    pub conclusion: bool,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Explanation of reasoning
    pub explanation: String,
    /// Method used
    pub method: ReasoningMethod,
}

/// Logic-guided neural generator
#[derive(Debug)]
pub struct LogicGuidedGenerator {
    /// Logical constraints that must be satisfied
    constraints: Vec<Formula>,
    /// Generation parameters
    temperature: f64,
}

impl LogicGuidedGenerator {
    /// Creates a new logic-guided generator
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            temperature: 0.7,
        }
    }

    /// Sets the temperature for generation
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Adds a logical constraint
    pub fn add_constraint(&mut self, constraint: Formula) {
        self.constraints.push(constraint);
    }

    /// Generates text while respecting logical constraints
    pub fn generate(&self, prompt: &str) -> Result<String> {
        // In a real implementation, this would:
        // 1. Use a neural model to generate candidates
        // 2. Filter candidates that violate logical constraints
        // 3. Return the best valid candidate

        if self.constraints.is_empty() {
            return Ok(format!("Generated (unconstrained): {}", prompt));
        }

        Ok(format!(
            "Generated (with {} constraints): {}",
            self.constraints.len(),
            prompt
        ))
    }

    /// Validates if generated text satisfies constraints
    pub fn validate(&self, _text: &str) -> bool {
        // Simple validation - in practice would parse and check constraints
        true
    }

    /// Gets the number of constraints
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
}

impl Default for LogicGuidedGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Constraint satisfaction problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintProblem {
    /// Variables in the problem
    pub variables: Vec<String>,
    /// Domain for each variable
    pub domains: HashMap<String, Vec<String>>,
    /// Constraints between variables
    pub constraints: Vec<Constraint>,
}

impl ConstraintProblem {
    /// Creates a new constraint problem
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            domains: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a variable with its domain
    pub fn add_variable(&mut self, var: impl Into<String>, domain: Vec<String>) {
        let var = var.into();
        self.variables.push(var.clone());
        self.domains.insert(var, domain);
    }

    /// Adds a constraint
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}

impl Default for ConstraintProblem {
    fn default() -> Self {
        Self::new()
    }
}

/// Constraint between variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Variables involved
    pub variables: Vec<String>,
    /// Constraint type
    pub constraint_type: ConstraintType,
}

impl Constraint {
    /// Creates a new constraint
    pub fn new(variables: Vec<String>, constraint_type: ConstraintType) -> Self {
        Self {
            variables,
            constraint_type,
        }
    }
}

/// Type of constraint
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    /// All variables must have different values
    AllDifferent,
    /// Variables must satisfy a specific relation
    Relation(String),
    /// Custom constraint
    Custom(String),
}

/// Symbolic constraint solver
#[derive(Debug)]
pub struct ConstraintSolver {
    problem: ConstraintProblem,
}

impl ConstraintSolver {
    /// Creates a new constraint solver
    pub fn new(problem: ConstraintProblem) -> Self {
        Self { problem }
    }

    /// Solves the constraint problem
    pub fn solve(&self) -> Result<Option<HashMap<String, String>>> {
        // Simple backtracking solver
        let mut assignment: HashMap<String, String> = HashMap::new();
        if self.backtrack(&mut assignment) {
            Ok(Some(assignment))
        } else {
            Ok(None)
        }
    }

    fn backtrack(&self, assignment: &mut HashMap<String, String>) -> bool {
        // Check if assignment is complete
        if assignment.len() == self.problem.variables.len() {
            return true;
        }

        // Select unassigned variable
        let var = self
            .problem
            .variables
            .iter()
            .find(|v| !assignment.contains_key(*v));

        if let Some(var) = var {
            if let Some(domain) = self.problem.domains.get(var) {
                for value in domain {
                    assignment.insert(var.clone(), value.clone());

                    if self.is_consistent(assignment) && self.backtrack(assignment) {
                        return true;
                    }

                    assignment.remove(var);
                }
            }
        }

        false
    }

    fn is_consistent(&self, assignment: &HashMap<String, String>) -> bool {
        for constraint in &self.problem.constraints {
            if !self.check_constraint(constraint, assignment) {
                return false;
            }
        }
        true
    }

    fn check_constraint(
        &self,
        constraint: &Constraint,
        assignment: &HashMap<String, String>,
    ) -> bool {
        // Check if all variables in constraint are assigned
        let all_assigned = constraint
            .variables
            .iter()
            .all(|v| assignment.contains_key(v));

        if !all_assigned {
            return true; // Not yet assigned, can't violate
        }

        match &constraint.constraint_type {
            ConstraintType::AllDifferent => {
                let values: Vec<_> = constraint
                    .variables
                    .iter()
                    .filter_map(|v| assignment.get(v))
                    .collect();
                let unique: HashSet<_> = values.iter().collect();
                values.len() == unique.len()
            }
            _ => true, // Other constraints assumed satisfied
        }
    }
}

/// Neural-symbolic rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralRule {
    /// Rule name
    pub name: String,
    /// Symbolic pattern
    pub pattern: Formula,
    /// Neural confidence threshold
    pub confidence_threshold: f64,
    /// Action to take when rule fires
    pub action: String,
}

impl NeuralRule {
    /// Creates a new neural-symbolic rule
    pub fn new(name: impl Into<String>, pattern: Formula, action: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pattern,
            confidence_threshold: 0.7,
            action: action.into(),
        }
    }

    /// Sets the confidence threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold.clamp(0.0, 1.0);
        self
    }
}

/// Explainable neuro-symbolic model
#[derive(Debug)]
pub struct ExplainableModel {
    /// Neural-symbolic rules
    rules: Vec<NeuralRule>,
    /// Feature importance scores
    feature_importance: HashMap<String, f64>,
}

impl ExplainableModel {
    /// Creates a new explainable model
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            feature_importance: HashMap::new(),
        }
    }

    /// Adds a neural-symbolic rule
    pub fn add_rule(&mut self, rule: NeuralRule) {
        self.rules.push(rule);
    }

    /// Sets feature importance
    pub fn set_feature_importance(&mut self, feature: impl Into<String>, importance: f64) {
        self.feature_importance
            .insert(feature.into(), importance.clamp(0.0, 1.0));
    }

    /// Makes a prediction with explanation
    pub fn predict_with_explanation(&self, input: &str) -> NeuroSymbolicExplanation {
        let mut fired_rules = Vec::new();
        let mut total_confidence = 0.0;

        // Check which rules fire
        for rule in &self.rules {
            // Simplified rule matching
            if input.contains(&rule.name) {
                fired_rules.push(rule.name.clone());
                total_confidence += 0.8;
            }
        }

        let confidence = (total_confidence / self.rules.len() as f64).min(1.0);

        let rationale = self.generate_rationale(&fired_rules);

        NeuroSymbolicExplanation {
            prediction: confidence > 0.5,
            confidence,
            fired_rules,
            feature_contributions: self.feature_importance.clone(),
            rationale,
        }
    }

    fn generate_rationale(&self, fired_rules: &[String]) -> String {
        if fired_rules.is_empty() {
            "No rules matched the input".to_string()
        } else {
            format!("Matched rules: {}", fired_rules.join(", "))
        }
    }

    /// Gets the number of rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for ExplainableModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Explanation for a neuro-symbolic prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroSymbolicExplanation {
    /// Predicted outcome
    pub prediction: bool,
    /// Confidence score
    pub confidence: f64,
    /// Rules that fired
    pub fired_rules: Vec<String>,
    /// Feature contributions
    pub feature_contributions: HashMap<String, f64>,
    /// Human-readable rationale
    pub rationale: String,
}

/// Legal knowledge compiler
#[derive(Debug)]
pub struct KnowledgeCompiler {
    /// Compiled knowledge base
    compiled_kb: Vec<CompiledRule>,
}

impl KnowledgeCompiler {
    /// Creates a new knowledge compiler
    pub fn new() -> Self {
        Self {
            compiled_kb: Vec::new(),
        }
    }

    /// Compiles natural language legal text into symbolic form
    pub fn compile(&mut self, text: &str) -> Result<CompiledRule> {
        // Simplified compilation - extract key legal concepts
        let conditions = self.extract_conditions(text);
        let conclusion = self.extract_conclusion(text);

        let rule = CompiledRule {
            source_text: text.to_string(),
            conditions,
            conclusion,
            confidence: 0.85,
        };

        self.compiled_kb.push(rule.clone());
        Ok(rule)
    }

    fn extract_conditions(&self, text: &str) -> Vec<Predicate> {
        let mut conditions = Vec::new();

        if text.contains("if") || text.contains("when") {
            conditions.push(Predicate::atom("condition_present"));
        }

        if text.contains("must") || text.contains("shall") {
            conditions.push(Predicate::atom("obligation"));
        }

        conditions
    }

    fn extract_conclusion(&self, text: &str) -> Predicate {
        if text.contains("liable") {
            Predicate::atom("liability")
        } else if text.contains("valid") {
            Predicate::atom("validity")
        } else {
            Predicate::atom("conclusion")
        }
    }

    /// Gets all compiled rules
    pub fn compiled_rules(&self) -> &[CompiledRule] {
        &self.compiled_kb
    }

    /// Gets the number of compiled rules
    pub fn rule_count(&self) -> usize {
        self.compiled_kb.len()
    }
}

impl Default for KnowledgeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Compiled legal rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledRule {
    /// Original source text
    pub source_text: String,
    /// Extracted conditions
    pub conditions: Vec<Predicate>,
    /// Extracted conclusion
    pub conclusion: Predicate,
    /// Compilation confidence
    pub confidence: f64,
}

/// Automated theorem prover for legal statutes
#[derive(Debug)]
pub struct TheoremProver {
    /// Axioms (assumed true)
    axioms: Vec<Formula>,
    /// Theorems to prove
    theorems: Vec<(String, Formula)>,
}

impl TheoremProver {
    /// Creates a new theorem prover
    pub fn new() -> Self {
        Self {
            axioms: Vec::new(),
            theorems: Vec::new(),
        }
    }

    /// Adds an axiom
    pub fn add_axiom(&mut self, axiom: Formula) {
        self.axioms.push(axiom);
    }

    /// Adds a theorem to prove
    pub fn add_theorem(&mut self, name: impl Into<String>, formula: Formula) {
        self.theorems.push((name.into(), formula));
    }

    /// Attempts to prove a theorem
    pub fn prove(&self, theorem_name: &str) -> Result<ProofResult> {
        let theorem = self
            .theorems
            .iter()
            .find(|(name, _)| name == theorem_name)
            .context("Theorem not found")?;

        // Simplified proof - check if theorem matches any axiom
        let provable = self.can_prove(&theorem.1);

        Ok(ProofResult {
            theorem_name: theorem_name.to_string(),
            provable,
            proof_steps: if provable {
                vec!["Follows from axioms".to_string()]
            } else {
                vec!["Cannot prove with current axioms".to_string()]
            },
        })
    }

    fn can_prove(&self, formula: &Formula) -> bool {
        // Simplified: check if formula is in axioms
        for axiom in &self.axioms {
            if self.formulas_match(axiom, formula) {
                return true;
            }
        }
        false
    }

    fn formulas_match(&self, f1: &Formula, f2: &Formula) -> bool {
        // Simplified structural equality
        std::mem::discriminant(f1) == std::mem::discriminant(f2)
    }

    /// Gets the number of axioms
    pub fn axiom_count(&self) -> usize {
        self.axioms.len()
    }

    /// Gets the number of theorems
    pub fn theorem_count(&self) -> usize {
        self.theorems.len()
    }
}

impl Default for TheoremProver {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of theorem proving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResult {
    /// Theorem name
    pub theorem_name: String,
    /// Whether the theorem was proven
    pub provable: bool,
    /// Proof steps
    pub proof_steps: Vec<String>,
}

/// Probabilistic logic program
#[derive(Debug)]
pub struct ProbabilisticLogicProgram {
    /// Probabilistic facts
    probabilistic_facts: HashMap<Predicate, f64>,
    /// Deterministic rules
    rules: Vec<Rule>,
}

impl ProbabilisticLogicProgram {
    /// Creates a new probabilistic logic program
    pub fn new() -> Self {
        Self {
            probabilistic_facts: HashMap::new(),
            rules: Vec::new(),
        }
    }

    /// Adds a probabilistic fact
    pub fn add_probabilistic_fact(&mut self, predicate: Predicate, probability: f64) {
        self.probabilistic_facts
            .insert(predicate, probability.clamp(0.0, 1.0));
    }

    /// Adds a deterministic rule
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Computes probability of a query
    pub fn query_probability(&self, query: &Predicate) -> f64 {
        // Direct probabilistic fact
        if let Some(&prob) = self.probabilistic_facts.get(query) {
            return prob;
        }

        // Try to derive from rules
        for rule in &self.rules {
            if &rule.head == query {
                let body_prob = self.compute_body_probability(&rule.body);
                if body_prob > 0.0 {
                    return body_prob;
                }
            }
        }

        0.0
    }

    fn compute_body_probability(&self, body: &[Predicate]) -> f64 {
        if body.is_empty() {
            return 1.0;
        }

        // Assume independence and multiply probabilities
        let mut prob = 1.0;
        for predicate in body {
            let p = self.probabilistic_facts.get(predicate).unwrap_or(&0.0);
            prob *= p;
        }
        prob
    }

    /// Gets the number of probabilistic facts
    pub fn fact_count(&self) -> usize {
        self.probabilistic_facts.len()
    }

    /// Gets the number of rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for ProbabilisticLogicProgram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predicate_creation() {
        let pred = Predicate::new("contract", vec!["party1".to_string(), "party2".to_string()]);
        assert_eq!(pred.name, "contract");
        assert_eq!(pred.arguments.len(), 2);
    }

    #[test]
    fn test_predicate_atom() {
        let atom = Predicate::atom("liable");
        assert_eq!(atom.name, "liable");
        assert!(atom.arguments.is_empty());
    }

    #[test]
    fn test_formula_construction() {
        let f1 = Formula::atom("A");
        let f2 = Formula::atom("B");
        let and_formula = Formula::and(vec![f1, f2]);

        match and_formula {
            Formula::And(formulas) => assert_eq!(formulas.len(), 2),
            _ => panic!("Expected And formula"),
        }
    }

    #[test]
    fn test_hybrid_reasoner_facts() {
        let mut reasoner = HybridReasoner::new();
        let fact = Predicate::atom("contract_valid");
        reasoner.add_fact(fact.clone());

        let result = reasoner.reason(&fact).unwrap();
        assert!(result.conclusion);
        assert_eq!(result.confidence, 1.0);
        assert_eq!(result.method, ReasoningMethod::Symbolic);
    }

    #[test]
    fn test_hybrid_reasoner_neural() {
        let mut reasoner = HybridReasoner::new();
        reasoner.set_neural_score("liability", 0.85);

        let query = Predicate::atom("liability");
        let result = reasoner.reason(&query).unwrap();
        assert!(result.conclusion);
        assert_eq!(result.confidence, 0.85);
        assert_eq!(result.method, ReasoningMethod::Neural);
    }

    #[test]
    fn test_hybrid_reasoner_rules() {
        let mut reasoner = HybridReasoner::new();

        let condition = Predicate::atom("breach");
        let conclusion = Predicate::atom("damages");

        reasoner.add_fact(condition.clone());
        reasoner.add_rule(Rule::new(
            "breach_rule",
            conclusion.clone(),
            vec![condition],
        ));

        let result = reasoner.reason(&conclusion).unwrap();
        assert!(result.conclusion);
    }

    #[test]
    fn test_logic_guided_generator() {
        let mut generator = LogicGuidedGenerator::new().with_temperature(0.5);
        generator.add_constraint(Formula::atom("must_cite_law"));

        assert_eq!(generator.constraint_count(), 1);

        let result = generator.generate("Draft a contract").unwrap();
        assert!(result.contains("constraints"));
    }

    #[test]
    fn test_constraint_problem() {
        let mut problem = ConstraintProblem::new();
        problem.add_variable("party1", vec!["Alice".to_string(), "Bob".to_string()]);
        problem.add_variable("party2", vec!["Charlie".to_string(), "Alice".to_string()]);

        problem.add_constraint(Constraint::new(
            vec!["party1".to_string(), "party2".to_string()],
            ConstraintType::AllDifferent,
        ));

        assert_eq!(problem.variables.len(), 2);
        assert_eq!(problem.constraints.len(), 1);
    }

    #[test]
    fn test_constraint_solver() {
        let mut problem = ConstraintProblem::new();
        problem.add_variable("X", vec!["1".to_string(), "2".to_string()]);
        problem.add_variable("Y", vec!["2".to_string(), "3".to_string()]);
        problem.add_constraint(Constraint::new(
            vec!["X".to_string(), "Y".to_string()],
            ConstraintType::AllDifferent,
        ));

        let solver = ConstraintSolver::new(problem);
        let solution = solver.solve().unwrap();

        assert!(solution.is_some());
        let assignment = solution.unwrap();
        assert_ne!(assignment.get("X"), assignment.get("Y"));
    }

    #[test]
    fn test_neural_rule() {
        let rule = NeuralRule::new("contract_rule", Formula::atom("offer"), "accept_contract")
            .with_threshold(0.8);

        assert_eq!(rule.name, "contract_rule");
        assert_eq!(rule.confidence_threshold, 0.8);
    }

    #[test]
    fn test_explainable_model() {
        let mut model = ExplainableModel::new();

        let rule = NeuralRule::new("contract_rule", Formula::atom("offer"), "accept");
        model.add_rule(rule);
        model.set_feature_importance("offer_present", 0.9);

        assert_eq!(model.rule_count(), 1);

        let explanation = model.predict_with_explanation("contract_rule test");
        assert!(!explanation.fired_rules.is_empty());
    }

    #[test]
    fn test_knowledge_compiler() {
        let mut compiler = KnowledgeCompiler::new();

        let text = "If a party breaches the contract, they shall be liable for damages.";
        let compiled = compiler.compile(text).unwrap();

        assert!(compiled.source_text.contains("breach"));
        assert!(!compiled.conditions.is_empty());
        assert_eq!(compiler.rule_count(), 1);
    }

    #[test]
    fn test_theorem_prover() {
        let mut prover = TheoremProver::new();

        let axiom = Formula::atom("contract_valid");
        prover.add_axiom(axiom.clone());

        prover.add_theorem("theorem1", axiom);

        assert_eq!(prover.axiom_count(), 1);
        assert_eq!(prover.theorem_count(), 1);

        let result = prover.prove("theorem1").unwrap();
        assert!(result.provable);
    }

    #[test]
    fn test_probabilistic_logic_program() {
        let mut plp = ProbabilisticLogicProgram::new();

        let fact = Predicate::atom("breach");
        plp.add_probabilistic_fact(fact.clone(), 0.7);

        assert_eq!(plp.fact_count(), 1);

        let prob = plp.query_probability(&fact);
        assert!((prob - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn test_probabilistic_logic_with_rules() {
        let mut plp = ProbabilisticLogicProgram::new();

        let condition = Predicate::atom("breach");
        let conclusion = Predicate::atom("damages");

        plp.add_probabilistic_fact(condition.clone(), 0.8);
        plp.add_rule(Rule::new(
            "damage_rule",
            conclusion.clone(),
            vec![condition],
        ));

        let prob = plp.query_probability(&conclusion);
        assert!(prob > 0.0);
    }

    #[test]
    fn test_logical_operator_display() {
        assert_eq!(LogicalOperator::And.to_string(), "AND");
        assert_eq!(LogicalOperator::Implies.to_string(), "IMPLIES");
    }

    #[test]
    fn test_formula_implies() {
        let ant = Formula::atom("A");
        let cons = Formula::atom("B");
        let implies = Formula::implies(ant, cons);

        match implies {
            Formula::Implies(_, _) => (),
            _ => panic!("Expected Implies formula"),
        }
    }
}
