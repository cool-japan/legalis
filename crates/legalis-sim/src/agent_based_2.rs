//! Agent-Based Modeling 2.0 - Advanced agent intelligence and coordination.
//!
//! This module provides advanced agent-based modeling capabilities including:
//! - Deep reinforcement learning (DQN, Actor-Critic)
//! - Multi-agent coordination protocols
//! - Emergent behavior detection
//! - Social network dynamics
//! - Cultural evolution modeling

use rand::prelude::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

// ============================================================================
// Deep Reinforcement Learning Agents
// ============================================================================

/// Experience replay buffer for DQN training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceReplay {
    /// Buffer of experiences (state, action, reward, next_state, done).
    buffer: VecDeque<DQNExperience>,
    /// Maximum buffer capacity.
    capacity: usize,
}

/// Single experience tuple for replay buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DQNExperience {
    /// State representation (vector of features).
    pub state: Vec<f64>,
    /// Action taken.
    pub action: usize,
    /// Reward received.
    pub reward: f64,
    /// Next state.
    pub next_state: Vec<f64>,
    /// Whether episode terminated.
    pub done: bool,
}

impl ExperienceReplay {
    /// Creates a new experience replay buffer.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Adds an experience to the buffer.
    pub fn add(&mut self, experience: DQNExperience) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(experience);
    }

    /// Samples a random batch of experiences.
    pub fn sample(&self, batch_size: usize) -> Vec<DQNExperience> {
        let mut rng = rand::rng();

        let sample_size = batch_size.min(self.buffer.len());
        self.buffer
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, sample_size)
            .cloned()
            .collect()
    }

    /// Returns the current size of the buffer.
    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    /// Checks if buffer has enough samples for training.
    pub fn can_sample(&self, batch_size: usize) -> bool {
        self.buffer.len() >= batch_size
    }
}

/// Deep Q-Network agent with experience replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DQNAgent {
    /// Agent identifier.
    pub id: Uuid,
    /// Q-value table: (state_hash, action) -> Q-value.
    q_table: HashMap<(u64, usize), f64>,
    /// Experience replay buffer.
    replay_buffer: ExperienceReplay,
    /// Learning rate.
    pub alpha: f64,
    /// Discount factor.
    pub gamma: f64,
    /// Epsilon for epsilon-greedy policy.
    pub epsilon: f64,
    /// Minimum epsilon value.
    pub epsilon_min: f64,
    /// Epsilon decay rate.
    pub epsilon_decay: f64,
    /// Number of actions available.
    pub num_actions: usize,
    /// Batch size for training.
    pub batch_size: usize,
    /// Number of training steps performed.
    pub training_steps: usize,
}

impl DQNAgent {
    /// Creates a new DQN agent.
    pub fn new(
        num_actions: usize,
        alpha: f64,
        gamma: f64,
        epsilon: f64,
        buffer_capacity: usize,
        batch_size: usize,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            q_table: HashMap::new(),
            replay_buffer: ExperienceReplay::new(buffer_capacity),
            alpha,
            gamma,
            epsilon,
            epsilon_min: 0.01,
            epsilon_decay: 0.995,
            num_actions,
            batch_size,
            training_steps: 0,
        }
    }

    /// Chooses an action using epsilon-greedy policy.
    pub fn choose_action(&self, state: &[f64]) -> usize {
        use rand::Rng;
        let mut rng = rand::rng();

        if rng.random_range(0.0..1.0) < self.epsilon {
            // Explore: random action
            rng.random_range(0..self.num_actions)
        } else {
            // Exploit: best action
            self.best_action(state)
        }
    }

    /// Returns the best action for a given state.
    pub fn best_action(&self, state: &[f64]) -> usize {
        let state_hash = Self::hash_state(state);
        (0..self.num_actions)
            .max_by(|a, b| {
                let q_a = self.q_table.get(&(state_hash, *a)).unwrap_or(&0.0);
                let q_b = self.q_table.get(&(state_hash, *b)).unwrap_or(&0.0);
                q_a.partial_cmp(q_b).unwrap()
            })
            .unwrap_or(0)
    }

    /// Adds an experience to the replay buffer.
    pub fn remember(&mut self, experience: DQNExperience) {
        self.replay_buffer.add(experience);
    }

    /// Trains the agent on a batch of experiences.
    pub fn train(&mut self) {
        if !self.replay_buffer.can_sample(self.batch_size) {
            return;
        }

        let batch = self.replay_buffer.sample(self.batch_size);

        for exp in batch {
            let state_hash = Self::hash_state(&exp.state);
            let next_state_hash = Self::hash_state(&exp.next_state);

            // Compute target Q-value
            let next_q_max = if exp.done {
                0.0
            } else {
                (0..self.num_actions)
                    .map(|a| *self.q_table.get(&(next_state_hash, a)).unwrap_or(&0.0))
                    .fold(f64::NEG_INFINITY, f64::max)
            };

            let target = exp.reward + self.gamma * next_q_max;

            // Update Q-value
            let current_q = *self.q_table.get(&(state_hash, exp.action)).unwrap_or(&0.0);
            let new_q = current_q + self.alpha * (target - current_q);
            self.q_table.insert((state_hash, exp.action), new_q);
        }

        // Decay epsilon
        if self.epsilon > self.epsilon_min {
            self.epsilon *= self.epsilon_decay;
        }

        self.training_steps += 1;
    }

    /// Hashes a state vector to a u64.
    fn hash_state(state: &[f64]) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for &val in state {
            (val * 1000.0).round().to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }
}

/// Actor-Critic agent for policy gradient methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorCriticAgent {
    /// Agent identifier.
    pub id: Uuid,
    /// Actor policy: (state_hash, action) -> probability.
    actor: HashMap<(u64, usize), f64>,
    /// Critic value function: state_hash -> value.
    critic: HashMap<u64, f64>,
    /// Actor learning rate.
    pub alpha_actor: f64,
    /// Critic learning rate.
    pub alpha_critic: f64,
    /// Discount factor.
    pub gamma: f64,
    /// Number of actions available.
    pub num_actions: usize,
}

impl ActorCriticAgent {
    /// Creates a new Actor-Critic agent.
    pub fn new(num_actions: usize, alpha_actor: f64, alpha_critic: f64, gamma: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            actor: HashMap::new(),
            critic: HashMap::new(),
            alpha_actor,
            alpha_critic,
            gamma,
            num_actions,
        }
    }

    /// Chooses an action using the current policy.
    pub fn choose_action(&self, state: &[f64]) -> usize {
        use rand::Rng;
        let mut rng = rand::rng();

        let state_hash = Self::hash_state(state);
        let mut action_probs = Vec::with_capacity(self.num_actions);
        let mut sum = 0.0;

        for action in 0..self.num_actions {
            let prob = *self.actor.get(&(state_hash, action)).unwrap_or(&1.0);
            action_probs.push(prob);
            sum += prob;
        }

        // Normalize probabilities
        if sum > 0.0 {
            for prob in &mut action_probs {
                *prob /= sum;
            }
        } else {
            // Uniform distribution if no policy learned
            let uniform = 1.0 / self.num_actions as f64;
            action_probs.fill(uniform);
        }

        // Sample from distribution
        let r: f64 = rng.random_range(0.0..1.0);
        let mut cumsum = 0.0;
        for (action, &prob) in action_probs.iter().enumerate() {
            cumsum += prob;
            if r < cumsum {
                return action;
            }
        }

        self.num_actions - 1
    }

    /// Updates the agent based on a transition.
    pub fn update(
        &mut self,
        state: &[f64],
        action: usize,
        reward: f64,
        next_state: &[f64],
        done: bool,
    ) {
        let state_hash = Self::hash_state(state);
        let next_state_hash = Self::hash_state(next_state);

        // Compute TD error
        let v_curr = *self.critic.get(&state_hash).unwrap_or(&0.0);
        let v_next = if done {
            0.0
        } else {
            *self.critic.get(&next_state_hash).unwrap_or(&0.0)
        };

        let td_error = reward + self.gamma * v_next - v_curr;

        // Update critic
        let new_v = v_curr + self.alpha_critic * td_error;
        self.critic.insert(state_hash, new_v);

        // Update actor (increase probability of action if TD error is positive)
        let old_prob = *self.actor.get(&(state_hash, action)).unwrap_or(&1.0);
        let new_prob = (old_prob + self.alpha_actor * td_error).max(0.0);
        self.actor.insert((state_hash, action), new_prob);
    }

    /// Gets the state value from the critic.
    pub fn get_value(&self, state: &[f64]) -> f64 {
        let state_hash = Self::hash_state(state);
        *self.critic.get(&state_hash).unwrap_or(&0.0)
    }

    /// Hashes a state vector to a u64.
    fn hash_state(state: &[f64]) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for &val in state {
            (val * 1000.0).round().to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }
}

// ============================================================================
// Multi-Agent Coordination Protocols
// ============================================================================

/// Message types for multi-agent coordination.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoordinationMessage {
    /// Request for proposal (Contract Net Protocol).
    CFP {
        /// Task identifier.
        task_id: Uuid,
        /// Task description.
        description: String,
        /// Deadline for proposals.
        deadline: u64,
    },
    /// Proposal in response to CFP.
    Propose {
        /// Task identifier.
        task_id: Uuid,
        /// Bid amount.
        bid: f64,
        /// Capability score.
        capability: f64,
    },
    /// Accept proposal.
    Accept {
        /// Task identifier.
        task_id: Uuid,
    },
    /// Reject proposal.
    Reject {
        /// Task identifier.
        task_id: Uuid,
    },
    /// Request for cooperation (AMAS).
    Cooperate {
        /// Cooperation identifier.
        coop_id: Uuid,
        /// Agents involved.
        agents: Vec<Uuid>,
        /// Goal description.
        goal: String,
    },
    /// Agreement to cooperate.
    Agree {
        /// Cooperation identifier.
        coop_id: Uuid,
    },
    /// Information sharing.
    Inform {
        /// Information type.
        info_type: String,
        /// Information data.
        data: Vec<f64>,
    },
}

/// Contract Net Protocol coordinator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractNetProtocol {
    /// Coordinator identifier.
    pub id: Uuid,
    /// Active tasks.
    tasks: HashMap<Uuid, Task>,
    /// Proposals received.
    proposals: HashMap<Uuid, Vec<Proposal>>,
}

/// Task in Contract Net Protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task identifier.
    pub id: Uuid,
    /// Task description.
    pub description: String,
    /// Deadline timestamp.
    pub deadline: u64,
    /// Task complexity.
    pub complexity: f64,
}

/// Proposal for a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Agent identifier.
    pub agent_id: Uuid,
    /// Task identifier.
    pub task_id: Uuid,
    /// Bid amount.
    pub bid: f64,
    /// Capability score.
    pub capability: f64,
}

impl ContractNetProtocol {
    /// Creates a new Contract Net Protocol coordinator.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            tasks: HashMap::new(),
            proposals: HashMap::new(),
        }
    }

    /// Announces a new task.
    pub fn announce_task(&mut self, task: Task) -> CoordinationMessage {
        let task_id = task.id;
        let description = task.description.clone();
        let deadline = task.deadline;

        self.tasks.insert(task_id, task);
        self.proposals.insert(task_id, Vec::new());

        CoordinationMessage::CFP {
            task_id,
            description,
            deadline,
        }
    }

    /// Receives a proposal for a task.
    pub fn receive_proposal(&mut self, proposal: Proposal) {
        if let Some(props) = self.proposals.get_mut(&proposal.task_id) {
            props.push(proposal);
        }
    }

    /// Selects the best proposal for a task based on bid and capability.
    pub fn select_best_proposal(&self, task_id: &Uuid) -> Option<Uuid> {
        self.proposals.get(task_id).and_then(|props| {
            props
                .iter()
                .max_by(|a, b| {
                    let score_a = a.capability / a.bid.max(0.001);
                    let score_b = b.capability / b.bid.max(0.001);
                    score_a.partial_cmp(&score_b).unwrap()
                })
                .map(|p| p.agent_id)
        })
    }

    /// Awards the task to the selected agent.
    pub fn award_task(&mut self, task_id: &Uuid, agent_id: &Uuid) -> Vec<CoordinationMessage> {
        let mut messages = Vec::new();

        if let Some(props) = self.proposals.get(task_id) {
            for prop in props {
                if &prop.agent_id == agent_id {
                    messages.push(CoordinationMessage::Accept { task_id: *task_id });
                } else {
                    messages.push(CoordinationMessage::Reject { task_id: *task_id });
                }
            }
        }

        messages
    }
}

impl Default for ContractNetProtocol {
    fn default() -> Self {
        Self::new()
    }
}

/// Adaptive Multi-Agent System (AMAS) coordinator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AMASCoordinator {
    /// Coordinator identifier.
    pub id: Uuid,
    /// Active cooperations.
    cooperations: HashMap<Uuid, Cooperation>,
    /// Agent capabilities.
    agent_capabilities: HashMap<Uuid, Vec<String>>,
}

/// Cooperation instance in AMAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooperation {
    /// Cooperation identifier.
    pub id: Uuid,
    /// Goal description.
    pub goal: String,
    /// Participating agents.
    pub agents: Vec<Uuid>,
    /// Cooperation status.
    pub active: bool,
}

impl AMASCoordinator {
    /// Creates a new AMAS coordinator.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            cooperations: HashMap::new(),
            agent_capabilities: HashMap::new(),
        }
    }

    /// Registers an agent's capabilities.
    pub fn register_agent(&mut self, agent_id: Uuid, capabilities: Vec<String>) {
        self.agent_capabilities.insert(agent_id, capabilities);
    }

    /// Forms a cooperation for a goal.
    pub fn form_cooperation(
        &mut self,
        goal: String,
        required_capabilities: Vec<String>,
    ) -> Option<Cooperation> {
        // Find agents with required capabilities
        let mut selected_agents = Vec::new();
        let mut covered_capabilities = std::collections::HashSet::new();

        for (agent_id, caps) in &self.agent_capabilities {
            for cap in caps {
                if required_capabilities.contains(cap) && !covered_capabilities.contains(cap) {
                    selected_agents.push(*agent_id);
                    covered_capabilities.insert(cap.clone());
                    break;
                }
            }

            if covered_capabilities.len() == required_capabilities.len() {
                break;
            }
        }

        if covered_capabilities.len() == required_capabilities.len() {
            let coop = Cooperation {
                id: Uuid::new_v4(),
                goal,
                agents: selected_agents,
                active: true,
            };
            let coop_id = coop.id;
            self.cooperations.insert(coop_id, coop.clone());
            Some(coop)
        } else {
            None
        }
    }

    /// Gets active cooperations for an agent.
    pub fn get_agent_cooperations(&self, agent_id: &Uuid) -> Vec<&Cooperation> {
        self.cooperations
            .values()
            .filter(|c| c.active && c.agents.contains(agent_id))
            .collect()
    }

    /// Ends a cooperation.
    pub fn end_cooperation(&mut self, coop_id: &Uuid) {
        if let Some(coop) = self.cooperations.get_mut(coop_id) {
            coop.active = false;
        }
    }
}

impl Default for AMASCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Emergent Behavior Detection
// ============================================================================

/// Emergent behavior pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentPattern {
    /// Pattern identifier.
    pub id: Uuid,
    /// Pattern name.
    pub name: String,
    /// Pattern description.
    pub description: String,
    /// Agents involved.
    pub agents: Vec<Uuid>,
    /// Time step when detected.
    pub detected_at: u64,
    /// Confidence score.
    pub confidence: f64,
    /// Pattern metrics.
    pub metrics: HashMap<String, f64>,
}

/// Emergent behavior detector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentBehaviorDetector {
    /// Detector identifier.
    pub id: Uuid,
    /// Detected patterns.
    patterns: Vec<EmergentPattern>,
    /// Detection threshold.
    pub threshold: f64,
    /// Agent behavior history: agent_id -> behavior sequence.
    behavior_history: HashMap<Uuid, Vec<Vec<f64>>>,
}

impl EmergentBehaviorDetector {
    /// Creates a new emergent behavior detector.
    pub fn new(threshold: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            patterns: Vec::new(),
            threshold,
            behavior_history: HashMap::new(),
        }
    }

    /// Records agent behavior at a time step.
    pub fn record_behavior(&mut self, agent_id: Uuid, behavior: Vec<f64>) {
        self.behavior_history
            .entry(agent_id)
            .or_default()
            .push(behavior);
    }

    /// Detects clustering patterns (agents behaving similarly).
    pub fn detect_clustering(&mut self, time_step: u64, min_cluster_size: usize) {
        let mut clusters: Vec<Vec<Uuid>> = Vec::new();

        // Get recent behaviors
        let recent_behaviors: Vec<(Uuid, &Vec<f64>)> = self
            .behavior_history
            .iter()
            .filter_map(|(id, history)| history.last().map(|b| (*id, b)))
            .collect();

        // Simple clustering based on behavior similarity
        for (agent_id, behavior) in &recent_behaviors {
            let mut added_to_cluster = false;

            for cluster in &mut clusters {
                if let Some(first_agent) = cluster.first()
                    && let Some((_, first_behavior)) =
                        recent_behaviors.iter().find(|(id, _)| id == first_agent)
                    && Self::behavior_similarity(behavior, first_behavior) > self.threshold
                {
                    cluster.push(*agent_id);
                    added_to_cluster = true;
                    break;
                }
            }

            if !added_to_cluster {
                clusters.push(vec![*agent_id]);
            }
        }

        // Report clusters that meet minimum size
        for cluster in clusters {
            if cluster.len() >= min_cluster_size {
                let mut metrics = HashMap::new();
                metrics.insert("cluster_size".to_string(), cluster.len() as f64);

                self.patterns.push(EmergentPattern {
                    id: Uuid::new_v4(),
                    name: "behavioral_clustering".to_string(),
                    description: format!(
                        "Cluster of {} agents with similar behavior",
                        cluster.len()
                    ),
                    agents: cluster,
                    detected_at: time_step,
                    confidence: 0.8,
                    metrics,
                });
            }
        }
    }

    /// Detects coordination patterns (agents acting in sync).
    pub fn detect_coordination(&mut self, time_step: u64, window_size: usize) {
        if self
            .behavior_history
            .values()
            .all(|h| h.len() >= window_size)
        {
            let agents: Vec<Uuid> = self.behavior_history.keys().copied().collect();

            // Check for synchronized behavior changes
            if agents.len() >= 2 {
                for i in 0..agents.len() {
                    for j in (i + 1)..agents.len() {
                        let agent_a = agents[i];
                        let agent_b = agents[j];

                        if let (Some(history_a), Some(history_b)) = (
                            self.behavior_history.get(&agent_a),
                            self.behavior_history.get(&agent_b),
                        ) {
                            let sync_score =
                                Self::synchronization_score(history_a, history_b, window_size);

                            if sync_score > self.threshold {
                                let mut metrics = HashMap::new();
                                metrics.insert("synchronization_score".to_string(), sync_score);

                                self.patterns.push(EmergentPattern {
                                    id: Uuid::new_v4(),
                                    name: "coordination".to_string(),
                                    description: "Agents showing coordinated behavior".to_string(),
                                    agents: vec![agent_a, agent_b],
                                    detected_at: time_step,
                                    confidence: sync_score,
                                    metrics,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    /// Gets all detected patterns.
    pub fn get_patterns(&self) -> &[EmergentPattern] {
        &self.patterns
    }

    /// Computes behavior similarity (cosine similarity).
    fn behavior_similarity(a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }

    /// Computes synchronization score between two behavior histories.
    fn synchronization_score(a: &[Vec<f64>], b: &[Vec<f64>], window_size: usize) -> f64 {
        let start = a.len().saturating_sub(window_size);
        let window_a = &a[start..];
        let window_b = &b[start..];

        if window_a.len() != window_b.len() {
            return 0.0;
        }

        let similarities: Vec<f64> = window_a
            .iter()
            .zip(window_b.iter())
            .map(|(ba, bb)| Self::behavior_similarity(ba, bb))
            .collect();

        if similarities.is_empty() {
            0.0
        } else {
            similarities.iter().sum::<f64>() / similarities.len() as f64
        }
    }
}

// ============================================================================
// Social Network Dynamics
// ============================================================================

/// Social network node representing an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNode {
    /// Agent identifier.
    pub id: Uuid,
    /// Influence score.
    pub influence: f64,
    /// Trust scores for other agents.
    pub trust: HashMap<Uuid, f64>,
    /// Opinion on various topics.
    pub opinions: HashMap<String, f64>,
}

/// Social network dynamics simulator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNetworkDynamics {
    /// Network identifier.
    pub id: Uuid,
    /// Nodes in the network.
    nodes: HashMap<Uuid, SocialNode>,
    /// Edges (connections) between agents.
    edges: HashMap<(Uuid, Uuid), f64>,
    /// Influence propagation rate.
    pub propagation_rate: f64,
}

impl SocialNetworkDynamics {
    /// Creates a new social network dynamics simulator.
    pub fn new(propagation_rate: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            propagation_rate,
        }
    }

    /// Adds a node to the network.
    pub fn add_node(&mut self, node: SocialNode) {
        self.nodes.insert(node.id, node);
    }

    /// Adds a connection (edge) between two agents.
    pub fn add_edge(&mut self, from: Uuid, to: Uuid, weight: f64) {
        self.edges.insert((from, to), weight);
    }

    /// Simulates opinion propagation for one time step.
    pub fn propagate_opinions(&mut self, topic: &str) {
        let mut opinion_updates: HashMap<Uuid, f64> = HashMap::new();

        for (node_id, node) in &self.nodes {
            let mut weighted_opinions = 0.0;
            let mut total_weight = 0.0;

            // Collect opinions from neighbors
            for ((from, to), &edge_weight) in &self.edges {
                if to == node_id
                    && let Some(neighbor) = self.nodes.get(from)
                    && let Some(&neighbor_opinion) = neighbor.opinions.get(topic)
                {
                    let trust = node.trust.get(from).unwrap_or(&0.5);
                    let weight = edge_weight * trust;
                    weighted_opinions += neighbor_opinion * weight;
                    total_weight += weight;
                }
            }

            // Update opinion based on neighbors
            if total_weight > 0.0 {
                let neighbor_avg = weighted_opinions / total_weight;
                let current_opinion = node.opinions.get(topic).unwrap_or(&0.0);
                let new_opinion =
                    current_opinion + self.propagation_rate * (neighbor_avg - current_opinion);
                opinion_updates.insert(*node_id, new_opinion);
            }
        }

        // Apply updates
        for (node_id, new_opinion) in opinion_updates {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.opinions.insert(topic.to_string(), new_opinion);
            }
        }
    }

    /// Computes network polarization for a topic.
    pub fn compute_polarization(&self, topic: &str) -> f64 {
        let opinions: Vec<f64> = self
            .nodes
            .values()
            .filter_map(|n| n.opinions.get(topic).copied())
            .collect();

        if opinions.is_empty() {
            return 0.0;
        }

        let mean = opinions.iter().sum::<f64>() / opinions.len() as f64;
        let variance =
            opinions.iter().map(|o| (o - mean).powi(2)).sum::<f64>() / opinions.len() as f64;

        variance.sqrt()
    }

    /// Gets the number of nodes in the network.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Gets the number of edges in the network.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Gets a node by ID.
    pub fn get_node(&self, id: &Uuid) -> Option<&SocialNode> {
        self.nodes.get(id)
    }
}

// ============================================================================
// Cultural Evolution
// ============================================================================

/// Meme (cultural unit) in cultural evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meme {
    /// Meme identifier.
    pub id: Uuid,
    /// Meme content/type.
    pub content: String,
    /// Fitness/attractiveness score.
    pub fitness: f64,
    /// Mutation rate.
    pub mutation_rate: f64,
}

/// Agent with cultural traits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalAgent {
    /// Agent identifier.
    pub id: Uuid,
    /// Memes possessed by the agent.
    pub memes: Vec<Meme>,
    /// Openness to new memes.
    pub openness: f64,
}

/// Cultural evolution simulator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalEvolution {
    /// Simulator identifier.
    pub id: Uuid,
    /// Cultural agents.
    agents: HashMap<Uuid, CulturalAgent>,
    /// Global meme pool.
    meme_pool: Vec<Meme>,
    /// Selection pressure (higher = stronger selection).
    pub selection_pressure: f64,
}

impl CulturalEvolution {
    /// Creates a new cultural evolution simulator.
    pub fn new(selection_pressure: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            agents: HashMap::new(),
            meme_pool: Vec::new(),
            selection_pressure,
        }
    }

    /// Adds an agent to the simulation.
    pub fn add_agent(&mut self, agent: CulturalAgent) {
        // Add agent's memes to pool
        for meme in &agent.memes {
            if !self.meme_pool.iter().any(|m| m.id == meme.id) {
                self.meme_pool.push(meme.clone());
            }
        }

        self.agents.insert(agent.id, agent);
    }

    /// Simulates one generation of cultural transmission.
    pub fn simulate_generation(&mut self) {
        use rand::Rng;
        let mut rng = rand::rng();

        let agent_ids: Vec<Uuid> = self.agents.keys().copied().collect();

        for agent_id in &agent_ids {
            // Select a random neighbor for cultural transmission
            if agent_ids.len() > 1 {
                let neighbor_id = loop {
                    let idx = rng.random_range(0..agent_ids.len());
                    if agent_ids[idx] != *agent_id {
                        break agent_ids[idx];
                    }
                };

                // Transmit memes from neighbor
                if let (Some(agent), Some(neighbor)) =
                    (self.agents.get(agent_id), self.agents.get(&neighbor_id))
                    && !neighbor.memes.is_empty()
                    && rng.random_range(0.0..1.0) < agent.openness
                {
                    let meme_idx = rng.random_range(0..neighbor.memes.len());
                    let transmitted_meme = neighbor.memes[meme_idx].clone();

                    // Acceptance based on fitness and selection pressure
                    let acceptance_prob =
                        1.0 / (1.0 + (-self.selection_pressure * transmitted_meme.fitness).exp());

                    if rng.random_range(0.0..1.0) < acceptance_prob {
                        // Mutation
                        let mut meme = transmitted_meme;
                        if rng.random_range(0.0..1.0) < meme.mutation_rate {
                            meme.id = Uuid::new_v4();
                            meme.fitness += rng.random_range(-0.1..0.1);
                            meme.fitness = meme.fitness.clamp(0.0, 1.0);
                        }

                        // Add to agent's memes
                        if let Some(agent_mut) = self.agents.get_mut(agent_id) {
                            agent_mut.memes.push(meme.clone());

                            // Add to global pool if new
                            if !self.meme_pool.iter().any(|m| m.id == meme.id) {
                                self.meme_pool.push(meme);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Computes meme diversity (number of unique memes).
    pub fn compute_diversity(&self) -> usize {
        self.meme_pool.len()
    }

    /// Gets the most prevalent memes across all agents.
    pub fn get_prevalent_memes(&self, top_n: usize) -> Vec<(Uuid, usize)> {
        let mut meme_counts: HashMap<Uuid, usize> = HashMap::new();

        for agent in self.agents.values() {
            for meme in &agent.memes {
                *meme_counts.entry(meme.id).or_insert(0) += 1;
            }
        }

        let mut counts: Vec<(Uuid, usize)> = meme_counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts.truncate(top_n);

        counts
    }

    /// Gets the number of agents.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experience_replay_buffer() {
        let mut replay = ExperienceReplay::new(10);

        for i in 0..15 {
            replay.add(DQNExperience {
                state: vec![i as f64],
                action: 0,
                reward: 1.0,
                next_state: vec![i as f64 + 1.0],
                done: false,
            });
        }

        assert_eq!(replay.size(), 10);
        assert!(replay.can_sample(5));

        let sample = replay.sample(5);
        assert_eq!(sample.len(), 5);
    }

    #[test]
    fn test_dqn_agent_creation() {
        let agent = DQNAgent::new(4, 0.1, 0.99, 0.1, 1000, 32);

        assert_eq!(agent.num_actions, 4);
        assert_eq!(agent.alpha, 0.1);
        assert_eq!(agent.gamma, 0.99);
    }

    #[test]
    fn test_dqn_agent_choose_action() {
        let agent = DQNAgent::new(4, 0.1, 0.99, 0.0, 1000, 32);
        let state = vec![1.0, 2.0, 3.0];

        let action = agent.choose_action(&state);
        assert!(action < 4);
    }

    #[test]
    fn test_dqn_agent_training() {
        let mut agent = DQNAgent::new(4, 0.1, 0.99, 0.0, 1000, 32);

        // Add experiences
        for _ in 0..50 {
            agent.remember(DQNExperience {
                state: vec![1.0, 2.0],
                action: 0,
                reward: 1.0,
                next_state: vec![2.0, 3.0],
                done: false,
            });
        }

        let initial_steps = agent.training_steps;
        agent.train();
        assert!(agent.training_steps > initial_steps);
    }

    #[test]
    fn test_actor_critic_agent_creation() {
        let agent = ActorCriticAgent::new(4, 0.1, 0.1, 0.99);

        assert_eq!(agent.num_actions, 4);
        assert_eq!(agent.alpha_actor, 0.1);
        assert_eq!(agent.alpha_critic, 0.1);
    }

    #[test]
    fn test_actor_critic_agent_update() {
        let mut agent = ActorCriticAgent::new(4, 0.1, 0.1, 0.99);

        let state = vec![1.0, 2.0];
        let next_state = vec![2.0, 3.0];

        let initial_value = agent.get_value(&state);
        agent.update(&state, 0, 1.0, &next_state, false);
        let updated_value = agent.get_value(&state);

        assert_ne!(initial_value, updated_value);
    }

    #[test]
    fn test_contract_net_protocol() {
        let mut cnp = ContractNetProtocol::new();

        let task = Task {
            id: Uuid::new_v4(),
            description: "Test task".to_string(),
            deadline: 100,
            complexity: 5.0,
        };

        let task_id = task.id;
        let msg = cnp.announce_task(task);

        match msg {
            CoordinationMessage::CFP { task_id: tid, .. } => {
                assert_eq!(tid, task_id);
            }
            _ => panic!("Expected CFP message"),
        }
    }

    #[test]
    fn test_contract_net_proposal_selection() {
        let mut cnp = ContractNetProtocol::new();

        let task = Task {
            id: Uuid::new_v4(),
            description: "Test task".to_string(),
            deadline: 100,
            complexity: 5.0,
        };
        let task_id = task.id;
        cnp.announce_task(task);

        let agent1 = Uuid::new_v4();
        let agent2 = Uuid::new_v4();

        cnp.receive_proposal(Proposal {
            agent_id: agent1,
            task_id,
            bid: 10.0,
            capability: 0.5,
        });

        cnp.receive_proposal(Proposal {
            agent_id: agent2,
            task_id,
            bid: 5.0,
            capability: 0.8,
        });

        let best = cnp.select_best_proposal(&task_id);
        assert_eq!(best, Some(agent2)); // Better capability/cost ratio
    }

    #[test]
    fn test_amas_coordinator() {
        let mut amas = AMASCoordinator::new();

        let agent1 = Uuid::new_v4();
        let agent2 = Uuid::new_v4();

        amas.register_agent(agent1, vec!["skill_a".to_string(), "skill_b".to_string()]);
        amas.register_agent(agent2, vec!["skill_c".to_string()]);

        let coop = amas.form_cooperation(
            "Complete task".to_string(),
            vec!["skill_a".to_string(), "skill_c".to_string()],
        );

        assert!(coop.is_some());
        let coop = coop.unwrap();
        assert!(coop.agents.contains(&agent1));
        assert!(coop.agents.contains(&agent2));
    }

    #[test]
    fn test_emergent_behavior_detector() {
        let mut detector = EmergentBehaviorDetector::new(0.8);

        let agent1 = Uuid::new_v4();
        let agent2 = Uuid::new_v4();

        detector.record_behavior(agent1, vec![1.0, 2.0, 3.0]);
        detector.record_behavior(agent2, vec![1.0, 2.0, 3.0]);

        detector.detect_clustering(1, 2);

        let patterns = detector.get_patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].name, "behavioral_clustering");
    }

    #[test]
    fn test_coordination_detection() {
        let mut detector = EmergentBehaviorDetector::new(0.8);

        let agent1 = Uuid::new_v4();
        let agent2 = Uuid::new_v4();

        // Add synchronized behaviors
        for i in 0..5 {
            detector.record_behavior(agent1, vec![i as f64, i as f64]);
            detector.record_behavior(agent2, vec![i as f64, i as f64]);
        }

        detector.detect_coordination(5, 3);

        let patterns = detector.get_patterns();
        assert!(!patterns.is_empty());
        assert_eq!(patterns[0].name, "coordination");
    }

    #[test]
    fn test_social_network_dynamics() {
        let mut network = SocialNetworkDynamics::new(0.1);

        let agent1 = Uuid::new_v4();
        let agent2 = Uuid::new_v4();

        let mut node1 = SocialNode {
            id: agent1,
            influence: 0.8,
            trust: HashMap::new(),
            opinions: HashMap::new(),
        };
        node1.opinions.insert("topic1".to_string(), 0.9);
        node1.trust.insert(agent2, 0.7);

        let mut node2 = SocialNode {
            id: agent2,
            influence: 0.6,
            trust: HashMap::new(),
            opinions: HashMap::new(),
        };
        node2.opinions.insert("topic1".to_string(), 0.1);
        node2.trust.insert(agent1, 0.7);

        network.add_node(node1);
        network.add_node(node2);
        network.add_edge(agent1, agent2, 1.0);
        network.add_edge(agent2, agent1, 1.0);

        assert_eq!(network.node_count(), 2);
        assert_eq!(network.edge_count(), 2);

        network.propagate_opinions("topic1");

        let polarization = network.compute_polarization("topic1");
        assert!(polarization >= 0.0);
    }

    #[test]
    fn test_cultural_evolution() {
        let mut evolution = CulturalEvolution::new(1.0);

        let meme1 = Meme {
            id: Uuid::new_v4(),
            content: "Idea A".to_string(),
            fitness: 0.8,
            mutation_rate: 0.1,
        };

        let agent1 = CulturalAgent {
            id: Uuid::new_v4(),
            memes: vec![meme1],
            openness: 0.5,
        };

        let meme2 = Meme {
            id: Uuid::new_v4(),
            content: "Idea B".to_string(),
            fitness: 0.6,
            mutation_rate: 0.1,
        };

        let agent2 = CulturalAgent {
            id: Uuid::new_v4(),
            memes: vec![meme2],
            openness: 0.5,
        };

        evolution.add_agent(agent1);
        evolution.add_agent(agent2);

        assert_eq!(evolution.agent_count(), 2);
        assert_eq!(evolution.compute_diversity(), 2);

        evolution.simulate_generation();

        // Diversity might increase due to transmission and mutation
        let new_diversity = evolution.compute_diversity();
        assert!(new_diversity >= 2);
    }

    #[test]
    fn test_prevalent_memes() {
        let mut evolution = CulturalEvolution::new(1.0);

        let meme_id = Uuid::new_v4();
        let meme = Meme {
            id: meme_id,
            content: "Popular idea".to_string(),
            fitness: 0.9,
            mutation_rate: 0.05,
        };

        // Add multiple agents with same meme
        for _ in 0..3 {
            let agent = CulturalAgent {
                id: Uuid::new_v4(),
                memes: vec![meme.clone()],
                openness: 0.5,
            };
            evolution.add_agent(agent);
        }

        let prevalent = evolution.get_prevalent_memes(1);
        assert_eq!(prevalent.len(), 1);
        assert_eq!(prevalent[0].0, meme_id);
        assert_eq!(prevalent[0].1, 3);
    }
}
