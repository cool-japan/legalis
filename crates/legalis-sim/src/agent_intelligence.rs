//! Agent Intelligence Module
//!
//! This module provides advanced AI capabilities for agents including:
//! - Reinforcement learning for adaptive behavior
//! - Game-theoretic reasoning for strategic interactions
//! - Bounded rationality models for realistic decision-making
//! - Belief-Desire-Intention (BDI) architecture
//! - Memory and learning systems

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// Reinforcement Learning
// ============================================================================

/// Q-Learning agent for reinforcement learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLearningAgent {
    /// Agent identifier
    pub id: Uuid,
    /// Q-table: state -> action -> Q-value
    pub q_table: HashMap<String, HashMap<String, f64>>,
    /// Learning rate (alpha)
    pub learning_rate: f64,
    /// Discount factor (gamma)
    pub discount_factor: f64,
    /// Exploration rate (epsilon)
    pub exploration_rate: f64,
    /// Minimum exploration rate
    pub min_exploration_rate: f64,
    /// Exploration decay rate
    pub exploration_decay: f64,
    /// Total episodes
    pub episodes: u64,
}

impl QLearningAgent {
    /// Create a new Q-learning agent
    pub fn new(learning_rate: f64, discount_factor: f64, exploration_rate: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            exploration_rate,
            min_exploration_rate: 0.01,
            exploration_decay: 0.995,
            episodes: 0,
        }
    }

    /// Get Q-value for state-action pair
    pub fn get_q_value(&self, state: &str, action: &str) -> f64 {
        self.q_table
            .get(state)
            .and_then(|actions| actions.get(action))
            .copied()
            .unwrap_or(0.0)
    }

    /// Update Q-value using Q-learning update rule
    pub fn update_q_value(
        &mut self,
        state: &str,
        action: &str,
        reward: f64,
        next_state: &str,
        available_actions: &[String],
    ) {
        let current_q = self.get_q_value(state, action);

        // Find max Q-value for next state
        let max_next_q = available_actions
            .iter()
            .map(|a| self.get_q_value(next_state, a))
            .fold(f64::NEG_INFINITY, f64::max);

        // Q-learning update: Q(s,a) = Q(s,a) + alpha * (reward + gamma * max(Q(s',a')) - Q(s,a))
        let new_q = current_q
            + self.learning_rate * (reward + self.discount_factor * max_next_q - current_q);

        self.q_table
            .entry(state.to_string())
            .or_default()
            .insert(action.to_string(), new_q);
    }

    /// Choose action using epsilon-greedy policy
    pub fn choose_action(&self, state: &str, available_actions: &[String]) -> Option<String> {
        if available_actions.is_empty() {
            return None;
        }

        // Exploration: random action
        if rand::random::<f64>() < self.exploration_rate {
            let idx = (rand::random::<f64>() * available_actions.len() as f64) as usize;
            return Some(available_actions[idx].clone());
        }

        // Exploitation: best action
        let mut best_action = None;
        let mut best_q = f64::NEG_INFINITY;

        for action in available_actions {
            let q = self.get_q_value(state, action);
            if q > best_q {
                best_q = q;
                best_action = Some(action.clone());
            }
        }

        best_action.or_else(|| Some(available_actions[0].clone()))
    }

    /// Decay exploration rate
    pub fn decay_exploration(&mut self) {
        self.exploration_rate =
            (self.exploration_rate * self.exploration_decay).max(self.min_exploration_rate);
        self.episodes += 1;
    }
}

/// SARSA agent (on-policy reinforcement learning)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarsaAgent {
    /// Agent identifier
    pub id: Uuid,
    /// Q-table: state -> action -> Q-value
    pub q_table: HashMap<String, HashMap<String, f64>>,
    /// Learning rate
    pub learning_rate: f64,
    /// Discount factor
    pub discount_factor: f64,
    /// Exploration rate
    pub exploration_rate: f64,
}

impl SarsaAgent {
    /// Create a new SARSA agent
    pub fn new(learning_rate: f64, discount_factor: f64, exploration_rate: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            exploration_rate,
        }
    }

    /// Update Q-value using SARSA update rule
    pub fn update_q_value(
        &mut self,
        state: &str,
        action: &str,
        reward: f64,
        next_state: &str,
        next_action: &str,
    ) {
        let current_q = self
            .q_table
            .get(state)
            .and_then(|actions| actions.get(action))
            .copied()
            .unwrap_or(0.0);

        let next_q = self
            .q_table
            .get(next_state)
            .and_then(|actions| actions.get(next_action))
            .copied()
            .unwrap_or(0.0);

        // SARSA update: Q(s,a) = Q(s,a) + alpha * (reward + gamma * Q(s',a') - Q(s,a))
        let new_q =
            current_q + self.learning_rate * (reward + self.discount_factor * next_q - current_q);

        self.q_table
            .entry(state.to_string())
            .or_default()
            .insert(action.to_string(), new_q);
    }
}

// ============================================================================
// Game Theory
// ============================================================================

/// Payoff matrix for 2-player games
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoffMatrix {
    /// Actions available to player 1
    pub player1_actions: Vec<String>,
    /// Actions available to player 2
    pub player2_actions: Vec<String>,
    /// Payoffs: (player1_action_idx, player2_action_idx) -> (player1_payoff, player2_payoff)
    pub payoffs: Vec<Vec<(f64, f64)>>,
}

impl PayoffMatrix {
    /// Create a new payoff matrix
    pub fn new(
        player1_actions: Vec<String>,
        player2_actions: Vec<String>,
        payoffs: Vec<Vec<(f64, f64)>>,
    ) -> Self {
        Self {
            player1_actions,
            player2_actions,
            payoffs,
        }
    }

    /// Get payoff for action pair
    pub fn get_payoff(&self, p1_action_idx: usize, p2_action_idx: usize) -> Option<(f64, f64)> {
        self.payoffs
            .get(p1_action_idx)
            .and_then(|row| row.get(p2_action_idx))
            .copied()
    }

    /// Find Nash equilibria (pure strategy)
    pub fn find_pure_nash_equilibria(&self) -> Vec<(usize, usize)> {
        let mut equilibria = Vec::new();

        for (i, _) in self.player1_actions.iter().enumerate() {
            for (j, _) in self.player2_actions.iter().enumerate() {
                if self.is_nash_equilibrium(i, j) {
                    equilibria.push((i, j));
                }
            }
        }

        equilibria
    }

    /// Check if action pair is a Nash equilibrium
    fn is_nash_equilibrium(&self, p1_action: usize, p2_action: usize) -> bool {
        let (p1_payoff, p2_payoff) = match self.get_payoff(p1_action, p2_action) {
            Some(p) => p,
            None => return false,
        };

        // Check if player 1 has incentive to deviate
        for i in 0..self.player1_actions.len() {
            if i != p1_action {
                if let Some((alt_p1_payoff, _)) = self.get_payoff(i, p2_action) {
                    if alt_p1_payoff > p1_payoff {
                        return false;
                    }
                }
            }
        }

        // Check if player 2 has incentive to deviate
        for j in 0..self.player2_actions.len() {
            if j != p2_action {
                if let Some((_, alt_p2_payoff)) = self.get_payoff(p1_action, j) {
                    if alt_p2_payoff > p2_payoff {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Create prisoner's dilemma game
    pub fn prisoners_dilemma() -> Self {
        Self::new(
            vec!["Cooperate".to_string(), "Defect".to_string()],
            vec!["Cooperate".to_string(), "Defect".to_string()],
            vec![
                vec![(-1.0, -1.0), (-3.0, 0.0)],
                vec![(0.0, -3.0), (-2.0, -2.0)],
            ],
        )
    }
}

/// Game-theoretic agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTheoreticAgent {
    /// Agent identifier
    pub id: Uuid,
    /// Strategy probabilities for each action
    pub strategy: HashMap<String, f64>,
    /// Opponent model: action -> expected frequency
    pub opponent_model: HashMap<String, f64>,
    /// Historical payoffs
    pub payoff_history: Vec<f64>,
}

impl GameTheoreticAgent {
    /// Create a new game-theoretic agent
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            strategy: HashMap::new(),
            opponent_model: HashMap::new(),
            payoff_history: Vec::new(),
        }
    }

    /// Update opponent model based on observed action
    pub fn observe_opponent_action(&mut self, action: &str) {
        let count = self.opponent_model.entry(action.to_string()).or_insert(0.0);
        *count += 1.0;
    }

    /// Get expected opponent action frequency
    pub fn get_opponent_frequency(&self, action: &str) -> f64 {
        let total: f64 = self.opponent_model.values().sum();
        if total == 0.0 {
            0.0
        } else {
            self.opponent_model.get(action).copied().unwrap_or(0.0) / total
        }
    }

    /// Choose best response action given payoff matrix and opponent model
    pub fn best_response(&self, matrix: &PayoffMatrix) -> Option<usize> {
        let mut best_action = None;
        let mut best_expected_payoff = f64::NEG_INFINITY;

        for (i, _) in matrix.player1_actions.iter().enumerate() {
            let mut expected_payoff = 0.0;
            for (j, action) in matrix.player2_actions.iter().enumerate() {
                let freq = self.get_opponent_frequency(action);
                if let Some((payoff, _)) = matrix.get_payoff(i, j) {
                    expected_payoff += freq * payoff;
                }
            }

            if expected_payoff > best_expected_payoff {
                best_expected_payoff = expected_payoff;
                best_action = Some(i);
            }
        }

        best_action
    }

    /// Record payoff from game round
    pub fn record_payoff(&mut self, payoff: f64) {
        self.payoff_history.push(payoff);
    }

    /// Get average payoff
    pub fn average_payoff(&self) -> f64 {
        if self.payoff_history.is_empty() {
            0.0
        } else {
            self.payoff_history.iter().sum::<f64>() / self.payoff_history.len() as f64
        }
    }
}

impl Default for GameTheoreticAgent {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Bounded Rationality
// ============================================================================

/// Bounded rationality decision model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedRationalityAgent {
    /// Agent identifier
    pub id: Uuid,
    /// Aspiration level (satisficing threshold)
    pub aspiration_level: f64,
    /// Search depth (limited computational capacity)
    pub search_depth: usize,
    /// Heuristic weights for decision factors
    pub heuristic_weights: HashMap<String, f64>,
    /// Cognitive load (affects decision quality)
    pub cognitive_load: f64,
}

impl BoundedRationalityAgent {
    /// Create a new bounded rationality agent
    pub fn new(aspiration_level: f64, search_depth: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            aspiration_level,
            search_depth,
            heuristic_weights: HashMap::new(),
            cognitive_load: 0.0,
        }
    }

    /// Satisficing: choose first option above aspiration level
    pub fn satisfice(&self, options: &[(String, f64)]) -> Option<String> {
        options
            .iter()
            .take(self.search_depth)
            .find(|(_, value)| *value >= self.aspiration_level)
            .map(|(name, _)| name.clone())
    }

    /// Heuristic-based decision with weighted factors
    pub fn heuristic_decision(
        &self,
        options: &HashMap<String, HashMap<String, f64>>,
    ) -> Option<String> {
        let mut best_option = None;
        let mut best_score = f64::NEG_INFINITY;

        for (option_name, factors) in options.iter().take(self.search_depth) {
            let mut score = 0.0;
            for (factor, value) in factors {
                let weight = self.heuristic_weights.get(factor).copied().unwrap_or(1.0);
                score += weight * value;
            }

            // Apply cognitive load penalty
            score *= 1.0 - self.cognitive_load;

            if score > best_score {
                best_score = score;
                best_option = Some(option_name.clone());
            }
        }

        best_option
    }

    /// Update aspiration level based on experience
    pub fn adapt_aspiration(&mut self, achieved_value: f64, adaptation_rate: f64) {
        self.aspiration_level += adaptation_rate * (achieved_value - self.aspiration_level);
    }

    /// Set heuristic weight for a decision factor
    pub fn set_heuristic_weight(&mut self, factor: &str, weight: f64) {
        self.heuristic_weights.insert(factor.to_string(), weight);
    }
}

// ============================================================================
// BDI (Belief-Desire-Intention) Architecture
// ============================================================================

/// Belief in BDI architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    /// Belief identifier
    pub id: String,
    /// Belief content/description
    pub content: String,
    /// Certainty level (0.0 to 1.0)
    pub certainty: f64,
    /// Last update timestamp
    pub last_updated: i64,
}

/// Desire/Goal in BDI architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desire {
    /// Desire identifier
    pub id: String,
    /// Description of desired state
    pub description: String,
    /// Priority (higher = more important)
    pub priority: f64,
    /// Satisfaction condition
    pub satisfaction_condition: String,
}

/// Intention (committed plan) in BDI architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    /// Intention identifier
    pub id: String,
    /// Associated desire
    pub desire_id: String,
    /// Planned actions
    pub plan: Vec<String>,
    /// Current step in plan
    pub current_step: usize,
    /// Commitment strength (0.0 to 1.0)
    pub commitment: f64,
}

/// BDI Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BdiAgent {
    /// Agent identifier
    pub id: Uuid,
    /// Current beliefs
    pub beliefs: Vec<Belief>,
    /// Current desires
    pub desires: Vec<Desire>,
    /// Current intentions
    pub intentions: Vec<Intention>,
    /// Deliberation threshold (minimum priority to form intention)
    pub deliberation_threshold: f64,
}

impl BdiAgent {
    /// Create a new BDI agent
    pub fn new(deliberation_threshold: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            beliefs: Vec::new(),
            desires: Vec::new(),
            intentions: Vec::new(),
            deliberation_threshold,
        }
    }

    /// Add or update a belief
    pub fn add_belief(&mut self, belief: Belief) {
        if let Some(existing) = self.beliefs.iter_mut().find(|b| b.id == belief.id) {
            *existing = belief;
        } else {
            self.beliefs.push(belief);
        }
    }

    /// Add a desire
    pub fn add_desire(&mut self, desire: Desire) {
        self.desires.push(desire);
    }

    /// Deliberate: form intentions from desires
    pub fn deliberate(&mut self) {
        // Select high-priority desires that exceed threshold
        let mut eligible_desires: Vec<_> = self
            .desires
            .iter()
            .filter(|d| d.priority >= self.deliberation_threshold)
            .collect();

        // Sort by priority
        eligible_desires.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

        // Form intentions for top desires (if not already intended)
        for desire in eligible_desires.iter().take(3) {
            if !self.intentions.iter().any(|i| i.desire_id == desire.id) {
                // Create simple plan (in practice, would use planning algorithm)
                let plan = vec![format!("achieve_{}", desire.id)];
                let intention = Intention {
                    id: Uuid::new_v4().to_string(),
                    desire_id: desire.id.clone(),
                    plan,
                    current_step: 0,
                    commitment: desire.priority,
                };
                self.intentions.push(intention);
            }
        }
    }

    /// Execute current intentions (returns next actions)
    pub fn act(&mut self) -> Vec<String> {
        let mut actions = Vec::new();

        for intention in &mut self.intentions {
            if intention.current_step < intention.plan.len() {
                actions.push(intention.plan[intention.current_step].clone());
                intention.current_step += 1;
            }
        }

        // Remove completed intentions
        self.intentions.retain(|i| i.current_step < i.plan.len());

        actions
    }

    /// Revise beliefs and intentions based on new information
    pub fn revise(&mut self, observation: &str) {
        // Update beliefs based on observation
        let belief = Belief {
            id: format!("obs_{}", Uuid::new_v4()),
            content: observation.to_string(),
            certainty: 0.9,
            last_updated: chrono::Utc::now().timestamp(),
        };
        self.add_belief(belief);

        // Reconsider intentions if beliefs changed significantly
        // (simplified: just re-deliberate)
        self.deliberate();
    }
}

// ============================================================================
// Memory and Learning
// ============================================================================

/// Experience entry for episodic memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    /// Experience identifier
    pub id: Uuid,
    /// Timestamp
    pub timestamp: i64,
    /// State at time of experience
    pub state: HashMap<String, String>,
    /// Action taken
    pub action: String,
    /// Outcome/reward
    pub outcome: f64,
    /// Contextual features
    pub context: HashMap<String, f64>,
}

/// Memory system for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    /// Agent identifier
    pub agent_id: Uuid,
    /// Episodic memory (experiences)
    pub episodic_memory: Vec<Experience>,
    /// Semantic memory (learned patterns)
    pub semantic_memory: HashMap<String, f64>,
    /// Working memory capacity
    pub working_memory_capacity: usize,
    /// Memory decay rate (for forgetting)
    pub decay_rate: f64,
}

impl AgentMemory {
    /// Create a new memory system
    pub fn new(agent_id: Uuid, capacity: usize, decay_rate: f64) -> Self {
        Self {
            agent_id,
            episodic_memory: Vec::new(),
            semantic_memory: HashMap::new(),
            working_memory_capacity: capacity,
            decay_rate,
        }
    }

    /// Store an experience
    pub fn store_experience(&mut self, experience: Experience) {
        self.episodic_memory.push(experience);

        // Limit memory size (keep most recent)
        if self.episodic_memory.len() > self.working_memory_capacity {
            self.episodic_memory.remove(0);
        }
    }

    /// Retrieve similar experiences
    pub fn recall_similar(&self, state: &HashMap<String, String>, k: usize) -> Vec<Experience> {
        let mut experiences: Vec<_> = self
            .episodic_memory
            .iter()
            .map(|exp| {
                let similarity = self.compute_similarity(state, &exp.state);
                (exp.clone(), similarity)
            })
            .collect();

        experiences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        experiences
            .into_iter()
            .take(k)
            .map(|(exp, _)| exp)
            .collect()
    }

    /// Compute similarity between states (simple Jaccard similarity)
    fn compute_similarity(
        &self,
        state1: &HashMap<String, String>,
        state2: &HashMap<String, String>,
    ) -> f64 {
        let mut common = 0;
        let mut total = 0;

        for (key, value) in state1 {
            total += 1;
            if state2.get(key) == Some(value) {
                common += 1;
            }
        }

        for key in state2.keys() {
            if !state1.contains_key(key) {
                total += 1;
            }
        }

        if total == 0 {
            0.0
        } else {
            common as f64 / total as f64
        }
    }

    /// Learn pattern from experiences
    pub fn learn_pattern(&mut self, pattern_name: &str, value: f64) {
        let current = self
            .semantic_memory
            .get(pattern_name)
            .copied()
            .unwrap_or(0.0);
        // Exponential moving average
        let new_value = 0.9 * current + 0.1 * value;
        self.semantic_memory
            .insert(pattern_name.to_string(), new_value);
    }

    /// Apply memory decay
    pub fn apply_decay(&mut self) {
        for value in self.semantic_memory.values_mut() {
            *value *= 1.0 - self.decay_rate;
        }
    }

    /// Get pattern strength
    pub fn get_pattern_strength(&self, pattern_name: &str) -> f64 {
        self.semantic_memory
            .get(pattern_name)
            .copied()
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qlearning_agent_creation() {
        let agent = QLearningAgent::new(0.1, 0.9, 0.3);
        assert_eq!(agent.learning_rate, 0.1);
        assert_eq!(agent.discount_factor, 0.9);
        assert_eq!(agent.exploration_rate, 0.3);
    }

    #[test]
    fn test_qlearning_update() {
        let mut agent = QLearningAgent::new(0.1, 0.9, 0.0);
        let actions = vec!["left".to_string(), "right".to_string()];

        agent.update_q_value("s1", "left", 10.0, "s2", &actions);
        assert!(agent.get_q_value("s1", "left") > 0.0);
    }

    #[test]
    fn test_qlearning_action_selection() {
        let mut agent = QLearningAgent::new(0.1, 0.9, 0.0);
        let actions = vec!["left".to_string(), "right".to_string()];

        // Set higher Q-value for "right"
        agent.q_table.insert(
            "s1".to_string(),
            [("right".to_string(), 10.0)].iter().cloned().collect(),
        );

        let action = agent.choose_action("s1", &actions);
        assert_eq!(action, Some("right".to_string()));
    }

    #[test]
    fn test_qlearning_exploration_decay() {
        let mut agent = QLearningAgent::new(0.1, 0.9, 0.5);
        let initial = agent.exploration_rate;
        agent.decay_exploration();
        assert!(agent.exploration_rate < initial);
        assert!(agent.exploration_rate >= agent.min_exploration_rate);
    }

    #[test]
    fn test_sarsa_agent() {
        let mut agent = SarsaAgent::new(0.1, 0.9, 0.3);
        agent.update_q_value("s1", "a1", 5.0, "s2", "a2");
        // Just verify no panic
        assert_eq!(agent.learning_rate, 0.1);
    }

    #[test]
    fn test_payoff_matrix_prisoners_dilemma() {
        let game = PayoffMatrix::prisoners_dilemma();
        assert_eq!(game.player1_actions.len(), 2);
        assert_eq!(game.player2_actions.len(), 2);

        // Check defect-defect outcome
        let payoff = game.get_payoff(1, 1);
        assert_eq!(payoff, Some((-2.0, -2.0)));
    }

    #[test]
    fn test_nash_equilibrium() {
        let game = PayoffMatrix::prisoners_dilemma();
        let equilibria = game.find_pure_nash_equilibria();

        // Prisoner's dilemma has one Nash equilibrium: (Defect, Defect)
        assert_eq!(equilibria.len(), 1);
        assert_eq!(equilibria[0], (1, 1));
    }

    #[test]
    fn test_game_theoretic_agent() {
        let mut agent = GameTheoreticAgent::new();
        agent.observe_opponent_action("cooperate");
        agent.observe_opponent_action("cooperate");
        agent.observe_opponent_action("defect");

        let freq = agent.get_opponent_frequency("cooperate");
        assert!((freq - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_game_theoretic_best_response() {
        let mut agent = GameTheoreticAgent::new();
        agent.observe_opponent_action("Cooperate");
        agent.observe_opponent_action("Cooperate");

        let game = PayoffMatrix::prisoners_dilemma();
        let best = agent.best_response(&game);

        // If opponent cooperates, best response is to defect
        assert_eq!(best, Some(1));
    }

    #[test]
    fn test_bounded_rationality_satisficing() {
        let agent = BoundedRationalityAgent::new(5.0, 10);
        let options = vec![
            ("low".to_string(), 3.0),
            ("medium".to_string(), 6.0),
            ("high".to_string(), 9.0),
        ];

        let choice = agent.satisfice(&options);
        assert_eq!(choice, Some("medium".to_string()));
    }

    #[test]
    fn test_bounded_rationality_heuristic() {
        let mut agent = BoundedRationalityAgent::new(5.0, 10);
        agent.set_heuristic_weight("cost", -1.0);
        agent.set_heuristic_weight("quality", 2.0);

        let mut options = HashMap::new();
        let mut option1 = HashMap::new();
        option1.insert("cost".to_string(), 10.0);
        option1.insert("quality".to_string(), 5.0);
        options.insert("opt1".to_string(), option1);

        let choice = agent.heuristic_decision(&options);
        assert_eq!(choice, Some("opt1".to_string()));
    }

    #[test]
    fn test_bdi_agent_beliefs() {
        let mut agent = BdiAgent::new(0.5);
        let belief = Belief {
            id: "b1".to_string(),
            content: "weather is sunny".to_string(),
            certainty: 0.9,
            last_updated: 0,
        };

        agent.add_belief(belief);
        assert_eq!(agent.beliefs.len(), 1);
    }

    #[test]
    fn test_bdi_agent_deliberation() {
        let mut agent = BdiAgent::new(0.5);

        let desire = Desire {
            id: "d1".to_string(),
            description: "achieve goal".to_string(),
            priority: 0.8,
            satisfaction_condition: "goal_reached".to_string(),
        };

        agent.add_desire(desire);
        agent.deliberate();

        assert_eq!(agent.intentions.len(), 1);
    }

    #[test]
    fn test_bdi_agent_action() {
        let mut agent = BdiAgent::new(0.5);

        let desire = Desire {
            id: "d1".to_string(),
            description: "test".to_string(),
            priority: 0.9,
            satisfaction_condition: "done".to_string(),
        };

        agent.add_desire(desire);
        agent.deliberate();

        let actions = agent.act();
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_agent_memory_storage() {
        let agent_id = Uuid::new_v4();
        let mut memory = AgentMemory::new(agent_id, 100, 0.01);

        let exp = Experience {
            id: Uuid::new_v4(),
            timestamp: 0,
            state: HashMap::new(),
            action: "test".to_string(),
            outcome: 5.0,
            context: HashMap::new(),
        };

        memory.store_experience(exp);
        assert_eq!(memory.episodic_memory.len(), 1);
    }

    #[test]
    fn test_agent_memory_capacity() {
        let agent_id = Uuid::new_v4();
        let mut memory = AgentMemory::new(agent_id, 2, 0.01);

        for i in 0..5 {
            let exp = Experience {
                id: Uuid::new_v4(),
                timestamp: i,
                state: HashMap::new(),
                action: format!("action_{}", i),
                outcome: i as f64,
                context: HashMap::new(),
            };
            memory.store_experience(exp);
        }

        assert_eq!(memory.episodic_memory.len(), 2);
    }

    #[test]
    fn test_agent_memory_pattern_learning() {
        let agent_id = Uuid::new_v4();
        let mut memory = AgentMemory::new(agent_id, 100, 0.01);

        memory.learn_pattern("success_rate", 0.8);
        memory.learn_pattern("success_rate", 0.9);

        let strength = memory.get_pattern_strength("success_rate");
        assert!(strength > 0.0);
    }

    #[test]
    fn test_agent_memory_decay() {
        let agent_id = Uuid::new_v4();
        let mut memory = AgentMemory::new(agent_id, 100, 0.1);

        memory.learn_pattern("temp", 1.0);
        let before = memory.get_pattern_strength("temp");

        memory.apply_decay();
        let after = memory.get_pattern_strength("temp");

        assert!(after < before);
    }

    #[test]
    fn test_agent_memory_recall() {
        let agent_id = Uuid::new_v4();
        let mut memory = AgentMemory::new(agent_id, 100, 0.01);

        let mut state1 = HashMap::new();
        state1.insert("location".to_string(), "A".to_string());

        let exp = Experience {
            id: Uuid::new_v4(),
            timestamp: 0,
            state: state1.clone(),
            action: "move".to_string(),
            outcome: 10.0,
            context: HashMap::new(),
        };

        memory.store_experience(exp);

        let similar = memory.recall_similar(&state1, 1);
        assert_eq!(similar.len(), 1);
    }
}
