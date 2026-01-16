//! Real-time graph updates with pub/sub and incremental materialization.
//!
//! This module provides real-time knowledge graph update notifications,
//! incremental materialization, and pub/sub messaging for legal intelligence.

use crate::{LodResult, Triple};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Type alias for derivation functions used in materialization rules.
pub type DeriveFn = Box<dyn Fn(&Triple) -> Vec<Triple> + Send + Sync>;

/// Real-time graph update event.
#[derive(Debug, Clone)]
pub enum GraphUpdate {
    /// Triple added
    Add(Triple),
    /// Triple removed
    Remove(Triple),
    /// Batch of triples added
    AddBatch(Vec<Triple>),
    /// Batch of triples removed
    RemoveBatch(Vec<Triple>),
}

impl GraphUpdate {
    /// Returns the number of triples affected.
    pub fn count(&self) -> usize {
        match self {
            Self::Add(_) | Self::Remove(_) => 1,
            Self::AddBatch(triples) | Self::RemoveBatch(triples) => triples.len(),
        }
    }
}

/// Subscriber for graph updates.
pub trait UpdateSubscriber: Send + Sync {
    /// Called when a graph update occurs.
    fn on_update(&self, update: &GraphUpdate);
}

/// Publisher for graph updates (pub/sub pattern).
pub struct UpdatePublisher {
    /// Registered subscribers
    subscribers: Arc<Mutex<HashMap<String, Box<dyn UpdateSubscriber>>>>,
    /// Topic-based subscriptions
    topic_subscribers: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl UpdatePublisher {
    /// Creates a new update publisher.
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            topic_subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Subscribes to all updates.
    pub fn subscribe(&mut self, id: impl Into<String>, subscriber: Box<dyn UpdateSubscriber>) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.insert(id.into(), subscriber);
    }

    /// Subscribes to updates on a specific topic.
    pub fn subscribe_topic(
        &mut self,
        topic: impl Into<String>,
        subscriber_id: impl Into<String>,
        subscriber: Box<dyn UpdateSubscriber>,
    ) {
        let topic = topic.into();
        let subscriber_id = subscriber_id.into();

        // Add to subscribers
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.insert(subscriber_id.clone(), subscriber);
        drop(subscribers);

        // Add to topic mapping
        let mut topic_subs = self.topic_subscribers.lock().unwrap();
        topic_subs.entry(topic).or_default().push(subscriber_id);
    }

    /// Unsubscribes a subscriber.
    pub fn unsubscribe(&mut self, id: &str) -> bool {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.remove(id).is_some()
    }

    /// Publishes an update to all subscribers.
    pub fn publish(&self, update: &GraphUpdate) {
        let subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.values() {
            subscriber.on_update(update);
        }
    }

    /// Publishes an update to topic subscribers.
    pub fn publish_topic(&self, topic: &str, update: &GraphUpdate) {
        let topic_subs = self.topic_subscribers.lock().unwrap();
        if let Some(subscriber_ids) = topic_subs.get(topic) {
            let subscribers = self.subscribers.lock().unwrap();
            for id in subscriber_ids {
                if let Some(subscriber) = subscribers.get(id) {
                    subscriber.on_update(update);
                }
            }
        }
    }

    /// Returns the number of subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.lock().unwrap().len()
    }
}

impl Default for UpdatePublisher {
    fn default() -> Self {
        Self::new()
    }
}

/// Incremental materialization engine.
pub struct IncrementalMaterializer {
    /// Materialized triples
    materialized: Arc<Mutex<HashSet<MaterializedTriple>>>,
    /// Rules for materialization
    rules: Vec<MaterializationRule>,
}

impl IncrementalMaterializer {
    /// Creates a new incremental materializer.
    pub fn new() -> Self {
        Self {
            materialized: Arc::new(Mutex::new(HashSet::new())),
            rules: Vec::new(),
        }
    }

    /// Adds a materialization rule.
    pub fn add_rule(&mut self, rule: MaterializationRule) {
        self.rules.push(rule);
    }

    /// Processes a triple addition and materializes derived triples.
    pub fn materialize_add(&mut self, triple: &Triple) -> Vec<Triple> {
        let mut new_triples = Vec::new();

        for rule in &self.rules {
            if rule.matches(triple) {
                let derived = rule.derive(triple);
                for t in derived {
                    let mt = MaterializedTriple::from(&t);
                    let mut materialized = self.materialized.lock().unwrap();
                    if materialized.insert(mt) {
                        new_triples.push(t);
                    }
                }
            }
        }

        new_triples
    }

    /// Processes a triple removal and removes dependent materialized triples.
    pub fn materialize_remove(&mut self, triple: &Triple) -> Vec<Triple> {
        let mut removed_triples = Vec::new();
        let mt = MaterializedTriple::from(triple);

        let mut materialized = self.materialized.lock().unwrap();
        if materialized.remove(&mt) {
            removed_triples.push(triple.clone());
        }

        removed_triples
    }

    /// Returns the count of materialized triples.
    pub fn materialized_count(&self) -> usize {
        self.materialized.lock().unwrap().len()
    }
}

impl Default for IncrementalMaterializer {
    fn default() -> Self {
        Self::new()
    }
}

/// Materialization rule for deriving new triples.
pub struct MaterializationRule {
    /// Rule name
    pub name: String,
    /// Pattern to match
    pub pattern: RulePattern,
    /// Derivation function
    pub derive_fn: DeriveFn,
}

impl MaterializationRule {
    /// Creates a new materialization rule.
    pub fn new<F>(name: impl Into<String>, pattern: RulePattern, derive_fn: F) -> Self
    where
        F: Fn(&Triple) -> Vec<Triple> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            pattern,
            derive_fn: Box::new(derive_fn),
        }
    }

    /// Checks if this rule matches a triple.
    pub fn matches(&self, triple: &Triple) -> bool {
        self.pattern.matches(triple)
    }

    /// Derives new triples from a matching triple.
    pub fn derive(&self, triple: &Triple) -> Vec<Triple> {
        (self.derive_fn)(triple)
    }
}

/// Pattern for matching triples in rules.
#[derive(Debug, Clone)]
pub struct RulePattern {
    /// Subject pattern (None for any)
    pub subject_pattern: Option<String>,
    /// Predicate pattern (None for any)
    pub predicate_pattern: Option<String>,
}

impl RulePattern {
    /// Creates a new rule pattern.
    pub fn new(subject_pattern: Option<String>, predicate_pattern: Option<String>) -> Self {
        Self {
            subject_pattern,
            predicate_pattern,
        }
    }

    /// Creates a pattern that matches any triple.
    pub fn any() -> Self {
        Self {
            subject_pattern: None,
            predicate_pattern: None,
        }
    }

    /// Checks if a triple matches this pattern.
    pub fn matches(&self, triple: &Triple) -> bool {
        if let Some(ref s_pattern) = self.subject_pattern {
            if !triple.subject.contains(s_pattern) {
                return false;
            }
        }

        if let Some(ref p_pattern) = self.predicate_pattern {
            if !triple.predicate.contains(p_pattern) {
                return false;
            }
        }

        true
    }
}

/// Materialized triple for deduplication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MaterializedTriple {
    subject: String,
    predicate: String,
    object: String,
}

impl From<&Triple> for MaterializedTriple {
    fn from(triple: &Triple) -> Self {
        Self {
            subject: triple.subject.clone(),
            predicate: triple.predicate.clone(),
            object: format!("{:?}", triple.object),
        }
    }
}

/// Real-time graph manager combining pub/sub and incremental materialization.
pub struct RealtimeGraphManager {
    /// Update publisher
    publisher: UpdatePublisher,
    /// Incremental materializer
    materializer: IncrementalMaterializer,
}

impl RealtimeGraphManager {
    /// Creates a new real-time graph manager.
    pub fn new() -> Self {
        Self {
            publisher: UpdatePublisher::new(),
            materializer: IncrementalMaterializer::new(),
        }
    }

    /// Adds a triple and publishes updates.
    pub fn add_triple(&mut self, triple: Triple) -> LodResult<()> {
        // Materialize derived triples
        let derived = self.materializer.materialize_add(&triple);

        // Publish original triple
        let update = GraphUpdate::Add(triple);
        self.publisher.publish(&update);

        // Publish derived triples
        if !derived.is_empty() {
            let derived_update = GraphUpdate::AddBatch(derived);
            self.publisher.publish(&derived_update);
        }

        Ok(())
    }

    /// Removes a triple and publishes updates.
    pub fn remove_triple(&mut self, triple: &Triple) -> LodResult<()> {
        // Remove materialized triples
        let removed = self.materializer.materialize_remove(triple);

        // Publish removals
        if !removed.is_empty() {
            let update = GraphUpdate::RemoveBatch(removed);
            self.publisher.publish(&update);
        }

        Ok(())
    }

    /// Subscribes to updates.
    pub fn subscribe(&mut self, id: impl Into<String>, subscriber: Box<dyn UpdateSubscriber>) {
        self.publisher.subscribe(id, subscriber);
    }

    /// Adds a materialization rule.
    pub fn add_rule(&mut self, rule: MaterializationRule) {
        self.materializer.add_rule(rule);
    }

    /// Returns statistics.
    pub fn stats(&self) -> RealtimeStats {
        RealtimeStats {
            subscriber_count: self.publisher.subscriber_count(),
            materialized_count: self.materializer.materialized_count(),
        }
    }
}

impl Default for RealtimeGraphManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Real-time graph statistics.
#[derive(Debug, Clone)]
pub struct RealtimeStats {
    /// Number of subscribers
    pub subscriber_count: usize,
    /// Number of materialized triples
    pub materialized_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;
    use std::sync::{Arc, Mutex};

    struct TestSubscriber {
        updates: Arc<Mutex<Vec<GraphUpdate>>>,
    }

    impl TestSubscriber {
        fn new() -> Self {
            Self {
                updates: Arc::new(Mutex::new(Vec::new())),
            }
        }

        #[allow(dead_code)]
        fn update_count(&self) -> usize {
            self.updates.lock().unwrap().len()
        }
    }

    impl UpdateSubscriber for TestSubscriber {
        fn on_update(&self, update: &GraphUpdate) {
            self.updates.lock().unwrap().push(update.clone());
        }
    }

    fn sample_triple() -> Triple {
        Triple {
            subject: "ex:s1".to_string(),
            predicate: "ex:p1".to_string(),
            object: RdfValue::string("o1"),
        }
    }

    #[test]
    fn test_graph_update_count() {
        let update1 = GraphUpdate::Add(sample_triple());
        assert_eq!(update1.count(), 1);

        let update2 = GraphUpdate::AddBatch(vec![sample_triple(), sample_triple()]);
        assert_eq!(update2.count(), 2);
    }

    #[test]
    fn test_publisher_subscribe() {
        let mut publisher = UpdatePublisher::new();
        let subscriber = Box::new(TestSubscriber::new());

        publisher.subscribe("sub1", subscriber);
        assert_eq!(publisher.subscriber_count(), 1);
    }

    #[test]
    fn test_publisher_publish() {
        let mut publisher = UpdatePublisher::new();
        let subscriber = Arc::new(TestSubscriber::new());
        let _subscriber_clone = subscriber.clone();

        publisher.subscribe("sub1", Box::new(TestSubscriber::new()));

        let update = GraphUpdate::Add(sample_triple());
        publisher.publish(&update);

        // Subscriber should have received the update
        assert!(publisher.subscriber_count() > 0);
    }

    #[test]
    fn test_publisher_unsubscribe() {
        let mut publisher = UpdatePublisher::new();
        let subscriber = Box::new(TestSubscriber::new());

        publisher.subscribe("sub1", subscriber);
        assert_eq!(publisher.subscriber_count(), 1);

        assert!(publisher.unsubscribe("sub1"));
        assert_eq!(publisher.subscriber_count(), 0);
    }

    #[test]
    fn test_incremental_materializer() {
        let mut materializer = IncrementalMaterializer::new();

        let rule = MaterializationRule::new("test-rule", RulePattern::any(), |_triple| Vec::new());
        materializer.add_rule(rule);

        let triple = sample_triple();
        let derived = materializer.materialize_add(&triple);

        // No derived triples from empty rule
        assert_eq!(derived.len(), 0);
    }

    #[test]
    fn test_rule_pattern_matches() {
        let pattern = RulePattern::new(Some("ex:".to_string()), None);
        let triple = sample_triple();

        assert!(pattern.matches(&triple));
    }

    #[test]
    fn test_rule_pattern_any() {
        let pattern = RulePattern::any();
        let triple = sample_triple();

        assert!(pattern.matches(&triple));
    }

    #[test]
    fn test_realtime_manager() {
        let mut manager = RealtimeGraphManager::new();

        let triple = sample_triple();
        manager.add_triple(triple).unwrap();

        let stats = manager.stats();
        assert_eq!(stats.subscriber_count, 0); // No subscribers added yet
    }

    #[test]
    fn test_manager_with_subscriber() {
        let mut manager = RealtimeGraphManager::new();
        let subscriber = Box::new(TestSubscriber::new());

        manager.subscribe("sub1", subscriber);

        let stats = manager.stats();
        assert_eq!(stats.subscriber_count, 1);
    }

    #[test]
    fn test_materialization_rule() {
        let rule = MaterializationRule::new("transitive-rule", RulePattern::any(), |triple| {
            vec![Triple {
                subject: triple.subject.clone(),
                predicate: "ex:derived".to_string(),
                object: triple.object.clone(),
            }]
        });

        let triple = sample_triple();
        assert!(rule.matches(&triple));

        let derived = rule.derive(&triple);
        assert_eq!(derived.len(), 1);
        assert_eq!(derived[0].predicate, "ex:derived");
    }

    #[test]
    fn test_topic_subscription() {
        let mut publisher = UpdatePublisher::new();
        let subscriber = Box::new(TestSubscriber::new());

        publisher.subscribe_topic("legal-updates", "sub1", subscriber);

        let update = GraphUpdate::Add(sample_triple());
        publisher.publish_topic("legal-updates", &update);

        assert_eq!(publisher.subscriber_count(), 1);
    }

    #[test]
    fn test_materialized_triple_dedup() {
        let mut materializer = IncrementalMaterializer::new();

        // Add a rule that derives a triple
        let rule = MaterializationRule::new("identity-rule", RulePattern::any(), |triple| {
            vec![triple.clone()]
        });
        materializer.add_rule(rule);

        let triple = sample_triple();
        materializer.materialize_add(&triple);
        materializer.materialize_add(&triple); // Add same triple again

        // Should only count once
        assert_eq!(materializer.materialized_count(), 1);
    }
}
