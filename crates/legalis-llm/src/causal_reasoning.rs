//! Causal Reasoning for Legal Analysis
//!
//! Enables cause-and-effect analysis for legal scenarios, including
//! causation determination, counterfactual reasoning, and causal inference.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Causal event in a legal scenario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CausalEvent {
    /// Unique identifier
    pub id: String,
    /// Event description
    pub description: String,
    /// Timestamp (optional)
    pub timestamp: Option<i64>,
    /// Event type
    pub event_type: EventType,
    /// Properties
    pub properties: HashMap<String, String>,
}

impl CausalEvent {
    /// Creates a new causal event.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        event_type: EventType,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            timestamp: None,
            event_type,
            properties: HashMap::new(),
        }
    }

    /// Sets the timestamp.
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Adds a property.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Type of causal event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// Action by an agent
    Action,
    /// State change
    StateChange,
    /// External condition
    Condition,
    /// Legal consequence
    Consequence,
}

/// Causal link between events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CausalLink {
    /// Cause event ID
    pub cause: String,
    /// Effect event ID
    pub effect: String,
    /// Type of causation
    pub causation_type: CausationType,
    /// Strength of causal relationship (0.0-1.0)
    pub strength: f64,
    /// Evidence supporting this causal link
    pub evidence: Vec<String>,
}

impl CausalLink {
    /// Creates a new causal link.
    pub fn new(
        cause: impl Into<String>,
        effect: impl Into<String>,
        causation_type: CausationType,
    ) -> Self {
        Self {
            cause: cause.into(),
            effect: effect.into(),
            causation_type,
            strength: 1.0,
            evidence: Vec::new(),
        }
    }

    /// Sets the strength.
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Adds evidence.
    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence.push(evidence.into());
        self
    }
}

/// Type of causation in legal analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CausationType {
    /// Factual causation (but-for test)
    FactualCausation,
    /// Legal causation (proximate cause)
    LegalCausation,
    /// Intervening cause
    InterveningCause,
    /// Concurrent cause
    ConcurrentCause,
    /// Superseding cause
    SupersedingCause,
}

/// Causal graph for legal scenarios
pub struct CausalGraph {
    /// Name of the scenario
    name: String,
    /// Events in the graph
    events: HashMap<String, CausalEvent>,
    /// Causal links
    links: Vec<CausalLink>,
}

impl CausalGraph {
    /// Creates a new causal graph.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            events: HashMap::new(),
            links: Vec::new(),
        }
    }

    /// Adds an event to the graph.
    pub fn add_event(&mut self, event: CausalEvent) {
        self.events.insert(event.id.clone(), event);
    }

    /// Adds a causal link.
    pub fn add_link(&mut self, link: CausalLink) -> Result<()> {
        // Validate events exist
        if !self.events.contains_key(&link.cause) {
            return Err(anyhow!("Cause event not found: {}", link.cause));
        }
        if !self.events.contains_key(&link.effect) {
            return Err(anyhow!("Effect event not found: {}", link.effect));
        }

        // Check for cycles (simplified - only direct cycles)
        if link.cause == link.effect {
            return Err(anyhow!("Self-causation not allowed"));
        }

        self.links.push(link);
        Ok(())
    }

    /// Gets an event by ID.
    pub fn get_event(&self, id: &str) -> Option<&CausalEvent> {
        self.events.get(id)
    }

    /// Gets all causes of an event.
    pub fn get_causes(&self, event_id: &str) -> Vec<&CausalLink> {
        self.links.iter().filter(|l| l.effect == event_id).collect()
    }

    /// Gets all effects of an event.
    pub fn get_effects(&self, event_id: &str) -> Vec<&CausalLink> {
        self.links.iter().filter(|l| l.cause == event_id).collect()
    }

    /// Performs but-for test: would the effect occur without the cause?
    pub fn but_for_test(&self, cause_id: &str, effect_id: &str) -> bool {
        // Check if there's a direct or indirect causal path
        self.has_causal_path(cause_id, effect_id)
    }

    /// Checks if there's a causal path from cause to effect.
    pub fn has_causal_path(&self, from: &str, to: &str) -> bool {
        let mut visited = HashSet::new();
        let mut queue = vec![from];

        while let Some(current) = queue.pop() {
            if current == to {
                return true;
            }

            if visited.contains(current) {
                continue;
            }
            visited.insert(current);

            for link in self.get_effects(current) {
                queue.push(&link.effect);
            }
        }

        false
    }

    /// Finds all causal paths from one event to another.
    pub fn find_causal_paths(&self, from: &str, to: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();
        let mut visited = HashSet::new();

        self.find_paths_recursive(from, to, &mut current_path, &mut visited, &mut paths);

        paths
    }

    fn find_paths_recursive(
        &self,
        current: &str,
        target: &str,
        current_path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        if current == target {
            current_path.push(current.to_string());
            all_paths.push(current_path.clone());
            current_path.pop();
            return;
        }

        if visited.contains(current) {
            return;
        }

        visited.insert(current.to_string());
        current_path.push(current.to_string());

        for link in self.get_effects(current) {
            self.find_paths_recursive(&link.effect, target, current_path, visited, all_paths);
        }

        current_path.pop();
        visited.remove(current);
    }

    /// Identifies proximate causes (legally relevant causes).
    pub fn find_proximate_causes(&self, effect_id: &str) -> Vec<String> {
        let mut proximate = Vec::new();

        for link in self.get_causes(effect_id) {
            if matches!(
                link.causation_type,
                CausationType::FactualCausation | CausationType::LegalCausation
            ) && link.strength >= 0.5
            {
                proximate.push(link.cause.clone());
            }
        }

        proximate
    }

    /// Identifies intervening causes that break the causal chain.
    pub fn find_intervening_causes(&self, effect_id: &str) -> Vec<String> {
        let mut intervening = Vec::new();

        for link in self.get_causes(effect_id) {
            if link.causation_type == CausationType::InterveningCause {
                intervening.push(link.cause.clone());
            }
        }

        intervening
    }

    /// Performs counterfactual analysis: what if the cause didn't happen?
    pub fn counterfactual_analysis(&self, cause_id: &str) -> CounterfactualResult {
        let mut affected_events = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![cause_id];

        while let Some(current) = queue.pop() {
            if visited.contains(current) {
                continue;
            }
            visited.insert(current);

            for link in self.get_effects(current) {
                affected_events.push(link.effect.clone());
                queue.push(&link.effect);
            }
        }

        let affected_count = affected_events.len();
        CounterfactualResult {
            removed_cause: cause_id.to_string(),
            affected_events,
            analysis: format!(
                "Removing event '{}' would affect {} downstream events",
                cause_id, affected_count
            ),
        }
    }

    /// Calculates causal attribution scores for all causes of an effect.
    pub fn calculate_attribution(&self, effect_id: &str) -> HashMap<String, f64> {
        let causes = self.get_causes(effect_id);
        let mut attributions = HashMap::new();

        if causes.is_empty() {
            return attributions;
        }

        let total_strength: f64 = causes.iter().map(|l| l.strength).sum();
        let causes_count = causes.len();

        for link in &causes {
            let attribution = if total_strength > 0.0 {
                link.strength / total_strength
            } else {
                1.0 / causes_count as f64
            };
            attributions.insert(link.cause.clone(), attribution);
        }

        attributions
    }

    /// Exports the causal graph in DOT format for visualization.
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str(&format!("digraph \"{}\" {{\n", self.name));
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Add nodes
        for event in self.events.values() {
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\"];\n",
                event.id, event.description
            ));
        }

        dot.push('\n');

        // Add edges
        for link in &self.links {
            let label = format!("{:?} ({:.2})", link.causation_type, link.strength);
            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                link.cause, link.effect, label
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

/// Result of counterfactual analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualResult {
    /// The cause that was removed
    pub removed_cause: String,
    /// Events that would be affected
    pub affected_events: Vec<String>,
    /// Analysis description
    pub analysis: String,
}

/// Legal causation analyzer
pub struct LegalCausationAnalyzer;

impl LegalCausationAnalyzer {
    /// Analyzes tort causation (negligence cases).
    pub fn analyze_tort_causation(graph: &CausalGraph, harm_id: &str) -> CausationAnalysis {
        let factual_causes = graph.find_proximate_causes(harm_id);
        let intervening = graph.find_intervening_causes(harm_id);

        CausationAnalysis {
            effect: harm_id.to_string(),
            factual_causes: factual_causes.clone(),
            proximate_causes: factual_causes
                .into_iter()
                .filter(|c| !intervening.contains(c))
                .collect(),
            intervening_causes: intervening,
            analysis_type: "Tort (Negligence)".to_string(),
        }
    }

    /// Analyzes criminal causation.
    pub fn analyze_criminal_causation(graph: &CausalGraph, result_id: &str) -> CausationAnalysis {
        let factual_causes = graph.find_proximate_causes(result_id);
        let intervening = graph.find_intervening_causes(result_id);

        CausationAnalysis {
            effect: result_id.to_string(),
            factual_causes: factual_causes.clone(),
            proximate_causes: factual_causes
                .into_iter()
                .filter(|c| !intervening.contains(c))
                .collect(),
            intervening_causes: intervening,
            analysis_type: "Criminal".to_string(),
        }
    }

    /// Analyzes contract breach causation.
    pub fn analyze_contract_causation(graph: &CausalGraph, damage_id: &str) -> CausationAnalysis {
        let factual_causes = graph.find_proximate_causes(damage_id);

        CausationAnalysis {
            effect: damage_id.to_string(),
            factual_causes: factual_causes.clone(),
            proximate_causes: factual_causes,
            intervening_causes: Vec::new(),
            analysis_type: "Contract Breach".to_string(),
        }
    }
}

/// Result of causation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausationAnalysis {
    /// The effect being analyzed
    pub effect: String,
    /// Factual causes (but-for test)
    pub factual_causes: Vec<String>,
    /// Proximate (legal) causes
    pub proximate_causes: Vec<String>,
    /// Intervening causes
    pub intervening_causes: Vec<String>,
    /// Type of legal analysis
    pub analysis_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_event() {
        let event = CausalEvent::new("e1", "Car accident", EventType::Action)
            .with_timestamp(1234567890)
            .with_property("location", "Main St");

        assert_eq!(event.id, "e1");
        assert_eq!(event.description, "Car accident");
        assert_eq!(event.timestamp, Some(1234567890));
        assert_eq!(
            event.properties.get("location"),
            Some(&"Main St".to_string())
        );
    }

    #[test]
    fn test_causal_link() {
        let link = CausalLink::new("e1", "e2", CausationType::FactualCausation)
            .with_strength(0.8)
            .with_evidence("Witness testimony");

        assert_eq!(link.cause, "e1");
        assert_eq!(link.effect, "e2");
        assert!((link.strength - 0.8).abs() < f64::EPSILON);
        assert_eq!(link.evidence.len(), 1);
    }

    #[test]
    fn test_causal_graph_basic() {
        let mut graph = CausalGraph::new("test");

        let e1 = CausalEvent::new("e1", "Event 1", EventType::Action);
        let e2 = CausalEvent::new("e2", "Event 2", EventType::Consequence);

        graph.add_event(e1);
        graph.add_event(e2);

        assert_eq!(graph.events.len(), 2);
        assert!(graph.get_event("e1").is_some());
    }

    #[test]
    fn test_causal_graph_links() {
        let mut graph = CausalGraph::new("test");

        graph.add_event(CausalEvent::new("e1", "Event 1", EventType::Action));
        graph.add_event(CausalEvent::new("e2", "Event 2", EventType::Consequence));

        let link = CausalLink::new("e1", "e2", CausationType::FactualCausation);
        graph.add_link(link).unwrap();

        assert_eq!(graph.links.len(), 1);
        assert_eq!(graph.get_effects("e1").len(), 1);
        assert_eq!(graph.get_causes("e2").len(), 1);
    }

    #[test]
    fn test_but_for_test() {
        let mut graph = CausalGraph::new("test");

        graph.add_event(CausalEvent::new("e1", "Negligence", EventType::Action));
        graph.add_event(CausalEvent::new("e2", "Injury", EventType::Consequence));

        graph
            .add_link(CausalLink::new("e1", "e2", CausationType::FactualCausation))
            .unwrap();

        assert!(graph.but_for_test("e1", "e2"));
        assert!(!graph.but_for_test("e2", "e1"));
    }

    #[test]
    fn test_causal_paths() {
        let mut graph = CausalGraph::new("test");

        graph.add_event(CausalEvent::new("e1", "Event 1", EventType::Action));
        graph.add_event(CausalEvent::new("e2", "Event 2", EventType::StateChange));
        graph.add_event(CausalEvent::new("e3", "Event 3", EventType::Consequence));

        graph
            .add_link(CausalLink::new("e1", "e2", CausationType::FactualCausation))
            .unwrap();
        graph
            .add_link(CausalLink::new("e2", "e3", CausationType::FactualCausation))
            .unwrap();

        let paths = graph.find_causal_paths("e1", "e3");
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec!["e1", "e2", "e3"]);
    }

    #[test]
    fn test_counterfactual_analysis() {
        let mut graph = CausalGraph::new("test");

        graph.add_event(CausalEvent::new("e1", "Cause", EventType::Action));
        graph.add_event(CausalEvent::new("e2", "Effect 1", EventType::StateChange));
        graph.add_event(CausalEvent::new("e3", "Effect 2", EventType::Consequence));

        graph
            .add_link(CausalLink::new("e1", "e2", CausationType::FactualCausation))
            .unwrap();
        graph
            .add_link(CausalLink::new("e1", "e3", CausationType::FactualCausation))
            .unwrap();

        let result = graph.counterfactual_analysis("e1");
        assert_eq!(result.removed_cause, "e1");
        assert_eq!(result.affected_events.len(), 2);
    }

    #[test]
    fn test_attribution() {
        let mut graph = CausalGraph::new("test");

        graph.add_event(CausalEvent::new("c1", "Cause 1", EventType::Action));
        graph.add_event(CausalEvent::new("c2", "Cause 2", EventType::Action));
        graph.add_event(CausalEvent::new("e", "Effect", EventType::Consequence));

        graph
            .add_link(
                CausalLink::new("c1", "e", CausationType::FactualCausation).with_strength(0.6),
            )
            .unwrap();
        graph
            .add_link(
                CausalLink::new("c2", "e", CausationType::FactualCausation).with_strength(0.4),
            )
            .unwrap();

        let attribution = graph.calculate_attribution("e");
        assert!((attribution.get("c1").unwrap() - 0.6).abs() < 1e-6);
        assert!((attribution.get("c2").unwrap() - 0.4).abs() < 1e-6);
    }

    #[test]
    fn test_tort_causation_analysis() {
        let mut graph = CausalGraph::new("negligence_case");

        graph.add_event(CausalEvent::new(
            "negligence",
            "Driver's negligence",
            EventType::Action,
        ));
        graph.add_event(CausalEvent::new(
            "injury",
            "Pedestrian injury",
            EventType::Consequence,
        ));

        graph
            .add_link(CausalLink::new(
                "negligence",
                "injury",
                CausationType::FactualCausation,
            ))
            .unwrap();

        let analysis = LegalCausationAnalyzer::analyze_tort_causation(&graph, "injury");
        assert_eq!(analysis.factual_causes.len(), 1);
        assert_eq!(analysis.factual_causes[0], "negligence");
    }
}
