//! JSON-LD (linked data) export for statutes, dependency graphs and timelines.
//!
//! JSON-LD attaches an explicit `@context` to plain JSON so that the data
//! becomes a node-and-edge graph addressable by IRI, ready for semantic-web
//! tooling, triple stores or knowledge graphs. Every statute is emitted as a
//! typed node under a configurable base IRI, with dependencies expressed as
//! IRI-valued `dependsOn` links so consumers can follow them.

use serde_json::{Map, Number, Value};

use super::{effect_type_label, timeline_event_parts};
use crate::functions::{VizResult, format_condition};
use crate::types_3::Timeline;
use crate::types_4::DependencyGraph;
use crate::types_5::VizError;
use legalis_core::Statute;
use std::collections::HashMap;

/// Exports crate model types as JSON-LD documents.
#[derive(Debug, Clone)]
pub struct JsonLdExporter {
    base_iri: String,
    vocab: String,
}

impl JsonLdExporter {
    /// Creates a new exporter with default example IRIs.
    pub fn new() -> Self {
        Self {
            base_iri: "https://legalis.example/statute/".to_string(),
            vocab: "https://legalis.example/vocab#".to_string(),
        }
    }

    /// Sets the base IRI prepended to statute identifiers to form node `@id`s.
    pub fn with_base_iri(mut self, base_iri: impl Into<String>) -> Self {
        self.base_iri = base_iri.into();
        self
    }

    /// Sets the default vocabulary IRI used for the `@vocab` term.
    pub fn with_vocab(mut self, vocab: impl Into<String>) -> Self {
        self.vocab = vocab.into();
        self
    }

    /// Builds a JSON-LD document for a list of statutes.
    pub fn statutes_to_json_ld(&self, statutes: &[Statute]) -> Value {
        let graph: Vec<Value> = statutes.iter().map(|s| self.statute_node(s)).collect();
        self.document(graph)
    }

    /// Builds a JSON-LD document for a dependency graph.
    ///
    /// Each node id becomes a typed `Statute` node, and outgoing edges become
    /// IRI-valued `dependsOn` links.
    pub fn dependency_graph_to_json_ld(&self, graph: &DependencyGraph) -> Value {
        let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
        for edge in graph.graph.edge_indices() {
            let Some((source, target)) = graph.graph.edge_endpoints(edge) else {
                continue;
            };
            if let (Some(from), Some(to)) = (
                graph.graph.node_weight(source),
                graph.graph.node_weight(target),
            ) {
                outgoing.entry(from.clone()).or_default().push(to.clone());
            }
        }
        let mut nodes = Vec::new();
        for index in graph.graph.node_indices() {
            let id = match graph.graph.node_weight(index) {
                Some(id) => id.clone(),
                None => continue,
            };
            let mut node = Map::new();
            node.insert("@id".to_string(), self.iri(&id));
            node.insert("@type".to_string(), Value::String("Statute".to_string()));
            node.insert("identifier".to_string(), Value::String(id.clone()));
            if let Some(dependencies) = outgoing.get(&id) {
                let links: Vec<Value> = dependencies.iter().map(|dep| self.iri(dep)).collect();
                node.insert("dependsOn".to_string(), Value::Array(links));
            }
            nodes.push(Value::Object(node));
        }
        self.document(nodes)
    }

    /// Builds a JSON-LD document for a timeline.
    pub fn timeline_to_json_ld(&self, timeline: &Timeline) -> Value {
        let mut nodes = Vec::new();
        for (date, event) in &timeline.events {
            let (event_type, statute_id, detail) = timeline_event_parts(event);
            let mut node = Map::new();
            node.insert(
                "@type".to_string(),
                Value::String("TimelineEvent".to_string()),
            );
            node.insert("date".to_string(), Value::String(date.clone()));
            node.insert(
                "eventType".to_string(),
                Value::String(event_type.to_string()),
            );
            node.insert("statute".to_string(), self.iri(statute_id));
            if let Some(detail) = detail {
                node.insert("detail".to_string(), Value::String(detail.to_string()));
            }
            nodes.push(Value::Object(node));
        }
        self.document(nodes)
    }

    /// Serializes a JSON-LD [`Value`] to a compact string.
    ///
    /// # Errors
    ///
    /// Returns [`VizError::ExportError`] if serialization fails.
    pub fn to_json_string(&self, value: &Value) -> VizResult<String> {
        serde_json::to_string(value).map_err(|e| VizError::ExportError(e.to_string()))
    }

    /// Serializes a JSON-LD [`Value`] to a pretty-printed string.
    ///
    /// # Errors
    ///
    /// Returns [`VizError::ExportError`] if serialization fails.
    pub fn to_json_string_pretty(&self, value: &Value) -> VizResult<String> {
        serde_json::to_string_pretty(value).map_err(|e| VizError::ExportError(e.to_string()))
    }

    fn document(&self, graph: Vec<Value>) -> Value {
        let mut root = Map::new();
        root.insert("@context".to_string(), self.context());
        root.insert("@graph".to_string(), Value::Array(graph));
        Value::Object(root)
    }

    fn context(&self) -> Value {
        let mut context = Map::new();
        context.insert("@vocab".to_string(), Value::String(self.vocab.clone()));
        context.insert("id".to_string(), Value::String("@id".to_string()));
        context.insert("type".to_string(), Value::String("@type".to_string()));
        let mut depends_on = Map::new();
        depends_on.insert("@id".to_string(), Value::String("dependsOn".to_string()));
        depends_on.insert("@type".to_string(), Value::String("@id".to_string()));
        context.insert("dependsOn".to_string(), Value::Object(depends_on));
        let mut statute_ref = Map::new();
        statute_ref.insert("@type".to_string(), Value::String("@id".to_string()));
        context.insert("statute".to_string(), Value::Object(statute_ref));
        Value::Object(context)
    }

    fn statute_node(&self, statute: &Statute) -> Value {
        let mut node = Map::new();
        node.insert("@id".to_string(), self.iri(&statute.id));
        node.insert("@type".to_string(), Value::String("Statute".to_string()));
        node.insert("identifier".to_string(), Value::String(statute.id.clone()));
        node.insert("title".to_string(), Value::String(statute.title.clone()));
        node.insert(
            "effectType".to_string(),
            Value::String(effect_type_label(&statute.effect.effect_type).to_string()),
        );
        node.insert(
            "description".to_string(),
            Value::String(statute.effect.description.clone()),
        );
        node.insert(
            "version".to_string(),
            Value::Number(Number::from(statute.version)),
        );
        if let Some(jurisdiction) = &statute.jurisdiction {
            node.insert(
                "jurisdiction".to_string(),
                Value::String(jurisdiction.clone()),
            );
        }
        if let Some(discretion) = &statute.discretion_logic {
            node.insert(
                "discretionLogic".to_string(),
                Value::String(discretion.clone()),
            );
        }
        if !statute.derives_from.is_empty() {
            let links: Vec<Value> = statute.derives_from.iter().map(|d| self.iri(d)).collect();
            node.insert("dependsOn".to_string(), Value::Array(links));
        }
        if !statute.applies_to.is_empty() {
            let entities: Vec<Value> = statute
                .applies_to
                .iter()
                .map(|a| Value::String(a.clone()))
                .collect();
            node.insert("appliesTo".to_string(), Value::Array(entities));
        }
        if !statute.preconditions.is_empty() {
            let conditions: Vec<Value> = statute
                .preconditions
                .iter()
                .map(|c| Value::String(format_condition(c)))
                .collect();
            node.insert("precondition".to_string(), Value::Array(conditions));
        }
        Value::Object(node)
    }

    fn iri(&self, id: &str) -> Value {
        Value::String(format!("{}{}", self.base_iri, id))
    }
}

impl Default for JsonLdExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_5::TimelineEvent;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn sample_statute() -> Statute {
        Statute::new(
            "s1",
            "Adult Rights",
            Effect::new(EffectType::Grant, "Grants rights"),
        )
        .with_jurisdiction("US")
        .with_version(2)
        .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18))
        .with_derives_from("federal-1")
        .with_applies_to("Person")
    }

    #[test]
    fn statutes_document_has_context_and_graph() {
        let exporter = JsonLdExporter::new();
        let doc = exporter.statutes_to_json_ld(&[sample_statute()]);
        let serialized = exporter.to_json_string(&doc).expect("serialize");
        // It must be valid JSON that parses back to an equivalent value.
        let parsed: Value = serde_json::from_str(&serialized).expect("valid json");
        assert_eq!(parsed, doc);
        assert!(doc.get("@context").is_some());
        let graph = doc.get("@graph").and_then(Value::as_array).expect("graph");
        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn statute_node_carries_expected_fields() {
        let exporter = JsonLdExporter::new().with_base_iri("urn:law:");
        let doc = exporter.statutes_to_json_ld(&[sample_statute()]);
        let node = &doc["@graph"][0];
        assert_eq!(node["@id"], Value::String("urn:law:s1".to_string()));
        assert_eq!(node["@type"], Value::String("Statute".to_string()));
        assert_eq!(node["effectType"], Value::String("Grant".to_string()));
        assert_eq!(node["version"], Value::Number(Number::from(2u32)));
        assert_eq!(node["jurisdiction"], Value::String("US".to_string()));
        assert_eq!(
            node["dependsOn"],
            Value::Array(vec![Value::String("urn:law:federal-1".to_string())])
        );
        assert!(node["precondition"].as_array().is_some());
    }

    #[test]
    fn dependency_graph_emits_depends_on_links() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "requires");
        graph.add_dependency("a", "c", "requires");
        let exporter = JsonLdExporter::new().with_base_iri("urn:law:");
        let doc = exporter.dependency_graph_to_json_ld(&graph);
        let nodes = doc["@graph"].as_array().expect("graph");
        let node_a = nodes
            .iter()
            .find(|n| n["identifier"] == Value::String("a".to_string()))
            .expect("node a");
        let deps = node_a["dependsOn"].as_array().expect("depends");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&Value::String("urn:law:b".to_string())));
    }

    #[test]
    fn timeline_events_become_typed_nodes() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Enacted {
                statute_id: "s1".to_string(),
                title: "Enacted Law".to_string(),
            },
        );
        let exporter = JsonLdExporter::new().with_base_iri("urn:law:");
        let doc = exporter.timeline_to_json_ld(&timeline);
        let node = &doc["@graph"][0];
        assert_eq!(node["@type"], Value::String("TimelineEvent".to_string()));
        assert_eq!(node["eventType"], Value::String("Enacted".to_string()));
        assert_eq!(node["statute"], Value::String("urn:law:s1".to_string()));
        assert_eq!(node["detail"], Value::String("Enacted Law".to_string()));
    }
}
