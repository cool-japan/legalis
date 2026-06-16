//! Generic XML export for interoperability.
//!
//! Produces well-formed, indented XML that any XML toolchain (XSLT, XPath,
//! schema validators, downstream parsers) can consume. All element text and
//! attribute values are escaped, and the structure mirrors the crate's model so
//! statutes, dependency graphs and timelines map cleanly onto elements.

use super::{effect_type_label, escape_xml, timeline_event_parts};
use crate::functions::format_condition;
use crate::types_3::Timeline;
use crate::types_4::DependencyGraph;
use legalis_core::Statute;

/// Renders crate model types as XML documents.
#[derive(Debug, Clone)]
pub struct XmlExporter {
    indent: String,
    declaration: bool,
}

impl XmlExporter {
    /// Creates a new exporter with two-space indentation and an XML declaration.
    pub fn new() -> Self {
        Self {
            indent: "  ".to_string(),
            declaration: true,
        }
    }

    /// Sets the per-level indentation string.
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }

    /// Omits the leading `<?xml ...?>` declaration (useful when embedding).
    pub fn without_declaration(mut self) -> Self {
        self.declaration = false;
        self
    }

    /// Renders a `<statutes>` document.
    pub fn statutes_to_xml(&self, statutes: &[Statute]) -> String {
        let mut out = String::new();
        self.write_prologue(&mut out);
        self.write_line(&mut out, 0, "<statutes>");
        for statute in statutes {
            self.write_statute(&mut out, statute, 1);
        }
        self.write_line(&mut out, 0, "</statutes>");
        out
    }

    /// Renders a `<dependencyGraph>` document with `<nodes>` and `<edges>`.
    pub fn dependency_graph_to_xml(&self, graph: &DependencyGraph) -> String {
        let mut out = String::new();
        self.write_prologue(&mut out);
        self.write_line(&mut out, 0, "<dependencyGraph>");
        self.write_line(&mut out, 1, "<nodes>");
        for index in graph.graph.node_indices() {
            if let Some(id) = graph.graph.node_weight(index) {
                self.write_line(&mut out, 2, &format!("<node id=\"{}\"/>", escape_xml(id)));
            }
        }
        self.write_line(&mut out, 1, "</nodes>");
        self.write_line(&mut out, 1, "<edges>");
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let from = graph.graph.node_weight(source).cloned().unwrap_or_default();
                let to = graph.graph.node_weight(target).cloned().unwrap_or_default();
                let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
                self.write_line(
                    &mut out,
                    2,
                    &format!(
                        "<edge from=\"{}\" to=\"{}\" relation=\"{}\"/>",
                        escape_xml(&from),
                        escape_xml(&to),
                        escape_xml(&relation)
                    ),
                );
            }
        }
        self.write_line(&mut out, 1, "</edges>");
        self.write_line(&mut out, 0, "</dependencyGraph>");
        out
    }

    /// Renders a `<timeline>` document of `<event>` elements.
    pub fn timeline_to_xml(&self, timeline: &Timeline) -> String {
        let mut out = String::new();
        self.write_prologue(&mut out);
        self.write_line(&mut out, 0, "<timeline>");
        for (date, event) in &timeline.events {
            let (event_type, statute_id, detail) = timeline_event_parts(event);
            self.write_line(
                &mut out,
                1,
                &format!(
                    "<event date=\"{}\" type=\"{}\" statute=\"{}\">",
                    escape_xml(date),
                    escape_xml(event_type),
                    escape_xml(statute_id)
                ),
            );
            if let Some(detail) = detail {
                self.write_line(
                    &mut out,
                    2,
                    &format!("<detail>{}</detail>", escape_xml(detail)),
                );
            }
            self.write_line(&mut out, 1, "</event>");
        }
        self.write_line(&mut out, 0, "</timeline>");
        out
    }

    fn write_statute(&self, out: &mut String, statute: &Statute, depth: usize) {
        self.write_line(
            out,
            depth,
            &format!(
                "<statute id=\"{}\" version=\"{}\">",
                escape_xml(&statute.id),
                statute.version
            ),
        );
        self.write_line(
            out,
            depth + 1,
            &format!("<title>{}</title>", escape_xml(&statute.title)),
        );
        self.write_line(
            out,
            depth + 1,
            &format!(
                "<effect type=\"{}\">{}</effect>",
                escape_xml(effect_type_label(&statute.effect.effect_type)),
                escape_xml(&statute.effect.description)
            ),
        );
        if let Some(jurisdiction) = &statute.jurisdiction {
            self.write_line(
                out,
                depth + 1,
                &format!("<jurisdiction>{}</jurisdiction>", escape_xml(jurisdiction)),
            );
        }
        if let Some(discretion) = &statute.discretion_logic {
            self.write_line(
                out,
                depth + 1,
                &format!(
                    "<discretionLogic>{}</discretionLogic>",
                    escape_xml(discretion)
                ),
            );
        }
        if !statute.preconditions.is_empty() {
            self.write_line(out, depth + 1, "<preconditions>");
            for condition in &statute.preconditions {
                self.write_line(
                    out,
                    depth + 2,
                    &format!(
                        "<condition>{}</condition>",
                        escape_xml(&format_condition(condition))
                    ),
                );
            }
            self.write_line(out, depth + 1, "</preconditions>");
        }
        if !statute.applies_to.is_empty() {
            self.write_line(out, depth + 1, "<appliesTo>");
            for entity in &statute.applies_to {
                self.write_line(
                    out,
                    depth + 2,
                    &format!("<entityType>{}</entityType>", escape_xml(entity)),
                );
            }
            self.write_line(out, depth + 1, "</appliesTo>");
        }
        if !statute.derives_from.is_empty() {
            self.write_line(out, depth + 1, "<derivesFrom>");
            for source in &statute.derives_from {
                self.write_line(
                    out,
                    depth + 2,
                    &format!("<source>{}</source>", escape_xml(source)),
                );
            }
            self.write_line(out, depth + 1, "</derivesFrom>");
        }
        self.write_line(out, depth, "</statute>");
    }

    fn write_prologue(&self, out: &mut String) {
        if self.declaration {
            out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        }
    }

    fn write_line(&self, out: &mut String, depth: usize, content: &str) {
        for _ in 0..depth {
            out.push_str(&self.indent);
        }
        out.push_str(content);
        out.push('\n');
    }
}

impl Default for XmlExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_5::TimelineEvent;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn statutes_xml_is_wellformed_and_escaped() {
        let statutes = vec![
            Statute::new(
                "s&1",
                "Tax <Law>",
                Effect::new(EffectType::Prohibition, "Forbids \"x\""),
            )
            .with_jurisdiction("US"),
        ];
        let xml = XmlExporter::new().statutes_to_xml(&statutes);
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<statute id=\"s&amp;1\" version=\"1\">"));
        assert!(xml.contains("<title>Tax &lt;Law&gt;</title>"));
        assert!(xml.contains("<effect type=\"Prohibition\">Forbids &quot;x&quot;</effect>"));
        assert!(xml.contains("<jurisdiction>US</jurisdiction>"));
        assert!(xml.trim_end().ends_with("</statutes>"));
    }

    #[test]
    fn statute_preconditions_are_rendered() {
        let statutes = vec![
            Statute::new("s1", "Law", Effect::new(EffectType::Grant, "Grants"))
                .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18)),
        ];
        let xml = XmlExporter::new().statutes_to_xml(&statutes);
        assert!(xml.contains("<preconditions>"));
        assert!(xml.contains("<condition>Age"));
    }

    #[test]
    fn dependency_graph_xml_has_nodes_and_edges() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "requires");
        let xml = XmlExporter::new().dependency_graph_to_xml(&graph);
        assert!(xml.contains("<node id=\"a\"/>"));
        assert!(xml.contains("<node id=\"b\"/>"));
        assert!(xml.contains("<edge from=\"a\" to=\"b\" relation=\"requires\"/>"));
    }

    #[test]
    fn timeline_xml_renders_events_with_detail() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Amended {
                statute_id: "s1".to_string(),
                description: "Clarified <scope>".to_string(),
            },
        );
        timeline.add_event(
            "2021-06-01",
            TimelineEvent::Repealed {
                statute_id: "s1".to_string(),
            },
        );
        let xml = XmlExporter::new().timeline_to_xml(&timeline);
        assert!(xml.contains("<event date=\"2020-01-01\" type=\"Amended\" statute=\"s1\">"));
        assert!(xml.contains("<detail>Clarified &lt;scope&gt;</detail>"));
        assert!(xml.contains("<event date=\"2021-06-01\" type=\"Repealed\" statute=\"s1\">"));
    }

    #[test]
    fn declaration_can_be_omitted() {
        let xml = XmlExporter::new()
            .without_declaration()
            .statutes_to_xml(&[]);
        assert!(!xml.contains("<?xml"));
        assert!(xml.starts_with("<statutes>"));
    }
}
