#![cfg(test)]
use super::*;
use legalis_core::{Effect, EffectType};

#[test]
fn test_sparql_exporter_export_graph_no_prefixes() {
    let exporter = SparqlExporter::new().with_prefixes(false);
    let mut graph = DependencyGraph::new();
    graph.add_statute("test");
    let sparql = exporter.export_graph(&graph);
    assert!(!sparql.contains("PREFIX"));
    assert!(sparql.contains("INSERT DATA {"));
}
#[test]
fn test_sparql_exporter_export_concept_graph() {
    let exporter = SparqlExporter::new();
    let mut graph = ConceptRelationshipGraph::new("Test");
    let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
    let c2 = LegalConcept::new("c2", "Data", "Data concept", "rights");
    graph.add_concept(c1);
    graph.add_concept(c2);
    graph.add_relationship(ConceptRelationship::new(
        "c1",
        "c2",
        ConceptRelationType::IsA,
    ));
    let sparql = exporter.export_concept_graph(&graph);
    assert!(sparql.contains("# SPARQL INSERT Queries - Legal Concepts"));
    assert!(sparql.contains("PREFIX skos:"));
    assert!(sparql.contains("rdf:type skos:Concept"));
    assert!(sparql.contains("skos:prefLabel \"Privacy\""));
    assert!(sparql.contains("skos:broader"));
}
#[test]
fn test_sparql_exporter_export_to_turtle() {
    let exporter = SparqlExporter::new();
    let mut graph = DependencyGraph::new();
    graph.add_statute("test-statute");
    let turtle = exporter.export_to_turtle(&graph);
    assert!(turtle.contains("@prefix leg:"));
    assert!(turtle.contains("@prefix rdf:"));
    assert!(turtle.contains("@prefix rdfs:"));
    assert!(turtle.contains("rdf:type leg:Statute"));
    assert!(turtle.contains("test-statute"));
}
#[test]
fn test_sparql_exporter_default() {
    let exporter = SparqlExporter::default();
    assert!(exporter.include_prefixes);
}
#[test]
fn test_jupyter_notebook_integration_creation() {
    let integration = JupyterNotebookIntegration::new();
    assert_eq!(integration.kernel, "python3");
    assert_eq!(integration.metadata.len(), 0);
}
#[test]
fn test_jupyter_notebook_integration_with_kernel() {
    let integration = JupyterNotebookIntegration::new().with_kernel("julia-1.0");
    assert_eq!(integration.kernel, "julia-1.0");
}
#[test]
fn test_jupyter_notebook_integration_add_metadata() {
    let mut integration = JupyterNotebookIntegration::new();
    integration.add_metadata("author", "Test Author");
    integration.add_metadata("version", "1.0");
    assert_eq!(integration.metadata.len(), 2);
    assert_eq!(
        integration.metadata.get("author"),
        Some(&"Test Author".to_string())
    );
}
#[test]
fn test_jupyter_notebook_integration_create_notebook() {
    let integration = JupyterNotebookIntegration::new();
    let notebook = integration.create_notebook("Test Title", "Test Description");
    assert!(notebook.contains("\"cells\":"));
    assert!(notebook.contains("Test Title"));
    assert!(notebook.contains("Test Description"));
    assert!(notebook.contains("\"cell_type\": \"markdown\""));
    assert!(notebook.contains("\"cell_type\": \"code\""));
    assert!(notebook.contains("import matplotlib.pyplot as plt"));
    assert!(notebook.contains("import networkx as nx"));
    assert!(notebook.contains("\"nbformat\": 4"));
    assert!(notebook.contains("\"python3\""));
}
#[test]
fn test_jupyter_notebook_integration_create_decision_tree_notebook() {
    let integration = JupyterNotebookIntegration::new();
    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let notebook = integration.create_decision_tree_notebook(&tree);
    assert!(notebook.contains("Legal Decision Tree Analysis"));
    assert!(notebook.contains("Interactive visualization"));
}
#[test]
fn test_jupyter_notebook_integration_create_dependency_graph_notebook() {
    let integration = JupyterNotebookIntegration::new();
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-1");
    graph.add_statute("statute-2");
    graph.add_statute("statute-3");
    let notebook = integration.create_dependency_graph_notebook(&graph);
    assert!(notebook.contains("Legal Statute Dependencies"));
    assert!(notebook.contains("Network analysis of 3 statutes"));
}
#[test]
fn test_jupyter_notebook_integration_generate_python_code() {
    let integration = JupyterNotebookIntegration::new();
    let mut graph = DependencyGraph::new();
    graph.add_statute("statute-a");
    graph.add_statute("statute-b");
    graph.add_dependency("statute-a", "statute-b", "depends_on");
    let code = integration.generate_python_code(&graph);
    assert!(code.contains("import networkx as nx"));
    assert!(code.contains("import matplotlib.pyplot as plt"));
    assert!(code.contains("G = nx.DiGraph()"));
    assert!(code.contains("G.add_node('statute-a'"));
    assert!(code.contains("G.add_node('statute-b'"));
    assert!(code.contains("G.add_edge('statute-a', 'statute-b'"));
    assert!(code.contains("nx.draw_networkx"));
    assert!(code.contains("plt.show()"));
    assert!(code.contains("Total statutes:"));
}
#[test]
fn test_jupyter_notebook_integration_default() {
    let integration = JupyterNotebookIntegration::default();
    assert_eq!(integration.kernel, "python3");
}
